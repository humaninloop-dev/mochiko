# Wave-0 captured hook payloads

Real hook stdin, captured on 2026-09-13 (macOS arm64, Claude Code 2.1.258) by the wave-0 transport
probe (`../wave0-probe-report.md`) using throwaway plugins in that session's scratchpad. They live
here because the scratchpad is session-scoped and four of these payloads are load-bearing fixtures
for the wave-4 contract cases (`../wave4-contract-plan.md`); the shapes would otherwise have to be
re-captured. Each file is the payload exactly as the platform sent it, one JSON line, with one
exception noted below. `pre-tool-use-read-home.json` is also the evidence behind leg 2's finding
that `Read` carries `tool_input.file_path`, a key the published hooks reference documents nowhere.

| file | source: probe run · entry tag · fire | wave-4 row that consumes it |
|---|---|---|
| `pre-tool-use-bash-heredoc.json` | `r01-t1-dontask` · `ENTRY=broad` · `fires/1789279894-15791-broad-PreToolUse-Bash.json` | `G-BASH-HEREDOC` (deny) |
| `pre-tool-use-bash-redirect-false-allow.json` | `r0-retry-2` · `ENTRY=broad` · `fires/1789280577-27819-broad-PreToolUse-Bash.json` | `G-BASH-REDIRECT` (must-deny regression) |
| `pre-tool-use-read-home.json` | `r01-t1-dontask` · `ENTRY=broad` · `fires/1789279891-15713-broad-PreToolUse-Read.json` | `G-READ` (allow, no reminder) |
| `subagent-start.json` | `r0-substart` · `ENTRY=subagent` · `fires/1789281016-41290-subagent-SubagentStart-notool.json` | every `reminder-input` row |

**Why the redirect payload matters.** Its `tool_input.command` is
`printf 'retry probe' > "<absolute path>/probe-home/c.md"` — the command that the probe wrapper's
`grep`/`sed` `field()` helper truncated at the first escaped quote, producing a false `allow` and a
created file. It is the one cell in wave 0 where the gate failed to hold, and it is why record D3
requires `check --hook-json -` to parse the payload in Rust. Keeping it verbatim, escaped quotes
included, is the whole point: a hand-written fixture would not reproduce the defect.

**The `SubagentStart` shape.** Its key set is `session_id · transcript_path · cwd · prompt_id ·
agent_id · agent_type · hook_event_name` — **no `permission_mode`**, which is why it is captured
rather than synthesized from the documented field list. The `r0-substart` run produced two fires in
the same turn, one for an unnamed `Agent` spawn and one for a spawn given a name; they are identical
in every field except `agent_id` (`a2a3cf72c66e578e5` kept here, `a2e307af39787f03f` its twin), and
both carry `agent_type: general-purpose`. That identity is the evidence for the wave-0 finding that
the event's agent-type matcher cannot distinguish a named teammate from an unnamed subagent.

**Redaction — one substitution, and it does not achieve anonymity.** In every file the absolute
home-directory prefix `/Users/<user>` was replaced with `/PLACEHOLDER-HOME` inside `transcript_path`.
Nothing else was changed: every field name, every field order, every other value, and both session
ids are as captured. Be aware that this substitution is cosmetic — the same username still appears
a few characters later inside the project-slug segment of `transcript_path`, and inside `cwd` and
the redirect payload's own command text, because those are not home-directory prefixes and the
redaction instruction was deliberately narrow. The same paths already appear in
`../wave0-probe-plan.md` and `../wave0-probe-report.md`. Widening the redaction would break the
redirect fixture, whose absolute path is what the write-operator parse is tested against.
