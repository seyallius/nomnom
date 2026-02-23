//! flag_panel.rs - Toggleable flag buttons grouped by category

use crate::core::flags::{all_flags, Flag, FlagCategory};
use dioxus::prelude::*;
use std::collections::HashMap;

// -------------------------------------------- Public Functions --------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FlagPanelProps {
    pub active_flags: Signal<Vec<Flag>>,
}

/// Grid of toggle buttons for each yt-dlp flag, grouped by category
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

// -------------------------------------------- Private Helper Functions --------------------------------------------

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
