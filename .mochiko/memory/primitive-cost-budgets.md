# Primitive cost budgets — the D7 char-budget ledger

Provenance: `validator-scope-and-verbosity` D7 (record:
`.mochiko/brainstorms/validator-scope-and-verbosity/record.md`), the guardrails-vs-detail
benchmark verdict (`DECISIONS.md` 2026-08-10 benchmark-verdict row;
`.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`), the Wave 1 build
(v0.63.0), and the Wave 2 editorial extension (v0.64.0 — budgets seeded from the audited cut
results per the Wave 2 sketch in `report/build-plan.md`). Cited by the char-budget pre-assert
in `.claude/rules/mochiko/primitive-edits.md`.

**The rule.** A budget is the measured winning-variant character count **+25% headroom**
(rounded up). Budgets are always measured, never invented: a new budget enters this ledger only
from a benchmark-measured winner or a ruled editorial cut shipped through the edit ceremony
(Wave 2 seeded its budgets from its own audited cut results this way at v0.64.0). All counts
are **characters of the parsed value — never `wc -c` bytes** (byte counts over-reject
unicode-bearing text that is under the char cap; a v1 census defect was exactly this).

**Classes measured:** skill body (content after the closing frontmatter `---`) · skill
frontmatter `description:` value · agent frontmatter `description:` value. `references/` files
are exempt (on-demand data, never auto-loaded).

## Skill bodies

Wave 1 rows (benchmark winners, v0.63.0) and Wave 2 rows (audited editorial-cut results,
v0.64.0) share the same rule. `authoring-architecture` and `grooming-operating-docs` were
audited Wave 2 body no-ops (nothing D4-cuttable); their budgets are their surviving bodies
+25%, same as every other row.

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 3,942 | 4,928 |
| analysis-codebase | 6,509 | 8,137 |
| authoring-constitution | 17,240 | 21,550 |
| authoring-feature-map | 12,330 | 15,413 |
| review-governance-intent | 7,089 | 8,862 |
| validation-constitution | 6,734 | 8,418 |
| testing-governance-injection | 3,540 | 4,425 |
| authoring-requirements | 4,101 | 5,127 |
| authoring-user-stories | 5,361 | 6,702 |
| authoring-prototype | 8,898 | 11,123 |
| review-specifications | 11,271 | 14,089 |
| authoring-architecture | 5,250 | 6,563 |
| authoring-technical-requirements | 10,628 | 13,285 |
| brownfield-integration | 6,342 | 7,928 |
| executing-tdd-cycle | 9,676 | 12,095 |
| grooming-operating-docs | 2,666 | 3,333 |
| testing-end-user | 13,125 | 16,407 |
| patterns-api-contracts | 10,729 | 13,412 |
| patterns-code-minimalism | 3,455 | 4,319 |
| patterns-entity-modeling | 13,468 | 16,835 |
| patterns-system-design | 8,837 | 11,047 |
| patterns-technical-decisions | 4,626 | 5,783 |
| patterns-vertical-tdd | 5,189 | 6,487 |
| review-brainstorm | 11,508 | 14,385 |
| review-feasibility | 15,246 | 19,058 |
| review-plan-artifacts | 10,855 | 13,569 |
| review-code-minimalism | 3,689 | 4,612 |

The v0.63.0 floor-line additions (analysis-iterative, review-governance-intent,
validation-constitution, review-specifications) and the v0.64.0 ones (review-brainstorm,
review-feasibility, review-plan-artifacts) ride inside these budgets — the +25% headroom
absorbed them by design.

## Skill descriptions

Hard cap for every skill description, budgeted or not: **1,536 chars** (the delivery-truncation
cap).

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 476 | 595 |
| analysis-codebase | 349 | 437 |
| authoring-constitution | 481 | 602 |
| authoring-feature-map | 495 | 619 |
| review-governance-intent | 483 | 604 |
| validation-constitution | 481 | 602 |
| testing-governance-injection | 483 | 604 |
| authoring-requirements | 379 | 474 |
| authoring-user-stories | 425 | 532 |
| authoring-prototype | 493 | 617 |
| review-specifications | 490 | 613 |
| authoring-architecture | 488 | 610 |
| authoring-technical-requirements | 496 | 620 |
| brownfield-integration | 491 | 614 |
| executing-tdd-cycle | 498 | 623 |
| grooming-operating-docs | 490 | 613 |
| testing-end-user | 500 | 625 |
| patterns-api-contracts | 486 | 608 |
| patterns-code-minimalism | 564 | 705 |
| patterns-entity-modeling | 497 | 622 |
| patterns-system-design | 541 | 677 |
| patterns-technical-decisions | 469 | 587 |
| patterns-vertical-tdd | 496 | 620 |
| review-brainstorm | 491 | 614 |
| review-feasibility | 500 | 625 |
| review-plan-artifacts | 500 | 625 |
| review-code-minimalism | 492 | 615 |

