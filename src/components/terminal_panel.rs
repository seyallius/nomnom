//! terminal_panel.rs - Raw command input for advanced users.
//!
//! This component provides a terminal-like input field where power users
//! can type and execute arbitrary yt-dlp commands directly.
//!
//! # Use Cases
//!
//! - Testing new flags not yet in the GUI
//! - Running yt-dlp with complex filter expressions
//! - Executing other CLI tools
//!
//! # Behavior
//!
//! - Input is prefixed with `$ ` to simulate a terminal
//! - Pressing Enter or clicking "Run" executes the command
//! - Output streams to the shared log panel

use crate::core::runner;
use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`TerminalPanel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TerminalPanelProps {
    /// Signal for log output (shared with main download).
    pub log_lines: Signal<Vec<String>>,
    /// Signal tracking whether a command is running.
    pub is_running: Signal<bool>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders a terminal-style command input field.
///
/// Provides a text input pre-filled with `"yt-dlp "` where users can
/// type arbitrary commands. Commands are executed via [`runner::run_raw_command`].
///
/// # Arguments
///
/// * `props` - Contains `log_lines` and `is_running` signals.
///
/// # Interactivity
///
/// - Text input with `$ ` prefix decoration
/// - "Run" button to execute command
/// - Enter key also triggers execution
/// - Disabled while a command is running
///
/// # Example
///
/// ```rust,ignore
/// let log_lines = use_signal(Vec::<String>::new);
/// let is_running = use_signal(|| false);
///
/// rsx! {
///     TerminalPanel { log_lines, is_running }
/// }
/// ```
#[component]
pub fn TerminalPanel(props: TerminalPanelProps) -> Element {
    let mut raw_input = use_signal(|| String::from("yt-dlp "));
    let log_lines = props.log_lines;
    let is_running = props.is_running;

    let on_run = move |_| {
        let cmd = raw_input.read().clone();
        let log = log_lines;
        let running = is_running;

        spawn(async move {
            runner::run_raw_command(cmd, log, running).await;
        });
    };

    rsx! {
        div {
            style: "
                background: #0d0d1a;
                border: 1px solid #2a2a4a;
                border-radius: 6px;
                padding: 10px 12px;
                display: flex;
                flex-direction: column;
                gap: 6px;
            ",

            div {
                style: "font-size: 11px; color: #6c63ff; letter-spacing: 1px;",
                "⌨  TERMINAL"
            }

            div {
                style: "display: flex; gap: 8px; align-items: center;",

                span { style: "color: #6c63ff; font-size: 14px;", "$ " }

                input {
                    style: "
                        flex: 1;
                        background: transparent;
                        border: none;
                        color: #e0e0e0;
                        font-family: monospace;
                        font-size: 13px;
                        outline: none;
                    ",
                    r#type: "text",
                    value: "{raw_input}",
                    oninput: move |e| raw_input.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            let cmd = raw_input.read().clone();
                            let log = log_lines;
                            let running = is_running;
                            spawn(async move {
                                runner::run_raw_command(cmd, log, running).await;
                            });
                        }
                    },
                }

                button {
                    style: "
                        padding: 6px 14px;
                        background: #2a2a4a;
                        color: #e0e0e0;
                        border: 1px solid #3a3a6a;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 12px;
                    ",
                    disabled: *is_running.read(),
                    onclick: on_run,
                    "Run"
                }
            }
        }
    }
}
