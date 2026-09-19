---
report: review
round: 1
---

# A3 — wave 1 audit: prose primitives and the crate

Seat: independent gate grader under the OLD form (`validator`, full read, default FAIL). Authored
nothing in any unit. Six units, each with its own verdict block and outcome line.

Pre-pass, run first-hand:

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
mochiko-cli migrate validate · 0 rejecting · 105 advisory
rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 (none)
allowlist-suppressed edges: 171
$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 1 · sequences 1..8 (8 migrations)
state sha256:30333fa8… · 74 documents · 1067 rules
$ mochiko-cli --version
mochiko-cli 0.2.0 · grammar 1..1
```

## VALIDATE — A1 · `.claude/rules/mochiko/primitive-edits.md`

**Verdict: FAIL — 2 blocking, 3 advisory.**

- PASS · Criterion survival. All 11 command criteria and all 12 skill criteria present with their
  substance; `grep -n '^  [0-9]\+\. \*\*'` returns 1–11 at lines 110–191 and 1–12 at 244–332, and
  every removed line in `git diff HEAD` is a heading rewrite carrying its text forward.
- PASS · Char-budget paragraph. The D7 pre-assert text is relocated verbatim into its own
  paragraph; the diff shows no word changed between the old lines 34–42 and the new lines 55–63.
- PASS · D3 stated. "The deterministic pre-pass runs first, and the grader runs it", plus "A
  pre-pass result quoted from the brief is not evidence, and nothing that output asserts is
  re-derived by judgment."
- PASS · D6 stated. Same grader seat resumed, fix-delta read, second FAIL to the user, fix again
  or drop, no run raises the bound — all present.
- PASS · D7 stated. Plain fresh seat (`general-purpose`, no persona), explicit `model:` alias
  never below the producer's tier, `opus` for a lead edit, rendered contract pasted verbatim, a
  hand-written contract section a floor miss on the same terms as an omitted alias.
- PASS · D9 grammar verbatim. The outcome-line grammar in the file matches the record's D9
  statement character for character, `cost:` clause included.
- PASS · D11 stated. One seat per wave, per-unit verdict block and outcome line, split only on
  context, said in the lines.
- PASS · `common.gate-loop-bound` cited by id, and the id resolves: `0008-gate-form.yaml` mints it
  at line 14 and mints `validation-primitive-edit.gate-loop-bound` at line 220.
- **FAIL (blocking 1) · The loop-bound paragraph restates the number it says lives elsewhere.**
  The same paragraph writes "A FAIL allows one fix and one re-audit" and then "The number lives in
  `common.gate-loop-bound` and nowhere else". Both cannot be true. D4 homes the number in the
  migration text precisely so a change is one migration entry; as written, changing it would leave
  this file stale and self-certifying. Fix: keep the id citation, drop the count from the prose,
  or reword to "restated here for the reader, authoritative in `common.gate-loop-bound`".
- **FAIL (blocking 2) · Skill criterion 1 is tagged wholly `[suite]`; the suite reads only the `!`
  lines.** `case_converted_shape` (`evals/contract/run.py:3826-3858`) asserts three things per
  primitive: every `!` line renders its own primitive, the `!` lines enumerate the render's
  sections in the render's order, and the literal `allowed-tools: Bash(mochiko-cli *)` is in the
  body. `CONVERTED_MARK` is the bang-line prefix itself — nothing reads the heading `## Rules —
  delivered by mochiko-cli`, nothing tests "no raw Read of a schema or of a family common file is
  demanded", nothing tests that a member's obligated reference read sequences in that section.
  Those three are judgment and the tag says they are mechanical. The command twin (criterion 1)
  was split correctly and even carries "Nothing mechanical reads these headings"; the skill twin
  was not. Fix: split it the same way.
- Advisory · Skill criterion 8's `[CLI]`. The legend defines `[CLI]` as "asserted by `mochiko-cli
  migrate validate`", but criterion 8's mechanical limb is the char-budget measurement, which is
  the pre-pass's *other* limb. The item says so itself ("`migrate validate`'s `budget` finding is
  advisory and counts rule text only — it is never the ledger comparison"), so no grader is
  misled, but the legend wording does not cover the tag it licenses.
- Advisory · A fourth `validator` reference left the file beyond the brief's three: "dispatch a
  separate validator" (old line 52). Ruled by D7, and its substance survives in "The editor never
  grades their own edit" plus the gate-grader paragraph. No protected content lost.
- Advisory · Prose-primitive routing softened from "the matching `validation-*` / `review-*` skill
  applies" to "reached as the domain lens where one exists". Consistent with D3/D7 — the gate
  grader now takes every unit — but the record's build item 2 does not name this change.

Tag inventory otherwise checks out. Every code named in a `[CLI]` tag exists in `Code` at
`crates/mochiko-cli/src/validate.rs:40-109` and every one of them is rejecting, not advisory
(`severity()` at 184-202; the advisory set is `deixis · unused-var · unused-condition ·
unused-moment · enforces-coverage · condition-coverage · budget · cite-foreign ·
labels-inherited · retired-selector · pointless-override · orphan-block · zero-member-label ·
skeleton-sigil`, and `budget` is the one the file names and correctly marks advisory).
`section-set` does apply to commands (`check_sections`, 1545-1610). Skill criterion 5's `[CLI]`
holds: `cite-unresolved` scans rule *text*, not only a structured field (1865-1912). Skill
criterion 3's `[suite]` limb on the frozen floor set is real — `case_converted_shape` compares
`EXPECTED` and `EXPECTED_SKILLS` floor ids against the render at `run.py:3897-3927`.

`audit: primitive-edits.md · validator · opus · 4 files · 1 rounds · 2 blocking`

## VALIDATE — A2 · `plugins/mochiko/skills/mochiko/SKILL.md` (router)

**Verdict: PASS — 0 blocking, 1 advisory.**

- PASS · Nothing removed. `git diff HEAD` on the router is additions only; the one apparent
  deletion is a line re-wrap of the existing `validation-*` sentence.
- PASS · The `validator` persona wording is untouched — "on the `validator` persona (today:
  `validation-constitution`; a PASS is still human-gated downstream)" survives verbatim, which is
  what wave 3 is meant to sweep.
- PASS · Accurate to the render. `mochiko-cli rules validation-primitive-edit --section preamble
  --plugin-root plugins/mochiko` declares six sections — `independence · scope · inputs · verdict ·
  output · reserved` — and the `SKILL.md` carries seven `!` lines (preamble plus those six, in that
  order) at lines 31–37. Every claim in the new table row maps to a rendered floor: unit keying and
  the criteria that follow it, first-hand pre-pass (`pre-pass-first-hand`), evidence-read line
  absent ⇒ FAIL (`tamper-proof-clause`, `evidence-floor`), the outcome line (`sec.output`), plain
  seat with explicit alias (`plain-seat-explicit-tier`), never the editor (`author-grader`), one
  seat per wave, the bounded loop (`gate-loop-bound`) and the second FAIL (`second-fail-user`).
- PASS · Budget. The router body is deliberately unbudgeted (ledger line 508, "its body IS the
  router index"), and the router `description:` is untouched, so the growth takes no budget.
- Advisory · The new row restates the loop-bound number ("a FAIL allows one fix and one
  re-audit"), and the router is not in `common.gate-loop-bound`'s consumer sweep list, which names
  only `primitive-edits.md` and `validation-primitive-edit.gate-loop-bound`. A rename or tombstone
  of that id would miss this row. Same defect class as A1 blocking 1, one altitude lower.

`audit: router SKILL.md · validator · opus · 3 files · 1 rounds · 0 blocking`

## VALIDATE — A3 · `.mochiko/memory/primitive-cost-budgets.md`

**Verdict: FAIL — 2 blocking, 1 advisory.**

Measurements taken first-hand with the ledger's own canonical snippet (lines 519–534), payload =
body chars plus the seven rendered blocks, `--plugin-root plugins/mochiko`:

| primitive | body | render | payload | description |
|---|---|---|---|---|
| validation-primitive-edit | 3,449 | 11,582 | 15,031 | 730 |
| patterns-model-tiering | 3,112 | 15,246 | 18,358 | 1,208 |

- PASS · Fourth seeding path. "a ruled birth seed (`author-grader-consolidation`, build item 1,
  2026-09-19: …)" is anchored to the record and the build item, and it is coherent with the rule
  paragraph it joins — that paragraph lists the admissible sources a budget may enter from, and
  the new clause states why the three existing ones did not fit (all assume a primitive that has
  already shipped).
- PASS · `patterns-model-tiering` restamp. Re-measured payload 18,358 matches the row exactly;
  body 3,112 and description 1,208 match the row's "unchanged" claims; the stated +494 is exactly
  15,246 − 14,752 against the prior row. The prior chain ([v0.110.0] +7,012, [v0.108.0] +3,844,
  the [v0.106.0] and [v0.102.0] seeds) is kept intact. Floor pin 7 → 8 confirmed: the preamble
  render prints `- class: floor · 8 rules`, and `0008-gate-form.yaml` mints exactly one rule on
  that document (`patterns-model-tiering.persona-less-grader-pin`, line 46), so the
  single-component attribution holds.
- PASS · `validation-primitive-edit` description row. 730 measured, matches; "no headroom" matches
  the birth-seed rule.
- **FAIL (blocking 1) · The `validation-primitive-edit` bodies row does not reproduce.** The row
  reads "payload: body 3,449 + render 11,573" and seeds the budget at 15,022 with no headroom. The
  render measures **11,582**, so the payload is **15,031** and the primitive ships 9 chars over its
  own birth budget on the day it is seeded. The method is not in doubt: the identical script
  reproduces `patterns-model-tiering` to the character. The figure most likely predates P1's A2 fix
  round (three rule rewordings, migration re-stamped to hash `80f48ff1…`) and was not re-measured.
  The ledger's own bar is "identical results required of every grader". Fix: restate the row at
  body 3,449 + render 11,582 = 15,031, and correct the same figure in the CHANGELOG entry.
- **FAIL (blocking 2) · The R5 disclosure paragraph makes a false superlative on a stale figure.**
  It says 730 "is the library's largest description" and cites "`patterns-model-tiering` 643" as
  precedent. Measured across all 31 skill `SKILL.md` files, the largest description is
  `patterns-model-tiering` at **1,208** — larger than 730 — and 643 is that skill's *birth* figure
  from v0.77.0. The same ledger contradicts the paragraph two sections up: the bodies row reads
  "description 998 → 1,208 chars under the 1,536 cap". Fix: drop the superlative, or re-key it to
  "the largest in the review family", and correct or drop the 643 precedent. The 709 and 686
  figures are right.
- Advisory · The trailing parenthetical "(Wave 2 seeded its budgets from its own audited cut
  results this way at v0.64.0)" now sits after the newly appended fourth path, so its antecedent
  reads as the birth seed rather than the editorial-cut path it belongs to.

`audit: primitive-cost-budgets.md · validator · opus · 3 files · 1 rounds · 2 blocking`

## VALIDATE — A4 · `CHANGELOG.md` 0.111.0 and the two manifests

**Verdict: FAIL — 1 blocking, 2 advisory.**

- PASS · Version sync. `plugins/mochiko/.claude-plugin/plugin.json` 0.110.0 → 0.111.0 and
  `.claude-plugin/marketplace.json` metadata 0.110.0 → 0.111.0, both in the diff.
- PASS · Names the record (`author-grader-consolidation/record.md`), the D-list (D2–D7, D9, D11),
  the supersession of `author-grader-value-tiering`, the new skill with its eleven floors, and the
  migration `0008-gate-form.yaml` with all four of its mints.
- PASS · The tagging claim is honest: "the inventory corrected the assumption that the contract
  suite checks a command's scaffold headings, which it does not" — which I independently confirmed
  against `run.py`.
- **FAIL (blocking) · The grammar widening and the crate version bump are absent.** The entry never
  says `mint-rule` now takes an optional `section:`, and never says the plugin now needs
  `mochiko-cli` **0.2.0**. That is the one consumer-facing consequence of the wave:
  `0008-gate-form.yaml` carries a section-less `mint-rule` on `command-common/common`, which a
  0.1.x binary rejects at parse as `op-malformed`, so the whole log fails to replay and every
  command and skill halts. `crates/mochiko-cli/Cargo.toml` and `Cargo.lock` carry 0.1.0 → 0.2.0
  and the entry does not mention it. Fix: add the minimum-binary sentence.
- Advisory · The budget line repeats the stale render figure — "payload 15,022 (body 3,449 +
  render 11,573)". Same defect as A3 blocking 1; fix both together.
- Advisory · "Gates: … contract suite full sandbox run green" is written as an established result.
  `build-log.md` records `cargo test` 472 green, the full similarity sweep green, `migrate
  validate` 0 rejecting and views idempotent, but no contract-suite sandbox run. The line must be
  true at the bump, per GI-012 gate 6 (a SKIPPED suite is not green). Not counted blocking because
  the brief admits gate-figure placeholders.

`audit: CHANGELOG + manifests · validator · opus · 3 files · 1 rounds · 1 blocking`

## VALIDATE — crate diff (B1 source, B2 tests)

**Verdict: PASS — 0 blocking, 1 advisory.** Read against `.claude/rules/mochiko/rust-cli.md`; this
is the independent non-author code review that file requires.

- PASS · B1 · A section-less mint on a common library appends a block.
  `crates/mochiko-cli/src/replay.rs:509` — `None if is_library => schema.blocks.push(decoded)`.
- PASS · B1 · A section-less mint on any other kind is `op-malformed`. The rejection is at *parse*
  (`migration.rs:703-711`), and `ParseError::MalformedChange` maps to `op-malformed`
  (`migration.rs:116`, `replay.rs:248`). Rejecting at parse is the right altitude: the document's
  kind is in hand there, so it is a malformed op rather than a state a replay could discover.
- PASS · B1 · A `section:` present on a common document is `op-inapplicable` (`replay.rs:481-489`
  via `inapplicable()`, which carries `Code::OpInapplicable`, `replay.rs:285-292`).
- PASS · B1 · Blocks still reject `class` / `kind` / `when` / `enforces`. Unchanged and still
  reached, because the minted rule lands in `schema.blocks` and the block checks key on document
  kind: `validate.rs:1776-1796` for `class` and `1798-1815` for the `kind`/`when`/`enforces` loop,
  both emitting `Code::ExtendsClassLocal`, which is rejecting.
- PASS · B1 · Mint-once holds. `was_minted` guards before the match and `state.mint` runs only
  after a successful arm; all three error arms return early (`replay.rs:471-513`).
- PASS · B1 · The replay arm is total on the kind. All four combinations of
  `Some/None × library/not` are written, including the unreachable fourth with a comment saying
  why it is written anyway. The `..` in `Change::MintRule { section, rule, .. }` swallows only
  `doc`, which is bound from `change.doc()` at `replay.rs:340` — the new field is destructured
  explicitly.
- PASS · B1 · Bright line (GI-019). The change is pure data shape: no grading of meaning, no
  dispatch or sequencing, no gating beyond the two admitted grounds.
- PASS · B2 · Nine new tests, and they cover the claimed surface: positive append on both library
  kinds (`replay.rs` tests 1–2, each asserting block order and that `sections` stays empty), the
  extends resolution from a dependent stub (test 3), both rejections (`op-inapplicable` test 4,
  `op-malformed` test 5, plus the parse-level pair in `tests/migration.rs`), the class-on-block
  rejection (test 6), and mint-once in both directions — over a genesis-imported block and over a
  block minted earlier in the same log, with "mint-once keeps the first write, not the last".
- PASS · B2 · **The control/treatment pair is sound, and it is the stronger form.**
  `a_minted_common_block_resolves_a_dependent_stubs_extends` asserts the control raises
  `extends-unresolved · demo.lead` and the treatment raises none. A bare `load == Ok` would pass
  vacuously if the check never fired at all — a wrong stub id in the fixture, or the check
  unimplemented. The control proves the assertion has teeth by showing that the same fixture minus
  the mint produces exactly the finding whose absence the treatment claims. It also asserts no
  `extends-class-local`, so the pass is not bought by inheriting something it may not.
- PASS · B2 · `assert_clean` and the rejecting assertions do not contradict each other:
  `assert_clean` reads `replay.findings` (apply-time), while the block-shape tests assert over
  `replay.validation` and `is_deliverable()`. The test's own comment names that split.
- Advisory · B2 · The class-on-block test iterates `class`, `kind`, `enforces` but not `when`,
  which is the fourth field in the same source loop (`validate.rs:1800`). One more tuple closes it.

`audit: crate diff (migration.rs · replay.rs · their tests) · validator · opus · 4 files · 1 rounds · 0 blocking`

## VALIDATE — crate fixtures (B3 refresh, B4 layers, B5 README)

**Verdict: PASS — 0 blocking, 0 advisory.**

- PASS · B3 · Every moved number reproduces against the tree. `mochiko-cli migrate status
  --plugin-root plugins/mochiko` prints "sequences 1..8 (8 migrations)" and "74 documents · 1067
  rules", matching `fidelity.rs` (73 → 74 docs, sequences 1..8, 1043 → 1067 rules), `validate.rs`
  (74 docs, 1067), and `views.rs` (73 → 74, all three call sites). `migrate validate --report`
  prints "rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 · allowlist-suppressed
  edges: 171", matching the `matrix_similar.rs` corpus pins exactly.
- PASS · B3 · A refresh, not a weakening. No assertion is deleted or loosened; every change is a
  constant moving with a comment naming the migration that moved it. The zero-cluster assertion
  survives in both similarity tests, and the command-family pin keeps its `0` cluster slot in the
  tuple `(329, 12_607, 0, 60)`.
- PASS · B3 · The version bump is confined. The diff touches `version` in
  `crates/mochiko-cli/Cargo.toml`, with a comment giving the reason, and the matching `Cargo.lock`
  line. Nothing else in either file moves.
- PASS · B3 · `render.rs` 36 → 37 floor indexes and the `patterns-model-tiering` pin 7 → 8 both
  match the renders: the preamble prints `- class: floor · 8 rules` for that skill and
  `- class: floor · 11 rules` for the new one.
- PASS · B4 · Four layers, run first-hand:

```
$ cargo test -p mochiko-cli
test result: ok. 472 passed; 0 failed  (14 binaries plus doc-tests; exit 0)
$ cargo fmt --all --check
(no output; exit 0)
$ cargo clippy --all-targets -- -D warnings
Finished `dev` profile … (exit 0; zero warning or error lines)
$ cargo audit --deny warnings
Scanning Cargo.lock for vulnerabilities (31 crate dependencies)  (exit 0)
$ MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar the_detector_reproduces_the_live_runs_figures_over_the_corpus
test result: ok. 1 passed; 0 failed … finished in 110.87s
```

  The opt-in sweep was run deliberately: the default suite skips it
  (`matrix_similar.rs:983-986`), so P3's three corpus constants would otherwise have gone
  unverified by `cargo test` alone.
- PASS · B5 · The `mint-rule` row is accurate to the code. "`section:` is required on a command or
  skill schema and rejected on a common library, which carries its blocks at the document's top
  level: there, omit it and the rule is appended as a block" states exactly the three outcomes the
  parse and the replay arm produce, with the right severity for each.
- PASS · B5 · The no-bump sentence holds. "`mint-rule` gained an optional `section:` … and the log
  stayed at grammar 1" is consistent with the binary (`grammar 1..1`) and with the paragraph's own
  standard, because the widening cannot make an existing file mean something different — it only
  admits a shape that was previously malformed. The `Cargo.toml` comment's supporting claim that
  "no binary carrying the narrower rule was released" is true: `git tag --list 'mochiko-cli-v*'`
  and `git ls-remote --tags origin` return no crate release tag.

`audit: crate fixtures (5 test files · Cargo.toml · Cargo.lock · migrations README) · validator · opus · 8 files · 1 rounds · 0 blocking`

## Failure narrative

Five blocking, three units; the crate carries none. A1: the loop-bound paragraph
writes the number ("one fix and one re-audit") while claiming "the number lives in
`common.gate-loop-bound` and nowhere else"; and skill criterion 1 is tagged `[suite]`
in full when the suite reads only the bang lines.

A3: the new skill's render measures 11,582, not the recorded 11,573, so its birth
budget is seeded 9 chars under its own payload; and the R5 paragraph calls 730 "the
library's largest description" when `patterns-model-tiering` measures 1,208.

A4: the entry never tells a consumer the plugin now needs `mochiko-cli` 0.2.0, though
`0008-gate-form.yaml` halts every command under a 0.1.x binary.

## Notes of note

The brief's summary of `case_converted_shape` is under-inclusive: besides the four
assertions it names, the case also compares every pre-registered floor set against
what the binary renders, checks the `floors:` line against the section renders, and
checks probe-argument registration. Skill criterion 3's `[suite]` limb is accurate.

CLAUDE.md still names `mochiko:validator` as the grader for a shipped primitive edit,
which now disagrees with the rewritten rule file. Build item 3 defers that sweep to
wave 3, so this is ruled rather than a defect — but it is a live disagreement between
two governance surfaces until wave 3 lands.

A1 blocking 1 and the A2 advisory are one defect at two altitudes, and the A1 sibling
audit raised a third instance inside `validation-primitive-edit.gate-loop-bound`.
Worth a single fix pass across all three.
