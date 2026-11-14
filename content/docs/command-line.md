---
title: Command line
summary: Every mightty command.
description: Every mightty command: opening windows, the settings screen, mightty ctl, the MCP server and running programs without a window.
order: 10
---
# Command line

The `mightty` binary inside the app is also the command-line tool. [Install](/docs/install/#the-mightty-command) explains how to put it on your `PATH`.

## Open a window

```
mightty [--SETTING=VALUE...] [-e PROGRAM [ARGS...]]
```

Opens a window. With `-e`, the window runs that program in the current folder and closes when the program exits. Any setting can be given as a flag; see [Settings](/docs/settings/#flags).

## Edit settings

```
mightty config [--export]
```

Opens the settings screen in the terminal you are in. With `--export`, it prints every setting and action as JSON instead.

## Control a running Mightty

```
mightty ctl COMMAND
```

Controls panes in a running Mightty: running commands, pressing keys, reading output, changing focus and watching events. See [Agents and scripting](/docs/agents/#mightty-ctl).

## Serve MCP

```
mightty mcp [--all-panes]
```

Runs an MCP server for AI agents on standard input and output. See [Agents and scripting](/docs/agents/#mcp).

## Run a program without a window

```
mightty run [--cols N] [--rows N] [--screen] -- PROGRAM [ARGS...]
```

Runs a program in a terminal with no window, then prints what the program displayed: its scrollback and screen, or the screen only with `--screen`. The terminal is 80×24 unless you set `--cols` and `--rows`. `mightty run` exits with the program's exit code, which makes it useful in tests.

## Help and version

```
mightty --help
mightty --version
```
