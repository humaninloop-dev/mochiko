# Wave 5 — P2 plan: thirty skill re-points

Phase-1 plan only; no repo file edited. Figures are **characters** (the budget ledger's unit) on binary 0.1.0 against
plugin 0.105.0, before P1's `floors:` line; my render byte sums reproduce the lead's §0 table to within 9 bytes per
family, so the two measurements agree.

## 1. Inventory — all 30 load-first blocks

Every block was found. For all 30 the ids enumerated in the block equal the preamble render's `sections` list, and the
hand-pinned floor count equals the render's `class: floor` pin — no defects. The family common file is named in all
eight review and all eight authoring members, in none of patterns or the dense five. The 30-row table is collapsed to
hold the line cap; every skill and its pin appears below.

**Nothing member-specific (22, pin in parens).** review — review-brainstorm (9) · review-code-minimalism (3) ·
review-governance-intent (16). authoring — authoring-architecture-store (9) · authoring-constitution (12) ·
authoring-epic (10) · authoring-technical-requirements (8) · authoring-user-stories (4). patterns —
patterns-adopt-first (7) · patterns-architecture-shelves (5) · patterns-code-minimalism (3) · patterns-map-minimalism
(3) · patterns-model-tiering (4) · patterns-plan-minimalism (2) · patterns-sound-loop (6). dense five —
analysis-codebase (3) · brownfield-integration (6) · executing-tdd-cycle (10) · testing-end-user (7).

| skill | family | pin | member-specific content |
|---|---|---|---|
| review-feasibility | review | 9 | **third obligated read** — `references/FEASIBILITY-LENS.md`, sequenced schema, then common, then lens |
| review-plan-artifacts (11) · review-specifications (8) · review-sufficiency (8) · validation-constitution (14) | review | — | per-section glosses; "body carries identity and procedure/teaching only" |
| authoring-feature-map (16) · authoring-prototype (4) | authoring | — | same, plus two "empty by design" markers in `authoring-prototype` |
| authoring-requirements | authoring | 4 | glosses incl. two "empty by design"; body-scope clause; pointer says "file's or **script's** content" |
| patterns-transport-floor | patterns | 11 | names its two lanes inline (`messaging`, `shared_write_surface`) |
| patterns-vertical-tdd | patterns | 5 | names its condition inline (`new_end_to_end_path`) |
| testing-gap-finding | dense five | 9 | names its conditions inline (run scope, depth, mutation-tool presence) |

## 2. What the generic shape drops, and what covers it

Checked by rendering all 210 blocks. **Covered, drops safely:** the glosses and "empty by design" markers (the render's
`sections` line prints a richer title per section; an empty section renders a `note:` giving the reason); the inline
condition lists (the preamble prints the whole `conditions` block); the `extends:` and `${var}` sentences (no rendered
section carries an unresolved `${` or `extends:`). **Carried forward:** `review-feasibility`'s lens read, as its own
sentence after the read-back, and `authoring-requirements`' pointer wording, whose target is a script where the halt
paragraph says "file's or skill's procedure" — kept unless the lead rules the generic wording covers a script.
**Dropped by choice:** the body-scope clause in seven members states a scope, not an obligation; each drop is recorded
in that member's strip entry.

## 3. Blocker — the char budget fails on all 30 as the plan is written

Plan §3 says "the body shrinks, so under budget". It does not. The new block runs 2,012–2,282 chars against old blocks
of 1,082–1,891, so every body grows, by +192 (`review-plan-artifacts`) to +1,079 (`patterns-architecture-shelves`).
Budgets re-seeded at conversion with **no headroom**, so if the budgeted quantity stays "body + own `schema.yaml`" all
30 are over by exactly their body delta; re-keying it to the post-conversion payload (body + render) still leaves 27 of
30 over. A re-seed ruling is owed before any audit can clear its char pre-assert, and `primitive-edits.md` criterion
8's definition of the budgeted quantity needs a converted-skill branch — a **third** rules-file clause the plan's
two-clause scope does not carry. Proposal: re-seed all 30 to the measured post-conversion delivered-at-invoke payload,
body + render, no headroom (the third seeding path's fifth use), and amend criterion 8 to say so; the ledger is not in
my §1 file set, so I draft the clause and figures and hand them over unless the lead assigns it to me. Per family under
that definition, against the F3 baseline (body + own schema + family common per fire): review 113,051 vs 119,895
(−5.7 %) · authoring 143,924 vs 150,576 (−4.4 %) · patterns 110,503 vs 95,858 (+15.3 %) · dense five 86,794 vs 81,799
(+6.1 %) — both overages the pre-stated ones. Figures move once P1's line lands; I re-measure per family.

## 4. Per-family conversion procedure

One family per unit in D9's order, `review-brainstorm` first and alone as the referent for V2. Per skill: add
`allowed-tools: Bash(mochiko-cli *)` to the frontmatter, `name` and `description` byte-untouched; replace the whole
`## Rules — load the schema first` section with `## Rules — delivered by mochiko-cli` carrying the halt paragraph
verbatim from plan §3 (name substituted at its two occurrences), then seven `!` lines — `preamble`, then the six family
section ids in the preamble's printed order — then the read-back sentence verbatim, then any preserved member sentence.
Nothing outside that section changes, verified by a diff restricted to the frontmatter line and the section.

## 5. Strip-entry shape

Two `[v0.106.0]` supersession-by-ruling entries per skill, appended newest-first to `.mochiko/strips/<skill>.md` under
one wave-context comment per file citing `cli-schema-delivery` D3 as amended (skill-side form), D7, D9, the wave-open
rulings in `wave5-plan.md`, the `DECISIONS.md` row, and the pre-edit ref `git show <pre-wave
HEAD>:plugins/mochiko/skills/<skill>/SKILL.md`. Entry one supersedes the load-first block: `Disposition: superseded →
## Rules — delivered by mochiko-cli`, `Tier failed: n/a — supersession by ruling`, Content the old section verbatim
from `git show`, `Kept deliberately` naming what survives in the render plus any preserved member sentence, `Consumers
assessed` recording that the family common files are unchanged and nothing shared leaves. Entry two supersedes the
hand-pinned floor count the same way, citing the CLI-printed pin and the `floors:` index, and naming
`primitive-edits.md` criterion 3 as the consumer amended this wave.

## 6. The two rules-file clauses, verbatim

Appended to the skill-pair block's criterion 1:

> **On a converted skill** — one whose rules `mochiko-cli` renders at fire (`cli-schema-delivery` D3, the skill-side
> form) — the section instead reads `## Rules — delivered by mochiko-cli`, its seven `!` lines are the enumeration, and
> no raw Read of the schema or of the family common file is demanded: the render resolves every `extends:` stub and
> every `${var}` before the model sees it.

Appended to criterion 3:

> **On a converted skill** the hand-pinned count is gone by ruling: the pin is the `- class: floor · N rules` line the
> render prints under `pins` in the preamble block together with the `floors:` index line beneath it, and the read-back
> sentence cites both — a hard-coded number there is the defect, not its absence.

Two phrases go dangling on a converted skill and may draw a V2 finding: criterion 2's "the load-first block" (readable
through criterion 1's new sentence) and criterion 6's closing common-file co-Read (discharged by the render, as
criterion 11 says on the command side). No edits drafted; say the word and I will.

## 7. README and per-family checks

README, one sentence at line 18: "every command's rules are rendered at fire by the binary, and a command **halts**
when it is missing" becomes "every command's and skill's rules are rendered at fire by the binary, and a command or
skill **halts** when it is missing". Nothing else in the file. Checks per family, `target/release` on `PATH`: every `!`
line renders with head and end lines; each preamble's `class: floor` pin equals the count of ids on its `floors:` line
(after the lead signals P1 closed); `uv run scripts/check-skill-schema.py --skill <name>` cited per pair with only
conversion-expected findings, plus a full sweep at family close; char-budget figures per skill under whichever
definition the lead rules in §3; a diff confirming frontmatter and the Rules section are the only changes; the strip
Content field machine-compared against `git show`.

**Risks.** `allowed-tools` is a grant on commands and evidently additive there, but its restrictiveness on skills is
unprobed (record F9.8, F12a). If it restricts, every converted skill loses Read and Grep. The referent lands alone
partly to catch this; I confirm the skill still reads a file before converting the other seven. The inventory ran as a
script, not a haiku explorer — the extraction is mechanical and exact — and the interpretive read of the 30 blocks
stayed on the session tier per `patterns-model-tiering`.
