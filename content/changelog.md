---
title: Changelog
---
# Changelog

All notable changes to Mightty are listed here, newest first. Every version can be downloaded from the [releases page](https://github.com/mightty-org/mightty/releases).

## Version 0.2.0

Released November 12, 2026.

### Added

- About Mightty, in the app menu and the command palette, shows the version and the licences of Mightty and of the themes and font it includes.
- Check for Updates, in the app menu and the command palette. Mightty also checks when it starts and once a day.
- Installing updates in place. Install Update downloads and verifies the new version, then offers Install and Restart or Install on Quit. Mightty never installs or restarts without asking.
- `auto-update`, with the values `check`, `download` and `off`, controls the automatic check.

### Changed

- Releases are started from a workflow that sets the version, builds, signs and notarizes the app, and publishes the release with notes generated from the merged pull requests.

### Security

- An update is installed only if its checksum matches the release, macOS finds it signed by Mightty's Developer ID team and notarized by Apple, and it is Mightty in the version that was offered. It is verified when it is downloaded and again when it is installed.
- An update waits to be installed in a folder that only the current user can access, and its archive is inspected before it is unpacked.

## Version 0.1.0

Released November 12, 2026. This was the first release. The sections below list what went into it, by date.

## November 12, 2026

### Added

- Release builds for Apple Silicon and Intel in a single app, signed and notarized by Apple, and delivered as a disk image.

### Changed

- `mightty ctl run` and `mightty ctl spawn` pass several arguments after `--` as separate words, so `-- rm "my file"` removes one file. A single argument is still passed to the shell as written.
- `mightty ctl run` without `--pane` closes the hidden shell it starts when the command finishes.
- `libmightty` accepts kitty images only when they are sent inline. It no longer reads files or shared memory, in line with its documented guarantee of no I/O.

### Performance

- Scrollback uses less memory. When a pane's output pauses, each scrollback line releases the space it does not use. With eight panes of `git log` output, memory use fell from 86 MB to 50 MB.
- Unicode-heavy output is about 6% faster.
- Closing a pane or a window releases all of its memory. Previously the drawing layer and the window were kept.
- Sequences that repeat a character or advance by tab stops thousands of times are handled in time proportional to the text they produce.
- Scrolling is no longer slow when many images are placed relative to one another.

### Fixed

- An error in the config file no longer blocks the app behind a modal alert. The error is shown on a sheet and the app keeps running.
- Answering a close confirmation on one window no longer closes another window that also has a confirmation open.
- Open Config File (<kbd>⌘,</kbd>) now works when the login shell is fish.
- A paste confirmed on a sheet is encoded for the program's paste mode at the time of confirmation, not at the time the sheet appeared.
- Prompt marks and images below a scroll region that starts at the top of the screen keep their positions when the region scrolls.
- bash reports the correct command when `HISTCONTROL` keeps the command out of history.
- A Mightty that is too busy to accept a connection no longer has its control socket removed by another `mightty` command.

### Security

- A PNG with a second header can no longer bypass the image size limit.
- Escape sequences over 1 MiB are discarded. Previously they were truncated and still processed.
- A folder name containing control characters can no longer cause a pane's shell-integration secret to be displayed.
- Images sent by file name are validated before the file is read in full.
- Loading a damaged font is bounded in both memory and time.
- Bidirectional formatting characters are removed from notification text.
- The MCP server uses the pane ID from its environment only when the control socket from the same environment is in use.

## November 10, 2026

### Added

- A settings screen that runs in a terminal tab, opened with <kbd>⌘,</kbd>. It completes values as you type, previews themes and fonts before you choose one, and writes to the config file without changing your comments.
- The 819 themes from iTerm2-Color-Schemes, built in.
- Automatic reload of the config file when it is saved, and Open Config File, which opens it in `$VISUAL` or `$EDITOR`.
- A tab bar drawn by Mightty in every title bar style. Tabs can be dragged to reorder them, into a new window, or onto another window, and renamed, moved or closed from a context menu.
- `macos-titlebar-style`, with the values `native`, `transparent`, `tabs` and `hidden`.
- Undo for closing: <kbd>⌘Z</kbd> restores a closed tab, pane or window with its programs still running, within `undo-timeout`. After that, <kbd>⌘⇧T</kbd> reopens it with new shells.
- A quick terminal that slides in from the top of the screen, and `global:` key bindings that work in any app.
- A command palette, opened with <kbd>⌘⇧P</kbd>.
- A scrollbar, shown while scrolling.
- Rectangular selection with <kbd>⌥</kbd>-drag.
- `bell-features`: the system sound, a custom sound, a Dock bounce, 🔔 in the title, or a flash of the pane's border.
- Secure Keyboard Entry, enabled automatically at password prompts, and `password-mask`, which shows a `*` for each character typed at one.
- `window-colorspace = display-p3`.
- Support for programs setting and reading colours (OSC 4, 10 to 12, 104, 110 to 112), setting the font family (OSC 50), reporting progress (OSC 9;4) and setting the pointer shape (OSC 22), and for the XTVERSION and DECRQM queries.
- Kitty graphics: animation, relative placements, Unicode placeholders, and images passed through shared memory.
- Underline styles (double, curly, dotted and dashed) and underline colours.
- Character widths from Unicode 17. An emoji written with a variation selector occupies two cells. Programs can turn this off with mode 2027, and `grapheme-width = legacy` turns it off globally.

### Changed

- The app is named Mightty in the menu bar, the Dock and Finder.
- Translucent windows show a background colour set by a program.
- The quick terminal animates out as well as in, and `quick-terminal-size` sets its size.

### Security

- The remaining ways for output to imitate a prompt or a command are closed, and limits on sizes and signals are stricter.

## November 9, 2026

### Added

- The kitty graphics protocol.
- Symbols Nerd Font, built in as a fallback font.
- Scrolling when a selection is dragged past the top or bottom edge of a pane.

### Security

- Protection against hostile output: imitated prompts and commands, injected input, oversized data, and output designed to stall the terminal.

## November 4, 2026

### Added

- The kitty keyboard protocol.
- Events on the control socket, including panes opening and closing and commands finishing.

## November 3, 2026

### Added

- Key bindings with `keybind = TRIGGER=ACTION`, in Ghostty's format.
- Appearance settings: background opacity and blur, padding, cursor style and cell size.
- Font settings: fallback fonts, separate bold and italic families, OpenType features and variable font axes.
- Themes that follow the macOS light and dark appearance, and `config-file` for reading another file.
- Clipboard access for programs (OSC 52), and mouse settings.
- Reloading settings without restarting.
- Support in Mightty's rasterizer for CFF and variable fonts and colour emoji, and synthesized bold and italic for families without them.
- The app icon.

## November 2, 2026

### Added

- Tabs and splits.
- Search through the screen and scrollback, with <kbd>⌘F</kbd>.
- Links: hold <kbd>⌘</kbd> to see the destination and <kbd>⌘</kbd>-click to open it.
- Attention tracking for coding agents: an agent that is waiting is marked on its pane and counted on the Dock icon.
- The app bundle, and notifications.
- Shell integration for bash and fish.
- Restoring windows, tabs and splits at launch.
- Input method support.
- A confirmation before closing a pane, a window or the app while a program is running.
- `macos-option-as-alt`.
- Mightty's own font rasterizer for TrueType fonts, enabled with `font-backend = mightty`.

### Performance

- Lower memory use, both idle and under load.
- Output is drawn at the display's refresh rate. After a key press, the frame is drawn as soon as the echo arrives instead of at the next refresh.

### Fixed

- Emoji sequences and combining marks occupy one cell.

## November 1, 2026

### Added

- A config file, themes, colour settings and scrollback settings.

## October 30, 2026

### Added

- The first version: a terminal emulator written in Rust with no third-party crates, rendered with Metal, with a control socket and an MCP server.
- Settings given as `--name=value` flags.
