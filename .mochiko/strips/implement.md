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
entries below. **Also stale:** the shape is **v5** as of the v0.35.0 wave below, and the
"standing producer / standing verifier" claim is superseded by that wave's seat-recycling binding.

<!-- Wave context: the command-content-schema build wave (v0.92.0) — `commands/implement.md`
splits into a narrative `.md` (Identity & Mission · the obligated schema read · Adaptive Goal
Protocol) and `plugins/mochiko/schemas/implement.yaml` (mint-once rules at D12 grain;
labels from `command-labels.yaml`); source text = the approved simplified rewrite,
`.mochiko/brainstorms/command-content-schema/implement-rewrite.md` (build step 0). Ruling for
every [v0.92.0] entry below: `.mochiko/brainstorms/command-content-schema/record.md` (D2 · D6 ·
D7 · D9 · D12 · build item 4's M4 verbatim rule) → `DECISIONS.md` 2026-08-26
command-content-schema row. Every Content field quotes the SHIPPED v0.91.0 text — what actually
left the file (the GI-006 referent); the rewrite's replacement wording lives at the named new
homes, never restated here. -->

<!-- Wave context: the D14 section-nesting amendment (v0.93.0) — `implement.yaml`'s flat
`rules:` list reflows into six first-class `sections:` nodes ({id, title, intent, rules}),
all 104 rule IDs and texts carried unchanged (pure relocation, D11 continuity trivial; checker
stats identical pre/post: rules 104 · floor 34 · must 69 · advisory 1 · fail-condition 15).
Ruling for every [v0.93.0] entry: record D14 (post-build amendment, user-ruled 2026-08-26) →
`DECISIONS.md` 2026-08-26 command-content-schema row. -->

<!-- Wave context: the D15 referential-closure amendment (v0.94.0) — rule texts must be
self-contained (every reference resolves within the block or the schema's addressable
namespace); the checker gains a curated deixis lint (warning-class). One live instance
reworded, ID kept per D11. Ruling: record D15 (post-build amendment 2, user-directed
2026-08-26) → `DECISIONS.md` 2026-08-26 command-content-schema row. -->

## [v0.94.0] `impl.staffing-latitude` text — deixis reworded, document-shape remark dropped (D15)

- **Disposition:** superseded → the same rule block, ID kept (D11 reword): "Beyond this
  schema's `class: floor` rules, everything is your per-run judgment: how you staff, sequence,
  and run the cycles; teammates or subagents per seat is your call."
- **Tier failed:** n/a — supersession by ruling (record D15; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, the v0.93.0 shipped text — "There is no Bindings section. Beyond the
  floor these rules state, everything is your per-run judgment: how you staff, sequence, and
  run the cycles; teammates or subagents per seat is your call."
- **Kept deliberately:** the obligation survives whole — floor-bounded per-run latitude over
  staffing, sequencing, and cycle execution; "the floor these rules state" becomes the
  addressable "this schema's `class: floor` rules". The dropped sentence "There is no Bindings
  section." dies without relocation: a document-shape remark, not an obligation — the schema's
  existence states it, and D15 classes such remarks as deixis.
- **Consumers assessed:** no other rule, the `.md`, or any strip cites the old wording; the
  charter audit keys on the label set, ID continuity, and floor survival — all unaffected (ID
  kept, class unchanged, labels unchanged). The checker's D15 lint verified firing on the old
  text and silent on the new.

## [v0.93.0] Schema flat grammar — `rules:` key, grammar header line, six comment dividers (D14)

- **Disposition:** superseded → the `sections:` grammar: the top-level `rules:` key becomes six
  `sections:` nodes (`impl.sec.roles` · `impl.sec.reserved` · `impl.sec.tools` ·
  `impl.sec.ways-of-working` · `impl.sec.boundaries` · `impl.sec.fail-conditions`); each
  divider's wording survives verbatim as its section's `title:` (the two-line FAIL divider's
  parenthetical as the `intent:` line); the grammar header line is reworded "as amended by D14".
- **Tier failed:** n/a — supersession by ruling (record D14; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, what left the file —
  the grammar header line: "# Grammar (D6): rule blocks {id, labels, class, text, ruling?,
  pointer?} under vars:." · the top-level key: "rules:" · the six dividers:
  "# ── Roles & Responsibilities — seat wiring ──" · "# ── Reserved to the user — never the
  run's ──" · "# ── Tools bindings ──" · "# ── Ways of Working ──" · "# ── Boundaries — the
  non-waivable floor ──" · "# ── Not done — default FAIL (the fail-condition set; any one
  standing fails / the run; the .md Not-done line hard-codes this set's count) ──" (rule
  padding of ─ characters elided; no other content in the lines).
- **Kept deliberately:** every rule block byte-identical under its section (+4-space indent
  only — block scalars are indentation-relative, parsed texts unchanged); all 104 IDs, labels,
  classes, rulings, pointers untouched; the remaining grammar-header lines (id/labels/class/
  text/ruling/pointer) survive with a section-id clause added.
- **Consumers assessed:** the checker reworked the same wave (section grammar asserted,
  per-section stats, flat `rules:` now a finding); the `.md` Rules section re-pointed at the
  six section IDs (own entry below); the charter-audit criteria in
  `.claude/rules/mochiko/primitive-edits.md` key on the `fail-condition` label set and
  `impl.*` ID continuity — both unaffected by relocation; D11 extends to `impl.sec.*` IDs.

## [v0.93.0] `.md` Rules-section source-of-truth sentence — re-pointed at section IDs (D14)

- **Disposition:** superseded → the same paragraph in `commands/implement.md`, now enumerating
  the six `impl.sec.*` section IDs as the rule map; the Not-done line gains the address
  "(section `impl.sec.fail-conditions`)", its label-keyed count unchanged.
- **Tier failed:** n/a — supersession by ruling (record D14; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "It is the source of truth for this
  run's binding rules — seat wiring and independence, decisions reserved to the user, ways of
  working, tool bindings, and the non-waivable Boundaries floor."
- **Kept deliberately:** every named responsibility survives as a section gloss in the new
  enumeration; the surrounding obligations (first-action raw Read, `${var}` substitution,
  pointer binding, read-whole gate) untouched.
- **Consumers assessed:** the charter audit's Not-done count guard reads the same label-keyed
  sentence, still present; checker's `NOT_DONE_RE` verified matching post-edit (PASS run).

## [v0.92.0] Command `description:` — reworded by the approved rewrite (D7)

- **Disposition:** superseded → the referent's description: same output claims (working,
  verified code, TDD-built, independently verified against real infrastructure), the batch now
  "one selected capability-batch", the run named by its three stages (sufficiency check at
  entry · design phase for any gaps it finds · TDD cycle cards).
- **Tier failed:** n/a — supersession by ruling (record D7 — source text is the approved
  simplified rewrite; `DECISIONS.md` 2026-08-26 row). Not in the referent's own deletion
  ledger; recorded on the build lead's plan-approval ruling (flag 1).
- **Content:** verbatim — "Turn one capability-batch carrying ratified scope into working code —
  a sufficiency check at entry, a design phase where it finds gaps, then cycle cards built TDD
  and independently verified against real infrastructure."
- **Kept deliberately:** every substantive term survives reworded — the entry sufficiency
  check, the conditional design phase, the TDD build, independent real-infrastructure
  verification; "ratified scope" moves from the description into the Entry step's own gate
  wording.
- **Consumers assessed:** commands carry no per-primitive character budget (user ruling, both
  budget waves), so the char-budget pre-assert does not fire; `disable-model-invocation: true`
  is unchanged, the description maintainer- and router-facing only. Cross-file re-points (the
  router's command row) ride the wave lead's re-point set, not this entry.

## [v0.92.0] `## Roles & Responsibilities` — the whole section moves to the schema (D2/D7)

- **Disposition:** superseded → `plugins/mochiko/schemas/implement.yaml`: the DM floor and seat
  wiring as the seat-wiring rules (`impl.staffing-latitude` … `impl.seat-gap-finder-blind`)
  and the user's reserved set as the reserved-to-user rules (`impl.gate-design-checkpoint` ·
  `impl.gate-card-confirm` · `impl.gate-final-acceptance` · `impl.user-runopen-rulings` ·
  `impl.infeasible-card-escalation` · `impl.adopt-first-user-call` ·
  `impl.ambiguity-escalation` · `impl.scope-escalation-fail`), text per the referent's R&R
  section at D12 grain.
- **Tier failed:** n/a — supersession by ruling (record D2 — rules move to the schema,
  narrative stays; D7 — R&R seat wiring + reserved-to-user items are stage-1 scope;
  `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Roles & Responsibilities

There is **no Bindings section**. The bare minimum that must always happen is carried here as
the Delivery Manager's owned responsibilities; everything beyond it is your per-run judgment —
how you staff, sequence, and run the cycles is yours to shape; teammates or subagents per seat
is your call.

**You, the Delivery Manager — the always-happens floor:**

- Gate entry honestly, run the sufficiency check through a non-author seat, and open the run
  with its contract stated (protocol).
- Fire the design phase on any gap — and again mid-run, scoped to the discovery, when a builder
  hits undesigned structure (Tools).
- Surface rounds consumed and seats spawned to the user at each checkpoint.
- Batch reserved-to-user questions to the cycle checkpoint (Ways of Working); never sit on a
  build-blocking one.
- Execute the acceptance landing whole at user acceptance (Tools).
- Close the run with a verdict against the done condition.

**Other seats:**

- **The sufficiency seat** — grades the entry check: a seat that authored none of the spec, the
  architecture store, or the product baselines it grades from, and never a seat that will design
  or build this batch. Exempt from plan approval like any grading seat.
- **Design seats (producing, when the phase fires)** — staffing is your call:
  `technical-analyst` for the design deltas, `principal-architect` for a store delta,
  `qa-engineer` for the `**TEST:**` cases; `staff-engineer` stays the builder and never designs
  its own gaps. They author exactly the gaps the check named and nothing else (Tools). Each
  plans first and works only on a plan you approved.
- **The card-authoring seat** — a technical-analyst-class design seat, never the builder who
  will execute the cards; QA authors the `**TEST:**` cases within the slicing that seat sets.
- **Builders (producing seats)** — decompose each card into concrete tasks at build time, the
  decomposition disclosed in the cycle report, and build test-first; craft in Tools.
- **Verification seats** — never the implementer: implementation and verification are never
  the same seat. Verification executes against real infrastructure and reads the code and
  its evidence — per-cycle grading, the whole-implementation final validation, and the
  per-cycle code-minimalism lens (Tools). The same independence covers this run's design-time
  grades: the cycle-card review before the card confirm — its grade covering buildability, and
  an infeasible judgment escalating to the user as a business-level scope decision — and the
  judgment content of any build-time `baseline-delta.md` entry before the user's acceptance
  (Tools). The landing verification seat is scope-extended to
  the graded folds; lane runs add the map-delta boundary check (the accepted work made no
  map write beyond the marked delta) to the same seat.
- **The gap-finding seat** — a fresh `devils-advocate`, dispatched blind per run: never the
  seat that built these cycles, and never one that saw this feature's design-time test cases.
  It hunts what the builder and the test author both missed (Tools); the mutation lens rides
  the existing verification seat, which already holds code sight.
- **The user** — the sufficiency verdict's routing at run-open: each store trip the check raised
  (ruled here, or deferred on the record), each in-flight conflict it raised, and any disputed
  clause the grader could not clear · the **design checkpoint** — the design and its store delta
  signed, blocking, before the first cycle · the **card confirm** — the slicing ruled, blocking,
  before build · an infeasible card judgment, escalated as a business-level scope decision · a
  commodity-category adopt-first ruling or an `IP-XXX` provisioning call halted out of a cycle
  (Tools) · architecture-deviation consent: a cycle that adds or removes a box, adds,
  removes, or redirects an arrow, or moves a responsibility across a boundary of the store delta
  signed this run stops and is presented — build as approved, or amend the delta by the user's
  ruling first · requirement ambiguity or a judgment call a producer flags — answered by the user,
  investigable gaps excepted · scope escalation (work bigger than the run was framed; the
  run stays FAIL unless the user explicitly accepts) · exempting a grading round from the
  attempt count (Boundaries) · an epic member's attempt-exhaustion disposition — carve the
  member out or hold the whole run (Boundaries; never the lead's) · a disputed gap-finding
  kind, and each beyond-spec gap finding's disposition — fix now, book to `BACKLOG.md`, or
  accept as designed (Tools) · gap-rework bound exhaustion or a no-progress gap-rework round
  (Boundaries) · final acceptance (accept /
  amend / reject).
```

- **Kept deliberately:** nothing of the section remains in the `.md` — the schema carries it
  whole; the `.md`'s Rules section names "seat wiring and independence, decisions reserved to
  the user" as schema territory, and the obligated raw Read delivers it. Two in-section cuts
  the rewrite made before the move have their own entries below (the deviation-gate grammar
  single-homing; the Tools-side restatement pointers). One further referent compression — the
  gap-finding seat's "hunts what the builder and the test author both missed" — takes no
  entry: the shipped wording is preserved verbatim in this entry's Content block above.
- **Consumers assessed:** the charter-form audit re-keys to the `.md` + schema pair in the same
  wave (`.claude/rules/mochiko/primitive-edits.md` — pair grading, D9); no skill or template
  addresses this command's section headings (the [v0.69.0] entry's grep finding, unchanged
  since).

## [v0.92.0] `## Tools` — the whole section moves to the schema (D2/D7)

- **Disposition:** superseded → the tool-binding rules in
  `plugins/mochiko/schemas/implement.yaml` (`impl.tools-referenced-never-restated` through
  `impl.register`), text per the referent's Tools section at D12 grain; skill-owned floors ride
  as `pointer:` rules.
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — Tools bindings are stage-1
  scope; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Tools

Each tool below is referenced, never restated — its procedure lives in its home.

- **Sufficiency check** — the entry instrument, procedure in `mochiko:review-sufficiency`: it
  owns the clause set, the per-row / per-card scope split, the gap forms, the absent-baseline
  branch, and the trip-versus-gap distinction. What binds here: the grading seat authored none
  of the sources (Roles & Responsibilities); the verdict is, per row, *sufficient* or a gap
  list; any gap fires the design phase over exactly those gaps and nothing else; a disputed
  clause defaults to gap and the dispute goes to the user — the grader never clears alone. The
  verdict lands as **`sufficiency-report.md`** in the feature dir (Reports) and is the run's
  durable assessment record: it carries the store-consult result and any no-delta claim, the
  trips for the user's disposition at run-open, the `quickstart.md` null path where no real
  external-integration surface exists, and any `[MODIFY]` amendment the check named against a
  delivered feature.
- **Design phase** (conditional — fires on any gap) — design seats author **only the named
  gaps**, rung-justified per `mochiko:patterns-plan-minimalism`; outputs land at
  `.mochiko/features/FEAT-XXX/` as deltas mirroring their baselines (`data-model.md`,
  `contracts/`; a delta against a prose baseline in appliable before/after form), plus the
  **store delta** where the structural trigger fired — its structure and scope bound
  `mochiko:patterns-system-design`'s, its grammar and lifecycle
  `mochiko:authoring-architecture-store`'s. A non-author seat grades the output —
  `mochiko:review-plan-artifacts` for conformance to the gap list and card quality (blocking),
  `mochiko:review-feasibility` for buildability and cross-artifact contradiction — and then the
  **design checkpoint** follows: plain blocking text where the user signs the design and the
  store delta before the first cycle starts (Boundaries). The user may stop there and resume the
  build later.
  **Absent baselines:** where the check graded a baseline absent, the phase's first duty is the
  seed — empty scaffolds stating so where no code is delivered; reconstructed from delivered
  code and **confirmed with the user at the design checkpoint** where it exists. The seed is the
  baseline write; this feature's design still lands as deltas, never merged into the seed.
  **Map-entry hardening:** the phase asserts the design-implied dependency relations and the
  sharpened extent onto the capability's map entry with provenance, and fills the entry-side
  architecture link when a store delta is produced (`mochiko:authoring-feature-map`); status
  stays as the scope source set it, and intended-vs-designed drift surfaces to the user at the
  checkpoint. **Where the check's delivered-feature clause fired**, the same phase writes the
  `[MODIFY]` marked delta on the **affected delivered feature's** map entry — the amendment the
  sufficiency report named, in that skill's delta grammar — so the entry carrying the break is
  the entry that records it.
  **Mid-run re-fire:** a builder hitting undesigned structure mid-cycle halts that cycle and the
  phase re-fires **scoped to the discovery** — same grade, same checkpoint; the
  architecture-deviation gate anchors to the signed delta once one exists.
  **Over an epic the phase always fires**, for the joint spine at `.mochiko/epics/EPIC-XXX/` —
  the joint design plan, the joint architecture and seam design with every cross-member seam
  owner **named at design time** (no later-lander default inside an epic), batch ordering, and
  any **shared-baseline delta** authored once in the spine under a single pen-holder (a
  single-member baseline keeps its per-feature delta); every spine artifact is a deliverable
  under `templates/artifact-format.md`, and every epic shared-write surface is governed by the
  transport floor (Boundaries). Shape, mint, and close: `mochiko:authoring-epic`.
- **Card authoring + the card confirm** — after the design phase, or directly on a zero-gap
  verdict. `tasks.md` holds **cycle cards** from the tasks template (rendered by
  `mochiko-cli template tasks`, or its schema `plugins/mochiko/schemas/tasks.yaml` Read raw when
  the binary is absent — the shipped schema is the first-class source of truth) per
  `mochiko:patterns-vertical-tdd`, which owns the slicing judgment and the `**TEST:**` grammar:
  per card, stories + feature rationale, dependencies, acceptance criteria by ID, a `**TEST:**`
  real-infrastructure gate, cycle-level brownfield exposure — no task lists, no file paths, the
  builder decomposes at build time. Where the spec carries a Screens & Flows manifest, each
  UX-bearing card's `**TEST:**` gate names the FLOW-XXX paths it verifies. The authoring seat is
  never the executing builder, and QA authors the cases within its slicing (Roles &
  Responsibilities). **On the zero-gap path** the card-authoring seat also performs the
  map-entry dependency and extent assertion the design phase would have made, surfacing
  intended-vs-designed drift at the confirm. The verification seat then reviews the cards — card
  quality per `mochiko:review-plan-artifacts`, buildability its own judgment — and the **card
  confirm** follows: its own blocking checkpoint where the user rules the slicing before build.
- **Craft skills** — card decomposition + TDD via `mochiko:executing-tdd-cycle` (its
  `cycle-report.md` format — the disclosed decomposition, honest difficulties, deviations,
  `domain_deps_added` — is the uncertainty carrier; brownfield touches ride
  `mochiko:brownfield-integration`; the pre-code ladder rides
  `mochiko:patterns-code-minimalism` at decomposition, rungs disclosed) · verification via
  `mochiko:testing-end-user` — evidence captured, never assumed — plus the per-cycle
  code-minimalism lens via `mochiko:review-code-minimalism`: the verification seat reads
  the cycle's diff, `cycle-report.md`, and the codebase around the diff (reuse claims
  never on trust); `minimalism:` findings are advisory to the checkpoint verdict, never a
  cycle-failing gate.
- **Design inputs** — **`sufficiency-report.md`** and, where the design phase ran, its deltas —
  including the **signed store delta**, the anchor for the deviation check and the
  built-vs-signed diff — at `.mochiko/features/FEAT-XXX/`; the product baselines at
  `.mochiko/product/` — `data-model.md`, `contracts/`, `constraints-and-decisions.md`, and the
  architecture store, whose concern rows carry the `NFR-XXX` numeric quality targets the built
  code must respect — and `spec.md` for
  the cards' cited acceptance criteria.
- **Progress surface** — `tasks.md`'s per-card checkboxes, flipped as cycles complete.
- **Reports** — land in `.mochiko/features/FEAT-XXX/` (product-lane runs:
  `.mochiko/product/lane-<slug>/`): `sufficiency-report.md`, cycle reports, verification reports,
  the final-validation report, the built-vs-signed diff report. Every one is a **report** under
  `templates/report-format.md` (machine-first frontmatter, `ultra` register, clean =
  frontmatter-only; you bounce an envelope-breaking report per its rule 9), and each producing
  seat's brief names the envelope path. Repeat runs append (dated);
  delta files overwrite only via the graded fold.
- **Regression scope** — quality gates run the full repository suite; the final validation
  additionally executes the accumulated `**TEST:**` gates of previously delivered features
  in this feature's territory — the union of those features' durable gate sets at
  `.mochiko/features/FEAT-XXX/gates.md` and the cases on their cards — and this feature's
  gates exercise any seam whose earlier side
  is already delivered — seam ownership sits with the later-landing feature, per
  `mochiko:authoring-feature-map`. Over an epic, the accumulated `**TEST:**` gates run once
  over the **union** of the members' territories. **This sweep's reach explicitly covers a delta
  fix that breaks a *different* delivered feature without being structural** — the territory
  gates are what catch it, and a failure there fails the run like any other delivered-feature
  regression.
- **Cold verification** — the final validation builds and runs the quality gates from a
  dependency-cold snapshot of the uncommitted working state
  (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to
  `.claude/worktrees/mochiko-<purpose>/`), its results part of the acceptance evidence;
  ensure the `/.claude/worktrees` ignore entry exists first. Over an epic, one cold snapshot
  covers all members.
