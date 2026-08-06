# specify.md KM landing step landed (v0.54.0)

**Status:** ruled (defect close)
**Date:** 2026-08-06

## Context

The governance v1.0.0 validator run (2026-08-06) surfaced that the KM pin
(`.mochiko/memory/knowledge-management.md`) names specify landings among the five command
landing moments, but `plugins/mochiko/commands/specify.md` carried no KM landing reference —
the other four carrying commands do (`brainstorm.md:45`, `plan.md:75-76`, `implement.md:84-87`,
`setup.md:60-62`). Recorded as a pin deviation line plus a `BACKLOG.md` item; fix routed
through the primitive-edit ceremony.

## Decision

Add one Bindings line to `specify.md` (explicit-ritual wording, user-ruled over the thin
pointer; mirrors `implement.md`'s KM-landing style and position, just before **Register**):

> **KM landing:** where `.mochiko/memory/knowledge-management.md` exists, spec acceptance is
> a landing — run its landing ritual (close/move any BACKLOG item the spec discharges, touch
> `ROADMAP.md`) and its command-boundary invariants fix-on-sight.

Shipped at v0.54.0. Pure addition — no strip entry owed. Independent author≠grader audit
(`mochiko:validator`, grading the command against its own text): **PASS round 1** — coherence,
preserved responsibilities (git-diff-verified single three-line hunk), and pin fidelity all
pass; two non-blocking advisories recorded (parenthetical omits ritual part 1, covered by the
"run its landing ritual" pointer; deviation strike is a lead obligation — executed this
landing). The pin's deviation clause and its revisit-trigger half struck per the pin's own
stated trigger.

Same bump discharges the two standing release-gate debts (GI-012): `marketplace.json` synced
0.10.0 → 0.54.0 (gate 5, GI-016 first execution) and the `CHANGELOG.md` 0.54.0 entry appended
(gate 4, GI-010).

## Rationale

Pin ↔ shipped-command agreement is a KM core invariant surface; the deviation was the only
carrying command out of step. Explicit-ritual wording chosen because specify's landing moment
(spec acceptance) is user-facing and benefits from naming the ritual parts at the point of
obligation, matching implement's precedent.

## Alternatives considered

- **Thin pointer line** (brainstorm.md:45 style — "run its close ritual" only): rejected by
  user ruling in favor of the explicit form.
- **Patch bump 0.53.1:** rejected — repo convention is all-minor bumps.
