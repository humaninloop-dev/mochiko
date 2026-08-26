# Author≠grader audit — `review-plan-artifacts` v0.86.0 true-deletion body cut

**Verdict: PASS** (round 2, after fixes) · Round 1: FAIL · Grader: independent validator, did not
author any of the graded material · Date: 2026-08-26 · Ceremony:
`.claude/rules/mochiko/primitive-edits.md`

No rule was lost. All 113 inventory entries have a home, both budgets pass, and every
consumer-cited term survives. Round 1 failed on one blocking class — the cut created three stale
reference-to-body pointers while the strip and pass report both claimed "No dead pointers
created" — and all four fixes verified clean at round 2. The round-2 delta verify is recorded at
the foot of this file.

## 1. Deterministic char-budget pre-assert (D7)

Measured with the canonical python snippet in `.mochiko/memory/primitive-cost-budgets.md`
(characters of the parsed value, never `wc -c` bytes).

| Class | Measured | Budget | Delivery cap | Result |
|---|---|---|---|---|
| Skill body | **4,901** | 6,127 | — | PASS |
| `description:` | **589** | 625 | 1,536 | PASS |

The pre-cut baseline was re-measured independently from `git show HEAD` at **13,521** body /
**589** description. So 13,521 → 4,901 is **−63.75%**, which matches the −63.8% recorded in the
strip, the ADR and the `DECISIONS.md` row. The description is byte-identical to baseline, so this
cut did not touch it.

## 2. Preserved responsibilities — all 113 rules walked

Source of truth: `evals/review-plan-artifacts/rules.json`, authored independently of the
compressor from the pre-cut baseline.

**LOST: none. 113/113 have a home.**

- **Survives in the new body: 82.** R-001 through R-019, R-026, R-034, R-042 through R-051,
  R-054, R-059, R-060, R-061, R-063, R-064 through R-068, R-070 through R-087, R-089, R-091
  through R-110, R-112, R-113. Several carry a reference file or the always-delivered
  `description:` as a co-home. R-084 is the notable one: the verdict trio
  `ready / needs-revision / critical-gaps` survives verbatim in the `description:` and in
  `ISSUE-TEMPLATES.md` § Verdict Criteria, while the body carries only `critical-gaps` plus the
  divergence override.
- **Survives solely in an untouched `references/` file: 31.** R-020 through R-025, R-027 through
  R-033, R-035 through R-041 (the analysis, store-delta and design mirror checklists), R-052,
  R-053, R-055, R-056, R-057, R-058, R-062, R-069, R-088, R-090, R-111.

`git status` confirms only `SKILL.md` was modified, so `references/ARTIFACT-CHECKLISTS.md` and
`references/ISSUE-TEMPLATES.md` are genuinely untouched and valid as single sources.

The three inventory-driven restorations the ADR claims are all present and verified: R-070
(flag-only-between-artifacts), R-093 (good-enough-is-never-ready), R-098
(obvious-never-exempts-documentation).

## 3. Internal coherence of the new body

Everything the body points at resolves:

- The anchor `#scope-boundary--handoff-to-review-feasibility` exists at
  `references/ARTIFACT-CHECKLISTS.md:341`.
- The `ISSUE-TEMPLATES.md` pointer targets exist: Severity Levels, Classification Rules, Issue
  Documentation Format, Working Report Shape, Verdict Criteria.
- `python scripts/check-artifacts.py` resolves against
  `plugins/mochiko/skills/review-plan-artifacts/scripts/check-artifacts.py`, relative to the
  skill directory — the same convention the untouched reference uses at its Automated Validation
  block.
- Both arms of the schema citation are named: `mochiko-cli template tasks --check` and the raw
  `plugins/mochiko/schemas/tasks.yaml` degraded path. The schema file exists.
- Default-FAIL posture present (`Floors:` opening, plus the `description:`).
- Independence present ("never the author", twice).
- Verdict vocabulary reachable via the `description:`, the ISSUE-TEMPLATES pointer, and the
  body's own critical-gaps override clause.

What does **not** hold is the reverse direction — see the blocking findings below.

## 4. Record-layer consistency

The numbers agree across every surface: strip `[v0.86.0]`, the ADR, the `DECISIONS.md` top row
and the cost ledger all carry 13,521 → 4,901, −63.8%, budget 4,901/6,127, shipped v0.86.0. Every
disposition-map claim and every MANDATORY KEPT reconciliation line checks out true against the
actual body, with one exception recorded as blocking finding 4.

