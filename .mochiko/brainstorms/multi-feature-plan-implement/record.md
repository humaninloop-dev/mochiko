# Multi-feature plan & implement runs — decision record

**Status:** accepted (2026-08-14) — solo cold review via blind-map dispatch (34-angle map, topic-only spawn); verdict critical-gaps → 6/6 dispositioned in one user-ruled batch "as recommended" (C4 = sustain); verify round 1 CLEAN (1 non-blocking hygiene note lead-repaired same round). Acceptance explicitly covered the Q5–Q10 adoption streak as deliberate delegation, the disposition batch, and the D4 `Assumed`→`Confident` flip.
**Opened:** 2026-08-14
**Topic:** give the user flexibility to run `/mochiko:plan` over multiple features together and `/mochiko:implement` to build those features together — today both commands are one-capability-batch-per-run.

## Ground facts (verified against the repo at open)

- **F1** — `plan.md` charter: "One run per capability-batch, ordered by the selected rows' dependency closure." A capability-batch is one capability plus the work rows selected for the run. Entry blocks when selected rows depend on rows not yet `delivered`; batches run in dependency-closure order (`plugins/mochiko/commands/plan.md`).
- **F2** — `implement.md`: same one-capability entry gate; `$ARGUMENTS` = one capability ID (`FEAT-XXX`); the acceptance landing executes that capability's graduation batch (extent fold, status `delivered`, In-flight pointer clear, specs-index touch).
- **F3** — The capability-batch pipeline key is a ruled decision: `pm-role-and-feature-derivation` D7 (2026-08-13, built v0.68.0) — pipeline keys to capability-batch, row-level dependency closure. A multi-capability run amends or extends that ruling's territory.
- **F4** — Package artifacts are per-feature by construction: `.mochiko/features/FEAT-XXX/` holds the deltas; shared product baselines live at `.mochiko/product/` and change only via graded folds at landing.
- **F5** — Plan's convergence gate is the plan-the-plan proposal (approved list = the run's done condition and default-FAIL floor) with an independent contest seat; implement's convergence is run-open confirmation naming batch, scope type, attempt bound.

## Decisions

- **D1 — Batch formation: user declares, lead contests.** `Confident` (adopted as recommended)
  The user names the multi-feature batch at invocation (e.g. `/mochiko:plan FEAT-001 FEAT-002`).
  The lead checks the map for evidence of relatedness — shared parent capability, dependency
  rows between the batched features, shared design surfaces — and files a recommendation
  (keep the batch or split it) that the user rules on before the run proceeds. Relatedness is
  thereby stated on record every time; no objective map-derived gate blocks a batch the map
  has not caught up to, and no ungated free-for-all erodes the one-capability default.
  *Rationale:* mirrors the existing proposal-contest pattern (F5); flexibility stays with the
  user while an independent read keeps "closely related" honest.

- **D2 — Storage: shared spine in the batch's home, deltas stay per-feature.** `Confident`
  The batch (reframed "epic" at Q4) gets one home holding the shared spine — manifest (member
  features, status), the joint plan-the-plan proposal, the joint architecture/seam design,
  batch ordering, plus shared-baseline deltas (D10) — while per-feature design deltas land in their own
  `.mochiko/features/FEAT-XXX/` dirs, linked from the manifest. One place to track and steer;
  downstream machinery (graduation, graded folds, delta-scope runs, regression scope) keeps
  reading the per-feature dirs it already reads.
  *Rationale:* the user's "one place to track and change" need is served by the spine; a pure
  merged package would dissolve at landing anyway (every fold is feature-keyed) and would
  stand up a competing durable unit against `pm-role-and-feature-derivation` D2's
  transience ruling. User accepted the contest: "i think that is good point".
  *Amended by D10 (review fold):* a baseline touched by two or more members is authored once
  in the spine, not per-feature; single-member baselines keep per-feature deltas.

- **D3 — Epic = transient first-class delivery unit.** `Confident` (adopted as recommended,
  after a clarifying exchange on what persists)
  The batch is named **epic**, with identity (`EPIC-XXX`), a home directory, a manifest, and
  status — first-class in the pipeline: mintable, trackable, and the key plan/implement runs
  accept. Its **role** is transient: while in flight it is the active unit and member
  features' pending work rows carry an `EPIC-XXX` marker on the map; at delivery the rows
  fold into their capabilities' extents exactly as today, the markers vanish, and the epic
  closes — its directory persists as readable record (like a closed spec workspace), never
  as a living map layer. The map stays two-typed (durable capabilities + transient work);
  `pm-role-and-feature-derivation` D2/D6 are not reopened.
  *Rationale:* epics coordinate delivery; capabilities state what the product is. A durable
  epic layer would be parent/leaf nesting reborn, and a delivered epic has no ongoing job.

