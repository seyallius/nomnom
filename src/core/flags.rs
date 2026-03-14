//! flags.rs - Complete yt-dlp flag definitions and categorisation.
//!
//! This module provides:
//! - The [`Flag`] struct representing a single yt-dlp CLI flag
//! - The [`FlagCategory`] enum for logical grouping in the UI
//! - A registry of all available flags via [`all_flags`]
//!
//! # Design Philosophy
//!
//! Flags are defined as static `&'static str` to avoid allocations.
//! Each flag carries metadata (label, description, category) to make
//! the UI self-documenting without any extra configuration.
//!
//! # Adding New Flags
//!
//! Append to the [`all_flags`] function:
//!
//! ```rust,ignore
//! Flag {
//!     flag: "--your-new-flag",
//!     label: "Human Label",
//!     description: "What this flag does",
//!     category: FlagCategory::Misc,
//! },
//! ```

use serde::{Deserialize, Serialize};

// -------------------------------------------- Types --------------------------------------------

/// A single yt-dlp CLI flag with associated display metadata.
///
/// # Example
///
/// ```rust,ignore
/// Flag {
///     flag: "--yes-playlist",
///     label: "Yes Playlist",
///     description: "Download entire playlist if URL points to one",
///     category: FlagCategory::Playlist,
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    /// CLI flag string as passed to yt-dlp, e.g. `"--yes-playlist"`.
    pub flag: &'static str,
    /// Human-readable label displayed on the toggle button.
    pub label: &'static str,
    /// Detailed description shown as a tooltip on hover.
    pub description: &'static str,
    /// Category used for visual grouping in the flag panel.
    pub category: FlagCategory,
}

/// Logical category for grouping flags in the sidebar UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlagCategory {
    /// Flags controlling playlist download behavior.
    Playlist,
    /// Flags for embedding or writing metadata.
    Metadata,
    /// Flags controlling output container format.
    Format,
    /// Flags for subtitle download and embedding.
    Subtitles,
    /// Flags for audio extraction and format.
    Audio,
    /// Flags for proxy, SSL, and geo-restriction handling.
    Network,
    /// Miscellaneous flags that don't fit other categories.
    Misc,
}

impl FlagCategory {
    /// Returns the display label with emoji prefix.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// assert_eq!(FlagCategory::Playlist.label(), "📋 Playlist");
    /// ```
    pub fn label(&self) -> &'static str {
        match self {
            FlagCategory::Playlist => "📋 Playlist",
            FlagCategory::Metadata => "🏷️  Metadata",
            FlagCategory::Format => "🎞️  Format",
            FlagCategory::Subtitles => "💬 Subtitles",
            FlagCategory::Audio => "🎵 Audio",
            FlagCategory::Network => "🌐 Network",
            FlagCategory::Misc => "⚙️  Misc",
        }
    }
}

// -------------------------------------------- Public API --------------------------------------------

