//! runner.rs - yt-dlp subprocess management and output streaming.
//!
//! This module provides:
//! - Command string building for UI preview ([`build_command_string`])
//! - Async subprocess spawning with live output ([`run_download`])
//! - Raw command execution for the terminal panel ([`run_raw_command`])
//!
//! # Architecture
//!
//! The runner uses `tokio::process::Command` for async subprocess management.
//! Output is streamed line-by-line via `AsyncBufReadExt` and written to
//! a Dioxus [`Signal<Vec<String>>`] for real-time UI updates.
//!
//! # Error Handling
//!
//! All errors are captured and written to the log as user-friendly messages.
//! The application never panics from subprocess failures.

use crate::core::flags::Flag;
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
/// Shared between the async runner and the UI stop button.
/// Wrapped in `Arc<Mutex<...>>` so it can be cloned into both the
/// spawned async task and the button's `onclick` handler.
///
/// # Usage
///
/// ```rust,ignore
/// let handle: ChildHandle = Arc::new(Mutex::new(None));
/// // pass into run_download — it stores the child inside
/// // pass a clone into the stop button — it calls cancel_download
/// ```
pub type ChildHandle = Arc<Mutex<Option<Child>>>;

// -------------------------------------------- Public API --------------------------------------------

/// Builds a human-readable command string for the UI preview.
///
/// This function constructs the exact command that would be run in a terminal,
/// useful for showing users what will execute before they click download.
///
/// # Arguments
///
/// * `url` - The video/playlist URL to download.
/// * `flags` - Slice of active flags to include in the command.
/// * `output_dir` - Directory where files will be saved.
///
/// # Returns
///
/// A formatted command string like:
/// ```text
/// yt-dlp --add-metadata -o "/home/user/Downloads/%(title)s.%(ext)s" "https://youtube.com/..."
/// ```
///
/// If `url` is empty, returns a placeholder: `"yt-dlp [url] ..."`.
///
/// # Example
///
/// ```rust,ignore
/// let cmd = build_command_string(
///     "https://youtube.com/watch?v=abc",
///     &[Flag { flag: "--no-overwrites", ... }],
///     "/home/user/Downloads"
/// );
/// assert!(cmd.starts_with("yt-dlp"));
/// ```
pub fn build_command_string(url: &str, flags: &[Flag], output_dir: &str) -> String {
    if url.trim().is_empty() {
        return "yt-dlp [url] ...".to_string();
    }

    let flags_str = flags.iter().map(|f| f.flag).collect::<Vec<_>>().join(" ");

    let output_template = format!(
        "-o \"{}/%(title)s.%(ext)s\"",
        output_dir.trim_end_matches('/')
    );

    format!("yt-dlp {} {} \"{}\"", flags_str, output_template, url)
}

/// Kills the active child process stored in the handle.
///
/// This is the cancel/stop function called by the UI stop button.
/// It locks the handle, takes the child out, and sends SIGKILL.
///
/// # Arguments
///
/// * handle - The shared child handle to kill.
/// * log_lines - Signal to write a cancellation log message.
/// * is_running - Signal to reset running state.
///
/// # Behavior
///
/// - If no child is running, does nothing silently.
/// - On kill success, logs “⛔ Download canceled by user.”
/// - On kill error, logs the error message.
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
        // Drop the child out of the slot
        *lock = None;
    }
    is_running.set(false);
}

/// Spawns yt-dlp as a subprocess and streams output to the log.
///
/// # Arguments
///
/// * url - The video/playlist URL to download.
/// * flags - Active flags to pass to yt-dlp.
/// * output_dir - Directory for output files.
/// * log_lines - Signal to receive output lines (will be cleared first).
/// * is_running - Signal to track running state.
/// * child_handle - Shared slot to store the child process for cancellation.
pub async fn run_download(
    url: String,
    flags: Vec<Flag>,
    output_dir: String,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
    child_handle: ChildHandle,
) {
    is_running.set(true);
    log_lines.write().clear();
    log_lines.write().push("▶ Starting download…".to_string());

    let output_template = format!("{}/%(title)s.%(ext)s", output_dir.trim_end_matches('/'));

    // Build args vec
    let mut args: Vec<String> = vec![
        "--remote-components".to_string(),
        "ejs:github".to_string(), // Allow yt-dlp to download the scripts from GitHub automatically
        "-o".to_string(),
        output_template,
    ];
    for flag in &flags {
        // Each flag may have multiple tokens e.g. "--audio-format mp3"
        for token in flag.flag.split_whitespace() {
            args.push(token.to_string());
        }
    }
    args.push(url.clone());

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let result = Command::new(&shell)
        .arg("-i")
        .arg("-c")
        .arg("yt-dlp \"$@\"") // $@ expands positional args safely
        .arg("bash") // $0 = script name placeholder, NOT in $@
        .args(&args) // each arg passed as its own element, no parsing
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
            // ── Stream stdout ──────────────────────────────────────────────
            // We must take stdout/stderr BEFORE storing the child,
            // because storing moves it into the Mutex.
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            // Store child in the shared handle so the stop button can kill it
            {
                let mut lock = child_handle.lock().unwrap();
                *lock = Some(child);
            }

            // Stream stdout lines
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(line);
                }
            }

            // Stream stderr lines
            if let Some(stderr) = stderr {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(format!("⚠ {line}"));
                }
            }

            // Wait for process and report exit status
            // Take child back out of the handle to call .wait()
            let child_opt = {
                let mut lock = child_handle.lock().unwrap();
                lock.take()
            };

            if let Some(mut child) = child_opt {
                match child.wait().await {
                    Ok(status) if status.success() => {
                        log_lines.write().push("✔ Download complete!".to_string());
                    }
                    Ok(status) => {
                        // Check if we were canceled (is_running already false)
                        // to avoid showing a confusing exit code after user cancel
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
            // If child_opt is None, user cancelled — message already logged by cancel_download

            is_running.set(false);
        }
    }
}

/// Executes an arbitrary command string from the terminal panel.
///
/// This function allows power users to run any yt-dlp command directly,
/// bypassing the GUI flag selection.
///
/// # Arguments
///
/// * raw - The raw command string to execute.
/// * log_lines - Signal to receive output lines.
/// * is_running - Signal to track running state.
/// * child_handle - Shared slot to store the child process for cancellation.
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
