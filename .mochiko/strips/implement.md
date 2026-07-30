# Strip notes — `commands/implement.md`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (BACKLOG item 7, the
**fifth and final** one-shot-command wave after specify's v0.13.0, slice's v0.14.0, plan's v0.15.0,
and tasks' v0.16.0). The wave also ran the **D2 conversion assessment** (one-shot → team-form) and
re-checked the **S8 home-revision checkpoint** against implement's needs (a standing producer spanning
the whole cycle sequence + the fix-pass loop, a standing verifier fired once per cycle + a
whole-implementation final validation, and a per-cycle confidence gate that auto-approves
deterministic-CLI-pass cycles — **no new shape gap at that wave, when the shape was v2**, so it made
no template revision and no cross-command re-audit). **Stale as a standing claim:** the shape is now
**v4** (2026-07-30), and its D3 devolution changed exactly that confidence gate — see the v0.31.0
entries below.

## [v0.32.0] Build note + shape-v4 re-conform — implement honors the approved architecture (AD-D6; 2026-07-30)

Design record: `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D6 with folds R2/R7/R8,
seam note N1). Not a strip — **additions** (recorded in `DECISIONS.md` row AD-D6, lead-owned landing);
logged here with the version stamp for the audit trail and to name the seam-N1 resolution the record left
to build.

> **Version note:** originally stamped **v0.30.0**; while in flight, origin/main released **v0.30.0** and
> **v0.31.0** (the shape-v3→v4 mesh + devolved-cycle rewrite, the two entries below). The merge rebased
> these AD-D6 additions onto v4, so they land at **v0.32.0** and fold into v4's devolved branch (see the
> re-conform bullet).

- **Briefed input (D6.1):** the approved `architecture.md` joins the design inputs read at Phase 0 step 4
  and is added to the producer's per-cycle brief — it is the **anchor** for the two new mechanisms below.
- **Deviation escalation (D6.2 + R7) — the diagram-anchored mechanical test:** "does this cycle add/remove
  a box, add/remove/redirect an arrow, or move a responsibility across a boundary on the approved diagram?"
  — **self-checked by the producer at cycle open AND cycle close**, reported in `cycle-report.md` and
  surfaced at the cycle checkpoint (Phase 1 step 3). The user re-rules and the approved target is
  **amendable mid-implement with consent** (a consented target amendment updating `architecture.md`, the
  same mechanism as plan's design-time return to G3). Drift caught one cycle deep, never deferred to landing.
- **Built-vs-approved landing diff (D6.3 + R8) — new build capability:** at final validation (Phase 2 step
  3), when an **approved structural delta existed** in `architecture.md`, the `authoring-architecture`
  dispatch runs in **diff mode** (approved target + built code → "built as approved" or the divergence). The
  divergence is surfaced at the **G5** acceptance presentation. This is a *new* capability (R8 — the prior
  `authoring-architecture` only wrote prose from built code); assigned to that dispatch as a named build
  item, taking the approved artifact as input.
- **Seam N1 made explicit (the record's carry-forward):** the `authoring-architecture` dispatch now has
  **two distinct firing conditions**, kept separate at build — the **diff** fires on *approved-delta-existed*
  (broad, independent of what was built, so a silently-descoped approved delta cannot escape both mechanisms),
  run at final validation to reach the G5 decision; the **`ARCHITECTURE.md` fold** fires only on a *built
  structural change* (narrow, the KM writer moment), at Finalize. An approved-but-not-built delta triggers the
  diff without forcing a doc update. **Placement resolution (build decision):** the record has the diff "at
  landing" yet its divergence "surfaces at implement's acceptance," and acceptance (G5, Phase 3) precedes the
  Finalize landing (Phase 4) — resolved by running the diff at final validation (Phase 2, end) so its report
  is available at G5, while the doc fold stays at Finalize. Flagged as a build-seam resolution the record
  deferred (N1).
- **Shape-v4 re-conform (the merge work, this task):** the AD-D6 additions were re-applied onto main's
  v4-conformed implement (the two v0.31.0 entries below) rather than the v3 confidence gate they were first
  written against. The fold: v4 replaced the confidence gate with the **per-cycle checkpoint carrying the
  devolved branch** (a deterministic-CLI-100%-pass + no-deviation + empty-`domain_deps_added` cycle clears on
  qa's PASS-with-evidence, unread by the lead). The architecture deviation self-check **integrates as a
  reported deviation**: a surfaced deviation is a `cycle-report.md` deviation, which **de-devolves the cycle**
  (removing it from the clean branch → lead checkpoint + consented-target-amendment decision) — so the
  deviation rides v4's existing "any reported deviation returns to the lead" rule rather than adding a
  parallel gate. The built-vs-approved diff (Phase 2 step 3) sits on the **lead-routed endgame** (the devolved
  branch clears cycles, never the endgame), consistent with v4's "Clearing under the mesh". Verify hand-off is
  peer-routed (producer→verifier) per the mesh.
- **Consequent edits:** Phase 0 entry gate retargeted to `/mochiko:plan` (the package producer) after the
  `/mochiko:tasks` retirement (see `strips/tasks.md` v0.32.0); done-condition gains clauses **(4)** (the diff
  ran when an approved delta existed) and **(5)** (G5 cleared), atop v4's clause (3) (lead reads escalated
  cycles + final validation only); the per-cycle checkpoint predicate, G5 presentation, state-recovery table,
  and the "What you own" footer updated to carry the deviation check + the diff. The audit-passed "No G2"
  reword ("there is no feasibility-rejection gate") is preserved. **No shape gap** — both mechanisms are
  per-workflow gates/steps folded into v4 doctrine, not a shape revision; shape stays **v4**.

## [v0.31.0] Lead-as-switchboard routing superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place at command altitude: the verifier is still cold-spawned at the first cycle verification (a spawn-timing parameter), the producer↔verifier peer edge is now declared on the roster, and the doctrine stays in the shape.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - seat roster: "spawned **cold at the first cycle verification**, never in contact with the producer"
  - Phase 1 step 2: "**Verify — same round, never skipped.** Message the verifier to verify the cycle against real infrastructure"
  - Contract, Producer ↔ validator: "(verifier cold-spawned at the first cycle verification, evidence/reports lead-routed, no producer↔verifier contact)"
- **Kept deliberately (not superseded):** Phase 2 step 1's lead-routed final validation — the endgame is the lead's under v4, now stated rather than left implicit; and Phase 1 step 1's lead-dispatched targeted retry — a retry follows a failure, and the verdict on a non-clean unit is the lead's.

## [v0.31.0] The clean-cycle verdict devolves to the producer↔verifier pair (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Clearing under the mesh"). implement supplies the parameters: the **cycle** is its clearing unit, and the escalated branch's checkpoint keying is a `production`/`regulated`-tier domain-registry addition.
- **Tier failed:** n/a — supersession by ruling (record **D3**), not a minimalism strip.
- **Content (superseded, faithfully compressed):**
  - Phase 1 step 3 header + read: "**Confidence gate + verdict (you).** Read `cycle-report.md` + the verification report + qa's evidence." — the lead read every cycle, clean deterministic ones included; those now clear unread.
  - Phase 1 step 3 branch: "if every verification is a deterministic CLI check that passed 100%, **auto-approve** and advance to the next cycle" — the auto-approve was the lead's act; it is now the pair's, on qa's PASS-with-evidence.
  - Contract done-condition (3): "*you* Read the cycle-reports + verification reports" → the final-validation report plus every **escalated** cycle's reports; "qa's status is input, never the gate" gains "wherever judgment exists".
  - Contract human gates: "the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass → auto-approve; GUI / subjective / any-failure / a `production`+-tier domain-registry addition → human checkpoint)" → restated as the per-cycle checkpoint carrying the **exact skip predicate**, per shape v4's Contract requirement.
  - "What you own": "the verdict against the default-FAIL done-condition (qa grades from real infrastructure, you Read the cycle-reports + verification reports and decide …)"
  - frontmatter `description:`: "with a confidence-based per-cycle gate"

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
