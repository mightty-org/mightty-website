---
title: Changelog
---
# Changelog

## Version 0.5.0

Released November 23, 2026. [Download Mightty 0.5.0](https://github.com/mightty-org/mightty/releases/download/v0.5.0/Mightty.dmg), or read the [release notes](https://github.com/mightty-org/mightty/releases/tag/v0.5.0) on GitHub.

### Added

- Inline images sent with OSC 1337, as `imgcat` sends them, in PNG, JPEG, GIF, WebP, HEIC and HEIF. See [Protocols](/docs/protocols/#images).
- Sixel images, from `img2sixel`, `chafa`, ImageMagick, gnuplot and other programs.
- Tab colours. Right-click a tab and choose Tab Colour to mark it with a built-in colour, or choose New Colour… to name your own. `tab-color = Name=#hex` defines colours in the config file, and `set_tab_color` sets one from a key binding. See [Tabs and windows](/docs/tabs-and-windows/#tab-colours).
- Hide Mightty (<kbd>⌘H</kbd>), Hide Others (<kbd>⌥⌘H</kbd>) and Show All in the Mightty menu, with the actions `hide`, `hide_others` and `show_all`.
- Programs can ask whether their pane is visible (`CSI ? 998 n`), and be told when it changes (mode 2033), so they can pause what you can't see.
- Soft reset (DECSTR), the cursor position report with its page (DECXCPR), and the screen size query (`CSI 19 t`).

### Changed

- Shells get `LANG` from your macOS language and region when Mightty is opened without one, as it is from the Dock or the Finder. Editors and pagers no longer show characters like ⌘ as bytes. A `LANG`, `LC_ALL` or `LC_CTYPE` you set yourself is kept.
- Pressing <kbd>e</kbd> on the settings screen opens your editor straight away. It used to start an interactive shell first, which could take seconds.

### Security

- Image formats other than PNG are decoded by macOS in a separate, sandboxed process that can't open your files or reach the network, with limits on image size and decoding time.

## Version 0.4.0

Released November 20, 2026. [Download Mightty 0.4.0](https://github.com/mightty-org/mightty/releases/download/v0.4.0/Mightty.dmg), or read the [release notes](https://github.com/mightty-org/mightty/releases/tag/v0.4.0) on GitHub.

### Added

- The standard Mac text-editing keys work at the prompt. <kbd>⌘←</kbd> and <kbd>⌘→</kbd> move to the start or end of the line, <kbd>⌥←</kbd> and <kbd>⌥→</kbd> move by word, <kbd>⌘⌫</kbd> deletes to the start of the line, <kbd>⌘⌦</kbd> deletes to the end, and <kbd>⌥⌦</kbd> deletes the next word. They are default key bindings and can be changed.
- The command being typed at a prompt can be edited like text in a text field. <kbd>⇧←</kbd> and <kbd>⇧→</kbd> select it by character, with <kbd>⌥</kbd> by word and with <kbd>⌘</kbd> to the start or end. Typing or pasting replaces the selection, and <kbd>⌘X</kbd> cuts it. The new `cut_to_clipboard` action is bound to <kbd>⌘X</kbd>.
- Copied text also goes on the clipboard as HTML, with its colours, bold, italic, underlines and strikethrough. Apps that paste rich text, such as Notes, Mail and Slack, keep how the text looked, and apps that paste plain text still get plain text. `copy-with-colors` turns this off.
- `selection-word-chars` sets which characters end a word when you double-click.
- Programs can ask whether the theme is dark or light (`CSI ? 996 n`), and be told when it changes (mode 2031).
- Programs can be sent the window size as input whenever it changes (mode 2048), in order with the keys typed.
- Programs can ask which terminal capabilities Mightty supports (XTGETTCAP) and read the current text style, cursor shape and scroll region (DECRQSS). Neovim, for example, now detects true colour without `COLORTERM` being set.

### Changed

- In zsh and bash, the shell integration marks where the typed command begins, so selecting with the keyboard stops at the end of the prompt.
- <kbd>Home</kbd> and <kbd>End</kbd> move to the start and end of the line in zsh.

## Version 0.3.0

Released November 17, 2026. [Download Mightty 0.3.0](https://github.com/mightty-org/mightty/releases/download/v0.3.0/Mightty.dmg), or read the [release notes](https://github.com/mightty-org/mightty/releases/tag/v0.3.0) on GitHub.

### Added

- Clicking in the command being typed at a prompt moves the cursor there. `cursor-click-to-move` turns this off.
- Delete and Forward Delete with part of the command selected delete that part.
- A command that runs for 10 seconds or more marks its pane as needing you when it finishes, if you aren't looking at that pane, and posts a notification if you are in another app. The notification says whether the command succeeded and how long it took. `notify-on-command-finish` and `notify-on-command-finish-after` control this.
- <kbd>⌘↑</kbd> and <kbd>⌘↓</kbd> scroll to the previous or next prompt, with the command and its output below it. The action is `jump_to_prompt`, and the command palette has both directions.

### Changed

- A tab is marked with ● when any of its panes needs you. Previously only the window title was.
- Mightty asks for permission to show a count on its Dock icon along with permission for notifications, the first time a pane needs you. macOS hides the count without it.

### Fixed

- Jumping to a pane, with <kbd>⌘⇧A</kbd>, a notification or `mightty ctl focus`, shows its tab if the tab was hidden.

## Version 0.2.0

Released November 12, 2026. [Download Mightty 0.2.0](https://github.com/mightty-org/mightty/releases/download/v0.2.0/Mightty.dmg), or read the [release notes](https://github.com/mightty-org/mightty/releases/tag/v0.2.0) on GitHub.

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

Released November 12, 2026. [Download Mightty 0.1.0](https://github.com/mightty-org/mightty/releases/download/v0.1.0/Mightty.dmg), or read the [release notes](https://github.com/mightty-org/mightty/releases/tag/v0.1.0) on GitHub.

Version 0.1.0 was the first release. The sections below list what went into it, by the date each change was made.

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

- Key bindings with `keybind = TRIGGER=ACTION`.
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
