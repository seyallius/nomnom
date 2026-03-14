//! core/mod.rs - Core business logic modules for yt-dlp operations.
//!
//! This module re-exports the core logic components that are independent
//! of the UI framework. These modules contain no Dioxus-specific code
//! and can be tested or reused independently.
//!
//! # Modules
//!
//! - [`download_mode`] — Download type, source, and quality enums
//! - [`flags`] - All yt-dlp flag definitions and metadata
//! - [`presets`] - Pre-configured flag bundles for common use cases
//! - [`runner`] - Subprocess spawning and output streaming

pub mod download_mode;
pub mod flags;
pub mod presets;
pub mod runner;
