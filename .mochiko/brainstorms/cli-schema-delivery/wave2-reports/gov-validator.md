# AM-2 — independent validation of the governance surface set v3.0.0

**VALIDATION RESULT: PASS**

**Seat:** gov-validator · **Skill:** `mochiko:validation-constitution` · **Date:** 2026-09-04
**Graded set:** the CLAUDE.md governance region (lines 106–141) + the user-ruled
`## Non-negotiable constraints` paragraphs at CLAUDE.md:72 and :74 · `.claude/rules/mochiko/`
(4 files) · `.mochiko/memory/governance-ledger.md`. There is no `constitution.md`; none on disk.
**Contract:** `.mochiko/memory/governance-intent.md`, ratified 2026-09-04 by Deepesh.
**Author:** gov-producer. This seat authored none of it. Single writer of this file.

- **Checklist items:** 60/61 passed · 1 n-a by recorded ruling. Core 49 + 2 selected module
  fragments (`release-gates` 4 · `knowledge-management` 8). `layer-rules` and `evolution-notes`
  fragments not run — declined durable (GI-013, GI-014).
- **Surface integrity:** region markers present, nothing outside them changed but the two
  user-ruled prose paragraphs · index → home → ledger closes both ways for GI-003, GI-004,
  GI-005, GI-006, GI-009, GI-012, GI-017, GI-019, GI-020, GI-022 · all 4 rules files
  `paths`-scoped with valid globs · scope coverage complete for the concerns changed this run ·
  standing new-file read line present (advisory A3) · no universal principle relocated into a
  rules file.
- **Trace closure:** forward — every AM-2 synthesis element reaches a surface line and, where
  principle-bearing, a Three-Part ledger entry. Reverse — every changed surface line reaches a
  synthesis element. GI-022 new and minted. GI-020 superseded-by-ruling with its trace and the
  AM-1 text preserved in place.
- **Floor / module accounting:** production floor + depth `high` asserted in both the region
  stamp and the ledger header, agreeing · compliance modules none, matching the GI-001 fact
  profile · template modules match GI-009/GI-010/GI-012 one-for-one, declines recorded · all four
  Essential Floor categories carry a principle (GI-003 · GI-004 · GI-005 · GI-006), none waived ·
  waiver GI-008 unchanged, one-for-one with the synthesis, naming no legal-mandate obligation.
- **Anti-patterns found:** none.
- **Version bump:** MAJOR — 2.0.1 → 3.0.0. GI-020 incompatibly redefined (the raw-Read degraded
  path withdrawn) and GI-022 minted. Matches the ledger semver policy, the region stamp, the
  ledger version line, and the amendment-log row.
- **Issues requiring fix:** none blocking.
- **Advisory:** 9, below. All non-blocking; none changes a ruling.

---

## 1 — Blocking findings

**None.** Every element the ratified synthesis carries is realized or truthfully flagged. Default
FAIL is discharged item by item in §2 and §3.

## 2 — What was checked, and the result

### Trace closure — forward (AM-2 element → surface + ledger)

| AM-2 element | Surface line | Ledger Three-Part | Result |
|---|---|---|---|
| GI-002 identity + risk live, trigger fired conditional | CLAUDE.md:119 tech stack · :139 amend triggers | none — flagged FP-3; GI-002 is a fact element, not principle-bearing; obligations routed to ledger:42–49 and GI-012 | PASS |
| Real commands | CLAUDE.md:130 · rust-cli.md:31–37 · ledger:236–241, 249–252 | GI-012, GI-005 | PASS |
| GI-004 floor note (schema audit unit, count self-check retired, PATCH activation, code author≠grader) | CLAUDE.md:114 | ledger:105–117 | PASS |
| GI-005 floor note (schema-rule limb mechanical, two regimes) | CLAUDE.md:115 | ledger:143–147 | PASS |
| GI-006 row (migration log as carrier) | CLAUDE.md:116 | ledger:171–174 | PASS |
| GI-019 AM-2 (widened admission, three clauses, advisory placement) | CLAUDE.md:72 prose · :118 index | ledger:313–335 | PASS |
| GI-020 AM-2 (superseded-by-ruling) | CLAUDE.md:74 prose · :119 index | ledger:352–422 | PASS |
| GI-008 note | — (row unchanged) | ledger:18–24 | PASS |
| GI-012 row | CLAUDE.md:131–132 | ledger:225–280 | PASS |
| GI-007 notes (narrowed to markdown primitives) | — | ledger GI-004/005/006 trace lines | PASS (advisory A6) |
| GI-022 | CLAUDE.md:120 | ledger:424–444 | PASS |
| Store scaffold disclosure | — (not a ruling) | — | PASS — scaffold present, `Scope:` line matches the disclosed text verbatim |
| AM-2 amendment-log entry | — | ledger:466 + addendum :468–472 | PASS |

