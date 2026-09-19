# Wave 4 — the hook ship (staff-engineer seat; plan only, awaiting lead approval)

**Ruling home:** record D1 (as amended 2026-09-13 — the `SubagentStart` line), D3, D7, D9, D11
wave 4 · ledger AM-3 (GI-019 clause iv, the GI-012 wave-4 precondition, the amnesty paragraph and
its known gap) · `.claude/rules/mochiko/rust-cli.md` and `primitive-edits.md` as rewritten ·
`.mochiko/strips/README.md` · `wave0-probe-report.md` legs 5–7 · `wave4-contract-plan.md`.
**Scope line:** build everything, stage the bump, land nothing. The `plugin.json` bump that ships
these hooks MUST NOT land before the crate's first publish carrying all four controls (GI-012, AM-3
review C5); a maintainer break-glass install never substitutes for consumers.

## (a) `hooks/hooks.json` — two registrations, `if` on the handler objects

Wave 0 leg 7: an `if` on the matcher-group object is silently ignored and every matched call fires.
It sits beside `type` on the handler. Added to the existing `SessionStart` /
`UserPromptExpansion` / `PreToolUse`-`Skill` entries, leaving those untouched:

```json
    "PreToolUse": [
      { "matcher": "Write|Edit",
        "hooks": [
          { "type": "command", "if": "Write(.mochiko/**)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 },
          { "type": "command", "if": "Edit(.mochiko/**)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 },
          { "type": "command", "if": "Write(**/*.md)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 },
          { "type": "command", "if": "Edit(**/*.md)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 }
        ] },
      { "matcher": "Bash",
        "hooks": [ { "type": "command", "if": "Bash(*.mochiko*)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 } ] },
      { "matcher": "PowerShell",
        "hooks": [ { "type": "command", "if": "PowerShell(*.mochiko*)",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/artifact-gate.sh", "timeout": 5 } ] }
    ],
    "SubagentStart": [
      { "hooks": [ { "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/scripts/seat-reminder.sh", "timeout": 5 } ] }
    ]
```

**Why four handlers on `Write|Edit` rather than one.** `.mochiko/**` alone would gut D9: the sniff
exists to catch a report written *outside* every home, which is mostly outside `.mochiko/`.
`**/*.md` alone would miss a non-`.md` deliverable — `contracts/api.yaml` — where location and file
set still bind. Both patterns together cover each, and a `.mochiko/**/*.md` call matches two
handlers and fires the wrapper twice. The wrapper is stateless and reads only stdin, so a double
fire is two identical answers; a deny from either is the platform's answer. **Assumed, and wave 4
measures it:** double-fire behaviour and its cost against the ≤ 60 s aggregate watch (GI-012 note,
never a bump gate). If the double fire proves costly, the fallback is one unnarrowed handler and a
cost line, not a narrowing that drops the sniff.

**`SubagentStart` carries no matcher.** Wave 0: `agent_type` is `general-purpose` for named and
unnamed spawns alike, so a matcher cannot single out a teammate. **`PowerShell`'s `if` is
unprobed** — no PowerShell on macOS; the QA plan already records that row with `report()`, not an
assert.

## (b) The two wrappers — POSIX `sh`, fail-open, no JSON parsing

**`hooks/scripts/artifact-gate.sh`.** The wave-0 false allow (leg 3, `r0-retry-2`) came from the
shipped `field()` helper truncating a command at an escaped quote. This wrapper therefore parses
**nothing**: it pipes stdin through untouched and lets the Rust side read the payload.

```sh
command -v mochiko-cli >/dev/null 2>&1 || { allow; exit 0; }        # fail-open, D7 floor
out=$(mochiko-cli check --hook-json - --plugin-root "$ROOT" 2>/dev/null); code=$?
[ "$code" -eq 4 ] && { printf '%s\n' "$out"; exit 0; }              # the only deny
allow; exit 0                                                        # 0,1,2,3 and anything else
```

