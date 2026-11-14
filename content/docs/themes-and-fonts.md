---
title: Themes and fonts
summary: Choosing a theme and a font.
description: Choose one of 819 built-in themes or your own, follow the macOS appearance, and set fonts, ligatures and OpenType features in Mightty.
order: 5
---
# Themes and fonts

## Themes

Mightty includes all 819 themes from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes).

To choose one, open the settings (<kbd>⌘,</kbd>) and press <kbd>t</kbd>. Mightty previews each theme as you move through the list. You can also name a theme in the config file:

```
theme = Catppuccin Mocha
```

To follow the macOS appearance, give one theme for light mode and one for dark mode:

```
theme = light:Catppuccin Latte,dark:Catppuccin Mocha
```

### Your own themes

Put your own themes in `~/.config/mightty/themes/`, one file per theme. A theme file uses Ghostty's format and contains colour settings only. A theme of yours with the same name as a built-in theme replaces it.

To adjust a single colour of a theme, add the colour setting to your config after the `theme` line.

### Colours set by programs

Programs can change colours as well, as theme scripts and some editors do with OSC 4, 10, 11 and 12. Reloading the settings keeps these changes. Resetting the terminal removes them.

## Fonts

To choose a font, open the settings and press <kbd>↩</kbd> on `font-family`. The list shows the monospaced fonts installed on your Mac, and <kbd>Tab</kbd> shows the rest. Mightty previews each font as you move through the list.

In the config file, you can give `font-family` more than once to add fallback fonts. Mightty tries them in order before the fonts macOS would choose:

```
font-family = JetBrains Mono
font-family = Symbols Nerd Font Mono
font-size = 14
```

| Setting | Effect |
|---|---|
| `font-ligatures` | Ligatures are on by default: in fonts that have them, `->`, `!=` and `===` are drawn as single shapes. Set it to `false` to turn them off. |
| `font-feature` | Turns an OpenType feature on or off: `ss01` turns one on, `-calt` turns one off, and `cv01=2` selects an alternate. |
| `font-variation` | Sets an axis of a variable font, such as `wght=450`. |
| `font-variation-bold`, `font-variation-italic` | Set axes for the bold and italic styles. |

If a font family has no bold or italic, Mightty makes them from the regular face.

### Icons

The icons used by shell prompts and file listings come from Symbols Nerd Font, which is built in. They appear whichever font you choose.

### Character widths

Character widths follow Unicode 17. An emoji written with a variation selector takes two cells, as it does in Ghostty and kitty. Some shells count such an emoji as one cell and then redraw the line incorrectly. If you see that, set `grapheme-width = legacy` and Mightty counts it as one cell too.
