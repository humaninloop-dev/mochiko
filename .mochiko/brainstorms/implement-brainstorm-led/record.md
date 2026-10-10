# Implement reimagined around brainstorm's thinking — decision record

Status: **accepted** 2026-10-11 (user: "accept") · cold read `needs-revision` (18 raised → 15 survivors, 0 Critical · 8 Important · 7 Minor, 2 coverage; 15/15 dispositioned: S1, S2, S4, S7, S8 user-ruled one by one, S6 and S9 ruled inline as coverage candidates, the rest one user-confirmed repair batch), verify round 1 NOT CLEAN (15/15 CLEAN, V1 user-ruled, V2–V6 lead-repaired), verify round 2 NOT CLEAN, Minor only (V2's title and W1, lead-repaired; closed seat-unverified by the user's word) · frozen for the cold read 2026-10-11 after the accept screen ("confirm") · opened 2026-10-11 · lead: Claude (session) · rulings: deepeshBodh

## Topic (user's words)

> based on the latest specify and architecture brainstorm, and specify implementation, i want to
> reimagine brainstorm led implement too. architecture brainstorm is yet to be implemented and will
> be implemented togther with implement brainstorm changes

## Orientation

- Session index read 2026-10-11 (`.mochiko/brainstorms/index.md`). No earlier session on
  implement's questioning with the user. The three sessions the topic names:
  - `specify-brainstorm-discovery` — accepted and built 2026-10-10 at v0.119.0 (a story-discovery
    stage before authoring, closing in a ruled slate that binds authoring).
  - `architecture-brainstorm-interview` — accepted 2026-10-11, **not built** (BACKLOG build items
    A and B). The user's topic: it is built together with this session's outcome.
  - `brainstorm-target-state` D19-brainstorm-only-reach — the target applies to brainstorm alone;
    each other command is a separate concern. This session is that concern for implement.
- F1 — `plugins/mochiko/commands/implement.md` lines 19–23: "Every gap fires an in-run
  **design phase** that authors exactly those gaps — and the user signs the result before any
  code is written. 2. **Cycle cards** are authored from the design and confirmed by the user, then
  built test-first, foundation cycles before feature cycles."
- F2 — `mochiko-cli rules implement --section impl.sec.reserved` (plugin 0.119.0),
  `impl.gate-design-checkpoint` (`class: floor`): "Where the check named gaps, no code is written
  before the user signs the design and the store delta — the store delta signed on a rendered
  diagram plus its named AX-XXX row changes and the pinned-base diff of the baselines";
  `impl.gate-card-confirm` (`class: floor`): "a blocking checkpoint where the user rules the
  slicing before build."
- F3 — same render, `impl.sec.ways-of-working`, `impl.escalation-batching`: "Reserved-to-user
  questions accumulate and land as one batch at the cycle checkpoint; only a question the build
  cannot proceed without interrupts mid-cycle."
- F4 — `mochiko-cli rules implement --section preamble` (plugin 0.119.0): 114 rules over six
  sections; pins "kind: fail · 15 rules", "class: floor · 37 rules"; moments run-open · entry ·
  design-checkpoint · card-confirm · cycle-checkpoint · final-validation · landing · acceptance.
- F5 — `.mochiko/brainstorms/architecture-brainstorm-interview/record.md` lines 1009–1011
  (`architecture-brainstorm-interview` D6-desk-stays-engine): "brainstorm's deciding, specify's story stage, the desk's focus stage,
  and implement's design phase (which runs it itself when no visit file exists) all run the same
  engine." Lines 1030–1031: "Implement's design phase keeps its own questioning until then."
- F6 — same record, lines 1239–1245 (`architecture-brainstorm-interview` D8-draw-before-code, changed at review): the ten stamped
  implement touches — "the design-input line · the offer · drawing into the store's diagram files
  before code · `impl.design-outputs-home` · `impl.deviation-gate` · the built-vs-signed diff · the
  landing flip · gap-finding's first message · `impl.gate-design-checkpoint`'s rendered-diagram
  wording · and a tenth … implement rebuilds the derived index at every design checkpoint and every
  landing, structural or not".
- F7 — `BACKLOG.md` lines 47–50: "**Shared thinking-stage design session** … a new brainstorm on
  the one stage brainstorm's deciding, specify's story stage, the desk's focus stage and
  implement's design phase all run … Trigger: build items A and B landed."
- F8 — `.mochiko/brainstorms/specify-brainstorm-discovery/record.md` line 445 (`specify-brainstorm-discovery` D1-story-discovery-stage):
  "`/mochiko:specify` gains a story-discovery stage that runs after the intent synthesis is
  confirmed and before any story is authored."
- Lead's reading (marked as reading): implement today asks the user to *sign* — the design
  checkpoint, the card confirm, the cycle-checkpoint batches, final acceptance — over artifacts
  seats have already authored. Specify and the desk both just moved from "author, then confirm" to
  "think with the user, then author inside the ruled set". Implement is the one run command left
  on the old shape, and the architecture record already promised its design phase a thinking
  stage (F5) while deferring the engine (F7).
- Artifact home rendered 2026-10-11 (`mochiko-cli home --plugin-root plugins/mochiko
  .mochiko/brainstorms/implement-brainstorm-led`): closed set `record.md`, `synthesis.md`,
  `build-log.md`, `wave<n>-<slug>.md`; sub-directories `inputs`, `research`, `referents`; reports
  under `reports/`.
- Knowledge-management file present (`.mochiko/memory/knowledge-management.md`): the close ritual
  applies.

## Frame card — proposed 2026-10-11, awaiting the user's first reply

| Line | Text | Source |
|---|---|---|
| Problem | The founder running `/mochiko:implement` meets seats that author first — the design phase drafts the gaps, the cycle cards are sliced — and is then asked to sign. Implement's moments with the user are sign-offs over finished drafts, the shape specify and the desk just left. Whether a real run has felt it: unknown, asked. | lead's guess (F1–F3) |
| Destination | Implement thinks with the user the way brainstorm does wherever the run decides something the user should own, before seats author — consistent with specify's story stage and the desk's focus stage — and this session's output is built in one build together with architecture build items A and B. Session ends with decisions that build can carry. | user's words (topic) + lead's guess |
| Must not break | Implement's floors (37) and fails (15); the sufficiency check as binding; TDD cycles and real-infrastructure verification; author≠grader, sound loop, plan approval; the architecture record D1–D9 as accepted; specify's story stage as built at v0.119.0. | lead's guess |
| Betting on | The "think, then author inside the ruled set" shape transfers to a build run without making implement much heavier, and the user wants to think during a build, not only sign. | lead's guess |
| Out of scope | The shared thinking-stage engine (its own BACKLOG session, F7); brainstorm's and specify's own rules; the cycle build itself (red/green/refactor, verification mechanics); setup, feature. | lead's guess |
| Size | standard — close to too-big (implement is the largest rule set, 114 rules) | lead's proposal |

### Frame hardening — turn 1 (2026-10-11)

- User: "the problem and destination make sense" — Problem and Destination confirmed as written.
- Must not break, Betting on, Out of scope and Size not addressed; the fact question (has a real
  implement run shown the problem) not answered. Re-put as the remaining lines; the out-of-scope
  line's shared-engine item put as a two-option choice, since it moves the size.

### Frame hardening — turn 2 (2026-10-11)

- User: "the must not break, if the changes is required in the floor and fail rules to help with
  brainstorm led implement , thats okay." — Must not break corrected: implement's floors and fail
  rules may change where brainstorm-led implement needs it (each change leaves by a recorded
  supersession at the build, per `.claude/rules/mochiko/primitive-edits.md`); the rest of the
  line stands.
- Out of scope (the shared-engine choice), Betting on and Size still unanswered; re-put compactly.

### Frame hardening — turn 3 (2026-10-11)