`patterns-system-design` (541) and `patterns-code-minimalism` (564) include the v0.64.0
RETURNED clauses (fire-rate probe evidence, user-ruled — see their strips); their budgets
derive from the restored values.

## Agent descriptions

| agent | winner chars | budget (+25%) |
|---|---|---|
| principal-architect | 756 | 945 |
| validator | 269 | 337 |
| devils-advocate | 316 | 395 |
| requirements-analyst | 303 | 379 |
| product-manager | 438 | 548 |
| product-engineer | 392 | 490 |
| qa-engineer | 251 | 314 |
| staff-engineer | 274 | 343 |
| tech-lead | 719 | 899 |
| technical-analyst | 402 | 503 |

Wave 2 agent rows (v0.64.0) use the audited measurements (auditor's reproducible canonical-snippet
counts, +1 over the shipper's block-scalar parse — the audited number is authoritative).

v0.67.0 rotation (architect-role restructure, audit-measured per the Wave-2 seeding precedent):
`system-architect` row retired with the persona (was 482/603 — strip: `.mochiko/strips/system-architect.md`);
`tech-lead` seeded from its audited description (719); `principal-architect` re-measured at its
rewritten description (756 — the rotation was a ruled edit shipped through the ceremony, audit PASS).
`patterns-plan-minimalism` (new skill, v0.67.0) deliberately unbudgeted — ships hard-cap-only
(desc ≤1,536); audited at body 4,079 / desc 499; a body budget may seed from a future ruled cut
or benchmark, never invented.
`patterns-map-minimalism` (new skill, v0.68.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); audited at body 4,662 / desc 499 (five-guardrail carrier, disclosed size class above
its two siblings — justification ruled HOLDS at audit); a body budget may seed from a future
ruled cut or benchmark, never invented. Standing caution from the same wave's audit, updated at
v0.74.0: `authoring-feature-map` body now measures 15,863 against its 15,413 budget — a
**+450-char total overage** declared and ruled HOLDS at the v0.74.0 V3 audit (the mandated
two-arm schema re-point per schema-based-template-guidance §5 — the feature-entry/features-index
read-pointers swap to the `invoke mochiko-cli template <name>; if absent, Read
plugins/mochiko/schemas/<name>.yaml` form; no restored prose). Overage history: +248 at v0.72.0
(the epic `[EPIC-XXX]` marker grammar + within-epic seam-owner rule, multi-feature-plan-implement
D8/D13) + 202 more at v0.74.0 (the two-arm re-point). Its description 598 against 619 (21 chars,
unchanged). Any further body addition must re-justify its own overage or cut — the budget itself
is unchanged.
`patterns-sound-loop` (new skill, v0.70.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
audited at body 5,729 / desc 500; re-measured body 5,849 at the v0.71.0 neutrality-line
amendment (ruled edit, strip + audit PASS); a body budget may seed from a future ruled cut or
benchmark, never invented.
`patterns-transport-floor` (new skill, v0.71.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); audited at body 5,398 / desc 450; a body budget may seed from a future ruled cut or
benchmark, never invented.
`authoring-epic` (new skill, v0.72.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
audited at body 7,503 / desc 497 (single-sources a whole delivery object across eight
D-rulings, disclosed above its floor siblings — justification ruled HOLDS at audit); a body
budget may seed from a future ruled cut or benchmark, never invented.
`patterns-adopt-first` (new skill, v0.73.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
authored to a plan-set build cap of body ≤6,500 / desc ≤500 and measured at birth at body 6,493
/ desc 497 (single-sources three rulings of the build-vs-off-the-shelf discipline — D2's
two-part obligation with its two-sided limb and external-claim binding, D3's constraint-challenge
route-back, D4's retrofit-cost gate — so it sits above its floor siblings and inside the build
cap); the build cap is a plan-time bound, not a budget: a body budget may seed from a future
ruled cut or benchmark, never invented.

## Unbudgeted primitives

The `mochiko` router skill (body deliberately unbudgeted — its 25k body IS the router index,
the discoverability surface; its description is 206 chars) and all **commands** have no
per-primitive budget: no measured winner or ruled cut exists for them (commands excluded from
both waves by user ruling). They are covered only by the standing hard cap (skill
`description:` ≤ 1,536) plus the justified-exemption path. The former M1 near-cap risk
(four description skills within ~30 chars of the cap) was retired at v0.64.0 — all four are
now budgeted rows above, slimmed to ≤500 and probe-verified (14/14 routing hits).

## How to measure

Canonical measurement (python3, from the repo root; identical results required of every grader):

```python
import re

text = open(PATH).read()
fm, body = re.match(r'^(---\n.*?\n---\n)(.*)$', text, re.S).groups()

# skill body: characters after the closing frontmatter fence
body_chars = len(body)

# description (skill or agent): the parsed value — label stripped,
# folded continuation lines joined with single spaces
m = re.search(r'^description:\s*(.*(?:\n  .*)*)', fm, re.M)
desc_chars = len(' '.join(l.strip() for l in m.group(1).splitlines() if l.strip()))
```
