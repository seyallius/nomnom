//! runner.rs - Builds and spawns the yt-dlp subprocess

use dioxus::prelude::*;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::core::flags::Flag;

// -------------------------------------------- Public Functions --------------------------------------------

/// Build the preview command string shown in the UI
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

/// Spawn yt-dlp and stream output lines into `log_lines`
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

/// Run an arbitrary raw command string (from the terminal panel)
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
