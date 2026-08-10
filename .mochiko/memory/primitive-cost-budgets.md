# Primitive cost budgets — the D7 char-budget ledger

Provenance: `validator-scope-and-verbosity` D7 (record:
`.mochiko/brainstorms/validator-scope-and-verbosity/record.md`), the guardrails-vs-detail
benchmark verdict (`DECISIONS.md` 2026-08-10 benchmark-verdict row;
`.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`), and the Wave 1 build
(v0.63.0). Cited by the char-budget pre-assert in `.claude/rules/mochiko/primitive-edits.md`.

**The rule.** A budget is the measured winning-variant character count **+25% headroom**
(rounded up). Budgets are always measured, never invented: a new budget enters this ledger only
from a benchmark-measured winner or a ruled editorial cut shipped through the edit ceremony
(Wave 2 seeds its budgets from its own cut results the same way). All counts are **characters
of the parsed value — never `wc -c` bytes** (byte counts over-reject unicode-bearing text that
is under the char cap; a v1 census defect was exactly this).

**Classes measured:** skill body (content after the closing frontmatter `---`) · skill
frontmatter `description:` value · agent frontmatter `description:` value. `references/` files
are exempt (on-demand data, never auto-loaded).

## Skill bodies

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

The v0.63.0 floor-line additions (analysis-iterative, review-governance-intent,
validation-constitution, review-specifications) ride inside these budgets — the +25% headroom
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

## Agent descriptions

| agent | winner chars | budget (+25%) |
|---|---|---|
| principal-architect | 738 | 923 |
| validator | 269 | 337 |
| devils-advocate | 316 | 395 |
| requirements-analyst | 303 | 379 |
| product-manager | 438 | 548 |
| product-engineer | 392 | 490 |

## Untested primitives

The remaining 17 skills, 4 agents, and all commands have **no per-primitive budget** until a
measured winner exists (Wave 2, or a future benchmark). They are covered only by the standing
hard cap (skill `description:` ≤ 1,536) plus the justified-exemption path. This is deliberate:
it prevents the gate from force-failing the four near-cap untested description skills
(`patterns-system-design` 1,514 · `review-feasibility` 1,513 · `authoring-architecture` 1,511 ·
`review-brainstorm` 1,506 — all outside the tested setup+specify substrate, M1 watch) before
they are measured, while the hard cap still catches over-cap growth everywhere.

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
