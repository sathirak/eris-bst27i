use crate::arch::trit::Tryte;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    // Arithmetic (R-Format equivalent)
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },

    // Immediate Arithmetic (I-Format)
    Addi { rd: usize, rs1: usize, imm: i32 },

    // Memory
    Lw { rd: usize, rs1: usize, imm: i32 },
    Sw { rs1: usize, rs2: usize, imm: i32 },

    // Branching & Jumping
    Beq { rs1: usize, rs2: usize, imm: i32 }, // Branch if Equal
    Jal { rd: usize, imm: i32 },              // Jump and Link

    // Upper Immediate
    Lui { rd: usize, imm: i32 },
    // NOP / Invalid
    Nop,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ControlSignals {
    pub alu_op: AluOp,
    pub alu_src: bool,
    pub reg_write: bool,
    pub mem_read: bool,
    pub mem_write: bool,
    pub mem_to_reg: bool,
    pub branch: bool,
    pub jump: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AluOp {
    Add,
    Sub,
    PassB,
    #[default]
    None,
}

impl Instruction {
    pub fn decode(&self) -> (ControlSignals, i32) {
        use Instruction::*;

        let mut signals = ControlSignals::default();
        let mut immediate = 0;

        match self {
            // --- R-Type: ADD, SUB ---
            Add { .. } => {
                signals.alu_op = AluOp::Add;
                signals.reg_write = true;
                signals.alu_src = false; // Use Register rs2
            }
            Sub { .. } => {
                signals.alu_op = AluOp::Sub;
                signals.reg_write = true;
                signals.alu_src = false;
            }

            // --- I-Type: ADDI ---
            Addi { imm, .. } => {
                signals.alu_op = AluOp::Add;
                signals.reg_write = true;
                signals.alu_src = true; // Use Immediate
                immediate = *imm;
            }

            // --- Memory: LW (Load Word) ---
            Lw { imm, .. } => {
                signals.alu_op = AluOp::Add; // Calc address: rs1 + imm
                signals.reg_write = true;
                signals.alu_src = true;
                signals.mem_read = true; // Read from mem
                signals.mem_to_reg = true; // Data comes from mem, not ALU
                immediate = *imm;
            }

            // --- Memory: SW (Store Word) ---
            Sw { imm, .. } => {
                signals.alu_op = AluOp::Add; // Calc address: rs1 + imm
                signals.mem_write = true; // Write to mem
                signals.alu_src = true;
                immediate = *imm;
            }

            // --- Branch: BEQ ---
            Beq { imm, .. } => {
                signals.alu_op = AluOp::Sub; // Compare by subtracting
                signals.branch = true;
                signals.alu_src = false; // Compare two registers
                immediate = *imm;
            }

            // --- Jump: JAL ---
            Jal { imm, .. } => {
                signals.jump = true;
                signals.reg_write = true; // Save PC+4 to rd
                // JAL often uses a specialized ALU path or dedicated adder,
                // but for simple alu_ctrl we might set it to Pass or Add.
                immediate = *imm;
            }

            // --- Upper Immediate: LUI ---
            Lui { imm, .. } => {
                signals.alu_op = AluOp::PassB; // Pass immediate through ALU
                signals.reg_write = true;
                signals.alu_src = true;
                immediate = *imm; // Ensure this is shifted correctly (<< 12) beforehand or here
            }

            Nop => {}
        }

        (signals, immediate)
    }
}

// Constants for Opcode Mapping
const OP_ADD: i128 = 1;
const OP_SUB: i128 = 2;
const OP_ADDI: i128 = 3;
const OP_LW: i128 = 4;
const OP_SW: i128 = 5;
const OP_BEQ: i128 = 6;
const OP_JAL: i128 = 7;
const OP_LUI: i128 = 8;
// const OP_HALT: i128 = 0; // standard zero is usually NOP or HALT

impl From<Tryte> for Instruction {
    fn from(machine_code: Tryte) -> Self {
        let opcode = extract_value(&machine_code, 0, 5);
        let rd = extract_value(&machine_code, 5, 8) as usize;
        let rs1 = extract_value(&machine_code, 8, 11) as usize;
        let rs2 = extract_value(&machine_code, 11, 14) as usize;

        // Immediate covers the upper part.
        let imm_long = extract_value(&machine_code, 14, 27);
        let imm = imm_long as i32;

        // Match Opcode to Instruction Variant
        match opcode {
            OP_ADD => Instruction::Add { rd, rs1, rs2 },
            OP_SUB => Instruction::Sub { rd, rs1, rs2 },

            OP_ADDI => Instruction::Addi { rd, rs1, imm },

            // For Stores, 'rd' is essentially irrelevant in destination logic,
            // but standard encoding often keeps the field layout consistent.
            OP_LW => Instruction::Lw { rd, rs1, imm },
            OP_SW => Instruction::Sw { rs1, rs2, imm },

            OP_BEQ => Instruction::Beq { rs1, rs2, imm },

            OP_JAL => Instruction::Jal { rd, imm },

            OP_LUI => Instruction::Lui { rd, imm },

            _ => Instruction::Nop, // Unknown opcode maps to NOP
        }
    }
}

fn extract_value(tryte: &Tryte, start: usize, end: usize) -> i128 {
    let mut value: i128 = 0;
    let mut power: i128 = 1;

    for i in start..end {
        if i >= 27 {
            break;
        }

        let trit_val = tryte.0[i].to_i8() as i128;
        value += trit_val * power;
        power *= 3;
    }

    value
}

impl Instruction {
    pub fn rs1(&self) -> usize {
        match self {
            Instruction::Add { rs1, .. }
            | Instruction::Sub { rs1, .. }
            | Instruction::Addi { rs1, .. }
            | Instruction::Lw { rs1, .. }
            | Instruction::Sw { rs1, .. }
            | Instruction::Beq { rs1, .. } => *rs1,
            _ => 0,
        }
    }

    pub fn rs2(&self) -> usize {
        match self {
            Instruction::Add { rs2, .. }
            | Instruction::Sub { rs2, .. }
            | Instruction::Sw { rs2, .. }
            | Instruction::Beq { rs2, .. } => *rs2,
            _ => 0,
        }
    }

    pub fn rd(&self) -> usize {
        match self {
            Instruction::Add { rd, .. }
            | Instruction::Sub { rd, .. }
            | Instruction::Addi { rd, .. }
            | Instruction::Lw { rd, .. }
            | Instruction::Jal { rd, .. }
            | Instruction::Lui { rd, .. } => *rd,
            _ => 0,
        }
    }
}
