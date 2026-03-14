//! runner.rs - yt-dlp subprocess management and output streaming.
//!
//! This module provides:
//! - [`DownloadRequest`]     — All parameters for a single download invocation
//! - [`build_command_string`] — Human-readable command preview for the UI
//! - [`run_download`]         — Async subprocess with live log streaming
//! - [`run_raw_command`]      — Raw command execution for the terminal panel
//! - [`cancel_download`]      — Kill the active child process
//!
//! # Architecture
//!
//! `run_download` accepts a [`DownloadRequest`] value that bundles the type,
//! source, quality, flags, paths, and URL. This replaces the old flat signature
//! and makes the caller's intent explicit.
//!
//! The internal [`build_exec_args`] helper translates the request into a plain
//! `Vec<String>` that is passed to the shell via `"$@"` expansion — each arg is
//! a separate process argument, so no shell quoting or injection is possible.
//!
//! # Error Handling
//!
//! All errors are captured and written to the log as user-friendly messages.
//! The application never panics from subprocess failures.

use crate::core::{
    download_mode::{DownloadSource, DownloadType, Quality},
    flags::Flag,
};
use dioxus::prelude::*;
use std::{
    process::Stdio,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

// -------------------------------------------- Types --------------------------------------------

/// Thread-safe slot holding the active child process, if any.
///
/// Shared between the async runner task and the UI stop button.
/// `Arc<Mutex<...>>` because it must cross async task + `onclick` handler boundaries.
pub type ChildHandle = Arc<Mutex<Option<Child>>>;

/// All parameters required to execute or preview a yt-dlp download.
///
/// This struct is the single contract between the UI layer and the runner.
/// Construct it in the download button handler and pass it to [`run_download`].
///
/// # Example
///
/// ```rust,ignore
/// let req = DownloadRequest {
///     url: "https://youtube.com/watch?v=abc".into(),
///     batch_file: String::new(),
///     archive_file: "/home/user/archive.txt".into(),
///     download_type: DownloadType::Video,
///     download_source: DownloadSource::Single,
///     quality: Quality::HD1080,
///     output_dir: "/home/user/Videos".into(),
///     extra_flags: active_flags.read().clone(),
/// };
/// runner::run_download(req, log_lines, is_running, child_handle).await;
/// ```
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// Video/playlist/channel URL. Empty when source is [`DownloadSource::Batch`].
    pub url: String,
    /// Path to a text file with one URL per line. Used for [`DownloadSource::Batch`].
    pub batch_file: String,
    /// Path to a yt-dlp download archive file (`--download-archive`). Empty = disabled.
    pub archive_file: String,
    /// Whether to download video or extract audio.
    pub download_type: DownloadType,
    /// URL source and output folder organisation strategy.
    pub download_source: DownloadSource,
    /// Video resolution cap. Ignored for [`DownloadType::Audio`].
    pub quality: Quality,
    /// Root directory for all output files.
    pub output_dir: String,
    /// Additional flags toggled by the user in the flag panel.
    pub extra_flags: Vec<Flag>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Builds a human-readable command string for the UI preview panel.
///
/// Produces the exact command a user would type in a terminal, with proper
/// quoting around arguments that contain spaces or special characters.
///
/// # Arguments
///
/// * `req` — The full download configuration.
///
/// # Returns
///
/// A formatted string like:
/// ```text
/// yt-dlp -f "bestvideo[height<=1080]+bestaudio/best[height<=1080]" \
///   -o "/home/user/Videos/%(title)s..." \
///   --embed-thumbnail --add-metadata "https://youtube.com/..."
/// ```
///
/// Returns a placeholder if no URL or batch file is specified.
pub fn build_command_string(req: &DownloadRequest) -> String {
    let has_input = !req.url.trim().is_empty() || !req.batch_file.trim().is_empty();
    if !has_input {
        return "yt-dlp [paste a URL or pick a batch file above]".to_string();
    }

    let exec_args = build_exec_args(req);

    // Quote any arg that contains a space, bracket, or percent sign for display.
    let display: Vec<String> = exec_args
        .iter()
        .map(|a| {
            if a.contains(' ') || a.contains('[') || a.contains('%') || a.contains('>') {
                format!("\"{}\"", a)
            } else {
                a.clone()
            }
        })
        .collect();

    format!("yt-dlp {}", display.join(" "))
}

/// Kills the active child process stored in the handle.
///
/// Called by the UI stop button. Locks the handle, takes the child,
/// and sends SIGKILL. Safe to call when no child is running (no-op).
///
/// # Arguments
///
/// * `handle`     — The shared child handle.
/// * `log_lines`  — Signal to write a cancellation message.
/// * `is_running` — Signal to reset running state.
pub fn cancel_download(
    handle: &ChildHandle,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
) {
    let mut lock = handle.lock().unwrap();
    if let Some(child) = lock.as_mut() {
        match child.start_kill() {
            Ok(_) => {
                log_lines
                    .write()
                    .push("⛔ Download cancelled by user.".to_string());
            }
            Err(e) => {
                log_lines
                    .write()
                    .push(format!("✗ Failed to kill process: {e}"));
            }
        }
        *lock = None;
    }
    is_running.set(false);
}

/// Spawns yt-dlp as a subprocess and streams output line-by-line to the log.
///
/// # Arguments
///
/// * `req`          — Full download configuration (type, source, quality, flags, paths).
/// * `log_lines`    — Signal to receive output lines (cleared before the download starts).
/// * `is_running`   — Signal tracking running state.
/// * `child_handle` — Shared slot to store the child process for cancellation.
pub async fn run_download(
    req: DownloadRequest,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
    child_handle: ChildHandle,
) {
    is_running.set(true);
    log_lines.write().clear();

    // Validate input before spawning.
    let has_input = !req.url.trim().is_empty()
        || (req.download_source == DownloadSource::Batch && !req.batch_file.trim().is_empty());

    if !has_input {
        log_lines
            .write()
            .push("⚠ Please enter a URL or pick a batch file first.".to_string());
        is_running.set(false);
        return;
    }

    log_lines.write().push("▶ Starting download…".to_string());
    log_lines
        .write()
        .push(format!("  {}", build_command_string(&req)));

    let args = build_exec_args(&req);
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    let result = Command::new(&shell)
        .arg("-i")
        .arg("-c")
        .arg("yt-dlp \"$@\"") // $@ safely expands each positional arg
        .arg("bash") // $0 — script name placeholder, not in $@
        .args(&args) // $1..n become $@
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match result {
        Err(e) => {
            log_lines
                .write()
                .push(format!("✗ Failed to spawn yt-dlp: {e}"));
            log_lines
                .write()
                .push("  Make sure yt-dlp is installed and in your PATH.".to_string());
            is_running.set(false);
        }
        Ok(mut child) => {
            // Take stdout/stderr BEFORE storing the child (storing moves it).
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            // Store child so the stop button can kill it.
            {
                let mut lock = child_handle.lock().unwrap();
                *lock = Some(child);
            }

            // Stream stdout.
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(line);
                }
            }

            // Stream stderr (prefixed with warning symbol).
            if let Some(stderr) = stderr {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(format!("⚠ {line}"));
                }
            }

            // Take child back to call .wait().
            let child_opt = {
                let mut lock = child_handle.lock().unwrap();
                lock.take()
            };

            if let Some(mut child) = child_opt {
                match child.wait().await {
                    Ok(status) if status.success() => {
                        log_lines.write().push("✔ Done!".to_string());
                    }
                    Ok(status) => {
                        // Only show error if we weren't cancelled.
                        if *is_running.read() {
                            log_lines
                                .write()
                                .push(format!("✗ yt-dlp exited with: {status}"));
                        }
                    }
                    Err(e) => {
                        log_lines.write().push(format!("✗ Wait error: {e}"));
                    }
                }
            }
            // child_opt == None means the user cancelled — message already logged.

            is_running.set(false);
        }
    }
}

