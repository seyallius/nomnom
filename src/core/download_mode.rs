//! download_mode.rs - Download type, source, and quality configuration enums.
//!
//! Provides the three core axes that drive every yt-dlp invocation:
//! - [`DownloadType`]   — video stream vs. audio extraction
//! - [`DownloadSource`] — single URL, batch file, playlist, or channel
//! - [`Quality`]        — video resolution cap (video mode only)
//!
//! These enums are the single source of truth for command-building logic
//! in [`runner`](super::runner) and for UI state in the mode selector.

use serde::{Deserialize, Serialize};

// -------------------------------------------- Types --------------------------------------------

/// Whether to download a merged video stream or extract audio only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DownloadType {
    /// Download video + audio merged into a container (MP4 by default).
    #[default]
    Video,
    /// Extract audio only — passes `-x` to yt-dlp with a chosen audio format.
    Audio,
}

/// The origin of URLs and how output files should be organised on disk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DownloadSource {
    /// A single URL typed or pasted by the user.
    #[default]
    Single,
    /// A plain-text file with one URL per line (`--batch-file`).
    Batch,
    /// A playlist URL — files sorted into `Playlists/@uploader/playlist/` sub-folders.
    Playlist,
    /// A channel URL — files sorted into `Channels/@uploader/` sub-folders.
    Channel,
}

/// Video resolution cap; ignored when [`DownloadType`] is [`Audio`](DownloadType::Audio).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Quality {
    /// No height cap — `bestvideo+bestaudio/best`.
    Best,
    /// Cap at 1080p — `bestvideo[height<=1080]+bestaudio/best[height<=1080]`.
    #[default]
    HD1080,
    /// Cap at 720p — `bestvideo[height<=720]+bestaudio/best[height<=720]`.
    HD720,
    /// Cap at 480p — `bestvideo[height<=480]+bestaudio/best[height<=480]`.
    SD480,
}

// -------------------------------------------- Public API --------------------------------------------

impl DownloadType {
    /// Returns a short display label with emoji.
    pub fn label(&self) -> &'static str {
        match self {
            DownloadType::Video => "📹 Video",
            DownloadType::Audio => "🎵 Audio",
        }
    }
}

impl DownloadSource {
    /// Returns a short display label.
    pub fn label(&self) -> &'static str {
        match self {
            DownloadSource::Single => "Single URL",
            DownloadSource::Batch => "Batch File",
            DownloadSource::Playlist => "Playlist",
            DownloadSource::Channel => "Channel",
        }
    }

    /// Returns an emoji icon for compact display.
    pub fn icon(&self) -> &'static str {
        match self {
            DownloadSource::Single => "🔗",
            DownloadSource::Batch => "📄",
            DownloadSource::Playlist => "📋",
            DownloadSource::Channel => "📺",
        }
    }

    /// Returns `true` when this source uses a typed/pasted URL (not a batch file path).
    #[allow(dead_code)]
    pub fn needs_url(&self) -> bool {
        !matches!(self, DownloadSource::Batch)
    }

    /// Builds the yt-dlp `-o` output template for this source type.
    ///
    /// Organises files into logical sub-directories mirroring the user's
    /// personal command conventions (Playlists/, Channels/, flat root for singles).
    ///
    /// # Arguments
    ///
    /// * `base_dir` — The root output directory chosen by the user.
    pub fn output_template(&self, base_dir: &str) -> String {
        let dir = base_dir.trim_end_matches('/');
        match self {
            DownloadSource::Single | DownloadSource::Batch => format!(
                "{}/%(title)s - [%(uploader)s - %(upload_date>%b %d %Y)s].%(ext)s",
                dir
            ),
            DownloadSource::Playlist => format!(
                "{}/Playlists/@%(uploader)s/%(playlist_title)s/%(playlist_index)03d - %(title)s - [%(upload_date>%b %d %Y)s].%(ext)s",
                dir
            ),
            DownloadSource::Channel => format!(
                "{}/Channels/@%(uploader)s/%(title)s - [%(upload_date>%b %d %Y)s].%(ext)s",
                dir
            ),
        }
    }
}

impl Quality {
    /// Returns a short display label (e.g. `"1080p"`).
    pub fn label(&self) -> &'static str {
        match self {
            Quality::Best => "Best",
            Quality::HD1080 => "1080p",
            Quality::HD720 => "720p",
            Quality::SD480 => "480p",
        }
    }

    /// Returns the yt-dlp `-f` format selector for this quality level.
    pub fn format_string(&self) -> &'static str {
        match self {
            Quality::Best => "bestvideo+bestaudio/best",
            Quality::HD1080 => "bestvideo[height<=1080]+bestaudio/best[height<=1080]",
            Quality::HD720 => "bestvideo[height<=720]+bestaudio/best[height<=720]",
            Quality::SD480 => "bestvideo[height<=480]+bestaudio/best[height<=480]",
        }
    }
}
