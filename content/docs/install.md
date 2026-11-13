---
title: Install
summary: Download Mightty, or build it from source.
order: 1
---
# Install

Mightty runs on macOS 12 or later, on Apple Silicon or Intel.

## Download

1. [Download Mightty](https://github.com/mightty-org/mightty/releases/latest/download/Mightty.dmg) and open `Mightty.dmg`.
2. Drag Mightty onto the Applications folder in the window that opens.
3. Eject the disk image, then open Mightty from Applications. The app is signed and notarized by Apple, so it opens like any other app.

### Other versions

The link above always gives the latest release. The [download page](/download/) lists every version with its download links, release notes and checksum.

## Updating

Mightty checks for a new version when it starts and once a day. You can also choose Mightty › **Check for Updates…** at any time.

When a new version is available, click **Install Update**. Mightty downloads it and verifies that it is signed by Mightty's developer and notarized by Apple. It then asks when to install:

- **Install and Restart** installs the update and reopens Mightty. Restarting closes the programs running in Mightty, so it asks first if any are running. Your windows, tabs and splits reopen afterwards.
- **Install on Quit** leaves everything running and installs the update the next time you quit.

The `auto-update` setting controls the automatic check:

| Value | Effect |
|---|---|
| `check` | Tells you when a new version is available. This is the default. |
| `download` | Also downloads and verifies the new version, then asks when to install it. |
| `off` | Checks only when you choose Check for Updates. |

Mightty never installs an update or restarts without asking. Version 0.1.0 has no updater: to move on from it, download the latest version as described above and replace the old copy.

## The `mightty` command

The binary inside the app is also the command-line tool, which provides `mightty ctl`, `mightty config` and `mightty mcp`. To run it from any shell, link it into a folder on your `PATH`:

```sh
ln -s /Applications/Mightty.app/Contents/MacOS/mightty ~/.local/bin/mightty
```

If you built Mightty from source, the app is in `~/Applications` instead. The [command line](/docs/command-line/) page lists every command.

## Building from source

You need the Xcode command line tools (`xcode-select --install`) and Rust, from [rustup.rs](https://rustup.rs).

```sh
git clone https://github.com/mightty-org/mightty.git
cd mightty
scripts/bundle.sh --install
```

The script builds `Mightty.app`, signs it ad hoc so that it runs on your Mac, and copies it to `~/Applications`. To install it somewhere else, pass a folder after `--install`. To update, run the script again. Without `--install`, the app is left at `target/release/Mightty.app`.

To run Mightty without installing it:

```sh
cargo run --release                  # a window with your login shell
cargo run --release -- -e htop       # a window running htop, closed when it exits
```

Everything works this way except notification banners, which macOS shows only for an app bundle.
