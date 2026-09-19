---
report: cycle
feature: hook-enforced-artifact-schema
round: 2
---

# AM-3 — governance producer report (surface set v3.1.0)

**Seat:** gov-producer · **Skill:** `mochiko:authoring-constitution` · **Date:** 2026-09-14
**Input:** the ratified synthesis `.mochiko/memory/governance-intent.md` (ratified 2026-09-14 by
Deepesh), with `.mochiko/brainstorms/hook-enforced-artifact-schema/wave2-amendments.md` §A–§F as
the lead's verbatim-where-marked proposal. Where the two differed, the synthesis won — three known
differences, all resolved to the synthesis: clause (iv) carries its C1 condition (the proposal
states it unconditionally); the amnesty clause is the D4e-corrected form (path is not a relaxable
measure, and the `Edit`-to-an-undeclared-name gap is named); the reach clause (D9/C4) and the what
the gate reads clause (C7) exist at all, which the proposal has neither of.
**Status:** authored on a lead-approved plan
(`scratchpad/gov-producer-plan.md`, approved with rulings on FP-1…FP-4 and FC-1…FC-3), then revised
in a lead-scoped fix round over the validator’s three findings and, after its delta-confirm PASS,
over its two non-blocking residuals (§6); awaiting the user’s acceptance gate.
**Register:** surfaces `full`; this report `ultra`, with constraint sentences left unambiguous.
**Floor read-back at open:** 12 `class: floor` rules, ids stated before the first procedural step.

## 1 — What was written

| File | Changed | Preserved |
|------|---------|-----------|
| `CLAUDE.md` | six lines: the kernel-class trace parenthetical (72, outside the markers, Card 2) · ratified stamp (109) · GI-019 index line (118) · release-gates line (129, the C5 clause) · amend-triggers line (135) · path-scoped-rules read line (139, residual R2) | everything else byte-for-byte, incl. the `mochiko:output-style` block (carve-out) and the GI-020 paragraph — both verified 0 diff hits |
| `.claude/rules/mochiko/rust-cli.md` | `paths` widened by three globs (FP-5, fix round B1) · header (two rulings → three) · bright-line bullet rewritten · log-is-truth record shorthand qualified (FP-3) | the five ratified globs unchanged and in order; dependency, quality-gate, release, break-glass and metadata bullets byte-identical |
| `.mochiko/memory/governance-ledger.md` | version line · semver MINOR limb · first-publish paragraph (+ the M2 expiry sentence) · GI-012 (precondition paragraph + M2 expiry sentence · gate-note watch · Testability limbs · trace) · GI-019 (7 Enforcement bullets · preserved prior text · two-tier Testability · trace) · amendment-log row 3.1.0 · AM-3 addendum (+ the FP-5 clause) | hash-verified identical: waiver table and both notes · GI-003 · GI-004 · GI-005 · GI-006 · GI-009 · GI-010 · GI-017 · **GI-020 entire** · GI-022 · confrontation rulings · floor-status paragraph · AM-2 and AM-1 addenda · header floor/depth line |
| `.mochiko/memory/governance-trace-summary.md` | regenerated whole at v3.1.0 (Shape 4) from a stale v1.0.0 manifest | prior rounds reachable by the FP-4 pointer sentence and git |

Not touched: `primitive-edits.md`, `operating-docs.md`, `output-style.md`, every
`plugins/mochiko/` primitive, `plugin.json`, the synthesis. No commits. No git mutations.

The artifact home was rendered before writing the trace summary, as the skill's artifact-home rule
requires: `mochiko-cli home .mochiko/memory/governance-trace-summary.md` resolves to home `memory`,
a declared deliverable, no template bound, no bound declared.

The region stays pointer-only under GI-017. Both clauses added to it name their detail home rather
than restating it: neither enumerates the four supply-chain controls, and no region line carries
hook mechanics. Grepping the region for `PreToolUse`, `SubagentStart`, `check --hook-json`,
`amnesty`, `exit 4` and `EXIT_CONFORMANCE` returns zero hits.

## 2 — Trace summary (Shape 4)

Emitted in full at `.mochiko/memory/governance-trace-summary.md` — one row per GI element
GI-001…GI-022, the reverse table (changed surface line → GI element), the floor-coverage line, the
five flagged proposals, waivers and narrowings. Not duplicated here (GI-017).

Summary: eight elements carry AM-3 content — GI-001 and GI-021 through the re-versioned stamp,
GI-002 through the amendment-policy discharge line, GI-012 through four touches, GI-019 through
the admission and everything hanging off it, GI-006 through the preserved prior text, GI-017 as
the binding routing constraint, GI-020 untouched but reconciled with D7e inside GI-019's scope
clause. All four floor categories accounted; module set unchanged; GI-008 the sole waiver, row
untouched.

## 3 — Flagged proposals (authored on the lead's plan-gate ruling; the user's to overturn)

Each stays listed here and in the trace summary for the acceptance gate. Each is reversible by a
single edit.

