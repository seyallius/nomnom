//! presets.rs - Pre-configured flag sets for common use cases

use crate::core::flags::{all_flags, Flag, FlagCategory};
use serde::{Deserialize, Serialize};

// -------------------------------------------- Public Types --------------------------------------------

/// A named collection of pre-selected flags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    /// Flag strings (matched against Flag::flag)
    pub flag_keys: Vec<&'static str>,
}

// -------------------------------------------- Public Functions --------------------------------------------

/// All built-in presets
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

/// The preset that is active on first launch
pub fn default_preset() -> Preset {
    all_presets().into_iter().next().unwrap()
}

/// Resolve a preset's flag_keys into actual Flag structs
pub fn resolve_preset_flags(preset: &Preset) -> Vec<Flag> {
    let all = all_flags();
    preset
        .flag_keys
        .iter()
        .filter_map(|key| all.iter().find(|f| f.flag == *key).cloned())
        .collect()
}
