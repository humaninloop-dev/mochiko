---
report: review
round: 1
seat: A1
form: old (validator, full read)
---

## Pre-pass — run first-hand, shared by all three units

Command 1, run by this seat from the repo root:

```
mochiko-cli migrate validate --report --plugin-root plugins/mochiko
```

Decisive line: `mochiko-cli migrate validate · 0 rejecting · 105 advisory`

Advisory findings naming the three units (none gating):

```
condition-coverage · command/setup · mode · value "amend" is declared but named by no rule's `when:`
condition-coverage · command/setup · seats · value "single" is declared but named by no rule's `when:`
enforces-coverage · command/setup · - · 17 floor/gate rules no fail node enforces: ...
budget · command/setup · - · 43 rules · 9220 resolved characters of rule text
budget · skill/patterns-model-tiering · - · 22 rules · 9385 resolved characters of rule text
budget · skill/validation-primitive-edit · - · 21 rules · 5357 resolved characters of rule text
zero-member-label · command/setup · attempt-economy · registered but carried by no rule in this document
pointer resolution: 84 checked against plugins/mochiko
```

The `enforces-coverage` advisory fires on all six commands, not on this wave's edit.
The `budget` advisory counts rule text only and is never the ledger comparison (criterion 8).

Command 2, the char-budget measurement, the canonical python3 snippet from
`.mochiko/memory/primitive-cost-budgets.md` "How to measure", body and `description:` taken as
characters of the parsed value, render taken as the seven delivered blocks:

```
== patterns-model-tiering
body 3112 · render 15246 · payload 18358 · description 1208
== validation-primitive-edit
body 3449 · render 11573 · payload 15022 · description 730
```

The [suite] limbs were also discharged first-hand, `python3 evals/contract/run.py --host-only`,
a filtered 7-of-89 run and explicitly not a gate run:

```
ok    command `setup`: the `!` lines enumerate every section, in the render's order
ok    command `setup`: the Bash grant the `!` lines need is in the frontmatter
ok    the pre-registered floor set matches the setup render (20 ids)
ok    skill `patterns-model-tiering`: the `!` lines enumerate every section, in the render's order
ok    the pre-registered floor set matches the patterns-model-tiering render (8 ids)
ok    skill `validation-primitive-edit`: the `!` lines enumerate every section, in the render's order
ok    the pre-registered floor set matches the validation-primitive-edit render (11 ids)
contract suite: 7/7 cases passed
```

## Unit 1 — the `setup` pair

VALIDATE: the `setup` command pair — `plugins/mochiko/commands/setup.md` plus its rules as
`mochiko-cli` renders them (preamble and the six `setup.sec.*` sections).

Checklist run: canonical-scaffold criteria 1–11, `.claude/rules/mochiko/primitive-edits.md`.

Evidence read: `plugins/mochiko/commands/setup.md` (whole file, 62 lines) · the seven rendered
blocks from `mochiko-cli rules setup --section <id> --plugin-root plugins/mochiko` ·
`plugins/mochiko/migrations/0008-gate-form.yaml` (whole file) ·
`.mochiko/brainstorms/author-grader-consolidation/record.md` decisions and build surface ·
`DECISIONS.md` row 2026-09-19 · `scripts/similar-rules-allowlist.yaml` diff · the working-tree
diff establishing that `setup.md` itself is unmodified this wave.

Pre-pass: quoted in the shared section above; `0 rejecting · 105 advisory`.

1. Scaffold conformance — PASS. All four required frontmatter keys present with the literal
   `allowed-tools: Bash(mochiko-cli *)`; headings run `# Setup — Governance From Interrogated
   Intent, On Native Surfaces` · `## Identity & Mission` · `## Rules — delivered by mochiko-cli`
   · `## Adaptive Goal Protocol` with Entry, Goal, Not-done last, and no fifth top-level section.
2. Rules-block enumeration — PASS. The `.md`'s six `!` section lines are exactly the six the
   preamble prints (`roles` · `reserved` · `tools` · `ways-of-working` · `boundaries` ·
   `fail-conditions`); the two `setup.sec.tools` tokens and the one `setup.sec.fail-conditions`
   token in the Goal and Not-done prose each resolve to a live node. Suite limb ok, quoted above.
3. FAIL survival — PASS. The Not-done line cites the printed pin, not a number: "their count is
   the `kind: fail` line under `pins` in the preamble block", and obliges halt-and-surface when a
   delivered end-line count disagrees. Preamble prints `- kind: fail · 6 rules`; the
   fail-conditions block ends `6 rules`. `fail-segment`, `tombstone-integrity`, `mint-once` all
   non-rejecting.