`plugin.json` is still at 0.85.0 while the record layer stamps v0.86.0. That ordering is correct
— the audit precedes the bump — but gate 4 (`CHANGELOG.md` 0.86.0 entry) and the
`marketplace.json` sync remain outstanding at bump time.

## 5. Consumer contract

Verified by grep against the new body. Every term survives:

- "material divergence" auto-FAIL, BLOCKING conformance — present, both limbs of the definition.
- Adopt-first disclosure BLOCKING at conformance strength with `mochiko:patterns-adopt-first`
  named — present, including the "no shelf candidate exists" literal.
- Rung honesty advisory against `mochiko:patterns-plan-minimalism` — present, with
  "advisory, never drives the verdict".
- Cycle-cards complete check set including oracle semantics — present, with the v0.75.0
  semantic-grading wording intact.
- 3-state verdict — reachable (see R-084 above).
- Independent, never the author — present.

Consumers re-read and confirmed still true: `skills/mochiko/SKILL.md:89`,
`skills/patterns-adopt-first/SKILL.md:99`, `skills/review-code-minimalism/SKILL.md:26`,
`skills/review-feasibility/SKILL.md:3,11`, `agents/devils-advocate.md:20`, plus
`authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md` and
`review-brainstorm/references/EXTERNAL-CLAIMS.md`.

## Blocking findings

The compressor verified body-to-reference pointers but never checked the reverse direction. Three
pointers in the untouched references now name sections the cut deleted or renamed.

**1. `references/ARTIFACT-CHECKLISTS.md:9` — dead pointer to a deleted section.**
Reads: "The verdict is derived **mechanically** from the issue counts (see SKILL.md → Verdict
Criteria)". The body no longer has a Verdict Criteria section; it only forwards to
`ISSUE-TEMPLATES.md`. Re-point this line directly at `ISSUE-TEMPLATES.md` § Verdict Criteria,
which is the actual single source.

**2. `references/ARTIFACT-CHECKLISTS.md:18` and `:22` — dead pointer to the deleted Review Focus
table.** Line 18 reads "graded from the Review Focus row in SKILL.md"; line 22 reads "the SKILL.md
row is the complete check set". The surviving home is the body paragraph
``**Cycle cards (`tasks.md`) — the complete check set:**``. Change to "the Cycle cards check set
in SKILL.md" and "the SKILL.md paragraph is the complete check set". This is the very pointer the
strip and ADR say the cut preserved the cycle-card set in order to protect — the content was
protected, the label was not.

**3. `references/ARTIFACT-CHECKLISTS.md:219` and `references/ISSUE-TEMPLATES.md:135` — stale
section name.** Both read "SKILL.md → Incremental Review Mode"; the section is now titled
"Incremental mode". Rename in both files.

**4. Correct the false claim in two places.** `.mochiko/strips/review-plan-artifacts.md`
`[v0.86.0]`, final sentence of Consumers assessed, and
`evals/review-plan-artifacts/pass-report.md:163` both assert "No dead pointers created."
Findings 1 through 3 disprove it. Replace with the accurate statement and record the three
re-labels. Because the reference files are part of the shipped primitive, those edits belong
inside this same v0.86.0 strip entry riding the same ruling — not a fresh ceremony.

## Advisory, not blocking

- **R-069 is partial in the body.** The spot-check enumeration names three of the rule's four
  targets ("entity names, requirement IDs, decision references") and drops *constraint
  alignment*. The rule still has a home — `ARTIFACT-CHECKLISTS.md` § Named consistency groups is
  explicitly "the spot-check lens", and the body routes there — so this is a subset, not a
  contradiction. Making it exact costs 21 chars and stays well inside budget.
- **R-085 survives form-changed.** The "run the Quality Checklist before finalizing" meta-rule
  now lives only as the Protocol chain plus "never skip a check". All 15 of its items survive
  individually, and the strip discloses that the checklist-as-section form dies by ruling, so
  this is acceptable as ruled.
- **Ledger bookkeeping drift.** The description row records winner 500 / budget 625, but the
  description measures 589 and did so before this cut as well. Pre-existing, inside budget, not
  this edit's defect — correct it at the next release-gate sweep.

Fixes 1 through 4 are the only things between this cut and PASS. Re-dispatch the audit once they
land.

---

# Round 2 — bounded delta verify (2026-08-26)

**Verdict: PASS.** All four blocking findings are discharged. Scope of this pass was the delta
only: the five label edits, no check content changed, the record-layer corrections now true, and
no new dead pointer.

## The five label edits landed as claimed

