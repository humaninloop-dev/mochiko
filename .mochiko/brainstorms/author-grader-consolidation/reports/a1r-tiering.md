---
report: review
round: 2
---

VALIDATE: `patterns-model-tiering` pair — `plugins/mochiko/skills/patterns-model-tiering/SKILL.md`
+ its `mochiko-cli` render (preamble + the six sections its preamble lists).

Checklist run: the skill-pair block, items 1–12, of `.claude/rules/mochiko/primitive-edits.md`
(read whole, this round, first).

Evidence read: `.claude/rules/mochiko/primitive-edits.md` ·
`plugins/mochiko/skills/patterns-model-tiering/SKILL.md` · the seven rendered blocks
(`preamble` · `sec.trigger` · `sec.scope` · `sec.discipline` · `sec.inputs` · `sec.disclosure` ·
`sec.reserved`), each rendered first-hand by this seat · `plugins/mochiko/migrations/0008-gate-form.yaml`
· `.mochiko/memory/primitive-cost-budgets.md` (row + canonical snippet) ·
`.mochiko/brainstorms/author-grader-consolidation/record.md` (D7, F12, wave-1 build item 1) ·
`DECISIONS.md` 2026-09-19 row · `evals/contract/expected-skills.json` ·
`.mochiko/strips/patterns-model-tiering.md`. No seat's plan or report was opened.

Pre-pass (run first-hand by this seat, quoted):

```
mochiko-cli migrate validate --report --plugin-root plugins/mochiko
mochiko-cli migrate validate · 0 rejecting · 105 advisory
```

Advisory line for this primitive, noted, never gating:

```
budget · skill/patterns-model-tiering · - · 22 rules · 9385 resolved characters of rule text
```

Char-budget measurement, the ledger's canonical python3 snippet over the body and the
`description:` value, plus characters of the parsed value of each rendered block:

```
body            3,112
description     1,208   (cap 1,536)
render         15,246   = preamble 1,737 + trigger 2,592 + scope 1,248
                        + discipline 4,907 + inputs 1,162 + disclosure 2,860 + reserved 740
payload        18,358   = body 3,112 + render 15,246
budget         10,852
overage        +7,506
```

A note on the binary: the `mochiko-cli` on PATH changed from 0.1.0 to 0.2.0 part-way through this
audit. Every figure above was re-taken under 0.2.0 and is identical to the 0.1.0 figures; the
version string is the same character length on both, so no measurement depends on which ran.

1. **Load-first section — PASS.** `SKILL.md` line 24 carries `## Rules — delivered by mochiko-cli`;
   lines 38–44 are the seven `!` lines, `preamble` first. Lines 26–36 carry the delivery contract
   (version-triple line, end line, halt-and-surface on anything else, never Read a schema file).
   No raw schema or common-file Read is demanded anywhere in the body.
2. **Section enumeration — PASS.** The six `--section` arguments behind the `preamble` line are
   `trigger` · `scope` · `discipline` · `inputs` · `disclosure` · `reserved` — the patterns family
   set exactly, in the preamble's own printed order. Set-wise match confirmed against the
   preamble's `sections` list. All seven renders exited 0, so every `sec.*` token in the `.md`
   resolves to a live node; no `sec.*` token appears outside the `!` lines. `section-set` raised
   nothing in the pre-pass.
3. **Floor-count pin + read-back — PASS.** The render prints `- class: floor · 8 rules` under
   `pins`, with the `floors:` index line listing eight ids beneath it. `SKILL.md` lines 46–48
   oblige stating back "the floor count the preamble's `class: floor` pin prints and the ids its
   `floors:` line lists" — it cites both and carries no hard-coded number. The frozen set in
   `evals/contract/expected-skills.json` is re-keyed to `floor_pin: 8` with
   `patterns-model-tiering.persona-less-grader-pin` among its eight `floor_ids`, so the
   `converted-shape` cross-check matches the render rather than a stale 7.
4. **Floor survival — PASS.** `0008-gate-form.yaml` carries exactly one change against
   `skill/patterns-model-tiering`, and it is a `mint-rule`. I replayed the log through 0007 into a
   scratch directory and diffed all seven blocks against the current render: the only differences
   are the new rule's block in `sec.discipline`, the pin 7 → 8, the new id in the `floors:` line,
   and that section's rule count 8 → 9. No floor left, no text changed. `protected-exit` and
   `anchor-format` raised nothing.
