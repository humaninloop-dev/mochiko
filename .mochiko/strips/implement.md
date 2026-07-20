# Strip notes — `commands/implement.md`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (BACKLOG item 7, the
**fifth and final** one-shot-command wave after specify's v0.13.0, slice's v0.14.0, plan's v0.15.0,
and tasks' v0.16.0). The wave also ran the **D2 conversion assessment** (one-shot → team-form) and
re-checked the **S8 home-revision checkpoint** against implement's needs (a standing producer spanning
the whole cycle sequence + the fix-pass loop, a standing verifier fired once per cycle + a
whole-implementation final validation, and a per-cycle confidence gate that auto-approves
deterministic-CLI-pass cycles — **no new shape gap; shape stays v2**, so no template revision and no
cross-command re-audit this wave).

## [v0.17.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** implement runs a producer↔verifier cycle across a
  **variable-length cycle sequence** (foundation cycles before feature cycles, each execute→verify in
  the same round, targeted retry ≤3/cycle) then a **final-validation + fix-pass loop** (≤3 passes) — the
  **longest producer horizon of any converted command**: not two fixed phases but *N* cycles plus fix
  passes over a **codebase that accumulates as it goes**. The context-retention bet is implement's own
  and is its strongest: a **standing producer seat** (`staff-engineer`) carries (1) the conventions the
  foundation cycles set forward into the feature cycles (the brownfield "follow existing patterns"
  consistency, now *within its own* growing implementation), (2) whole-implementation knowledge into a
  **fix pass that is unconstrained by cycle boundaries** (it may touch any cycle's files — a cold spawn
  would rebuild the entire implementation's mental map from disk), and (3) targeted-retry coherence (it
  re-opens only the failed tasks of code it wrote). The verifier maps to a **standing qa seat**: cold at
  the first cycle verification, then messaged once per cycle and again for the whole-implementation final
  validation — its retained per-cycle context is what makes the final validation informed by what it
  already checked rather than a cold whole-repo read. The verifier never contacts the producer, and the
  verification skill is never mounted on staff — independence stays structural. Transport rides the v3
  fix (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn, the
  foundation-cycle-1 implement).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's, slice's, plan's, and tasks' own checkpoints all
  unfired; brainstorm v2 measured standing seats *more* expensive than dispatches). Implement is
  **two-seat** (nearer tasks'/slice's cost than plan's three-seat load), so its team-form tax is moderate
  if the retention payoff doesn't land. Two honest weak points. First, **implement's producer craft is
  specifically built to reconstruct context from disk**: `brownfield-integration`'s whole discipline is
  "read the full file first, identify its conventions, follow them" — so a cold per-cycle producer is
  *designed to be safe* re-reading the accumulating code, and the retention payoff is narrower than the
  raw cycle count suggests (it is the *authorial judgment* — why a pattern was chosen, what scope
  discipline deliberately left out — which the `cycle-report.md` records as prose but a cold reader must
  re-derive, not the code itself, which is fully on disk). Second, **the qa seat is the weaker team-form
  fit** (implement's analogue of plan's fire-once architect): its verification is **Tier-1 deterministic**
  — real-infra evidence + quality-gate exit codes, re-run fresh each cycle, and the final validation
  re-runs the full suite regardless — so a cold-respawned verifier would reconstruct almost nothing;
  modeled as a standing seat messaged per-cycle for uniform transport, its persistence buys the least of
  the two seats. Ruled team-form anyway per D2's declared default + S4 (no prior dogfood evidence
  required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:implement`" BACKLOG item, Implement-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check (the foundation-cycle-1 implement); the standing producer
  seat is messaged (not respawned) across cycles, across targeted retries, **and across the cycle→fix-pass
  boundary** (whole-implementation knowledge carried into a cross-cycle fix pass); the verifier spawns
  **cold at the first cycle verification**, is messaged once per cycle and for the whole-implementation
  final validation, and **never contacts the producer**.

## [v0.17.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four
  requirements (default-FAIL done-condition, independent validation, bounded iteration, named human
  gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this
  command states only what is specific to *this* workflow: the cycle sequence, the execute→verify
  pairing, the retry / fix-pass bounds, and the two implementation gates." — restated loop-discipline's
  own enumeration; the workflow-specific tail survives as the converted goal + the sections themselves.

## [v0.17.0] Per-run contract fill (`workflow-contract.md` → `implement-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the authoring-time-fill
  rule); the per-workflow values survive as the command's authoring-time Contract section (implement's are
  a four-part done-condition, the targeted-retry / fix-pass / convergence-stall bounds, and the confidence
  gate + G5 + G1/G3/G4 + the no-G2 note)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/implement-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof —
  not this command body."

