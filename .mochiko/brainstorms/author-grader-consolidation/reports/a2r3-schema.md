---
report: review
round: 3
---

## VALIDATE

VALIDATE: schema content — `plugins/mochiko/migrations/0008-gate-form.yaml` plus its regenerated
view diff (`.mochiko/schema-views/commands/setup.yaml`, `common/common.yaml`,
`skills/patterns-model-tiering.yaml`, new `skills/validation-primitive-edit.yaml`). 5 files.

Checklist run: the AM-2 five (GI-004 as re-expressed at v3.0.0, `.mochiko/memory/governance-ledger.md`)
— intent stated · anchor present where required · ID lifecycle right · floor and fail survival ·
register. Plus ruling fidelity against `.mochiko/brainstorms/author-grader-consolidation/record.md`
D2, D3 (C1 fold), D4, D5, D6, D7, D9, D11 and build-surface item 1.

Evidence read: `plugins/mochiko/migrations/0008-gate-form.yaml` · `git diff HEAD --
.mochiko/schema-views/` · `.mochiko/schema-views/skills/validation-primitive-edit.yaml` ·
`.mochiko/schema-views/common/common.yaml` · `.mochiko/schema-views/commands/setup.yaml` ·
`.mochiko/schema-views/skills/patterns-model-tiering.yaml` ·
`.mochiko/schema-views/labels/command-labels.yaml` · `.mochiko/schema-views/labels/skill-labels.yaml` ·
`.mochiko/schema-views/common/skill-review-common.yaml` ·
`.mochiko/schema-views/skills/validation-constitution.yaml` ·
`plugins/mochiko/migrations/README.md` · `.mochiko/memory/governance-ledger.md` GI-004 ·
`.mochiko/memory/primitive-cost-budgets.md` · `.mochiko/brainstorms/author-grader-consolidation/record.md`
D1–D11 + build surface · `.mochiko/brainstorms/cli-schema-delivery/record.md` (AM-2 five wording).

### Pre-pass — run first-hand, quoted

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`:

```
pointer resolution: 84 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 (none)
mochiko-cli migrate validate · 0 rejecting · 105 advisory
budget · skill/validation-primitive-edit · - · 21 rules · 5314 resolved characters of rule text
```

`mochiko-cli migrate status --plugin-root plugins/mochiko`:

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..8 (8 migrations)
state sha256:7b58e8c16be44a8c8b22f64de5ba70bbb50d1db2e3da0c65ca9e56add4988343 · 74 documents · 1067 rules
```

`mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views` then
`git status --short .mochiko/schema-views`:

```
mochiko-cli views emit · 74 documents · .mochiko/schema-views
 M .mochiko/schema-views/commands/setup.yaml
 M .mochiko/schema-views/common/common.yaml
 M .mochiko/schema-views/skills/patterns-model-tiering.yaml
?? .mochiko/schema-views/skills/validation-primitive-edit.yaml
```

Tree sha256 over all 74 views is identical before and after the emit (`6d0ad137…`). No new change;
the committed diff is the replay.

`mochiko-cli rules validation-primitive-edit --section preamble --plugin-root plugins/mochiko`:

```
pins
- class: floor · 11 rules
floors: validation-primitive-edit.author-grader · validation-primitive-edit.plain-seat-explicit-tier ·
validation-primitive-edit.rendered-contract-only · validation-primitive-edit.from-file-floor ·
validation-primitive-edit.pre-pass-first-hand · validation-primitive-edit.binary-verdict ·
validation-primitive-edit.default-fail · validation-primitive-edit.tamper-proof-clause ·
validation-primitive-edit.gate-loop-bound · validation-primitive-edit.evidence-floor ·
validation-primitive-edit.second-fail-user
```

Pin 11 matches the 11 named floors. Six sections rendered, all non-empty: `sec.independence` 4 ·
`sec.scope` 3 · `sec.inputs` 3 · `sec.verdict` 7 · `sec.output` 3 · `sec.reserved` 1 = 21 rules,
agreeing with the pre-pass budget line.

`mochiko-cli rules setup --section setup.sec.roles`: 5 rules, `setup.validate-seat-form` present.
`mochiko-cli rules setup --section setup.sec.boundaries`: 7 rules, `setup.gate-loop-bound` resolving
`common.gate-loop-bound`'s full text.

Char-budget measurement, run first-hand — the seven rendered blocks summed:

