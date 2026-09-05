# Strip notes — `skills/authoring-technical-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T2;
ratified 2026-07-24).

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the authoring family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/authoring-technical-requirements/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any authoring: **Read `schema.yaml` (this skill's own directory)
  and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same declared first
  action** — schema, then common. The schema is the source of truth for this skill's binding
  rules, nested in six sections, each addressable by its section ID:
  `authoring-technical-requirements.sec.independence` ·
  `authoring-technical-requirements.sec.scope` ·
  `authoring-technical-requirements.sec.inputs` ·
  `authoring-technical-requirements.sec.artifact` ·
  `authoring-technical-requirements.sec.output` ·
  `authoring-technical-requirements.sec.reserved`. Interpret it live: a rule's `kind:` names
  what it is, and an absent `kind:` reads `constraint`; a `pointer:` rule binds you to that
  file's or skill's procedure, referenced never restated; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
  `extends: authoring-common.<slug>` inherits text/labels/pointer from
  `skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `authoring-technical-requirements.*` ID stays the citable ID. The floor pin:
  the 8 rules of `class: floor` are non-waivable. Before the first authoring step, state the
  floor count back — a skipped or partial read leaves that count blank: halt and surface it,
  and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The `extends:` stub resolution
  and the family common-file co-Read are discharged by the render, which resolves every stub
  before the model sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-authoring-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin:
  the 8 rules of `class: floor` are non-waivable. Before the first authoring step, state the
  floor count back — a skipped or partial read leaves that count blank: halt and surface it,
  and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A)

Ruling for every [v0.101.0] entry: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers), `DECISIONS.md` 2026-09-01 rows (skill-content-schema + the wave-2
family-door ruling); census: `.mochiko/brainstorms/skill-content-schema/census-authoring.md`
§B (ATR). Schema home: `plugins/mochiko/skills/authoring-technical-requirements/schema.yaml`.
Minted IDs carry the `authoring-technical-requirements.` prefix (omitted below). Map —
census row → minted ID:
1 `business-source-floor` · 2a `deliverable-envelope` (C-A4 stub) + 2b
`statement-carries-content` (lettered split; carries the "Density is not a gap" tail) ·
3 `design-ladder-binding` · 4 `letter-is-spirit` (C-A1 stub) · 5a
`business-requirements-routing` · 5b `decision-technique-routing` · 5c
`problem-space-not-solutions` · 5d `boundaries-not-preferences` · 5e `slicing-routing` ·
6 `layer-prefixes` · 7a `integration-boundary-downstream` · 7b `sensitivity-downstream` ·
8a `constraints-are-facts` · 8b `impact-references-decisions` · 9
`no-exceptions-documentation` · 10 `decision-slots-binding` · 11a `no-topology-decisions` ·
11b `entangled-decision-route` · 12 `platform-implies-ip` · 13a `nfr-store-home` · 13b
`store-write-at-sign-off` · 14 `fast-is-not-a-requirement` · 15 `integration-declarations` ·
16 `sensitivity-declarations` · 17a `trace-nfr-to-business` · 17b
`trace-constraint-decision` · 17c `trace-constraint-impact` · 17d `trace-infrastructure` ·
18 `technology-agnostic` · 19 `rationalization-stop` · 20 `sequential-ids` · 21
`declaration-rows-complete` · reference stubs (both reference files untouched, stubs
point): 22 `field-definitions` · 23 `never-compress-register` · 24 `declaration-closure` ·
25 `completeness-procedures` · 26 `grade-is-not-self-asserted` · 27
`layer-internal-consistency`.
Build count 39 vs the census header's 37: the §B table itself enumerates 32 body rows
against its own header's 31, and 2a/2b is an approved lettered split — disclosed for the
audit (census build-corrections appendix at landing). C-A3 membership is reference-side by
ruling (wave-lead F1): ATR mints no two-arm body stub — the two-arm store-row form sits in
`references/ARTIFACT-TEMPLATES.md` §2 and rides `field-definitions`' pointer, the wave-1
RF/C5 mirror. TR-death guard (v0.91.0 D3): no TR-XXX rule resurfaces in the schema —
verified at mint, nothing to mint.
Accounting: body 10,500 → 3,204 + schema 16,742 = payload 19,946 (census estimate
~23,900); the retained body is the load-first Rules block plus teaching prose (artifact
walk, Common Rationalizations table); every removed line is a relocation or transfer
recorded below — no content growth claimed. Description byte-untouched at 598.

## [v0.101.0] [v0.28.0]/[v0.23.0] kept-sets — protection transfers to schema rules (D8/C4)

- **Disposition:** superseded — protection transfers per D8/C4, each rule inheriting
  protected status in the provenance sidecar: letter/spirit epigraph → `letter-is-spirit`
  (C-A1 stub, floor) · "'Fast' is not a requirement" → `fast-is-not-a-requirement` (floor;
  the no-deferrals no-exceptions clause rides the same rule) · "constraints are facts" →
  `constraints-are-facts` · the documentation no-exceptions line →
  `no-exceptions-documentation` (floor) · the Red-Flags STOP no-exceptions tail →
  `rationalization-stop` (floor) · Traceability Rules mandatory links →
  `trace-nfr-to-business` / `trace-constraint-decision` / `trace-constraint-impact` /
  `trace-infrastructure` · the T2 statement-line form → `statement-carries-content` (body
  arm) + `field-definitions` (reference arm — the T2 field forms stay in
  `references/ARTIFACT-TEMPLATES.md`, untouched) · the three-artifact spine →
  `layer-prefixes` plus the body's retained artifact-walk prose.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01
  skill-content-schema row; original protections: 2026-07-25 skill-succinctness-strip +
  2026-07-24 token-reduction-wave-2 rows).
- **Content:** the kept-set enumerations in the [v0.28.0] and [v0.23.0] entries below —
  each named line now lives verbatim-or-strengthened in its schema rule's `text`.
- **Kept deliberately:** the Common Rationalizations table stays in the body whole
  (teaching, census §B preamble); the fenced worked example homes stay reference-side.
- **Consumers assessed:** the [v0.28.0] wave-open enumeration stands — no consumer links
  the moved lines by anchor; the router row is unchanged.

## [v0.101.0] Quality Checklist — dedup-to-schema (lead F4 ruling; [v0.28.0] KEPT member)

- **Disposition:** superseded — dedup-to-schema: each row mirrors a minted rule, no extra
  rule minted. Row → ID map: source/type/severity → `field-definitions` · C-XXX ↔ D-XXX →
  `trace-constraint-decision` · no-topology-D-rows → `no-topology-decisions` · NFR
  target+method+source on its store row → `fast-is-not-a-requirement` + `nfr-store-home` +
  `trace-nfr-to-business` · IP coverage → `platform-implies-ip` + `trace-infrastructure` ·
  INT/DS rows present with downstream home → `declaration-rows-complete` ·
  cross-references consistent → `layer-internal-consistency` · technology-agnostic →
  `technology-agnostic` · sequential IDs → `sequential-ids`. The before-finalizing duty
  itself lives in `completeness-procedures`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content (verbatim):**

  ```
  - [ ] Every constraint has a source, type, and severity classification
  - [ ] Every decision references the constraints that shaped it (C-XXX ↔ D-XXX)
  - [ ] No topology choice recorded as a D-XXX row here — structural rulings live in the architecture store's delta
  - [ ] Every NFR has a numeric target AND measurement method AND its FR-XXX / SC-XXX source, on its store concern row (never a standalone `nfrs.md`)
  - [ ] Every constraint implying platform provisioning has a corresponding IP-XXX
  - [ ] Every INT-XXX and DS-XXX declaration is present as a row, with its downstream home named
  - [ ] Cross-references between artifacts are consistent
  - [ ] Language is technology-agnostic (except real infrastructure constraints)
  - [ ] ID sequences are sequential with no gaps (C-001..., D-001..., IP-001..., INT-001..., DS-001...)
  ```

- **Consumers assessed:** none cites the checklist by anchor; the completeness procedures
  in `references/TRACEABILITY-PATTERNS.md` are the richer surviving home, untouched.

## [v0.101.0] [v0.81.0] D12 products — protection transfers (census ATR §A row 2)

- **Disposition:** superseded — protection transfers to `nfr-store-home` (floor),
  `no-topology-decisions` (floor), and `entangled-decision-route`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row; original
  protection: 2026-08-19 product-architecture-schema D12, `Contested`, user-ruled absorb).
- **Content (verbatim, the load-bearing lines):** "**There is no `nfrs.md`:** an NFR-XXX
  row lives as fields on its architecture-store concern row, so one concern has one home —
  stance, pattern, targets, as-built, drift together. The **ids and the grammar are this
  skill's** (numeric target · measurement method · source · category), the **row shape is
  the store's** (`plugins/mochiko/schemas/architecture-store.yaml`)." · "**Structural
  decisions are not this artifact's.** A *topology* choice — component boundaries,
  interaction style, responsibility placement — is recorded in the architecture store's
  delta, whose ruling **is** its decision record; it never becomes a D-XXX row here. …
  Where a technology decision and a topology choice are genuinely entangled, record the
  technology decision here and let the store's delta carry the shape, cross-citing by ID."
- **Consumers assessed:** `authoring-architecture-store` and the store schema carry the
  store-side halves, untouched; the negative statement "there is no `nfrs.md`" survives in
  the rule text per the v0.81.0 sweep's posture.

## [v0.101.0] [v0.91.0] D3 products — protection transfers (census ATR §A row 3)

- **Disposition:** superseded — protection transfers per D8/C4: NFR business source (R4)
  → `trace-nfr-to-business` · INT/DS re-home (R1) → `integration-declarations` +
  `sensitivity-declarations` + `declaration-rows-complete` · the design-ladder blockquote
  → `design-ladder-binding`. The TR-layer death is a conversion guard, not a rule — no
  TR-XXX rule exists in the schema (nothing to mint).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row; original
  protection: 2026-08-26 plan-stage-utility D3 + wave-lead R1/R4).
- **Content (verbatim, the load-bearing lines):** "**An NFR's source is its business
  source** — the FR-XXX or SC-XXX whose promise the target serves — so the trace chain
  resolves FR-XXX / SC-XXX → NFR-XXX." · "**Optimistic integration maps are incomplete.**
  Every external dependency fails eventually — so every INT-XXX declaration MUST carry
  through to documented failure modes and a fallback when its boundary is authored in
  `mochiko:patterns-api-contracts`." · "**Every element answers the design ladder**
  (`mochiko:patterns-plan-minimalism`) before it enters the package; the simplest-execution
  stops are disclosed by the design phase as it authors, never re-derived here."
- **Consumers assessed:** `patterns-entity-modeling` / `patterns-api-contracts` downstream
  homes unchanged and still pointed at; the [v0.91.0] entries below keep the full lineage.

## [v0.101.0] Remaining body obligations relocated to schema (D3 — grouped)

- **Disposition:** relocated → `plugins/mochiko/skills/authoring-technical-requirements/schema.yaml`,
  per the map entry above: the When-NOT-to-Use section whole (5a–e) · the
  declarations-vs-downstream blockquote's routing arms (7a/7b) · the deliverable-envelope
  paragraph (2a/2b) · §§1–4 obligation lines (6 · 8a/8b · 9 · 10 · 11a/11b · 12 · 15 ·
  16 · 20 · 21) · the NFR §2 obligations (13a/13b · 14 · 17a) · the Technology-Agnostic
  section (18) · the Traceability Rules links (17a–d) · the Red-Flags STOP paragraph (19).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** faithfully compressed — each line's operative text survives in its mapped
  rule's `text` (floors quoted verbatim in the transfer entries above); pre-conversion
  body verbatim in git history of `SKILL.md` at v0.100.0.
- **Kept deliberately:** the artifact-walk prose (authoring order + both reference
  pointers), the Common Rationalizations table, and the overview identity voice stay in
  the body; `references/` files untouched.
- **Consumers assessed:** `technical-analyst` mounts the skill (contract intact — same
  artifacts, same boundaries, now schema-carried); the router row unchanged; no consumer
  links a removed section anchor.

## [v0.91.0] Fix round — the ladder blockquote's dead "authoring proposal" re-keyed (V1 ripple)

- **Disposition:** superseded → "the simplest-execution stops are disclosed by the design phase
  as it authors, never re-derived here".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4**, which lists plan's proposal
  approval among the dead gates). Ripple of the fix round's **V1** ruling: the design-phase
  authoring proposal does not survive in any form — the sufficiency gap list is the scope
  contract, the design checkpoint the user gate.
- **Content (superseded fragment, verbatim — main-pass v0.91.0 text):**

  ```
  the simplest-execution stops are disclosed in the design phase's authoring proposal, never re-derived here.
  ```

- **Kept deliberately:** the obligation entire — every element answers the design ladder
  **before** it enters the package, and the stops are disclosed rather than **re-derived here**.
  Only the named disclosure site moved, to match the ladder skill's own surviving grammar.
- **Budget:** body 10,507 → **10,500** against the 13,285 budget (the replacement is shorter);
  description unchanged at 598 against 620. Both inside.
- **Consumers assessed:** three skills carried the same blockquote pattern —
  `mochiko:patterns-entity-modeling` and `mochiko:patterns-api-contracts` were re-keyed to the
  identical wording in this round, and `mochiko:patterns-plan-minimalism` is the ladder that
  owns the grammar. All four now agree; nothing in the library names the dead proposal.

## [v0.91.0] The mandatory FR→TR layer dies — `requirements.md` and TR-XXX retired whole — plan-stage retirement D3

- **Disposition:** superseded → deleted with no replacement artifact. The skill re-scopes to the
  `constraints-and-decisions.md` layer it already owned.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D3**: "The FR→TR layer dies as a
  mandatory artifact; it does not move to specify. No per-feature `requirements.md`. Real
  technical decisions (D-XXX, C-XXX, IP-XXX) land where they already live —
  `constraints-and-decisions.md` and the store". Rationale on the record: moving TR into specify
  "would make every spec technical, kill the deliberate business/technical layer split, and
  re-fatten specify with the same restatement this session is cutting").
- **Content (superseded text, verbatim — SKILL.md §1 whole):**

  ```
  ### 1. Technical Requirements (requirements.md) -- TR-XXX

  Map every business FR to one or more TRs, each addressing a distinct technical concern the FR implies but does not state (worked decomposition + field definitions: ARTIFACT-TEMPLATES.md).

  **No orphan TRs.** Every TR maps to at least one FR. **No unmapped FRs.** Every FR has at least one TR.

  **No exceptions:** Not for "simple" systems. Not for "obvious" mappings. Not even when the FR appears to map 1:1 — decompose anyway.
  ```

  Plus, in the same file: the Traceability mandatory link `- TR -> FR (every technical
  requirement traces to business source)`; three Quality Checklist items (`- [ ] Every FR has at
  least one TR (no unmapped business requirements)`, `- [ ] Every TR maps to at least one FR (no
  orphan technical requirements)`, `- [ ] Every TR has testable acceptance criteria`); the ID
  sequence line's TR terms; and the Common Rationalizations row `| "Requirements are
  straightforward, TRs would just duplicate FRs" | Straightforward FRs hide technical complexity.
  Decompose anyway -- translation is the job, not transcription. |`.

  In `references/ARTIFACT-TEMPLATES.md`: §1 whole (the `requirements.md` document template, the
  TR field-definition table, the four-row decomposition example, and the Writing Criteria worked
  example). In `references/TRACEABILITY-PATTERNS.md`: the TR box of the traceability web, the
  TR→FR / TR→C / TR→NFR cross-reference sections with their patterns and validation rules, both
  dependency chains' TR links, the Forward (FR→TR) and Backward (TR→FR) completeness procedures
  with the coverage-table output format, the TR examples in consistency rules 1–3, and the TRs
  column of the traceability matrix.
- **Kept deliberately:** the C-XXX / D-XXX / IP-XXX layer in full (field schemas, C↔D
  traceability, the structural-decisions carve-out to the architecture store's delta, the
  constraint-vs-preference test, constraint and infrastructure type tables); the NFR-XXX grammar
  and its store-row home; the technology-agnostic writing rule and its infrastructure-fact
  exception; the whole deliverable-envelope framing; and every non-TR Rationalizations row and
  Checklist item. **INT-XXX and DS-XXX survive** — see the next entry.
- **Budget:** body **11,285 → 10,507 chars** against the budget of **13,285** — a net shrink of
  778, comfortably inside. Description 496 → 598 against its 620 budget (see the description
  entry below). `references/` files are budget-exempt.
- **Consumers assessed (the ones inside this seat's scope were fixed; the rest are reported):**
  the router's skill row (re-keyed same wave); `mochiko:patterns-plan-minimalism`'s element list
  and rung-scope line, which named TRs explicitly (re-keyed same wave);
  `mochiko:testing-gap-finding`'s fence, whose `requirements.md` member re-keys under the same
  ruling (done same wave). **Out of scope, reported to the wave lead as open ripples:**
  `agents/technical-analyst.md` names `requirements.md` as an artifact it authors;
  `schemas/tasks.yaml`, `templates/techanalyst-report-template.md`,
  `templates/feasibility-report-template.md`, and `templates/artifact-format.md` all still name
  `requirements.md` or use `TR-012` as an ID example;
  `skills/review-feasibility/references/FEASIBILITY-LENS.md` carries a worked contradiction
  example built on `TR-009`.

## [v0.91.0] NFR source re-keyed from TR-XXX to the business source (FR-XXX / SC-XXX) — plan-stage retirement D3, ripple

- **Disposition:** superseded → an NFR's source is the FR-XXX or SC-XXX whose promise the target
  serves; the trace chain resolves `FR-XXX / SC-XXX → NFR-XXX`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3). The record is **silent** on this
  consequence; it was surfaced by this seat during the build and **ruled by the wave lead (R4)**
  before the edit, because D3 kills the upper link of a chain this skill asserts still resolves.
- **Content (superseded text, verbatim):**

  ```
  (numeric target · measurement method · source · category), the **row shape is the store's**
  (`plugins/mochiko/schemas/architecture-store.yaml`); trace chains are unchanged — TR-XXX → NFR-XXX
  still resolves, only the path moved.
  ```

  And in `references/ARTIFACT-TEMPLATES.md`: `Trace chains are unchanged: `TR-XXX → NFR-XXX`
  still resolves, and `Applies to:` still cites TR-XXX. Only the path moved.`; the Source field
  rule `On the statement line; business requirement, SLA, or stakeholder justifying the target`;
  the `Applies to` field rule `| Applies to | No | TR-XXX references | Which technical
  requirements this NFR constrains |`; and the worked example line `**Applies to:** TR-001 ·
  TR-005`.
- **Kept deliberately:** every-NFR-has-a-numeric-target, the measurement-method obligation and
  what it must name (tool, conditions, frequency), the six categories with their vague-vs-
  measurable examples, the no-`nfrs.md` rule and the store-row home, and the "'Fast' is not a
  requirement" floor. `Applies to:` survives as a field — it now cites the C-XXX or IP-XXX the
  target constrains rather than a dead TR.
- **Consumers assessed — TWO LIVE DEAD LINKS REMAIN, both outside this seat's writable scope and
  both reported to the wave lead:** `skills/authoring-architecture-store/SKILL.md` asserts
  "`TR-XXX → NFR-XXX` trace chains keep resolving", and
  `schemas/architecture-store.yaml` defines an NFR row's source as "the TR-XXX that demanded it"
  with a worked example reading "NFR-014 (from TR-021)". Both need the same re-key; neither is
  fixed here.

## [v0.91.0] INT-XXX / DS-XXX re-homed onto `constraints-and-decisions.md` — plan-stage retirement D3 as extended by the wave lead (R1)

- **Disposition:** superseded → the two declarations survive as thin rows on
  `constraints-and-decisions.md`, alongside IP-XXX, with their downstream authoring pointers
  unchanged.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3, **as extended by the wave lead's
  R1 ruling of 2026-08-26**). Recorded honestly: **D3 itself is silent on INT-XXX and DS-XXX** —
  it names only D-XXX, C-XXX and IP-XXX as landing in `constraints-and-decisions.md`. Both
  declarations lived in the dying `requirements.md`, so D3 would have deleted them by side
  effect. This seat surfaced the gap rather than dropping them; the wave lead ruled the re-home
  and undertook to disclose the D3 extension in the wave report for the user's acceptance.
- **Content (superseded text, verbatim):**

  ```
  ### 4. System Integrations -- INT-XXX (thin analysis declaration)

  At the analysis layer, flag **which** external systems the feature depends on and **how critical** each is — an INT-XXX declaration that becomes a technical requirement ("the feature MUST integrate with `<system>`; its unavailability is `<criticality>`").
  ```

  ```
  ### 5. Data Sensitivity -- DS-XXX (thin analysis declaration)

  At the analysis layer, flag **which** data the feature treats as sensitive — a DS-XXX declaration that becomes a technical requirement ("the feature handles `<data>`, which is sensitive and MUST be classified and protected").
  ```

  Both re-keyed from "an analysis-layer declaration that becomes a technical requirement" to "a
  row on `constraints-and-decisions.md`". The declaration text itself is otherwise unchanged.
- **Kept deliberately — this is the point of the re-home:** both load-bearing obligations
  survive intact. INT-XXX still carries "every external dependency fails eventually — so every
  INT-XXX declaration MUST carry through to documented failure modes and a fallback when its
  boundary is authored"; DS-XXX still carries the requirement that sensitive data is present and
  must be governed. Both downstream homes are untouched: the per-endpoint `x-integration`
  boundary in `mochiko:patterns-api-contracts`, the per-attribute classification in
  `mochiko:patterns-entity-modeling`. The declare-here-author-there split is unchanged.
- **Additions riding this ruling (no strip owed, recorded for the auditor):** a Part 4 document
  template and a Declarations field-definition table in `references/ARTIFACT-TEMPLATES.md`, an
  INT/DS declaration-closure cross-reference section and completeness procedure in
  `references/TRACEABILITY-PATTERNS.md`, and one Quality Checklist item — the new rows needed a
  shape, and the closure check is what keeps a declaration from being written and then orphaned.
- **Consumers assessed:** the router row (re-keyed same wave, and it names the INT/DS
  declarations explicitly); `mochiko:patterns-entity-modeling` and
  `mochiko:patterns-api-contracts` (canonical downstream homes — both unchanged and still
  correctly pointed at).

## [v0.91.0] `description:` re-scoped to the constraint layer — plan-stage retirement D3

- **Disposition:** superseded → the same description naming `constraints-and-decisions.md` as
  the authored artifact and the design phase as the firing site.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3).
- **Content (superseded text, verbatim):**

  ```
  This skill MUST be invoked when authoring the technical-requirements layer of a feature spec — decomposing FRs into technical requirements (TR-XXX), hard constraints (C-XXX), measurable NFRs (NFR-XXX), and infrastructure-provisioning requirements (IP-XXX), traced to a business source. SHOULD also invoke on 'TR-', 'C-', 'NFR-', 'IP-', or 'technical requirements'. Owns the constraints-and-decisions.md structure (D-XXX schema) — NOT the decision technique (mochiko:patterns-technical-decisions).
  ```

- **Budget:** description-class edit, canonical snippet: **496 → 598 chars** against the
  recorded budget of **620**. Inside budget; no justification owed.
- **Kept deliberately:** the traced-to-a-business-source obligation, the artifact-structure
  ownership, the NOT-the-decision-technique boundary to `mochiko:patterns-technical-decisions`,
  and the `'C-'`/`'NFR-'`/`'IP-'` trigger phrases. The `'TR-'` trigger and the 'technical
  requirements' phrase were dropped — they name a retired artifact, so keeping them would route
  work to a skill that no longer authors it; `'D-'` and 'technical constraints' were added in
  their place.
- **Consumers assessed:** the router row (re-keyed same wave); `agents/technical-analyst.md`
  mounts this skill and describes its artifacts as including `requirements.md` — **out of this
  seat's writable scope, reported to the wave lead**.

## [v0.81.0] `nfrs.md` dies as a file; the Structural Decisions subsection dies whole — product-architecture-schema D12

- **Disposition:** superseded → the architecture store. Two independent absorbs land in one edit:
  (a) **NFR-XXX** loses its `nfrs.md` document home and becomes fields on the store's concern
  rows — the **ids and the grammar stay this skill's**, the row shape is the store schema's;
  (b) the **Structural Decisions** subsection dies whole — structural-origin D-XXX are replaced by
  the store delta, whose ruling **is** the decision record, so the `Origin` axis has nothing left
  to distinguish and goes with it.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12, `Contested` — user ruled
  absorb against the lead's coexist recommendation, blast radius priced and accepted;
  `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — SKILL.md):**

  ```
  **Structural Decisions section (architecture-time D-XXX rows).** `constraints-and-decisions.md`
  carries a designated **Structural Decisions** subsection inside Section 2, grouping the D-XXX rows
  that record *topology* choices — component boundaries, interaction style, responsibility placement —
  decided during the architecture stage. These rows are authored by the **architecture seat**
  (`mochiko:patterns-system-design`), **not** this skill's analysis-time author, who keeps its own
  technology-decision D-XXX rows and **preserves** this section rather than filling it. Both origins
  share one D-XXX field schema, the same ADR discipline (`mochiko:patterns-technical-decisions`), and
  one continuous D-XXX sequence; the architecture delta summary links each structural change to its row
  here. (Template + the `Origin` marker: ARTIFACT-TEMPLATES.md.)
  ```

  ```
  ### 3. Non-Functional Requirements (nfrs.md) -- NFR-XXX

  Define measurable quality attributes. Every NFR has a numeric target. Field schema in ARTIFACT-TEMPLATES.md.
  ```

  ```
  - [ ] Architecture-time topology decisions live as D-XXX rows in the **Structural Decisions** section (authored by the architecture seat; the analysis-time author preserves it, never fills it)
  ```

  Plus the Overview's "three traceable analysis artifacts" line and the layer-enumeration line
  naming `nfrs.md (NFR-XXX)`.