/// Executes an arbitrary command string from the terminal panel.
///
/// Bypasses the GUI flag selection — useful for power users and testing.
///
/// # Arguments
///
/// * `raw`          — The raw command string to execute.
/// * `log_lines`    — Signal to receive output lines.
/// * `is_running`   — Signal tracking running state.
/// * `child_handle` — Shared slot to store the child process for cancellation.
pub async fn run_raw_command(
    raw: String,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
    child_handle: ChildHandle,
) {
    is_running.set(true);
    log_lines.write().push(format!("$ {raw}"));

    if raw.trim().is_empty() {
        is_running.set(false);
        return;
    }

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let result = Command::new(&shell)
        .args(["-i", "-c", &raw])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match result {
        Err(e) => {
            log_lines.write().push(format!("✗ {e}"));
            is_running.set(false);
        }
        Ok(mut child) => {
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            {
                let mut lock = child_handle.lock().unwrap();
                *lock = Some(child);
            }

            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(line);
                }
            }
            if let Some(stderr) = stderr {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(format!("⚠ {line}"));
                }
            }

            let child_opt = {
                let mut lock = child_handle.lock().unwrap();
                lock.take()
            };
            if let Some(mut child) = child_opt {
                let _ = child.wait().await;
            }

            is_running.set(false);
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Builds the execution argument list for a [`DownloadRequest`].
///
/// Returns a `Vec<String>` where each element is a separate process argument.
/// No shell quoting is applied — arguments are passed via `"$@"` expansion,
/// so the shell never interprets special characters.
///
/// # Argument Order
///
/// 1. Type flags (`-f FORMAT` for video, `-x` for audio)
/// 2. Output template (`-o TEMPLATE`)
/// 3. Download archive (`--download-archive PATH` if set)
/// 4. Extra user-selected flags
/// 5. Source (`--batch-file PATH` or the URL string)
fn build_exec_args(req: &DownloadRequest) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    //note: Allow yt-dlp to download the scripts from GitHub automatically.
    // "--remote-components ejs:github"

    // ── 1. Type-specific format/extraction flags ───────────────────────────
    match req.download_type {
        DownloadType::Video => {
            args.push("-f".to_string());
            args.push(req.quality.format_string().to_string());
        }
        DownloadType::Audio => {
            args.push("-x".to_string());
        }
    }

    // ── 2. Output template ────────────────────────────────────────────────
    let template = req.download_source.output_template(&req.output_dir);
    args.push("-o".to_string());
    args.push(template); // passed as a single arg — no quoting needed

    // ── 3. Download archive (optional) ───────────────────────────────────
    if !req.archive_file.trim().is_empty() {
        args.push("--download-archive".to_string());
        args.push(req.archive_file.clone());
    }

    // ── 4. Extra flags from the flag panel ───────────────────────────────
    for flag in &req.extra_flags {
        // Some flag strings include a value: "--audio-format mp3" → two tokens.
        for token in flag.flag.split_whitespace() {
            args.push(token.to_string());
        }
    }

    // ── 5. URL or batch file ──────────────────────────────────────────────
    match &req.download_source {
        DownloadSource::Batch if !req.batch_file.trim().is_empty() => {
            args.push("--batch-file".to_string());
            args.push(req.batch_file.clone());
        }
        _ if !req.url.trim().is_empty() => {
            args.push(req.url.clone());
        }
        _ => {}
    }

    args
}
