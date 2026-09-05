# Captured hook input

Real hook stdin, captured from Claude Code on 2026-09-04 (macOS, arm64) with a scratch probe
plugin, and kept here so the plugin's hook scripts can be tested against the shapes the platform
actually sends rather than against the shapes the documentation describes. Neither payload's
field set matches the published reference exactly, which is why they are captured rather than
written by hand.

Five values are placeholders, not what was captured: `session_id`, `transcript_path`, `cwd`,
`prompt_id`, and (in the `PreToolUse` payload) `tool_use_id`. They carried a local absolute path
and live session identifiers. Every field name, every field order, and every other value is
exactly as captured — the scripts extract by field name, so the shape is the whole point. The
`SessionStart` payload carries only three of those five, because only three are in it.

- **`user-prompt-expansion.json`** — the decisive one. `prompt` carries the raw user line
  (`/upe:expand hello`), not the expanded command body: this event fires *before* expansion, so a
  hook on it cannot see whether the command's own delivery slot already rendered its rules. That
  is why `dependency-halt.sh` confirms presence in one line instead of injecting rules.
- **`pre-tool-use-skill.json`** — `tool_input.skill` carries the namespaced skill name. The
  `arguments` and `input_context` keys are absent when the call passes none.
- **`session-start.json`** — captured on a `startup` session. Its field set is **smaller than the
  published reference**: `session_id`, `transcript_path`, `cwd`, `hook_event_name`, and `source`,
  and nothing else. There is no `permission_mode`, no `agent_id`, and no `agent_type`. A sample
  written from the documented field list would have carried keys the platform does not send, which
  is the reason to capture rather than synthesize. `session-start.sh` reads only `cwd`.

Also measured at capture time and not visible in these files: a `UserPromptExpansion` matcher
accepts an anchored regex against the namespaced command name (`"^mochiko:"` matches
`mochiko:brainstorm`).
