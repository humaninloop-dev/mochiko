# V2 — plugin-side audit (wave 3, `cli-schema-delivery`)

**Verdict: PASS.** All seventeen items pass, graded against `primitive-edits.md` as it now stands, `rust-cli.md`, wave3-plan §3 and §8, and record
D3/D7 — on the files and on branches I exercised myself, never on P2's report. Six findings below; none blocks the landing. Binary under test:
`mochiko-cli 0.1.0 · grammar 1..1`, built here and staged off-tree so I could control its presence on `PATH`.
`git diff --stat -- plugins/mochiko/schemas/` is empty, so the shipped schema is byte-unchanged this wave.

## A. The `brainstorm` pair

**1. Scaffold (criterion 1 as amended) — PASS.** Frontmatter carries `description`, `argument-hint`, `disable-model-invocation: true` and the added
`allowed-tools: Bash(mochiko-cli *)`. Headings in order: `# Brainstorm — Think Together, Review Cold` · `## Identity & Mission` · `## Rules —
delivered by mochiko-cli` · `## Adaptive Goal Protocol`, steps Entry, Goal, Not done last. No extra top-level section, no `schemas/` path anywhere.

**2. Byte preservation — PASS.** Diffed against `git show HEAD:`: `## Identity & Mission` identical at 667 bytes, Entry plus Goal at 668 bytes.

**3. Halt clause and the seven `!` lines — PASS.** The halt paragraph, stripped of the plan's blockquote markers, diffs empty against wave3-plan
§3.1 across all ten lines; the seven `!` lines diff empty against the plan's block, in order, each with `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`.
Nothing else is in the section: 19 non-blank lines, being the heading, the paragraph, the seven lines and the next heading.

**4. Criterion 2 enumeration — PASS.** Live section ids are `roles`, `reserved`, `tools`, `ways-of-working`, `boundaries`, `fail-conditions`, with
`harness` and `bindings` under `tombstones:`. The seven `--section` arguments are those six plus `preamble`, matching set-wise, and every
`brainstorm.sec.*` token in the `.md` is one of the six.

**5. Criterion 3 as amended — PASS.** Step 3 carries no number: it cites the `kind: fail` line under `pins` and obliges a halt when a delivered
end-line count disagrees. The schema has four `kind: fail` rules and exactly four `brainstorm.fail.*` ids, so the correspondence holds both ways.
The fifth `kind: fail` text match is prose in a section `intent`, not a rule key — see F1.

**6. Criteria 4, 5, 8 — PASS.** No ID vanished; the schema is byte-unchanged. Both supersessions are recorded as `[v0.104.0]` entries in
`.mochiko/strips/brainstorm.md`, prepended above the `[v0.100.0]` block, each carrying Disposition, `Tier failed: n/a — supersession by ruling`,
verbatim Content, Kept deliberately and Consumers assessed. Diffed against `git show HEAD:`, both fenced blocks match exactly.

**7. Criterion 9, the D13 checker — PASS.** Run as `uv run --with pyyaml python3 scripts/check-command-schema.py --schema
plugins/mochiko/schemas/brainstorm.yaml --md plugins/mochiko/commands/brainstorm.md`:

```
FINDING: brainstorm.md: no Not-done line hard-coding the `kind: fail` count — want 'the 4 rules of `kind: fail`' (D7 C2 guard, re-keyed at build item 4)
FINDING: brainstorm.md: canonical heading `## Rules — load the schema first` absent (scaffold D2)
check-command-schema: 2 findings, 9 warnings — FAIL (advisory)
```

Both are of the conversion-expected class, and I confirmed the delta rather than assuming it: the same checker against the HEAD `.md` reports
`0 findings, 9 warnings`, the nine identical in both runs. The two are exactly the substitutions amended criteria 1 and 3 license.

**8. Char-budget pre-assert — not applicable, verified.** `primitive-cost-budgets.md` records the ruling under "Unbudgeted primitives": "all
commands have no per-primitive budget".

**9. Delivery — PASS.** All seven blocks render at exit 0 against `--plugin-root plugins/mochiko`. Every head line reads `mochiko-cli rules
brainstorm · section <id> · binary 0.1.0 · grammar 1 · plugin 0.103.0`, every end line `mochiko-cli rules end · brainstorm · <id> · <N> rules`, both
in the halt clause's exact shape. The preamble prints `- kind: fail · 4 rules`, the fail-conditions end line `4 rules`: pin and delivery agree.

## B. The hooks

**10. `hooks.json` per F14 — PASS.** It parses, and a scripted assertion confirms the top level is exactly `{"hooks": …}`; `SessionStart` has no
matcher, `UserPromptExpansion` matches `^mochiko:`, `PreToolUse` matches `Skill`; every hook is `type: command`, `timeout: 5`, command under
`${CLAUDE_PLUGIN_ROOT}/hooks/scripts/`.

**11. D7 conformance from the scripts — PASS.** `dependency-halt.sh` blocks on two conditions only: `command -v mochiko-cli` failing, and `migrate
status` exiting 3; every other non-zero status exits 0 silently, so the gate is absence and grammar skew and nothing else. Nothing is graded or
dispatched, and no schema path appears in either script. The converted check is `grep -q -F '!\`mochiko-cli rules' "$primitive"` against the
primitive's own `.md`; a name outside `mochiko:` exits 0 before it, `session-start.sh` ends in `exit 0` on every path, both are POSIX `sh` with no
`jq` and no `awk`, and all three scripts pass `sh -n` and carry the executable bit.

**12. Branches exercised here — PASS.** Fixture captures on stdin, `CLAUDE_PLUGIN_ROOT=plugins/mochiko`, the primitive name substituted in.

| case | exit | decisive output |
|---|---|---|
| (a) `mochiko:brainstorm`, binary absent | 2 | stderr `mochiko-cli is not installed — /mochiko:brainstorm cannot run without it. Install: cargo install mochiko-cli` |
| (b) same, binary present | 0 | stdout parses; `additionalContext` = `mochiko-cli present · rules delivered by the command's own render` |
| (c) `mochiko:specify` (unconverted) | 0 | no output, binary absent and present alike |
| (d) `PreToolUse` skill `mochiko:review-brainstorm` | 0 | no output |
| (e) `session-start.sh`, present / absent | 0 / 0 | `mochiko-cli 0.1.0 · grammar 1..1 · plugin 0.103.0 · log grammar 1 · in range` / `… Install: cargo install mochiko-cli` |

