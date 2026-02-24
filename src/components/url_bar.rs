//! url_bar.rs - URL input, folder picker, command preview, and download button.
//!
//! This is the primary input component where users:
//! - Paste video/playlist URLs
//! - Select download folder
//! - Preview the command that will run
//! - Trigger the download
//!
//! # Visual Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ [Paste URL here...          ] [📁 Folder] [▶ Download]     │
//! │ 📂 Save to: /home/user/Downloads                           │
//! │ $ yt-dlp --add-metadata -o "..." "https://..."             │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use crate::core::{flags::Flag, runner, runner::ChildHandle};
use dioxus::prelude::*;
// -------------------------------------------- Types --------------------------------------------

/// Props for the [`UrlBar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct UrlBarProps {
    /// Signal holding the URL to download.
    pub url: Signal<String>,
    /// Signal holding the output directory path.
    pub output_dir: Signal<String>,
    /// Memoized command preview string.
    pub built_command: ReadSignal<String>,
    /// Signal holding active flags for the download.
    pub active_flags: Signal<Vec<Flag>>,
    /// Signal for log output.
    pub log_lines: Signal<Vec<String>>,
    /// Signal tracking whether a download is in progress.
    pub is_running: Signal<bool>,
    /// Shared child process handle — used to kill the process on stop.
    pub child_handle: Signal<ChildHandle>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders the URL input bar with folder picker and download controls.
///
/// This component is the main entry point for starting downloads.
/// It displays the URL input, folder selection, command preview,
/// and the download button.
///
/// # Arguments
///
/// * `props` - Contains all signals needed for URL input and download control.
///
/// # Behavior
///
/// - URL input: Text field for pasting video URLs
/// - Folder button: Opens native OS folder picker dialog
/// - Download button: Spawns yt-dlp via [`runner::run_download`]
/// - Command preview: Shows the exact command that will run
///
/// # Validation
///
/// - Empty URL shows a warning in the log
/// - Download button is disabled while a download is running
///
/// # Example
///
/// ```rust,ignore
/// let url = use_signal(String::new);
/// let output_dir = use_signal(|| "/downloads".to_string());
/// let built_command = use_memo(move || build_cmd(&url.read()));
/// let active_flags = use_signal(Vec::<Flag>::new);
/// let log_lines = use_signal(Vec::<String>::new);
/// let is_running = use_signal(|| false);
///
/// rsx! {
///     UrlBar {
///         url,
///         output_dir,
///         built_command,
///         active_flags,
///         log_lines,
///         is_running,
///     }
/// }
/// ```
#[component]
pub fn UrlBar(props: UrlBarProps) -> Element {
    let mut url = props.url;
    let output_dir = props.output_dir;
    let built_command = props.built_command;
    let active_flags = props.active_flags;
    let log_lines = props.log_lines;
    let is_running = props.is_running;
    let child_handle = props.child_handle;

    // Handles folder picker button click.
    // Opens a native OS folder dialog and updates `output_dir` on selection.
    let on_pick_folder = move |_| {
        let mut output_dir = output_dir;
        async move {
            if let Some(folder) = rfd::AsyncFileDialog::new()
                .set_title("Choose download folder")
                .pick_folder()
                .await
            {
                output_dir.set(folder.path().to_string_lossy().to_string());
            }
        }
    };

    // Handles download button click.
    // Validates URL, spawns download task, and updates log.
    let on_download = move |_| {
        let url_val = url.read().clone();
        let flags = active_flags.read().clone();
        let dir = output_dir.read().clone();
        let handle = child_handle.read().clone(); // clone the Arc, cheap

        if url_val.trim().is_empty() {
            log_lines
                .clone()
                .write()
                .push("⚠ Please enter a URL first.".to_string());
            return;
        }

        let log = log_lines;
        let running = is_running;

        spawn(async move {
            runner::run_download(url_val, flags, dir, log, running, handle).await;
        });
    };

    // Stop button: kills the child process via the shared handle
    let on_stop = move |_| {
        let handle = child_handle.read().clone();
        runner::cancel_download(&handle, log_lines, is_running);
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            // ── URL row ───────────────────────────────────────
            div {
                style: "display: flex; gap: 8px; align-items: center;",

                input {
                    style: "
                        flex: 1;
                        padding: 10px 14px;
                        background: #1e1e2e;
                        border: 1px solid #3a3a5a;
                        border-radius: 6px;
                        color: #e0e0e0;
                        font-size: 14px;
                        outline: none;
                    ",
                    r#type: "text",
                    placeholder: "Paste YouTube / any URL here…",
                    value: "{url}",
                    oninput: move |e| url.set(e.value()),
                }

                button {
                    style: button_style(ButtonKind::Folder, false),
                    onclick: on_pick_folder,
                    "📁 Folder"
                }

                // ── Download OR Stop button (mutually exclusive) ──────────
                if *is_running.read() {
                    button {
                        style: button_style(ButtonKind::Stop, false),
                        onclick: on_stop,
                        "⏹ Stop"
                    }
                } else {
                    button {
                        style: button_style(ButtonKind::Download, false),
                        onclick: on_download,
                        "▶ Download"
                    }
                }

                // button {
                //     style: button_style(*is_running.read()),
                //     disabled: *is_running.read(),
                //     onclick: on_download,
                //     if *is_running.read() { "⏳ Downloading…" } else { "▶ Download" }
                // }
            }

            // ── Output folder display ─────────────────────────
            div {
                style: "
                    font-size: 12px;
                    color: #888;
                    padding: 0 4px;
                ",
                "📂 Save to: "
                span { style: "color: #a0a0c0;", "{output_dir}" }
            }

            // ── Command preview ───────────────────────────────
            div {
                style: "
                    background: #0d0d1a;
                    border: 1px solid #2a2a4a;
                    border-radius: 6px;
                    padding: 10px 14px;
                    font-size: 12px;
                    color: #7878aa;
                    word-break: break-all;
                    font-family: monospace;
                ",
                span { style: "color: #6c63ff;", "$ " }
                "{built_command}"
            }
        }
    }
}

// -------------------------------------------- Internal Helpers --------------------------------------------

/// Variants for the button style function.
enum ButtonKind {
    Download,
    Stop,
    Folder,
}

/// Returns inline CSS for a button based on its kind.
fn button_style(kind: ButtonKind, _disabled: bool) -> &'static str {
    match kind {
        ButtonKind::Download => {
            "
            padding: 10px 18px;
            background: #6c63ff;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: bold;
        "
        }
        ButtonKind::Stop => {
            "
            padding: 10px 18px;
            background: #ff4444;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: bold;
        "
        }
        ButtonKind::Folder => {
            "
            padding: 10px 18px;
            background: #2a2a4a;
            color: #e0e0e0;
            border: 1px solid #3a3a6a;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
        "
        }
    }
}
