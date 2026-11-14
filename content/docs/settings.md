---
title: Settings
summary: How to change settings, and what each one does.
description: How to change Mightty settings in the settings screen or the config file, and a reference of every setting with its default.
order: 3
---
# Settings

Settings are kept in a plain-text config file. The settings screen edits that file, and Mightty applies it whenever it changes.

## The settings screen

Press <kbd>⌘,</kbd> to open the settings in a new tab. Settings are grouped into sections, and each one shows its current value and a description.

| Key | Action |
|---|---|
| <kbd>↑</kbd> <kbd>↓</kbd> | Move through the list. |
| <kbd>Tab</kbd> | Go to the next section. |
| <kbd>/</kbd> | Search. |
| <kbd>↩</kbd> | Edit the setting. An on/off setting switches straight away. |
| <kbd>d</kbd> | Restore the default. |
| <kbd>t</kbd> | Choose a theme. |
| <kbd>e</kbd> | Open the config file in your editor. |

When a setting accepts one of a fixed set of values, Mightty completes the value as you type and shows the rest of the first match in grey. Press <kbd>→</kbd> or <kbd>Tab</kbd> to accept it, or <kbd>↑</kbd> <kbd>↓</kbd> to choose another.

The theme picker and the font settings open a list that you can filter by typing. Mightty previews each choice on the settings tab as you move to it. Press <kbd>↩</kbd> to keep the choice or <kbd>Esc</kbd> to go back to the previous one. The font list shows monospaced fonts first; press <kbd>Tab</kbd> to show every font.

Settings that can be given more than once, such as `font-family`, are edited in the config file. Press <kbd>e</kbd> to open it.

The same screen runs in any terminal with `mightty config`.

## The config file

Settings are stored in `~/.config/mightty/config`, or in `$XDG_CONFIG_HOME/mightty/config` if that variable is set. Each line has the form `name = value`. A `#` starts a comment.

```
font-family = JetBrains Mono
font-size = 13
theme = Catppuccin Mocha
macos-titlebar-style = tabs
```

Mightty checks the file once a second and applies it when it changes, so a change saved from any editor takes effect within a second. The settings screen changes individual lines and leaves your comments and the order of the file as they are.

Mightty → Open Config File opens the file in a new tab, in the editor named by `$VISUAL` or `$EDITOR`. It creates the file first if it doesn't exist.

Settings use Ghostty's names wherever they mean the same thing, so many lines from a Ghostty config work unchanged. If Mightty doesn't understand a line, it reports the line number and applies the rest of the file.

To split your settings across files, use `config-file = PATH`. Mightty reads that file at the point where the line appears.

## Flags

Any setting can also be given as a command-line flag. A flag overrides the config file.

```sh
mightty --font-size=16 --window-width=120 -e htop
```

`--config-file=PATH` reads another file after the default one.

## Every setting

{{settings}}
