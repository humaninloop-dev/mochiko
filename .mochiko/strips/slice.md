# Strip notes — `commands/slice.md`

Entry formats: `strips/README.md`. Wave context: the slice cluster wave (BACKLOG item 7, the
second one-shot-command wave after specify's at v0.13.0). The wave also ran the **D2 conversion
assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint** against
slice's needs (no new shape gap — shape stays v2).

## [v0.14.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** slice runs a producer↔reviewer cycle (≤3
  rounds, gap-list-driven revision, cold reviewer) whose context-retention bet is slice's own:
  a **standing producer seat** holds (1) cross-round **extend-obligation** placements so a targeted
  revision doesn't silently drop an obligation placed earlier, (2) the **depth reasoning across a
  wrong-depth flip** (decompose ↔ whole-spec — an outcome shift specify has no analogue for), and
  (3) **foundation + Feature-Done coherence** across a targeted fix. The reviewer maps to specify's
  critic seat (cold first review, same-seat after, no producer contact). Transport rides the v3 fix
  (`agent-dispatch.md` Seat transport + addressability probe).
- **Steelman recorded:** zero successful team-form runs at conversion time (the slice-scoped
  pipeline is undogfooded; specify's own checkpoint has not fired). Slice's inputs are unusually
  cheap-and-complete on disk (`spec.md` is the whole source; `slices.md` is a small ID overlay), so
  a one-shot producer reconstructs context more cheaply here than in specify — the retention payoff
  is *smaller* for slice. Slice is single-reviewer (nearer the surgical/one-shot pole) and a
  null-exit round writes no file. Ruled team-form anyway per D2's declared default + S4.
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:slice`" BACKLOG item, Slice-build follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here.

## [v0.14.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor
  all four requirements (default-FAIL done-condition, independent validation, bounded iteration,
  named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not
  restated here…" — restated loop-discipline's own enumeration.

## [v0.14.0] Per-run contract fill (`workflow-contract.md` → `slice-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the
  authoring-time-fill rule); the per-workflow values survive as the command's Contract section
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/slice-contract.md`
  with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is
  the inspectable proof — not this command body."

## [v0.14.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-slices`' description + REGISTRY's split note)
- **Tier failed:** 1
- **Content:** stated three times pre-wave — L8 ("The reviewer *recommends* a status; **you own the
  clearing verdict** — its status is input, never the gate"), the Contract Team clause, and the
  footer ("the reviewer grades from the files, you Read `spec.md` + the artifacts and decide … its
  status is input").

## [v0.14.0] Slicing-judgment enumeration in the goal-framing
- **Disposition:** relocated → `mochiko:authoring-slices` (single-sources the slicing judgment); the
  converted goal names the deliverable's parts once (the specify pattern)
- **Tier failed:** 1
- **Content:** "(the story→slice judgment: dependency-closed ordering, foundation designation,
  extend obligations, Feature-Done declaration)"

## [v0.14.0] Review-checks enumeration in the review step
- **Disposition:** relocated → `mochiko:review-slices` (single-sources the 13 checks); the command
  briefs the skill by name + the deliverable + the null-exit-round special
- **Tier failed:** 1
- **Content:** "the `review-slices` checks — story coverage, dependency closure, foundation
  legitimacy, sizing, extend-obligation visibility, Feature-Done SC coverage + seams, spec stamp,
  and the depth second-guess in both directions"

## [v0.14.0] Null-exit criterion parenthetical
- **Disposition:** relocated → `mochiko:authoring-slices` (null-exit criterion) + `slices-template`
  usage note 6; the command keeps only the loop-mechanics it owns
- **Tier failed:** 1
- **Content:** "(fewer than two distinct value seams)"

## [v0.14.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task
  tool (never inline agent behavior); do not modify git or push."

## [v0.14.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`)"

## [v0.14.0] "Why this workflow is net-new" blockquote
- **Disposition:** relocated → `.mochiko/brainstorms/vertical-graduation/synthesis.md` (Problem
  Statement) + `authoring-slices` Overview + REGISTRY row 44; the design-record pointer folds into
  the converted goal
- **Tier failed:** 2 (motivational provenance, three existing homes — the shape of specify's deleted
  HIL-comparison blockquote)
- **Content:** "> Why this workflow is net-new (no HIL ancestor): the pipeline's unit was the whole
  feature — every story crossed each stage together, so P1 stories could not reach verified code
  until the entire spec was planned and tasked, and whole-spec artifacts diluted producer and
  reviewer attention. This command creates the smaller unit; the downstream commands' slice-scoped
  entries consume it. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md`."

## [v0.14.0] KEPT: "No G2 — slice is single-reviewer, so plan's feasibility-rejection slot is intentionally unused."
- **Tier-2 evidence:** prevents a false-defect reading of the gate sequence — without it, an
  auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate. The note records the
  deliberate single-reviewer structure (no feasibility-rejection slot, unlike plan). Provenance:
  slice is net-new single-reviewer by design (REGISTRY row 44; `vertical-graduation/synthesis.md`).
