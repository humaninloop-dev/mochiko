---
report: review
round: 3
seat: w2-gate-audit
tier: opus
wave: human-readable-ids wave 2 (plugin minting) — schema content, skill pairs, prose primitives
units: 32 of 32
verdict: 32 PASS · 0 blocking (P1 and P2 re-confirmed at round 2 after their advisory fixes; S4 and S5 added at round 3)
kit: eval kit changes PASS (non-gate read)
---

## Notes of note

- Every unit passes; nothing blocks the wave. The advisories below are non-gating. Each one names its fix.
- The two advisories with the most weight: `report-format.md` rule 5's examples omit the owner qualifier the IDs section requires for cross-file cites (P2), and the IDs section's quote rule does not mention that `ids` unmasks quotes in `.html` files (P1).
- Contract: rendered first-hand, byte-identical to the lead's `vpe-render-1-49.txt`. The brief also paraphrased `one-seat-per-wave`, which `brief-carries-unit` does not allow. Grading ran against the render only.
- `mochiko-cli rules skill-review-common` exits with "no command or skill named". The bound came from the `setup.gate-loop-bound` render, which extends `common.gate-loop-bound`: one fix, one re-audit, and a second FAIL goes to the user.
- The strips home declares no template and no bound (`mochiko-cli home .mochiko/strips`), so `check` allows any body. Going around the hook therefore skipped no check: the first-hand pre-pass ran the same check the hook would have.
- The installed `mochiko-cli` 0.3.0 has no `ids` subcommand; only `target/debug` carries it. The IDs section names `ids rename`/`rekey`, so H-L1, a binary release, is a wave-4 precondition.
- The write hook denied one probe because its scratch path ended in `.mochiko/memory/governance-intent.md`. I did not retry it, so whether `ids rename` needs `--plugin-root` is untested.

## Floor read-back and seat

The preamble's `class: floor` pin prints **11** rules. Ids: `author-grader` · `plain-seat-explicit-tier` ·
`rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` · `binary-verdict` · `default-fail` ·
`tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` · `second-fail-user`.

Seat `w2-gate-audit`: plain, persona-less, `model: opus` (runtime `claude-opus-5-5`), and author of nothing in any unit. One seat took
all 30 units, and every file fit this context, so no split was needed. Render command:
`for s in preamble validation-primitive-edit.sec.{independence,scope,inputs,verdict,output,reserved}; do mochiko-cli rules validation-primitive-edit --section $s --plugin-root plugins/mochiko; done`.
`diff` against `scratchpad/vpe-render-1-49.txt` printed nothing (IDENTICAL). Binary `mochiko-cli 0.3.0 · grammar 1..2`.

## Pre-pass — validate, status, views

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`, run first-hand:

```
pointer resolution: 88 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1175 · in-kind pairs scored: 192052 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory
```

`mochiko-cli migrate status --plugin-root plugins/mochiko`:

```
log plugins/mochiko/migrations · grammar 2 · sequences 1..49 (46 migrations)
state sha256:f1d8ce9cee6da51d0b4bcff2fd18d3994f6527df810ff1b70a8e4264513fdb3b · 87 documents · 1175 rules
```

Views ≡ replay. `mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>` printed `87 documents`, and
`diff -r` against `.mochiko/schema-views/` came back empty. I also emitted a 0001–0045 log copy through `--log-dir`, and it equals `git show HEAD:` for all
28 changed views. The view diff is therefore exactly 0046 + 0047 + 0049. A rule-node comparison of the two emits (pyyaml)
found: 0 gone; 8 new (`review-common.joined-ids`, 5 `*.joined-ids` stubs, `arch.grader-checks-joined-ids`,
`feat.grader-checks-joined-ids`); 14 changed, `text` only (class, kind, labels and anchor untouched); 383 floor/fail rules
survive with class and kind unchanged.

## Pre-pass — strips check (23 files)

`mochiko-cli check --path .mochiko/strips/<f>.md --content - --plugin-root plugins/mochiko < <f>`, run on each of the 23
files: `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}` × 23. Read-back: every file
carries exactly one `## [v0.118.0]` supersession-by-ruling entry with Disposition, Tier failed `n/a — supersession by ruling`
and Content (superseded). Kept deliberately and Consumers assessed appear where `.mochiko/strips/README.md` asks for them.
`evolution-notes-module.md` is new and carries the `*-module.md` header.

## Pre-pass — char-budget measurement

Method: the canonical snippet from `.mochiko/memory/primitive-cost-budgets.md` for body and `description:`, plus the characters
of each block the `SKILL.md` `!` lines deliver (`mochiko-cli rules <skill> --section <id>`, stdout+stderr). Script:
`scratchpad/gate-w2/measure.py`. Calibration: against a 0001–0045 log copy, five rows reproduce their ledger payloads exactly
(`review-brainstorm` 14,222 = 3,089 + 11,133; `authoring-epic` 14,104; `authoring-feature-map` 22,698; `review-sufficiency`
16,621; `patterns-sound-loop` 12,596). Pre = HEAD body on log 1..45 · Now = tree body on log 1..49.

| skill | budget | pre | render Δ | body Δ | now | standing |
|---|---|---|---|---|---|---|
| review-governance-intent | 16,274 | 16,200 | +571 | 0 | 16,771 | **+497, new** |
| validation-constitution | 15,017 | 14,795 | +570 | 0 | 15,365 | **+348, new** |
| review-plan-artifacts | 18,013 | 17,704 | +568 | 0 | 18,272 | **+259, new** |
| review-specifications | 16,174 | 16,790 | +568 | 0 | 17,358 | +1,184 (was +616) |
| review-brainstorm | 12,824 | 14,222 | +564 | 0 | 14,786 | +1,962 (was +1,398) |
| authoring-requirements | 12,373 | 12,633 | +145 | +96 | 12,874 | +501 (was +260) |
| authoring-epic | 14,062 | 14,104 | +346 | 0 | 14,450 | +388 (was +42) |
| authoring-feature-map | 22,323 | 22,698 | +7 | 0 | 22,705 | +382 (was +375) |
| authoring-architecture-store | 19,733 | 21,761 | +118 | 0 | 21,879 | +2,146 (was +2,028) |
| authoring-technical-requirements | 20,775 | 21,622 | +92 | 0 | 21,714 | +939 (was +847) |
| authoring-user-stories | 13,444 | 13,727 | 0 | +1 | 13,728 | +284 (was +283) |
| patterns-vertical-tdd | 15,775 | 15,935 | 0 | −1 | 15,934 | +159 (was +160) |
| testing-gap-finding | 19,977 | 23,173 | 0 | +35 | 23,208 | +3,231 (was +3,196) |
| review-feasibility | 12,220 | 12,011 | +84 | 0 | 12,095 | 125 under |
| authoring-constitution | 29,614 | 29,138 | +24 | 0 | 29,162 | 452 under |
| patterns-entity-modeling (body only) | 16,835 | 14,592 | — | +84 | 14,676 | 2,159 under |
| patterns-api-contracts (body only) | 13,412 | 12,278 | — | +84 | 12,362 | 1,050 under |