- **Gap-finding pass** — the final validation's discovery layer, procedure in
  `mochiko:testing-gap-finding`, referenced never restated. It runs on **selection-scope and
  epic runs only**; a delta-scope or product-lane run skips it and the final-validation report
  **states the skip explicitly**, never a silent no-op. Over an epic it runs once, over the
  union of member territories. **Dispatch is two-message and blind:** the first message to the
  fresh gap-finding seat (Roles & Responsibilities) carries only the feature's `spec.md`,
  **`sufficiency-report.md` and the design-phase deltas** where they exist, and Screens & Flows, plus
  the product baselines `data-model.md`,
  `contracts/`, and the store's concern rows carrying the `NFR-XXX` targets — never the code,
  `tasks.md`, the `**TEST:**` cases, the cycle
  reports, or the verification reports; the seat states its derived expectations, and only then
  does probing begin. The seat's brief carries the model-tiering routing rule (Ways of Working),
  and its delegated reads stay inside that same fence. Alongside it, the **mutation lens** runs
  on the verification seat, at **high depth only**; its skips are disclosed per the skill, so a
  run at high depth owes either mutation results or a stated skip. **Findings split by kind:** a
  finding demonstrating spec-required behavior broken — evidence captured, the spec clause cited
  — fails the final validation; a beyond-spec finding is advisory to the checkpoint. You confirm
  each finding's kind at the checkpoint verdict against the cited clause; a disputed kind
  defaults advisory and the dispute goes to the user (Roles & Responsibilities) — the finder
  never gates alone. A gap surfaced in a previously delivered feature's territory is not this
  run's rework: it routes to a `/mochiko:feature` delta card, cited in the report.
- **Store landing** — a built structural change folds into the architecture store per
  `mochiko:authoring-architecture-store`, in three parts: the delta's elements **flip
  `built`** and their `FEAT-XXX` keys clear (transcription — it rides this run's landing
  audit); the touched rows' `As-built:` and `Drift:` fields are **written as judgment and
  independently graded** like any other governing-surface write (Ways of Working); and the
  **orphan check** runs — an in-flight-class element keying no open feature is flagged, never
  left. The store skill regenerates the derived root `ARCHITECTURE.md` from the result; the
  index is never hand-edited here. Where `.mochiko/memory/knowledge-management.md` exists, the
  same landing carries its KM obligations.
- **Baseline touches** — mid-fix discovery that the work touches a product baseline → the
  dispatched run authors `baseline-delta.md` in its feature dir at discovery — a minimal
  enumerated delta in appliable form. **A build-time technical decision is written the same way
  and never in place:** a `D-XXX`, `C-XXX`, or `IP-XXX` row discovered at decomposition is
  authored as a `baseline-delta.md` entry against `constraints-and-decisions.md`, and its
  judgment content is graded **as judgment** by the landing verification seat — an independent
  non-author grade before the user's acceptance, the `As-built:`/`Drift:` pattern — while the
  landing's three-way diff stays the transcription check of faithful application only. **Two
  calls are never the builder's:** a commodity-category adopt-first ruling and an `IP-XXX`
  provisioning call each halt the cycle to the user's checkpoint, where
  `mochiko:patterns-adopt-first`'s constraint-challenge keeps its firing site (Boundaries).
- **Acceptance landing** — at user acceptance, one landing executes whole, branched by scope
  type. **Selection scope** — the same landing that executes the store landing above executes
  the map's graduation batch per `mochiko:authoring-feature-map`: this run's delivered work rows
  fold into the capability's extent lines and the rows vanish (pending rows persist) · the
  capability's status is set `delivered` (dated), never regressing · the
  `FEATURES.md` index line updates ·
  the specs-index row is touched — the spec reads closed exactly when all its selected
  work rows have folded (derived, never asserted). No separate feature-close stage
  exists. **Epic** — one landing executes **each member's** graduation batch (as above) plus
  the **epic close** per `mochiko:authoring-epic`: the `[EPIC-XXX]` row markers vanish, the
  manifest is stamped delivered (dated), the spine directory persists as record; every touched
  baseline still folds exactly once — a **shared-baseline delta folds once from the spine**, a
  single-member baseline from its feature delta — each via the graded three-way diff below.
  Multi-spec closure is compositional: each spec reads closed exactly when all **its own**
  selected work rows have folded, however many specs one epic landing touches.
  **Delta scope** — the entry's marked delta folds per `mochiko:authoring-feature-map`'s
  delta fold. **Both scopes:** every touched baseline folds via a graded fold — three-way
  diff: pre-fold baseline + delta vs folded result; delta applied whole, nothing else
  changed — checked by the landing verification seat (Roles & Responsibilities). **One carve:
  the store's fold IS the Store landing above** — status flips, graded `As-built:`/`Drift:`
  writes, and the orphan check, not a three-way diff; it is folded exactly once, like any
  other touched baseline. A delta
  whose baseline file is absent at fold time folds into a fresh `.mochiko/product/` file
  (empty pre-fold side), the absence surfaced to the user as a seeding gap. The same landing
  folds back the gap findings the user ruled fix-now or backlog: each is authored — QA craft,
  in the `**TEST:**` grammar it already owns — into `.mochiko/features/FEAT-XXX/gates.md`,
  minted there if absent, so it rides the territory accumulation at every later final
  validation (`mochiko:testing-gap-finding`). Findings the user accepted as designed do not
  fold.
- **Register** — user-facing prose per `templates/output-style.md`.
```

- **Kept deliberately:** nothing of the section remains in the `.md` beyond the Rules section's
  "tool bindings" naming; the referenced-never-restated posture survives as the opener rule
  `impl.tools-referenced-never-restated`. In-section cuts the rewrite made before the move
  (restatement pointers, the territory-accumulation rationale clause, the repeated
  referenced-never-restated tags) have their own entries below. Three further referent
  compressions take no separate entries — the sufficiency bullet's "the entry instrument" /
  "durable assessment record" phrasings, preserved verbatim in this entry's Content block;
  and the design checkpoint's "plain blocking text" form plus the epic seam-owner "(no
  later-lander default inside an epic)" parenthetical, restored schema-side at
  `impl.acceptance-plain-text` and `impl.epic-seam-owners` respectively.
- **Consumers assessed:** every pointed-at skill and template is untouched — the section
  referenced, never restated, so its move re-homes pointers without changing any procedure's
  owner; the tasks-template render path (`mochiko-cli template tasks` / `tasks.yaml` raw Read)
  is carried into the schema unchanged.

## [v0.92.0] `## Ways of Working` — the whole section moves to the schema (D2/D7)

- **Disposition:** superseded → the ways-of-working rules in
  `plugins/mochiko/schemas/implement.yaml` (`impl.author-grader-default-fail` …
  `impl.acceptance-plain-text`), text per the referent's Ways of Working section at D12 grain.
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — Ways of Working is stage-1
  scope; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Ways of Working

- **Author ≠ grader** — no output is cleared by its author, default FAIL. Any seat that
  writes code or artifacts plans first and works only on a plan you approved; grading,
  verification, and fact-finding seats are exempt.
- **Escalation cadence** — reserved-to-user questions accumulate and land as one batch at the
  cycle checkpoint; only a question the build cannot proceed without interrupts mid-cycle.
  Advisory verifier findings ride the same rule — a Minor advisory finding defaults to a
  `BACKLOG.md` booking, never an in-cycle fix; an Important-or-above advisory finding blocks
  the cycle and enters the checkpoint batch.
- **Model tiering** — exploration and fact-finding dispatches ride the class-keyed tiering
  floor: locate/enumerate reads go to a native `Explore` subagent spawned `model: haiku`,
  interpretive or absence-driven reads stay session tier, and every seat brief carries the
  routing rule. Class key, dispatch ladder, and brief obligation:
  `mochiko:patterns-model-tiering`, referenced never restated.
- **Delta re-verification** — re-verification is scoped to the delta: a test-only or
  records-only change gets a delta-grade of the changed surface, never a full gate re-sweep;
  a delta round re-runs no quality gates, the prior gate evidence standing while the graded
  head is unmoved — and the graded object is the code tree (`git rev-parse
  HEAD:<code-dir>`), so a records-only commit does not move the graded head.
- **Commits and acceptance** — suggest commits; never run git mutations, never push — an
  ephemeral, self-removed verification snapshot is not a mutation of refs, index, tracked
  content, or history. User acceptance is plain blocking text, never a timed prompt.
```

- **Kept deliberately:** all five bullets survive as rules — none dropped; only the model-tiering
  bullet's trailing "referenced never restated" tag dies (its own entry below).
- **Consumers assessed:** `mochiko:patterns-model-tiering` and `mochiko:patterns-sound-loop`
  untouched; the [v0.79.0] redeclarable-set entry below reads the run-open point, which stays
  in the `.md` protocol — unaffected.

## [v0.92.0] `## Boundaries — the non-waivable floor` — the whole section moves to the schema (D2/D7)

- **Disposition:** superseded → the floor rules in `plugins/mochiko/schemas/implement.yaml`
  (`impl.baselines-never-in-place` … `impl.transport-floor`), each `class: floor` —
  must-survive under the re-keyed charter audit (M3), skill-owned floors as `pointer:` rules;
  text per the referent's Boundaries section at D12 grain (the attempt-economy bullet alone
  yields five blocks, the record's D12 worked example). Second landing site: the "Architecture
  before detail" bullet's sign-half — no code before the user signs — lands at
  `impl.gate-design-checkpoint`, outside the Boundaries floor range.
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — Boundaries is stage-1 scope;
  D6/M3 — `class: floor` gains its audit consumer; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Boundaries — the non-waivable floor

- **Baselines are never edited in place.** Product baselines change only through the landing's
  graded fold — never mid-run. The design phase writes **deltas beside them**, and a build-time
  technical decision takes the same delta path (Tools). **One carve, and only one:** a store
  write at the design checkpoint's user sign-off is legal, and only as in-flight-class delta
  elements. Ruled truth in the store is never edited in place either — the signed delta stands
  beside it and the landing folds it.
- **Architecture before detail.** Where the sufficiency check named gaps, no code is written
  before the user has signed the design phase's output — the store delta especially, signed on
  a rendered diagram plus its named `AX-XXX` row changes (no render surface → present source
  plus the changed-element table, and record it). A later contradiction with the signed delta
  returns to the user for a consented amendment, never designed around silently.
- **Feature work never overrules the constitution.** A governance conflict conforms, or is
  amended/waived through `governance-ledger.md` — the user's ruling.
- **A ratified constraint is never silently overridden.** A commodity-category check colliding
  with one files a constraint-challenge finding — the constraint's text · the real requirement
  it plausibly restates · the candidate it excludes — reserved to the user like any governance
  conflict; only the colliding decision pauses, the run proceeds elsewhere. Shape and trigger:
  `mochiko:patterns-adopt-first`.
- **The attempt economy.** A cycle consumes an **attempt** every time a verification seat
  grades it — whatever the round is called (rework, completion, targeted fix, re-grade);
  default 3 attempts per cycle, redeclarable only at run open. Exempting a round from the
  count is reserved to the user, never lead discretion. Two consecutive rounds with
  unchanged findings is a no-progress stop: halt the cycle, present state. **In an epic**, a
  member that exhausts its attempt bound or hits the no-progress stop halts **member-scoped**;
  the disposition — carve the member out (its rows return to pending, the epic continues) or
  hold the whole run — is **reserved to the user** (never lead discretion), because carve-out
  breaks the one-unit promise.
  **Gap-rework at final validation** is the same economy's analogue at the whole-run scale:
  rework driven by the gap-finding pass carries a **whole-run bound, default 2 rounds**,
  redeclarable only at run open (protocol). A finding that localizes to one cycle's territory
  charges that cycle's remaining attempts instead. Bound exhaustion, or a round whose findings
  are unchanged, halts the run and presents state — the disposition is **reserved to the
  user**.
- **Gates are never severity-triaged.** A failed `**TEST:**` gate or quality gate fails the
  cycle per the done condition; `minimalism:` findings stay advisory at any severity
  (Tools).
- **The lane never widens in place.** A product-lane run discovering it stands on an
  in-flight feature's territory files the finding to that run and aborts.
- **The sound-loop floor.** A judgment-authored write to a governing surface obliges the loop:
  a seat produces on a plan you approved, an independent non-author seat reviews before the
  user's gate, the user rules — this run's shape (the sufficiency grade by a non-author seat,
  design seats on approved plans graded by a non-author before the design checkpoint, card
  authoring split from building and ruled at the card confirm, builders on approved plans,
  verification seats never the implementer, final acceptance) already carries it. Trigger test,
  exemptions, seat wiring, and disclosure: `mochiko:patterns-sound-loop`, referenced never
  restated.
- **The transport floor.** A run that composes more than one seat gains a floor on its
  composition and messaging: a split trigger — message legs on any multi-seat messaging,
  topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs,
  composition-safe shapes, and disclosure: `mochiko:patterns-transport-floor`, referenced
  never restated.
```

- **Kept deliberately:** every floor survives as a `class: floor` rule — the attempt-economy
  defaults (3 per cycle, 2 gap-rework rounds) now valued from the schema's `vars:` block, the
  `.md` run-open line de-literalized to match (build lead's flag-1 ruling, this wave). Three
  in-section cuts the rewrite made before the move have their own entries below (the carve-out
  rationale clause; the sound-loop seat re-list; the repeated referenced-never-restated tags).
  One further referent compression — the attempt definition's "(rework, completion, targeted
  fix, re-grade)" round-name list — takes no entry: the shipped wording is preserved verbatim
  in this entry's Content block above.
- **Consumers assessed:** `mochiko:patterns-sound-loop`, `mochiko:patterns-transport-floor`,
  and `mochiko:patterns-adopt-first` untouched — each floor keeps its pointer at the new home;
  the re-keyed charter audit (primitive-edits.md, same wave) grades floor presence across the
  pair.

## [v0.92.0] The `**Not done — default FAIL**` list — 15 clauses become the `fail-condition` rule set (D7)

- **Disposition:** superseded → the 15 `impl.fail.*` rules labeled `fail-condition` in
  `plugins/mochiko/schemas/implement.yaml`, in the referent's order
  (`impl.fail.sufficiency-unrecorded` … `impl.fail.no-acceptance`); the `.md`'s Not-done line
  re-keys to the count pointer — "the 15 rules labeled `fail-condition` in
  `plugins/mochiko/schemas/implement.yaml`" — N=15 pinned (record N6), the C2 guard, the count
  match checker-verified (D13).
- **Tier failed:** n/a — supersession by ruling (record D7 — the FAIL clauses move labeled
  `fail-condition`, the Not-done line re-keys to the label set; C2 hardening — risk accepted
  eyes-open by the user; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim —

```
**Not done — default FAIL:** an unrecorded sufficiency verdict · gaps present but the design
phase skipped, or its design and store delta unsigned · a cycle card built by the seat that
authored it, or built before the card confirm · an unchecked cycle card · a failing quality
gate · verification without real-infrastructure evidence · a regression in a previously
delivered feature's gates · a build-time baseline write made in place rather than as a
judgment-graded `baseline-delta.md` entry · a surfaced store deviation neither built as
approved nor consented as an amendment · a signed-delta landing without its built-vs-signed
diff, or leaving an in-flight-class element neither flipped `built` nor keyed to an open
feature · a touched baseline accepted without its graded
fold · a selection-scope or epic run without its gap-finding pass · a delta-scope or lane run
whose report does not state the skip · an unresolved spec-violation gap finding · user
acceptance not given.
```

- **Kept deliberately:** all fifteen clauses survive one-for-one as rules — none dropped, none
  merged; the default-FAIL posture survives on the `.md` line itself ("any one standing fails
  the run") plus its out-of-sync halt sentence.
- **Consumers assessed:** the charter-form audit's FAIL-survival criterion (leg iv) re-keys
  from this list's literal text to the `fail-condition` label set in the same wave
  (`.claude/rules/mochiko/primitive-edits.md`); the D13 checker binds the `.md` count to the
  schema set deterministically.

## [v0.92.0] Protocol step 3 — the done-condition detail paragraph compresses to the fixed close (D2/D7)

- **Disposition:** superseded → the `.md`'s compressed `### 3. Done condition — fixed` plus
  the schema rules now carrying the detail: `impl.regression-sweep` (accumulated territory
  gates + seam exercise), `impl.graded-fold`, the landing rules (`impl.landing-verifier-folds` ·
  `impl.landing-selection` · `impl.landing-epic` · `impl.landing-delta`), the diff/orphan
  obligations at `impl.fail.store-landing-incomplete`, and the sufficiency/design/card/gap
  obligations at their `impl.fail.*` counterparts.
