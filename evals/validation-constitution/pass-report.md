# Compression pass report — `validation-constitution`

Pass opened 2026-08-26 (sixth of the compression series). Compressor: session lead. Mode:
user-directed true-deletion cut on the standing precedent (v0.82.0/v0.83.0/v0.87.0/v0.88.0
— "cut now, eval validates later"; eval slot = post-cut regression check).

## Measurements (chars, Python `len`, never `wc -c`)

| Surface | Baseline | Draft | Δ |
|---------|----------|-------|---|
| `SKILL.md` body | 7,630 | 5,103 | **−33.1%** |
| `SKILL.md` `description:` | 481 | untouched (out of scope, D6) | 0 |
| `references/QUALITY-CHECKLIST.md` + `ANTI-PATTERNS.md` | 10,124 | untouched | 0 |

On ratification, budget re-seeds 6,734/8,418 → 5,103/6,379 (R11).

## Why the floor is shallow (−33.1%)

**Already D4-cut at the v0.63.0 benchmark wave** (12,358 → 6,915, −44%: the six procedure
steps superseded then, their checks single-sourced to `references/QUALITY-CHECKLIST.md`).
The current body IS the keep-set plus the v0.65.0 adaptive-depth extensions. This pass
removes the remaining table forms (Common Mistakes · Red Flags · Common Rationalizations,
~2,300 chars — each distinct rule surviving as a Floors clause) and compresses the
VALIDATION RESULT fenced block to a field-enumeration clause (every field kept).

Rule inventory: 69 entries (9 floor · 36 must · 14 format · 10 vocab),
non-compressor-authored (`rules.json`). One inventory-driven restoration pre-gate: the
R-047 red-flag STOP-and-restart meta-rule with its named rationalization family
(R-048/051/052/054/055).

## Protected-content reconciliation (R2)

`.mochiko/strips/validation-constitution.md` read end to end. Protected set and fate:

1. **[v0.25.0] KEPT: Red Flags + Common Rationalizations pair** — every distinct rule
   survives as a Floors clause (STOP-and-restart meta-rule; minor-update; authoring≠
   validation; user-satisfaction; anti-patterns-apply; placeholder-scan-regardless;
   pragmatism inversion; later-rarely-comes; user-asked-to-skip with
   document-against-recommendation; missing-parts-FAIL). The KEPT status of the *table
   forms* ends by this ruling.
2. **[v0.63.0] guardrails keep-set** (set-not-file goal + VALIDATION RESULT block ·
   every-set-MUST-pass + letter-is-spirit floors · Step-1 checklist assembly with both
   reference pointers · anti-rationalization triad · version bump · Related) — every
   member's obligation survives; the VALIDATION RESULT fenced form compresses to a
   field-complete clause; section forms end by this ruling.
3. **[v0.63.0] review-evidence floor line** — substance intact, wording compressed.
4. **[v0.65.0] adaptive-depth extensions** (floor + declared level accounting line;
   MAJOR-bump low→high flip) — survive verbatim-in-substance.
5. **[v0.76.0] schema re-key** — lives in `references/QUALITY-CHECKLIST.md`, untouched.

## Disposition map (summary; verbatim home: git history pre-cut)

Overview → opening paragraph (set-not-file, graded properties, must-pass, letter-is-spirit,
evidence floor moved to Floors). When NOT to Use → Not-for line + verify-it-IS-a-set
clause. Core Process inputs → Inputs paragraph (from-file-never-report; the five inputs
with exact locations; three missing-input FAILs; stale constitution.md flag). Step 1 →
Protocol leg 1 (module-parameterized assembly, routed content, verify-every-item,
never-unselected-fragments). Quantification Requirements → Protocol legs 2–3 (measurable
criteria + ANTI-PATTERNS pointer with the three named patterns; the excess-governance
anti-pattern with full GI-017 calibration). Step 8 table → Protocol leg 4 (all MAJOR/MINOR/
PATCH triggers; examples die). Step 9 fenced block → VALIDATION RESULT paragraph (all
fields + sub-items enumerated). Common Mistakes / Red Flags / Rationalizations → Floors.
Related → closing producer-pair clause.

## Consumers assessed (R10)

`agents/validator.md:9,28` (mount + authoritative-binary-grade framing — intact) · router
`:31,:43,:146` (authoritative grade, default FAIL, set members, trace closure — survive) ·
`authoring-constitution:34,130` (producer↔validator pair, never co-mounted — survives in
the closing clause) · `testing-governance-injection:23` (trace/structure/placement boundary
— survives) · `review-governance-intent:33` (downstream Tier-2 family — survives) ·
`analysis-codebase:121` (optional pointer — survives) · `schemas/governance-surfaces.yaml`
(set vocabulary — survives). No dead pointers created (both references untouched).

## Eval status (deferred)

Post-cut regression check pending per the standing pattern; `rules.json`
non-compressor-authored.

## Ratification gate

Pending user ruling.
