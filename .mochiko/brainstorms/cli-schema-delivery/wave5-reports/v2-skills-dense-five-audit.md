# V2 audit — wave 5, the dense five, plus the two shared items

**Overall verdict: PASS**, for the five pairs and for both shared items. Graded against
`.claude/rules/mochiko/primitive-edits.md` as it now stands, from the files themselves; P2's report
was read only as a claim to be checked. Pre-edit reference: `7d098b9`. Binary built from this
worktree, 0.1.0, grammar 1, plugin 0.105.0.

## Per-pair verdicts

- `analysis-codebase` — **PASS**
- `brownfield-integration` — **PASS**
- `executing-tdd-cycle` — **PASS**
- `testing-end-user` — **PASS**
- `testing-gap-finding` — **PASS**

## 1. Frontmatter, diff scope, and the Rules section

Scripted per pair against `git show 7d098b9:<path>`: the frontmatter delta is the single added line
`allowed-tools: Bash(mochiko-cli *)` for all five, so `name` and `description` are byte-identical
and criterion 7 holds. Excising the block from `## Rules` to the next `## ` heading, all five
bodies are byte-identical outside the Rules section.

All five heads read exactly `## Rules — delivered by mochiko-cli`. The halt paragraph matches the
plan's blockquote with the name substituted, machine-compared after whitespace normalisation. Seven
`!` lines each, correctly prefixed and suffixed. The read-back sentence is verbatim in all five,
and no member carries a block after it. The arguments are `preamble` plus the **review six-set** —
`independence · scope · inputs · verdict · output · reserved` — reused under the small-families
door ruling, in each preamble render's printed order, which equals each schema's declared order. No
residual mention of `schema.yaml`, a common file, or "load the schema" survives.

## 2. Delivery

All 35 `!` commands were run by hand with `target/release` on `PATH`. Every one of the 35 blocks
opens with its version-triple head line and closes with its matching end line; zero failures, zero
empty blocks. Each preamble's floor pin equals its `floors:` id count, and the sequence is exactly
**3 · 6 · 10 · 7 · 9** in the brief's order.

## 3. Empty markers and inline conditions

Confirmed by rendering, not by reading the claim. Four of the five carry exactly one empty section
each — `independence` in `analysis-codebase`, `brownfield-integration`, and `testing-end-user`,
and `verdict` in `executing-tdd-cycle` — and every one renders at 0 rules with a `note:` line
giving a census-grounded reason, for example "Deliberately empty — this executor's outcome grammar
is the report field contract carried in `executing-tdd-cycle.sec.output`; the clearing verdict is
the lead's". `testing-gap-finding` has no empty section, which is why P2's "explicit empty markers"
line covers four members rather than five.

`testing-gap-finding`'s inline conditions list is covered: its preamble prints all three dimensions
with values, resolution mode, and an explanatory note apiece — `run_scope`, `depth`, and
`mutation_tool` — which is strictly more than the sentence carried. Its strip entry records the
substitution.

## 4. Strips

Each of the five strip files carries exactly two `[v0.106.0]` entries as the top two in the file
(next stamp `v0.103.0`), with every field the README demands of a supersession-by-ruling entry.
Content was machine-verified: entry one's fenced block dedents to a string equal to the whole
pre-edit Rules section from `7d098b9` for all five, and entry two's is a verbatim substring of that
same section for all five.

## 5. Criterion 9 — the deterministic checker

Per pair, each of the five reports **exactly two findings**, both conversion-expected: the missing
`## Rules — load the schema first` heading and the missing hand pin. Warnings match the pre-edit
baseline exactly, reconstructed from `7d098b9`'s `SKILL.md` plus the current schema — 0 · 0 · 0 ·
0 · 1 in the brief's order.

## 6. Criterion 8 and IDs

I recomputed all five: body characters plus the seven rendered blocks' characters, hook lines
excluded. Every one reproduces P2's table, and the family total of **88,380** matches the report
and the lead's figure. `git diff 7d098b9 --stat` on the skill schemas is empty, so no id vanished
and no tombstone is owed. Every `<skill>.sec.*` token in these bodies resolves to a live node,
including the five that sit outside the `!` lines (see the observations).

## 7. Shared item — the `[v0.106.0]` budget block

**PASS.** I recomputed **all thirty rows**, not the five the brief asked for, by re-measuring each
skill's body and re-running its seven `!` commands. Every row's total, body figure, and render
figure matches my measurement exactly; no row is missing the re-seed and no row disagrees. There
are exactly thirty schema-bearing skills and the four families account for all of them.

The block states the quantity re-key in its own words, citing D10 clause 6 and the 2026-09-04
`DECISIONS.md` row: the payload is the body plus the seven rendered blocks, measured after the
`floors:` line landed, with the dependency-hook lines excluded as the harness's output and no +25 %
headroom. Every row keeps its prior figure and prior payload split as history, which I verified
structurally across all thirty.

The two family common rows are untouched — the diff adds and removes no table row for either — and
the block's stated sizes are exact: `skill-review-common.yaml` at 1,627 characters and
`skill-authoring-common.yaml` at 1,285. The four family aggregates the block states all reproduce:

| family | measured | block | F3 baseline | delta |
|---|---|---|---|---|
| review | 116,609 | 116,609 | 119,895 | −2.7 % |
| authoring | 147,304 | 147,304 | 150,576 | −2.2 % |
| patterns | 112,701 | 112,701 | 95,858 | +17.6 % |
| dense five | 88,380 | 88,380 | 81,799 | +8.0 % |
| all thirty | 464,994 | — | 448,128 | +3.8 % |

## 8. Shared item — the whole-library D13 sweep

**PASS, reproduced.** `uv run scripts/check-skill-schema.py` with no `--skill` reports exactly
**60 findings**: 30 of the missing-canonical-heading class and 30 of the missing-floor-pin class,
one of each per converted skill. Zero findings of any other class anywhere in the library. The
sweep's own tail line reports four common blocks, all bound by at least one stub.

## 9. Report honesty

P2's dense-five report survives checking, including its wave-totals table and its shared-files
paragraph. Every figure in both reproduces.

## Non-blocking observations

- **Three dense-five bodies still tell the reader that rules live "in the schema".**
  `testing-end-user` says the classification criteria "live in the schema's
  `testing-end-user.sec.verdict` section"; `executing-tdd-cycle` and `analysis-codebase` carry three
  more of the same shape. The ids all resolve, so criterion 2 holds, but the wording points at a
  file the converted skill is now forbidden to read, while the halt paragraph two screens earlier
  says "Never Read a schema file instead". P2 could not have fixed this without breaking the
  byte-identical-outside-Rules rule the plan imposed, so it is correctly out of scope here. It wants
  a follow-up item. No other family has a `.sec.` token outside its `!` lines.
- **The preamble `legend` still prints `kind: … fail …` and two `enforces:` lines** on skill
  renders, though skill-pair criterion 11 makes both illegal in a skill schema. Raised in all four
  family audits; P1's surface, and still owed a ruling before the wave lands.
- **Two member sentences were rewritten rather than carried byte-identically** —
  `review-feasibility`'s lens read and `authoring-requirements`' script-pointer line. Both preserve
  their substance and both old texts are in the strips, but plan §3 asked for byte-identical
  carries. Worth a line in the record so the next auditor is not surprised.

**Fix list: none.** Nothing blocks this family or either shared item.
