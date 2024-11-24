use tokio::time::{self, Duration, MissedTickBehavior};

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

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
        Stopwatch {}
    }
}

#[component]
fn Stopwatch() -> Element {
    let mut initial_dur = use_signal(|| Duration::ZERO);
    let mut dur = use_signal(|| *initial_dur.peek());
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
                }
            }));
        }
    };

    let on_reset = move |_| {
        let mut runner_write = runner.write();
        if !runner_write.is_none() {
            *runner_write = None;
            *dur.write() = *initial_dur.peek();
        }
    };

    rsx! {
        div {
            class: "centered",
            input {
                r#type: "text",
                value: format_dur(dur())
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
                onclick: on_reset,
                "Reset"
            }
        }
    }
}

fn format_dur(dur: Duration) -> String {
    let mut secs = dur.as_secs();
    if secs / 60 > 1 {
        let mut mins = secs / 60;
        secs %= 60;
        if mins / 60 > 1 {
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