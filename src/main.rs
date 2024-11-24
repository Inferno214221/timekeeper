use tokio::time::{self, Duration, MissedTickBehavior};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
    // TODO: add cli args
}

enum TimerMode {
    Timer,
    Stopwatch,
    // Alarm
}

#[component]
fn App() -> Element {
    let mode = use_signal(|| TimerMode::Stopwatch);

    rsx! {
        style { {include_str!("../assets/main.css")} }
        link { rel: "stylesheet", href: "main.css" }
        match *mode.read() {
            TimerMode::Timer => None,
            TimerMode::Stopwatch => Stopwatch()
        }
    }
}

#[component]
fn Stopwatch() -> Element {
    let mut initial_dur = use_signal(|| Duration::ZERO);
    let mut dur = use_signal(|| *initial_dur.peek());
    let mut input_digits = use_signal(|| 0_u64);
    // let mut display_dur = use_signal(|| format_dur(dur()));
    let mut runner: Signal<Option<Task>> = use_signal(|| None);

    let on_start_pause = move |_| {
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
                    dur += Duration::from_secs(1);
                    // display_dur.set(format_dur(*dur.peek()));
                }
            }));
        }
    };

    let on_input_update = move |event: Event<KeyboardData>| {
        info!("{:?}", char::from_u32(event.key().legacy_charcode()));
        match event.key() {
            Key::Character(character) => {
                let first = character.chars().next();
                if first.is_some_and(|c| c.is_ascii_digit()) {
                    let digit = first.unwrap().to_digit(10).unwrap();
                    let mut digits_write = input_digits.write();
                    *digits_write = *digits_write * 10 + digit as u64;
                    // TODO: use checked mul
                    // display_dur.set(format_digits(*digits_write));
                    initial_dur.set(dur_from_str(&format_digits(*digits_write)).unwrap());
                    dur.set(*initial_dur.peek());
                }
            },
            Key::Backspace => {
                input_digits /= 10;
                // display_dur.set(format_digits(*input_digits.peek()));
                initial_dur.set(dur_from_str(&format_digits(*input_digits.peek())).unwrap());
                dur.set(*initial_dur.peek());
            },
            Key::Enter => todo!(),
            _ => ()
        }
    };

    rsx! {
        div {
            id: "main-display",
            class: "centered",
            input {
                autofocus: true,
                id: "time-display",
                r#type: "text",
                value: format_dur(dur()),
                onkeyup: on_input_update,
                oninput: move |_| {
                    if runner.peek().is_none() { return; }
                    runner.set(None);
                    dur.set(*initial_dur.peek());
                }
            }
        }
        // TODO: show state
        div {
            class: "centered",
            button {
                onclick: on_start_pause,
                // TODO: use icons
                "Resume/Pause"
            }
            button {
                onclick: move |_| {
                    if runner.peek().is_none() { return; }
                    runner.set(None);
                    dur.set(*initial_dur.peek());
                },
                "Reset"
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