### Trace closure — reverse (changed surface line → element)

All 9 CLAUDE.md hunks, all 6 `rust-cli.md` bullets plus its frontmatter, and all 13 changed
ledger blocks map to a named synthesis element. `git diff -U0 -- CLAUDE.md` yields hunks at 72,
74, 109, 114, 118, 123, 127, 133, 137 — two outside the markers (both user-ruled), seven inside.
No orphan line.

### Three-Part completeness — every changed ledger entry

| Entry | Enforcement | Testability (Pass + Fail) | Rationale | Trace |
|---|---|---|---|---|
| GI-004 | ✓ | ✓ | ✓ | ✓ |
| GI-005 | ✓ | ✓ | ✓ | ✓ |
| GI-006 | ✓ | ✓ | ✓ | ✓ |
| GI-012 | ✓ | ✓ | ✓ | ✓ |
| GI-019 | ✓ | ✓ | ✓ | ✓ |
| GI-020 | ✓ | ✓ two tiers | ✓ + Marks | ✓ |
| GI-022 | ✓ | ✓ | ✓ | ✓ |

### The named checks

- **GI-020 two-tier Testability.** Split present at ledger:384–395. Assertable tier names the
  contract suite's absence/skew cases and the log's hard set; dormant tier explicitly labelled
  "Dormant until the wave-3 pilot re-points the first primitive". Revisit trigger present at
  :397–400 (pilot abort criteria, reversal cost priced).
- **Transition clause.** Present with expiry at CLAUDE.md:74 and ledger:374–377. Expiry
  pre-authorized as PATCH at ledger:50–54. Unsupported set at ledger:378–382 includes
  PowerShell-only Windows and scopes the no-build-step property to the plugin.
- **GI-019.** Three clauses (i)(ii)(iii) present at ledger:320–331. Widened admission named
  (`cli-schema-delivery` D11) at ledger:313 and CLAUDE.md:72, :118. Bright-line text at
  CLAUDE.md:72 unchanged — the diff touches only the trace parenthetical.
- **GI-012.** Gate 6 with contract suite ✓ · SKIPPED blocks ✓ (:239–241) · gate 5 = view ≡ replay
  ✓ (:234–235) · crate release train ✓ (:247–255) · Contested substrate mark ✓ (:260–265) ·
  read-back metric reported not gating ✓ (:257–258).
- **GI-003 / GI-009 / GI-010 / GI-017 / GI-021 verbatim.** Confirmed — no diff hunk touches them.
  Depth `high` re-recorded in both the region stamp and the ledger header, agreeing.
- **Pointer-only region (GI-017).** `sed -n '106,141p' CLAUDE.md | grep -E
  "SessionStart|UserPromptExpansion|PreToolUse|ruling anchor|class: floor|kind: fail|hash:|anchor:"`
  → zero hits. No migration grammar, no anchor rule, no hook mechanics restated.
- **Output-style carve-out.** `shasum` of the block before and after: both
  `2a02616ee64f8dc9e358ce22ab0ff4472422e856`. Byte-identical. `mochiko:domain-registry` never
  existed in this file (n-a).
- **No shipped primitive changed.** `git status --short -- plugins/mochiko` → empty.
- **Placeholders.** Zero hits for `[PLACEHOLDER]`, `[COMMAND]`, `[THRESHOLD]`, `GI-XXX` across
  all three surfaces.
- **`rust-cli.md`.** Six `paths` globs, all valid, no malformed pattern. Body is operative rules
  plus pointers; metadata routed to the ledger, not restated. Break-glass line present
  (`cargo install --path crates/mochiko-cli`), correctly fenced as maintainer-only.
