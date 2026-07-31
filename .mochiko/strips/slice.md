# Strip notes — `commands/slice.md`

Entry formats: `strips/README.md`. Wave context: the slice cluster wave (BACKLOG item 7, the
second one-shot-command wave after specify's at v0.13.0). The wave also ran the **D2 conversion
assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint** against
slice's needs (no new shape gap at that wave, when the shape was v2). **Stale as a standing claim:**
the shape is now **v5** (2026-07-30) — see the v0.35.0 section immediately below, which rebuilt this
command goal-shaped; the v0.31.0 entry's "now v4" claim is likewise frozen history.

---

## [v0.37.0] `@`-reference recovery superseded — the platform bug it named is resolved
- **Disposition:** superseded → user ruling (2026-08-01). The bug-attributed re-enter workaround retires; the detected-feature resolution (already in G1's evidence) gains a confirm.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or to confirm the detected feature."
- **Kept deliberately:** the detected-feature confirm — G1's evidence resolution now reads "the resolved `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens)". Only the re-enter workaround and the bug attribution left.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.35.0 — the goal-shape rebuild, step 4 of 4 (CS-D10)

**Wave context:** command goal-shape rebuild, **step 4** — the remaining-five wave
(`brainstorm` · `specify` · **slice** · `implement` · `setup`), design
`.mochiko/brainstorms/command-succinctness-strip/record.md` (CS-D3/D4/D5 + D8 + D10; `DECISIONS.md`
2026-07-30), against **shape v5** with the obligated `mochiko:loop-discipline` read **retained** —
its drop is deferred to a named live-run trigger (pilot-checkpoint ruling 5), so a v5 command that
omits it is non-conformant, not early. Worked example: the audit-PASSed `plan` pilot (v0.34.0).
slice is **not** KM-carrying (the five KM commands are brainstorm/specify/plan/implement/setup), so
it takes no `.mochiko/memory/knowledge-management.md` read and its Bindings carry no KM landing. It
declares the **in-loop critique** branch of P6, so `templates/sized-end-stage-review.md` must not
appear in it — grepped, absent (check 1's negative direction).

**Measured: 1,611 → 1,242 words (−22.9%), 11,968 → 8,792 B (−26.5%)** — `wc`-measured after the
final edit round, per the pilot's standing habit (re-sweep every figure after *each* round; three
stale-headline instances in the pilot build all traced to a summary written before the last edit
landed). Against the wave's pre-authored floor of **1,076 w: +166 (+15.4%)** — over, the safe side
of CS-D8, since landing materially *under* a floor row would signal dropped content. slice's
reduction is the shallowest of the wave by design: it was already a *converted, twice-stripped*
command (v0.14.0 wave + v0.31.0 mesh re-conform), so the flow/Contract narrative it carried was
thinner than the pilot's 2,873 words.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=4** gate lines,
**S=2** seat rows, **A=3** artifacts, **R=8** resume rows): preamble 110/130 · Goal 125/150 · Seats
& checks 176/190 · Constraints 470/540 · **Bindings 112/126 (88.9%)** · Recovery 160/172.

> **Authorship note, recorded because the audit trail depends on it.** This file was rewritten
> goal-shaped twice inside the same wave: a first goal-shaped draft landed on disk at 23:27
> (1,308 w) from a parallel wave author, colliding with this seat's assignment; this seat's own
> draft was rejected by the write-guard ("modified since read"). Rather than overwrite a peer's
> just-written text, the collision was reported to the lead and the on-disk draft was **repaired in
> place** — its prose survives wherever it was conformant. The repair set is enumerated below under
> *Defects carried by the first goal-shaped draft*. **Author ≠ grader still holds:** both authoring
> hands are producer-side; nothing here is a self-grade, and the independent
> `validation-command-shape` audit is unrun at the time of writing.

## [v0.35.0] Defects carried by the first goal-shaped draft, fixed at the repair round

Logged as wave evidence, not as strips — none of this content left the command; it was mis-sized,
mis-classed, or missing.

- **Two check-6 ceiling FAILs.** Seats & checks **195/190** — the reviewer row enumerated
  `advocate-report.md`'s contents ("severity-classified findings, product-framed clarifying
  questions, a recommended verdict (`ready` / `needs-revision` / `critical-gaps`)"), which is
  `mochiko:review-slices`' own description text plus `templates/advocate-report-template.md`'s
  structure; cut to the report name (see the skill-owned strip below). Bindings **128/126** — caused
  by the Governance-brief entry stripped below. Both now PASS.
- **Gates were not in order** — the draft kept the historical labels, which forced the sequence
  **G1 · G3 · G5 · G4**: both a gap (no G2) and a mis-ordering (acceptance before escalation)
  against the anatomy's "the gates, **in order**". Resolved by the renumber below.
- **Dropped from the declared end state:** the designated **foundation slice**, **dependency-closed
  ordering**, and **cross-cutting extend obligations** — the last absent from the file entirely.
  All three are `DECISIONS.md`-traceable (vertical-graduation, 2026-07-02) and the v0.14.0
  slicing-judgment strip kept "the converted goal names the deliverable's parts once" as the
  surviving half of its relocation. Restored into **Goal**. This is the same failure class the
  pilot's fix round named — loss inside a *compressed clause* whose surrounding sentence still
  reads complete — arriving this time in the Goal rather than in a gate line.
- **P10 gap:** no ID namespace was bound (the shape requires each artifact's path *and* ID
  namespace). Added to Bindings: slices `S#`; `US-#` / `SC-#` are `spec.md`'s, echoed never minted.
- **Generic FAIL prose in the not-done states** — "default **FAIL**" is shape doctrine (the anatomy
  states the initial state), not a state of this workflow, and check 13 grades exactly that;
  replaced with the structural state "no `slices.md` and no recorded null exit". The
  G-amend/override clause sat in the Goal and was **missing from Bounds**; moved to Bounds, its own
  block.
- **In-file duplication:** Bindings re-listed the Goal's conformance conditions (story homing,
  Feature-Done, spec stamp). Goal states the checkable condition; Bindings states the referent.
  Deduped to Bindings' path + template + ID namespaces.