Every `description:` is the same before and after. The common view `.mochiko/schema-views/common/skill-review-common.yaml` went
from 1,274 to 1,917 characters (+643; `git show HEAD:` before). That is +290 against the 1,627 row; the row's figure is the lead's
ruling. The validate `budget` findings count rule text only and are not the ledger comparison.

## S1 — schema content `0046-joined-id-templates`

VALIDATE: schema content `plugins/mochiko/migrations/0046-joined-id-templates.yaml` with its regenerated view diff (10 template views).
Checklist run: `judgment-items-schema`, the AM-2-required-cli-dependency five.
Evidence read: `0046-joined-id-templates.yaml` (header, intent, anchor, 10 `replace-document` ops) · `git diff -- .mochiko/schema-views/templates/{spec,tasks,governance-intent,governance-surfaces,feature-entry,features-index,architecture-concerns,architecture-spine,architecture-store,design-baseline}.yaml`, read in full · record D2, D5, D9, D11, D12, D19, D21 · `w2-schema-plan.r1.frozen.md` §1–§2.
Pre-pass: `0 rejecting · 113 advisory`, with no advisory naming a template; views ≡ replay; HEAD views ≡ 1..45 replay.
- PASS — intent stated: the intent names D1, D2, D9, D11, D19 and D21. It also names the owner qualifiers, the SC padding, the cycle heading, the trace-line ripple and the bare carve-outs (D12, D21). Each one appears in the view diff.
- PASS — anchor present: "`anchor: 2026-10-08 human-readable-ids D21`"; it is bare, as an anchor must be (D12).
- PASS — ID lifecycle right: whole-document replaces. No rule id is minted, retired or reused, and the template section names are unchanged.
- PASS — floor and fail survival: templates carry no floor or fail rule, and the replaced documents keep every section, `required:` and `max_lines:`. The diff touches ID tokens, two overview insertions (the tasks owner line, the feature-entry file-slug line) and the governance-intent trace-matching line only.
- PASS — register: the inserted contract and overview text keeps the templates' `full` register.
- PASS — departure 1a (governance-intent "(deterministic …)" kept): matching on the GI number is still deterministic. The new parenthetical makes the line true now that ledger mentions are joined and markers stay bare (D12).
- PASS — departure 1b (`spec` derived-features `FEAT-YYY` row joins `US-2`): it applies D2 and D21's scope clause to an example ID the plan missed. That row's sibling already shows `US-1-{{slug}}`.
- PASS — fidelity checks: `<!-- GI-… -->` markers stay bare (`governance-surfaces.yaml:41-70,100`); `[US-1-{{slug}}](stories/US-1.md)` keeps the path bare; SC is three-digit (`SC-001-{{slug}}`, D9); cross-file SC carries its spec slug (`` `tenant-workspaces` SC-007-tenant-read-speed ``); the cycle `Raised` cell is behind the run key; and every H1 drift `shows` string survives ("`sequential GI-001, GI-002`", `**Depends on:** C1`).
- Advisory (non-gating): `governance-intent.yaml:160`'s check still reads `(GI-0XX, …)` while the two sibling checks in the same template went joined. Fix: join it to `GI-0XX-<slug>` in the next migration that touches this template.

VERDICT: PASS

Issues requiring fix: none.

## S2 — schema content `0047-joined-id-minting-rules`

VALIDATE: schema content `plugins/mochiko/migrations/0047-joined-id-minting-rules.yaml` with the views of its 14 reworded rules (10 documents).
Checklist run: `judgment-items-schema`, the AM-2-required-cli-dependency five.
Evidence read: `0047-joined-id-minting-rules.yaml` in full · `git diff -- .mochiko/schema-views/commands/{brainstorm,setup,implement}.yaml .mochiko/schema-views/skills/{authoring-requirements,authoring-feature-map,authoring-epic,authoring-architecture-store,authoring-technical-requirements,authoring-constitution,review-feasibility}.yaml` · `plugins/mochiko/skills/authoring-epic/SKILL.md:1-40` · record D1, D3, D4, D9, D11, D12, D13, D19, D21 · schema plan r1 §1 carrier table.
Pre-pass: `0 rejecting · 113 advisory`; rule-node diff: the 14 rewords changed `text` only.
- PASS — intent stated: the intent names D1 with D2, D3, D9, D13, D19, D21 and B4. It also names EPIC's new definition line, the owner forms and the IDs-section pointers, and each one is in the ops.
- PASS — anchor present: "`anchor: 2026-10-08 human-readable-ids D1`". No reword is re-anchored, so the prior rulings' anchors survive (for example `brainstorm.decision-cards` keeps "`2026-10-06 brainstorm-target-state D9`").
- PASS — ID lifecycle right: 14 `reword-rule` ops, ids kept, no mint, no tombstone.
- PASS — floor and fail survival: no reworded rule changed class or kind, and all 383 floor/fail rules survive. Each reword keeps its prior clause and adds the joined form. `setup.synthesis-artifact`'s dropped "GI-XXX namespace" survives as "numbered per that template's GI-ID rule".
- PASS — register: plain rule register, matching the rewords' neighbours.
- PASS — carrier coverage: rules left as written (US path, SCR/FLOW row kinds, patterns-vertical-tdd, the kind lists) match the plan's classification under the lead's Q1 and A5 rulings. GAP and BR have no log home.
- PASS — flag 3 (A8): `authoring-epic/SKILL.md:16` `[EPIC-XXX]` describes the marker in overview prose. `authoring-epic.map-grammar-routing` routes its written form to `authoring-feature-map`, whose reworded rule carries `[EPIC-XXX-<slug>]`, and an authoring-epic seat never writes the marker. The router line `skills/mochiko/SKILL.md:73` is the same class.
- Advisory (non-gating): `impl.baseline-entry-grammar` says "written joined" only for the `**Raised:**` key. The `proposed (<key>)` lifecycle value is left to the IDs section's every-mention rule, while the architecture-store template example shows it joined (A5). Fix: one clause, "the key written joined", in the 0048 reword that already touches this rule's neighbourhood.
- Advisory (non-gating): `authoring-epic/SKILL.md:16` and the router's `[EPIC-XXX]` mentions go joined at wave 3's touch of those files.

VERDICT: PASS

Issues requiring fix: none.

## S3 — schema content `0049-graders-check-joined-ids`

