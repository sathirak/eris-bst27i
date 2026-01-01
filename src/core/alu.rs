use crate::arch::{
    circuits::ErisCircuit,
    instructions::AluOp,
    trit::{Trit, TritField, Tryte},
};

#[derive(Default)]
pub struct ArithmeticLogicUnit {
    circuit: ErisCircuit,
    pub result: Tryte,
    pub zero_flag: Trit,
    input_a: Tryte,
    input_b: Tryte,
    alu_ctrl: AluOp,
}

impl ArithmeticLogicUnit {
    pub fn alu_set(&mut self, input_a: Tryte, input_b: Tryte, alu_ctrl: AluOp) {
        self.input_a = input_a;
        self.input_b = input_b;
        self.alu_ctrl = alu_ctrl;
    }

    pub fn alu_exec(&mut self) {
        match self.alu_ctrl {
            AluOp::Add => self.add(),
            AluOp::Sub => self.sub(),
            AluOp::PassB => {
                self.result = self.input_b;
            }
            AluOp::None => {}
        }
    }

    pub fn alu_reset(&mut self) {
        self.result = TritField::default();
        self.zero_flag = Trit::default();
        self.input_a = TritField::default();
        self.input_b = TritField::default();
        self.alu_ctrl = AluOp::default();
    }
}

impl ArithmeticLogicUnit {
    /// Adds input_a and input_b, storing the result and setting flags.
    pub fn add(&mut self) {
        let mut carry = Trit::Zero;
        let mut is_zero_result = true;

        // Iterate from Least Significant Trit (0) to Most Significant (26)
        for i in 0..27 {
            let a = self.input_a.0[i];
            let b = self.input_b.0[i];

            // Use the circuit's full adder
            let (sum, new_carry) = self.circuit.full_trit_adder(a, b, carry);

            if sum != Trit::Zero {
                is_zero_result = false;
            }

            self.result.0[i] = sum;
            carry = new_carry;
        }

        self.zero_flag = if is_zero_result {
            Trit::Positive
        } else {
            Trit::Zero
        };
    }

    /// Subtracts input_b from input_a (A - B)
    /// Logic: A + (-B)
    pub fn sub(&mut self) {
        let mut carry = Trit::Zero;
        let mut is_zero_result = true;

        for i in 0..27 {
            let a = self.input_a.0[i];
            let b = self.input_b.0[i];

            // In Balanced Ternary, negation is just inverting the Trit.
            // 1 becomes -1, -1 becomes 1, 0 stays 0.
            let b_negated = match b {
                Trit::Positive => Trit::Negative,
                Trit::Zero => Trit::Zero,
                Trit::Negative => Trit::Positive,
            };

            let (sum, new_carry) = self.circuit.full_trit_adder(a, b_negated, carry);

            if sum != Trit::Zero {
                is_zero_result = false;
            }

            self.result.0[i] = sum;
            carry = new_carry;
        }

        self.zero_flag = if is_zero_result {
            Trit::Positive
        } else {
            Trit::Zero
        };
    }
}
