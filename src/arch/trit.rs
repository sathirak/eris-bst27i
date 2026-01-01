use std::fmt::{Display, Formatter, Result};

// TRIT
#[derive(PartialEq, Debug, Copy, Clone, Default, Hash, Eq)]
pub enum Trit {
    Negative, // -1
    #[default]
    Zero, //  0
    Positive, // +1
}

impl Display for Trit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_i8())
    }
}

impl Trit {
    pub fn to_i8(self) -> i8 {
        match self {
            Trit::Negative => -1,
            Trit::Zero => 0,
            Trit::Positive => 1,
        }
    }

    pub fn from_i8(val: i8) -> Self {
        match val {
            -1 => Trit::Negative,
            0 => Trit::Zero,
            1 => Trit::Positive,
            _ => Trit::Zero, // Default fallback
        }
    }
}

// TRITFIELD
#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct TritField<const N: usize>(pub [Trit; N]);

/// 7,625,597,484,987 = 7TB
pub type Tryte = TritField<27>;

impl<const N: usize> TritField<N> {
    pub fn to_i128(&self) -> i128 {
        let mut value: i128 = 0;
        let mut power: i128 = 1;

        self.0.iter().for_each(|trit| {
            value += trit.to_i8() as i128 * power;
            power *= 3;
        });

        value
    }

    pub fn from_i128(value: i128) -> TritField<N> {
        let mut result = TritField::default();
        let mut n = value;

        // Little-Endian is standard for RISC-style emulators
        for i in 0..N {
            if n == 0 {
                break;
            }

            let mut rem = n % 3;
            n /= 3;

            if rem > 1 {
                rem -= 3;
                n += 1;
            } else if rem < -1 {
                rem += 3;
                n -= 1;
            }

            result.0[i] = match rem {
                -1 => Trit::Negative,
                0 => Trit::Zero,
                1 => Trit::Positive,
                _ => unreachable!(),
            };
        }
        result
    }
}

impl<const N: usize> Default for TritField<N> {
    fn default() -> Self {
        Self([Trit::default(); N])
    }
}

impl<const N: usize> Display for TritField<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for trit in self.0.iter() {
            write!(f, "{},", trit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use env_logger;

    use crate::arch::trit::{Trit, TritField};

    #[test]
    fn test_trit_conversion() {
        let _ = env_logger::try_init();

        // 1. Test Negative ( -1 -3 -9 = -13 )
        let source = TritField([Trit::Negative, Trit::Negative, Trit::Negative]);
        let decimal = source.to_i128();
        assert_eq!(decimal, -13);

        // 2. Convert back to a larger field (5 trits)
        let wider = TritField::<5>::from_i128(decimal);

        println!("TritField {}", wider);

        // Check if the first 3 trits match the source
        assert_eq!(wider.0[0], Trit::Negative);
        assert_eq!(wider.0[1], Trit::Negative);
        assert_eq!(wider.0[2], Trit::Negative);
        // Check if padding is Zero
        assert_eq!(wider.0[3], Trit::Zero);
        assert_eq!(wider.0[4], Trit::Zero);

        println!("Decimal: {}, Wider Field: {:?}", decimal, wider);
    }
}