4. ID continuity — PASS. Migration 0008 carries only `mint-rule` and `import-document` ops, so no
   `setup.*` id leaves; `id-format`, `id-prefix`, `id-duplicate`, `cite-unresolved`,
   `tombstone-integrity` non-rejecting. The prose reference inside `setup.validate-seat-form`
   ("bounded by setup.gate-loop-bound") resolves to the node minted in the same migration.
5. `class: floor` must-survive — PASS. Nothing left; the wave is a pure addition. The one new
   floor, `setup.gate-loop-bound`, carries `anchor: 2026-09-19 author-grader-consolidation D6`;
   `protected-exit` and `anchor-format` non-rejecting.
6. Substance across the pair — PASS. Plan approval (`setup.plan-approval-producers`) ·
   independence with no self-grading row (`setup.author-grader-default-fail` floor,
   `setup.stress-test-cold-seat`, and the new `setup.validate-seat-form` requiring "a fresh seat
   that authored no surface") · seven reservations in `setup.sec.reserved` · twelve bindings in
   `setup.sec.tools` plus the Entry step · seven floors in `setup.sec.boundaries`. `setup` is not
   DM-chartered, so the `mochiko:patterns-sound-loop` pointer is correctly absent and not demanded.
7. Done-condition class — PASS, run-command branch only. Entry carries the `$ARGUMENTS` gating
   and the mode proposal; Goal states a fixed done condition; Not-done defaults to FAIL and is
   count-pinned. No negotiated per-run goal is present, and none is demanded.
8. Preserved responsibilities — PASS. The migration removes nothing, so no strip entry is owed
   (pure additions ride the decision row). Both minted rules trace to the accepted ruling recorded
   at `DECISIONS.md` 2026-09-19. One fidelity note, non-blocking, in Notes of note.
9. Deterministic pre-pass — PASS. Run by this seat and quoted above.
10. Provenance anchors — PASS. Both mints carry a well-formed anchor (D6 for the bound, D7 for
    the seat form); `anchor-format` non-rejecting, and record D6/D7 cover each.
11. Ontology-grammar conformance — PASS. All thirteen named CLI checks non-rejecting.
    `setup.validate-seat-form` uses `kind: binding` from the nine-kind set; `setup.gate-loop-bound`
    is an `extends: common.gate-loop-bound` stub declaring `class: floor` locally and inheriting
    text and labels only, with no `when:` on a floor. Judgment limb, the extraction bar: the
    common block has a single extender, below the 3+ bar, and that is a ruled single home — record
    D4 ("the loop bound is a single rule in the command common block ... its number written in the
    rule's own text"), disclosed as the exception at OQ1. A recorded ruling governs the bar, and
    the similarity pass reports `clusters: 0 (none)`, so no allowlist edge is owed.

VERDICT: PASS

Issues requiring fix: none.

## Unit 2 — the `patterns-model-tiering` pair

VALIDATE: the `patterns-model-tiering` skill pair —
`plugins/mochiko/skills/patterns-model-tiering/SKILL.md` plus its rendered preamble and six
`patterns-model-tiering.sec.*` sections.

Checklist run: skill-pair criteria 1–12, `.claude/rules/mochiko/primitive-edits.md`.

Evidence read: `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` (whole file, 49 lines) ·
the seven rendered blocks · `plugins/mochiko/migrations/0008-gate-form.yaml` ·
`.mochiko/memory/primitive-cost-budgets.md` row 88 and its working-tree diff ·
`evals/contract/expected-skills.json` diff · the record's D7 and build surface.

Pre-pass: quoted in the shared section above; `0 rejecting · 105 advisory`, and the char-budget
measurement `body 3112 · render 15246 · payload 18358 · description 1208`.

1. Load-first section — PASS. `## Rules — delivered by mochiko-cli` carries the seven `!` lines
   and the delivery-failure halt clause; no raw schema Read is demanded.
2. Section enumeration — PASS. The six `!` section lines are the patterns family six-set
   (`trigger` · `scope` · `discipline` · `inputs` · `disclosure` · `reserved`), set-wise equal to
   the preamble's printed sections and in its order. `section-set` non-rejecting; suite limb ok.
3. Floor-count pin and read-back — PASS. Preamble prints `- class: floor · 8 rules` with the
   `floors:` index beneath it; the body's closing sentence cites both surfaces ("the floor count
   the preamble's `class: floor` pin prints and the ids its `floors:` line lists") and carries no
   hard-coded number. `expected-skills.json` re-keyed `floor_pin` 7 to 8 with the new id, and the
   suite confirms the frozen set matches the render at 8 ids.
4. Floor survival — PASS. Nothing left. The one new floor,
   `patterns-model-tiering.persona-less-grader-pin`, carries `anchor: 2026-09-19
   author-grader-consolidation D7`, and D7 rules exactly that obligation, closing F12.
5. ID continuity — PASS. `tombstone-integrity`, `mint-once`, `id-duplicate`, `cite-unresolved`
   non-rejecting; migration 0008 mints one id here and retires none.
6. `extends:` conformance — PASS. The minted rule declares its own text and binds no stub;
   `extends-unresolved`, `extends-cross-family`, `extends-class-local` non-rejecting.
7. `description:` untouched — PASS. `SKILL.md` is unmodified in the working tree, so the value is
   byte-identical across the wave; measured 1,208 chars, under the 1,536 delivery cap and under
   no per-skill description budget row (this skill has none).
8. Budget — **FAIL**. Measured delivered-at-invoke payload is 18,358 (body 3,112 + render 15,246)
   against a budget of 10,852, a standing overage of +7,506. The ledger records the overage as
   +7,012 at payload 17,864, stamped `[v0.110.0]`; the render grew +494 this wave and the row was
   not restamped, while the sibling `validation-primitive-edit` rows were added at `[v0.111.0]` in
   the same ledger diff. No overage was named in the audit brief, which the pre-assert requires.
9. Pointer resolution — PASS. `pointer resolution: 84 checked against plugins/mochiko`, no
   `pointer-unresolved` finding; the minted rule declares no pointer.
10. Deterministic pre-pass — PASS. Run by this seat and quoted above.
11. Skill-grammar conformance — PASS. `skill-grammar`, `rule-kind-unknown`, `when-undeclared`,
    `moment-declaration` non-rejecting. The minted rule uses `kind: bound` from the eight-kind
    skill set; no `kind: fail`, no `enforces:`, no `moments:` block; the new floor carries no
    `when:` and is always delivered.
12. Provenance anchors — PASS. The anchor rides the migration that writes the rule, well-formed
    and carried by the binary at apply; record D7 covers it.

VERDICT: FAIL

Issues requiring fix:

- **Item 8, budget.** The payload grew +494 past the recorded standing overage and nothing
  records it. Fix: name the overage in the audit brief with the genuine-new-obligation
  justification (the `persona-less-grader-pin` floor minted by record D7, closing F12), and
  restamp the ledger row at `.mochiko/memory/primitive-cost-budgets.md` line 88 to a
  `[v0.111.0]` standing ruled-HOLDS overage of +7,506 at payload 18,358 (body 3,112 + render
  15,246), keeping the `[v0.110.0]` figures as history in the same row, exactly as that row
  already keeps its `[v0.108.0]` and `[v0.106.0]` predecessors. The substantive growth is a
  ruled new floor and the argument holds on its face; what is missing is the naming and the
  stamp the pre-assert requires before a grader may rule it holds.

## Unit 3 — the `validation-primitive-edit` pair

VALIDATE: the `validation-primitive-edit` skill pair, born this wave —
`plugins/mochiko/skills/validation-primitive-edit/SKILL.md` plus its rendered preamble and six
`validation-primitive-edit.sec.*` sections.

Checklist run: skill-pair criteria 1–12, `.claude/rules/mochiko/primitive-edits.md`, plus the
router discoverability check.

Evidence read: `plugins/mochiko/skills/validation-primitive-edit/SKILL.md` (whole file, 51
lines) · the seven rendered blocks · the `import-document` block of
`plugins/mochiko/migrations/0008-gate-form.yaml` · `plugins/mochiko/skills/mochiko/SKILL.md`
lines 35–36 and 121–125 · `.mochiko/memory/primitive-cost-budgets.md` rows and diff ·
`scripts/similar-rules-allowlist.yaml` diff · `evals/contract/expected-skills.json` ·
the record's D2, D3, D6, D7, D9, D11 and build surface item 1.

Pre-pass: quoted in the shared section above; `0 rejecting · 105 advisory`, and the char-budget
measurement `body 3449 · render 11573 · payload 15022 · description 730`.

1. Load-first section — PASS. `## Rules — delivered by mochiko-cli` carries the seven `!` lines,
   the version-triple and end-line proceed condition, and the never-Read-a-schema clause. The
   skill obliges no external reference read, so nothing else sequences there.
2. Section enumeration — PASS. The six `!` section lines are the review family six-set
   (`independence` · `scope` · `inputs` · `verdict` · `output` · `reserved`), set-wise equal to
   the preamble's printed sections and in its order; every section carries rules, so no empty
   marker is owed. `section-set` non-rejecting; suite limb ok.
3. Floor-count pin and read-back — PASS. Preamble prints `- class: floor · 11 rules` with its
   `floors:` index; the body's closing sentence cites both and carries no hard-coded number.
   `expected-skills.json` carries the skill as a post-freeze review-family member with
   `floor_pin` 11 and eleven ids, and the suite confirms the set matches the render at 11 ids.
4. Floor survival — PASS. Born this wave; the eleven floors are minted, none leaves. Each of the
   eight locally-authored rules that needed one carries an anchor to D2, D3, D6, D7, D9 or D11;
   `protected-exit` and `anchor-format` non-rejecting.
5. ID continuity — PASS. All ids are first mints under the `validation-primitive-edit.` prefix;
   `mint-once`, `id-duplicate`, `cite-unresolved`, `tombstone-integrity` non-rejecting. The one
   cross-family prose citation, `common.gate-loop-bound` inside `.gate-loop-bound`, is the
   deliberate single-home pointer the migration's own common-rule text names in return.
6. `extends:` conformance — PASS. Four stubs, all onto the skill's own family library
   (`review-common.author-grader` · `.never-excess` · `.default-fail` · `.evidence-floor`), each
   declaring `class:` locally and inheriting text and labels only; the `<skill>.*` id stays
   citable. `extends-unresolved`, `extends-cross-family`, `extends-class-local` non-rejecting.
   The near-dup bar is served: three new edges in `scripts/similar-rules-allowlist.yaml` record
   the stub-versus-local adjudications, and the similarity pass reports `clusters: 0 (none)`.
7. `description:` — PASS. Born this wave, so the item reads "≤ 1,536 chars" only; measured 730.
8. Budget — PASS. Measured payload 15,022 (body 3,449 + render 11,573) equals the first-seed row
   exactly, and the measured description 730 equals its row exactly. Both rows are seeded
   `[v0.111.0]` on the ruled birth seed, the fourth seeding path the same diff opens under record
   build item 1. No headroom, no overage, nothing to argue.
9. Pointer resolution — PASS. `pointer resolution: 84 checked`, no `pointer-unresolved`; this
   schema declares no `pointer:`.
10. Deterministic pre-pass — PASS. Run by this seat and quoted above.
11. Skill-grammar conformance — PASS. `skill-grammar`, `rule-kind-unknown`, `when-undeclared`,
    `moment-declaration` non-rejecting. Kinds used are `binding`, `routing`, `duty`, `bound` and
    `reservation`, all in the eight-kind skill set, with `constraint` left as the omitted default
    on five rules; no `kind: fail`, no `enforces:`, no `moments:`, no `conditions:` and therefore
    no `when:` term to resolve. The `verdict` var resolves in the rendered `.default-fail` text.
12. Provenance anchors — PASS. Anchors ride the minting migration and are enforced by the binary
    at apply; the cited decisions cover each rule they sit on.

Router discoverability — PASS. `plugins/mochiko/skills/mochiko/SKILL.md` line 125 carries the row
under its own "Primitive-edit gate" heading, and lines 35–36 name the skill in the family intro.
The row is accurate against the render: unit keying, the first-hand pre-pass, the evidence-read
line, the outcome line, the plain seat with an explicit alias, one seat per wave, the one
re-audit, the second FAIL to the user, and both negative boundaries.

VERDICT: PASS

Issues requiring fix: none.

## Failure narrative

One unit fails, on one item. `patterns-model-tiering`'s payload measures 18,358 against a 10,852
budget. The overage is not new — the ledger carries a ruled-HOLDS +7,012 from the v0.110.0
seat-default-key wave — but this wave's minted floor adds +494 of render, and neither the ledger
row nor the audit brief records the new figure. The pre-assert makes an unnamed overage a FAIL.

Tried before ruling FAIL: the ledger diff was read in full to see whether the row had been
restamped in the same change (it had not, while the new skill's two rows were added at v0.111.0
in that same diff); the body was measured separately and is unchanged at 3,112, localizing the
whole +494 to the render and so to the one minted rule; and the justification class was checked
against the row's own precedent, where the v0.110.0 growth was ruled a genuine new obligation by
the prior audit. The substance holds. The record does not.

## Notes of note

- Outcome-line file counts are the unit's graded surfaces: two per pair, three for
  `validation-primitive-edit`, whose router row is graded too.
- Unit 1, criterion 8, non-blocking. Record build item 1 asks setup's rule for "a plain fresh
  seat"; the minted rule says "a fresh seat that authored no surface" and drops "plain". The
  same item says the persona name leaves at wave 3, not here, so the instruction pulls both
  ways and the built rule stays neutral on persona. No floor and no protected line is lost.
- Unit 1, criterion 11, non-blocking. The same obligation renders `kind: bound` on the skill
  and the omitted `constraint` default on `setup.gate-loop-bound`, because a stub inherits
  text, labels and pointer but not `kind:`. Both are legal; no criterion demands parity.
- The contract-suite evidence above is a filtered 7-of-89 host run, not release gate 6.