- **D4 — Minting and stewardship: desk stewards, two entry doors, mint-once.** `Confident`
  (was `Assumed`; sustained at review disposition C4 — the ahead-of-run apparatus cost was
  surfaced and the user ruled sustain. Named driver: the user's own Q3 need — "track the
  batch and make changes" over time — plus the cross-workflow first-class reframe; pre-run
  and cross-run coordination is the point, not an accident.)
  The `/mochiko:feature` desk owns the epic's life: mint, membership change, status view,
  close. `/mochiko:plan` invoked with a bare feature list may mint inline through D1's
  declare-and-contest — but only after resolving against open epics: any membership overlap
  with an existing epic surfaces to the user (join it / rule on the overlap), never a silent
  duplicate. Specify's selection stage may **propose** an epic (when one derivation spans
  capabilities), never mint one. One feature's pending rows belong to at most one open epic
  at a time; every workflow resolves `EPIC-XXX` by lookup, re-minting does not exist.
  *Rationale:* one owner, two doors, no orphan or duplicate epics; the mint-once guard came
  from the user's own re-mint question.

- **D5 — One epic type.** `Confident`
  No product/tech epic split. The single epic carries both faces: the manifest (membership,
  why together) is the product view; the spine (joint architecture, seam design, build
  ordering) is the tech view — plan and implement consume the spine sections, not a
  different object type. Batching non-feature technical work (product-lane items) is parked
  as an open thread, triggered the first time the lane holds two related items worth one run.
  *Rationale:* two types cost two registries, type migration, and a mapping ruling — no
  current need pays; user raised the overkill suspicion themself and agreed.

