---
title: Shell integration
summary: What Mightty learns from your shell.
description: How Mightty integrates with zsh, bash and fish to mark prompts, commands, output and exit codes, without editing your dotfiles.
order: 8
---
# Shell integration

Shell integration tells Mightty where each prompt, command and block of output begins, and the exit code of each command. It works in zsh, in bash 3.2 or later, and in fish 3.0 or later.

You don't need to set anything up or edit your dotfiles. Mightty starts your shell, your startup files run as usual, and Mightty then loads a small script that sends these marks.

## What it enables

- `mightty ctl run` and the MCP tool `run_command` return a command's output and exit code.
- `mightty ctl commands` and the MCP tool `list_commands` list recent commands with their output.
- New tabs and splits open in the folder your shell is in.

## Marks can't be forged

Each pane has a secret that only its shell knows, and Mightty ignores marks that don't carry it. A program can't print fake marks to make its output look like a prompt or a finished command.

## Using it in another terminal

The scripts are in Mightty's source. To use the integration in another terminal, source the script for your shell:

| Shell | Script |
|---|---|
| zsh | `crates/mightty-mux/shell/mightty.zsh` |
| bash | `crates/mightty-mux/shell/mightty.bash` |
| fish | `crates/mightty-mux/shell/fish/vendor_conf.d/mightty.fish` |
