# Strip notes — `templates/plan-template.md`

Entry formats: `strips/README.md`. Wave context: the combined plan-surface wave —
`.mochiko/brainstorms/plan-structure-yagni/record.md` (D1–D7, plan overthinking) and
`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` (D1–D7, architect
role), landed in one wave per architect-role D5.

<!-- Lineage note: from v0.76.0 this primitive IS `plugins/mochiko/schemas/plan.yaml` — the
template retired into it (entry below) and this file continues as the schema's strip home, one
file per primitive, one continuous history. Wave context for [v0.81.0]: the
product-architecture-schema Stage-1 build wave. Ruling:
`.mochiko/brainstorms/product-architecture-schema/record.md` (D3 · D10 + its S8/S13 folds · D12)
→ `DECISIONS.md` 2026-08-19 product-architecture row. The history CLOSES at v0.91.0: the
`plan.md` artifact was ruled dead outright by the plan-stage retirement, so `plan.yaml` is
deleted and no successor primitive exists. The command half of the same retirement is logged at
`.mochiko/strips/plan.md` [v0.91.0]. -->

## [v0.91.0] Schema deleted — the `plan.md` artifact dies with its command

- **Disposition:** superseded → nothing. The `plan.md` summary artifact this schema governed is
  ruled dead outright: no restatement artifact survives the plan-stage retirement. The file
  `plugins/mochiko/schemas/plan.yaml` is deleted, the `plan` name leaves `mochiko-cli`'s known-name
  set, and its compile-time embedded copy leaves the binary. This closes the primitive's history —
  template (to v0.76.0) then schema (v0.76.0 to v0.91.0).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/plan-stage-utility/record.md`
  D4, which names `plan.md` among the dead artifacts — "`plan.md` (the summary artifact) dies — no
  restatement artifact" — and D1, which retires the command that produced it; `DECISIONS.md`
  2026-08-26 "Plan-stage retirement" row. The command-side ledger is
  `.mochiko/strips/plan.md` [v0.91.0].)
- **Content (superseded schema, full verbatim below):**

````yaml
# Schema data file for the `plan` pipeline artifact template.
# Source of truth for `mochiko-cli template plan` and `--check`. Read raw when the binary is absent (D8).
template: plan
title: Implementation Plan
form: artifact-format.md
register: full   # per artifact-format.md rule 11
overview: |
  The implementation-plan deliverable (plan.md). A SUMMARY over the validated artifacts —
  tables + "See X" pointers, never restated content. Each row is a menu to prune to the
  approved proposal; an artifact the proposal did not include is omitted, not listed
  incomplete.

sections:
  - name: Header
    required: true
    contract: |
      Title line "# Implementation Plan: [FEATURE]" plus the metadata line:
      **Feature** `[feature-slug]` | **Date** [DATE] | **Spec** [link], and
      **Input**: Feature specification from `.mochiko/specs/<feature>/spec.md`.
    check: "Is the header present with feature slug, date, spec link, and the input pointer to spec.md?"

  - name: Summary
    required: true
    contract: |
      Extracted from the feature spec: the primary requirement + the technical approach from
      decisions. One compact paragraph.
    check: "Does the Summary state the primary requirement and the technical approach, drawn from the spec and decisions?"

  - name: Architecture
    required: true
    contract: |
      ALWAYS present — every plan run consults the architecture store, so this section is never
      omitted. Two parts, the second conditional.
      (1) The consult record, always: which store surfaces were read (the derived root index +
      the full AX-XXX summary table always; spine.md only when the structural-change trigger
      fired; the touched concern files by name), the trip check's outcome — each AX-XXX row this
      feature touches that stands `open` or `not-now`, with the user's disposition (ruled, or
      deferred on the record) — and, when no delta was authored, the ONE-LINE no-delta claim:
      the judgment that this feature changes no structure, stated, never left silent.
      (2) The delta pointers, only when the run authored a store delta and the user signed it:
      a one-line Delta summary (what this feature changes structurally) plus a pointer table —
      rendered diagram / changed spine elements -> the signed delta in the package; changed
      concern rows -> their AX-XXX ids and the new element statuses (in-flight / modifying /
      removing, keyed FEAT-XXX). Pointers only, never the delta restated.
    check: "Is the Architecture section present with its consult record — surfaces read, trip-check outcome and disposition, and (absent a delta) the one-line no-delta claim — plus, only where a signed store delta exists, a delta summary and pointer table naming the changed AX-XXX rows and element statuses?"

  - name: Key Decisions
    required: true
    contract: |
      Table (Decision | Choice | Shaped By | Rationale) rolling up the D-XXX decisions, with the
      full records in constraints-and-decisions.md (pointer, never restated).
    check: "Is the Key Decisions table present, each row citing its D-XXX and pointing to constraints-and-decisions.md rather than restating it?"

  - name: Infrastructure Requirements
    required: true
    contract: |
      Table (ID | Type | Source | Priority) of IP-XXX items; full definitions live in
      constraints-and-decisions.md Part 3 (pointer).
    check: "Is the Infrastructure Requirements table present with IP-XXX rows and their sources/priorities, pointing to constraints-and-decisions.md?"

  - name: Entities
    required: true
    contract: |
      Table (Entity | Status NEW/EXTENDS/REUSES | Attributes count | Relationships count |
      Sensitivity) summarizing data-model.md; full entity definitions with sensitivity
      annotations live in data-model.md (pointer).
    check: "Is the Entities table present with status, counts, and highest sensitivity per entity, pointing to data-model.md?"

  - name: Endpoints
    required: true
    contract: |
      Table (Method | Path | Description | Integration) summarizing the API; full OpenAPI spec
      lives in contracts/api.yaml (pointer).
    check: "Is the Endpoints table present summarizing method/path/description/integration, pointing to contracts/api.yaml?"

  - name: Artifacts
    required: true
    contract: |
      The artifacts this run's APPROVED proposal produced — each complete and graded. The rows
      are the menu to prune to the proposal; an artifact the proposal did not include is omitted
      here, not listed incomplete. Table (Artifact | Status): requirements.md,
      constraints-and-decisions.md, the signed store delta (only where the run authored one —
      user-signed), data-model.md, contracts/api.yaml, quickstart.md (conditional — or "not
      applicable — no external integration surface"), tasks.md (cycle cards). NFR targets are
      not an artifact row: they live on the store's concern rows.
    check: "Does the Artifacts table list exactly the artifacts the approved proposal produced, each marked complete, with conditional artifacts (the store delta, quickstart) handled per their rules?"

  - name: Next Steps
    required: true
    contract: |
      Run `/mochiko:implement` to execute this package — the accepted design, architecture, and
      cycle cards are its entry condition.
    check: "Does Next Steps point to /mochiko:implement as the execution step with the package as its entry condition?"

skeleton: |
  # Implementation Plan: [FEATURE]

  **Feature**: `[feature-slug]` | **Date**: [DATE] | **Spec**: [link]
  **Input**: Feature specification from `.mochiko/specs/<feature>/spec.md`

  ## Summary

  [Extract from feature spec: primary requirement + technical approach from decisions]

  ## Architecture

  **Store consulted**: derived index + AX summary table · [spine.md — only if the structural-change trigger fired] · touched concern files: [AX-XXX, AX-XXX]

  **Trips**: [AX-XXX (`open`/`not-now`) — ruled: [the ruling] | deferred on the record] — *or* "none — no touched row stands `open` or `not-now`".

  **Delta**: [one line — what this feature changes structurally] — *or* the no-delta claim: "no structural change — [the one-line judgment]".

  *The table below is present only where the user signed a store delta; omit it otherwise.*

  | Aspect | Where |
  |--------|-------|
  | Rendered diagram · changed spine elements | [the signed delta in this package] |
  | Changed concern rows | [AX-XXX → `in-flight (FEAT-XXX)` / `modifying (FEAT-XXX)` / `removing (FEAT-XXX)`] |

  ## Key Decisions

  | Decision | Choice | Shaped By | Rationale |
  |----------|--------|-----------|-----------|
  | [D-001 title] | [chosen option] | [C-XXX references] | [brief why] |

  See `constraints-and-decisions.md` for full decision records.

  ## Infrastructure Requirements

  | ID | Type | Source | Priority |
  |----|------|--------|----------|
  | [IP-001] | [type] | [C-XXX/NFR-XXX] | [MUST/SHOULD] |

  See `constraints-and-decisions.md` Part 3 for full infrastructure requirement definitions.

  ## Entities

  | Entity | Status | Attributes | Relationships | Sensitivity |
  |--------|--------|-----------|--------------|-------------|
  | [Entity name] | [NEW/EXTENDS/REUSES] | [count] | [count] | [highest classification] |

  See `data-model.md` for full entity definitions with sensitivity annotations.

  ## Endpoints

  | Method | Path | Description | Integration |
  |--------|------|-------------|-------------|
  | [HTTP method] | [path] | [description] | [external system if any] |

  See `contracts/api.yaml` for full OpenAPI specification.

  ## Artifacts

  | Artifact | Status |
  |----------|--------|
  | `requirements.md` | ✅ Complete |
  | `constraints-and-decisions.md` | ✅ Complete |
  | Store delta | ✅ Complete — user-signed *(only where the run authored one)* |
  | `data-model.md` | ✅ Complete |
  | `contracts/api.yaml` | ✅ Complete |
  | `quickstart.md` | ✅ Complete — *or* "not applicable — no external integration surface" (conditional; see `patterns-api-contracts`) |
  | `tasks.md` (cycle cards) | ✅ Complete |

  ## Next Steps

  Run `/mochiko:implement` to execute this package — the accepted design, architecture, and
  cycle cards are its entry condition.
````

- **Kept deliberately:** the Architecture section's two-part contract — the mandatory consult
  record (surfaces read, trip-check outcome and disposition, the one-line no-delta claim) — is not
  lost with the file: it survives as sufficiency-check clause 4, which requires the store
  consulted, the trip check run, and the no-delta claim recordable, with trips dispositioned by the
  user at run-open (record D2 clause 4 and its "Trips are not gaps" paragraph; addendum A4
  sharpens the no-delta evidence). The Entities and Endpoints summary tables die with the
  restatement artifact, but their subjects survive in the baselines they pointed at —
  `data-model.md` and `contracts/api.yaml`, whose homes are unchanged (D4). The `quickstart.md`
  null-path record, carried on the Artifacts row's conditional, rehomes to the sufficiency verdict
  report (D4). The Next Steps section — "Run `/mochiko:implement` to execute this package — the
  accepted design, architecture, and cycle cards are its entry condition" — dies whole: implement
  no longer has an accepted-package entry condition (D1, D4 dead gates).
- **Consumers assessed:** `crates/mochiko-cli` — `TEMPLATE_NAMES` drops `plan` (9 → 8) and the
  `include_str!` arm embedding this file is removed, in this same wave; leaving the arm would break
  the build, since the embed is compile-time. `commands/plan.md`'s Package-artifacts bullet named
  the schema by path in its two-arm render/raw-Read form — that command is deleted in the same
  wave. `skills/mochiko/SKILL.md`'s schema pointer and `evals/review-plan-artifacts/rules.json`
  are other seats' surfaces in this wave. The raw-Read degraded path (GI-020) stays honest: the
  remaining eight schemas are unchanged and still readable without the binary, and
  `every_shipped_schema_file_is_readable_yaml_and_every_known_name_has_one` still covers them.

## [v0.81.0] The Architecture section's omit-conditional dies — consult is mandatory (D3/D10)

- **Disposition:** superseded → `required: true` and a two-part contract: a **mandatory consult
  record** (surfaces read, the trip check's outcome and disposition, and — absent a delta — the
  one-line no-delta claim) plus **conditional delta pointers** where the user signed a store
  delta.
- **Tier failed:** n/a — supersession by ruling (record D3/D10 and its S8 consult-metering and
  S13 no-delta-claim folds; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim, three sites.
  (1) The section's `required:` value — `false`.
  (2) The contract's opening: "Present ONLY when the approved proposal included
  `architecture.md`; otherwise omit this section entirely. The system view — components,
  boundaries, interactions, and the delta this feature introduces — was designed first and
  signed off at the architecture gate. It lives in `architecture.md`; see it for the container
  diagram, key-flow sequence diagrams, and component register (never restated here — pointers
  only). Carries a one-line Delta summary (what this feature changes structurally, or "no
  structural change") and a pointer table: container diagram / sequence diagrams / component
  register -> architecture.md; structural decisions (D-XXX) -> constraints-and-decisions.md."
  (3) The `check` string at `plan.yaml:40`, verbatim: "If (and only if) the approved proposal
  included architecture.md, is the Architecture section present as pointers-only with a delta
  summary? If the proposal omitted architecture, is the section absent?"
  (4) The matching skeleton block, whose conditional line read: "*Present only when the approved
  proposal included `architecture.md`; otherwise omit this section.*"
- **Kept deliberately:** the **pointers-only discipline** survives whole — the section still
  never restates the delta, and its table still points rather than reproduces. The
  one-line Delta summary survives, and its "or no structural change" alternative is not merely
  kept but promoted: under D10's S13 fold the no-delta claim is a required, gate-shown judgment
  line, where the old wording let it read as an optional phrasing choice.
- **Consumers assessed:** `review-plan-artifacts`'s Architecture checklist (ARTIFACT-CHECKLISTS
  L100-133) is P4's this wave and re-writes to this grammar; `commands/plan.md`'s consult
  contract (its own strip entries) is the authoring side of the same ruling; `mochiko-cli
  template plan` renders from this data file and needs no code change (schema data is the source
  of truth, D8).

## [v0.81.0] Artifacts table — `nfrs.md` and conditional `architecture.md` rows retired (D3/D12)

- **Disposition:** superseded → the store-delta row ("only where the run authored one —
  user-signed"); `nfrs.md` has no row because the file dies (D12), its `NFR-XXX` targets living
  on store concern rows.
- **Tier failed:** n/a — supersession by ruling (record D3/D12; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim, three sites. (1) Contract fragment: "requirements.md,
  constraints-and-decisions.md, nfrs.md, architecture.md (only when the proposal included
  architecture — signed off), data-model.md, contracts/api.yaml, quickstart.md (conditional — or
  "not applicable — no external integration surface"), tasks.md (cycle cards)." (2) The `check`
  string's parenthetical: "with conditional artifacts (architecture, quickstart) handled per
  their rules". (3) Two skeleton rows: "| `nfrs.md` | ✅ Complete |" and "| `architecture.md` |
  ✅ Complete — signed off *(only when the proposal included architecture)* |".
- **Kept deliberately:** the **menu-to-prune** rule is untouched — the rows remain a menu pruned
  to the approved proposal, and an artifact the proposal did not include is still omitted rather
  than listed incomplete. `quickstart.md` keeps its conditional row and its "not applicable — no
  external integration surface" null form verbatim (D12 ruled quickstart kept, user-probed). The
  store delta stays conditional, on a different condition: the run authored one.
- **Consumers assessed:** `commands/plan.md`'s Package-artifacts list (own strip entry, same
  wave); `review-plan-artifacts`'s conformance rows (P4's, same wave).

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/plan.yaml + mochiko-cli template plan
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!-- Form: templates/artifact-format.md (the deliverable envelope). plan.md is a summary
     over the validated artifacts — tables + "See X" pointers, never restated content.
     Register: `full` per artifact-format.md rule 11. -->

# Implementation Plan: [FEATURE]

**Feature**: `[feature-slug]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `.mochiko/specs/<feature>/spec.md`