**FP-1 — the ≤ 60 s aggregate hook-cost watch lands in GI-012's gate note.** The synthesis rules
C9 under GI-002's risk surface; GI-002 has no Three-Part entry (FP-3 at AM-2, accepted), so the
ruling would otherwise live only in the synthesis and not be found where a bump is checked. The
gate note is the established home for the "measured, never gating" idiom. **This is the second
GI-012 touch beyond Card 7's C5 clause** — disclosed as such in the AM-3 addendum. *Alternative:*
synthesis-only.

**FP-2 — GI-012 Testability gains one Pass limb and one Fail limb.** `three-part-rule` (floor)
requires Testability to cover Enforcement; the synthesis names the C5 enforcement clause and says
nothing about its testable form. *Alternative:* the clause without a testable form.

**FP-3 — `rust-cli.md`'s log-is-truth bullet qualified to `(record `cli-schema-delivery`
D1/D2/D6)`.** Outside Card 4's literal scope, lead-ruled at the plan gate on the ground that this
run's own header rewrite introduces a second driver record whose D1/D7 sits three lines above, so
leaving the bare shorthand would be a defect this run creates. Nothing else in that bullet
changed. *Alternative:* leave it byte-identical and carry the ambiguity.

**FP-4 — the trace summary's superseded-rounds pointer sentence.** Regenerating Shape 4 whole
overwrites the v1.0.0 manifest and its ruled FP-1; the pointer keeps the prior rounds reachable
(GI-006) without restating them (GI-017). *Alternative:* no pointer, recovery from git alone.

**FP-5 — `rust-cli.md`'s `paths` gains three globs** (fix round, validator B1):
`plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `CHANGELOG.md` —
all three verified on disk. Every GI-012 gate fires at the `plugin.json` bump and AM-3 puts the
set's strictest MUST NOT there, so a globs-honest reading has to reach the bump's own files;
`CHANGELOG.md` is GI-010's home. AM-2 added `.github/workflows/**` on the same reasoning (its
FP-1/FP-2). Ruled at the acceptance gate like its precedents. *Alternative:* keep the five ratified
globs and leave the wave-4 precondition with no touch-time carrier at the files it gates.

## 4 — Formulation calls (approved at the plan gate)

**FC-1 — the region's C5 clause sits on the `plugin.json` bump line (129), not the release-tag
line (130).** The clause gates a `plugin.json` bump; line 130's subject is a `mochiko-cli-v*` tag.
Line 130 is verbatim.

**FC-2 — the seven AM-3 GI-019 bullets are appended after the advisory-clause bullet**, not
inserted after the AM-2 admission bullet as §C says, so each admission stays contiguous with its
own argument. Every pre-existing bullet is preserved in order.

**FC-3 — the AM-3 mark split rides inline in the hook-floor bullet; no `Marks` block added.**
Follows GI-019's own precedent rather than GI-020's. The `Assumed` covers the D3-derived exit-code
contract and the `check --hook-json -` interface; the exit-4 fact was verified against the built
crate (`EXIT_CONFORMANCE = 4`, `crates/mochiko-cli/src/hook.rs:37`).

**FC-4 — GI-012's trace line was extended, which the plan did not itemise.** The plan listed
GI-019's trace extension only. GI-012 gained two AM-3 clauses, and this ledger's standing idiom is
that each amend appends to the touched principle's trace line; leaving it stale would have made
the C5 and C9 touches reconstructible only from the amendment-log row. Disclosed rather than
silent, and strikeable by one edit.

## 5 — Observations (not authored)

**O-1 — the ledger's semver MAJOR limb lacks "depth-level flip (`low`→`high`)"**, which the
`governance-surfaces` template's Shape 3 carries. Pre-existing, unrelated to AM-3, harmless today
(GI-021 is already `high`, terminal). **Owed at the next PATCH.** Recorded in the trace summary's
closing lines. AM-2's O-1 precedent for leaving such a thing unauthored.

**O-2 — `wave2-reports/` is not a declared sub-directory of the `brainstorm-session` home.**
`mochiko-cli home` on this report's own path answers "`wave2-reports/` is NOT a declared
sub-directory of this home"; reports belong at `.mochiko/brainstorms/<slug>/reports/`. Written to
the lead-named path because write conformance is procedural this run (the hooks ship at wave 4)
and AM-2 set the same path. This is exactly what review C10's violator pass is scheduled to fix,
before the wave-4 plugin installs locally. Disclosed, not corrected here.

**O-3 — the installed plugin cache (0.108.0) does not carry `0005-artifact-homes.yaml`.** The home
render against the cache answers "no declared home governs this path"; against the repo tree's
five-file log it resolves. Consistent with the synthesis's build-state disclosure that the wave-3
migration is authored untracked in the working tree. Both were rendered; the repo tree's answer
was used.

## 6 — Fix round (2026-09-14, after the validator's FAIL)

The independent `validation-constitution` seat graded round 1 FAIL on one blocking and two minor
findings (`reports/wave2-gov-validator.md`). All three applied, nothing else; each is formulation,
no ruling changed.

