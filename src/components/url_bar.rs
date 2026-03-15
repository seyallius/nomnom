//! url_bar.rs - Input controls, folder picker, archive, and download trigger.
//!
//! This component adapts its input field based on the active [`DownloadSource`]:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │  [Paste URL…                       ] [📁 Folder] [▶ Download]│
//! │  or                                                          │
//! │  [📄 batch.txt                     ] [📁 Pick ] [▶ Download] │
//! │  ──────────────────────────────────────────────────────────  │
//! │  ⏺ Archive: [downloaded.txt ] [📁] (optional)               │
//! │  ──────────────────────────────────────────────────────────  │
//! │  📂 Save to: /home/user/Downloads                           │
//! │  $ yt-dlp -f "..." -o "..." "https://..."                   │
//! └──────────────────────────────────────────────────────────────┘
//! ```

use crate::core::{
    download_mode::{DownloadSource, DownloadType, Quality},
    flags::Flag,
    runner::{self, ChildHandle, DownloadRequest},
};
use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`UrlBar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct UrlBarProps {
    /// Current download type (needed to build [`DownloadRequest`]).
    pub download_type: Signal<DownloadType>,
    /// Current download source (drives URL vs. batch file input).
    pub download_source: Signal<DownloadSource>,
    /// Current video quality (needed to build [`DownloadRequest`]).
    pub quality: Signal<Quality>,
    /// The URL the user typed or pasted.
    pub url: Signal<String>,
    /// Path to the batch text file (used when source is Batch).
    pub batch_file: Signal<String>,
    /// Path to the yt-dlp download archive file (optional).
    pub archive_file: Signal<String>,
    /// Output directory for downloaded files.
    pub output_dir: Signal<String>,
    /// Memoised command preview string (built in parent).
    pub built_command: ReadSignal<String>,
    /// Currently active flags from the flag panel.
    pub active_flags: Signal<Vec<Flag>>,
    /// Log output signal shared with the output panel.
    pub log_lines: Signal<Vec<String>>,
    /// Whether a download is currently in progress.
    pub is_running: Signal<bool>,
    /// Shared child process handle for cancellation.
    pub child_handle: Signal<ChildHandle>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders the main input area with adaptive input, folder picker, and download trigger.
///
/// # Behaviour
///
/// - When `download_source` is `Batch`, shows a file-picker button for the batch `.txt`.
/// - Otherwise shows a plain URL text input.
/// - Archive file row is always visible — leave blank to skip.
/// - Download button is disabled while `is_running`.
/// - Stop button replaces Download button during an active download.
#[component]
pub fn UrlBar(props: UrlBarProps) -> Element {
    let mut url = props.url;
    let batch_file = props.batch_file;
    let mut archive_file = props.archive_file;
    let output_dir = props.output_dir;
    let built_command = props.built_command;
    let active_flags = props.active_flags;
    let log_lines = props.log_lines;
    let is_running = props.is_running;
    let child_handle = props.child_handle;
    let download_type = props.download_type;
    let download_source = props.download_source;
    let quality = props.quality;

    let is_batch = *download_source.read() == DownloadSource::Batch;

    // ── Handlers ────────────────────────────────────────────────────────────

    let on_pick_batch = move |_| {
        let mut batch_file = batch_file;
        async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title("Choose batch URL file")
                .add_filter("Text files", &["txt"])
                .pick_file()
                .await
            {
                batch_file.set(file.path().to_string_lossy().to_string());
            }
        }
    };

    let on_pick_archive = move |_| {
        let mut archive_file = archive_file;
        async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title("Choose download archive file")
                .add_filter("Text files", &["txt"])
                .pick_file()
                .await
            {
                archive_file.set(file.path().to_string_lossy().to_string());
            }
        }
    };

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

    let on_download = move |_| {
        let req = DownloadRequest {
            url: url.read().clone(),
            batch_file: batch_file.read().clone(),
            archive_file: archive_file.read().clone(),
            download_type: download_type.read().clone(),
            download_source: download_source.read().clone(),
            quality: quality.read().clone(),
            output_dir: output_dir.read().clone(),
            extra_flags: active_flags.read().clone(),
        };

        let log = log_lines;
        let running = is_running;
        let handle = child_handle.read().clone();

        spawn(async move {
            runner::run_download(req, log, running, handle).await;
        });
    };

    let on_stop = move |_| {
        let handle = child_handle.read().clone();
        runner::cancel_download(&handle, log_lines, is_running);
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            // ── Primary input row ─────────────────────────────────────────
            div {
                style: "display: flex; gap: 8px; align-items: center;",

                // URL text input OR batch file display + picker
                if is_batch {
                    // Batch file: show path (read-only display) + pick button
                    div {
                        style: "
                            flex: 1;
                            display: flex;
                            align-items: center;
                            gap: 8px;
                            padding: 10px 14px;
                            background: #1e1e2e;
                            border: 1px solid #3a3a5a;
                            border-radius: 6px;
                        ",
                        span { style: "font-size: 14px;", "📄" }
                        span {
                            style: "
                                flex: 1;
                                color: #e0e0e0;
                                font-size: 13px;
                                font-family: monospace;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                white-space: nowrap;
                            ",
                            if batch_file.read().is_empty() {
                                span { style: "color: #444;", "Pick a .txt file with one URL per line…" }
                            } else {
                                "{batch_file}"
                            }
                        }
                        button {
                            style: "
                                padding: 4px 10px;
                                background: #2a2a4a;
                                color: #a0a0c0;
                                border: 1px solid #3a3a6a;
                                border-radius: 4px;
                                cursor: pointer;
                                font-size: 12px;
                                white-space: nowrap;
                            ",
                            onclick: on_pick_batch,
                            "Pick file"
                        }
                    }
                } else {
                    // Single / Playlist / Channel: URL text input
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
                        placeholder: "Paste a YouTube / any URL here…",
                        value: "{url}",
                        oninput: move |e| url.set(e.value()),
                    }
                }

                button {
                    style: btn_style(BtnKind::Folder),
                    onclick: on_pick_folder,
                    "📁"
                }

                if *is_running.read() {
                    button {
                        style: btn_style(BtnKind::Stop),
                        onclick: on_stop,
                        "⏹ Stop"
                    }
                } else {
                    button {
                        style: btn_style(BtnKind::Download),
                        onclick: on_download,
                        "▶ Download"
                    }
                }
            }

            // ── Archive file row ──────────────────────────────────────────
            div {
                style: "display: flex; align-items: center; gap: 8px;",

                span { style: "font-size: 11px; color: #444; min-width: 70px;", "⏺ Archive" }

                input {
                    style: "
                        flex: 1;
                        padding: 6px 10px;
                        background: #0d0d1a;
                        border: 1px solid #1e1e3a;
                        border-radius: 4px;
                        color: #888;
                        font-size: 11px;
                        font-family: monospace;
                        outline: none;
                    ",
                    r#type: "text",
                    placeholder: "Path to download archive .txt (optional)",
                    value: "{archive_file}",
                    oninput: move |e| archive_file.set(e.value()),
                }

                button {
                    style: "
                        padding: 5px 8px;
                        background: #1a1a2e;
                        color: #555;
                        border: 1px solid #2a2a4a;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 11px;
                    ",
                    onclick: on_pick_archive,
                    "📁"
                }
            }

            // ── Output folder display ────────────────────────────────────
            div {
                style: "font-size: 11px; color: #555; padding: 0 2px;",
                "📂 "
                span { style: "color: #888;", "{output_dir}" }
            }

            // ── Command preview ──────────────────────────────────────────
            div {
                style: "
                    background: #060612;
                    border: 1px solid #1e1e3a;
                    border-radius: 6px;
                    padding: 10px 14px;
                    font-size: 11px;
                    color: #555;
                    word-break: break-all;
                    font-family: monospace;
                    line-height: 1.6;
                ",
                span { style: "color: #6c63ff;", "$ " }
                span { style: "color: #7878aa;", "{built_command}" }
            }
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Button kind variants for styling.
enum BtnKind {
    Download,
    Stop,
    Folder,
}

/// Returns inline CSS for a button based on its kind.
fn btn_style(kind: BtnKind) -> &'static str {
    match kind {
        BtnKind::Download => {
            "
            padding: 10px 18px;
            background: #6c63ff;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: bold;
            white-space: nowrap;
        "
        }
        BtnKind::Stop => {
            "
            padding: 10px 18px;
            background: #ff4444;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: bold;
            white-space: nowrap;
        "
        }
        BtnKind::Folder => {
            "
            padding: 10px 12px;
            background: #1e1e3a;
            color: #888;
            border: 1px solid #2a2a4a;
            border-radius: 6px;
            cursor: pointer;
            font-size: 15px;
        "
        }
    }
}
