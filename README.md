# waygate-cs

A terminal board for your Claude Code sessions. Find the one you want, see where it left off, and resume it in a new tab.

```
╭ Projects ──────────────────────╮╭ All sessions · 140 ──────────────────╮╭ Where it left off ───────────────────╮
│ ● Waiting on me             14 ││▌● Billing export refactor         2h ││ Billing export refactor              │
│ ◇ All sessions             140 ││▌  api · Keep the old endpoint an… ││ ~/code/api   ● waiting on you        │
│────────────────────────────────││   Flaky login test                3h ││                                      │
│ ● api                       13 ││   web · Why does this fail only o… ││ catch me up ──────────────────────── │
│ ● web                       36 ││ ● Postgres migration plan         5h ││ Goal: Split the export into jobs…    │
╰────────────────────────────────╯╰──────────────────────────────────────╯╰──────────────────────────────────────╯
```

waygate-cs reads the transcripts Claude Code already keeps in `~/.claude/projects` and never changes them.

## Install

waygate-cs runs on macOS and Linux and needs the `claude` CLI on your `PATH`.

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/brad-j/waygate-cs/releases/latest/download/waygate-cs-installer.sh | sh
```

The installer puts the binary in `~/.cargo/bin`. Release archives for each platform, with checksums, are on the [releases page](https://github.com/brad-j/waygate-cs/releases).

To build from source instead, with Rust 1.88 or newer:

```sh
cargo install --git https://github.com/brad-j/waygate-cs
```

Then run `waygate-cs` from anywhere.

## What you get

- **Three panes.** Projects on the left, with a session count each. Sessions in the middle, newest first. On the right, where the session left off: Claude's last reply rendered as Markdown, your last prompt, the files it wrote and any artifacts it published.
- **Waiting on me.** Sessions where Claude spoke last and asked you something get an amber ●. Mark one done with `d` or by clicking its dot. New activity brings it back, and questions older than 14 days drop off on their own. With `WAYGATE_CS_JEV_KEY` set, [Jev](https://typesafe.ai/) decides instead of the question marks, so offers like "Want me to also…?" stop counting and requests like "run this and tell me what it prints" start counting. See [Jev](#jev).
- **Live.** Sessions active in the last 90 seconds pulse green, and the board updates as they change.
- **Resume in a new tab.** `Enter` or a double-click opens `claude --resume` in a new tab, in the session's own folder, and leaves the board open. This works in tmux anywhere, and in Ghostty, iTerm2 and Terminal.app on macOS. In any other terminal, including every Linux terminal outside tmux, or with `R`, waygate-cs quits and resumes in the same window. If `angreal` is on your `PATH`, `A` resumes in it instead of Claude Code.
- **Search.** `/` fuzzy-matches titles and project names and searches the full text of prompts and replies.
- **Transcript.** `t` shows the whole conversation, with tool calls folded into one-liners. Click one to expand it.
- **Stats.** `s` shows sessions per week, cost by project, the most-used tools and the longest threads.
- **Catch me up.** `c` asks Claude (Haiku, through `claude -p`) for a three-line summary of the session and caches it. It only runs when you press the key, and costs a few cents.
- **Mouse.** Click, double-click, right-click for a menu, scroll any pane, drag the borders to resize at the ┇ grip (the widths are remembered), and click links to open them.

A tips box lists the less obvious controls at start-up. Press any key to close it, or `x` to stop showing it. Press `?` in the board for every key.

## Files

| Path | What |
|---|---|
| `~/.claude/projects/*/*.jsonl` | Claude Code's transcripts, read only |
| `~/Library/Caches/waygate-cs/index.json` (macOS) or `~/.cache/waygate-cs/` | Index cache, safe to delete |
| `~/Library/Caches/waygate-cs/waiting.json` (macOS) or `~/.cache/waygate-cs/` | Jev's answers, one per session update, safe to delete |
| `~/Library/Application Support/waygate-cs/state.json` (macOS) or `~/.local/share/waygate-cs/` | Done marks, pane widths, summaries, whether to show tips. Read from the older `waygate` or `cs` folder if this one is missing |

## Environment

| Variable | Default | |
|---|---|---|
| `CLAUDE_CONFIG_DIR` | `~/.claude` | Where Claude Code keeps its data |
| `WAYGATE_CS_CLAUDE` | `claude` | The binary to launch |
| `WAYGATE_CS_RESUME` | | Set to `here` to always resume in place |
| `WAYGATE_CS_SUMMARY_MODEL` | `haiku` | Model for catch-me-up summaries |
| `WAYGATE_CS_JEV_KEY` | | API key for Jev. Setting it turns Jev on |
| `WAYGATE_CS_JEV_URL` | `https://api.orcarouter.ai/v1/systemone` | Jev endpoint |
| `WAYGATE_CS_JEV_MODEL` | `typesafe/jev-1.13` | Jev model |

`waygate-cs --list` prints sessions as plain text for scripts.

## Jev

Jev is TypeSafe's yes/no model. When `WAYGATE_CS_JEV_KEY` is set, waygate-cs asks it about every session from the last 14 days where Claude spoke last, isn't live and isn't marked done: does Claude need something from you before the work can go on? A session is waiting at a probability of 0.5 or more.

Each request sends the last 1,000 bytes of Claude's last reply and nothing else from the transcript. Before sending, waygate-cs masks anything that looks like a credential: common key prefixes (`sk-`, `ghp_`, `AKIA` and others), `Bearer` and `Basic` tokens, values of names like `API_KEY=` or `"password":`, secret-named URL parameters, and long runs of mixed letters and digits. Masking catches common shapes, not every secret, so leave Jev off if your sessions handle credentials that must not leave the machine. Each session is asked once per update, in the background, and the answer is cached. Sessions not answered yet, or whose request failed, use the question-mark check. A rejected key turns Jev off until the next start.

The defaults point at [OrcaRouter](https://www.orcarouter.ai/models/typesafe/jev-1.13), so an OrcaRouter key works as is. For TypeSafe's own API, set `WAYGATE_CS_JEV_URL=https://api.typesafe.ai/v1/systemone` and `WAYGATE_CS_JEV_MODEL=jev-latest`.

## License

MIT
