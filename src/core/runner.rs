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

use dioxus::prelude::*;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::core::flags::Flag;

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

/// Spawns yt-dlp as a subprocess and streams output to the log.
///
/// This is the main download function that:
/// 1. Constructs the argument list from flags and URL
/// 2. Spawns yt-dlp with piped stdout/stderr
/// 3. Streams output lines to `log_lines` in real-time
/// 4. Updates `is_running` state on completion
///
/// # Arguments
///
/// * `url` - The video/playlist URL to download.
/// * `flags` - Active flags to pass to yt-dlp.
/// * `output_dir` - Directory for output files.
/// * `log_lines` - Signal to receive output lines (will be cleared first).
/// * `is_running` - Signal to track running state.
///
/// # Async Behavior
///
/// This function runs asynchronously and updates signals as output arrives.
/// The UI will re-render automatically as `log_lines` is modified.
///
/// # Error Handling
///
/// - If yt-dlp fails to spawn, an error message is pushed to `log_lines`
/// - If yt-dlp exits non-zero, the exit status is logged
/// - stderr lines are prefixed with `⚠` for visibility
///
/// # Example
///
/// ```rust,ignore
/// spawn(async move {
///     run_download(
///         url.clone(),
///         flags.clone(),
///         output_dir.clone(),
///         log_lines,
///         is_running,
///     ).await;
/// });
/// ```
pub async fn run_download(
    url: String,
    flags: Vec<Flag>,
    output_dir: String,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
) {
    is_running.set(true);
    log_lines.write().clear();
    log_lines.write().push("▶ Starting download…".to_string());

    let output_template = format!("{}/%(title)s.%(ext)s", output_dir.trim_end_matches('/'));

    // Build args vec
    let mut args: Vec<String> = vec!["-o".to_string(), output_template];

    for flag in &flags {
        // Each flag may have multiple tokens e.g. "--audio-format mp3"
        for token in flag.flag.split_whitespace() {
            args.push(token.to_string());
        }
    }
    args.push(url.clone());

    let result = Command::new("yt-dlp")
        .args(&args)
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
            // Stream stdout
            if let Some(stdout) = child.stdout.take() {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(line);
                }
            }
            // Collect stderr
            if let Some(stderr) = child.stderr.take() {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(format!("⚠ {line}"));
                }
            }

            match child.wait().await {
                Ok(status) if status.success() => {
                    log_lines.write().push("✔ Download complete!".to_string());
                }
                Ok(status) => {
                    log_lines
                        .write()
                        .push(format!("✗ yt-dlp exited with: {status}"));
                }
                Err(e) => {
                    log_lines.write().push(format!("✗ Wait error: {e}"));
                }
            }

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
/// * `raw` - The raw command string to execute (e.g., `"yt-dlp -f best URL"`).
/// * `log_lines` - Signal to receive output lines.
/// * `is_running` - Signal to track running state.
///
/// # Parsing
///
/// The command is split on whitespace. The first token is the command,
/// remaining tokens are arguments.
///
/// # Security Note
///
/// This function runs arbitrary commands on the user's system.
/// It is intended for advanced users who understand the risks.
///
/// # Example
///
/// ```rust,ignore
/// spawn(async move {
///     run_raw_command(
///         "yt-dlp --version".to_string(),
///         log_lines,
///         is_running,
///     ).await;
/// });
/// ```
pub async fn run_raw_command(
    raw: String,
    mut log_lines: Signal<Vec<String>>,
    mut is_running: Signal<bool>,
) {
    is_running.set(true);
    log_lines.write().push(format!("$ {raw}"));

    let tokens: Vec<&str> = raw.split_whitespace().collect();
    if tokens.is_empty() {
        is_running.set(false);
        return;
    }

    let (cmd, args) = tokens.split_first().unwrap();

    let result = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match result {
        Err(e) => {
            log_lines.write().push(format!("✗ {e}"));
            is_running.set(false);
        }
        Ok(mut child) => {
            if let Some(stdout) = child.stdout.take() {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(line);
                }
            }
            if let Some(stderr) = child.stderr.take() {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_lines.write().push(format!("⚠ {line}"));
                }
            }
            let _ = child.wait().await;
            is_running.set(false);
        }
    }
}
