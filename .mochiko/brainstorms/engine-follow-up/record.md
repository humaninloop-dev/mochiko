# Engine follow-up — the remaining callers of the shared thinking stage — decision record

Status: **accepted** 2026-10-15 (user: "accept") · **built** 2026-10-18 at v0.121.0 (branch `engine-follow-up-build`, three waves; `build-log.md`, gate audits `reports/audit.md`, `audit-w2.md`, `audit-w3.md`, all PASS) · cold read `needs-revision` (19 raised → 16 survivors, 0 Critical · 6 Important · 10 Minor, 3 coverage; 16/16 dispositioned: S2 with S14 explored now — the reopen that made D10-two-switch-test — S15 ruled inline, S1/S3 settled by the re-sort, S4/S8, S5, S6/S13 and S16 user-ruled one by one, the rest one user-confirmed repair batch), verify round 1 NOT CLEAN (16/16 CLEAN; D10's bounded verify V5 and four fold-introduced Minors → repair batch 2), verify round 2 NOT CLEAN, Minor only (V1's count and W1, lead-repaired; closed seat-unverified by the user's word) · frozen for the cold read 2026-10-15 after the accept screen ("confirmed") · opened 2026-10-15 · lead: Claude (session) · rulings: deepeshBodh

## Topic (user's words)

> Engine follow-up session — /mochiko:feature joins the shared thinking stage (its unit and where
> its ruled set lives); whether setup's interrogation and specify's intent stage take the contract
> half; implement's other moments on the engine (card slicing and the seed's hand-off to **TEST:**
> cases, cycle-checkpoint batches, final validation's gap-finder reading the ruled set,
> acceptance). Provenance: BACKLOG "Engine follow-up session"; implement-brainstorm-led
> D4-shaped-for-six, OQ2.