/// Returns all available yt-dlp flags known to the application.
///
/// This is the single source of truth for flag definitions.
/// The UI iterates over this list to render toggle buttons,
/// and presets reference `Flag::flag` strings to select subsets.
pub fn all_flags() -> Vec<Flag> {
    vec![
        // ── Playlist ─────────────────────────────────────
        Flag {
            flag: "--yes-playlist",
            label: "Yes Playlist",
            description: "Download entire playlist if URL points to one",
            category: FlagCategory::Playlist,
        },
        Flag {
            flag: "--no-playlist",
            label: "No Playlist",
            description: "Download only single video even if part of a playlist",
            category: FlagCategory::Playlist,
        },
        Flag {
            flag: "--playlist-reverse",
            label: "Reverse Playlist",
            description: "Download playlist in reverse order",
            category: FlagCategory::Playlist,
        },
        // ── Metadata ──────────────────────────────────────
        Flag {
            flag: "--add-metadata",
            label: "Add Metadata",
            description: "Write metadata tags to the video file",
            category: FlagCategory::Metadata,
        },
        Flag {
            flag: "--embed-thumbnail",
            label: "Embed Thumbnail",
            description: "Embed the video thumbnail as cover art in the file",
            category: FlagCategory::Metadata,
        },
        Flag {
            flag: "--embed-chapters",
            label: "Embed Chapters",
            description: "Add chapter markers from the video description to the file",
            category: FlagCategory::Metadata,
        },
        Flag {
            flag: "--embed-info-json",
            label: "Embed Info JSON",
            description: "Embed the full video metadata JSON as a file attachment",
            category: FlagCategory::Metadata,
        },
        Flag {
            flag: "--write-description",
            label: "Write Description",
            description: "Save video description to a .description file",
            category: FlagCategory::Metadata,
        },
        Flag {
            flag: "--write-info-json",
            label: "Write Info JSON",
            description: "Save video metadata to a .info.json file on disk",
            category: FlagCategory::Metadata,
        },
        // ── Format ────────────────────────────────────────
        Flag {
            flag: "--merge-output-format mp4",
            label: "Force MP4",
            description: "Merge output into an MP4 container",
            category: FlagCategory::Format,
        },
        Flag {
            flag: "--merge-output-format mkv",
            label: "Force MKV",
            description: "Merge output into an MKV container",
            category: FlagCategory::Format,
        },
        Flag {
            flag: "--remux-video mp4",
            label: "Remux → MP4",
            description: "Remux the video into mp4 without re-encoding",
            category: FlagCategory::Format,
        },
        // ── Subtitles ─────────────────────────────────────
        Flag {
            flag: "--write-subs",
            label: "Write Subs",
            description: "Download subtitle files to disk",
            category: FlagCategory::Subtitles,
        },
        Flag {
            flag: "--write-auto-subs",
            label: "Auto Subs",
            description: "Download auto-generated subtitles",
            category: FlagCategory::Subtitles,
        },
        Flag {
            flag: "--embed-subs",
            label: "Embed Subs",
            description: "Embed subtitles directly into the video file",
            category: FlagCategory::Subtitles,
        },
        Flag {
            flag: "--sub-langs en",
            label: "English Subs",
            description: "Restrict subtitle download to English only",
            category: FlagCategory::Subtitles,
        },
        // ── Audio ─────────────────────────────────────────
        Flag {
            flag: "--extract-audio",
            label: "Extract Audio",
            description: "Convert video to audio-only output",
            category: FlagCategory::Audio,
        },
        Flag {
            flag: "--audio-format mp3",
            label: "Format: MP3",
            description: "Set audio output format to MP3",
            category: FlagCategory::Audio,
        },
        Flag {
            flag: "--audio-format m4a",
            label: "Format: M4A",
            description: "Set audio output format to M4A",
            category: FlagCategory::Audio,
        },
        Flag {
            flag: "--audio-quality 0",
            label: "Best Audio Quality",
            description: "Use best audio quality (0 = best for VBR encoders)",
            category: FlagCategory::Audio,
        },
        // ── Network ───────────────────────────────────────
        Flag {
            flag: "--no-check-certificates",
            label: "Skip SSL Check",
            description: "Bypass SSL certificate verification",
            category: FlagCategory::Network,
        },
        Flag {
            flag: "--geo-bypass",
            label: "Geo Bypass",
            description: "Bypass geographic restrictions via fake X-Forwarded-For",
            category: FlagCategory::Network,
        },
        Flag {
            flag: "--proxy socks5://127.0.0.1:1080",
            label: "Use Proxy",
            description: "Route traffic through local SOCKS5 proxy on port 1080",
            category: FlagCategory::Network,
        },
        // ── Misc ──────────────────────────────────────────
        Flag {
            flag: "--no-overwrites",
            label: "No Overwrites",
            description: "Skip download if output file already exists",
            category: FlagCategory::Misc,
        },
        Flag {
            flag: "--continue",
            label: "Continue",
            description: "Resume partially downloaded files",
            category: FlagCategory::Misc,
        },
        Flag {
            flag: "--no-part",
            label: "No .part Files",
            description: "Do not use .part files — write directly to final filename",
            category: FlagCategory::Misc,
        },
        Flag {
            flag: "--verbose",
            label: "Verbose",
            description: "Print verbose debug output to the log",
            category: FlagCategory::Misc,
        },
        Flag {
            flag: "--restrict-filenames",
            label: "Restrict Filenames",
            description: "Use only ASCII characters in output filenames",
            category: FlagCategory::Misc,
        },
        Flag {
            flag: "--sponsorblock-remove all",
            label: "SponsorBlock",
            description: "Remove sponsored segments using SponsorBlock data",
            category: FlagCategory::Misc,
        },
    ]
}
