use tokio::time::Duration;
use clap::{Arg, ArgAction, ArgGroup, Command, Id};
use derive_more::derive::{Display, Error};

use crate::{timer_mode::TimerMode, utils::{dur_from_alt_str, dur_from_str}};

#[derive(Debug, Clone)]
pub struct Args {
    pub mode: Option<TimerMode>,
    pub duration: Option<Duration>,
    pub start: bool
}

#[derive(Debug, Display, Error)]
pub struct DurationParseError;

pub fn get_args() -> Args {
    let command = Command::new("simple-stopwatch-timer")
        .author("Inferno214221")
        .version("0.1.0")
        .about("A simple stopwatch / timer with a GUI written in Rust using Dioxus.")
        .disable_version_flag(true)
        .arg(
            Arg::new("VERSION").help(
                "Print version"
            ).short('v').long("version").action(ArgAction::Version)
        ).arg(
            Arg::new("STOPWATCH")
                .help("Launch the program in stopwatch mode")
                .short('w')
                .long("stopwatch")
                .short_alias('u')
                .alias("up")
                .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("TIMER")
                .help("Launch the program in timer mode")
                .short('t') // t / d
                .long("timer")
                .short_alias('d')
                .alias("down")
                .action(ArgAction::SetTrue)
        ).group(
            ArgGroup::new("MODE")
                .args(["STOPWATCH", "TIMER"])
                .multiple(false)
        ).arg(
            Arg::new("DURATION")
                .help("The initial duration of set the stopwatch / timer")
                .action(ArgAction::Set)
                .value_parser(|s: &str| {
                    dur_from_str(s)
                        .or_else(|| dur_from_alt_str(s))
                        .ok_or(DurationParseError)
                })
        ).arg(
            Arg::new("START")
                .help("Start the stopwatch / timer immediately")
                .short('s')
                .long("start")
                .short_alias('r')
                .alias("run")
                .action(ArgAction::SetTrue)
        );

    let matches = command.get_matches();

    Args {
        // Incorrect flags can't be passed so parse() can't fail
        mode: matches.get_one::<Id>("MODE").and_then(|m| m.to_string().parse().ok()),
        duration: matches.get_one::<Duration>("DURATION").cloned(),
        start: matches.get_flag("START")
    }
}