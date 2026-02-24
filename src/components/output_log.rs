//! output_log.rs - Scrollable terminal-style log output panel.
//!
//! This component displays the streaming output from yt-dlp in a
//! scrollable, color-coded terminal-style panel.
//!
//! # Color Coding
//!
//! | Prefix/Content | Color | Meaning |
//! |----------------|-------|---------|
//! | `✔` | Green | Success |
//! | `✗` or `⚠` | Red | Error/Warning |
//! | `▶` or `$` | Purple | Command start |
//! | `[download]` | Cyan | Download progress |
//! | `[info]` | Orange | Information |
//! | Other | Gray | General output |

use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`OutputLog`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OutputLogProps {
    /// Signal containing all log lines to display.
    /// Updated by [`runner::run_download`](crate::core::runner::run_download).
    pub log_lines: Signal<Vec<String>>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders a scrollable log panel with color-coded output.
///
/// Each line from `log_lines` is rendered with syntax highlighting
/// based on its content. The panel auto-scrolls as new lines arrive.
///
/// # Arguments
///
/// * `props` - Contains `log_lines` signal for reading output.
///
/// # Empty State
///
/// When `log_lines` is empty, displays a placeholder message:
/// "Output will appear here…"
///
/// # Styling
///
/// - Dark background (`#050510`)
/// - Monospace font
/// - Color-coded lines via [`log_line_style`]
///
/// # Example
///
/// ```rust,ignore
/// let log_lines = use_signal(Vec::<String>::new);
///
/// rsx! {
///     OutputLog { log_lines }
/// }
/// ```
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

// -------------------------------------------- Internal Helpers --------------------------------------------

/// Returns CSS color style based on line content.
///
/// Analyzes the line content to determine appropriate color coding
/// for better visual parsing of yt-dlp output.
///
/// # Arguments
///
/// * `line` - The log line to analyze.
///
/// # Returns
///
/// A static CSS color string.
///
/// # Matching Rules
///
/// 1. Lines starting with `✔` → Green (success)
/// 2. Lines starting with `✗` or `⚠` → Red (error/warning)
/// 3. Lines starting with `▶` or `$` → Purple (command)
/// 4. Lines containing `[download]` → Cyan (download progress)
/// 5. Lines containing `[info]` → Orange (info)
/// 6. All other lines → Gray (default)
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
