# waygate

A terminal board for your Claude Code sessions. Find the one you want, see where it left off, and resume it in a new tab.

```
╭ Projects ──────────────────────╮╭ All sessions · 140 ──────────────────╮╭ Where it left off ───────────────────╮
│ ● Waiting on me             14 ││▌● Billing export refactor         2h ││ Billing export refactor              │
│ ◇ All sessions             140 ││▌  api · Keep the old endpoint an… ││ ~/code/api   ● waiting on you        │
│────────────────────────────────││   Flaky login test                3h ││                                      │
│ ● api       ▂  ▃  ▄  ▃▅▄    13 ││   web · Why does this fail only o… ││ catch me up ──────────────────────── │
│ ● web       █  ▄▃▃▅  ▂  ▄   36 ││ ● Postgres migration plan         5h ││ Goal: Split the export into jobs…    │
╰────────────────────────────────╯╰──────────────────────────────────────╯╰──────────────────────────────────────╯
```

waygate reads the transcripts Claude Code already keeps in `~/.claude/projects` and never changes them.

## Install

```sh
cargo install --path .
```

Then run `waygate` from anywhere.

## What you get

- **Three panes.** Projects on the left, with a 30-day activity sparkline each. Sessions in the middle, newest first. On the right, where the session left off: Claude's last reply rendered as Markdown, your last prompt, the files it wrote and any artifacts it published.
- **Waiting on me.** Sessions where Claude spoke last and asked you something get an amber ●. Mark one done with `d` or by clicking its dot. New activity brings it back, and questions older than 14 days drop off on their own.
- **Live.** Sessions active in the last 90 seconds pulse green, and the board updates as they change.
- **Resume in a new tab.** `Enter` or a double-click opens `claude --resume` in a new tab, in the session's own folder, and leaves the board open. This works in Ghostty, iTerm2, Terminal.app and tmux. In any other terminal, or with `R`, waygate quits and resumes in place.
- **Search.** `/` fuzzy-matches titles and project names and searches the full text of prompts and replies.
- **Transcript.** `t` shows the whole conversation, with tool calls folded into one-liners. Click one to expand it.
- **Stats.** `s` shows sessions per week, cost by project, the most-used tools and the longest threads.
- **Catch me up.** `c` asks Claude (Haiku, through `claude -p`) for a three-line summary of the session and caches it. It only runs when you press the key, and costs a few cents.
- **Mouse.** Click, double-click, right-click for a menu, scroll any pane, drag the borders to resize (the widths are remembered), and click links to open them.

Press `?` in the board for every key.

## Files

| Path | What |
|---|---|
| `~/.claude/projects/*/*.jsonl` | Claude Code's transcripts, read only |
| `~/Library/Caches/waygate/index.json` (macOS) or `~/.cache/waygate/` | Index cache, safe to delete |
| `~/Library/Application Support/waygate/state.json` (macOS) or `~/.local/share/waygate/` | Done marks, pane widths, summaries. Read from the old `cs` folder if this one is missing |

## Environment

| Variable | Default | |
|---|---|---|
| `CLAUDE_CONFIG_DIR` | `~/.claude` | Where Claude Code keeps its data |
| `WAYGATE_CLAUDE` | `claude` | The binary to launch |
| `WAYGATE_RESUME` | | Set to `here` to always resume in place |
| `WAYGATE_SUMMARY_MODEL` | `haiku` | Model for catch-me-up summaries |

`waygate --list` prints sessions as plain text for scripts.
