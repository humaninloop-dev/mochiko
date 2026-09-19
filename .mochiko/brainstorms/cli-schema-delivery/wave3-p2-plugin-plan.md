# P2 — plugin-side plan (wave 3)

## 1. The measurement (F14) — **branch B**

Three headless host runs, scratch plugin `evals/.work/p2-probe/upe`, haiku.
- **`UserPromptExpansion` stdin:** `session_id` · `transcript_path` · `cwd` · `prompt_id` · `permission_mode` · `hook_event_name` · `expansion_type: "slash_command"` · `command_name: "upe:expand"` · `command_args: "hello"` · `command_source: "plugin"` · `prompt: "/upe:expand hello"`. **`prompt` is the raw user line** — no command body, and the `!` line's marker (`PROBE-EXPANDED-MARKER`) absent. The hook fires before expansion.
- **Branch rule:** the preamble head line can never appear in `prompt`, so A is impossible. **Build B** — a one-line presence confirmation, rules never injected; no double delivery, so abort criterion (2) is untouched.
- **`PreToolUse`/`Skill` stdin:** the same first five fields plus `tool_name: "Skill"` · `tool_input.skill: "upe:probe-skill"` · `tool_use_id`. No `arguments`/`input_context` when unset.
- **Matcher (third run):** `"matcher": "^upe:"` fired on `UserPromptExpansion` — an anchored regex matches the namespaced command name, so `"^mochiko:"` is safe.
- Both captures go to `evals/contract/fixture/hook-input/` in Phase 2. No tokens, but they carry my home path, transcript path and three ids. **I will placeholder those five values, keeping every field name and shape** (P3 tests shape only). Say so if you want them byte-verbatim.

## 2. `plugins/mochiko/hooks/hooks.json`

Auto-discovered; no manifest change. Every entry `type: command`, `timeout: 5`, command `"${CLAUDE_PLUGIN_ROOT}/hooks/scripts/<name>.sh"`.
```json
{ "hooks": {
  "SessionStart":        [ { "hooks": [ { … "session-start.sh" … } ] } ],
  "UserPromptExpansion": [ { "matcher": "^mochiko:", "hooks": [ { … "dependency-halt.sh" … } ] } ],
  "PreToolUse":          [ { "matcher": "Skill",     "hooks": [ { … "dependency-halt.sh" … } ] } ] } }
```

## 3. Scripts — POSIX `sh`; `grep`, `sed`, `tr`, `command -v` only (no `jq`, no `awk`)

Stdin read once into `$input`. One shared extractor `field <key>` — `tr -d '\n'`, then `grep -o` the `"key": "value"` pair, `head -n 1` so a value echoed inside `prompt` or `command_args` cannot shadow the real key, then `sed` out the value. JSON values escaped by `sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' | tr '\n' ' '`.

**(a) `session-start.sh`** — no matcher, always `exit 0`, never blocks. Absent binary → the §3.2(a) install line. Present → `mochiko-cli --version` (which already prints `mochiko-cli <v> · grammar <a>..<b>`) plus `migrate status --plugin-root "$ROOT"`: exit 3 → its output verbatim (the D5 line); exit 0 → `<version line> · plugin <p> · log grammar <g> · in range`, `g` sed'd from the status line, `p` from `$ROOT/.claude-plugin/plugin.json` via `field`; any other exit → print both (log absent, empty or unsound — loud, blocks nothing). Then best-effort: `disableSkillShellExecution` in `$HOME/.claude/settings.json` or `<cwd>/.claude/settings.json` → one unsupported-environment line (GI-020), detection failure silent.

**(b) `dependency-halt.sh`** — one file, both registrations.
1. `field hook_event_name`: `UserPromptExpansion` → `name=$(field command_name)`, `file=$ROOT/commands/${name#mochiko:}.md`; `PreToolUse` → `name=$(field skill)`, `file=$ROOT/skills/${name#mochiko:}/SKILL.md`; anything else → `exit 0`.
2. `case "$name" in mochiko:*) ;; *) exit 0 ;; esac`.
3. **Converted check:** `grep -q '!`mochiko-cli rules' "$file"` fails → `exit 0`, silent. The `.md` is the truth, no list. Every skill fails this at wave 3, so the `Skill` limb is a tested no-op.
4. **Presence:** `command -v mochiko-cli` fails → block with `mochiko-cli is not installed — /$name cannot run without it. Install: cargo install mochiko-cli`.
5. **Range:** `migrate status` exit 3 → block with its output verbatim. Any other non-zero → `exit 0` (fail-open; only absence and skew gate).
6. Else **branch B** context: `mochiko-cli present · rules delivered by the command's own render`.