Preceding turn (user's words, 2026-10-14): "in release 0.120.0 we released brainstorm based
workflow for architecture and few more. I would like to rollout for other remaining workflows too".

## Orientation

- Session index read 2026-10-15 (`.mochiko/brainstorms/index.md`). The newest entry is
  `implement-brainstorm-led` (accepted and built 2026-10-11 at v0.120.0); no session since. This
  session is the follow-up that record names.
- F1 — `BACKLOG.md` lines 65–71: "**Engine follow-up session** … a brainstorm on:
  `/mochiko:feature` joining the engine (its unit and where its ruled set lives); whether setup's
  interrogation and specify's intent stage take the contract half; implement's other moments (card
  slicing and the seed's hand-off to the `**TEST:**` cases, cycle-checkpoint batches, final
  validation's gap-finder reading the ruled set, acceptance). Trigger: build item C landed —
  **fired 2026-10-11** (built at v0.120.0)."
- F2 — `.mochiko/brainstorms/implement-brainstorm-led/record.md` lines 683–687
  (`implement-brainstorm-led` D4-shaped-for-six): "This build moves the four onto the full stage
  (D3-four-move-now); from the same build setup's interrogation and specify's intent stage follow
  the engine's asking half through their unchanged pointers, the contract off for them, their
  domain content in their own rules (changed at review, S4). A named follow-up session joins
  `/mochiko:feature`, takes implement's other moments, and rules whether setup or specify's intent
  take the contract half."
- F3 — same record, line 697–698 (D4-shaped-for-six, accepted risk): "Feature keeps its own asking
  until the follow-up; the slots are named for users not yet built against, so the follow-up may
  still find a misfit — feature's fit is `Assumed` (S10)."
- F4 — same record, lines 1081–1084 (OQ2): "The follow-up session: `/mochiko:feature` joins the
  engine; whether setup's interrogation and specify's intent stage take the contract half (their
  asking moves in this build, frame change 3); implement's other moments (card slicing,
  cycle-checkpoint batches, final validation, acceptance)."
- F5 — same record, *Defaults batch*, "Where TEST cases sit": "The stage ends at the design files;
  the qa-engineer's `**TEST:**` cases, and the seed's hand-off to them, belong to card slicing —
  the follow-up session (bm D7, E3)."
- F6 — `mochiko-cli rules analysis-iterative --section analysis-iterative.sec.scope` (plugin
  0.120.0), `analysis-iterative.caller-slots`: "A caller fills these slots in its own rules: its
  opening · the unit the user rules · its deliverable and where the ruled set is saved · whether
  the ruled-set contract is on, and who writes after · the blind list's cadence, inputs and job ·
  its independent leg."
- F7 — `.mochiko/schema-views/commands/setup.yaml` lines 49–52 (`setup.interrogation-inline`):
  "Run the interrogation yourself, inline — the agenda's dimensions worked adaptively via
  mochiko:analysis-iterative, then the catalog deck, card by card, recommend-then-arbitrate";
  `pointer: mochiko:analysis-iterative`. `.mochiko/schema-views/commands/feature.yaml` carries no
  `analysis-iterative` pointer and no `thinking-stage` line (grep, 2026-10-15).
- F8 — `BACKLOG.md` lines 36, 78, 87: the first ai-fileops implement, desk and specify runs — the
  engine's first real runs — are all still open.
- Lead's reading (marked as reading): after v0.120.0, `/mochiko:feature` is the one user command
  that rules things with the user and asks its own way, off the engine. Setup and specify's intent
  already ask the engine's way but stop short of its "the next writer stays inside what you ruled"
  half. Implement's later moments with the user — card confirm, cycle-checkpoint batches, final
  validation, acceptance — are still sign-offs over finished work. And none of the engine's callers
  has yet run for real (F8), so every rollout here builds on an untested base.
- Artifact home rendered 2026-10-15 (`mochiko-cli home --plugin-root plugins/mochiko
  .mochiko/brainstorms/engine-follow-up`): closed set `record.md`, `synthesis.md`, `build-log.md`,
  `wave<n>-<slug>.md`; sub-directories `inputs`, `research`, `referents`; reports under `reports/`.
- Knowledge-management file present (`.mochiko/memory/knowledge-management.md`): the close ritual
  applies.

## Run notes

- Floor read-back — brainstorm, 9: user-record-acceptance · author-grader-default-fail ·
  artifact-home · transport-floor · thinking-stage · fail.record-unaccepted ·
  fail.unreviewed-no-waiver · fail.survivor-undispositioned · fail.index-mismatch.
  analysis-iterative, 2: open-unknowns · contract-inside. Fail pin 4 = fail-conditions end line 4.
- Floor read-back — transport floor, 11: message-lane-trigger · topology-lane-trigger ·
  neither-lane-waivable · composition-steer · single-writer-per-surface · mesh-hold ·
  content-pinned-supersession · quiesce-before-cold-grade · no-ritual-sends · fan-in-confirmation ·
  version-floor. Model tiering, 8: class-key-session-tier · seat-default-key · override-is-the-pin ·
  seat-deviation-bounds · persona-less-grader-pin · worker-return-is-a-claim · brief-obligation ·
  worker-seat-set-reserved.
- Composition: the lead holds the record alone (single writer). The blind-map seat
  (`mochiko:devils-advocate`, persona default tier, no deviation) writes only
  `reports/angle-map.md`; any blind second-list seat writes only its own `research/` file. No
  shared write surface; the message lane fires on the end-read resume (mesh hold, content-pinned
  orders, fan-in confirmation). Blind-map seat spawned 2026-10-15 at frame hardening with the
  frame's problem, destination and out-of-scope lines only.

- Record order (disclosed): the frame card sections below sat at the file's tail until 2026-10-15,
  after the defaults batch — the first fact insert had anchored above them; moved here, text
  unchanged, before the freeze.

## Frame card — proposed 2026-10-15, awaiting the user's first reply

| Line | Text | Source |
|---|---|---|
| Problem | The founder using mochiko meets two ways of being asked. `/mochiko:feature` asks its own way, off the shared engine; setup's interrogation and specify's intent stage ask the engine's way but nothing holds the next writer inside what was ruled; implement's card confirm, cycle-checkpoint batches, final validation and acceptance are still "sign this finished draft". Whether a real run has felt any of it: unknown. | lead's guess (F1–F4, F7) |
| Destination | Every place a user command rules something with the user either runs on the shared engine — feature with its unit and its ruled set's home designed; setup and specify's intent with the contract half ruled in or out; implement's other moments placed on the engine — or is ruled to stay a plain sign-off, with the reason. Session ends with decisions one build can carry. | user's words (topic) + lead's guess |
| Must not break | The engine's 21 rules as built at v0.120.0 — a caller never loosens a mechanic, a needed difference becomes a slot by ruling; capability writes stay specify's (feature's charter); the sufficiency check as binding; TDD cycles and real-infrastructure verification; the gap-finder's blindness fence; author≠grader, sound loop, plan approval; the four callers as built. Floors and fails of feature, setup, specify and implement may change only by a recorded supersession at the build. | lead's guess |
| Betting on | The slots named for six (F6) fit feature and implement's later moments with no new mechanic — feature's fit is `Assumed` (F3); and designing more on the engine before any engine caller has run for real (F8) is worth doing now, the build waiting for a first run if the user wants. | lead's guess |
| Out of scope | The engine's own asking rules; brainstorm, specify's story stage, the desk's focus stage and implement's design stage as built; red/green/refactor and verification mechanics; the *Three-command convergence session* (boilerplate outside the stage); the ai-fileops dogfood runs; the build itself. | lead's guess |
| Size | standard, close to too-big — three separate parts, feature's a new design | lead's proposal |

### Frame hardening — turn 1 (2026-10-15): hardened

- User: "confirmed" — all six lines confirmed as written; size `standard` ruled.

## Frame card — hardened 2026-10-15

As proposed above, every line confirmed by the user ("confirmed"); each line's source now reads
"lead's guess, confirmed (turn 1)", the Destination's "user's words (topic) + lead's guess,
confirmed (turn 1)". Size `standard`, ruled by the user. Open frame lines: none. Deciding may
start.

## Facts — the remaining callers (gathered 2026-10-15, plugin 0.120.0)

- F9 — `mochiko-cli rules feature --section preamble`: moments "visit-open: Map health surfaced
  before the ask, every visit. · close: The verdict against the visit's done condition, with map
  integrity intact."; pins "kind: fail · 1 rules", "class: floor · 14 rules".
- F10 — `mochiko-cli rules feature --section feat.sec.tools`, `feat.delta-cards`: "One delta card
  per the tasks template's card shape … a bug's acceptance is its reproduction-failing test, an
  improvement carries 1–3 acceptance criteria … Each card — bug and improvement alike — takes the
  sound-loop review leg (feat.sound-loop-floor) before dispatch." `feat.dispatch-scope-split`:
  "the desk-confirmed card is implement's delta entry".
- F11 — `feat.sec.boundaries`, `feat.growth-door` (`class: floor`): "Work rows are delivery
  bookkeeping the desk may cut through the growth door: an extend verdict only, with acceptance
  criteria on the card." `feat.sec.reserved`, `feat.user-reserved`: "The user holds retire and
  merge rulings, route overrides, and every selection." `feat.sec.roles`, `feat.pm-seat`: "It
  recommends with reasons; it never rules."
- F12 — `feat.sec.ways-of-working`, `feat.artifact-home` (`class: floor`): "render `mochiko-cli
  home .mochiko/features` and, for a desk sitting, `mochiko-cli home
  .mochiko/features/desk/<date-slug>` before the first write … the desk's own output is
  `derivation.md`". `mochiko-cli home .mochiko/features/desk/<date-slug>`: "home: feature-desk —
  One specify-desk sitting … derivation.md · no template · 300 lines, whole file".
- F13 — `mochiko-cli rules implement --section preamble`: moments include "card-confirm: The
  user's ruling on the cycle slicing, before any card is built. · cycle-checkpoint: The per-cycle
  grading point where escalations and findings land as one batch. · final-validation: The
  whole-build verification pass — regression sweep, cold verification, gap finding. ·
  acceptance: The user's accept / amend / reject that closes the run."
- F14 — `impl.sec.reserved`, `impl.gate-card-confirm` (`class: floor`): "The card confirm is the
  user's: a blocking checkpoint where the user rules the slicing before build." `impl.sec.roles`,
  `impl.seat-card-author-independence`: "The card-authoring seat is a design-class seat, never the
  builder who will execute the cards; qa-engineer authors the **TEST:** cases within its slicing."
  `impl.card-review-before-confirm`: "The verification seat reviews the cards before the card
  confirm".
- F15 — `impl.sec.ways-of-working`, `impl.escalation-batching`: "Reserved-to-user questions
  accumulate and land as one batch at the cycle checkpoint; only a question the build cannot
  proceed without interrupts mid-cycle." `impl.finding-severity-routing`: "a Minor finding defaults
  to a BACKLOG.md booking … an Important-or-above finding blocks the cycle and joins the checkpoint
  batch."
- F16 — `impl.sec.tools`, `impl.gap-finding-blind-dispatch`: "The first message carries only
  spec.md, the sufficiency report and, where one exists, the signed architecture.md with the store
  diagram files it lists, Screens & Flows, and the baselines … never the code, tasks.md, the
  **TEST:** cases, or any report." `design-stage.md` is not in the list.
- F17 — `impl.sec.reserved`, `impl.beyond-spec-disposition`: "Each beyond-spec finding's
  disposition is the user's: fix now, book to BACKLOG.md, or accept as designed."
  `impl.gate-final-acceptance` (`class: floor`): "Final acceptance is the user's: accept / amend /
  reject closes the run."
- F18 — `impl.design-stage-unit`: "Per gap the user gives the direction in their own words and one
  "I'd know it works if…" line — the seed, `open` allowed and carried as an open question, never
  invented."
- F19 — `mochiko-cli rules specify`, `spec.intent-synthesis-governs`: "The confirmed intent
  synthesis governs the run and lands verbatim as the spec's Intent section."
  `spec.intent-probe-discipline`: "it closes in a one-screen synthesis the user confirms before any
  authoring."
- F20 — `mochiko-cli rules setup`, `setup.gate-synthesis-ratification` (`class: floor`): "Synthesis
  ratification is the user's."; `setup.fail.pre-ratification-authoring`: "A surface authored before
  the intent was ratified."; `setup.fail.unclosed-trace`: "An unclosed trace from ratified intent to
  authored surfaces."; moment "acceptance: The user's final acceptance of the surface set, flagged
  proposal by flagged proposal."
- Lead's reading (marked as reading): of implement's four later moments, only the card confirm is
  "a seat drafts, then you sign" (F14); the cycle checkpoint and final validation already put
  questions and findings to the user (F15, F17), and acceptance is a gate by nature. Setup (F20) and
  specify's intent (F19) already rule first and bind what follows — the contract half may add
  little there. Feature's desk drafts a card, then confirms it (F10), and its sitting home today is
  declared as a *specify*-desk sitting (F12).

## Decision map — first drawn 2026-10-15

- **Askable now:** the placement test — which of implement's later moments change, and how
  (unblocks every implement moment and frames feature's visits) · the gap-finder reading the
  user's rulings · setup's contract half · specify intent's contract half.
- **In fog:** feature — which visits run the stage (hangs on the placement test) · feature's unit
  (hangs on which visits) · feature's ruled set's home (hangs on the unit; costly to undo) ·
  feature's contract, next writer, blind list and independent leg (hang on the unit) · card
  slicing's shape and the seed's hand-off to the **TEST:** cases (hangs on the placement test) ·
  the cycle-checkpoint and final-validation shapes (hang on the placement test).
- **Out of scope:** as the hardened frame.
- **Defaults named:** build timing — the design is ruled now, the build waits for at least one
  logged ai-fileops engine run (lead's guess, for the batch).

### Q1 — Which of implement's later moments change, and by what test? (put 2026-10-15)

- Facts: F13–F17. Blind second list: none — a principle, cheap to undo before the build
  (brainstorm.blind-second-list cadence), disclosed.
- Options put: (a) three-way test — a moment where a seat drafts and you sign flips to "you rule
  first, the seat writes inside it" (the card confirm); a moment that asks you questions takes the
  engine's asking form only (the cycle-checkpoint batch, final validation's finding dispositions);
  accept/amend/reject stays a plain gate (acceptance); (b) draft-first only — the card confirm
  flips, the other three stay as built (cheaper shape); (c) every user moment runs the full stage,
  contract on (rejected by the lead: acceptance closes the run, nothing writes after it). Case
  against (a): it adds weight to every cycle's checkpoint in a long build, with no real run showing
  the checkpoint hurts (OQ1 of the earlier record). Lead's pick (a).
- User: "ask in simple language with example" — no ruling; Q1 re-put in plain words, one worked
  example per option (same options, same pick, same case against).
- User: "as recommended" — (a) ratified (streak: 1).

### D1-three-way-test — Implement's later moments split three ways: the drafted one flips, the asking ones take the engine's form, the gate stays — `Confident`

**Statement.** Of `/mochiko:implement`'s moments after the design checkpoint: the card confirm —
today a seat slices and the user signs (F14) — becomes a ruling before slicing: the user rules how
the work splits, and the card-authoring seat slices inside it, the ruled-set contract on. The
cycle-checkpoint batch (F15) and final validation's finding dispositions (F17) take the engine's
asking form — every question with at least two options, one the cheaper shape, the lead's pick and
the case against it; a real choice on its own turn, every other question in one batch screen — with
the contract off. Final acceptance (F17) stays a plain accept / amend / reject gate.

**Why.** The checkpoint and final validation already put questions to the user, so asking them the
engine's way removes the last mixed way of asking inside implement; the card confirm is the one
"a seat drafts, then you sign" moment left after the design stage.

**Rejected roads.** (b) the card confirm only — cheaper, but leaves two moments asking their own
way. (c) the full stage, contract on, at every moment — acceptance closes the run and nothing
writes after it, so a contract has nothing to hold.

**Accepted risk.** Every cycle's checkpoint grows a little in a long build, and no real run has
shown the checkpoint hurts (the earlier record's OQ1) — tested on the first ai-fileops implement
run.

**How it was decided.** Q1, its own turn, three options with the cheaper shape named and the case
against given; re-put in plain words with one example per option at the user's ask; user: "as
recommended" — (a) ratified (streak: 1). No blind second list: a principle, cheap to undo before
the build, disclosed.

**Amended at Q6 (the user's word, D6-promises-kept-table).** Acceptance stays a gate — accept /
amend / reject — but is no longer *plain*: it is shown as a promises-kept table. The card
confirm's flip is kept and scoped by D5-card-order-asked. *(As changed at review, V1: the card
confirm stays after the cards; the order is asked before slicing only on a real choice —
D5-card-order-asked as changed.)*

**Amended in the S2 reopen (the user's word, D10-two-switch-test).** The three buckets give way to
D10's two switches. The checkpoint rulings a writer acts on and final validation's dispositions
are asked the engine's way with the next writer held (*on*), not "contract off"; acceptance is a
gate with the writer *held* by the landing verifier. Rejected road (c)'s reason — "nothing writes
after it" — was false (S1) and no longer stands.

**Changed at review.** —

## Decision map — after D1-three-way-test (2026-10-15)

- **Ruled:** D1-three-way-test.
- **Askable now:** which feature visits run the stage (unblocked by D1) · card slicing's shape
  and the seed's hand-off to the **TEST:** cases (unblocked by D1) · the gap-finder reading the
  user's rulings · setup's contract half · specify intent's contract half.
- **In fog:** feature's unit (hangs on which visits) · feature's ruled set's home (hangs on the
  unit; costly to undo) · feature's contract, next writer, blind list and independent leg (hang on
  the unit).
- **Graduated to defaults by D1** (lead's guess, for the batch): the cycle-checkpoint shape — a
  real choice gets its own turn at the checkpoint, every other question one batch screen, the
  build-blocking interrupt unchanged; final validation's dispositions — each beyond-spec finding
  put with fix now / book / accept as designed as its options, the lead's pick and the case
  against, a costly one on its own turn.
- **Defaults named:** build timing (above).

### Q2 — Which feature visits run the stage? (put 2026-10-15)

- Facts: F9–F11. Lead's reading (marked as reading): the desk's visits are a map question
  (nothing ruled), a bug card, an improvement card, a growth row with its card, a new capability
  (routed to specify, which has the stage), tidying (merge / retire, the PM proposes, the user
  rules) and epic housekeeping. Under D1-three-way-test the card-cutting visits are "a seat
  drafts, then you confirm" (F10); tidying, routing and epic rulings are questions.
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put: (a) every card-cutting visit (bug, improvement, growth) runs the stage, contract
  on — the user says what should change and how they would know it works, and the card's seat
  writes inside it; tidying, routing and epic rulings take the asking form only; a map question
  runs nothing; (b) improvement and growth only — a bug card is written straight from its
  reproduction, as today (cheaper shape). Rejected by the lead: (c) feature stays as built — it
  leaves the one command off the engine, which the frame exists to end. Case against (a): a bug fix
  gets an extra step even when "fixed" is obvious. Lead's pick (a) — the engine opens with what the
  report already says and asks only what it leaves open, so a clear bug costs one confirm screen.

## Blind map landed (2026-10-15) — folded

- Arrival confirmed (fan-in, 1 of 1): `reports/angle-map.md` — 57 angles, 39 load-bearing, 11
  classes (A–K); seat reports the fence held (`record_contact: none`), its haiku locate spawn
  returned nothing and its locates ran on the seat tier, disclosed. The seat holds for the end
  read on the lead's explicit open.
- F21 — `plugins/mochiko/commands/feature.md` lines 57–61 (from the blind map, C3): "a crisp demand
  converges in a single exchange — state the goal and its done condition, get the nod, and go. The
  protocol never imposes brainstorm ceremony on a clear ask".
- F22 — `mochiko-cli rules architecture --section preamble` (from the blind map, C8): "visit_kind ·
  values: ruling|delta|other · resolution: entry-derived · note: fixed by the converged visit goal
  … other — a store query, a drift visit … or a fired trigger's routing."
- **Bearing on D1-three-way-test — goes back to the user** (brainstorm.blind-map-fold): G1 (at the
  card confirm the founder may have nothing real to fork — slicing is seat craft; what they can
  see is "what works after each cycle, in which order"), G3 (the card confirm sits right after the
  design checkpoint on the gap path, and is the first ruling after run-open on the zero-gap path),
  I3 (acceptance could take a landing-table form — each seed beside its evidence — rather than
  stay plain; "amend" undefined in engine terms), J1 (the founder's net stop count across the whole
  chain). Each comes back as its own askable question, stated as reopening that part of D1; D1
  stands until the user rules otherwise.

## Decision map — after the blind-map fold (2026-10-15)

- **Ruled:** D1-three-way-test.
- **Askable now:**
  1. Which feature visits run the stage — Q2, open (C1, C3, C8 from the blind map; F21, F22).
  2. The card confirm: what the user rules there, and the seed's hand-off to the **TEST:** cases —
     reopens D1's card-confirm part if the answer is "nothing to fork" (G1, G2, G3, J3 from the
     blind map).
  3. Acceptance's form — plain gate or a landing table over the chain; reopens D1's gate part (I3,
     A2 from the blind map).
  4. The founder's net stop count across the chain, with merges or retirements (J1 from the blind
     map).
  5. The gap-finder reading the user's rulings — in scope or a verification mechanic, and the
     shared-framing risk if the **TEST:** cases came from the same seeds (B5, I1 from the blind
     map).
  6. Setup's contract half — additive, duplicate or replacing its ratification, trace fail and
     flagged-proposal acceptance; synthesis or transcription (E1, E2, E3 from the blind map).
  7. Setup's catalog deck under the engine's asking — "card by card" against own-turn-or-batch and
     the streak rule (E4 from the blind map; new).
  8. Specify intent's contract half — who the next writer is; slate against an intent line (F1,
     F2, F3 from the blind map).
  9. Setup's and intent's silently inherited slots — the blind list, provenance marks, the status
     line; and setup's missing thinking-stage floor (E5, and E6 folded, from the blind map; new).
  10. The wider inventory — every other place a command rules something (run-open, selection,
      setup's mode and conflict rulings, the desk's drift dispositions …): place each, or narrow
      the destination by the user's word (B1 from the blind map; new).
  11. The build's cut line and order — what ships first, against the three pending ai-fileops runs
      and the neighbour items; feature deferred to a logged visit is a real road (A1, A4, K1, K3
      from the blind map; graduated from the build-timing default).
- **In fog:** feature's unit (C2 from the blind map) · feature's ruled set's home, readers and
  lifetime (D1, D2, J2, and D3 folded, from the blind map) · feature's contract, next writer,
  meaning-kept grade, blind list and floor/fail (C4, and C5, C7 folded, from the blind map) — all
  hang on Q2.
- **Defaults** (lead's guess, for the batch): the cycle-checkpoint shape (D1), refined — an
  ambiguity only the user can settle is asked as a labelled fact question, a business-level
  infeasible card as a fork, a finding as a disposition; a build-blocking question still
  interrupts at once (H1, H2, H3 from the blind map) · final validation's dispositions (D1), with
  "accept as designed" checked against the run's ruled set (I2 from the blind map) · altitude —
  every implement moment that joins puts its questions as plain consequences with a worked example,
  never a schema, as the design stage already does (J3 from the blind map) · no new engine slot —
  a decision that needs one goes to the user as a frame change (B4 from the blind map) · the
  supersessions owed are listed at the build (K2 from the blind map).
- **Dropped for the end read** (non-load-bearing, not folded): A3 (untested bets — the accept
  screen carries them), C6, E7, E8, E9, F4, G4, H4, H5, H6, I4, J4, K4, K5.

### Q2 — re-put with the blind map's facts (2026-10-15)

- Same options and pick; added: F21 (feature promises a clear ask converges in one exchange — under
  (a) a clear bug's stage is that one exchange: the ruled set stated from the report, one nod) and
  F22 (the architecture desk runs its stage only on visits that rule something; (a) is the same cut
  for feature: card visits run it, ruling visits take the asking form, queries run nothing).
- User: "as recommeed a" — (a) ratified (streak: 2). The ruling arrived while the re-put was being
  written, so the user ruled on the first wording; F21 and F22 support (a) and change neither
  option nor pick — told to the user with the next question.

### D2-card-visits-stage — Feature's card visits think first; its other rulings take the asking form; queries run nothing — `Confident`

**Statement.** On `/mochiko:feature`, every visit that cuts a card — a bug, an improvement, a
growth row (F10, F11) — runs the shared thinking stage with the ruled-set contract on: the user
says what should change and how they would know it works, and the card's writer writes the card
inside it. Grooming (merge, retire, status) and epic rulings are asked the engine's way with the
writer *on*; route overrides are asked the engine's way, writer *off*. *(As changed at review, V1 —
D10-two-switch-test's re-sort.)* A map question runs nothing; a new capability routes to
`/mochiko:specify`, which carries the stage. A clear ask still converges in one exchange (F21):
the stage opens with what the demand already says, asks only what it leaves open, and closes on
one confirm. The cut mirrors the architecture desk's `visit_kind` (F22).

**Why.** Feature is the one user command off the engine; under D1-three-way-test its card visits
are "a seat drafts, then you confirm" (F10) and its grooming and routing rulings are questions.
The engine's inputs-in-hand rule keeps a clear bug to one screen, so the stage costs a clear ask
almost nothing and gives an unclear one (should a "+" email be accepted?) to the user.

**Rejected roads.** (b) improvement and growth only, the bug card written straight from its
reproduction — cheaper, but a bug whose correct behaviour is unclear would be decided by the
seat. (c) feature as built — leaves the one command off the engine, which the frame exists to end.

**Accepted risk.** A quick bug fix gains a confirm screen even when "fixed" is obvious; and
feature's asking has never been seen in a live run (A4 from the blind map — the v0.61.0
live-feature-run watch still open), so its fit stays `Assumed` until the first logged visit.

**How it was decided.** Q2, its own turn, three options with the cheaper shape named and the case
against given; user: "as recommeed a" — (a) ratified (streak: 2); the blind map's F21 and F22
arrived mid-ruling and support it. No blind second list: cheap to undo before the build,
disclosed.

**Changed at review.** (S2 reopen, the user's word at the re-sort confirm, 2026-10-15.) Grooming
and epic rulings read "asked the engine's way, writer *on*" under D10-two-switch-test. (S16, QR5,
own turn; user: "as recommded", 2026-10-15.) The one-exchange close on a clear ask departed from
the desk's and specify's own-screen set confirm with no reason (S16). Ruled (a): a single-card
clear ask whose set the demand already states closes with the goal nod, as `feature.md` promises
(F21) — the goal and the set say the same thing; the set takes a screen of its own whenever there
is more than one card or any fork was put. Rejected: (b) a screen of its own always, like the desk
and specify. Added accepted risk: a goal nod stands for the set on the one-card path — the
passive-acceptance shape the desk's split guards against — watched on the first logged feature
visit.

## Decision map — after D2-card-visits-stage (2026-10-15)

- **Ruled:** D1-three-way-test · D2-card-visits-stage.
- **Askable now:** feature's unit (unblocked by D2) · items 2–11 of the map above, unchanged.
- **In fog:** feature's ruled set's home, readers and lifetime (hangs on the unit) · feature's
  contract, next writer, meaning-kept grade, blind list and floor/fail (hang on the unit).
- **Defaults:** as above.

### Q3 — What does the user rule on a feature card? (put 2026-10-15)

- Facts: F10, F11. `mochiko-cli rules architecture --section arch.sec.tools`, `arch.focus-unit`:
  "The user rules each focus item as three things: the part … the direction for it in their own
  words, and one sentence on how they would know it is right, or `open` … a target number comes
  only from the user". `mochiko-cli rules specify`, `spec.slate-unit`: "a one-breath story line —
  who, does what, why — with its priority and a seed test: one sentence on how they would know it
  works, or `test: open` … Acceptance scenarios, requirements and success criteria come after,
  inside the ruled lines". F18 (implement's direction and seed per gap).
- Lead's reading (marked as reading): the three built callers share one unit shape — the thing,
  the user's direction in their own words, one "I'd know it works if…" sentence or `open` — with
  the detail written after, inside it.
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put: (a) the family shape, reused — per card, the change in the user's words and one
  "I'd know it works if…" sentence or `open`; a bug's reproduction is a fact in hand, never asked;
  the card's seat writes the 1–3 acceptance criteria (or the bug's failing test) inside it, a
  target number only from the user (cheaper shape — reuse); (b) the acceptance criteria are the
  unit — the user rules each of the 1–3 criteria word for word, the seat writes the rest. Case
  against (a): the seat turns one sentence into the criteria implement builds against, room for
  drift the user sees only at the confirm. Lead's pick (a).
- User: "as recommeed" — (a) ratified (streak: 3 — flagged to the user; the next fork is put case
  against first with a weak lean, per analysis-iterative.ratified-streak).

### D3-card-unit-reused — On a feature card the user rules the change in their own words and one "I'd know it works if…" sentence — `Confident`

**Statement.** The unit the user rules on a feature card visit (D2-card-visits-stage) is the
family shape the built callers share: the change in the user's own words and one "I'd know it
works if…" sentence, or `open`, carried as an open question and never filled in. A bug's
reproduction is a fact in hand, never asked again. The card's seat writes the card's 1–3
acceptance criteria — or the bug's reproduction-failing test — inside it; a target number comes
only from the user.

**Why.** It is the unit specify's slate, the architecture desk's focus set and implement's design
stage already use (F18; `arch.focus-unit`; `spec.slate-unit`), so the founder rules the same kind
of thing at every desk, and stays at "what I want" instead of test wording.

**Rejected roads.** (b) the acceptance criteria as the unit, ruled word for word — precise, but
puts test wording on the founder and departs from the family shape.

**Accepted risk.** The seat turns one sentence into the criteria implement builds against — room
for drift the user sees only at the confirm; who grades the meaning kept is the open
independent-leg question.

**How it was decided.** Q3, its own turn, two options with the cheaper shape (reuse) named and the
case against given; user: "as recommeed" — (a) ratified (streak: 3, flagged). No blind second
list: cheap to undo before the build, disclosed.

**Changed at review.** —

## Decision map — after D3-card-unit-reused (2026-10-15)

- **Ruled:** D1-three-way-test · D2-card-visits-stage · D3-card-unit-reused.
- **Askable now:** where feature's ruled set lives, its readers and lifetime (unblocked by D3;
  costly to undo — blind second list dispatched, the fork waits for it) · items 2–11 as before.
- **In fog:** feature's contract, next writer, meaning-kept grade, blind list and floor/fail
  (hang on the home).

### Facts for the home fork (gathered 2026-10-15)

- F23 — `mochiko-cli home .mochiko/features/<FEAT-ID>` (plugin 0.120.0): "home: feature — One
  capability's work home … tasks.md · template `tasks` … design-stage.md · template
  `design-stage`" among ten declared files.
- F24 — `mochiko-cli template feature-entry`, `## Work rows`: "`pending` — {{increment}} ·
  acceptance: {{criteria}} · cut by {{spec-slug or lane-run}}"; budget "`## Work rows` — 20 lines
  (optional)".
- F25 — `.mochiko/schema-views/commands/feature.yaml` is the only schema view naming
  `features/desk` (grep, 2026-10-15); the feature-desk home's title reads "One specify-desk
  sitting" (F12) while `feat.artifact-home` makes `derivation.md` "the desk's own output".
- Blind second list dispatched 2026-10-15: one fresh persona-less `general-purpose` seat, spawned
  `model: opus` (explicit pin for a persona-less seat), given the frame as the opening and the
  question only — never the lead's options, the conversation or the ruled set; it returns its
  roads in its message and writes nothing (the lead saves them under `research/`).

### Q4 — The wider inventory: place every other ruling point, or narrow the destination? (put 2026-10-15)

- Facts (from the blind map, B1; each id confirmed present in `.mochiko/schema-views/commands/`,
  2026-10-15): implement's run-open rulings, scope escalation, design-first-write; specify's
  selection gate, post-stories frame confirm, acceptance, filter-disagreement escalation; setup's
  mode ruling, conflict rulings, coverage-survivor routing, final acceptance; feature's
  merge/retire, route overrides, selection; the architecture desk's reserved rulings.
- Blind second list: none — cheap to undo, disclosed.
- Options put, case against first: (a) apply D1-three-way-test to every one in one table — each
  row "you rule first" / "asked the engine's way" / "stays a gate" — confirmed in the defaults
  batch, a row the test does not settle getting its own turn; placing is not building, the cut line
  (item 11) decides what ships; (b) narrow the destination to the topic's list by the user's word
  (a frame change), the rest parked as one BACKLOG item for after the first engine runs (cheaper
  shape). Case against (a), put first: it reaches into specify, setup, the desk and brainstorm —
  the commands whose first engine runs are still pending — and most rows will read "stays a gate".
  Lead's weak lean (a): one screen now gives the whole picture, and building stays the cut line's.
- User: "as recommeed" — (a) ratified (streak: 4 — flagged again to the user; the lead moves the
  forks that are not real choices into the defaults batch, and keeps the case-against-first, weak-
  lean form for the next fork).

### D4-every-place-sorted — Every other ruling place is sorted by the same test, in one table, and sorting is not building — `Confident`

**Statement.** Beyond the topic's list, every place a mochiko user command rules something with
the user — implement's run-open rulings, scope escalation and design-first-write; specify's
selection gate, post-stories frame confirm, acceptance and filter-disagreement escalation; setup's
mode ruling, conflict rulings, coverage-survivor routing and final acceptance; feature's
merge/retire, route overrides and selection; the architecture desk's reserved rulings; and the
rulings in other sections the review found (S3) — is placed in one table on D10-two-switch-test's
two switches, how it is asked (the engine's way or a yes / no gate) and whether the next writer is
held (*on* · *held* · *off*), confirmed on one screen; a row the test does not settle takes its own
turn. *(As changed at review, V1 — first placed by D1-three-way-test's buckets.)* Placing a row commits no build: the cut line (map item 11) decides what ships.
The built stages stay out of scope as the frame says.

**Why.** The destination says "every place"; one table gives the whole picture for one screen of
the user's time, and leaves the build's reach to the cut line.

**Rejected roads.** (b) narrow the destination to the topic's list, the rest parked for after the
first engine runs — cheaper now, but leaves "every place" unmet and the inconsistency unmapped.

**Accepted risk.** The table reaches into specify, setup, the desk and brainstorm — commands whose
first engine runs are pending — and only 6 of the 39 rows stay a gate, so the reach is real
*(V1; the count corrected at QR7)*; D9-one-build-now cut
nothing, so the table's changes ship in the one build with no cut line behind them, and the
pending ai-fileops runs test them after. *(As changed at review, S11.)*

**How it was decided.** Q4, its own turn, put case against first with a weak lean (after the
streak flag); user: "as recommeed" — (a) ratified (streak: 4, flagged again). No blind second
list: cheap to undo, disclosed.

**Changed at review.** (S3, the S2 reopen; the user's word at the re-sort confirm, 2026-10-15.)
The first table read only each command's `*.sec.reserved` render and missed rulings in other
sections (S3); the re-sort under D10-two-switch-test adds them and replaces the first table's
placements. (S11, repair batch RB4; user: "confirmed".) The accepted risk's "the cut line carries
the build risk" no longer read true after D9-one-build-now; updated.

## Decision map — after D4-every-place-sorted (2026-10-15)

- **Ruled:** D1-three-way-test · D2-card-visits-stage · D3-card-unit-reused · D4-every-place-sorted.
- **Askable now (own turns — real choices or costly):**
  1. Where feature's ruled set lives — waits for the blind second list.
  2. The card confirm: what the user rules there — may amend D1's card-confirm part (G1, G3 from
     the blind map).
  3. Acceptance's form — may amend D1's gate part (I3, A2 from the blind map).
  4. The gap-finder reading the user's rulings (B5, I1 from the blind map).
  5. Setup's catalog deck under the engine's asking (E4 from the blind map).
  6. The build's cut line and order (A1, A4, K1, K3 from the blind map).
- **In fog:** feature's contract, next writer, meaning-kept grade, blind list and floor/fail (hang
  on the home).
- **Moved to the defaults batch by the lead** (not real choices; each can be pulled to its own
  turn — lead's guess): setup's contract half **off**, with the reason — its floor ratification
  gate, its unclosed-trace fail, its fresh-seat validate step and its flagged-proposal acceptance
  already hold the writer, and turning it on would move a floor gate from batch-after to
  halt-before (E1, E2, E3 from the blind map) · specify intent's contract half **off**, with the
  reason — the intent lands verbatim and governs the run (F19), its next writer is the lead's own
  story stage, which already runs a contract, so a second one would be inert or double (F1, F2,
  F3 from the blind map) · setup's and intent's inherited slots ruled as inherited — the engine's
  defaults (blind list on a costly fork, provenance marks kept in the conversation, the status
  line) stand for both, and setup gains a `setup.thinking-stage` floor pointer like the four
  built callers (E5, E6 from the blind map) · the seed's hand-off — every "I'd know it works if…"
  seed on the slate, the design stage or a feature card becomes at least one **TEST:** case whose
  Assert keeps its meaning, checked at the card review; an `open` seed rides open and is listed at
  the card confirm, never blocking (G2 from the blind map) · the net stop count — reported from
  the D4 table at the accept screen, with any merge it suggests put then (J1 from the blind map) ·
  and the earlier defaults (checkpoint shape, dispositions, altitude, no new engine slot,
  supersessions listed at the build).

### Q5 — At the card confirm, what does the user rule? (put 2026-10-15)

- Facts: F14; `impl.card-contents` (stories, dependencies, acceptance criteria by ID, a **TEST:**
  gate, brownfield exposure); the tasks template: "Where the work opens a new end-to-end path …
  the first cycle is a walking skeleton" and "`[P]` marks parallel-eligible cards, derived from
  dependencies". G1 from the blind map: slicing is seat craft; the founder-visible part is what
  works after each step, and in which order.
- Lead's reading (marked as reading): by the card confirm the stories and seeds are ruled; the
  walking skeleton and the dependencies fix much of the order; what is left for the founder is
  the order among independent outcomes and the step size — often nothing.
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put, case against first: (a) keep D1's flip, scoped — before slicing, the lead shows a
  rough order of what works after each step and asks only where there is a real choice (which
  independent outcome first, few big steps or many small); with no real choice the stage closes on
  one confirm and the seat slices; (b) amend D1 — the card confirm stays a plain yes/no, but the
  cards are shown as "after step 1 you can…, after step 2 you can…", the user able to reorder
  (cheaper shape). Case against (a), first: a founder rarely has a slicing view before seeing one,
  so most runs get a confirm that reads like today's sign-off, with one more turn before slicing.
  Lead's weak lean (a): cards get written once, and when an order does matter (demo the export
  first) the founder says so before the work is cut.
- User: "as recommeed" — (a) ratified (streak: 5, flagged).

### D5-card-order-asked — Before slicing, the user rules the order of what works, asked only where there is a real choice — `Confident`

**Statement.** *(As changed at review, S4/S8 and V1.)* Before the card-authoring seat slices, the lead shows a rough order of what works after each step — as a mock in the message —
and asks only where there is a real choice: which independent outcome comes first, or few big
steps against many small. Where the walking skeleton and the dependencies leave no choice, no
question is put. The ruled order and step size are the ruled set, the writer *on*, the
card-authoring seat the next writer, the order saved as one line in `tasks.md`'s header. The card
confirm then fires after the cards, as today (`impl.gate-card-confirm` unchanged), the
verification seat's card review before it also checking the cards follow the ruled order; the
cards are shown as "after step n you can…". *(Statement as changed at review, S4/S8 — below.)*

**Why.** The cards are written once, and an order that matters to the founder (a Friday demo of
the export) is said before the work is cut; with no real choice it costs one screen, as today.

**Rejected roads.** (b) the card confirm stays a plain yes/no over finished cards shown as "after
step 1 you can…" — cheaper, but a reorder rewrites the cards, and it amends D1.

**Accepted risk.** A founder rarely has a slicing view before seeing one (G1 from the blind map),
so most runs get no order question and the card confirm as today; a run with a real choice gets
one question more *(V1)* — tested on the first ai-fileops implement run (how often a real choice is
put). A delta or lane run builds from one desk card, so no order question arises there *(V2)*;
should a lane run ever slice several cycles, where its order line lives is raised to the user then
*(W1, QR7 — the visit-file branch dropped: implement does not write the desk's file)*.

**How it was decided.** Q5, its own turn, put case against first with a weak lean; user: "as
recommeed" — (a) ratified (streak: 5, flagged). No blind second list: cheap to undo, disclosed.

**Changed at review.** (S4 with S8, QR2, own turn; user: "as recommded", 2026-10-15.) The card
didn't say whether the confirm fires before slicing (on the order) or after (on the cards), and
default 22's "no moment added" contradicted one reading (S4); minting `design-stage.md` on the
zero-gap path contradicted `impl.design-stage`'s "nothing changes" (S8). Put as (a) two moments —
the order question only on a real choice, the card confirm after the cards as today — or (b) one
moment, the order confirm replacing the card confirm by supersession of its floor text. User: (a).
The statement carries it; the order's home is one line in `tasks.md`'s header, never
`design-stage.md`; default 4's home and default 22's count change with it (the defaults batch's
lines are annotated). (S9, repair batch RB2; user: "confirmed".) The order's contract gains its
`kind: fail`, as the family's other contract-on callers carry: cards cut against a ruled order
without the user's word fail the run.

- Default named (lead's guess, for the batch) — *superseded at review (S8, QR2)*: the card order's
  ruled set saved in the run's `design-stage.md`, minted on the zero-gap path. Now: one line in
  `tasks.md`'s header. The card confirm keeps its own screen after the design checkpoint (G3 from
  the blind map) — stands.

## Decision map — after D5-card-order-asked (2026-10-15)

- **Ruled:** D1–D5.
- **Askable now:** where feature's ruled set lives (waits for the blind list) · acceptance's form ·
  the gap-finder reading the user's rulings · setup's catalog deck under the engine's asking · the
  build's cut line and order.
- **In fog:** feature's contract, next writer, meaning-kept grade, blind list and floor/fail.
- **Defaults:** as after D4, plus the card order's home above.

### Q6 — Acceptance: a plain accept / amend / reject, or a promises-kept table? (put 2026-10-15)

- Facts: F17 (`impl.gate-final-acceptance`); `impl.reports-envelope` lists "the final-validation
  report, the built-vs-signed diff" among a run's reports; `impl.cold-verification`: "Results are
  part of the acceptance evidence."; the earlier record's D8-checkpoint-shows-landing (line 897):
  the design checkpoint shown as how each ruling landed. I3 and A2 from the blind map: passive
  acceptance is cured by the sign-off's form, a landing table over the chain; "amend" is undefined.
- Blind second list: none — cheap to undo, disclosed.
- Options put, case against first: (a) as D1 ruled — a plain accept / amend / reject over the
  final-validation report (cheaper shape); (b) still a gate, shown as a promises-kept table — each
  "I'd know it works if…" line from the slate, the design stage or the feature card beside the
  test that proves it and its result — with "amend" naming the rows the user disputes, each then
  put as fix now or book. Case against (b), first: one more table to assemble and read at the end
  of a long run, when the final-validation report already carries the evidence. Lead's weak lean
  (b): it is where the founder checks their own words were kept, instead of signing a report.

### Blind second list landed — feature's home (2026-10-15)

- Arrival confirmed (fan-in, 1 of 1): six viable roads (R1–R6), four rejected (R7–R10), the seat's
  pick R2; saved at `research/second-list-feature-home.md`. Seat disclosure: its haiku locate
  dispatch returned no report; locates on the seat tier.
- The lead's own roads, sketched in its working notes before the list landed but **not written to
  this record before it** (disclosed): implement's `design-stage.md` reused (= R3) · a desk visit
  file in the feature-desk home (= R2) · on the card in `tasks.md` (= R5) · the entry's work-row
  line (= R6). The list's differences from the lead's: R1 (`derivation.md` as is), R4 (a new
  per-capability `card-stage.md`), and the four rejected roads with reasons; and one fact the lead
  had not found — the lane home declares no `tasks.md`, and a zero-gap delta run never opens a
  design stage, so its read path needs its own clause.
- Q6 is open; the home fork is put after it.
- Q6, user: "as recommded" — (b) ratified (streak: 6, flagged). It amends D1-three-way-test's gate
  part (the amendment line is on D1's card).

### D6-promises-kept-table — Acceptance stays a gate, shown as a promises-kept table — `Confident`

**Statement.** `/mochiko:implement`'s final acceptance stays the user's accept / amend / reject
gate (`impl.gate-final-acceptance`), shown as a promises-kept table: each "I'd know it works if…"
line from the slate, the design stage or the feature card, in the user's words, beside the test
that proves it and its result; an `open` line reads "open — no target set", any measured value
shown, no pass / fail. "Amend" names the rows the user disputes, and each disputed row is then put
as fix now or book. The lead composes the table at acceptance and shows it in the conversation in
the user-facing register, each row's words quoted verbatim from their file; no new file. The
final-validation report and the built-vs-signed diff stay the evidence beneath the table.
*(As changed at review, S7.)*

**Why.** It is where the founder checks their own words were kept, instead of signing a report —
the passive-acceptance harm the problem names (A2, I3 from the blind map); and it gives "amend" a
meaning.

**Rejected roads.** (a) a plain accept / amend / reject over the final-validation report, as
D1-three-way-test first said — cheaper, but leaves the founder signing a report.

**Accepted risk.** One more table to assemble and read at the end of a long run.

**How it was decided.** Q6, its own turn, put case against first with a weak lean; user: "as
recommded" — (b) ratified (streak: 6, flagged). Amends D1-three-way-test's gate part by the user's
word. No blind second list: cheap to undo, disclosed.

**Changed at review.** (S7, repair batch RB1; user: "confirmed", 2026-10-15.) The table had no home,
author or register, and an `open` line could not "show the measured value" when an open seed takes
no test (S7). Now: composed by the lead at acceptance, shown in the conversation in the user-facing
register, the words verbatim, no new file — the report envelope's ultra register would mangle
them; an `open` line reads "open — no target set". (The S2 reopen: acceptance's writer reads
*held* by the landing verifier, D10-two-switch-test.)

## Decision map — after D6-promises-kept-table (2026-10-15)

- **Ruled:** D1 (amended by D6) · D2 · D3 · D4 · D5 · D6.
- **Askable now:** where feature's ruled set lives (blind list landed) · the gap-finder reading the
  user's rulings · setup's catalog deck under the engine's asking · the build's cut line and order.
- **In fog:** feature's contract, next writer, meaning-kept grade, blind list and floor/fail (hang
  on the home).
- **Defaults:** as after D5.

### Q7 — Where does feature's ruled set live? (put 2026-10-15)

- Facts: F10, F12, F23, F24, F25; the blind list's ground facts (the lane home declares no
  `tasks.md`; `arch.feature-delta-visit` closes in a `visit.md` keyed by the lower-cased FEAT id;
  `impl.design-inputs` reads only `.mochiko/architecture/desk/`; a zero-gap delta run never opens
  a design stage).
- Blind second list: landed (above); its differences from the lead's roads shown to the user with
  the question.
- Options put, case against first: (a) a typed desk visit file — a new `visit.md` in the
  feature-desk home (`.mochiko/features/desk/<date-slug>/`), the sibling of the architecture desk's,
  keyed to the FEAT id or `lane-<slug>`, holding the goal, each card's change and seed with
  provenance, a pointer to the card cut, and the verdict; the card points back; implement reads it
  on every path (the blind list's R2; the lead's own road too); (b) on the card itself — the
  user's words and seed written as card fields in `tasks.md` before the pen passes (cheaper shape;
  the blind list's R5, the lead's own road too) — it fails a growth row, whose `tasks.md` implement
  rewrites in selection scope, and a lane card, whose home has no `tasks.md`. Rejected: R1
  `derivation.md` as is (no shape to grade against; "derivation" is specify's word), R3
  pre-seeding implement's `design-stage.md` (mixes desk intake with the gap agenda; overwritten by
  the next run), R4 a per-capability `card-stage.md` (leaves residue under a FEAT for an
  undispatched card), R6 the entry's work-row line (puts unruled thought on the live map), R7–R10
  (the blind list's reasons). Case against (a), first: the heaviest road — a migration, a
  template, edits to two commands, a fallback key for a two-capability visit, and one more rule so
  a zero-gap delta run reads it — when (b) does bugs and improvements far cheaper. Lead's weak lean
  (a): one home for every card kind, the same shape as the architecture desk's, and something the
  card's review can grade the meaning against.
- User: "as recommded" — (a) ratified (streak: 7, flagged).

### D7-desk-visit-file — Feature's ruled set lives in a typed visit file at the feature desk — `Confident`

**Statement.** A feature card visit (D2-card-visits-stage) keeps its ruled set in one typed visit
file, `visit.md`, in the feature-desk home `.mochiko/features/desk/<date-slug>/` — the sibling of
the architecture desk's visit file, keyed in its folder slug to the lower-cased FEAT id or
`lane-<slug>`. It holds the visit goal and done condition; per card, the user's change in their
own words and the "I'd know it works if…" line or `open` (D3-card-unit-reused) with provenance;
the card itself, in its own part below the set; and the verdict. The capability entry carries the
marked delta, pointing at the file. The lead holds the file's pen and transcribes the set, then
passes the pen to the card's writer for the card part — desk-authored by default
(`patterns-sound-loop.default-seat-wiring`), a producing seat where the desk staffs one
(`feat.author-grader`), the `devils-advocate` review leg grading the meaning kept *(V3)*; the
dispatched `/mochiko:implement` run reads
the card and its set there on every path, the zero-gap delta path included. *(Statement as changed
at review, S6 — below.)*

**Why.** One home for every card kind — bug, improvement, growth row, lane card — in the shape the
architecture desk already uses, keeping unruled thought off the live map, and giving the card's
review leg something to grade the meaning against.

**Rejected roads.** (b) the set as card fields in `tasks.md` (the blind list's R5) — cheaper, but a
growth row's `tasks.md` is rewritten by implement in selection scope and a lane home has no
`tasks.md`; R1 `derivation.md` as is (no shape; specify's word); R3 pre-seeding implement's
`design-stage.md` (mixes desk intake with the gap agenda; overwritten); R4 a per-capability
`card-stage.md` (residue under a FEAT for an undispatched card); R6 the entry's work-row line
(unruled thought on the live map); R7–R10 as the blind list gives them.

**Accepted risk.** The heaviest road: a migration declaring the file and its template, edits to
`/mochiko:feature` and `/mochiko:implement`, a key rule for a two-capability visit, and a read rule
for the zero-gap delta path.

**How it was decided.** Q7, its own turn, put case against first with a weak lean after the blind
second list landed and its differences were shown; user: "as recommded" — (a) ratified (streak: 7,
flagged). Provenance: `blind list` and the lead's own road both (R2).

**Changed at review.** (S6 with S13, QR4, own turn; user: "as recommded", 2026-10-15.) Q7 went to
the user without the fact it hung on — the card's own home is unsettled (`feat.stable-ground-triage`
"card on the entry"; no card section in the feature-entry template; no `tasks.md` in a lane home)
— and the cheaper hybrid was never offered (S6). Re-put with the fact, the hybrid and "leave it to
the build", the user's own view asked for first (S13's engaging check). User: (a) — D7 stands, and
the card lives in the visit file beside its set; the entry's marked delta points at it; implement's
delta entry reads it there. Rejected: (b) the hybrid — card fields for a single bug or improvement
card, the visit file for growth rows and multi-card visits; (c) the card's home left to the build.
Added accepted risk: the file is written by two seats in turn (the lead, then the card seat — an
explicit pen hand-off), implement's entry rule changes to read the desk folder, and "card on the
entry" reads as "the entry points at the card". OQ3 closes into this line.

- Defaults graduated by D7 (lead's guess, for the batch; the in-fog feature items, now settled
  without a real choice): feature's next writer is the card's producing seat as today
  (`feat.author-grader`), the user's words quoted in its brief · feature's independent leg is the
  card's existing sound-loop review leg, which gains a meaning-kept grade against the visit file
  (C4 from the blind map) · feature gains a `feat.thinking-stage` floor pointer and a
  `kind: fail` for a card written with no ruled set behind it, or changing a ruling without the
  user's word, mirroring the desk's and implement's (C5 from the blind map) · a `visit_kind`
  condition for feature — card · ruling · other — mirroring the desk's (F22) · feature's blind list
  once per visit that cuts more than one card or a growth row, skipped on a single bug or
  improvement card, mirroring the desk's single-row skip (C7 from the blind map) · a visit that
  cuts cards on two capabilities writes one visit file per capability · `impl.design-inputs` and
  `impl.design-stage-inputs` widened to the feature-desk visit file, and the sufficiency check's
  per-card form reads it on the zero-gap delta path · the feature-desk home's title corrected from
  "One specify-desk sitting" · `feature.md`'s "never imposes brainstorm ceremony on a clear ask"
  kept, reworded to point at the stage's one-confirm close (K2 from the blind map).

## Decision map — after D7-desk-visit-file (2026-10-15)

- **Ruled:** D1 (amended by D6) · D2–D7.
- **Askable now:** the gap-finder reading the user's rulings · setup's catalog deck under the
  engine's asking · the build's cut line and order.
- **In fog:** none — feature's remaining items graduated to defaults by D7.
- **Defaults:** as after D5, plus D7's.

### Q8 — Does the final-validation gap-finder read the user's ruled lines? (put 2026-10-15)

- Facts: F16 (`impl.gap-finding-blind-dispatch` — the first message's list, `design-stage.md` not
  in it; "never … the **TEST:** cases"); `impl.seat-gap-finder-blind`: "never a seat that built
  these cycles or saw this feature's design-time test cases"; `spec.slate-unit`: "the acceptance
  screen shows each seed beside the Independent Test expanded from it" — the spec the finder
  already reads carries the slate's seeds. Scope: the topic names "final validation's gap-finder
  reading the ruled set" (user's words), so the fence question is in scope by the topic; the
  verification mechanics beyond it stay out (B5 from the blind map).
- Lead's reading (marked as reading): the finder already shares its promise source with the test
  author — `spec.md` — and stays blind to the tests; the design stage's and the feature visit's
  directions and seeds are promises of the same kind that today never reach it.
- Blind second list: none — cheap to undo, disclosed.
- Options put, case against first: (a) the finder's first message adds the user's own lines only —
  each direction and "I'd know it works if…" line from the run's `design-stage.md` and any feature
  visit file — never the ruled pieces (design detail, unbuilt) and never the **TEST:** cases; it
  derives its own expectations from them and probes around them; (b) the fence stays as built
  (cheaper shape — do nothing): D6-promises-kept-table already checks each seed against its test
  at acceptance. Case against (a), first: every extra input to a blind finder narrows what it
  imagines — it may hunt around the promises and miss what nobody promised (I1 from the blind
  map). Lead's weak lean (a): the promises are the finder's job, and it sees the user's words, not
  the test author's reading of them.
- User: "as recommded" — (a) ratified (streak: 8, flagged).

### D8-finder-reads-promises — The gap-finder's first message adds the user's own lines, never the design detail or the tests — `Confident`

**Statement.** Final validation's blind gap-finder (`impl.gap-finding-blind-dispatch`) gains one
input in its first message: the user's own lines from the run's ruled sets — each direction and
"I'd know it works if…" line in the run's `design-stage.md` and in any feature-desk visit file
keyed to the run (D7-desk-visit-file) — never the ruled pieces (design detail, unbuilt), never the
**TEST:** cases, the code or any report. It derives its own expectations from them, as it does from
`spec.md`, and probes around them. The rest of the fence and the two-message dispatch are
unchanged.

**Why.** The finder already shares its promise source with the test author (`spec.md`, which
carries the slate's seeds) and stays blind to the tests; the design stage's promises are the same
kind and today never reach it, nor do the feature visit's on the runs that carry a gap-finding
pass — selection-scope and epic runs; delta and lane runs skip it (`impl.gap-finding-scope`).
*(As changed at review, S10.)*

**Rejected roads.** (b) the fence as built — cheaper; D6-promises-kept-table checks each seed
against its own test, but nothing probes around a design-stage or feature-card promise.

**Accepted risk.** Every extra input to a blind finder narrows what it imagines; it may hunt around
the promises and miss what nobody promised (I1 from the blind map) — watched on the first
gap-finding run after the build.

**How it was decided.** Q8, its own turn, put case against first with a weak lean; user: "as
recommded" — (a) ratified (streak: 8, flagged). In scope by the topic's own words ("final
validation's gap-finder reading the ruled set"). No blind second list: cheap to undo, disclosed.

**Changed at review.** (S10, repair batch RB3; user: "confirmed", 2026-10-15.) The Why overstated
the feature file's reach: it reaches the finder only on selection-scope and epic runs, since delta
and lane runs skip gap-finding; narrowed.

### Setup's catalog deck — already ruled, not re-asked (2026-10-15)

- F26 — `.mochiko/brainstorms/implement-brainstorm-led/research/obligation-map.md` line 214
  (G44): "agenda items not worth a turn named as defaults and confirmed on one screen; a hanging
  default first. The deck stays card by card — **ruled** (Q2, "Keep card by card"): setup's own
  cadence, a slot fill already in `setup.interrogation-inline`'s text; no edit". Line 211 (G41):
  "deck cards already offer keep · tighten · drop · re-rank".
- The blind map's E4 (a live conflict between the deck's card-by-card and the engine's
  own-turn-or-batch) was ruled by the user at the 0.120.0 build: the deck's cadence is a slot fill
  in setup's own rule. Taken as a fact in hand (provenance `handed in`), never re-asked
  (analysis-iterative.inputs-in-hand); item 5 leaves the askable list.

## Decision map — after D8-finder-reads-promises (2026-10-15)

- **Ruled:** D1 (amended by D6) · D2–D8; setup's deck handed in as already ruled.
- **Askable now:** the build's cut line and order.
- **In fog:** none.
- **Defaults:** as after D7.

### Q9 — What ships, and when? (put 2026-10-15)

- Facts: `BACKLOG.md` lines 36, 78, 87 (F8) — the first ai-fileops implement, desk and specify
  runs, the engine's first real runs, all open; A4 from the blind map — `BACKLOG.md` lines 670–676,
  the live-feature-run watch open since v0.61.0; the 0.120.0 build moved four callers with no run
  (the earlier record's D3-four-move-now accepted risk); BACKLOG's *Engine-desk build — lines
  incomplete* and *tool and kit residuals* items and the *Three-command convergence session* touch
  the same files (K3 from the blind map). The build's pieces: feature on the engine (D2, D3, D7);
  implement's later moments (D1, D5, D6, D8); setup's small touches (defaults); and whatever rows of
  the D4 table change something.
- Blind second list: none — a sequencing choice, cheap to undo, disclosed.
- Options put, case against first: (a) wait — build nothing until at least one of the three
  ai-fileops engine runs is logged, then one combined build (cheaper shape — do nothing now); (b)
  one combined build now, as 0.120.0 did; (c) split — feature and setup's small touches now,
  implement's later moments and the D4 table's changes after the first ai-fileops implement run.
  Case against (a), first: it delays the rollout the user asked for, on runs that cost the user's
  own time and have no date. Lead's weak lean (a): one logged run tells whether the engine's asking
  is too heavy — the weight every decision here leans on — and the design is recorded either way.
- User: "b" — (b), against the lead's weak lean (streak: 0 — broken).

### D9-one-build-now — One combined build now, with no wait for the ai-fileops runs — `Contested`

**Statement.** Everything this session rules ships in one combined build, started without waiting
for the first ai-fileops engine runs: feature on the engine (D2-card-visits-stage,
D3-card-unit-reused, D7-desk-visit-file), implement's later moments (D1-three-way-test,
D5-card-order-asked, D6-promises-kept-table, D8-finder-reads-promises), setup's small touches, and
the rows of the D4-every-place-sorted table that change something — one bump, as 0.120.0 shipped.
The three pending ai-fileops runs stay open and test the build's result.

**Why.** The user's call: the rollout they asked for, in one build.

**Rejected roads.** (a) wait for at least one logged engine run, then build — the lead's weak
lean; rejected by the user. (c) split — feature and setup now, implement's later moments after the
first implement run; rejected by the user.

**Accepted risk.** The build stacks a second unproven layer on an engine that has not run once (A1
from the blind map): a problem in a first run cannot be told apart between the 0.120.0 move, this
build and the design; and the pending ai-fileops runs will test the post-build state, not 0.120.0
(K3 from the blind map).

**How it was decided.** Q9, its own turn, three options with the cheaper shape (wait) named and
the case against given first, the lead's lean weak; user: "b" — against the lead's lean, a
deliberate choice (`Contested`). No blind second list: a sequencing choice, disclosed.

**Changed at review.** —

## The D4 table — every other ruling place, sorted (drawn 2026-10-15, for the defaults batch)

Source: each command's `*.sec.reserved` render (plugin 0.120.0), read by the lead in full
(a completeness-driving enumeration, seat tier). Test: D1-three-way-test — a seat drafts and you
sign → you rule first · you are asked a question → asked the engine's way (contract off) · you
accept or reject a finished outcome → stays a gate.

| # | Command · place (rule) | Placement | Why |
|---|---|---|---|
| T1 | implement · run-open rulings — store trips, in-flight conflicts, uncleared clauses (`impl.user-runopen-rulings`) | asked the engine's way | each is a question with options (rule now or defer) |
| T2 | implement · desk offer — visit the desk or go on (`impl.desk-offer`) | asked the engine's way | a two-option question; "go on here" the cheaper one |
| T3 | implement · adopt-first and IP-XXX calls (`impl.adopt-first-user-call`) | asked the engine's way | a real fork; rides the checkpoint (D1) |
| T4 | implement · scope escalation (`impl.scope-escalation-fail`) | asked the engine's way | accept the bigger work or FAIL — a question |
| T5 | implement · design-first-write at acceptance (`impl.design-first-write`) | stays a gate | signs a finished write; rides D6's screen |
| T6 | specify · framing when the argument is empty (`spec.reserved-to-user`) | asked the engine's way | an open probe, which the engine allows |
| T7 | specify · clarification answers only the user can settle (`spec.reserved-to-user`) | asked the engine's way | a labelled fact question |
| T8 | specify · filter disagreement (`spec.filter-disagreement-escalates`) | asked the engine's way | a fork between two producers' views |
| T9 | specify · post-stories frame adjust (`spec.confirm-frame-post-stories`) | asked the engine's way | the PM's adjustment put as keep / adjust |
| T10 | specify · selection — which rows build now (`spec.gate-selection`) | stays a gate | a pick from the finished selection card, which already carries the recommendation |
| T11 | specify · acceptance (`spec.gate-acceptance`) | stays a gate | already shows each seed beside its test (`spec.slate-unit`) |
| T12 | specify · clicking prototype screens (`spec.reserved-to-user`) | unchanged | a reaction, not a ruling |
| T13 | setup · mode when ambiguous (`setup.user-mode-ruling`) | asked the engine's way | a three-option fork |
| T14 | setup · reality-vs-intent conflicts (`setup.user-conflict-rulings`) | asked the engine's way | each conflict a fork |
| T15 | setup · coverage-survivor routing (`setup.coverage-survivor-routing`) | asked the engine's way | explore now / rule inline / defer |
| T16 | setup · synthesis ratification (`setup.gate-synthesis-ratification`) | stays a gate | ratifies a finished synthesis |
| T17 | setup · final acceptance, flagged proposal by proposal (`setup.gate-final-acceptance`) | stays a gate | as built; the contract stays off (defaults) |
| T18 | architecture · drift dispositions (`arch.user-reserved-rulings`) | asked the engine's way | each finding a question, outside the focus stage |
| T19 | architecture · shelf-scope override (`arch.user-reserved-rulings`) | asked the engine's way | a ruling with options |
| T20 | architecture · governance-ledger waiver for a floor drop (`arch.user-reserved-rulings`) | stays a gate | a yes / no on a named waiver |

Placed elsewhere, not rows: feature's merge / retire, route overrides, selection and epic rulings
(D2-card-visits-stage) · the desk's row stances, baseline confirmation and amendments (its focus
stage, as built) · setup's interrogation cards (on the engine already) · specify's intent and
slate (as built and the defaults) · implement's design stage, design checkpoint, agenda split and
seat additions (as built) · all of brainstorm (out of scope by the frame). Count: 20 rows — 13
asked the engine's way, 6 stay a gate, 1 unchanged; none "you rule first" (every drafted-then-
signed place is already placed by D2, D5 or a built stage).

## Defaults batch — put for one confirmation (2026-10-15)

Implement: (1) the checkpoint — a real choice its own turn, the rest one batch screen; an ambiguity
only the user can settle asked as a fact question; a build-blocking question still interrupts at
once · (2) final-validation findings — fix now / book / accept as designed, the lead's pick and the
case against, "accept as designed" checked against the run's ruled set · (3) altitude — every
implement question in plain consequences with a worked example, never a schema · (4) the card
order saved in the run's `design-stage.md` (minted on the zero-gap path) *[changed at review, S8:
one line in `tasks.md`'s header; RB11, V2: no order question on a delta or lane run (one desk
card); a lane run slicing several cycles raises its order line's home to the user (W1, QR7)]*; the card
confirm keeps its own screen · (5) every "I'd know it works if…" line becomes at least one **TEST:** case keeping
its meaning, checked at the card review; an `open` line rides open, listed at the card confirm.
Feature: (6) the card's producing seat writes, the user's words quoted in its brief *[RB12, V3: the
card's writer follows the desk's wiring — desk-authored by default, a producing seat where the
desk staffs one]* · (7) the card's
review leg gains a meaning-kept grade against the visit file *[RB2, S9: the grader is the review
leg's `devils-advocate` by `patterns-sound-loop`'s default wiring; no new skill]* · (8) a `feat.thinking-stage` floor and
a `kind: fail` for a card written with no ruled set, or changing a ruling · (9) a `visit_kind`
condition — card · ruling · other · (10) the blind list once per visit with more than one card or a
growth row, skipped on a single bug or improvement · (11) a two-capability visit writes one visit
file per capability · (12) implement reads the visit file on every path, the zero-gap delta path by
the sufficiency check's per-card form *[RB3, S10: at entry, in every scope — selection, delta, lane,
an epic's member — whatever the gap count; the card itself is read there too, D7's change]* · (13) the feature-desk home's "specify-desk" title corrected
· (14) `feature.md`'s "no ceremony on a clear ask" line kept, reworded to the one-confirm close.
Setup and intent: (15) setup's contract half off — its ratification gate, trace fail, validate seat
and flagged-proposal acceptance already hold the writer · (16) intent's contract half off — it
lands verbatim and governs, and the story stage after it already runs a contract · (17) both keep
the engine's defaults as inherited; setup gains a `setup.thinking-stage` floor pointer.
Everywhere: (18) the D4 table above *[re-sorted under D10 at review]*; its "asked the engine's way" rows built as one pointer line *[the named subset, S5 — D10's changed-at-review line]*
per command (a `common.*` block each extends), never rule by rule · (19) no new engine slot — a
decision that needs one returns to the user as a frame change · (20) the supersessions owed listed
at the build, each with its strip entry · (21) the engine-desk "lines incomplete" and "tool and kit
residuals" items ride this build where they touch the same files; the *Three-command convergence
session* stays separate, after it · (22) the net stop count, reported at the accept screen: no
moment added to implement (the card confirm moves earlier, acceptance changes form) *[changed at
review, S4: one question added to implement, only when a real order choice exists; the card
confirm stays after the cards]*; a feature card
visit gains at most one confirm screen, none on a clear ask whose goal nod and set close are one
exchange; no merge suggested.

- User: "confirmed" — all 22 defaults and the D4 table confirmed as put (2026-10-15).

## Stop-rule check (2026-10-15)

Askable: none. In fog: none. Open frame lines: none. Defaults batch: confirmed. Blind map: landed,
its 39 load-bearing angles folded (each marked "from the blind map" above), the non-load-bearing
angles not folded listed as dropped for the end read. Deciding is finished.
*[Corrected at review, S12 / RB5: 36 of the 39 load-bearing angles were folded; B2, B3 and B6 were
never folded — taken up at review as S2 and S14, the reopen that made D10-two-switch-test.]*

Disclosed for the review: eight rulings in a row (Q1–Q8) were "as recommended"; the streak was
flagged to the user at three and after, and from Q4 each fork was put case against first with a
weak lean; Q9 broke it, the user ruling against the lead's lean. *[Updated at review, S13 / RB6:
15 of the 16 forks — Q1–Q8, QR1, QR1a and QR2–QR6 — were ratified as recommended, Q9 the one
against; the user's own view was asked for first on the costliest ruling (QR4) and the lean
ratified. The marks stay `Confident`, each ruling the user's explicit word, the streak on each
card's how-it-was-decided line.]*

## Accept screen — put 2026-10-15

**Decisions.**
- D1-three-way-test — implement's later moments: the drafted one flips, the asking ones take the
  engine's form, acceptance stays a gate (amended by D6).
- D2-card-visits-stage — feature's card visits (bug, improvement, growth) think first, contract
  on; grooming, routing and epic rulings take the asking form; queries run nothing.
- D3-card-unit-reused — on a feature card the user rules the change in their own words and one
  "I'd know it works if…" line.
- D4-every-place-sorted — every other ruling place sorted by D1's test in one table; sorting is not
  building.
- D5-card-order-asked — before slicing, the order of what works is asked only where there is a
  real choice.
- D6-promises-kept-table — acceptance stays a gate, shown as each promise beside its proof.
- D7-desk-visit-file — feature's ruled set lives in a typed `visit.md` at the feature desk.
- D8-finder-reads-promises — the gap-finder adds the user's own lines, never design detail or tests.
- D9-one-build-now — one combined build now, no wait for the ai-fileops runs (`Contested`).

**Defaults batch.** The 22 items above, as confirmed.

**Out of scope.** As the hardened frame: the engine's own asking rules; brainstorm, specify's story
stage, the desk's focus stage and implement's design stage as built; red/green/refactor and
verification mechanics beyond the gap-finder's input (D8); the *Three-command convergence
session*; the ai-fileops runs; the build itself.

**Open questions.**
- OQ1 — The evidence base: no engine caller has run for real; whether the founder has felt "two
  ways of being asked" is unknown — the diagnosis is `Assumed` until a run shows it (frame problem
  line; A1 from the blind map).
