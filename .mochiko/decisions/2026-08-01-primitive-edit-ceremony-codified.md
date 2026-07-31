# Primitive-edit ceremony codified — CLAUDE.md line + `.claude/rules/` scoped reminder

**Status:** ruled · **Date:** 2026-08-01

## Context

The `@`-reference recovery supersession
(`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`) began as an ad-hoc edit: five
command files were stripped of protected content with no `.mochiko/strips/` entry and no independent
audit. The strip-note + author ≠ grader audit ceremony already lived in the KM invariants and
`.mochiko/strips/README.md`, but nothing surfaced it at the moment a primitive file is edited — so a
small, "obvious" removal slipped past both the record and the check. The removed line was in record
§7's protected set and had already been caught once by `validation-command-shape` check 14 at the
v0.34.0 plan pilot.

## Decision (user-ruled, 2026-08-01)

The ceremony is made explicit at its touch-point:

- **CLAUDE.md** gains a line in the "Landing work (the subtractive ritual)" section: editing a
  shipped primitive is itself a landing (record → check), even a one-line removal; protected content
  leaves only as a recorded supersession-by-ruling.
- **`.claude/rules/mochiko/primitive-edits.md`** — a new `paths`-scoped rule over
  `plugins/mochiko/{commands,skills,agents,templates}/**` that fires the checklist at edit time: the
  two strip-entry types and their fields, the independent audit, and the protected-content rule.

Pure addition: no strip note (nothing removed); no ROADMAP/BACKLOG move (not a work item).

## Rationale

The ceremony's failure mode is precisely a *small, plausible* edit that does not feel like a
landing — so a reminder that lives only in a non-loaded README or the KM invariants is not enough. A
`paths`-scoped rule delivers it exactly when a primitive file is touched, which is when the
discipline is needed. CLAUDE.md carries the durable statement; the rule carries the touch-time
delivery; they cross-link.

## Alternatives considered

- **Leave it in the README + KM invariants only:** rejected — that is the configuration that just
  failed; the reminder never reached the edit moment.
- **CLAUDE.md line only, no rule:** rejected — CLAUDE.md is read at session start, not at each
  primitive edit; the scoped rule is the touch-time trigger.
- **A hook / automated gate:** out of scope and against the no-kernel-infrastructure constraint;
  compliance stays manual (per the subtractive-ritual note) until more commands run in-repo.

**Kept deliberately:** the full contracts stay single-sourced — `.mochiko/strips/README.md` (entry
formats) and `templates/command-shape.md` (shape); the new surfaces reference them, never restate
them.
