//! main.rs - Entry point for yt-dlp-gui desktop application

use dioxus::prelude::*;

mod app;
mod components;
mod core;

fn main() {
    let cfg = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("yt-dlp GUI")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 750.0))
                .with_resizable(true),
        );

    LaunchBuilder::desktop().with_cfg(cfg).launch(app::App);
}
