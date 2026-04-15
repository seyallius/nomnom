//! output_preset_selector — dropdown + custom template input for output path presets.

use crate::core::presets::OutputPreset;
use dioxus::prelude::*;

// --------------------------------------------- Types ---------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct OutputPresetSelectorProps {
    pub output_preset: Signal<OutputPreset>,
    pub output_dir: Signal<String>,
}

// -------------------------------------------- Public API --------------------------------------------

/// Renders the output-preset dropdown and (when Custom is active) a raw template input.
#[component]
pub fn OutputPresetSelector(props: OutputPresetSelectorProps) -> Element {
    let mut output_preset = props.output_preset;
    let output_dir = props.output_dir;

    // Preview the resolved template so the user sees exactly what yt-dlp will get.
    let preview = use_memo(move || {
        let dir = output_dir.read().clone();
        output_preset
            .read()
            .build_template(&dir)
            .unwrap_or_else(|| format!("{}/… (source default)", dir.trim_end_matches('/')))
    });

    // Local signal for the custom template text input.
    let mut custom_input = use_signal(|| {
        if let OutputPreset::Custom(t) = &*output_preset.read() {
            t.clone()
        } else {
            String::new()
        }
    });

    let is_custom = use_memo(move || matches!(*output_preset.read(), OutputPreset::Custom(_)));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 6px;",

            // ── Label row ──────────────────────────────────────────────────
            div {
                style: "font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em; opacity: 0.7; font-family: monospace;",
                "Output Preset"
            }

            // ── Preset dropdown ────────────────────────────────────────────
            select {
                style: "
                    background: #1a1a2e;
                    border: 1px solid #2a2a4a;
                    border-radius: 4px;
                    color: #e0e0e0;
                    padding: 5px 8px;
                    font-size: 12px;
                    cursor: pointer;
                    width: 100%;
                ",
                onchange: move |evt| {
                    let val = evt.value();
                    let chosen = match val.as_str() {
                        "flat"         => OutputPreset::Flat,
                        "by_uploader"  => OutputPreset::ByUploader,
                        "by_year"      => OutputPreset::ByYear,
                        "playlist_tree"=> OutputPreset::PlaylistTree,
                        "channel_tree" => OutputPreset::ChannelTree,
                        "custom"       => OutputPreset::Custom(custom_input.read().clone()),
                        _              => OutputPreset::Auto,
                    };
                    output_preset.set(chosen);
                },

                // Render one <option> per preset variant.
                for preset in OutputPreset::all() {
                    {
                        let key = preset_key(&preset);
                        let label = format!("{} {}", preset.icon(), preset.label());
                        let selected = *output_preset.read() == preset;
                        rsx! {
                            option {
                                value: "{key}",
                                selected: selected,
                                "{label}"
                            }
                        }
                    }
                }
                // Custom option appended separately (not in `all()`)
                option {
                    value: "custom",
                    selected: *is_custom.read(),
                    "✏️  Custom Template"
                }
            }

            // ── Custom template input (only visible when Custom is selected) ──
            if *is_custom.read() {
                input {
                    r#type: "text",
                    placeholder: "/path/to/%(uploader)s/%(title)s.%(ext)s",
                    value: "{custom_input.read()}",
                    style: "
                        background: #101010;
                        border: 1px solid #4a4a8a;
                        border-radius: 4px;
                        color: #e0e0e0;
                        padding: 5px 8px;
                        font-size: 11px;
                        font-family: monospace;
                        width: 100%;
                        box-sizing: border-box;
                    ",
                    oninput: move |evt| {
                        let val = evt.value();
                        custom_input.set(val.clone());
                        output_preset.set(OutputPreset::Custom(val));
                    },
                }
            }

            // ── Template preview ───────────────────────────────────────────
            div {
                style: "
                    font-size: 10px;
                    font-family: monospace;
                    color: #888;
                    background: #0d0d1a;
                    border-radius: 3px;
                    padding: 4px 8px;
                    word-break: break-all;
                    white-space: pre-wrap;
                ",
                "→ {preview.read()}"
            }
        }
    }
}

// --------------------------------------------- Internal Helpers ---------------------------------------------

/// Maps an [`OutputPreset`] to its stable HTML `<option value>` string.
fn preset_key(preset: &OutputPreset) -> &'static str {
    match preset {
        OutputPreset::Auto => "auto",
        OutputPreset::Flat => "flat",
        OutputPreset::ByUploader => "by_uploader",
        OutputPreset::ByYear => "by_year",
        OutputPreset::PlaylistTree => "playlist_tree",
        OutputPreset::ChannelTree => "channel_tree",
        OutputPreset::Custom(_) => "custom",
    }
}