```
preamble 1966 · independence 1974 · scope 1383 · inputs 1279 · verdict 2679 · output 1566 · reserved 672
RENDER TOTAL: 11519
```

Exactly the 11,519 render component of the seeded row in `.mochiko/memory/primitive-cost-budgets.md`,
against the same log state (`7b58e8c1…` · 74 documents · 1067 rules · floor pin 11 · binary 0.2.0).
No schema-side overage; no argued overage is owed.

### AM-2 five

PASS · **Intent stated** — the header `intent:` is one line naming all four changes: the
common-block loop bound, setup's bound and validate-seat rule, the persona-less grader tier pin,
and the `validation-primitive-edit` skill. It matches what the `changes:` list does, and nothing in
the file falls outside it.

PASS · **Anchor present where required** — header `anchor: 2026-09-19 author-grader-consolidation D7`,
well-formed under the log README's `YYYY-MM-DD <session-slug> D<n>` format. The file carries no
`supersede-rule`, no `tombstone-rule`, and no `set-rule-field` lowering `class`, `kind`, or
`anchor`, so the hard set demands no anchor at all; one is carried anyway. Every rule executing a
named decision carries its own anchor (D2, D3, D6, D7, D9, D11).

PASS · **ID lifecycle right** — every op is a mint or an import; there is no reword, move, split,
merge, or tombstone, so no id-survival or parent-record obligation arises. All four rule ids and
the document are absent from HEAD (`git grep` over the committed views returns 0 for
`common.gate-loop-bound`, `setup.gate-loop-bound`, `setup.validate-seat-form`, and
`patterns-model-tiering.persona-less-grader-pin`; `validation-primitive-edit.yaml` is untracked).
The filename prefix `0008` agrees with `sequence: 8`, and `migrate status` shows 1..8 with no gap
or collision. `section:` is carried on the three command and skill mints, correctly omitted on the
common-library mint.

PASS · **Floor and fail survival** — the view diff is +34/−0 across three files, with zero deletion
lines. No `class: floor` rule and no `kind: fail` rule left the corpus. The change is purely
additive and raises protection: four new floors ship (`setup.gate-loop-bound`,
`patterns-model-tiering.persona-less-grader-pin`, and 11 inside the new skill, four of those
promoting `review-common` rules to `class: floor`). Raising protection needs no authority under the
log README.

PASS · **Register** — registry limb: every label used is already live, so no `registry-add` is
owed. Command side `user-gate`, `independence`, `seats`; skill side `boundary`, `independence`,
`binding`, `fence`, `evidence`, `verdict`, `reporting`, `user-gate`. Prose limb: the new rule text
is written in the validation family's impersonal third person, matching `validation-constitution`'s
own voice rather than the second-person review-family register.

### Ruling fidelity

PASS · **D2** — `validation-primitive-edit.gate-job` names the gate as a binary the lead cannot
ship past, sites it at the shipped `plugins/mochiko/` primitive before the `plugin.json` bump
(GI-004), and routes the other two jobs away: findings a user rules downstream to the review
family, setup's governance surface set to `mochiko:validation-constitution`.

PASS · **D3 and the C1 fold** — the posture survives as `default-fail`, the tamper-proof clause as
`tamper-proof-clause` (no evidence-read line is FAIL, automatically). The I3 fold lands verbatim in
`pre-pass-first-hand`, including "a pre-pass result quoted from the brief is not evidence" and the
never-re-derive clause. The I1 fold lands as three unit-keyed rules, and `judgment-items-schema`
names the AM-2 five in the ledger's own order. The mechanical items are pushed to the pre-pass and
read from its output. C1 is honored twice over: `0008` carries no op at all against
`skill/validation-constitution`, so its completeness floors stand unchanged, and `gate-job` states
the routing.

PASS · **D4** — the number lives in one home. Grep over all 74 emitted views: "one re-audit" and
"one fix" hit `common/common.yaml:50` only; "second FAIL" hits `common/common.yaml:51` and
`skills/validation-primitive-edit.yaml:202`, the second being `second-fail-user`, whose ruled
subject it is under D6; "repeat FAIL" hits `validation-primitive-edit.yaml:194`, the title of the
section holding `second-fail-user` and nothing else; "one round" has no hits anywhere. That title
carries no number and matches the corpus convention — all 37 reserved-section titles in the views
are subject-naming phrases of the same shape. `setup` carries the bound by `extends:` and is its
sole extender, per OQ1.

