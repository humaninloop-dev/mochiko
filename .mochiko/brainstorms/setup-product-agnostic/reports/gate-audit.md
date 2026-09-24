---
report: review
pass: gate
seat: G1
tier: opus
contract: mochiko-cli rules validation-primitive-edit (binary 0.2.0 · grammar 1 · plugin 0.114.0) — floor pin 11, read back before the first step
wave: setup-product-agnostic wave 1
round: 1
units_graded: 22
units_pass: 21
units_fail: 1
blocking: 1
not_graded: unit 7 (crates/mochiko-cli/tests/**, evals/**) — producer not landed, held for resume
round_2: re-audit of 0023's fix (same seat resumed) plus the new implement pair — both PASS, 0 blocking
---

## Failure narrative

One unit fails: `0023-setup-agnostic-setup-rule-set.yaml`. It retires `setup.design-truth-write`
and `setup.product-truth-leg`, and `0019` rewrites the design-baseline writer sentence to match:
the setup lead writes only the empty scaffold, and `product-designer` writes content on two paths.
The live rule `impl.design-landing` was not touched and still names "the setup leg (the truth
part, the brownfield system-part seed, the scaffold)" as a `${designer_seat}` write path. That
contradicts `setup.design-scaffold-unconditional` and the landed template. The record calls a
consumer found later a build defect, not a new ruling. It was found by a first-hand sweep of
`.mochiko/schema-views/`, and no other consumer surfaced.

## Notes of note

- Advisory: `scripts/similar-rules-allowlist.yaml:558` names the tombstoned
  `analysis-codebase.capability-signals-seed-feature-map`, a validate warning this wave created.
- Advisory: `catalog/backend-service.md` BE-DEP lost its blank line before `**Content:**`.
- Tiering: no `Explore` seat spawned. Every read was grade-deciding or a completeness-sensitive
  stale-consumer sweep, which stays at seat tier.
- The envelope hook denied a first write of this report (Notes at 19 lines against 15). This
  version fits the bound.

## Pre-pass (shared run, first-hand)

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` gives
  `mochiko-cli migrate validate · 0 rejecting · 113 advisory`, exit 0.
- `mochiko-cli migrate status` gives `sequences 1..23 (23 migrations)` and
  `state sha256:b5e23504… · 80 documents · 1135 rules`.
- Views: `mochiko-cli views emit` into scratch, then `diff -r` against `.mochiko/schema-views/`,
  gives identical output. The derived view equals the replay.
- Pre-wave control: a scratch log of `0001`–`0018` replays to `sha256:13ea99d9…` · 1142 rules,
  the ledger's quiesced tree. Every budget delta below is measured against it with the ledger's
  canonical snippet: characters of the parsed value, body, description, and all seven rendered
  blocks summed.
- Floor sets (`class: floor` and `kind: fail` pins plus the `floors:` line) are identical pre vs
  post for setup, specify, architecture, implement and all seven skill pairs.
- `authoring-architecture-store` is confirmed byte-identical with `git diff --quiet`.

## Unit 1 — command pair `setup`

VALIDATE: command pair `setup` — `plugins/mochiko/commands/setup.md` + `mochiko-cli rules setup` (seven blocks)
Checklist run: canonical-scaffold criteria 1–11, run-command branch of item 7
Evidence read: `plugins/mochiko/commands/setup.md` (file + diff) · render of preamble and all six sections · `.mochiko/schema-views/commands/setup.yaml` diff · `plugins/mochiko/migrations/0023-setup-agnostic-setup-rule-set.yaml` · `.mochiko/strips/setup.md` [v0.115.0] · record D5
Pre-pass: `0 rejecting · 113 advisory`. Preamble pins: `kind: fail · 6 rules` · `class: floor · 20 rules`. Sections 5 · 6 · 11 · 6 · 7 · 6 = 41, matching D5's post-cut counts. The six end lines agree with the preamble.
- PASS scaffold headings and order: frontmatter keys, `# Setup — …`, Identity & Mission, the Rules block, and Entry, Goal, Not done are all unchanged. Only step 2's prose moved.
- PASS preserved responsibilities: the goal's reconstruction, amend-surfacing and `Assumed` baseline sentences leave by D5 ("the `Assumed` baseline sentences … go"). The strip entry quotes them verbatim, matching the diff. The governance-set sentence is kept byte for byte.
- PASS floor survival: 20 floors before and after. The floor rewords are all named in D5's amended seven: `blind-map-dispatch` (S11), `map-never-overwrite`, and `fail.no-feature-map`, whose text is verbatim D5.
- PASS independence: `design-truth-write`, which had its own grader, is gone. No seat grades its own row.
- PASS reserved-to-user: `user-card-rulings` stays in `sec.reserved`. `user-map-confirmation` is tombstoned with a D5 disposition.
- PASS done-condition, run branch: Entry handles `$ARGUMENTS`. The Goal is fixed and names the three every-path scaffolds. Not done cites the CLI pin with no hard-coded count and keeps the halt clause.
- PASS argued overage: commands are unbudgeted, so no overage applies.
VERDICT: PASS
Issues requiring fix: none

## Unit 2 — command pair `specify`

VALIDATE: command pair `specify` — `plugins/mochiko/commands/specify.md` (unchanged, `git diff --quiet`) + render
Checklist run: canonical-scaffold criteria 1–11, run-command branch
Evidence read: `plugins/mochiko/commands/specify.md` · `.mochiko/schema-views/commands/specify.yaml` diff · `0023` op on `spec.missing-map-surfaced` · preamble render
Pre-pass: `0 rejecting · 113 advisory`. Pins `kind: fail · 9` · `class: floor · 17` are identical to the pre-wave replay.
- PASS scaffold: `.md` untouched.
- PASS preserved responsibilities: the reword keeps the surfacing duty, the `/mochiko:setup` offer, "never silently tolerated", and `when: {feature_map: absent}`. It drops only the struck brownfield reconstruction, per record build step 4 (S9).
- PASS floor survival: floor set identical. The reworded rule is `class: must`.
- PASS independence, reserved content, done-condition: nothing moved.
- PASS argued overage: n/a.
VERDICT: PASS
Issues requiring fix: none

## Unit 3 — command pair `architecture`

VALIDATE: command pair `architecture` — `plugins/mochiko/commands/architecture.md` (unchanged) + render
Checklist run: canonical-scaffold criteria 1–11, desk branch of item 7
Evidence read: `plugins/mochiko/commands/architecture.md` · `.mochiko/schema-views/commands/architecture.yaml` diff · `0022` op on `arch.dm-store-integrity-close` · `0023` op on `arch.tools-brownfield-reconstruction` · `arch.shelf-scope-source` text · `authoring-architecture-store` view (`scope-line-durable-home`)
Pre-pass: `0 rejecting · 113 advisory`. Pins `kind: fail · 1` · `class: floor · 23` are identical to pre-wave.
- PASS scaffold: `.md` untouched.
- PASS preserved responsibilities: `tools-brownfield-reconstruction` keeps its derivation, confirmation, archive and say-so duties and its D16 anchor. It loses only the pointer to the retired setup bootstrap. `dm-store-integrity-close`, a floor, keeps its three duties and adds the D4 detector.
- PASS floor survival: 23 floors before and after. The floor reword is additive and ruled by D4.
- PASS D4 detector placement: record build step 4 names `authoring-architecture-store`, but D4's statement names "the architecture desk's write to the `Scope:` line". `arch.shelf-scope-source` makes a `Scope:` override "an ordinary store write" at the desk. The command is the right single home.
- PASS independence, reserved content, done-condition (desk branch, per-visit): unchanged.
VERDICT: PASS
Issues requiring fix: none

## Unit 4a — schema content `0019-setup-agnostic-modules-out`

VALIDATE: schema content — `plugins/mochiko/migrations/0019-setup-agnostic-modules-out.yaml` + view diffs `templates/{governance-intent,governance-surfaces,design-baseline}.yaml`, `common/skill-review-common.yaml`, `skills/{patterns-plan-minimalism,authoring-constitution,validation-constitution,patterns-design-direction,patterns-craft-floor}.yaml`
Checklist run: AM-2 five
Evidence read: the migration's op headers and all 15 rule ops in full · each view diff above · `patterns-code-minimalism.lazy-not-negligent` (the new pointer target) · record D1–D5
Pre-pass: `0 rejecting · 113 advisory`. Views equal replay. Budget effect is a shrink on every touched skill; see the skill-pair units.
- PASS intent stated: `intent:` names D1 and the template folds of D2–D5, and each hunk maps to one of them.
- PASS anchor: the header anchor is `2026-09-24 setup-product-agnostic D1`. The supersession of `authoring-constitution.module-mechanical-attachment`, anchored to adaptive-depth D7, carries its own anchor. The CLI `protected-exit` check is clean.
- PASS ID lifecycle: the supersede of `module-mechanical-attachment` and the tombstone of `s4-fail-safe` match D1 ("the fact-validation fail-safe goes with the dimension it guarded"). Rewords keep their ids. `GI-002`→`GI-001` is a template example id, forward-only per D4/Q8.
- PASS floor and fail survival: the floor rewords are `waivers-authored-not-skipped`, `no-accessibility-content`, `code-floor-in-view`, `floor-both-ways` and `every-principle-traces`. Each drops only module or D4.2 text, which record build step 5 rules moot. The "Essential Floor line carried by `patterns-code-minimalism`" claim holds: `lazy-not-negligent` names accessibility explicitly.
- PASS register: full register, consistent with sibling templates.
- Advisory: the region's "Amend via" line and the ledger's Amendment policy both list the six events. That puts one fact in two homes, a GI-017 tension, but the prior template had the same pattern.
VERDICT: PASS
Issues requiring fix: none

## Unit 4b — schema content `0020-setup-agnostic-rule-not-instance`

VALIDATE: schema content — `plugins/mochiko/migrations/0020-setup-agnostic-rule-not-instance.yaml` + view diffs `skills/{authoring-constitution,validation-constitution,review-governance-intent}.yaml`
Checklist run: AM-2 five
Evidence read: the migration in full · three view hunks · record D2 · `validation-constitution/references/QUALITY-CHECKLIST.md`, which holds the worked cases the reword cites
- PASS intent stated: mint, check and re-key are all named.
- PASS anchor: D2 on the header and on the minted rule.
- PASS ID lifecycle: one clean mint, `authoring-constitution.rule-not-instance`, in `sec.artifact`. The other two ops are rewords that keep their ids.
- PASS floor and fail survival: no floor touched. `status-vocabulary-and-criteria` keeps its adaptive-depth anchor and loses only the fact-profile criterion, per D1.
- PASS register: consistent. "(cases: the checklist)" resolves to the new No Product Instance section.
VERDICT: PASS
Issues requiring fix: none

## Unit 4c — schema content `0021-setup-agnostic-seven-dimensions`

VALIDATE: schema content — `plugins/mochiko/migrations/0021-setup-agnostic-seven-dimensions.yaml` + view diff `skills/review-governance-intent.yaml`
Checklist run: AM-2 five
Evidence read: the migration in full · two view hunks · record D3
- PASS intent stated · PASS anchor (D3) · PASS ID lifecycle: two rewords, ids kept. · PASS floor and fail survival: neither rule is a floor. · PASS register.
VERDICT: PASS
Issues requiring fix: none

## Unit 4d — schema content `0022-setup-agnostic-closed-event-set`

VALIDATE: schema content — `plugins/mochiko/migrations/0022-setup-agnostic-closed-event-set.yaml` + view diffs `skills/authoring-constitution.yaml`, `commands/architecture.yaml`
Checklist run: AM-2 five
Evidence read: the migration in full · the view hunks for `amend-preserves-verbatim`, `ledger-riders-and-trace-manifest` and `arch.dm-store-integrity-close` · record D4 and Q8
- PASS intent stated: the three changes are named.
- PASS anchor: D4.
- PASS ID lifecycle: three rewords, ids and anchors kept. `ledger-riders` is also reworded in `0019`; the net text is the D4 form.
- PASS floor and fail survival: the `dm-store-integrity-close` floor gains a clause and loses nothing.
- PASS register: advisory only. "a fact-profile module" in `amend-preserves-verbatim` reads less plainly than "a fact profile or an attached compliance module", the template's own wording.
VERDICT: PASS
Issues requiring fix: none

## Unit 4e — schema content `0023-setup-agnostic-setup-rule-set`

VALIDATE: schema content — `plugins/mochiko/migrations/0023-setup-agnostic-setup-rule-set.yaml` + view diffs `commands/{setup,specify,architecture}.yaml`, `skills/analysis-codebase.yaml`
Checklist run: AM-2 five
Evidence read: the migration in full · the four view diffs · `.mochiko/schema-views/commands/implement.yaml` lines 608–671 (`impl.design-landing` and the baseline-fold floor) · `.mochiko/schema-views/templates/design-baseline.yaml` diff · record D5 and build surface step 4
Pre-pass: `0 rejecting · 113 advisory`. One advisory bears on this unit: `allowlist names analysis-codebase.capability-signals-seed-feature-map: not a live rule ID (stale entry?)`.
- PASS intent stated: strike five, amend seven, re-point consumers.
- PASS anchor: D5 on the header and on the three supersessions of anchored rules. The anchors were impeccable D6 ×2 and plan-stage-utility, and D5 names each struck id.
- FAIL ID lifecycle: the dispositions are true for what they touch. Five retired, `feature-map-greenfield` reworded with `when:` nulled, and `fail.no-feature-map` enforces narrowed to the one live rule. A surviving rule's prose still references the retired node, though. `impl.design-landing` names "the setup leg (the truth part, the brownfield system-part seed, the scaffold)" as a `${designer_seat}` write path, contradicting `setup.design-scaffold-unconditional` and the `0019` template's writer sentence.
- PASS floor and fail survival: setup keeps 20 floors and 6 fails. `fail.no-feature-map` stays a `kind: fail` floor with a non-empty `enforces:`.
- PASS register: consistent with setup's register.
VERDICT: FAIL
Issues requiring fix:
1. Item: ID lifecycle, a prose reference to a retired node. Missing: `impl.design-landing` still names the retired setup leg as a `product-designer` write path. Fix: add a `reword-rule` op on `command/implement` `impl.design-landing`, in `0023` restamped or in a new `0024` under the D5 anchor, replacing the last sentence with the template's form. Suggested text: "The setup lead writes only its empty scaffold; ${designer_seat} is the only writer of its content, on two paths: a build-time baseline-delta.md entry, and this fold." Then regenerate the views. The implement command pair's render joins the re-audit.

## Unit 5a — skill pair `authoring-constitution`

VALIDATE: skill pair — `plugins/mochiko/skills/authoring-constitution/SKILL.md` + render
Checklist run: skill-pair criteria 1–12
Evidence read: `SKILL.md` diff · `.mochiko/schema-views/skills/authoring-constitution.yaml` diff · `.mochiko/strips/authoring-constitution.md` [v0.115.0] SKILL.md entry · render of all seven sections, swept for module and fact-profile residue
Pre-pass: `0 rejecting · 113 advisory`. The three `condition-coverage` advisories on this skill predate the wave. Budget by canonical snippet: body 7,696→7,219 (−477), description 481→461 (budget 602), render 22,194→21,919, payload 29,890→29,138 against 29,614. Under budget; the standing +276 overage clears.
- PASS scaffold headings and order: load-first section and seven `!` lines unchanged.
- PASS preserved responsibilities: four SKILL.md fragments leave. The strip quotes all four verbatim against the diff and keeps the engineering-module rows.
- PASS floor survival: `class: floor · 13 rules`, identical set.
- PASS independence: unchanged.
- PASS reserved-to-user: `sec.reserved` unchanged.
- PASS done-condition: n/a for a skill.
- PASS argued overage: none needed.
- PASS description: the edited value is 461 characters, ≤ 1,536, and drops only the D1 trigger.
VERDICT: PASS
Issues requiring fix: none

## Unit 5b — skill pair `validation-constitution`

VALIDATE: skill pair — `plugins/mochiko/skills/validation-constitution/SKILL.md` (untouched) + render
Checklist run: skill-pair criteria 1–12
Evidence read: `.mochiko/schema-views/skills/validation-constitution.yaml` diff · the `never-excess` stub (`extends: review-common.never-excess`) · render sweep · `.mochiko/strips/validation-constitution.md`
Pre-pass: `0 rejecting`. Payload 14,797→14,795 against 15,017. Body 3,280 and description 475 unchanged.
- PASS scaffold: SKILL.md untouched.
- PASS preserved responsibilities: `version-bump` drops "module attach/detach", which traces to the fact-profile module trigger (strip history lines 81–82). The event-(5) bump stays derivable from principle add or drop. `validation-result-block` keeps "module fragments" and "modules matched" for the engineering modules.
- PASS floor survival: `class: floor · 14`, identical set.
- PASS independence and reserved content: unchanged.
- PASS argued overage: none.
VERDICT: PASS
Issues requiring fix: none

## Unit 5c — skill pair `review-governance-intent`

VALIDATE: skill pair — `plugins/mochiko/skills/review-governance-intent/SKILL.md` + render
Checklist run: skill-pair criteria 1–12
Evidence read: `SKILL.md` diff · `.mochiko/schema-views/skills/review-governance-intent.yaml` diff · `.mochiko/strips/review-governance-intent.md` [v0.115.0] · render sweep for risk, fact and module terms (none left)
Pre-pass: `0 rejecting`. Body 3,160→3,141, description 483 (budget 604), payload 16,266→16,200 against 16,274.
- PASS scaffold · PASS preserved responsibilities: the two body fragments are stripped verbatim. · PASS floor survival: `class: floor · 16`, identical. · PASS independence · PASS reserved · PASS overage: none.
VERDICT: PASS
Issues requiring fix: none

## Unit 5d — skill pair `analysis-codebase`

VALIDATE: skill pair — `plugins/mochiko/skills/analysis-codebase/SKILL.md` + render
Checklist run: skill-pair criteria 1–12
Evidence read: `SKILL.md` diff · `.mochiko/schema-views/skills/analysis-codebase.yaml` diff · `.mochiko/strips/analysis-codebase.md` [v0.115.0] · `0023` tombstone and reword ops
Pre-pass: `0 rejecting`, plus the stale-allowlist warning. Body 4,726→4,698, render 9,674→9,438, payload 14,400→14,136 against 13,776. That leaves +360 over, on the standing ruled-HOLDS overage recorded at [v0.114.0] as +624. The overage falls and no new growth rides this edit, so the ruled overage holds.
- PASS scaffold · PASS preserved responsibilities: the Output line's seeding phrase is stripped verbatim. Stack detection, the design-system facts and the floor read are kept, per D5's Inputs line. · PASS floor survival: `class: floor · 4`, identical. · PASS independence · PASS reserved · PASS overage.
VERDICT: PASS
Issues requiring fix: none

## Unit 5e — skill pair `patterns-design-direction` (rules only)

VALIDATE: skill pair — `SKILL.md` (untouched) + render
Evidence read: view diff (`no-accessibility-content`) · `platform-routing` text, whose no-value fallback exists · `patterns-code-minimalism.lazy-not-negligent`
Pre-pass: `0 rejecting`. Payload 16,828→16,763 against 16,828.
- PASS all items: one floor reword, ruled by D1/D6 and record build step 4. `class: floor · 3`, identical.
VERDICT: PASS
Issues requiring fix: none

## Unit 5f — skill pair `patterns-craft-floor` (rules only)

VALIDATE: skill pair — `SKILL.md` (untouched) + render
Evidence read: view diff (`code-floor-in-view`)
Pre-pass: `0 rejecting`. Payload 14,266→14,237 against 14,266.
- PASS all items: one floor reword drops the module home. `class: floor · 2`, identical.
VERDICT: PASS
Issues requiring fix: none

## Unit 5g — skill pair `patterns-plan-minimalism` (rules only)

VALIDATE: skill pair — `SKILL.md` (untouched) + render
Evidence read: view diff (`floor-both-ways`) · the `0019` op
Pre-pass: `0 rejecting`. Payload 10,696→10,664 against 10,824.
- PASS all items: the floor reword drops "compliance-module obligations" only. The floor-both-ways duty and its plan-structure-yagni anchor are kept. `class: floor · 2`, identical.
VERDICT: PASS
Issues requiring fix: none

## Unit 6a — prose primitive `skills/mochiko/SKILL.md` (router)

VALIDATE: prose primitive — `plugins/mochiko/skills/mochiko/SKILL.md`
Checklist run: coherence · preserved responsibilities
Evidence read: file diff · `.mochiko/strips/mochiko.md` [v0.115.0]
Pre-pass: `0 rejecting`. Body 47,270→47,148, unbudgeted. Description 206, unchanged.
- PASS coherence: the setup row matches D3/D5 and the new `setup.md` goal. The `product-designer` row matches the template's content-writer paths at the fold.
- PASS preserved responsibilities: three fragments stripped verbatim. The ratify, grade and accept sequence is kept.
VERDICT: PASS
Issues requiring fix: none

## Unit 6b — prose primitive `templates/constitution-modules/release-gates.md`

VALIDATE: prose primitive — `plugins/mochiko/templates/constitution-modules/release-gates.md`
Evidence read: file diff · `.mochiko/strips/release-gates-module.md` (new file)
- PASS coherence: three checks remain. · PASS preserved responsibilities: one check is stripped verbatim under D1/D6/S9.
VERDICT: PASS
Issues requiring fix: none

## Unit 6c — prose primitives `authoring-constitution/references/` (six files)

VALIDATE: prose primitives — `INTERROGATION-AGENDA.md` · `COMPLIANCE-MODULES.md` · `ESSENTIAL-FLOOR.md` · `catalog/README.md` · `catalog/universal-floor.md` · `catalog/backend-service.md`
Evidence read: all six diffs · the eight [v0.115.0] entries in `.mochiko/strips/authoring-constitution.md`, each checked verbatim against the removed lines
- PASS coherence: seven dimensions with stable numbers. Dimension 3 reads code or `Scope:` first. Dimension 8 carries trust vectors and the S10 SLO clause. The D2 dimension-9 test and the Handed off route are in. The struck-dimension clause and the six-event Amend bullet with the forward-only legacy clause match `0019`'s templates. `COMPLIANCE-MODULES.md` is retired-from-setup with its content kept for OQ1.
- PASS preserved responsibilities: every removed passage appears verbatim in a supersession entry with a ruling and a Kept-deliberately line. The pure additions are D2/D3/D4 and ride the decision row.
- Advisory: `catalog/backend-service.md` BE-DEP dropped the blank line between `**When kept:**` and `**Content:**` (lines 110–111). Nothing parses it; cosmetic.
VERDICT: PASS
Issues requiring fix: none

## Unit 6d — prose primitives `validation-constitution/references/` (two files)

VALIDATE: prose primitives — `QUALITY-CHECKLIST.md` · `ANTI-PATTERNS.md`
Evidence read: both diffs · three [v0.115.0] entries in `.mochiko/strips/validation-constitution.md`
- PASS coherence: the No Product Instance section's worked cases are D2's sort, generalized. Floor Accounting is consistent with `0019`'s ledger shape. The amend-route item points at the ledger's event list rather than restating it.
- PASS preserved responsibilities: all eight checklist lines and two amend and semver fragments are stripped verbatim. The ANTI-PATTERNS row is recorded as an out-of-list consumer.
VERDICT: PASS
Issues requiring fix: none

## Unit 6e — prose primitive `patterns-vertical-tdd/references/TEST-GRAMMAR.md`

VALIDATE: prose primitive — `TEST-GRAMMAR.md`
Evidence read: file diff · `.mochiko/strips/patterns-vertical-tdd.md` [v0.115.0] · the `design-baseline` Accessibility section, which is the new pointer target
- PASS coherence: the assert routes via the baseline's pointer, which now names the Essential Floor line, with the floor-line fallback kept. · PASS preserved: the fragment is stripped verbatim.
VERDICT: PASS
Issues requiring fix: none

## Unit 6f — prose primitive `review-brainstorm/references/EXTERNAL-CLAIMS.md`

VALIDATE: prose primitive — `EXTERNAL-CLAIMS.md`
Evidence read: file diff · `.mochiko/strips/review-brainstorm.md` [v0.115.0]
- PASS coherence · PASS preserved: the pointer is stripped verbatim and the exclusion bullet is kept.
VERDICT: PASS
Issues requiring fix: none

## Unit 6g — prose primitive `review-feasibility/references/FEASIBILITY-LENS.md`

VALIDATE: prose primitive — `FEASIBILITY-LENS.md`
Evidence read: file diff · `.mochiko/strips/review-feasibility.md` [v0.115.0] · `review-common.never-excess` render, which now carries the same wording
- PASS coherence: consistent with the common block. · PASS preserved: the fragment is stripped verbatim.
VERDICT: PASS
Issues requiring fix: none

## Outcome lines

- audit: setup (command pair) · G1 · opus · 4 files · 1 rounds · 0 blocking
- audit: specify (command pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: architecture (command pair) · G1 · opus · 4 files · 1 rounds · 0 blocking
- audit: 0019-setup-agnostic-modules-out (schema content) · G1 · opus · 10 files · 1 rounds · 0 blocking
- audit: 0020-setup-agnostic-rule-not-instance (schema content) · G1 · opus · 4 files · 1 rounds · 0 blocking
- audit: 0021-setup-agnostic-seven-dimensions (schema content) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: 0022-setup-agnostic-closed-event-set (schema content) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: 0023-setup-agnostic-setup-rule-set (schema content) · G1 · opus · 7 files · 1 rounds · 1 blocking
- audit: authoring-constitution (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: validation-constitution (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: review-governance-intent (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: analysis-codebase (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: patterns-design-direction (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: patterns-craft-floor (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: patterns-plan-minimalism (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: mochiko router (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: release-gates module (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: authoring-constitution references (prose) · G1 · opus · 7 files · 1 rounds · 0 blocking
- audit: validation-constitution references (prose) · G1 · opus · 3 files · 1 rounds · 0 blocking
- audit: TEST-GRAMMAR (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: EXTERNAL-CLAIMS (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
- audit: FEASIBILITY-LENS (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking

## Round 2 — pre-pass (re-run first-hand)

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` gives
  `mochiko-cli migrate validate · 0 rejecting · 113 advisory`, exit 0, and zero `warning:` lines.
  The stale-allowlist warning is gone after the two-line removal in
  `scripts/similar-rules-allowlist.yaml`, so that advisory closes.
- `mochiko-cli migrate status` gives `sequences 1..23 (23 migrations)` and
  `state sha256:156ab089… · 80 documents · 1135 rules`.
- `git diff --stat -- .mochiko/schema-views` gives `15 files changed, 221 insertions(+), 265
  deletions(-)`. The one new file is `commands/implement.yaml`.
- Views emitted again into scratch and diffed against `.mochiko/schema-views/`: identical.
- Implement preamble pins, `floors:` line and section counts are identical to the pre-wave
  `0001`–`0018` replay: `kind: fail · 15 rules` · `class: floor · 37 rules` · sections
  16 · 9 · 47 · 9 · 15 · 15. The seven end lines agree with the preamble.

## Unit 4e (round 2) — schema content `0023-setup-agnostic-setup-rule-set`

VALIDATE: schema content — `plugins/mochiko/migrations/0023-setup-agnostic-setup-rule-set.yaml` (restamped, 19 ops) + its view diffs, now including `commands/implement.yaml`
Checklist run: the failed item (ID lifecycle) plus what the appended op could break: anchor, register, and the view diff
Evidence read: the migration header and its appended `reword-rule` on `command/implement` `impl.design-landing` · `git diff -- .mochiko/schema-views/commands/implement.yaml` · the rendered `impl.design-landing` and a render sweep of all six implement sections for setup-leg, seed and bootstrap claims · `.mochiko/schema-views/templates/design-baseline.yaml` writer sentence
Pre-pass: `0 rejecting · 113 advisory`. Views equal replay.
- PASS ID lifecycle: `impl.design-landing` keeps its id, `class: must`, `when: {ux_bearing: present}` and its anchor. Only the last sentence changes, and everything before it is verbatim in the view diff. The new sentence matches the `0019` template and `setup.design-scaffold-unconditional`: the lead writes only the empty scaffold, and the designer seat writes content on two paths. No surviving rule text in `.mochiko/schema-views/` still names the retired setup leg.
- PASS anchor: the header anchor stays `2026-09-24 setup-product-agnostic D5`, and the reworded rule keeps `2026-09-19 impeccable-design-integration D7`. A reword is not an exit, so no protected-exit anchor is owed. The hash is restamped and the log replays.
- PASS register: consistent with the rule's own voice.
- PASS unchanged items: intent, and floor and fail survival, stand as graded in round 1. The appended op touches no floor.
VERDICT: PASS
Issues requiring fix: none

## Unit 8 — command pair `implement`

VALIDATE: command pair `implement` — `plugins/mochiko/commands/implement.md` (unchanged, `git diff --quiet`) + `mochiko-cli rules implement` (seven blocks)
Checklist run: canonical-scaffold criteria 1–11, run-command branch of item 7 plus the `implement` additionally clause
Evidence read: `plugins/mochiko/commands/implement.md` (frontmatter, headings, Rules block, the full Adaptive Goal Protocol) · render of preamble and all six sections · `.mochiko/schema-views/commands/implement.yaml` diff · `impl.absent-surfaces` and `impl.design-absent-baseline-seed` (the paths the setup cut feeds)
Pre-pass: `0 rejecting · 113 advisory`. Pins and floor set are identical to pre-wave. Commands are unbudgeted in the ledger, so no char budget applies.
- PASS scaffold headings and order: the required frontmatter keys, `# Implement — …`, Identity & Mission, the Rules block with seven `!` lines, and Entry, Goal, Not done are all unchanged.
- PASS preserved responsibilities: the only change is the writer sentence of `impl.design-landing`, ruled by D5. The fold duties, the grading seat and the when-gate are kept verbatim.
- PASS floor survival: 37 floors and 15 fails, identical. `impl.sound-loop-floor` is present, as required on this DM-chartered command.
- PASS independence: the landing write is still graded by the landing verification seat, and no seat grades its own row.
- PASS reserved-to-user: `sec.reserved` count is unchanged at 9.
- PASS done-condition, run branch: Entry carries the gating and the neither-source routing. The Goal is fixed. Not done cites the CLI pin with its halt clause and no hard-coded count.
- PASS implement additionally: the run-open confirmation names the batch and scope type and restates both attempt bounds at their only redeclaration point. It presents the verdict and states the done condition, and the run closes at the existing final-acceptance gate (`impl.gate-final-acceptance`). No new ceremony.
- PASS setup-cut consumers: `impl.absent-surfaces` still offers `/mochiko:setup` for a missing brownfield analysis, which setup still produces. `impl.design-absent-baseline-seed` seeds absent baselines from delivered code, so no path relied on the retired setup bootstrap.
VERDICT: PASS
Issues requiring fix: none

## Outcome lines — round 2

- audit: 0023-setup-agnostic-setup-rule-set (schema content) · G1 · opus · 4 files · 2 rounds · 0 blocking
- audit: implement (command pair) · G1 · opus · 3 files · 1 rounds · 0 blocking

## Unit 7 — crate tests + eval kits (S3's write set, 21 files)

VALIDATE: crate + evals — `crates/mochiko-cli/tests/{fidelity,validate,matrix_similar}.rs` · six template fixtures under `crates/mochiko-cli/tests/fixtures/template/` · `evals/plan/setup/{observable.yaml,evals.json}` · `evals/{validation-constitution,review-governance-intent,review-brainstorm,review-specifications}/{rules.json,rekey.md}` · `evals/{validation-constitution,review-governance-intent}/evals.json`
Checklist run: independent non-author code review per `.claude/rules/mochiko/rust-cli.md` for the crate files; each kit's `rekey.md` precedent for the eval kits. No strips are owed under `evals/**`.
Evidence read: all three `.rs` diffs in full · `render.rs` fixture-comparison procedure · `tests/fixtures/genesis-corpus/.mochiko/provenance.yaml`, searched for each of the wave's eight retired ids · `scripts/similar-rules-allowlist.yaml` rows the comments cite · all four `rules.json` and `rekey.md` diffs · the three `evals.json` diffs, field by field against HEAD · the three review-governance-intent fixture syntheses, for the re-keyed status grounds · `evals/plan/README.md` · `evals/plan/setup/observable.yaml` diff
Pre-pass (run first-hand):
- `cargo test --all`: 17 result lines, all `test result: ok`, 0 failed. Among them: fidelity 17 passed · validate 100 passed · matrix_similar 48 passed · render 39 passed · replay 62 passed · cli 51 passed.
- `cargo fmt --all --check`: clean. `cargo clippy --all-targets -- -D warnings`: `Finished`, no warnings.
- `uv run evals/run.py command check-rubric setup` gives `rubric OK: 33 observable, 8 out-of-instrument, 41 total`.
- `git diff --stat -- crates evals` gives `21 files changed, 306 insertions(+), 122 deletions(-)`. `git diff --quiet -- evals/contract` is clean. No `src/`, `Cargo.toml` or `Cargo.lock` change.
- PASS pins and comments only, figures derived from the landed log: census 332/803/1135/264/119/36 reconciles with `migrate status` (1135 rules) and with the eight retirements and one mint the comments list. The pointer pin 82 equals validate's `pointer resolution: 82 checked`, and the two retired rules are the only ones that pointed at `references/COMPLIANCE-MODULES.md`. The corpus similarity pin 1135 / 178,230 / 0 / 184 equals validate's stats line. The −2/+1/−1 explanation names allowlist rows that exist at lines 224, 244 and 242, plus the removed `map-mirror` row.
- PASS `RETIRED_SIDECAR_ANCHORS` 1 → 3: of the wave's eight retired ids, exactly two appear in the sidecar fixture, `setup.baselines-bootstrap` and `authoring-constitution.module-mechanical-attachment`. Both are absent from the live state, and the replay test enforces that absence. The 0003 CLI-form table dropping `setup.feature-map-brownfield` matches its tombstone.
- PASS fixture recapture: each of the six fixtures equals the live `mochiko-cli template <name> [--check]` output minus its trailing `schemas:` line, which is the comparison `render.rs` performs. Each diff carries only `0019`'s template text.
- PASS `rules.json` parity: by script against the seven-block render, all four kits have zero text, class or section mismatches, and floor sets equal the render's `floors:` line. The id sets are equal except `review-specifications` (30 vs 31, missing `sf-direction-checks`), the pre-existing H6 gap, ruled out of this wave and disclosed in its `rekey.md`. Every `tempts` id resolves: 68 in validation-constitution, 76 in review-governance-intent. The moved entries equal the entries each `rekey.md` names, and only `rule` fields moved. The `findings-through-leads-pen` drift repair is disclosed.
- PASS `rekey.md`: zero removed lines in all four. Each adds one `## Re-key 2026-09-24 — setup-product-agnostic` section in the file's own shape: JSON stamp, counts, text re-keyed, invariants.
- PASS goldens: in the two skill kits only `expected_output` moved. Assertions, prompts, tempts and fixtures are byte-identical, and the fixture directories are untouched. No retired term survives in any `expected_output`; the only hits are in frozen prompts. The review-governance-intent status sentences use the landed `critical-gaps` ground ("a principle intent restating a product instance, not the rule"), and the grounds they cite exist in the fixtures: tidewell GI-011, halyard GI-011, quillon GI-012. The plan/setup kit also rewrites `assertions`, which `evals/plan/README.md` says nothing reads, so no instrument moves.
- PASS `observable.yaml`: the three struck ids are gone. The whys are re-keyed against landed text: `blind-map-dispatch` project name, and `feature-map-greenfield` and `map-never-overwrite` on every path. The three pre-existing uncovered ids are added as observable (`design-scaffold-unconditional`, `validate-seat-form`, `gate-loop-bound`). The unplanted-branch note is edited in place, which is correct for that list.
- Advisory: the validation-constitution g3 golden reads the planted pci-dss waiver row as a product instance under the new check. That reading is defensible under the checklist's named-data case, but it is the loosest re-key of the set.
VERDICT: PASS
Issues requiring fix: none

- audit: crate tests + eval kits (unit 7) · G1 · opus · 21 files · 1 rounds · 0 blocking
