//! output_log.rs - Scrollable terminal-style log output panel

use dioxus::prelude::*;

// -------------------------------------------- Public Functions --------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct OutputLogProps {
    pub log_lines: Signal<Vec<String>>,
}

/// Scrollable log panel showing yt-dlp stdout/stderr
#[component]
pub fn OutputLog(props: OutputLogProps) -> Element {
    let log_lines = props.log_lines;

    rsx! {
        div {
            style: "
                flex: 1;
                background: #050510;
                border: 1px solid #1a1a3a;
                border-radius: 6px;
                padding: 10px 14px;
                overflow-y: auto;
                font-family: monospace;
                font-size: 12px;
                line-height: 1.6;
                display: flex;
                flex-direction: column;
                gap: 1px;
                min-height: 180px;
            ",

            if log_lines.read().is_empty() {
                div {
                    style: "color: #333; font-style: italic;",
                    "Output will appear here…"
                }
            }

            for (i, line) in log_lines.read().iter().enumerate() {
                div {
                    key: "{i}",
                    style: log_line_style(line),
                    "{line}"
                }
            }
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Color-code log lines by content
fn log_line_style(line: &str) -> &'static str {
    if line.starts_with("✔") {
        "color: #50fa7b;"
    } else if line.starts_with("✗") || line.starts_with("⚠") {
        "color: #ff5555;"
    } else if line.starts_with("▶") || line.starts_with("$") {
        "color: #6c63ff;"
    } else if line.contains("[download]") {
        "color: #8be9fd;"
    } else if line.contains("[info]") {
        "color: #ffb86c;"
    } else {
        "color: #b0b0c0;"
    }
}
