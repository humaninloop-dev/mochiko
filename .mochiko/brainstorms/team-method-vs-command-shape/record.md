# Record — team-method-vs-command-shape

**Opened:** 2026-07-25 · **Mode:** bare session (direct `mochiko:analysis-iterative`
invocation — no team; lead-penned record, confidence marks per element)
**Topic:** what the kinako MVP-H1 lead+team build session's *observed* working method
teaches — and contests — in the codified mochiko command shape
(`plugins/mochiko/templates/command-shape.md` v3) and its team-form commands.
**Input:** [inputs/kinako-mvp-h1-team-note.md](inputs/kinako-mvp-h1-team-note.md) — the
session design note, transcript-backed (session `de55b447…` + four teammate transcripts).
**User's opening stance:** likes the team structure the kinako session ran; wants it
contrasted with mochiko's command structure.

## Grounding (what IS)

- Shape v3 Layer 2, **Independence by structure**: cold seats never in the room before
  their stage, and *"producer↔validator traffic routes through the lead — who talks to
  whom is the independence guarantee"* (`templates/command-shape.md:132-134`). The lead
  also owns the loop, every verdict, every human gate (Layer 1, "One lead").
- `commands/implement.md`: single standing producer + single standing verifier, both
  lead-routed with *"no producer↔verifier contact"* (Contract, line ~183); cycle loop is
  **sequential-only** — parallel cycle execution is an explicit deliberate-shortcut
  deferral *"pending dogfooding"* (`commands/implement.md:105-107`).
- Structuring lives upstream in a separate command (`/mochiko:tasks` → `tasks.md`);
  implement consumes it. Mochiko task artifacts carry cycles, file paths, `[EXTEND]`/
  `[MODIFY]` markers — but none of the kinako note's three concurrency-safety mechanisms
  (frozen seam signatures, contested-file single-ownership, named wait-fallbacks).
- The kinako note (transcript-backed): 30 peer-to-peer messages vs 2 lead messages;
  verification routed engineer→qa directly; two producer streams concurrent in one tree;
  observed costs — qa drifted into coordination hub (inbox 11), 29 duplicated full-suite
  runs, and a warm-machine blind spot that shipped the run's one bug.
- **Confound to carry:** the kinako run was hand-rolled, not a mochiko command run — its
  discipline lived in three long lead-authored spawn prompts and an `opus` lead, not in
  codified doctrine. One run, one repo, one author's machine.

## Contrast map (the forks)

- **F1 — Routing topology.** Lead-as-switchboard (shape: routing *is* the independence
  guarantee) vs peer-routed verification with the lead as exception handler (kinako
  rule 4). Direct doctrinal collision.
- **F2 — Parallel producer streams.** Sequential-only single producer (implement) vs two
  concurrent streams made safe by the three mechanisms. The kinako run is dogfood
  evidence bearing on the deferral implement itself named.
- **F3 — Lead role & cost.** Owns loop/verdicts/every gate and reads every report vs a
  2-messages-in-108-minutes exception handler (who still spent 204k output tokens — more
  than either engineer).
