# Compression pass report — `review-plan-artifacts`

Pass opened 2026-08-26. Compressor: session lead. Mode: **user-directed cut** on the
`review-brainstorm` v0.83.0 true-deletion precedent (itself on the `review-feasibility`
v0.82.0 "cut now, eval validates later" ruling — the pre-cut eval grid is superseded as an
instrument and re-purposed as a post-cut regression check).

## Pass narrative

1. User directive: reduce the skill by 90%, like the `review-brainstorm` pass.
2. Route chosen from the precedent directly: **true deletion, single file** — no new
   reference file, no relocation (the review-brainstorm first-draft breakup was rejected by
   the user as verbosity-shifting; that lesson is inherited, not relearned). The two
   existing `references/` files and the `description:` are untouched.
3. Rule inventory (113 entries: 11 floor · 70 must · 16 format · 16 vocab) authored by a non-compressor seat (`rules.json`, this directory) — the
   independence requirement of the `compressing-skills` procedure, satisfied by authorship
   rather than after-the-fact review.
4. Draft: every behavioral rule of the baseline body survives as a compressed clause, or is
   deleted only where its single source already lives in an untouched `references/` file
   (the mirror-checklist tables) — mapped span-by-span below. All rationale prose, worked
   framing, and long-form tables deleted.
5. Rule-complete floor as measured: **4,901 chars (−63.8%)** (after three inventory-driven clause restorations: R-070, R-093, R-098). Strict −90.0% (≤1,352) requires
   deleting ruled/protected rule families outright — named at the ratification gate.

## Measurements (chars, Python `len`, never `wc -c`)

| Surface | Baseline | Draft | Δ |
|---------|----------|-------|---|
| `SKILL.md` body | 13,521 | 4,901 | **−63.8%** |
| `SKILL.md` `description:` | 589 | untouched (out of scope, D6) | 0 |
| `references/ARTIFACT-CHECKLISTS.md` | 23,387 | untouched | 0 |
| `references/ISSUE-TEMPLATES.md` | 4,767 | untouched | 0 |
| `scripts/check-artifacts.py` | 13,504 | out of scope (D6) | 0 |

No new files. On ratification, body budget re-seeds per R11 (landed body × 1.25, rounded up)
in `.mochiko/memory/primitive-cost-budgets.md` (current row: 10,855 / 13,569).

## Why the rule-complete floor sits higher than `review-brainstorm`'s (−78.8%)

Three structural differences, not looser compression:

1. **The cycle-card check set is body-only.** `references/ARTIFACT-CHECKLISTS.md` states
   "This file carries no separate cycle-card checklist; the SKILL.md row is the complete
   check set" — seven ruled checks (incl. v0.75.0 oracle semantics, v0.76.0 `--check`
   citation) with no reference home. Relocation is fenced (D5).
2. **Incremental Review Mode is doubly protected** ([v0.15.0] KEPT report-shape block,
   [v0.26.0] KEPT section) and plan-only unique — its escalation rules and
   `consistency_checks:` field set have no other home.
3. **Four lenses, three of them consumer-cited.** The router row, `patterns-adopt-first`,
   `review-code-minimalism`, `review-feasibility`, `devils-advocate`, and
   `TRACEABILITY-PATTERNS.md` each cite this skill's lens vocabulary (conformance /
   material divergence auto-FAIL / rung honesty advisory / adopt-first BLOCKING /
   cross-artifact consistency grade owned here) — paraphrase that breaks a consumer is a
   failed rule (R10).

## Protected-content reconciliation (R2)

`.mochiko/strips/review-plan-artifacts.md` read end to end before drafting. Protected set
and its fate in the draft:

1. **[v0.15.0] KEPT: "Report shape (incremental mode)" block** — survives compressed: the
   yaml example's wording is superseded; its substance (the `incremental: true` flag, the
   `full_review:` / `consistency_only:` scope lists, the six `consistency_checks:` fields,
   fail-also-lands-as-finding) survives as clauses in the Incremental-mode paragraph.
