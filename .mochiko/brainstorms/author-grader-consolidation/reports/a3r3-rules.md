---
report: review
round: 3
unit: .claude/rules/mochiko/primitive-edits.md
seat: gate grader — plain fresh seat, authored nothing in the unit
tier: opus
verdict: PASS
blocking: 0
---

**VALIDATE:** `.claude/rules/mochiko/primitive-edits.md`, the wave-1 Check-section
rewrite, graded against its pre-wave form via `git diff HEAD` (read-only git, no
mutations). Round 3; round 2 failed this unit on one finding, graded below. This seat
authored nothing here and starts from a default-FAIL posture.

**Checklist run:** internal coherence plus preserved responsibilities against
`.mochiko/brainstorms/author-grader-consolidation/record.md` D2, D3, D4, D6, D7, D9, D11
and its build-surface item 2 — every numbered criterion surviving with its substance (11
command, 12 skill), every `[CLI]` / `[suite]` / `[judgment]` tag true against a mechanism
that exists, the grader-form paragraphs stating D3/D6/D7/D9/D11, the bound cited by id
with no count stated (D4), and removals confined to D3 and D7.

**Evidence read:**

- `.claude/rules/mochiko/primitive-edits.md` — the whole file, 387 lines, plus its
  `git diff HEAD` against the pre-wave form.
- `.mochiko/brainstorms/author-grader-consolidation/record.md` — D2 through D11 and the
  wave-1 build surface, items 1 through 7.
- `crates/mochiko-cli/src/validate.rs` — the `Code` enum at lines 40–109 and the
  `tombstone-integrity`, `mint-once`, `id-duplicate`, `anchor-format`, `section-set` arms.
- `crates/mochiko-cli/src/replay.rs` — the `protected-exit` and `mint-once` arms, `retire`.
- `evals/contract/run.py` — `case_converted_shape`, `converted_primitives`, `GRANT`,
  `BANG_LINE`, `EXPECTED_SKILLS_FILE`.
- `plugins/mochiko/migrations/0008-gate-form.yaml` and `DECISIONS.md`, for the cited ids
  and the 2026-09-19 row.

**Pre-pass:** run first-hand by this seat, never quoted from the dispatching brief.

    $ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
    pointer resolution: 84 checked against plugins/mochiko
    rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 (none)
    mochiko-cli migrate validate · 0 rejecting · 105 advisory

0 rejecting, so the unit proceeds. The 105 advisory rows are `budget`,
`condition-coverage` and `zero-member-label` findings; none gates. The char-budget half of
the pre-pass has no subject here: this unit is a repo rule file under `.claude/rules/`,
not a plugin primitive, so no budgeted class and no ledger row apply to it.

**Grader form — D3, D6, D7.**

- **D3, first-hand pre-pass — PASS.** "The deterministic pre-pass runs first, and the
  grader runs it", with "A pre-pass result quoted from the brief is not evidence" and the
  clause forbidding judgment re-derivation of what that output asserts.
- **D3, judgment items keyed by unit — PASS.** Criteria keyed by kind: command pair,
  schema-bearing skill pair, schema content, every other primitive. Schema content carries
  the AM-2 five — intent stated, anchor present where required, ID lifecycle right, floor
  and fail survival, register.
- **D6 — PASS.** Same grader seat resumed; the delta read stated as "only what the fix
  touched and what it could have broken, never the whole cluster again"; a second FAIL
  halting to the user with both fix lists; the user ruling fix again or drop.
- **D7 — PASS.** Plain fresh `general-purpose` seat with no persona; an explicit `model:`
  alias at or above the producing tier and `opus` when the lead edited; an omitted alias
  named a floor miss against `patterns-model-tiering.persona-less-grader-pin`; and
  `mochiko:validation-primitive-edit`'s render pasted verbatim, a hand-written contract
  section being a floor miss on the same terms.

**Grader form — D9, D11, D4.**

- **D9 — PASS.** The outcome-line grammar matches the record token for token: the literal
  `audit:` key, then unit, seat, tier, file count, round count and blocking count in that
  order, with the optional cost field bracketed last. The launched-session-only condition
  on that field and its `total_cost_usd` source are both carried, and a landing whose
  audits left no lines is called incomplete.
- **D11 — PASS.** One seat takes every unit of a wave, each unit keeping its own verdict
  block and its own outcome line tagged with the seat, the seat splitting into two only
  when the units' files would not fit its context and saying so in the lines.
- **D4 — PASS.** `common.gate-loop-bound` is cited by id, alongside
  `validation-primitive-edit.gate-loop-bound`, with the rename-or-tombstone sweep stated.
  No digit and no count of fixes or re-audits appears anywhere in the file. One wording
  overstatement in the same paragraph is noted below; it is not a count.

**Command criteria 1–11 — PASS.**

- **1.** `[judgment]` except the `allowed-tools` grant `[suite]` — `GRANT` is the literal
  `allowed-tools: Bash(mochiko-cli *)` at `evals/contract/run.py:3797`.
- **2.** `[suite]` enumeration is asserted order-wise (`requested == expected`), so the
  set-wise claim holds a fortiori · `[CLI]` `section-set` asserts `COMMAND_SECTIONS` for a
  command schema · `[judgment]` for tokens in `.md` prose, which nothing mechanical reads.
