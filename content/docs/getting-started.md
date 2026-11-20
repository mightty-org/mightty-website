---
title: First steps
summary: The keys you will use every day.
description: The Mightty keyboard shortcuts you will use every day: tabs, windows, splits, search, selection and text size.
order: 2
---
# First steps

The keys you'll use most. Each is also in the menus and the command palette (<kbd>⌘⇧P</kbd>), and can be changed in the [key bindings](/docs/keybindings/).

## Tabs and windows

| Key | Action |
|---|---|
| <kbd>⌘T</kbd> | Open a tab in the current folder. |
| <kbd>⌘N</kbd> | Open a window in the current folder. |
| <kbd>⌘1</kbd> to <kbd>⌘8</kbd> | Go to the tab in that position. <kbd>⌘9</kbd> goes to the last tab. |
| <kbd>⌘⇧[</kbd> <kbd>⌘⇧]</kbd> | Go to the previous or next tab. |
| <kbd>⌘W</kbd> | Close the pane. |
| <kbd>⌘⇧W</kbd> | Close the window. |
| <kbd>⌘Z</kbd> | Bring back what you just closed. |
| <kbd>⌘⇧T</kbd> | Reopen the last closed tab, pane or window. |

<kbd>⌘Z</kbd> works for 5 seconds after you close something, and restores it with its programs still running. After that, <kbd>⌘⇧T</kbd> reopens it with new shells. [Tabs and windows](/docs/tabs-and-windows/) covers both in more detail.

## Splits

| Key | Action |
|---|---|
| <kbd>⌘D</kbd> | Split the pane to the right. |
| <kbd>⌘⇧D</kbd> | Split the pane downwards. |
| <kbd>⌘[</kbd> <kbd>⌘]</kbd> | Go to the previous or next pane. |
| <kbd>⌘⌥</kbd> + arrow | Go to the pane in that direction. |
| <kbd>⌘⌃</kbd> + arrow | Move the divider. |
| <kbd>⌘⌃=</kbd> | Make all panes the same size. Double-clicking a divider does the same. |
| <kbd>⌘⇧↩</kbd> | Zoom the pane to fill the window. Press it again to restore the layout. |

## Search and selection

| Key | Action |
|---|---|
| <kbd>⌘F</kbd> | Search the pane, including its scrollback. |
| <kbd>⌘G</kbd> <kbd>⌘⇧G</kbd> | Go to the next or previous match. |
| <kbd>⌘K</kbd> | Clear the scrollback and the screen above the prompt. |
| <kbd>⌘↑</kbd> <kbd>⌘↓</kbd> | Scroll to the previous or next prompt. |

To select text, drag over it. Double-click selects a word and triple-click selects a line. Hold <kbd>⌥</kbd> while dragging to select a rectangle. Dragging past the top or bottom edge of the pane scrolls it.

A double-click selects a whole path or URL. To have it stop at other characters, such as `/`, list them in `selection-word-chars`.

<kbd>⌘C</kbd> copies the selection with its colours and styles, so pasting into Notes, Mail or Slack keeps how it looked. Apps that paste plain text get plain text. `copy-with-colors = false` copies plain text only.

Hold <kbd>⌘</kbd> over a link to see where it goes, and <kbd>⌘</kbd>-click to open it.

## Editing the command

The command you type at the prompt can be edited with the same keys as a Mac text field:

| Key | Action |
|---|---|
| <kbd>⌘←</kbd> <kbd>⌘→</kbd> | Move to the start or end of the line. |
| <kbd>⌥←</kbd> <kbd>⌥→</kbd> | Move back or forward by a word. |
| <kbd>⌘⌫</kbd> <kbd>⌘⌦</kbd> | Delete to the start or end of the line. |
| <kbd>⌥⌫</kbd> <kbd>⌥⌦</kbd> | Delete the previous or next word. |
| <kbd>⇧←</kbd> <kbd>⇧→</kbd> | Select by character. Add <kbd>⌥</kbd> to select by word, or <kbd>⌘</kbd> to select to the start or end. |
| <kbd>⌘X</kbd> | Cut the selected part of the command. |

Click in the command to move the cursor there. With part of the command selected, with the mouse or the keyboard, typing or pasting replaces it and <kbd>⌫</kbd> deletes it.

Clicking and editing a selection need [shell integration](/docs/shell-integration/). The click can be turned off with `cursor-click-to-move`.

## Text size

| Key | Action |
|---|---|
| <kbd>⌘=</kbd> <kbd>⌘-</kbd> | Make the text larger or smaller. |
| <kbd>⌘0</kbd> | Return to the configured size. |

## Where to go next

- [Settings](/docs/settings/) (<kbd>⌘,</kbd>) for fonts, colours and behaviour.
- [Tabs and windows](/docs/tabs-and-windows/) for dragging tabs between windows and the quick terminal.
- [Agents and scripting](/docs/agents/) for what Mightty does when a coding agent is waiting for you.
