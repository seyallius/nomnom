//! app.rs - Root application component and global state management.
//!
//! This module owns all reactive state and wires child components together.
//!
//! # Architecture
//!
//! All state lives here and flows down through props. No global context is used.
//!
//! # Data Flow
//!
//! ```text
//! App (owns all Signals)
//! ├── PresetPanel    → reads/writes active_preset, active_flags,
//! │                    download_type, download_source, quality
//! ├── ModeSelector   → reads/writes download_type, download_source, quality,
//! │                    active_preset
//! ├── UrlBar         → reads/writes url, batch_file, archive_file, output_dir,
//! │                    reads built_command, active_flags, log_lines, is_running
//! ├── TerminalPanel  → reads/writes log_lines, is_running
//! ├── FlagPanel      → reads/writes active_flags
//! └── OutputLog      → reads log_lines
//! ```
//!
//! # Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Header: "📥 nomnom"                                         │
//! ├──────────────┬──────────────────────────────────────────────┤
//! │ PresetPanel  │ ModeSelector (type / source / quality)       │
//! │ FlagPanel    │ UrlBar (input + archive + folder + button)   │
//! │              │ TerminalPanel (raw cmd)                      │
//! │              │ OutputLog (streaming log)                    │
//! └──────────────┴──────────────────────────────────────────────┘
//! ```

use crate::{
    components::{
        flag_panel::FlagPanel, mode_selector::ModeSelector, output_log::OutputLog,
        preset_panel::PresetPanel, terminal_panel::TerminalPanel, url_bar::UrlBar,
    },
    core::{
        download_mode::{DownloadSource, DownloadType, Quality},
        flags::Flag,
        presets::{default_preset, resolve_preset_flags, Preset},
        runner::{self, ChildHandle},
    },
};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

// -------------------------------------------- Public API --------------------------------------------

/// Root application component holding all shared reactive state.
///
/// # State Defaults
///
/// | Signal            | Default                              |
/// |-------------------|--------------------------------------|
/// | `download_type`   | `Video`                              |
/// | `download_source` | `Single`                             |
/// | `quality`         | `HD1080`                             |
/// | `url`             | empty                                |
/// | `batch_file`      | empty                                |
/// | `archive_file`    | empty                                |
/// | `output_dir`      | OS Downloads folder (or `.`)         |
/// | `active_flags`    | resolved from default preset         |
/// | `active_preset`   | `Some("single_video")`               |
/// | `log_lines`       | empty                                |
/// | `is_running`      | `false`                              |
#[component]
pub fn App() -> Element {
    // ── Download configuration state ─────────────────────────────────────

    // Whether to download video or extract audio.
    let download_type: Signal<DownloadType> = use_signal(DownloadType::default);

    // URL source + output folder organization strategy.
    let download_source: Signal<DownloadSource> = use_signal(DownloadSource::default);

    // Video resolution cap.
    let quality: Signal<Quality> = use_signal(Quality::default);

    // ── Input state ───────────────────────────────────────────────────────

    // The URL the user wants to download.
    let url = use_signal::<String>(String::new);

    // Path to a batch text file (one URL per line).
    let batch_file = use_signal::<String>(String::new);

    // Path to a yt-dlp download archive file (optional; empty = disabled).
    let archive_file = use_signal::<String>(String::new);

    // Output directory for all downloaded files.
    let output_dir = use_signal(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    });

    // ── Preset + flag state ───────────────────────────────────────────────

    let initial_preset = default_preset();
    let initial_flags = resolve_preset_flags(&initial_preset);

    // Currently active flags — populated by preset or manual toggle.
    let active_flags: Signal<Vec<Flag>> = use_signal(|| initial_flags);

    // Currently active preset. `None` = Custom mode.
    let active_preset: Signal<Option<Preset>> = use_signal(|| Some(initial_preset));

    // ── Runtime state ─────────────────────────────────────────────────────

    // Lines captured from yt-dlp stdout/stderr — streamed to the log panel.
    let log_lines: Signal<Vec<String>> = use_signal(Vec::new);

    // Whether a download is currently in progress.
    let is_running = use_signal(|| false);

    // Shared handle to the active child process for cancellation.
    let child_handle: Signal<ChildHandle> = use_signal(|| Arc::new(Mutex::new(None)));

    // ── Derived state ─────────────────────────────────────────────────────

    // Memoised command preview — recomputed whenever any input signal changes.
    let built_command = use_memo(move || {
        runner::build_command_string(&runner::DownloadRequest {
            url: url.read().clone(),
            batch_file: batch_file.read().clone(),
            archive_file: archive_file.read().clone(),
            download_type: download_type.read().clone(),
            download_source: download_source.read().clone(),
            quality: quality.read().clone(),
            output_dir: output_dir.read().clone(),
            extra_flags: active_flags.read().clone(),
        })
    });

    // ── Render ────────────────────────────────────────────────────────────

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background: #0b0b14;
                color: #e0e0e0;
                font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
                overflow: hidden;
            ",

            // ── Header ──────────────────────────────────────────────────
            div {
                style: "
                    padding: 12px 20px;
                    background: #0f0f1e;
                    border-bottom: 2px solid #6c63ff;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    flex-shrink: 0;
                ",
                span { style: "font-size: 20px;", "📥" }
                h1 {
                    style: "
                        margin: 0;
                        font-size: 16px;
                        color: #6c63ff;
                        letter-spacing: 2px;
                        font-weight: bold;
                    ",
                    "nomnom"
                }
                span {
                    style: "
                        font-size: 11px;
                        color: #444;
                        letter-spacing: 1px;
                        margin-left: 4px;
                    ",
                    "gib me URLs"
                }

                // Spacer
                div { style: "flex: 1;" }

                // Status indicator
                if *is_running.read() {
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 6px;
                            font-size: 11px;
                            color: #6c63ff;
                        ",
                        span { "⏳" }
                        "Downloading…"
                    }
                }
            }

            // ── Main body ────────────────────────────────────────────────
            div {
                style: "
                    display: flex;
                    flex: 1;
                    overflow: hidden;
                ",

                // ── Left sidebar ─────────────────────────────────────────
                div {
                    style: "
                        width: 280px;
                        min-width: 240px;
                        background: #0e0e1c;
                        border-right: 1px solid #1e1e36;
                        display: flex;
                        flex-direction: column;
                        overflow-y: auto;
                        padding: 14px 10px;
                        gap: 16px;
                    ",

                    PresetPanel {
                        active_preset,
                        active_flags,
                        download_type,
                        download_source,
                        quality,
                    }

                    // Divider
                    div {
                        style: "
                            height: 1px;
                            background: #1e1e36;
                            margin: 0 4px;
                        "
                    }

                    FlagPanel { active_flags }
                }

                // ── Right content area ────────────────────────────────────
                div {
                    style: "
                        flex: 1;
                        display: flex;
                        flex-direction: column;
                        overflow: hidden;
                        padding: 12px;
                        gap: 10px;
                    ",

                    ModeSelector {
                        download_type,
                        download_source,
                        quality,
                        active_preset,
                    }

                    UrlBar {
                        download_type,
                        download_source,
                        quality,
                        url,
                        batch_file,
                        archive_file,
                        output_dir,
                        built_command,
                        active_flags,
                        log_lines,
                        is_running,
                        child_handle,
                    }

                    TerminalPanel {
                        log_lines,
                        is_running,
                        child_handle,
                    }

                    OutputLog { log_lines }
                }
            }
        }
    }
}