- **D6 — Plan run over an epic: one unit, one plan, single gates.** `Confident`
  (user condition: "as proposed if it means the features will be considered as one unit and
  one plan" — ruling written to honor it)
  One plan-the-plan proposal covers all members (spine artifacts + each member's artifact
  list, rung-justified per member); one contest brief; one architecture sign-off on one
  rendered joint diagram (members' deltas + the seams between them); one `plan.md` spine over
  the package. **Acceptance is of the whole package** — per-feature verdicts exist only as
  the amendment mechanism inside that single gate: the user may accept some members and send
  one back, which opens a targeted rework round in the same run; the run closes accepted
  only when every member is accepted. No partial planned exit.
  **Entry unblocking:** a dependency between rows inside the epic no longer blocks entry —
  the joint design orders it; a dependency pointing outside the epic at a non-`delivered`
  row still blocks, unchanged.
  *Rationale:* drivers (a) ceremony and (b) coherence both land here; the per-feature verdict
  keeps a weak member from hostaging the batch without breaking the one-unit exit.

- **D7 — Implement run over an epic: one unit, member-scoped halts, disposition reserved.**
  `Confident` (user: "take whole")
  One run-open contract naming the epic, members, scope type, and the per-cycle attempt
  bound (unchanged). One merged cycle sequence from the joint plan — shared foundation
  cycles first, then in-epic dependency order; cards stay feature-tagged and every cycle
  report lands in its owner's `.mochiko/features/FEAT-XXX/` dir. One final validation from
  one cold snapshot covering all members; accumulated territory `**TEST:**` gates run once
  over the union of member territories. One acceptance landing executes each member's
  graduation batch (extent fold, `delivered`, In-flight pointer clear) plus the epic close —
  markers vanish, spine archives. A member that exhausts its attempt bound or hits the
  no-progress stop halts member-scoped; the disposition — carve the member out (rows return
  to pending, epic continues) or hold the whole run — is **reserved to the user**, never the
  lead's, because carve-out breaks the one-unit promise (D6).
  *Amended by review folds:* shared baselines fold once from the spine delta (D10) · the
  landing is selection-scope by constraint (D11) · seam bookkeeping writes the D13
  design-time owner · multi-spec closure confirmed compositional (C6 note, Review trail).

- **D8 — Identity mechanics.** `Confident` (user: "take whole")
  Home `.mochiko/epics/EPIC-XXX/` — `manifest.md` (members, status open/delivered/
  closed-partial, why-together line from the D1 contest) + spine files beside it (joint
  proposal, joint architecture delta, seam design, ordering, shared-baseline deltas per
  D10). No separate epics index — the
  directory is the registry, the desk lists it. ID grammar `EPIC-XXX`, sequential, same
  family as `FEAT-XXX`. Map marker: member features' pending work rows carry `[EPIC-XXX]`
  inline — grammar owned by `mochiko:authoring-feature-map`, exact form at build; no new map
  sections. At close the manifest is stamped delivered + dated; the dir stays in place as
  record (D3) — no move, no trail file.

- **D9 — Build surface, supersessions owed, evidence honesty.** `Confident`
  (user: "take whole, deliberate delegation")
  Carrier: slim new model-invoked skill **`mochiko:authoring-epic`** single-sourcing the
  manifest + spine shape, the mint/overlap guard (D4), and close semantics (D7/D8);
  commands reference, never restate; router row added. Touched commands: `plan.md` (epic
  entry, D6 gates, in-epic unblocking) · `implement.md` (D7 whole) · `feature.md` (desk
  stewardship) · `specify.md` (selection may propose) · `authoring-feature-map` (row marker
  + fold-vanish). **Supersessions owed at build:** `plan.md`'s "One run per
  capability-batch" line and `pm-role-and-feature-derivation` D7's pipeline-key clause —
  amended, not deleted: an epic is a run unit composing capability-batches; strips +
  DECISIONS.md annotation per the primitive-edit ceremony. **Evidence honesty:** n=0 — no
  live multi-feature run has ever been attempted; every ruling here is design-reasoned.
  First-live-run watch lands in `BACKLOG.md` at the session landing.
  *Review-fold rider:* `authoring-epic` additionally owns the D10 spine shared-baseline
  delta shape and the transport-steer disclosure; the `authoring-feature-map` touch adds the
  D13 seam-owner grammar.

- **D10 — Shared-baseline deltas author once in the spine; transport-floor steer applies.**
  `Confident` (review fold — C1, Critical, user-ruled "as recommended")
  A product baseline touched by two or more members gets **one joint delta authored in the
  epic spine** under a single pen-holder; a baseline touched by one member keeps its
  per-feature delta (D2 amended). The landing folds each baseline exactly once — spine delta
  for shared baselines, feature delta otherwise — preserving the singular-delta-per-baseline
  graded fold (`implement.md` three-way diff). The transport floor's composition steer
  (`mochiko:patterns-transport-floor`, non-waivable, v0.71.0) governs every epic shared-write
  surface — spine files and shared baseline deltas: concurrent writers get worktree isolation
  or a single pen-holder, disclosed at run open.
  *Rationale:* two per-feature deltas on one baseline had no fold order or overlap
  resolution, and closely related features — the epic's whole scope — are precisely the ones
  sharing data models and contracts; the steer is existing machinery, not new invention.

- **D11 — Epics are selection-scope-only.** `Confident` (review fold — C2)
  Every member enters as selection scope (spec-accepted selection or growth rows). Delta-scope
  cards — bug/improvement deltas on delivered capabilities — cannot join an epic; D7's
  graduation-shaped landing is thereby correct by constraint, not silent presumption.
  Batching related delta cards is parked as an open thread — trigger: the first time two
  related delta cards genuinely want one run.

- **D12 — Driver (c) folds into (a); build speed deferred to multi-stream.** `Confident`
  (review fold — C3)
  Epic "build efficiency" means ceremony reduction — one final validation, one landing, one
  union regression sweep — never parallel building; D7's cycle sequence stays sequential.
  Actual build speed is explicitly deferred to the recorded deferred direction — multi-stream
  implement / build-room merge (ROADMAP Later; BACKLOG deferred-direction item: frozen seams
  · single-ownership · wait-fallbacks). Noted: an epic dogfood (multi-feature,
  dependency-rich) is a likely tripper of the "code-free until dogfooding /
  sequential-cycle-too-slow" revisit bet — surface it when felt.

- **D13 — Within-epic seam ownership assigned at design time.** `Confident` (review fold — C5)
  The later-lander seam default cannot apply inside an epic (members land simultaneously);
  the spine's seam design names each cross-member seam's owner explicitly, and that
  assignment writes the map's seam bookkeeping at close. Exact grammar at build in
  `mochiko:authoring-feature-map` (rides the D8/D9 touch).

## Session trail

- **Q1 — driver.** User: all three — (a) ceremony overhead, (b) design coherence, (c) build efficiency — **scoped to closely related features**. Not a general any-N-features affordance; relatedness is the qualifier.
- **Q2 — batch formation.** Options: pure user call · map-derived gate · user declares + lead contests. Lead recommended (c) declare-and-contest; user: "as recommended" → **D1**.
- **Q3 — package shape.** Lead recommended run-scope-only batching; user countered with a merged `BATCH-XXX` dir ("one place to track and make changes"). Lead contested once — downstream FEAT-keying (F4) + transience ruling — and offered the spine hybrid; user accepted the point → **D2**, then reframed upward: make the batch a **first-class citizen, named "epic", across mochiko workflows**.
- **Q4 — epic ontology.** Lead recommended transient first-class; user asked what "transient" means concretely (tag? docs persist?) — clarified plainly: real object with home + artifacts, persists as record, map presence is a temporary row marker, active role ends at delivery. User: "yes, (a) works" → **D3**.
- **Q5 — minting and stewardship.** Lead recommended all-three-surfaces with desk as steward; user asked whether a workflow-minted epic gets re-minted elsewhere — clarified plainly: mint-once, identity resolved by lookup, overlap guard at every mint door. User moved on without explicit confirm → **D4** marked `Assumed`, to be confirmed at acceptance.
- **Q6 — product vs tech epic.** Lead: overkill — one type, spine already the tech view; product-lane batching parked with trigger. User: "agree, one epic type" → **D5**.
- **Q7 — plan-run gates.** Lead proposed single gates with per-feature verdicts inside acceptance; user conditioned: only if the features are one unit, one plan — ruling written whole-package-acceptance, per-feature verdict = in-run amendment only → **D6**.
- **Q8 — implement mechanics.** Proposed one-unit shape incl. member-scoped halt with user-reserved carve-out; user: "take whole" → **D7**.
- **Q9 — identity mechanics.** Proposed home/ID/marker/archive package; user: "take whole" → **D8**.
- **Q10 — build surface.** Proposed carrier skill + five command touches + supersessions + n=0 honesty; adoption streak Q5–Q10 flagged, user owned it: "take whole, deliberate delegation" → **D9**. Streak stands as deliberate delegation on record.
- **Convergence** — decisions D1–D9 settled; record frozen for cold review; D4 (`Assumed`) to be explicitly confirmed at acceptance.

## Review trail

- **Sizing gate** — lead recommended solo cold review via blind-map dispatch; user: "as recommended". Solo reviewer, default FAIL.
- **Dispatch** — two-message blind-map protocol: message 1 = topic + goal only (no record path, no session outcomes — the "epic" concept itself is session content and was withheld); record path sent only after the angle map returned.
- **Angle map** — 34 angles returned blind (driver/mandate · scope semantics · dependency/ordering · seams/shared surfaces · plan structure · implement structure · landing/partiality · failure modes · cost/fit/rollout · desk dispatch).
- **Verdict: critical-gaps.** 6 survivors — 1 Critical, 3 Important, 2 Minor. Ground facts F1–F5 re-verified accurate by the reviewer; record fitness clean. Lead re-verified the reviewer's ROADMAP/BACKLOG citations (multi-stream deferred direction · sequential-too-slow revisit bet) before presenting dispositions.
  - **C1 (Critical, coverage #13–15):** concurrent baseline-delta folds undefined — two members' deltas on one product baseline have no fold order/overlap resolution; the graded fold is singular-delta-per-baseline by construction (`implement.md` three-way diff); transport floor's composition steer (concurrent writers → worktree isolation or single pen-holder) never invoked despite the epic minting new shared-write surfaces (spine, shared baselines). Demands: shared-baseline authorship rule + steer application.
  - **C2 (Important, coverage #7):** delta-scope members unaddressed — D7's landing presumes selection scope (graduation batches); two related bug/improvement deltas on delivered capabilities have no place. Demands: epic scope-type policy.
  - **C3 (Important, coverage #3/#4):** driver (c) build efficiency claimed, delivered only as ceremony reduction — D7 is sequential; multi-stream implement deferred direction + sequential-too-slow standing bet never reconciled. Demands: define (c) or defer to multi-stream.
  - **C4 (Important, coverage #29):** D4's ahead-of-run apparatus (desk minting, specify proposal, cross-run overlap guard) unpaid by any stated run-scoped driver; D4 is the one `Assumed` decision. Demands: name the driver or trim to run-scoped minting.
  - **C5 (Minor, coverage #12):** within-epic seam ownership on the map post-close unstated (no later-lander exists). Demands: assignment rule.
  - **C6 (Minor, coverage #25):** multi-spec closure across one landing not confirmed. Demands: one-line confirmation.
- **Dispositions — 6/6 in one user-ruled batch "as recommended" (C4 = sustain):**
  C1 → **D10** (spine-authored shared-baseline delta + transport-floor steer; D2/D7 amended) ·
  C2 → **D11** (selection-scope-only; delta batching parked with trigger) ·
  C3 → **D12** ((c) folds into (a); build speed deferred to multi-stream; revisit-bet tripper noted) ·
  C4 → **D4 sustained**, `Assumed` flipped `Confident` with the driver named (user's Q3 tracking need + cross-workflow reframe) ·
  C5 → **D13** (design-time seam-owner assignment) ·
  C6 → confirmation note: multi-spec closure derivation is compositional and unaffected — each spec closes exactly when all its own selected rows have folded, regardless of how many specs one epic landing touches.
- **Duplicate delivery noted** — the reviewer re-sent the identical findings after the lead's nudge; content matched, no supersession.
- **Verify round 1: CLEAN.** All 6 folds verified faithful and internally consistent, record fit; bounded scope honored (no fresh cold read, no coverage hunt on D10–D13, no second reopen). One non-blocking hygiene note — D2/D8 spine enumerations missing the D10 shared-baseline-delta class — lead-repaired same round (pointers added to both lists).
- **Acceptance (2026-08-14):** user accepted the record ("ceept"), covering the adoption streak (deliberate delegation), the 6/6 disposition batch, and the D4 flip. Landing executed: index updated · `DECISIONS.md` row · `BACKLOG.md` "Epic build" section (build item + first-live-run watch) · `ROADMAP.md` Next row (Next cap trip 8/7 groomed by merging the two delivered plan-surface rows).

## Build trail

- **Built same day (2026-08-14) at v0.72.0**, user-directed ("start build"). Wave under the sound-loop + transport floors: two producer seats on lead-approved plans, disjoint file ownership (skills cluster: `authoring-epic` new + `authoring-feature-map` + router · commands cluster: plan/implement/feature/specify + strip); mesh-hold approvals (one plan correction: the D13 cross-ref self-reference fixed to `mochiko:authoring-epic`; one lead ruling: declared-overage over offset-trim); fan-in confirmation on every deliverable; two fresh author≠grader validator seats on the quiesced tree.
- **Audits: 8/8 PASS round 1, zero fix rounds.** Skills validator ruled the `authoring-feature-map` 248-char body overage HOLDS (genuine new obligation, D8/D13). Commands validator confirmed the charter contracts intact (every pre-wave FAIL clause survives), the supersession strip complete with the verbatim line, and internal coherence across the four commands.
- **Supersessions executed:** `plan.md` "One run per capability-batch" line — strip `[v0.72.0]` in `.mochiko/strips/plan.md`; `pm-role-and-feature-derivation` D7 pipeline-key clause — annotated amended on its `DECISIONS.md` row (epic composes capability-batches; single-capability runs + dependency closure stand).
- **Ripple (lead, audited):** plugin.json 0.72.0 · CHANGELOG entry · marketplace synced · budget-ledger notes (`authoring-epic` unbudgeted at birth; feature-map caution updated to 15,661 with the ruled overage) · ARCHITECTURE.md skill counts 33 · build item → trail, watch stays · index/ROADMAP/BACKLOG stamps updated.

## Open threads

- **Product-lane batching (from D5)** — whether an epic (or epic-like unit) may batch product-lane technical work (migrations, infra). Trigger: first time the lane holds two related items worth one run.
- **Delta-scope batching (from D11)** — whether related bug/improvement delta cards on delivered capabilities may batch into one run (epic or lighter unit). Trigger: first time two related delta cards want one run.
