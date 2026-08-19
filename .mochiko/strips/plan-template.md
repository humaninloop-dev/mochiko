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
→ `DECISIONS.md` 2026-08-19 product-architecture row. -->

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
