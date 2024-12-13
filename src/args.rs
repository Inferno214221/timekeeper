use tokio::time::Duration;
use clap::{Arg, ArgAction, ArgGroup, Command, Id};
use derive_more::derive::{Display, Error};

use crate::{timer_mode::TimerMode, utils::{dur_from_alt_str, dur_from_str}};

#[derive(Debug, Clone)]
pub struct Args {
    pub mode: Option<TimerMode>,
    pub duration: Option<Duration>,
    pub start: bool,
    pub always_on_top: bool,
    pub follow_workspace: bool
}

#[derive(Debug, Display, Error)]
pub struct DurationParseError;

pub fn get_args() -> Args {
    let command = Command::new("timekeeper")
        .author("Inferno214221")
        .version("0.1.0")
        .about("A simple stopwatch / timer with a GUI written in Rust using Dioxus.")
        .disable_version_flag(true)
        .arg(
            Arg::new("VERSION")
            .help("Print version")
            .short('v')
            .long("version")
            .action(ArgAction::Version)
        ).arg(
            Arg::new("STOPWATCH")
                .help("Launch the program in stopwatch mode")
                .short('w')
                .long("stopwatch")
                .visible_short_alias('u')
                .visible_alias("up")
                .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("TIMER")
                .help("Launch the program in timer mode")
                .short('t')
                .long("timer")
                .visible_short_alias('d')
                .visible_alias("down")
                .action(ArgAction::SetTrue)
        ).group(
            ArgGroup::new("MODE")
                .args(["STOPWATCH", "TIMER"])
                .multiple(false)
        ).arg(
            Arg::new("DURATION")
                .help("The initial duration of set the stopwatch / timer (e.g. 1:00:00 or 1h)")
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
                .visible_short_alias('r')
                .visible_alias("run")
                .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("KEEP_ON_TOP")
                .help("Attempt to keep the window above others")
                .short('k')
                .long("keep-on-top")
                .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("FOLLOW_WORKSPACE")
                .help("Attempt to keep the window on the current workspace at all times")
                .short('f')
                .long("follow-workspaces")
                .action(ArgAction::SetTrue)
        );

    let matches = command.get_matches();

    Args {
        mode: matches.get_one::<Id>("MODE").map(
            |m| m.to_string().parse()
                .expect("Incorrect flags can't be passed so parse() can't fail")
        ),
        duration: matches.get_one::<Duration>("DURATION").cloned(),
        start: matches.get_flag("START"),
        always_on_top: matches.get_flag("KEEP_ON_TOP"),
        follow_workspace: matches.get_flag("FOLLOW_WORKSPACE")
    }
}