- **F4 — Where structuring lives.** Separate upstream `/mochiko:tasks` command vs an
  in-session synchronous architect (idle after 7 minutes — but its task doc carried the
  three safety mechanisms mochiko's task artifacts currently lack).

**Already aligned (no fork):** independent verifier / producer never grades own work;
deviations reported, never silently resolved; a designated whole-suite runner (kinako's
29 duplicated gate runs are the *absence* of mochiko's discipline, not a critique of it);
cold-checkout gate missing in both (kinako rule 7 — a gap either way).

## Decisions

- **D1 — F1 routing topology: the mesh becomes Layer 2's default.** `Contested` — the
  user ruled A (absorb the mesh as shape doctrine) over the lead's recommendation B
  (routing as a per-command declared parameter); the lead's one push-back ran as the Q2
  scope challenge and the user took the carve (D2), so D1 stands as A-scoped. Peer-routed
  in-loop verification is the default: producers hand work to the verifier directly, the
  lead rules on policy / deviations / scope — exception handler, not switchboard. The
  Layer 2 "Independence by structure" sentence is re-carved: routing no longer *carries*
  independence for in-loop seats (kinako evidence: qa independent behaviorally — re-ran
  every TEST task, checked premises — while never routing through the lead).
- **D2 — Mesh scope: in-loop traffic only; cold review stays cold, reframed as a stage
  property.** `Confident` — user adopted the recommendation after an explicit steelman of
  mesh-everywhere. The mesh default binds the kinako-evidenced class (in-loop
  producer↔verifier↔producer). Cold end-stage review keeps structural isolation — cold
  spawn, counterparts withheld, the bounded one-shot cross-exam — but the shape now
  states it as a property of the *review stage* (who is in the room before their stage),
  not as a routing doctrine over all traffic. Rationale: the kinako run contained no
  cold-review stage (its qa was in-loop from spawn), so mesh-everywhere would have been
  an evidence-free extrapolation into the one structure whose independence genuinely is
  carried by who-talks-to-whom.

- **D3 — Cycle clearing under the mesh: the clean-cycle verdict devolves to the pair;
  the lead rules on escalations and the endgame.** `Confident` — user adopted the
  recommendation. The confidence gate relocates, branch-split: the **auto-approve branch
  devolves** — a cycle whose verifications are all deterministic-CLI at 100% pass, with
  no reported deviations and no domain-deps additions, advances on qa's
  PASS-with-evidence, unread by the lead. Everything else — any failure, GUI/subjective
  verification, reported deviation, `production`-tier domain-registry addition —
  escalates to the lead + human checkpoint exactly as today. The lead's done-condition
  read shifts to final validation plus escalated cycles. Guard named in doctrine: the
  devolved branch is *exactly* the deterministic-and-clean one — "qa's status is input,
  never the gate" stays true wherever judgment exists; for a green deterministic run the
  classification always was the gate and the lead's read was ceremony. (Kinako-faithful:
  its lead read one verification — the commissioned final validation — and ruled on the
  one surfaced deviation; nothing else needed it.)

- **R1 — Reframe (user, mid-session): the target is the command *surface*, not any one
  command.** The lead's Q4 (implement multi-stream adoption mode) was posed at command
  level; the user redirected: *"I don't want to go deep into individual commands,
  because I am looking to reduce them."* Q4 is withdrawn unruled — its substance
  (stream partition, the three safety mechanisms in task artifacts) survives as design
  input to whatever command absorbs the delivery chain, not as an implement.md ruling.
  D1–D3 stand unaffected: they are shape-level (Layer 2 routing, cold-review carve,
  verdict devolution), and a smaller command surface still runs on the shape.

- **D4 — Surface reduction, first step: `/mochiko:tasks` merges into `/mochiko:plan`.**
  `Contested` — the user ruled a deliberately smaller step ("start small") than the
  lead's recommended four-command surface (setup · brainstorm · specify · build): plan
  and tasks combine into one design-room command; specify, slice, implement, setup,
  brainstorm unchanged. Surface: 7 → 6. Evidence note, carried honestly: this specific
  merge is **adjacency-grounded, not kinako-evidenced** — the kinako session merged
  structuring into the *build* room (architect seat inside the implement session),
  whereas plan+tasks merges the two design-time loops on their own grounds: same
  producer→validator shape, shared spec input, tasks consuming plan's outputs
  immediately, and an inter-command boundary whose only content was invocation ceremony.
  The fuller reduction (build absorbing structuring per the kinako seam; slice folding
  into the spec side) is `Deferred` — recorded as the lead-recommended direction, open
  for a later session, not ruled here.

- **D5 — The merged plan's human gate: one final acceptance on the whole package.**
  `Confident` — user adopted the recommendation. The command's named gate moves to the
  end: design + mapping + task breakdown accepted as one package, which remains
  implement's unchanged entry condition. The standalone design-acceptance signature
  dissolves — it was load-bearing only while a command boundary sat there. Mid-course
  rulings (NEEDS-CLARIFICATION decisions, contested design forks, scope changes) still
  route to the user as they arise — existing `loop-discipline` gate routing, untouched.
  Every validator runs unchanged (feasibility, plan-artifacts, task-artifacts): the
  reduction is ceremony, never verification. The B-steelman's residual (structuring
  wasted on a design the user would have redirected) is absorbed as a sizing-gated
  design checkpoint *on request* in judgment-heavy runs, not a standing gate.

## Open questions — dispositions at close

- **Cold-checkout gate** (kinako rule 7 — the one bug that shipped): a gap in *both*
  structures. → **BACKLOG build item** (user chose close-over-settle at the wrap gate);
  carries one open placement ruling — implement's final validation vs the audit /
  feature-close charter.
- **Layer 2 rewrite detail:** the mesh doctrine must name the traffic classes —
  peer-routable = verification hand-offs; lead-routed = coordination notices, policy,
  deviations, scope (the counterweight to kinako's observed qa hub-drift, inbox 11).
  → **folded into the Layer 2 build item.** Version coordination flagged there:
  standing-seat-lifecycle's deferred Layer 2 rewrite targets the same surface at v4+.
- **Deferred by ruling** (direction recorded, not ruled — re-open in a dedicated
  session): the build-room merge (structuring seat inside implement — the seam the
  kinako evidence actually witnessed), slice folding into the spec side (toward the
  four-command surface: setup · brainstorm · specify · build), multi-stream implement's
  adoption mode + the three safety mechanisms' artifact home. → **BACKLOG deferred-
  direction item**, design input preserved here (R1/D4).

## Close (2026-07-25)

Concluded by user ruling ("close the record, and do bookkeeping"). **Record un-reviewed**
(bare session — direct skill invocation; no team seats, no sizing gate offered as a
command run would have: the validator seat passed to the user, who accepted or
counter-ruled every element inline as it was made — D1/D4 `Contested`, D2/D3/D5
`Confident`, Q4-original withdrawn at R1).

**Landed:** ROADMAP Key Decisions (team-method row, 2026-07-25) · BACKLOG
"Team-method-vs-command-shape build items" (Layer 2 mesh rewrite D1–D3 · plan-absorbs-
tasks merge D4–D5 · cold-checkout gate · deferred direction) · index entry → accepted.
Build not started in this session.
