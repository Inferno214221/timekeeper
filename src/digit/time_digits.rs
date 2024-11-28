use std::{fmt::Display, time::Duration};

use crate::digit::digit::*;
use crate::digit::digits::*;
use crate::digit::double_digits::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDigits {
    hours: Digits,
    mins: DoubleDigits,
    secs: DoubleDigits
}

impl TimeDigits {
    pub fn push(&mut self, value: Digit) {
        self.hours.0.push(self.mins.0[0]);
        self.mins.0[0] = self.mins.0[1];
        self.mins.0[1] = self.secs.0[0];
        self.secs.0[0] = self.secs.0[1];
        self.secs.0[1] = value;
    }

    pub fn pop(&mut self) -> Digit {
        // TODO: return an Option?
        let ret_val = self.secs.0[1];
        self.secs.0[1] = self.secs.0[0];
        self.secs.0[0] = self.mins.0[1];
        self.mins.0[1] = self.mins.0[0];
        self.mins.0[0] = self.hours.0.pop().unwrap_or(Digit::Zero);
        ret_val
    }

    pub fn as_secs(&self) -> u64 {
        todo!()
    }
}

impl Display for TimeDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hours.is_zero() {
            if self.mins.is_zero() {
                
                write!(f, "{}", trim_zeros(&self.secs.to_string()))
            } else {
                write!(f, "{}:{:0>2}", trim_zeros(&self.mins.to_string()), self.secs)
            }
        } else {
            write!(f, "{}:{:0>2}:{:0>2}", trim_zeros(&self.hours.to_string()), self.mins, self.secs)
        }
    }
}

fn trim_zeros(s: &str) -> String {
    let mut trimmed = s.trim_start_matches('0').to_owned();
    if trimmed.is_empty() {
        trimmed = String::from("0")
    }
    trimmed
}

impl From<Duration> for TimeDigits {
    fn from(value: Duration) -> Self {
        let dur_secs = value.as_secs();
        TimeDigits {
            hours: Digits::from(dur_secs / 3600),
            mins: DoubleDigits::try_from((dur_secs % 3600) / 60).unwrap(),
            secs: DoubleDigits::try_from(dur_secs % 60).unwrap()
        }
    }
}

impl TryFrom<&TimeDigits> for Duration {
    type Error = ReplaceMeError;

    fn try_from(value: &TimeDigits) -> Result<Self, Self::Error> {
        Ok(Duration::from_secs(
            u64::try_from(value.hours.clone())?.checked_mul(3600)
                .and_then(
                    |s| s.checked_add((u8::from(value.mins) as u64).checked_mul(60)?)
                )
                .and_then(
                    |s| s.checked_add(u8::from(value.secs) as u64)
                ).ok_or(ReplaceMeError)?
        ))
    }
}