`allow` prints the explicit-allow JSON — `{"hookSpecificOutput":{"hookEventName":"PreToolUse",
"permissionDecision":"allow"}}` — because the platform denies a background subagent's call when no
hook returns a decision (D3/C2, wave-0 platform fact). Exit 0 on the CLI already prints its own
allow JSON (with `additionalContext` where amnesty applied), so exit 0 echoes `$out` when it is
non-empty and falls back to the bare allow when it is not. The wrapper's own exit status is always
0: a non-zero wrapper is a platform-level failure, not a verdict.

**`hooks/scripts/seat-reminder.sh`.** Prints, on a `SubagentStart` payload only:

```
{"hookSpecificOutput":{"hookEventName":"SubagentStart","additionalContext":
 "mochiko gate: artifacts under declared homes take their shape from `mochiko-cli home <path>`; existing files are not templates."}}
```

That line is frozen — the contract suite reads it as a golden (`R-LINE-EXACT`), so it is authored
once here and never reworded without a row change. Event check is a **presence test**, never field
extraction: `grep -q '"hook_event_name"[[:space:]]*:[[:space:]]*"SubagentStart"'`, silent exit 0
otherwise (`R-WRONG-EVENT`). No binary on `PATH` → silent exit 0, no line (`R-ABSENT`). It emits no
`permissionDecision` key at all (`R-INJECT`), and nothing at the top level (`R-NO-TOPLEVEL`).

Both wrappers carry `timeout: 5` and fail open on absence, timeout, or any unexpected output — the
D7c hook floor, re-ratified for these two at AM-3.

## (c) Shipped-primitive edits — five, three of them taking a strip entry

I resolved every `.mochiko/` path named in `commands/`, `agents/`, `skills/` and `templates/`
against the shipped homes with `mochiko-cli home <path>`. Twenty-five resolve as declared
deliverables or as directories; **three name a path the homes do not carry**, and two skills need
the D1a rule that no `mint-rule` op could reach:

| # | primitive | edit | strip |
|---|---|---|---|
| 1 | `skills/patterns-entity-modeling/SKILL.md` | add the authoring-time home rule as markdown — the log carries no schema document for this skill, so wave 3's mint could not reach it | none — additive |
| 2 | `skills/patterns-api-contracts/SKILL.md` | same | none — additive |
| 3 | `skills/executing-tdd-cycle/SKILL.md` | its `description:` names `.mochiko/specs/<feature>/tasks.md`; `tasks.md` is a `feature`-home deliverable at `.mochiko/features/<FEAT-ID>/` | **supersession-by-ruling** |
| 4 | `templates/analyst-report-template.md` | output location `.mochiko/specs/<feature>/analyst-report.md` — an undeclared name in the spec home; it is a report, so it lands in that home's `reports/` under a `report:` type | **supersession-by-ruling** |
| 5 | `templates/techanalyst-report-template.md` | same, for `techanalyst-report.md` | **supersession-by-ruling** |

The three re-points are decisions, not altitude cuts: the line was *made wrong by the wave-3 homes
ruling*, which is the strips README's supersession form (`Disposition: superseded → <the rewrite>`,
`Tier failed: n/a — supersession by ruling`, citing record D2/D3 and `0005`). Edit 3 touches a
budgeted class (`description:`), so it takes the char-budget pre-assert before the audit;
`executing-tdd-cycle` has **no headroom** at 20,063, so the re-point must not lengthen the value.

**One ruling owed inside (c):** the envelope enum is cycle · verification · final-validation ·
review · feasibility · disclosure. Both analyst reports are producer disclosures carrying
assumptions and open questions, so `report: disclosure` is my proposal; the alternative — declaring
the two names as spec-home deliverables — takes a sixth migration and re-opens the census.

No command `.md` needs a `Tools` or `Not-done` re-point: every command's rules come from the log,
and `impl.reports-envelope` was already re-keyed in `0005`.

## (d) The crate fix — first-touch amnesty over the file set

AM-3's amnesty paragraph makes **file set relaxable**: an undeclared name in a declared home, on an
existing file, is amnestied where the write does not worsen it, and "existing files are never
wedged". **Reproduced against the built binary today, both legs deny:** a `Write` and an `Edit`
over an existing `.mochiko/specs/<slug>/notes.md` each return exit 4 with the undeclared-file
reason. So the gap is wider than the ledger's "bare allow, no `additionalContext`" — today there is
no allow at all, and a mis-homed file cannot be edited.