2. **[v0.26.0] KEPT: Red Flags (incl. the two generic bullets), Common Rationalizations,
   Incremental Review Mode** — Red Flags and Rationalizations survive compressed into
   **Floors** (letter-IS-the-spirit, this-case-is-different, severity-down rationalization,
   noted-but-not-blocking, feature-size / seniority / time-pressure / found-enough,
   vague-spec propagation, N/A-with-justification); Incremental Review Mode survives as its
   own paragraph. The KEPT survivor status of their *long form* ends by this ruling.
3. **[v0.64.0] guardrails keep-set** — Scope table → the sibling-boundary sentence +
   boundary-table pointer; Review Focus table → the cycle-card set survives complete, the
   analysis / store-delta / design / cross-artifact rows supersede to their single-source
   checklists in ARTIFACT-CHECKLISTS.md (each row token verified present there — see
   disposition map); Issue/Verdict pointers → survive; **Step 2 deterministic pre-assert →
   survives as Protocol leg 1** (failed count is ground truth, non-waivable); Incremental
   mode → survives; Quality Checklist → each item's rule survives in Lenses / Protocol /
   Floors (itemized in the disposition map); Common Mistakes → compressed into Floors +
   the ISSUE-TEMPLATES Anti-Patterns list (pre-existing single source); Related → inline
   pointers (`advocate-report-template` named in Protocol).
4. **[v0.64.0] review-evidence floor line** — substance intact, verbatim wording superseded
   by its compressed form in Floors (same treatment as review-brainstorm v0.83.0).
5. **[v0.67.0] three-lens re-key** (conformance BLOCKING · rung-honesty advisory ·
   completeness-within-scope · verdict precedence override · the hunt-class-7 seam
   sentence) — survives compressed, no rule deleted.
6. **[v0.53.0] code-review punt carve-out** — survives ("sole carve-out
   `mochiko:review-code-minimalism`, implement-side").
7. **[v0.75.0] oracle semantics (D2) + time-anchor/foundation kill (D1/D3)** — oracle
   semantics survives with its ruled wording; no time anchor or foundation word
   reintroduced.
8. **[v0.76.0] `tasks --check` citation (D7/D8)** — survives with both arms (binary +
   raw-schema fallback).
9. **[v0.81.0] store-delta re-key (D3/D10/D12/D14)** — the store-delta grade survives via
   the completeness lens (store delta when carried, else the no-delta claim); the detailed
   checks' single source is already the ARTIFACT-CHECKLISTS Store Delta section (written at
   v0.81.0); `architecture_conformance` survives in the `consistency_checks:` field list.
10. **Adopt-first disclosure lens (v0.73.0 wave)** — survives compressed at conformance
    strength with the `patterns-adopt-first` pointer and the advisory limb.

## Disposition map (baseline body section → home in the draft)

Verbatim home for all removed text: git history of this SKILL.md, pre-cut.

- *Overview ¶1 (three-lens prose + not-implementation framing)* — compressed into the
  opening paragraph + Lenses.