| ID | Finding | What changed |
|----|---------|--------------|
| **B1** (blocking) | `rust-cli.md`'s `paths` did not reach the files the AM-3 clause gates | `paths` gains `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `CHANGELOG.md`; recorded as **FP-5** here, in the trace summary, and as one clause in the ledger's AM-3 addendum |
| **M1** (minor) | GI-019's conformance Testability limb is unassertable until wave 4 ships the hooks | Testability re-shaped into GI-020's two tiers: **assertable at ratification** (the dependency-halt limbs, plus `check` existing and returning exit 4 — verified against the crate) and **dormant until the wave-4 hook ship**, activated at that `plugin.json` bump as a pre-authorized PATCH activation, no fresh amend |
| **M2** (minor) | the wave-4 precondition clause was one-shot, with no expiry | One sentence in the GI-020 transition-clause form added at both ledger homes — the amendment-policy first-publish paragraph and the GI-012 wave-4 paragraph: once the wave-4 bump has landed under the precondition, striking the clause is a pre-authorized PATCH amendment recorded in the log row. The region line stays as authored |

M1's factual basis was re-verified on disk: `plugins/mochiko/hooks/hooks.json` carries only
`SessionStart`, `UserPromptExpansion` and the `Skill`-matched `PreToolUse`, with two scripts
(`dependency-halt.sh`, `session-start.sh`) — no conformance gate ships today.

**Residuals after the delta-confirm PASS.** The validator passed the fix round with two
non-blocking residuals; both taken in a lead-scoped second pass, nothing else touched.

| ID | Residual | What changed |
|----|----------|--------------|
| **R1** | the 3.1.0 amendment-log row under-described the fix round | the row's GI-delta cell gains a fix-round clause naming all three: the two-tier GI-019 Testability with the conformance limb dormant until the wave-4 hook ship (activation a pre-authorized PATCH) · the M2 post-landing strike, itself a pre-authorized PATCH at both ledger homes · `rust-cli.md` `paths` += the three bump-surface globs. Precedent for recording pre-authorized PATCHes in the row: rows v2.0.1, v3.0.1, v3.0.3 |
| **R2** | the region's new-file read line did not match the rules globs after FP-5 | the path-scoped-rules line (139) names `.claude-plugin/` (both manifests) and `CHANGELOG.md` alongside the existing seven. Nothing else on the line, nothing else in the region |

A correction made while verifying the fix round: round 1's report called the ratified glob set
"six". It is **five** on disk. Fixed here and in the trace summary's FP-5 alternative clause.

## 7 — Verification run after writing

- `git diff --stat` over the governance surfaces: `CLAUDE.md` (10 ±) ·
  `.claude/rules/mochiko/rust-cli.md` (24 ±) · `.mochiko/memory/governance-ledger.md` (144 ±) ·
  `.mochiko/memory/governance-trace-summary.md` (121 ±). `.mochiko/memory/governance-intent.md`
  (88 ±) is the lead's pre-existing diff, not this seat's.
- `CLAUDE.md`: exactly five changed lines — 72, 109, 118, 129, 135. The `mochiko:output-style`
  block and the GI-020 paragraph return 0 diff hits, byte-identical.
- Ledger: every deletion in the diff is one of seven intended replacements (version line · semver
  MINOR limb · first-publish paragraph tail · GI-012 Testability · GI-012 trace · GI-019
  Testability · GI-019 trace). No principle entry deleted. Six preserved regions hash-compared
  against `HEAD` and identical, GI-020's entry among them.
- `rust-cli.md`: three hunks, the `paths` frontmatter and the four trailing bullets
  hash-identical to `HEAD`.
- Version agreement: region stamp `v3.1.0 · 2026-09-14 (AM-3)` · ledger `**Version:** 3.1.0` ·
  amendment-log row `| 3.1.0 | 2026-09-14 | AM-3 …` · trace-summary header `v3.1.0 (2026-09-14,
  AM-3)`. All four agree.

## 8 — Floor-rule self-check

`ratified-synthesis-only` — authored from the ratified synthesis; the proposal was input only.
`missing-synthesis-stop` — not triggered. `no-constitution-file` — none written or read.
`every-principle-traces` — every amended surface line keeps its `<!-- GI-XXX -->` comment; GI-019's
and GI-012's ledger trace lines extended. `realized-or-flagged` — every AM-3 delta element routed
or flagged; C9, the one element with no sanctioned home, was raised as FP-1 and authored on the
lead's ruling. `markers-only-regeneration` — region edits sit between the markers; the one edit
outside them is the user-ruled Card 2 scope. `preserved-blocks` — the output-style block re-emitted
byte-identical; no domain-registry block exists (GI-013 declined durable). `three-part-rule` —
GI-019 and GI-012 keep all three components; both new enforcement clauses have testable limbs.
`floor-categories-accounted` — all four live through GI-003/004/005/006, untouched. `waivers-
authored-not-skipped` — GI-008 sole, row and notes verbatim. `no-unsanctioned-selection` — no
principle added, removed, merged or reinterpreted; four flagged proposals and four formulation
calls carry everything past the synthesis's literal text. `never-route-unselected-modules` — module
set unchanged: compliance none · knowledge-management (core + CHANGELOG) · release-gates.