VALIDATE: schema content `plugins/mochiko/migrations/0049-graders-check-joined-ids.yaml` with its minted rules' views (common block, 5 review skills, 2 desk commands).
Checklist run: `judgment-items-schema`, the AM-2-required-cli-dependency five, plus the payload budget of the five member skills and the common row.
Evidence read: `0049-graders-check-joined-ids.yaml` in full · `git diff -- .mochiko/schema-views/common/skill-review-common.yaml .mochiko/schema-views/skills/{review-specifications,review-plan-artifacts,validation-constitution,review-governance-intent,review-brainstorm}.yaml .mochiko/schema-views/commands/{architecture,feature}.yaml` · `mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.verdict` (the stub resolves to the common text) · record D4, D6 (S7), D13 (build), D16 (S8) · wave plan item 3.
Pre-pass: `0 rejecting · 113 advisory`; `pointer resolution: 88 checked`; no new advisory; budgets as tabled above.
- PASS — intent stated: the intent names D6 with D4, D13 and D16, the five review skills, the two desk graders, and "No floor or fail". All seven ops match.
- PASS — anchor present: the migration anchor is "`2026-10-08 human-readable-ids D6`", and each minted rule carries it.
- PASS — departure 1c (`review-common.joined-ids` carries `anchor:`): the 0008 precedent covers it and validate accepts it. It keeps the common block's provenance where the stubs resolve it.
- PASS — ID lifecycle right: 8 new ids, unique, no reuse. The 5 stubs are `extends:` with a local `class: must`, which follows the family-common convention.
- PASS — floor and fail survival: nothing touched; the intent's "No floor or fail" holds.
- PASS — register: plain grader register. The rule names the checks D6 assigns (bareness, slug fit to the topic per D13 with the direction word, drift per S7, the definition decides per D16, the number within its scope per D4), so it is not a restatement of the pointed-at section.
- PASS — pairing coverage: the five skills plus `arch.seat-tech-lead-grader` and `feat.author-grader` (Q2) cover every minting producer's grader named in the plan's pairing list.
- PASS — argued overage: the five members grow +564 to +571 from one D6 stub each. Three are newly over (+497, +348, +259) and two grow on existing overages. This is a genuine new obligation the user ruled, not restored prose, so it HOLDS. The common view went from 1,274 to 1,917, +290 against its 1,627 row, also under D6; the row's figure is the lead's.

VERDICT: PASS

Issues requiring fix: none.

## K1 — skill pair `authoring-requirements`

