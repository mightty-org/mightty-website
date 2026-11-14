---
title: Key bindings
summary: How to change keys, and what each action does.
description: How to change key bindings in Mightty, including global keys, and a reference of every action and its default key.
order: 4
---
# Key bindings

Key bindings are set in the [config file](/docs/settings/#the-config-file) with `keybind` lines, in the same format Ghostty uses. Each line binds one key to one action:

```
keybind = cmd+shift+t=new_tab
keybind = ctrl+shift+h=text:echo hello\r
keybind = cmd+k=unbind
keybind = global:ctrl+grave_accent=toggle_quick_terminal
```

The menus and the command palette show the first key bound to each action.

## Writing a key

A key is written as zero or more modifiers followed by the key itself, joined with `+`. The modifiers are `cmd`, `ctrl`, `alt` and `shift`.

The key is either the character it types without modifiers, such as `t`, `[` or `=`, or one of these names:

| Keys | Names |
|---|---|
| Editing | `enter` or `return`, `tab`, `space`, `escape`, `backspace`, `delete` |
| Navigation | `up`, `down`, `left`, `right`, `home`, `end`, `page_up`, `page_down` |
| Function keys | `f1` to `f12` |
| Punctuation | `plus`, `minus`, `equal`, `comma`, `period`, `slash`, `backslash`, `semicolon`, `apostrophe`, `left_bracket`, `right_bracket`, `grave_accent` |

## Removing a binding

- `keybind = cmd+k=unbind` removes the binding for that key. The key is then passed to the program running in the pane.
- `keybind = clear` removes every binding, including the defaults.

## Global keys

A key that starts with `global:` works in any app, not only when Mightty is in front. Global keys don't need Accessibility permission from macOS.

## Every action

{{actions}}
