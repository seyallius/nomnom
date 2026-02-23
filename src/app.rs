//! app.rs - Root application component and global state

use dioxus::prelude::*;

use crate::components::{
    flag_panel::FlagPanel, output_log::OutputLog, preset_panel::PresetPanel,
    terminal_panel::TerminalPanel, url_bar::UrlBar,
};
use crate::core::{flags::Flag, presets::Preset};

// -------------------------------------------- Public Functions --------------------------------------------

/// Root app component holding all shared state
#[component]
pub fn App() -> Element {
    // -- URL the user wants to download
    let url = use_signal(|| String::new());

    // -- Active flags selected by the user
    let active_flags: Signal<Vec<Flag>> = use_signal(Vec::new);

    // -- Currently active preset (None = custom)
    let active_preset: Signal<Option<Preset>> =
        use_signal(|| Some(crate::core::presets::default_preset()));

    // -- Output folder chosen by the user
    let output_dir = use_signal(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    });

    // -- Log lines from yt-dlp stdout/stderr
    let log_lines: Signal<Vec<String>> = use_signal(Vec::new);

    // -- Is a download currently running?
    let is_running = use_signal(|| false);

    // -- Built command preview
    let built_command = use_memo(move || {
        crate::core::runner::build_command_string(
            &url.read(),
            &active_flags.read(),
            &output_dir.read(),
        )
    });

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
                    "yt-dlp  GUI"
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
