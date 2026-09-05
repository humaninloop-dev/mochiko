# Strip notes — `skills/review-sufficiency`

Entry formats: `strips/README.md`. First entry set: the skill-content-schema conversion
(v0.100.0). This skill was born by ruling at v0.91.0 (plan-stage-utility D2 as amended +
Addendum) and had no strip file before this wave; per the conversion census (J-2, lead ruling
R-c) it carries **no wholesale body protection** — no `KEPT:` line exists. Its D2-ruled
machinery is `DECISIONS.md`-traceable content: each rule move below cites the `DECISIONS.md`
2026-08-26 plan-stage-retirement row (→ `.mochiko/brainstorms/plan-stage-utility/record.md`
D2 as amended), the same ceremony class as a supersession-by-ruling, with **no whole-body
claim** made.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the review family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/review-sufficiency/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action at invoke, before any grading step: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
  action.** The schema is the source of truth for this skill's binding rules; this body carries
  identity and procedure only. Its rules are nested in six sections, each addressable by its
  section ID: `review-sufficiency.sec.independence` (author/grader separation) ·
  `review-sufficiency.sec.scope` (the grading unit and the delta collapse) ·
  `review-sufficiency.sec.inputs` (the read fence and its one carve) ·
  `review-sufficiency.sec.verdict` (the gate semantics, the ten clauses, the grading floors) ·
  `review-sufficiency.sec.output` (the report contract) · `review-sufficiency.sec.reserved`
  (what only the user rules).

  Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
  `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
  the schema's declared `conditions:`, except that a `class: floor` rule is always read and
  always delivered — `when:` gates when its obligation applies, never whether it reaches you.
  Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
  `pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
  own, and the stub's `review-sufficiency.*` ID stays the citable ID; `${verdict}` in inherited
  text substitutes from this schema's `vars:`. Labels come from
  `../../schemas/skill-labels.yaml`.

  The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
  first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
  continue.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The per-section glosses this
  block carried are covered by the render, whose `sections` line prints a title per section and
  whose empty sections carry a `note:` giving the reason. The "this body carries identity and
  procedure only" clause is dropped: it stated a scope rather than an obligation, and the new
  halt paragraph states the same split. The `extends:` stub resolution and the family
  common-file co-Read are discharged by the render, which resolves every stub before the model
  sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-review-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
  first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
  continue.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.100.0] Skill-content-schema conversion — body obligations moved to `schema.yaml`; the pair is now the graded unit

Wave context: skill-content-schema D1–D9 as amended (`.mochiko/brainstorms/skill-content-schema/record.md`,
`DECISIONS.md` 2026-09-01); census referent `.mochiko/brainstorms/skill-content-schema/census.md` §B
(RSUF rows). Census-row → minted-ID map: 1 `binding-gate` · 2 `author-grader` (stub; census row 25
dedups into it per the census's own note — one minted rule) · 3 `fence-read-set` · 4 `fence-never-reads` ·
5 `clause10-carve` · 6 `unit-and-collapse` · 7 `clause9-delta-inapplicable` · 8 `every-applicable-clause` ·
9–18 the ten clauses `clause-testable-criteria` · `clause-contract-exposure` · `clause-data-exposure` ·
`clause-structural-trigger` · `clause-nfr-targets` · `clause-commodity-exposure` ·
`clause-dependency-order` · `clause-ux-trace` · `clause-delivered-feature` · `clause-in-flight` ·
19 `absent-baseline` · 20 `trips-user-ruled` · 21 `verdict-routing` · 22 `disputed-clause-gap` ·
23 `report-binding` · 24 `default-fail` (stub) · 26 `na-justified` · 27 `evidence-floor` (LOCAL rule,
not a stub — the report-specific target keeps local text per census §C C1 edge / near-dup R6; allowlist
edge) · 28 `verdict-is-input` (stub) · 29 (description never-reads copy) no rule minted — `description:`
untouched per D3. All IDs carry the `review-sufficiency.` prefix.
Structural-vs-content accounting (D8/C1): delivered pair vs pre-conversion body = **0.81×** —
relocation net of common-stub convergence (three stubs) and the row-25 dedup; structural overhead
only, no content growth.

### Relocation — gate semantics and scope machinery (rules `binding-gate` · `unit-and-collapse` · `clause9-delta-inapplicable` · `verdict-routing`)
- **Disposition:** relocated → the named `schema.yaml` rules; each move cites the `DECISIONS.md`
  2026-08-26 plan-stage-retirement row (plan-stage-utility D2 as amended).
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3).
- **Content:** "A `sufficient` verdict licenses cards and build directly; any gap scopes the in-run
  design phase to exactly the named gaps, nothing else" · "Selection scope grades **per selected work
  row** … Delta scope grades **per delta card**, and only three clauses apply: criteria testable (1) ·
  touched surfaces identified (2, 3) · store consult and trip check run (4)" · "Clause 9 does not
  apply under delta scope — the desk's delta card is itself the `[MODIFY]` instrument … a delta fix
  discovered structural re-fires the design phase rather than clearing here" · "Binding at entry — a
  gap list routes to the design phase, zero gaps routes to cards and build."