## [v0.35.0] KEPT re-grade — "No G2" **superseded with grounds** (the wave's one contested call)

- **Disposition:** superseded → the goal-shaped anatomy's ordered gate list, via a contiguous
  **gate renumber**. The survivor-provenance entry below (`[v0.14.0] KEPT: "No G2 …"`) is retired;
  the note no longer appears in the command.
- **Tier failed:** n/a — supersession by ruling (**CS-D8** re-grade against the new anatomy,
  "never auto-carried"), executed under the step-4 briefing's instruction to re-grade this entry
  honestly.
- **Content (the retired note, verbatim as it last shipped):** "**No G2** — slice is
  single-reviewer, so plan's feasibility-rejection slot is intentionally unused."
- **Grounds, stated plainly.** The entry's Tier-2 evidence was a *reader-inference* failure:
  "without it, an auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate." That
  failure has two preconditions — a **gap in the file's gate labels**, and a reader inferring a
  dropped gate from it. In the goal-shaped file the gates are an explicit ordered list in
  Constraints, and the shape home rules the label to be the command's own ("`G3`, or a plain name
  where the workflow numbers nothing; what makes it a gate is the three parts, not a number").
  Renumbering to **G1 entry · G2 clarification · G3 escalation · G4 acceptance** leaves the first
  precondition no path: there is no gap to misread, and the gate set is countable by the grader's
  own `G` term (4). Checked before ruling it: **nothing outside the command references slice's gate
  numbers** (`implement.md`'s and `strips/implement.md`'s "G5" are implement's own) — grepped, so
  the renumber breaks no cross-file pointer.
- **Kept deliberately:** the entry's *second* element — the record of the deliberate
  **single-reviewer** structure — is not carried by prose but by structure: P5's table has exactly
  one grading row, and the validation-model line names "a **single reviewer**". The comparison to
  plan's feasibility-rejection slot is provenance, and its home is this note.
- **Contested, and why it is flagged rather than settled here:** the honest counter-argument is
  that the renumber *is* the supersession's enabling condition — had the historical numbering been
  kept (as the first goal-shaped draft kept it), the gap would still exist and the note would still
  bind. So the call is a renumber decision first and a survivor re-grade second. It is put to the
  audit in exactly those terms. The tie-breaker taken: keeping the old labels also produced the
  **out-of-order** gate sequence above, so the numbering could not be preserved *and* conformant.
- **Line-cite drift:** `BACKLOG.md` (step-4 item) and the session record §9.4 cite this survivor as
  `.mochiko/strips/slice.md:111–115`. Those line numbers now point elsewhere — the retired entry is
  reachable by its heading, `[v0.14.0] KEPT: "No G2 …"`, at the bottom of this file.

## [v0.35.0] The flow body and the Contract section retired into the five-block anatomy

- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters` → the preamble's
  probe-seat line (the rest relocated, below) · `Session constraints` → **G1** + Bindings ·
  `The seats` → the **Seats & checks** table (both seats, their spawn timing and peer edges) ·
  `The flow`'s Triage → **G1** · its Decomposition loop → the seat rows + **G2**/**G3** + Bounds +
  the loop invariants · its Acceptance → **G4** (both shapes) · its Finalize → the Recovery table's
  accepted row + Bindings' Next step · `Recovery` → **Recovery** (unchanged in name and role, rows
  re-derived). The `Contract` section's four clauses → **Goal**
  (done-condition + not-done states), the **Seats & checks** table (producer↔validator), and
  **Constraints** (bounds + gates).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** the `## Team-form parameters` / `## Session constraints` / `## The seats` /
  `## The flow` / `## Contract` / `## Recovery` sections plus the closing footer — 1,611 words of
  ordered procedure
  and appendix. Not reproduced verbatim: every *rule* inside them is resolved individually in the
  ledger below, and the deleted remainder is connective narration (step sequencing, "loop to
  produce", "initialize `round = 1`", and the lead's job restated per phase). Recoverable in full at
  `git show c47684d:plugins/mochiko/commands/slice.md`.
- **Gate renumber map (consequent, and the grounds are the entry above):** G1 entry → **G1** · G3
  clarification → **G2** · G4 escalation → **G3** · G5 acceptance → **G4**. The former G2 was never
  occupied. Every in-file reference was repointed (Goal ×2, Recovery ×2); no external pointer
  existed.
- **Kept deliberately:** every gate, bound, routing class, trigger, guard and artifact binding — see
  the ledger.

## [v0.35.0] The `What you own (not the seats)` footer deleted

- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate, the same class the pilot deleted. 106 words restating
  the entry/guard/governance prerequisites, the loop and gap routing, the verdict ownership, the
  human gates, and the producer↔reviewer collapse prohibition — each of which is now a Constraints
  line, a Seats-table cell, or shape doctrine.
- **Kept deliberately:** the one clause with no other home — "verifying each dispatch wrote its
  expected files (a missing output → log and ask retry/abort — a null-exit round expects
  `slicer-report.md` only)" — is **not** dropped. Per the pilot's precedent it survives as the
  Recovery block's evidence-driven resume, where a null-exit `slicer-report.md` with no
  `advocate-report.md` is an explicit row that advances to review, plus G3's escalation menu.

## [v0.35.0] Team-form parameters section relocated to the shape home

- **Disposition:** relocated → `templates/command-shape.md` **Layer 2** (Hard requirement — agent
  teams · Seat transport · the addressability probe · the no-fallback `Contested` bet). The
  surviving parameter is P2, the probe seat, now one clause in the preamble.
- **Tier failed:** 1 (altitude).
- **Content:** "Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The
  **authoritative first-spawn probe** is the producer — always the first seat filled. Transport
  mechanics + the addressability check: `templates/agent-dispatch.md` (Seat transport). The
  no-fallback bet is the same `Contested` dogfood-pilot ruling as the other team-form commands."
- **Note — the relocated pointer was also stale:** Seat transport left `agent-dispatch.md` for
  `command-shape.md` Layer 2 at **v5** (CS-D6), so the command's citation had been pointing at a
  section its target no longer carried. The rebuild retires the pointer rather than repairing it.

## [v0.35.0] Governance dispatch-brief line relocated to the briefing home

- **Disposition:** relocated → `templates/agent-dispatch.md` field 3 ("Input(s) to read … incl. the
  governance obligated-read line naming relevant `.claude/rules/mochiko/` files"), which every
  dispatch already honors. G1 keeps what is slice's own: the governance region as entry evidence,
  and surfaced-never-auto-resolved.
- **Tier failed:** 1 (altitude — true of every command that dispatches a producer under
  governance; the audit-PASSed `plan` carries no such line).
- **Content:** "**Governance prerequisite:** … Present → governance reaches the producer natively at
  spawn; add to its brief the one-line **obligated read** naming the `.claude/rules/mochiko/` files
  relevant to what it authors." — carried into the first goal-shaped draft as a Bindings entry, where
  it was also the cause of that block's ceiling FAIL.
- **Consumers assessed:** n/a — command-local; the home is a shared template, unedited by this wave.

## [v0.35.0] Skill-owned and shape-owned content stripped from the command body

- **Disposition:** relocated → the primitives that already own it (no new home written; each
  verified by reading the owner this run).
- **Tier failed:** 1 (altitude).
- **Content:**
  - The **`advocate-report.md` contents** enumeration — "severity-classified findings,
    product-framed clarifying questions, and a recommended verdict (`ready` / `needs-revision` /
    `critical-gaps`)". Homes: `mochiko:review-slices` (its description states the severity
    classification and the 3-state recommended verdict) + `templates/advocate-report-template.md`.
    **Kept deliberately:** the reviewer row still names the report as its output and still binds the
    two files it grades from, which is slice's own parameter.
  - The **check-8 keyed markers**: "the reviewer's status is **input, never the gate**",
    "disjoint agents, disjoint skills, **structurally separated**", and "a reviewer respawn is
    **cold by design**". Home: `command-shape.md` Layer 2 (Clearing · Independence by structure).
    All three are markers the grader's floor now greps for, so they could not survive as prose.
    **Kept deliberately:** their behavior — "every verdict is yours" on the validation-model line,
    independence shown by the table rather than asserted, and recovery's respawn treated as
    routine.
  - The entry-gate **memory-model parenthetical** — "(workspace evidence — there is no context-file
    `status`)". Home: `command-shape.md` Layer 1 Recovery ("never a context `phase` field"). This is
    the *second* instance of the relocation already logged at v0.14.0 for the Recovery preamble;
    the entry gate carried its own copy.
  - "**Single reviewer, never the producer**" → the trailing clause dropped as the no-self-grading
    rule (home: Layer 2 + `agent-dispatch.md`'s one hard line; P5's table is where it is checkable).
    **Kept deliberately:** "a **single reviewer**", which is slice's own cardinality and the
    surviving half of the retired "No G2" evidence.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. slice carries **one** `KEPT:` entry (the "No G2" note) plus
the *Kept deliberately* fields of the v0.31.0 supersession and the v0.14.0 conversion note, plus the
row trace. Grepped before any cut. **19 rows: 17 survive translated · 1 superseded with grounds · 1
not-applicable (never in slice's text) · 0 dropped.**

Per the pilot's wave discipline, the routing classes and named-cause recoveries were grepped
individually rather than trusted to a gate line that reads whole — which is how the three missing
end-state elements above were caught.

| protected line | source | resolved |
|---|---|---|
| `slices.md` overlay of ordered, named story groups; downstream stages run per slice | DECISIONS 2026-07-02 (vertical-graduation) | Preamble goal line · Goal · Bindings · frontmatter `description` |
| A designated **foundation slice** | same | Goal (end state) · G4 evidence · Recovery's finalize row. **Restored at the repair round** |
| **Dependency-closed** Slice-order | same | Goal (end state). **Restored at the repair round** |
| Cross-cutting **extend obligations** placed | same | Goal (end state). **Restored at the repair round** — absent from the first draft entirely |
| **Feature-Done declared at decomposition, verified at close** | same | Goal (section complete) + the loop invariant "declared, not verified — executes at feature-close, once every slice ships" |
| The **null exit** — recommend whole-spec, no file, reasoning disclosed | same | Goal (the `or` branch) · producer row · reviewer row (grades the depth call) · G4's second shape · two Recovery rows |
| An **un-homeable cross-cutting story** is a spec-amendment escalation, never a forced placement | same | **G2**, third routing class, with the `/mochiko:specify` amendment offer |
| Decomposition adds **one** grouping-acceptance gate | same | **G4**, two-shape |
| Staleness guard · graded amendment · extend-mode · artifact layout | same, but homed in `templates/slices-template.md`'s **Graduation contract** | Never slice's text (slice *produces* the contract; the downstream commands consume it) — no drop |
| Cold reviewer **arrival**; producer↔reviewer peer edge; every verdict the lead's | DECISIONS 2026-07-30 (Layer-2 mesh) + v0.31.0 | Reviewer row (cold at first review, standing after) · both peer-edge cells · the validation-model line |
| **No devolved branch** — slice has no deterministic-CLI verification, so the branch cannot apply; declared, not left implicit | v0.31.0 *Kept deliberately* | Loop invariants, in those terms |
| The **null-exit round is graded and ruled exactly as before** | v0.31.0 *Kept deliberately* | Reviewer row's depth-call clause + G4's second shape; the round reaches review whether or not a file was written |
| Standing producer holds extend-obligation placements, depth reasoning across a wrong-depth flip, foundation/Feature-Done coherence | v0.14.0 conversion note (the retention bet) | Seat row carries the operative fact ("one **named standing seat** across rounds"); the *rationale* stays in the conversion note below, its single home |
| A **wrong-depth** finding flips the outcome shape and counts as a round | v0.14.0 conversion note + current body | Loop invariants, in those terms |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) **and** a two-option prompt (re-enter, or confirm the detected feature) | record §7 protected set (the `command-altitude` retrofit-regression warning); the pilot lost this and restored it under audit | **G1**'s decides-clause, cause and both options intact — grepped explicitly because the pilot's audit found it dropped there |
| The gap-routing classes — **preference** ruled at the clarification gate · **"Research this"** knowledge → a native `Explore` pass, never the user | record D5 fold (a) graded exemplar; `loop-discipline` gap routing | **G2**, both named (with the un-homeable-story class above making three) |
| The **already-decomposed guard** — no graduated slice → offer re-decomposition or stop; any graduated slice → halt and escalate, a recorded `BACKLOG.md` deferral, not an oversight | current body; deferral recorded in `BACKLOG.md` ("Slice spec-amend mode", 2026-07-02) | **G1** · Recovery's halt row |
| Round reports cleaned by default, retainable on request; **never offer to delete `slices.md`** | current body | Bindings' artifact entry |
| **`KEPT:` "No G2"** | v0.14.0 survivor-provenance entry | **Superseded with grounds** — see the re-grade entry above. The single-reviewer half survives structurally in P5/P6 |


## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: the reviewer is still cold-spawned at first review, and the producer↔reviewer peer edge is declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 is a message to the same seat carrying the reviewer's gap list verbatim"
  - reviewer seat: "spawned **cold at first review**, never in contact with the producer"
  - Contract, Producer ↔ validator: "(reviewer cold-spawned, gap lists lead-routed, no producer↔reviewer contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — slice has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit. The **null-exit** round is unaffected: it is graded and ruled exactly as before.

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
- **RETIRED at v0.35.0** — superseded with grounds by the CS-D8 re-grade at the top of this file
  (the gate renumber leaves the gap-misreading no path). Kept here as provenance; the evidence below
  is the standing claim as it read at v0.14.0, not a live one.
- **Tier-2 evidence:** prevents a false-defect reading of the gate sequence — without it, an
  auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate. The note records the
  deliberate single-reviewer structure (no feasibility-rejection slot, unlike plan). Provenance:
  slice is net-new single-reviewer by design (REGISTRY row 44; `vertical-graduation/synthesis.md`).
