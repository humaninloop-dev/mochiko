---
report: review
round: 2
---

Pre-pass, run first-hand this round:

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 84 checked against plugins/mochiko
rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 (none)
mochiko-cli migrate validate · 0 rejecting · 105 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 1 · sequences 1..8 (8 migrations)
state sha256:7b58e8c16be44a8c8b22f64de5ba70bbb50d1db2e3da0c65ca9e56add4988343 · 74 documents · 1067 rules

$ mochiko-cli --version
mochiko-cli 0.2.0 · grammar 1..1
```

```
VALIDATE: .claude/rules/mochiko/primitive-edits.md (unit 1 — the Check section rewrite)
Checklist run:  internal coherence + preserved responsibilities against record D2, D3, D6, D7,
                D9, D11 and build-surface item 2; tag truth against the `Code` enum in
                crates/mochiko-cli/src/validate.rs and `case_converted_shape` in
                evals/contract/run.py
Evidence read:  .claude/rules/mochiko/primitive-edits.md (working tree and `git diff HEAD`) ·
                .mochiko/brainstorms/author-grader-consolidation/record.md lines 177-460 ·
                crates/mochiko-cli/src/validate.rs (Code enum, check_citations,
                tombstone-integrity block) · crates/mochiko-cli/src/migration.rs (disposition
                field) · evals/contract/run.py (case_converted_shape in full) ·
                evals/contract/expected-skills.json ·
                plugins/mochiko/migrations/0008-gate-form.yaml ·
                .mochiko/schema-views/common/common.yaml
Conformance:
  Criterion survival, command block — PASS. All eleven survive, titles and order byte-identical
    to HEAD (1 Scaffold conformance … 11 Ontology-grammar conformance); the diff adds tags and
    the two "run by the grader" clauses, and removes no criterion body.
  Criterion survival, skill block — PASS. All twelve survive, titles and order identical to
    HEAD. Item 7 gains a born-in-wave clause, item 8 an advisory-budget clause; nothing dropped.
  Removed lines confined to D3/D7 — PASS. Four removals: "Then the independent author ≠ grader
    audit" (superseded by the new opening paragraph), "graded by `mochiko:validator`" on the
    skill pair (D7), "dispatch a separate validator" (D7 — the clause relocates into the opening
    paragraph as "The editor never grades their own edit"), and the prose-primitive routing verb
    "applies" demoting to "reached as the domain lens where one exists" (D7, the gate grader now
    owns the unit). No protected or DECISIONS.md-traceable line leaves.
  D3 stated — PASS. First-hand pre-pass with "a pre-pass result quoted from the brief is not
    evidence"; judgment items keyed by unit across four kinds; the AM-2 five named verbatim for
    schema content; the delta re-audit reads "only what the fix touched and what it could have
    broken".
  D6 stated — PASS. Same grader seat resumed · delta read · second FAIL halts to the user with
    both fix lists · fix again or drop · no run raises the bound · the ledger waiver path named.
  D7 stated — PASS. Plain fresh seat, `general-purpose`, no persona · explicit `model:` alias
    equal to the tier the edit was produced at and never below · `opus` when the lead made the
    edit · omitted alias a floor miss citing `patterns-model-tiering.persona-less-grader-pin`
    (the id resolves in the log) · render pasted verbatim · hand-written contract section a
    floor miss on the same terms.
  D9 line grammar verbatim — PASS. `audit: <unit> · <seat> · <tier> · <n> files · <n> rounds ·
    <n> blocking[ · cost: $<x>]` matches the record character for character; both landing homes
    named; the cost field correctly restricted to launched sessions.
  D11 stated — PASS. One seat over every unit of a wave, per-unit verdict block and outcome
    line tagged with the seat, and the split-on-context escape with its disclosure duty.
  D4 — no count stated (round-1 finding 1) — PASS. The paragraph now reads "How many such
    rounds a landing gets is `common.gate-loop-bound`'s to say and nowhere else's", names both
    citing sites for the rename sweep, and states no number. See note 1.
  Skill criterion 1 tags (round-1 finding 2) — PASS. Split into a [suite] limb (the `!`-line
    enumeration and the `allowed-tools` grant) and a [judgment] limb (the heading itself, the
    no-raw-Read clause, the reference-read sequencing). True to `case_converted_shape`, which
    compares the `!` lines to `parse_preamble` order and greps the literal
    `allowed-tools: Bash(mochiko-cli *)` and nothing else of the body.
  [CLI] tags name real codes — PASS. Every code cited across both blocks exists in the `Code`
    enum: fail-segment · tombstone-integrity · mint-once · id-format · id-prefix · id-duplicate ·
    cite-unresolved · protected-exit · anchor-format · section-set · skill-grammar ·
    rule-kind-unknown · class-unknown · condition-declaration · when-undeclared · when-value ·
    moment-undeclared · moment-declaration · enforces-required · enforces-unresolved ·
    enforces-misplaced · extends-unresolved · extends-class-local · extends-cross-family ·
    pointer-unresolved · budget.
  [suite] tags true to `case_converted_shape` — PASS. Command criterion 1's claim that nothing
    mechanical reads the scaffold headings is correct. Skill criterion 3's [suite] limb for the
    frozen floor set is correct and in fact understated: the host case asserts the pre-registered
    set matches the render id-for-id and that the `floors:` line agrees with the section renders.
  ID-continuity tags — FAIL (blocking B1, below).
