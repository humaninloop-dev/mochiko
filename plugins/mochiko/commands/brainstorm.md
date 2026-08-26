---
description: Think a problem through with the user and leave one hardened, cold-reviewed decision record behind.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one hardened decision record
behind. Empty topic → ask what we are thinking through.

## Rules — load the schema first

Your first action, before any questioning, before any seat is spawned: **Read
`plugins/mochiko/schemas/brainstorm.yaml` raw, in full.** It is the source of truth for this
run's binding rules, nested in three sections, each addressable by its section ID:
`brainstorm.sec.harness` (lead role, seat wiring, review independence, and the decisions
reserved to the user) · `brainstorm.sec.bindings` (deliverable, index, synthesis, register,
next step) · `brainstorm.sec.fail-conditions` (the Not-done set). The raw Read is the
first-class read: no binary, no render step. Interpret it live: substitute every `${var}` from
its `vars:` block at read time; a `pointer:` rule binds you to that skill's procedure,
referenced never restated; labels come from `plugins/mochiko/schemas/command-labels.yaml`. A
rule you have not read is not thereby waived — this run is not open until the schema is read
whole.

## Goal

`.mochiko/brainstorms/<slug>/record.md` exists, each decision carrying statement + rationale +
confidence mark (`Confident` / `Assumed` / `Contested` / `Unsure` / `Deferred`); the record was
cold-reviewed and every surviving finding dispositioned — or the user's waiver of the review is
recorded on it; the session's entry in `.mochiko/brainstorms/index.md` is updated with where the
outcome landed; and the user accepted the record.

**Not done — default FAIL:** the 4 rules labeled `fail-condition` in
`plugins/mochiko/schemas/brainstorm.yaml` (section `brainstorm.sec.fail-conditions`) — any one
standing fails the run. If the schema's `fail-condition` count is not 4, the pair is out of
sync: halt and surface it before closing.