PASS · **D5** — `common.gate-loop-bound` opens "A gate verdict of FAIL", scoping itself by its own
words. Both consumers are gate sites: setup's validate step and the primitive-edit gate. `0008`
carries no op against `command/implement`, so `attempt_bound_cycle` and `gap_rework_bound` are
untouched, and no input-job loop is bounded anywhere in the file.

PASS · **D6** — `common.gate-loop-bound` carries all four limbs: the number one, the same grader
seat resumed (I5), the delta read, and the second-FAIL halt to the user with both fix lists, fix
again or drop. `second-fail-user` carries the C2 strike in the record's own words — a bump carrying
an unfixed FAIL is a bump without audits PASS, and overruling a grader rides the ledger's waiver
path, never a run's own call.

PASS · **D7** — `plain-seat-explicit-tier` (persona-less fresh seat, explicit `model:` alias, the
tier produced at and never below, `opus` where the lead made the edit, an omitted alias a floor
miss whatever the verdict says), `rendered-contract-only` (the render is the contract; a
hand-written contract section is a floor miss "on the same terms as an omitted `model:` alias", the
record's phrasing), `brief-carries-unit` (unit, paths, pre-pass commands, nothing else), and
`patterns-model-tiering.persona-less-grader-pin` closing F12. The `validator` retirement is
correctly absent — D10 puts it at wave 3, and no op touches the persona.

PASS · **D9** — `outcome-line` reproduces the grammar character for character:
`audit: <unit> · <seat> · <tier> · <n> files · <n> rounds · <n> blocking[ · cost: $<x>]`. Both
landing homes are named, the wave's `build-log.md` entry and the `.mochiko/decisions/` record for
an ad-hoc defect close. The cost condition is faithful: present only where the audit ran as a
launched session, read from that session's own reported total.

PASS · **D11** — `one-seat-per-wave` carries the ruling near-verbatim: one seat over every unit,
independence resting on authorship rather than count, a per-unit verdict block and outcome line
tagged with the seat, and the two-seat split admitted only on context with the lines saying so.

PASS · **Build-surface item 1** — all five bullets delivered. `common.gate-loop-bound` sits in the
command common block with the number in its own text and labels `user-gate`, `independence`,
extended by `setup` alone. The `patterns-model-tiering` rule is present. Setup's validate-step rule
points its loop at `setup.gate-loop-bound` and its grader at a plain fresh seat, states
`validation-constitution`'s floors unchanged, and removes no persona name (the diff deletes
nothing). The new skill sits on the review six-set with the floor-count pin, carrying the D3
judgment sets, the tamper-proof clause, read-the-files-never-the-report, the fix-list form, the
pointer at `common.gate-loop-bound`, and the D11 shape. The M4 sweep is armed:
`common.gate-loop-bound`'s own text names `.claude/rules/mochiko/primitive-edits.md` and
`validation-primitive-edit.gate-loop-bound` as its consumers. Validate is green and the views
re-emit idempotently.

PASS · **Nothing unpaid-for added** — every rule traces to a ruling or to standing gate content.
The two candidates were checked and both are paid for. "An argued overage where the pre-pass shows
one" is the existing char-budget clause of the gate ceremony, and the judgment it names — whether
the justification holds — is genuinely a grader's. "For a prose primitive the items are coherence
and preserved responsibilities" is build-surface item 5's own wording for the `primitive-edits.md`
unit.

VERDICT: PASS

Issues requiring fix: none.

## Notes of note

Three PASS items rest on a reading, none blocking. Scaffold headings sit in the judgment list
though D3's prose called them mechanical: build-surface item 2 binds the split to what `migrate
validate` actually checks, and the CLI never reads a primitive's `.md`.

`validation-primitive-edit.gate-loop-bound` restates the delta-read and no-raise clauses beside its
citation. Not a D4 miss — the number is single-homed by grep, D3 limb 3 grants the delta read at
this gate, and build-surface item 1 names this rule a citer so the M4 sweep covers it. The three
review skills extending `review-common.its-command-states-them` each have a command; this has none.

Four `class: must` rules carry no anchor and are unprotected: `unit-keyed`, `brief-carries-unit`,
`judgment-items-prose`, `verdict-block`. Corpus-normal; the first and last carry D3/D7 content.

## Outcome

audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 5 files · 3 rounds · 0 blocking
