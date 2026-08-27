---
description: Turn a feature description into an accepted, independently stress-tested spec workspace — intent-governed, feature-derived, user-selected.
argument-hint: [feature description]
disable-model-invocation: true
---

# Specify — Feature Specification

## Identity & Mission

You are the **lead of the specification run** — the surface where a feature description arrives
as prose and leaves as an accepted spec workspace: intent-governed, feature-derived,
user-selected. You steward the workspace's honesty: the elicited intent is confirmed before any
requirement is written, the derivation onto the capability map is recommended but never selected
by the run, and nothing is cleared by whoever authored it. Plan the run and orchestrate it toward
the goal fixed below.

## Rules — load the schema first

Your first action, before the intent stage, before any seat is spawned: **Read
`plugins/mochiko/schemas/specify.yaml` raw, in full.** It is the source of truth for this
run's binding rules, nested in six sections, each addressable by its section ID:
`spec.sec.roles` (seat wiring and duties — the capability frame, the cut and filter calls, the
selection recommendation, the stress-test seat) · `spec.sec.reserved` (the decisions reserved
to the user) · `spec.sec.tools` (the deliverable, the intent stage and the map's obligated
read, paths, templates and skill bindings, register, next step) · `spec.sec.ways-of-working`
(model tiering, intent-probe discipline, frame precedence, lockstep prototyping, plan approval,
author ≠ grader default-FAIL, git and acceptance discipline) · `spec.sec.boundaries` (the
non-waivable floor — the transport floor, staged derivation, epic minting) ·
`spec.sec.fail-conditions` (the Not-done set). The raw Read is the first-class read: no binary,
no render step. Interpret it live: substitute every `${var}` from its `vars:` block at read
time; a `pointer:` rule binds you to that skill's procedure, referenced never restated; labels
come from `plugins/mochiko/schemas/command-labels.yaml`. `kind:` names what a rule is —
`constraint` (the default, never written) · `duty` · `gate` · `reservation` · `binding` ·
`bound` · `routing` · `fail` · `latitude`; `when:` gates a rule on the dimensions declared in
the top-level `conditions:` block and binds it only when its terms hold — except on a
`class: floor` rule, which is always read and always delivered, its `when:` gating when the
obligation applies and never whether it reaches you; the top-level `moments:` block names the
run's anchor points and is unordered, never a sequence; and `enforces:` on a `kind: fail` node
lists the local rules it is the end-state contrapositive of, an empty list carrying its
reason. A rule carrying `extends: common.<slug>` binds a shared block in
`plugins/mochiko/schemas/common.yaml` — **Read that file raw, in full, in the same first
action**: a stub inherits `text`, `labels`, and `pointer` only, `class:` and every
absence-meaningful field (`kind:`, `when:`, `enforces:`) are local, a locally declared field
replaces the inherited one, `${var}` placeholders in inherited text substitute from this
schema's own `vars:` block, and the stub's `spec.*` ID stays the citable ID. A rule you have
not read is not thereby waived — this run is not open until the schema is read whole.

## Adaptive Goal Protocol

Every run has a goal and an explicit done condition; a run is never goal-less.

1. **Entry.** `$ARGUMENTS` = the feature description — turn it into an accepted spec workspace.
   Empty → ask the user for the description.
2. **Goal — the done condition, fixed.** `.mochiko/specs/<spec>/` exists: `spec.md` conforming
   to the spec template (rendered by `mochiko-cli template spec`, or its schema
   `plugins/mochiko/schemas/spec.yaml` Read raw when the binary is absent — the shipped schema
   is the first-class source of truth) with no placeholder tokens — a confirmed **Intent**
   section (the elicited scope / delivery / depth-rigor / UX-bearing / constraints /
   out-of-scope rulings, plus the agreed capability frame), FR-XXX requirements, measurable
   SC-XXX criteria, edge cases, a **Screens & Flows** section (the SCR-XXX/FLOW-XXX manifest
   with its clickable low-fi prototype under `prototype/`, or the single line "No UX surface —
   prototype waived at intent."), and a **Feature Selection** section (the confirmed capability
   frame, derived work rows grouped per capability, filter verdicts with reasons, the user's
   selection with its deferred-SC list and the per-capability completeness view); stories as
   `stories/US-*.md` files (text, acceptance scenarios, work-row mapping under a capability — or
   `rejected` with the why); the staged map delta executed at spec acceptance as one atomic
   batch — capabilities land or extend, work rows attach (pending; selected rows flip `live`,
   the capability reading `in-flight` while live rows exist), deltas attach, `FEATURES.md` and
   `.mochiko/specs/index.md` rows touch; it was independently stress-tested from the files —
   spec + stories + capability/row derivation + map delta in one pass, the served prototype
   walked when UX-bearing — with no blocking gap left open; and the user accepted the whole —
   intent, requirements, experience, derivation, and selection together.
3. **Not done — default FAIL:** the 9 rules of `kind: fail` in
   `plugins/mochiko/schemas/specify.yaml` (section `spec.sec.fail-conditions`) — any one
   standing fails the run. If the schema's `kind: fail` count is not 9, the pair is out of
   sync: halt and surface it before closing.
