//! components/mod.rs - UI component modules for the application.
//!
//! Each component is a Dioxus [`component`] function receiving props
//! and rendering part of the UI. All shared state flows through props
//! as [`Signal<T>`] values — no global context is used.
//!
//! # Components
//!
//! | Module            | Component        | Purpose                                    |
//! |-------------------|------------------|--------------------------------------------|
//! | [`flag_panel`]    | `FlagPanel`      | Toggle buttons for individual yt-dlp flags |
//! | [`mode_selector`] | `ModeSelector`   | Type / source / quality pill selectors     |
//! | [`output_log`]    | `OutputLog`      | Scrollable color-coded log output         |
//! | [`preset_panel`]  | `PresetPanel`    | Clickable preset cards (video + audio)     |
//! | [`terminal_panel`]| `TerminalPanel`  | Raw command input for power users          |
//! | [`url_bar`]       | `UrlBar`         | URL/batch input, folder picker, download   |
//!
//! # Props Pattern
//!
//! All components follow the same pattern:
//! 1. Define a `Props` struct with `#[derive(Props, Clone, PartialEq)]`
//! 2. Accept signals from the parent as fields
//! 3. Read/write signals to update shared state

pub mod flag_panel;
pub mod mode_selector;
pub mod output_log;
pub mod preset_panel;
pub mod terminal_panel;
pub mod url_bar;
