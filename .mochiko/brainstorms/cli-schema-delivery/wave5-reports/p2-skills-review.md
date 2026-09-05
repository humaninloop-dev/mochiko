# Wave 5 — P2 family report: review (8 of 30)

All eight review members are re-pointed to `mochiko-cli` delivery, with sixteen strip entries
recorded. Every check green. Pre-edit ref for every strip Content field: `7d098b9`.

Members: `review-brainstorm` (the referent) · `review-code-minimalism` · `review-feasibility` ·
`review-governance-intent` · `review-plan-artifacts` · `review-specifications` ·
`review-sufficiency` · `validation-constitution`.

## Checks

**Diff scope.** Asserted by script against `7d098b9`, not eyeballed: for all eight, everything
outside the Rules section is byte-identical, and the frontmatter delta is exactly the one line
`allowed-tools: Bash(mochiko-cli *)`. `name` and `description` untouched.

**Delivery.** 7/7 blocks render for every member, each opening with its version-triple head line
and closing with its end line. Each preamble's `class: floor` pin equals the count of ids on its
`floors:` line: 9 · 3 · 9 · 16 · 11 · 8 · 8 · 14.

**D13 checker.** A full-library sweep reports exactly sixteen findings, all conversion-expected:
one "no line pinning the `class: floor` count" and one "canonical heading absent" per converted
skill, both superseded by this wave's ruling. Zero findings of any other class anywhere in the
library. The referent's three warnings are pre-existing schema-condition coverage notes on
`schema.yaml`, which this wave does not touch.

**Strip Content.** Machine-compared against `git show 7d098b9:` — for all eight, entry one's
Content is the verbatim old Rules section and entry two's is the verbatim pin sentences.

## Budget — the D10.6 re-keyed payload

Body chars plus the seven rendered blocks' chars, hook lines excluded, measured after the
`floors:` line landed.

| skill | body | render | payload | old budget (body + schema) |
|---|---|---|---|---|
| review-brainstorm | 2,833 | 9,991 | 12,824 | 11,470 |
| review-code-minimalism | 3,711 | 6,953 | 10,664 | 8,449 |
| review-feasibility | 2,721 | 9,499 | 12,220 | 10,572 |
| review-governance-intent | 3,160 | 13,114 | 16,274 | 14,663 |
| review-plan-artifacts | 3,363 | 14,650 | 18,013 | 17,890 |
| review-specifications | 3,441 | 12,733 | 16,174 | 15,600 |
| review-sufficiency | 3,262 | 12,161 | 15,423 | 14,950 |
| validation-constitution | 3,263 | 11,754 | 15,017 | 13,285 |

Family total 116,609 against the record's F3 review baseline of 119,895 — **2.7 % under**, so
criterion (2) holds for this family at the aggregate, as ruled at the wave open. These eight
figures are the re-seed values for the `[v0.106.0]` block in
`.mochiko/memory/primitive-cost-budgets.md`, which lands once all four families are measured.

## Member-specific content

`review-feasibility` keeps its obligated reference read: a sentence after the read-back reading
"Your first action after the read-back, before any hunting: **Read `references/FEASIBILITY-LENS.md`
(this skill's own directory) raw, in full.**" Ordering intact — the lens still precedes the hunt.

Four members (`review-plan-artifacts` · `review-specifications` · `review-sufficiency` ·
`validation-constitution`) carried per-section glosses and a "this body carries identity and
procedure only" clause. The glosses are covered by the render, whose `sections` line prints a
title per section. The body-scope clause is dropped by the approved ruling and named in each
member's strip entry rather than removed silently.

## Referent probe

`review-brainstorm` was converted alone first and probed headless against the real plugin. The
session made a `Read` tool call and returned the file's first line, so the tool grant is additive
rather than restrictive on skills. It also returned all nine floor ids in its read-back, which
exist only in the render, with no schema Read anywhere in the transcript. Full result reported to
the lead separately.