VALIDATE: skill pair `authoring-requirements` — `SKILL.md` and `scripts/validate-requirements.py` with its seven rendered blocks.
Checklist run: `judgment-items-pair`.
Evidence read: `plugins/mochiko/skills/authoring-requirements/SKILL.md:1-80` · `git diff` of `SKILL.md` and `scripts/validate-requirements.py` · the seven-block render on log 1..45 against 1..49, diffed · `.mochiko/strips/authoring-requirements.md` `[v0.118.0]` · `scratchpad/gate-w2/fx/reqs-*.md`, run first-hand.
Pre-pass: `0 rejecting · 113 advisory`; `budget · skill/authoring-requirements · - · 21 rules · 3456 resolved characters`, advisory. Payload 12,874 = body 3,535 + render 9,339, description 379, unchanged.
- PASS — scaffold headings and order: seven `!` lines, the read-back sentence and the `## Rules — delivered by mochiko-cli` block are unchanged. The edit sits inside the two format fences.
- PASS — preserved responsibilities: the render diff is exactly 0047's three rewords (`fr-format`, `fr-numbering`, `sc-format`), each keeping its RFC 2119 and numbering clauses. The body keeps the requirement text.
- PASS — floor survival: pin `class: floor · 5 rules`, ids unchanged.
- PASS — independence and reserved section: both unchanged.
- PASS — done-condition branch: n/a; skills carry none.
- PASS — flag 2 (Q1 lookahead, a scope addition by the lead's ruling): first-hand red. The HEAD script on a bare bulleted list of three finds `fr 1 ['FR-001']`; the tree script finds 3. A mixed list of five finds all 5 and fails exactly the two-word and upper-case slugs. The strip names the bug, red and green, and the superseded lookahead.
- PASS — argued overage: +501, from render +145 (0047, D1, D21) and body +96 (joined FR/SC definition lines, D2, D19, P10). That is a genuine new obligation, so it HOLDS.

VERDICT: PASS

Issues requiring fix: none.

## K2 — skill pair `authoring-user-stories`

VALIDATE: skill pair `authoring-user-stories` — `SKILL.md`, `references/EXAMPLES.md`, `scripts/validate-user-stories.py` with the render.
Checklist run: `judgment-items-pair`.
Evidence read: `git diff` of the three files · the render on 1..45 against 1..49 (identical) · `.mochiko/strips/authoring-user-stories.md` `[v0.118.0]` · `scratchpad/gate-w2/fx/stories-mine.md`, run on the HEAD and tree scripts.
Pre-pass: `0 rejecting · 113 advisory`. Payload 13,728 = body 4,531 + render 9,197; description 425, unchanged.
- PASS — scaffold, floor survival (pin 5, unchanged), independence and reserved section: the render is byte-identical, and the body edit is one fenced heading.
- PASS — preserved responsibilities: the story template keeps title, priority and the template parts. The `stories/US-<n>.md` path stays bare (D12). The kind headings stay.
- PASS — script: the HEAD script finds 1 of 4 headings and the tree script 4 (joined, bare, legacy, one-word). Only the one-word slug fails `header_format`. Keeping the bare `### US-<n> —` form follows D16 and is disclosed. File mode 755 is kept.
- PASS — argued overage: +284 (was +283), body +1 from the heading form (D2, P4). It HOLDS.

VERDICT: PASS

Issues requiring fix: none.

## K3 — skill pair `patterns-vertical-tdd`

VALIDATE: skill pair `patterns-vertical-tdd` — `SKILL.md`, `references/BUNDLE-IDENTIFICATION.md`, `references/TEST-GRAMMAR.md` with the render.
Checklist run: `judgment-items-pair`.
Evidence read: `git diff` of the three files · the render on 1..45 against 1..49 (identical) · `.mochiko/strips/patterns-vertical-tdd.md` `[v0.118.0]` · `templates/tasks` view (card heading order).
Pre-pass: `0 rejecting · 113 advisory`. Payload 15,934 = body 5,907 + render 10,027; description 497, unchanged.
- PASS — scaffold, floor survival (pin 6, unchanged), independence and reserved section: the render is unchanged, and the body edits sit inside two fences.
- PASS — preserved responsibilities: the slicing blocks keep their wrong/right teaching. BUNDLE's worked cards follow the tasks template's `C<n>-<slug> — <title>:` order. Its Story→Cycle table qualifies stories (`` `task-tracker` US-1-… ``), as a `tasks.md` ID index citing a spec must (D11). `C2, C3, C4, C5` at :63 stays bare (D5, a list of four).
- PASS — coherence: TEST-GRAMMAR's `` `storefront` FLOW-002-cart-checkout-path `` and `SCR-004-account-settings-screen` reuse rule 1's slugs in `artifact-format.md`.
- PASS — argued overage: +159 (was +160), body −1. Within its standing overage.

VERDICT: PASS

Issues requiring fix: none.

## K4 — skill pair `testing-gap-finding`

VALIDATE: skill pair `testing-gap-finding` — `SKILL.md` with the render.
Checklist run: `judgment-items-pair`.
Evidence read: `git diff -- plugins/mochiko/skills/testing-gap-finding/SKILL.md` · the render on 1..45 against 1..49 (identical) · `.mochiko/strips/testing-gap-finding.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`. Payload 23,208 = body 7,005 + render 16,203; description 770, unchanged.
- PASS — scaffold, floor survival (pin 9, unchanged), independence and reserved section: the render is unchanged; the edit sits inside the gates.md example fence.
- PASS — preserved responsibilities: the example keeps its heading, its fold comment and its Source line. FEAT is joined, and the SC carries its spec owner (D2, D11).
- PASS — argued overage: +3,231 (was +3,196), body +35 from D2 and D11 forms. It HOLDS.

VERDICT: PASS

Issues requiring fix: none.

## P1 — prose primitive `templates/artifact-format.md`

VALIDATE: prose primitive `plugins/mochiko/templates/artifact-format.md` (the new `## IDs` section, rule 1, header, version line, consumed-by).
Checklist run: `judgment-items-prose` (coherence, preserved responsibilities), each IDs bullet held against D1–D22 and the build-time changes.
Evidence read: `git diff -- plugins/mochiko/templates/artifact-format.md` read in full · record cards D1–D22 with "Changed at review" and "Changed at build" · `w2-prose-plan.r2.frozen.md` §2.1–§2.2 · `grep -rln 'IDs section' .mochiko/schema-views/` (10 views) · `target/debug/mochiko-cli ids {rename,rekey} --help` · `crates/mochiko-cli/src/ids.rs:1218-1224` · `.mochiko/strips/artifact-format.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`. Section measured at 6,825 characters over 81 lines (file 7,100 → 14,557). No budget row exists, and none is invented.
- PASS — coherence with the record. Durable vs local follows D10 with S6's `Q1`. Joined form follows D19, D9 and D22, with S5's file-name exception. Topic words follow D13 and its build change (direction words count toward three). Every mention follows D2, file names D12 with D19 S5, compounds D5, and owners D11 with S4, B1 as narrowed (only a record link names a line's owner) and the lead's run-key ruling. Sub-IDs follow D22, machine-read spots D12 and D21, quotes D15 with S1 and V5, definition-decides D16 with S8 and D4, and rename/rekey D4, D7 and V3 with wave 1's A2 refusal sentence and C1.
- PASS — preserved responsibilities: rule 1 keeps its rule, its eight families and its closing sentence. The v3 version line and the old wording are verbatim in the strip. "a bare ID" became "the ID alone", which keeps the gloss rule's meaning once "bare" is defined.
- PASS — consumed-by line: it matches the 10 views that cite the IDs section.
- PASS — flag 6 (size 6,825 against the plan's ~5,500): every overrun source is a ruled obligation (B1 narrowed, A2 with C1, B1's quote clause, A1's file-name exception). The file is read on demand, not budgeted, and nothing restated was found.
- PASS — flag 9 (`GI-031` for "`GI-020`"): "GI-001 to 022" are decided here. 031 is outside that range, which keeps wave 3 from back-filling an example.
- PASS — flag 10 (rekey line): `ids rekey <owning-file> <old-ID> <new-ID>` with "the slug travels with the entry" is true on both the wave-1 surface and the wave-1b debug `--help` (`<OLD_ID>` bare or joined, `<NEW_ID>` bare). `ids rename … (the ID bare)` matches `<ID> The ID, bare`.
- Advisory (non-gating): "`mochiko-cli ids` never rewrites text inside a `"…"` quote" overstates the tool. `quotes_masked` (`ids.rs:1221`) unmasks quotes in `.html`/`.htm`, where an attribute is a link. Fix: add "(outside `.html` files, where a quoted attribute is a link)".
- Advisory (non-gating): at wave 1b, the rekey line could name the joined old-ID form for the case where two entries share a number. That case is 0048's landing shape.

VERDICT: PASS

Issues requiring fix: none.

## P2 — prose primitive `templates/report-format.md`

VALIDATE: prose primitive `plugins/mochiko/templates/report-format.md` (rule 5).
Checklist run: `judgment-items-prose`.
Evidence read: `git diff -- plugins/mochiko/templates/report-format.md` · the IDs section's Owners bullet · the lead's Q-A ruling (`build-log.md` 2026-10-08, "W2-prose plan FAIL … Q-A default stands") · `.mochiko/strips/report-format.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`; no budgeted class.
- PASS — coherence: rule 5 now points at the IDs section for how IDs are written. The examples share slugs with `artifact-format.md` and `CYCLE-REPORT-FORMAT.md`, and `T4.2` stays bare as the local-label contrast (D10).
- PASS — preserved responsibilities: "never re-quote their text" and the one-line-context allowance survive. The strip records both superseded lines.
- Advisory (non-gating, the most material in the wave): a report always cites upstream FR/SC/C IDs from outside their owning file. Rule 5's examples (`FR-003-csv-report-export`, `C-012-password-hashing-policy`, `SC-005-checkout-completion-rate`) show no owner, and unlike rule 1 of `artifact-format.md`, rule 5 says only "in their joined form". A seat copying them writes cites the `ids` check cannot tie to a definition (D18 as narrowed) and that a rename will not follow. The Q-A ruling covers examples in prose, so this is not a defect of the ruled edit. Fix: add "behind their owner where cited outside their own file", as rule 1 says.

VERDICT: PASS

Issues requiring fix: none.

## P3 — prose primitive `templates/advocate-report-template.md`

VALIDATE: prose primitive `plugins/mochiko/templates/advocate-report-template.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/advocate-report-template.md` `[v0.118.0]` · `grep -rn -i clarification plugins/mochiko .mochiko/schema-views` and `grep -rnE '### C[0-9]+:'` (no other consumer).
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: `### C1:` → `### Q1:` (D10 S6). No other primitive or view names the label.
- PASS — preserved responsibilities: the question, gap-id and options slots are unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P4 — prose primitive `templates/analyst-report-template.md`

VALIDATE: prose primitive `plugins/mochiko/templates/analyst-report-template.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/analyst-report-template.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: `US2` is corrected to the family form and joined behind its spec (`` `lunch-orders` US-2-order-expiry-handling ``), per Q-B and D11. `G3` and `A1` stay local labels.
- PASS — preserved responsibilities: the slot keeps its meaning.

VERDICT: PASS

Issues requiring fix: none.

## P5 — prose primitive `templates/techanalyst-report-template.md`

VALIDATE: prose primitive `plugins/mochiko/templates/techanalyst-report-template.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/techanalyst-report-template.md` `[v0.118.0]` · `FEASIBILITY-LENS.md:26`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: `NFR-003-global-p95-latency`. NFR is project-wide, so it carries no owner. Flag 9's slug change keeps `api-response-latency` on NFR-001 alone and matches FEASIBILITY-LENS's NFR-003.
- PASS — preserved responsibilities: line 16's kind list and `G2` stay.

VERDICT: PASS

Issues requiring fix: none.

## P6 — prose primitive `templates/feasibility-report-template.md`

VALIDATE: prose primitive `plugins/mochiko/templates/feasibility-report-template.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/feasibility-report-template.md` `[v0.118.0]` · 0047's `review-feasibility.findings-cite-ids` view.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the `at` example is `` `product` C-003-no-network-egress ↔ `product` D-007-hosted-vector-store ``, the same form as 0047's reword (D11, Q4).
- PASS — preserved responsibilities: every findings field is unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P7 — prose primitive `templates/constitution-modules/evolution-notes.md`

VALIDATE: prose primitive `plugins/mochiko/templates/constitution-modules/evolution-notes.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/evolution-notes-module.md` (new, full) · `ESSENTIAL-FLOOR.md:103`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the GAP mint slot is `GAP-XXX-<slug>`, and the example is `GAP-002-test-coverage-shortfall`, the same slug as ESSENTIAL-FLOOR (B4). GAP is project-wide.
- PASS — preserved responsibilities: the table and the confrontation line keep their meaning. The new strip file follows the module-strip header.

VERDICT: PASS

Issues requiring fix: none.

## P8 — prose primitive `templates/constitution-modules/knowledge-management.md`

VALIDATE: prose primitive `plugins/mochiko/templates/constitution-modules/knowledge-management.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/knowledge-management-module.md` `[v0.118.0]` · the `governance-surfaces` view's `**Trace**: GI-XXX-<slug>`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: both GI trace slots are joined, matching the ledger trace form in `governance-surfaces`. Line 141's `D16`, a decided citation, stays.
- PASS — preserved responsibilities: the collision-rulings slot is unchanged in meaning.

VERDICT: PASS

Issues requiring fix: none.

## P9 — prose primitive `skills/authoring-constitution/references/ESSENTIAL-FLOOR.md`

VALIDATE: prose primitive `plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/authoring-constitution.md` `[v0.118.0]` · prose plan §2.12.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the worked note's `GAP-002-test-coverage-shortfall` is the same slug as evolution-notes. It sits inside a `>` blockquote that quotes no source, which the plan's §2.12 lists under the lead's Q-A sub-ruling.
- PASS — preserved responsibilities: a one-token change. The decided citations `D1`, `D2`, `D4`, `D4.1`, `D5` stay (wave 3).

VERDICT: PASS

Issues requiring fix: none.

## P10 — prose primitive `skills/authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md`

VALIDATE: prose primitive `…/authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file read in full · `.mochiko/strips/authoring-technical-requirements.md` `[v0.118.0]` · 0047's `sequential-ids` view.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: definition headings lead with the joined ID (`### C-001-<slug>:`). Same-file cites carry no owner. FR sources sit behind `` `<spec-slug>` ``. `Applies to` sits behind `` `product` ``, with the spec's-own-file alternative stated. Numbering rule 4 links the IDs section (`../../../templates/artifact-format.md` resolves).
- PASS — preserved responsibilities: Format cells keep their meaning, and numbering rules 1–5 keep their rules. The kind-list lines stay.

VERDICT: PASS

Issues requiring fix: none.

## P11 — prose primitive `skills/authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md`

VALIDATE: prose primitive `…/authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file read in full · `git show HEAD:` of the same file (C-001's three meanings at :38, :88, :113) · `.mochiko/strips/authoring-technical-requirements.md` `[v0.118.0]` · the per-artifact scope line of the IDs section.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the slugs name their topics. Chains and narratives are joined. Matrix FRs are qualified. The `"C-003-…"` quote is illustrative (§2.12).
- PASS — C-001 carries three slugs (disclosed): the pre-edit file already used `C-001` for three topics in three independent examples (PostgreSQL :38, AWS environment :88, identity provider :113). A C-family number is scoped to its constraints file (F24, D4). One slug per example world is the faithful token-only edit; renumbering would change content. The strip records it.
- PASS — preserved responsibilities: every pattern, chain and rule keeps its teaching.

VERDICT: PASS

Issues requiring fix: none.

## P12 — prose primitive `skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`

VALIDATE: prose primitive `…/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/executing-tdd-cycle.md` `[v0.118.0]` · `build-log.md` 2026-10-08 G1 entries.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the `deviations` example cites `` `product` C-012-password-hashing-policy ``. `T3.4` stays local.
- PASS — flag 5 (G1): the narrative cites `` `FEAT-XXX-<slug>` C3-<slug> ``, the run-key owner. The stale `feature: user-auth` values are untouched by the user's ruling and booked for the backlog.
- PASS — preserved responsibilities: field definitions and the example report are otherwise unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P13 — prose primitive `skills/executing-tdd-cycle/references/TASK-PARSING.md`

VALIDATE: prose primitive `…/executing-tdd-cycle/references/TASK-PARSING.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · the `templates/tasks` view card heading · `.mochiko/strips/executing-tdd-cycle.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the card pattern `### - [ ] C{N}-<slug> — {title}` matches the template. The legacy `### - [ ] Cycle {N}:` stays readable (D16; the crate's legacy reader, C1). The checkbox locus is generalised to the card's `###` line.
- PASS — preserved responsibilities: the walking-skeleton ordinal at :25 stays, and every field row stays.

VERDICT: PASS

Issues requiring fix: none.

## P14 — prose primitive `skills/testing-end-user/references/TASK-PARSING.md`

VALIDATE: prose primitive `…/testing-end-user/references/TASK-PARSING.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/testing-end-user.md` `[v0.118.0]` · the build report's span note (located only).
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: END and the cycle-number reading both accept the joined heading and the legacy one. Without this change, gate extraction would miss every new card.
- PASS — flag 4 (fence :77-82, not in §2.12's list): the span is parser pseudo-code that quotes no source. The edit is the planned §2.3 change and is required for correctness; the Q-A sub-ruling's class (illustrative) fits it.
- PASS — preserved responsibilities: the legacy task-line form and the gate labels stay.

VERDICT: PASS

Issues requiring fix: none.

## P15 — prose primitive `skills/patterns-api-contracts/SKILL.md` (unconverted skill body)

VALIDATE: `plugins/mochiko/skills/patterns-api-contracts/SKILL.md`. I graded it as a prose primitive: it has no `!` lines and no log document (it is absent from `migrate validate`), so no render exists to pair. The body budget applies.
Checklist run: `judgment-items-prose` plus the body char-budget pre-assert.
Evidence read: `git diff` of the file · `.mochiko/strips/patterns-api-contracts.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`; body 12,362 ≤ 13,412, description 486, unchanged.
- PASS — coherence: the traceability cells are qualified and joined, and `US#1`/`US#4` are corrected to the family form (Q-B).
- PASS — preserved responsibilities: the table's columns and rows are unchanged.
- PASS — budget: 1,050 under.

VERDICT: PASS

Issues requiring fix: none.

## P16 — prose primitive `skills/patterns-entity-modeling/SKILL.md` (unconverted skill body)

VALIDATE: `plugins/mochiko/skills/patterns-entity-modeling/SKILL.md`. Graded as a prose primitive with its body budget, for the same reason as P15 (no `!` lines, no log document).
Checklist run: `judgment-items-prose` plus the body char-budget pre-assert.
Evidence read: `git diff` of the file · `.mochiko/strips/patterns-entity-modeling.md` `[v0.118.0]` · ARTIFACT-TEMPLATES' constraints header (`{feature_id}`), which supports a spec-owned DS.
Pre-pass: `0 rejecting · 113 advisory`; body 14,676 ≤ 16,835, description 497, unchanged.
- PASS — coherence: the traceability lists put the owner before the first member (the IDs section's compound rule). `US#n` is corrected (Q-B). `` `<spec-slug>` DS-001-<slug> `` is one of the two owners the IDs section allows.
- PASS — preserved responsibilities and budget: the entity examples are unchanged otherwise; 2,159 under.

VERDICT: PASS

Issues requiring fix: none.

## P17 — prose primitive `skills/patterns-entity-modeling/references/VALIDATION-RULES.md`

VALIDATE: prose primitive `…/patterns-entity-modeling/references/VALIDATION-RULES.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/patterns-entity-modeling.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the BR mint site is joined (`BR-001-date-range-order`, …), each slug coined from the rule's line (D19: BR has no name).
- PASS — preserved responsibilities: fields, rules and messages are unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P18 — prose primitive `skills/patterns-technical-decisions/references/DECISION-RECORD.md`

VALIDATE: prose primitive `…/patterns-technical-decisions/references/DECISION-RECORD.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/patterns-technical-decisions.md` `[v0.118.0]` · the lead's Q-B ruling (widened to `DR-XXX`).
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: `Superseded by D-XXX-<slug>` and `D1`–`D5` → `D-00N-…` correct the family to the file's own `D-XXX` entries (Q-B). The dependency table keeps its names after the ID.
- PASS — preserved responsibilities: the template's statuses and the impact chain's meaning are unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P19 — prose primitive `skills/review-brainstorm/references/RECORD-FITNESS.md`

VALIDATE: prose primitive `…/review-brainstorm/references/RECORD-FITNESS.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · 0047's `brainstorm.decision-cards` view · `.mochiko/strips/review-brainstorm.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: "headed by its joined ID and the decision's name (`### D<n>-<slug> — <name>`)" matches 0047's card rule word for word in form.
- PASS — preserved responsibilities: the six parts and the reporting rule are unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P20 — prose primitive `skills/review-feasibility/references/FEASIBILITY-LENS.md`

VALIDATE: prose primitive `…/review-feasibility/references/FEASIBILITY-LENS.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file read in full · 0047's `findings-cite-ids` view · `.mochiko/strips/review-feasibility.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the worked examples are joined as same-file definitions. The `at` contract and the vague-evidence row use the `` `product` `` owner form of 0047. "`GI-007`" → `GI-031-hexagonal-layer-boundaries` follows the collision guard.
- PASS — preserved responsibilities: every class, verdict word and anti-pattern is unchanged. `IP-XXX` kind mentions stay.

VERDICT: PASS

Issues requiring fix: none.

## P21 — prose primitive `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`

VALIDATE: prose primitive `…/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/review-plan-artifacts.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the example is `` `<spec-slug>` FR-003-<slug> not addressed in contracts ``, a cross-file cite with its owner. The other placeholders are family names.
- PASS — preserved responsibilities: severity and row meaning are unchanged.

VERDICT: PASS

Issues requiring fix: none.

## P22 — prose primitive `skills/review-plan-artifacts/scripts/check-artifacts.py`

VALIDATE: prose primitive (script) `…/review-plan-artifacts/scripts/check-artifacts.py`.
Checklist run: `judgment-items-prose`, plus a first-hand run.
Evidence read: `check-artifacts.py:146-175` · `git diff` of the file · the HEAD and tree scripts run on `scratchpad/gate-w2/fx/tasks-mine.md` · `.mochiko/strips/review-plan-artifacts.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — behaviour: `FR-012-order-cutoff-time`, `FR-012` and `fr-012` count as 1, and so do `US-1-…` and `US-1` (HEAD: 3 and 2). The legacy `FR-ABC-001` survives the `rstrip('-')`. Pass/fail is unchanged (count > 0), so `tier1-preassert` folds the same outcome. File mode 644 is kept.
- PASS — preserved responsibilities: no check is removed.

VERDICT: PASS

Issues requiring fix: none.

## P23 — prose primitive `skills/validation-constitution/references/QUALITY-CHECKLIST.md`

VALIDATE: prose primitive `…/validation-constitution/references/QUALITY-CHECKLIST.md`.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff` of the file · `.mochiko/strips/validation-constitution.md` `[v0.118.0]` · the `governance-surfaces` view.
Pre-pass: `0 rejecting · 113 advisory`.
- PASS — coherence: the ledger Trace form is `GI-XXX-<slug>`, and the region comment stays bare (D12). The real-ID example `GI-031-error-response-logging` sits outside 001–022. `"permanent (D4.1 pending)"` stays a decided citation.
- PASS — preserved responsibilities: both checklist items keep their checks.

VERDICT: PASS

Issues requiring fix: none.

## Departures and flags — dispositions

- 1a, 1b, 1c: S1 and S3, each PASS as argued there.
- 2 (Q1): K1 PASS, red and green proved first-hand. Advisory: `CHANGELOG.md` does not mention that the FR count changed for bulleted lists. Fix: one clause in the script sentence.
- 3 (A8): S2 PASS; the marker is named, not minted.
- 4 (TASK-PARSING :77-82): P14 PASS.
- 5 (G1): P12 PASS (user ruling).
- 6 (size): P1 PASS (argued).
- 7 (write route): the pre-pass allows all 23. The strips home has no template or bound, so nothing escaped a check.
- 8 (budgets): every overage HOLDS (D6 for the render; D1, D2, D19 and D21 for minting forms). Ledger rows and the common figure wait on the lead.
- 9 (`GI-031`, NFR-003 slug): P1 and P5 PASS.
- 10 (rekey line): P1 PASS, with an advisory for 1b.
- `CHANGELOG.md` `[Unreleased]` (read alongside): its census matches the pre-pass (87 documents, 1,175 rules, 0 rejecting, 113 advisory, 185 suppressed). It names the break (D20), the three migrations, the 23 strips and the `ids` dependency.

## Eval kit changes — non-gate read

`KIT: eval kit changes · PASS`
- Traceability: the seven added rows each trace to a 0049 op. That is five `rules.json` entries (`<skill>.joined-ids`, `source: log:…`, rule text equal to the rendered stub, checked on `review-plan-artifacts.sec.verdict`) and two plan-kit observables (`arch.grader-checks-joined-ids`, `feat.grader-checks-joined-ids`), each with a declared unplanted facet. The `evals.json` change traces to 0046's cycle heading.
- 0045 form: each new `## Re-key 2026-10-08 — human-readable-ids` carries the JSON `rekeyed` block (`at`, `ruling` citing the existing `DECISIONS.md` 2026-10-08 row, `source`), the field-scoped paragraph, Counts, Added table and Invariants verified. The counts match the views: rules.json against view ids 35/35, 36/36, 37/37, 27/27, and 31/32 with the disclosed pre-existing `sf-direction-checks` gap; order is preserved.
- No weakened expectation: every diff is an insertion, except `evals.json`, whose `not_contains` pattern widens to `(Cycle 5|C5\b)` and so catches both heading forms. Q-T1's untempted stubs are disclosed in four `rekey.md` files; review-brainstorm's goldens carry no `tempts` at all.
- `uv run evals/plan/run.py check-rubric <kit>`, read-only, evals tree unchanged after: architecture `46 observable, 3 out-of-instrument, 49 total` (README updated to 46/49 ✓); feature `48 observable, 3 out-of-instrument, 51 total` (48/51 ✓); specify, brainstorm and setup OK. implement is red with `uncovered:['impl.craft-floor-binding', 'impl.design-audit-advisory', 'impl.design-finish', 'impl.design-first-write', 'impl.design-landing']`, none of them a wave-2 id. It predates the wave (Q-T2).
- Advisory: review-brainstorm's new section drops 0045's "Re-running the script … changes nothing" invariant line. README's `specify 39 of 52` is stale against 40/52 (pre-existing, Q-T2).

## Outcome lines

- `audit: S1 0046-joined-id-templates · w2-gate-audit · opus · 11 files · 1 rounds · 0 blocking`
- `audit: S2 0047-joined-id-minting-rules · w2-gate-audit · opus · 11 files · 1 rounds · 0 blocking`
- `audit: S3 0049-graders-check-joined-ids · w2-gate-audit · opus · 9 files · 1 rounds · 0 blocking`
- `audit: K1 authoring-requirements pair · w2-gate-audit · opus · 4 files · 1 rounds · 0 blocking`
- `audit: K2 authoring-user-stories pair · w2-gate-audit · opus · 4 files · 1 rounds · 0 blocking`
- `audit: K3 patterns-vertical-tdd pair · w2-gate-audit · opus · 4 files · 1 rounds · 0 blocking`
- `audit: K4 testing-gap-finding pair · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- Round 1, superseded by the round-2 line below: `audit: P1 templates/artifact-format.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- Round 1, superseded by the round-2 line below: `audit: P2 templates/report-format.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P3 templates/advocate-report-template.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P4 templates/analyst-report-template.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P5 templates/techanalyst-report-template.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P6 templates/feasibility-report-template.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P7 constitution-modules/evolution-notes.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P8 constitution-modules/knowledge-management.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P9 authoring-constitution/references/ESSENTIAL-FLOOR.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P10 authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P11 authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P12 executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P13 executing-tdd-cycle/references/TASK-PARSING.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P14 testing-end-user/references/TASK-PARSING.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P15 patterns-api-contracts/SKILL.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P16 patterns-entity-modeling/SKILL.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P17 patterns-entity-modeling/references/VALIDATION-RULES.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P18 patterns-technical-decisions/references/DECISION-RECORD.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P19 review-brainstorm/references/RECORD-FITNESS.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P20 review-feasibility/references/FEASIBILITY-LENS.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P21 review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P22 review-plan-artifacts/scripts/check-artifacts.py · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: P23 validation-constitution/references/QUALITY-CHECKLIST.md · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`

## Round 2 — P1 and P2 re-read after the advisory fixes

Same seat (`w2-gate-audit`) and tier (`opus`), resumed, against the same render (rendered first-hand at round 1, byte-identical to `vpe-render-1-49.txt`; the contract and the log are unchanged since). The re-read covers only what each fix touched and what it could have broken. Round-1 P1 and P2 verdicts and outcome lines are superseded by the blocks and lines below.

Scope proof: reversing the seat's `scratchpad/w2p-resume.diff` P1 and P2 hunks on scratch copies of the two current files restores the round-1 text exactly. The IDs section goes back to 6,825 characters, and rule 5 goes back to the round-1 wording. These hunks are the only change to either primitive since round 1. The diff's ledger rows and `CHANGELOG.md` sentence are not these units.

Pre-pass, re-run first-hand: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 113 advisory`; `mochiko-cli migrate status --plugin-root plugins/mochiko` → `sha256:f1d8ce9c… · 87 documents · 1175 rules`, both unchanged. `mochiko-cli check --path .mochiko/strips/{artifact-format,report-format}.md --content - --plugin-root plugins/mochiko` → `allow` × 2; each file still carries exactly one `[v0.118.0]` entry.

### P1 — round 2

VALIDATE: prose primitive `plugins/mochiko/templates/artifact-format.md`, round 2 — the quotes bullet's HTML exception.
Checklist run: `judgment-items-prose` (coherence, preserved responsibilities), held to the fix and what it could break.
Evidence read: `plugins/mochiko/templates/artifact-format.md` (the quotes bullet, now `"…"` quote "(except in HTML files, where a quoted attribute is a link)") · `crates/mochiko-cli/src/ids.rs:740-800` (`masked`, `masked_as`) and `:1218-1224` (`quotes_masked`) · `.mochiko/strips/artifact-format.md` `[v0.118.0]` (the added line) · `w2p-resume.diff` P1 hunk.
Pre-pass: as above. The IDs section measures 6,886 characters (+61), and no budget row exists.
- PASS — coherence with the tool: `quotes_masked` returns false only for the `.html`/`.htm` extensions, and `masked_as` drops only the `"…"` mask when `quotes` is false. The fence and `>` blockquote masks are set per line before that check, in every file. The clause sits directly after `"…"` quote, so it scopes to the quote mask alone, as the code does. The strip line says the same ("The `>` blockquote and code-fence masks hold in every file").
- PASS — nothing broken: the rest of the bullet is verbatim, including the "a seat never re-slugs a verbatim quote of a source" sentence. No other line of the file moved. The size argument from round 1 still holds at +61.
- PASS — strip: the added line cites its ground (this audit's advisory, the lead's resume, N4 as narrowed, `quotes_masked`). It says "a pure addition: nothing is superseded", which the diff confirms.

VERDICT: PASS

Issues requiring fix: none.

### P2 — round 2

VALIDATE: prose primitive `plugins/mochiko/templates/report-format.md`, round 2 — rule 5's owner clause and owned examples.
Checklist run: `judgment-items-prose`, held to the fix and what it could break.
Evidence read: `plugins/mochiko/templates/report-format.md:53-57` · the IDs section's Owners bullet (`artifact-format.md`: FR and SC behind the spec's slug, the C family behind `` `product` `` for the product baseline) · `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md:55` · `grep -rnoE` over `plugins/mochiko` for every `` `lunch-orders` `` cite and every `FR-003-`/`SC-005-`/`C-012-` slug · `.mochiko/strips/report-format.md` `[v0.118.0]` (the added line) · `w2p-resume.diff` P2 hunk.
Pre-pass: as above; no budgeted class.
- PASS — owner forms: `` `lunch-orders` FR-003-csv-report-export `` and `` `lunch-orders` SC-005-checkout-completion-rate `` put a spec slug before FR and SC. `` `product` C-012-password-hashing-policy `` puts the product baseline's owner before a C-family ID. All three use the code-span qualifier form the Owners bullet prescribes, and `T4.2` stays bare as a local label (D10).
- PASS — coherence: the clause "behind their owner where cited outside their own file" is word for word rule 1's in `artifact-format.md`, so the two shared rules now agree. The slugs are unchanged and still one per ID across the write set: `csv-report-export` sits only on FR-003, matching rule 1 and the compounds example. `checkout-completion-rate` matches rule 1. `C-012` with `` `product` `` matches `CYCLE-REPORT-FORMAT.md:55`. `lunch-orders` is the fictional spec slug that `artifact-format.md:134` and `analyst-report-template.md:17` already use.
- PASS — nothing broken: "never re-quote their text" and "where the ID alone would be unreadable" are verbatim. No other rule moved.
- PASS — strip: the added line names the clause and the three owned examples and gives the ground (D11, this audit's advisory, the lead's resume). It marks the Disposition quote above it as the first-landed form, superseded before any release.

VERDICT: PASS

Issues requiring fix: none.

### Round-2 outcome lines

- `audit: P1 templates/artifact-format.md · w2-gate-audit · opus · 2 files · 2 rounds · 0 blocking`
- `audit: P2 templates/report-format.md · w2-gate-audit · opus · 2 files · 2 rounds · 0 blocking`

## Round 3 — S4 and S5, two new schema-content units

Same seat (`w2-gate-audit`) and tier (`opus`), resumed, against the same render (rendered first-hand at round 1; the contract is unchanged since). Both units are new at this round, so each takes a full read under the AM-2-required-cli-dependency five, and each outcome line reads `1 rounds`.

Pre-pass, run first-hand:
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` gives `0 rejecting · 113 advisory`, run twice with identical output. Against the round-1 run, one line changed: `budget · command/implement · - · 114 rules · 34627 resolved characters of rule text` (was 34319, +308). No advisory names `templates/governance-intent`.
- `mochiko-cli migrate status --plugin-root plugins/mochiko` gives `sequences 1..50 (48 migrations)` and `state sha256:c7e23ab026d07fb3… · 87 documents · 1175 rules`, the lead's figures.
- The views emitted to scratch (87 documents) equal `.mochiko/schema-views/`. Against the emit taken before 0048, only `commands/implement.yaml` and `templates/governance-intent.yaml` differ.
- Rule-node diff from that earlier replay to 1..50: 0 rules gone, 0 new, and one changed (`impl.baseline-diff-review`, text only). All 383 floor and fail rules survive with class and kind unchanged.
- Budget: commands carry no per-primitive budget (`primitive-cost-budgets.md:34-35`, `:542-543`), so implement's +308 is the advisory rule-text figure only. Neither unit touches a skill, an agent or another budgeted class.
- Template goldens: all 24 `mochiko-cli template` renders (12 templates, producer and check) equal `crates/mochiko-cli/tests/fixtures/template/*.txt` byte for byte once the source line is stripped, which is how `tests/render.rs:1230-1253` compares them. `governance-intent.check.txt` carries 0050's joined check. No `cargo` ran.

### S4 — schema content `0048-landing-rekey`

VALIDATE: schema content `plugins/mochiko/migrations/0048-landing-rekey.yaml` with its regenerated view diff (`commands/implement.yaml`).
Checklist run: `judgment-items-schema`, the AM-2-required-cli-dependency five, plus the lead's two named checks: the text against `ids rekey --help`, and the ruling that no refusal path enters the rule.
Evidence read: `0048-landing-rekey.yaml` in full · the view diff at `implement.yaml:711-733` (one rule) · `implement.yaml:1117-1125` (`impl.fail.unreviewed-baseline-diff`) · `target/debug/mochiko-cli ids rekey --help` · `crates/mochiko-cli/src/rename.rs:205-212`, `:240-325`, `:360-381`, `:771-790`, `:1599-1621` · `crates/mochiko-cli/src/cli.rs:1094-1099` · record D4, D7, D15, Q5 (a).
Pre-pass: as above.
- PASS — intent stated: the intent cites D4 as changed at review, D7 V3, D15 V3 and Q5 (a). It names the joined old ID, the slug that travels, the mentions that follow, and the untied mentions settled by hand. It calls the change one reword of a floor rule that keeps its id, class and anchor. Each claim is in the op and the view diff.
- PASS — anchor present: the migration carries "`anchor: 2026-10-08 human-readable-ids D4`", bare as an anchor must be (D12). D4 as changed at review is the rekey ruling, so it covers this floor reword. The rule's own "`anchor: 2026-09-24 delta-files-vs-direct-baseline-edits D3`" is unchanged.
- PASS — ID lifecycle right: one `reword-rule` op on `command/implement` `impl.baseline-diff-review`. The id is kept, and nothing is minted or tombstoned.
- PASS — floor and fail survival: the rule stays `class: floor` with labels `[landing, evidence]`. The reword only adds text: "the second run to land renumbers its own entries" is kept word for word, and the clause after it is new. The landing order is unchanged: the unmarked-write test, then the duplicate-id check and renumber, then the flip. `impl.fail.unreviewed-baseline-diff` still enforces the rule, and its text ("the duplicate-id check") still matches.
- PASS — register: plain rule register. The command sits in a code span, and its placeholders follow `--help`'s argument names.
- PASS — text against the tool: `--help` gives `<OWNING_FILE> <OLD_ID> <NEW_ID>`. The old ID may be "joined (`D-012-csv-export-format`) to pick one entry when two share the number", and the new ID is "bare: the slug travels with the entry". The command previews "unless --write". The rule says each of these in the same terms. At a landing, the two holders are provably two entries (`rename.rs:771-790`): one file holds same-kind sites of both, or each owns an entry file. So the joined form the rule demands works.
- PASS — untied parenthetical: a mention tied to no holder is never rewritten and is listed in `untied` (`rename.rs:205-212`, `:360-381`). The preview prints each as `untied mention …` (`:1620-1621`). So "a bare mention the preview lists as untied is settled by hand" describes what the tool does.
- PASS — the lead's ruling (no refusal path in the rule): the rule leaves refusals to the tool, and nothing in the contract demands a refusal clause in a floor rule.
- Advisory (non-gating; it concerns the crate, not this unit): the ruling's premise holds only in part. The shared-holder refusals name their next step (`rename.rs:248-253`, `:284-291`, `:293-297`), but two refusals do not:
  - `:268-273` "`{old}` is defined twice alike in one file … so a rekey cannot move one (R1)". A landing reaches it when both runs coin the same slug for the same number in one file.
  - `:321-324` "carries `{old}` in a number-only path, which a rekey would orphan".

  Both refuse loudly (`error:`, exit 2, `cli.rs:1094-1099`), so nothing lands wrong. But the seat is left with no next step. Fix: each message names its step; for R1, "give one of the two entries a different slug by hand, then rekey it by that joined ID". Book it as a crate follow-up. The floor rule stays as ruled.

VERDICT: PASS

Issues requiring fix: none.

### S5 — schema content `0050-governance-intent-check-joined`

VALIDATE: schema content `plugins/mochiko/migrations/0050-governance-intent-check-joined.yaml` with its regenerated view diff (`templates/governance-intent.yaml`).
Checklist run: `judgment-items-schema`, the AM-2-required-cli-dependency five, plus whether it closes this seat's round-1 S1 advisory.
Evidence read: `0050-governance-intent-check-joined.yaml` (header, intent, anchor, the one `replace-document` op) · the view diff against the emit taken before 0050 · `mochiko-cli template governance-intent` producer and `--check` renders · record D12, D21 · this report's S1 advisory.
Pre-pass: as above.
- PASS — intent stated: "Join the one GI placeholder 0046 left bare … Nothing else in the template changes." It cites D21 and this audit's S1 advisory, and the view diff holds it to that.
- PASS — anchor present: "`anchor: 2026-10-08 human-readable-ids D21`", bare.
- PASS — ID lifecycle right: a whole-document replace of template `governance-intent`. Templates carry no rule ids, and no section name changes.
- PASS — floor and fail survival: the view diff is the one check, `(GI-0XX,` to `(GI-0XX-<slug>,`, plus its reflow. Every section, every `required:` and `max_lines:`, and `conformance: extra_headings: allow` are unchanged.
- PASS — register: one token changes, and the check now reads like its siblings.
- PASS — closes the round-1 S1 advisory: both GI checks in the check render now carry `GI-0XX-<slug>` (lines 5 and 9), matching every producer placeholder. No bare `GI-0XX` is left in the view or in either render. That follows D21's rule for minting forms. The bare `<!-- GI-… -->` markers live in `governance-surfaces`, which this unit leaves untouched (D12).

VERDICT: PASS

Issues requiring fix: none.

### Round-3 outcome lines

- `audit: S4 0048-landing-rekey · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
- `audit: S5 0050-governance-intent-check-joined · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking`
