# Primitive cost budgets — the D7 char-budget ledger

Provenance: `validator-scope-and-verbosity` D7 (record:
`.mochiko/brainstorms/validator-scope-and-verbosity/record.md`), the guardrails-vs-detail
benchmark verdict (`DECISIONS.md` 2026-08-10 benchmark-verdict row;
`.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`), the Wave 1 build
(v0.63.0), and the Wave 2 editorial extension (v0.64.0 — budgets seeded from the audited cut
results per the Wave 2 sketch in `report/build-plan.md`). Cited by the char-budget pre-assert
in `.claude/rules/mochiko/primitive-edits.md`.

**The rule.** A budget is the measured winning-variant character count **+25% headroom**
(rounded up). **A re-asserted or re-measured figure is a current measurement, never a
re-derivation base** — running ×1.25 over a grown body would ratchet the budget up every time a
primitive is edited, which is exactly the drift the budget exists to catch. A budget moves only by
the seeding paths below. Budgets are always measured, never invented: a new budget enters this
ledger only from a benchmark-measured winner or a ruled editorial cut shipped through the edit
ceremony
(Wave 2 seeded its budgets from its own audited cut results this way at v0.64.0). All counts
are **characters of the parsed value — never `wc -c` bytes** (byte counts over-reject
unicode-bearing text that is under the char cap; a v1 census defect was exactly this).

**Classes measured:** skill body (content after the closing frontmatter `---`) · skill
frontmatter `description:` value · agent frontmatter `description:` value. `references/` files
are exempt (on-demand data, never auto-loaded), as are `scripts/` and schema data files.

**Last authoritative sweep: v0.81.0 release gate** (2026-08-19) — every wave-touched skill and
agent re-measured with the canonical snippet in one pass, against the quiesced tree, after all
validator verdicts sealed. 15 primitives measured, 14 matched their recorded figures exactly, 1
had drifted (`authoring-architecture-store`, +54, corrected below). Sweeping at the gate rather
than trusting per-seat reports is what caught it: a figure reported mid-fix-round is a snapshot,
and the tree kept moving after several of this wave's were taken.

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
| ~~authoring-architecture~~ | 5,250 | 6,563 |
| authoring-technical-requirements | 10,628 | 13,285 |
| brownfield-integration | 6,342 | 7,928 |
| executing-tdd-cycle | 9,676 | 12,095 |
| grooming-operating-docs | 2,666 | 3,333 |
| testing-end-user | 13,125 | 16,407 |
| patterns-api-contracts | 10,729 | 13,412 |
| patterns-code-minimalism | 3,455 | 4,319 |
| patterns-entity-modeling | 13,468 | 16,835 |
| patterns-system-design | 9,304 (re-asserted v0.81.0) | 11,047 |
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
| ~~authoring-architecture~~ | 488 | 610 |
| authoring-technical-requirements | 496 | 620 |
| brownfield-integration | 491 | 614 |
| executing-tdd-cycle | 498 | 623 |
| grooming-operating-docs | 490 | 613 |
| testing-end-user | 500 | 625 |
| patterns-api-contracts | 486 | 608 |
| patterns-code-minimalism | 564 | 705 |
| patterns-entity-modeling | 497 | 622 |
| patterns-system-design | 649 (re-asserted v0.81.0) | 677 |
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
| principal-architect | 936 (re-asserted v0.81.0) | 945 |
| validator | 269 | 337 |
| devils-advocate | 316 | 395 |
| requirements-analyst | 303 | 379 |
| product-manager | 438 | 548 |
| product-engineer | 392 | 490 |
| qa-engineer | 299 | 374 |
| staff-engineer | 274 | 343 |
| tech-lead | 888 (re-asserted v0.81.0) | 899 |
| technical-analyst | 402 | 503 |