Block/context shapes — UPE block is `printf '%s\n' "$msg" >&2; exit 2`; every other path exits 0 with one `{"hookSpecificOutput":{"hookEventName":"<event>", …}}` object carrying `additionalContext` for context, or `permissionDecision: "deny"` + `permissionDecisionReason` for the `PreToolUse` block. No judgment, no dispatch, no behavior gate (D7, GI-019); the 5-second bound is the `timeout` field, and a hung binary hits it and fails open.

**(c) maintainer hook** — repo `.claude/settings.json`, never shipped. `PostToolUse` on `Edit|Write`: if the stdin `file_path` is under `plugins/mochiko/migrations/`, run `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` and print it; else nothing. Exit 0 always. The file today holds only `enabledPlugins`, so this is a pure addition.

## 4. `commands/brainstorm.md`

Frontmatter gains `allowed-tools: Bash(mochiko-cli *)`, nothing else. `## Identity & Mission` byte-identical. `## Rules — load the schema first` replaced whole by `## Rules — delivered by mochiko-cli`: the §3.1 halt paragraph verbatim, then the seven `!` lines in §3.1's order, each with `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`; nothing else. Protocol steps 1–2 byte-identical; step 3 replaced by §3.1's Q-B text (no hand pin; the pin is the preamble's `- kind: fail · N rules` line, which `render.rs` emits as exactly that string). Schema verified: 6 live sections + 2 tombstones (`harness`, `bindings`), 4 `kind: fail`, 7 `class: floor`, 7 `extends: common.*`.

## 5. Strips (`.mochiko/strips/brainstorm.md`, newest-first, `[v0.104.0]`)

Two supersession-by-ruling entries citing record D3 as amended, the 2026-09-04 Q-A/Q-B rulings, and the `DECISIONS.md` row. **(1) The Rules block** — superseded → the seven `!` lines + halt clause; verbatim content including the reading-grammar prose (now the CLI's preamble `legend`) and the `common.yaml` co-Read obligation (now resolved in the render); kept: Identity & Mission, Entry, Goal; consumers: none shared. **(2) The hand-pinned count** — superseded → the CLI-printed pin; verbatim the two old step-3 sentences; consumers assessed: `primitive-edits.md` criterion 3.

## 6. `.claude/rules/mochiko/primitive-edits.md`

`paths` gains `plugins/mochiko/migrations/**` and `plugins/mochiko/hooks/**`. **Criterion 3** gains the ruled converted branch: no hand-pinned count; the count is the CLI-printed preamble pin, the `.md` cites it and obliges halt-and-surface on an end-line disagreement; grade the citation and the halt, never a number — fail-rule survival and the two-way `.fail.*`/`kind: fail` correspondence stand. **"Schema data files"** gains one sentence: from v0.104.0 a converted command's rules are served from the migration log at `plugins/mochiko/migrations/`, and editing that content is a new migration file under the log (grammar in its README), never an in-place edit.

**Two companion conflicts, flagged not fixed** — V2 grades the whole set, and two clauses fail a correctly converted command as written. **Criterion 1** fixes the heading as `## Rules — load the schema first`; §3.1 renames it (proposed: *on a converted command the heading reads `## Rules — delivered by mochiko-cli`*). **Criterion 11**'s last clause obliges a raw `common.yaml` Read where a stub binds; brainstorm has 7 and the render resolves them (proposed: *on a converted command stub resolution happens in the render; no raw common-file Read is demanded*). One sentence each, same v3.0.1 PATCH row. Approve or decline — I touch neither otherwise, and name them in my report as V2 failures by construction if declined. Criterion 2 needs no edit; §8 already reads the enumeration as the seven `--section` arguments.

## 7. `README.md`

**Install** becomes two steps: the two existing marketplace lines unchanged, then the binary, with the plain statement that the plugin depends on it from this version and converted commands halt without it — install line `cargo install --git https://github.com/humaninloop-dev/mochiko mochiko-cli` until the publish (Q-C). **"The template-schema CLI (optional)"** is retitled and rewritten under it: what the binary serves (rules and templates replayed from the migration log the plugin carries), the `rules`/`template`/`migrate` usage, the resolution order. Every "optional", "no binary dependency", and "agents Read those YAML files raw" claim is struck, the intro's "markdown-only" framing with them; no raw-YAML-when-absent sentence survives. `cargo install --path crates/mochiko-cli` stays, marked maintainer-side.

## 8. Order, checks, deviations

Order: `brainstorm.md` → hooks → strip → rules file → README → maintainer settings. Before I report: JSON parse on `hooks.json` and `settings.json`; `sh -n` on all three scripts; executable bits; both `!` renders exercised against the plugin log once P1 lands it; nothing under `plugins/mochiko/` outside my §1 row touched. **Deviations:** the `SessionStart` other-non-zero branch prints rather than staying silent; the fixture captures are placeholder-sanitized. **`mochiko-cli` is not on my `PATH`** — I author against the source and cannot run it end to end; P3's cases exercise it.
