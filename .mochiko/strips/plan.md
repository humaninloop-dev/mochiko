# Strip notes — `commands/plan.md`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (BACKLOG item 7, the third
one-shot-command wave after specify's v0.13.0 and slice's v0.14.0). The wave also ran the **D2
conversion assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint**
against plan's needs (a standing producer spanning two phases + two reviewer seats, one of them
fire-once — no new shape gap at that wave, when the shape was v2). **Stale as a standing claim:** the
shape is now **v4** (2026-07-30) — see the v0.31.0 entry below.

## [v0.32.0] Build note + shape-v4 re-conform — merged design-room command: absorbs `/mochiko:tasks` + gains the architecture stage (2026-07-30)

Design records: `.mochiko/brainstorms/team-method-vs-command-shape/record.md` (D4/D5 — plan absorbs
tasks) + `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D1–D9 with folds R1–R10,
seam notes N1–N3). Not a strip wave — a feature build; the architecture-stage **additions** are recorded
in the `DECISIONS.md` rows AD-D1–D9 (lead-owned landing), not here (Job-4 rule: pure additions ride the
decision row, the v3 run-cost precedent). This note logs the version stamp, the **relocation** (tasks'
structuring loop moved *into* plan), the consequent cross-reference change, and the **shape-v4
re-conform** the merge required. Overall command surface 7 → 6 — see the tasks retirement note
(`strips/tasks.md` v0.32.0).

> **Version note:** this build was originally stamped **v0.30.0**; while it was in flight, origin/main
> released **v0.30.0** and **v0.31.0** (the shape-v3→v4 mesh rewrite + the six-command re-conform,
> below). The merge rebased this build onto v4, so it lands at **v0.32.0** and is re-stamped throughout.

- **Relocation IN (from `commands/tasks.md`, now retired):** the entire Mapping → Tasks structuring loop
  — the standing `task-architect` (`patterns-vertical-tdd`) producer seat, the `devils-advocate`
  (`review-task-artifacts`) reviewer in its early-mapping-then-cumulative modes, the two-sub-stage round
  loop, and the task-artifact deliverables (`task-mapping.md` · `tasks.md`) — relocated into plan's
  **Phase 4**. tasks' standalone `tasks.md`-acceptance gate (its G5) **dissolves** into plan's single
  final **package acceptance (G7)** per team-method D5 (the standalone signature was load-bearing only
  while a command boundary sat there). The `review-task-artifacts` validator is **unchanged** in
  structure — same agent, same skill, same checklists; only its caller moved. The completeness reviewer
  is now **one standing `devils-advocate` seat** that runs `review-plan-artifacts` across the design
  stages and `review-task-artifacts` across structuring (the skill is named per dispatch, never loaded as
  frontmatter — shape Layer 2), rather than two separately-spawned reviewers across two commands.
- **Addition — the architecture stage (AD-D1–D9; recorded in DECISIONS, summarized here for the trail):**
  a new **Phase 2** between Analysis and Detailed design, authored by a **new standing `system-architect`
  seat** (`mochiko:patterns-system-design`) — the delta `architecture.md` artifact + the structural D-XXX
  rows into `constraints-and-decisions.md`, its own **early sign-off gate (G3)** presenting the *rendered*
  diagram (degrade-with-record fallback, D8/R5, marked as a shape-exception in the command), always-on
  incl. the no-delta form (D5), and a bootstrap **baseline-confirmation gate (G2)** when no
  `ARCHITECTURE.md` exists (R6a). The `principal-architect` feasibility seat gains an **architecture pass**
  (topology feasibility + governance conformance) — the carve-out of its former "never grades past Phase 1"
  bar (R1, named build work); `review-plan-artifacts` gains architecture-coverage + conforms-to-architecture
  checks (referenced, the skill owns them). Detailed design (former Phase 2, now **Phase 3**) must conform
  to the approved architecture; a contradiction found in authoring **returns to G3** for a consented target
  amendment (R2).
- **Gate renumber (consequent):** the architecture gates insert early, so plan's gates renumber —
  G1 (entry) · **G2** baseline-confirm (bootstrap) · **G3** architecture sign-off · G4 feasibility/governance
  rejection (was G2, now also carrying the governance two-exit, D9.3) · G5 clarification (was G3) · G6
  exit-early (was G4) · **G7** final package acceptance (was plan's G5 *and* tasks' G5, merged). Note the
  renumber against main's v0.31.0 entry below: that entry conformed the *two-phase* plan, where
  feasibility-rejection was **G2**; in the merged command it is **G4** (its "G2" references are frozen
  two-phase history).
- **Shape-v4 re-conform (the merge work, this task):** the merged command was re-authored against
  `command-shape.md` **v4** (main's v0.31.0 bumped it from v3). The v4 idiom adopted: (a) the **in-loop
  mesh** — each producer is **peer-edged with the completeness reviewer**, handing finished artifacts
  directly (peer-routable delivery), while **delivery is not a start signal** (the lead opens every round
  and every review pass); (b) the **feasibility architect stays lead-gated** — fired selectively, its
  concerns routed through the lead at **G4** (not peer-edged, matching main's v0.31.0 narrowing on the
  two-phase plan); (c) the roster **names each seat's peer edges** per the v4 seat-roster PARAM; (d) the
  Contract states **"No devolved branch"** — every plan review is a judgment grade (feasibility,
  completeness, architecture coverage, task-artifact quality), never all-deterministic-CLI, so no gate is
  skipped and every verdict is the lead's; (e) "no producer↔reviewer contact" is dropped from the Contract
  (independence now rides disjoint agents/skills + cold *arrival*, not routing). The architecture stage's
  own peer edge: `system-architect` is peer-edged with the completeness reviewer for the coverage grade;
  the architecture *feasibility pass* is lead-gated like the analysis pass.
- **Cross-reference change:** Phase 5's next-step pointer `→ /mochiko:tasks` is superseded; the merged
  command produces the whole package and points `→ /mochiko:implement`. `templates/plan-template.md` gained
  an **Architecture** section (pointers to `architecture.md`, per the summary-not-restatement rule) and now
  lists `architecture.md` / `task-mapping.md` / `tasks.md` in its Artifacts manifest.
- **Producer report added:** `templates/sysarchitect-report-template.md` — the `system-architect`'s
  self-disclosure carrier (report: disclosure, per `report-format.md`), parallel to the techanalyst /
  taskarchitect report templates.
- **Conversion re-assessment:** the merge does not re-open the team-form ruling — all three producers
  (technical-analyst, system-architect, task-architect) and both reviewers stay standing/cold seats per the
  existing conversion assessments below and tasks' (retired) assessment. **S8 home-revision checkpoint
  re-checked:** the merged command is a larger team (3 producers + 2 reviewers, 7 gates) but rides the
  existing shape — Layer 1 as-you-go artifact + producer-authored uncertainty branch, Layer 2 mesh
  peer-edges + independence-by-cold-arrival — with **no new shape gap** (the rendered-diagram gate is a
  per-workflow gate, not shape doctrine; marked shape-exception where a line would otherwise restate
  shape). Shape stays **v4** (this build conforms to it, does not revise it). The first-dogfood
  confirm-or-revert checkpoint carries forward: the open "Dogfood `/mochiko:plan`" item now exercises the
  merged, architecture-first command.

## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: both reviewers are still cold-spawned at their own stage (a spawn-timing parameter), and the producer↔reviewer peer edges are declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 within a phase is a message to the same seat carrying the reviewers' gap list verbatim"
  - feasibility reviewer: "spawned **cold after the Phase-1 analysis is authored**, never in contact with the producer"
  - completeness reviewer: "spawned **cold at the first completeness review**, never in contact with the producer"
  - Phase 1 step 1: "on round > 1 the message carries the reviewers' gap list for targeted revision"
  - Contract, Producer ↔ validator: "(both reviewers cold-spawned, gap lists lead-routed, no producer↔reviewer contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — plan has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit. Also kept lead-gated: the **feasibility architect's engagement** (fired once, re-fired only on a structural change) and **Phase 2's mode-selecting message** — both policy calls under the traffic classes, not hand-offs.
- **In-wave correction (audit round, 2026-07-30):** the peer edge this wave first wrote was **blanket** — "**Peer-edged with both reviewers:** it hands each round's finished artifacts straight to them" — and its Phase 1 step 1 counterpart "handing them to the reviewers directly when the round's set is complete". The audit caught that this silently peer-routed the *architect*, whose fire-once/re-fire-on-structural-change engagement is a lead-gated policy call, and that it read as licensing a completeness pass before the feasibility gate. Narrowed to the **completeness reviewer only**, with the lead sequencing when it grades. Logged as an in-wave correction, not a separate version: the superseded text never shipped outside this wave.
  - **Second audit round (same wave):** the narrowing was applied to the roster bullet and Phase 1 step 1 but **not propagated**, leaving three sites still asserting the blanket edge. Substance was upheld; only propagation failed. Also superseded, same correction: the feasibility-reviewer bullet's "peer-edged with the producer thereafter" → "**lead-gated thereafter** — you fire it, and its concerns reach the producer through you (G2)"; and the Contract's "gap-list hand-offs peer-routed producer↔reviewer per the shape's mesh, with every verdict yours" → the completeness list peer-routed, "the architect's routes through you at G2". Added in the same pass (not a supersession): the completeness reviewer's **verifying-side hold** — "Delivery is not a start signal — it grades only when you open the pass (Phase 1: after the architect; Phase 2: on your mode-selecting message)". The producer-side hold is universal and lives in the shape home; this one is plan-specific — two-reviewer ordering plus a mode-selected Phase 2 — so it binds at the seat and makes Phase 1 step 1's "you sequence when it grades" a reference to a bound rule rather than a bare assertion.

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