## [v0.17.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  qa's-status-is-input boundary also lives on `qa-engineer`'s persona + REGISTRY's "independent Tier-1
  validator" row). The per-phase Verdict *steps* (Phase 1 step 3, Phase 2 step 2) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated three times pre-wave — the lead framing ("qa presents evidence and a checkpoint
  recommendation; **you own the clearing verdict** … qa's status is input, never the gate"), the Contract
  Team clause ("verifier `mochiko:qa-engineer` … never implements … the verification skill is never
  mounted on staff"), and the footer ("the verdict (qa grades from real infrastructure, you Read the
  cycle-reports + verification reports and decide against the default-FAIL done-condition … qa's status is
  input)").

## [v0.17.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules); the
  "always dispatch via the Task tool" line superseded by the team-form conversion (transport now per shape
  Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.17.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase`
  field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`)" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives, as does
  Phase 0 step 4's "workspace-as-state, no registry field" (a genuine survivor, as in the siblings).

## [v0.17.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in ROADMAP's
  Decision Trail + `.mochiko/transform/implement/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps," routed on an
  autonomously-evaluated gate verdict, and had **no** final-acceptance gate — it could churn indefinitely
  or self-declare done. The deterministic caps, the lead-owned verdict (qa's status is input), and the new
  G5 acceptance gate close the gates HIL lacked." — the shape of specify's / plan's / tasks' deleted
  HIL-comparison blockquotes; its rationale is carried by the Contract done-condition (the deterministic
  caps + lead-owned verdict + G5), so no unique behavior is lost.

## [v0.17.0] Slice-scoped entry — de-restated to the Graduation-contract reference
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section — the
  single home of the consumption rules); Phase 0 step 6 now *applies* the contract by reference for slice
  resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the one-shot entry variant declared the Graduation contract "the single source of
  the consumption rules; do not restate it" and then restated slice-resolution + staleness-guard rules
  beneath that self-declaration — the same D1 churn liability the plan wave's `validation-command-shape`
  audit caught on plan's identical entry and the tasks wave de-restated in-conversion; applied here
  proactively by that prior ruling, **NOT contested**)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order whose `slices/<slice>/tasks.md` has unchecked tasks") and the **staleness guard** ("the live
  `spec.md` story-ID set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`").
  implement's genuine own bindings were **kept**: the entry gate + cycle loop read `slices/<slice>/tasks.md`;
  the design inputs are the shared feature-root artifacts plus `slices/<slice>/{plan.md, task-mapping.md}`;
  per-slice outputs (`cycle-report.md` + verification reports) land under `slices/<slice>/` and what that
  does to the done-condition's artifact set; the **full-repository-suite regression net** (implement is the
  only slice-scoped consumer that runs the quality gates, so "the gates run the full repo suite" is its own
  operationalization of the contract's regression-safety rule, not a restatement); and the
  **feature-declared-not-verified-at-last-slice** surfacing (implement is the pipeline's terminal stage —
  only it reaches the last slice's G5, so the Feature-Done handoff is uniquely its responsibility).
- **Note:** the Graduation contract is on the ≥3-consumer queue (plan/tasks/implement slice-scoped
  variants) — this strip relocates implement's *local restatement* to the contract home; it does not rule
  the shared contract. **implement was the last restating consumer** (per the tasks-wave queue note:
  "plan + tasks are now locally de-restated, and implement.md's entry variant is the remaining restating
  consumer") — with this strip, all three consumers are locally de-restated; only the shared-contract
  ruling remains queued.
