//! preset_panel.rs - Preset selector cards in the sidebar.
//!
//! This component renders clickable preset cards that allow users to
//! quickly select pre-configured flag combinations.
//!
//! # Visual Layout
//!
//! ```text
//! ⚡ PRESETS
//! ┌──────────────────────┐
//! │ 🎬 Best Video        │ ← Active (purple border)
//! │ Best quality video   │
//! └──────────────────────┘
//! ┌──────────────────────┐
//! │ 🎵 Audio Only (MP3)  │ ← Inactive
//! │ Extract audio as MP3 │
//! └──────────────────────┘
//! ...
//! ┌──────────────────────┐
//! │ 🔧 Custom            │ ← Clears all flags
//! │ Pick flags manually  │
//! └──────────────────────┘
//! ```

use crate::core::flags::Flag;
use crate::core::presets::{Preset, all_presets, resolve_preset_flags};
use dioxus::prelude::*;

// -------------------------------------------- Types --------------------------------------------

/// Props for the [`PresetPanel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PresetPanelProps {
    /// Signal holding the currently active preset.
    /// `None` indicates "Custom" mode.
    pub active_preset: Signal<Option<Preset>>,
    /// Signal holding active flags (updated when preset is selected).
    pub active_flags: Signal<Vec<Flag>>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders clickable preset cards for quick flag configuration.
///
/// Displays all built-in presets plus a "Custom" option that clears
/// all flags for manual selection.
///
/// # Arguments
///
/// * `props` - Contains `active_preset` and `active_flags` signals.
///
/// # Behavior
///
/// - Clicking a preset resolves its flags and updates `active_flags`
/// - Clicking "Custom" sets `active_preset` to `None` and clears flags
/// - Active preset shows purple border, inactive shows dark border
///
/// # Example
///
/// ```rust,ignore
/// let active_preset = use_signal(|| Some(default_preset()));
/// let active_flags = use_signal(Vec::<Flag>::new);
///
/// rsx! {
///     PresetPanel { active_preset, active_flags }
/// }
/// ```
#[component]
pub fn PresetPanel(props: PresetPanelProps) -> Element {
    let mut active_preset = props.active_preset;
    let mut active_flags = props.active_flags;

    rsx! {
        div {
            h3 {
                style: "
                    margin: 0 0 10px 0;
                    font-size: 12px;
                    text-transform: uppercase;
                    letter-spacing: 1px;
                    color: #6c63ff;
                ",
                "⚡ Presets"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 6px;",

                for preset in all_presets() {
                    {
                        let is_active = active_preset.read().as_ref().map(|p| p.id) == Some(preset.id);
                        let preset_clone = preset.clone();

                        rsx! {
                            div {
                                key: "{preset.id}",
                                style: preset_card_style(is_active),
                                onclick: move |_| {
                                    let flags = resolve_preset_flags(&preset_clone);
                                    active_flags.set(flags);
                                    active_preset.set(Some(preset_clone.clone()));
                                },

                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    span { "{preset.icon}" }
                                    span {
                                        style: "font-size: 13px; font-weight: bold;",
                                        "{preset.label}"
                                    }
                                }
                                div {
                                    style: "font-size: 11px; color: #888; margin-top: 3px;",
                                    "{preset.description}"
                                }
                            }
                        }
                    }
                }

                // Custom option
                {
                    let is_custom = active_preset.read().is_none();
                    rsx! {
                        div {
                            style: preset_card_style(is_custom),
                            onclick: move |_| {
                                active_preset.set(None);
                                active_flags.set(vec![]);
                            },
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { "🔧" }
                                span {
                                    style: "font-size: 13px; font-weight: bold;",
                                    "Custom"
                                }
                            }
                            div {
                                style: "font-size: 11px; color: #888; margin-top: 3px;",
                                "Pick flags manually below"
                            }
                        }
                    }
                }
            }
        }
    }
}

// -------------------------------------------- Internal Helpers --------------------------------------------

/// Returns inline CSS for a preset card based on active state.
///
/// # Arguments
///
/// * `active` - Whether this preset is currently selected.
///
/// # Returns
///
/// A static CSS string for the card style.
fn preset_card_style(active: bool) -> &'static str {
    if active {
        "
            padding: 8px 10px;
            background: #1e1e4a;
            border: 1px solid #6c63ff;
            border-radius: 6px;
            cursor: pointer;
            transition: all 0.15s;
        "
    } else {
        "
            padding: 8px 10px;
            background: #161626;
            border: 1px solid #2a2a4a;
            border-radius: 6px;
            cursor: pointer;
            transition: all 0.15s;
        "
    }
}