- **3, 4, 5.** `fail-segment`, `tombstone-integrity`, `mint-once`, `id-format`,
  `id-prefix`, `id-duplicate`, `cite-unresolved`, `protected-exit` and `anchor-format` all
  carry live arms; the judgment limbs cover the `.md`'s pin citation and halt clause,
  references carried in prose, and the lifecycle each disposition claims.
- **6, 7, 8.** `[judgment]` only, correctly — nothing mechanical reads substance across
  the pair, the done-condition branch, or preserved responsibilities.
- **9, 10, 11.** Pre-pass `[CLI]`; anchor presence and format `[CLI]` with ruling-coverage
  `[judgment]`; all thirteen ontology-grammar codes named in 11 exist in the enum.

**Skill criteria 1–12 — PASS.**

- **1, 2.** The `[suite]` limbs match what `BANG_LINE` and `GRANT` assert; `[CLI]`
  `section-set` resolves the family set for a skill schema; the delivered-by-CLI heading,
  the no-raw-Read clause and prose tokens stay `[judgment]`.
- **3.** Pin `[CLI]`, citing sentence `[judgment]`, frozen floor set `[suite]` — the case
  does assert that the pre-registered floor set matches the primitive's render from
  `expected-skills.json`, plus the `floors:` line against the section renders.
- **4, 5, 6.** `protected-exit`, `anchor-format`, `tombstone-integrity`, `mint-once`,
  `id-duplicate`, `cite-unresolved`, `extends-unresolved`, `extends-cross-family` and
  `extends-class-local` all carry arms; the near-dup bar and the dispositions stay
  `[judgment]`, correctly.
- **7, 8.** `description:` byte-identity is correctly `[judgment]`; 8 names its own
  mechanism inline and disclaims the advisory `budget` row as the ledger comparison.
- **9 through 12.** `pointer-unresolved`, `skill-grammar`, `rule-kind-unknown`,
  `when-undeclared`, `moment-declaration` and `anchor-format` all exist; 12's
  ruling-coverage limb is `[judgment]`, correctly.

**Round-2 fix — the disposition limb — PASS, and fixed more widely than raised.**
`tombstone-integrity` raises on exactly three conditions at
`crates/mochiko-cli/src/validate.rs:1284-1310` — tombstoned twice, both live and
tombstoned, an empty `disposition` — and never reads what a disposition says. Skill
criterion 5 and command criteria 3 and 4 now each carry a `[judgment]` limb saying so in
the binary's own terms: the disposition is free text, so the grader reads each tombstone's
and each supersession's disposition and confirms the lifecycle it claims actually
happened. The limb landed on all three items asserting ID lifecycle, not only the one
round 2 named. The claim matches the code exactly.

**Preserved responsibilities — PASS.** All 23 numbered criteria survive with their
substance, 11 command and 12 skill, enumerated and compared item by item against the
pre-wave form. Every change inside an item is an added tag or an added clarification,
never a deletion. The one added carve-out, skill criterion 7's allowance for a skill born
in its own wave, is required rather than a strip: the wave mints
`validation-primitive-edit`, which has no prior `description:` to be byte-identical to, so
without it the item would be ungradeable on the wave's own new skill.

**Removals and pointers — PASS.** Removals are confined to D3 and D7: the two
`mochiko:validator` grader sentences become "the gate grader", and criteria 9 and 10
change "cited in the audit brief" to run by the grader with its output quoted. The
`skill-content-schema` D8/I6 citation survives the first of those rather than leaving with
the persona name. No `mochiko:validator` reference remains; the surviving phrase "no
validator-for-skills exists" names a missing validation skill, not the persona. Every
cited id resolves — `common.gate-loop-bound`,
`patterns-model-tiering.persona-less-grader-pin` and
`validation-primitive-edit.gate-loop-bound` are live in migration `0008-gate-form.yaml`,
and the new sources entry's `DECISIONS.md` 2026-09-19 row exists.

**VERDICT:** PASS. Blocking findings: 0. The round-2 finding is repaired correctly and
applied to all three items that assert ID lifecycle. The unit is internally coherent, all
23 criteria survive, every tag is true in the sense that governs what a grader must do,
and the grader-form paragraphs carry D3, D4, D6, D7, D9 and D11 as the record states them.

**Issues requiring fix:** none blocking. Two optional wording repairs follow, neither
changing what a grader does.

## Notes of note

Two tag disagreements, both non-blocking, both against the legend's gloss rather than any
obligation a grader would act on. First, the `[CLI]` gloss reads "asserted by
`mochiko-cli migrate validate`", but skill criteria 3 and 8 source their mechanical limb
from the render instead. Both name their mechanism inline and 8 disclaims the advisory
budget row, so no grader is sent to a check that does not exist, and the legend's own
second clause covers them; widen the gloss to name the render. This is materially unlike
the round-2 finding, where the claimed check existed nowhere. Second, "cite that id rather
than restate what it holds" overstates: the paragraph restates every limb of
`common.gate-loop-bound` except the number, and D6 obliges it to carry that shape. Read
"restate the number it holds".

audit: .claude/rules/mochiko/primitive-edits.md · validator · opus · 1 files · 3 rounds · 0 blocking
