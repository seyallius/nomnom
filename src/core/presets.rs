//! presets.rs - Pre-configured download profiles for common scenarios.
//!
//! This module provides:
//! - The [`Preset`] struct (now includes type, source, and quality axes)
//! - Eight built-in presets via [`all_presets`] matching the user's own commands
//! - Flag resolution via [`resolve_preset_flags`]
//!
//! # Preset Matrix
//!
//! Presets are organised along two axes:
//!
//! | Source ╲ Type | Video            | Audio            |
//! |---------------|------------------|------------------|
//! | Single URL    | `single_video`   | `single_audio`   |
//! | Batch File    | `batch_video`    | `batch_audio`    |
//! | Playlist      | `video_playlist` | `audio_playlist` |
//! | Channel       | `channel_video`  | `channel_audio`  |
//!
//! # Adding New Presets
//!
//! Append to [`all_presets`] and reference flag strings from [`flags::all_flags`]:
//!
//! ```rust,ignore
//! Preset {
//!     id: "my_preset",
//!     label: "My Preset",
//!     description: "What it does",
//!     icon: "🚀",
//!     download_type: DownloadType::Video,
//!     download_source: DownloadSource::Single,
//!     quality: Quality::HD1080,
//!     flag_keys: vec!["--embed-thumbnail", "--add-metadata"],
//! }
//! ```

use crate::core::{
    download_mode::{DownloadSource, DownloadType, Quality},
    flags::{all_flags, Flag},
};
use serde::{Deserialize, Serialize};

// -------------------------------------------- Types --------------------------------------------

/// A named, pre-configured download profile.
///
/// A preset bundles a download type, source, quality level, and a set of
/// flags so users can get the right command with a single click.
///
/// # Fields
///
/// - `id`              — Unique stable identifier (used for equality checks)
/// - `label`           — Short human-readable name shown in the UI
/// - `description`     — One-line explanation shown beneath the label
/// - `icon`            — Emoji for visual scanning
/// - `download_type`   — Video or audio extraction
/// - `download_source` — Single, batch, playlist, or channel
/// - `quality`         — Video resolution cap (ignored for audio)
/// - `flag_keys`       — Flags to activate, matched against [`Flag::flag`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    /// Stable unique identifier.
    pub id: &'static str,
    /// Human-readable display name.
    pub label: &'static str,
    /// Brief description shown in the card subtitle.
    pub description: &'static str,
    /// Emoji icon for the preset card.
    pub icon: &'static str,
    /// Whether this preset downloads video or extracts audio.
    pub download_type: DownloadType,
    /// URL source and output folder organisation strategy.
    pub download_source: DownloadSource,
    /// Video resolution cap (ignored for audio presets).
    pub quality: Quality,
    /// Flag strings to activate, matched against [`Flag::flag`] in [`all_flags`].
    pub flag_keys: Vec<&'static str>,
}

// -------------------------------------------- Private Constants --------------------------------------------

/// Base flags shared by all video download presets.
const VIDEO_BASE: &[&str] = &[
    "--embed-thumbnail",
    "--add-metadata",
    "--embed-chapters",
    "--embed-info-json",
    "--embed-subs",
    "--merge-output-format mp4",
    "--no-overwrites",
    "--continue",
];

/// Base flags shared by all audio extraction presets.
const AUDIO_BASE: &[&str] = &[
    "--extract-audio",
    "--audio-format mp3",
    "--audio-quality 0",
    "--embed-thumbnail",
    "--add-metadata",
    "--embed-chapters",
    "--embed-info-json",
    "--no-overwrites",
    "--continue",
];

// -------------------------------------------- Public API --------------------------------------------

