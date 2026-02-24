//! presets.rs - Pre-configured flag bundles for common download scenarios.
//!
//! This module provides:
//! - The [`Preset`] struct representing a named flag collection
//! - Built-in presets via [`all_presets`]
//! - Resolution logic via [`resolve_preset_flags`]
//!
//! # What is a Preset?
//!
//! A preset is a pre-selected combination of flags tailored for a specific
//! use case (e.g., "Audio Only" or "Full Archive"). Users can click a preset
//! to instantly configure multiple flags at once.
//!
//! # Adding New Presets
//!
//! To add a new preset, append to the [`all_presets`] function:
//!
//! ```rust,ignore
//! Preset {
//!     id: "my_preset",
//!     label: "My Custom Preset",
//!     description: "What this preset does",
//!     icon: "🎸",
//!     flag_keys: vec!["--add-metadata", "--no-overwrites"],
//! },
//! ```
//!
//! Note: `flag_keys` must match `Flag::flag` strings defined in [`flags`](super::flags).

use crate::core::flags::{all_flags, Flag};
use serde::{Deserialize, Serialize};

// -------------------------------------------- Types --------------------------------------------

/// A named, pre-configured collection of yt-dlp flags.
///
/// Presets allow users to quickly select common flag combinations
/// without manually toggling each flag.
///
/// # Example
///
/// The "Audio Only" preset bundles:
/// - `--extract-audio`
/// - `--audio-format mp3`
/// - `--audio-quality 0`
/// - `--add-metadata`
/// - `--add-thumbnail`
/// - `--no-overwrites`
///
/// # Fields
///
/// - `id` - Unique identifier used for equality checks and persistence
/// - `label` - Display name shown in the UI
/// - `description` - Brief explanation of what the preset does
/// - `icon` - Emoji displayed next to the label
/// - `flag_keys` - List of flag strings to enable (matched against [`Flag::flag`])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    /// Unique identifier for this preset.
    pub id: &'static str,
    /// Human-readable display name.
    pub label: &'static str,
    /// Brief description shown in the UI.
    pub description: &'static str,
    /// Emoji icon for visual identification.
    pub icon: &'static str,
    /// Flag strings to enable when this preset is selected.
    /// These are matched against [`Flag::flag`] in [`all_flags`].
    pub flag_keys: Vec<&'static str>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Returns all built-in presets available to the user.
///
/// # Returns
///
/// A `Vec<Preset>` containing all defined presets in display order.
///
/// # Built-in Presets
///
/// | ID | Label | Purpose |
/// |----|-------|---------|
/// | `best_video` | Best Video | High-quality video with metadata |
/// | `audio_only` | Audio Only (MP3) | Extract audio as MP3 |
/// | `playlist` | Full Playlist | Download entire playlists |
/// | `with_subs` | Video + Subtitles | Video with embedded English subtitles |
/// | `archive` | Full Archive | Download everything (video, subs, metadata, JSON) |
pub fn all_presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "best_video",
            label: "Best Video",
            description: "Download best quality video with metadata and thumbnail",
            icon: "🎬",
            flag_keys: vec![
                "--add-metadata",
                "--add-thumbnail",
                "--merge-output-format mp4",
                "--no-overwrites",
                "--continue",
            ],
        },
        Preset {
            id: "audio_only",
            label: "Audio Only (MP3)",
            description: "Extract audio as high-quality MP3",
            icon: "🎵",
            flag_keys: vec![
                "--extract-audio",
                "--audio-format mp3",
                "--audio-quality 0",
                "--add-metadata",
                "--add-thumbnail",
                "--no-overwrites",
            ],
        },
        Preset {
            id: "playlist",
            label: "Full Playlist",
            description: "Download entire playlist in MP4 with metadata",
            icon: "📋",
            flag_keys: vec![
                "--yes-playlist",
                "--add-metadata",
                "--add-thumbnail",
                "--merge-output-format mp4",
                "--no-overwrites",
                "--continue",
            ],
        },
        Preset {
            id: "with_subs",
            label: "Video + Subtitles",
            description: "Download video and embed English subtitles",
            icon: "💬",
            flag_keys: vec![
                "--add-metadata",
                "--write-subs",
                "--write-auto-subs",
                "--embed-subs",
                "--sub-langs en",
                "--merge-output-format mp4",
                "--no-overwrites",
            ],
        },
        Preset {
            id: "archive",
            label: "Full Archive",
            description: "Download everything: video, subs, metadata, description",
            icon: "🗄️",
            flag_keys: vec![
                "--add-metadata",
                "--add-thumbnail",
                "--write-description",
                "--write-info-json",
                "--write-subs",
                "--write-auto-subs",
                "--embed-subs",
                "--sub-langs en",
                "--merge-output-format mkv",
                "--no-overwrites",
                "--continue",
            ],
        },
    ]
}

/// Returns the default preset shown on first application launch.
///
/// Currently returns the first preset ("Best Video").
///
/// # Returns
///
/// The default [`Preset`] to display as active on startup.
///
/// # Panics
///
/// Panics if [`all_presets`] returns an empty vec (should never happen).
pub fn default_preset() -> Preset {
    all_presets().into_iter().next().unwrap()
}

/// Resolves a preset's flag keys into actual [`Flag`] structs.
///
/// This function looks up each `flag_key` in the global flag registry
/// and returns the matching [`Flag`] instances.
///
/// # Arguments
///
/// * `preset` - The preset whose flags should be resolved.
///
/// # Returns
///
/// A `Vec<Flag>` containing all flags referenced by the preset.
/// Any flag keys that don't exist in [`all_flags`] are silently skipped.
///
/// # Example
///
/// ```rust,ignore
/// let preset = all_presets()[0].clone();
/// let flags = resolve_preset_flags(&preset);
/// assert!(!flags.is_empty());
/// ```
pub fn resolve_preset_flags(preset: &Preset) -> Vec<Flag> {
    let all = all_flags();
    preset
        .flag_keys
        .iter()
        .filter_map(|key| all.iter().find(|f| f.flag == *key).cloned())
        .collect()
}
