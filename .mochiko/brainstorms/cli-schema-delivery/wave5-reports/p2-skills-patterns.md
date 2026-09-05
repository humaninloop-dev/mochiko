# Wave 5 — P2 family report: patterns (25 of 30)

All nine patterns members are re-pointed to `mochiko-cli` delivery, with eighteen strip entries
recorded. Every structural check green. Pre-edit ref for every strip Content field: `7d098b9`.

Members: `patterns-adopt-first` · `patterns-architecture-shelves` · `patterns-code-minimalism` ·
`patterns-map-minimalism` · `patterns-model-tiering` · `patterns-plan-minimalism` ·
`patterns-sound-loop` · `patterns-transport-floor` · `patterns-vertical-tdd`.

## Checks

**Diff scope.** Asserted by script against `7d098b9`: for all nine, everything outside the Rules
section is byte-identical, and the frontmatter delta is exactly the one line
`allowed-tools: Bash(mochiko-cli *)`.

**Delivery.** 7/7 blocks render for every member, each with its head and end lines. Each preamble's
`class: floor` pin equals the count of ids on its `floors:` line: 7 · 5 · 3 · 3 · 4 · 2 · 6 · 11 ·
5. The six ids are the patterns set — `trigger` · `scope` · `discipline` · `inputs` ·
`disclosure` · `reserved` — in the preamble's printed order. This family ships no common file, so
no stub resolution is at stake and each member's strip records that nothing shared leaves.

**D13 checker.** The library sweep reports 50 findings across the 25 converted skills — one
missing-pin and one missing-heading each, both conversion-expected. Zero findings of any other
class library-wide.

**Strip Content.** Machine-compared against `git show 7d098b9:` for all nine.

## Budget — the D10.6 re-keyed payload

| skill | body | render | payload | old budget (body + schema) |
|---|---|---|---|---|
| patterns-adopt-first | 3,061 | 11,181 | 14,242 | 12,910 |
| patterns-architecture-shelves | 4,003 | 10,490 | 14,493 | 12,361 |
| patterns-code-minimalism | 3,259 | 7,059 | 10,318 | 8,024 |
| patterns-map-minimalism | 3,288 | 7,957 | 11,245 | 9,362 |
| patterns-model-tiering | 2,633 | 8,219 | 10,852 | 8,793 |
| patterns-plan-minimalism | 3,834 | 6,990 | 10,824 | 8,594 |
| patterns-sound-loop | 2,812 | 9,576 | 12,388 | 10,933 |
| patterns-transport-floor | 3,114 | 9,450 | 12,564 | 10,556 |
| patterns-vertical-tdd | 5,896 | 9,879 | 15,775 | 14,325 |

Family total 112,701 against the record's F3 patterns baseline of 95,858 — **17.6 % over**. This is
the overage the wave open pre-stated for this family, and the reason is structural: patterns ships
no common file, so its baseline counts only body plus own schema, while the render adds the fixed
per-block overhead of a version-triple head line and an end line seven times over, plus the
preamble's `conditions`, `pins`, `floors:`, `legend`, and `sections` blocks. The trade was accepted
at the wave open for deterministic delivery. The lead's §0 projection for this family was +10.4 %
measured on the render alone; the gap to +17.6 % is the body growth, which the §0 figure did not
carry.

## Member-specific content

`patterns-transport-floor` named its two lanes inline (`messaging`, `shared_write_surface`) and
`patterns-vertical-tdd` named its single condition (`new_end_to_end_path`). Both are covered by the
render, whose preamble prints the whole `conditions` block with each value and its resolution note,
which is strictly more than the sentence carried. Each strip entry records that explicitly rather
than letting the sentence vanish unremarked. No member of this family carried a body-scope clause
or an obligated reference read, so nothing else needed carrying forward.
