//! main.rs - Entry point for the nomnom desktop application.
//!
//! This file is responsible for:
//! - Configuring the Dioxus desktop window (title, size, resizability)
//! - Launching the application with the root [`app`] component
//!
//! The application is built using the Dioxus framework for Rust,
//! providing a native desktop GUI with async capabilities.

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
