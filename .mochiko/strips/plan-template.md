# Strip notes — `templates/plan-template.md`

Entry formats: `strips/README.md`. Wave context: the combined plan-surface wave —
`.mochiko/brainstorms/plan-structure-yagni/record.md` (D1–D7, plan overthinking) and
`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` (D1–D7, architect
role), landed in one wave per architect-role D5.

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
