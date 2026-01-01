use crate::{
    arch::{
        instructions::{ControlSignals, Instruction},
        trit::{Trit, Tryte},
    },
    core::{
        address_space::AddressSpace,
        alu::ArithmeticLogicUnit,
        registers::{RegAddr, Registers},
    },
};

pub struct CentralProcessingUnit {
    registers: Registers,
    address_space: AddressSpace,
    arithmetic_logic_unit: ArithmeticLogicUnit,
    current_instruction: Instruction,
    control_signals: ControlSignals,
    immediate: i32,
}

impl CentralProcessingUnit {
    pub fn from(
        registers: Registers,
        address_space: AddressSpace,
        arithmetic_logic_unit: ArithmeticLogicUnit,
    ) -> Self {
        Self {
            registers,
            address_space,
            arithmetic_logic_unit,
            current_instruction: Instruction::Nop,
            control_signals: ControlSignals::default(),
            immediate: 0,
        }
    }
}

impl CentralProcessingUnit {
    fn fetch(&mut self) -> Tryte {
        let pc_val = *self.registers.read_pc();
        self.address_space.read(pc_val)
    }

    fn decode(&mut self, raw_instr: Tryte) {
        self.current_instruction = Instruction::from(raw_instr);

        let (signals, imm) = self.current_instruction.decode();
        self.control_signals = signals;
        self.immediate = imm;
    }

    fn execute(&mut self) {
        let instr = self.current_instruction;
        let signals = self.control_signals;

        let rs1_addr = self.usize_to_regaddr(instr.rs1());
        let rs2_addr = self.usize_to_regaddr(instr.rs2());
        let rd_addr = self.usize_to_regaddr(instr.rd());

        let r_val_1 = self.registers.read_gpr(rs1_addr);
        let r_val_2 = self.registers.read_gpr(rs2_addr);

        let input_a = r_val_1;

        // Mux: Choose between Register 2 or Immediate
        let input_b = if signals.alu_src {
            // Convert i32 immediate to Tryte
            Tryte::from_i128(self.immediate as i128)
        } else {
            r_val_2
        };

        self.arithmetic_logic_unit.alu_reset();
        self.arithmetic_logic_unit
            .alu_set(input_a, input_b, signals.alu_op);
        self.arithmetic_logic_unit.alu_exec();
        let alu_result = self.arithmetic_logic_unit.result;

        let mut result_to_write = alu_result;

        // Store
        if signals.mem_write {
            self.address_space.write(alu_result, r_val_2);
        }

        if signals.mem_read {
            result_to_write = self.address_space.read(alu_result);
        }

        if signals.reg_write {
            let final_data = if signals.mem_to_reg {
                result_to_write
            } else if signals.jump {
                let current_pc_val = self.registers.read_pc().to_i128();
                Tryte::from_i128(current_pc_val + 1)
            } else {
                alu_result
            };

            self.registers.write_gpr(rd_addr, final_data);
        }

        self.update_pc(signals);
    }

    fn update_pc(&mut self, signals: ControlSignals) {
        let current_pc = self.registers.read_pc().to_i128();
        let zero_flag = self.arithmetic_logic_unit.zero_flag;

        let next_pc_val = if signals.jump {
            current_pc + (self.immediate as i128)
        } else if signals.branch && (zero_flag == Trit::Positive) {
            current_pc + (self.immediate as i128)
        } else {
            current_pc + 1
        };

        self.registers.write_pc(&Tryte::from_i128(next_pc_val));
    }

    pub fn cycle(&mut self) {
        let raw_instr = self.fetch();
        self.decode(raw_instr);
        self.execute();
    }