## Summary

[Extract from feature spec: primary requirement + technical approach from decisions]

## Architecture

*Present only when the approved proposal included `architecture.md`; otherwise omit this section.*

The system view — components, boundaries, interactions, and the delta this feature introduces —
was designed first among the design work and **signed off** at the architecture gate. It lives in
`architecture.md`; see it for the container diagram, the key-flow sequence diagrams, and the
component register (never restated here — pointers only).

**Delta summary**: [one line — what this feature changes structurally, or "no structural change"].

| Aspect | Where |
|--------|-------|
| Container diagram (delta-marked) · key-flow sequence diagrams · component register | `architecture.md` |
| Structural decisions (D-XXX) | `constraints-and-decisions.md` — structural-decisions section |

## Key Decisions

| Decision | Choice | Shaped By | Rationale |
|----------|--------|-----------|-----------|
| [D-001 title] | [chosen option] | [C-XXX references] | [brief why] |

See `constraints-and-decisions.md` for full decision records.

## Infrastructure Requirements

| ID | Type | Source | Priority |
|----|------|--------|----------|
| [IP-001] | [type] | [C-XXX/NFR-XXX] | [MUST/SHOULD] |

See `constraints-and-decisions.md` Part 3 for full infrastructure requirement definitions.

## Entities

| Entity | Status | Attributes | Relationships | Sensitivity |
|--------|--------|-----------|--------------|-------------|
| [Entity name] | [NEW/EXTENDS/REUSES] | [count] | [count] | [highest classification] |

