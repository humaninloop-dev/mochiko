# Wave 3 — census plan (staff-engineer seat; approved by the lead 2026-09-13, build not yet open)

**Ruling home:** `record.md` D2, D4, D5, D6, D11 wave 3, F9/F10/F13, OQ1/OQ2. **Inputs:**
`wave3-census-raw.md` (346 patterns), `wave1-template-readiness.md`, the ADR
`2026-08-22-verbosity-envelope-enforcement.md`, `.mochiko/memory/knowledge-management.md`.
**Done condition:** one migration · `migrate validate` clean · derived views regenerated · budget
table user-ratified before anything ships · **no crate change** (the mechanism is wave 1's, frozen) ·
**`check`'s median measured against the log with `0005` applied and stated beside the 100 ms cache
trigger** (§6a, the reviewer's G3 fact).
**Forward constraint, binding here:** the conformance keys and the `home` `import-document` ops ride
**one migration file** — split them and that file takes a grammar bump, because `schema::Section`
ignores unknown keys and an older binary would read a budget-less template silently.

## 1. How 346 patterns collapse

Counted off the raw census, and they sum to 346 exactly:

| class | patterns | becomes |
|---|---|---|
| report/review files under a run home (`*/reports/`, `*/reviews/`, `wave<n>-reports/`) | 174 | one `reports/` rule per home — open by name, typed envelope (D2). The 47 `wave<n>-reports/` files move in wave 5's violator pass, not here (R4, ruled) |
| brainstorm session working files (non-report) | 44 | **R4, ruled** — a patterned deliverable plus three subdirs |
| maintainer eval harness (`.mochiko/benchmarks/**`) | 42 | **R5** — propose out of scope, stated |
| root-level docs (8 distinct across both repos) | 14 | **R6** — propose not a home (§2 note) |
| declared deliverables, plus `B53`/desk/`landing/`/`evidence/` | 72 | §2's closed sets; **R1**, **R2**, and the §2 seat calls |

## 2. The home list — 21 homes

`bounds`: T = per template section · W = whole file · E = elsewhere, cited. Eighteen rows for 21
homes: the four `contracts` homes share one row because their file sets are identical.

| # | home | path segments | bounds | deliverables (closed) | subdirs | reports/ |
|---|---|---|---|---|---|---|
| 1 | `brainstorms-index` | `.mochiko` `brainstorms` | W | `index.md` | — | no |
| 2 | `brainstorm-session` | `.mochiko` `brainstorms` `<slug>` | W | `record.md` · `synthesis.md` · `build-log.md` · `wave<n>-<slug>.md` (R4, ruled) | `inputs` `research` `referents` (R4, ruled) | **yes** |
| 3 | `decisions` | `.mochiko` `decisions` | W | `<date-slug>.md` | — | no |
| 4 | `specs-index` | `.mochiko` `specs` | W | `index.md` | — | no |
| 5 | `spec` | `.mochiko` `specs` `<slug>` | T | `spec.md`→`spec` · `derivation.md` · `data-model.md` · `constraints-and-decisions.md` · `quickstart.md` | `stories` `contracts` `prototype` `map-delta` | yes |
| 6 | `spec-stories` | … `<slug>` `stories` | W | `US-<n>.md` | — | no |
| 8 | `spec-prototype` | … `<slug>` `prototype` | E → the prototype manifest in `spec.md` | `README.md` · `<any>` | `<any>` | no |
| 9 | `spec-map-delta` | … `<slug>` `map-delta` | W | `FEATURES.md` · `<FEAT-ID>.md` · `README.md` · `selection-card.md` · `specs-index.md` | — | no |
| 10 | `features-index` | `.mochiko` `features` | T | `<FEAT-ID>.md`→`feature-entry` | — | no |
| 11 | `feature` | `.mochiko` `features` `<FEAT-ID>` | T | `tasks.md`→`tasks` · `plan.md` · `requirements.md` · `design-closure.md` · `baseline-delta.md` · `sufficiency-report.md` · `data-model.md` · `constraints-and-decisions.md` · `architecture.md` · `proposal.md` · `contest-brief.md` · `gates.md` | `contracts` | **yes** |
| 13 | `feature-desk` | `.mochiko` `features` `desk` `<date-slug>` | W | R2 | — | yes |
| 14 | `epic` | `.mochiko` `epics` `<EPIC-ID>` | T | `manifest.md` · `proposal.md` · `contest-brief.md` · `architecture.md` · `data-model.md` · `constraints-and-decisions.md` · `quickstart.md` · `build-order.md` · `screens-and-flows.md` · `implement-log.md` (log, R3) | `contracts` | **yes** |
| 16 | `product` | `.mochiko` `product` | T | `data-model.md` · `constraints-and-decisions.md` · `quickstart.md` | `architecture` `contracts` | no |
| 17 | `product-architecture` | `.mochiko` `product` `architecture` | T | `spine.md`→`architecture-store` · `concerns.md` | `concerns` | no |
| 18 | `product-concerns` | … `architecture` `concerns` | W | `<AX-ID>.md` | — | no |
| 20 | `product-lane` | `.mochiko` `product` `<slug>` | W | `sufficiency-report.md` · `baseline-delta.md` | — | **yes** |
| 7 · 12 · 15 · 19 | the four `contracts` homes — under `spec`, `feature`, `epic`, `product` | … `contracts` | W | `<slug>.md` · `api.yaml` · `README.md` | — | no |
| 21 | `memory` | `.mochiko` `memory` | T | `governance-intent.md`→`governance-intent` · `governance-ledger.md` · `governance-trace-summary.md` · `knowledge-management.md` · `codebase-analysis.md`→`codebase-analysis` · `primitive-cost-budgets.md` | — | no |

Homes 16–19 nest under `.mochiko/product`, and 20 uses `<slug>` at the depth of the literals
`architecture` and `contracts`; longest-literal-prefix resolves it, which wave 1 tested.

**Two seat calls with disclosure.** `reviews/` is not declared anywhere: a review is a report
(`report: review`) and lands in `reports/` (D2/I5), re-homing 22 kinako files. And
`reports/evidence/<id>/` is **forbidden** — the resolver admits exactly `reports/<name>`, and
evidence belongs in the report's own fields under the machine-first envelope, not in a sub-tree.
`epics/<EPIC-ID>/landing/` is forbidden for the same reason: EPIC-002 already dropped it, and its
four files are reports.

**Disclosed hole (new, worth your eye).** Nothing catches a session artifact written to the **repo
root**: a `Write` to `./record.md` resolves outside every home, the sniff fires only on a `report:`
type, and a closed root set would deny the consuming project's own markdown. So `CLAUDE.md`'s "the
top level is reserved for the living operating docs" stays prose-enforced — which is why R6 proposes
the root docs are not a home: their location has no wrong value to catch, their bounds live in the KM
invariants (D5/I6), and `mochiko:grooming-operating-docs` is the responder.

## 3. Templates — the heading map, and the four decisions

From `wave1-template-readiness.md`. Ready or nearly ready:

| template | `heading:` map | action |
|---|---|---|
| `spec` | all 11 sections map 1:1 | add `max_lines` only |
| `governance-intent` | 13 of 14; `Header` governs none | mark `Header` heading-less |
| `feature-entry` | 7 of 8; `Header & Status` governs none | same |
| `codebase-analysis` | 3 overrides: em dash → colon (`Part 1: Inventory (Factual)`, `Part 2: …`, `Appendix: …`); `Header` heading-less | 3 `heading:` values + 1 heading-less |

The four needing a decision, each with my proposal:

- **`tasks` (R7).** The skeleton omits the declared `## Cycle Cards`, putting cards as
  `### - [ ] Cycle 1:` under `## Cycle Format` — F10's kinako drift is that skeleton followed
  faithfully. **Fix the skeleton** to emit `## Cycle Cards`; declaring the defect would ratify it. The
  fix rides the `replace-document` op and, being schema content, takes **no strip entry** (lead's
  ruling against the primitive-edits ceremony).
- **`architecture-store` (R8).** Its five sections name *files* and its skeleton concatenates three
  files' headings. **Split into `architecture-spine` and `architecture-concerns`**, bound per file at
  homes 17 and 18; the store template keeps layout guidance with no conformance block.
- **`features-index` (R9a).** `FEATURES.md` is a title, a blockquote and one table — zero `##`.
  **Frontmatter and placeholders only, no heading grammar, whole-file bound**; heading order is
  vacuous here and inventing headings would change the artifact.
- **`governance-surfaces` (R9b).** Its skeleton says it "is a SET of five distinct surfaces", and D5
  already puts `CLAUDE.md` outside the gate with the rules files at location-only. **No conformance
  block this wave**, revisited if the ledger and trace manifest get their own templates.

**One new template: `report-envelope`.** The `reports:` binding needs a log template to resolve
against, and the envelope lives today in the shipped markdown `templates/report-format.md`, which
the binary may not read (GI-020). Wave 3 mints it as `template/report-envelope` carrying
`frontmatter.required: [report, feature]` and the enum `cycle · verification · final-validation ·
review · feasibility · disclosure` — the same six `report-format.md` line 23 carries, so the
markdown stays the human home and the log becomes the machine one.

## 4. The budget table — proposal, derived from the ADR never from observed sizes

The ADR gives two numbers and one posture, and every row below keys to one of them:

- **A4p** — `artifact-format.md` rule 4: "Overview / context / rationale prose defaults to ≤ 3
  lines". A prose section's budget is 3 content lines + its `##` line + 2 blanks = **6**.
- **A4l** — rule 4: "list entries … are one line each". A list section's budget is a stated entry
  ceiling × 1 line + 3. The ceiling is the ADR's compact intent, not kinako's count.
- **RE** — the ADR's report posture: "reviews should land at report-envelope scale", "clean =
  frontmatter-only".

| kind · section | budget | key | observed |
|---|---|---|---|
| every prose section (`spec` Overview, `feature-entry` Capability/Extent, `governance-intent` prose) | 6 | A4p | — |
| `spec` Intent | 10 | A4l, 6 ruling lines + 3 | — |
| `spec` Key Entities · Assumptions · Open Questions | 15 · 10 · 10 | A4l, ceilings 12 · 7 · 7 | — |
| `spec` Screens & Flows · Feature Selection | 30 · 20 | A4l, two tables + pointer; three sub-parts | — |
| `tasks` Overview · Cycle Format | 8 · 6 | A4p; doctrine belongs in the skill (rule 7) | — · 24 |
| `feature-entry` Work rows · Relations · Story trace · Obligations | 20 each | A4l, ceiling 17 | — |
| `governance-intent` Waivers · Minted intents · Module selections | 25 each | A4l, ceiling 22 | — |
| `codebase-analysis` Part 1 · Part 2 · Appendix | 40 · 40 · 15 | A4l, ceilings 37 · 37 · 12 | 197 (file) |
| `report-envelope` every payload section | 15 | RE | 200–1,100/report |
| whole-file (W homes) · `baseline-delta.md` · `implement-log.md` per entry | 150 · 300 · 60 | RE + A4l | up to 2,694 |
| **`none` (OQ1):** `spec` User Stories · Edge Cases · Functional Requirements · Success Criteria; `tasks` Cycle Cards | none | a count the feature owns is not a verbosity lever | 1,731 (`tasks.md`) |

Every numeric row is **tight by construction** — the observed column shows the gap the census
measures, and no row is derived from it.

## 5. The authoring-time rule (D1a second arm)

One rule, minted per producing primitive, `class: floor · kind: binding · labels: [binding]`,
anchored `2026-09-13 hook-enforced-artifact-schema D1`. Text:

> Before the first write to an artifact home, render `mochiko-cli home <path>` and hold what it
> returns: the declared home, its closed file set, the template bound to each file, and the bound. A
> name the set does not carry is raised upstream, never minted from a sibling run; a templated file's
> section budgets come from `mochiko-cli template <kind>` before drafting, not after.

| primitive | section | id |
|---|---|---|
| `brainstorm` · `setup` · `specify` · `feature` · `architecture` · `implement` | `<prefix>.sec.ways-of-working` | `<prefix>.artifact-home` |
| `authoring-architecture-store` · `authoring-feature-map` · `authoring-user-stories` · `authoring-requirements` · `authoring-technical-requirements` · `authoring-epic` · `authoring-prototype` · `authoring-constitution` | `<name>.sec.artifact` | `<name>.artifact-home` |
| `patterns-vertical-tdd` · `executing-tdd-cycle` · `patterns-entity-modeling` · `patterns-api-contracts` · `analysis-codebase` | `<name>.sec.artifact` | `<name>.artifact-home` |

19 mints. `ways-of-working` over `boundaries` because it is a step in how a run proceeds, and
`class: floor` carries the non-waivability without moving it. Verified: all six commands carry
`sec.ways-of-working`, and every authoring-family skill carries `sec.artifact`.

**The `impl.reports-envelope` re-key.** It sits in `impl.sec.tools`, `class: must`, **no anchor**, so
a plain `reword-rule` suffices and no ruling anchor is owed. Its path changes from
`.mochiko/features/FEAT-XXX/` to that home's `reports/`, and its five named kinds become the
envelope's enum (D2/M8).

## 6. The migration — `0005-artifact-homes.yaml`, sequence 5

Sequences 1–4 are taken; 5 is the next. One file, per D11 wave 3, ~2,900 lines.

| op | count | what |
|---|---|---|
| `import-document` `kind: home` | 21 | §2 |
| `import-document` `kind: template` | 3 | `report-envelope`, `architecture-spine`, `architecture-concerns` (R8) |
| `replace-document` `kind: template` | 5 | `spec` · `tasks` · `feature-entry` · `governance-intent` · `codebase-analysis` — conformance keys |
| `mint-rule` | 19 | §5 |
| `reword-rule` | 1 | `impl.reports-envelope` |

**Why it is large, stated rather than worked around:** a template is opaque, so adding `max_lines`
restates the whole template — the only ops are `import-document` and `replace-document`. A
`set-document-field` op would fix it and is **declined**: a crate change against a frozen crate. If
you prefer two files, homes and templates must stay together in the first and only the rule mints may
move; I have not assumed that.

## 6a. What `0005` costs the gate (the reviewer's G3 fact, folded)

`check` is **replay-dominated**: its cost is decoding the log, not starting the process. Measured on
this box — 3 ms of process start; 3 ms on the 152-line fixture log; **37 ms median on the real
12,063-line / 640 KB log, with 30 ms of user CPU**. So log growth is the cost lever, and `0005` adds
~2,900 lines (+24%), which projects to a ~45 ms median — under the > 100 ms cache trigger, but
projected, not measured.

**Therefore the done condition measures it.** After `0005` lands, report `check --hook-json -`'s
median over 20 runs against the grown log beside the 100 ms trigger. Over the trigger, the
`${CLAUDE_PLUGIN_DATA}` cache at the single `load_for_delivery` call site in `cli.rs` (key: the
existing `State::content_hash`) becomes a wave-4 item on the lead's word — designed in wave 1, still
not built, and never built on a projection.

## 7. User ruling vs seat's call

| # | ruling owed | my proposal |
|---|---|---|
| R1 | `B53`-class ids | forbid; re-home its 9 cycle reports under the owning feature's `reports/` |
| R2 | the two desk-output homes | one home at `features/desk/<date-slug>/`; `specs/<slug>/derivation.md` stays the spec's own deliverable |
| R3 | `implement-log.md` (OQ2) | stays in the epic home as `form: log`, 60 lines per entry |
| R4 | the brainstorm session working set | **ruled by the lead:** `reports/` only, no `wave<n>-reports` subdir and no second convention; `wave<n>-<slug>.md` plus the three subdirs accepted; the physical re-home is wave 5's violator pass — §9 |
| R5 | `.mochiko/benchmarks/**` | out of scope, stated: a maintainer eval harness, not command-minted. D5 named the whole tree, so the carve is the user's |
| R6 | root operating docs | not a home (§2) |
| R7–R9b | the four template decisions | §3 |
| R10 | the budget table as one artifact (D6) | §4 |
| R11 | the five `max_lines: none` rows (OQ1) | §4's last row |

**Seat's calls, disclosed.** `reviews/` → `reports/`; `reports/evidence/` and `landing/` forbidden;
the `report-envelope` mint; the section each rule lands in; the `reword-rule` route for the re-key;
segment tokens per home; every budget number in §4, which the user then ratifies as one table.

## 8. Plan-minimalism ladder

- **21 homes — required.** One per distinct path pattern; fewer needs a token matching two shapes.
- **`reports/` collapse — simpler shape.** 172 patterns to one rule per home, against declaring 172 names.
- **`report-envelope` template — required.** The `reports:` binding cannot resolve without it, and the binary may not read the shipped markdown.
- **Splitting `architecture-store` — simpler shape.** One template per file is what the `home` binding already assumes.
- **`set-document-field` op — cut, required rung fails.** A crate change against a frozen crate; the cost is migration size, which is diffable.
- **Root-docs home — cut, simpler shape.** Location has no wrong value to catch, and a closed root set would deny the consuming project's own files.
- **`benchmarks` home — cut, minimum now.** No command mints it; propose out of scope rather than invent a shape.
- **Five `max_lines: none` rows — cut, minimum now.** A count the feature owns is not a verbosity lever (OQ1).
- **Fixing the `tasks` skeleton — required.** The skeleton is what a producer copies; declaring its defect would ratify F10's drift.

## 9. R4, answered — and the lead's three build notes

**R4 (lead, 2026-09-13).** The brainstorm home declares `reports/` only: no `wave<n>-reports` subdir
and no second report convention. `wave<n>-<slug>.md` as a patterned deliverable, plus the
`inputs`/`research`/`referents` subdirs, are accepted. The physical re-home of the 47
`wave<n>-reports/` files across both repos is **not in this migration** — it belongs to the wave-5
violator pass (D11: split or re-homed by ruling, the consumer's plugin still at the pre-gate
version), which runs on mochiko's own tree too, with the pointer fixes in the records and the session
index part of that pass. So `0005` is declarative and the move is one scripted act before the gate
goes live. Note that this session's own files, this plan included, are inside that set.

**Three notes for the build, folded above.** The repo-root disclosed hole (§2) goes into `record.md`
at wave close as a stated limit — nothing to build. The `tasks` skeleton fix rides the
`replace-document` op and takes no strip entry (§3). `check`'s cost after `0005` is measured, not
projected, and the measurement is part of the done condition (§6a).

**Still owed before the build opens:** the user's eleven rulings (§7) — nothing else. The G8
contradiction the wave-1 review surfaced is settled in the record: D4f now reads "no size check where
bounds live elsewhere; shape checks still run where a template binds; location + set always bind",
which is what wave 1 built and what §2's `bounds` column and §4's table already assume.
