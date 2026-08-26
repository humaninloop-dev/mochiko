# Compression pass report — `review-specifications`

Pass opened 2026-08-26, directly after the `review-plan-artifacts` v0.87.0 landing.
Compressor: session lead. Mode: **user-directed cut** on the standing true-deletion
precedent (v0.82.0 review-feasibility · v0.83.0 review-brainstorm · v0.87.0
review-plan-artifacts — "cut now, eval validates later"; pre-cut eval grid superseded, slot
runs as a post-cut regression check).

## Pass narrative

1. User directive: same cut as the prior passes; `review-specifications` selected as the
   fattest remaining `review-*` body (12,184 chars) at the candidate survey.
2. Route: **true deletion, single file** — this skill has **no `references/` directory**, so
   there is no relocation temptation and no reference-homed bucket: every surviving rule
   must survive in the body (or as an explicit pointer to another primitive's single
   source). `description:` untouched (490 chars, out of scope D6).
3. Rule inventory (81 entries: 7 floor · 54 must · 1 should · 8 format · 11 vocab) authored by a non-compressor seat (`rules.json`, this directory).
4. Draft: six paragraphs — identity+routing · method (question craft + the six-class
   taxonomy) · feature layer (baseline rule + 10 checks) · Screens & Flows (walk rules + 8
   checks) · severity+output · floors. All worked-example tables, the process/checklist
   sections, and rationale prose deleted; every check, class, and floor survives as a
   compressed clause.
5. Rule-complete floor as measured: **6,187 chars (−49.2%)** (after five inventory-driven clause restorations: R-012, R-015, R-064, R-065, R-066).

## Why the rule-complete floor is the shallowest of the four passes

This skill is nearly all rules and has nowhere to lean:

1. **Zero reference files.** The three earlier cuts leaned on untouched single-source
   references (28k / 17k / 10k chars of checklist data). Here every ruled check lives in
   the body alone.
2. **18 ruled checks in two tables** — the feature-layer 10 (feature-map-layer D16/R13/D21/D8,
   v0.58.0) and the Screens & Flows 8 (ux-mocking D7, v0.50.0) — each check an
   independently ruled grading obligation with a severity.
3. **The six-class defect taxonomy is a consumer-anchored canonical home** —
   `devils-advocate:52` points here for "the canonical gap taxonomy, severity rubric, and
   structured output format" (the v0.25.0 RETURNED landing exists precisely because this
   pointer was once dishonest). Class 6 carries the v0.67.0 calibration clause.
4. **Already twice-compressed**: v0.26.0 editorial wave (−23% lines) and the v0.63.0
   benchmark-ruled guardrails cut (When-to-Use + nine-step process deleted). The remaining
   prose was thin before this pass.

## Measurements (chars, Python `len`, never `wc -c`)

| Surface | Baseline | Draft | Δ |
|---------|----------|-------|---|
| `SKILL.md` body | 12,184 | 6,187 | **−49.2%** |
| `SKILL.md` `description:` | 490 | untouched (out of scope, D6) | 0 |

No references, no scripts. On ratification, budget re-seeds per R11 (current row:
11,271 / 14,089).

## Protected-content reconciliation (R2)

`.mochiko/strips/review-specifications.md` read end to end before drafting. Protected set
and its fate in the draft:

