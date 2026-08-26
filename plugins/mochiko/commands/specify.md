---
description: Turn a feature description into an accepted, independently stress-tested spec workspace — intent-governed, feature-derived, user-selected.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn `$ARGUMENTS` (the feature description) into an accepted spec workspace. Empty →
ask the user for the description.

## Goal

`.mochiko/specs/<spec>/` exists: `spec.md` conforming to the spec template (rendered by
`mochiko-cli template spec`, or its schema `plugins/mochiko/schemas/spec.yaml` Read raw when
the binary is absent — the shipped schema is the first-class source of truth) with
no placeholder tokens — a confirmed **Intent** section (the elicited scope / delivery /
depth-rigor / UX-bearing / constraints / out-of-scope rulings, plus the agreed capability
frame), FR-XXX requirements,
measurable SC-XXX criteria, edge cases, a **Screens & Flows** section (the SCR-XXX/FLOW-XXX
manifest with its clickable low-fi prototype under `prototype/`, or the single line "No UX
surface — prototype waived at intent."), and a **Feature Selection** section (the confirmed
capability frame, derived work rows grouped per capability, filter verdicts with reasons, the
user's selection with its deferred-SC list and the per-capability completeness view);
stories as `stories/US-*.md` files (text, acceptance scenarios, work-row mapping under a
capability — or `rejected` with the why); the staged map delta executed at spec acceptance as
one atomic batch — capabilities land or extend, work rows attach (pending; selected rows flip
`live`, the capability reading `in-flight` while live rows exist), deltas attach,
`FEATURES.md` and `.mochiko/specs/index.md` rows touch; it was independently stress-tested
from the files — spec + stories + capability/row derivation + map delta in one pass, the
served prototype walked when UX-bearing — with no blocking gap left open; and the user
accepted the whole — intent, requirements, experience, derivation, and selection together.

**Not done — default FAIL:** the 9 rules labeled `fail-condition` in
`plugins/mochiko/schemas/specify.yaml` (section `spec.sec.fail-conditions`) — any one
standing fails the run. If the schema's `fail-condition` count is not 9, the pair is out of
sync: halt and surface it before closing.

## Rules — load the schema first

Your first action, before the intent stage, before any seat is spawned: **Read
`plugins/mochiko/schemas/specify.yaml` raw, in full.** It is the source of truth for this
run's binding rules, nested in three sections, each addressable by its section ID:
`spec.sec.harness` (seat wiring, the intent stage and capability frame, lockstep prototyping,
derivation and selection, independence, decisions reserved to the user) · `spec.sec.bindings`
(paths, templates, and skill bindings) · `spec.sec.fail-conditions` (the Not-done set). The
raw Read is the first-class read: no binary, no render step. Interpret it live: substitute
every `${var}` from its `vars:` block at read time; a `pointer:` rule binds you to that
skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/command-labels.yaml`. A rule you have not read is not thereby
waived — this run is not open until the schema is read whole.
