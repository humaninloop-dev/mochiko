# Implementation Plan: [FEATURE]

**Feature**: `[feature-slug]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `.mochiko/specs/<feature>/spec.md`

## Summary

[Extract from feature spec: primary requirement + technical approach from decisions]

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
| `data-model.md` | ✅ Complete |
| `contracts/api.yaml` | ✅ Complete |
| `quickstart.md` | ✅ Complete |

## Next Steps

Run the tasks workflow to generate implementation tasks from this plan.
