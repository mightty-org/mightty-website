---
title: Agents and scripting
summary: How Mightty tells you an agent is waiting, and how agents and scripts drive it.
order: 7
---
# Agents and scripting

Mightty works with coding agents in two ways. It notices when an agent running in a pane is waiting for you, and it lets agents and scripts control panes through an MCP server and a command-line client.

## When an agent is waiting for you

Mightty recognizes these agents by their command name: `claude`, `codex`, `gemini`, `aider`, `opencode`, `amp`, `goose`, `cursor-agent`, `copilot`, `crush`, `qwen` and `droid`. It also recognizes them when they run as a script under node, bun, deno or python.

A pane needs your attention when any of these happens:

- An agent in it has been working and then goes quiet.
- A program in it rings the bell.
- A program in it asks for a notification, with OSC 9, OSC 777 or `mightty ctl notify`.

If you aren't looking at that pane, Mightty tells you in three places:

- The pane gets an orange border, and the title of its window starts with ●.
- The Dock icon shows the number of panes that need you. It bounces once if Mightty is in the background.
- A notification appears if you are in another app.

Press <kbd>⌘⇧A</kbd> to go to the pane that has been waiting longest.

### Claude Code

Claude Code can tell Mightty the moment it asks for permission, without waiting to go quiet. Add this hook to `~/.claude/settings.json`:

```json
{ "hooks": { "Notification": [{ "hooks": [{ "type": "command", "command": "mightty ctl notify 'Claude needs you'" }] }] } }
```

## MCP

`mightty mcp` is an MCP server that gives an agent access to panes. To add it to Claude Code:

```sh
claude mcp add mightty -- mightty mcp
```

The server provides these tools:

| Tool | What it does |
|---|---|
| `run_command` | Runs a command and returns its output and exit code. |
| `open_pane` | Opens a pane: a split beside the pane you are in, a tab, or a window. |
| `read_pane` | Reads a pane's screen or scrollback. |
| `list_commands` | Lists recent commands in a pane, with their output and exit codes. |
| `wait_for` | Waits for text to appear, for a command to finish, or for the pane's program to exit. |
| `send_text`, `send_keys` | Types into a pane. |
| `list_panes`, `focus_pane`, `close_pane` | Lists panes, brings one to the front, or closes one. |
| `notify_user` | Marks the agent's pane as needing you. |

Some details of how the tools behave:

- **Panes the agent opens** don't take keyboard focus, and they stay open after their command ends.
- **`run_command` without a pane** runs in a hidden shell. The agent reuses that shell for the whole session, so `cd` and environment variables carry over from one call to the next. Mightty closes the shell when the session ends.
- **Access is limited** to the agent's own pane and the panes it opened. To give an agent access to every pane, start the server with `mightty mcp --all-panes`.
- **Without a running Mightty**, `mightty mcp` runs panes itself, with no windows.

## mightty ctl

Every running Mightty has a control socket. `mightty ctl` is a command-line client for it:

```sh
mightty ctl list                           # panes: state, window, title, folder
mightty ctl run --pane 1 -- make test      # run in pane 1, print the output, exit with its code
mightty ctl spawn --right -- npm run dev   # a split beside this pane (or --down, --tab)
mightty ctl wait 3 --text "ready"          # wait until pane 3 shows "ready"
mightty ctl read 3 --lines 20              # the last 20 lines of pane 3
mightty ctl keys 3 ctrl-c                  # press keys in pane 3
mightty ctl attention                      # panes that need you, longest waiting first
mightty ctl watch command.finished         # events as JSON lines
```

Run `mightty ctl` with no arguments to list every command and flag.

**Which Mightty it talks to.** Inside a Mightty pane, `mightty ctl` uses the Mightty that pane belongs to. Anywhere else, it uses the one that was started most recently.

**Commands after `--`.** `run` and `spawn` take the command to run after `--`. Several arguments are passed as separate words, so `-- rm "my file"` removes one file. A single argument is passed to the shell as written, so `-- 'make && ./run | less'` works as a shell line. An argument that contains a control character is refused, because the terminal would act on it as a key press.

**`run` without `--pane`** starts a hidden shell for the command and closes it when the command finishes.

### JSON-RPC

The socket speaks JSON-RPC, so a program in any language can use it directly. `mightty ctl call METHOD [JSON]` makes a single call and prints the result.