See `data-model.md` for full entity definitions with sensitivity annotations.

## Endpoints

| Method | Path | Description | Integration |
|--------|------|-------------|-------------|
| [HTTP method] | [path] | [description] | [external system if any] |

See `contracts/api.yaml` for full OpenAPI specification.

## Artifacts

The artifacts this run's **approved proposal** produced — each complete and graded. An artifact
the proposal did not include is omitted here, not listed incomplete; the rows below are the
menu to prune to the proposal.

| Artifact | Status |
|----------|--------|
| `requirements.md` | ✅ Complete |
| `constraints-and-decisions.md` | ✅ Complete |
| `nfrs.md` | ✅ Complete |
| `architecture.md` | ✅ Complete — signed off *(only when the proposal included architecture)* |
| `data-model.md` | ✅ Complete |
| `contracts/api.yaml` | ✅ Complete |
| `quickstart.md` | ✅ Complete — *or* "not applicable — no external integration surface" (conditional; see `patterns-api-contracts`) |
| `tasks.md` (cycle cards) | ✅ Complete |

## Next Steps

Run `/mochiko:implement` to execute this package — the accepted design, architecture, and
cycle cards are its entry condition.
````
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/plan.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template plan`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/plan.md` (re-pointed by P4) · `skills/mochiko/SKILL.md` router row (re-described CLI/schema-delivered by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.67.0] Fixed-set Artifacts checklist + always-on architecture section made proposal-conditional
- **Disposition:** superseded → the re-keyed template: the `## Artifacts` table is captioned as the menu to prune to the run's **approved proposal** (an artifact the proposal did not include is omitted, not listed incomplete), and the `architecture.md` row is qualified "*(only when the proposal included architecture)*"; the `## Architecture` section carries a "*Present only when the approved proposal included `architecture.md`; otherwise omit this section.*" lead-in.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/plan-structure-yagni/record.md` D1/D2 artifact-set demotion + D6a as amended HF-4 conditional architecture; combined wave `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` D5)
- **Content:** `## Artifacts` table presented as a fixed eight-row all-`✅ Complete` checklist (every listed artifact produced every run); row "| `architecture.md` | ✅ Complete — signed off |" (unconditional) · `## Architecture` section opener "The system view … **was designed first among the design work and signed off at the architecture gate**" (architecture assumed always present)
- **Kept deliberately:** the table's rows as the illustrative menu, the See-X pointer discipline, the `quickstart.md` conditional row, the `## Architecture` **Delta summary** "no structural change" line, every other section.
- **Consumers assessed:** n/a — template; `plan.md` (same wave, same stamp) re-keys the same fixed-set / conditional-architecture ruling command-side; `review-plan-artifacts` grades the produced `plan.md` against the approved proposal (conformance, sibling seat).
