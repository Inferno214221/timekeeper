use std::fmt::Display;

use crate::digit::digit::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DoubleDigits(pub [Digit; 2]);

impl DoubleDigits {
    pub fn is_zero(&self) -> bool {
        self.0 == [Digit::Zero, Digit::Zero]
    }
}

impl Display for DoubleDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map(|d| d.to_string()).concat())
    }
}

impl TryFrom<u8> for DoubleDigits {
    type Error = ReplaceMeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 100 {
            Ok(DoubleDigits([
                Digit::try_from(value / 10).unwrap(),
                Digit::try_from(value % 10).unwrap()
            ]))
        } else {
            Err(ReplaceMeError)
        }
    }
}

impl TryFrom<u64> for DoubleDigits {
    type Error = ReplaceMeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        DoubleDigits::try_from(
            u8::try_from(value).map_err(|_| ReplaceMeError)?
        )
    }
}

impl From<DoubleDigits> for u8 {
    fn from(value: DoubleDigits) -> Self {
        value.0[0] as u8 * 10 + value.0[1] as u8
    }
}