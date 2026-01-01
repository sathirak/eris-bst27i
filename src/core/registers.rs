use std::collections::HashMap;

use crate::arch::trit::{TritField, Tryte};

pub type RegAddr = TritField<3>;

#[derive(Default)]
pub struct Registers {
    pc: Tryte,
    gpr: HashMap<RegAddr, Tryte>,
}

impl Registers {
    pub fn read_pc(&self) -> &Tryte {
        &self.pc
    }

    pub fn write_pc(&mut self, new_pc: &Tryte) {
        self.pc = *new_pc;
    }

    pub fn read_gpr(&self, index: RegAddr) -> Tryte {
        if index.to_i128() == 0 {
            Tryte::default()
        } else {
            self.gpr.get(&index).copied().unwrap_or_default()
        }
    }

    pub fn write_gpr(&mut self, index: RegAddr, value: Tryte) {
        if index.to_i128() == 0 {
        } else {
            self.gpr.insert(index, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a RegAddr from an integer
    fn make_reg(val: i128) -> RegAddr {
        RegAddr::from_i128(val)
    }

    #[test]
    fn test_pc_read_write() {
        let mut regs = Registers {
            pc: Tryte::default(),
            gpr: HashMap::new(),
        };

        let new_val = Tryte::from_i128(123);
        regs.write_pc(&new_val);

        assert_eq!(*regs.read_pc(), new_val);
    }

    #[test]
    fn test_gpr_zero_register_is_immutable() {
        let mut regs = Registers {
            pc: Tryte::default(),
            gpr: HashMap::new(),
        };

        let r0 = make_reg(0);
        let some_val = Tryte::from_i128(99);

        // Attempt to write to R0
        regs.write_gpr(r0, some_val);

        // R0 must always return 0 (Tryte::default)
        assert_eq!(
            regs.read_gpr(r0).to_i128(),
            0,
            "Register 0 must remain zero"
        );
    }

    #[test]
    fn test_gpr_read_write() {
        let mut regs = Registers {
            pc: Tryte::default(),
            gpr: HashMap::new(),
        };

        let r1 = make_reg(1);
        let r2 = make_reg(2);
        let val1 = Tryte::from_i128(42);
        let val2 = Tryte::from_i128(-5);

        regs.write_gpr(r1, val1);
        regs.write_gpr(r2, val2);

        assert_eq!(regs.read_gpr(r1), val1);
        assert_eq!(regs.read_gpr(r2), val2);
    }

    #[test]
    fn test_uninitialized_register_defaults_to_zero() {
        let regs = Registers {
            pc: Tryte::default(),
            gpr: HashMap::new(),
        };

        let r10 = make_reg(10);
        assert_eq!(regs.read_gpr(r10).to_i128(), 0);
    }
}