- **Content (superseded, verbatim — references/ARTIFACT-TEMPLATES.md):** the whole
  `### Structural Decisions` block (its explanatory blockquote + the `### D-004: [Structural
  Decision Title]` record template with Context / Options / Choice / Consequences / Governance
  alignment); the `Origin` column and its `| D-004 | Avatar processing placement | async worker
  off a queue | NFR-002 | structural |` example row in the Decision Summary index; the `Origin`
  field-definition row and the "one shared sequence across both origins" clause on the `ID` row;
  and the `nfrs.md` **document template** (the `# Non-Functional Requirements: {feature_id}`
  header, the NFR Summary ID-index table, and the `## NFR-001` record skeleton).
- **Kept deliberately:** the **entire NFR grammar** — the field definitions table (ID · Title ·
  Category · Source · Requirement · Target · Measured · Applies to), the six categories with
  their bad-vs-measurable examples, the "Writing Measurement Methods" guidance and its worked
  example (reframed from a document section to a row's fields), and the `"Fast" is not a
  requirement` floor with its no-exceptions clause. **Trace chains preserved**: `TR-XXX →
  NFR-XXX` resolves unchanged, `Applies to:` still cites TR-XXX — D12 moves the path, never the
  ids. **C-XXX hard constraints and IP-XXX rows stay** in `constraints-and-decisions.md` (D12
  reduces that artifact, it does not kill it), as does the whole technology-decision D-XXX record
  format, the ADR-technique pointer to `mochiko:patterns-technical-decisions`, and the C↔D / IP
  traceability rules. `quickstart.md` is untouched — user-ruled keep.
- **Replacement guidance added (rides the decision row):** a short "structural decisions are not
  this artifact's" paragraph replacing the deleted subsection, naming where topology rulings go
  and how to handle a technology decision entangled with one (record the technology decision
  here, let the delta carry the shape, cross-cite by ID) — the deletion would otherwise leave the
  author with no route for the entangled case.
- **Consumers assessed:** `mochiko:review-plan-artifacts` graded `nfrs.md` with a standalone
  checklist and graded the structural-D-XXX links in its Architecture checklist — both re-keyed
  in the same edit set by this seat. `mochiko:review-feasibility`'s classes 1–5 cite C-XXX /
  D-XXX / NFR-XXX by id only, and stay correct (its architecture-pass seam re-keyed by this
  seat). `mochiko:testing-gap-finding` runtime-NFR probes re-pointed by this seat.
  `mochiko:patterns-technical-decisions` owns the decision *technique* and names no origin axis —
  verified unedited. `mochiko:patterns-system-design` (P3) no longer authors rows into this
  artifact. `plan.md` / `implement.md` baseline lists and the `feature-sizing` D9/D15
  baseline-set clauses are P2's this wave.
- **Consumers assessed — second pass (V4 delta, B2 + B2-extension).** The first pass swept skills
  and the router but missed the artifact-chain carriers that name this skill's outputs by
  filename. Five further `nfrs.md` sites re-keyed, all listing-class (an enumeration of produced
  or reviewed artifacts, no behavior stated):
  `templates/artifact-format.md:5` (the deliverable envelope's pipeline artifact chain) ·
  `templates/feasibility-report-template.md:15` (also logged under `strips/review-feasibility.md`) ·
  `templates/techanalyst-report-template.md:16` (`produced:` example, gains "+ store-delta NFR rows
  when touched") · `agents/technical-analyst.md:23` (the skill's own pointer, which now states the
  store home explicitly rather than merely dropping the filename) ·
  `schemas/tasks.yaml:23,103` (the generated-from provenance pointer, stated twice — the `check:`
  string and the rendered example; logged in `strips/tasks-template.md`, that schema's strip
  lineage). Router `technical-analyst` row re-keyed the same pass (drops the NFRs limb, "approved
  architecture" → signed store delta). **Post-fix sweep:** no `nfrs.md` reference survives in
  `plugins/` except deliberate negative statements ("there is no `nfrs.md`") and the D16
  migration-source lists in `commands/architecture.md` and `authoring-architecture-store`, which
  must name the absorbed file to reconstruct from it.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 11,255 → 10,628 chars (−6%); description 1,001 → 496
  chars (−50%). Body cut: the **When to Use** section deleted whole (six bullets restating the
  description's invocation conditions — TR-XXX / C-XXX / NFR-XXX / IP-XXX authoring plus the
  INT-XXX and DS-XXX analysis-declaration bullets; each obligation survives in §4 System
  Integrations and §5 Data Sensitivity respectively, and in the three-artifact spine). Description
  cut: the trigger-phrase enumeration trimmed and the trailing "Produces requirements.md,
  constraints-and-decisions.md, and nfrs.md from a business specification" sentence dropped; the
  MUST clause, core triggers, and the constraints-and-decisions.md-ownership +
  `patterns-technical-decisions` sibling distinction kept. Verbatim homes: git history of this
  file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when authoring the technical-requirements layer of a feature specification — decomposing business functional requirements into technical requirements (TR-XXX), documenting hard constraints (C-XXX), defining measurable non-functional requirements (NFR-XXX) with numeric targets, and specifying infrastructure-provisioning requirements (IP-XXX), each traced to a business source. SHOULD also invoke when the work involves "TR-", "C-", "NFR-", "IP-", "technical requirements", "hard constraints", "non-functional requirements", "infrastructure provisioning", or authoring the constraints-and-decisions.md artifact and its C↔D / IP traceability. This skill owns the constraints-and-decisions.md artifact structure (the D-XXX field schema) and traceability — NOT the decision-evaluation technique (use mochiko:patterns-technical-decisions to evaluate alternatives and write ADRs). Produces requirements.md, constraints-and-decisions.md, and nfrs.md from a business specification.
- **Kept deliberately:** the guardrails keep-set — the three-artifact spine (§§1–5 incl. the
  Structural Decisions subsection), the analysis-vs-downstream router blockquote, Traceability
  Rules mandatory links, Technology-Agnostic Writing, the "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, the three no-exceptions
  lines, the Common Rationalizations table, the Quality Checklist, the Red Flags STOP paragraph,
  the letter/spirit epigraph, and all `references/` pointers (ARTIFACT-TEMPLATES,
  TRACEABILITY-PATTERNS).
- **KEPT reconciliation:** the [v0.28.0] and [v0.23.0] kept-sets below survive this cut in full —
  the When-to-Use bullets are not in any prior KEPT/protected set, and no `DECISIONS.md`-traceable
  line was removed. No prior KEPT or protected line is touched.
- **Consumers assessed:** technical-analyst (mounts it) · patterns-entity-modeling,
  patterns-technical-decisions (cross-reference the artifacts / D-XXX schema) · mochiko router.
  None links the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.28.0] Reference-copied field tables, homed mistake rows, and excuse-column red flags stripped (body 229 → 135, −41%, in-band)
- **Disposition:** deduped → `references/ARTIFACT-TEMPLATES.md` (Read: every field appears in
  its Field Definitions with extra Format/Rules columns, plus document templates — and the SKILL
  already declared it the home): all five in-body field tables (TR / C / D / IP / NFR) and the
  sign-in decomposition example (richer 4-TR worked table there) · deleted (Tier 1, in-file
  homes): the §4/§5 canonical-home blockquotes (restated the top analysis-vs-downstream router
  blockquote, which stays; the x-integration field list and four-level taxonomy they enumerated
  live only in `patterns-api-contracts` / `patterns-entity-modeling` — one-line canonical-home
  clauses folded into the §4/§5 declaration paragraphs), the Completeness-check line (its six
  checks restate the kept Quality Checklist), the Red Flags trigger bullets + no-exceptions list
  (the six bullets map ~1:1 onto the Common Rationalizations table's Excuse column — five are
  semantic parallels, none verbatim; bullet 4's excuse shifted, "sensitivity is obvious"
  (authoring shortcut) vs the kept row's "classification is a security team concern"
  (delegation excuse) — its substance is homed at §5 + rationalizations row 4. STOP framing
  kept as one paragraph, the table kept whole as the discipline core, vertical-tdd precedent) · **Common Mistakes deleted whole** (all 6 rows homed: transcribing → decomposition
  rule + rationalizations row 1; unmeasurable NFRs → "'Fast' is not a requirement" + the
  reference's NFR-categories table; never-bounded → §4's kept optimistic-maps paragraph;
  preferences-disguised → the reference's Distinguishing table + violation test; unclassified →
  §5 + rationalizations row 4; orphans → Traceability Rules + checklist) · densified: the
  technology-agnostic Wrong/Right table → rule + one pair (constraints-MAY-name-tech exception
  kept), the D-technique blockquote → 2 lines (the boundary is also in the description,
  When-NOT-to-Use, ARTIFACT-TEMPLATES' Part-2 blockquote, and TRACEABILITY-PATTERNS' note —
  width-only, no line delta)
- **Tier failed:** 1 throughout (every cut had a verified richer home, most in the already-
  declared reference) · n/a for the densifications
- **Content:** five field tables, one example sentence, two blockquotes, one 4-row table, six
  mistake subsections, eleven red-flag/no-exception bullets; nothing written to `templates/` —
  dedups run against pre-existing reference content, D4's destination ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files (technical-analyst, plan,
  patterns-entity-modeling ×2, patterns-technical-decisions ×2, artifact-format, mochiko
  router); none links a section anchor. Kept: the three-artifact spine, Traceability Rules
  mandatory links, INT/DS declaration paragraphs, "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, three no-exceptions
  lines, Common Rationalizations table, Quality Checklist, the letter/spirit epigraph (R4b:
  anchored to the envelope density rules directly above it). Session ruling: wave-4 batch-2
  ratified 2026-07-25.

## [v0.23.0] Description fields collapsed into the statement line across TR/C/IP/NFR blocks (T2, user-ruled)
- **Disposition:** revised per the wave-2 T2 ruling — the separate `**Description:**` paragraph field is deleted from all block templates (`references/ARTIFACT-TEMPLATES.md`) and field-definition tables (SKILL.md + reference); the ID line's statement IS the description
- **Tier failed:** artifact density (epic D4 extension): kinako's requirements.md 61k B / constraints-and-decisions.md 67k B were dominated by per-item field ceremony (label lines + a Description paragraph restating the statement), re-paid ~10× per feature
- **Content:** per-block forms compressed — TR: `**FR-XXX · MUST** — statement` + Criteria bullets + `**Deps:**` line (was Title/Source/Priority/Description/AC-checkboxes/Dependencies-list); C: type·severity·source on the statement line + one-line Impact (was 6 labeled fields); D: one-to-two-line Context + compact options table + ≤3-line Rationale + one-line Consequences (options/choice/ADR substance kept — `patterns-technical-decisions` owns the technique); IP: same collapse; NFR: statement line + Target/Measured/Applies-to lines (was 6 labeled fields + paragraph Requirement + paragraph Measurement Method). Summary tables kept and designated the **ID index** per `templates/artifact-format.md`. `references/TRACEABILITY-PATTERNS.md` pattern examples aligned to the statement-line form.
- **Consumers assessed:** plan producer (technical-analyst) + review-plan-artifacts (ARTIFACT-CHECKLISTS retargeted this wave) + review-feasibility (reads the artifacts; field-agnostic, checked — no edit needed) + downstream tasks/implement readers (consume IDs + statements, unaffected).

## [v0.23.0] Corrections landed in-wave (not strips)
- **Content:** (1) ARTIFACT-TEMPLATES' constraint Severity value set said `Hard` ("all constraints are hard boundaries by definition") while SKILL.md's field table says `blocking / significant / minor` — aligned to the SKILL (blocking/significant/minor), the pre-existing drift resolved toward the skill body; (2) `Constitution Alignment` field renamed `Governance alignment` (post-dissolution vocabulary, the plan-wave Constitution→Governance precedent), now optional-and-omitted when no principle applies.