1. **[v0.26.0] KEPT: severity table + Core Principle table** — the severity rubric survives
   with its spec-specific wording ("cannot build without this answer" / "will cause
   rework" / "polish, log and defer") as one compressed line; the Core Principle survives
   as the product-not-implementation rule with its altitude sentence — the wrong/right
   worked-example rows die (teaching prose). The KEPT status of both *table forms* ends by
   this ruling.
2. **[v0.25.0] RETURNED five-class table (canonical-home landing)** — all five classes
   survive with compact descriptors; the canonical-home relationship ("the canonical hunt
   taxonomy `devils-advocate` leans on") survives verbatim-in-substance.
3. **[v0.67.0] class-6 excess row** — survives with its full calibration clause (driver /
   cheaper shape / floor-compliance-NFR never excess).
4. **[v0.63.0] guardrails keep-set** (Overview · When NOT to Use · Core Principle ·
   Question Format · Gap Categories incl. external-claims verify · feature-layer table ·
   S&F table · Severity · Output Format · density paragraph · Quality Checklist · Common
   Mistakes · Related Skills) — every member's obligation survives as a clause; the
   *section forms* end by this ruling. Quality-Checklist items each map to a surviving
   clause; Common-Mistakes rows carrying distinct rules survive in Floors (5–7 gaps per
   round · scope creep is not a gap · check existing context first · related gaps
   grouped); rows duplicating Method rules ride those clauses.
5. **[v0.63.0] review-evidence floor line** — substance intact, wording compressed under
   this ruling.
6. **[v0.58.0] feature-layer 10-check table + R13 git-baseline rule** — all 10 checks and
   the baseline rule survive compressed, severities kept; the map-machinery
   single-source pointer to `authoring-feature-map` survives.
7. **[v0.50.0] Screens & Flows section (UX-D7)** — both legal shapes, the serve-and-click
   obligation, the authority split, and all 8 checks survive compressed.
8. **[v0.53.0] code-review carve-out** — survives in the Not-for line.
9. **[v0.82.0] envelope-citation wording** ("never prose style; undisclosed overage …
   advisory finding per its rule 8") — survives verbatim-in-substance.

## Disposition map (baseline body section → home in the draft)

Verbatim home for all removed text: git history of this SKILL.md, pre-cut.

- *Overview* — compressed into the opening line (input-not-verdict, WHAT-not-HOW).
- *When NOT to Use (6 bullets)* — the "Not for:" line; all six routes survive.
- *Core Principle + wrong/right table (4 example rows)* — the rule survives in Method; the
  example rows die (one contrast pair's substance carried by the altitude sentence).
- *Question Format* — Method clause (2–3 options · user meaning · why it matters ·
  Clarifications shape, never a variant).
- *Gap Categories (2 tables + 2 paragraphs)* — Method: the five category names, the six
  classes with descriptors, the posture-smuggling rule with its defining example, the
  external-claims verify pointer.
- *The feature layer (paragraph + 10-check table)* — the Feature-layer paragraph, complete.
- *Screens & Flows (paragraph + 8-check table)* — the Screens & Flows paragraph, complete.
- *Severity Classification table* — one line in Severity + output.
- *Output Format* — same paragraph (template single source + inline fallback).
- *Review Process (2 paragraphs)* — Floors (density/substance/rule-8 advisory; the floor
  line).
- *Quality Checklist (11 items)* — each item's rule survives in Method / Feature layer /
  S&F / Floors; the section form dies.
- *Common Mistakes (8 rows)* — distinct rules into Floors; duplicates ride Method.
- *Related Skills (3 bullets)* — inline pointers (authoring-requirements in Not-for;
  authoring-feature-map in Feature layer; analysis-iterative in Not-for).

## Consumers assessed (R10)

`agents/devils-advocate.md:52` (canonical gap taxonomy · severity rubric · structured
output format — all three anchors survive) and `:19` (gap review of drafted spec) · router
`skills/mochiko/SKILL.md:60` (feature-layer grade included — survives) ·
`authoring-prototype` (prototype graded with the spec by this skill, independent reviewer —
survives) · `authoring-feature-map` (derivation + staged map delta graded here in one pass
— survives) · `analysis-iterative` (disjoint-trigger boundary — survives in the Not-for
line) · `review-brainstorm/references/EXTERNAL-CLAIMS.md:94` (consumer listing — the verify
pointer survives). No dead pointers created (this skill has no references; the one outbound
relative link, EXTERNAL-CLAIMS.md, is unchanged).

## Eval status (deferred)

Post-cut regression check pending, per the standing pattern: `rules.json` (this directory,
non-compressor-authored) + 3 goldens; probe before any priced grid; `preregistration.md`
before the grid. A lost load-bearing rule re-adds via the strips re-add path.

## Ratification gate

Pending user ruling.
