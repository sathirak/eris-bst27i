use crate::arch::trit::Trit;

#[derive(Default)]
pub struct ErisCircuit {}

impl ErisCircuit {
    /// Full Trit Adder logic: returns (Sum, Carry)
    /// Balanced Ternary: -1, 0, 1
    pub fn full_trit_adder(&self, input_a: Trit, input_b: Trit, carry_in: Trit) -> (Trit, Trit) {
        let a = input_a.to_i8();
        let b = input_b.to_i8();
        let c = carry_in.to_i8();

        let sum_raw = a + b + c;

        // Balanced Ternary mapping:
        // Sum 2  => Trit::Negative (-1), Carry: Trit::Positive (1)
        // Sum 3  => Trit::Zero     (0),  Carry: Trit::Positive (1)
        // Sum -2 => Trit::Positive (1),  Carry: Trit::Negative (-1)
        // Sum -3 => Trit::Zero     (0),  Carry: Trit::Negative (-1)
        let (s, c_out) = match sum_raw {
            3 => (0, 1),
            2 => (-1, 1),
            1 => (1, 0),
            0 => (0, 0),
            -1 => (-1, 0),
            -2 => (1, -1),
            -3 => (0, -1),
            _ => unreachable!(),
        };

        (Trit::from_i8(s), Trit::from_i8(c_out))
    }

    /// Minimum function (Equivalent to Kleene Logic AND)
    /// Negative < Zero < Positive
    pub fn min(&self, input_a: Trit, input_b: Trit) -> Trit {
        match (input_a, input_b) {
            (Trit::Negative, _) | (_, Trit::Negative) => Trit::Negative,
            (Trit::Zero, _) | (_, Trit::Zero) => Trit::Zero,
            (Trit::Positive, Trit::Positive) => Trit::Positive,
        }
    }
}
