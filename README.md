<p align="center">
  <img src="docs/longbow_logo_full.png" />
</p>

<p align="center">
  <a href="https://github.com/cullenwerks/ARST-RUST/actions/workflows/ci.yml">
    <img src="https://github.com/cullenwerks/ARST-RUST/actions/workflows/ci.yml/badge.svg" alt="CI status" />
  </a>
  <a href="https://github.com/cullenwerks/ARST-RUST/actions/workflows/build.yml">
    <img src="https://github.com/cullenwerks/ARST-RUST/actions/workflows/build.yml/badge.svg" alt="Build status" />
  </a>
</p>

Create and manage Arma Reforger Dedicated Servers on Windows or Linux with this easy to use GUI
tool.

This is a Rust + [Tauri](https://tauri.app/) rewrite of the original C#/WinForms tool. The
backend is Rust; the interface is Svelte + TypeScript running in a webview.

## Features

- Load and save server configuration files
- Powerful mod management with import/export of mod lists to share with your mates
- Easily select the scenario you want to play
- Live server status: address, RCON, join code, player count, and rolling FPS/memory charts
- Streams SteamCMD and server output into an in-app log
- Monitors your server and automatically restarts it if it crashes
- Optional UPnP port forwarding
- Run the dedicated server natively on Windows, natively on Linux, or on Linux-through-WSL from a
  Windows host — Longbow detects which OS it's running on and only offers the targets that
  actually work there

## Requirements

**Windows**
- [WebView2 runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (preinstalled on
  current Windows 11 builds)
- Any dependencies for the Arma Reforger Dedicated Server itself (namely the Microsoft Visual
  C++ Runtime)

**Linux** (Fedora `.rpm`, Debian/Ubuntu `.deb`, or the portable AppImage)
- `webkit2gtk` and `gtk3` (the packages declare these as dependencies, so a package-manager
  install pulls them in automatically)
- SteamCMD is downloaded and managed by Longbow itself, same as on Windows

## Download

Every push to `main` builds and publishes a new [Release](https://github.com/cullenwerks/ARST-RUST/releases)
automatically, tagged `vX.Y.Z-build.N` — grab the installer for your platform from the latest one
(NSIS/MSI for Windows, `.rpm`/`.deb`/AppImage for Linux). The Windows installer isn't
code-signed, so Windows SmartScreen will show an "unrecognized app" warning on first run — click
"More info" → "Run anyway".

## Server Parameters

See [here](docs/PARAMETERS.md) for more information on the parameters in the app.

## Building

Requires [Rust](https://rustup.rs/) and [Node.js](https://nodejs.org/).

```bash
cd app
npm install
npm run tauri dev     # run in development
npm run tauri build   # produce a release build and installer
```

Note that `cargo build` alone does not pick up frontend changes — the web assets are embedded
into the binary at compile time, so use the `npm run tauri` commands above (or re-run
`npm run build` before `cargo build`).

To run the backend test suite:

```bash
cd app/src-tauri
cargo test
```

## Discord

Come discuss with fellow users, seek help etc. on the Discord server
[here](https://discord.gg/BPZmmqAvvu)
