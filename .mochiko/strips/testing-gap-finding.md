# Strip notes — `skills/testing-gap-finding`

Entry formats: `strips/README.md`. Skill born at v0.79.0 (the QA gap-finding build); this file
opens with the first edit that superseded any of its shipped text.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the dense-five family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/testing-gap-finding/SKILL.md`. -->

<!-- Wave context: wave 6 of the CLI schema-delivery build (v0.107.0) — the end state. No schema
file ships in the plugin: the 20 files under `plugins/mochiko/schemas/` and the 30
`skills/*/schema.yaml` were deleted, and every delivery they served now has a CLI form. Ruling for
the [v0.107.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6, with
the `DECISIONS.md` 2026-09-05 row and that session's `wave6-plan.md`. Pre-edit verbatim text:
`git show 62aa99d:plugins/mochiko/skills/testing-gap-finding/SKILL.md`. -->

## [v0.107.0] the fold-back paragraph's "live in the schema"

- **Disposition:** superseded → "are delivered by `mochiko-cli`"
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md`
  D9 wave 6; `DECISIONS.md` 2026-09-05)
- **Content:** "live in the schema; the artifact looks like this:"
- **Kept deliberately:** the sentence's subject — that the grammar, authorship, and artifact rules
  for the durable gate set are rules rather than prose — and the worked artifact example that
  follows it. Only the file the reader was pointed at changed, and this skill's own halt clause
  already forbids reading one.

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any derivation or probing step: **Read `schema.yaml` (this
  skill's own directory) raw, in full** — the small families ship no common file, so the
  pair's own schema is the whole first action. The schema is the source of truth for this
  pass's binding rules, nested in six sections, each addressable by its section ID:
  `testing-gap-finding.sec.independence` · `testing-gap-finding.sec.scope` ·
  `testing-gap-finding.sec.inputs` · `testing-gap-finding.sec.verdict` ·
  `testing-gap-finding.sec.output` · `testing-gap-finding.sec.reserved`. Interpret it live:
  a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule's
  `when:` resolves against the schema's declared `conditions:` — run scope, depth, and
  mutation-tool presence — and gates when the obligation applies, never whether it is
  delivered; a rule of `class: floor` is always read and always delivered whatever its
  `when:`; a `pointer:` rule binds you to that file's or skill's procedure, referenced never
  restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the
  9 rules of `class: floor` are non-waivable. Before the first derivation or probing step,
  state the floor count back — a skipped or partial read leaves that count blank: halt and
  surface it, and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The block's inline naming of
  this schema's own `conditions:` is covered by the render, whose preamble prints the whole
  `conditions` block with each value and its resolution note — strictly more than the sentence
  carried.
- **Consumers assessed:** none shared — the block was this skill's own text, and this family
  ships no common file.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the
  9 rules of `class: floor` are non-waivable. Before the first derivation or probing step,
  state the floor count back — a skipped or partial read leaves that count blank: halt and
  surface it, and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.103.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2C, small families)

Ruling for every entry below: skill-content-schema D3 (obligations move, procedure stays
prose) / D8/C4 (protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema
ruled · Skill-schema wave-2 family doors ruled — the small-families door: abort tripped, the
dense five convert on the B/C drivers, review six-set reused); census:
`.mochiko/brainstorms/skill-content-schema/census-small-families.md` §A (TGF) + §B (TGF rows
1–28). Schema home: `plugins/mochiko/skills/testing-gap-finding/schema.yaml`. Minted IDs
carry the `testing-gap-finding.` prefix (omitted below). Map — census §B row → minted ID:
1a `test-execution-is-testing-end-user` · 1b `test-grammar-consumed-never-redefined` ·
2 `one-pass-at-final-validation` · 3 `delta-lane-skip-stated` ·
4 `real-infrastructure-never-mocks` · 5 `blindness-fence-inclusion-list` ·
6 `spine-outside-fence` · 7 `structural-exclusions` · 8 `delegated-reads-inherit-fence` ·
9 `two-message-dispatch` · 10 `seat-devils-advocate` ·
11 `expectations-numbered-before-probing` · 12 `observability-advisory-only` ·
13 `probe-kit-breadth-invariant` · 14 `lens-alongside-never-inside` ·
15 `mutation-lens-bounds` · 16a `tool-absent-skip-noted` · 16b `flaky-suite-skip-noted` ·
17 `surviving-mutants-advisory` · 18 `tool-advisory-posture` · 19 `finding-kind-split` ·
20 `adjudication-lead-and-user` · 21a `rework-bound` · 21b `bound-exhaustion-user-ruled` ·
22 `out-of-territory-routing` · 23 `done-condition` · 24 `report-disclosure` ·
25 `zero-findings-clean-pass` · 26 `fold-back-authorship` · 27 `gates-artifact-contract` ·
28 `reviewer-mirror-checklist`. 31 rules: 28 census rows + three a/b limb splits (rows 1 and
21 per the census disposition column; row 16 split lead-ruled at plan approval so the
tool-absent duty carries `when: {mutation_tool: absent}` without stranding the
flaky-suite limb).
**Section distribution (review six-set reused by the door ruling, no empty markers —
census §B: TGF fits 6/6):** independence {10, 14, 26} · scope {1a, 1b, 2, 3, 4, 13, 15,
18, 22} · inputs {5, 6, 7, 8, 9, 11} · verdict {19, 12, 17, 23, 25} · output {16a, 16b,
24, 27, 28} · reserved {20, 21a, 21b}.
**Floor count 9 (rows 3 · 4 · 5 · 7 · 8 · 9 · 14 · 18 · 20), lead-ruled at plan
approval:** the census tally cell's "TGF 8" disagrees with its own row-grain enumeration
(nine `floor` markers); the row grain wins (fifth application of the 2B precedent), and the
tally-cell correction is queued for the census §K landing appendix.
**`conditions:` declared (census §B `when:` dims):** `run_scope`
[selection · epic · delta · product-lane] entry-derived · `depth` [low · high]
entry-derived · `mutation_tool` presence surface-presence. `when:` carriers: 2 · 3 (floor —
always delivered, applicability gated) · 15 · 16a · 16b. **`depth: low` is named by no
rule's `when:` — a deliberate absence, lead-confirmed:** the probe kit runs at both depth
levels (rule 13's own text); only the mutation lens is depth-gated. Named here and in the
audit brief per the 2B idiom.
**J2-6 POLARITY NOTE — do not "fix" `zero-findings-clean-pass`:** "Zero findings is a clean
pass — no never-zero rule" is the DELIBERATE polar opposite of the review family's
`review-common.default-fail` posture (census J2-6: D8's disclosure-based honesty mechanism
vs the grader's earned-verdict posture). The keep-distinct allowlist edge vs
`review-common.default-fail` is the wave closer's, pre-listed in census §C.
Accounting (seat-measured snapshot; the closer re-measures at the gate — the wave-1 V2/R1
lesson): body 11,053 → 5,271 (obligations out + the load-first Rules block in) + schema
14,111 = **payload 19,382** (census §F estimate ~20,200, ×1.75 vs est ×1.8); the delta over
the pre-conversion body is structural overhead (IDs, keys, section scaffolding, reading
grammar) — no content growth claimed. The skill was unbudgeted (hard-cap-only); the
conversion first-seeds its budget at the measured payload (ledger's third seeding path, no
headroom — the wave closer executes the ledger row). Description byte-untouched at 709.

## [v0.103.0] Blindness fence, exclusions, and delegation guard — protection transfers (census §A rows 1–2)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.blindness-fence-inclusion-list` (floor, binding),
  `testing-gap-finding.spine-outside-fence` (must),
  `testing-gap-finding.structural-exclusions` (floor), and
  `testing-gap-finding.delegated-reads-inherit-fence` (floor), per D8/C4; the provenance
  sidecar carries the protected status. The fence's ruled guards ride the rule texts
  verbatim in substance: "those two artifacts only, never the `FEAT-XXX/` run-output
  directory at large" and "concern rows only" (the v0.91.0 A6 guard + the v0.81.0 D12
  narrow read) in `blindness-fence-inclusion-list`; "Delegated reads inherit the inclusion
  list" (the v0.79.0 V2/F2 fence-delegation guard) in `delegated-reads-inherit-fence`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01
  skill-content-schema rows; protecting lineage `DECISIONS.md` 2026-08-19
  qa-gap-finding-verification D3, 2026-08-19 product-architecture-schema D12 fence guard,
  2026-08-26 plan-stage-utility D3/A6).
- **Content:** the `## The blindness fence` section's inclusion-list paragraph, the
  spine-exclusion sentence, the structural-exclusion line, and the delegated-reads
  paragraph — moved whole into the four rule texts, wording preserved in substance
  (the inclusion list verbatim member-for-member).
- **Kept deliberately:** the fence's anchoring rationale stays body prose in the Overview
  ("Sight of the declared cases anchors the hunt on existing coverage … the fence and its
  dispatch order exist to prevent exactly that") — teaching, not obligation.
- **Consumers assessed:** the router's `testing-gap-finding` row restates the fence list
  (re-keyed at v0.91.0) and names the skill as its single source — intact; `implement.md`
  dispatches the pass and names the fence by reference, not by list; `mochiko:qa-engineer`
  and `mochiko:devils-advocate` mount the skill without restating the list.

## [v0.103.0] Two-message dispatch, seat, expectations duty, and scope carve — protection transfers (census §A rows 3, 9, 10)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.two-message-dispatch` (floor, duty),
  `testing-gap-finding.expectations-numbered-before-probing` (must, duty),
  `testing-gap-finding.seat-devils-advocate` (must, binding),
  `testing-gap-finding.one-pass-at-final-validation` (must,
  `when: {run_scope: [selection, epic]}`), and `testing-gap-finding.delta-lane-skip-stated`
  (floor, duty, `when: {run_scope: [delta, product-lane]}`), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting
  lineage `DECISIONS.md` 2026-08-19 qa-gap-finding-verification D2/D3/D4-as-amended-I1).
- **Content:** the `## Run scope and placement` section (one-pass placement, the three
  scope bullets, real-infrastructure line — the latter to
  `testing-gap-finding.real-infrastructure-never-mocks`, floor), the two-message dispatch
  numbered pair, the seat line ("Persona carries the judgment, this skill carries the
  procedure"), and the expectation-derivation lead-in ("enumerate expected behaviors as a
  numbered list — the numbering makes the done condition's count auditable").
- **Kept deliberately:** the five derivation families stay body prose whole (the
  expectation-derivation narrative, census-named procedure); the probe-kit table stays
  whole; its depth line moved to `testing-gap-finding.probe-kit-breadth-invariant`.
- **Consumers assessed:** `implement.md` dispatches the two-message pass by reference;
  `mochiko:devils-advocate` is mounted as the seat, unchanged.

## [v0.103.0] Mutation lens set — protection transfers (census §A row 7)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.lens-alongside-never-inside` (floor),
  `testing-gap-finding.mutation-lens-bounds` (must, bound,
  `when: {depth: high, mutation_tool: present}`),
  `testing-gap-finding.tool-absent-skip-noted` (must, duty,
  `when: {mutation_tool: absent}`), `testing-gap-finding.flaky-suite-skip-noted` (must,
  duty, `when: {mutation_tool: present}`),
  `testing-gap-finding.surviving-mutants-advisory` (must), and
  `testing-gap-finding.tool-advisory-posture` (floor — the GI-019 carve-out language),
  per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting
  lineage `DECISIONS.md` 2026-08-19 qa-gap-finding-verification D5/D10).
- **Content:** the `## The mutation lens` section's five obligation bullets and the
  advisory-posture closing paragraph ("never gates progress, never dispatches agents,
  never holds judgment this skill owns").
- **Kept deliberately:** the lens's grey-box teaching sentence and the tool-class-per-stack
  line (cargo-mutants · mutmut · Stryker) stay body prose.
- **Consumers assessed:** the verification seat mounts the lens through this skill;
  no consumer restates the bounds.

## [v0.103.0] Finding split, adjudication, rework bound, routing — protection transfers (census §A rows 4–5)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.finding-kind-split` (must),
  `testing-gap-finding.observability-advisory-only` (must),
  `testing-gap-finding.adjudication-lead-and-user` (floor, reservation),
  `testing-gap-finding.rework-bound` (must, bound),
  `testing-gap-finding.bound-exhaustion-user-ruled` (must, reservation), and
  `testing-gap-finding.out-of-territory-routing` (must, routing), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting
  lineage `DECISIONS.md` 2026-08-19 qa-gap-finding-verification D6).
- **Content:** the `## Findings — split by kind, never by severity` section's two kind
  definitions, the adjudication paragraph, the rework-bound paragraph, and the
  out-of-territory routing paragraph — moved whole; the a/b split at row 21 separates the
  bound (charter mechanics) from the reservation (user disposition).
- **Consumers assessed:** `implement.md` carries the run-side gap-rework bound in its own
  schema (`impl.gap-rework-bound` — its command states it for the run; this skill states
  the pass side); the lead's checkpoint verdict and the user gate are unchanged.

## [v0.103.0] Done condition, disclosure, zero-clean polarity — protection transfers (census §A row 8)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.done-condition` (must, duty),
  `testing-gap-finding.report-disclosure` (must, binding), and
  `testing-gap-finding.zero-findings-clean-pass` (must — the J2-6 polarity note in the map
  entry above governs its audit reading), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting
  lineage `DECISIONS.md` 2026-08-19 qa-gap-finding-verification D8).
- **Content:** the `## Done condition and disclosure` section whole — the done sentence,
  the four disclosure fields, and "Zero findings is a clean pass — no never-zero rule, no
  quota. The disclosure is the honesty mechanism, not the finding tally."
- **Consumers assessed:** the final-validation report contract is consumed by the
  implement run's checkpoint; field set unchanged.

## [v0.103.0] Fold-back authorship + `gates.md` artifact contract — protection transfers (census §A row 6)

- **Disposition:** superseded — protection transfers to
  `testing-gap-finding.fold-back-authorship` (must — QA craft authors, never the
  exploratory seat; as-designed does not fold) and
  `testing-gap-finding.gates-artifact-contract` (must, binding — mint at first fold or at
  card authoring inside the implement run, survives graduation, union read), per D8/C4.
  The v0.91.0 mint-moment re-key wording is preserved verbatim in substance.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting
  lineage `DECISIONS.md` 2026-08-19 qa-gap-finding-verification D7 + 2026-08-26
  plan-stage-utility mint-moment re-key, this file's [v0.91.0] entries).
- **Content:** the `## Fold-back — the durable gate set` section's authorship paragraph
  and artifact paragraph.
- **Kept deliberately:** the `gates.md` example block stays body prose (teaching); the
  boundary sentences of the Overview's `**Boundary.**` paragraph moved to
  `testing-gap-finding.test-execution-is-testing-end-user` and
  `testing-gap-finding.test-grammar-consumed-never-redefined` (census row 1a/1b), the
  "This skill owns the pass" identity line folding into the Overview.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` owns the `**TEST:**` grammar and
  the card-authoring moment (both referenced, never restated); `mochiko:qa-engineer` is
  named as the folding craft, unchanged; the router row names `gates.md` without its mint
  moment.

## [v0.103.0] Quality Checklist deleted — boxes mapped row-wise (census §B row 28)

- **Disposition:** relocated → `testing-gap-finding.reviewer-mirror-checklist` (the census
  set-rule) + the rules its boxes mirror; the section leaves the body. Box map: 1 scope/skip
  = rows 2/3 · 2 message-1 fence = row 9 · 3 delegated reads = row 8 · 4 numbered
  expectations = row 11 · 5 breadth = row 13 · 6 lens/skip noted = rows 15/16a/16b · 7 kinds
  with evidence = row 19 · 8 lead confirms/disputes to user = row 20 · 9 out-of-territory
  = row 22 · 10 done condition = row 23 · 11 disclosure = row 24 · 12 fold-back/as-designed
  = row 26. Items dedup against the rules per the census disposition; no box is lost.
  The `## When NOT to Use` section slims in the same edit: the per-cycle, delta/lane, and
  re-running bullets dedup to rows 2/3/1a; the a11y and property-based decline bullets stay
  body prose (no census row — recorded declines, lead-confirmed at plan approval).
- **Tier failed:** 1 (mirror restatement of the schema's own rules).
- **Content:** the 12 checklist boxes and the three deduped When-NOT bullets, verbatim in
  git history (pre-v0.103.0).
- **Consumers assessed:** no consumer references the checklist boxes or the When-NOT
  bullets; the Anti-patterns table stays whole.

## [v0.91.0] Fix round 2 — the durable gate set's mint moment re-keyed off plan time (V1 multiline sweep)

- **Disposition:** superseded → "minted at first fold (or at **card authoring inside the
  implement run**)".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1: cards are authored inside the
  implement run — after the design phase, or directly on a zero-gap sufficiency verdict — never
  in a separate plan run). Caught by V1's **multiline-aware** sweep: the phrase wrapped across
  two lines as "at plan\ntime", so every line-scoped grep in this wave — including the
  twenty-term sweeps this seat ran — missed it.
- **Content (superseded text, verbatim, with its line break):**

  ```
  **The artifact:** `.mochiko/features/FEAT-XXX/gates.md` — minted at first fold (or at plan
  time, when the cards are authored) and **surviving graduation**:
  ```

- **Kept deliberately:** the whole artifact contract — `gates.md` mints at first fold, **survives
  graduation** (work rows vanish, the gate set persists), and is the named read source of the
  accumulated territory `**TEST:**` gates, read at every later final validation as the union of
  the territory features' `gates.md` plus their cards' cases. The parenthetical's *point* is
  unchanged: the file can also come into being when the cards are authored, ahead of any fold —
  only the name of that moment moved.
- **Budget:** unbudgeted (hard-cap-only). Body 11,052 → **11,053**; description unchanged at 709,
  inside the 1,536 cap. (Canonical-snippet count taken after the edit, not estimated.)
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` owns card authoring and was re-keyed to
  the same moment in the main pass ("inside the `/mochiko:implement` run — after the design
  phase, or directly on a zero-gap sufficiency verdict"); the router's `testing-gap-finding` row
  names the fold-back target `gates.md` but not its mint moment — no re-key owed there.

**Sweep lesson, recorded for the next wave:** two of this wave's misses (this one and
`authoring-epic`'s Why-together line) were invisible to line-scoped greps — one because the
phrase wrapped mid-sentence, one because it named a mechanism rather than a stage. A
vocabulary-retirement sweep should run multiline-aware (`grep -Pzo` or equivalent) and include
the *mechanism* names a retirement kills, not only the stage names.

## [v0.91.0] Blindness-fence inclusion list: the `requirements.md` member re-keyed to the sufficiency report + design-phase deltas — plan-stage retirement D3, Addendum A6

- **Disposition:** superseded → two named members in its place — the run's **sufficiency report**
  and the feature's **design-phase deltas**.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3 fence consequence: "`requirements.md`
  is a named member of `mochiko:testing-gap-finding`'s explicit inclusion list; its slot re-keys
  to the sufficiency report + the design-phase deltas (spec-layer artifacts, never code); the
  narrowing is recorded, its adequacy watched (Open questions)"; plus Addendum **A6**: "map
  entries live at `.mochiko/features/FEAT-XXX-<slug>.md`; the `FEAT-XXX/` directory is run
  output, out of fence").
- **Content (superseded text, verbatim):**

  ```
  not a layer label: `spec.md` (FR-XXX, SC-XXX, stories, declared edge cases) · the feature's
  `requirements.md` · Screens & Flows (SCR-XXX, FLOW-XXX) · `data-model.md` (entities, state
  ```

  Replaced by the same list with the `requirements.md` member swapped for "the run's
  **sufficiency report** and the feature's **design-phase deltas** — those two artifacts only,
  never the `FEAT-XXX/` run-output directory at large".
- **A6 application, recorded because it is a judgment call an auditor will check:** the fence
  names no map-entry path today, so A6's map-entry half is a no-op here. Its operative half is
  the run-output distinction, and it bites on the *new* members: both replacements live under
  `.mochiko/features/FEAT-XXX/`, which A6 declares out of fence as a directory. The fence
  therefore admits the **two named artifacts only** and says so explicitly, rather than
  admitting the directory that contains them. Without that guard the re-key would have widened
  the fence to the cards and cycle reports the pass is structurally blind to.
- **Budget:** the skill is unbudgeted (hard-cap-only). Body 10,929 → 11,052 chars; description
  unchanged at 709. No budget obligation; the ≤1,536 description cap holds.
- **Kept deliberately:** every other inclusion-list member (`spec.md`, Screens & Flows,
  `data-model.md`, `contracts/`, the store's concern rows for their `NFR-XXX` targets), the
  spine-stays-outside-the-fence guard, the whole structural exclusion list (code, cards,
  `**TEST:**` cases, cycle reports, the builder's tests), the delegated-reads-inherit-the-list
  rule, and two-message dispatch. The narrowing is one member wide.
- **Open watch (from the ruling, not discharged here):** whether the sufficiency report plus the
  design-phase deltas carry what `requirements.md` carried for the blind explorer's expectation
  derivation. The record parks this in Open questions, to be watched at the first gap-finding
  pass under the new shape.
- **Consumers assessed:** the router's `testing-gap-finding` row restates the fence list and was
  re-keyed in the same wave; `mochiko:patterns-vertical-tdd` owns the `**TEST:**` grammar this
  skill consumes (untouched); `implement.md` (P1's rewrite dispatches the pass at final
  validation).

## [v0.81.0] Runtime-NFR references re-pointed from `nfrs.md` to the store's concern rows — product-architecture-schema D12

- **Disposition:** superseded → the architecture store's concern rows
  (`.mochiko/product/architecture/concerns.md` plus any graduated
  `concerns/AX-XXX-<slug>.md`), which now home the `NFR-XXX` targets. `nfrs.md` dies as a file;
  the ids and the targets survive, so all three references move rather than drop.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12, `Contested` — the absorb
  names `testing-gap-finding`/gates runtime-NFR re-points among its added consumer rewires;
  `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — three fragments):**

  ```
  `contracts/` (`api.yaml` and any sibling contract documents) ·
  `nfrs.md`. All define externally-observable promised behavior, so the pass stays black-box.
  ```

  ```
  4. **Runtime NFR** — each `nfrs.md` numeric target (p95, availability, limits) as a measurable
     expectation against the built system.
  ```

  ```
    broken, with **evidence captured** and the **spec clause cited**. A broken `nfrs.md` numeric
    target qualifies.
  ```
- **Kept deliberately:** the blindness fence's shape and every other admissible input verbatim
  (`spec.md` · the feature's `requirements.md` · Screens & Flows · `data-model.md` ·
  `contracts/`); the explicit-inclusion-list-not-a-layer-label rule; the structural exclusions
  (code · cards · `**TEST:**` cases · cycle reports · the builder's tests); two-message dispatch;
  the finding-kind split, which still makes a broken numeric target a **spec-violation** — the
  clause it cites moved home, its blocking force did not.
- **Fence guard (addition riding the decision row, no strip):** the inclusion admits the store's
  **concern rows only**. The **spine deep view is excluded** — it is design structure, not
  externally-observable promised behavior, and the per-feature `architecture.md` it replaces was
  never an admissible input either. Admitting the whole store would have silently widened the
  fence that D12's re-point had no mandate to widen; the narrow read keeps the pass black-box.
- **Consumers assessed:** the router's `testing-gap-finding` row restates the fence inclusion
  list verbatim and is re-keyed in the same edit set by this seat (P4). `implement.md` (P2)
  dispatches the pass and names the fence by reference, not by list. `mochiko:qa-engineer` and
  `mochiko:devils-advocate` mount the skill without restating the list (grep clean). The skill's
  own `description:` names the ownership set, never the inclusion list — unchanged, 709 chars.