Round-1 residue: findings 1 and 2 both discharged.
VERDICT: FAIL
Issues requiring fix:
  B1 (blocking) — the ID-continuity criteria tag a judgment limb as CLI-asserted. Skill-pair
    criterion 5 is tagged wholly **[CLI]** (`tombstone-integrity`, `mint-once`, `id-duplicate`,
    `cite-unresolved`), with no judgment limb at all. Its body carries three clauses the binary
    does not decide: "A reword keeps its ID", "a split mints children recording the parent",
    "a merge tombstones the losers". `tombstone-integrity` asserts only that an id is not
    tombstoned twice, is not simultaneously live and tombstoned, and that its `disposition`
    string is non-empty — `disposition` is a free-text `String`
    (crates/mochiko-cli/src/migration.rs:302, parsed by `field_str` at :693), so nothing
    mechanical separates a reword that kept its id from a tombstone-plus-fresh-mint carrying any
    non-empty disposition. Under the file's own legend ("[CLI] — asserted by `mochiko-cli
    migrate validate` … never re-derived by judgment") a grader reads a clean pre-pass and
    passes the criterion without ever opening the disposition.
    Command-pair criterion 4 carries the identical three clauses and the same [CLI] list; its
    judgment limb is scoped to "a reference carried in prose rather than in a structured field"
    and so does not reach them. Command criterion 3 repeats "a reword keeps its ID" under a
    [CLI] limb whose judgment half covers only the `.md`'s citation and halt clause.
    Fix: add a judgment limb to skill criterion 5 and widen the judgment limbs of command
    criteria 3 and 4, naming the disposition read — whether the tombstone's disposition shows a
    reword that should have kept its id, a split whose children record the parent, or a merge
    whose losers are the ones tombstoned. This is the defect class round 1 found on skill
    criterion 1, left standing on its sibling.
```

```
VALIDATE: .mochiko/memory/primitive-cost-budgets.md (unit 2 — release-gate sweep)
Checklist run:  the fourth seeding-path sentence · the two `validation-primitive-edit` rows ·
                the restamped `patterns-model-tiering` row · the R5 disclosure paragraph, each
                re-measured with the ledger's canonical snippet against the quiesced tree
Evidence read:  .mochiko/memory/primitive-cost-budgets.md (working tree and `git diff HEAD`) ·
                plugins/mochiko/skills/validation-primitive-edit/SKILL.md ·
                plugins/mochiko/skills/patterns-model-tiering/SKILL.md ·
                .mochiko/strips/patterns-model-tiering.md · a canonical-snippet sweep of all 27
                skill `description:` values
Conformance:
  Fourth seeding path — PASS. Named, dated, anchored to `author-grader-consolidation` build
    item 1, and its reason stated: the three prior paths all assume a primitive that already
    shipped. It lands in the governing seeding-path sentence, which is the right home.
  `validation-primitive-edit` body row — PASS, re-measured and exact. Canonical snippet against
    the quiesced tree: body 3,449 · render 11,519 across the seven blocks (preamble 1,966 ·
    independence 1,974 · scope 1,383 · inputs 1,279 · verdict 2,679 · output 1,566 ·
    reserved 672) · payload 14,968. Stamp: 14,968 (body 3,449 + render 11,519). Match.
  `validation-primitive-edit` description row — PASS, re-measured and exact. Parsed value 730.
    Stamp: 730, budget 730 (no headroom). Match. Floor pin 11, as the row states.
  `patterns-model-tiering` restamp — PASS, re-measured and exact. Body 3,112 · render 15,246 ·
    payload 18,358 · description 1,208 · floor pin 8 (the row's 7 → 8). 18,358 − 10,852 = 7,506,
    the stated standing overage. The claimed +494 whole-delta over the v0.110.0 payload of
    17,864 is arithmetically exact, and the single minted floor
    `patterns-model-tiering.persona-less-grader-pin` is present in the preamble's `floors:` line.
  R5 disclosure paragraph (round-1 finding 4) — PASS, verified against a full sweep. The
    library's four largest skill descriptions on the current tree are patterns-model-tiering
    1,208 · validation-primitive-edit 730 · testing-gap-finding 709 · review-sufficiency 686.
    The paragraph names exactly those three precedents at exactly those figures, calls 1,208 the
    largest (correct), and qualifies with "at the figures these carry on the current tree",
    which is the honest hedge — none of the three is a budgeted row in the table. The 643 birth
    figure for patterns-model-tiering is corroborated twice in the ledger's own v0.77.0 and
    v0.79.0 paragraphs. 1,536 − 730 = 806, as claimed. The description's clause inventory holds:
    four SHOULD trigger phrases and two negative boundaries.
  Round-1 residue: findings 3 and 4 both discharged. The row itself records the two superseded
    readings and names the release-gate sweep as the authority, which is the right disclosure.
VERDICT: PASS
Issues requiring fix: none blocking. Advisory notes 2 and 3 below.
```