- *Letter/spirit paragraph (L16 aphorism)* — compressed into Floors ("the letter IS the
  spirit").
- *Review-evidence floor line* — compressed into Floors (wording superseded).
- *Scope table* — compressed to the sibling-boundary sentence; the check-by-check seam
  stays single-sourced in the ARTIFACT-CHECKLISTS boundary table (pointer kept).
- *When NOT to Use (5 bullets)* — compressed to the "Not for:" line (all four targets +
  the drafting-completion condition survive).
- *Review Focus table* — analysis row → ARTIFACT-CHECKLISTS "Analysis Artifacts" (FR→TR
  coverage, orphan TRs, testable criteria, sourced constraints, ≥2 alternatives +
  rationale, IP coverage: all present there); store-delta row → "Architecture Store Delta"
  checklist (delta↔diagram, qualifying-flow, AX-row legality, floor precedence, NFR
  targets, lifecycle keyed to FEAT-XXX, consult record, no-delta claim: all present);
  design row → "Design Artifacts" checklists (entity coverage, relationships, sensitivity,
  endpoint coverage, schemas, error handling, integration boundaries: all present);
  cross-artifact row → "Cross-Artifact Consistency" (alignment, decisions + signed store
  delta honored, ID citation, traceability: all present); **cycle-cards row → survives in
  the body, complete** (no reference home exists — the reference explicitly punts to this
  row). The brownfield-discovery out-of-scope note → already stated in ARTIFACT-CHECKLISTS
  ("Out of scope here"); the sequencing-is-the-lead's clause → "over whichever sets the
  caller supplies".
- *Issue Classification section* — compressed to the ISSUE-TEMPLATES pointer in Protocol.
- *Review Process / Step 2 pre-assert* — survives as Protocol leg 1 (command line, failed
  count ground truth, fold-in).
- *Incremental Review Mode (3 subsections + report-shape yaml)* — compressed to one
  paragraph; every escalation rule, the time budget, the never-you scope split, and the
  report-shape fields survive as clauses.
- *Verdict Criteria section* — compressed into Protocol (mechanical counts + divergence
  override) and Lenses (rung honesty never drives the verdict).
- *Quality Checklist (15 items)* — each item's rule survives: pre-assert run (Protocol) ·
  graded against approved proposal (opening + Lenses) · conformance / divergence
  (Lenses) · adopt-first (Lenses) · rung honesty (Lenses) · all applicable checks
  (Protocol "run every applicable check") · store-delta / no-delta (Lenses) · oracle
  semantics (Cycle cards) · severity classification + evidence + actionable fixes +
  verdict-matches-counts + strengths (Protocol) · cross-artifact concerns (completeness
  lens + Incremental mode) · feasibility handoff (opening).
- *Common Mistakes table (6 rows)* — over-classification → ISSUE-TEMPLATES "Severity
  inflation" (pre-existing) + Floors; missing evidence / vague suggestions → Protocol +
  ISSUE-TEMPLATES; implementation details → "Not for: code review" + ISSUE-TEMPLATES
  "Implementation focus"; skipping cross-artifact → Incremental floors; feasibility →
  opening handoff clause.
- *Red Flags (6 bullets) + Common Rationalizations (7 rows)* — compressed into Floors (see
  reconciliation item 2).
- *Related (3 bullets)* — inline: `review-feasibility` (opening), `patterns-plan-minimalism`
  (Lenses), `advocate-report-template` (Protocol).

## Consumers assessed (R10)

`agents/devils-advocate.md` (mounts the skill; completeness + cycle-cards role intact) ·
router `skills/mochiko/SKILL.md:89` (conformance / material divergence / rung honesty /
store delta / no-delta / cycle cards / 3-state verdict — every cited term survives verbatim)
· `patterns-adopt-first` (BLOCKING sibling check — survives) · `review-code-minimalism`
(rung-honesty grade here — survives) · `review-feasibility` (boundary vocabulary — survives;
its side untouched) · `authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md`
(cross-artifact consistency grade owned here — survives) ·
`review-brainstorm/references/EXTERNAL-CLAIMS.md` (disclosure-line consumer list — the
disclosure check lives in the ARTIFACT-CHECKLISTS consistency checklist, untouched).
`commands/plan.md` names neither the skill nor its section anchors (dispatch rides the
devils-advocate seat). Audit round 1 caught three stale reference→body pointer labels the
cut created (Verdict-Criteria see-pointer, the cycle-card "Review Focus row" wording at
lines 18/22, the "Incremental Review Mode" section name at ARTIFACT-CHECKLISTS:219 +
ISSUE-TEMPLATES:135) — re-labeled in the same landing, label text only, recorded in the
strip entry. After the re-labels: no dead pointers.

## Eval status (deferred)

Post-cut regression check pending, per the v0.82.0/v0.83.0 pattern: `rules.json` (this
directory, non-compressor-authored from the baseline body) + 3 goldens; probe before any
priced grid; `preregistration.md` before the grid. A lost load-bearing rule re-adds via the
strips re-add path, never by silent edit.

## Ratification gate

Pending user ruling: ship the rule-complete cut (−63.8%) · force deeper toward −90% with the
rule deaths named · ship nothing.
