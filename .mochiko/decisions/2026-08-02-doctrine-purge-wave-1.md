# Doctrine purge wave 1 — `loop-discipline` skill and `command-shape.md` deleted; commands self-contained

**Status:** ruled
**Date:** 2026-08-02

## Context

Mochiko's cross-cutting doctrine layer (ARCHITECTURE.md "Cross-cutting doctrine") grew into a
set of shared homes every command was obligated to read: the `loop-discipline` skill (the four
sound-loop rules), `command-shape.md` (the codified command pattern, Layers 1–2), plus the
briefing, sizing, and per-run-contract templates. The user opened a deliberate purge campaign:
remove the cross-cutting artifacts one by one so each command decides its own loop and process
independently — discipline expressed *in* each command, not imposed *on* it from a shared home.

Wave-1 scope was ruled in-session on 2026-08-02: **`loop-discipline` + `command-shape.md`
only.** `agent-dispatch.md`, `sized-end-stage-review.md`, and `workflow-contract.md` survive
this wave (candidates for later waves). An earlier same-session answer to purge
`workflow-contract.md` alongside the skill was superseded by this scope ruling.

Safety basis (Explore loss analysis, this session): every command already carries its own Goal
with default-FAIL Not-done, counted Bounds, Floor gates, Seats & checks table, Bindings, and
Recovery. What only the two purged files carried — the weight-card factor names, the floor
bound/waiver rules, the team spawn/roster protocol, the seat-lifecycle defaults, the mesh
hand-off hold, and the ground rules (git ban, register, acceptance form, /resume mortality) —
is inlined into each command rather than dropped.

## Decision

1. **Delete** `plugins/mochiko/skills/loop-discipline/` and
   `plugins/mochiko/templates/command-shape.md` (verbatim content preserved in
   `.mochiko/strips/loop-discipline.md` and `.mochiko/strips/command-shape.md`, stamp v0.46.0).
2. **Commands become self-contained.** Each of the six commands absorbs, in its own phrasing,
   the mechanics it previously got only by the obligated reads: the four weight-card rigor
   factors (reversibility · blast radius · precedent · input confidence, scored on the artifact
   under review) · the floor rules (declared bounds lead-counted, rising only at a user
   checkpoint, re-declared on the record; cost ranges are bounds; busting escalates; rulings
   batched into fewest checkpoints; the lead's own folds/record take one non-discretionary cold
   grade, waived only by recorded user waiver at the weight card; every departure from the
   stated default is one trail line) · team transport (env check + first-spawn probe, `name:`
   discriminator, SendMessage transport, roster verification via the team config `members`
   array, kill-and-respawn on forbidden transport, spawn prompt names skill + role, seat
   announcements) · seat lifecycle (recycle at ~≥3 counted units or the command's stated
   override, respawn-as-reset from on-disk artifacts, versioned successor names, end-of-need
   shutdown, no ritual sends) · the mesh hand-off hold · ground rules (suggests commits, never
   runs git mutations or pushes; no machinery vocabulary user-facing; user acceptance is plain
   blocking text; resume from workspace evidence — sessions/teams do not survive `/resume`).
3. **Obligated reads dropped.** The `mochiko:loop-discipline` and `command-shape.md` reads
   leave every command preamble. The v5/v7 transition note's "a command that omits the read is
   non-conformant" clause is superseded; the read-drop deferral and its live-run trigger
   (2026-07-30 pilot-checkpoint ruling 5; CS-D7 V1) are superseded — the read is not dropped
   *from* the shape, the shape itself is gone.
4. **Command audit bar re-keyed.** With no shared checklist, `mochiko:validator` grades a
   command edit against **the command's own text**: internal coherence (default-FAIL goal,
   counted bounds, named floor gates, no self-grading seat row, workspace-evidence recovery)
   plus preserved responsibilities. `CLAUDE.md` and `.claude/rules/mochiko/primitive-edits.md`
   re-keyed accordingly (supersedes the v0.45.0 re-key onto `command-shape.md` as checklist).
5. **Gap-routing taxonomy dropped entirely.** The knowledge/preference/scope routing table has
   no new home; commands and `review-feasibility` reword to plain escalation language — the
   lead routes findings by judgment.
6. **Surviving doctrine templates reworded, not purged**: `agent-dispatch.md` (v8),
   `sized-end-stage-review.md` (v3), `workflow-contract.md` (v3) drop their "Governed by"
   attributions and command-shape citations; `report-format.md` and `output-style.md` re-point
   their shape citations at the owning command / local statement. The queued
   `loop-discipline` split-gate assert-union edit (Cluster-2 ratification wave) is moot.

## Rationale

- The core bet (CLAUDE.md): discipline lives in the quality of the primitives, not in
  enforcement plumbing. An obligated cross-cutting read is plumbing by another name — the
  commands were already carrying their contracts; the shared homes had become attribution and
  conformance machinery around content the commands restate.
- Independence is the goal of the campaign: a command whose loop rules live in its own file can
  evolve them without a shape-version ceremony rippling across six files.
- Nothing behavioral is lost: the inlining ruling moves every runtime mechanic into the
  command that uses it; only authoring/conformance doctrine (anatomy, [PARAM] split, slot
  index P1–P20, conformance bullets) dies with the shape file.

## Alternatives considered

- **Purge all five doctrine artifacts in one landing** — rejected for this wave; scope ruled to
  the two above, the rest queued for later waves.
- **Drop the mechanics instead of inlining** (commands ship as-is minus pointers) — rejected:
  real behavior loss (weight-card gate unexecutable as written, no spawn protocol).
- **Mint a thin replacement checklist for audits** — rejected: recreates a shared artifact,
  against the purge intent.
- **Keep the skill, drop only the obligated read** (the deferred read-drop as originally
  ruled) — overtaken: the campaign removes the artifact, not just the read.
