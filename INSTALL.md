# Installing nomnom

This guide covers building nomnom from source on **Linux**, **macOS**, and **Windows**.

---

## Prerequisites

### 1. Rust toolchain

Install Rust via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # rustc 1.78+ required
cargo --version
```

### 2. yt-dlp

nomnom is a GUI wrapper — yt-dlp must be on your `PATH`.

| Platform         | Command                 |
|------------------|-------------------------|
| Linux / macOS    | `pip install -U yt-dlp` |
| macOS (Homebrew) | `brew install yt-dlp`   |
| Windows (winget) | `winget install yt-dlp` |
| Windows (scoop)  | `scoop install yt-dlp`  |

### 3. Platform-specific dependencies

#### Linux

Install the WebView and GUI system libraries:

```bash
# Debian / Ubuntu
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf \
  libxdo-dev

# Fedora / RHEL
sudo dnf install -y \
  webkit2gtk4.1-devel \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch Linux
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  patchelf
```

#### macOS

Xcode Command Line Tools (provides `clang`, `libc`, etc.):

```bash
xcode-select --install
```

No additional packages needed — macOS includes WebKit natively.

#### Windows

Install the [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
and select the **"Desktop development with C++"** workload.

WebView2 is bundled with Windows 10/11 — no extra install needed.

---

## Building

### Clone the repo

```bash
git clone https://github.com/YOUR_USERNAME/nomnom.git
cd nomnom
```

### Development build (fast compile, debug symbols)

```bash
cargo run
```

### Release build (optimised, smaller binary)

```bash
cargo build --release
# binary at: target/release/nomnom  (or nomnom.exe on Windows)
```

---

## Packaging for distribution

nomnom uses [Tauri's
`cargo-tauri`](https://tauri.app/reference/cli/) / [Dioxus CLI](https://dioxuslabs.com/learn/0.6/CLI) for packaging.

### Install the Dioxus CLI

```bash
cargo install dioxus-cli
```

### Bundle for your current platform

```bash
dx bundle --release
```

Output:

- **Linux:**   `target/dx/nomnom/release/bundle/deb/*.deb` and `*.AppImage`
- **macOS:**   `target/dx/nomnom/release/bundle/dmg/*.dmg`
- **Windows:** `target/dx/nomnom/release/bundle/msi/*.msi`

### Cross-compile notes

Cross-compilation is not officially supported by Dioxus desktop bundling.
Use the provided [GitHub Actions workflow](./.github/workflows/release.yml)
to build on native runners for each platform (recommended for releases).

---

## Running the AppImage (Linux)

```bash
chmod +x nomnom-*.AppImage
./nomnom-*.AppImage
```

---

## Installing the .deb (Linux)

```bash
sudo dpkg -i nomnom_*_amd64.deb
# then launch:
nomnom
```

---

## Installing the .dmg (macOS)

1. Open `nomnom_*.dmg`
2. Drag **nomnom.app** to your **Applications** folder
3. First launch: right-click → **Open** (to bypass Gatekeeper on first run)

---

## Installing the .msi (Windows)

1. Double-click `nomnom_*_x64_en-US.msi`
2. Follow the installer wizard
3. Launch **nomnom** from the Start Menu

---

## Troubleshooting

### `yt-dlp: command not found`

nomnom spawns yt-dlp via your login shell (`$SHELL` on Unix, `cmd` on Windows).
Make sure yt-dlp is on the `PATH` for your **login shell**, not just your interactive shell.

Quick test:

```bash
# Unix
bash -i -c "yt-dlp --version"

# Windows (PowerShell)
yt-dlp --version
```

### Linux: `error: failed to run custom build command for webkitgtk`

You're missing the WebKit development libraries. Re-run the apt/dnf/pacman command
in the [Prerequisites](#prerequisites) section.

### macOS: app is damaged / can't be opened

Run this once to strip the quarantine attribute:

```bash
xattr -cr /Applications/nomnom.app
```

### Windows: VCRUNTIME missing

Install the [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe).