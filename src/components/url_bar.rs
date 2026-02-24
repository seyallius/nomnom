//! url_bar.rs - URL input, folder picker, command preview, and download button

use crate::core::flags::Flag;
use crate::core::runner;
use dioxus::prelude::*;

// -------------------------------------------- Public Functions --------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct UrlBarProps {
    pub url: Signal<String>,
    pub output_dir: Signal<String>,
    pub built_command: ReadOnlySignal<String>,
    pub active_flags: Signal<Vec<Flag>>,
    pub log_lines: Signal<Vec<String>>,
    pub is_running: Signal<bool>,
}

/// URL input bar with folder picker, command preview, and download trigger
#[component]
pub fn UrlBar(props: UrlBarProps) -> Element {
    let mut url = props.url;
    let output_dir = props.output_dir;
    let built_command = props.built_command;
    let active_flags = props.active_flags;
    let log_lines = props.log_lines;
    let is_running = props.is_running;

    let on_pick_folder = move |_| {
        let mut output_dir = output_dir.clone();
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
        let url_val = url.read().clone();
        let flags = active_flags.read().clone();
        let dir = output_dir.read().clone();

        if url_val.trim().is_empty() {
            log_lines
                .clone()
                .write()
                .push("⚠ Please enter a URL first.".to_string());
            return;
        }

        let log = log_lines.clone();
        let running = is_running.clone();

        spawn(async move {
            runner::run_download(url_val, flags, dir, log, running).await;
        });
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
                    style: button_style(false),
                    onclick: on_pick_folder,
                    "📁 Folder"
                }

                button {
                    style: button_style(*is_running.read()),
                    disabled: *is_running.read(),
                    onclick: on_download,
                    if *is_running.read() { "⏳ Downloading…" } else { "▶ Download" }
                }
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

// -------------------------------------------- Private Helper Functions --------------------------------------------

fn button_style(disabled: bool) -> &'static str {
    if disabled {
        "
            padding: 10px 18px;
            background: #333;
            color: #666;
            border: none;
            border-radius: 6px;
            cursor: not-allowed;
            font-size: 13px;
        "
    } else {
        "
            padding: 10px 18px;
            background: #6c63ff;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
        "
    }
}
