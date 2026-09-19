---
report: disclosure
round: 2
seat: P1 (schema producer)
wave: wave 2 — the plan-QA leg
unit: plugins/mochiko/migrations/0009-plan-qa-leg.yaml + the derived views + the contract-suite pre-registration
---

## Scope

Exactly the P1 row of `wave2-plan-qa.md` §2, detailed in §3.1: migration `0009-plan-qa-leg.yaml`
(sequence 9, header anchor `2026-09-03 producer-plan-enforcement D8`), the derived-view re-emit,
the contract-suite pre-registration for the new `review-seat-plan` member, and allowlist rows for
whatever near-dup edges the sweep actually reports.

Not mine, and untouched: the new skill's `SKILL.md`, the router row, the budget ledger,
`primitive-edits.md`, `CHANGELOG.md` and the two manifests (all P2); the frozen-census crate tests
(all P3); any crate source; the `plugin.json` version bump; every `DECISIONS.md` / `BACKLOG.md` /
`ROADMAP.md` landing move (§6, the lead's). No `git` mutation of any kind.

Rule ids and section ids are taken as pinned in §3.1 and are not renamed anywhere below.

## Write set

Created — `plugins/mochiko/migrations/0009-plan-qa-leg.yaml` ·
`.mochiko/schema-views/skills/review-seat-plan.yaml` (emitted, never hand-written).

Changed by `views emit`, which leaves untouched documents byte-identical — under
`.mochiko/schema-views/`: `skills/patterns-sound-loop.yaml` · `skills/patterns-plan-minimalism.yaml`
· `common/common.yaml` · `commands/architecture.yaml` · `commands/feature.yaml` ·
`commands/implement.yaml`. Brainstorm, setup and specify stubs render as `extends:`, so stay put.

Changed by hand — `evals/contract/expected-skills.json` · `evals/contract/run.py` (the
`PROBE_ARGUMENTS` row only) · `evals/contract/README.md` (figures only) ·
`scripts/similar-rules-allowlist.yaml` (only if the sweep reports a new edge). Nothing else;
disjoint from P2's six paths and P3's five crate-test files.

## Reads

- **Every reworded text is quoted from its derived view, not from memory** — the nine rule blocks
  in `patterns-sound-loop.yaml`, `common.yaml`, `implement.yaml`, `architecture.yaml`,
  `feature.yaml` and `patterns-plan-minimalism.yaml` were extracted by id before drafting.
- **No floor pin moves**: every change is `reword-rule`, `set-rule-field` on `anchor`, or
  `import-document`; no `class:` changes. `expected-skills.json` gives `patterns-sound-loop`
  `floor_pin` 6 and `patterns-plan-minimalism` 2, and `run.py`'s `EXPECTED` carries
  `arch.author-grader-separation`, `arch.sound-loop-floor` and their siblings — all unchanged.
- **The three common stubs exist and need no edit**: `brainstorm.plan-approval-producers`,
  `setup.plan-approval-producers` and `spec.plan-approval` each carry
  `extends: common.plan-approval-producers`, so rewording the block reaches all three.

## Reads, continued

- **The review six-set is the one `validation-primitive-edit` used**: `Family::of` in
  `crates/mochiko-cli/src/validate.rs` falls through to `Family::Review` for any stem that is not
  `authoring-`/`patterns-`, and `Family::sections()` gives independence · scope · inputs · verdict
  · output · reserved.
- **`seats` is not a skill label.** `.mochiko/schema-views/labels/skill-labels.yaml` carries
  thirteen labels and `seats` is not among them; `Code::LabelUnknown` is a rejecting code. See
  Stops and hand-offs.
- **Baseline before the change**: `migrate validate` 0 rejecting · 105 advisory; `migrate status`
  sequences 1..8 · 74 documents · 1067 rules.

## Change list — the five changes outside the command schemas

Header: `grammar: 1` · `id: 0009-plan-qa-leg` · `sequence: 9` · `anchor: 2026-09-03
producer-plan-enforcement D8` · intent one line · `hash:` written by `migrate stamp`.

1. **`reword-rule`** `skill/patterns-sound-loop` id `patterns-sound-loop.leg-1-seat-produces` (floor; id survives; the schema's floor pin stays 6). Text per lead ruling R3 — the rule's existing opening clause kept verbatim ahead of the user-authored D8 item 1 sentence, so no protected responsibility leaves silently: "Leg 1 — production sits with a seat, never the lead: the producing seat plans first, read-only, in a plan-only dispatch, and stops; a fresh peer of its persona type grades the plan per `mochiko:review-seat-plan`; the lead approves only a passed plan and resumes the seat to work. The plan grade is additive to leg 2, never a substitute."
2. **`set-rule-field`** `skill/patterns-sound-loop` id `patterns-sound-loop.leg-1-seat-produces` field `anchor` value `2026-09-03 producer-plan-enforcement D8` — replacing `2026-08-13 charter-ritual-balance`. Changing a live anchor is a protection-lowering exit (`replay.rs`, `lowers_protection`); the header anchor is the authority it needs, and it is present.
3. **`reword-rule`** `skill/patterns-sound-loop` id `patterns-sound-loop.disclosure-line` (must; anchor left at `2026-08-13 charter-ritual-balance`, since §3.1 item 4 re-anchors floors only). Text: "Every visit/run close report carries one line, pinned grammar: `floor: tripped|clear · seats: <who produced> / <who reviewed> · plans: <seat>:PASS|FAIL(n)[ dirty]…`. The `plans:` segment carries one entry per producing seat — `n` the re-plan rounds consumed on the run's one shared counter, and `dirty` appended where the seat changed the tree while planning. When clear: `floor: clear` alone — no seats segment and no plans segment, since no seats existed to name."
4. **`reword-rule`** `command-common/common` id `common.plan-approval-producers` (block; reaches the brainstorm, setup and specify stubs unchanged; "the lead", never DM). Text, verbatim from §3.1 item 3: "Any seat that writes artifacts plans first, read-only, in a plan-only dispatch, and stops; a fresh peer of its persona type — a generic seat for a persona-less producer — grades the plan per `mochiko:review-seat-plan`; the lead approves only a passed plan and resumes the seat to work; grading and fact-finding seats are exempt."
5. **`reword-rule`** `skill/patterns-plan-minimalism` id `patterns-plan-minimalism.grading-routing` (must; D8 item 9). Text, verbatim from §3.1 item 5: "`mochiko:review-seat-plan` grades a seat plan's rung-claim presence — blocking on presence; `mochiko:review-plan-artifacts` grades the design package's disclosed rung honesty at review — advisory, gap-list conformance blocking."

## Change list — the eight command-local rewords

Each keeps its id and its class. The four floors among them also take a `set-rule-field` on
`anchor` (item 9).

1. **`reword-rule`** `command/implement` `impl.plan-approval-producers` (must; exemption set keeps `verification`; DM wording): "Every seat that writes code or artifacts plans first, read-only, in a plan-only dispatch, and stops; a fresh peer of its persona type — a generic seat for a persona-less producer — grades the plan per `mochiko:review-seat-plan`, and you approve only a passed plan and resume the seat to work; grading, verification, and fact-finding seats are exempt."
2. **`reword-rule`** `command/implement` `impl.design-gaps-only` (must; keeps `pointer: mochiko:patterns-plan-minimalism`): "Design seats author exactly the named gaps, nothing more, each on a plan a fresh peer passed and you approved — rung-justified per the pointer skill."
3. **`reword-rule`** `command/implement` `impl.builder-decompose-disclose` (must): "Builders decompose each card into concrete tasks at build time, disclosed in the cycle report, and build test-first — on a plan a fresh peer passed and you approved."
4. **`reword-rule`** `command/implement` `impl.sound-loop-floor` (floor; keeps `pointer: mochiko:patterns-sound-loop`): "A judgment-authored write to a governing surface obliges the sound loop: a seat produces on a plan a fresh peer passed and you approved, an independent non-author seat reviews, the user rules. This run's seat wiring already carries it end to end."
5. **`reword-rule`** `command/architecture` `arch.seat-architect-producer` (must; `${producer_seat}` var kept verbatim): "${producer_seat} is the producing seat: baseline authoring, shelf-walk stance batches, amendments, and delta authoring. Recommends with reasons; never rules. Plans first, read-only, in a plan-only dispatch, and works only on a plan a fresh peer passed and you approved."
6. **`reword-rule`** `command/architecture` `arch.author-grader-separation` (floor): "Author ≠ grader — wherever a seat produces (baseline, stance batch, amendment, delta), no output is cleared by its author; a producing seat plans first, read-only, in a plan-only dispatch, and works only on a plan a fresh peer of its persona type passed per `mochiko:review-seat-plan` and you approved."
7. **`reword-rule`** `command/architecture` `arch.sound-loop-floor` (floor; pointer kept): "A judgment-authored write to a governing surface obliges the loop: a seat produces on a plan a fresh peer passed and you approved, an independent non-author seat reviews before the user's gate — the user's ruling alone never substitutes for the review leg — and every baseline, stance batch, and amendment takes that review leg. Status flips and orphan cleanup are transcription and ride the landing audit. Trigger test, exemptions, seat wiring, and disclosure: mochiko:patterns-sound-loop, referenced never restated."
8. **`reword-rule`** `command/feature` `feat.author-grader` (must) and `feat.sound-loop-floor` (floor; pointer kept) — FA: "Wherever a seat produces (delta card, baseline-delta.md, grooming proposal), no output is cleared by its author; a producing seat plans first, read-only, in a plan-only dispatch, and works only on a plan a fresh peer of its persona type passed per `mochiko:review-seat-plan` and you approved." FSL: as its current text, with "a seat produces on a plan you approved" replaced by "a seat produces on a plan a fresh peer passed and you approved" and every other clause byte-identical.
9. **`set-rule-field`** `anchor` = `2026-09-03 producer-plan-enforcement D8` on the four floors — `impl.sound-loop-floor` and `arch.author-grader-separation` (neither carries one today, so this raises protection) and `arch.sound-loop-floor` / `feat.sound-loop-floor` (each replacing `2026-08-13 charter-ritual-balance`, authorised by the header anchor).

## Change list — `review-seat-plan` import, independence

**`import-document`** kind `skill`, name `review-seat-plan`, content `kind: skill` · `skill:
review-seat-plan` · `vars: {verdict: PASS}` · the six review-family sections in family order.

**`review-seat-plan.sec.independence`** — title "Independence — who grades a plan, and at what remove"; intent "The fresh peer the plan goes to, the persona it is spawned from, and the two ways the criteria reach it."

- `.author-grader` — `class: floor`, `extends: review-common.author-grader`, no local text.
- `.fresh-peer-grader` — `class: floor`, `kind: binding`, `labels: [independence]`, anchor `2026-09-03 producer-plan-enforcement D3`. Text: "A seat's plan is graded by a fresh spawn of the same persona type as its author — a `staff-engineer` plan by a fresh `staff-engineer`, a `technical-analyst` plan by a fresh `technical-analyst` — never the author's own context, and never the lead. A producer spawned without a mochiko persona is graded by a fresh generic seat carrying this skill's render. `devils-advocate` is not a seat in this loop."
- `.two-way-delivery` — `class: must`, `kind: binding`, `labels: [binding]`, anchor `2026-09-03 producer-plan-enforcement D4`. Text: "Delivery runs two ways at once: this skill is model-invoked by its description, and every grader brief names it besides, with the render pasted — the grader's persona varies, so the criteria cannot ride a persona's preloaded `skills:` list. Until the install refresh lands, the grader Reads the repo pair by path instead, and the run records which path that grade used."

## Change list — `review-seat-plan` import, scope

**`review-seat-plan.sec.scope`** — title "Scope — the plan before the work, not the artifact after it"; intent "What this skill grades, and the leg it adds to rather than replaces."

- `.plan-not-artifact` — `class: must`, `kind: routing`, `labels: [boundary]`, anchor `2026-09-03 producer-plan-enforcement D5`. Text: "What is graded is the seat's plan, before its work is spent — never the artifact that plan describes. The grade is additive to the sound loop's leg 2 and never a substitute: the produced artifact is still graded by a non-author seat before the user's gate."
- `.its-command-states-them` — `class: must`, `extends: review-common.its-command-states-them`.
- `.never-excess` — `class: must`, `extends: review-common.never-excess`.

## Change list — `review-seat-plan` import, inputs and verdict

**`review-seat-plan.sec.inputs`** — title "Inputs — the plan verbatim, and the tree behind its claims"; intent "How the plan reaches the grader, and what the grader must check itself."

- `.plan-verbatim` — `class: floor`, `labels: [fence, evidence]`, anchor `2026-09-03 producer-plan-enforcement D2`. Text: "The plan reaches the grader as the seat's own verbatim text in the brief — never a summary, and never the author's report of it. The brief also names the seat's assigned scope and the other seats' declared write sets, which items 1 and 2 are graded against. The plan is persisted nowhere; that brief is the only home it has."
- `.no-on-trust` — `class: must`, `kind: duty`, `labels: [evidence]`. Text: "A claim the plan makes about the tree is checked against the tree: where the plan says something already exists, is already reused, or already carries a shape, the grader reads what the plan says it read. An on-trust claim is a finding under item 3, never a pass."

**`review-seat-plan.sec.verdict`** — title "Verdict — default FAIL against seven items"; intent "The posture, the seven criteria, and who re-grades a revision."

- `.default-fail` — `class: floor`, `extends: review-common.default-fail`.
- `.seven-items` — `class: must`, `kind: binding`, `labels: [verdict]`, anchor `2026-09-03 producer-plan-enforcement D5`. Text: "Seven items, 1 through 6 blocking and 7 advisory. (1) Scope fidelity — the plan covers exactly the brief's assigned scope (named gap, card, or deliverable); anything beyond, or anything owed and missing, fails it outright. (2) Write set declared — every path the seat will create or change is listed, disjoint from other seats' declared sets, none outside the brief's surfaces. (3) Reads named — where the plan claims something already exists or should be reused, it names what it read to know that; no on-trust claims. (4) Rung claims present — design seats: each design element carries its simplest-execution rung claim per `mochiko:patterns-plan-minimalism`, presence blocking and the claim's honesty advisory; builders: each task names the test it will write first. (5) Stops and hand-offs named — user-reserved questions appear as questions, never as decisions; the verification seat or `**TEST:**` gate the work hands to is named; the attempt bound is acknowledged. (6) No self-clearing step — no step where the seat grades, accepts, or lands its own output. (7) Size bound — a plan the grader cannot read in one sitting is a finding, never a FAIL."
- `.fail-cites-fix` — `class: must`, `labels: [verdict]`. Text: "A FAIL cites the item it failed and gives the fix. A verdict naming no item and no fix is not something the seat can revise against, and is not a grade."
- `.same-grader-regrades` — `class: must`, `kind: binding`, `labels: [independence]`, anchor `2026-09-03 producer-plan-enforcement D6`. Text: "After a FAIL the same grader seat re-grades the revision, resumed — it owns the fix list; a fresh grader only where that seat is gone. The re-plan bound, its shared counter, and the escalation past it belong to the dispatching command; they are referenced, never restated here."

## Change list — `review-seat-plan` import, output and reserved

**`review-seat-plan.sec.output`** — title "Output — one line to the lead, and nothing persisted"; intent "The grammar the verdict lands in, and the layer it lands beneath."

- `.verdict-to-lead` — `class: must`, `kind: binding`, `labels: [reporting, binding]`, anchor `2026-09-03 producer-plan-enforcement D2`. Text: "The verdict returns to the lead in one grammar and nowhere else: `PLAN GRADE: <seat> · PASS|FAIL · <items failed: fixes>`. The lead carries it into the close report's disclosure-line `plans:` segment. The plan itself is never written to disk, so this line and that segment are the whole of what survives the round."
- `.verdict-is-input` — `class: must`, `extends: review-common.verdict-is-input`.

**`review-seat-plan.sec.reserved`** — title "Reserved — the approval is the lead's, the second miss is the user's"; intent "The two dispositions no grader takes for itself."

- `.approval-is-the-leads` — `class: floor`, `kind: reservation`, `labels: [user-gate]`, anchor `2026-09-03 producer-plan-enforcement D6`. Text: "The approval is the lead's alone: it approves only on a PASS, and it may send a PASSed plan back with feedback on its own judgment. A second consumption of the re-plan bound goes to the user — re-plan again, re-staff, or narrow the scope — and is never the grader's call."

## Floor accounting

Reworded floors, each keeping its id so no pin moves: `patterns-sound-loop.leg-1-seat-produces`
(schema pin stays 6) · `arch.author-grader-separation` · `arch.sound-loop-floor` ·
`feat.sound-loop-floor` · `impl.sound-loop-floor` (the three command pins in `run.py`'s `EXPECTED`
are unchanged). `patterns-plan-minimalism` keeps its pin of 2 — `grading-routing` is a `must`.

**`review-seat-plan` expected floor pin: 5** — `review-seat-plan.approval-is-the-leads` ·
`.author-grader` · `.default-fail` · `.fresh-peer-grader` · `.plan-verbatim`. Counted here from
the fifteen rules above before §3.1 item 6 was corrected from its earlier self-contradictory 6,
and the two now agree. `review-common.evidence-floor` is deliberately not extended: the plan is
transient (D2), so the verdict's landing is `.verdict-to-lead` plus the disclosure line. P1
confirms the pin from the render before writing the pre-registration row, and reports any
disagreement rather than editing the row to match.

## Rung claims and the test named first

This seat executes a pinned wave plan rather than designing one, so item 4 is discharged on both
limbs. The three elements that are choices, disclosed rung-wise:

- Migration `0009` as one file — rung 1 (required): the log is the only editing surface for schema
  content, and a wave's sequence is lead-allocated.
- `[independence]` alone on `.fresh-peer-grader` — rung 2 (simpler shape): a registry widening is
  avoided where one live label already carries the rule.
- No `evidence-floor` stub — rung 1 (required): D2 makes the plan transient, so nothing lands.

Tests named first, in order: `migrate validate --report` at 0 rejecting · views ≡ replay via the
committed diff · the contract suite's `converted-shape` floor-set assertion against the render ·
the full similarity sweep. Each runs before the next edit, not after all of them.

## Command sequence

```
mochiko-cli migrate stamp plugins/mochiko/migrations/0009-plan-qa-leg.yaml
mochiko-cli migrate validate --report --plugin-root plugins/mochiko
mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views
mochiko-cli rules review-seat-plan --section preamble --plugin-root plugins/mochiko
mochiko-cli migrate status --plugin-root plugins/mochiko
MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar
```

`validate` must show 0 rejecting; an advisory delta from 105 is reported, never suppressed. The
preamble render is where the floor pin and ids are read for pre-registration. `status` should
print sequences 1..9 · 75 documents · 1082 rules. The sweep's baselines are P3's to re-key.

## Pre-registration diff

- **`evals/contract/expected-skills.json`** — `families.review.members` gains `review-seat-plan`
  between `review-plan-artifacts` and `review-specifications`; `skills` gains a row in the same
  alphabetical slot, modelled field-for-field on the `validation-primitive-edit` post-freeze row:
  `family: "review"`, the five `floor_ids` sorted, `floor_pin: 5`, `schema_bytes: 0`,
  `common: "schemas/skill-review-common.yaml"`, `common_bytes: 0`, `baseline_bytes: 0`,
  `body_bytes_pre: 0`, and `baseline_source: "post-freeze member, 2026-09-03
  producer-plan-enforcement D4 — no pre-conversion baseline exists, never measured"`. No existing
  row is touched; `frozen_utc`, `binary`, `plugin_version` and `labels_registry` stay as frozen.
- **`evals/contract/run.py`** — one `PROBE_ARGUMENTS` row in the review-family block, after
  `review-plan-artifacts`: argument `"the P1 schema seat's plan"` with the note that the graded
  plan arrives verbatim in the brief rather than as a file (D2), so there is no artifact path to
  name and the skill's own read-back is what runs. Nothing else in `run.py` moves.

## Pre-registration diff, continued

- **`evals/contract/README.md`** — eighty-nine cases to ninety-one and a hundred and fifty-eight
  sessions to a hundred and sixty-two; thirty-one skill delivery and thirty-one skill absence
  cases each to thirty-two; the gate-split paragraph's hundred and fifty-nine metered to a hundred
  and sixty-three, hundred and fifty-eight cases to a hundred and sixty-two, ninety-three across
  thirty-one skills to ninety-six across thirty-two, thirty-seven absence cases to thirty-eight,
  and "v0.112.0 added four" appended to the wave-figure sentence; the floor-ids table's "252
  across thirty-one" to "257 across thirty-two" with `review-seat-plan`'s five named as a
  post-freeze member of the 2026-09-03 producer-plan-enforcement ruling. `--list` is run and its
  number is what the prose is made to agree with, in that direction.
- **`scripts/similar-rules-allowlist.yaml`** — rows only for edges the sweep actually reports,
  each with its keep-distinct reason. None is written speculatively.

## Stops and hand-offs

- **Reserved to the lead — the `seats` label.** §3.1 item 6 pins `.fresh-peer-grader` to
  `[independence, seats]`, but `seats` is absent from the skill-labels registry and an unknown
  label rejects. This plan executes `[independence]` alone; the alternative is one extra
  `registry-add` on `skill-labels`. Say the word and it goes in.
- **Hand-off to P2.** The pin is **5**, not 6. P2's read-back names no number, so no P2 edit is
  owed, but its note recording "six floors by construction" wants correcting.
- **Near-dup convergence.** If any reword makes three or more texts identical, §3.1 item 4 sends
  them to `common.yaml` rather than the allowlist. Not expected — the three pointer floors keep
  their command-specific tails — but if the sweep says otherwise, that call is the lead's first.
- **Gate grader.** The §4 plain `general-purpose` seat reads this unit as "schema content (0009 +
  view diff, AM-2 five)"; the re-audit is the same seat resumed, one round. Re-plan bound: one
  round on the wave's shared counter, and a second consumption goes to the user.

## Risks

1. **Leg 1's new text is a splice of two authorships.** Lead ruling R3 keeps the existing opening
   clause verbatim ahead of the user-authored D8 item 1 sentence, so nothing protected leaves and
   the earlier drop is no longer a risk. What remains is that the sentence reads longer than
   either half alone; the mitigation is that both halves are quoted exactly, neither is
   paraphrased, and the migration carries the prior text with the anchor by construction — the
   supersession record D8 item 1 asked for.
2. **`review-seat-plan` becomes the only review member with no `evidence-floor` stub.** Ruled by
   §3.1 item 6 on D2's transience, but it is a visible family asymmetry and the most likely FAIL
   from a grader checking the family shape rather than the ruling.
3. **Figures drift between the README prose and `--list`.** The case and session counts are
   hand-written English in three places and the contract suite is the only thing that can settle
   them. Mitigation: `--list` is run first and the prose made to match it, never the reverse.

## Execution

Executed on the approved plan, round 2. `migrate stamp` wrote
`sha256:a58d7ab9d5ca501316e7f8b9a515f358b3c85bb1e29d2b3d7686c364e65b59af`.

```
mochiko-cli migrate validate · 0 rejecting · 106 advisory
rules scanned: 1082 · in-kind pairs scored: 161376 · clusters: 0 (none)
allowlist-suppressed edges: 170
```

Four new 2-member clusters appeared before the allowlist rows; none reached the 3-or-more
identical bar, so the convergence call did not fire. Suppressed: the pointer trio's third leg
`feat.sound-loop-floor`↔`impl.sound-loop-floor`, plus `review-seat-plan`'s three, reasons recorded.

## Execution — pin, figures, and the gate reads

Pin reads exactly as planned: `class: floor · 5 rules`, ids `author-grader` ·
`fresh-peer-grader` · `plan-verbatim` · `default-fail` · `approval-is-the-leads`.

Final `migrate status`, for P3's census re-key:

```
sequences 1..9 (9 migrations) · 75 documents · 1082 rules
state sha256:0c6b146078b31dae329e3f32f1e1f9adb77967c2b595ca1861067545f6d57d2d
```

Advisories 105 → 106, the one new line being the new skill's budget row — `budget ·
skill/review-seat-plan · - · 15 rules · 4339 resolved characters` — wave 1's shape exactly.

## Execution — verification and changed files

`views emit` run twice: 75 documents both times, the same seven paths, no further churn.
`run.py --list` counts **91** cases, which is what the README prose was then made to match.
`run.py --host-only` is **7/7 green**, `converted-shape` included: "the pre-registered floor set
matches the review-seat-plan render (5 ids)" and "review-seat-plan: the `floors:` line agrees
with the section renders".

Files this seat wrote, and no others: `plugins/mochiko/migrations/0009-plan-qa-leg.yaml` (new) ·
`.mochiko/schema-views/skills/review-seat-plan.yaml` (new, emitted) · the six re-emitted views
(`skills/patterns-sound-loop.yaml` · `skills/patterns-plan-minimalism.yaml` ·
`common/common.yaml` · `commands/architecture.yaml` · `commands/feature.yaml` ·
`commands/implement.yaml`) · `evals/contract/expected-skills.json` · `evals/contract/run.py` ·
`evals/contract/README.md` · `scripts/similar-rules-allowlist.yaml`. No git mutation.

## Execution — deviations

One, disclosed. `evals/contract/README.md` carried a second stale session figure outside the four
the plan named — the host-cases paragraph's "a hundred and fifty-one sessions", left behind at
v0.111.0 and made more wrong by this wave. Repaired to "a hundred and sixty-two" as a
fix-on-sight integrity repair, inside the same file the plan already owns.

The `expected-skills.json` `provenance` field also gained one sentence recording this wave's
post-freeze member, matching the precedent the wave-1 landing set. `frozen_utc`, `binary`,
`plugin_version` and `labels_registry` are untouched, and no pre-freeze row moved in any field.

## Execution — R1 breach and its repair

The first execution carried five `set-rule-field · field: anchor` ops where R1 licensed two. Three
of them re-anchored sidecar-anchored floors away from `2026-08-13 charter-ritual-balance`, which
the crate's fidelity test caught ("the anchor moved"). The plan's own change 9 named all five in
one bullet and did not separate raising protection from replacing a live anchor, which is how the
breach passed its own read.

Repaired in place, the migration being unshipped: the three ops on
`patterns-sound-loop.leg-1-seat-produces`, `arch.sound-loop-floor` and `feat.sound-loop-floor`
removed; the two on `impl.sound-loop-floor` and `arch.author-grader-separation` kept, both rules
having carried no anchor before, so each raises protection and needs no authority. No rule text
changed, and the D8 ruling still rides the migration's header anchor and the reworded texts.

## Execution — post-repair anchors

New hash: `sha256:2f0af318339374d772767424a5662a46453de3f98bd796a384d71cec26f8a9dd`. The five
anchors as emitted, read back from the views:

```
patterns-sound-loop.leg-1-seat-produces  anchor: 2026-08-13 charter-ritual-balance
arch.sound-loop-floor                    anchor: 2026-08-13 charter-ritual-balance
feat.sound-loop-floor                    anchor: 2026-08-13 charter-ritual-balance
impl.sound-loop-floor                    anchor: 2026-09-03 producer-plan-enforcement D8
arch.author-grader-separation            anchor: 2026-09-03 producer-plan-enforcement D8
```

## Execution — post-repair figures

`validate --report` 0 rejecting · 106 advisory · clusters 0 · suppressed 170. `status` unchanged
at sequences 1..9 · 75 documents · 1082 rules; state hash now
`sha256:d5cb369aef8f31d3b82e9f5a8bd7498e671cfc7d7156eda6e3514083ca7a81fd` — the figure P3's census
takes, in place of the pre-repair one. Views re-emitted to the same seven paths; `--host-only` 7/7.
