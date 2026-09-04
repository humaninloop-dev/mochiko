# P2 — plugin side: what was built and what was checked

**Seat:** P2, wave 3 of `cli-schema-delivery`. **Plan:** `wave3-reports/p2-plugin-plan.md`,
approved 2026-09-04 with the criterion-1 and criterion-11 companion clauses granted.
**Binary under test:** host release build, `mochiko-cli 0.1.0 · grammar 1..1`. **Log:**
`plugins/mochiko/migrations/` (P1's move, closed before these tests ran).

## 1. The measurement that set the branch

`UserPromptExpansion` receives the **raw user line** in `prompt`, not the expanded command body.
Measured twice: once on a scratch probe plugin, and again on the real converted `brainstorm`,
where the field came back as `/mochiko:brainstorm CONTRACT PROBE — …` with no rendered block in
it. The event fires before expansion, so a hook on it cannot see whether the command's own
delivery slot already rendered. **Branch A is impossible; branch B is what shipped** — the hook
confirms presence in one line and never injects rules. Injecting unconditionally would have
double-delivered on every fire and tripped abort criterion (2) by construction.

Two further platform facts settled along the way, both undocumented before this wave:

- A `UserPromptExpansion` **matcher accepts an anchored regex** against the namespaced command
  name. `"^mochiko:"` fires on `mochiko:brainstorm`, verified on the real plugin.
- `PreToolUse` on `Skill` carries `tool_input.skill` namespaced, with `arguments` and
  `input_context` absent when the call passes none.

A third capture followed at the lead's request, in one extra run: a real **`SessionStart`**
stdin. Its field set is **smaller than the published reference** — `session_id`,
`transcript_path`, `cwd`, `hook_event_name`, `source`, and nothing else. No `permission_mode`, no
`agent_id`, no `agent_type`. A sample written from the documented field list, which is what P3's
plan would otherwise have synthesized, would have carried keys the platform does not send.

All three captures are committed at `evals/contract/fixture/hook-input/`, with a README naming
the placeholdered values and this field-set discrepancy.

## 2. What was built

| file | change |
|---|---|
| `plugins/mochiko/commands/brainstorm.md` | `allowed-tools: Bash(mochiko-cli *)` added; Rules section replaced by the halt clause plus seven `!` lines; Not-done re-keyed to the CLI pin |
| `plugins/mochiko/hooks/hooks.json` | new — `SessionStart`, `UserPromptExpansion` (`^mochiko:`), `PreToolUse` (`Skill`), every hook `timeout: 5` |
| `plugins/mochiko/hooks/scripts/session-start.sh` | new — presence, version, range, policy detection; always exits 0 |
| `plugins/mochiko/hooks/scripts/dependency-halt.sh` | new — both registrations, absence-and-skew gating only |
| `.mochiko/strips/brainstorm.md` | two `[v0.104.0]` supersession-by-ruling entries, prepended |
| `.claude/rules/mochiko/primitive-edits.md` | two `paths` globs; converted branches on criteria 1, 3, 11; the schema-data-files sentence |
| `README.md` | Install rewritten as two required steps; the CLI section re-authored |
| `.claude/settings.json` + `.claude/hooks/validate-migrations.sh` | maintainer advisory hook, never shipped |
| `evals/contract/fixture/hook-input/` | three captures plus their README |

`Identity & Mission` and protocol steps 1 and 2 are byte-identical to the pre-edit file. No file
under `plugins/mochiko/` outside my row was touched; the shipped schemas are byte-unchanged, as
the transition clause requires.

## 3. Checks run, with their output

**Static.** Both hook scripts and the maintainer script pass `sh -n`; `hooks.json` and
`settings.json` parse; executable bits set on all three.

**Renders — all seven blocks, against the plugin log.** Every head line and end line landed in
the exact shape the halt clause names, every block far below the ~30,000-char inline ceiling.

| section | chars | bytes | rules |
|---|---|---|---|
| preamble | 2,054 | 2,101 | 0 |
| roles | 1,581 | 1,602 | 5 |
| reserved | 1,434 | 1,461 | 5 |
| tools | 1,668 | 1,695 | 7 |
| ways-of-working | 1,923 | 1,951 | 7 |
| boundaries | 724 | 742 | 1 |
| fail-conditions | 1,122 | 1,141 | 4 |

Total delivered: **10,506 characters** (10,693 bytes) against the 12,819 raw read the `.md` used
to obligate — an **18 % reduction**, with roughly 2,300 characters of headroom under abort
criterion (2). Characters and bytes diverge because the separator is a multibyte middot;
characters are the canonical measure. This reconciles with P3's independent figure of 10,513 to
within 7 — exactly one trailing newline per block, which their sum keeps and mine strips.

The rise above the lead's pre-wave figure of 10,088 is entirely P1's new `legend` block in the
preamble. The largest block sits at 7 % of the inline ceiling. The preamble prints
`- kind: fail · 4 rules`, and the fail-conditions end line reports `4 rules`: the pin the `.md`
now cites agrees with the delivery.

**Hook behavior — 20 paths, all as designed.** With the binary off `PATH`: a converted command
exits 2 with the install line on stderr; an unconverted mochiko command, a foreign command, a
mochiko skill, a foreign skill, and a non-`Skill` tool call each exit 0 silently; a decoy
`"command_name"` planted inside the prompt text does not shadow the real key. With the binary
present and in range: the dependency hook emits its one-line context and `session-start.sh`
prints `mochiko-cli 0.1.0 · grammar 1..1 · plugin 0.103.0 · log grammar 1 · in range`. Against a
log forced to `grammar: 99`, `migrate status` exits 3 and both hooks forward the binary's own
message verbatim — the command hook by exit 2, the skill hook by a `deny` object. Every emitted
JSON parses.

Driven by the committed `SessionStart` capture, `session-start.sh` prints the in-range line with
the binary present, the install line with it absent, and — with `cwd` pointed at a settings file
carrying `disableSkillShellExecution` — the unsupported-environment line beside the in-range line.
That last limb had not been exercised before the capture existed.

The `Skill` limb is a no-op at wave 3, as planned: no shipped skill carries a
`` !`mochiko-cli rules `` line, so the converted check exits 0. I exercised it forward anyway
against a synthetic converted skill, and its deny and context objects are both well-formed.

**End to end, on the real plugin.** One headless fire of `/mochiko:brainstorm` on haiku
delivered all seven blocks. The model's read-back named **all seven `class: floor` rule ids
exactly and nothing else** — `user-record-acceptance`, `author-grader-default-fail`,
`transport-floor`, and the four `fail.*`. That is P3's pre-registered bar met on the first
replicate. No schema file was read, no `common.yaml`, no `command-labels.yaml`, no Bash
permission denial, and the raw `` !`mochiko-cli `` literal did not pass through. All six
`mochiko:*` commands registered under `--plugin-dir`, so the wave-0 manifest quirk did not bite.

## 4. Three findings P3 needs before writing its cases

1. **`--output-format stream-json` does not carry the expanded prompt.** The rendered blocks are
   absent from every stream row, so an assertion of the form "all seven head lines present in
   what the model saw" cannot be made against the stream. They *are* present in the session
   transcript at the `transcript_path` the hook stdin reports, which is where I verified them.
2. **Hook rows are reported unevenly.** `SessionStart` appears as `hook_started` and
   `hook_response` rows carrying `stdout`, so its line is assertable from the stream.
   `UserPromptExpansion` produced **no stream row at all** despite firing — I proved firing with
   a marker file on a staged copy. An assertion that the dependency hook's line reached the model
   must use the transcript, not the stream.
3. **Two halt-clause phrases are now prose in the command body.** The strings
   `shell command execution disabled by policy` and `mochiko-cli rules not delivered` each appear
   once in a healthy transcript, because the halt clause names them. An assertion that either is
   absent will false-positive unless it is scoped past the clause itself.

## 5. Deviations from the plan, with reasons

- **The maintainer hook is a script, not an inline command.** Extracting `file_path` inside a
  JSON string inside `sh -c` produced escaping that was unreadable and could not be syntax-checked.
  It is now `.claude/hooks/validate-migrations.sh`, referenced through `$CLAUDE_PROJECT_DIR`.
  Same behavior, still advisory, still always exit 0, and now testable.
- **Both scripts derive the plugin root from `$0`,** falling back from `$CLAUDE_PLUGIN_ROOT`.
  Substitution into the hook's command string is documented and proven; export into the script's
  environment is not.
- **The dependency hook stays silent on a `migrate status` exit that is neither 0 nor 3.** The
  plan implied it would still confirm presence. It should not: with a broken or empty log the
  render will fail anyway, and claiming "rules delivered" there would be false. Gating is
  unchanged — still absence and skew only.
- **The presence line names the primitive kind.** The command path emits the planned string
  verbatim, `mochiko-cli present · rules delivered by the command's own render`; the skill path
  says `skill's` instead of `command's`, because on that limb the planned wording is wrong.
- **`session-start.sh` prints on a non-zero, non-3 exit** rather than staying silent — approved
  in advance. It blocks nothing, and silence would hide a broken log until the first fire.

## 6. Two things left for others

- **The README tagline still reads "Kernel-free agent-skill framework"** while the crate is now
  admitted kernel-class delivery infrastructure. Outside my scope; I did not touch it. I did
  repair one clause my own change falsified, "an obligated first read of the command's own rule
  schema", which now reads "the command's own rule set in front of the model before any work
  begins".
- **Commands carry no per-primitive budget** by the user ruling recorded in
  `primitive-cost-budgets.md`, so the char-budget pre-assert does not apply to this edit. The V2
  brief should say so rather than leave it looking skipped.

Nothing was committed. No git mutation was made by this seat.