    fn usize_to_regaddr(&self, index: usize) -> RegAddr {
        RegAddr::from_i128(index as i128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        address_space::AddressSpace, alu::ArithmeticLogicUnit, registers::Registers,
    };

    // Constants from your Instruction enum logic
    const OP_ADD: i128 = 1;
    const OP_ADDI: i128 = 3;

    #[test]
    fn test_cpu_arithmetic() {
        // 1. Setup
        let mut regs = Registers::default();
        let mut mem = AddressSpace::default();
        let alu = ArithmeticLogicUnit::default();

        // 2. Load Program into Memory
        // Instruction 0: ADDI x1, x0, 5  (Set Register 1 to 5)
        let instr0 = create_instruction(OP_ADDI, 1, 0, 0, 5);
        mem.write(Tryte::from_i128(0), instr0);

        // Instruction 1: ADDI x2, x0, 10 (Set Register 2 to 10)
        let instr1 = create_instruction(OP_ADDI, 2, 0, 0, 10);
        mem.write(Tryte::from_i128(1), instr1);

        // Instruction 2: ADD x3, x1, x2  (Set Register 3 to x1 + x2 = 15)
        let instr2 = create_instruction(OP_ADD, 3, 1, 2, 0);
        mem.write(Tryte::from_i128(2), instr2);

        // 3. Initialize CPU
        let mut cpu = CentralProcessingUnit::from(regs, mem, alu);

        // 4. Run Cycles
        cpu.cycle(); // Exec ADDI x1
        cpu.cycle(); // Exec ADDI x2
        cpu.cycle(); // Exec ADD x3

        // 5. Assertions
        // Check Register 3
        let reg3_addr = RegAddr::from_i128(3);
        let result = cpu.registers.read_gpr(reg3_addr);

        assert_eq!(
            result.to_i128(),
            15,
            "Register 3 should contain 5 + 10 = 15"
        );

        // Check PC incremented correctly (3 instructions executed = PC 3)
        assert_eq!(cpu.registers.read_pc().to_i128(), 3, "PC should be at 3");
    }

    #[test]
    fn test_cpu_memory_store_load() {
        // 1. Setup
        let regs = Registers::default();
        let mut mem = AddressSpace::default();
        let alu = ArithmeticLogicUnit::default();

        const OP_ADDI: i128 = 3;
        const OP_LW: i128 = 4;
        const OP_SW: i128 = 5;

        // 2. Load Program
        // Instr 0: ADDI x1, x0, 42   -> Load value 42 into Reg 1
        mem.write(
            Tryte::from_i128(0),
            create_instruction(OP_ADDI, 1, 0, 0, 42),
        );

        // Instr 1: SW x1, x0, 100    -> Store Reg 1 (42) into Mem[0 + 100]
        // Note: rs1=0 (base), rs2=1 (src data), imm=100 (offset)
        mem.write(Tryte::from_i128(1), create_instruction(OP_SW, 0, 0, 1, 100));

        // Instr 2: LW x2, x0, 100    -> Load Mem[0 + 100] into Reg 2
        mem.write(Tryte::from_i128(2), create_instruction(OP_LW, 2, 0, 0, 100));

        // 3. Init CPU
        let mut cpu = CentralProcessingUnit::from(regs, mem, alu);

        // 4. Run Cycles
        cpu.cycle(); // Reg 1 = 42
        cpu.cycle(); // Mem[100] = 42

        // Verify intermediate state: Check if memory actually updated
        let mem_addr_100 = Tryte::from_i128(100);
        assert_eq!(
            cpu.address_space.read(mem_addr_100).to_i128(),
            42,
            "Memory at 100 should be 42"
        );

        cpu.cycle(); // Reg 2 = Mem[100]

        // 5. Assertions
        let reg2_addr = RegAddr::from_i128(2);
        let result = cpu.registers.read_gpr(reg2_addr);

        assert_eq!(
            result.to_i128(),
            42,
            "Register 2 should have loaded 42 from memory"
        );
    }

    // --- Test Helper ---

    /// Encodes instruction fields into a single Tryte (Machine Code)
    /// Layout: [Op:0..5] [Rd:5..8] [Rs1:8..11] [Rs2:11..14] [Imm:14..27]
    fn create_instruction(opcode: i128, rd: i128, rs1: i128, rs2: i128, imm: i128) -> Tryte {
        // Basic Shift Logic: value * 3^shift
        let mut total = 0;

        total += opcode * 3_i128.pow(0);
        total += rd * 3_i128.pow(5);
        total += rs1 * 3_i128.pow(8);
        total += rs2 * 3_i128.pow(11);
        total += imm * 3_i128.pow(14);

        Tryte::from_i128(total)
    }
}
