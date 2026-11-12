---
title: Architecture
summary: How Mightty is put together, and how output gets to the screen.
order: 12
---
# Architecture

Mightty is a single Rust workspace of twelve crates with no third-party dependencies. Everything beyond Rust's standard library is either written in the repository or called from macOS through hand-written bindings: `openpty` for pseudo-terminals, Cocoa through `objc_msgSend`, Metal, and CoreText.

This page describes the design principles, the layers and the crates in each, the threads at run time, the path data takes through the system, and where the trust boundaries are.

## Design principles

| Principle | What it means in the code |
|---|---|
| No third-party code | The workspace has no external crates, including for tests and build tools. A script in CI fails the build if one is added. |
| A core without I/O | The parser and the terminal state take bytes and produce state. They open no files, start no threads and draw nothing, so they can be tested by feeding bytes and can be embedded as a library. |
| Sessions are separate from windows | A pane, which is a process and its terminal state, is owned by the multiplexer and not by the window that shows it. |
| One control interface | The app, the command-line client and the MCP server all drive panes through the same JSON-RPC methods. |
| Output is untrusted | Anything a program prints is treated as hostile input: sizes are bounded, and output cannot trigger actions without the user. |
| AI stays outside the core | Agents get structured access through the control socket and MCP. The core crates contain no agent-specific logic. |

## Layers

{{layers}}

The terminal core does not know about windows, sockets or agents, and the foundation crates do not depend on each other.

### Interfaces

| Crate | Responsibility |
|---|---|
| `mightty-macos` | The app: windows, tabs, splits, input, menus, the settings screen, the Metal renderer and the CoreText glyph cache. |
| `mightty` | The `mightty` binary and its commands, including `mightty ctl`, the command-line client for the control socket. |
| `mightty-mcp` | The MCP server. It is a client of the control socket. |
| `mightty-ffi` | `libmightty`: the parser and terminal state as a C library. |

### Sessions

| Crate | Responsibility |
|---|---|
| `mightty-mux` | The multiplexer. It owns every pane, runs the threads that read from and write to each pseudo-terminal, tracks whether a pane needs attention, and serves all of this on the control socket. It also holds the shell integration scripts. |

### Terminal core

| Crate | Responsibility |
|---|---|
| `mightty-term` | The terminal state: screens, scrollback, cursor, modes, styles, prompt marks and kitty graphics. |
| `mightty-render` | Drawing that needs no GPU: colours, and box-drawing and block characters drawn as shapes instead of being taken from the font. |
| `mightty-font` | Mightty's own font reader and rasterizer, for platforms without CoreText. On macOS it is used only with `font-backend = mightty`. |

### Foundations

| Crate | Responsibility |
|---|---|
| `mightty-vt` | The escape-sequence parser: a state machine after Paul Williams' design, with UTF-8 decoding, colon sub-parameters and APC strings. |
| `mightty-pty` | Pseudo-terminals and the processes attached to them. |
| `mightty-json` | JSON, for the control socket and MCP. |
| `mightty-image` | PNG and zlib decoding, for colour emoji and kitty graphics. |

## Panes and windows

A pane is a process on a pseudo-terminal, together with the terminal state that its output builds. The multiplexer owns every pane. A window only displays one.

This separation is what allows a closed tab to keep running until <kbd>⌘Z</kbd> brings it back, a tab to move between windows without its program noticing, and an agent to run commands in a pane that has no window at all.

## Threads

The app is one process. Its threads have fixed roles:

| Thread | How many | Responsibility |
|---|---|---|
| Main | One | The AppKit event loop. It handles input, owns all window state and draws every frame. |
| Pane reader | One per pane | Reads the pane's pseudo-terminal, parses what it reads into the terminal state, and sends any replies the program asked for. |
| Pane writer | One per pane | Writes keyboard input and pasted text to the pseudo-terminal, so that the main thread never waits on a program that isn't reading. |
| Status monitor | One | Re-evaluates each pane's status four times a second, and releases scrollback memory for panes that have gone quiet. |
| Control socket | One, plus one or two per connection | Accepts connections, runs their JSON-RPC calls and sends them events. |

Each pane's terminal state is protected by a lock. The reader holds the lock while it applies one read, and the main thread holds it only long enough to copy what a frame needs.

## Data flow

### From a program to the screen

1. **Reading.** The pane's reader thread reads its pseudo-terminal and parses what it reads. macOS returns at most 1 KiB per read, and a separate parsing thread only added the cost of passing data between the two, so reading and parsing share a thread.
2. **Parsing.** The parser finds runs of printable text eight bytes at a time and writes each run into the screen in one step. The screen is a ring of lines, so scrolling moves no text. Each cell is 8 bytes: a character and a style number, with each distinct style stored once. A line keeps room for the full width while output is streaming, and gives back what it doesn't use once the pane goes quiet.
3. **Scheduling.** A pane with new output asks to be drawn, and the main thread draws it at most once per display refresh. Output that follows a pause is drawn immediately. After a key press, the frame is drawn about 2 ms later, once the echo has arrived, instead of at the next refresh. With `window-vsync` on, which is the default, the frame still waits for the display to show it.
4. **Drawing.** A frame copies what it needs while the terminal is locked, then draws without the lock. Cell backgrounds and the cursor come first, then underlines and glyphs from a glyph atlas, all in one instanced draw of quads. Each image adds one draw, under or over the text.

Replies that a program asks for, such as the cursor position, are sent by the reader thread after each read. They don't wait for a frame to be drawn or for a backlog of output to be parsed.

### From the keyboard to a program

1. The main thread receives the key event and checks it against the key bindings. A bound key runs its action and goes no further.
2. Otherwise, the key is encoded for the program in the pane: in the kitty keyboard protocol if the program asked for it, and in the traditional encoding if not.
3. The bytes are queued for the pane's writer thread, which writes them to the pseudo-terminal.

## The control interface

The app serves the multiplexer on a Unix socket at `<runtime dir>/<pid>.sock`. Every pane receives the path in `MIGHTTY_SOCKET`. The protocol is JSON-RPC: requests such as `pane.spawn`, `pane.run` and `pane.read`, and events such as `pane.opened` and `command.finished`.

`mightty ctl` and `mightty mcp` are clients of this socket, and the app's own features use the same methods. A feature that scripts should be able to reach is added as a socket method first.

## Trust boundaries

| Boundary | How it is enforced |
|---|---|
| Program output | Parsed with bounded sizes for sequences, images and fonts. Control characters are removed from titles. Clipboard reads and risky pastes need the user's confirmation. |
| Shell integration marks | Each pane's shell holds a secret, and marks without it are ignored, so a program can't imitate a prompt or a finished command. |
| The control socket | It sits in a folder that only the user can read, and each connection is checked to come from the same user. |
| Agents over MCP | An agent can use only its own pane and the panes it opened, unless the server is started with `--all-panes`. |

## Settings

The app checks the config file once a second and applies it when it changes. `mightty config` edits the same file in place, one line at a time, so your comments and the order of the file are preserved.

The list of settings, with their defaults and descriptions, is defined in the app. `mightty config --export` prints it as JSON, and the [settings reference](/docs/settings/#every-setting) on this site is generated from that output.

## libmightty

The parser and the terminal state are also available as a C library, with a hand-written header in `include/mightty.h`. The library does no I/O and starts no threads: you feed it bytes and read back cells. For the same reason, it accepts kitty images only when they are sent inline, not by file or shared memory. `cargo test` builds a C program against the header, so the header and the library can't drift apart.

```sh
scripts/install-lib.sh /usr/local
cc app.c $(pkg-config --cflags --libs mightty)
```