The fix: route `Resolution::UndeclaredFile` through `settle` with a `file-set:<name>` fault key, so
a baseline carrying the same fault excuses it and the allow names it in `additionalContext`; a
fresh write at an undeclared name still denies, because there is no baseline. Tests: the four cells
(fresh Write denies · Write over an existing undeclared name allows with the name in context · Edit
likewise · a *different* undeclared name in the same home still denies), plus a contract row. This
is the one `src/` change wave 4 takes, and it goes to an independent non-author code review.

## (e) Audit plan — author ≠ grader on every edited surface

- Each of the five edited primitives: `mochiko:validator` against its own pair — the `.md` plus the
  rules `mochiko-cli` renders for it — held to the canonical-scaffold criteria in
  `primitive-edits.md`, with the char-budget pre-assert first on edit 3.
- The two wrappers and `hooks.json`: reviewed by a non-author seat against this plan's (a) and (b),
  the wave-0 facts they encode, and the QA rows they must satisfy.
- The crate fix: an independent non-author code review, per `rust-cli.md`.
- The strip entries: checked as part of each primitive's audit, not separately.

## (f) Release mechanics — staged, not landed

| step | value |
|---|---|
| `plugins/mochiko/.claude-plugin/plugin.json` | 0.108.0 → **0.109.0** (MINOR: new hooks, no removal) |
| `.claude-plugin/marketplace.json` | synced to 0.109.0 in the same commit (gate 5) |
| `CHANGELOG.md` | one entry: the two hooks, the five primitive edits, the crate fix, the precondition |
| gate 6 | `cargo test` PASS **plus the contract suite's full deterministic set** — the 78 sandboxed cases and QA's new rows, not the host four; a SKIPPED suite is not green |

**Precondition, stated once and carried on the commit:** this bump does not land until
`mochiko-cli` has published with all four controls, including the two still owed — signed release
tags and the `crates-io` environment's approval rule (the publish job is `if: false` at
`.github/workflows/release.yml:100`). Until then the work sits staged on the branch.

## (g) Ownership split with the QA seat — no shared file

| surface | owner |
|---|---|
| `evals/contract/**` — cases, rows, fixtures, `expected-skills.json` | QA |
| `plugins/mochiko/hooks/**` · the five `.md` primitives · `.mochiko/strips/*` | me |
| `crates/mochiko-cli/**` — the amnesty fix and its tests | me |
| `plugin.json` · `marketplace.json` · `CHANGELOG.md` | me, staged |
| the frozen reminder line | me (authored), QA reads it as a golden |

No floor id moves this wave — every edit is markdown or crate code, not a log rule — so the
contract freeze needs no re-key and the two seats share no file.

## (h) Plan-minimalism ladder

- **Two wrappers — required.** Two events, two output shapes; one script switching on the event is
  the `dependency-halt.sh` shape and it is what made that file hard to read.
- **No JSON parsing in the gate wrapper — required.** Wave 0's false allow is the evidence.
- **Four `Write|Edit` handlers — simpler shape.** The alternative that keeps both the sniff and the
  non-`.md` deliverables is an unnarrowed handler, which costs more on every call.
- **The amnesty fix — required rung.** The ratified paragraph says existing files are never wedged
  and the build wedges them.
- **A sixth migration for the two analyst reports — cut, minimum now.** Re-pointing two prose
  templates is smaller than re-opening the census, and `reports/` already admits them by name.
- **A `Task` spawn matcher — cut, required rung fails.** Wave 0 left the spawn tool's wire name
  unverified, and `SubagentStart` reaches every seat without one.

## (i) Question for the lead

The amnesty item is the one I would not decide alone. AM-3 ratifies file set as a relaxable measure
and says existing files are never wedged; the built binary denies every write to an existing
mis-homed file, and the ledger's own description of the gap (a bare allow with no context) does not
reproduce. **Do I implement D4e as ratified — an existing undeclared name becomes editable, with
the violation named in `additionalContext` — or keep the build's stricter deny and take the ledger
paragraph back to the user for correction?** The first makes the gate looser than it is today on
exactly the files wave 5's violator pass will rewrite; the second leaves a ratified sentence that
the binary does not honour.
