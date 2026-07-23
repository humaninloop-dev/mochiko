# Strip notes — `commands/plan.md`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (BACKLOG item 7, the third
one-shot-command wave after specify's v0.13.0 and slice's v0.14.0). The wave also ran the **D2
conversion assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint**
against plan's needs (a standing producer spanning two phases + two reviewer seats, one of them
fire-once — no new shape gap; shape stays v2).

## [v0.19.0] Wave context — token-reduction build, wave 1 (plan cluster)

Provenance: the token-reduction epic (`.mochiko/brainstorms/workflow-token-reduction/record.md`, D1–D6,
accepted 2026-07-23) + its coupled `standing-seat-lifecycle` record, both landed as ROADMAP Key
Decisions. This wave is **contained to the plan cluster** — implement-cluster and command-shape Layer-2
items are queued waves (BACKLOG). **No `command-shape.md` revision this wave → no cross-command
re-audit** (the practiced trigger rule). **Additions (not strips — logged here for provenance):**
the **D5 review-sizing step** (Phase 0, part of G1 — roster both / completeness-only /
none-with-recorded-waiver, defaults keyed to governance tier + slice role; bound *by reference* to the
shape's sized review applied to plan's **in-loop** roster, no `command-shape.md` edit; in-planning user
ratification); the **D2 cost entry** (Phase 5, referencing the new `templates/run-costs-template.md`);
the **`## Review` section** added to `templates/plan-template.md` (the D5 waiver's durable home). The
seat-declaration qualifiers (both reviewers "per the G1 sizing ruling"), the Contract
validation-model / done-condition / **producer↔validator** / human-gates updates, and the
state-recovery conditioning all ride the D5 sizing ruling — additions/edits in service of D5, not
independent strips. The **producer↔validator** qualifier ("reviewers on the G1-sized roster" + the
none-branch validator = you + the user) is a **coherence propagation beyond the three Contract
sub-edits the plan named** — landed to keep the Contract internally consistent once the done-condition
became sizing-conditional, mirroring `brainstorm.md`'s sized producer↔validator clause (its
"Validator = the reviewers per the sizing ruling … under a waiver the user is the sole validator").

> **Sizing-binding watch-item for the audit:** the Phase-0 sizing step uses the conformant
> "Run the shape's sized review with these bindings" idiom (matching brainstorm.md:30 / setup.md:129),
> stating only plan's parameters and referencing the mechanic — no `<!-- shape-exception -->` marker
> carried. Its one novelty is applying the sized-review mechanic to plan's **in-loop** roster (the shape
> frames it end-stage), disclosed in-line and authorized by the in-planning ratification. If the
> `validation-command-shape` grep floor reads the binding as restated shape prose, the codified marker
> is the ready remedy (per the plan) — landed in-wave, not reworded around.

## [v0.19.0] D6d — round-report retain/clean offer → clean-by-default
- **Disposition:** superseded in place (ruled behavior change — the offer is gone; cleanup now defaults to clean)
- **Tier:** n/a — a user-ruled behavior change (epic **D6d** + Q6 "no softening"), not an altitude/purpose failure
- **Content (verbatim, replaced):** "Offer a lightweight retain/clean choice for the intermediate round
  reports; never offer to delete `plan.md` or the six artifacts — they are the deliverables." → now:
  "**Clean the intermediate round reports by default** (retain only on request); never touch `plan.md`
  or the six artifacts — they are the deliverables."
