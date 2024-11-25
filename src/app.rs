use dioxus::prelude::*;
use dioxus::desktop;

use super::timer_mode::TimerMode;
use super::stopwatch_timer::StopwatchTimer;

#[component]
pub fn App() -> Element {
    let mut mode = use_signal(|| TimerMode::Stopwatch);
    // ? is it even worth having this seperate?

    use_effect(move || desktop::window().set_title(mode.read().win_title()));

    let set_mode = move |event: Event<FormData>| {
        mode.set(event.value().parse().unwrap());
    };

    rsx! {
        style {{ include_str!("./main.css") }}
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