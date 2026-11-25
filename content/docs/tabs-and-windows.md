---
title: Tabs and windows
summary: Moving tabs around, getting closed ones back, the quick terminal, and opening scripts and links with Mightty.
description: Mightty tabs, title bar styles, what happens when a program ends, restoring closed tabs and windows with their programs running, the quick terminal, and Mightty as the default terminal.
order: 6
---
# Tabs and windows

This page covers the tab bar, the title bar styles, what happens when a program ends, restoring closed tabs, the quick terminal, opening scripts and links with Mightty, and how windows come back when you reopen Mightty.

## The tab bar

A window shows its tab bar when it has two or more tabs. With `macos-titlebar-style = tabs`, the bar is always visible.

- **Switch tabs.** Click a tab, or press <kbd>⌘1</kbd> to <kbd>⌘9</kbd>.
- **Close a tab.** Point at it and click the close button that appears, or middle-click it.
- **Rename a tab.** Double-click it.
- **Reorder tabs.** Drag a tab along the bar.
- **Move a tab to its own window.** Drag it off the bar.
- **Move a tab to another window.** Drag it onto that window's tab bar.
- **Merge two windows.** Drag one window by its title bar onto the tab bar of the other. All of its tabs move there.

Right-click a tab for Rename Tab, Tab Colour, Move Tab to New Window, Close Tab and Close Other Tabs.

## Tab colours

Tab Colour marks a tab with a colour, which is blended into the tab's background. A coloured tab keeps its colour when Mightty restores your windows.

There are seven built-in colours: Red, Orange, Yellow, Green, Blue, Purple and Gray. To add your own, choose New Colour…, pick a colour and give it a name, such as Prod. Mightty saves it to your config file as a `tab-color` line, and it is then offered for every tab. You can also write the lines yourself:

```
tab-color = Prod=#d0342c
tab-color = Staging=#e8a33d
```

A name given again replaces that colour, and your colour can take a built-in colour's name to replace it. Names match in any case. Tabs refer to colours by name, so changing a colour's value recolours every tab marked with it.

The command palette lists each colour as "Tab Colour: …". To set one with a key, bind `set_tab_color`:

```
keybind = cmd+shift+r=set_tab_color:Prod
keybind = cmd+shift+0=set_tab_color:none
```

When there are more tabs than fit, the bar scrolls sideways and keeps the selected tab in view.

## Title bar styles

`macos-titlebar-style` sets what the top of the window looks like.

| Value | Result |
|---|---|
| `native` | The standard macOS title bar, with the tab bar below it. This is the default. |
| `transparent` | The title bar takes the terminal's background colour. |
| `tabs` | The tabs replace the title bar and sit next to the window buttons. |
| `hidden` | No title bar. Drag the window by its top edge. The tab bar appears at the top when there are two or more tabs. |

## When a program ends

When the program in a pane ends, such as the shell after `exit`, `close-on-exit` decides whether the pane closes.

| Value | Result |
|---|---|
| `success` | The pane closes if the program succeeded. If it failed or was ended by a signal, the pane stays open. This is the default. |
| `always` | The pane always closes. |
| `never` | Every pane stays open. |

A pane that stays open keeps its last output and shows a bar along the bottom saying how the program ended, in red if it failed. Press <kbd>↩</kbd> or click the restart button to run it again in the same pane, or click the close button to close the pane.

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

## Opening scripts and links with Mightty

Mightty can open these from the Finder, the Dock, your browser or `open -a Mightty`:

- **A script or program**, such as a `.command` file, runs in a new window, in the folder the file is in. The window stays open when it ends, with the bar from [When a program ends](#when-a-program-ends), so you can read its output.
- **A folder**, dropped on Mightty's Dock icon, opens a shell in that folder.
- **An `ssh://` link**, such as `ssh://me@example.com:2222`, opens a window running `ssh` to that host. Links with anything other than a user, host and port are refused.

A file downloaded from the Internet asks before it runs.

To have Mightty open `.command` files, programs and `ssh://` links by default, choose Mightty → Make Mightty the Default Terminal. macOS asks you to confirm for the links. Shell scripts ending in `.sh` still open in your editor; Mightty is listed in Open With for them. If you move Mightty to another folder, choose the menu item again.

## Restoring windows at launch

When you reopen Mightty, it restores its windows, tabs and splits, each in the folder it was in. It doesn't do this when "Close windows when quitting an application" is turned on in System Settings.

To decide for Mightty alone, set `window-save-state` to `always` or `never`.
