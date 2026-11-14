---
title: Protocols
summary: The escape sequences Mightty supports beyond VT100 and xterm.
description: The terminal protocols Mightty supports: kitty graphics and keyboard protocols, Unicode 17, styled underlines, OSC 8 links, OSC 52 and more.
order: 9
---
# Protocols

Programs control a terminal by writing escape sequences. Mightty supports the VT100 and xterm sequences that programs depend on, and the newer protocols listed on this page.

## Images

Mightty supports the kitty graphics protocol.

- Images can be sent inline, or passed as a file, a temporary file or shared memory, in PNG format or as raw pixels.
- Images scroll with the text and can be drawn under or over it.
- Images can be placed relative to each other and can be animated, so `kitten icat` plays GIFs.
- Unicode placeholders are supported, which lets images display through tmux.

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

## Clipboard and notifications

- **OSC 52** lets a program copy to the clipboard and read from it, including over ssh. By default, copying is allowed and Mightty asks before a program reads the clipboard. The `clipboard-write` and `clipboard-read` settings change this.
- **OSC 9 and OSC 777** notifications mark the pane as needing your attention. See [Agents and scripting](/docs/agents/).

## Queries

A program can ask for the cursor position, the terminal's name and version (XTVERSION), and whether a mode is on (DECRQM).

## Safety

Output can come from any program, including one on a server you reach over ssh, so Mightty treats all of it as untrusted.

- **Titles and commands.** Control characters are removed from window titles and command lines.
- **Pasting.** Mightty asks before pasting text that could run a command: text that contains control characters, or text with a line break when the program hasn't turned on bracketed paste.
- **Passwords.** Secure Keyboard Entry turns on at password prompts, so other apps can't read what you type.
- **Size limits.** Images and escape sequences have size limits. An escape sequence over the limit is ignored, not acted on in part.
- **Files.** A program can ask Mightty to read an image from a file, but only a complete PNG, and Mightty's reply never reveals whether a file exists.
