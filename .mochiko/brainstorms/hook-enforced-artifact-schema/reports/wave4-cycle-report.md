---
report: cycle
wave: 4
seat: staff-engineer (se-wave1)
date: 2026-09-15
scope: the hook ship — two registrations, two wrappers, the file-set amnesty fix, five primitive edits, the staged bump
hooks_registered: PreToolUse Write|Edit (unnarrowed) · PreToolUse Bash and PowerShell (if narrowed on .mochiko) · SubagentStart (no matcher)
wrappers_new: hooks/scripts/artifact-gate.sh · hooks/scripts/seat-reminder.sh
crate_src_changed: src/conform.rs (the UndeclaredFile arm routes through settle) · src/hook.rs (the PowerShell write vocabulary)
primitive_edits: 5 (2 additive · 3 supersession-by-ruling with strip entries)
strips_written: executing-tdd-cycle.md · analyst-report-template.md · techanalyst-report-template.md
gate_cargo_test: PASS — 457 passed, 0 failed (453 before; two amnesty tests, two PowerShell tests)
gate_cargo_fmt: PASS
gate_cargo_clippy: PASS (--all-targets -D warnings)
gate_cargo_audit: PASS — 31 crate dependencies, 0 advisories
gate_migrate_validate: PASS — 0 rejecting · 104 advisory (unchanged; the log was not touched)
gate_contract_host: PASS — 7/7 host cases, including the QA seat's three new ones
gate_contract_sandbox: NOT RUN — 78 metered sandbox sessions; the lead's word, per wave 3
wrapper_median_ms: 31.5 (20 runs) · binary alone 27.0 · trigger is 100
per_run_projection_s: 6.6 and 5.0 on the two wave-0 sessions · cap is 60
plugin_json_staged: 0.108.0 to 0.109.0 — written into the tree, uncommitted and unreleased; no tag, no publish (the GI-012 precondition)
committed: no
powershell_vocabulary: 6 write cmdlets plus the shared redirects — the arm was inert before this wave
budget_finding: 11 skills stand over their payload budgets from wave 3's mints — ruled 11/11 HOLDS at audit unit 6, ledger stamped
audit_round_1: units 3-5 PASS · unit 6 11/11 HOLDS · units 1-2 FAIL (F1 dual homes · F2 an unrunnable fenced command)
corrections_from_audit_1: 2 of 2 — both prose sections rewritten to the ruled mint form; 11 ledger rows stamped v0.109.0
code_review_1: PASS — 5 Minor; W3/W4/W5 mine, folded into the same round; W1/W2 are QA's
shell_parse_gaps_disclosed: 3 — a cd into the home, a target in a shell variable, a relative write from inside a home
---

## What shipped

Two hook registrations in `plugins/mochiko/hooks/hooks.json`, two POSIX `sh` wrappers, one crate
fix, five primitive edits, three strip entries, and the release files staged at 0.109.0.

## The two lead calls, as implemented

**One handler for `Write|Edit`, no `if`.** The registration is a single un-narrowed handler. The
`if` narrowing survives only on `Bash` and `PowerShell`, both as `*.mochiko*`, where the volume is.
Cost consequence measured below. The plan's four-handler design and its double fire are gone, and
with them the `.mochiko/**` plus `**/*.md` pair that would have fired the wrapper twice on every
`.mochiko/**/*.md` write.

**D4e implemented as ratified.** An undeclared file name in a declared home is now amnestied where
the file already exists, and the violation is named in `additionalContext`. A new file at an
undeclared name still denies. The path is never relaxed: outside every home, and an undeclared
sub-directory, both stay as they were.

## The true pre-fix behaviour, for the ledger's factual correction

The ledger describes the gap as a bare allow with no context. It does not reproduce. Measured
against the binary installed before this wave, a `Write` over an existing undeclared name returned:

```
permissionDecision: deny · exit 4
`notes.md` is not a declared deliverable of `.mochiko/specs/<slug>/`. Declared: `spec.md`, ...
```

So the pre-fix behaviour was a **deny at exit 4**, on both `Write` and `Edit` — strictly worse than
the ledger says, and it would have wedged exactly the files the violator pass must rewrite. The
ledger sentence takes its correction in the pre-authorized PATCH at the bump, not here.

## The crate fix