- **Consumers assessed:** plan-only (the finalize step is plan's own; single-consumer, in-wave per D9).

## [v0.19.0] D5 — validation-model "unsized by design" phrasing replaced
- **Disposition:** superseded in place (the in-loop critique is now sized at G1, not unsized by design)
- **Tier:** n/a — a user-ruled scope change (epic **D5** + in-planning ratification), not an altitude/purpose failure
- **Content (verbatim, replaced):** "the bounded in-loop critique — every round, unsized by design; no
  sized end-stage review (the shape's in-loop-critique branch)." → now: "…every round, at the
  **G1-sized roster** (both / completeness-only / none-with-recorded-waiver, per Phase 0); no sized
  *end-stage* review (the shape's in-loop-critique branch, with the roster count sized at G1)." The "no
  sized end-stage review / in-loop-critique branch" reference is **retained** — plan still has no
  end-stage review; only its in-loop roster is now sized.
- **Consumers assessed:** plan-only (Contract fill is plan's own; single-consumer, in-wave per D9).

## [v0.15.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** plan runs a producer↔two-reviewer cycle (≤3 rounds
  per phase, gap-list-driven revision, cold reviewers) across **two phases** (Analysis → Design) whose
  context-retention bet is plan's own — the longest horizon of any converted command: a **standing
  producer seat** holds (1) the Phase-1 analysis rationale carried into the Phase-2 design across six
  artifacts (why a decision beat its alternatives, which constraint shaped it, what NFR targets bind —
  authored from lived context, not reconstructed from files), and (2) the C↔D dependency web so a
  targeted revision after a feasibility rejection stays coherent. The two reviewers map to: a
  **standing completeness advocate** (`devils-advocate`, cold at first spawn, spans both phases — its
  retained Phase-1 context is what makes the Phase-2 incremental consistency check a spot-check, not a
  full re-read) and a **cold fire-once feasibility architect** (`principal-architect`, grades once
  post-Phase-1-produce, re-fires only on a structural change, never grades Phase 2). Neither reviewer
  contacts the producer — independence stays structural. Transport rides the v3 fix
  (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's + slice's own checkpoints unfired; brainstorm v2
  measured standing seats *more* expensive than dispatches). Plan is the **most expensive command to
  run as a standing team** — three seats across two phases vs specify/slice's two — so it pays the
  largest team-form tax if the retention payoff doesn't land. The **fire-once architect is the weakest
  team-form fit**: it usually fires once and sits dormant, getting little from persistence (modeled as
  a standing seat messaged sparsely — uniform transport, and it keeps its Phase-1 read on a
  structural-change re-fire; the honest steelman is that the architect alone would be fine as a
  bounded one-shot subagent). And the design artifacts **reconstruct relatively cheaply from disk** —
  the six artifacts are richly ID'd and the FR→TR→entity→schema traceability is written *in the files*,
  so the retention payoff, while real, is smaller than "six artifacts / two phases" suggests. Ruled
  team-form anyway per D2's declared default + S4 (no prior dogfood evidence required; checkpoint
  below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:plan`" BACKLOG item, Plan-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check; the standing producer seat is messaged (not
  respawned) across rounds and across the phase boundary; the completeness advocate spawns cold and is
  messaged in Phase 2 for incremental mode; the feasibility architect fires once and re-fires only on
  a structural change; neither reviewer contacts the producer.

## [v0.15.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all
  four requirements (default-FAIL done-condition, independent validation, bounded iteration, named
  human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here…"
  — restated loop-discipline's own enumeration.

## [v0.15.0] Per-run contract fill (`workflow-contract.md` → `plan-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the
  authoring-time-fill rule); the per-workflow values survive as the command's authoring-time Contract
  section (plan's are richer — a per-phase round cap and five gates)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/plan-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable
  proof — not this command body."

## [v0.15.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-feasibility` + `review-plan-artifacts` descriptions
  + REGISTRY). The per-phase Verdict *steps* (Phase 1 step 4, Phase 2 step 3) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated at the lead framing ("Each reviewer *recommends* a status; **you own the
  clearing verdict** — their status is input, never the gate") and again in the footer ("the verdict
  (each reviewer grades from the files, you Read the artifacts and decide against the default-FAIL
  done-condition — their status is input)").

## [v0.15.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.15.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`):" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives.

## [v0.15.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in
  ROADMAP's Decision Trail + `.mochiko/transform/plan/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on
  each agent's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline`
  reqs 1 & 3. The two reviewers' three-state statuses survive only as input to your verdict; the
  deterministic cap and the new G5 acceptance gate close the gates HIL lacked." — the shape of
  specify's deleted HIL-comparison blockquote; its rationale is carried by the Contract done-condition
  + `review-feasibility`'s "Preserve `infeasible` as a distinct state" doctrine, so no unique behavior
  is lost.

## [v0.15.0] Slice-scoped entry — restated Graduation-contract rules (audit catch)
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section —
  the single home of the consumption rules); Phase 0 step 5 now *applies* the contract by reference
  for slice resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the step declared "the single source … do not restate it" and then restated most
  of it — the D1 churn liability)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order lacking `slices/<slice>/plan.md`"), the staleness guard ("the live `spec.md` story-ID
  set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`"), extend-mode ("the
  shared feature-root artifacts are brownfield input the producer extends in place — never re-derives,
  never forks per-slice copies"), and the graded-amendment definition ("a **breaking** change … is a
  graded amendment … never a silent rewrite"). The four genuine plan bindings were **kept**: G4 on
  over-scope, the `[MODIFY]`-surfaced-for-this-round's-reviews behavior, the per-slice-output →
  done-condition mapping, and the reviewer briefing sets.
- **Note:** caught by the `validation-command-shape` audit — the assessment had passed this entry as
  at-altitude on its "do not restate it" self-declaration; the audit found the restatement beneath it.
  Fixed in-wave, no version bump. The Graduation contract is on the ≥3-consumer queue (plan/tasks/
  implement slice-scoped variants) — this strip relocates plan's *local restatement* to the contract
  home; it does not rule the shared contract.
