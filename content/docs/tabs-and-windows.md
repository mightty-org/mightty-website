---
title: Tabs and windows
summary: Moving tabs around, getting closed ones back, and the quick terminal.
description: Mightty tabs, title bar styles, restoring closed tabs and windows with their programs running, and the quick terminal.
order: 6
---
# Tabs and windows

This page covers the tab bar, the title bar styles, restoring closed tabs, the quick terminal, and how windows come back when you reopen Mightty.

## The tab bar

A window shows its tab bar when it has two or more tabs. With `macos-titlebar-style = tabs`, the bar is always visible.

- **Switch tabs.** Click a tab, or press <kbd>⌘1</kbd> to <kbd>⌘9</kbd>.
- **Close a tab.** Point at it and click the close button that appears.
- **Rename a tab.** Double-click it.
- **Reorder tabs.** Drag a tab along the bar.
- **Move a tab to its own window.** Drag it off the bar.
- **Move a tab to another window.** Drag it onto that window's tab bar.
- **Merge two windows.** Drag one window by its title bar onto the tab bar of the other. All of its tabs move there.

Right-click a tab for Rename Tab, Move Tab to New Window, Close Tab and Close Other Tabs.

When there are more tabs than fit, the bar scrolls sideways and keeps the selected tab in view.

## Title bar styles

`macos-titlebar-style` sets what the top of the window looks like.

| Value | Result |
|---|---|
| `native` | The standard macOS title bar, with the tab bar below it. This is the default. |
| `transparent` | The title bar takes the terminal's background colour. |
| `tabs` | The tabs replace the title bar and sit next to the window buttons. |
| `hidden` | No title bar. Drag the window by its top edge. The tab bar appears at the top when there are two or more tabs. |

## Undo and reopen

When you close a tab, pane or window, Mightty keeps its programs running in the background for the length of `undo-timeout`, which is 5 seconds by default. Press <kbd>⌘Z</kbd> within that time to bring it back exactly as it was.

After the timeout, the programs are ended. <kbd>⌘⇧T</kbd> still reopens what you closed, with the same splits, titles and folders, but with new shells.

A pane whose shell had already exited when you closed it is not kept.

## The quick terminal

The quick terminal is a terminal that slides down from the top of the screen, over whichever app you are using, on any Space. Open it from the Window menu, or bind a global key to it:

```
keybind = global:ctrl+grave_accent=toggle_quick_terminal
```

Press the key again to hide it. It also hides when you switch to another app or window.

| Setting | Effect |
|---|---|
| `quick-terminal-autohide = false` | Keeps it open when you switch away. |
| `quick-terminal-position` | Places it at another edge of the screen, or in the centre. |
| `quick-terminal-size` | Sets how much of the screen it covers. |

## Restoring windows at launch

When you reopen Mightty, it restores its windows, tabs and splits, each in the folder it was in. It doesn't do this when "Close windows when quitting an application" is turned on in System Settings.

To decide for Mightty alone, set `window-save-state` to `always` or `never`.
