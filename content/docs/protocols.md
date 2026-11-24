---
title: Protocols
summary: The escape sequences Mightty supports beyond VT100 and xterm.
description: The terminal protocols Mightty supports: kitty graphics, inline images and Sixel, the kitty keyboard protocol, Unicode 17, styled underlines, OSC 8 links, OSC 52 and more.
order: 9
---
# Protocols

Programs control a terminal by writing escape sequences. Mightty supports the VT100 and xterm sequences that programs depend on, and the newer protocols listed on this page.

## Images

Mightty shows images sent in three ways. However an image arrives, it scrolls with the text and is cleared with it.

**The kitty graphics protocol.**

- Images can be sent inline, or passed as a file, a temporary file or shared memory, in PNG format or as raw pixels.
- Images can be drawn under or over the text, placed relative to each other, and animated, so `kitten icat` plays GIFs.
- Unicode placeholders are supported, which lets images display through tmux.

**Inline images (OSC 1337 `File=`)**, as `imgcat` sends them.

- PNG, JPEG, GIF, WebP, HEIC and HEIF are supported. An image can be sent in one sequence, or in parts with `MultipartFile`, `FilePart` and `FileEnd`.
- `width` and `height` can be given in cells, pixels or as a percentage of the window, and `preserveAspectRatio` and `doNotMoveCursor` are supported.
- Only images marked `inline=1` are shown. A file sent for download is ignored, because output never writes files.

**Sixel**, as `img2sixel`, `chafa -f sixel`, ImageMagick and gnuplot send it.

- Images can have up to 256 colours at a time. They are shown as the data arrives.
- Mode 80 (sixel display mode) and mode 8452 (where the cursor goes after an image) are supported.
- Primary device attributes report Sixel support, and XTSMGRAPHICS reports the number of colours and the largest image size.
- An image is at most as wide as the window and has at most 16.7 million pixels.

Mightty decodes PNG itself. Other formats are decoded by macOS in a separate process that runs in a sandbox and can't open your files or reach the network, so a malformed image can't harm Mightty or your data. Each image has 2 seconds to decode.

## Keyboard

Mightty supports the kitty keyboard protocol with all five of its options. Programs that use it, such as Neovim, Helix, Kakoune and fish, can tell <kbd>Ctrl-I</kbd> from <kbd>Tab</kbd> and <kbd>Esc</kbd> from <kbd>Alt</kbd>, and can see when a key is released.

Input methods are supported. The text being composed is drawn at the cursor.

## Text

- **Widths.** Character widths follow Unicode 17. Emoji sequences and combining marks stay in one cell. Mode 2027 is on by default, and a program that counts widths the older way can turn it off.
- **Underlines.** Single, double, curly, dotted and dashed underlines, in any colour (SGR 58). Editors use these to mark errors and warnings.
- **Links.** OSC 8 hyperlinks, opened with <kbd>⌘</kbd>-click.

## Colours and appearance

| Sequence | What a program can do |
|---|---|
| OSC 4, 10, 11, 12 | Set and read the 16 colours and the default foreground, background and cursor colours. |
| OSC 104, 110 to 112 | Reset those colours. |
| OSC 50 | Set the font family of its pane. The settings screen uses this to preview fonts. |
| OSC 22 | Set the shape of the mouse pointer. |
| OSC 9;4 | Report progress, shown as a thin bar along the top of the pane. |
| `CSI ? 996 n`, mode 2031 | Ask whether the theme is dark or light, and be told when it changes. |
| `CSI ? 998 n`, mode 2033 | Ask whether its pane is visible, and be told when it changes: when its tab is switched away from, its window is hidden or minimized, or another pane is zoomed. A program can pause animations it isn't showing. |

## Clipboard and notifications

- **OSC 52** lets a program copy to the clipboard and read from it, including over ssh. By default, copying is allowed and Mightty asks before a program reads the clipboard. The `clipboard-write` and `clipboard-read` settings change this.
- **OSC 9 and OSC 777** notifications mark the pane as needing your attention. See [Agents and scripting](/docs/agents/).

## Queries

A program can ask Mightty about itself and its state:

| Sequence | What a program can ask |
|---|---|
| DSR, XTVERSION | The cursor position, and the terminal's name and version. |
| DECXCPR | The cursor position with its page, as `CSI ? row ; column ; 1 R`. |
| DECRQM | Whether a mode is on. |
| XTGETTCAP | Which terminfo capabilities Mightty supports, such as true colour (`RGB`, `Tc`), styled underlines (`Smulx`), the cursor shape (`Ss`) and synchronized output (`Sync`). Neovim uses this to detect true colour when `COLORTERM` isn't set. |
| DECRQSS | The current text style (`m`), cursor shape (`SP q`) and scroll region (`r`). |
| XTWINOPS 14, 16, 18, 19 | The size of the text area and of a cell in pixels, and the size of the grid in cells. 19 reports the same as 18, because the window's text area is the screen a program can use. |

A program that sets mode 2048 is sent the size in its input, as `CSI 48 ; rows ; columns ; height ; width t`, when it sets the mode and whenever the size changes. Full-screen programs get the new size in order with the keys typed, instead of only from a signal.

A program can also undo the modes and settings it changed, without clearing the screen, with a soft reset (DECSTR, `CSI ! p`).

## Safety

Output can come from any program, including one on a server you reach over ssh, so Mightty treats all of it as untrusted.

- **Titles and commands.** Control characters are removed from window titles and command lines.
- **Pasting.** Mightty asks before pasting text that could run a command: text that contains control characters, or text with a line break when the program hasn't turned on bracketed paste.
- **Passwords.** Secure Keyboard Entry turns on at password prompts, so other apps can't read what you type.
- **Size limits.** Images and escape sequences have size limits. An escape sequence over the limit is ignored, not acted on in part.
- **Files.** A program can ask Mightty to read an image from a file, but only a complete PNG, and Mightty's reply never reveals whether a file exists.
- **Images.** Image formats other than PNG are decoded in a sandboxed process, with limits on size and time.