- **Tier failed:** n/a — supersession by ruling (record D2/D7; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim (the paragraph's closing epic sentence is the relocation entry below):

```
3. **Run to the done condition.** The sufficiency verdict is recorded as a report; where it
   named gaps, the design phase ran over exactly those gaps and the user signed its design and
   store delta at the **design checkpoint** before the first cycle; the cycle cards were
   authored by a seat that did not build them and ruled by the user at the **card confirm**.
   Every `tasks.md` cycle card is `[x]`; each card was
   decomposed into concrete tasks by its builder at build time — the decomposition disclosed
   in the cycle report, never pre-written — and the built code was implemented test-first
   (red/green/refactor) and independently verified — executed `**TEST:**` gates, quality
   gates with exit codes, captured real-infrastructure evidence — per cycle and once for the
   whole implementation; the feature's verification also ran the **accumulated TEST gates of
   previously delivered features in its territory**, and any seam against an
   earlier-delivered feature was exercised here, against the real delivered side; on a
   selection-scope or epic run the final validation also ran the **blind gap-finding pass**
   (Tools), a delta-scope or product-lane run stating that skip explicitly in its
   final-validation report; the code
   meets its criteria, holds traceability to requirements, and aligns with the project's
   governance; where a store delta was **signed this run — whenever signed**, at the design
   checkpoint or at a mid-run re-fire — a built-vs-signed diff
   report exists — owed on the **signed-delta-existed trigger alone**, so a feature descoped
   to nothing is caught at its landing rather than weeks later by the orphan sweep — and any
   divergence it names was ruled by the user; and the acceptance
   landing executed whole — map bookkeeping, the store landing, and every touched baseline's
   graded fold. The
   run closes at final acceptance (accept / amend / reject).
```

- **Kept deliberately:** the compressed step 3 keeps the paragraph's spine — every card `[x]`,
  test-first build, independent real-infrastructure verification per cycle and whole, criteria
  met, traceability, governance alignment, the acceptance landing executed whole, the close at
  final acceptance (accept / amend / reject), and "And nothing below stands." binding the
  fail-condition set. The descoped-feature rationale clause inside this paragraph leaves with
  no home — its own supersession entry below.
- **Consumers assessed:** the charter audit's fixed-done-condition leg (iii) still reads the
  `.md`; the detail obligations it used to read here are graded through the pair from this
  wave (legs iv–vi, primitive-edits.md).

## [v0.92.0] The descoped-feature rationale clause — deleted, no home (protected-trail survivor leaves by ruling)

- **Disposition:** superseded → deleted; no home carries it. The trigger it explained survives
  whole — the built-vs-signed diff owed on the signed-delta-existed trigger alone, at the
  landing rules and `impl.fail.store-landing-incomplete`.
- **Tier failed:** n/a — supersession by ruling (record D7 — the approved rewrite's deletion
  ledger names this cut; `DECISIONS.md` 2026-08-26 row). Recorded as a supersession, never a
  Tier-1 strip, because the clause is carried verbatim in the [v0.91.0] plan-time store-delta
  anchor entry's **Kept deliberately** field below — protected-trail content leaves only by
  recorded ruling; classing confirmed by the build lead at plan approval (flag 3).
- **Content:** verbatim — "so a feature descoped
  to nothing is caught at its landing rather than weeks later by the orphan sweep".
- **Kept deliberately:** the trigger's character — the diff owed on the delta's existence
  alone, not on the work having stayed in scope — survives as rule text; only the explanatory
  clause dies.
- **Consumers assessed:** none — explanatory prose local to this command; the [v0.91.0] entry
  that carried it reads historically.

## [v0.92.0] Entry's absent-surface handling — relocated to the sufficiency rule's tail

- **Disposition:** relocated → the sufficiency tool rule's absent-surfaces tail in
  `plugins/mochiko/schemas/implement.yaml` (per the referent's Tools sufficiency bullet); the
  `.md` Entry keeps the one-line summary ("Absent surfaces are surfaced to the user, never
  auto-resolved, never run-failing").
- **Tier failed:** 1 (altitude — the Entry step carried tool-level procedure the sufficiency
  bullet owns)
- **Content:** verbatim — "A missing governance region is surfaced, never auto-resolved;
  present → each code-touching brief names the relevant `.claude/rules/mochiko/` files as an
  obligated read. On a brownfield codebase a missing or stale
  `.mochiko/memory/codebase-analysis.md` is surfaced the same way — offer `/mochiko:setup`, or
  proceed greenfield with the warning logged; **a store with no ruled content — scaffold-only
  or absent** — is surfaced the same way too: offer the `/mochiko:architecture` bootstrap,
  never fail the run for it. The check still runs in every case: rows touching an absent
  surface grade per its absent-baseline branch."
- **Consumers assessed:** `mochiko:review-sufficiency` owns the absent-baseline branch itself
  and is untouched — the relocation moves the command-side handling only.

## [v0.92.0] Step 3's epic build shape — relocated beside the mechanisms it restated

- **Disposition:** relocated → the card-authoring rule's epic line plus the epic clauses of
  the cold-verification, regression-scope, and acceptance-landing rules in
  `plugins/mochiko/schemas/implement.yaml` (per the referent's Tools bullets — each member of
  the shape lands beside the mechanism it restated).
- **Tier failed:** 1 (altitude — the done condition restated build mechanics the tool bullets
  own)
- **Content:** verbatim (protocol step 3's closing sentence) — "**Over an epic:** one merged
  **sequential** cycle sequence from the joint design — shared foundation cycles first, then
  in-epic dependency order — with feature-tagged cards whose reports land in each member's
  `.mochiko/features/FEAT-XXX/`; one final validation from one cold snapshot covering all
  members, the accumulated territory `**TEST:**` gates running once over the **union** of
  member territories; one acceptance landing executes each member's graduation batch plus the
  epic close (`mochiko:authoring-epic`)."
- **Consumers assessed:** none — the epic shape's authoritative homes
  (`mochiko:authoring-epic`, the map skill's graduation batch) are untouched; this was the
  command's own restatement.

## [v0.92.0] Epic entry-rule detail — relocated to `mochiko:authoring-epic`, pointer stays

- **Disposition:** relocated → `mochiko:authoring-epic` (the skill owns epic mint and entry
  semantics); the `.md` Entry keeps the compressed rule list + pointer ("Epic entry rules —
  delta cards never join, in-epic dependencies don't block, outside-epic ones do:
  `mochiko:authoring-epic`").
- **Tier failed:** 1 (altitude — entry-rule detail the epic skill owns)
- **Content:** verbatim — "**Epic entry:** `$ARGUMENTS` naming an `EPIC-XXX` resolves to its
  members by lookup — minting is the desk's (`/mochiko:feature`), never declared here. Every
  member gates on ratified selection as selection scope (delta-scope cards never join an
  epic); an in-epic dependency does not block, an outside-epic dependency at a non-`delivered`
  row still blocks."
- **Consumers assessed:** `mochiko:authoring-epic` — flagged to the wave's skill-ripple check
  to confirm the skill states the desk-only mint and the three membership/dependency rules;
  the `.md`'s compressed list still names each rule meanwhile, so no rule is context-lost if
  the skill lags.

## [v0.92.0] Deviation-gate grammar — single-homed at the floor rule

- **Disposition:** relocated → single-homed at the floor rule `impl.deviation-gate` in
  `plugins/mochiko/schemas/implement.yaml`; the reserved-to-user rule keeps the consent
  decision and its two dispositions as a pointer (per the referent — R&R points at Boundaries,
  the grammar stated once).
- **Tier failed:** 2 (duplication — the grammar's second site named no behavior beyond the
  floor rule's)
- **Content:** verbatim (the R&R user bullet's site) — "architecture-deviation consent: a
  cycle that adds or removes a box, adds,
  removes, or redirects an arrow, or moves a responsibility across a boundary of the store delta
  signed this run stops and is presented — build as approved, or amend the delta by the user's
  ruling first"
- **Consumers assessed:** the gate's anchor history rides the [v0.91.0]/[v0.81.0] entries
  below, unaffected; the grammar itself survives byte-equivalent at `impl.deviation-gate`.

## [v0.92.0] R&R restatements inside Tools — reduced to pointers

- **Disposition:** relocated → the seat-wiring rules (`impl.staffing-latitude` …
  `impl.seat-gap-finder-blind`); each tool rule carries a pointer in place of the restatement.
- **Tier failed:** 2 (duplication — restated seat rules Roles & Responsibilities already
  carried)
- **Content:** verbatim, two sites — (i) card authoring: "The authoring seat is
  never the executing builder, and QA authors the cases within its slicing (Roles &
  Responsibilities)."; (ii) gap-finding dispatch: "the first message to the
  fresh gap-finding seat (Roles & Responsibilities) carries only".
- **Consumers assessed:** none — command-local pointers; the seat definitions themselves moved
  whole in the R&R supersession above.

## [v0.92.0] Epic carve-out rationale clause — deleted

- **Disposition:** deleted
- **Tier failed:** 1 (altitude — rationale; the user-reserved disposition it explained
  survives whole in the schema's attempt-economy floor rules)
- **Content:** verbatim (Boundaries, the attempt economy's epic member-halt disposition) —
  ", because carve-out
  breaks the one-unit promise"
- **Consumers assessed:** none — command-local rationale.

## [v0.92.0] Territory-accumulation rationale clause — deleted

- **Disposition:** deleted
- **Tier failed:** 1 (altitude — rationale; the gates fold-back rule and the accumulation
  mechanism survive whole)
- **Content:** verbatim (the Acceptance landing's gap-finding fold-back sentence) — ", so it
  rides the territory accumulation at every later final
  validation"
- **Consumers assessed:** none — the accumulation's homes (`mochiko:testing-gap-finding`, the
  regression-scope rule) are untouched.

## [v0.92.0] Sound-loop bullet's six-item seat re-list — deleted

- **Disposition:** deleted — the schema's `impl.sound-loop-floor` rule points at the
  seat-wiring and independence rules instead of re-listing them (per the referent: "This run's
  seat wiring (Roles & Responsibilities) already carries it end to end").
- **Tier failed:** 2 (duplication — each of the six items restates a seat rule Roles &
  Responsibilities carries)
- **Content:** verbatim — "— this run's shape (the sufficiency grade by a non-author seat,
  design seats on approved plans graded by a non-author before the design checkpoint, card
  authoring split from building and ruled at the card confirm, builders on approved plans,
  verification seats never the implementer, final acceptance) already carries it."
- **Consumers assessed:** `mochiko:patterns-sound-loop` owns the floor and is untouched; each
  of the six seat facts survives as its own seat-wiring/independence rule.

## [v0.92.0] Repeated "referenced never restated" tags — deleted, stated once

- **Disposition:** deleted — the posture is stated once, at the rule
  `impl.tools-referenced-never-restated` (the shipped Tools opener's successor); the three
  trailing tags were repeats.
- **Tier failed:** 2 (duplication)
- **Content:** verbatim, three sites — the closing tag of (i) Ways of Working / Model tiering:
  "Class key, dispatch ladder, and brief obligation:
  `mochiko:patterns-model-tiering`, referenced never restated."; (ii) Boundaries / the
  sound-loop floor: "Trigger test,
  exemptions, seat wiring, and disclosure: `mochiko:patterns-sound-loop`, referenced never
  restated."; (iii) Boundaries / the transport floor: "Trigger test, floor legs,
  composition-safe shapes, and disclosure: `mochiko:patterns-transport-floor`, referenced
  never restated." — each pointer sentence survives; only the ", referenced never restated"
  tag died.
- **Consumers assessed:** none — the pointed-at skills are untouched.

<!-- Wave context: the plan-stage-retirement build wave (v0.91.0) — `/mochiko:plan` retires as a
command and `/mochiko:implement` becomes the pipeline's single downstream run behind an entry
sufficiency check with a conditional in-run design phase. Ruling for every [v0.91.0] entry below:
`.mochiko/brainstorms/plan-stage-utility/record.md` (D1 with mechanics a–e · D2 as amended by the
Addendum's A1–A6 · D3 · D4 · D6 · D7) → `DECISIONS.md` 2026-08-26 plan-stage-retirement row. -->

## [v0.91.0] Gap-finding dispatch fence — the `requirements.md` inclusion-list slot (D3)

- **Disposition:** superseded → the same slot re-keyed to **`sufficiency-report.md` and the
  design-phase deltas** where they exist; both are spec-layer artifacts, never code, so the
  blindness fence is unchanged in kind.
- **Tier failed:** n/a — supersession by ruling (record D3's fence consequence; `DECISIONS.md`
  2026-08-26 row). `requirements.md` ceases to exist as an artifact under D3, so its slot could
  not survive.
- **Content:** verbatim — "the first message to the fresh gap-finding seat (Roles &
  Responsibilities) carries only the feature's `spec.md`, `requirements.md`, and Screens & Flows,
  plus the product baselines `data-model.md`, `contracts/`, and the store's concern rows carrying
  the `NFR-XXX` targets".
- **Kept deliberately:** the whole rest of the fence is byte-for-byte intact — the two-message
  dispatch, the exclusion list (never the code, `tasks.md`, the `**TEST:**` cases, the cycle
  reports, or the verification reports), the expectations-stated-before-probing rule, the
  model-tiering brief obligation and the delegated-reads-inside-the-fence clause, the
  selection-scope-and-epic-only firing rule with its stated-skip obligation, the epic
  once-over-the-union rule, the high-depth mutation lens, the findings-split-by-kind rule with
  its disputed-kind-defaults-advisory routing, and the delivered-territory routing to a
  `/mochiko:feature` delta card.
- **Consumers assessed:** `mochiko:testing-gap-finding` owns the canonical inclusion list and
  takes the same re-key in this wave, including the Addendum A6 path correction (map entries live
  at `.mochiko/features/FEAT-XXX-<slug>.md`; the `FEAT-XXX/` directory is run output, out of
  fence) — ruled to live in that skill only, not restated here. The record's Open questions carry
  the adequacy watch: whether the new slot carries for expectation derivation what
  `requirements.md` carried.

## [v0.91.0] Design inputs — `plan.md`, `requirements.md`, and the plan-run store-delta anchor (D3, D4)

- **Disposition:** superseded → the bullet re-keyed to **`sufficiency-report.md`**, the
  **design-phase deltas** (including the signed store delta) at `.mochiko/features/FEAT-XXX/`,
  the product baselines, and `spec.md`.
- **Tier failed:** n/a — supersession by ruling (record D3 — the FR→TR layer dies as a mandatory
  artifact, no per-feature `requirements.md`; record D4 — `plan.md` the summary artifact dies, no
  restatement artifact; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "- **Design inputs** — the feature's `plan.md` and — where the plan run
  authored one — its **signed store delta** (the anchor for the deviation check and the
  built-vs-signed diff) with the feature's other deltas at `.mochiko/features/FEAT-XXX/`, plus its
  `requirements.md` there;".
- **Kept deliberately:** the signed store delta survives as an input and keeps both of its stated
  roles (the anchor for the deviation check and the built-vs-signed diff) — only its provenance
  moves from the plan run to this run's design phase. The rest of the bullet is byte-for-byte
  intact: the product baselines at `.mochiko/product/` — `data-model.md`, `contracts/`,
  `constraints-and-decisions.md`, and the architecture store with the `NFR-XXX` numeric quality
  targets its concern rows carry — and `spec.md` for the cards' cited acceptance criteria.
- **Consumers assessed:** `commands/plan.md` is retired whole in this wave, so no producer of
  either artifact remains. The real technical decisions `requirements.md` used to carry land where
  they already lived — `constraints-and-decisions.md` and the store — authored by the design phase
  (Tools) or, at build time, through the judgment-graded `baseline-delta.md` path (record D1
  mechanic d). `mochiko:authoring-technical-requirements`, whose subject dies with D3, takes its
  build-wave disposition in the same wave's re-point set.

## [v0.91.0] `$ARGUMENTS` empty-resolution keyed to a planned package (D1)

- **Disposition:** superseded → "empty → resolve the next capability with selected rows carrying
  ratified scope from the map and confirm with the user".
- **Tier failed:** n/a — supersession by ruling (record D1 — implement becomes the single
  downstream run, entry re-gating on ratified selection; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "empty → resolve the next capability-batch with a planned package from
  the map and confirm with the user."
- **Kept deliberately:** the resolution itself and its confirm-with-the-user close survive
  unchanged; only the selector moves from "has a planned package" to "carries ratified scope",
  which is exactly the Entry gate the same edit installs.
- **Consumers assessed:** none outside this command — the `$ARGUMENTS` line is command-local.

## [v0.91.0] The plan-time store-delta anchor cluster (D1 mechanic a)

- **Disposition:** superseded → every anchor re-keyed to **the delta signed this run, whenever
  signed** — at the design checkpoint or at a mid-run re-fire.
- **Tier failed:** n/a — supersession by ruling (record D1 mechanic a — the deviation gate anchors
  to the signed delta once one exists, and the built-vs-signed diff trigger fires on any delta
  signed this run, whenever signed; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, three sites —
  (i) done condition: "where a store delta was approved at plan time, a built-vs-signed diff
  report exists — owed on the **approved-delta-existed trigger alone**";
  (ii) Not-done list: "an approved-delta landing without its built-vs-signed diff";
  (iii) Roles & Responsibilities, the user's architecture-deviation consent: "moves a
  responsibility across a boundary of the signed store delta";
  and the Reports bullet's artifact name "the built-vs-approved diff report", renamed to
  built-vs-signed for consistency with the done condition's own term.
- **Kept deliberately:** the trigger's *character* is what the ruling protects and it survives
  whole — the diff is owed on the delta's existence alone, not on the work having stayed in
  scope, so a feature descoped to nothing is still caught at its landing rather than weeks later
  by the orphan sweep; that rationale clause is carried verbatim. The deviation gate's own test
  (a box added or removed, an arrow added, removed, or redirected, a responsibility moved across
  a boundary) and its two dispositions (build as approved, or amend the delta by the user's
  ruling first) survive byte-for-byte, as does the orphan half of the Not-done clause.
- **Consumers assessed:** the Store landing tool bullet and the acceptance landing's one-carve
  rule read the delta's lifecycle, not its provenance, and are untouched;
  `mochiko:authoring-architecture-store` keys in-flight-class elements to `FEAT-XXX`, not to a
  producing command, so it needs no edit for this anchor move.

## [v0.91.0] Epic entry — the accepted-package precondition and the joint-plan reference (D1, D1 b)

- **Disposition:** superseded → members gate on **ratified selection** like any selection-scope
  batch, and the epic's design phase **always fires** for the joint spine (the joint design plan
  replacing the joint plan-the-plan proposal as the spine's planning artifact).
- **Tier failed:** n/a — supersession by ruling (record D1 — implement is the single downstream
  run, no accepted-package precondition; D1 mechanic b — the epic run always fires the design
  phase for the joint spine, the joint-proposal spine artifact re-keying to the joint
  design-phase plan; D4 — plan's inline epic mint door dies, minting is desk-only;
  `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, two sites —
  (i) Entry: "**Epic entry:** `$ARGUMENTS` naming an `EPIC-XXX` gates on that epic's **accepted
  package** — every member's `tasks.md` (and its signed store delta where the proposal produced
  one) complete at `.mochiko/features/FEAT-XXX/`, the joint spine accepted at
  `.mochiko/epics/EPIC-XXX/` (`mochiko:authoring-epic`); any member incomplete → block, point
  to `/mochiko:plan`.";
  (ii) done condition, epic branch: "one merged **sequential** cycle sequence from the joint
  plan".
- **Kept deliberately:** every other epic rule survives byte-for-byte — the `EPIC-XXX` lookup
  resolution, every member entering as selection scope with delta-scope cards never joining an
  epic, the in-epic dependency not blocking while an outside-epic dependency at a
  non-`delivered` row still blocks, the merged sequential cycle sequence itself with shared
  foundation cycles first then in-epic dependency order, feature-tagged cards reporting into
  each member's `.mochiko/features/FEAT-XXX/`, one final validation from one cold snapshot over
  the union of member territories, the member-scoped attempt-exhaustion halt with its
  user-reserved carve-out disposition, and the one acceptance landing executing each member's
  graduation batch plus the epic close. The `mochiko:authoring-epic` pointer survives at every
  site it stood.
- **Consumers assessed:** `mochiko:authoring-epic` re-keys in the same wave — epic runs now enter
  implement directly and the joint-proposal spine artifact becomes the joint design-phase plan;
  the spine's shared-baseline single-pen-holder rule, the cross-member seam-owner-named-at-design-
  time rule, and the spine artifacts' `templates/artifact-format.md` deliverable binding are
  carried into this command's Design phase tool bullet rather than dropped with `plan.md`.
  `commands/plan.md` and its inline declare-and-contest mint door retire whole in this wave; epic
  minting survives at the desk (`/mochiko:feature`), per the record's partial-kill disposition.

## [v0.91.0] Delta-scope entry gated on a plan-run-confirmed card (D6)

- **Disposition:** superseded → the run gates on the **desk-confirmed delta card** directly
  (`/mochiko:feature` mints it and its existing review leg covers it); run-open absorbs the
  card-vs-entry confirmation the delta-scope plan run used to perform.
- **Tier failed:** n/a — supersession by ruling (record D6; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "**Delta scope** gates on the delta card confirmed by a delta-scope plan
  run; the card's acceptance criteria (a bug's reproduction-failing-test, or 1–3 criteria on the
  delta) are the cycle's criteria."
- **Kept deliberately:** the card's acceptance criteria remain the cycle's criteria, in the same
  two forms (a bug's reproduction-failing-test, or 1–3 criteria on the delta) — only the
  confirming authority moves from a plan run to the desk plus this run's run-open. Delta scope
  keeps every other property it had: the sufficiency check runs per-card rather than per-row
  (D6), the gap-finding pass is skipped with the skip stated explicitly in the final-validation
  report, and the landing is the marked delta's fold.
- **Consumers assessed:** `commands/feature.md` already mints and reviews the delta card and needs
  no new obligation; its routing line re-points to implement in this wave. Product-lane runs are
  declared **inherited-as-is** by D6 — their absence from Entry's scope branches predates this
  session and is neither worsened nor repaired here; the record's Open questions carry it.

## [v0.91.0] Selection-scope entry gated on the accepted plan package (D1)

- **Disposition:** superseded → entry re-gates on the capability entry's **ratified selection**
  alone; the sufficiency check at entry (`mochiko:review-sufficiency`) replaces the
  accepted-package precondition as the measure of whether there is enough design to build.
- **Tier failed:** n/a — supersession by ruling (record D1 — `/mochiko:plan` retires and implement
  becomes the single downstream run; the record's Build surface names this precondition's removal
  explicitly; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "**Selection scope** additionally gates on the accepted package the plan
  run produced — the batch's `tasks.md` complete alongside its `plan.md`, and its **signed store
  delta** where the plan run authored one, at `.mochiko/features/FEAT-XXX/`; missing or incomplete
  → block, point to `/mochiko:plan`;".
- **Kept deliberately:** the dependency-order gate that closed the same sentence survives verbatim
  — "a capability-batch whose selected rows depend on rows not yet `delivered` blocks — batches
  run in the rows' dependency order". So does everything the precondition used to guarantee, now
  produced in-run rather than gated on: `tasks.md` (Card authoring, whose seat is never the
  executing builder) and a signed store delta where the structural trigger fires (Design phase,
  signed by the user at the design checkpoint before the first cycle). Existing accepted packages
  remain valid frozen history — a feature holding one enters with the check trivially satisfied
  (record, Migration).
- **Consumers assessed:** `commands/plan.md` retires whole in this wave, so the block-and-point
  routing had no target left. `commands/specify.md`'s next-step line and `commands/feature.md`'s
  growth-row routing re-point from `/mochiko:plan` to `/mochiko:implement` in the same wave.

## [v0.91.0] Identity & Mission — the accepted-package charter framing, and the command heading (D1)

- **Disposition:** superseded → implement chartered as the pipeline's **single downstream run**:
  ratified selection in, a sufficiency check at entry, a conditional design phase the user signs,
  cards, then working verified code out. Heading re-titled to match.
- **Tier failed:** n/a — supersession by ruling (record D1; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, two sites —
  (i) heading: "# Implement — Execute the Task Breakdown";
  (ii) mission: "You are chartered **Delivery Manager of the goal** — the same charter
  `/mochiko:plan`'s lead holds, carried from accepted package to accepted implementation: this run
  turns one capability-batch's accepted `tasks.md` (cycle cards for the capability's selected work
  rows, resolved from `.mochiko/features/FEAT-XXX/`) into working, verified code — TDD-built,
  foundation cycles before feature cycles, verified against real infrastructure. An **epic**
  (`EPIC-XXX`) run turns the accepted packages of its member features into one merged, verified
  build — one run over the whole epic (`mochiko:authoring-epic`)."
- **Kept deliberately:** the **Delivery Manager of the goal** charter itself survives verbatim —
  D1 retires the plan command, not the posture the two commands shared, and the charter-form
  contract (`.mochiko/decisions/2026-08-13-charter-plan-implement.md`) binds this file unchanged.
  So do the mission's substantive terms: the capability-batch as the unit, TDD-built, foundation
  cycles before feature cycles, verified against real infrastructure, the epic as one merged
  verified build over the whole epic with its `mochiko:authoring-epic` pointer, "The working code
  is the deliverable", and "Plan the run and orchestrate it toward the done condition."
- **Consumers assessed:** `commands/feature.md`'s recorded wording — "the same split
  `/mochiko:plan` and `/mochiko:implement` name at their Entry" — re-points in this wave; the
  charter ADR's requirement that this command keep a protocol step **literally labeled Entry** is
  honored, and the audit's floor-present + run-goal-contract-present criteria are unaffected by
  the mission rewrite.

## [v0.91.0] Command `description:` — the accepted-breakdown precondition (D1)

- **Disposition:** superseded → a description naming ratified scope in, the entry sufficiency
  check, the conditional design phase, and the TDD build with independent real-infrastructure
  verification.
- **Tier failed:** n/a — supersession by ruling (record D1; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "Execute an accepted task breakdown into working code, TDD-built and
  independently verified against real infrastructure."
- **Kept deliberately:** the description's back half survives in substance — working code as the
  output, TDD-built, independently verified against real infrastructure. Only the "accepted task
  breakdown" precondition dies, because the breakdown is now authored inside this run.
- **Consumers assessed:** commands carry no per-primitive character budget
  (`.mochiko/memory/primitive-cost-budgets.md` — commands were excluded from both budget waves by
  user ruling), so the D7 char-budget pre-assert does not fire on this edit; the hard caps are the
  only bound and are not approached. `disable-model-invocation: true` is unchanged, so the
  description is maintainer- and router-facing only. The `mochiko` router's command row re-points
  in this wave.

<!-- Wave context: the product-architecture-schema Stage-1 build wave (v0.81.0) — one schema-backed
architecture store replaces the five-surface architecture split, and the whole pipeline re-keys in
one wave (D15). Ruling for every [v0.81.0] entry below:
`.mochiko/brainstorms/product-architecture-schema/record.md` (D3 · D10 + its S6 fold · D11 as
narrowed · D12) → `DECISIONS.md` 2026-08-19 product-architecture row. -->

## [v0.81.0] The KM landing's `ARCHITECTURE.md` dual-target fold — replaced by the store landing (D10/D11)

- **Disposition:** superseded → the **Store landing** tool bullet: status flips + cleared
  `FEAT-XXX` keys (transcription, riding this run's landing audit), graded `As-built:`/`Drift:`
  writes (judgment), and the orphan check — all per
  `mochiko:authoring-architecture-store`, which regenerates the derived root `ARCHITECTURE.md`.
- **Tier failed:** n/a — supersession by ruling (record D3/D10/D11-as-narrowed; `DECISIONS.md`
  2026-08-19 row). `mochiko:authoring-architecture` is retired this wave, so its pointer could
  not survive.
- **Content:** verbatim — "- **KM landing** — where `.mochiko/memory/knowledge-management.md`
  exists, a built structural change folds into `ARCHITECTURE.md` — the fold is dual-target (the
  feature's `architecture.md` accumulates the approved delta) per
  `mochiko:authoring-architecture`."
- **Kept deliberately:** the KM conditionality survives as the bullet's closing sentence — where
  `.mochiko/memory/knowledge-management.md` exists, the same landing carries its KM obligations.
  What the dual target cost is what D3 bought: intent and built-state now live on one surface, so
  there is one fold, not two. The fold's *graded* character not only survives but widens — under
  D11 as narrowed, `As-built:` and `Drift:` are judgment writes and take the review leg; only
  status flips and orphan cleanup are transcription.
- **Consumers assessed:** `plan.md`'s In-flight pointer bullet deleted in the same wave (its own
  strip entry); the acceptance-landing clear below; `mochiko:authoring-architecture` retired by
  P1's cluster; the pinned KM In-flight invariant (AT-D6-C) re-keyed by the wave's landing.

## [v0.81.0] The acceptance landing's In-flight-pointer clear — deleted (D10 orphan rule)

- **Disposition:** superseded → the store landing's status flips and orphan check; the landing's
  opening clause re-points from "the same landing that folds `ARCHITECTURE.md`" to "the same
  landing that executes the store landing above".
- **Tier failed:** n/a — supersession by ruling (record D10; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim — "the `ARCHITECTURE.md` In-flight pointer is cleared ·" (selection-scope
  branch of the Acceptance landing bullet).
- **Kept deliberately:** every other member of the graduation batch survives byte-for-byte — the
  delivered work rows folding into extent with pending rows persisting, the dated non-regressing
  `delivered` status, the `FEATURES.md` index line, the specs-index touch and its
  derived-never-asserted closure rule, the whole epic branch including the
  fold-each-baseline-exactly-once rule and the shared-baseline-folds-once-from-the-spine rule,
  the delta-scope branch, the graded three-way diff, the absent-baseline seeding-gap surface, and
  the gap-finding fold-back into `gates.md`.
- **Consumers assessed:** `mochiko:authoring-feature-map`'s graduation batch (P4's file, same
  wave) — the batch loses one member and gains none.

## [v0.81.0] Deviation gate, entry gates, and design inputs re-targeted to the store (D3)

- **Disposition:** superseded → the signed store delta as the deviation anchor and the entry
  gate's object; the built-vs-approved diff renamed built-vs-signed.
- **Tier failed:** n/a — supersession by ruling (record D3; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim, four sites — (1) entry, selection scope: "and its `architecture.md`
  where the proposal included one"; (2) entry, epic: "(and its `architecture.md` where the
  proposal produced one)"; (3) user bullet: "on the approved diagram stops and is presented —
  build as approved, or amend `architecture.md` first"; (4) design inputs: "`architecture.md`
  (the anchor for the deviation check and the built-vs-approved diff)".
- **Kept deliberately:** the deviation gate's **test** is untouched — a cycle that adds or
  removes a box, adds, removes, or redirects an arrow, or moves a responsibility across a
  boundary still stops and is presented, and the two dispositions are still build-as-approved or
  amend-first (the amendment now being the user's ruling on the store delta). The block-and-point
  behaviour of both entry gates survives unchanged.
- **Consumers assessed:** `plan.md`'s sign-off site (own strip entry, same wave);
  `mochiko:patterns-system-design` (P1's, transformed to serve store deltas).

## [v0.81.0] Built-vs-approved diff — now owed on the approved-delta-existed trigger alone (D10/S6)

- **Disposition:** superseded → the diff runs whenever an approved delta existed, **even when
  nothing was built**, catching a descoped feature at its landing rather than weeks later by the
  orphan sweep.
- **Tier failed:** n/a — supersession by ruling (record D10's S6 review fold; `DECISIONS.md`
  2026-08-19 row).
- **Content:** verbatim — "where a structural delta was approved at plan time, a built-vs-approved
  diff report exists and any divergence it names was ruled by the user" (protocol step 3).
- **Kept deliberately:** the user-rules-every-divergence half survives byte-for-byte. This entry
  is a **widening, not a relaxation**: the old wording could be read as satisfied by a run that
  built nothing having nothing to diff.
- **Consumers assessed:** the Duty-1 landing-diff trigger it inherits from moves to
  `mochiko:authoring-architecture-store` (P1's cluster); the diff report stays in the run's
  Reports list, unchanged.

## [v0.81.0] `nfrs.md` reads re-pointed to store concern rows (D12)

- **Disposition:** superseded → the store's concern rows, which carry the `NFR-XXX` targets; the
  ids themselves are unchanged.
- **Tier failed:** n/a — supersession by ruling (record D12; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim, two sites — (1) design inputs: "`nfrs.md` for the numeric quality
  targets the built code must respect"; (2) the gap-finding pass's blind dispatch fence: "plus
  the product baselines `data-model.md`, `contracts/`, and `nfrs.md`".
- **Kept deliberately:** the blind fence's exclusion list is byte-for-byte intact — the seat
  still never receives the code, `tasks.md`, the `**TEST:**` cases, the cycle reports, or the
  verification reports, and its delegated reads stay inside the same fence. Only the NFR read's
  *path* moved; the seat still gets the numeric targets it derives expectations from, so the
  pass's discovery power is unchanged.
- **Consumers assessed:** `testing-gap-finding`'s runtime-NFR re-point is P4's this wave;
  `plan.md`'s product-surface list (own strip entry).

## [v0.81.0] FAIL clause re-keyed to the store, plus a new landing-integrity clause

- **Disposition:** superseded → "a surfaced store deviation neither built as approved nor
  consented as an amendment", with a **new** clause added beside it: an approved-delta landing
  without its built-vs-signed diff, or leaving an in-flight-class element neither flipped `built`
  nor keyed to an open feature.
- **Tier failed:** n/a — supersession by ruling (record D3/D10; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim — "a surfaced architecture deviation neither built as approved nor
  consented as an amendment".
- **Kept deliberately:** every other clause of the `**Not done — default FAIL**` list survives
  byte-for-byte — unchecked cycle card, failing quality gate, verification without
  real-infrastructure evidence, regression in a delivered feature's gates, touched baseline
  without its graded fold, the gap-finding pass clause and its stated-skip counterpart, the
  unresolved spec-violation gap finding, and user acceptance. The added clause is net-new
  obligation from D10's orphan rule, not a restatement.
- **Consumers assessed:** the charter-form audit reads this list for surviving FAIL clauses
  (`.claude/rules/mochiko/primitive-edits.md`, charter exception leg iv).

## [v0.79.0] Accumulated territory `**TEST:**` gates re-pointed to their named home (`gates.md`)

- **Disposition:** superseded → the same Regression-scope clause, now naming the read source:
  the union of the territory features' durable gate sets at
  `.mochiko/features/FEAT-XXX/gates.md` and the cases on their cards.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-19 QA gap-finding row;
  record `.mochiko/brainstorms/qa-gap-finding-verification/record.md` D7 as amended at review
  by I5 — the fold-back had no target artifact, and implement.md's accumulated-gates reads had
  no named home).
- **Content:** verbatim superseded text (Tools → Regression scope): "the final validation
  additionally executes the accumulated `**TEST:**` gates of previously delivered features
  in this feature's territory, and this feature's gates exercise any seam whose earlier side
  is already delivered"
- **Kept deliberately:** the seam clause and its `mochiko:authoring-feature-map` seam-ownership
  pointer, and the epic-union sentence closing the same bullet — all byte-for-byte; only the
  read source was named. Deliberately NOT re-pointed: the three other accumulated-gates reads
  (Adaptive Goal Protocol step 3, the step-3 epic clause, and the Regression-scope epic
  sentence). Their wording stays correct and now resolves through this one definitional home;
  restating the path at four sites is what the plan ladder's minimum-now rung refuses. A later
  wave finding those reads unhomed should read this line before "fixing" them.
- **Consumers assessed:** `plan.md` confirmed NOT a consumer at review (reviewer kill list,
  same record); `patterns-vertical-tdd` owns the `**TEST:**` grammar, not the accumulation
  read — untouched; `mochiko:authoring-feature-map` owns graduation, and `gates.md` survives
  graduation by the D7/I5 ruling — no map-side change.

## [v0.79.0] Run-open redeclarable set widened from one bound to two

- **Disposition:** superseded → "The done condition is fixed; the attempt bounds — the
  per-cycle bound and the final-validation gap-rework bound — are the only redeclarable terms."
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-19 QA gap-finding row;
  record D6 as amended at review by I3 — gap-rework at final validation carries a whole-run
  bound, default 2 rounds, redeclarable only at run open).
- **Content:** verbatim superseded text (Adaptive Goal Protocol step 2): "The done condition is
  fixed; only the attempt bound is redeclarable."
- **Kept deliberately:** the fixed done condition, the per-cycle default of 3, and run open as
  the single redeclaration point for every bound — the widening adds a second bound, never a
  second point.
- **Consumers assessed:** the charter-command audit exception in
  `.claude/rules/mochiko/primitive-edits.md` requires run-open confirmation to name the
  "attempt bound (redeclarable there and only there)" — still satisfied, the set widened and
  the point did not move; `plan.md`'s run open carries no attempt bound, so it is not a
  consumer.

## [v0.78.0] Model-tiering floor line retargeted — `mochiko:explorer` superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded floor line: locate/enumerate reads go to "a
  native `Explore` subagent spawned `model: haiku`".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents.
- **Content:** verbatim superseded phrase: "the cheap explorer seat (`mochiko:explorer`)"
  (line wrap varies per command; only this phrase changed).
- **Kept deliberately:** the rest of the floor line byte-for-byte — the class-key summary,
  session-tier carve-outs, the every-seat-brief obligation, and the closing
  `mochiko:patterns-model-tiering` referenced-never-restated pointer.
- **Consumers assessed:** the same phrase edited in all six commands in the same v0.78.0
  wave (entry mirrored per command strip file); the pointed-at skill reworded in the same
  wave (`strips/patterns-model-tiering.md`).

---

<!-- Wave context: charter conversion wave (v0.69.0) — ADR
`.mochiko/decisions/2026-08-13-charter-plan-implement.md`: the D10 charter anatomy extends from
the desk to the two pipeline commands; behavior-preserving re-format, no obligation added or
dropped; supersedes D10's "this command only" clause. -->

## [v0.69.0] v8 Goal · Harness · Bindings anatomy → six-section charter (the build run)
- **Disposition:** superseded → the six-section charter that now IS `commands/implement.md`
  (Identity & Mission · Adaptive Goal Protocol · Roles & Responsibilities · Tools · Ways of
  Working · Boundaries). The v8 default-FAIL-goal anatomy is replaced whole; the audit re-keys
  to grade *floor present + run goal contract present* in place of *default-FAIL goal present*
  (`.claude/rules/mochiko/primitive-edits.md`, charter exception as broadened this wave).
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-13-charter-plan-implement.md` + `DECISIONS.md` row 2026-08-13 —
  charter extended to the two pipeline commands, D10's this-command-only clause superseded;
  behavior-preserving by the same ruling, so this entry records a shape supersession only.
  One named delta: the run-open contract statement (protocol step 2) states at open what v8
  carried as the `$ARGUMENTS`-empty confirm plus Bounds' "redeclarable at run open" point —
  composed, not invented; sanctioned by the ruling's own wording).
- **Content:** the entire pre-charter `commands/implement.md`, verbatim:

```
---
description: Execute an accepted task breakdown into working code, TDD-built and independently verified against real infrastructure.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown

**Goal:** turn one capability-batch's accepted `tasks.md` (cycle cards for the capability's
selected work rows, resolved from
`.mochiko/features/FEAT-XXX/`) into working, verified code — TDD-built, foundation cycles
before feature cycles, verified against real infrastructure. `$ARGUMENTS` = the capability ID
(`FEAT-XXX`); empty → resolve the next capability-batch with a planned package from the map and confirm
with the user.

## Goal

Every `tasks.md` cycle card is `[x]`; each card was decomposed into concrete tasks by its
builder at build time — the decomposition disclosed in the cycle report, never pre-written —
and the built code was implemented test-first (red/green/refactor) and independently
verified — executed `**TEST:**` gates, quality gates
with exit codes, captured real-infrastructure evidence — per cycle and once for the whole
implementation; the feature's verification also ran the **accumulated TEST gates of
previously delivered features in its territory**, and any seam against an earlier-delivered
feature was exercised here, against the real delivered side; the code meets its criteria,
holds traceability to requirements, and aligns
with the project's governance; where a structural delta was approved at plan time, a
built-vs-approved diff report exists and any divergence it names was ruled by the user; the
acceptance landing executed whole — map bookkeeping and every touched baseline's graded
fold; and the user accepted the implementation.

**Not done — default FAIL:** an unchecked cycle card · a failing quality gate · verification
without real-infrastructure evidence · a regression in a previously delivered feature's
gates · a surfaced architecture deviation neither built as
approved nor consented as an amendment · a touched baseline accepted without its graded
fold · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Plan approval:** any seat that writes code or artifacts plans first and works only on a
  plan you approved; grading, verification, and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author — implementation and verification are
  never the same seat; verification executes against real infrastructure and reads the code
  and its evidence, default FAIL.
- **Bounds:** a cycle consumes an **attempt** every time a verification seat grades it —
  whatever the round is called (rework, completion, targeted fix, re-grade); default 3
  attempts per cycle, redeclarable at run open. Exempting a round from the count is reserved
  to the user, never lead discretion. Two consecutive rounds with unchanged findings is a
  no-progress stop: halt the cycle, present state. Re-verification is scoped to the delta —
  a test-only or records-only change gets a delta-grade of the changed surface, never a full
  gate re-sweep; a delta round re-runs no quality gates, the prior gate evidence standing
  while the graded head is unmoved — and the graded object is the code tree (`git rev-parse
  HEAD:<code-dir>`), so a records-only commit does not move the graded head. Rounds consumed
  and seats spawned are surfaced to the user at each checkpoint.
- **Reserved to the user:** architecture-deviation consent — a cycle that adds or removes a
  box, adds, removes, or redirects an arrow, or moves a responsibility across a boundary on
  the approved diagram stops and is presented: build as approved, or amend `architecture.md`
  first · requirement ambiguity or a judgment call a producer flags — answered by the user,
  investigable gaps excepted · scope escalation (work bigger than the run was framed; the
  run stays FAIL unless the user explicitly accepts) · exempting a grading round from the
  attempt count (Bounds) · final acceptance (accept / amend / reject).
- **Escalation cadence:** reserved-to-user questions accumulate and land as one batch at the
  cycle checkpoint; only a question the build cannot proceed without interrupts mid-cycle.
  Advisory verifier findings ride the same rule — a Minor advisory finding defaults to a
  `BACKLOG.md` booking, never an in-cycle fix; an Important-or-above advisory finding blocks
  the cycle and enters the checkpoint batch. A failed `**TEST:**` gate or quality gate is
  never severity-triaged — it fails the cycle per the Goal; `minimalism:` findings stay
  advisory at any severity, per Bindings.
- **Entry:** the run gates on a capability entry with selected work rows carrying ratified
  scope — the scope source is
  a spec's accepted selection, or a feature-command card: growth rows enter as selection
  scope, a bug/improvement delta as delta scope. Neither → block: new
  capability to `/mochiko:specify`, feature-keyed delta to `/mochiko:feature`. **Selection
  scope** additionally gates on the accepted package the plan run produced — the batch's
  `tasks.md` complete alongside its `plan.md`, and its `architecture.md` where the proposal
  included one, at `.mochiko/features/FEAT-XXX/`; missing or
  incomplete → block, point to `/mochiko:plan`; a capability-batch whose selected rows depend
  on rows not yet `delivered` blocks — batches run in the rows' dependency order. **Delta scope** gates
  on the delta card confirmed by a delta-scope plan run; the card's acceptance criteria (a
  bug's reproduction-failing-test, or 1–3 criteria on the delta) are the cycle's criteria. A
  missing governance region is surfaced, never auto-resolved; present → each code-touching
  brief names the relevant `.claude/rules/mochiko/` files as an obligated read.
- Suggest commits; never run git mutations, never push — an ephemeral, self-removed
  verification snapshot is not a mutation of refs, index, tracked content, or history.
  User acceptance is plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** the working code; `tasks.md`'s per-card checkboxes are the progress
  surface, flipped as cycles complete.
- **Craft skills:** card decomposition + TDD via `mochiko:executing-tdd-cycle` (its
  `cycle-report.md` format — the disclosed decomposition, honest difficulties, deviations,
  `domain_deps_added` — is the uncertainty carrier; brownfield touches ride
  `mochiko:brownfield-integration`; the pre-code ladder rides
  `mochiko:patterns-code-minimalism` at decomposition, rungs disclosed) · verification via
  `mochiko:testing-end-user` — evidence captured, never assumed — plus the per-cycle
  code-minimalism lens via `mochiko:review-code-minimalism`: the verification seat reads
  the cycle's diff, `cycle-report.md`, and the codebase around the diff (reuse claims
  never on trust); `minimalism:` findings are advisory to the checkpoint verdict, never a
  cycle-failing gate.
- **Design inputs:** the feature's `plan.md` and — where the proposal produced one —
  `architecture.md` (the anchor for the deviation check and the built-vs-approved diff) with
  the feature's other deltas at
  `.mochiko/features/FEAT-XXX/`, plus its `requirements.md` there; the product baselines at
  `.mochiko/product/` — `data-model.md`, `contracts/`, `constraints-and-decisions.md`,
  `nfrs.md` for the numeric quality targets the built code must respect — and `spec.md` for
  the cards' cited acceptance criteria.
- **Reports** land in `.mochiko/features/FEAT-XXX/` (product-lane runs:
  `.mochiko/product/lane-<slug>/`): cycle reports, verification reports, the
  final-validation report, the built-vs-approved diff report. Repeat runs append (dated);
  delta files overwrite only via the graded fold.
- **Regression scope:** quality gates run the full repository suite; the final validation
  additionally executes the accumulated `**TEST:**` gates of previously delivered features
  in this feature's territory, and this feature's gates exercise any seam whose earlier side
  is already delivered — seam ownership sits with the later-landing feature, per
  `mochiko:authoring-feature-map`.
- **Cold verification:** the final validation builds and runs the quality gates from a
  dependency-cold snapshot of the uncommitted working state
  (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to
  `.claude/worktrees/mochiko-<purpose>/`), its results part of the acceptance evidence;
  ensure the `/.claude/worktrees` ignore entry exists first.
- **KM landing:** where `.mochiko/memory/knowledge-management.md` exists, a built structural
  change folds into `ARCHITECTURE.md` — the fold is dual-target (the feature's
  `architecture.md` accumulates the approved delta) per `mochiko:authoring-architecture`.
- **Baseline touches:** mid-fix discovery that the work touches a product baseline → the
  dispatched run authors `baseline-delta.md` in its feature dir at discovery — a minimal
  enumerated delta in appliable form. A product-lane run discovering it stands on an
  in-flight feature's territory files the finding to that run and aborts — the lane never
  widens in place.
- **Acceptance landing:** at user acceptance, one landing executes whole, branched by scope
  type. **Selection scope** — the same landing that folds `ARCHITECTURE.md` executes the
  map's graduation batch per `mochiko:authoring-feature-map`: this run's delivered work rows
  fold into the capability's extent lines and the rows vanish (pending rows persist) · the
  capability's status is set `delivered` (dated), never regressing · the
  `FEATURES.md` index line updates · the `ARCHITECTURE.md` In-flight pointer is cleared ·
  the specs-index row is touched — the spec reads closed exactly when all its selected
  work rows have folded (derived, never asserted). No separate feature-close stage
  exists. **Delta scope** — the entry's marked delta folds per `mochiko:authoring-feature-map`'s
  delta fold. **Both scopes:** every touched baseline folds via a graded fold — three-way
  diff: pre-fold baseline + delta vs folded result; delta applied whole, nothing else
  changed — checked by the landing verification seat, scope-extended; lane runs add the
  map-delta boundary check (the accepted work made no map write beyond the marked delta) to
  the same seat. A delta whose baseline file is absent at fold time folds into a fresh
  `.mochiko/product/` file (empty pre-fold side), the absence surfaced to the user as a
  seeding gap.
- **Register:** user-facing prose per `templates/output-style.md`.
```

- **Kept deliberately:** everything the v8 body carried survives, re-homed:
  - frontmatter `description:` + `disable-model-invocation` and the `# Implement — Execute
    the Task Breakdown` title — unchanged, verbatim.
  - `**Goal:**` preamble → the mission sentence → **Identity & Mission** (joined by the
    Deliverable binding's "the working code"); the `$ARGUMENTS` resolution → the **Adaptive
    Goal Protocol** tail, verbatim.
  - Harness **Entry** bullet → protocol step 1, **literally labeled Entry** — `feature.md`'s
    "the same split … name at their Entry" and the D8 build rider's recorded wording stay
    true, `feature.md` untouched; the delta-card-criteria clause and the
    `.claude/rules/mochiko/` obligated read ride inside it.
  - The whole `## Goal` paragraph → protocol step 3, the fixed done condition, carried whole;
    the reserved "final acceptance (accept / amend / reject)" is its close.
  - `**Not done — default FAIL:**` → protocol tail, all seven clauses verbatim.
  - Bounds → split three ways by kind: the attempt economy (definition · default 3 ·
    user-only exemption · no-progress stop) → **Boundaries**, with "redeclarable at run open"
    tightened to its existing single point (protocol step 2 names it; "only" was implicit in
    v8's one named point); the delta re-verification method (delta-grade · no gate re-runs ·
    graded object = code tree) → **Ways of Working / Delta re-verification**; the
    rounds-and-seats transparency sentence → the DM floor.
  - Lead bullet: "Plan the run and orchestrate it toward the Goal" → **Identity & Mission**;
    "teammates or subagents per seat is your call" → Roles & Responsibilities preamble.
  - Plan approval + Independence bullets → **Ways of Working / Author ≠ grader**, merged; the
    implementation-vs-verification split restated at the verification seat (Other seats).
  - Escalation cadence → split: checkpoint batching + advisory-finding routing → **Ways of
    Working / Escalation cadence**; "a failed gate is never severity-triaged" +
    `minimalism:`-advisory-at-any-severity → **Boundaries**.
  - Reserved list → the user seat, complete: architecture-deviation consent with its
    box/arrow/responsibility trigger whole · ambiguity/judgment calls (investigable gaps
    excepted) · scope escalation · attempt-round exemption (pointing at Boundaries) · final
    acceptance (compressed; protocol carries it).
  - Bindings → **Tools**, entry-for-entry: Deliverable (split: code → Identity & Mission,
    checkboxes → Progress surface) · Craft skills · Design inputs · Reports · Regression
    scope · Cold verification (recipe verbatim) · KM landing · Baseline touches (authoring
    half; the lane-abort half → **Boundaries / The lane never widens in place**) · Acceptance
    landing (whole; the landing seat's scope-extension + lane map-delta boundary check
    restated at the verification seat) · Register — each whole.
- **Consumers assessed:** `feature.md:97`'s "the same split `/mochiko:plan` and
  `/mochiko:implement` name at their Entry" stays true by the labeled Entry step —
  `feature.md` untouched. The router's universal-anatomy lines re-worded same wave
  (`.mochiko/strips/mochiko.md` [v0.69.0]); `.claude/rules/mochiko/primitive-edits.md`
  exception broadened (rules file, not a shipped primitive). The v0.67.0 DECISIONS
  validator-isolation row's "v8 implement carries the dependency-cold snapshot as a
  **Bindings** line" reads historically (the snapshot machinery survives in Tools/Cold
  verification). Skills and templates reference `plan.md`/`tasks.md` the artifacts, never
  this command's sections (grep-verified this wave). `plan.md` converted same wave (own
  strip).

---

<!-- Wave context: pm-role-and-feature-derivation build wave (v0.68.0) — pipeline-commands cluster.
The map re-types to durable capabilities + transient pending/live work rows (2026-08-13 ruling);
implement's run unit re-keys to the capability-batch and its selection-scope landing gains the
row folds (delivered rows fold into the capability's extent and vanish; pending rows persist),
alongside the unchanged baseline-delta folds. Command stays v8. -->

## [v0.68.0] Feature (leaf) run-unit superseded by the capability-batch + row-level dependency ordering
- **Disposition:** superseded → the re-keyed Goal / Entry: the run unit is the capability-batch (the capability's selected work rows), `$ARGUMENTS` the capability ID; a capability-batch whose selected rows depend on rows not yet `delivered` blocks, batches running in the rows' dependency order.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-13 "PM role & feature derivation ruled (D1–D12)"; record `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`, D7 + the D6 inventory "plan.md / implement.md — leaf keying *superseded* (capability-batch, D7)")
- **Content (superseded, verbatim):**
  - Goal: "turn one feature's accepted `tasks.md` (cycle cards, resolved from `.mochiko/features/FEAT-XXX/`)" · "`$ARGUMENTS` = the feature ID (`FEAT-XXX`); empty → resolve the next planned undelivered feature from the map"
  - Entry: "the feature's `tasks.md` complete alongside its `plan.md`" · "a selected feature ordered earlier and not yet `delivered` blocks — one run per feature, in dependency order."
- **Kept deliberately:** the package-gate machinery (`tasks.md` + `plan.md` + conditional `architecture.md` at `.mochiko/features/FEAT-XXX/`), the point-to-plan block, the `.mochiko/features/FEAT-XXX/` path (unchanged; the capability's dir per D7 fold), all TDD / verification / bounds machinery, regression scope, and the delta-scope gate.
- **Consumers assessed:** none — entry points. Ripple: `plan.md` (same re-key, own strip), `specify.md` (Next-step re-key, own strip), `authoring-feature-map` (capability-batch vocabulary, builder-map).

---

## [v0.68.0] Two-source scope enumeration superseded by the growth-rows-selection / bug-delta-scope split
- **Disposition:** superseded → the Entry re-word: scope source = a spec's accepted selection or a feature-command card — growth rows as selection scope, a bug/improvement delta as delta scope.
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`, **D8 "Build-time rider" (user-ratified 2026-08-13, at the v0.68.0 build)**: a growth-door dispatch enters selection scope, a bug/improvement delta card stays delta scope — the split's recorded home, minted because the desk-cluster audit flagged it as a lead coordination call with no recorded home (GI-006); also annotated on the `DECISIONS.md` 2026-08-13 "PM role & feature derivation ruled" pm-role row as the "Build rider". Carried verbatim in `feature.md` (Tools + Boundaries) and `plan.md` / `implement.md` (Entry).)
- **Content (superseded, verbatim):**
  - Entry: "the scope source is a spec's accepted Feature Selection or a feature-command delta card"
- **Kept deliberately:** the neither-source routing (new capability → `/mochiko:specify`; feature-keyed delta → `/mochiko:feature`), and the delta-scope gate mechanics (the delta card confirmed by a delta-scope plan run; the card's acceptance criteria are the cycle's criteria).
- **Consumers assessed:** none mount commands. `plan.md`'s Entry + Feature-scope binding carried the same line (own strip, same stamp); `feature.md` (Tools + Boundaries) carries the same split verbatim per the D8 Build-time rider — all three scope-split strips (desk + these two) cite that one recorded home.

---

## [v0.68.0] Acceptance-landing selection-scope map-bookkeeping extended to row folds
- **Disposition:** superseded → the re-keyed Selection-scope landing: this run's delivered work rows fold into the capability's extent lines and the rows vanish (pending rows persist); the capability's status is set `delivered` (dated), never regressing; the spec reads closed when all its selected work rows have folded.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-13 "PM role & feature derivation ruled (D1–D12)"; record `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`, D2 (the fold moment — a work row's content folds into the capability's extent and the row disappears; sticky-delivered survives as "capability `delivered`, live rows visible, no status regression") / D7 (implement's landing folds the delivered rows) + the D6 inventory "implement's landing fold *extended* to execute row folds; baseline-delta folds *unchanged*")
- **Content (superseded, verbatim):**
  - "the feature's status flips to `delivered` (dated) · this feature's marked deltas fold into its extent lines"
  - "the spec reads closed exactly when all its selected FEAT-IDs read `delivered` (derived, never asserted)"
- **Kept deliberately:** the `FEATURES.md` index touch, the `ARCHITECTURE.md` In-flight pointer clear, the specs-index touch, "No separate feature-close stage exists", the **Delta scope** branch, and the **Both scopes** baseline graded-fold (three-way diff) — the baseline-delta folds are unchanged. The sticky-delivered delta-carry is re-typed onto the row fold (D6: "invariant 5 sticky-delivered delta-carry *re-typed* (pending/live rows; stickiness survives as no-status-regression)").
- **Consumers assessed:** none mount commands. `authoring-feature-map` carries the graduation-batch / row-fold grammar (builder-map this wave).

---

## [v0.67.0] Always-on architecture.md in the selection-scope entry gate + design inputs made proposal-conditional
- **Disposition:** superseded → the entry + design-inputs re-key in the same file: selection scope gates on the accepted package the plan run produced — `tasks.md` + `plan.md`, and `architecture.md` **only where the proposal included one**; the deviation check and built-vs-approved diff were already conditional on an approved structural delta and are untouched.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/plan-structure-yagni/record.md` D6a as amended HF-4 — architecture conditional-but-reserved, quiet tie-back; combined wave `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` D5)
- **Content:** Entry "the feature's `tasks.md` complete alongside its `plan.md` and `architecture.md` at `.mochiko/features/FEAT-XXX/`" · Design inputs "the feature's `plan.md` and `architecture.md` (the anchor for the deviation check and the built-vs-approved diff)"
- **Kept deliberately:** `tasks.md` + `plan.md` as the always-present selection-scope entry gate; the deviation-consent reserved item, the built-vs-approved diff ("where a structural delta was approved at plan time"), and the KM `ARCHITECTURE.md` fold — all already conditional on a structural change existing, untouched.
- **Consumers assessed:** n/a — command entry point; `plan.md` (same wave, same stamp) is the producing side of the same conditional-architecture ruling.

## [v0.61.0] Spec-folder package gate + unconditional graduation landing superseded by feature-keyed entry + scope-branched landing with graded folds
- **Disposition:** superseded → the feature-keyed rewrite in the same file: the canonical entry gate (selection scope additionally gating on the accepted package resolved from `.mochiko/features/FEAT-XXX/`; delta scope gating on a delta card confirmed by a delta-scope plan run), product baselines at `.mochiko/product/` + this feature's deltas as design inputs, reports in `.mochiko/features/FEAT-XXX/` (product-lane runs: `.mochiko/product/lane-<slug>/`), the scope-branched Acceptance-landing binding (selection scope = the graduation batch, surviving verbatim; delta scope = the delta fold; every touched baseline folded via the graded fold — three-way diff — checked by the landing verification seat, scope-extended; lane runs add the map-delta boundary check to the same seat, no new seat), the Baseline-touches binding (`baseline-delta.md` authored at discovery; product-lane abort-and-reroute)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Land feature-sizing & entry-points rulings (D1–D15)"; record `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`, D7 (the lane map-delta boundary check joins the landing verification seat) / D8-as-amended / D9 / D14 (findings 6 + 7: dispatched-run `baseline-delta.md` at discovery, product-lane files-and-aborts) / D15 (graded folds, appliable delta form) — D9's supersession note retires feature-map D17/D18 (artifact layout inside the spec folder, extend-mode home, cross-spec reach), feature-map D10's plan-artifacts-in-spec-folder clause, and feature-map D19's read mechanics)
- **Content:** header "(cycle cards, under `features/FEAT-XXX/` in the spec folder)" / "empty → resolve the next planned undelivered feature from `.mochiko/specs/` and confirm with the user." · Goal "the acceptance landing's map bookkeeping executed whole" (extended to "— map bookkeeping and every touched baseline's graded fold") · Entry bullet "the accepted package gates the run — the feature's `tasks.md` complete alongside its `plan.md` and `architecture.md` under `features/FEAT-XXX/`; missing or incomplete → block, point to `/mochiko:plan`. A selected feature ordered earlier and not yet `delivered` blocks — one run per feature, in dependency order." · Design-inputs clause "under `features/FEAT-XXX/`, the spec-root `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md`, `nfrs.md`" · Reports "under `features/FEAT-XXX/` in the spec folder" · Acceptance-landing binding head "**Acceptance landing — map bookkeeping:** at user acceptance, the same landing that folds `ARCHITECTURE.md` executes the map's graduation batch" (the unconditional form — the graduation-batch text itself survives whole under the selection-scope branch)
- **Kept deliberately:** the full graduation batch verbatim under selection scope (status flip dated · marked deltas fold into extent lines · `FEATURES.md` index touch · `ARCHITECTURE.md` In-flight pointer clear · derived specs-index closure · no separate feature-close stage) · TDD cycle discipline and every craft-skill binding (`mochiko:executing-tdd-cycle` with cycle-report as uncertainty carrier, `mochiko:brownfield-integration`, `mochiko:testing-end-user`, the `mochiko:review-code-minimalism` lens with advisory-only `minimalism:` findings) · Bounds/attempt machinery, no-progress stop, graded-head rule, delta-grade scoping, cold-snapshot final validation with the worktrees ignore entry · verification seats, evidence rules, real-infrastructure gates · regression scope (accumulated TEST gates + later-landing seam ownership) · architecture-deviation consent + built-vs-approved diff · escalation cadence · KM-landing dual-target `ARCHITECTURE.md` fold · human approval gates (plain blocking acceptance, never timed) · no-git-mutation lines. Pure additions riding the decision row: the graded-fold not-done state, the repeat-run append rule, the Baseline-touches binding.
- **Consumers assessed:** none mount commands — entry points. Ripple assessed: `plan` (same cluster, same stamp) · the new feature command (dispatches lane work here; the landing verification seat's extended scope lives in this file, referenced not restated there) · `specify`/`setup` (parallel seat this wave) · the router skill (unaffected) · `mochiko:authoring-feature-map` (carries the graduation-batch and delta-fold grammar; the parallel map builder owns it this wave).

## [v0.58.0] Slice scope superseded by per-feature runs; acceptance landing absorbs map bookkeeping
- **Disposition:** superseded → the feature-scoped rewrite in the same file: one run per feature (`$ARGUMENTS` = FEAT-XXX, package under `features/FEAT-XXX/`), the Regression-scope binding (accumulated delivered-feature TEST gates + later-landing-feature seam ownership), the Acceptance-landing map-bookkeeping binding (status flip · delta fold · `FEATURES.md` index touch · In-flight pointer clear · derived specs-index closure), machinery per `mochiko:authoring-feature-map`
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D13/D17/D18; D13 dissolves feature-close into this landing)
- **Content:** header Goal line "`$ARGUMENTS` = optional feature ID; empty → resolve from `.mochiko/specs/` and confirm with the user." · Reports binding "under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped" · the whole **Slice scope** binding: "**Slice scope** (the spec's Delivery Slices section holds a decomposition; its Graduation contract governs): the run reads `slices/<slice>/tasks.md`; quality gates still run the full repository suite; when the last slice clears, the feature is declared, not verified — Feature-Done executes at feature-close, surfaced as the next step, never reported complete here." · KM-landing clause "— slice-scoped, the fold is dual-target (feature-root `architecture.md` accumulates the approved delta) … at feature close the In-flight pointer is removed."
- **Kept deliberately:** v0.56.0 Bounds + Escalation-cadence bullets verbatim, all snapshot-isolation machinery (graded-head rule, delta-grade scoping, cold-snapshot final validation) untouched · TDD + real-infrastructure verification with evidence and exit codes · architecture-deviation consent + built-vs-approved diff · the `ARCHITECTURE.md` fold itself (still dual-target — the second target re-keyed feature-root → `features/FEAT-XXX/architecture.md`) · full-repository quality-gate suite (was the slice binding's live half, now in Regression scope) · the In-flight pointer removal (relocated from KM landing into the acceptance-landing batch, same landing per D13) · no-git-mutation + plain-blocking-acceptance lines. Feature-Done's obligations dissolve into this landing per D13 — per-feature SC coverage is the TEST-gate verification; cross-feature seams are owned here by the later-landing feature.
- **Consumers assessed:** none — commands are entry points, nothing mounts them. Ripple noted: plan (same wave) · authoring-feature-map (carries the landing batch grammar, wave 1) · testing-end-user (gate parsing unchanged).

## [v0.49.0] Task-checkbox progress superseded by per-card checkboxes; builder decomposes
- **Disposition:** superseded → Goal "every `tasks.md` cycle card is `[x]`" + decompose-at-build clause; Bindings deliverable/inputs/slice-scope re-keys
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D2.1+D6)
- **Content:** Goal "Every `tasks.md` task is `[x]`" · Not-done "an unchecked task" · Bindings "`tasks.md`'s checkboxes (`T{N}.{X}` namespace) are the progress surface, flipped as tasks complete" · `task-mapping.md` in Design inputs · Slice-scope key "(accepted `slices.md` present)".
- **Kept deliberately:** TDD-built + real-infrastructure verification, cold-snapshot final validation, architecture-deviation consent, KM landing, all acceptance gates — untouched. `spec.md` added to Design inputs (cards cite acceptance-criteria IDs the builder resolves).
- **Consumers assessed:** executing-tdd-cycle + testing-end-user (both re-keyed same wave) · router · ARCHITECTURE.md implement section.

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's all-tasks-[x], TDD, per-cycle + whole-run real-infrastructure verification with evidence and exit codes, traceability/governance alignment, built-vs-approved diff condition · implementation-verification seat split (Independence line) · diagram-anchored deviation check with user consent, never silently built · package entry gate · governance obligated-read brief line · craft-skill bindings (executing-tdd-cycle with cycle-report as uncertainty carrier, brownfield-integration, testing-end-user) · design-inputs list · cold dependency-cold snapshot verification with the worktrees ignore entry (validator snapshot-isolation intent preserved) · slice full-suite + feature-declared-not-verified rules · KM ARCHITECTURE.md fold · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Execute an accepted task breakdown into working, verified code via an independent producer→verifier team loop — a staff-engineer seat implements each cycle through red/green/refactor TDD (foundation cycles before feature cycles) and fix-passes the final validation; a qa-engineer seat verifies every cycle and then the whole implementation against real infrastructure with captured evidence and quality-gate exit codes; the approved architecture is briefed input, guarded by a diagram-anchored deviation self-check at cycle open and close and by a built-vs-approved diff at final validation. A per-cycle checkpoint carries a deterministic-clean devolved branch; a named final-acceptance gate closes the run. Package-gated, cycle-by-cycle, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown (Cycle-by-Cycle, Foundation → Feature)

**Goal:** turn an accepted `tasks.md` into working, verified code — one cycle at a time, foundation
cycles before feature cycles, each implemented through red/green/refactor TDD and independently
verified against real infrastructure, until every cycle clears and a whole-implementation final
validation passes. `$ARGUMENTS` = optional feature ID or description; empty or
detected-from-workspace is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate; the
one exception is the cycle checkpoint's devolved clean branch below. Every dispatch carries its
own brief in the spawn or send prompt — the seat's role and skill (named as a hint, the agent
decides fit), the exact inputs to Read, where the output lands (write vs return), the bar it
must clear, its peer edges and holds, and the independence reminder that matches the seat
(author: never grade your own output; grader: read the artifact itself, default FAIL, quote
evidence) — the seat owns none of this context and gets all of it from you; on a retry, a
peer-routed gap list is pointed at and the round opened, a relayed one pasted verbatim.
This file is self-contained: implement's whole contract lives here.
**First-spawn probe:** the `staff-engineer` producer — foundation cycle 1 is
implemented before anything verifies it.

## Goal

Every `tasks.md` task is `[x]` with its `cycle-report.md`; a verification report per cycle and one for
the whole implementation, each naming its real-infrastructure evidence tree and its quality-gate exit
codes; the built code meets its criteria, holds traceability to requirements, and aligns with the
project's governance; where an approved structural delta existed, its built-vs-approved diff report
exists and any divergence it names was ruled at G5; the KM landing ran; and the user accepted at G5.

**Not done:** an unchecked task, or a cycle with no report · a failing quality gate · a cycle or the
final validation with no real-infrastructure evidence · a warm-only final validation · a surfaced
architecture deviation neither built as approved nor consented as an amendment · an approved delta
with no built-vs-approved diff report · a departure with no trail line · out of rounds · G5
unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration` | implements each cycle through red/green/refactor TDD → `cycle-report.md`; targeted retry of only the failed tasks; the final-validation fix pass, unconstrained by cycle boundaries; reports its architecture deviation self-check; never verifies | standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation cycle 1 | hands each completed cycle straight to the verifier; retries and fix passes are dispatched by you |
| verifier | `qa-engineer` × `testing-end-user` | verifies each cycle, then the whole implementation, against real infrastructure — executes the cycle's `**TEST:**` tasks, runs the quality gates, captures evidence → a verification report naming its evidence tree + a checkpoint recommendation; never implements | cold at the first cycle verification, standing after | peer-edged with the producer for cycle hand-offs; the endgame is lead-routed |
| arch-diff | `principal-architect` × `authoring-architecture`, diff mode | reports built vs. approved — "built as approved", or the divergence | disposable, at final validation, whenever an approved structural delta existed | none — never the verifier seat |
| arch-scribe | `principal-architect` × `authoring-architecture` | folds the resulting system into `ARCHITECTURE.md` | disposable, at finalize, on a built structural change only, per the KM landing | none |

**Validation model:** the loop's bounded in-loop critique — qa's per-cycle verification plus the
final validation, unsized by design. The verification skill is **never** mounted on the producer.
Outside the devolved branch qa's output is **lead-adjudicated input** and the verdict is yours. One
verifier, so implement numbers no G2 — there is no feasibility-rejection gate. No seat ever grades
its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`** — a
nameless spawn is a one-shot subagent, the forbidden transport; every later round is a
`SendMessage` to that same named seat. Verify from the roster: the `members` array in
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an agent team;
failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn prompt names
the skill and role itself. Tell the user up front they can watch or message any teammate; announce
each seat in one line when filled; never narrate or reply to teammate housekeeping. A peer-routed
hand-off is **not a start signal** — the producer revises only when you open the next round, and
your brief carries that hold.

**Seat lifecycle:** the counted unit is the **cycle**; at each gate pause count each standing
seat's completed cycles and recycle at ~≥3 — counted, never observed; the user may order a recycle
at any gate. **Override —** the verifier recycles per
**slice** boundary, its final-validation incarnation additionally briefed from the on-disk
verification reports. A retry or fix-pass respawn carries the failed-task list **and** the
just-failed `cycle-report.md`, relayed at dispatch: the next attempt overwrites that file. A
respawn is a reset: briefed from the on-disk artifact set alone, versioned successor name
(`producer-2`), never the dead seat's bare name. End-of-need shutdown; no ritual sends.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS` · rules: the user · decides: the resolved `<feature>` (an
  explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the
  user before the run opens).
- **Package gate** — evidence: `tasks.md` present and complete alongside the accepted `plan.md` and
  `architecture.md`, plus the design inputs and `slices.md` (Bindings) · rules: the user · decides:
  whether the run opens. Missing or incomplete → block, pointing the user to `/mochiko:plan`.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  breakdown — **reversibility** (rework cost if the build is wrong) · **blast radius** (how much
  downstream work reads the built code as authoritative) · **precedent** (first-of-kind, or
  mirroring an audit-cleared pattern) · **input confidence** (scored on the artifact under review;
  a user ruling discounts ambiguity risk only, and one introducing new surface raises consistency
  risk) — plus the process you compose from it — the stated default below, or your departures from
  it · rules: the user · decides: the run's composed process. Rigor scales with
  cost-of-being-wrong, never task size; diff size is at most a hint.
- **Governance surface** — evidence: `CLAUDE.md`'s `<!-- mochiko:governance:begin -->` region ·
  rules: the user, when it is absent · decides: proceed on governing context, or run
  `/mochiko:setup` first. Absence is **surfaced, never auto-resolved** — governing context, not a
  blocking gate. Present → each code-touching brief carries the one-line obligated read naming the
  `.claude/rules/mochiko/` files relevant to that cycle's file paths.
- **Cycle checkpoint** — evidence: `cycle-report.md` (deviation self-check, `domain_deps_added`), the
  verification report, qa's classified evidence + recommendation · rules: you, except on the devolved
  branch · decides: the cycle advances, or a targeted retry. It carries this command's **devolved
  branch**, skipped **exactly** when every verification in the cycle is a deterministic CLI check at
  100% pass **and** no deviation is reported **and** `domain_deps_added` is empty **and** both
  reports are clean by the envelope's prose check (`report-format.md`): the cycle then
  clears on qa's PASS-with-evidence, unread by you, counted from its one-line clearance notice.
  Otherwise it fires — any failure, any GUI or subjective verification, any reported deviation, any
  registry addition, any prose on a passing report — and you rule on the reports.
- **Architecture deviation** — evidence: the producer's diagram-anchored self-check, run at cycle
  open **and** cycle close — does this cycle add or remove a box, add, remove or redirect an arrow,
  or move a responsibility across a boundary on the approved diagram? · rules: the user · decides:
  build as approved, or a consented amendment of `architecture.md` before the cycle resumes. A yes
  stops the cycle and you present it — never silently built.
- **G3 clarification** — evidence: an ambiguity or blocker the producer flags · rules: the user ·
  decides: the answer fed forward into the next dispatch, logged in-session. You route each
  finding by judgment: **a genuine judgment call is ruled here**; a gap answerable by
  investigation routes to a native `Explore` pass (the "Research this" branch), never to the
  user; work bigger than the run was framed is G4's.
- **G4 exit-early / escalation** — evidence: a cap trip, a failing set unchanged round-over-round,
  `IMPLEMENT_STOP`, or a scope gap · rules: the user, on the last evidence · decides:
  continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the user
  explicitly accepts. Neither G3 nor G4 ends the loop on its own.
- **G5 final acceptance** — evidence: your clearing verdict on the final validation — the cycle /
  task / fix-pass counts, quality-gate results, an evidence summary, any noted gaps — **and** the
  built-vs-approved architecture result where an approved delta existed · rules: the user · decides:
  accept (done) / amend (the changes become the failure list; re-enter the relevant cycle or fix
  pass, still bounded, and clear a verdict again) / reject (the work remains under
  `.mochiko/specs/<feature>/` and in the working tree).
- **Floor gates:** the run-start weight card · the package gate · the governance surface's absence
  ruling · **G3**'s judgment-call ruling · **G4** · **G5** · the architecture-deviation consent — the
  user's whatever you compose, never departable. G1 and the cycle checkpoint (yours by definition)
  are not. Batch rulings into the fewest checkpoints that respect these gates. **Verification depth
  is floored:** it may thin on a light cycle — quality gates plus spot
  evidence rather than full evidence capture — never to zero. No cycle closes without
  real-infrastructure evidence; *none* is a reviewer-count option, never a verification one. No
  lead-penned surface takes a standing cold grade here: the uncertainty carrier is
  producer-authored — were you to pen a deliverable surface, it would take one cold-seat grade
  non-discretionarily, waivable only by recorded user waiver at the weight card.
- **Bounds:** **targeted retry** — trace a checkpoint failure to its tasks and re-open only those,
  **max 3 attempts per cycle**, never regressing passing code; **fix pass** — failure-scoped after a
  final-validation failure, **max 3 passes**; **convergence stall** — the same failure pattern
  across **2+ rounds** surfaces rather than silently continuing, no-progress being an unchanged
  failing set; kill-switch `.mochiko/specs/<feature>/IMPLEMENT_STOP`, checked before each seat send.
  You count every round. Any bound this run declares — including a declared cost range — has you as
  its named counter, **rises only at a user checkpoint**, and is re-declared only on the record;
  busting a bound escalates, never silently continues.
- **Ordering invariants:** cycles run in dependency order, **all foundation cycles before feature
  cycles**, the current cycle being the first with unchecked tasks. **Sequential-only** — parallel
  cycle execution is a `deliberate-shortcut-ledger` deferral, not a capability drop. Every produced
  cycle is paired with a verification in the same round, never skipped: the hand-off is peer-routed,
  the pairing is yours to enforce. The final validation is lead-routed, never devolved.
- **Cold tree:** the final validation builds and runs the quality gates from a dependency-cold
  snapshot of the **uncommitted working state** — `git ls-files -co --exclude-standard
  :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`, carrying no warm items —
  its results part of G5's evidence.
- **Per-cycle qa isolation:** yours to compose per run — that snapshot plus a declared carry-set of
  warm gitignored items, dependencies **copied or installed, never linked**. Tear **either**
  snapshot down only after its evidence is captured and any snapshot-only failure dispositioned; a
  failed cycle's, after its retry.
- **Scaffolding:** from the detected stack, create any missing ignore files (`.gitignore` /
  `.dockerignore` / lint-ignore) and the `/.claude/worktrees` ignore entry, project-relative, once
  before the cycle loop.
- **Slice scope** *(when an accepted `slices.md` exists)* — that file's **Graduation contract** is
  the single home for slice resolution, the staleness guard, scope, extend-mode, graded amendment,
  and layout; not restated. implement's own bindings on top: the package gate and the cycle loop read
  `slices/<slice>/tasks.md`; the design inputs are the shared feature-root artifacts plus
  `slices/<slice>/{plan.md, task-mapping.md}`; the quality gates still run the **full repository
  suite** — earlier slices' tests are the regression net that catches a design amendment breaking
  shipped behavior; and when the last slice in Slice-order clears G5 the *feature* is **declared, not
  verified** — `slices.md`'s Feature-Done section executes at feature-close, owned by no workflow
  yet: surface it as the next step, never report feature completion here.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push — the ban's surface is refs,
  index, tracked content, and history, so the ephemeral, self-removed verification snapshot above
  is not a mutation of it. No internal machinery vocabulary in user-facing prose — the
  conversation is yours and the user's, in the mochiko register (`templates/output-style.md`).
  User acceptance is plain blocking text, never a timed prompt. Reports are written as the work
  progresses, never reconstructed at the end.

## Bindings

- **Deliverable:** the **working code**, in `tasks.md`'s cycle / task (`T{N}.{X}`) ID namespace, whose
  checkboxes flip `[ ]` → `[x]` as tasks complete.
- **Reports** under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped, where the
  Goal's artifact set reads them: `cycle-report.md` and a verification report per cycle, the
  final-validation report, the built-vs-approved diff report.
- **Design inputs:** `plan.md`, `architecture.md` (the **anchor** for both architecture mechanisms),
  `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`,
  `requirements.md`. Workspace-as-state, no registry field.
- **Uncertainty carrier:** producer-authored — `cycle-report.md`'s honest difficulties, deviations,
  `domain_deps_added` and flagged blockers (`mochiko:executing-tdd-cycle`'s format); qa's evidence
  and status live in its verification reports.
- **Fact route:** real infrastructure — executed `**TEST:**` tasks and quality-gate exit codes;
  knowledge gaps go to a native `Explore` pass.
- **Clearing unit + checkpoint keying:** the **cycle**; a surfaced architecture deviation
  **de-devolves** it, and a non-empty `domain_deps_added` **always** forces the escalated human
  checkpoint — never auto-approved, no stamp read.
- **Run-start declaration:** one line at the head of `tasks.md` — the deliverable's progress surface,
  where Recovery already notes the resume stage — for a default run; a run that departs from the
  stated default, or declares non-default bounds, writes a departure record at
  `.mochiko/specs/<feature>/implement-contract.md` beside the reports instead — the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume. Counted unit: the
  **cycle**, the unit the bounds and the lifecycle cadence already count.
- **Departure trail:** one line per departure from the stated default, appended under that same
  `tasks.md` declaration as it is taken and carried into G5's evidence — never your context alone;
  the trail names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its ritual + invariants
  under fix-on-sight; a **built** structural change folds the built system into `ARCHITECTURE.md`.
  No copy → skip.

## Recovery

Note the resume stage on the deliverable, with the run's counter state — cycles and rounds
consumed · bounds declared · departures taken. Sessions and teams do not survive `/resume`, and a
shared account limit can throttle the team and the main session together — escalation then has
nowhere to go but pause. Resume from workspace evidence, never a context `phase` field, respawning
only what the stage needs — a respawned producer re-reads the cycle's tasks, the design inputs,
and any failed-task list, and a respawn is cold by design.

| Evidence in the workspace | Resume at |
|---|---|
| no `tasks.md`, or an incomplete package | entry blocked |
| `slices.md` present | resolve the current slice; the rows below then read `slices/<slice>/tasks.md` and per-slice reports |
| `tasks.md` present, ignore files absent | scaffolding |
| unchecked tasks remain, the current cycle has no `cycle-report.md` this round | implement the current cycle |
| the current cycle's `cycle-report.md` present, no verification report this round | verify the current cycle |
| a surfaced deviation unruled | the architecture-deviation gate |
| the current cycle not passed, within the cap | retry / cycle loop control |
| all tasks `[x]`, no final verification report | final validation |
| final validation failed, within the cap | fix pass / loop control |
| final validation cleared, an approved delta existed, no diff report | the built-vs-approved diff |
| final validation cleared, not yet accepted | G5 |
| accepted | finalize — report the code and reports, the per-cycle and fix-pass round counts, the cycle / task / fix-pass counts with quality-gate status, the KM landing, a suggested commit (`feat: implement <feature>`), and the next step |
| `IMPLEMENT_STOP` present | escalate (G4) |
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left; "the shape's devolved branch" re-attributed to "this command's devolved branch" (description + cycle checkpoint); G3's gap-taxonomy vocabulary reworded to plain lead-judgment routing.
- **Kept deliberately:** the devolved branch's four conditions (already inline, unchanged) — plus all gates/bounds/bindings/recovery and the inlined weight-card factors, floor rules, transport, lifecycle, mesh hold, ground rules (with the snapshot ban-reading carve-out), counter-state recovery.
- **Consumers assessed:** none.

---
**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Per-cycle isolation rationale pointer
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
Triggers, evidence provenance, the git-dependent-gate fallback,
  and rationale: `.mochiko/brainstorms/validator-worktree-isolation/record.md` (D3–D7).
```
- **Kept deliberately:** every operative clause of the isolation binding — lead-composed per run, the declared carry-set, deps copied-or-installed-never-linked, and the teardown ordering.

## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/implement.md` **16,046 → 16,021 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the first v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4**
ruling **convert-on-touch** and all six commands staying v6-form. implement's conversion was
**deferred by user ruling at the v0.42.0 touch** — that wave was ruled surgical and v6-form, the
**F66** trigger left live and the conversion routed to a dedicated wave. This is that wave; the
F66 deferral is discharged. BACKLOG: "convert-on-touch residuals".

It also carries the **first-conversion ceiling-term obligation**, user-ratified at the v0.40.0 wave
close (2026-08-01) — *the first conversion measures its Constraints/Bindings blocks and lands any
needed check-6 term in the same wave* — which discharges item 4 of the v0.40.0 shape note's
*Deliberately not encoded* list. Both terms landed:
`.mochiko/strips/validation-command-shape.md` [v0.43.0].

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded, per
check 6): preamble **114/130** (published as 118 while the 4-word form marker stood;
retired at the wave close) · Goal **145/150** · Seats & checks **324/340** (unchanged) ·
Constraints **1031/1110** · Bindings **263/290** · Recovery **239/242** (unchanged). Term
derivation as check 6 requires: **G = 9** — the eight prior gate lines plus the run-start weight
card, all nine carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so
Constraints is 90·(9+2) = 990 **plus the new +120 P18 term** = 1110. **S = 4** and **R = 13**, both
unchanged. **A = 5**, unchanged from the v0.42.0 reading (working code · `cycle-report.md` · the
per-cycle verification report · the final-validation report · the built-vs-approved diff report),
so Bindings is 90 + 12·5 + 30 (KM) **plus the new +110 P19/P20 term** = 290.

> **One A-term judgment, recorded so the next auditor does not re-derive it.** P19 names
> `implement-contract.md` as a **departing** run's per-run carrier. It is **not counted in A**: it
> is neither a deliverable nor a round report, and it exists only on a departing run. Counting it
> (A = 6) would raise the Bindings ceiling to 302 and so only loosen the check — the conservative
> reading is the one measured here.

## [v0.43.0] The Goal's end state loses its seat choreography and its lead-read clause
- **Disposition:** superseded → rewritten in place as artifact state. The verified evidence the end
  state named **survives as the artifacts that carry it** — a verification report per cycle and one
  for the whole implementation, each naming its real-infrastructure evidence tree and its
  quality-gate exit codes.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded
  by `validation-command-shape` check 23, v7-form only).
- **Protected content, leaving by ruling and named as such:** the second clause is
  `DECISIONS.md`-traceable — the [v0.31.0] entry below records the team-method **D3** rewrite of
  the Contract done-condition into exactly this text ("the final-validation report plus every
  **escalated** cycle's reports"). It is superseded by a cited ruling, not dropped.
- **Content (v6, verbatim — the two clauses that left):**
  ```
  `qa-engineer` verification passed on every
  cycle **and** on the whole-implementation final validation, on real-infrastructure evidence and
  quality-gate exit codes; you Read the final-validation report and every escalated cycle's reports and
  found no blocking gap —
  ```
- **Kept deliberately:**
  - **All four build-state findings**, in substance verbatim: "criteria met, gates passing,
    traceability to requirements holding, the build aligned with the project's governance" → "the
    built code meets its criteria, holds traceability to requirements, and aligns with the
    project's governance", *gates passing* carried by the quality-gate exit codes named one clause
    earlier. Nothing in the finding set was dropped.
  - **The real-infrastructure evidence and quality-gate exit codes** — moved from *what qa did* to
    *what the reports contain*, so the Goal and the D5 verification floor (now in P18) read the
    same evidence.
  - **The not-re-read consequence of the devolved branch** — untouched at the Cycle-checkpoint
    constraint the v0.35.0 ledger assigned it to: "the cycle then clears on qa's PASS-with-evidence,
    **unread by you**".
  - **"the KM landing ran" and "the user accepted at G5"** — both are explicit end-state elements
    in the shape's own Goal spec (the KM landing under fix-on-sight; user acceptance as part of the
    end state), so neither reads as residue.
- **Consumers assessed:** not a shared primitive. Two cross-file consumers checked: the grader's
  check 23 (this is the text it was written for — `.mochiko/strips/validation-command-shape.md`
  [v0.40.0]) and the five remaining commands, whose Goal blocks **stay exactly as written** — the
  residue clause is v7-form-only and each converts at its own touch (shape note [v0.40.0], *The
  Goal block's process residue left the end state*, Consumers assessed).

## [v0.43.0] The not-done state `a non-clean cycle advanced without your verdict`
- **Disposition:** superseded → deleted from the Goal. The rule it echoed is unchanged at its
  ledgered home, the **Cycle checkpoint** constraint: "Otherwise it fires — any failure, any GUI or
  subjective verification, any reported deviation, any registry addition — and you rule on the
  reports."
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). It made the lead's own
  process step a done-condition element, which is the residue class check 23 fails; and it can
  never be rescued as a floor gate, because the cycle checkpoint reads `rules: you`, not `rules:
  the user`, and check 21's floor-gate test keys on the latter.
- **Protected content, leaving by ruling:** `DECISIONS.md`-traceable to team-method **D3**
  ([v0.31.0] below). The v0.35.0 CS-D8 ledger assigns that row's home to the **Constraints**
  cycle-checkpoint line — "Same line's *Otherwise it fires — …*, all four classes enumerated" — not
  to this Goal state. The protected content stays where the ledger put it; only the Goal echo left.
- **Content (v6, verbatim):** `a non-clean cycle advanced without your verdict`
- **Kept deliberately:** the whole devolved-branch predicate and its four escalation classes, in the
  Cycle-checkpoint constraint; the lead's verdict ownership, in the validation-model line.
- **Consumers assessed:** as above — not a shared primitive; grader check 23 and the five v6-form
  commands, both unaffected.

## [v0.43.0] Two not-done states re-read from process to artifact state
- **Disposition:** superseded → rewritten in place. The same states, named by the artifact that is
  missing rather than by the step that did not run.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above).
- **Content (v6, verbatim → v7):**
  - `a cycle or the final validation unverified` → `a cycle or the final validation with no
    real-infrastructure evidence`
  - `an approved delta whose diff never ran` → `an approved delta with no built-vs-approved diff
    report`
- **Protected content:** the second is `DECISIONS.md`-traceable to **AD-D6.3/R8**, whose v0.35.0
  ledger row names *the Goal's end state and not-done state* among its four resolution homes. Both
  homes survive in the converted Goal — the diff report exists (end state) / an approved delta with
  no diff report (not-done) — so the row is **preserved**, and this entry records a rewording, not
  a supersession of the row. The first state is re-anchored on the D5 verification floor now stated
  in P18, which is what "unverified" always meant here.
- **Kept deliberately:** **`a warm-only final validation`** — untouched, exactly as the [v0.42.0]
  entry below logged it ("Warm-only as a named not-done state — untouched in the Goal block"); and
  every other not-done state, unedited.
- **Consumers assessed:** as above.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 8 to 9**.
- **`**Floor gates:**`** (P18) — the floor set (the run-start weight card · the package gate · the
  governance surface's absence ruling · **G3**'s preference ruling · **G4** · **G5** · the
  architecture-deviation consent) with the non-floor two named, so the absence is stated rather
  than inferred; the **D5
  verification-depth floor** at its named natural site (`workflow-token-reduction` **D5** as split
  at S5, ruled 2026-07-23, no-softening confirmed — all three limbs carried: depth may thin on a
  light cycle, never to zero, and *none* is a reviewer-count option only); and the
  lead-penned-surface element stated as an **absence**, implement's P11 being producer-authored.
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration on the deliverable's progress surface for a default run, an instantiated
  `implement-contract.md` for a departing one, and the **cycle** named as the counted unit (check
  22), the same unit the Bounds and the P17 lifecycle line already count.
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance.

**Two judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is seven of nine, and the ground is *who rules*, not how heavy the gate
   is.** *(Corrected at the audit's fix round 1 — the governance surface moved into the floor set;
   the original six-of-nine reading is superseded, see the axis note below.)* The two left as
   departable defaults are the two whose ruling was never the user's to lose: **G1 entry** and the
   **cycle checkpoint** (`rules: you`, so check 21's test excludes it by construction). **G3 is
   marked floor on its narrow limb only** — the *preference ruling* is the user's under floor
   invariant 1; *when* it is presented stays the lead's under **D3**'s consolidation authority,
   which is home doctrine and is deliberately not restated in the command.

   **Why G1 clears, stated on the ground that actually carries it.** The first draft argued only
   the explicit-ID case (an explicit `<feature>` in `$ARGUMENTS` leaves nothing to rule), which is
   too narrow — it says nothing about the detected-feature branch. The clearing ground is
   structural: **the package gate is floor and its evidence is the resolved feature's package**,
   deciding "whether the run opens". So a lead that composes out G1's confirm still puts the
   resolved feature in front of the user before the run opens, on the very next gate. G1's confirm
   is a convenience stop, not the invariant's only carrier — which is exactly what makes it safely
   departable and the package gate not.

   **Why the governance surface is floor: the blocking and floor axes are independent, and the
   first draft conflated them.** *Blocking* asks whether the run stops; *floor* asks who rules.
   This gate is **non-blocking AND floor** — its own protected text ("Absence is **surfaced, never
   auto-resolved** — governing context, not a blocking gate") settles the first axis and says
   nothing about the second, while `rules: the user, when it is absent` settles the second
   outright. Excluding it produced a contradiction inside one block: a gate reading `rules: the
   user` sitting in a not-floor list whose own sentence defines floor as never-departable. The
   alternative repair — reading the surfacing as departable — was **not taken**: it would be a
   behavior change to a protected pre-wave line, which no ruling in this wave authorizes.
2. **The declaration and the trail share one surface.** Both land at the head of `tasks.md` — the
   deliverable's progress surface, where Recovery already notes the resume stage, so a resumed lead
   finds declaration, departures and resume state in one place. **`cycle-report.md` was rejected as
   a home:** the P17 lifecycle line records that the next attempt **overwrites** that file, so a
   trail parked there is a trail that can vanish mid-run.

**Recovery left untouched, deliberately.** The shape's counter-state clause is home doctrine (v7
Recovery block); implement's pause line — "Note the resume stage on the deliverable" — does not
contradict it and names the same surface P19 binds, so no edit was owed.

### R21 heavy-site measurement — the first measured v7 conversion

`lead-owned-process-flexibility` **R21** carries a recorded-open obligation: *a measured cost
estimate for declaration + trail + composition on one light and one heavy run* (verify N3, narrowed
by **A3** to the estimate alone). implement is the **heavy site** — the library's densest command.
Figures are `wc`-measured after the last edit (**re-measured at the audit's fix round 1**, which
added 3 w to the P18 binding — the superseded figures were 16,028 B / +1,526 / Constraints 1028,
recorded here so the drift is traceable rather than silently overwritten). **No offsetting saving
is claimed, because none exists:** the Goal strip returned 67 B against 1,611 B of additions.

**File growth.** `commands/implement.md` **14,502 → 16,021 B** (+1,519; words 2,014 → 2,242,
+11.3%). Attribution, each construct measured on its own text:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +266 | +44 |
| `**Floor gates:**` — floor set + D5 depth floor + P11 absence (P18) | +674 | +100 |
| `**Run-start declaration:**` (P19) | +473 | +61 |
| `**Departure trail:**` (P20) | +173 | +27 |
| Goal block, D6(b) residue strip | −67 | −4 |
| **net** | **+1,519** | **+228** |

**Per-run read cost.** implement.md is an obligated read once per run, so this is **+1,519 B on
every implement run** — 10.5% on top of the command itself. It is not the whole delta a run pays
this wave: the shape-home edits add **+450 B to `command-shape.md`** (31,816 → 32,266 — the
v6-form weight-card clause 236 B, its footer stamp 213 B), and *that* one is paid by **every
team-form run of any command**, converted or not, because the shape home is the shared always-read
floor. Against v7's own doctrine cost (+11,399 B/run, measured at
v0.40.0), conversion is the small half of the bill.

**Run-time cost of declaration + trail — an estimate, and marked as one.** Three components, none
yet observed on a live run:

- **The declaration, every run.** One line on `tasks.md` stating the four-factor read and the
  composed process. At the density this repo's own cards use, ~30–60 words (~200–400 B), produced
  once and re-read on every resume. It is the only one of the three a **default** run pays.
- **The trail, per departure.** ~15–25 words (~100–170 B) a line. A run that takes the stated
  default pays **zero**, and the cost scales with departures — the intended shape: the lead buys
  flexibility by the line.
- **The contract, departing runs only.** `templates/workflow-contract.md` measures **5,572 B**
  today, so a departing run reads 5.6 KB and writes a filled copy of comparable size to
  `.mochiko/specs/<feature>/implement-contract.md`. The largest run-time item by far, and
  **conditional by construction** — no default run touches it.

**The honest read at this site.** A default implement run pays the +1,519 B read plus one
declaration line — ~1.8 KB, on the command carrying the library's largest protected surface. A
departing run adds ~5.6 KB of template plus its fill, plus a line per departure. **The light site
stays unmeasured**, so R21 remains open at half; the next conversion of a light command closes it.

---

## [v0.42.0] Cold checkout's **fresh clone** superseded by the git-semantics filtered snapshot

- **Disposition:** superseded → the `**Cold tree:**` + `**Per-cycle qa isolation:**` constraints
  that replaced it, plus the extended `**Scaffolding:**` constraint and the verifier seat row.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/validator-worktree-isolation/record.md`
  **D3(i)/D4**, with D5–D7 for the option, carry-set and teardown; `DECISIONS.md` row
  2026-08-01 "Validator snapshot-isolation ruled"). Not a minimalism strip: the line was made
  wrong, not verbose — executed against an uncommitted implementation, a clone of HEAD gates a
  tree that does not contain the work under validation (F22–F24).
- **Content:**
  ```
  - **Cold checkout:** the final validation builds and runs the quality gates from a **fresh clone**
    of the repository, never only the warm working tree; that clone's results are part of G5's
    evidence.
  ```
- **Kept deliberately:**
  - **The works-warm-only catch** — the whole ground of the 2026-07-31 ruling. The snapshot drops
    gitignored dirs exactly as a clone does, so the catch is preserved by mechanism, not by
    assertion (record F14/F15, F32/F57c).
  - **G5 evidence status** — "its results part of G5's evidence" survives verbatim in intent.
  - **Warm-only as a named not-done state** — untouched in the Goal block ("a warm-only final
    validation"). See the deviation note below on where the Constraints echo went.
- **Consumers assessed** (the three-carrier set enumerated at D3(i), one disposition each):
  - `plugins/mochiko/commands/implement.md` — **edited** under this entry (the carrier being
    superseded).
  - `.mochiko/decisions/2026-07-31-team-method-escalations-closed.md` — **marker appended**
    ("Superseded-mechanism note (2026-08-01)"); the ADR's existing text is untouched, intent
    standing.
  - `DECISIONS.md`, the 2026-07-31 cold-checkout row — **annotated** in the same landing
    ("cold-checkout *mechanism* superseded 2026-08-01 → the validator snapshot-isolation row;
    intent stands").

**Two ceiling-forced deviations, recorded rather than silent.** implement's Constraints ceiling is
90·(G+2) = **900 w** at G = 8, and the block stood at **817 w** before this wave — 83 w of headroom
against a ruled addition set (mechanism · per-cycle option · carry-set · teardown · fallback ·
evidence provenance) that measured ~136 w at first draft. Both moves below were made to land the
ruling inside the floor rather than ship a check-6 FAIL; neither drops a responsibility.

1. **"never only the warm working tree" is not restated in Constraints.** The prohibition survives
   as the Goal block's not-done state, which this wave did not touch. Dropping the Constraints echo
   removed a duplication; the mandatory phrasing of the replacement ("builds and runs the quality
   gates from a dependency-cold snapshot") carries the same obligation at the point of use.
2. **U7's report-provenance obligation landed in the Seats & checks verifier row**, not Constraints —
   "→ a verification report **naming its evidence tree** + a checkpoint recommendation" (+5 w;
   Seats 319 → 324 against its 340 ceiling). That row covers both reports U7 names, per-cycle and
   final, since the same seat produces both. The variant/carry-set enumeration ("warm, or snapshot
   path + variant + carry-set") is single-sourced to the record and reached from the Constraints
   reference, which names evidence provenance explicitly.

Post-edit measurement, all blocks: preamble 114/130 · Goal 149/150 · Seats & checks 324/340 ·
Constraints **887/900** · Bindings 175/180 · Recovery 239/242. G unchanged at 8 (neither new bullet
carries the three-part `evidence:`/`rules:`/`decides:` form), so the ceiling term is unchanged.

**Two measurement notes, so the next auditor does not re-derive them.**

1. **The Bindings term is A = 5**, not the **A = 6** this file's v0.35.0 entry measured.
   `validation-command-shape` check 6 now bars counting a **KM-landing fold target** — a doc the
   command folds *into* rather than produces — as an own-output, which removes `ARCHITECTURE.md`
   from the set. What remains: working code · `cycle-report.md` · the per-cycle verification
   report · the final-validation report · the built-vs-approved diff report. Ceiling
   90 + 12·5 + 30 = **180**, measured **175**. **This retires the v0.35.0 entry's "At-risk
   measurement" flag** (`Bindings passes at A=6 and A=5 but fails at A=4`): the failing case needed
   the built-vs-approved diff report discounted *as well*, and the skill bars only the fold target,
   so A = 4 is not reachable under the written rule. The flag is answered, not re-argued.
2. **The "preamble 114/130" figure above counts the `# ` title line; strict body-only is 103.**
   Check 6's exclusion is written for a block's `## Heading`, and the preamble has no `##` heading,
   so the margin is unsettled by the letter of the rule. Recorded rather than ruled because both
   readings clear 130 comfortably — nothing in this wave turns on it.

**v7 convert-on-touch deferred at this touch — by user ruling.** The record's Open thread 5 fires
convert-on-touch (F66) at build scoping because this build touches implement; the user ruled this
wave **surgical and v6-form**, with conversion going to a dedicated wave. So implement carries **no**
`<!-- shape-form: v7 -->` marker and no P18–P20 bindings, and is graded on the v6 slot set. **The F66
trigger stays live** — the next touch faces the same decision.

*Pure additions this wave, riding the decision row rather than this note:*

- **The U1 ban clarification** — one sentence in `command-shape.md`'s Layer 1 Ground rules: the
  ban's surface is refs, index, tracked content, and history; an ephemeral self-removed
  verification worktree is not a mutation of it.
- **The `mochiko-` snapshot name prefix** — the snapshot home is
  `.claude/worktrees/mochiko-<purpose>/`, never a bare `<purpose>`. Ground: **F76** measured that
  `git worktree add` refuses a **non-empty existing directory** (`fatal: '…' already exists`), and
  the docs' name-reuse rule keys on directory existence without distinguishing a registered
  worktree from a plain one — so a snapshot parked on a name the harness later wants would block
  worktree creation there. **F77** puts real traffic at that path: background sessions isolate into
  `.claude/worktrees/` as well. The prefix makes the collision impossible by construction rather
  than by convention. The periodic sweep is *not* the hazard here — F72–F75 establish it as
  worktree-registry-scoped, so an unregistered directory is not a target.

---

## [v0.38.0] `RETURNED:` — the seat-recycling binding, re-added as a P17 lifecycle line

- **Evidence:** `.mochiko/brainstorms/team-lead-strategic-compaction/record.md` **TC-D5/TC-D6**
  (`DECISIONS.md` 2026-07-31) + `.mochiko/brainstorms/plan-run-transport-forensics/record.md`
  **R1** (user-ruled 2026-08-01) + the open BACKLOG item "Standing-seat build items — surface
  specified 2026-07-31". Wave note: `.mochiko/strips/command-shape.md` v0.38.0. Not an override
  re-add — every ground the v0.35.0 revert gave is discharged below, by name.

**The three grounds of the v0.35.0 revert, each answered:**

1. *"The wave's contract is translation under true-reductions-only accounting, and this was its
   only line of new behavior."* — **Spent.** That was the goal-shape translation wave's contract;
   this is the build wave the same entry pointed at ("the BACKLOG item remains the build's proper
   home"), and its contract is to build.
2. *"D1's cycle floor is probe-calibrated and the D4 probe is deferred, so the command would carry
   an approximate threshold."* — **Still true of the probe, and answered by where the number now
   lives.** The D4 probe remains deferred, but **TC-D6 ruled the ~≥3 default into Layer 2** as a
   probe-tunable shape value. So implement carries **no threshold at all**: the re-added line
   names its *unit* (the cycle) and its *override*, and inherits the number. When the probe tunes
   the default, one shape edit re-tunes all six commands and implement needs no touch — which is
   the outcome this ground was protecting.
3. *"Standing-seat D3 — the Layer-2 reframe that would give each command a per-seat lifecycle
   `[PARAM]` — is unbuilt … so the invariant was a workaround for a missing shape slot rather
   than a home."* — **Discharged: the slot now exists.** Shape **v6** carries the two-axis Layer 2
   and **P17**, so the re-added text is a slot binding, not a workaround.

**What came back, and it is smaller than what left.** The revert removed an 88-w `Seat recycling`
invariant from Constraints plus "recycled per Constraints" / "recycled per slice" from the two
spawn cells. What returns is a **48-w** `**Seat lifecycle:**` line beneath the Seats & checks
table — **40 w less**, and in a different block, because the doctrine the 88 words carried is now
in the shape home. **Constraints is untouched by this re-add**, measured **784/900** at this
revision — *not* the 796 the v0.35.0 entry records: that figure was correct when written and the
**v0.37.0 `@`-reference supersession took 12 words out of G1**. Re-measured rather than carried,
because a figure quoted from a prior wave is exactly the kind that goes stale unread. The words
land in Seats & checks instead: **271 → 319**, against the ceiling's new `+60` P17 term
(280 → 340).

**What the line binds, all three of implement's genuine differences from the Layer-2 default:**
the **counted unit is the cycle** (implement's Bounds count three different things — retries per
cycle, fix passes, and stall rounds — so the lifecycle denominator is ambiguous without this) ·
the **verifier's per-slice override** with its final-validation incarnation briefed from the
on-disk verification reports (standing-seat **D1**, the ruled asymmetry) · the **retry/fix-pass
respawn** carrying the failed-task list **and** the just-failed `cycle-report.md` **relayed at
dispatch** (D1's S1 fold, on F-g's finding that the next attempt overwrites that file — a later
re-read would hit the wrong report).

**Not re-added, deliberately:** the producer's `~≥3` cycle floor and the gate-pause/cache-warmth
condition — both are now the Layer-2 default and restating them in the command would be the
restatement the shape forbids (TC-D6 as amended, RI-2: "implement conforms on the producer floor
but carries one explicit override"). The two **spawn cells are left as they are** ("standing
across the cycle sequence and the fix-pass loop" · "cold at the first cycle verification,
standing after"): shape v6 states that a roster row reading *standing* describes the **seat**,
not one context, so the cells are accurate under recycling and needed no re-edit — this is the
clause that made the v0.35.0 cell edits unnecessary rather than merely reverted.

## [v0.37.0] `@`-reference recovery superseded — the platform bug it named is resolved
- **Disposition:** superseded → user ruling (2026-08-01). The bug-attributed re-enter workaround retires; the most-recent-feature resolution is relocated into the decides-clause with a confirm.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or to confirm the detected feature (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`)."
- **Kept deliberately:** the resolution clause and a confirm — G1 now decides "the resolved `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens)". Only the re-enter workaround and the bug attribution left.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry; implement carried the resolution clause `plan` referenced, and keeps it.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.36.0 — the production-only re-key (stage 4)

**Wave context:** the PO narrowing build, stage 4 of 5 — the two commands aligned with the
constitution cluster rewritten earlier in the same wave. Scope ADR:
`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md`, scoping PO-D1–D7 from
`.mochiko/brainstorms/production-only-focus/record.md`. **One site, in Bindings**; shape stays **v5**
(G = 8, S = 4, blocks unmoved), Bindings 178 → 175 w.

## [v0.36.0] The cycle checkpoint no longer keys on tier
- **Disposition:** superseded → the one universal gate at the asserted floor in
  `authoring-constitution/references/DOMAIN-DEPENDENCIES.md` ("Growth" — human ruling before
  registry entry; the checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty), with
  the same always-forces reading now in `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`'s
  field table
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md`; PO-D2 retired the tier fork)
- **Content:** "the escalated checkpoint keys on tier — `production`/`regulated` (the `CLAUDE.md`
  stamp) forces it for a `domain_deps_added` entry; non-blocking below." → "a non-empty
  `domain_deps_added` **always** forces the escalated human checkpoint — never auto-approved, no
  stamp read."
- **Kept deliberately:** P14 stays bound — the clearing unit (**the cycle**) and the
  architecture-deviation de-devolution are unchanged; only the checkpoint's key moved. Noted while
  editing: the Cycle-checkpoint constraint already required an empty `domain_deps_added` for
  devolution *unconditionally*, so the tier fork had left Bindings and Constraints in latent
  disagreement — this re-key closes it rather than creating consistency that was never there.

---

# v0.35.0 — the goal-shape wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave following the
plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 +
D10; pilot checkpoint ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`). Authored
against **shape v5** with the obligated `mochiko:loop-discipline` read **retained** — the drop is
deferred to a named live-run trigger, so a v5 command that omits it is non-conformant, not early.
implement declares the **in-loop critique** branch of P6, so it must **not** reference
`templates/sized-end-stage-review.md` (check 1's negative direction) — it does not.

**Measured: 3,230 → 1,868 words (−42.2%), 23,873 → 13,331 B (−44.2%)** — `wc`-measured after the
**seat-recycling revert**, which was the last edit (the pre-revert file measured 1,962 w / 13,959 B;
those figures are superseded here and everywhere in this note, per the pilot's standing
headline-sweep habit). Against the wave's pre-authoring floor row of **1,354 w: +514 w (+38.0%)** —
over, not under, which is CS-D8's safe side. The overage is accounted by block: implement's protected
surface is the densest in the library and three blocks sit at 92–98% of their ceilings after every
trim below. The floor row assumed a G=5/S=3 file; this one lands **G=8, S=4** because two source
decision points were restored to gate form and the architecture dispatch was split into two seat rows
(both below).

Block sizes against the grader's ceilings, **with the term derivation as check 6 requires** —
**G=8** gate lines (bullets matching `^- \*\*` containing `evidence:`: G1 entry · package gate ·
governance surface · cycle checkpoint · architecture deviation · G3 · G4 · G5), **S=4** seat rows,
**A=6** own-outputs (working code · `cycle-report.md` · the per-cycle verification report · the
final-validation report · the built-vs-approved diff report · the `ARCHITECTURE.md` fold), **R=13**
resume rows: preamble 114/130 · Goal 144/150 · **Seats & checks 271/280 (96.8%)** · Constraints
796/900 (88.4%) · Bindings 178/192 (92.7%) · **Recovery 239/242 (98.8%)**.

> **At-risk measurement, flagged rather than resolved (auditor: check this term first).** Bindings
> passes at A=6 (178/192) and at A=5 (178/180) but **fails at A=4 (178/168)**. A=4 requires
> discounting two enumerated own-outputs — the built-vs-approved diff report and the
> `ARCHITECTURE.md` fold — both of which this run's own dispatches produce. The +30 KM term is
> claimed and the KM-landing binding is present. If the grader counts A=4, the fix is a real cut,
> not a re-argued term.

**Three ceiling pressures resolved by relocation or reduction, never by loosening a ceiling** (the
pilot's rule — recalibrating a ceiling to fit a file the author wrote is the forbidden
quota-override, and implement is the file most likely to want it):

1. **Constraints opened at 945/810 (G=7).** Resolved by restoring **two source decision points to
   gate form** — the pre-rewrite Phase 0 carried *step 1* (capture and resolve `<feature>`) and
   *step 2* (the plan-package-complete entry gate) as distinct steps with distinct evidence and
   distinct decisions, and *step 3* (governance) as a third. Collapsing all three into one `G1`
   line, as the first draft did, hid two real gates and cost the ceiling 180 words it was entitled
   to. G=8 → 900. The remaining 45 words came from the reductions in the ledger's *dropped* rows.
   The block finally lands at **796/900 (88%)** once the seat-recycling revert removes its 88 words —
   so the trims that bought its headroom stand, and the block is no longer the file's tightest.
2. **Bindings opened at 279/168.** Two bullets in the colliding draft (below) were
   **Constraints-class content parked in Bindings** — the governance obligated-read brief and
   project scaffolding are obligations, not referents. Both moved to their own class: governance to
   its gate line, scaffolding to an invariant.
3. **`·` separators counted as words** in the mechanical count (they are whitespace-delimited
   tokens, as in the pilot's own measurements). Two Bindings list lines were re-punctuated to commas
   (−9 tokens). **Recorded because it is a formatting change, not a content cut** — the auditor
   should treat those two lines as unreduced.

## [v0.35.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters` → the preamble's
  probe-seat line (the env check and transport mechanics are shape Layer 2, referenced) ·
  `Session constraints` → the package gate, the bounds' kill-switch, and Bindings' deliverable /
  ID-namespace lines · `The seats` → the **Seats & checks** table plus the validation-model line ·
  `Phase 0` → **G1** + the **package gate** + the **governance surface** + Bindings' design inputs +
  the scaffolding and slice-scope invariants · `Phase 1` → the **cycle checkpoint** + the
  **architecture deviation** gate + the ordering invariants · `Phase 2` → the ordering invariants'
  lead-routed final validation, the bounds' fix-pass cap, and Bindings' diff report · `Phase 3` →
  **G5** · `Phase 4` → the KM-landing binding + the Recovery table's `accepted` row · `Contract`'s
  four clauses → **Goal** (done-condition + not-done states), the **Seats & checks** table
  (producer↔validator), **Constraints** (bounds + gates) · `State recovery` → **Recovery**.
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** eleven `## Phase`/`## Contract`/`## State recovery`/roster sections, ~2,600 words of
  ordered procedure, appendix, and footer. Not reproduced verbatim — every *rule* inside them is
  resolved individually in the ledger below, and the deleted remainder is connective narration
  (step numbering, `Phase N step M` cross-references, per-phase restatements of the lead's job, and
  the reachability sentences that opened each phase). Recoverable in full at
  `git show 7898d86:plugins/mochiko/commands/implement.md` — the authoritative pre-wave baseline,
  292 lines / 3,230 w / 23,873 B (byte-identical at `c47684d`; the intervening step-1/step-2 commits
  landed shape v5 and the plan pilot without touching this file).
- **Kept deliberately:** every gate, bound, predicate, routing decision, trigger, ordering rule and
  artifact binding — resolved row by row in the CS-D8 ledger.

## [v0.35.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate, and implement carried the longest instance in the
  library (**~190 words**) restating the cycle sequence, the round counters, the execute→verify
  pairing, the verdict ownership, the devolved-branch clearing, the deviation escalation, the diff
  trigger, the fix-pass bound, every gate, the entry and governance prerequisites, scaffolding, and
  the never-mount-verification rule — each of which is now a Constraints line, a Seats cell, or a
  Bindings entry. The v0.17.0 wave already deduped this class once (*Verdict-ownership
  triplication*) and it grew back; the anatomy leaves it nowhere to hide.
- **Kept deliberately:** the one clause with no other home — "verifying each seat actually wrote its
  expected files (a missing output → log and ask retry/abort)" — is **not** dropped as behavior: it
  survives as the Recovery block's evidence-driven resume (a missing report *is* a resume row: two
  rows key on "no `cycle-report.md` this round" and "no verification report this round") plus G4's
  escalation menu. Same disposition the pilot gave the identical clause; the anatomy's Recovery
  block is the structural-prevention claim, and it is checkable in the table.

## [v0.35.0] Seam-N1 narration replaced by two seat rows
- **Disposition:** superseded → structure. The `architecture scribe` roster entry's ~55 words of
  "**two distinct firing conditions**, kept separate (seam N1)" prose, plus the two mid-body
  "seam N1 — distinct from …" reminders, are replaced by **two rows in the Seats table**:
  `arch-diff` (spawn: at final validation, whenever an approved structural delta existed) and
  `arch-scribe` (spawn: at finalize, on a built structural change only).
- **Tier failed:** 1 (altitude) — the seam was being *asserted in prose* three times because the
  single row could not show it. Two rows make the broad/narrow trigger split mechanically visible in
  the parameter the shape already provides for it (P5's spawn column), which is what the v0.32.0
  build note asked the build to resolve.
- **Kept deliberately:** both triggers, in their exact breadth — the diff fires on
  *approved-delta-existed* (independent of what was built, so a silently-descoped approved delta
  cannot escape) and the fold on *built structural change*; the diff's report reaching **G5**; and
  "never the verifier seat", stated once on `arch-diff` and shown by the table for both rows.
- **Deliberate call, flagged:** both rows are `principal-architect` × `authoring-architecture`, so
  the pair appears twice. Check 7 is satisfied — neither row grades an artifact it authored, and the
  two rows touch different artifacts (`architecture.md` + built code for the diff; `ARCHITECTURE.md`
  for the fold) — but the repetition is deliberate and worth the grader's eye.

## [v0.35.0] Skill- and shape-owned content stripped from the command body
- **Disposition:** relocated → the homes that already carry it (no new home written).
- **Tier failed:** 1 (altitude).
- **Content:**
  - `input, never the gate` — stated **three times** (roster, Contract done-condition, footer). Home:
    `command-shape.md` Layer 2 *Clearing*, and it is check 8's keyed marker. Survives once, as the
    validation-model line's "qa's output is **lead-adjudicated input** and the verdict is yours" —
    the pilot's audit-cleared phrasing.
  - "Disjoint agents, disjoint skills, structurally separated" (Contract, Producer ↔ validator).
    Home: Layer 2 *Independence by structure*, which states both phrasings; the table *shows* it.
  - "a verifier respawn is cold by design" (State recovery preamble). Home: Layer 2 *Independence by
    structure* — "a respawn is cold by design". Recovery keeps only what respawning *re-reads*.
  - "Never modify git or push" (Phase 4). Home: Layer 1 *Ground rules*.
  - "out of rounds = escalate, never done" (Contract bounds). Home: the shape's Constraints block.
    **Contested-adjacent:** the audit-PASSed pilot keeps this sentence, so keeping it was permitted;
    it is cut here because it is verbatim shape prose and Constraints needed the words. The
    semantics survive in the Goal's not-done state "out of rounds".
  - "drift caught one cycle deep, never deferred to landing" (Phase 1 / footer) and "the same
    mechanism as plan's design-time return to sign-off". Home: this note's v0.32.0 entry — design
    rationale and a cross-command provenance pointer, both of the class the pilot relocated out of
    Constraints under audit pressure.
  - "Round reports are cleaned by default; never offer to delete a deliverable." **This line never
    existed in implement** — it entered from the colliding draft (below), imported from `plan.md`.
    Removed as an import, not stripped: implement's cycle reports are the audit trail **and** the
    input a recycled producer is briefed from, so a default clean would break the seat-recycling
    binding.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects `KEPT:`/Tier-2-evidenced lines **and** every line
traceable to a `DECISIONS.md` row. implement carries **no `KEPT:` survivor-provenance entries**; its
protection set is the *Kept deliberately* fields of the four prior entries (v0.17.0 conversion,
v0.31.0 ×2, v0.32.0) plus the DECISIONS row trace, grepped before any cut. **All 26 rows survive
translated; one is superseded with grounds; zero dropped.** Per the pilot's warning that losses hide
in *compressed evidence clauses* rather than deleted sections, the devolved-branch predicate and the
deviation triggers were re-read clause by clause against `git show c47684d` after the last edit.

| protected line | source | resolved |
|---|---|---|
| AD-D6.1 — the approved `architecture.md` is **briefed input**, read at entry and carried in the producer's per-cycle brief | DECISIONS row AD-D6; v0.32.0 | Bindings' design inputs, marked "the **anchor** for both architecture mechanisms"; the package gate's evidence; the producer seat row's deviation self-check |
| AD-D6.2 / R7 — the **diagram-anchored mechanical test**, verbatim: add/remove a box · add/remove/redirect an arrow · move a responsibility across a boundary | DECISIONS row AD-D6; v0.32.0 | The **Architecture deviation** gate's evidence clause, all four triggers intact |
| AD-D6.2 — the self-check runs at **cycle open AND cycle close** | v0.32.0 (emphasised there) | Same gate line: "run at cycle **open** and cycle **close**" — the conjunction kept, not compressed to "each cycle" |
| AD-D6.2 — a surfaced deviation **stops and surfaces**, never silently built; the user re-rules; the target is **amendable mid-implement with consent**, updated before the cycle resumes | DECISIONS row AD-D6 | Same gate line's rules/decides clauses + "never silently built" |
| AD-D6.3 / R8 — the **built-vs-approved diff** fires whenever an **approved structural delta existed** (broad, independent of what was built), at final validation, in **diff mode** | DECISIONS row AD-D6; v0.32.0 | `arch-diff` seat row (skill + diff mode + spawn trigger) · **G5**'s evidence · the Goal's end state and not-done state · a Recovery row |
| AD-D6.3 — the divergence **surfaces at the acceptance gate** | DECISIONS row AD-D6 | **G5** evidence: "**and** the built-vs-approved architecture result where an approved delta existed" |
| Seam N1 — the diff's broad trigger vs the fold's narrow **built-structural-change** trigger, kept separate | v0.32.0 (build-seam resolution) | Two seat rows + the KM-landing binding's "a **built** structural change" (entry above) |
| Team-method D3 — the **cycle** is the clearing unit and the devolved branch applies to it | DECISIONS row; v0.31.0 | Bindings' **Clearing unit + checkpoint keying** (P14) + the cycle-checkpoint gate line |
| Team-method D3 — the predicate, **exactly**: every verification a deterministic CLI check at **100% pass** AND no deviation reported AND `domain_deps_added` empty | v0.31.0 | Cycle checkpoint, with "skipped **exactly** when" and all three conjuncts — the compressed-evidence-clause check's primary target, re-read against the prior text |
| Team-method D3 — a clean cycle clears on qa's **PASS-with-evidence, unread by the lead**, counted from its clearance notice | v0.31.0 | Same line's devolved clause; the Goal's "every **escalated** cycle's reports" carries the not-re-read consequence |
| Team-method D3 — **everything else returns to the lead**: any failure · any GUI or subjective verification · any reported deviation · any registry addition | v0.31.0 | Same line's "Otherwise it fires — …", all four classes enumerated |
| AD-D6 × D3 fold — a surfaced architecture deviation **is** a reported deviation and **de-devolves** the cycle (no parallel gate) | v0.32.0 | Bindings' P14 line ("a surfaced architecture deviation **de-devolves** it") — kept out of the gate line so the fold is stated once |
| Domain-allowlist D2/F2 — the `domain_deps_added` **visibility floor**: disclosed in the cycle report and surfaced at the checkpoint | DECISIONS row (2026-07-21) | Cycle-checkpoint evidence + Bindings' uncertainty carrier (the field named in both) |
| Domain-allowlist — the **confidence-gate hook**: a registry addition at `production`/`regulated` **forces** the human checkpoint regardless of deterministic-CLI pass; lower tiers surface non-blocking | DECISIONS row | **Superseded by v0.36.0** (the entry above): always forces, no stamp read — this row records the v0.35.0-era keying |
| v4 mesh D1/D2 — verifier **cold at the first cycle verification**, standing after; the producer↔verifier **peer edge** declared on the roster | v0.31.0 | Verifier seat row (spawn + peer-edge cells) |
| v4 mesh — the **endgame is lead-routed**; the devolved branch clears cycles, never the final validation | v0.31.0 *Kept deliberately* | Ordering invariants: "The final validation is lead-routed, never devolved" + the verifier row's peer-edge cell |
| v4 mesh — a **retry is lead-dispatched** (a retry follows a failure; the verdict on a non-clean unit is the lead's) | v0.31.0 *Kept deliberately* | Producer seat row: "retries and fix passes are dispatched by you" |
| Delivery is a hand-off, not a start signal — the pairing is the lead's to enforce | v0.31.0 / v4 mesh | Ordering invariants: "the hand-off is peer-routed, the pairing is yours to enforce" |
| Standing-seat **D1/D2** — conditioned checkpoint recycling: cycle floor (~≥3) + gate-pause check, same-name successors, artifact-only respawn briefs, just-failed report relayed at dispatch, verifier per slice | DECISIONS row (2026-07-23) | The **Seat recycling** invariant + the producer/verifier spawn cells. **An addition, not a translation — see the contested call below.** |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a **named cause** (the `@`-reference drop bug) and a **two-option prompt** (re-enter, or confirm the detected feature) | Pilot fix-round restore; record §7 protected set | **G1**, cause and both options intact, plus the detection rule (explicit ID, else the most recent in-progress feature). The pilot lost this once in a compressed evidence clause; written first here and re-checked last |
| Roadmap-v2 — **sequential implement**; parallel cycle execution is a `deliberate-shortcut-ledger` deferral, **not a capability drop** | DECISIONS row (2026-06-27); BACKLOG parallelism item | Ordering invariants, both clauses |
| Foundation cycles before feature cycles; current cycle = the first with unchecked tasks | current body | Ordering invariants |
| The **execute→verify pairing** — every produced cycle verified in the same round, **never skipped** | current body / v0.17.0 | Ordering invariants |
| The verification skill is **never mounted on staff**, and staff never grades its own cycle | v0.17.0 *Verdict-ownership* strip (deduped to once) | Validation-model line, once: "The verification skill is **never** mounted on the producer" — the second half is shown by the table, not asserted |
| Bounds — targeted retry **max 3/cycle** re-opening only the failed tasks and never regressing passing code · fix pass **max 3**, failure-scoped, **unconstrained by cycle boundaries** · convergence stall at **2+ rounds** · no-progress = an unchanged failing set · `IMPLEMENT_STOP` checked **before each seat send** | v0.17.0 + current body | The **Bounds** line (caps, stall, no-progress, kill-switch) + the producer seat row (the fix pass's cycle-boundary freedom) |
| **No G2** — a single verifier, so no feasibility-rejection gate (the audit-passed reword) | v0.32.0 (preserved there) | Validation-model line, where it now *follows from* the single-verifier fact rather than sitting as a standalone note |
| Slice binding 1 — the package gate and the cycle loop read `slices/<slice>/tasks.md` | v0.17.0 slice strip, *four genuine bindings kept* | Slice-scope constraint |
| Slice binding 2 — design inputs = shared feature-root artifacts **plus** `slices/<slice>/{plan.md, task-mapping.md}` | same | Slice-scope constraint |
| Slice binding 3 — per-slice outputs land under `slices/<slice>/`, and what that does to the artifact set | same | Bindings' **Reports** line ("or `slices/<slice>/` when slice-scoped, where the Goal's artifact set reads them") — moved out of Constraints to the block that owns paths |
| Slice binding 4 — the quality gates run the **full repository suite** (implement's own operationalization of the contract's regression-safety rule) | same | Slice-scope constraint, with the regression-net reason kept |
| Slice binding 5 — at the last slice the **feature is declared, not verified**; Feature-Done executes at feature-close, owned by no workflow; surface it, never report feature completion | same (implement is the pipeline's terminal stage) | Slice-scope constraint's closing clause |
| The **Graduation contract** is the single home; do not restate it | v0.17.0 audit catch (the D1 churn liability) | Slice-scope constraint opens by naming it as the single home for the six rules and restates none — the defect that entry was written about is not reintroduced |
| Vertical-graduation — the slice-scoped entry variant | DECISIONS row (2026-07-02) | The Slice-scope constraint + Bindings' per-slice report layout |
| Governance prerequisite — surface a missing region (offer `/mochiko:setup`), **never auto-resolve**; governing context, not a blocking gate; present → the **one-line obligated read** of the `paths`-relevant `.claude/rules/mochiko/` files in each code-touching brief | current body / setup-cluster rulings | The **Governance surface** gate, all three parts |
| Workspace-as-state, **no registry field** | v0.17.0 (named a genuine survivor there) | Bindings' design-inputs line |
| The producer's **honest** `cycle-report.md` is the producer-authored uncertainty carrier, not confidence marks | current body (P11) | Bindings' uncertainty carrier, with `mochiko:executing-tdd-cycle`'s format referenced as the field owner |
| KM landing under fix-on-sight; implement records **what it builds** | v0.32.0 / KM invariants | The KM-landing binding. **Superseded with grounds:** the "implement records what it builds" half is deduped — `plan.md`'s KM binding already states the division ("Plan records only what plan itself established … implement records what it builds"), so it survives stated once, in plan. The *behavior* (the fold fires on built structural change) is unchanged here |

**One routing correction, recorded rather than folded silently.** The prior body's line "Route
knowledge / preference / scope gaps per `loop-discipline` (→ **G3** / **G4** / escalate)" read as
*preference → G4*, which contradicts `loop-discipline`'s own routing table (knowledge → research;
**preference → the human gate**; scope → halt or split, i.e. the escalation gate). Translated to the
doctrine's mapping: **G3** rules the preference gap and routes knowledge to `Explore`; **G4** takes
the scope gap. Flagged because the compressed original is ambiguous enough that a reader could call
this a behavior change rather than a correction.

## [v0.35.0] `RETURNED:` — the seat-recycling binding stays unbuilt (contested call, user-ruled out)

**Raised as a contested addition, ruled out by the user at wave ratification, and reverted in the
same version.** Recorded in full because the ruling that keeps it unbuilt is the useful artifact —
standing-seat **D1–D4** remains ruled, and this entry is where the next builder finds why the command
does not yet carry it.

**What was raised.** The wave brief listed "producer checkpoint-recycling (cycle floor + gate-pause
check, same-name successors)" among implement's protected, DECISIONS-traceable surface. It **is**
ruled (`DECISIONS.md` 2026-07-23, standing-seat lifecycle **D1–D4**) but it was **never built into
`implement.md`** — a grep of `git show 7898d86:plugins/mochiko/commands/implement.md` for `recycl`
returns zero, and the file instead declared the pre-D1 claim, "one **named standing seat** across the
whole cycle sequence and the Phase-2 fix-pass loop". `BACKLOG.md` carries the build open:
"**Standing-seat build items (deferred)** — conditioned checkpoint recycling · respawn briefs from
artifacts · the Layer-2 transport-vs-lifecycle rewrite (**v4+**) · per-seat measurement."

**Authored in the include direction** (an 88-w **Seat recycling** invariant plus "recycled per
Constraints" / "recycled per slice" in the two spawn cells), on the ground that the pre-rewrite text
was stale against a ruling, and **flagged rather than silently resolved**.

**Ruled out. Grounds, as given at ratification:** the wave's contract is **translation under
true-reductions-only** accounting, and this was its only line of new behavior; D1's cycle floor is
**probe-calibrated and the D4 probe is deferred**, so the command would carry an approximate
threshold; standing-seat **D3** — the Layer-2 *transport vs context-lifecycle* reframe that would
give each command a per-seat lifecycle `[PARAM]` — is **unbuilt** (v4+, open in BACKLOG, and
`.mochiko/strips/command-shape.md` names it as deliberately not combined into the mesh revision), so
the invariant was a workaround for a missing shape slot rather than a home; and the BACKLOG item
remains the build's proper home.

**Reverted, exactly as costed.** The `Seat recycling` invariant deleted (88 w); the producer's spawn
cell back to "standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation
cycle 1"; the verifier's back to "cold at the first cycle verification, standing after". Measured
after: Constraints **796/900** (projected 796 — exact), Seats & checks **271/280**, file **1,868 w /
13,331 B** — 6 w under the ~1,874 projection, because the two spawn cells shed "recycled per
Constraints" / "recycled per slice" on top of the 88-w invariant. **The ruling is acknowledged here
and unbuilt by design** — the
standing-seat claim in the seat rows is therefore known-stale against D1, not an oversight.

**Never added, so the absence is not read as a drop:** D1's user escape hatch (the user may order a
recycle at any gate), D2's artifact-only respawn briefs and their sufficiency watch-item, and the
relay of the just-failed `cycle-report.md` at dispatch. All lifecycle policy with no consumer in the
goal-shaped file; they stay in the record until the D3 reframe lands. **Re-add trigger:** the
standing-seat build items shipping — the D3 Layer-2 reframe first, since it supplies the `[PARAM]`
this content belongs in.

## [v0.35.0] Collision note — an unledgered orphan draft occupied the working tree

**Baseline provenance, stated because it was briefly in doubt: this rewrite and its ledger derive
from HEAD**, not from the working tree. While the wave was in flight a since-terminated seat,
executing a superseded instruction, overwrote the working-tree `implement.md` at ~23:26 with a
**different** goal-shaped draft (1,934 w / 13,919 B, never committed, no strip entry). It was read
in full before being replaced and is snapshotted lead-side; it is an unledgered orphan and carries
no authority — **reference seed at most.** Every row of the CS-D8 ledger above was re-derived
against `git show 7898d86:…`.

It was also **not** conformant, which is why replacing it rather than extending it was the cheaper
path: measured against the grader's floor it failed check 6 on **four of six blocks** (Goal 155/150 ·
Seats & checks 258/235 at S=3 · **Constraints 792/630** at G=5 · **Bindings 279/168**), and check 13
on the architecture deviation, which it carried as a plain bullet with no
`evidence:`/`rules:`/`decides:` triple despite the body relying on the user re-ruling a surfaced
deviation.

**Three elements it shares with this file are independently HEAD-traceable** — verified line by line,
so nothing here rests on the orphan: the `No G2` note (HEAD:247, folded onto the validation-model
line here), "Neither ends the loop on its own" (HEAD:187), and `domain_deps_added` in the cycle
checkpoint (HEAD, 3 occurrences). The orphan influenced phrasing and placement only. **Two of its
calls were rejected:** the imported report-cleaning binding (see the strip entry above) and the
Constraints-class content parked in Bindings.

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
