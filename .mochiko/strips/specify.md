# Strip notes — `commands/specify.md`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (the first of
the five one-shot-command waves; BACKLOG item 7 of the pattern-codification build). The
wave also ran the **D2 conversion assessment** and the **S8 home-revision checkpoint**
(shape v1 → v2 — see the REGISTRY `command-shape.md` row).

## [v0.13.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** specify's producer spans up to 3
  revision rounds plus clarification feedback — the same longer-horizon context-retention
  bet `/mochiko:setup`'s authoring loop was ruled team-form on (standing analyst seat;
  gap lists lead-routed verbatim). The critic maps to setup's validator seat: cold at
  first critique, same-seat messages after, no producer contact — independence stays
  structural. Transport rides the v3 fix (`agent-dispatch.md` Seat transport +
  addressability probe).
- **Steelman recorded:** zero successful team-form runs observed at conversion time (two
  setup defect runs; the kinako acceptance test pending); brainstorm v2 measured standing
  seats as more expensive than dispatches. Ruled team-form anyway per D2's declared
  default + S4 (no prior dogfood evidence required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open
  "Dogfood `/mochiko:specify`" BACKLOG item) confirms the conversion or reverts it to
  one-shot Layer-1 form.

## [v0.13.0] Per-run contract fill (`workflow-contract.md` → `contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract section — the authoring-time-fill rule); the per-workflow values survive as the command's Contract section
- **Tier failed:** 1 (altitude — the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body."

## [v0.13.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko sound loop: invoke `mochiko:loop-discipline` and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per `agent-dispatch`. Those rules are not restated here…" — the requirement list restated `loop-discipline`'s own enumeration.

## [v0.13.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Producer ↔ validator clause; the `review-*` family boundary also lives in `review-specifications`' description + REGISTRY)
- **Tier failed:** 1
- **Content:** stated three times pre-wave — L8 ("The critic *recommends* a status; **you own the clearing verdict** — its status is input, never the gate"), L19 (Team clause "it produces lead-adjudicated input, never the authoritative grade"), L67 (footer "the advocate recommends a status *from* the spec, you Read the artifacts and decide").

## [v0.13.0] HIL done-condition comparison blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical provenance; preserved in ROADMAP's Decision Trail + `.mochiko/transform/specify/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL exited on the State-Analyst's *autonomous* verdict with no human acceptance — it could self-declare done on pass 1, violating `loop-discipline` req. 1. The advocate's three-way status survives as input to your verdict, plus the new G3 acceptance gate."

## [v0.13.0] Feature-numbering-script aside
- **Disposition:** deleted
- **Tier failed:** 2 (HIL-history note; provenance in `.mochiko/transform/specify/`)
- **Content:** "(No feature-numbering script — workspace-as-state replaces it.)"

## [v0.13.0] Spec-grammar enumeration in the produce brief
- **Disposition:** relocated → the grammar's single sources: `mochiko:authoring-requirements` + `mochiko:authoring-user-stories` + `templates/spec-template.md` (user-ratified; the brief keeps "the template to fill per those skills — no placeholder tokens", and the goal line names the deliverable's parts once)
- **Tier failed:** 1
- **Content:** "(prioritized P1/P2/P3 user stories with Given-When-Then, FR-XXX requirements, measurable SC-XXX, edge cases; technology-agnostic; no placeholder tokens)"

## [v0.13.0] Footer ground rules + one-shot transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules — homed at shape v2, this wave); the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push."

## [v0.13.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`)"

## [v0.13.0] KEPT: "Lead-inline because enrich-or-not is loop-entry triage: it conditions the input, it neither authors nor grades."
- **Tier-2 evidence:** guards the lead-inline vs seat boundary — without it the natural reading is to hand enrichment to the producer seat, coupling input conditioning into authoring (the producer would then author from input it shaped itself). Boundary provenance: the specify port's rehome ruling (`.mochiko/transform/specify/reconcile.md` — enrichment landed on the lead, not the analyst).
