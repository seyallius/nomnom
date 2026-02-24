//! app.rs - Root application component and global state management.
//!
//! This module defines the top-level [`App`] component that:
//! - Owns all shared reactive state via Dioxus [`Signal`]s
//! - Orchestrates the layout (sidebar + main content area)
//! - Wires child components together through props
//!
//! # Architecture
//!
//! All application state lives here and flows down to child components.
//! Child components read/write state through props, never global context.
//!
//! # Data Flow
//!
//! ```text
//! App (owns all Signals)
//! ├── preset_panel   → reads/writes active_preset, active_flags
//! ├── flag_panel     → reads/writes active_flags
//! ├── url_bar        → reads/writes url, output_dir, log_lines, is_running
//! ├── terminal_panel → reads/writes log_lines, is_running
//! └── output_log     → reads log_lines
//! ```

use dioxus::prelude::*;

use crate::{
    components::{
        flag_panel::FlagPanel, output_log::OutputLog, preset_panel::PresetPanel,
        terminal_panel::TerminalPanel, url_bar::UrlBar,
    },
    core::{flags::Flag, presets::Preset},
};

// -------------------------------------------- Public API --------------------------------------------

/// Root application component holding all shared reactive state.
///
/// This is the top-level component that:
/// 1. Initializes all application state with sensible defaults
/// 2. Computes derived state (like the command preview)
/// 3. Renders the full application layout
///
/// # State Initialization
///
/// | State           | Default Value                |
/// |-------          |----------------------------- |
/// | `url`           | Empty string                 |
/// | `active_flags`  | Empty (populated by preset)  |
/// | `active_preset` | First preset ("Best Video")  |
/// | `output_dir`    | OS download directory or `.` |
/// | `log_lines`     | Empty vec                    |
/// | `is_running`    | `false`                      |
///
/// # Layout Structure
///
/// ```text
/// ┌────────────────────────────────────────────┐
/// │ Header: "📥 nomnom... gib me URLs!"         │
/// ├──────────────┬─────────────────────────────┤
/// │ PresetPanel  │ UrlBar                      │
/// │ FlagPanel    │ TerminalPanel               │
/// │ (sidebar)    │ OutputLog                   │
/// └──────────────┴─────────────────────────────┘
/// ```
#[component]
pub fn App() -> Element {
    // ── Initialize all reactive state ─────────────────────────────────────

    // The URL the user wants to download.
    // Updated by [`UrlBar`] and read by the command builder.
    let url = use_signal::<String>(String::new);

    // Currently active flags selected by the user.
    // Can be populated by selecting a preset or toggling individual flags.
    let active_flags: Signal<Vec<Flag>> = use_signal(Vec::new);

    // The currently active preset.
    // `None` indicates "Custom" mode where the user picks flags manually.
    let active_preset: Signal<Option<Preset>> =
        use_signal(|| Some(crate::core::presets::default_preset()));

    // Output folder where downloads will be saved.
    // Defaults to the OS download directory, falling back to current directory.
    let output_dir = use_signal(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    });

    // Log lines captured from yt-dlp stdout/stderr.
    // Displayed in real-time by [`OutputLog`].
    let log_lines: Signal<Vec<String>> = use_signal(Vec::new);

    // Flag indicating whether a download is currently in progress.
    // Used to disable the download button and show loading state.
    let is_running = use_signal(|| false);

    // Memoized command preview string.
    // Recomputes automatically when `url`, `active_flags`, or `output_dir` change.
    let built_command = use_memo(move || {
        crate::core::runner::build_command_string(
            &url.read(),
            &active_flags.read(),
            &output_dir.read(),
        )
    });

    // ── Render the application layout ─────────────────────────────────────

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background: #0f0f0f;
                color: #e0e0e0;
                font-family: 'JetBrains Mono', 'Fira Code', monospace;
                overflow: hidden;
            ",

            // ── Header ──────────────────────────────────
            div {
                style: "
                    padding: 14px 20px;
                    background: #1a1a2e;
                    border-bottom: 2px solid #6c63ff;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                ",
                span { style: "font-size: 22px;", "📥" }
                h1 {
                    style: "margin: 0; font-size: 18px; color: #6c63ff; letter-spacing: 1px;",
                    "nomnom... gib me URLs!"
                }
            }

            // ── Main body ────────────────────────────────
            div {
                style: "
                    display: flex;
                    flex: 1;
                    overflow: hidden;
                ",

                // Left sidebar: presets + flags
                div {
                    style: "
                        width: 300px;
                        min-width: 260px;
                        background: #111122;
                        border-right: 1px solid #2a2a4a;
                        display: flex;
                        flex-direction: column;
                        overflow-y: auto;
                        padding: 12px;
                        gap: 16px;
                    ",
                    PresetPanel {
                        active_preset,
                        active_flags,
                    }
                    FlagPanel {
                        active_flags,
                    }
                }

                // Right area: url + command preview + terminal + log
                div {
                    style: "
                        flex: 1;
                        display: flex;
                        flex-direction: column;
                        overflow: hidden;
                        padding: 12px;
                        gap: 12px;
                    ",

                    UrlBar {
                        url,
                        output_dir,
                        built_command,
                        active_flags,
                        log_lines,
                        is_running,
                    }

                    TerminalPanel {
                        log_lines,
                        is_running,
                    }

                    OutputLog { log_lines }
                }
            }
        }
    }
}