```
VALIDATE: CHANGELOG.md [0.111.0] + plugins/mochiko/.claude-plugin/plugin.json +
          .claude-plugin/marketplace.json (unit 3)
Checklist run:  the six named elements · manifest agreement · the binary-version claim
Evidence read:  `git diff HEAD` over all three files · DECISIONS.md (the 2026-09-19 row) ·
                plugins/mochiko/skills/mochiko/SKILL.md (the router claim) ·
                plugins/mochiko/migrations/0008-gate-form.yaml · `mochiko-cli --version` ·
                `cargo test -p mochiko-cli`
Conformance:
  Names the record — PASS. `.mochiko/brainstorms/author-grader-consolidation/record.md`
    D2–D7, D9, D11.
  Names the decisions — PASS. `DECISIONS.md` 2026-09-19, plus the supersession of
    `author-grader-value-tiering`. The row is present in the DECISIONS.md diff.
  Names the new skill — PASS. `validation-primitive-edit`, with its six-set and eleven floors;
    the floor count is confirmed by the render's pin.
  Names migration 0008 — PASS. `0008-gate-form.yaml` (sequence 8) with its four mint sites;
    `setup.validate-seat-form` and `setup.gate-loop-bound` both resolve in the file.
  `mint-rule` widening, no grammar-version bump (round-1 finding 5) — PASS. The optional
    `section:` is described with its three cases, and the no-bump reason is given: the change is
    additive and no `mochiko-cli-v*` tag has been released, so no consumer reads the old shape.
  `mochiko-cli` 0.2.0 requirement (round-1 finding 5) — PASS, and true of the tree's binary:
    `mochiko-cli 0.2.0 · grammar 1..1`. The halt-rather-than-degrade consequence is stated.
  Both manifests at 0.111.0 — PASS. plugin.json 0.110.0 → 0.111.0; marketplace.json metadata
    0.110.0 → 0.111.0.
  Collateral claims spot-checked — PASS. The router does gain a Primitive-edit gate table and
    does name the new member beside `validation-constitution`. The payload figures quoted in the
    entry (14,968 = 3,449 + 11,519, description 730) match my own measurement exactly.
  Gate claims — PASS on the three I can run. `migrate validate` 0 rejecting, quoted above;
    `cargo test -p mochiko-cli` green, 472 tests across 18 binaries, 0 failed, including
    `every_emitted_view_matches_the_committed_one`, which is the views ≡ replay gate.
  Round-1 residue: finding 5 discharged in both limbs.
VERDICT: PASS
Issues requiring fix: none blocking. Advisory note 4 below.
```

## Failure narrative

Unit 1 fails on one blocking finding, B1 above. The wave built its tag split from a build-time
inventory rather than a guess, and that inventory is right nearly everywhere — the
scaffold-headings correction and the floor-set cross-check are both sharper than the summary I
was briefed with. The hole is narrow and structural: three ID-continuity clauses describe
dispositions a human has to read, sitting under a tag that tells the grader the pre-pass already
answered them. Skill criterion 5 carries no judgment limb at all, so the instruction there is
unambiguous and wrong. Units 2 and 3 pass; every figure I re-measured against the quiesced tree
matched its stamp exactly, with none of the drift the v0.81.0 sweep caught.

## Notes of note

1. The loop-bound paragraph states no number, but "a fix and a re-audit" plus "A second FAIL
   halts" still lets a reader recover it. Build-surface item 2 requires both, so the file
   conforms; a later move of the number would still need this paragraph edited.
2. Ledger, pre-existing: the v0.79.0 paragraph still calls `testing-gap-finding` 709 the
   library's largest, in the same file as the new paragraph naming 1,208 the largest.
3. Ledger, pre-existing: "Classes measured" still describes a converted skill's payload as body
   plus `schema.yaml`, which v0.106.0 superseded with body plus render.

## Notes of note, continued

4. The CHANGELOG asserts the contract suite's full sandbox run green. That is unverifiable from
   this seat and must be sealed under release gate 6 before the bump.
5. The legend defines [CLI] as asserted by `migrate validate`, but skill criteria 3 and 8 use
   it for figures the render produces, and criterion 8 says outright that the budget finding is
   never the ledger comparison. Widening the legend to the binary's deterministic pre-pass
   output would remove the contradiction.

## Outcome lines

```
audit: .claude/rules/mochiko/primitive-edits.md · validator · opus · 8 files · 2 rounds · 1 blocking
audit: .mochiko/memory/primitive-cost-budgets.md · validator · opus · 5 files · 2 rounds · 0 blocking
audit: CHANGELOG.md [0.111.0] + both manifests · validator · opus · 6 files · 2 rounds · 0 blocking
```
