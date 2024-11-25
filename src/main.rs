use dioxus::prelude::*;
use dioxus::desktop::{tao, Config};

mod app;
mod stopwatch_timer;
mod timer_mode;
mod utils;

use app::App;

// TODO: add cli args

fn main() {
    // dioxus_logger::init(Level::INFO).expect("Logger initialisation failed");
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