use tokio::time::Duration;
use derive_more::derive::{Display, Error};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimerMode {
    Timer,
    Stopwatch
}

impl TimerMode {
    pub fn default_dur(&self) -> Duration {
        match *self {
            TimerMode::Timer => Duration::from_secs(5 * 60),
            TimerMode::Stopwatch => Duration::ZERO,
        }
    }
}

#[derive(Debug, Display, Error)]
pub struct InvalidEnumValue;

impl FromStr for TimerMode {
    type Err = InvalidEnumValue;
    
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "timer" => Ok(TimerMode::Timer),
            "stopwatch" => Ok(TimerMode::Stopwatch),
            _ => Err(InvalidEnumValue)
        }
    }
}