- OQ2 — Feature's asking has never been seen in a live run (the v0.61.0 live-feature-run watch is
  open); feature's fit on the engine stays `Assumed` until the first logged visit (A4 from the
  blind map).
- OQ3 — Where the card itself lives is not settled by the current rules: `feat.stable-ground-triage`
  says "card on the entry", the feature-entry template has no card section, the feature home
  declares `tasks.md`, and a lane home declares none (the blind list's ground facts). D7's pointer
  line needs that home; carried to the build, which reads the current rules and raises any gap to
  the user before writing.

**Untested bets, and how each is tested.**
- A clear feature ask still closes in one exchange under D2 — the first logged feature visit:
  exchanges from the demand to the dispatched card.
- Asking the engine's way at run-open, the checkpoints and final validation does not slow a long
  build much — the first ai-fileops implement run after the build: turns per checkpoint and the
  user's time.
- D5's order question finds a real choice often enough to earn its place — across the first runs:
  how often a real order choice is put, against a one-screen confirm.
- The promises-kept table catches drift the report hides — whether the user disputes a row the
  final report had passed.
- The gap-finder still finds what nobody promised with the user's lines in hand (D8) — the first
  gap-finding run: beyond-spec findings that trace to no promise.
- The engine's slots, named for six at v0.120.0, fit feature with no new mechanic — at the build:
  no engine slot added (default 19), or the user is asked.
- No command's ruled behaviour weakens — the build's gate audit, against the supersession list
  (default 20).

- User: "confirmed" — the accept screen confirmed as put (2026-10-15). The record freezes for the
  cold read; the lead holds every write to it until the review's verdict.

## Cold review (2026-10-15)

- The blind-map seat, resumed with the frozen record's path on the lead's explicit open
  (content-pinned), returned `reports/review.md` (fan-in, 1 of 1): status **needs-revision**,
  default FAIL, not cleared — raised 19 (Critical 0 · Important 7 · Minor 12), survived 16
  (Critical 0 · Important 6 · Minor 10), 3 withdrawn; coverage survivors 3 (S2, S14, S15).
  Cross-examination not run (solo, no counterpart). The seat corrects its angle map's
  `map_timing: end` to a front map. The verdict is in; the lead's writes resume.

### Disposition plan (brainstorm.coverage-survivor-routing, brainstorm.non-coverage-survivors)

- **Coverage candidates — the user rules each path** (explore now · rule inline · defer): S2 with
  S14 (the sorting test on named axes, and the middle forms — B2, B3, B6 from the blind map); S15
  (carry-forward: desk to specify, implement to desk — J2).
- **Own turns** (each challenges a ruling of the user's or offers a real choice): S1 (the contract at
  the checkpoint and the dispositions; D1's false reason) — hangs on S2's path; S3 (the table's
  missing rows outside the reserved sections) — placed under S2's outcome; S4 with S8 (when the
  card confirm fires, and where the card order lives on the zero-gap path); S5 (what "asked the
  engine's way" delivers, within the frame); S6 with S13 (Q7 re-put with the card-home fact and the
  hybrid road — the engaging check on the costliest ruling); S16's first half (the one-screen close
  against the desk's own-screen confirm).
- **The lead's repair batch** (facts, counts, wording, gaps inside a ruling — one named batch to the
  user): S7 (the promises-kept table's home, author and register; an `open` line shown open) · S9
  (the card order's kind fail and meaning grade; the desk grade's skill named) · S10 (the visit
  file's read on zero-gap growth-row and lane runs; D8's Why narrowed) · S11 (D4's accepted risk
  updated to D9's outcome) · S12 (the fold count corrected to 36, B2, B3, B6 named) · S13's mark
  notes · S16's 54 bare cites joined.
- **Changes nothing:** none — every survivor is dispositioned above.

### QR1 — S2 with S14: which path? (put 2026-10-15)

- Options put: (a) explore now — re-enter the thinking stage on the sorting test; the re-sort also
  settles S1 and S3; a new decision in this record's `D…` namespace, one bounded verify round
  (brainstorm.reopen-born-verify); (b) the lead rules it inline, the user confirms in one reply;
  (c) defer — D1 stands, the overlap carried open to the build (cheaper shape). Case against (a): a
  few more turns and a verify round. Lead's pick (a).
- User: "as recommded" — (a) explore now (streak: 1).

### QR1a — On what tests is a ruling place sorted? (put 2026-10-15)

- Facts: S1, S2, S14 (`reports/review.md`); `impl.dm-landing-whole`: "Execute the acceptance
  landing whole at user acceptance."; `impl.gates-fold`: "The landing folds the gap findings ruled
  fix-now or backlog into .mochiko/features/FEAT-XXX/gates.md … qa-engineer authoring each";
  `feat.pm-seat` (the PM drafts merge and retire proposals); `analysis-iterative.contract-pen` and
  `.contract-inside` (the contract half, bound `when: contract=on`).
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put: (a) one ladder, three tests in order — someone writes after your answer → you rule
  first, contract on; else real options → asked the engine's way; else a finished result → a
  yes / no gate; (b) two switches per place — how it is asked (the engine's way, or a yes / no gate)
  and, apart from it, whether the next writer is held (contract on when someone writes from the
  answer, off when no one does) — four forms, the middle ones included (S14); (c) D1's three buckets
  kept, with one tie-break line — when a place fits two, "someone writes after" decides (cheaper
  shape). Case against (b): four forms to build and explain, two rulings per place, and the
  contract — with its meaning-kept grade — spreading to more places. Lead's pick (b): acceptance
  shows why — it is a check of a finished result (a gate) and the landing writes after it (the
  contract); (a) would turn acceptance into "rule first", and (c) leaves the checkpoint's and the
  dispositions' contract to a reading again.
- User: "as recommded" — (b) ratified (streak: 2).

### D10-two-switch-test — Every ruling place is sorted on two switches: how it is asked, and whether the next writer is held — `Confident`

**Statement.** Born from the S2 coverage reopen, it restates D1-three-way-test's test. Each place a
user command rules something is set on two independent switches. **How it is asked:** the
engine's way — at least two real options, one the cheaper shape, the lead's pick and the case
against, a real choice on its own turn, the rest batched — wherever there are real options; a
yes / no gate where the user checks a finished result. **Whether the next writer is held:** *on*
where a seat writes from the answer and no existing rule holds it — the ruling reaches the writer
as its exact text, and that place's existing grader checks the meaning kept; *held* where a seat
writes from the answer and the command's own rules already hold it (the engine's contract stays
off there); *off* where no one writes from the answer — three settings, six combinations with the
two ways of asking. The thinking stages (D2's card visits and the four built callers) keep their
full stage. Under this test D1's moments read: the card order — the engine's way (the named
subset, not the full stage), *on* (D5) · the checkpoint rulings a builder or a design seat acts on — the engine's
way, *on* · final validation's dispositions — the engine's way, *on* (the fix, the booking and the
`gates.md` fold) · acceptance — a gate, *held* (the landing writes only folds of what was
accepted, checked by the landing verification seat, `impl.landing-verifier-folds`).

**Why.** D1's buckets overlapped (S2): "a seat drafts, you sign" and "you accept a finished result"
were told apart only by the lead's reading, and D1's one reason for the contract being off — that
nothing writes after acceptance — was false (S1: `impl.dm-landing-whole`, `impl.gates-fold`). Two
switches sort each place without a reading, and give acceptance both its gate and its hold.

**Rejected roads.** (a) one ladder, "someone writes after" first — it would turn acceptance into
"rule first". (c) D1's buckets with a tie-break — leaves the checkpoint's and the dispositions'
contract to a reading again (cheaper shape).

**Accepted risk.** Six combinations to build and explain, two settings per place, and the
meaning-kept check reaching more places — tested at the build by counting the places set *on*.
*(As changed at review, V5.)*

**How it was decided.** QR1 (the user chose to explore S2 now), then QR1a, its own turn, three
options with the cheaper shape named and the case against given; user: "as recommded" — (b)
ratified (streak: 2). The option as put carried two writer settings (on · off); the third, *held*,
entered at the re-sort and was ratified by the user's "confirmed" there. Reopen-born: one bounded
verify round (brainstorm.reopen-born-verify). No blind second list: cheap to undo before the
build, disclosed.

**Changed at review.** — (born at review) (S5, QR3, own turn; user: "as recommded", 2026-10-15.)
"Asked the engine's way" was not a defined rule set (S5). Ruled (a): outside a thinking stage it
is a named subset by pointer — each command's shared line names the engine rules that hold for its
questions (`analysis-iterative.one-fork-per-turn`, `.options-rule`, `.question-form`,
`.mock-in-message`, `.own-turn-or-batch`, `.ratified-streak`, `.vague-answer`,
`.fact-before-fork`, `.user-rules`) and, where the writer is *on*, `.contract-pen` and
`.contract-inside`; the blind list, the status line, the stop rule and provenance marks stay
stage-only. No engine text changes (default 19 holds); default 18's pointer line is that subset.
Rejected: (b) the whole engine at every such question; (c) no new delivery, the writer switch only.
Accepted risk: a cited rule id can drift from the engine unseen — nothing checks it (BACKLOG *tool
and kit residuals*); the build adds the cited ids to its gate audit. (V5, repair batch 2 RB9; user:
"confirmed", 2026-10-15.) The statement, the risk's count and the QR1a trace now agree on three
writer settings, six combinations, *held* noted as ratified at the re-sort; the attempt exemption
re-set to E · off; D5's card order named as the subset, not a full stage. D10's one bounded verify
was spent, so this fix closed on the user's word.

- D1-three-way-test gains its amendment line (below the D6 one) on the user's ruling here.
- The re-sort under D10 — every place re-set on the two switches, the rows S3 found added — is
  the lead's application of the test, put for one confirmation as the reopen's defaults (next).

## The re-sort under D10 — put for one confirmation (2026-10-15)

Asked: **E** = the engine's way · **G** = a yes / no gate. Writer: **on** · **held** (the
command's own rules) · **off**. Rows marked *new* are the S3 additions; *moved* are the placements
that change from D1, D2 or the first D4 table.

- **Implement** — card order: E · on (D5; the card review grades the order's meaning) · checkpoint
  rulings — infeasible card, ambiguity, adopt-first and IP-XXX calls, constraint challenge *new*:
  E · on (*moved*, S1; the cycle's verification seat checks the rework kept the ruling) · scope
  escalation: E · off · finding kind disputed *new*: E · off · final-validation dispositions: E · on
  (*moved*, S1; the landing verifier checks the `gates.md` fold) · gap-rework bound exhausted *new*:
  E · off · epic member halt *new*: E · off · attempt exemption *new*: E · off *[re-set at review,
V5: a choice between two real options]* · deviation gate
  *new*: E · on (the delta amended from the ruling; the existing non-author delta grade checks it) ·
  governance conflict (`impl.constitution-supremacy`) *new*: E · held (the ledger's own amend path)
  · run-open rulings: E · held (written into the sufficiency report, binding the run) · disputed
  sufficiency clause *new*: E · off (it defaults to a gap the design stage then rules) · desk offer:
  E · off · absent-surface offers *new*: E · off · design-first-write: G · held (the landing
  verifier) · acceptance: G · held (D6's table).
- **Specify** — framing on an empty argument: E (an open probe) · off · clarification answers: E (a
  fact question) · held (the analyst's pen under the slate) · filter disagreement, and a rejected
  `in` row after authoring *new*: E · held (the slate's contract) · post-stories frame adjust: E ·
  on (*moved*: the PM's map derivation writes from it; the map write's grader checks it) ·
  selection: E · held (*moved* from "stays a gate": a pick among rows with the PM's recommendation;
  the run builds only what is selected) · acceptance: G · held (it executes the staged map batch as
  shown) · prototype clicking: unchanged (a reaction, not a ruling).
- **Setup** — mode: E · off · reality-vs-intent conflicts: E · held (the trace from ratified intent
  to surfaces) · coverage routing: E · off · a second FAIL at the gate loop *new*
  (`setup.gate-loop-bound`): E · off · ratification: G · held · final acceptance: G · held
  (defaults 15 and 16 read as *held*: the engine's contract stays off because setup's and intent's
  own rules already hold the writer).
- **Feature** — card visits: the full stage · on (D2) · grooming — merge, retire, status: E · on
  (*moved*, S2: the map write keeps the ruling, its non-author grade checks it) · epic mint,
  membership, close: E · on (the epic write's grade) · route overrides: E · off · selection: E ·
  held · an architecture trigger at the growth door *new*: E · off (routing; a mint goes to
  specify).
- **Architecture desk** — drift dispositions: E · on (the rows written from them; tech-lead's
  sameness grade) · shelf-scope override: E · on · a trigger ruled closed *new*
  (`arch.dm-route-triggers`): E · off · floor waiver: G · held (the ledger records it).
- Count: 39 rows (the checkpoint's five escalation kinds counted as one row, specify's two filter
  escalations as one) — *on* 10, *held* 13, *off* 15, unchanged 1.
- User: "confirmed" — the re-sort confirmed as put (2026-10-15). It replaces the first D4 table's
  placements (T1–T20) and completes D4-every-place-sorted's "every place" with the S3 rows; D2's
  grooming and epic rulings now read "asked the engine's way, writer *on*".

### QR2 — S4 with S8: when does the card confirm fire, and where does the order live? (put 2026-10-15)

- Facts: S4, S8 (`reports/review.md`); `impl.gate-card-confirm` (`class: floor`): "a blocking
  checkpoint where the user rules the slicing before build"; `impl.card-review-before-confirm`:
  "The verification seat reviews the cards before the card confirm"; the review's quote of
  `impl.design-stage`: "on the zero-gap path no stage runs and nothing changes"; the tasks
  template's header carries "two generated-from pointers" (rendered `mochiko-cli template tasks`).
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put: (a) two moments — the order question before slicing only where there is a real
  choice (no screen when there is none), then the card confirm after the cards as today, the
  verification seat's review before it, the cards shown as "after step n you can…" and checked
  against the ruled order; `impl.gate-card-confirm` unchanged; default 22 reads "one question added,
  only when a real order choice exists"; (b) one moment — the order confirm before slicing replaces
  the card confirm, the user never sees the cards, the card review grades them against the ruled
  order, and `impl.gate-card-confirm`'s floor text changes to "rules the order before slicing" (by
  recorded supersession). Both: the ruled order saved as one line in `tasks.md`'s header, written
  by the lead before the card seat opens the file — never `design-stage.md`, so the zero-gap path
  mints nothing (S8). Case against (a): two stops when a real choice exists, and a second look at
  cards whose order was already ruled. Lead's pick (a): the floor gate keeps its meaning, the user
  still sees the cards, and the extra question is rare (G1 from the blind map).
- User: "as recommded" — (a) (streak: 3 — flagged; the next fork is put case against first with a
  weak lean). Fixed on D5-card-order-asked's card (changed-at-review line) and defaults 4 and 22.

### QR3 — S5: what does "asked the engine's way" deliver outside a thinking stage? (put 2026-10-15)

- Facts: S5 (`reports/review.md`); `analysis-iterative.engine-precedence`: "These asking rules hold
  in every command whose thinking stage runs on this skill"; the engine's 21 rules across its six
  sections (its preamble, plugin 0.120.0); default 18 ("one pointer line per command (a `common.*`
  block each extends)"); default 19 (no new engine slot); the frame's out-of-scope "the engine's own
  asking rules"; BACKLOG's *tool and kit residuals*: "a command's `pointer:` to a skill name is
  unchecked by `migrate validate`".
- Lead's reading (marked as reading): outside a stage the engine does not bind today, so a command
  that opts some of its rules in by its own text edits no engine rule and loosens no mechanic.
- Blind second list: none — cheap to undo before the build, disclosed.
- Options put, case against first: (a) a named subset by pointer — each command's shared line names
  the engine rules that hold for its questions outside a stage (one fork per turn, the options
  rule, the question form, the mock in the message, own turn or batch, the ratified streak, the
  vague answer, fact before fork, the user rules) and, where the writer is *on*, the contract pen
  and contract-inside; the blind list, the status line, the stop rule and provenance marks stay
  stage-only; no engine text changes; (b) the whole engine at every such question — the blind list
  on a costly fork mid-build, a status line each turn, the stop rule — the weight accepted in
  writing; (c) no new delivery — those rows keep today's asking, only the writer switch is built
  (cheaper shape; amends D10's "asked the engine's way"). Case against (a), first: the lead loads
  the engine at many points in a run and keeps two modes straight, and a list of rule names can
  drift from the engine unseen, since nothing checks a cited rule id. Lead's weak lean (a).
- User: "as recommded" — (a) (streak: 4, flagged). Fixed on D10-two-switch-test's card
  (changed-at-review line) and default 18.

### QR4 — S6 with S13: Q7 re-put — where does the card itself live, and does D7 stand? (put 2026-10-15)

- Facts — the one Q7 lacked (S6; the blind list's first ground fact): the card's own home is
  unsettled. `feat.stable-ground-triage`: "a single owning capability `delivered` → the delta
  lane, card on the entry"; `review-sufficiency` (`.mochiko/schema-views/skills/review-sufficiency.yaml`
  lines 39–41): "the desk's delta card is itself the `[MODIFY]` instrument, carrying the marked
  delta on the entry and folding at landing"; the feature-entry template declares no card section
  ("An undeclared `##` heading is denied. `###` and deeper are yours."); the feature home declares
  `tasks.md` (F23); a lane home declares none (`mochiko-cli home .mochiko/product/lane-x`, per the
  blind list); `commands/implement.md` lines 59–61: "a desk-confirmed delta card from
  `/mochiko:feature` (**delta scope** — the card's acceptance criteria are the cycle's criteria)".
- The engaging check S13 asks for: the lead asks for the user's own view on this, the costliest
  ruling, before its lean.
- Blind second list: the Q7 list stands (`research/second-list-feature-home.md`); its R5 and the
  hybrid the review names are put.
- Options put, case against first: (a) D7 stands, and the card lives in the visit file beside its
  ruled set — the lead transcribes the set, then passes the pen to the card seat for the card
  part; the entry carries the marked delta pointing at it; implement's delta entry reads the card
  there — one home, lane cards covered; (b) the hybrid — a single bug or improvement card carries
  the user's words as card fields on the card (its home still to settle, a lane needing one), the
  visit file only for growth rows and multi-card visits (cheaper for the common case; amends D7);
  (c) D7 unchanged, the card's home carried open to the build (cheapest now). Case against (a),
  first: the visit file becomes a file two seats write in turn, implement's entry rule changes to
  read a desk folder, and the card no longer sits "on the entry" in the plain sense. Lead's weak
  lean (a).
- User: "as recommded" — (a) (streak: 5, flagged; the user's own view was asked for first, and the
  user ratified the lean). Fixed on D7-desk-visit-file's card (statement and changed-at-review
  line); OQ3 closes.

### QR5 — S16: does a clear feature ask close in one exchange, or on a screen of its own? (put 2026-10-15)

- Facts: S16 (`reports/review.md`); `arch.gate-focus-confirm`: "Confirming the ruled set is the
  user's, on a screen of its own — never the goal's or the rows'"; F21 (`feature.md` lines 57–61):
  "a crisp demand converges in a single exchange"; specify's slate confirm "on a screen of its own,
  never the intent screen" (`spec.gate-slate-confirm`); D2-card-visits-stage and default 22 (the
  goal nod and the set close one exchange on a clear ask).
- Blind second list: none — cheap to undo, disclosed.
- Options put, case against first: (a) one exchange on a clear ask, with a reason written — a
  single card whose set the demand already states closes with the goal nod, as feature promises
  (F21); the set gets its own screen whenever there is more than one card or any fork was put
  (cheaper shape); (b) its own screen always, like the desk and specify — the goal nod never stands
  for the set, `feature.md`'s single-exchange line superseded. Case against (a), first: the desk and
  specify split the two screens for a reason — a nod to the goal read as a ruling on the set is the
  passive acceptance this session is about — and (a) departs from both. Lead's weak lean (a): on a
  one-card clear ask the goal and the set say the same thing, so a second screen asks it twice.
- User: "as recommded" — (a) (streak: 6, flagged). Fixed on D2-card-visits-stage's card
  (changed-at-review line).

### QR6 — S15 (coverage, J2): which path for the carry-forward? (put 2026-10-15)

- Facts: S15 (`reports/review.md`): "Desk to specify: a routed demand's desk conversation is not an
  input specify takes (spec.stage-inputs-in-hand names a brainstorm record or story-form stories).
  Implement to desk: an outside need goes to "a feature-desk card" (impl.design-stage-agenda),
  where the stage asks it again." Default 12 covers desk to implement only.
- A coverage candidate (brainstorm.coverage-survivor-routing): the user rules the path.
- Options put: (a) explore now — re-enter the thinking stage on the two hand-offs; (b) rule inline —
  the lead states the two readers and the user confirms in one reply: a demand the desk routes to
  `/mochiko:specify` hands its visit file in (the goal and the user's words so far), read as inputs
  in hand, never re-asked; an outside need `/mochiko:implement` sends to the desk arrives with the
  run's note handed in to the desk visit, which asks only what the note leaves open; (c) defer —
  the double ask accepted for now, recorded open (cheaper shape). Case against (b): two more input
  rules to build, in specify and the desk. Lead's pick (b): each is one input line, and without them
  the founder is asked the same thing twice.
- User: "as recommded" — (b) ruled inline (streak: 7, flagged); the two readers were on the screen
  as the option's text, so the user's word confirms them. They land as defaults 23 and 24 (no new
  decision — S15's materiality: "would add defaults, not change a D ruling"): (23) a demand the
  feature desk routes to `/mochiko:specify` hands its visit file in — the goal and the user's words
  so far — read as inputs in hand, never re-asked *[RB13, V4: a routing visit writes no visit file
  (D7 is card visits only); it hands in the desk sitting's existing `derivation.md`, carrying the
  goal and the user's words so far]*; (24) an outside need `/mochiko:implement` sends
  to the feature desk arrives with the run's note handed in to the desk visit, which asks only what
  the note leaves open.

### Repair batch — the lead's repairs, put for one confirmation (2026-10-15)

- **RB1 (S7)** — D6-promises-kept-table: the table is composed by the lead at acceptance and shown
  in the conversation in the user-facing register — each row's words quoted verbatim from the
  slate, the run's `design-stage.md` or the feature visit file, beside the final-validation
  report's evidence; no new file (the report envelope's ultra register would mangle the user's
  words). An `open` line reads "open — no target set", any measured value shown, no pass / fail.
- **RB2 (S9)** — D5-card-order-asked gains a `kind: fail`: cards cut against a ruled order without
  the user's word (the card review's order check is already in D5's statement, from S4). Default 7
  names its grader: the desk card's review leg — `devils-advocate`, by `patterns-sound-loop`'s
  default wiring — grades the card against the visit file for sameness of meaning, as tech-lead
  does at the architecture desk; no new skill.
- **RB3 (S10)** — default 12 widened: implement reads a feature-desk visit file keyed to its FEAT or
  lane at entry in every scope — selection, delta, lane, an epic's member — whatever the gap count.
  D8-finder-reads-promises' Why narrowed: the feature visit file reaches the finder only on
  selection-scope and epic runs, since delta and lane runs skip gap-finding
  (`impl.gap-finding-scope`).
- **RB4 (S11)** — D4-every-place-sorted's accepted risk updated: D9-one-build-now cut nothing, so the
  table's changes ship in the one build with no cut line behind them; the pending ai-fileops runs
  test them after.
- **RB5 (S12)** — the stop-rule check corrected: 36 of the 39 load-bearing angles were folded in the
  session; B2, B3 and B6 were never folded — taken up at review as S2 and S14, the reopen that
  made D10-two-switch-test.
- **RB6 (S13)** — the marks stay `Confident` — each ruling was the user's explicit word — with the
  streak on every card's how-it-was-decided line, and the stop-rule disclosure updated: 15 of the
  16 forks were ratified as recommended (Q1–Q8, QR1–QR6 and QR1a), Q9 the one against; the user's
  own view was asked for first on the costliest ruling (QR4) and the lean ratified.
- **RB7 (S16's bare cites)** — declined, with the reason, the user's call: `mochiko-cli ids --check`
  is advisory (human-readable-ids D18-build-done-check), and the record's bare `D<n>` collide with
  the blind map's own class-D angle ids (its D1, D2, D3), so a mechanical join would turn angle
  cites into decision cites; the cards' headings, the accept screen and the index carry the joined
  forms.
- **RB8 (the accept screen)** — refreshed after the verify: D10-two-switch-test added, D1's, D2's,
  D5's and D7's changed lines in its gists, OQ3 closed by D7's change, the defaults count 24.
- User: "confirmed" — the repair batch confirmed as put (2026-10-15). RB1–RB6 applied to D4, D5,
  D6 and D8's cards, defaults 7 and 12, and the stop-rule check; RB7 declined as put; RB8 waits for
  the verify.

### Verify round (dispatched 2026-10-15)

- The review seat is resumed once (brainstorm.fix-one-card-verify-once) to read only the changed
  cards and their gists — D1, D2, D4, D5, D6, D7 and D8's changed-at-review or amendment lines,
  defaults 4, 7, 12, 18, 22, 23 and 24, the re-sort under D10, the stop-rule check — and to give
  D10-two-switch-test its one bounded verify (brainstorm.reopen-born-verify: internal consistency
  and record-fitness, no fresh cold read, no blind-map coverage hunt, no second reopen).
- Returned (fan-in, 1 of 1), appended to `reports/review.md` as `verify_round_1`: **needs-revision,
  Minor only** — 16 of 16 survivor folds CLEAN; D10-two-switch-test's bounded verify NOT CLEAN (V5,
  Minor); four fold-introduced findings, all Minor (V1 superseded statement residue in D1, D2, D4,
  D5 · V2 the order line has no file on a lane run · V3 the cited wiring makes desk cards
  desk-authored, against D7's pen hand-off and default 6 · V4 default 23 hands in a visit file a
  routing visit never writes). Tally 5 raised, 5 survived (C0 · I0 · M5).
- Seat disclosure, surfaced to the user: during its first write of `reports/review.md` the seat
  took two artifact-hook denies — a dry-run whose frontmatter failed to parse ("the frontmatter is
  missing the required field `report`") and a refused shell copy ("artifacts under declared homes
  are written with Write/Edit") — and did not surface the hook's notice then; the file was then
  written by Write after a passing dry-run.

### Repair batch 2 — put for one confirmation (2026-10-15)

- **RB9 (V5, D10)** — D10's statement, accepted risk and trace agree on three writer settings —
  *on*, *held*, *off* — six combinations; the QR1a trace notes that *held* entered at the re-sort
  and was ratified by the user's confirm there. The attempt exemption re-set to E · off (a choice
  between two real options). D5 removed from D10's "keep their full stage" list: the card order
  runs the named subset, E · on. D10's one bounded verify is spent, so this fix closes on the
  user's word.
- **RB10 (V1)** — D4's statement re-worded to D10's two switches and its accepted risk's "most
  rows will read 'stays a gate'" corrected (7 of 39); D2's statement's "the contract off" for
  grooming and epic rulings corrected to writer *on*; D5's opening "the card-confirm flip holds"
  and its risk's "a turn earlier" corrected to the two-moment reading; D1's Q6 amendment line's
  "the card confirm's flip is kept" corrected — each marked "(as changed at review)".
- **RB11 (V2)** — a delta or lane run builds from one desk card, so no order question arises and no
  order line is needed; should a lane run ever slice several cycles, its order line sits in the
  lane's visit file beside the card (D7's change). Default 4 annotated.
- **RB12 (V3)** — who writes the desk card follows the desk's existing wiring: desk-authored by
  default (`patterns-sound-loop.default-seat-wiring`), a producing seat where the desk staffs one
  (`feat.author-grader`); either way it writes inside the set, and the `devils-advocate` review
  leg grades the meaning kept — with the lead as writer, that leg is the whole independent check.
  D7's "passes the pen to the card seat" reads "to the card's writer"; default 6 annotated. No
  wiring supersession.
- **RB13 (V4, the one small choice in this batch — pull it out to rule it on its own)** — a demand
  the desk routes to `/mochiko:specify` writes no visit file (D7 is card visits only); it hands in
  the desk sitting's existing `derivation.md` (`feat.artifact-home`: "the desk's own output"),
  carrying the goal and the user's words so far — read by specify as inputs in hand. Default 23
  re-worded. The other road: a routing visit writes a `visit.md` too.
- Then: verify round 2 reads the RB10–RB13 fixes only; RB9 closes on the user's word (D10's verify
  is spent); a second failed verify goes to the user (brainstorm.fix-one-card-verify-once).
- User: "confirmed" — repair batch 2 confirmed as put (2026-10-15), RB13 taken as put (the desk's
  `derivation.md`). Applied: RB9 to D10's statement, risk, trace and changed line and the re-sort's
  attempt-exemption row; RB10 to D1's Q6 line and D2's, D4's and D5's statements and risks; RB11 to
  D5's risk and default 4; RB12 to D7's statement and default 6; RB13 to default 23. RB9 closed on
  the user's word. Verify round 2 dispatched on RB10–RB13.

### Verify round 2 — returned 2026-10-15

- Returned (fan-in, 1 of 1), appended to `reports/review.md` as `verify_round_2`: **needs-revision,
  Minor only** — V1 NOT CLEAN (D4's accepted risk says "7 of the 39 rows stay a gate"; after RB9
  moved the attempt exemption to E · off the re-sort carries 6 — the fix took the pre-RB9 count) ·
  V2, V3, V4 CLEAN · new, fix-raised: W1 Minor (RB11's conditional branch has a lane run's order
  line written into the feature desk's visit file, against `impl.artifact-home`'s lane home and
  D7's desk pen). D10 not re-verified, as briefed. Tally 2 raised, 2 survived (C0 · I0 · M2).
- A second failed verify goes to the user (brainstorm.fix-one-card-verify-once). Put as QR7.

### QR7 — the second verify's two Minor items: how to close? (put 2026-10-15)

- The lead's repairs: (V1) D4's risk reads "6 of the 39 rows"; (W1) RB11's conditional branch
  dropped — should a lane run ever slice several cycles, where its order line lives is raised to
  the user then.
- Options put: (a) apply both and close the review on the user's word, seat-unverified (cheaper
  shape); (b) apply both and run a third verify. Case against (a): two lines close without a second
  reader. Lead's pick (a): one is a count, the other a deletion.
- User: "as recommded" — (a) (streak: 8 since QR1, flagged). Both applied (D4's risk; D5's risk and
  default 4); the review closes on the user's word, these two lines seat-unverified, disclosed.

## After review — the one screen of what changed (put 2026-10-15)

- **New:** D10-two-switch-test — every ruling place set on two switches: asked the engine's way or a
  yes / no gate, and the next writer *on* · *held* · *off* (born from the S2 coverage reopen).
- **Changed:** D1 — the checkpoint rulings and dispositions writer *on*, acceptance *held*; its
  false reason struck · D2 — grooming and epic rulings writer *on*; a clear one-card ask closes in
  one exchange, with its reason · D4 — completed with the rows outside the reserved sections and
  re-sorted (39 rows: *on* 10, *held* 13, *off* 15, unchanged 1; 6 gates) · D5 — two moments: the
  order question only on a real choice, the card confirm after the cards as today; the order in
  `tasks.md`'s header; a `kind: fail` · D6 — the table composed by the lead in the conversation,
  words verbatim, an `open` line shown open · D7 — the card lives in the visit file beside its set,
  written by the desk's card writer; the entry points at it; OQ3 closed · D8 — its reach narrowed
  to selection-scope and epic runs.
- **Defaults:** 24 now — 23 (a routed demand hands specify the desk's `derivation.md`) and 24 (an
  outside need reaches the desk with the run's note) added; 4, 6, 7, 12, 18 and 22 annotated.
- **Open questions:** OQ1 and OQ2 stand; OQ3 closed by D7's change.
- **Review trail:** cold read `needs-revision` (19 raised → 16 survivors: 0 Critical · 6 Important ·
  10 Minor, 3 coverage) → 16/16 dispositioned; verify round 1: 16/16 CLEAN, D10's bounded verify
  NOT CLEAN (V5) and four fold-introduced Minors → repair batch 2; verify round 2: V2–V4 CLEAN, V1's
  count and W1 → closed on the user's word, seat-unverified; RB7 (bare cites) declined by the
  user's word. Streak: across the session 16 of 17 forks were ratified as recommended — Q1–Q8 and
  all eight review forks (QR1, QR1a, QR2–QR7); Q9 the one against.

## Acceptance and close (2026-10-15)

- User: "accept" — the record accepted on the after-review screen.
- Close ritual (`.mochiko/memory/knowledge-management.md`): `DECISIONS.md` row (2026-10-15) ·
  `BACKLOG.md` *Engine follow-up session* → the trail, closed into this record; a new open item,
  *Engine follow-up build* (D9-one-build-now), in the *Architecture-desk brainstorm build* section ·
  `ROADMAP.md`'s Plan-surface builds Next row: the engine's "remaining" line names this session and
  its build (caps held) · the session index entry updated. The prior record
  (`implement-brainstorm-led`) is not superseded: this session fills the follow-up its
  D4-shaped-for-six named.
- Offered, never defaulted (brainstorm.next-step-offer): the build — D9-one-build-now ruled it to
  start without waiting for the ai-fileops runs. A synthesis is written only on request.
- Suggested commit (never run by the lead): the record, `reports/angle-map.md`, `reports/review.md`,
  `research/second-list-feature-home.md`, and the four operating-doc touches.



