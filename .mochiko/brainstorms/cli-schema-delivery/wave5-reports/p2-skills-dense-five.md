# Wave 5 — P2 family report: the dense five (30 of 30)

All five members are re-pointed to `mochiko-cli` delivery, with ten strip entries recorded. Every
structural check green. This closes P2's conversion work: all thirty schema-bearing skills now fire
from the migration log. Pre-edit ref for every strip Content field: `7d098b9`.

Members: `analysis-codebase` · `brownfield-integration` · `executing-tdd-cycle` ·
`testing-end-user` · `testing-gap-finding`.

## Checks

**Diff scope.** Asserted by script against `7d098b9`: outside-Rules content byte-identical on all
five, frontmatter delta exactly `allowed-tools: Bash(mochiko-cli *)`.

**Delivery.** 7/7 blocks render for every member with head and end lines. Each preamble's
`class: floor` pin equals its `floors:` id count: 3 · 6 · 10 · 7 · 9. These five reuse the review
six-set with explicit empty markers, per the small-families door ruling, and the render prints a
`note:` on each empty section giving its reason.

**D13 checker.** The library sweep now reports 60 findings across all 30 converted skills — one
missing-pin and one missing-heading each, both conversion-expected and superseded by this wave.
Zero findings of any other class library-wide.

**Strip Content.** Machine-compared against `git show 7d098b9:` for all five.

## Budget — the D10.6 re-keyed payload

| skill | body | render | payload | old budget (body + schema) |
|---|---|---|---|---|
| analysis-codebase | 4,688 | 9,088 | 13,776 | 12,098 |
| brownfield-integration | 4,831 | 8,023 | 12,854 | 10,577 |
| executing-tdd-cycle | 7,340 | 12,723 | 20,063 | 18,951 |
| testing-end-user | 9,563 | 12,147 | 21,710 | 20,791 |
| testing-gap-finding | 5,954 | 14,023 | 19,977 | 19,382 |

Family total 88,380 against the record's F3 baseline of 81,799 — **8.0 % over**, the pre-stated
overage for this family. Like patterns, these five ship no common file, so the baseline counts only
body plus own schema while the render adds the fixed per-block overhead.

## Member-specific content

`testing-gap-finding` named its own conditions inline — run scope, depth, and mutation-tool
presence. The render's preamble prints the whole `conditions` block with each value and its
resolution note, which is strictly more, and its strip entry records the substitution. No member of
this family carried a body-scope clause or an obligated reference read.

## Wave totals — all four families

| family | payload | F3 baseline | delta |
|---|---|---|---|
| review | 116,609 | 119,895 | −2.7 % |
| authoring | 147,304 | 150,576 | −2.2 % |
| patterns | 112,701 | 95,858 | +17.6 % |
| dense five | 88,380 | 81,799 | +8.0 % |
| **all thirty** | **464,994** | **448,128** | **+3.8 %** |

The two families that bind a common file come in under baseline, because the render resolves every
`extends:` stub and their common-file bytes leave the payload. The two that ship no common file go
over, by the fixed render overhead plus body growth. Criterion (2) is read per family as ruled at
the wave open, and both overages were pre-stated there as the eyes-open trade for deterministic
delivery.

## Shared files landed

`.claude/rules/mochiko/primitive-edits.md` carries all five converted-skill clauses (criteria 1, 2,
3, 6, 8). `README.md` carries the one amended sentence. `.mochiko/memory/primitive-cost-budgets.md`
carries the `[v0.106.0]` re-seed: all thirty rows re-keyed to body + render with prior figures kept
as history, the two family common-file rows untouched, and a paragraph recording that the budgeted
quantity itself re-keys under D10 clause 6.
