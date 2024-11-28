use dioxus::prelude::*;
use tokio::time::{self, Duration, MissedTickBehavior};
use soloud::*;

use crate::timer_mode::TimerMode;
use crate::utils::*;

#[component]
pub fn StopwatchTimer(mode: Signal<TimerMode>, def_dur: Option<Duration>, start: bool) -> Element {
    let mut handled_args = use_signal(|| false);
    let default_dur = move || {
        if !*handled_args.peek() && let Some(val) = def_dur {
            val
        } else {
            mode.read().default_dur()
        }
    };
    // TODO: maybe make a digits type alias and then impl From and Into Duration

    let mut initial_dur = use_signal(default_dur);
    let mut dur = use_signal(|| *initial_dur.peek());
    let mut input_digits = use_signal(|| digits_from_dur(default_dur()));
    let mut runner: Signal<Option<Task>> = use_signal(|| None);
    let mut alarm: Signal<Option<Task>> = use_signal(|| None);
    
    // A lot of closures are required because they all need to capture values

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
            wav.load_mem(include_bytes!("./alarm.oga")).unwrap();

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
        if !*handled_args.peek() {
            handled_args.set(true);
            return;
        }
        // When mode changes, reset all values
        initial_dur.set(default_dur());
        input_digits.set(digits_from_dur(default_dur()));
        stop_alarm();
        reset();
    });

    if !*handled_args.peek() && start {
        start_pause();
    }

    rsx! {
        div {
            id: "main-display",
            class: "centred",
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
            class: "centred",
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