use std::{fmt::Display, ops::{Add, Sub}};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct ReplaceMeError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl TryFrom<char> for Digit {
    type Error = ReplaceMeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.to_digit(10).ok_or(ReplaceMeError)?.try_into()
    }
}

impl TryFrom<u8> for Digit {
    type Error = ReplaceMeError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Digit::Zero,
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => Err(ReplaceMeError)?
        })
    }
}

impl TryFrom<u32> for Digit {
    type Error = ReplaceMeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Digit::try_from(
            u8::try_from(value).map_err(|_| ReplaceMeError)?
        )
    }
}

impl TryFrom<u64> for Digit {
    type Error = ReplaceMeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Digit::try_from(
            u8::try_from(value).map_err(|_| ReplaceMeError)?
        )
    }
}

impl From<Digit> for char {
    fn from(value: Digit) -> Self {
        char::from_digit(value as u32, 10).unwrap()
    }
}

impl Add for Digit {
    type Output = (Digit, Digit);
    
    fn add(self, rhs: Self) -> Self::Output {
        let ans = self as u8 + rhs as u8;
        ((ans / 10).try_into().unwrap(), (ans % 10).try_into().unwrap())
    }
}

impl Sub for Digit {
    type Output = (bool, Digit);

    fn sub(self, rhs: Self) -> Self::Output {
        let ans = self as i8 - rhs as i8;
        (ans.is_negative(), ans.unsigned_abs().try_into().unwrap())
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}