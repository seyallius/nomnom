//! preset_panel.rs - Preset cards organised into Video and Audio sections.
//!
//! Renders a visual grid of 8 preset cards (4 video + 4 audio) plus a
//! "Custom" option. Selecting a preset sets the download type, source,
//! quality, and active flags all at once.
//!
//! # Visual Layout
//!
//! ```text
//! ⚡ PRESETS
//! ─── 📹 VIDEO ──────────────────
//! [🎬 Single Video  ] [📄 Batch ]
//! [📋 Playlist      ] [📺 Channel]
//! ─── 🎵 AUDIO ──────────────────
//! [🎵 Single Audio  ] [📄 Batch ]
//! [🎧 Playlist      ] [📻 Channel]
//! ────────────────────────────────
//! [🔧 Custom                    ]
//! ```

use crate::core::{
    download_mode::{DownloadSource, DownloadType, Quality},
    flags::Flag,
    presets::{all_presets, resolve_preset_flags, Preset},
};
use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`PresetPanel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PresetPanelProps {
    /// Currently active preset — `None` means Custom mode.
    pub active_preset: Signal<Option<Preset>>,
    /// Active flags signal — updated when a preset is selected.
    pub active_flags: Signal<Vec<Flag>>,
    /// Download type signal — updated when a preset is selected.
    pub download_type: Signal<DownloadType>,
    /// Download source signal — updated when a preset is selected.
    pub download_source: Signal<DownloadSource>,
    /// Quality signal — updated when a preset is selected.
    pub quality: Signal<Quality>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders a two-section (Video / Audio) preset selector plus a Custom card.
///
/// Clicking a preset sets all four signals (`download_type`, `download_source`,
/// `quality`, `active_flags`) and marks that preset as active.
/// Clicking Custom clears flags and sets `active_preset` to `None`.
///
/// # Arguments
///
/// * `props` — Signals for the active preset, flags, type, source, and quality.
#[component]
pub fn PresetPanel(props: PresetPanelProps) -> Element {
    let mut active_preset = props.active_preset;
    let mut active_flags = props.active_flags;
    let mut download_type = props.download_type;
    let mut download_source = props.download_source;
    let mut quality = props.quality;

    let presets = all_presets();

    let video_presets: Vec<Preset> = presets
        .iter()
        .filter(|p| p.download_type == DownloadType::Video)
        .cloned()
        .collect();

    let audio_presets: Vec<Preset> = presets
        .iter()
        .filter(|p| p.download_type == DownloadType::Audio)
        .cloned()
        .collect();

    let is_custom = active_preset.read().is_none();

    rsx! {
        div {
            // ── Section heading ───────────────────────────────────────
            h3 {
                style: "
                    margin: 0 0 10px 0;
                    font-size: 11px;
                    text-transform: uppercase;
                    letter-spacing: 1.5px;
                    color: #6c63ff;
                ",
                "⚡ Presets"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                // ── Video presets ─────────────────────────────────────
                div {
                    div {
                        style: "
                            font-size: 10px;
                            color: #4a90d9;
                            letter-spacing: 1px;
                            text-transform: uppercase;
                            margin-bottom: 6px;
                            padding-bottom: 4px;
                            border-bottom: 1px solid #1a2a3a;
                        ",
                        "📹 Video"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 5px;",

                        for preset in video_presets {
                            {
                                let is_active = active_preset
                                    .read()
                                    .as_ref()
                                    .map(|p| p.id)
                                    == Some(preset.id);
                                let preset_clone = preset.clone();

                                rsx! {
                                    PresetCard {
                                        key: "{preset.id}",
                                        preset: preset_clone.clone(),
                                        is_active,
                                        on_click: move |_| {
                                            let flags = resolve_preset_flags(&preset_clone);
                                            active_flags.set(flags);
                                            download_type.set(preset_clone.download_type.clone());
                                            download_source.set(preset_clone.download_source.clone());
                                            quality.set(preset_clone.quality.clone());
                                            active_preset.set(Some(preset_clone.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Audio presets ─────────────────────────────────────
                div {
                    div {
                        style: "
                            font-size: 10px;
                            color: #d94aad;
                            letter-spacing: 1px;
                            text-transform: uppercase;
                            margin-bottom: 6px;
                            padding-bottom: 4px;
                            border-bottom: 1px solid #2a1a2a;
                        ",
                        "🎵 Audio"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 5px;",

                        for preset in audio_presets {
                            {
                                let is_active = active_preset
                                    .read()
                                    .as_ref()
                                    .map(|p| p.id)
                                    == Some(preset.id);
                                let preset_clone = preset.clone();

                                rsx! {
                                    PresetCard {
                                        key: "{preset.id}",
                                        preset: preset_clone.clone(),
                                        is_active,
                                        on_click: move |_| {
                                            let flags = resolve_preset_flags(&preset_clone);
                                            active_flags.set(flags);
                                            download_type.set(preset_clone.download_type.clone());
                                            download_source.set(preset_clone.download_source.clone());
                                            quality.set(preset_clone.quality.clone());
                                            active_preset.set(Some(preset_clone.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Custom ────────────────────────────────────────────
                div {
                    style: custom_card_style(is_custom),
                    onclick: move |_| {
                        active_preset.set(None);
                        active_flags.set(vec![]);
                    },
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        span { "🔧" }
                        span {
                            style: "font-size: 12px; font-weight: bold;",
                            "Custom"
                        }
                    }
                    div {
                        style: "font-size: 10px; color: #555; margin-top: 2px;",
                        "Pick type, source & flags manually"
                    }
                }
            }
        }
    }
}

// -------------------------------------------- Private Components --------------------------------------------

/// Props for the internal [`PresetCard`] component.
#[derive(Props, Clone, PartialEq)]
struct PresetCardProps {
    preset: Preset,
    is_active: bool,
    on_click: EventHandler<MouseEvent>,
}

/// A single clickable preset card showing icon, label, and description.
#[component]
fn PresetCard(props: PresetCardProps) -> Element {
    rsx! {
        div {
            style: preset_card_style(props.is_active),
            title: "{props.preset.description}",
            onclick: move |e| props.on_click.call(e),

            div {
                style: "display: flex; align-items: center; gap: 5px; margin-bottom: 2px;",
                span { style: "font-size: 14px; line-height: 1;", "{props.preset.icon}" }
                span {
                    style: "
                        font-size: 11px;
                        font-weight: bold;
                        color: #d0d0e0;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                    ",
                    "{props.preset.label}"
                }
            }

            div {
                style: "font-size: 9px; color: #555; line-height: 1.3; overflow: hidden;",
                "{props.preset.description}"
            }
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Returns inline CSS for a preset card based on active state.
fn preset_card_style(active: bool) -> &'static str {
    if active {
        "
            padding: 8px;
            background: #1a1a3e;
            border: 1px solid #6c63ff;
            border-radius: 6px;
            cursor: pointer;
        "
    } else {
        "
            padding: 8px;
            background: #131320;
            border: 1px solid #1e1e36;
            border-radius: 6px;
            cursor: pointer;
        "
    }
}

/// Returns inline CSS for the full-width Custom card.
fn custom_card_style(active: bool) -> &'static str {
    if active {
        "
            padding: 8px 10px;
            background: #1a1a3e;
            border: 1px solid #6c63ff;
            border-radius: 6px;
            cursor: pointer;
        "
    } else {
        "
            padding: 8px 10px;
            background: #131320;
            border: 1px solid #1e1e36;
            border-radius: 6px;
            cursor: pointer;
        "
    }
}
