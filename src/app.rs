use dioxus::prelude::*;

use crate::timer_mode::TimerMode;
use crate::stopwatch_timer::StopwatchTimer;
use crate::args::Args;

#[component]
pub fn App() -> Element {
    let context = use_context::<Args>();

    let mut mode = use_signal(|| context.mode.unwrap_or(TimerMode::Stopwatch));

    let set_mode = move |event: Event<FormData>| {
        mode.set(event.value().parse().unwrap());
    };

    rsx! {
        style {{ include_str!("../assets/main.css") }}
        if context.mode.is_none() {
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
        }
        div {
            id: "stopwatch-timer",
            StopwatchTimer {
                mode: mode,
                def_dur: context.duration,
                start: context.start
            }
        }
    }
}