Wave 2 agent rows (v0.64.0) use the audited measurements (auditor's reproducible canonical-snippet
counts, +1 over the shipper's block-scalar parse — the audited number is authoritative).

`qa-engineer` re-seeded 251→299 at the v0.77.0 audit's drift catch: the v0.75.0 vertical-TDD
build's ruled description rewrite (design-time test-case authoring added) shipped without its
ledger re-measure; 299 is the canonical-snippet count at both the v0.75.0 and v0.77.0 trees,
budget re-derived (+25% → 374). Bookkeeping correction, not a fresh cut.

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
v0.76.0: `authoring-feature-map` body now measures 15,863 against its 15,413 budget — a
**+450-char total overage** declared and ruled HOLDS at the v0.76.0 V3 audit (the mandated
two-arm schema re-point per schema-based-template-guidance §5 — the feature-entry/features-index
read-pointers swap to the `invoke mochiko-cli template <name>; if absent, Read
plugins/mochiko/schemas/<name>.yaml` form; no restored prose). Overage history: +248 at v0.72.0
(the epic `[EPIC-XXX]` marker grammar + within-epic seam-owner rule, multi-feature-plan-implement
D8/D13) + 202 more at v0.76.0 (the two-arm re-point). Its description 598 against 619 (21 chars,
unchanged). Any further body addition must re-justify its own overage or cut — the budget itself
is unchanged. Same-wave correction at v0.80.0: the slice-vocabulary purge SHRANK the body to
15,846 (overage narrows +450 → +433). **v0.81.0: 15,847 (+434)** — the
product-architecture-schema wave's three architecture-pointer re-keys; the +1 is arithmetically
forced by the `authoring-architecture` → `authoring-architecture-store` slug re-point, **no
prose added or restored** (two of the three re-keyed fragments shrank). Declared and ruled HOLDS
at the v0.81.0 audit; budget unchanged.
`patterns-vertical-tdd` body overage, recorded at v0.80.0 (the slice-vocabulary-purge audit):
measures 6,555 against its 6,487 budget (**+68**). History: 6,457 (inside budget) at v0.75.0;
**+86 silent drift at the v0.76.0 mochiko-cli merge's two-arm schema re-point, never ledgered**
— the same re-point whose `authoring-feature-map` +450 WAS declared and ruled HOLDS; this file's
share was missed and is recorded here retroactively. +12 at v0.80.0, declared and ruled HOLDS
at audit (arithmetically forced by the purge's replacement words: "increments" +8 across two
sites, "BUNDLE-" +4 across four link sites; no prose added or restored; description
simultaneously shrank 512 → 497). Budget unchanged — any further body addition must re-justify
its own overage or cut.
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

`patterns-model-tiering` (new skill, v0.77.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); measured at birth at body 4,160 / desc 643 (the description carries the class key's
both tiers plus the D5 boundary — above the ~500 norm, disclosed for the audit's ruling); a
body budget may seed from a future ruled cut or benchmark, never invented. The `explorer`
agent (new, v0.77.0) description measured at birth at 425; no budget until a ruled cut or
benchmark seeds one. (Row historical: the agent was deleted at the v0.78.0 explorer
retarget — strip `.mochiko/strips/explorer.md`; no live budget obligation remains.)

v0.81.0 rotation (the product-architecture-schema Stage-1 wave — one store, `/mochiko:architecture`
desk, D12 absorb): the `authoring-architecture` rows above are **struck through as historical** —
the skill was retired with the wave (D7; strip `.mochiko/strips/authoring-architecture.md`), so
neither its body 5,250/6,563 nor its description 488/610 carries a live obligation. Kept visible
rather than deleted, in the `system-architect` (v0.67.0) and `explorer` (v0.78.0) precedent.
`patterns-system-design` (transformed — altitude and diagram craft now serving store deltas) and
`principal-architect` (recharted as desk lead / store steward) were **re-asserted at v0.81.0**,
both measured at the transformed text and both **inside budget**: `patterns-system-design` body
9,304 against 11,047 (was 8,837) and description 649 against 677 · `principal-architect`
description 936 against 945 (was 756 — the recharter carries the desk-lead and empirical-drift
duties, 9 chars of headroom left). `tech-lead` description 888 against 899 (was 719 — the
store-write grading duty per D11-as-narrowed). No overage anywhere in the rotation, so no
justification is owed.

`authoring-architecture-store` and `patterns-architecture-shelves` (new skills, v0.81.0) are
**unbudgeted at birth** — hard-cap-only (desc ≤1,536); a body budget may seed from a future ruled
cut or benchmark, never invented. Both grew across the wave's validator-ruled fix rounds, so the
birth figure and the landed figure differ and both are recorded:

| skill | birth | landed (v0.81.0) | growth |
|---|---|---|---|
| authoring-architecture-store | body 8,879 / desc 486 | body 10,810 / desc 492 | +1,931 body |
| patterns-architecture-shelves | body 6,584 / desc 473 | body 6,927 / desc 473 | +343 body |

Every added char is a ruled obligation from a fix round — no restored playbook prose. Store: the
validator-ruled rounds, closing with a +5 NA2 reword. Shelves: the V1-A1 rephrase, the stance
suffix lead-in, and the scope-read line, closing with a +37 NB1 check line. The birth figures are
kept because an unbudgeted primitive's birth measurement is what a future ruled cut or benchmark
would seed a budget *from*; the landed figures are what the next edit is measured against.

How these settled, recorded so a later auditor reads the spread as process rather than drift: each
skill's size was reported more than once while its fix rounds were still running — the store at
8,857 before a +22 hyphenation fix gave the 8,879 birth figure, then at 10,751 before the +5 NA2
reword, then at 10,756 before a final +54 (the V4 no-ruled-content phrasing alignment, applied
wave-wide and reaching this skill last); the shelves at 6,890 before the +37 NB1 check line. **The
landed column is the canonical-snippet count taken at the release-gate sweep** — the authoritative
measurement, which is why it supersedes every relayed figure above — with the descriptions
(492 / 473) stable across every report. This is the standing rule working as designed: the gate
re-measures before the `plugin.json` bump, and it caught one figure that had moved after its seat
reported. The new
`/mochiko:architecture` command carries **no per-primitive budget**, like every other command
(both waves excluded commands by user ruling) — the hard cap and the justified-exemption path are
its only bar.

`testing-gap-finding` (new skill, v0.79.0, QA gap-finding build) likewise unbudgeted —
hard-cap-only (desc ≤1,536); measured at birth at body 10,559 / desc 709. The 709
description is the library's largest (prior high `patterns-model-tiering` 643) — ruled
HOLDS at the v0.79.0 audit (V2/F5): ownership set + scope carve + the six-item fence
inclusion list + the `testing-end-user`/`patterns-vertical-tdd` boundary, all
routing-load-bearing, no playbook prose. Body trail on the audit record: 9,938 at the V1
grade → +219 advisory alignments → +270 F2 fence guard → +132 final alignments + reflow. A
body budget may seed from a future ruled cut or benchmark, never invented. **Re-measured 10,929
at v0.81.0** (+370 — the D12 runtime-NFR re-point to store concern rows plus its spine-exclusion
fence guard); still unbudgeted, still hard-cap-only, description unchanged at 709.

## Unbudgeted primitives

The `mochiko` router skill (body deliberately unbudgeted — its body IS the router index, the
discoverability surface; **38,884 chars at v0.81.0**, grown by the wave's Architecture desk
cluster and its architecture row re-keys; its description is 206 chars) and all **commands** have no
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
