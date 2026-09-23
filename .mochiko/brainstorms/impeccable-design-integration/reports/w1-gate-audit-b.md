---
report: review
round: 1
seat: gate-b (seat B of 2 — the wave's units split across two gate seats because their files would not fit one seat's context)
tier: opus
wave: impeccable-design-integration wave 1 (plugin 0.114.0)
units: 14 (seat B's share)
verdict: 13 PASS · 1 FAIL (unit 6, 0013-design-baseline-home) · 1 blocking
---

## Failure narrative

One unit fails, the `0013-design-baseline-home` schema content. The `design-baseline` template's
skeleton prints `## Platform` followed by the unbracketed literal `web`. Every other truth-part
field in that skeleton is a `[placeholder]`. D6 rules that the platform value "warns and never
silently defaults", and the template's own contract repeats "never silently defaulted".
`setup.design-scaffold-unconditional` scaffolds `design.md` "on every path … carrying its required
truth headings". So a producer that shapes the scaffold from `mochiko-cli template design-baseline`
with no platform answer writes `web`. The checklist item ("Is the value one of web / ios / …?")
then passes it, so the non-author grade cannot catch the default. The fix is one skeleton line in a
new migration, followed by a view re-emit and any template-fixture re-capture. Nothing else in the
seat-B units blocks.

## Floor read-back

The contract preamble pin is `class: floor · 11 rules`, and `floors:` lists author-grader ·
plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand ·
binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor ·
second-fail-user. These are 11, matching the pin.

## Pre-pass — run first-hand, quoted

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`:

```
mochiko-cli migrate validate · 0 rejecting · 112 advisory
pointer resolution: 84 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1142 · in-kind pairs scored: 179962 · clusters: 0 (none)
allowlist-suppressed edges: 186
```

The advisory lines on this seat's units are quoted below and never gate:
`condition-coverage · command/implement · ux_bearing · value "absent" is declared but named by no rule's when:` ·
`enforces-coverage · command/implement · - · 31 floor/gate rules no fail node enforces: … impl.design-first-write … impl.design-audit-advisory …` ·
`budget · command/implement · - · 111 rules · 23755 resolved characters of rule text` ·
`enforces-coverage · command/setup · - · 17 floor/gate rules …` · `budget · command/setup · - · 46 rules · 11677 …` ·
`budget · skill/analysis-codebase · - · 15 rules · 4446 …` ·
`condition-coverage · skill/review-specifications · manifest-present · value "absent" …` ·
`budget · skill/review-specifications · - · 31 rules · 6662 …` · `budget · skill/review-sufficiency · - · 27 rules · 6209 …`.

`mochiko-cli migrate status`: `sequences 1..15 (15 migrations)` · `state sha256:d2f0b62b5bbf… · 80 documents · 1142 rules`.

Views ≡ replay: `mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/views` printed
`80 documents`, and `diff -r <scratch>/views .mochiko/schema-views` found no differences.

## Pre-pass — char-budget measurement

This uses the canonical snippet in `.mochiko/memory/primitive-cost-budgets.md` (§How to measure). The
skill payload is the `SKILL.md` body plus the seven `!`-line renders, counted in characters.

```
review-sufficiency        body 3262 render 12381 payload 15643 desc 686
review-specifications     body 3441 render 13370 payload 16811 desc 490
analysis-codebase         body 4726 render  9674 payload 14400 desc 349
authoring-prototype       body 4908 render 11259 payload 16167 desc 565
testing-gap-finding       body 6970 render 15852 payload 22822 desc 770
testing-end-user          body 9596 render 12353 payload 21949 desc 500
patterns-design-direction body 7027 render  9489 payload 16516 desc 784
patterns-craft-floor      body 6992 render  7274 payload 14266 desc 853
review-design-audit       body 4510 render  7534 payload 12044 desc 573
patterns-vertical-tdd     body 5908 render 10027 payload 15935 desc 497
agent descriptions: product-designer 449 · staff-engineer 274 · qa-engineer 299
```

Every figure matches the ledger row this wave restamped or seeded. Commands and the router carry
no budget (ledger §Unbudgeted primitives).

## Unit 1 — command pair `implement`

VALIDATE: command pair · `implement` (`plugins/mochiko/commands/implement.md` + render; rules changed by 0013)
Checklist run: canonical-scaffold criteria 1–11 (`.claude/rules/mochiko/primitive-edits.md`), plus the run-command done-condition branch and the implement branch
Evidence read: `plugins/mochiko/commands/implement.md` (byte-unchanged vs `6e4b264`) · all seven `mochiko-cli rules implement --section …` blocks (40,853 chars) · `git diff HEAD -- .mochiko/schema-views/commands/implement.yaml` · `plugins/mochiko/migrations/0013-design-baseline-home.yaml` · record D5/D7/D8/D13
Pre-pass: 0 rejecting. The pins are `kind: fail · 15 rules` and `class: floor · 37 rules`, and the fail-conditions end line reads `15 rules`, which agrees with the pin.
- Scaffold headings and order — PASS: the `.md` is unchanged, and the canonical frontmatter, headings, and Entry/Goal/Not-done order all stand.
- Preserved responsibilities — PASS: the rewords of `impl.design-inputs`, `impl.gap-finding-blind-dispatch`, `impl.graded-fold` ("One carve" to "Two carves", the store carve verbatim) and `impl.card-contents` only add text. No clause leaves in the view diff.
- Floor survival — PASS: no floor was reworded or tombstoned. There are two new floors: `impl.design-first-write` (a reservation, D7) and `impl.design-audit-advisory` (D8).
- Independence — PASS: `impl.design-landing` has the write "graded by the landing verification seat", and no seat grades its own row.
- Reserved-to-user content in reserved — PASS: `impl.design-first-write` renders in `impl.sec.reserved` as `[class: floor · kind: reservation · when: ux_bearing=present …]`.
- Done-condition branch (run + implement) — PASS: the goal is fixed. The first-write signature rides the existing final-acceptance gate and adds no new ceremony.
- Argued overage — n/a, because commands are unbudgeted.
- Prose tokens — PASS: every `impl.*` token in the render and the `.md` resolves (my first-hand `comm` sweep came back empty).
VERDICT: PASS
Issues requiring fix: none. Advisory only: the vars `design_seat` (technical-analyst) and `designer_seat` (product-designer) are two letters apart and easy to confuse. Also, `impl.baseline-delta-grammar` names no author, while `impl.design-landing` makes `product-designer` the only writer of a design `baseline-delta.md` entry.

## Unit 2 — command pair `setup`

VALIDATE: command pair · `setup` (`plugins/mochiko/commands/setup.md` + render; rules changed by 0014)
Checklist run: canonical-scaffold criteria 1–11, plus the run-command done-condition branch
Evidence read: `git diff HEAD -- plugins/mochiko/commands/setup.md` and the full file · all seven `mochiko-cli rules setup` blocks · `git diff HEAD -- .mochiko/schema-views/commands/setup.yaml` · `0014-setup-product-truth-leg.yaml` · record D6 · the S4 user ruling in `build-log.md`
Pre-pass: 0 rejecting. The pins are `kind: fail · 6 rules` and `class: floor · 20 rules`, and the fail end line reads `6 rules`.
- Scaffold headings and order — PASS: one clause was added in Goal (the design home joins the every-path carve-out), and no heading changed.
- Preserved responsibilities — PASS: the `.md` change is a pure insertion. `setup.user-card-rulings` and `setup.baselines-bootstrap` keep their prior text verbatim and append to it.
- Floor survival — PASS: no floor was touched, and the count stays at 20.
- Independence — PASS: `setup.design-truth-write` says "one writer, the `product-designer` seat — never you". It also requires "a plain fresh seat that authored none of it grades … explicit `model:` alias", with the loop bounded by `setup.gate-loop-bound`.
- Reserved-to-user content — PASS: `setup.user-card-rulings` makes every product-truth answer and the ratification of the truth part and the brownfield-seeded system part the user's. This matches the S4 user ruling.
- Done-condition branch (run) — PASS: the goal is fixed, and the Not-done line cites the pin.
- Prose tokens — PASS: every `setup.*` token resolves.
VERDICT: PASS
Issues requiring fix: none.

## Unit 3 — skill pair `review-sufficiency`

VALIDATE: skill pair · `review-sufficiency` (`plugins/mochiko/skills/review-sufficiency/SKILL.md` + render)
Checklist run: skill-pair criteria 1–12
Evidence read: `SKILL.md` (unchanged; the load-first block is lines 15–24) · the seven rendered blocks · `git diff HEAD -- .mochiko/schema-views/skills/review-sufficiency.yaml` · the 0013 rewords · the ledger row
Pre-pass: 0 rejecting. The pin is `class: floor · 8 rules`. The payload is 15,643 against a budget of 15,423.
- Load-first · enumeration · read-back — PASS: unchanged. The block has seven `!` lines (the preamble plus the review six-set), and the read-back cites the pin and `floors:`.
- Preserved responsibilities — PASS: the rewords of `clause-ux-trace` and `fence-read-set` are additive. The prior sentences, `when: {scope: selection}`, and the anchors are unchanged.
- Floor survival — PASS: no floor was touched. Independence and reserved — PASS: both are unchanged.
- Argued overage +220 — PASS (HOLDS): the overage is all render. `sec.verdict` gained +406, which is clause 8 re-keyed per D13 ("reads the Screens & Flows trace plus the direction contract and the design baseline"). `sec.inputs` gained +51, the design baseline read per D5. This is a genuine new obligation, not restored playbook prose.
- Description — PASS: unchanged at 686, under the 1,536 cap.
VERDICT: PASS
Issues requiring fix: none.

## Unit 4 — skill pair `review-specifications`

VALIDATE: skill pair · `review-specifications` (`plugins/mochiko/skills/review-specifications/SKILL.md` + render)
Checklist run: skill-pair criteria 1–12
Evidence read: `SKILL.md` (unchanged) · the seven rendered blocks (the verdict block in full) · `git diff HEAD -- .mochiko/schema-views/skills/review-specifications.yaml` · the spec template view diff (the Direction block's five fields) · the `patterns-design-direction` view (one block at the head of each spec's Screens & Flows)
Pre-pass: 0 rejecting. The pin is `class: floor · 8 rules`. The payload is 16,811 against a budget of 16,174.
- Load-first — PASS: unchanged. Preserved responsibilities — PASS: this is a pure mint (`review-specifications.sf-direction-checks`). Floor survival — PASS: no floor was touched.
- Coherence — PASS: the checked fields (Mode · Register · Contract · Declared tokens · Incumbent world) match the spec template's `### Direction` block. The severities use the existing grammar (Critical / Important), and `when: {manifest-present: present}` resolves against the declared condition.
- Argued overage +637 — PASS (HOLDS): the render grew +874, of which `sec.verdict` is +873, the minted direction-checks duty (D5, build item 7). That is a genuine new obligation, offset by the known −237 format constant.
- Description — PASS: unchanged at 490.
VERDICT: PASS
Issues requiring fix: none.

## Unit 5 — skill pair `analysis-codebase`

VALIDATE: skill pair · `analysis-codebase` (`SKILL.md` + `scripts/detect-stack.sh` + render)
Checklist run: skill-pair criteria 1–12, plus a first-hand run of the script
Evidence read: the `SKILL.md` body (unchanged) · `git diff HEAD -- …/scripts/detect-stack.sh` (+140) · the view diffs for `skills/analysis-codebase.yaml` and `templates/codebase-analysis.yaml` · `0014-setup-product-truth-leg.yaml`
Pre-pass: 0 rejecting. The pin is `class: floor · 4 rules`. The payload is 14,400 against a budget of 13,776.
Script runs, first-hand: on this repo, `design_system` came back as five empty arrays with rc 0. On a scratch fixture with next, react, tailwind, radix, `@fontsource/inter`, `next/font/google`, and token files, it returned `{"ui_frameworks":["nextjs","react"],"css_systems":["css-modules","tailwind"],"fonts":["Inter","Roboto_Mono","inter"],"token_files":["apps/web/tailwind.config.js","packages/ui/design-tokens.json"],"component_libraries":["radix"]}`. The path-argument form behaves the same. `set -e` is safe because every probe is a non-final `&&` or ends in `|| true`.
- Load-first — PASS: unchanged. Floor survival — PASS: no floor was touched.
- Preserved responsibilities — PASS: the checklist reword keeps every prior item verbatim. The template `replace-document` is additive: Part 1's max_lines goes from 40 to 50, the Detection Method row is added, and the checks are widened.
- Coherence — PASS: `analysis-codebase.design-facts-seed` ("facts, never stances … only through setup's `product-designer` write, and later the frontend shelf") matches D6/D12.
- Argued overage +624 — PASS (HOLDS): +446 is new render in `sec.output` (the minted `design-facts-seed` plus the checklist reword, D6), and +178 is the standing overage ruled at v0.109.0. `scripts/` is exempt.
- Description — PASS: unchanged at 349.
VERDICT: PASS
Issues requiring fix: none. Advisory only: the body's "What to Extract" list does not mention the design-system facts, though the rendered checklist does. Also, the template's Part 1 skeleton runs 77 lines against a 50-line budget. That gap predates this wave (about 67 against 40 at HEAD), and the wave raised the budget by the same 10 lines it added.

## Unit 6 — schema content `0013-design-baseline-home`

VALIDATE: schema content · `0013-design-baseline-home` (migration + view diff: homes/product-design, homes/product, templates/design-baseline, commands/implement, skills/review-sufficiency, skills/review-specifications)
Checklist run: the AM-2 five — intent stated · anchor present where required · ID lifecycle right · floor and fail survival · register
Evidence read: `plugins/mochiko/migrations/0013-design-baseline-home.yaml` (all 395 lines) · the six view files (the diff plus the new untracked views) · `mochiko-cli template design-baseline` with and without `--check` · `mochiko-cli home .mochiko/product/design/design.md` · record D5/D6/D7/D8/D11/D12/D13
Pre-pass: 0 rejecting, and the views equal the replay. The `home` lookup resolves `product-design`, where `design.md` is a declared deliverable bound to template `design-baseline`.
- Intent stated — PASS.
- Anchor present — PASS: the header carries D5, each mint carries D7 or D8, and `impl.card-contents` gains D8. Every cited decision covers its rule.
- ID lifecycle — PASS: six mints and five rewords, all keeping their IDs, with no tombstones and no reuse.
- Floor and fail survival — FAIL: the skeleton's `## Platform` holds the unbracketed literal `web`. That contradicts D6's ruled "never silently defaults" and the template's own contract. The design scaffold is written on every path, and `--check` would pass a copied `web`. See the Failure narrative.
- Register — PASS: full register. The D12 stance-vs-value seam and the D11 a11y pointer are each stated once and not restated.
VERDICT: FAIL
Issues requiring fix:
1. Floor and fail survival (D6 no-silent-default), in the `design-baseline` template skeleton. The literal `web` sits under `## Platform`. Fix: in a new migration, reword the skeleton's Platform line to a bracketed choice with the not-recorded form, for example `[web | ios | android | adaptive | desktop — or "not yet recorded"]`. Then re-emit `.mochiko/schema-views/templates/design-baseline.yaml` and re-capture any crate template fixture that snapshots this skeleton.

## Unit 7 — schema content `0014-setup-product-truth-leg`

VALIDATE: schema content · `0014-setup-product-truth-leg` (migration + view diff: commands/setup, skills/analysis-codebase, templates/codebase-analysis)
Checklist run: the AM-2 five
Evidence read: `plugins/mochiko/migrations/0014-setup-product-truth-leg.yaml` (all 327 lines) · the three view diffs · the sibling `setup.store-scaffold-unconditional` · record D6 · the S4 ruling in `build-log.md`
Pre-pass: 0 rejecting, and the views equal the replay.
- Intent stated — PASS.
- Anchor present — PASS: every mint carries D6, and D6 covers the five asks, the rule that audience and purpose are never asked, the inline asking by the lead, the producer paired with a non-author validator, the third bootstrap arm, and the unconditional scaffold.
- ID lifecycle — PASS: four mints and three additive rewords, with no tombstones.
- Floor and fail survival — PASS: none was touched, and the setup pins stay at 20 floors and 6 fails.
- Register — PASS.
VERDICT: PASS
Issues requiring fix: none. The scaffold defect lives in 0013's template, not in this migration's rules.

## Unit 8 — prose primitive, router

VALIDATE: prose primitive · router `plugins/mochiko/skills/mochiko/SKILL.md` · strip `.mochiko/strips/mochiko.md` [v0.114.0]
Checklist run: coherence · preserved responsibilities · strip-entry fields
Evidence read: the full word-level diff against `6e4b264` · the new strip entry
- Coherence — PASS: there are three new rows, and the clauses added on ten rows match the landed rules: the design audit is advisory, low depth is advisory-only (D13), and craft-floor leaves a11y to `patterns-code-minimalism` (D11). The table rows are intact.
- Preserved responsibilities — PASS: the only removal is the `product-engineer` row wording. It is covered by a supersession-by-ruling entry (D3/D4) with all five fields and the superseded row verbatim. Every other change is a pure insertion.
VERDICT: PASS
Issues requiring fix: none.

## Unit 9 — prose primitives, `staff-engineer` + `qa-engineer`

VALIDATE: prose primitives · `plugins/mochiko/agents/staff-engineer.md`, `plugins/mochiko/agents/qa-engineer.md` (kitted personas)
Checklist run: coherence · preserved responsibilities · the axis-4 keystone · the advisory grid read
Evidence read: the diffs of both files · the descriptions, measured unchanged at 274 and 299
- Coherence — PASS: `skills:` gains `patterns-craft-floor` on staff (D8) and `review-design-audit` on qa (V1). Each file adds one bullet, in the form its sibling bullets use, with no workflow trace beyond theirs.
- Preserved responsibilities — PASS: these are pure additions, so no strip is owed.
- Advisory grid — not run this wave, per the brief. It is advisory, informs the grade, and never gates it.
VERDICT: PASS
Issues requiring fix: none.

## Unit 10 — prose, budget ledger

VALIDATE: prose · `.mochiko/memory/primitive-cost-budgets.md`
Checklist run: coherence (every restamped or seeded figure reproduced first-hand) · preserved responsibilities
Evidence read: `git diff HEAD -U0` of the ledger · the canonical snippet · the measurements above
- Coherence — PASS: every figure reproduces exactly: three birth seeds with their seven-block splits, six overage restamps, the description rows (784 · 853 · 573 · authoring-prototype 565), the persona row at 449, the "+4.7 % over 15,775" line, and the "683 or more under the cap" line.
- Preserved responsibilities — PASS: each restamp keeps its prior history under "Prior:", and the persona row is re-keyed per D4.
VERDICT: PASS
Issues requiring fix: none. Advisory only: the `validation-primitive-edit` paragraph cites "`testing-gap-finding` 709" as a figure on the current tree, but that description now measures 770. It is a hard-cap-only primitive, so no row is owed.

## Unit 11 — prose, contract pre-registration

VALIDATE: prose · `evals/contract/expected-skills.json`, `evals/contract/run.py`, `evals/contract/README.md`
Checklist run: coherence · preserved responsibilities
Evidence read: the three diffs · `python3 evals/contract/run.py --list` · `python3 evals/contract/run.py --host-only`
Pre-pass (optional, first-hand): `contract suite: 7/7 cases passed, 7 ran, 5 measurement(s) recorded and not asserted` and `FILTERED — 7 of 97 declared cases.` The count of 97 matches the README's "ninety-seven cases".
- Coherence — PASS: the three post-freeze rows carry floor ids that match the render pins (3 · 2 · 2). The two impl floors are in `EXPECTED`, and `design-baseline` is in `TEMPLATE_NAMES`. The totals add up: 117 + 2 = 119, 257 + 7 = 264, 174 case sessions, and 175 metered.
- Preserved responsibilities — PASS: the provenance note is append-only, and no pre-freeze field moved.
VERDICT: PASS
Issues requiring fix: none.

## Unit 12 — prose, manifests + changelog

VALIDATE: prose · `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `CHANGELOG.md` [0.114.0]
Checklist run: coherence · version sync · rename named · agents path resolves
Evidence read: both manifest diffs · the full `CHANGELOG.md` [0.114.0] entry · a first-hand existence check of every `plugin.json` agents path (none missing)
- Coherence — PASS: both manifests are at 0.114.0, and `./agents/product-designer.md` resolves. The changelog names the no-alias break (D4, R22), migrations 0011–0015, the floor deltas, and the hand re-key of `ARCHITECTURE.md` with its reason. The three `[lead fills at close: …]` placeholders are expected.
VERDICT: PASS
Issues requiring fix: none.

## Unit 13 — prose, similarity allowlist

VALIDATE: prose · `scripts/similar-rules-allowlist.yaml`
Checklist run: coherence (each row has a reason and live ids) · preserved responsibilities
Evidence read: the diff (17 `- ids:` rows counted) · a first-hand check that every id resolves in `.mochiko/schema-views/` (none missing) · the pre-pass cluster line
- Coherence — PASS: each of the 17 rows names the distinct duty or owner it keeps apart. The pre-pass shows `clusters: 0` with 186 suppressed edges.
- Preserved responsibilities — PASS: append-only.
VERDICT: PASS
Issues requiring fix: none.

## Unit 14 — prose, rename ripple

VALIDATE: prose · `README.md:132`, `ARCHITECTURE.md:199,207,216`, `evals/agents/product-designer/`
Checklist run: coherence · preserved responsibilities
Evidence read: the `README.md` and `ARCHITECTURE.md` diffs · the `ARCHITECTURE.md` header (hand-maintained prose, not a derived index) · the kit move (all `R`) and the `preregistration.md` stamp · a `grep product-engineer` across `plugins/mochiko`, the views, `evals/contract`, and `scripts` (hits only in frozen 0007/0010 and 0015's intent)
- Coherence — PASS: the four lines are re-keyed. The stamp discloses that the kit's `rules.json`/`evals.json` ids are stale under the old name and get re-minted on the next read.
- Preserved responsibilities — PASS: word swaps only.
VERDICT: PASS
Issues requiring fix: none. Advisory only, and outside this unit's scope (it belongs to the landing): `ARCHITECTURE.md` does not yet name the three new skills or the `product-design` home, and its header still stamps v0.110.0.

## Outcome lines

audit: implement command pair · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: setup command pair · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: review-sufficiency skill pair · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: review-specifications skill pair · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: analysis-codebase skill pair · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 4 files · 1 rounds · 0 blocking
audit: 0013-design-baseline-home schema content · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 7 files · 1 rounds · 1 blocking
audit: 0014-setup-product-truth-leg schema content · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 4 files · 1 rounds · 0 blocking
audit: router mochiko SKILL.md · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 2 files · 1 rounds · 0 blocking
audit: staff-engineer + qa-engineer personas · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 2 files · 1 rounds · 0 blocking
audit: budget ledger · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 1 files · 1 rounds · 0 blocking
audit: contract pre-registration · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: manifests + CHANGELOG · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 3 files · 1 rounds · 0 blocking
audit: similar-rules allowlist · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 1 files · 1 rounds · 0 blocking
audit: rename ripple · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 5 files · 1 rounds · 0 blocking

## Round 2 — delta re-audit (same seat, resumed)

Scope: only what the fixes touched and what they could have broken. That is `0018` with its view and
fixtures, the ledger rows re-measured this round, `CHANGELOG.md` [0.114.0], `fidelity.rs`, and the spec
fixtures. I also read `0016`/`0017`, because they could change this seat's units. Neither touches
`implement`, `setup`, `review-sufficiency`, `review-specifications` or `analysis-codebase`. Those
payloads re-measure unchanged (15,643 · 16,811 · 14,400). `0016`'s one-block-per-surface keying
agrees with those units' "each touched surface's / per surface's Direction block" wording.

Pre-pass, re-run first-hand: `mochiko-cli migrate validate · 0 rejecting · 112 advisory` · `clusters: 0
(none)` · `allowlist-suppressed edges: 186` · status `sequences 1..18 (18 migrations)` ·
`state sha256:13ea99d9… · 80 documents · 1142 rules`. Views ≡ replay: a fresh `views emit` diffs
identical. `cargo test -p mochiko-cli --test render --test fidelity --test views`: 17 · 39 · 11
passed, 0 failed. `python3 evals/contract/run.py --host-only`: `7/7 cases passed`.

## Round 2 — unit 6 `0013-design-baseline-home` (fix: `0018-design-baseline-platform-placeholder`)

VALIDATE: schema content · `0018-design-baseline-platform-placeholder` + view `templates/design-baseline.yaml` + fixtures `design-baseline.{producer,check}.txt`, graded as the fix to unit 6
Checklist run: the AM-2 five on the fix, plus the prior issue's closure
Evidence read: `plugins/mochiko/migrations/0018-design-baseline-platform-placeholder.yaml` · a text diff of its template `content:` against 0013's (one line differs, `web` becomes `[web | ios | android | adaptive | desktop — or "not yet recorded"]`) · `mochiko-cli template design-baseline` Skeleton · both fixtures against the live render (identical except the `schemas:` footer, which `render.rs:1227` appends)
Pre-pass: as above.
- Intent stated — PASS: "so a scaffold never carries a silent platform default."
- Anchor present — PASS: D6, which covers the rule this line enforces.
- ID lifecycle — PASS: `replace-document` of a template (legal per `replay.rs`). No rule was minted or retired, and `fidelity.rs` holds 337/805/119/264.
- Floor and fail survival — PASS: the D6 no-silent-default now holds in the skeleton, and `--check`'s "or is it stated as not yet recorded" has a skeleton form to match.
- Register — PASS.
- Prior issue 1 — CLOSED.
VERDICT: PASS
Issues requiring fix: none.

## Round 2 — unit 10 budget ledger

VALIDATE: prose · `.mochiko/memory/primitive-cost-budgets.md` (rows re-measured this round)
Checklist run: coherence (figures reproduced first-hand) · preserved responsibilities
Evidence read: the changed rows · the canonical snippet re-run
- Coherence — PASS. authoring-prototype 16,201 (4,918 + 11,283, +1,221) · testing-gap-finding 22,906 (6,970 + 15,936, +2,929) · patterns-design-direction 16,828 (7,137 + 9,691, block split 1,639 · 687 · 1,627 · 3,060 · 811 · 1,247 · 620) · "+6.7 % over 15,775" (16,828 / 15,775 = 1.067) all reproduce exactly. The state stamps read `sha256:13ea99d9…`. My round-1 advisory is fixed: line 322 now reads "709 (770 from [v0.114.0] …)". The patterns-design-direction birth seed moves before the primitive ships, which is still under the +25 % abort bound.
- Preserved responsibilities — PASS: the prior history clauses are kept.
VERDICT: PASS
Issues requiring fix: none.

## Round 2 — unit 12 manifests + changelog

VALIDATE: prose · `CHANGELOG.md` [0.114.0] (manifests unchanged since round 1)
Checklist run: coherence
Evidence read: the [0.114.0] migrations paragraph · `crates/mochiko-cli/tests/fidelity.rs` diff (sequences 1..18 and the counts comment)
- Coherence — PASS: it names `0011`–`0018`, with 0016/0017/0018 described as rewords and template replacements under D5/D13/D6. The counts (80 documents, 1,142 rules, floors 119/264) agree with `migrate status` and `fidelity.rs`. The `[lead fills at close]` placeholders are still expected.
VERDICT: PASS
Issues requiring fix: none.

## Round 2 — outcome lines (supersede round 1 for re-audited units)

audit: 0013-design-baseline-home schema content · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 11 files · 2 rounds · 0 blocking
audit: budget ledger · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 1 files · 2 rounds · 0 blocking
audit: manifests + CHANGELOG · gate-b (seat B of 2, split: files exceed one seat's context) · opus · 4 files · 2 rounds · 0 blocking

Seat B roll-up: all 14 units PASS. The other eleven keep their round-1 lines unchanged, and none of them was affected by 0016–0018.
