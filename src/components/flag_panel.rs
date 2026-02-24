//! flag_panel.rs - Toggleable flag buttons grouped by category.
//!
//! This component renders a grid of toggle buttons for each yt-dlp flag,
//! organized by category. Users can click buttons to enable/disable flags.
//!
//! # Visual Layout
//!
//! ```text
//! 🚩 FLAGS
//! ────────────────────
//! 📋 Playlist
//! ┌────────┐ ┌────────┐
//! │ Yes PL │ │ No PL  │ ...
//! └────────┘ └────────┘
//!
//! 🏷️ Metadata
//! ┌────────┐ ┌────────┐
//! │ Add MD │ │ Thumb  │ ...
//! └────────┘ └────────┘
//! ```

/// - Active flags show purple background (`#6c63ff`)
/// - Inactive flags show dark background (`#1a1a2e`)
/// - Click toggles the flag in `active_flags`
use crate::core::flags::{all_flags, Flag, FlagCategory};
use dioxus::prelude::*;


// -------------------------------------------- Types --------------------------------------------

/// Props for the [`FlagPanel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FlagPanelProps {
    /// Signal holding the currently active flags.
    /// The panel reads this to show active state and writes to it on click.
    pub active_flags: Signal<Vec<Flag>>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders the flag selection panel with categorized toggle buttons.
///
/// This component displays all available flags organized by category.
/// Each flag is a clickable button that toggles its presence in `active_flags`.
///
/// # Arguments
///
/// * `props` - Contains `active_flags` signal for reading/writing selections.
///
/// # Styling
///
/// - Categories are displayed with emoji headers and borders
/// - Active flags have purple background
/// - Inactive flags have dark background
/// - Each button has a tooltip with the flag description
///
/// # Example
///
/// ```rust,ignore
/// let active_flags = use_signal(Vec::<Flag>::new);
///
/// rsx! {
///     FlagPanel { active_flags }
/// }
/// ```
#[component]
pub fn FlagPanel(props: FlagPanelProps) -> Element {
    let mut active_flags = props.active_flags;
    let flags = all_flags();

    // Group flags by category preserving order
    let categories: Vec<FlagCategory> = vec![
        FlagCategory::Playlist,
        FlagCategory::Metadata,
        FlagCategory::Format,
        FlagCategory::Subtitles,
        FlagCategory::Audio,
        FlagCategory::Network,
        FlagCategory::Misc,
    ];

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
                "🚩 Flags"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 14px;",

                for category in &categories {
                    {
                        let cat_flags: Vec<Flag> = flags
                            .iter()
                            .filter(|f| &f.category == category)
                            .cloned()
                            .collect();

                        rsx! {
                            div {
                                key: "{category.label()}",

                                div {
                                    style: "
                                        font-size: 11px;
                                        color: #a0a0c0;
                                        margin-bottom: 6px;
                                        border-bottom: 1px solid #2a2a4a;
                                        padding-bottom: 3px;
                                    ",
                                    "{category.label()}"
                                }

                                div {
                                    style: "display: flex; flex-wrap: wrap; gap: 6px;",

                                    for flag in cat_flags {
                                        {
                                            let flag_clone = flag.clone();
                                            let is_active = active_flags.read().contains(&flag);

                                            rsx! {
                                                button {
                                                    key: "{flag.flag}",
                                                    style: flag_btn_style(is_active),
                                                    title: "{flag.description}",
                                                    onclick: move |_| {
                                                        let mut flags_write = active_flags.write();
                                                        if flags_write.contains(&flag_clone) {
                                                            flags_write.retain(|f| f != &flag_clone);
                                                        } else {
                                                            flags_write.push(flag_clone.clone());
                                                        }
                                                    },
                                                    "{flag.label}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// -------------------------------------------- Internal Helpers --------------------------------------------

/// Returns inline CSS for a flag toggle button based on active state.
///
/// # Arguments
///
/// * `active` - Whether the flag is currently enabled.
///
/// # Returns
///
/// A static CSS string for the button style.
fn flag_btn_style(active: bool) -> &'static str {
    if active {
        "
            padding: 4px 8px;
            background: #6c63ff;
            color: white;
            border: 1px solid #8880ff;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            font-family: monospace;
        "
    } else {
        "
            padding: 4px 8px;
            background: #1a1a2e;
            color: #a0a0c0;
            border: 1px solid #2a2a4a;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            font-family: monospace;
        "
    }
}