`src/conform.rs` is the only `src/` file touched. The `Resolution::UndeclaredFile` arm built a bare
deny; it now builds one fault keyed `file-set:<name>` with no magnitude and hands it to `settle`
with the baseline. A baseline that exists carries the same fault at the same key, the
not-worsened comparison holds for two magnitude-less faults, and the write is allowed with the
standing violation reported. No baseline means no standing fault, so a fresh write denies.

Two module doc paragraphs were extended: check 2 now says the file set is relaxable, and the
amnesty section states the file-set-in / path-out boundary and why.

| cell | expected | measured |
|---|---|---|
| fresh `Write` at an undeclared name | deny | deny, reason names the file |
| `Write` over an existing undeclared name | allow + context | allow, context names the file |
| `Edit` over the same file | allow + context | allow, context names the file |
| a different undeclared name in the same home | deny | deny — an amnestied neighbour excuses nothing |

Two tests carry them: one at the conformance layer (`tests/conform.rs`), one end-to-end through the
payload with real files on disk (`tests/hook.rs`). Test count 453 to 457, with the PowerShell rows below.

## The PowerShell write vocabulary — the arm was inert

The QA seat's rows found it and the lead called it in: the `PowerShell` leg reached `check` — it
branches on `tool_name`, so it is host-testable — but the write-operator table was POSIX-shaped.
`Set-Content`, `Out-File`, `Add-Content` and `New-Item` all resolved to no target and **allowed**
every write into a home. The redirect row passed only because PowerShell shares `>` with the POSIX
shells.

`src/hook.rs` now keys the vocabulary to the tool name. The PowerShell table is `Set-Content`,
`Out-File`, `Add-Content`, `New-Item`, `Tee-Object`, and the `Copy-Item`/`Move-Item` pair, matched
case-insensitively, plus the shared `>` and `>>`. Named path parameters — `-Path`, `-FilePath`,
`-LiteralPath`, `-Destination` — name their target; a short list of parameters known to take a
non-path value has that value skipped, so a home path passed as `-Value` is content and not a
destination. Every other switch is treated as taking no value, which costs at most a missed source
on `Copy-Item -Force <src> <dst>` and never a false deny. Both ends of a move are targets, for the
same reason the POSIX `cp`/`mv` arm takes both.

`Tee-Object` is one row beyond the list the lead named. It is the direct analogue of `tee`, which
the POSIX table already carries, and leaving it out would have shipped a named evasion.

Two tests carry it: a 13-row deny matrix over every cmdlet, both redirects and a lower-case
spelling, with three allow controls — `Get-Content`, `Get-ChildItem`, and the false-deny control
where a home path rides as `-Value`; plus a cross-talk test proving a Bash line that merely names a
cmdlet is not measured against the PowerShell table.

**One QA row needs flipping.** `G-PWSH-NATIVE` is a recorded, unasserted row whose expectation
string still reads "the parse vocabulary has no PowerShell verbs — decision 'allow'". It now
reports `deny`. The suite stays green because the row is recorded rather than asserted, but the
expectation is stale and QA owns it.

## The wrappers

`artifact-gate.sh` parses nothing. It hands its own stdin to `mochiko-cli check --hook-json -`
untouched, prints the binary's line on exit 4 and on exit 0, and prints its own explicit allow for
1, 2, 3, a missing binary and anything unexpected. Its own exit status is always 0. The explicit
allow is load-bearing: the platform denies a background subagent's call when no hook returns a
decision, so silence would read as a deny on the transport every producing seat rides.

`seat-reminder.sh` tests for the event by presence over the whole payload, never by field
extraction, and prints one frozen line. It emits no `permissionDecision` key and nothing at the top
level. No binary on `PATH` means silent exit 0 — the line tells a seat to run the binary, so it is
worse than useless without it.

The frozen line, authored here and read by the suite as a golden:

```
mochiko gate: artifacts under declared homes take their shape from `mochiko-cli home <path>`; existing files are not templates.
```

## Wrapper probes — 12 cells, all as designed

