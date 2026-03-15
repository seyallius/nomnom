# nomnom 📥 🍽️

> **gib me URLs** — a blazing-fast, native yt-dlp GUI built with Rust + Dioxus

nomnom wraps [yt-dlp](https://github.com/yt-dlp/yt-dlp) in a slick desktop UI so you can download videos, playlists,
channels, and audio with a single click — no terminal required.

---

## ✨ Features

| Feature                    | Details                                                        |
|----------------------------|----------------------------------------------------------------|
| **8 one-click presets**    | Single · Batch · Playlist · Channel — for both Video and Audio |
| **Quality selector**       | Best · 1080p · 720p · 480p                                     |
| **Smart output templates** | Files auto-sorted by uploader, playlist, date                  |
| **Download archive**       | Never re-download a file you already have                      |
| **Batch mode**             | Feed a `.txt` file with one URL per line                       |
| **Flag panel**             | 30+ toggleable yt-dlp flags, categorised                       |
| **Terminal panel**         | Raw command input for power users                              |
| **Live streaming log**     | Colour-coded stdout/stderr in real-time                        |
| **Stop button**            | Cancels the active download instantly                          |

---

## 📦 Installation

### Pre-built binaries (recommended)

Download the latest release for your platform from the [Releases](../../releases) page:

| Platform    | File                                                       |
|-------------|------------------------------------------------------------|
| **Linux**   | `nomnom_x.x.x_amd64.deb` or `nomnom-x.x.x-x86_64.AppImage` |
| **macOS**   | `nomnom_x.x.x.dmg`                                         |
| **Windows** | `nomnom_x.x.x_x64_en-US.msi`                               |

> **Prerequisite:** `yt-dlp` must be installed and available on your `PATH`.
> See [Installing yt-dlp](#installing-yt-dlp) below.

### Build from source

See [INSTALL.md](./INSTALL.md) for full instructions.

---

## 🔧 Installing yt-dlp

nomnom is a GUI front-end — yt-dlp does the actual downloading.

**Linux / macOS**

```bash
# pip (recommended — gets auto-updates)
pip install -U yt-dlp

# or Homebrew (macOS)
brew install yt-dlp
```

**Windows**

```powershell
# winget
winget install yt-dlp

# or scoop
scoop install yt-dlp
```

Verify it works:

```bash
yt-dlp --version
```

---

## 🚀 Quick start

1. Launch nomnom
2. Pick a **preset** from the sidebar (e.g. *Single Video* or *Audio Playlist*)
3. Paste your URL into the input bar
4. Choose an output folder with **📁**
5. Hit **▶ Download**

The live log shows exactly what yt-dlp is doing, colour-coded by status.

---

## 🗂 Output folder structure

nomnom organises downloads automatically based on source type:

```
Downloads/
├── My Video Title - [ChannelName - Jan 01 2025].mp4        ← Single
├── Playlists/
│   └── @ChannelName/
│       └── PlaylistTitle/
│           ├── 001 - First Video - [Jan 2025].mp4
│           └── 002 - Second Video - [Jan 2025].mp4
└── Channels/
    └── @ChannelName/
        ├── Video One - [Dec 2024].mp4
        └── Video Two - [Nov 2024].mp4
```

---

## 🛠 Building from source

See [INSTALL.md](./INSTALL.md).

---

## 🤝 Contributing

PRs welcome! Please open an issue first to discuss what you'd like to change.

---

## 📄 License

MIT — see [LICENSE](./LICENSE).