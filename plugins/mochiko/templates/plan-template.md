<!-- Form: templates/artifact-format.md (the deliverable envelope). plan.md is a summary
     over the validated artifacts — tables + "See X" pointers, never restated content.
     Register: `full` per artifact-format.md rule 11. -->

# Implementation Plan: [FEATURE]

**Feature**: `[feature-slug]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `.mochiko/specs/<feature>/spec.md`

## Summary

[Extract from feature spec: primary requirement + technical approach from decisions]

## Architecture

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

| Artifact | Status |
|----------|--------|
| `requirements.md` | ✅ Complete |
| `constraints-and-decisions.md` | ✅ Complete |
| `nfrs.md` | ✅ Complete |
| `architecture.md` | ✅ Complete — signed off |
| `data-model.md` | ✅ Complete |
| `contracts/api.yaml` | ✅ Complete |
| `quickstart.md` | ✅ Complete — *or* "not applicable — no external integration surface" (conditional; see `patterns-api-contracts`) |
| `tasks.md` (cycle cards) | ✅ Complete |

## Next Steps

Run `/mochiko:implement` to execute this package — the accepted design, architecture, and
cycle cards are its entry condition.