| cell | result |
|---|---|
| fresh undeclared `Write` | deny, exit 4 |
| `Write` and `Edit` over an existing undeclared name | allow, context names the file |
| declared deliverable past its whole-file bound | deny, reason names lines and bound |
| `Bash` redirect into a home | deny, reason names the target |
| ordinary `Bash` | explicit allow |
| binary absent | explicit allow |
| unparsable payload | explicit allow |
| write outside every home | explicit allow |
| reminder on a `SubagentStart` payload | the frozen line |
| reminder on a `PreToolUse` payload | silent, exit 0 |
| reminder with no binary | silent, exit 0 |
| every cell | wrapper exit 0 |

## Cost, against the 60-second watch

| measure | value |
|---|---|
| wrapper, conforming path, median of 20 | 31.5 ms |
| wrapper, deny path | 30.9 ms |
| binary alone, same payload | 27.0 ms |
| shell overhead the wrapper adds | about 4.5 ms |

Per-run projection on the two sessions wave 0 measured, at the un-narrowed `Write|Edit` arm plus
the narrowed shell arm: 209 gated calls and 6.6 seconds on the larger session, 158 calls and 5.0
seconds on the smaller. Both sit an order of magnitude under the 60-second cap, and the per-call
median sits well under the 100 ms cache trigger.

The un-narrowed arm costs 5 extra calls on one session and 3 on the other against the narrowed
plan — about 0.16 and 0.09 seconds. That is the whole price of dropping the double fire and the
sniff contortions.

## Primitive edits

| # | primitive | edit | strip |
|---|---|---|---|
| 1 | `skills/patterns-entity-modeling/SKILL.md` | new Where the Artifact Lives section | none — additive |
| 2 | `skills/patterns-api-contracts/SKILL.md` | same | none — additive |
| 3 | `skills/executing-tdd-cycle/SKILL.md` | `description:` re-points `tasks.md` to the feature home | recorded |
| 4 | `templates/analyst-report-template.md` | output location into the spec home's `reports/` | recorded |
| 5 | `templates/techanalyst-report-template.md` | same | recorded |

Both new sections point at `mochiko-cli home <path>` and restate no budget and no file set. That is
deliberate: a number copied into a skill body is a second source of truth, and the three-row budget
amendment now with the user would falsify it.

The `report: disclosure` question the plan reserved answered itself. Both templates already carry
`report: disclosure` in their own skeleton frontmatter, and `disclosure` is already a member of the
envelope's closed type set. The re-point needed no new type and no sixth migration.

## Char-budget pre-assert

| primitive | class | measured | budget |
|---|---|---|---|
| patterns-entity-modeling | body | 14,355 | 16,835 |
| patterns-api-contracts | body | 11,877 | 13,412 |
| executing-tdd-cycle | `description:` | 501 | 623 |
| executing-tdd-cycle | delivered payload | 20,380 | 20,063 — over by 317 |

The plan said `executing-tdd-cycle` had no headroom for the re-point. That conflated two classes.
The `description:` is its own budgeted class at 623 and had 125 characters spare; the re-point spent
3. The payload overage is real but is **not** this edit's: the body is byte-identical to before this
wave.

## The budget finding — wave 3's mints, surfaced here

Sweeping every skill's delivered payload against the ledger found **11 over budget by 114 to 317
characters**, and they are exactly the 11 skills wave 3 minted an `artifact-home` rule into:
`analysis-codebase`, the eight authoring skills, `executing-tdd-cycle`, `patterns-vertical-tdd`.
Untouched skills sit around 237 characters **under**, so the mints cost roughly 350 to 550 each.

Nothing here is a fix I should make alone. The ledger's own precedent is `patterns-model-tiering`,
which carries a standing ruled-HOLDS overage of 3,844 from the sonnet-worker-rung mints, named in
its row and ruled by that wave's audit. The same form fits: name the overage in each audit brief,
let the grader rule the growth a genuine new obligation, then stamp the ledger rows at the version
that ships them. Budgets move only by the seeding paths, and a mint is not one of them, so the
argued-overage path is the correct one. `patterns-model-tiering`'s own overage is pre-existing and
out of this wave's scope.

## Path sweep — every `.mochiko/` path a shipped primitive names

38 distinct paths across `commands/`, `agents/`, `skills/` and `templates/`, each resolved with
`mochiko-cli home`. Three file-shaped paths named an undeclared name and are the three re-points
above. One more did, and is deliberately left alone:

