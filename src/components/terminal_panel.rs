//! terminal_panel.rs - Raw yt-dlp command input terminal

use crate::core::runner;
use dioxus::prelude::*;

// -------------------------------------------- Public Functions --------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TerminalPanelProps {
    pub log_lines: Signal<Vec<String>>,
    pub is_running: Signal<bool>,
}

/// Terminal-like input for running arbitrary yt-dlp commands
#[component]
pub fn TerminalPanel(props: TerminalPanelProps) -> Element {
    let mut raw_input = use_signal(|| String::from("yt-dlp "));
    let log_lines = props.log_lines;
    let is_running = props.is_running;

    let on_run = move |_| {
        let cmd = raw_input.read().clone();
        let log = log_lines.clone();
        let running = is_running.clone();

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
                            let log = log_lines.clone();
                            let running = is_running.clone();
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