5. **ID continuity — PASS.** Nothing vanished, so no tombstone is owed; the mint is a new id, not a
   reword, split, or merge. `tombstone-integrity`, `mint-once`, `id-duplicate` and
   `cite-unresolved` raised nothing in the pre-pass. No surviving rule text references a
   tombstoned or re-homed node.
6. **`extends:` conformance — PASS.** The minted rule declares no `extends:`; it carries its own
   `text`, `class: floor`, `kind: bound`, `labels: [boundary]`. The patterns family has no common
   file (`"common": null` in the frozen row), so no cross-family binding is possible here.
   `extends-unresolved`, `extends-cross-family` and `extends-class-local` raised nothing.
7. **`description:` untouched — PASS.** `SKILL.md` is byte-unchanged this wave — `git status`
   shows it neither modified nor untracked, its last commit being 3e43ef4 at v0.110.0. The value
   measures 1,208 characters of the parsed value, under the 1,536 delivery cap, and it never
   moved to schema.
8. **Budget = delivered-at-invoke payload — PASS, the overage argument holds.** Payload 18,358
   against a 10,852 budget, an overage of +7,506, which is the figure the ledger row carries at
   [v0.111.0]; body 3,112, description 1,208 and render 15,246 each match the row exactly. The
   row's delta claim is independently confirmed: replaying the log through 0007 renders 14,752,
   the row's stated [v0.110.0] render, so the whole growth this wave is render, +494, and none of
   it is body or frontmatter. The +494 traces to one cause — +445 for the minted rule's block in
   `sec.discipline` and +49 in the preamble, which is that same mint showing up as the pin, the
   `floors:` id and the section count. The justification holds on both limbs. Genuine new
   obligation: the rule binds a persona-less grader or reviewer spawn to an explicit `model:`
   alias, which record D7 mints to close F12, whose finding is precisely that nothing bound such
   a spawn before — `orchestrator-model-selection` D2's no-`inherit` rule binds persona *files*
   only. Never restored prose: `persona-less` appears nowhere in
   `.mochiko/strips/patterns-model-tiering.md`, nor does any phrase of the new text; the
   block-by-block diff shows nothing else returning. The nearest-sounding strip content,
   "the override is the pin", is a surviving rule under its own id, not a restoration.
9. **Pointer resolution — PASS.** `pointer resolution: 84 checked against plugins/mochiko` with
   `pointer-unresolved` raising nothing. The one pointer in this skill,
   `mochiko:patterns-sound-loop` on `class-key-session-tier`, renders intact.
10. **Deterministic pre-pass — PASS.** Run by this seat, both before the read and again under the
    0.2.0 binary; `0 rejecting · 105 advisory` both times, quoted above. Nothing the output
    asserts was re-derived by judgment.
11. **Skill-grammar conformance — PASS.** The minted rule's `kind: bound` is in the eight-kind
    skill set. No `kind: fail` and no `enforces:` anywhere in this schema; no `when:` on the new
    rule and so nothing to resolve against `conditions:`; no `moments:` block. `skill-grammar`,
    `rule-kind-unknown`, `when-undeclared` and `moment-declaration` raised nothing.
12. **Provenance anchors — PASS.** The mint carries `anchor: 2026-09-19 author-grader-consolidation D7`
    in the migration that writes it, and `anchor-format` raised nothing. The ruling covers the
    content: record D7 states the obligation in the same terms the rule text carries, wave-1 build
    item 1 names `patterns-model-tiering` as its home, and the `DECISIONS.md` 2026-09-19 row
    carries "explicit `model:` alias on every persona-less spawn". Nothing protected exits here,
    so no supersession-by-ruling is owed.

VERDICT: **PASS**

Issues requiring fix: none.

## Notes of note

The round-1 blocking item is closed on both limbs. The ledger row is restamped at [v0.111.0] and
its figures reproduce exactly under first-hand measurement, and the brief named the overage with
a justification that survives the genuine-new-obligation and never-restored-prose tests.

The `mochiko-cli` binary on PATH moved from 0.1.0 to 0.2.0 mid-audit. Figures were re-taken under
0.2.0 and are unchanged. A wave that measures while the binary is being reinstalled should expect
this and re-measure rather than trust the first pass.

audit: patterns-model-tiering pair · validator · opus · 8 files · 2 rounds · 0 blocking