- `.mochiko/memory/evolution-roadmap.md`, named in two constitution-module templates. Nothing in
  the plugin writes it — the roadmap cluster that would produce it is unported, and one of the two
  references is a disambiguation saying the module is *not* that file. When that cluster lands it
  takes a migration, which is the system working rather than a defect.

Two cosmetic findings, neither a gate failure, since no write targets a bare directory:
`commands/specify.md` spells the spec directory `<spec>` where the home's token is `<slug>`, and
directory references generally are unresolvable by construction.

## Release files, staged

`plugin.json` and `marketplace.json` both read 0.109.0, and `CHANGELOG.md` carries one entry naming
the two hooks, the amnesty change, the five primitive edits, and the precondition. The MINOR bump is
right: registrations and a relaxation, no removal.

**Confirmed for the lead: 0.109.0 is written into the tree and nothing else.** It is uncommitted,
untagged, unpublished, and no release workflow has run. The staging is three file edits, reversible
by discarding them.

**The precondition is unchanged and binding.** This bump must not be released before `mochiko-cli`
publishes with all four controls, two of them still owed. A maintainer break-glass install is not a
consumer's install. The work sits on the branch.

## Gates

| gate | result |
|---|---|
| `cargo test --all` | PASS — 457 passed, 0 failed |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `cargo audit --deny warnings` | PASS — 31 dependencies |
| `mochiko-cli migrate validate --report` | PASS — 0 rejecting, 104 advisory |
| contract suite, host cases | PASS — 7/7, 6 measurements recorded |
| contract suite, sandboxed | not run — 78 metered sessions |

The QA seat's three new host cases — `gate-input`, `reminder-input`, `if-placement` — pass against
these wrappers as built, including the arm assertion that `Write|Edit` is un-narrowed and the shell
arms are narrowed on `.mochiko`. The suite records the reminder line as a measurement and notes no
golden is frozen for it yet; the line above is the one to freeze.

## Fix round — audit units 1 and 2

Both findings were in the same two prose sections, and both are now written in the form the ruled
mint `authoring-technical-requirements.artifact-home` already uses.

**F1 — the artifact has more than one home.** The sections named the spec home alone. Resolved
against the log, `data-model.md` is a declared deliverable of **four** homes — spec, feature, epic
and product — and `api.yaml` and the per-endpoint contract files of the `contracts/` home under each
of those four scopes, every one of them bounded by the contract's own interface. Both sections now
name all four, so an implement-run design phase is not pointed at the spec side.

`quickstart.md` is the exception worth stating precisely: spec, epic and product carry it, and the
**feature home does not**. The section says so rather than rounding it up to four.

**F2 — the fenced command did not run.** `mochiko-cli home` takes a real repository-relative path
and has no placeholder mode; with a literal token in the path it resolves to the index home and
reports no deliverable. All three fenced commands across the two files are gone. The instruction now
reads "render `mochiko-cli home <that path>` for the file you are about to write", which is the
ruled mints' own wording.

Both files stay additive, and both stay under budget: 14,355 against 16,835 and 11,877 against
13,412.

**W3 — the exit-4 branch now guards on empty output.** The exit-0 branch already fell back to the
explicit allow when the binary said nothing; the deny branch did not, so an exit 4 with empty stdout
would have printed nothing at all — silence, which a background subagent's call reads as a deny
while telling the seat nothing. It now falls through to the explicit allow. Verified with a stub
binary that exits 4 and prints nothing: the wrapper emits the allow and exits 0.

**W4 — the Edit amnesty cell asserts the file name.** It checked that `additionalContext` was
present but not that it named the file, which is the exact limb the ledger's gap describes. It now
asserts both.

## The eleven ledger lines

Stamped in `.mochiko/memory/primitive-cost-budgets.md`, one per HOLDS row, in the
`patterns-model-tiering` standing-overage form. Every figure is a fresh canonical-snippet
measurement at the quiesced tree; none is derived by adding the mint to the recorded figure, because
the recorded figures are stale.

Each overage decomposes exactly, and the arithmetic closes on all eleven rows:

```
overage = mint(0005) + body delta − 237 (v0.107.0 render-format change) − 0003 render reduction
```

Worked on the largest: `executing-tdd-cycle` +317 = mint +512, body +42, format −237, no two-arm
render term. On the smallest: `authoring-feature-map` +114 = mint +522, body +18, format −237,
two-arm render −189.

