---
description: Turn a feature description into an accepted, independently stress-tested spec.md.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn `$ARGUMENTS` (the feature description) into an accepted `spec.md`. Empty →
ask the user for the description, or enrich from scratch with their consent.

## Goal

`.mochiko/specs/<feature>/spec.md` exists, conforming to `templates/spec-template.md` with no
placeholder tokens — prioritized P1/P2/P3 user stories, FR-XXX requirements, measurable SC-XXX
criteria, edge cases; it was independently stress-tested from the file with no blocking gap
left open; and the user accepted it.

**Not done — default FAIL:** a blocking gap open · the spec never graded by anyone but its
author · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. Sparse input (Who / Problem / Value unclear) → enrich it
  yourself, inline, via `mochiko:analysis-iterative` before authoring starts.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads `spec.md` itself —
  never the author's report — default FAIL.
- **Reserved to the user:** the feature framing when `$ARGUMENTS` is empty · clarification
  answers only they can settle · spec acceptance (accept / amend / reject).
- A missing `CLAUDE.md` governance region is surfaced (offer `/mochiko:setup`), never
  auto-resolved. Suggest commits; never run git mutations, never push. User acceptance is
  plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/specs/<feature>/spec.md` from `templates/spec-template.md` —
  `<feature>` a kebab-case slug derived from the description; never offer to delete it.
  Uncertainty lives in its Assumptions and Open Questions sections.
- **Governance input:** where the region is present, name the relevant
  `.claude/rules/mochiko/` files as an obligated read in the author's brief — `paths`-scoped
  rules do not fire for from-scratch authoring.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan` (or `/mochiko:slice` for a multi-story spec).
