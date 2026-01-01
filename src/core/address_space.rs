use std::collections::HashMap;

use crate::arch::trit::{TritField, Tryte};

pub type Address = TritField<27>;

#[derive(Default)]
pub struct AddressSpace {
    mmio: HashMap<Address, Tryte>,
}

impl AddressSpace {
    pub fn read(&self, address: Address) -> TritField<27> {
        self.mmio.get(&address).copied().unwrap_or_default()
    }

    pub fn write(&mut self, address: Address, value: Tryte) {
        self.mmio.insert(address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_success() {
        let mut space = AddressSpace {
            mmio: HashMap::new(),
        };

        let addr = Address::from_i128(12345); // Assuming TritField can be created from an integer
        let val = Tryte::from_i128(42);

        space.write(addr, val);

        let result = space.read(addr);
        assert_eq!(
            result, val,
            "The value read should match the value written."
        );
    }

    #[test]
    fn test_read_empty_address_returns_default() {
        let space = AddressSpace {
            mmio: HashMap::new(),
        };

        let addr = Address::from_i128(999);
        let result = space.read(addr);

        // This tests your .unwrap_or_default() logic
        assert_eq!(
            result,
            TritField::default(),
            "Reading an unmapped address should return the default value."
        );
    }

    #[test]
    fn test_overwrite_address() {
        let mut space = AddressSpace {
            mmio: HashMap::new(),
        };

        let addr = Address::from_i128(55);
        let first_val = Tryte::from_i128(10);
        let second_val = Tryte::from_i128(20);

        space.write(addr, first_val);
        space.write(addr, second_val); // Overwrite

        let result = space.read(addr);
        assert_eq!(
            result, second_val,
            "The address should hold the most recently written value."
        );
    }

    #[test]
    fn test_multiple_addresses_independence() {
        let mut space = AddressSpace {
            mmio: HashMap::new(),
        };

        let addr_a = Address::from_i128(1);
        let addr_b = Address::from_i128(2);
        let val_a = Tryte::from_i128(100);
        let val_b = Tryte::from_i128(200);

        space.write(addr_a, val_a);
        space.write(addr_b, val_b);

        assert_eq!(space.read(addr_a), val_a);
        assert_eq!(space.read(addr_b), val_b);
    }
}
