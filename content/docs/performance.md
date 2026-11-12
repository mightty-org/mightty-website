---
title: Performance
summary: Mightty against Ghostty and kitty, and how it was measured.
order: 11
---
# Performance

This page compares Mightty with Ghostty 1.4.0 and kitty 0.49.2 on start-up time, memory, throughput, CPU use and latency, and explains how each figure was measured.

## How it was measured

The figures come from `bench/suite.py` in the Mightty repository, run on an M2 with macOS 15.

- Each terminal is launched fresh at 120×40 in Menlo 14, with every other setting at its default.
- A probe runs inside the terminal, and the suite samples the terminal's process every 50 ms.
- The terminals take turns, round by round, so that a busy moment on the machine doesn't fall on only one of them.
- Each figure is the median of 10 launches or 3 sessions.

Absolute numbers change from run to run with the machine and its displays. Compare the terminals within a row, not against figures from another run.

## Start-up

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| From launch until the shell is running | 57 ms | 157 ms | 370 ms |

## Memory

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| Idle | 41 MiB | 72 MiB | 102 MiB |
| While printing as fast as it can | 78 MiB | 130 MiB | 116 MiB |
| After 260 MB of output | 55 MiB | 82 MiB | 113 MiB |

Memory here is the physical footprint that macOS reports, which is the figure Activity Monitor shows as Memory.

The scrollback defaults differ: Mightty keeps 10,000 lines, Ghostty keeps 10 MB and kitty keeps 2,000 lines. When a pane's output pauses, Mightty releases the memory its scrollback lines hold but don't use.

## Throughput

Each terminal prints a 16 MiB file with `cat`, timed from inside the terminal.

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| Plain text | 239 MiB/s | 213 MiB/s | 103 MiB/s |
| Text with colours | 186 MiB/s | 162 MiB/s | 121 MiB/s |
| CJK and other Unicode | 162 MiB/s | 150 MiB/s | 87 MiB/s |
| Moving the cursor at random (4 MB) | 171 MiB/s | 136 MiB/s | 93 MiB/s |

Single runs vary by about 10% on a busy machine, so a difference of a few percent between Mightty and Ghostty is not meaningful.

## CPU

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| Idle | 0.1% | 0.5% | 0.5% |
| While printing as fast as it can | 89% | 228% | 117% |
| Printing a steady 250 lines a second | 8.8% | 29% | 9.4% |

100% is one CPU core. At a steady 250 lines a second, Mightty and kitty use about the same amount.

## Replying to programs

Programs ask the terminal things, such as where the cursor is, and wait for the answer. This is the time from asking to having the answer:

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| Idle | 0.14 ms | 0.19 ms | 0.20 ms |
| While another program floods the terminal with output | 0.05 ms | 0.20 ms | 4.41 ms |

Mightty reads a pane's output in pieces of at most 1 KiB and sends any reply after each piece. A reply doesn't wait for the screen to be drawn.

## Typing

This is the time from a key press to the frame that shows its echo, on a 60 Hz display. Only Mightty is measured, because the probe sends keys through the app's own event queue and reads the time Metal reports for the frame reaching the screen, which isn't possible in another terminal. The figures don't include the keyboard or the window server, which come before the app.

| | `window-vsync = true` (default) | `window-vsync = false` |
|---|---|---|
| Idle | 24.9 ms | 8.0 ms |
| While flooded with output | 36.1 ms | 8.9 ms |

After a key press, Mightty draws about 2 ms later, once the echo has arrived, instead of waiting for the next refresh. With vsync on, that frame still waits for the display, behind any frame already queued. With vsync off, the display takes the newest frame, at the risk of tearing. vsync is on by default, as it is in Ghostty and kitty.

## Second throughput run

`bench/compare.sh` measures six workloads, alternating the terminals over 5 rounds of 3 runs. It runs Mightty as a pane in a copy of Mightty that is already open, not in a freshly launched one, so these figures are not measured the same way as the ones above and are from an earlier run.

| | Mightty | Ghostty | kitty |
|---|---|---|---|
| Plain text | 237 MiB/s | 168 MiB/s | 101 MiB/s |
| True colour | 202 MiB/s | 156 MiB/s | 101 MiB/s |
| Unicode | 166 MiB/s | 151 MiB/s | 89 MiB/s |
| Cursor motion | 186 MiB/s | 171 MiB/s | 95 MiB/s |
| Scroll region | 257 MiB/s | 239 MiB/s | 114 MiB/s |
| Long lines | 257 MiB/s | 280 MiB/s | 132 MiB/s |

For long lines, Mightty and Ghostty both reach the rate at which macOS can pass output to a terminal. Pooled over 63 runs, their medians are 258 and 263 MiB/s.

## Running the benchmarks

```sh
python3 -I bench/suite.py         # everything above except the second run
bench/compare.sh                  # the second run; needs Mightty open
```

See [Architecture](/docs/architecture/) for how output gets from a program to the screen.
