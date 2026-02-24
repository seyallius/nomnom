//! components/mod.rs - UI component modules for the application.
//!
//! This module re-exports all UI components. Each component is a Dioxus
//! [`component`] function that receives props and renders part of the UI.
//!
//! # Components
//!
//! | Module | Component | Purpose         |
//! |--------|-----------|-----------------|
//! | [`flag_panel`]     | `FlagPanel`     | Toggle buttons for individual flags       |
//! | [`output_log`]     | `OutputLog`     | Scrollable log output display             |
//! | [`preset_panel`]   | `PresetPanel`   | Clickable preset cards                    |
//! | [`terminal_panel`] | `TerminalPanel` | Raw command input field                   |
//! | [`url_bar`]        | `UrlBar`        | URL input, folder picker, download button |
//!
//! # Props Pattern
//!
//! All components follow the same pattern:
//! 1. Define a `Props` struct with `#[derive(Props, Clone, PartialEq)]`
//! 2. Accept signals from the parent as fields
//! 3. Read/write signals to update shared state

pub mod flag_panel;
pub mod output_log;
pub mod preset_panel;
pub mod terminal_panel;
pub mod url_bar;