- **Consumers assessed:** none — skill-local content.

### Relocation — the read fence (rules `fence-read-set` · `fence-never-reads` · `clause10-carve`)
- **Disposition:** relocated → the named `schema.yaml` rules; each move cites the `DECISIONS.md`
  2026-08-26 plan-stage-retirement row.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3).
- **Content:** the **Fence** paragraph whole — the read set (spec incl. S&F manifest · architecture
  store · product baselines · capability map entries), the never-set ("Never the code, `tasks.md`,
  `**TEST:**` cases, cycle reports, or this batch's own `FEAT-XXX/` run-output directory — all of
  those are downstream of this verdict, and reading them makes the check circular"), and the clause-10
  carve ("the sole run-output read this fence admits, scoped to the colliding surface, and never
  extended to that feature's code, cards, or cycle reports").
- **Consumers assessed:** none — skill-local content.

### Relocation — the ten clauses (rules `clause-testable-criteria` through `clause-in-flight`, ten rules)
- **Disposition:** relocated → the ten named `schema.yaml` rules, each clause with its own gap form
  carried verbatim; each move cites the `DECISIONS.md` 2026-08-26 plan-stage-retirement row. The
  selection-only clauses (NFR targets · commodity exposure · dependency order · UX trace ·
  delivered-feature exposure · in-flight exposure) gate on the declared `scope` condition.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3).
- **Content:** the body's "## The ten clauses" numbered list, verbatim per clause, including the
  own-structure exclusion (clause 4), the no-locks rule (clause 10), and the clause-8 no-manifest n/a.
- **Consumers assessed:** none — skill-local content.

### Relocation — branch rules (rules `absent-baseline` · `trips-user-ruled`)
- **Disposition:** relocated → the named `schema.yaml` rules; each move cites the `DECISIONS.md`
  2026-08-26 plan-stage-retirement row.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3).
- **Content:** "An absent baseline file grades its touched surfaces new (gap), never n/a. The design
  phase's first duty is then the seed: empty scaffolds where no code is delivered,
  reconstruct-and-confirm with the user at the design checkpoint where delivered code exists" · "A
  store trip — a touched row standing `open` or `not-now` — never becomes a gap. It rides the verdict
  report and is dispositioned by the user at run-open: warn and record; a recorded deferral is a legal
  escape, a silent skip is not."
- **Consumers assessed:** none — skill-local content.

### Relocation — report contract (rule `report-binding`)
- **Disposition:** relocated → `report-binding`; the move cites the `DECISIONS.md` 2026-08-26
  plan-stage-retirement row.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3).
- **Content:** the "## Verdict and output" report paragraph — `sufficiency-report.md` in
  `.mochiko/features/FEAT-XXX/` under the `templates/report-format.md` envelope, with the contents
  list (per-unit verdicts · clause-keyed gap list · store-consult result and no-delta claim · trips ·
  in-flight conflicts · `[MODIFY]` amendment naming · the `quickstart.md` null-path record).
- **Consumers assessed:** none — skill-local content.

### Relocation — floors (rules `every-applicable-clause` · `disputed-clause-gap` · `default-fail` · `author-grader` · `na-justified` · `evidence-floor` · `verdict-is-input`)
- **Disposition:** relocated → the named `schema.yaml` rules; each move cites the `DECISIONS.md`
  2026-08-26 plan-stage-retirement row. `default-fail`, `author-grader`, and `verdict-is-input` bind
  the family common blocks (`extends: review-common.*`, `class: floor` local); their member tails are
  superseded by the common blocks' strongest wording per near-dup convergence R2
  (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`), the tail content surviving in the local
  rules named below. `evidence-floor` stays a LOCAL rule — the report-specific target ("land in the
  report") keeps local text per census §C C1 edge / R6; allowlist keep-distinct edge.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3/D5).
- **Content:** the "## Floors — non-waivable" list verbatim: "Defaults to FAIL — a unit is
  insufficient until every applicable clause is graded. Absence of looking is never evidence of
  sufficiency" (tail survives in `every-applicable-clause`) · "Never author what you grade — you
  authored none of the three sources, and the design phase that closes a gap is a different seat"
  (dedup with the overview's "you never author the fix" per census; the different-seat fact survives
  in the body's identity prose) · "Every clause graded, or flagged n/a with its justification — never
  silently dropped. Clause 8's no-manifest n/a and clause 9's delta-scope n/a are the only structural
  ones" · "Verdict and dispositions land in the report — evidence living only in conversation is a
  floor violation" · "Your verdict is input to routing, never a clearing — the lead routes; the user
  rules trips, in-flight conflicts, and any disputed clause" (the reservation tail survives in
  `trips-user-ruled` and `disputed-clause-gap`; the in-flight-conflict reservation in
  `clause-in-flight`) · "A disputed clause defaults to gap and the dispute goes to the user; the
  grader never clears alone" · "sufficient only when every applicable clause holds … a clause that
  cannot be graded is a gap, never a pass."
- **Consumers assessed:** family common blocks C2/C3/C4; cross-grammar near-dup edge with command
  `common.yaml` is allowlist territory (census J-5).
