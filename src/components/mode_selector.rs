//! mode_selector.rs - Download type, source, and quality configuration UI.
//!
//! This component renders three rows of pill-style buttons:
//!
//! ```text
//! [📹 Video]  [🎵 Audio]
//! ─────────────────────────────────────────────
//! [🔗 Single URL] [📄 Batch File] [📋 Playlist] [📺 Channel]
//! ─────────────────────────────────────────────
//! Quality: [Best] [1080p ✓] [720p] [480p]     ← hidden for Audio
//! ```
//!
//! Clicking any pill updates the corresponding signal and clears
//! `active_preset` to signal "Custom" mode.

use crate::core::{
    download_mode::{DownloadSource, DownloadType, Quality},
    presets::Preset,
};
use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`ModeSelector`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ModeSelectorProps {
    /// Currently selected download type.
    pub download_type: Signal<DownloadType>,
    /// Currently selected download source.
    pub download_source: Signal<DownloadSource>,
    /// Currently selected video quality.
    pub quality: Signal<Quality>,
    /// Active preset — set to `None` when user makes a manual selection.
    pub active_preset: Signal<Option<Preset>>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders pill-button selectors for download type, source, and quality.
///
/// All three rows update their corresponding signals on click. Any manual
/// selection sets `active_preset` to `None` (Custom mode) so the sidebar
/// highlights the Custom card.
///
/// # Arguments
///
/// * `props` — Signals for type, source, quality, and the active preset.
#[component]
pub fn ModeSelector(props: ModeSelectorProps) -> Element {
    let mut download_type = props.download_type;
    let mut download_source = props.download_source;
    let mut quality = props.quality;
    let mut active_preset = props.active_preset;

    let is_video = *download_type.read() == DownloadType::Video;

    rsx! {
        div {
            style: "
                background: #111122;
                border: 1px solid #2a2a4a;
                border-radius: 8px;
                padding: 12px 14px;
                display: flex;
                flex-direction: column;
                gap: 10px;
            ",

            // ── Row 1: Type tabs ────────────────────────────────────────
            div {
                style: "display: flex; gap: 6px;",

                for variant in [DownloadType::Video, DownloadType::Audio] {
                    {
                        let active = *download_type.read() == variant;
                        let label = variant.label();
                        let variant_clone = variant.clone();
                        rsx! {
                            button {
                                key: "{label}",
                                style: type_tab_style(active),
                                onclick: move |_| {
                                    download_type.set(variant_clone.clone());
                                    active_preset.set(None);
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }

            // ── Row 2: Source pills ─────────────────────────────────────
            div {
                style: "display: flex; gap: 6px; flex-wrap: wrap;",

                for variant in [
                    DownloadSource::Single,
                    DownloadSource::Batch,
                    DownloadSource::Playlist,
                    DownloadSource::Channel,
                ] {
                    {
                        let active = *download_source.read() == variant;
                        let icon = variant.icon();
                        let label = variant.label();
                        let variant_clone = variant.clone();
                        rsx! {
                            button {
                                key: "{label}",
                                style: source_pill_style(active),
                                onclick: move |_| {
                                    download_source.set(variant_clone.clone());
                                    active_preset.set(None);
                                },
                                span { style: "margin-right: 4px;", "{icon}" }
                                "{label}"
                            }
                        }
                    }
                }
            }

            // ── Row 3: Quality pills (video only) ───────────────────────
            if is_video {
                div {
                    style: "display: flex; align-items: center; gap: 6px;",

                    span {
                        style: "font-size: 11px; color: #666; letter-spacing: 0.5px; min-width: 50px;",
                        "Quality"
                    }

                    for variant in [Quality::Best, Quality::HD1080, Quality::HD720, Quality::SD480] {
                        {
                            let active = *quality.read() == variant;
                            let label = variant.label();
                            let variant_clone = variant.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: quality_pill_style(active),
                                    onclick: move |_| {
                                        quality.set(variant_clone.clone());
                                        active_preset.set(None);
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Returns inline CSS for a type tab (Video / Audio) based on active state.
fn type_tab_style(active: bool) -> &'static str {
    if active {
        "
            padding: 8px 20px;
            background: #6c63ff;
            color: white;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: bold;
            font-family: monospace;
            letter-spacing: 0.5px;
        "
    } else {
        "
            padding: 8px 20px;
            background: transparent;
            color: #666;
            border: 1px solid #2a2a4a;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-family: monospace;
            letter-spacing: 0.5px;
        "
    }
}

/// Returns inline CSS for a source pill based on active state.
fn source_pill_style(active: bool) -> &'static str {
    if active {
        "
            padding: 6px 12px;
            background: #1e1e4a;
            color: #b0a8ff;
            border: 1px solid #6c63ff;
            border-radius: 20px;
            cursor: pointer;
            font-size: 12px;
            font-family: monospace;
        "
    } else {
        "
            padding: 6px 12px;
            background: transparent;
            color: #555;
            border: 1px solid #2a2a4a;
            border-radius: 20px;
            cursor: pointer;
            font-size: 12px;
            font-family: monospace;
        "
    }
}

/// Returns inline CSS for a quality pill based on active state.
fn quality_pill_style(active: bool) -> &'static str {
    if active {
        "
            padding: 4px 10px;
            background: #2a2a4a;
            color: #6c63ff;
            border: 1px solid #6c63ff;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            font-family: monospace;
            font-weight: bold;
        "
    } else {
        "
            padding: 4px 10px;
            background: transparent;
            color: #444;
            border: 1px solid #2a2a4a;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            font-family: monospace;
        "
    }
}
