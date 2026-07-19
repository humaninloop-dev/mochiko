# Strip notes — `commands/tasks.md`

Entry formats: `strips/README.md`. Wave context: the tasks cluster wave (BACKLOG item 7, the fourth
one-shot-command wave after specify's v0.13.0, slice's v0.14.0, and plan's v0.15.0). The wave also ran
the **D2 conversion assessment** (one-shot → team-form) and re-checked the **S8 home-revision
checkpoint** against tasks' needs (a standing producer spanning two phases + ONE standing reviewer with
an early-gate + cumulative-mode pattern — no new shape gap; shape stays v2).

## [v0.16.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** tasks runs a producer↔reviewer cycle (≤3 rounds per
  phase, gap-list-driven revision, cold reviewer) across **two phases** (Mapping → Tasks) where Phase 2
  **expands Phase 1's own mapping**. The context-retention bet is tasks' own: a **standing producer
  seat** carries (1) the Phase-1 **slicing judgment** — why each cycle was cut as a vertical slice, how
  foundation was separated from features, where the inter-cycle dependencies came from — forward into
  Phase 2, so the tasks expansion is authored *from* that lived rationale rather than reverse-engineered
  from `task-mapping.md`; and (2) coherence across a targeted revision (a mapping gap fixed in Phase 1,
  or a tasks gap in Phase 2, stays consistent with the slicing decisions the producer itself made). The
  single reviewer maps to a **standing devils-advocate seat**: cold at the first (mapping) review, then
  messaged in Phase 2 for the **cumulative** mode — a full `tasks.md` review plus a cross-check back to
  `task-mapping.md`. Its retained Phase-1 context is what makes the Phase-2 cross-check an incremental
  cross-check rather than a fresh read of the mapping. The reviewer never contacts the producer —
  independence stays structural. Transport rides the v3 fix (`agent-dispatch.md` Seat transport +
  addressability probe on the producer's first spawn).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's, slice's, and plan's own checkpoints all unfired;
  brainstorm v2 measured standing seats *more* expensive than dispatches). Tasks is **two-seat** (nearer
  slice's cost than plan's three-seat load), so its team-form tax is moderate if the retention payoff
  doesn't land. The honest weak point is the **input substrate**: `task-mapping.md` is small and richly
  structured — the story→cycle table, per-cycle type/dependencies, and the slice rationale are written
  *in the file* — so a fresh Phase-2 producer reconstructs the mapping's *content* cheaply from disk;
  the retention payoff is the *authorial judgment behind* the slices (the "why this cut" that the file
  records as prose but a cold reader must re-derive), not the mapping data itself, so the payoff is
  real but narrower than "two artifacts / two phases" suggests. And the single reviewer sits nearer the
  surgical/one-shot pole than plan's reviewer pair. Ruled team-form anyway per D2's declared default +
  S4 (no prior dogfood evidence required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:tasks`" BACKLOG item, Tasks-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check; the standing producer seat is messaged (not respawned)
  across rounds **and across the Mapping→Tasks phase boundary** (Phase-1 slicing judgment carried into
  the Phase-2 expansion); the reviewer spawns **cold at the first mapping review**, is messaged in Phase
  2 for the **cumulative** mode (both artifact sets supplied), and **never contacts the producer**.

## [v0.16.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all
  four requirements (default-FAIL done-condition, independent validation, bounded iteration, named
  human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here…" —
  restated loop-discipline's own enumeration.

## [v0.16.0] Per-run contract fill (`workflow-contract.md` → `tasks-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the authoring-time-fill
  rule); the per-workflow values survive as the command's authoring-time Contract section (tasks' are a
  per-phase round cap and four gates)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/tasks-contract.md` with the values below,
  then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not
  this command body."

## [v0.16.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-task-artifacts`' description + REGISTRY). The per-phase
  Verdict *steps* (Phase 1 step 3, Phase 2 step 3) are workflow mechanics and survive.
- **Tier failed:** 1
- **Content:** stated three times pre-wave — the lead framing ("The reviewer *recommends* a status;
  **you own the clearing verdict** — its status is input, never the gate"), the Contract Team clause
  ("reviewer … grades from the files, never authors"), and the footer ("the reviewer grades from the
  files, you Read the artifacts and decide against the default-FAIL done-condition — its status is
  input").

## [v0.16.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.16.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`):" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read; mochiko
  `plan` writes none)". The recovery table (evidence → resume-at) is the workflow-specific Recovery
  PARAM and survives.

## [v0.16.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in ROADMAP's
  Decision Trail + `.mochiko/transform/tasks/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on
  the advocate's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline`
  reqs 1 & 3. The reviewer's three-state status survives only as input to your verdict; the
  deterministic cap and the new G5 acceptance gate close the gates HIL lacked." — the shape of specify's
  / plan's deleted HIL-comparison blockquotes; its rationale is carried by the Contract done-condition,
  so no unique behavior is lost.

## [v0.16.0] Slice-scoped entry — de-restated to the Graduation-contract reference
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section — the
  single home of the consumption rules); Phase 0 step 6 now *applies* the contract by reference for slice
  resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the one-shot entry variant declared "the single source of the consumption rules;
  do not restate it" and then restated most of them beneath that self-declaration — the same D1 churn
  liability the plan wave's `validation-command-shape` audit caught on plan's identical entry; applied
  here proactively by that prior ruling, NOT contested)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order with `slices/<slice>/plan.md` but no `slices/<slice>/tasks.md`") and the **staleness
  guard** ("the live `spec.md` story-ID set must match the Spec stamp — mismatch → block and point to
  `/mochiko:slice`"). tasks' four genuine bindings were **kept**: scope = the slice's stories + extend
  obligations with a cycle serving any other story → **G4**; the plan-complete entry gate reads
  `slices/<slice>/plan.md`; per-slice outputs under `slices/<slice>/` and what that does to the
  done-condition's artifact set; and the reviewer briefing sets {this slice's artifacts}/{accumulated
  feature-root plan artifacts}.
- **Note:** the Graduation contract is on the ≥3-consumer queue (plan/tasks/implement slice-scoped
  variants) — this strip relocates tasks' *local restatement* to the contract home; it does not rule
  the shared contract.
