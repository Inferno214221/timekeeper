#![feature(let_chains)]
#![feature(iterator_try_reduce)]

use dioxus::prelude::*;
use dioxus::desktop::{tao::{dpi::LogicalSize, window::WindowBuilder}, Config};

mod args;
mod app;
mod stopwatch_timer;
mod timer_mode;
mod utils;
mod digit;

use args::get_args;
use app::App;

fn main() {
    // dioxus_logger::init(dioxus_logger::tracing::Level::INFO)
    //     .expect("Logger initialisation failed");

    const MIN_SIZE: LogicalSize<u32> = LogicalSize::new(200, 140);
    
    let args = get_args();

    let window = WindowBuilder::new()
        .with_title("TimeKeeper")
        .with_resizable(true)
        .with_inner_size(MIN_SIZE)
        .with_min_inner_size(MIN_SIZE)
        .with_always_on_top(args.always_on_top)
        .with_visible_on_all_workspaces(args.follow_workspace);
    // TODO: timer pauses when minimised or on another desktop - is that an XFCE issue?

    let config = Config::new()
        .with_window(window)
        .with_menu(None)
        .with_disable_context_menu(true);

    LaunchBuilder::new()
        .with_cfg(config)
        .with_context(args)
        .launch(App);
}