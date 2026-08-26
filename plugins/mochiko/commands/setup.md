---
description: Establish or update project governance from the user's interrogated intent, on the surfaces Claude Code natively loads.
disable-model-invocation: true
---

# Setup — Governance From Interrogated Intent, On Native Surfaces

**Goal:** establish or update the project's governance so it follows the user's declared
intent — never a fixed baseline — and lives where Claude Code natively loads it. There is no
`constitution.md`. `$ARGUMENTS` = optional setup request; empty is fine — propose the mode
from what the workspace shows.

**You are the lead.** Plan the run and orchestrate it toward the Goal.

## Rules — load the schema first

Your first action, before the mode is proposed, before any seat is spawned: **Read
`plugins/mochiko/schemas/setup.yaml` raw, in full.** It is the source of truth for this
run's binding rules, nested in three sections, each addressable by its section ID:
`setup.sec.harness` (how the run is led, staffed, and graded, and the decisions reserved to
the user) · `setup.sec.bindings` (surfaces, inputs, and hand-offs) ·
`setup.sec.fail-conditions` (the Not-done set). The raw Read is the first-class read: no
binary, no render step. Interpret it live: substitute every `${var}` from its `vars:` block
at read time; a `pointer:` rule binds you to that skill's procedure, referenced never
restated; labels come from `plugins/mochiko/schemas/command-labels.yaml`. A rule you have
not read is not thereby waived — this run is not open until the schema is read whole.

## Goal

The governance surface set exists and carries the user's ratified intent: the intent
synthesis was ratified by the user before any surface was authored; the trace from ratified
intent to authored surfaces closes across the set and an independent grade confirmed it from
the files; the governance region's semver is bumped; and the user accepted the set with the
trace summary in hand. The feature map exists at close: brownfield reconstructed and
user-confirmed, greenfield an empty scaffold (feature-map rules: `setup.sec.bindings`).
`Assumed` (feature-sizing record, open thread 4 — reconstruction burden, partial-baseline
poisoning): brownfield close also carries the bootstrapped product baselines at
`.mochiko/product/`; greenfield leaves **the baselines** to seed at the first implement
run's design phase. The architecture store's `spine.md` stub and its `Scope:` line are
outside that split — written on **both** paths (store rules: `setup.sec.bindings`).

**Not done — default FAIL:** the 6 rules labeled `fail-condition` in
`plugins/mochiko/schemas/setup.yaml` (section `setup.sec.fail-conditions`) — any one
standing fails the run. If the schema's `fail-condition` count is not 6, the pair is out of
sync: halt and surface it before closing.