/// Returns all eight built-in presets in display order.
///
/// Presets are grouped: four video presets, then four audio presets.
/// Each preset matches one of the user's real-world yt-dlp command patterns.
pub fn all_presets() -> Vec<Preset> {
    // ── Video Presets ──────────────────────────────────────────────────────
    let mut single_video_flags = VIDEO_BASE.to_vec();
    let single_video = Preset {
        id: "single_video",
        label: "Single Video",
        description: "One video at 1080p with metadata, chapters & thumbnail",
        icon: "🎬",
        download_type: DownloadType::Video,
        download_source: DownloadSource::Single,
        quality: Quality::HD1080,
        flag_keys: { single_video_flags.clone() },
    };

    let batch_video = Preset {
        id: "batch_video",
        label: "Batch Videos",
        description: "Videos from a .txt file — one URL per line",
        icon: "📄",
        download_type: DownloadType::Video,
        download_source: DownloadSource::Batch,
        quality: Quality::HD1080,
        flag_keys: VIDEO_BASE.to_vec(),
    };

    let mut playlist_video_flags = VIDEO_BASE.to_vec();
    playlist_video_flags.insert(0, "--yes-playlist");
    let video_playlist = Preset {
        id: "video_playlist",
        label: "Video Playlist",
        description: "Full playlist sorted into Playlists/@uploader/title/",
        icon: "📋",
        download_type: DownloadType::Video,
        download_source: DownloadSource::Playlist,
        quality: Quality::HD1080,
        flag_keys: playlist_video_flags,
    };

    let channel_video = Preset {
        id: "channel_video",
        label: "Channel Videos",
        description: "Archive entire channel sorted into Channels/@uploader/",
        icon: "📺",
        download_type: DownloadType::Video,
        download_source: DownloadSource::Channel,
        quality: Quality::HD1080,
        flag_keys: VIDEO_BASE.to_vec(),
    };

    // ── Audio Presets ──────────────────────────────────────────────────────
    let single_audio = Preset {
        id: "single_audio",
        label: "Single Audio",
        description: "Extract audio as MP3 with metadata, chapters & thumbnail",
        icon: "🎵",
        download_type: DownloadType::Audio,
        download_source: DownloadSource::Single,
        quality: Quality::HD1080,
        flag_keys: AUDIO_BASE.to_vec(),
    };

    let batch_audio = Preset {
        id: "batch_audio",
        label: "Batch Audio",
        description: "Extract audio from each URL in a .txt batch file",
        icon: "📄",
        download_type: DownloadType::Audio,
        download_source: DownloadSource::Batch,
        quality: Quality::HD1080,
        flag_keys: AUDIO_BASE.to_vec(),
    };

    let mut playlist_audio_flags = AUDIO_BASE.to_vec();
    playlist_audio_flags.insert(0, "--yes-playlist");
    let audio_playlist = Preset {
        id: "audio_playlist",
        label: "Audio Playlist",
        description: "Extract all audio from a playlist into Playlists/@uploader/",
        icon: "🎧",
        download_type: DownloadType::Audio,
        download_source: DownloadSource::Playlist,
        quality: Quality::HD1080,
        flag_keys: playlist_audio_flags,
    };

    let channel_audio = Preset {
        id: "channel_audio",
        label: "Channel Audio",
        description: "Extract all audio from a channel into Channels/@uploader/",
        icon: "📻",
        download_type: DownloadType::Audio,
        download_source: DownloadSource::Channel,
        quality: Quality::HD1080,
        flag_keys: AUDIO_BASE.to_vec(),
    };

    vec![
        single_video,
        batch_video,
        video_playlist,
        channel_video,
        single_audio,
        batch_audio,
        audio_playlist,
        channel_audio,
    ]
}

/// Returns the default preset shown on first launch.
///
/// # Panics
///
/// Panics if [`all_presets`] returns an empty vec (should never happen in practice).
pub fn default_preset() -> Preset {
    all_presets().into_iter().next().unwrap()
}

/// Resolves a preset's flag keys into actual [`Flag`] structs.
///
/// Looks up each `flag_key` string in the global flag registry and
/// returns the matching [`Flag`] instances. Unrecognised keys are silently skipped.
///
/// # Arguments
///
/// * `preset` — The preset whose flags should be resolved.
pub fn resolve_preset_flags(preset: &Preset) -> Vec<Flag> {
    let all = all_flags();
    preset
        .flag_keys
        .iter()
        .filter_map(|key| all.iter().find(|f| f.flag == *key).cloned())
        .collect()
}