- **Log-home tense.** Five sentences name the log's home — CLAUDE.md:74, :119, rust-cli.md:12–13,
  ledger:173–174, ledger:366–368. All five in end-state tense with today's repo-root reality
  stated. No sentence claims the plugin carries it now.
- **Producer's flagged proposals.** All five verified present in the files and correctly
  characterized as departures from the ratified scope. No sixth departure found unflagged; the
  two `paths` proposals are additionally recorded in the ledger addendum.

### Commands named — every one exists and runs

| Command | Result |
|---|---|
| `cargo test --all` | 300 passed, 0 failed |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --all-targets -- -D warnings` | exit 0 |
| `cargo audit --deny warnings` | ran, 31 deps scanned, no failure |
| `mochiko-cli migrate validate --log-dir migrations --plugin-root plugins/mochiko` | `0 rejecting · 105 advisory` |
| `python3 evals/contract/run.py --help` / `--list` | 2 declared cases: `absence`, `skew` |
| `cargo run -q -- template governance-surfaces --check --log-dir migrations` | 5 mirror-checklist items rendered |
| `cargo build --release -p mochiko-cli` | on no surface; not graded |

`--out` (rust-cli.md:29) is real: `mochiko-cli views emit --out <DIR>`, required, no default.

### The four first-publish controls, against the tree

| Control | Ledger claim | Tree |
|---|---|---|
| `cargo audit --deny warnings` in CI | present | ci.yml:65 — present |
| sha256 release assets | present | release.yml:88 `shasum -a 256`, uploaded — present |
| `cargo publish` behind manual-approval environment | **owed** | `environment: crates-io` declared at release.yml:102; job `if: false` at :100; approval rule not tree-visible — **owed stands** (advisory A2) |
| signed release tags | **owed** | no `gpg` / `sign` / `cosign` / `attest` anywhere — **owed stands** |

No control is claimed present that is not. The two owed are marked owed.

### Selected module fragments

**`release-gates` (4/4).** Environments and cadence stated with real names (ledger:267–268:
"none — nothing deploys; distribution is the Claude Code marketplace" · "manual, at
`plugin.json` bumps"). Six gates, each with a concrete verification, no placeholders. Rollback
documented with the time expectation explicitly declined and reasoned ("no time-bound SLO — no
operated service"). No compliance module attached, so the consistency item is vacuous.
*Deviation, carried from v1.0.0, non-blocking:* the gates are a numbered list, not the fragment's
table.

**`knowledge-management` (8/8).** All three enforcement surfaces present and untouched this run —
`.mochiko/memory/knowledge-management.md` · `.claude/rules/mochiko/operating-docs.md` ·
CLAUDE.md:141. Core artifacts on disk: ROADMAP · BACKLOG · DECISIONS · ARCHITECTURE, plus the
adopted CHANGELOG elective. GLOSSARY.md absent, matching the recorded GI-009 deviation. RUNBOOK
absent, matching the GI-011 decline. Landing ritual, invariants, and never-overwrite floor
unchanged.

## 3 — Advisory findings (non-blocking)

**A1 — `.mochiko/memory/governance-ledger.md:49` overclaims by one.** "The first two are also
release-train gates under GI-012." `cargo audit --deny warnings` is one of the four crate layers
at :249–250. sha256-published release assets appear nowhere in GI-012's release train. *Fix:*
strike "The first two" to "`cargo audit` is", or add the sha256 assets to the train.

**A2 — `.mochiko/memory/governance-ledger.md:45–46` gives an imprecise reason for "owed."**
The parenthetical is "the job is `if: false` today". `environment: crates-io` is in fact declared
(release.yml:102); what is absent is the environment's protection rule, which is a GitHub setting
and not tree-visible, plus the job being enabled. The conclusion (owed) is right and conservative;
only the reason is off. *Fix:* "the environment's approval rule is not tree-visible and the job is
`if: false` today".

**A3 — `CLAUDE.md:139` new-file read line omits `.github/workflows/`.** `rust-cli.md` now scopes
`.github/workflows/**` (FP-1), so a newly created workflow file gets no read reminder; the rule
still injects on Read of an existing one. *Fix:* add `.github/workflows/` to the list, or accept
that FP-1's carrier is Read-only.

**A4 — `.claude/rules/mochiko/primitive-edits.md:2–8` will not cover the plugin's wave-3
additions.** Its globs name `commands/`, `skills/`, `agents/`, `templates/`, `schemas/`. From
wave 3 the plugin also carries `plugins/mochiko/migrations/` and `plugins/mochiko/hooks/`. The
pre-authorized wave-6 re-key at ledger:52–54 is scoped to "schema strips → migrations" only —
hooks are uncovered by either. *Fix:* name hooks in the wave-6 re-key scope, or record it as a
wave-3 obligation.

**A5 — GI-011 has no ledger home.** Producer O-1, confirmed. The declined-durable line at
ledger:55–56 names GI-013 and GI-014 only; GI-011's decline lives in the synthesis alone. The line
is literally accurate ("declined durable **at AM-1**"), since GI-011 was declined at v1.0.0.
Pre-existing, outside AM-2 scope. *Fix:* fold at the next PATCH.

**A6 — GI-007's AM-2 narrowing is realized only indirectly.** The synthesis narrows the
"inapplicable in kind" exclusion so that the crate, the migration log, and the plugin path all
carry real executable gates. GI-004's trace line names the crate only; the log's gate rides GI-005
and the plugin path's rides GI-012. Trace closes, thinly. GI-007 is an exclusion, not
principle-bearing, so no principle is owed. *Fix:* extend GI-004's trace parenthetical, or none.

**A7 — the amendment-log row does not name the CLAUDE.md prose rewording.** Row 3.0.0
(ledger:466) names the `rust-cli.md` rewrite and the `paths` widening; the addendum (:468–472)
records only the `paths` proposals. Neither names the two `## Non-negotiable constraints`
paragraphs the producer edited. Reconstructibility (GI-006) is served by the synthesis's own AM-2
Scope bullet plus git. *Fix:* one clause on the addendum.

**A8 — `mochiko-cli` is not on PATH in this worktree.** GI-005's Testability Pass row and
rust-cli.md cite the bare command form; I ran it as `cargo run -q --`. Not a fictional command —
the install routes and the break-glass are both recorded — but the row as written is not runnable
in a source tree without one of them. *Fix:* none needed; noted for the next reader.

**A9 — I did not re-run the contract suite.** Deliberate: it drives headless sessions on the
sandbox's stored consumer subscription, the substrate GI-012 itself marks `Contested` against
adverse Terms-of-Service evidence. `sbx` and `docker` are both present, so a run would not have
skipped. I verified the suite exists, its case list is exactly the two the ledger's "2/2" names,
and its three-valued exit contract matches GI-012's SKIPPED clause. The "2/2 green, 2026-09-04"
instance is dated and attributed but not independently re-observed by this seat.

## 4 — Method

Graded from the files, never from the producer's report. Read in full: the ratified synthesis,
the ledger, the CLAUDE.md region and both prose paragraphs, all four rules files, the producer
report, the cold intent review and its verify pass. Diffed `CLAUDE.md`,
`.claude/rules/mochiko/`, and `.mochiko/memory/governance-ledger.md` at `-U0` to enumerate every
changed line, then walked closure in both directions against the synthesis. Ran every command
named on a surface (table above) plus the mirror-checklist view. Checked the four first-publish
controls against `ci.yml` and `release.yml` directly rather than against the ledger's account of
them. Confirmed the output-style carve-out by checksum, not by eye. Verified GI-022's own
Testability Pass row against the tree (no `FEATURES.md`, no `.mochiko/features/`). Confirmed the
head-and-tail output shape the crate release train gates on has a real test carrier
(`crates/mochiko-cli/tests/render.rs:316`, `:363`), so that clause is not a vague MUST.

The `evolution-notes` fragment was not run. Brownfield mode normally selects it, but GI-014
records a durable user decline at AM-1, and the checklist forbids checking a fragment the
synthesis did not select. Graded as n-a by recorded ruling, not as a gap; the brownfield context
it would carry lives in the synthesis, the ledger's confrontation-rulings table, and the
floor-status paragraph.

No surface was edited by this seat.

---

## Delta-confirm

**2026-09-04, same seat.** Bounded re-grade of the six advisories the producer applied (A1, A2,
A3, A4, A6, A7). A5 carried as a next-PATCH note; A8/A9 needed no action. Scope: only the changed
lines, plus the three integrity re-checks. No fresh read, no new angles.

| # | Fix | Where | Result |
|---|---|---|---|
| A1 | The sha256 overclaim struck. Now reads "`cargo audit --deny warnings` is also one of the four crate layers in GI-012's release train." | ledger:50–51 | **CONFIRMED** — and true: GI-012's train names `cargo audit --deny warnings` at ledger:256. sha256 is no longer claimed as a train gate. |
| A2 | The "owed" reason corrected: the `crates-io` environment is declared, its approval rule is a GitHub setting not visible in the tree, and the job is `if: false`. | ledger:46–47 | **CONFIRMED** — matches release.yml:100 (`if: false`) and :102 (`environment: crates-io`). Verdict stays "owed", the conservative direction. |
| A3 | `.github/workflows/` added to the standing new-file read line. | CLAUDE.md:139 | **CONFIRMED** — the line now covers all six roots `rust-cli.md` scopes. |
| A4 | The pre-authorized PATCH scope now names both halves, and rules the `paths`-glob half a **wave-3** obligation, landing when those directories ship rather than at wave 6. | ledger:55–59 | **CONFIRMED** — closes the hole. See the residual note below. |
| A6 | GI-004's Trace gains a routing sentence: the three executable gates live one per home — crate `cargo test` on GI-004, the log's hard set on GI-005, the plugin path's contract suite on GI-012. | ledger:138–140 | **CONFIRMED** — all three routings check out against GI-004's enforcement, GI-005's Testability, and GI-012 gate 6. |
| A7 | The AM-2 addendum now names both `## Non-negotiable constraints` paragraphs, attributes them to the Card 1 ruling, and asserts nothing else outside the markers changed. | ledger:475–479 | **CONFIRMED** — and the assertion is independently true: `git diff -U0 -- CLAUDE.md` yields exactly two hunks outside the markers, at :72 and :74. |

### No new break introduced

- **Trace, both directions, over the changed lines.** Each of the six edits maps to an existing
  element — GI-002/GI-012 · GI-002 · GI-020 delivery caveat · GI-004/GI-020 · GI-004/GI-007 ·
  GI-019/GI-020. No GI-ID minted, none orphaned, no principle added or removed. Version correctly
  stays 3.0.0 in all three places.
- **Region pointer-only (GI-017).** Re-run over lines 106–141: zero hits for `SessionStart`,
  `UserPromptExpansion`, `PreToolUse`, `ruling anchor`, `class: floor`, `kind: fail`, `hash:`,
  `anchor:`. Nothing restated. All six fixes land in the ledger and the operations line, not as
  new normative region text.
- **Output-style carve-out.** Still byte-identical —
  `2a02616ee64f8dc9e358ce22ab0ff4472422e856` before and after.
- **Scope.** Hunk shape unchanged: two outside the markers, seven inside. `plugins/mochiko/`
  untouched. `git diff --stat` lists the three surfaces plus `.mochiko/memory/governance-intent.md`
  — the latter is the ratified contract, not a surface, and its mtime (09:57:44) predates both my
  audit and this delta, so the producer did not touch it. The two reports are untracked.
- **Contradictions.** None found. A1's new cross-reference, A6's routing sentence, and A7's
  outside-the-markers assertion were each checked against the file they describe rather than taken
  on their word.

### Residual, carried (non-blocking)

**A4's fix adds timing detail past the literal I2 ruling.** The synthesis pre-authorizes the
`primitive-edits.md` re-key as a PATCH activation; splitting it into a wave-3 glob half and a
wave-6 ceremony half is the producer's reading, not a user ruling. It is the right reading — an
unscoped shipped directory is a primitive edited with no ceremony reaching its author — and it
lives in the ledger where the next amend will see it. The amendment-log row 3.0.0 does not mention
the wave-3 half; the policy bullet is its home, so this is a summary gap, not a contradiction.
Worth one line at the acceptance gate so the user rules it knowingly.

**A5 remains open by design** — GI-011's decline still has no ledger home, correctly deferred to
the next PATCH.

**DELTA-CONFIRM RESULT: PASS.** Six of six fixed, none partially. No new trace break, restatement,
or contradiction. The v3.0.0 PASS above stands.