The mint costs are 497 to 558 **gross**, reproducing the audit's figures exactly. My earlier
"350–550 net" reading is withdrawn: there was no prior headroom to net against — all eleven budgets
were re-seeded at v0.106.0 with none — and the 237 is not an average but an exact constant, which
the audit verified on six un-minted control skills and I reproduced by re-rendering each skill
against a 0001-0004 log.

The two-arm residual splits into two limbs with **different causes and different row sets**, so each
line names only the limbs its own row carries and cites each separately:

- **Render −8 to −189**, from the two-arm rule retired by `0003-two-arm-to-cli.yaml` (record
  `cli-schema-delivery` D9, anchor 2026-09-03). Seven rows: `analysis-codebase`,
  `authoring-feature-map`, `authoring-technical-requirements`, `patterns-vertical-tdd`, and the
  three authoring skills that bound the retired `authoring-common.two-arm-template` stub —
  `authoring-architecture-store`, `authoring-constitution`, `authoring-prototype`.
- **Body +1 to +42**, from the same wave's body reword, strip-recorded in its v0.107.0 entry. A
  different seven: the four above minus `authoring-architecture-store` and
  `authoring-technical-requirements`, plus `authoring-user-stories` and `executing-tdd-cycle`.

Nine rows carry at least one limb; two carry neither and name the mint and the format constant
alone. No budget number moved, and every row's prior history is preserved after the new clause.

**The reconstruction now closes (GI-006).** The audit found a 383-character hole on
`analysis-codebase`: recorded render 9,088 plus the mint's 523 predicts 9,611 against an actual
9,228, and no clause accounted for the difference. With both clauses present the identity holds on
every row — recorded render, less 237, less the row's two-arm render reduction, plus the mint,
equals the measured render. I checked all eleven mechanically; eleven of eleven close exactly.

## The shell parse's known gaps, disclosed

The `Bash` and `PowerShell` arms scan command text; they do not interpret a shell. Three shapes
reach a home and are **allowed**, measured against the shipped binary with a denying control beside
each:

| shape | result |
|---|---|
| `printf x > <abs home path>/spec.md` — the control | deny, exit 4 |
| `cd .mochiko/specs/<slug> && printf x > spec.md` | allow |
| `T=.mochiko/specs/<slug>/spec.md; printf x > "$T"` | allow |
| `printf x > spec.md`, with the payload's `cwd` already inside the home | allow |

All three are the same root: the scanner reads the literal token, and a directory change, a variable
and a relative path from inside a home each hide the token it would have matched. This is what D1c
means by best-effort, and the deny reason says so on every fire. The `Write`/`Edit` arm is unaffected
— it resolves an actual path. Naming these is the point: an undisclosed best-effort parse reads as a
guarantee.

## Carried items

1. The reminder golden is frozen. The QA seat's W2 fix carries the line in the suite as
   `REMINDER_GOLDEN` and asserts it from both ends — the script's own text and the line the
   wrapper emits — so a reword of either is caught rather than silently re-recorded.
2. The 11 standing payload overages are ruled and stamped. Nothing further is owed on them.
3. This report sits at the session's declared `reports/` directory, following the QA seat, because
   `wave4-reports/` is not a declared sub-directory. `wave1-reports/` and `wave3-reports/` still
   are not, and re-home in wave 5's violator pass. `wave4-plan.md` is 199 lines against a whole-file
   bound of 150, which the three-row amendment resolves and this wave cannot.
4. The sandboxed contract set stays unrun until the lead says otherwise.
5. `patterns-entity-modeling` and `patterns-api-contracts` carry no rule set, so they have no pair
   to grade. The audit routing for those two is the lead's call.

## Notes of note

The wave-0 platform facts held everywhere they were load-bearing. The explicit allow on every
non-deny path is the one that would have been easy to skip and expensive to miss.

The budget sweep was not in the plan. I ran it because the ceremony demands a pre-assert on the
edited primitive, and the edited primitive turned out to be over budget for a reason that was not
mine. A per-skill pre-assert would have reported one number and hidden ten.

The contract suite's stale-binary trap earns its place. A `cargo fmt` pass touched `src/hook.rs`
after the release build, and six of seven cases failed with the reason spelled out rather than
grading yesterday's binary. Every number above was measured after the last edit and the rebuild.