Four beyond the brief, all as designed: a foreign-namespace command and a `SessionStart` payload fed to the dependency hook both exit 0 silently;
against a log forced to `grammar: 99` the command limb exits 2 and the skill limb returns a parsing `permissionDecision: "deny"`, both forwarding
the binary's own message, while `session-start.sh` prints it and still exits 0.

**13. Fixture captures — PASS.** Three one-line JSON files, the `SessionStart` one being what drives case (e) above. No home path, no tokens and no
live ids: `session_id`, `transcript_path`, `cwd`, `prompt_id` and `tool_use_id` are placeholders. Field names, order and every other value are
intact, and the scripts extract by name. The README names those five fields and the `SessionStart` capture's smaller-than-documented field set.

## C. Rules file, README, maintainer hook

**14. `primitive-edits.md` — PASS.** `paths` gains exactly `plugins/mochiko/migrations/**` and `plugins/mochiko/hooks/**`. The diff is 24 insertions
and 4 deletions, each deleted line re-emitted verbatim with a clause appended, so nothing was removed. All three converted clauses open with "On a
converted command", nothing changes for the five unconverted ones, and the "Schema data files" paragraph gains one sentence. See F4.

**15. `README.md` — PASS.** Install is two required steps, the second stating the dependency and the halt, with the line `cargo install --git
https://github.com/humaninloop-dev/mochiko mochiko-cli`. No survivor of "optional" (the single hit is about output styles), "no binary dependency",
"markdown-only" or "Read those YAML files raw", and no bare `cargo install mochiko-cli`. Its log resolution order matches `resolve_log_dir`. See F6.

**16. Maintainer hook — PASS.** `.claude/settings.json` and `.claude/hooks/validate-migrations.sh` are repo-side; nothing was added under `plugins/`.
`PostToolUse` on `Edit|Write`, acting only when the extracted `file_path` matches `*plugins/mochiko/migrations/*`, always exit 0, `sh -n` clean. All
three branches driven: in-scope prints the validator report at 0, out-of-scope is silent at 0, in-scope with the binary absent is silent at 0.

**17. Report honesty — PASS.** `git status --short -- plugins/` shows only `commands/brainstorm.md`, P1's `migrations/` and the new `hooks/`. My
independent per-block measurement sits exactly one character and one byte above P2's on every one of the seven blocks — the trailing newline the
report says it strips — so my 10,513 characters and 10,700 bytes reconcile with its 10,506 and 10,693. Disclosed deviations match the code.

## Findings

1. **F1 — stale prose in the delivered payload (minor, lead follow-up).** The rendered `brainstorm.sec.fail-conditions` block opens with the section
   intent "…the .md Not-done line hard-codes this set's count", which this wave falsified. It misdescribes the `.md` rather than waiving an
   obligation, and both surfaces still say four, so no spurious halt follows. Not a criterion failure: amended criterion 3 grades the citation and
   the halt clause, and the schema is byte-unchanged by design. The fix is a new migration file, as P2's own added sentence prescribes.
2. **F2 — the D13 checker is not cited in P2's report (minor).** Criterion 9 obliges it in the audit brief; discharged here, so carry it forward.
3. **F3 — the hook messages name an install line that does not work yet (observation).** Both scripts print `Install: cargo install mochiko-cli`
   while the README carries the git-install line under Q-C. P2 is conformant, since plan §3.2(a) specified that string, but the two surfaces
   disagree for users until the publish, which is the lead's call.
4. **F4 — the converted clauses' sentence counts (observation).** Criteria 1 and 11 carry one sentence each, as P2's plan §6 proposed, though
   criterion 1's folds in two substitutions beyond the heading rename — the `allowed-tools` key and a cross-reference to criterion 3, both from plan
   §3.1. Criterion 3's branch runs to three sentences, but the one-sentence bar covered only the two companion clauses and this branch is the ruled
   Q-B option. None adds an obligation beyond the plan, and none touches an unconverted command.
5. **F5 — two nits worth one edit each.** The README usage block shows `mochiko-cli migrate status` and `migrate validate` bare, and with the log
   inside the plugin there is no `./migrations` at a plugin user's working directory, so `--plugin-root` belongs on those lines. The maintainer
   hook's `timeout` is 30 rather than five, which is legal: D7's five-second floor binds shipped hooks, and this one never ships.
6. **F6 — the "Kernel-free" tagline (lead item, per the brief).** It survives at README line 3 while the crate is admitted kernel-class delivery
   infrastructure. Not graded; recorded so it does not go missing at the landing.

**Fix list:** none blocking. F1 is the only item with a code consequence, and it belongs to a follow-up migration, not to this wave's unit.
