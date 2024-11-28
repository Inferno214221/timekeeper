use std::fmt::Display;

use crate::digit::digit::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Digits(pub Vec<Digit>);

impl Digits {
    pub fn is_zero(&self) -> bool {
        !self.0.iter().any(|&d| d != Digit::Zero)
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            self.0.iter().map(|d| char::from(*d)).collect::<String>()
        )
    }
}

impl From<u64> for Digits {
    fn from(value: u64) -> Self {
        let mut digits = Vec::new();
        let mut mut_value = value;
        while mut_value > 0 {
            digits.push(Digit::try_from(mut_value % 10).unwrap());
            mut_value /= 10;
        }
        Digits(digits.into_iter().rev().collect())
    }
}

impl TryFrom<Digits> for u64 {
    type Error = ReplaceMeError;
    
    fn try_from(value: Digits) -> Result<Self, Self::Error> {
        if value.0.is_empty() { return Ok(0); }
        value.0.into_iter().map(|d| d as u64).try_reduce(
            |c, d| c.checked_mul(10).and_then(|o| o.checked_add(d)).ok_or(ReplaceMeError)
        )?.ok_or(ReplaceMeError)
    }
}