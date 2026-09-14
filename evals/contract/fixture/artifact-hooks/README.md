# Captured hook input — the artifact gate and the seat reminder

Real hook stdin, captured on 2026-09-13 (macOS arm64, Claude Code 2.1.258) by the wave-0 transport
probe. Provenance per file, with the run id and entry tag it came from, is in the session's own
evidence home: `.mochiko/brainstorms/hook-enforced-artifact-schema/wave0-fixtures/README.md`. These
are copies of those four payloads, kept here so the cases stage from `evals/contract/fixture/` like
every other fixture.

**This directory is deliberately not `fixture/hook-input/`.** `load_captures()` globs that directory
and indexes by `hook_event_name`, first sorted filename winning. Three of these payloads are
`PreToolUse`, and `pre-tool-use-bash-heredoc.json` sorts before `pre-tool-use-skill.json` — dropping
them in there would silently hand the dependency-halt case a Bash payload where it expects
`tool_input.skill`, and its per-skill rows would stop testing what they name. Separate directory,
separate loader.

| file | what it carries | consumed by |
|---|---|---|
| `pre-tool-use-bash-heredoc.json` | `tool_input.command` with a real `cat > … <<'X'` heredoc, newlines intact | `G-BASH-HEREDOC` |
| `pre-tool-use-bash-redirect-false-allow.json` | the redirect whose **escaped quotes** defeated the shipped wrapper's `grep`/`sed` field extraction, producing a false allow and a created file | `G-BASH-REDIRECT` |
| `pre-tool-use-read-home.json` | `Read`'s `tool_input.file_path` — the key the published hooks reference documents nowhere | `G-READ` |
| `subagent-start.json` | a `SubagentStart` payload: `session_id · transcript_path · cwd · prompt_id · agent_id · agent_type · hook_event_name`, and **no `permission_mode`** | every `reminder-input` row |

Each case substitutes only the fields that bind the payload to its workspace — `cwd`, and the path
inside `file_path` or `command` — so the field set, the field order and the escaping stay whatever
the platform actually sent. The redirect payload's quoting is the whole point of keeping it: a
hand-written fixture would not reproduce the defect it guards against.

One value is a placeholder rather than what was captured: the absolute home-directory prefix in
`transcript_path`, replaced with `/PLACEHOLDER-HOME`. Nothing else was changed.
