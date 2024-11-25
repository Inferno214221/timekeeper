use std::str::FromStr;

use tokio::time::{self, Duration, MissedTickBehavior};
use soloud::*;
use derive_more::derive::{Display, Error};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dioxus::desktop::{self, tao, Config};

fn main() {
    // TODO: add cli args
    dioxus_logger::init(Level::INFO).expect("Logger initialisation failed");
    const MIN_SIZE: tao::dpi::LogicalSize<u32> =
        tao::dpi::LogicalSize::new(200, 160);

    let window = tao::window::WindowBuilder::new()
        .with_title("Simple Stopwatch")
        .with_resizable(true)
        .with_inner_size(MIN_SIZE)
        .with_min_inner_size(MIN_SIZE);

    LaunchBuilder::new().with_cfg(
        Config::new()
            .with_window(window)
            .with_menu(None)
    ).launch(App);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TimerMode {
    Timer,
    Stopwatch
}

impl TimerMode {
    fn default_dur(&self) -> Duration {
        match *self {
            TimerMode::Timer => Duration::from_secs(5 * 60),
            TimerMode::Stopwatch => Duration::ZERO,
        }
    }

    fn default_digits(&self) -> u64 {
        match *self {
            TimerMode::Timer => 500,
            TimerMode::Stopwatch => 0,
        }
    }

    fn win_title(&self) -> &str {
        match *self {
            TimerMode::Timer => "Simple Timer",
            TimerMode::Stopwatch => "Simple Stopwatch",
        }
    }
}

#[derive(Debug, Display, Error)]
struct InvalidEnumValue;

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

#[component]
fn App() -> Element {
    let mut mode = use_signal(|| TimerMode::Stopwatch);
    // ? is it even worth having this seperate?

    use_effect(move || desktop::window().set_title(mode.read().win_title()));

    let set_mode = move |event: Event<FormData>| {
        mode.set(event.value().parse().unwrap());
    };

    rsx! {
        style {{ include_str!("../assets/main.css") }}
        div {
            id: "mode-radio",
            input {
                r#type: "radio",
                name: "mode",
                value: "stopwatch",
                id: "mode-stopwatch",
                onchange: set_mode,
                checked: true
            }
            label {
                r#for: "mode-stopwatch",
                span {
                    class: "mat-icon",
                    "timer"
                }
                span {
                    "Stopwatch"
                }
            }
            input {
                r#type: "radio",
                name: "mode",
                value: "timer",
                id: "mode-timer",
                onchange: set_mode
            }
            label {
                r#for: "mode-timer",
                span {
                    class: "mat-icon",
                    "timelapse"
                }
                span {
                    "Timer"
                }
            }
        }
        div {
            id: "stopwatch-timer",
            StopwatchTimer {
                mode: mode
            }
        }
    }
}

#[component]
fn StopwatchTimer(mode: Signal<TimerMode>) -> Element {
    let mut initial_dur = use_signal(|| mode.read().default_dur());
    let mut dur = use_signal(|| *initial_dur.peek());
    let mut input_digits = use_signal(|| mode.read().default_digits());
    let mut runner: Signal<Option<Task>> = use_signal(|| None);
    let mut alarm: Signal<Option<Task>> = use_signal(|| None);

    // A lot of closures are requried because they all need to capture values

    let mut stop_alarm = move || {
        let mut alarm_write = alarm.write();
        if let Some(val) = *alarm_write {
            val.cancel();
            *alarm_write = None;
        }
    };

    let mut start_alarm = move || {
        alarm.set(Some(spawn(async move {
            let sl = Soloud::default().unwrap();
            let mut wav = audio::Wav::default();
            wav.load_mem(include_bytes!("../assets/alarm.oga")).unwrap();

            sl.play(&wav);
            let mut interval = time::interval(Duration::from_millis(100));
            while sl.voice_count() > 0 {
                interval.tick().await;
            }
            stop_alarm();
        })));
    };

    let mut reset = move || {
        dur.set(*initial_dur.peek());
        let mut runner_write = runner.write();
        if let Some(val) = *runner_write {
            val.cancel();
            *runner_write = None;
        }
    };

    let mut start_pause = move || {
        let mut runner_write = runner.write();
        if let Some(val) = *runner_write {
            if val.paused() {
                val.resume();
            } else {
                val.pause();
            }
        } else {
            *runner_write = Some(spawn(async move {
                let mut interval = time::interval(Duration::from_secs(1));
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                interval.tick().await;
                loop {
                    interval.tick().await;
                    match *mode.peek() {
                        TimerMode::Timer => {
                            dur -= Duration::from_secs(1);
                            if dur.peek().as_secs() == 0 {
                                if alarm.peek().is_none() {
                                    start_alarm();
                                }
                                reset();
                            }
                        },
                        TimerMode::Stopwatch => {
                            dur += Duration::from_secs(1);
                        },
                    };
                }
            }));
        }
    };

    let update_input = move |event: Event<KeyboardData>| {
        match event.key() {
            Key::Character(character) => {
                let first = character.chars().next();
                if first.is_some_and(|c| c.is_ascii_digit()) {
                    let digit = first.unwrap().to_digit(10).unwrap();
                    let mut digits_write = input_digits.write();

                    // In the event of an overflow, just keep the old value
                    *digits_write = digits_write.checked_mul(10)
                        .and_then(|d| d.checked_add(digit as u64))
                        .unwrap_or(*digits_write);

                    let digits_dur = dur_from_str(
                        &format_digits(*digits_write)
                    ).unwrap();

                    initial_dur.set(digits_dur);
                    dur.set(digits_dur);
                }
            },
            Key::Backspace => {
                input_digits /= 10;
                let digits_dur = dur_from_str(
                    &format_digits(*input_digits.peek())
                ).unwrap();

                initial_dur.set(digits_dur);
                dur.set(digits_dur);
            },
            Key::Enter => start_pause(),
            _ => ()
        }
    };

    use_effect(move || {
        // When mode changes, reset all values
        initial_dur.set(mode.read().default_dur());
        input_digits.set(mode.read().default_digits());
        stop_alarm();
        reset();
    });

    rsx! {
        div {
            id: "main-display",
            class: "centered",
            div {
                autofocus: true,
                id: "time-display",
                role: "textbox",
                tabindex: 0,
                onkeydown: update_input,
                oninput: move |_| reset(),
                span {{ format_dur(dur()) }}
            }
        }
        div {
            class: "centered",
            button {
                class: "mat-icon",
                onclick: move |_| start_pause(),
                if runner().is_none_or(|r| r.paused()) {
                    "play_arrow"
                } else {
                    "pause"
                }
            }
            button {
                class: "mat-icon",
                onclick: move |_| reset(),
                "replay"
            }
        }
    }
}

fn format_dur(dur: Duration) -> String {
    let mut secs = dur.as_secs();
    if secs / 60 >= 1 {
        let mut mins = secs / 60;
        secs %= 60;
        if mins / 60 >= 1 {
            let hours = mins / 60;
            mins %= 60;
            format!("{}:{:0>2}:{:0>2}", hours, mins, secs)
        } else {
            format!("{}:{:0>2}", mins, secs)
        }
    } else {
        format!("{}", secs)
    }
}

fn format_digits(digits: u64) -> String {
    let mut display = digits.to_string();
    if display.len() > 2 {
        display.insert(display.len() - 2, ':');
    }
    if display.len() > 5 {
        display.insert(display.len() - 5, ':');
    }
    display
}

fn dur_from_str(input: &str) -> Option<Duration> {
    let mut split = input.split(':').collect::<Vec<_>>();
    if split.len() <= 3 {
        let mut secs = split.pop()?.parse::<u64>().ok()?;
        if !split.is_empty() {
            let mut mins = split.pop()?.parse::<u64>().ok()?;
            if !split.is_empty() {
                let hours = split.pop()?.parse::<u64>().ok()?;
                mins += hours * 60;
            }
            secs += mins * 60;
        }
        Some(Duration::from_secs(secs))
    } else {
        None
    }
}