# nomnom 🍽️

> feeds it URLs. spits out videos. no questions asked.

A desktop GUI wrapper for [yt-dlp](https://github.com/yt-dlp/yt-dlp) built with [Dioxus](https://dioxus.rs/) in Rust.  
No more memorizing flags. No more copy-pasting commands. Just paste, click, eat.

![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)
![Dioxus](https://img.shields.io/badge/dioxus-0.6-purple?style=flat-square)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

---

## What is this?

`nomnom` is a point-and-click frontend for `yt-dlp`.  
Instead of typing (and forgetting) flags every time, you:

1. Paste a URL
2. Pick a preset **or** toggle individual flags
3. Choose where to save
4. Hit download

That's it. It streams the output live so you can watch it nom.

---

## Prerequisites

You need `yt-dlp` installed and available in your `PATH`.

```bash
# macOS
brew install yt-dlp

# Linux
pip install yt-dlp
# or grab the binary: https://github.com/yt-dlp/yt-dlp/releases

# Windows
winget install yt-dlp
````

Then clone and run:

```bash
git clone https://github.com/seyallius/nomnom
cd nomnom
cargo run --release
```

---

## Features

| Thing               | Details                                                                                        |
|---------------------|------------------------------------------------------------------------------------------------|
| **Presets**         | One-click configs for common tasks (Best Video, Audio Only, Playlist, Subtitles, Full Archive) |
| **Flag toggles**    | Every major yt-dlp flag as a button, grouped by category                                       |
| **Command preview** | See the exact command that will run, live, before you hit download                             |
| **Folder picker**   | Native OS folder dialog — no typing paths                                                      |
| **Terminal panel**  | Drop down to raw `yt-dlp` commands when you need full control                                  |
| **Live log**        | Streamed stdout/stderr with color-coded lines                                                  |
| **Async**           | Downloads run in the background; UI never freezes                                              |

---

## Project Structure

```text
nomnom/
├── Cargo.toml
└── src/
├── main.rs             # window config, app launch
├── app.rs              # root component, all shared state lives here
│
├── core/               # pure logic, no UI
│ ├── mod.rs
│ ├── flags.rs          # every yt-dlp flag definition
│ ├── presets.rs        # preset collections (bundles of flags)
│ └── runner.rs         # spawns yt-dlp, streams output
│
└── components/         # UI components, one per concern
├── mod.rs
├── flag_panel.rs       # left sidebar: flag toggle buttons
└── output_log.rs       # bottom: live log output
├── preset_panel.rs     # left sidebar: preset cards
├── terminal_panel.rs   # middle: raw command input
├── url_bar.rs          # top: URL input, folder picker, download button
```

### How data flows

```text
app.rs  (owns all Signals)
│
├──▶ preset_panel reads/writes active_preset, active_flags
├──▶ flag_panel reads/writes active_flags
├──▶ url_bar reads/writes url, output_dir
│    reads active_flags, built_command
│    writes log_lines, is_running
├──▶ terminal_panel reads/writes is_running, log_lines
└──▶ output_log reads log_lines
```

All state is `Signal<T>` defined in `app.rs` and passed down as props.  
No global state, no context magic. If you can read props, you can read the app.

---

## How to Extend

This section is the whole point of this README.  
Each extension has exactly one place to touch.

---

### Add a new flag

Open `src/core/flags.rs`.  
Find the `all_flags()` function. Add your flag to the right category:

```rust
Flag {
    flag: "--sponsorblock-remove all",
    label: "Skip Sponsors",
    description: "Remove sponsor segments via SponsorBlock",
    category: FlagCategory::Misc,
},
```

That's it. It will automatically appear as a toggle button in the UI under the correct category group.  
No component changes needed.

---

### Add a new flag category

Still in `src/core/flags.rs`:

1. Add a variant to the `FlagCategory` enum:

```rust
pub enum FlagCategory {
    // ...existing...
    Chapters, // 👈 new
}
```

2. Give it a label in the `label()` method:

```rust
FlagCategory::Chapters => "📑 Chapters",
```

3. Add it to the ordered list in `src/components/flag_panel.rs`:

```rust
let categories: Vec<FlagCategory> = vec![
    // ...existing...
    FlagCategory::Chapters, // 👈 add it where you want it to appear
];
```

4. Tag your new flags with `category: FlagCategory::Chapters` in `flags.rs`.

Done. The panel renders it as a new group automatically.

---

### Add a new preset

Open `src/core/presets.rs`.  
Add a new `Preset` to the `all_presets()` vec:

```rust
Preset {
    id: "music_video", // unique, used for equality checks
    label: "Music Video",
    description: "Best video + audio, embed thumbnail, no playlist",
    icon: "🎸",
    flag_keys: vec![
        "--add-metadata",
        "--add-thumbnail",
        "--no-playlist",
        "--merge-output-format mp4",
    ],
},
```

`flag_keys` are matched against `Flag::flag` strings defined in `flags.rs`.  
The preset card appears in the sidebar automatically. No UI code to touch.

> **Tip:** if you reference a flag key that doesn't exist in `all_flags()`,  
> it is silently skipped. `resolve_preset_flags()` uses `filter_map`.

---

### Change the default preset on startup

In `src/core/presets.rs`, `default_preset()` returns the first preset:

```rust
pub fn default_preset() -> Preset {
    all_presets().into_iter().next().unwrap()
}
```

Change `.next()` to `.find(|p| p.id == "audio_only")` to boot into a different preset:

```rust
pub fn default_preset() -> Preset {
    all_presets()
    .into_iter()
    .find(|p| p.id == "audio_only")
    .unwrap()
}
```

---

### Change how the command is built

Open `src/core/runner.rs`.  
`build_command_string()` builds the preview string.  
`run_download()` builds the actual `args: Vec<String>` passed to the subprocess.

Both live next to each other. If you want to add a fixed flag that always runs  
(e.g., always pass `--no-warnings`), add it to the args vec in `run_download()`:

```rust
let mut args: Vec<String> = vec![
    "-o".to_string(),
    output_template,
    "--no-warnings".to_string(), // 👈 always present
];
```

---

### Add a new UI panel

1. Create `src/components/your_panel.rs`
2. Define a `#[component]` function with a `Props` struct
3. Register it in `src/components/mod.rs`:

```rust
pub mod your_panel;
```

4. Import and place it in `src/app.rs` where you want it in the layout.  
   Pass whatever `Signal<T>` it needs as props — they're all defined at the top of `App()`.

The layout in `app.rs` is plain flexbox divs. The left sidebar and right column  
are clearly commented. Drop your component where it makes visual sense.

---

### Persist settings between sessions

Nothing is persisted yet. A good place to add it:

- **Read on startup:** inside `App()` in `app.rs`, replace the `use_signal(|| ...)` defaults  
  with values loaded from a config file.
- **Write on change:** use `use_effect` in `app.rs` to watch signals and serialize on change.
- Config format suggestion: `~/.config/nomnom/config.json` via `dirs::config_dir()` (already in deps).

The `Flag`, `Preset`, and their fields all derive `Serialize` / `Deserialize` already.

---

### Change the output filename template

In `src/core/runner.rs`, find:

rust
let output_template = format!("{}/%(title)s.%(ext)s", output_dir.trim_end_matches('/'));

`%(title)s`, `%(uploader)s`, `%(upload_date)s`, `%(id)s` etc. are yt-dlp  
[output template fields](https://github.com/yt-dlp/yt-dlp#output-template).  
Change the format string to whatever naming convention you want.  
A future extension: expose this as a text input in `url_bar.rs`.

---

## Architecture Philosophy

- **`core/`** knows nothing about Dioxus. It is plain Rust logic.  
  You can test it, reuse it, or swap the frontend without touching it.

- **`components/`** know nothing about `yt-dlp` internals.  
  They render state and fire events. That's all.

- **`app.rs`** is the glue. It owns state and wires components together.  
  If something feels hard to connect, the answer is usually: lift the signal to `app.rs`.

- **Signals flow down, events bubble up** via callbacks or by passing writable signals as props.

---

## License

[MIT](./LICENSE). eat freely.