`git diff` against HEAD over `references/` is **5 insertions, 5 deletions across 2 files** —
exactly the five lines, nothing else:

| Location | Before | After |
|---|---|---|
| `ARTIFACT-CHECKLISTS.md:9` | `(see SKILL.md → Verdict Criteria)` | `(see ISSUE-TEMPLATES.md → Verdict Criteria)` |
| `ARTIFACT-CHECKLISTS.md:18` | `the Review Focus row in SKILL.md` | `the Cycle cards check set in SKILL.md` |
| `ARTIFACT-CHECKLISTS.md:22` | `the SKILL.md row is the complete check set` | `the SKILL.md paragraph is the complete check set` |
| `ARTIFACT-CHECKLISTS.md:219` | `SKILL.md → Incremental Review Mode` | `SKILL.md → Incremental mode` |
| `ISSUE-TEMPLATES.md:135` | `SKILL.md → Incremental Review Mode` | `SKILL.md → Incremental mode` |

## No check content changed

All three `ARTIFACT-CHECKLISTS.md` hunks sit in prose — the file preamble (lines 6–22) and the
Cross-Artifact Consistency preamble (line 219). The `ISSUE-TEMPLATES.md` hunk sits in the
Assembled report prose. **No checklist table row, severity, question, or anchor was touched.**
Every one of the 31 reference-homed rules is therefore untouched and still homed.

## Every new label resolves

- `ISSUE-TEMPLATES.md → Verdict Criteria` → `## Verdict Criteria` exists at
  `ISSUE-TEMPLATES.md:141`.
- `the Cycle cards check set in SKILL.md` → body line 14,
  ``**Cycle cards (`tasks.md`) — the complete check set:**``. The new label now matches the
  body's own wording.
- `the SKILL.md paragraph is the complete check set` → same paragraph; "paragraph" is now
  accurate where "row" was not.
- `SKILL.md → Incremental mode` (both files) → body line 16, `**Incremental mode**`.

A sweep for every remaining `SKILL.md` mention across both references returns four hits, all four
carrying correct labels. Line 9 no longer points at `SKILL.md` at all.

## Body untouched by the fix round

Re-measured with the canonical snippet: **body 4,901, `description:` 589** — identical to round 1.
`git diff --stat` on `SKILL.md` is unchanged at 6 insertions / 200 deletions against HEAD, so the
round-1 pre-assert stands without re-derivation.

## Record-layer corrections are now true

- `.mochiko/strips/review-plan-artifacts.md:99–109` — the false sentence is replaced by a
  **"Reference-to-body pointer re-labels (audit round 1, blocking — ride this same ruling)"**
  bullet. It admits the original false claim, names all five edits with before/after, states
  "label text only (no check content touched)", and closes "After these re-labels: no dead
  pointers." Verified true against the actual diff. Correctly framed as riding the same v0.86.0
  ruling rather than opening a fresh ceremony.
- `evals/review-plan-artifacts/pass-report.md:165–167` — corrected the same way, consistent with
  the strip.

## No new dead pointer introduced

Confirmed by the exhaustive `SKILL.md` sweep above plus target-existence checks on all four
surviving pointers and the re-pointed line 9.

## Note outside the delta scope

A parallel persona-hygiene wave (`.mochiko/decisions/2026-08-26-persona-hygiene-pass.md`) landed
a `devils-advocate` description rewrite at the same v0.86.0 stamp. Since that agent is a consumer
of this skill, its lines were re-checked: `agents/devils-advocate.md:20` is byte-unchanged
("completeness review of the plan analysis/design sets and the cycle cards (`tasks.md`)"), the
`skills:` mount still lists `review-plan-artifacts`, and the Critical/Important/Minor severity
trio is intact. **R-058 and R-109 still hold**, so the round-1 consumer verification stands.

## Standing advisories (unchanged, non-blocking)

R-069 declined by the lead, with the reference's Named consistency groups staying the lens home —
consistent with the subset-not-contradiction read recorded above. R-085 form change remains
acceptable as ruled. The ledger's description-row drift (records 500, measures 589, inside budget)
is still owed a correction at the next release-gate sweep. `CHANGELOG.md` gate 4 and the
`marketplace.json` sync are still owed at the `plugin.json` bump to 0.86.0.

---

*Version re-stamp note (lead, 2026-08-26): this landing ships as **v0.87.0** — v0.86.0 was
claimed by the parallel persona-hygiene wave (session coordination recorded in the strip
entry's version stamp). Every "v0.86.0" in the audit text above refers to this landing and
reads as v0.87.0; the verdicts and measurements are unaffected.*