- User: "i would like to design the shared engine too." — option (b): the shared thinking stage
  moves into scope, pulled forward from `BACKLOG.md` *Shared thinking-stage design session*
  (F7; its trigger, "build items A and B landed", overridden by the user's word).
- Consequences the lead puts back for one confirm: Destination widens to the engine; Out of scope
  loses the engine and brainstorm's/specify's rules to the extent the engine re-homes them; Must
  not break gains "no command's ruled behaviour weakens when its copy moves into the engine";
  Betting on gains the four-command bet; Size proposed `too-big`.
- The fact question (has a real implement run shown the problem) left unanswered twice: carried as
  unknown; the diagnosis is `Assumed` until a run shows it.

### Frame hardening — turn 4 (2026-10-11)

- User: "should we focus just on shared engine at the moment?" — a destination question, put back
  as a two-option choice with the lead's pick.
- F9 — `.mochiko/brainstorms/architecture-brainstorm-interview/record.md` line 1574 (S14
  disposition): "the user's own confirm: implement's design phase runs the shared thinking stage".
  Lead's reading (marked as reading): implement's first seat on the engine — its design phase,
  where no visit file exists — is already ruled; an engine-only session still lands the core of
  "brainstorm-led implement".

### Frame hardening — turn 5 (2026-10-11): hardened

- User: "yes a" — option (a): the session designs the shared thinking stage; implement's design
  phase plugs in per F9; implement's other moments are handed to a named follow-up. The (a) frame
  as put, and the two added lines (the four-command bet; no command's ruled behaviour weakens),
  confirmed. Size `standard`.
- The session slug `implement-brainstorm-led` was derived before the topic narrowed; kept (slugs
  are not renamed mid-session), the index entry's title line says what the session is about.

## Frame card — hardened 2026-10-11

| Line | Text | Source |
|---|---|---|
| Problem | The founder running `/mochiko:implement` meets seats that author first — the design phase drafts the gaps, the cycle cards are sliced — and is then asked to sign; implement's moments with the user are sign-offs over finished drafts, the shape specify and the desk just left. Brainstorm, specify and the desk each carry (or will carry) their own copy of "think with the user, then author inside the ruled set", and implement's design phase was promised the same thinking (F5, F9) with no stage to run. Not yet shown in a real implement run (unknown — `Assumed`). | lead's guess, confirmed (turn 1) |
| Destination | Design the one shared thinking stage that brainstorm's deciding, specify's story stage, the desk's focus stage and implement's design phase all run, so the combined build ships it once — built together with architecture build items A and B. Implement's design phase plugs in per F9. Session ends with decisions that combined build can carry. | user's words (topic, turns 3, 5) + lead's guess, confirmed |
| Must not break | The sufficiency check as binding; TDD cycles and real-infrastructure verification; author≠grader, sound loop, plan approval; the architecture record D1–D9 as accepted; specify's story stage as built at v0.119.0; no command's ruled behaviour weakens when its rules move into the engine. Implement's floors and fail rules may change where brainstorm-led implement needs it, each by a recorded supersession at the build. | lead's guess, corrected (turn 2) and widened (turn 3), confirmed |
| Betting on | One engine serves four commands with different stakes — an open topic, a story set, architecture, a build — without flattening what each needs; and "think first, then author inside what the user ruled" works in a build run without making implement much heavier. | lead's guess, confirmed (turns 3, 5) |
| Out of scope | Implement's other moments — card slicing, cycle-checkpoint batches, final validation, acceptance — handed to a named follow-up session; setup's questioning; `/mochiko:feature`; the red/green/refactor and verification mechanics; the content of each command's output except where the engine changes how it is ruled. | lead's guess, confirmed (turn 5) |
| Size | standard | lead's proposal, ruled by the user ("yes a") |

Open frame lines: none. Deciding may start.

## Run notes

- Floor read-back — transport floor, 11: message-lane-trigger · topology-lane-trigger ·
  neither-lane-waivable · composition-steer · single-writer-per-surface · mesh-hold ·
  content-pinned-supersession · quiesce-before-cold-grade · no-ritual-sends · fan-in-confirmation ·
  version-floor. Message lane fires (the review seat is resumed later with the record); topology
  lane does not (the lead is the sole writer of this record and of the index entry; each seat
  writes only its own report file).
- Floor read-back — model tiering, 8: class-key-session-tier · seat-default-key ·
  override-is-the-pin · seat-deviation-bounds · persona-less-grader-pin · worker-return-is-a-claim ·
  brief-obligation · worker-seat-set-reserved.
- Session tier: lead on `claude-opus-5-5`.
- Seats (2026-10-11, at frame hardening):
  - Blind-map review seat `review-seat` — `mochiko:devils-advocate`, persona default tier
    (`opus`), no override; given the problem, destination and out-of-scope lines only, the session
    directory, the index and the lead's memory fenced; invokes `mochiko:review-brainstorm` for the
    blind angle map; writes only `reports/angle-map.md`. To be resumed with the record for the
    cold read (message two).
  - Blind second-list seat `second-list-engine` — persona-less `general-purpose`, spawned
    `model: opus` (disclosed: an interpretive seat, pinned to the strong alias; not a grader);
    given the frame and the engine-shape question only, never the lead's options or this record;
    writes only `research/second-list-engine.md`.
  - Every brief carries the model-tiering routing line.
- Fan-in expected: 2 returns (blind map · second list on the engine-shape fork).
- Blind second lists, disclosed (added at review, S14): Q1 had its own seat; Q2 and Q5 reused the
  engine-shape seat's list, said so in their trails; Q6, Q7 and Q8 — forks costly to undo — had no
  blind list and the trail did not say so: a miss. Q6's gap was closed at review by the S1 re-put,
  which showed the second list's unput road; Q7 ended in the user's own road; Q8 kept the floor
  whole.

## Facts for the engine

- F10 — `plugins/mochiko/skills/analysis-iterative/SKILL.md` lines 50–52: "## Two output shapes,
  one engine — The adaptive questioning the Overview describes is the single questioning engine. It
  carries two output shapes; the engine is identical and only the agenda and the concluding
  artifact differ." Line 42: "One question per turn — always". `mochiko-cli rules
  analysis-iterative` → "Error: no command or skill named 'analysis-iterative' in the log": it is
  a prose skill with no rule set. `plugins/mochiko/skills/mochiko/SKILL.md` line 124: "The
  questioning engine is `analysis-iterative` … it is not brainstorm-specific." Pointed at by
  brainstorm (`brainstorm.lead-inline-questioning`), specify's intent stage, and
  `review-specifications`; each of brainstorm and specify carries an own-rules-win clause over it.
- F11 — `.mochiko/decisions/2026-08-28-near-dup-convergence.md` lines 36–40: "R1 — Convergence
  licensed. A family spanning 3+ commands with near-identical wording … may converge to one
  `common.<slug>` block … D8's other limbs (per-command default, stub-carried binding, block =
  boilerplate never judgment) stand unchanged." Today's `common.*` blocks are single rule texts,
  one `extends:` stub per member rule (`grep extends: common. plugins/mochiko/migrations`: ten
  blocks, e.g. `common.transport-floor` ×6, `common.plan-approval-producers` ×3).
- F12 — specify's built story stage (`mochiko-cli rules specify`, plugin 0.119.0), rule by rule:
  `spec.lead-holds-story-stage` · `spec.story-forks` ("Put one fork per turn, its fact in hand
  first, with at least two real options — one the do-less shape … a one-row mock … a candidate
  gets its own turn only when it is a real choice or costly to undo, the rest named in one line
  … confirmed together on the slate screen. Each turn carries one status line … ruled · askable ·
  in fog.") · `spec.blind-journey-list` · `spec.slate-unit` · `spec.slate-home` ·
  `spec.slate-binds-authoring` · `spec.stage-inputs-in-hand` · `spec.handed-in-slate` ·
  `spec.slate-provenance` · `spec.story-stage-stop-rule` · `spec.gate-slate-confirm` ·
  `spec.slate-rulings-users` · `spec.analyst-brief-and-pen` · `spec.pm-vets-slate-once` ·
  `spec.own-rules-win` · `spec.fail.story-outside-slate` (`class: floor`).
- F13 — the desk's focus stage, ruled not built:
  `.mochiko/brainstorms/architecture-brainstorm-interview/record.md` line 1335 on, *Mechanisms
  carried*: "one fork per turn · at least two real options, one the do-less shape … · the fact a
  fork hangs on in hand before it is put · a rough in-message mock … · a status line (ruled ·
  askable · in fog) … · own-turn-or-batch … · the ratified-yes streak flag … · a vague answer
  re-asked as two concrete options · the stall clause … · the stop rule … · a provenance mark per
  item … Dropped: a frame card … · decision cards and a record file … · a blind angle map and a
  cold review (tech-lead's grade is the independent leg)."
- F14 — implement's design phase today (`mochiko-cli rules implement`, plugin 0.119.0):
  `impl.sufficiency-binding-verdict` ("gaps fire the design phase over exactly those gaps and
  nothing else") · `impl.design-gaps-only` ("Design seats author exactly the named gaps, nothing
  more, each on a plan a fresh peer passed and you approved") · `impl.design-seats-staffing`
  (technical-analyst, principal-architect, qa-engineer) · `impl.design-review-pair` · 
  `impl.design-outputs-home` · `impl.midrun-refire` ("the design phase re-fires scoped to the
  discovery — same grade, same checkpoint") · `impl.epic-design-always-fires` · the
  `impl.gate-design-checkpoint` floor (F2). No rule puts a design fork to the user before the
  seats author; the user's first ruling on the design is the checkpoint signature.
- Lead's reading (marked as reading) of F10–F14: the three stages share a fork core (one fork per
  turn, fact first, two options with a do-less one, a mock, own-turn-or-batch, a status line,
  stop and stall, provenance, a blind list, a handed-in skip, a ruled set binding the author, a
  fail for authoring outside it, own-rules-win). They differ in their opening (frame card ·
  intent synthesis · visit goal), their unit (decision card · story line + seed test · part +
  direction + "how I'd know"), the set's home (record · `spec.md` index · visit file), who
  authors after (nobody — the record is the deliverable · analyst · architect), the independent
  leg (cold review · PM vet and spec review · tech-lead grade) and the blind list's cadence. The
  "engine" analysis-iterative already claims to be (F10) holds none of the stage mechanics — each
  command restates them under its own-rules-win clause.

- Fan-in: blind map arrived 2026-10-11 (`reports/angle-map.md`: 45 angles, 33 load-bearing, ten
  classes A–J; fence held; one haiku Explore locate returned no report, the greps re-run on the
  seat tier — disclosed). Second list: pending.

## Decision map — first drawn 2026-10-11, blind map folded (shown with Q1)

Angles marked *(bm X)* come from the blind map; the rest are the lead's.

**Askable now**
- **Engine core and slots** — what the engine owns, what each command keeps; whether brainstorm is
  a user of the engine or its source (bm B1, B2, B3); the do-less road — no engine yet, local
  stages, converge after first runs — weighed by name, since it bears on the user's turn-3 ruling
  (bm A1). Put as Q1.

**In fog** (each with what it hangs on)
- **Engine home and delivery** — a widened `analysis-iterative`, a new skill delivered by
  `mochiko-cli`, `common.*` blocks, or a pointer into one command's rules; precedence flips from
  own-rules-win; where gates and fails stay; grammar or binary cost; change control; a testable
  "ships once" (bm C1–C4, A3, A4). Hangs on: engine core and slots.
- **Re-homing scope** — brainstorm's and specify's stages moved onto the engine in this build, or
  only the desk and implement built on it, the other two later; which version the pending
  ai-fileops specify run tests; the supersessions owed (bm J3, J4, C2, B5). Hangs on: core, home.
- **Implement — where the stage sits and when it fires** — after run-open or merged with it,
  before the design seats' plan dispatch; the zero-gap path; epic's joint design (bm D2, D3).
  Hangs on: core.
- **Implement — the agenda** — exactly the gap list, or widenable by the user, and how the gap
  list re-keys; which design surfaces the user thinks through and which are defaulted; the line
  against card slicing (bm D1, A2, D7). Hangs on: placement.
- **Implement — the ruled unit and the user's altitude** — per gap, per element, or per
  consequence; "direction + how I'd know" as the shared unit; technical forks put as consequences
  or worked scenarios, not schemas (bm E1, E2). Hangs on: core (whether the unit is an engine
  part or a slot).
- **Implement — handed-in sets** — a desk visit file that covers part of the gaps; the spec's
  slate and a brainstorm record arriving as facts, never re-asked (bm D4, I1). Hangs on: core.
- **Implement — the checkpoint and the gate count** — the design checkpoint after a ruled stage:
  a second signature, or a check that the drafts kept the set's meaning; the net count of user
  gates per run (bm D5, H1). Hangs on: placement and agenda.
- **Implement — mid-run refire** — does the stage re-run mid-build against the escalation
  batching rule (bm D6). Hangs on: placement.
- **Implement — seats** — who holds the conversation in a long resumable run; the blind list's
  inputs, count and fence; who grades "authored inside the set, meaning kept" (bm F1, F2, F3).
  Hangs on: core.
- **Implement — the set's home and its fail** — a new declared deliverable that survives
  stop-and-resume; one template for the four sets or one per command; one `kind: fail` per
  command enforcing a local rule (bm G1, H2). Hangs on: home.
- **Sizing and skips** — one size rule for the engine or per command; a one-gap delta or lane run
  (bm I2). Hangs on: core.
- **Build with items A and B** — the desk built on the engine or local-then-refactored; the
  BACKLOG trigger change; the line against the three-command convergence session (boilerplate
  there, judgment here) (bm J1, J2). Hangs on: home and re-homing scope.

**Out of scope** — implement's other moments (card slicing, cycle-checkpoint batches, final
validation, acceptance), handed to a named follow-up; setup's questioning; `/mochiko:feature`;
the red/green/refactor and verification mechanics.

**Blind-map angles taken as defaults, not forks** (each named when it arises, confirmed in the
batch): A5 (a falsifier for the engine — brainstorm's D23 counts as the regression watch) · B4
(shape for the four, never pre-built for more) · C5 (description budget and trigger collision,
inside the home fork) · E3 (the seed sentence handed to the card-slicing follow-up) · F4 (fan-in
of parallel seats under the transport floor) · G2 (one provenance vocabulary) · G4 (no new ID
family unless the home needs one) · I3 (greenfield seed kept apart, as `impl.design-absent-baseline-seed`
already rules).

**Blind-map angles dropped, listed for the end read** — F5 (the haiku Explore route returning no
report: a tooling watch already in BACKLOG, not a decision here) · G3 (final validation's
gap-finder reading the ruled set — final validation is out of scope; noted for the follow-up) ·
H3 (contract-suite cases for new floors — build work, not a decision) · J5 (build cost — a
consequence of the rulings, not a fork).

## Lead's options for Q1 — written before the second list was read

- (A) Stage engine with slots: the engine owns the whole stage's way of working — inputs in hand
  as facts and the handed-in skip, the fork core, the blind list mechanism, stop and stall,
  provenance, the confirm screen's form, and "the ruled set binds the author; anything off the set
  goes back to the user". Each command fills slots — its opening, its unit, its set's home, who
  authors after, its independent leg, its blind-list cadence and inputs — and keeps its own gate
  and its own `kind: fail` (command grammar). Brainstorm is a user: its session-level parts (frame
  card, decision map, size, front blind map, cold review, decision cards) stay its own.
- (B) Fork core only: the engine owns how one fork is put and how defaults batch (fork core,
  status line, stop and stall, provenance); every stage-level part stays in each command.
- (C) No engine yet (do-less; bears on the user's turn-3 ruling, bm A1): the desk and implement
  get local stages like specify's; convergence waits for the first real runs.
- Lead's pick: (A).

- Fan-in: second list arrived 2026-10-11 (`research/second-list-engine.md`: 15 roads — S1–S4 on
  scope, D0–D10 on form; fence held; two haiku Explore dispatches returned no report, locate reads
  re-run on the seat tier — disclosed). Fan-in complete: 2 of 2.

## Question trail

### Q1 — What does the shared engine own, and what does each command keep? (put 2026-10-11)

Unblocks: the engine's home, the re-homing scope, and every implement item on the map.

- Facts the second list adds (quoted there with path and line): a skill schema cannot carry a
  `kind: fail` rule or a moment (`crates/mochiko-cli/src/model.rs:224`;
  `crates/mochiko-cli/src/render.rs:47-48`) — under every road each command keeps its moment and its
  fail; implement's candidates are fixed by the gap list (`impl.design-gaps-only`), so a blind list
  there can only find forks inside each gap; `impl.sec.tools` already renders 26,201 bytes against
  a ~30k inline ceiling (`DECISIONS.md:49`), so implement's stage rules do not go into
  `impl.sec.tools`. *(Corrected at review, S12: as first written, "so implement's stage needs a new
  section" — a seventh section fails the command six-set, `.claude/rules/mochiko/primitive-edits.md`
  criterion 2; the ceiling is per section render, and the rules spread across the existing six,
  e.g. `impl.sec.ways-of-working`, 9 rules today.)*
- Where the second list and the lead's options differ (shown to the user before ruling):
  - Agree: the list's pick S2 (mechanics plus the ruled-set contract) is the lead's (A); the list
    adds a caller-set switch so brainstorm runs the mechanics only — folded into (A) as put.
  - The list's S1 is the lead's (B); its D1/D2 are the lead's (C).
  - Roads the lead did not list: S3 contract only (leaves brainstorm out, against
    `architecture-brainstorm-interview` D6-desk-stays-engine's four) · S4 one
    shared ruled-set file (breaks `spec.slate-home` and brainstorm's record) · D3 implement reached
    only through the desk (covers structural gaps only; data-model, contract and quickstart gaps
    stay sign-off-only) · D4 `/mochiko:brainstorm` as the engine (rejected at the desk: two commands
    per moment) · D9 share the artifact, not the conversation · D10 a facilitator seat (ruled out:
    a subagent cannot ask the user — *the reason as the second list gave it; corrected at review,
    S13, in D1-engine-asking-contract's rejected roads*).
  - Its delivery roads (D5–D8) belong to the next fork, the engine's home.
- Options put: (A) engine owns the asking and the ruled-set contract, brainstorm switches the
  contract off; (B) engine owns the asking only; (C) no engine yet — local copies, merge after first
  runs (do-less; bears on the user's turn-3 ruling, bm A1). Case against (A): abstract nouns each
  command must bind, a loose binding reopens the gap; n=0 runs at all four (bm A1). Lead's pick (A).
- User: "yes A, and the individual command like specify , architecture brainstorm extend it to
  their domain expertise" — (A) ratified, with the user's own addition folded in (streak: 1).

## Decisions

### D1-engine-asking-contract — The engine owns the asking and the ruled-set contract; each command extends it with its domain — `Confident`

**Statement.** One shared thinking stage — the engine — owns two halves. *The asking:* one fork
per turn; the fact a fork hangs on in hand before it is put; at least two real options, one the
do-less shape; a rough mock inside the message; the case against and a one-line pick;
own-turn-or-batch, the defaults named in one line as they arise and confirmed on one screen, never
by silence; a status line (ruled · askable · in fog); the ratified-yes streak flag; a vague answer
re-asked as two concrete options; the stop rule and the three-turn stall clause; a provenance mark
per item; a fenced blind second list from a fresh seat; inputs already in hand open the stage as
facts, and a skip skips the forks, never the close. *The ruled-set contract:* the lead transcribes
the user's rulings, then passes the pen with the user's words quoted; the seat that writes next
writes only inside the ruled set; anything off the set goes back to the user before it is written.
Brainstorm runs the asking half only — the contract is switched off for it by a caller-set
setting, since no seat writes from its record. Each command **extends the engine with its own
domain expertise** (the user's words): its opening, what the user rules (the unit and its domain
fields), where the ruled set is saved, who writes after, the blind list's cadence, inputs and job,
its independent leg, and its own gate and `kind: fail` — which a skill cannot carry (F15).

**Why.** The user's word. The contract half is the part that fixes the problem line — sign-offs
over drafts seats already authored; the asking alone would leave it as drifting copies. The three
stages already match item for item (F12, F13; the second list's shared-core reading). The skill
grammar keeps each command's gate and fail local under any road (F15), so the split follows the
grammar's own line.

**Rejected roads.** (B) the asking only — the contract stays three copies that drift, and implement
gets the conversation craft without a set that binds its design seats. (C) no engine yet, local
copies merged after first runs (bm A1) — the user's turn-3 ruling stands. Second-list roads: S3
contract only (leaves brainstorm out, against `architecture-brainstorm-interview` D6-desk-stays-engine's four) · S4 one shared saved
file for all four (breaks `spec.slate-home` and brainstorm's record) · D3 implement reached only
through the desk (data-model, contract and quickstart gaps stay sign-off-only) · D4
`/mochiko:brainstorm` as the engine (two commands per moment; rejected at the desk) · D9 share the
artifact only (the conversation craft keeps drifting) · D10 a facilitator seat — a teammate can
talk with the user ("You can also talk to any teammate directly without going through the lead",
https://code.claude.com/docs/en/agent-teams), but in-process teammates do not come back after a
resumed session ("`/resume` and `/rewind` do not restore in-process teammates", same page,
*Limitations*), and implement runs stop and resume; specify and the desk ruled the lead holds the
conversation. *(Reason corrected at review, S13: as first written, "a subagent cannot ask the
user".)*

**Accepted risk.** The engine is phrased in general nouns — "the ruled set", "the writer" — that
each command binds in its own rules; a loose binding reopens the gap, so each command's binding is
graded at the build. No stage of the four has run once (bm A1): the shared shape is a guess until a
run shows it; brainstorm's D23 counts are the regression watch (default, bm A5).

**How it was decided.** Q1, its own turn; the blind second list taken
(`research/second-list-engine.md`) and its differences shown; the case against given; user: "yes
A, and the individual command like specify , architecture brainstorm extend it to their domain
expertise" — ratified, with the user's own addition folded into the statement (streak: 1).

**Changed at review (S2; user: "as recommended", 2026-10-11).** "Anything off the set goes back to
the user before it is written" holds for anything that changes a ruling or that the user would
notice; a command with a later sign-off may let a small technical extra no ruled item touches wait
for it — implement's split is D8-checkpoint-shows-landing's. (S13, repair batch; user: "ok") The
rejected road D10's reason corrected: a teammate can talk with the user; the bar is that in-process
teammates do not come back after a resumed session, and implement runs stop and resume (source
re-read cold by the lead, quoted in the rejected roads). (V1, verify round 1; user: "i dont want to
make anything custom for setup and specify , the whole idea is shared skill in herited by all
workflows and any customisation needed sits in individual workflow commands", then "yes" to the
defaults) The engine carries one default for every slot, inherited by every caller with no
per-caller carve-out; a command customises only in its own rules. The defaults: the contract off;
the blind second list on a fork costly to undo; provenance marks shown as each item is ruled and
saved where the caller writes its output, else kept in the conversation (*Slots declared where*).

## Facts for the engine's home

- F15 — `research/second-list-engine.md` F1, quoting `crates/mochiko-cli/src/model.rs:224`:
  "`fail` is command-side only; a skill schema carrying it is a finding"; and
  `crates/mochiko-cli/src/render.rs:47-48`: "`kind: fail` and `enforces:` are illegal in a skill
  schema … and skills declare no `moments:` block".
- F16 — `impl.design-gaps-only`: "Design seats author exactly the named gaps, nothing more".
  Reading (the second list's, adopted as a reading): implement's blind list can only find forks
  inside each gap; a new candidate is scope escalation.
- F17 — `research/second-list-engine.md` F10: `mochiko-cli rules implement --section impl.sec.tools`
  renders 26,201 bytes against the ~30k inline ceiling recorded at `DECISIONS.md:49`.
- F18 — `DECISIONS.md` line 51 (skill-schema wave 2): "analysis-iterative · grooming-operating-docs ·
  testing-governance-injection stay prose".
- F19 — `mochiko-cli rules setup --section setup.sec.roles`, `setup.interrogation-inline`
  (`pointer: mochiko:analysis-iterative`): "Run the interrogation yourself, inline — the agenda's
  dimensions worked adaptively via mochiko:analysis-iterative".
- F20 — `plugins/mochiko/migrations/0001-genesis.yaml` lines 9777–9782: `common.transport-floor`, a
  pointer block — "mochiko:patterns-transport-floor governs composition and messaging …" — extended
  by six commands (`grep extends: common.transport-floor`: 6). The repo's shape for one discipline
  reached by many commands.

### Q2 — Where does the engine live? (put 2026-10-11)

Unblocks: the re-homing scope, the build with items A and B, implement's set home and fail.

- Blind list: the engine-shape seat's question covered form as well as scope, asked blind; its
  delivery roads D5–D8 stand as this fork's second list (disclosed — no second seat spawned). Its
  pick D6 is the lead's (A).
- Options put: (A) a new skill delivered by `mochiko-cli` (working name `patterns-thinking-stage`),
  reached through one shared pointer line each command extends with its domain part — the
  transport-floor shape (F20), no binary change; (B) grow `analysis-iterative` — reverses F18,
  reaches setup (F19, out of scope), rejected once by specify; (C) shared text blocks with
  fill-ins pasted into each command — needs the "boilerplate never judgment" limb widened (F11,
  S15 read against), no one place shows the stage, implement near its ceiling (F17). Not put: a new
  CLI document kind (crate code, grammar bump, a release — second list D8). Case against (A): one
  more skill, loaded by the lead mid-run. Lead's pick (A).
- Defaults named with it: *Precedence* — the engine's asking rules hold in every command; a command
  adds its domain and fills its slots, never loosens a mechanic; a difference a command needs
  becomes a slot by ruling; brainstorm's and specify's own-rules-win clauses leave by recorded
  supersession (bm C2). *analysis-iterative* — stays prose and stays the open-probe skill for
  setup's interrogation and specify's intent stage; its "single questioning engine" line is
  repointed at the new stage skill for the four stages (bm B5, C1).
- User: "compare a vs b" — a comparison put back, no ruling yet. Facts added for it:
  - F21 — `plugins/mochiko/skills/analysis-iterative/SKILL.md` line 15, *When NOT to Use*:
    "**Implementation details** — use planning skills instead".
  - F22 — `analysis-iterative`'s live pointers: `brainstorm.lead-inline-questioning`,
    `spec.intent-stage-first`, `setup.interrogation-inline` (F19); prose mentions in
    `review-specifications` and the router (`skills/mochiko/SKILL.md` lines 66, 124).
  - F23 — the preamble legend of every render: "class: floor is always delivered whatever its
    when:" — a mode setting can gate when a floor applies, never whether it reaches a caller.
  - F24 — `.mochiko/brainstorms/specify-brainstorm-discovery/record.md` line 592 (`specify-brainstorm-discovery` D5-specify-own-rules rejected
    roads): "(B) a third output shape in the shared questioning skill — grows a file setup and
    brainstorm read".
- User: "i am wondering why analysis-iterative have the intent in it , and not the individual
  commands" — a fact question, answered:
  - F25 — specify's intent agenda already lives in specify: `spec.intent-stage-first`
    (`pointer: mochiko:analysis-iterative`): "The intent stage runs before any authoring: the
    adaptive-probe agenda via mochiko:analysis-iterative — scope boundary · delivery intent ·
    depth-rigor expectation · UX-bearing … · constraints · out-of-scope." The pointer carries the
    *how* (adaptive probing); the agenda is specify's.
  - F26 — what `analysis-iterative` carries is a different shape: *Specification-input enrichment*
    (`SKILL.md` lines 58–62; `SPECIFICATION-INPUT.md`, `ENRICHMENT.md`) — the Who / Problem /
    Value triad. Ported from human-in-loop's specify cluster (commit `07c9dbc`, 2026-06-28, "Port
    specify cluster to mochiko form"); kept at v0.25.0 as "the live dispatch seam callers rely on
    (enrichment vs general analysis)" (`.mochiko/strips/analysis-iterative.md` lines 123–128).
  - F27 — no live caller of that shape: `grep -rn -i 'triad\|Who / Problem\|SPECIFICATION-INPUT\|ENRICHMENT.md'`
    over `plugins/mochiko` (`.md`, `.yaml`), the skill's own directory excluded, finds only
    `review-specifications`' "Not for: … pre-spec enrichment of a sparse idea" boundary line
    (`0001-genesis.yaml` line 8455). Method-scoped: not found by that grep over that scope.
  - Lead's reading (marked as reading): the user's instinct holds in today's tree — domain agendas
    already sit in the commands; the shared skill's domain-shaped part is a port leftover with no
    caller. Bears on Q2: the engine's home should hold the how, the commands the what (D1-engine-asking-contract's
    "extend with domain expertise").
- User: "i would rather not add a new skill, but ensure analysis-iterative is brought to the new
  shared skill we are building. any setup or specify dependecy should move out" — against the
  lead's pick (A); the user's own road: (B) as reshaped — the engine lives in `analysis-iterative`,
  and what setup and specify's intent stage take from it moves out (streak: 0).

### Frame change 1 (2026-10-11, the user's word at Q2)

- **Out of scope** — was: "setup's questioning". Now: "setup's questioning, except that setup's
  interrogation stops depending on `mochiko:analysis-iterative` — its pointer leaves
  `setup.interrogation-inline` and what setup needs on how to ask stays in setup's own rules."
  Source: the user's word ("any setup or specify dependecy should move out"). Re-check of the
  decisions already made: D1-engine-asking-contract — consistent (setup was never an engine user).

### D2-iterative-becomes-engine — The engine lives in `analysis-iterative`; its setup- and specify-shaped content moves out — `Contested`

**Statement.** No new skill. `mochiko:analysis-iterative` becomes the shared thinking stage of
D1-engine-asking-contract: it is converted from a prose skill to one whose rules `mochiko-cli`
renders from the migration log, and it carries the asking half and the switchable ruled-set
contract. Brainstorm's deciding, specify's story stage, the desk's focus stage and implement's
design phase reach it through one shared pointer line, each extending it with its domain part in
its own rules. The setup- and specify-shaped content inside the skill leaves it: the Who / Problem
/ Value enrichment variant (`SPECIFICATION-INPUT.md`, `ENRICHMENT.md`, and the *Two output shapes*
section) has no caller (F27) and is stripped, recorded. *(Amended at Q4 by the user's word, "as
recommended": setup's interrogation and specify's intent stage keep their pointer — never removed
and re-added. As first ruled, the two pointers left the skill now. Amended again at review, S4, by
the user's word: from this build both follow the engine's asking half, with no "not for you"
switch; the contract stays off for them; nothing in the skill waits for the follow-up session.)*

**Why.** The user's word: no new skill; the existing questioning skill is the one that becomes the
shared stage. It already calls itself "the single questioning engine" (F10), carries the
confidence marks and the anti-patterns the stage keeps, and its triggers and router entry exist.
Taking the setup- and specify-shaped content out answers part of the case against this road; the
rest — every engine change reaching setup and specify's intent (F19, F22, F23) — the user ruled no
problem at review (S4): they follow the engine's asking half like everyone else.

**Rejected roads.** (A) a new `patterns-thinking-stage` skill — the lead's pick, rejected by the
user: one more skill beside a questioning skill that already claims the engine's role. A "not for
you" caller switch gating the engine off for setup and specify's intent until a follow-up — the
lead's default, dropped at review (S4) by the user's word: receiving the engine's rules is no
problem, and a switch only creates a dependency on a later run. (C) shared text blocks pasted
into each command — the boilerplate-only limb (F11). A new CLI document kind — crate code and a
release (second list D8).

**Accepted risk.** Reverses a recorded ruling — `analysis-iterative` "stay[s] prose" (F18) — and
supersedes in part `specify-brainstorm-discovery` D5-specify-own-rules ("The shared questioning
skill is not edited for this", that record line 583) and its rejected road (F24); both leave by
recorded supersession at the build. From this build setup's interrogation and specify's intent
stage ask the engine's way — their asking changes with no setup or specify rule edited — and their
domain content stays in their own rules (setup's agenda and catalog deck,
`setup.interrogation-inputs`; the intent agenda, `spec.intent-stage-first`); the ruled-set contract
stays off for them; the build's no-weakening check covers them. The skill's `description:` and the
router's lines (`skills/mochiko/SKILL.md` lines 66, 124) are rewritten around the stage. The
skill joins no family today: its section set is a build choice.

**How it was decided.** Q2, its own turn; the second list's delivery roads stood as its blind
list; a side-by-side of (A) and (B) on the user's ask; the user's fact question on why the intent
shape sits in the skill, answered (F25–F27); then the user's own ruling against the lead's pick:
"i would rather not add a new skill, but ensure analysis-iterative is brought to the new shared
skill we are building. any setup or specify dependecy should move out" (streak: 0). Amended at Q4
(the user's word, "as recommended"): the user's "move out" read as setup's and specify's specific
content leaving the skill, setup staying a later user; frame change 1 reversed by frame change 2.

**Changed at review (S4, own turn; user: "is it a problem if setup received engine rules, i dont
think so , i want all the changes we planned to flow into analysis interative without creating
dependecy on a new run", then "yes" to the lead's reading, 2026-10-11).** The review found the card
adopting the road it rejected — setup and specify's intent still read the skill and floors reach
them whatever a setting says (F23) — with the reason absent from Q4's case against, the title still
saying "move out", and the change-control audit scoped to four callers. The floor exposure was put
plainly; the user ruled it no problem and widened: no "not for you" switch, setup and specify's
intent follow the engine's asking half from this build, their domain content in their own rules,
the contract off for them, nothing in the skill waiting on the follow-up, the no-weakening check
covering them. Title, statement note, why, rejected roads and accepted risk carry it;
D4-shaped-for-six, the *Change control* default and the *Probe-only callers* default change
with it (frame change 3).

### Q3 — Do brainstorm and specify move onto the engine in this same build? (put 2026-10-11)

- Facts: `BACKLOG.md` line 66 (*First ai-fileops specify run — the story stage's test*, open) and
  lines 128–140 (*Dogfood watch — the first target-state sessions*, open): neither rebuilt stage
  has run in the dogfood repo (bm J4).
- Options put: (a) all four move now, the stage ships once; (b) the desk and implement now,
  brainstorm and specify after their first runs (do-less now; breaks "ships once"). Case against
  (a): a first-run problem cannot be told apart between the move and the design. Lead's pick (a),
  with the pending ai-fileops specify run on the current build first, as a before-picture, if the
  user can.
- User, clarifying D2-iterative-becomes-engine first: "staying with previous question, so analysis-iterative will change and
  absorbe what we are planning for shared skill right?" — answered yes, with the before and after
  of the skill.
- User: "yes, not just 4 but also feature and setup if possible" — (a) ratified (streak: 1); and a
  widening asked conditionally: setup and `/mochiko:feature` as engine users. The widening bears on
  D2-iterative-becomes-engine as recorded (setup's dependency moves out) — put back to the user
  with the fit facts below, never folded silently.

### D3-four-move-now — Brainstorm, specify, the desk and implement all run the engine from this build — `Confident`

**Statement.** In the combined build, brainstorm's deciding and specify's story stage move onto
the engine alongside the desk's focus stage (built straight onto it — build item A) and implement's
design phase: each one's own copy of the asking rules and of the ruled-set contract is replaced by
the shared pointer line plus its domain part, so the stage ships once. Where the pending ai-fileops
specify run can be run on the current build first, it gives a before-picture to compare the first
engine-run against (the user's call, never a gate). The move is proven at the build by an
obligation map (changed at review, S7): before the move, every asking obligation brainstorm,
specify, setup's interrogation and specify's intent stage carry today is listed; after it, each maps
to an engine rule or the command's own domain part; every mismatch — a lost word like specify's
"one-row" mock — goes to the user to rule before the bump, and so does every obligation a caller
gains from the engine's defaults (setup's blind list on a costly fork; added at verify, V1). The pointer line to the engine is
`class: floor` in each user command, leaving only by recorded ruling. A plan-only grid comparison
is added only if the map leaves doubt.

**Why.** The destination's "the combined build ships it once". Leaving two of the four on their own
copies keeps the drift the engine exists to end.

**Rejected roads.** (b) the desk and implement now, brainstorm and specify after their first runs —
less now, but two copies beside the engine and a second migration of the same text later.

**Accepted risk.** Brainstorm (v0.117.0) and specify (v0.119.0) move before either has run in the
dogfood repo (bm J4): a problem in their first engine-run cannot be told apart between the move and
the design; the before-picture run is the mitigation where the user can make it.

**How it was decided.** Q3, its own turn, two options with the cheaper shape named and the case
against given; user: "yes, not just 4 but also feature and setup if possible" — ratified (streak:
1); the widening is put as its own question (Q4).

**Changed at review (S7, own turn; user: "as recommended", 2026-10-11).** The review found no
build-time check behind the frame's "no command's ruled behaviour weakens" — only "ships once" and
post-build dogfood — with the second list's partition and stub-class points never put. Put as (a)
a before-and-after obligation map, mismatches to the user, the pointer line `class: floor`; (b) a
plan-only grid comparison; (c) the audit and the dogfood runs. User: "as recommended" — (a), (b)
held in reserve. The statement carries it. (S11, repair batch; user: "ok") Re-slugged from
`D3-all-four-move-now` (four slug words) to the three-word `D3-four-move-now`, every cite in this
record swept.

## Facts for setup and feature as engine users

- F28 — `mochiko-cli rules setup` (plugin 0.119.0): `setup.interrogation-inline` — "Run the
  interrogation yourself, inline — the agenda's dimensions worked adaptively via
  mochiko:analysis-iterative, then the catalog deck, card by card, recommend-then-arbitrate";
  `setup.gate-synthesis-ratification` (`class: floor`): "Synthesis ratification is the user's.";
  `setup.synthesis-artifact`: "The synthesis: .mochiko/memory/governance-intent.md … the durable
  amend baseline"; moments: "interrogation: The inline agenda-then-catalog stage, worked card by
  card with the user. · ratification: The user's ratification of the synthesis — before any surface
  is authored."
- F29 — `mochiko-cli rules feature` (plugin 0.119.0): `feat.dm-converge-goal` — "Converge every
  visit to a one-line goal and its done condition."; `feat.delta-cards` — "One delta card per the
  tasks template's card shape … a bug's acceptance is its reproduction-failing test, an improvement
  carries 1–3 acceptance criteria … Each card … takes the sound-loop review leg"; `feat.pm-seat` —
  "It recommends with reasons; it never rules."; `feat.growth-door` (`class: floor`): "an extend
  verdict only, with acceptance criteria on the card."; moments: visit-open, close.
- Lead's reading (marked as reading): setup already has the engine's shape — think with the user
  (agenda, then the deck), a ruled synthesis the user ratifies, then a seat authors inside it, with
  a cold review between; its domain parts are the agenda and the catalog deck. The feature desk's
  shape is looser: a per-visit goal, then a delta card or a growth row whose acceptance criteria are
  the thing ruled; its unit and where the ruled set lives are new design work, not a rename.

### Q4 — Do setup and feature become engine users? (put 2026-10-11)

- Facts F28, F29. The conflict with D2-iterative-becomes-engine as first recorded (setup's
  dependency moves out) put plainly with the lead's reading: "move out" meant setup's specific
  content, setup staying a user.
- Options put: (i) six users in this build — frame widens, size too-big; (ii) shape the engine's
  slots for six, build four now, a named follow-up session joins setup and feature (with
  implement's other moments), setup's pointer left as it is until then; (iii) no — D2-iterative-becomes-engine as first
  recorded. Specify's intent stage follows setup's answer. Case against (ii): two commands keep
  their own asking for a while, a second build later. Lead's pick (ii).
- User: "as recommended" — (ii) ratified (streak: 2); the lead's reading of "move out" stands as
  the user's.

### Frame change 2 (2026-10-11, the user's word at Q4)

- **Out of scope** — frame change 1 reversed: setup's interrogation keeps its pointer to
  `mochiko:analysis-iterative` for now. Now: "setup's questioning and `/mochiko:feature` — the
  engine's slots are shaped so both, and specify's intent stage, can join later; a named follow-up
  session joins them, together with implement's other moments."
- **Destination** — gains: "the engine's slots named for setup, feature and specify's intent stage;
  their fit checked at the follow-up." *(Softened at review, S10: as first written, "the engine
  shaped so setup, feature and specify's intent stage can join it later without reopening it" — a
  promise nothing at the build can show.)*
- Re-check: D1-engine-asking-contract — consistent (its slot list is the one shaped for six);
  D2-iterative-becomes-engine — amended at Q4 (above); D3-four-move-now — consistent. The
  blind-map default B4 ("shape for the four, never pre-built for more") changes to "shape the slots
  for six; build four".

### Frame change 3 (2026-10-11, the user's word at review, S4)

- **Out of scope** — was: "setup's questioning and `/mochiko:feature` — … a named follow-up
  session joins them". Now: "`/mochiko:feature` and the contract half for setup and specify's
  intent — a named follow-up session takes them, with implement's other moments. Setup's
  interrogation and specify's intent stage follow the engine's asking half from this build through
  their unchanged pointers; their own rules and content are not edited."
- Re-check: D1-engine-asking-contract — consistent (the contract switch stays off for them);
  D2-iterative-becomes-engine and D4-shaped-for-six — amended at review (their cards);
  D3-four-move-now and D5–D8 — consistent.

### D4-shaped-for-six — The engine's slots are named for six users; four run the full stage now, feature joins at a follow-up — `Confident`

**Statement.** The engine's slots — opening, unit, where the set is saved, who writes after, the
blind list's cadence and job, the independent leg, the contract switch — are named for setup's
interrogation (its agenda and catalog deck as its domain, the governance-intent synthesis as its
ruled set), `/mochiko:feature`'s ruling visits, and specify's intent stage too; their fit is
checked at the follow-up, never claimed now (softened at review, S10). This build moves the four onto the full stage (D3-four-move-now); from
the same build setup's interrogation and specify's intent stage follow the engine's asking half
through their unchanged pointers, the contract off for them, their domain content in their own
rules (changed at review, S4). A named follow-up session joins `/mochiko:feature`, takes
implement's other moments, and rules whether setup or specify's intent take the contract half.

**Why.** Setup fits the engine closely (F28); the feature desk's unit and set home are new design
work (F29); six users in this session would make it, and the build, too big to finish well.

**Rejected roads.** (i) six full users now — too-big, and feature's unit and set home are new
design work (setup's asking does change in this build — frame change 3 — but not its contract or
domain). (iii) never — leaves two commands asking their own way for good, against the
user's "if possible".

**Accepted risk.** Feature keeps its own asking until the follow-up; the slots are named for users
not yet built against, so the follow-up may still find a misfit — feature's fit is `Assumed` (S10). Setup's and specify-intent's
asking changes in this build with no rule of theirs edited — covered by the no-weakening check.

**How it was decided.** Q4, its own turn, three options with the cheaper shape named and the case
against given; user: "as recommended" — ratified (streak: 2).

**Changed at review (S4; the user's word, 2026-10-11).** Setup's interrogation and specify's intent
stage follow the engine's asking half from this build, with no caller switch; the follow-up keeps
feature, implement's other moments, and the contract question for setup and intent (D2-iterative-becomes-engine
changed-at-review; frame change 3). (S10, S11, repair batch; user: "ok") "Shaped so … can join
without reopening" softened to "named for … fit checked at the follow-up", feature's fit marked
`Assumed`; re-slugged from `D4-shape-six-build-four` (four slug words) to the three-word
`D4-shaped-for-six`, every cite in this record swept.

### Q5 — Where does implement's stage sit, and when does it fire? (put 2026-10-11)

- Facts: `plugins/mochiko/commands/implement.md` line 78: "**Run-open confirmation — the entry
  gate.** One confirmation, no negotiation"; `.mochiko/decisions/2026-08-13-charter-plan-implement.md`
  line 27 (ruling 3): "Goal protocol maps existing gates — no new ceremony" (the goal protocol's
  convergence, not a stage); `impl.design-phase-fires-on-gap`; `impl.epic-design-always-fires`;
  `impl.zero-gap-map-assertion`.
- Blind list: the engine-shape seat's implement section, drawn blind, placed the stage "after
  run-open and before any design seat plans, firing only when the sufficiency check names gaps"
  (`research/second-list-engine.md`) — taken as this fork's blind list, disclosed; no new seat.
- Options put: (a) a new stage after run-open, before any design seat plans, firing on gaps only,
  once over the joint gap list on an epic; (b) folded into run-open; (c) do less — today's order,
  the checkpoint asked in the engine's form. Case against (a): one more stop (bm H1). Lead's pick
  (a).
- User: "as recommded" — ratified (streak: 3). Three ratified in a row: the next fork goes in the
  case-against-first form.

### D5-stage-after-runopen — Implement's stage runs after run-open, before any design seat drafts, on gaps only — `Confident`

**Statement.** Implement gains a moment for the engine's stage: after the run-open confirmation and
before any design seat drafts — the design seat's read-only piece proposal runs inside the stage,
riding its plan-only dispatch, and its plan is finished inside the ruled set once the stage closes
(D7-direction-then-pieces; reworded at verify, V2). It fires only when the sufficiency check names gaps; on
the zero-gap path there is no stage. On an epic it runs once, over the joint gap list and the joint
spine's design items — the seams between members, the joint architecture, the batch ordering
(`impl.epic-design-always-fires`) — which join the agenda beside the gaps (changed at review, S9).
The run-open stays one confirmation, no negotiation. The ruled
design set it closes in is the scope the design seats plan and draft inside.

**Why.** The problem line — seats draft first, the user signs after; the stage puts the user's
rulings before the first draft. Run-open's rule and the charter's no-new-ceremony line keep the
stage out of the run-open screen.

**Rejected roads.** (b) folded into run-open — run-open becomes a negotiation, against its own rule.
(c) today's order with an engine-style checkpoint — the user reacts to the seats' guesses, the
"draft first, then brainstorm at the gate" road the desk rejected
(`architecture-brainstorm-interview` D1-focus-stage-first, second-list road 6).

**Accepted risk.** One more stop in a run that already has many (bm H1); the checkpoint item may
merge or slim one.

**How it was decided.** Q5, its own turn, three options with the cheaper shape named and the case
against given; the blind list's placement shown; user: "as recommded" — ratified (streak: 3).

**Changed at review (S9, coverage, ruled inline; user: "as recommended", 2026-10-11).** On an epic
the design phase always fires for the joint spine, whose seams and ordering are not gaps; with the
agenda fixed to the gap list (D6-outside-need-asked) the largest design would be drafted with no
stage. Ruled inline: the spine's design items join the epic's agenda beside the gaps.

## Facts for the stage's agenda

- F30 — `mochiko-cli rules review-sufficiency --section review-sufficiency.sec.verdict` (plugin
  0.119.0): the clause set grades testable criteria · contract exposure · data exposure · structural
  trigger · NFR targets · commodity exposure · dependency order · UX trace and the rest, each with
  its gap form; `review-sufficiency.every-applicable-clause` (`class: floor`): "a clause that cannot
  be graded is a gap, never a pass."
- F31 — `impl.scope-escalation-fail`: "work bigger than the run was framed stays FAIL unless the user
  accepts it"; `impl.design-gaps-only`: "Design seats author exactly the named gaps, nothing more";
  `review-plan-artifacts`' description: "nothing materially past the gap list; material divergence
  auto-FAILs — BLOCKING".
- Lead's reading (marked as reading): which design areas the user thinks through and which are
  defaulted (bm A2) is already answered by the engine's own-turn-or-batch rule, gap by gap — a real
  choice or a costly one gets its own turn, the rest go to the defaults batch. The open edge is a
  design need the conversation finds outside the graded gap list (bm D1).

### Q6 — A design need outside the gap list (put 2026-10-11, case-against-first form)

- Options put: (a) ask the user — add to this run (the gap list grows, marked user-added in the
  sufficiency report, the design review grades against the grown list) or later (BACKLOG or the
  feature desk); (b) always later — the gap list stays as graded; (c) open agenda. Case against (a)
  first: the run's scope blurs and the "unless the user accepts" path turns routine. Lead's lean
  (a).
- User: "ask in simple language" — re-put in plain words with one example (an Invoice table and a
  create-invoice API on the list; a Refund table found mid-talk).
- User: "as recommded" — ratified (streak: 4). The next fork goes in the weak-lean form; questions
  stay short and plain.

### D6-outside-need-asked — A design need found outside the gap list is put to the user: this run, or later — `Confident`

**Statement.** The stage's agenda is the gap list the sufficiency check graded — on an epic, with
the joint spine's design items beside it (D5-stage-after-runopen; added at verify, V3). Inside each gap the
engine's own-turn-or-batch rule decides what the user thinks through. A design need the
conversation finds outside the list is put to the user, split by kind (changed at review, S1):
*new behaviour* — something the spec carries no story or acceptance criteria for — is never added
to this run: it goes to `/mochiko:specify`, or *later* (`BACKLOG.md` or a feature-desk card); *a
design piece inside a story the spec already carries* is put as *add to this run* or *later* — if
added, the sufficiency seat re-grades it as a gap before it joins (the report keeps its one
writer, the grader), and the design review grades against the grown list. A lane run never adds
(`impl.lane-never-widens`). Nothing outside the list is designed without that ruling.

**Why.** The user decides each widening at the moment it is noticed; it is the existing "unless the
user accepts it" rule (F31) applied where the need surfaces, and the design review's gap-list
conformance check (F31) still has one list to grade against.

**Rejected roads.** (b) always later — keeps runs tight but sends a need the user wants now to a
second run. (c) open agenda — the gap list stops meaning anything and the review's conformance
check has nothing fixed to hold.

**Accepted risk.** A run can grow mid-stage, so what was selected and what is built can differ —
bounded by each addition being the user's ruling, marked in the report.

**How it was decided.** Q6, its own turn, in the case-against-first form (streak at 3), re-put in
plain words at the user's ask; user: "as recommded" — ratified (streak: 4).

**Changed at review (S1, own turn; user: "as recommended", 2026-10-11).** The review found a
user-added gap ungraded by the sufficiency seat (`review-sufficiency.every-applicable-clause`), with
no acceptance criteria in the spec when it is new behaviour (`impl.card-contents`), a second writer
on the grader's report, no lane exception against `impl.lane-never-widens` (floor), and the second
list's road — "A candidate outside the gap list is scope escalation to the user, never a widened
gap" (`research/second-list-engine.md`) — never shown at Q6. Re-put with that road: (a) split by
kind — new behaviour to specify or later, a design piece inside an existing story re-graded by the
sufficiency seat before it joins, lanes never add; (b) D6-outside-need-asked as ruled, the holes patched; (c) always
later. User: "as recommended" — (a). The statement's split is that ruling; the *Sizing and skips*
default's "no size gate" no longer reaches a lane's widening.

### Q7 — For each gap, what does the user rule? (put 2026-10-11, weak-lean form, plain words)

- Options put: (a) what it should do in the user's words plus one "I'd know it works if…" line,
  the technical details in one defaults batch; (b) every piece — fields, types, status values;
  (c) yes or no on the lead's suggestion. Weak lean (a), unsure — (c) may serve small gaps.
- User: "i think the lead should take the my initial input as (a) suggest but help me brainstorm
  the pieces of it" — the user's own road: (a) as the opening, then the pieces brainstormed with
  the lead (streak: 0).

### D7-direction-then-pieces — Per gap the user gives the direction and a "how I'd know" line; the lead then brainstorms the pieces with them — `Confident`

**Statement.** For each gap, the user's first input is the gap's direction in their own words plus
one "I'd know it works if…" line — the seed (`open` allowed, carried as an open question, never
invented). The lead takes that as the opening; the design seat that owns the gap's surface
(technical-analyst for data and contract gaps, principal-architect for structural ones) then
proposes its pieces read-only — riding its existing plan-only dispatch, writing nothing — for a data
gap the fields, states and rules; for an API gap the inputs, outputs and errors; for a structural
gap the parts and how they talk (changed at review, S8). The lead puts those candidate pieces to the
user and brainstorms them through the engine: a
piece that is a real choice or costly to undo gets its own turn, put as what the user would notice
(plain consequences and a worked example with real values, never a schema); the rest go into the
defaults batch, each with its suggestion and its cost. The ruled set for a gap is the direction,
the seed and the ruled pieces; the design seats draft inside it.

**Why.** The user's word. The direction pins what the gap means in the user's terms; brainstorming
the pieces keeps the user in the decisions that shape the build without turning the stage into a
schema form, and the defaults batch keeps the purely technical choices to one screen (bm E1, E2).

**Rejected roads.** (a) as first put — direction and seed only, every piece batched: the pieces that
matter would arrive as defaults. (b) every piece ruled as a fork — technical, many turns, the
ratified-yes streak the problem names one layer down (bm E2). (c) yes or no on the lead's whole
suggestion — the sign-off shape the frame names.

**Accepted risk.** More turns per gap than (a) or (c); the lead's judgment of which pieces are real
choices decides the turn count, and the defaults batch is where a missed real choice would hide —
the user may pull any default out.

**How it was decided.** Q7, its own turn, in the weak-lean form (streak at 4), plain words with one
example; the user's own road, combining (a) with a brainstorm of the pieces (streak: 0).

**Changed at review (S8, own turn; user: "as recommended", 2026-10-11).** The review found the lead
framing pieces the design seats' skills own (`patterns-entity-modeling`, `patterns-api-contracts`,
`patterns-system-design`; CLAUDE.md axis 4), the seat's expertise arriving only as a correction at
the checkpoint, and a road never weighed — the design seat proposes candidate pieces read-only in
its plan-only dispatch and the lead puts them to the user. Put as (a) that road; (b) D7-direction-then-pieces as ruled;
(c) the user rules only what they would notice, the seat decides the rest, shown at sign-off. User:
"as recommended" — (a). The seat's plan is finished inside the ruled set once it closes and is
peer-graded as today (`impl.plan-approval-producers`); the proposal step writes nothing.

## Facts for the design checkpoint

- F32 — `impl.gate-design-checkpoint` (`class: floor`, F2): "no code is written before the user
  signs the design and the store delta"; `impl.fail.design-skipped` (`class: floor`, `kind: fail`):
  "Gaps were found but the design phase was skipped, or its design and store delta were never
  signed by the user."
- F33 — `.mochiko/brainstorms/architecture-brainstorm-interview/record.md` line 1172
  (`architecture-brainstorm-interview` D8-draw-before-code): the marked-up "diagram set is what the user signs at the design
  checkpoint"; the deviation gate and the built-vs-signed diff anchor to the signed object.
  Reading: removing the user's signature would amend an accepted architecture decision — the frame
  says it must not break.

### Q8 — What does the design sign-off look like after a ruled stage? (put 2026-10-11, plain words)

- Facts F32, F33. Options put: (a) a shorter sign-off — a reviewer first checks the drafts kept the
  user's meaning; the user sees one table of how each decision landed plus anything the seats added,
  and signs; (b) no sign-off when nothing was added — fewer stops, but undoes the accepted
  architecture rule that the user signs the drawings (F33) and the checkpoint floor; (c) today's
  full sign-off. Lead's pick (a).
- User: "as recommded" — ratified (streak: 1).

### D8-checkpoint-shows-landing — The design checkpoint stays the user's signature, shown as how each ruling landed — `Confident`

**Statement.** `impl.gate-design-checkpoint` stays a `class: floor` gate: no code before the user
signs. After a ruled stage it is presented as a landing table — each ruled item (direction, seed,
ruled piece, user-added gap) beside how it landed in the drafts and its status (kept, or changed
with why), plus every small technical extra the seats added on their own, each needing the user's
OK. A seat addition splits by size (changed at review, S2): a small technical extra no ruled item
or default touches (an index for speed) waits for the table; anything that changes a ruling, or
that the user would notice (invoices become deletable), halts to the user through the lead before
it is written. `mochiko:review-plan-artifacts` gains the meaning-kept grade — the drafts against the
ruled set, every seat addition flagged — beside its gap-list conformance check, and its result fills
the table. The signed object stays what `architecture-brainstorm-interview` D8-draw-before-code
names (the run's `architecture.md` and the delta-styled store diagrams it lists); the table is how
the signature is asked. On the zero-gap path nothing changes. The floor's wording is amended at the
build, never its force.

**Why.** The user already made the decisions in the stage; the sign-off only has to show they
survived the drafting, and the additions the user never ruled. Keeping the signature keeps
`architecture-brainstorm-interview` D8-draw-before-code and `impl.fail.design-skipped` whole (F32, F33).

**Rejected roads.** (b) no sign-off when the review finds nothing added — fewer stops, but it amends
an accepted architecture decision and a floor. (c) today's full sign-off — the user re-reads drafts
of decisions already made.

**Accepted risk.** The run keeps both the stage's close and the checkpoint (bm H1, D5-stage-after-runopen's
risk); the table is only as honest as the review's meaning-kept grade.

**How it was decided.** Q8, its own turn, three options in plain words with a mock of the table;
user: "as recommded" — ratified (streak: 1).

**Changed at review (S2, own turn; user: "as recommended", 2026-10-11).** The review found three
texts disagreeing on a seat's own addition — D1-engine-asking-contract ("anything off the set goes
back to the user before it is written"), the *Implement — the fail* default (such a write fails the
run) and this card (an OK at the checkpoint, after drafting) — and no named grader for meaning kept.
Put as (a) split by size, the fail narrowed, the meaning-kept grade to `review-plan-artifacts`; (b)
every addition halts first; (c) every addition waits for the sign-off. User: "as recommended" —
(a). The statement carries the split and the grader; D1-engine-asking-contract's line reads with this split for implement;
the fail default is narrowed to match.

## Supersessions owed at the build (S6, coverage, ruled inline; user: "as recommended", 2026-10-11)

Each leaves by a recorded supersession-by-ruling at the build (strip entries per primitive, the
`DECISIONS.md` row annotations):
- `architecture-brainstorm-interview`'s confirmed default *Where the rules live*
  (`.mochiko/brainstorms/architecture-brainstorm-interview/record.md` line 1375 — the desk's own
  schema "and a precedence clause stating that where the desk's rules and
  `mochiko:analysis-iterative` differ, the desk's rules win"): **flipped by the user's word** — the
  engine's asking rules win; the desk keeps its domain rules (the focus set, the visit file, its
  `kind: fail`) in its own schema.
- `brainstorm-target-state` D19-brainstorm-only-reach and `brainstorm.own-rules-win`: brainstorm's
  asking now reaches five more callers through the engine.
- `specify-brainstorm-discovery` D5-specify-own-rules ("The shared questioning skill is not edited
  for this") and `spec.own-rules-win`.
- `analysis-iterative` "stay[s] prose" (`DECISIONS.md` line 51) and the protected `[v0.25.0] KEPT:`
  entry and v0.63.0 keep-set naming `## Two output shapes, one engine` the caller-facing dispatch
  seam (`.mochiko/strips/analysis-iterative.md` lines 93, 123); and the same keep-set's anti-pattern
  rows (`## Common Mistakes`, `## When NOT to Use`, line 89), reworded under S3 and F21 (added at
  verify, V5).
- `architecture-brainstorm-interview` D6-desk-stays-engine's hand-off line ("Building that shared
  stage is the three-command convergence session's job" as changed at S15 to its own session) and
  the `BACKLOG.md` trigger "build items A and B landed" (lines 47–50), overridden by the user at
  frame turn 3.
- `impl.gate-design-checkpoint`'s wording (D8-checkpoint-shows-landing — the force kept).

## Touch set for the build (added at review, S15; user: "ok")

The primitives and gate files the combined build touches, beside architecture build items A and B
as ruled. Each primitive edit lands under the ritual — migration or strip, gate audit, budgets.
- `plugins/mochiko/skills/analysis-iterative/` — `SKILL.md` converted to a rendered skill (a new
  migration carrying the engine's rules and the contract switch); `SPECIFICATION-INPUT.md` and
  `ENRICHMENT.md` stripped; *Two output shapes* stripped; the common-mistakes rows reworded; the
  *When NOT to Use* line "**Implementation details** — use planning skills instead" (F21) rewritten,
  since implement's design phase now runs the skill (added at verify, V5); `description:`
  rewritten; `references/ADAPTIVE-EXAMPLES.md` and `SYNTHESIS.md` kept.
- The router `plugins/mochiko/skills/mochiko/SKILL.md` — lines 66 and 124 rewritten.
- A `common.*` pointer block in the migration log — the shared line each user command extends.
- `brainstorm` schema — its asking rules replaced by the stub plus its domain part;
  `brainstorm.own-rules-win` superseded; the pointer `class: floor`.
- `specify` schema — the story stage's asking rules replaced likewise, the contract named *on*;
  `spec.own-rules-win` superseded; the intent stage's pointer unchanged.
- `architecture` schema — build item A's focus stage written onto the engine, the contract *on*,
  the desk's precedence clause flipped (the supersession list).
- `implement` schema and `plugins/mochiko/commands/implement.md` — the new stage moment and its
  rules across the existing six sections (never `impl.sec.tools`, never a seventh);
  D6-outside-need-asked's split; D7-direction-then-pieces's read-only piece proposal;
  D8-checkpoint-shows-landing's landing-table wording of `impl.gate-design-checkpoint`; the new
  `kind: fail`; the declared home `design-stage.md` (feature and epic homes).
- `review-plan-artifacts` — the meaning-kept grade (D8-checkpoint-shows-landing).
- `review-sufficiency` — the re-grade of a design piece the user adds inside an existing story
  (D6-outside-need-asked).
- Gate files: `evals/contract/README.md` line 370 lists `analysis-iterative` among the prose skills
  that "need no stub" — it no longer is; `evals/plan/brainstorm/observable.yaml` line 103 observes
  `brainstorm.own-rules-win`, which leaves; the contract suite's frozen floor sets gain the new
  pointer floors and implement's fail.
- The obligation map (D3-four-move-now) — a build artifact in the build's session home.

## Decision map — after D8-checkpoint-shows-landing (2026-10-11)

Askable now: none. In fog: none — every load-bearing blind-map angle is ruled in D1–D8 or carried
by a named default below (bm A3, A4, B3, B5, C4, D4, D6, D7, F1, F2, F3, G1, H2, I1, I2, J1, J2,
J3). Out of scope: as frame change 3 (at review; frame change 2 when this map was drawn). Left:
the defaults batch, put on the accept screen.

## Defaults batch — named as they arose, put for one confirmation at the accept screen

- **Who holds the conversation.** The lead, inline, in the main pane, in every user command's
  stage; it transcribes the user's rulings (no sound loop) and passes the pen with the user's words
  quoted (bm F1).
- **Precedence.** The engine's asking rules hold in every user command; a command adds its domain
  part and fills its slots, never loosens a mechanic; a difference a command needs becomes a slot
  by ruling. `brainstorm.own-rules-win` and `spec.own-rules-win` leave by recorded supersession
  (bm C2).
- **Change control.** An engine change is a primitive edit to `analysis-iterative` under the landing
  ritual; its gate audit re-reads every caller's render — the four users, setup's interrogation and
  specify's intent stage (bm A4; widened at review, S4).
- **Slots declared where.** Each command's own rules carry its domain part; the contract switch is a
  caller-set condition in the skill (as `review-brainstorm`'s `lens`); one shared pointer line, a
  `common.*` block each user command extends (the transport-floor shape, F20). No grammar or binary
  change; if the build finds one needed, it goes back to the user (bm B3, C4). The contract switch
  defaults to *off* when a caller names none; specify's story stage, the desk's focus stage and
  implement's design stage name it *on* in their own rule text (a stub inherits text only, so the
  mode never rides the shared line); brainstorm, setup's interrogation and specify's intent stage
  name nothing and need no edit. *(Added at review, S5.)* Every slot carries one engine default,
  inherited by every caller, with no per-caller carve-out; a command customises only in its own
  rules (the user's word at verify, V1). The defaults: the contract *off*; the blind second list on
  a fork costly to undo (brainstorm's cadence — specify's story stage sets once per run, the desk once per
  multi-part visit, implement once per design phase, each in its own rules); provenance marks shown
  as each item is ruled and saved wherever the caller writes its output, kept in the conversation
  where that output has no place for them (setup's `governance-intent.md`).
- **"Ships once", measured.** After the build no stage-mechanic rule text survives duplicated across
  the four commands' renders (the similar-rules detector quiet on them), and each render resolves
  the pointer (bm A3).
- **Setup and specify's intent follow the asking half.** *(Was "Probe-only callers until the
  follow-up"; replaced at review, S4, by the user's word.)* Setup's interrogation and specify's
  intent stage keep their pointer and, from this build, follow the engine with its defaults like
  every caller — the blind list on a costly fork, provenance marks in the conversation, the contract
  off — with no caller switch and no carve-out; their domain content stays in their own rules (bm
  B5; D4-shaped-for-six as changed; defaults named at verify, V1).
- **What `analysis-iterative` keeps.** The confidence marks, the common-mistakes list and the
  adaptive examples — the list's rows reworded to agree with the engine, recorded as a
  supersession: "one question per turn — always" reads one fork per turn, with the defaults batch
  confirmed on one screen as the one stated exception; "always using structured options" keeps an
  open probe allowed for an unsure user beside the engine's options rule (as `brainstorm.question-form`
  does) *(added at review, S3)*; the enrichment variant and *Two output shapes* stripped, recorded;
  `SYNTHESIS.md` kept for a caller that names no deliverable; its `description:` and the router's
  lines rewritten; its section set taken from an existing family set at the build, none minted.
- **Implement — the blind list.** Once per design phase, a fresh persona-less seat spawned
  `model: opus`, given the spec, the gap list, the store index and any visit file — never the ruled
  set or the conversation — returns the hidden choices inside each gap; each difference is a
  candidate piece the user rules; skipped on a single-gap re-fire (bm F2; F16).
- **Implement — handed-in sets.** A desk visit file keyed to the feature, the spec's slate and
  Intent, and a named brainstorm record open the stage as facts, provenance `handed in`, never
  re-asked; a visit file that covers some gaps leaves the stage to run on the rest (bm D4, I1).
- **Implement — mid-build re-fire.** Undesigned structure that halts a cycle re-fires a short stage
  scoped to the discovery — build-blocking, so it interrupts (`impl.escalation-batching`'s own
  exception) — then the same review and landing-table sign-off (bm D6).
- **Implement — the set's home.** One new declared file in the feature home (working name
  `design-stage.md`; the epic home on an epic), the desk visit file's item shape — direction · seed ·
  pieces · provenance — surviving a stopped and resumed run (bm G1).
- **Implement — the fail.** A design-phase write with no ruled set behind it, a write that changes a
  ruling without the user's word, or a small technical extra the user did not OK at the sign-off,
  fails the run — a `kind: fail` mirroring `spec.fail.story-outside-slate`, scoped to runs where the
  stage fired (bm H2). *(Narrowed at review, S2.)*
- **Sizing and skips.** No size gate; a direction the user hands in is the opening and its pieces
  are still brainstormed where real choices remain; a skip skips forks, never the close (bm I2). A
  lane run never widens (D6-outside-need-asked; pointer added at verify, nit).
- **Where TEST cases sit.** The stage ends at the design files; the qa-engineer's `**TEST:**` cases,
  and the seed's hand-off to them, belong to card slicing — the follow-up session (bm D7, E3).
- **The build.** One combined build with architecture items A and B: the engine first, then the four
  users; item A's focus stage written straight onto the engine, never as a local copy; item B as
  ruled. BACKLOG at landing: the *Shared thinking-stage design session* item closes into this
  record; the *Three-command convergence session* stays, narrowed to boilerplate outside the stage,
  triggered by this build's landing; a new item for the follow-up session (bm J1, J2).
- **One provenance vocabulary.** `user's words` · `lead's guess, ratified` · `blind list` ·
  `handed in` (bm G2).
- **Greenfield seed kept apart.** As `impl.design-absent-baseline-seed` already rules (bm I3).
- **Regression watch and dogfood criteria.** Brainstorm's D23 counts are read on its first
  engine-run session; what implement's stage is judged on is decided from its first ai-fileops run
  log, never before it (bm A5).

## Open questions

- OQ1 — The evidence base: implement's problem has not been shown in a real run; the diagnosis is
  `Assumed` until one shows it.
- OQ2 — The follow-up session: `/mochiko:feature` joins the engine; whether setup's interrogation
  and specify's intent stage take the contract half (their asking moves in this build, frame change
  3); implement's other moments (card slicing, cycle-checkpoint batches, final validation,
  acceptance). A BACKLOG item at landing. *(Updated at verify, V4.)*
- OQ3 — Whether the before-picture ai-fileops specify run happens before the build lands — the
  user's call (D3-four-move-now).

## Untested bets (shown at the accept screen)

- One engine serves four commands without flattening what each needs — tested on each command's
  first engine-run, judged by the user.
- Brainstorm and specify do not get worse when moved — tested by brainstorm's D23 counts on its
  first engine-run, and specify's first ai-fileops run against the before-picture where one exists.
- Implement's stage does not make a run much heavier — tested as turns and the user's time to the
  set's close on the first ai-fileops implement run, judged by the user.
- The landing-table sign-off is enough — tested by the built-vs-signed diff on the first feature
  showing no unsigned deviation, and the user not asking for the full drafts.
- Brainstorming the pieces keeps questions at the right height — tested by how many defaults the
  user pulls into their own turn and how many forks the user finds too technical.

## Freeze

Accept screen shown 2026-10-11 — the eight decisions by name and gist, the eighteen defaults, the
out-of-scope line as changed, the three open questions, the five untested bets with their tests;
the pull offer made; the user pulled none. **User: "confirm"** — the defaults batch is confirmed as
one block; the record is frozen for the cold read. The review seat (`review-seat`, the blind-map
seat) is resumed with this record for message two. Writers hold until the verdict
(`patterns-transport-floor.quiesce-before-cold-grade`).

Disclosure (2026-10-11, at freeze): until this point the lead appended to this record through
shell heredoc redirects; at the freeze write the artifact hook denied a shell write under the
declared home ("artifacts under declared homes are written with Write/Edit, never through a shell
redirect"). Earlier appends had not been caught by the hook. From the freeze on, every write to
this record and the index goes through Write/Edit. Content is unaffected; the breach is the
channel.

## Cold review — landed 2026-10-11 (`reports/review.md`)

- Seat `review-seat` resumed with the frozen record (message two, fence on the session directory
  lifted for the record, `research/second-list-engine.md` and its own map; index and memory stayed
  fenced). Fan-in: 1 of 1 — the report arrived. No cross-examination questions sent.
- Verdict: `needs-revision` — 18 raised, 15 survivors: 0 Critical · 8 Important · 7 Minor; 2
  coverage (S6 Important, S9 Minor). Card parts 8/8 complete. The seat dismissed the heredoc channel
  breach as commentary after checking the record (`mochiko-cli check` allow, no expansion damage).
- S13's external claim re-read cold by the lead (EXTERNAL-CLAIMS re-read clause), 2026-10-11, at
  https://code.claude.com/docs/en/agent-teams: "You can also talk to any teammate directly without
  going through the lead." and, under *Limitations*: "**No session resumption with in-process
  teammates**: `/resume` and `/rewind` do not restore in-process teammates." Both quoted lines
  present — S13 survives.

### Disposition plan

- Own turn (a real choice, costly to get wrong, or a challenge to a user ruling —
  `brainstorm.non-coverage-survivors`, `brainstorm.user-survivor-challenge`): S1
  (D6-outside-need-asked, re-put with the second list's unput road) · S2 (seat additions —
  D1-engine-asking-contract, D8-checkpoint-shows-landing, the fail default) · S8
  (D7-direction-then-pieces, the design seat's candidate pieces) · S4 (D2-iterative-becomes-engine
  as amended — floors reaching setup) · S7 (the no-weakening check and the pointer stub's class).
- Coverage candidates, each put for a path — explore now · rule inline · defer
  (`brainstorm.coverage-survivor-routing`): S6 (the supersession list, item (a) on its own) · S9
  (epic spine items in the stage's agenda).
- The lead's repair batch, one named batch for the user: S3 · S5 · S10 · S11 · S12 · S13 · S14 ·
  S15, plus the text fixes S4 and S6 carry.

### Dispositions — 15/15 (2026-10-11)

| Survivor | Severity | Disposition | Where it landed |
|---|---|---|---|
| S1 | Important | own turn, re-put with the second list's road; user: "as recommended" — (a) split by kind | D6-outside-need-asked statement + changed at review |
| S2 | Important | own turn; user: "as recommended" — (a) split by size, fail narrowed, `review-plan-artifacts` grades meaning kept | D8-checkpoint-shows-landing; D1-engine-asking-contract changed-at-review line; *Implement — the fail* default |
| S3 | Important | repair batch; user: "ok" | *What `analysis-iterative` keeps* default |
| S4 | Important | own turn; user ruled the exposure no problem and widened ("yes" to the lead's reading) | D2-iterative-becomes-engine, D4-shaped-for-six, frame change 3, two defaults |
| S5 | Important | repair batch; user: "ok" | *Slots declared where* default |
| S6 | Important, coverage | ruled inline; user: "as recommended" — the desk's precedence default flipped by the user's word | *Supersessions owed at the build* |
| S7 | Important | own turn; user: "as recommended" — (a) obligation map, pointer `class: floor` | D3-four-move-now statement + changed at review |
| S8 | Important | own turn; user: "as recommended" — (a) the design seat proposes pieces read-only | D7-direction-then-pieces statement + changed at review |
| S9 | Minor, coverage | ruled inline; user: "as recommended" | D5-stage-after-runopen statement + changed at review |
| S10 | Minor | repair batch; user: "ok" | D4-shaped-for-six; frame change 2's destination line |
| S11 | Minor | repair batch; user: "ok" — re-slugged, cites swept | D3-four-move-now, D4-shaped-for-six |
| S12 | Minor | repair batch; user: "ok" | Q1's facts reading |
| S13 | Minor | lead re-read the source cold (survives); repair batch; user: "ok" | D1-engine-asking-contract rejected roads + changed-at-review line |
| S14 | Minor | repair batch; user: "ok" | Run notes disclosure |
| S15 | Minor | repair batch; user: "ok" | *Touch set for the build* |

S3 and S5 were Important in the report; they went to the repair batch as fixes of wording and
mechanism that offered no real choice, each shown to the user by name in the batch (S3 reworded
rows; S5 the off-by-default switch). No survivor challenged a ruling without its own turn except S6
(a), put by name inside the coverage message and ruled by the user.

## Verify pass

### Round 1 (2026-10-11, `reports/review.md` § Verify round 1)

- The review seat resumed for one bounded verify over the changed cards, sections and defaults.
  Result: S1–S15 all CLEAN; round NOT CLEAN on six fold-introduced defects.
- V1 · Important — setup's interrogation and specify's intent stage follow the asking half and
  "name nothing", but the asking half carries a blind second list and per-item provenance, with no
  text saying what a caller that fills no slot gets; the obligation map checks losses, not gains.
  A real choice about setup's and intent's behaviour — put to the user.
- V2 · Minor — D5-stage-after-runopen's "before any design seat's plan-only dispatch" against
  D7-direction-then-pieces's proposal riding that dispatch: reworded (lead repair).
- V3 · Minor — D6-outside-need-asked's agenda line missed the epic spine items
  (D5-stage-after-runopen): amended (lead repair).
- V4 · Minor — frame change 3 left stale text: OQ2, the decision-map out-of-scope line,
  D4-shaped-for-six's rejected road (i): each updated (lead repair).
- V5 · Minor — the supersession list missed the Common Mistakes keep-set; the touch set missed the
  *When NOT to Use* "Implementation details" line (F21): both added (lead repair).
- V6 · Minor — the review report's cold-read section cites the two decisions' old slugs as history:
  no change — a quote keeps its source form (`human-readable-ids` D15-protected-line-rewrites); the
  record itself is swept.
- Nits: the *Sizing and skips* default gained the lane pointer; Q1's trail marks the second list's
  D10 reason as corrected at S13.
- V1 put (2026-10-11): (a) off unless a command asks — no blind list, labels in the conversation;
  (b) brainstorm's settings by default; (c) decide later. Re-put in plain words at the user's ask.
  User: "i dont want to make anything custom for setup and specify , the whole idea is shared skill
  in herited by all workflows and any customisation needed sits in individual workflow commands" —
  the user's own road: no per-caller carve-out; the engine carries one default for every slot,
  inherited by every caller; a command customises only in its own rules. The engine's default
  values put back for one OK: the blind list on a fork costly to undo; provenance marks shown as
  ruled and saved where the caller writes, else in the conversation; the contract off. User: "yes".
  Landed in D1-engine-asking-contract's changed-at-review, the *Slots declared where* and *Setup and
  specify's intent* defaults, and D3-four-move-now's obligation map (gains as well as losses).

### Round 2 (2026-10-11, `reports/review.md` § Verify round 2)

- V1, V3, V4, V5, V6 CLEAN; both nits CLEAN. NOT CLEAN, Minor: V2 — D5-stage-after-runopen's
  statement fixed, its card title still "before any design seat plans". New with the V1 fold, Minor: W1 — the *Slots declared
  where* default's "specify sets once per run" can be read as covering specify's intent stage,
  which inherits the costly-fork default. Seat's recommended status: `ready` once the two one-line
  fixes land (transcription, no ruling). V6 note: `ids --check` on the report shows 0 drift. The
  seat discloses one bare governance ID in its own report (line 144; full form
  `GI-005-record-layer-integrity`), left for the landing's `ids` sweep.
- A second NOT CLEAN verify goes to the user (`brainstorm.fix-one-card-verify-once`): put with the
  two fixes and the after-review screen.

## Acceptance and landing

- After-review screen shown 2026-10-11: what changed per decision (D1–D8), the full supersession
  list, the renames, the touch set, the disclosed blind-list miss; the round-2 result and its two
  one-line fixes; and an ID tidy found by `mochiko-cli ids --check` on the record (7 cross-record
  mentions lacking their session qualifier, bare mentions of this record's own decisions). User:
  **"accept"** — the two fixes applied (D5-stage-after-runopen's title "drafts"; *Slots declared
  where*'s "specify's story stage"), the ID tidy applied (format only, no change of meaning —
  `human-readable-ids` D15-protected-line-rewrites), the record closed seat-unverified by the user's
  word, and the record accepted.
- `mochiko-cli ids --check` on this record after the tidy: 0 drift; the bare mentions left are the
  blind map's angle labels and the second list's road labels, which name no decision.
- Landing (KM close ritual, `.mochiko/memory/knowledge-management.md`): `DECISIONS.md` row
  2026-10-11, four prior rows annotated superseded in part (`architecture-brainstorm-interview`,
  `specify-brainstorm-discovery`, `brainstorm-target-state`, the skill-schema wave-2 doors row) ·
  `BACKLOG.md`: build item C added to *Architecture-desk brainstorm build* (one combined build with
  items A and B), the *Engine follow-up session* item added, the *Three-command convergence session*
  narrowed, the *Shared thinking-stage design session* item moved to the trail · `ROADMAP.md`: folded
  into the Plan-surface builds Next row (caps held) · the index entry updated and the four sessions'
  entries and records annotated.
- Seats closed: `review-seat` (blind map, cold read, two verify rounds) and `second-list-engine` are
  done; no seat holds a pen.
