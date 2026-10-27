# Second list: the lever's home

- Seat: `second-list-home` · date 2026-10-27 · model opus
- Inputs received: the session frame and the one question only.
- Disclosure: no file under the session directory was read; this file is the only write there. No Explore dispatch was made. Every read was a known-path read (the dispatch ladder's direct-call rung): the seven named skills' renders, `implement`'s full render, `model.rs` for condition grammar, and `mochiko-cli template report-envelope`. No dispatch returned empty.

## Facts that shape every road

- **A run-open key already fits the grammar.** `model.rs:283-292` accepts `user-ruled` and `moment-resolved(<moment>)` resolutions. The second is command-grammar only, so a skill declares a caller-fed key as `entry-derived`, as `testing-gap-finding` already does with `run_scope` and `depth`.
- **Thinning has precedents, each with a stated skip.** `impl.mutation-lens` is keyed `when: depth=high` and says "the run owes mutation results or a stated skip". `impl.gap-finding-scope` has a scope-keyed skip that is "never a silent no-op". `impl.fail.skip-unstated` and `testing-gap-finding.delta-lane-skip-stated` are floors. `impl.delta-reverification` already thins re-grades.
- **Floors the band must leave alone or re-word:**
  - `patterns-sound-loop.two-part-trigger` says "magnitude never gates it".
  - `patterns-sound-loop/SKILL.md:9` says "The entry door never lowers the review."
  - `patterns-sound-loop.leg-1-seat-produces` makes the plan grade mandatory.
  - `review-seat-plan.fresh-peer-grader` requires a fresh peer of the author's persona.
  - `testing-end-user.strict-order` says "No skipping steps", and `testing-end-user.rationalization-stop` says "not for simple tests that obviously pass".
  - `testing-gap-finding.probe-kit-breadth-invariant` says "breadth is invariant".
  - `impl.gates-never-triaged` · `impl.fail.no-evidence` · `impl.author-grader-default-fail`.
- **Unconditional `must` rules that are the real thinning candidates:**
  - `impl.gates-full-suite` says "Quality gates run the full repository suite."
  - `impl.cold-verification` · `impl.regression-sweep` · the per-cycle code-minimalism lens in `impl.craft-verify-bindings`.
  - `impl.plan-approval-producers` restates leg 1 as must.
- **The latitude today is "how" only.** `impl.staffing-latitude` reads: "Beyond this schema's `class: floor` rules, everything is your per-run judgment: how you staff, sequence, and run the cycles".
- **Trail hooks that already exist.** The `patterns-sound-loop.disclosure-line` grammar is pinned. The report envelope allows extra frontmatter fields and already has a `disclosure` report type.
- **A risk the floor already names.** `patterns-sound-loop.only-real-seats` names "the known under-classification risk". A lead-read stakes key is the same risk one level up.

## Roads

**R1. Amend `patterns-sound-loop` to hold all four parts.**
- What goes where:
  - Add an `entry-derived` `stakes` condition, a band section listing which review forms may vary, and an invariants section: non-author, default FAIL, reads surfaces never reports, grader tier at or above the producer's.
  - Add a `depth:` segment to the disclosure line as the trail.
- How implement keys on it: implement adds `stakes · moment-resolved(run-open)` plus one floor `pointer:` rule. Its thinnable must-rules take `when: stakes=…`.
- What it contradicts:
  - The skill's own headline at `SKILL.md:9`, and "magnitude never gates it" in `two-part-trigger`. Both need a carve saying stakes gates the form, never whether a leg fires.
  - `leg-1-seat-produces`, if the plan grade can thin.
- Cost: one amended skill, about 5 to 7 rules, a strip and an audit. No new primitive.
- Failure mode: the floor's own text ends up arguing with its softener. The skill's remit is who produces and who reviews, so verification steps like the full suite, cold build, regression and probes have no home there, and the band would restate other skills' steps.

**R2. A new sibling skill, a "rigor band", beside sound-loop, transport and tiering.**
- What goes where: the stakes key grammar with its levels and run-open signals, the step-class band table, the review-form invariants, and the trail grammar all live in the new skill.
- How implement keys on it:
  - implement adds the run-open condition and one floor `pointer:`.
  - It re-keys `impl.gates-full-suite`, `impl.cold-verification`, `impl.regression-sweep` and the minimalism lens with `when: stakes=…`.
  - It widens `impl.staffing-latitude` to "which and how deep, inside the band".
- What it contradicts: nothing in sound-loop if legs stay out of the band. The band must exclude probe breadth and `**TEST:**` execution.
- Cost: one new skill with a description, a router entry, a migration and an audit. About 10 to 12 rules, plus a re-key in each command.
- Failure mode: the band table names steps that other skills own. That restates them, which breaks GI-017, and it drifts when `testing-*` changes. It also adds a fifth sibling to a library the frame says regrew.

**R3. Split each part to the skill that owns the nearest procedure.**
- What goes where:
  - The stakes key goes in `patterns-model-tiering`.
  - Probe and evidence thinning goes in `testing-end-user` and `testing-gap-finding`, which already carries depth-keyed lenses.
  - The plan-leg form goes in `review-seat-plan`.
  - Review invariants and the trail go in sound-loop leg 2 and its disclosure line.
- How implement keys on it: each skill declares `stakes` as `entry-derived`, as gap-finding does with `depth`. implement passes the ruled value in every brief.
- What it contradicts: `testing-end-user.strict-order` and `rationalization-stop`, `review-seat-plan.fresh-peer-grader`, and `leg-1-seat-produces`. That is the most floors touched of any road.
- Cost: four or five amended skills, each with its own audit. No new primitive.
- Failure mode: no single definition of "low stakes", so five skills drift apart. The lead has to assemble the band at run-open, and the trail is scattered across reports.

**R4. Command rules only (cheap).**
- What goes where: implement adds the `stakes` condition, widens `impl.staffing-latitude` to depth, and re-keys four must-rules. It adds `impl.fail.thinning-unstated`, mirroring `impl.fail.skip-unstated`.
- How it keys: entirely in the command.
- What it contradicts: the frame's destination, "not more command rules", and bet C.
- Cost: about 5 or 6 command rules. No new primitive and no skill audit.
- Failure mode: no other command inherits it, and the rule count rises again, which is bet B's regrowth.

**R5. Reuse what exists, add almost nothing (cheapest).**
- What goes where: the existing `depth` (low/high) and `scope` conditions become the key. Re-key `impl.gates-full-suite`, `impl.cold-verification` and `impl.regression-sweep` to `when: depth=high`, as `impl.mutation-lens` already is. Generalize the existing "stated skip" duty into one sound-loop disclosure segment.
- How it keys: plain `when:` on conditions that already exist.
- What it contradicts: nothing on the floor.
- Cost: 3 or 4 rule edits and one segment.
- Failure mode: `depth` is a project-wide setting declared at setup, not stakes the lead reads and the user rules per run, so bet A goes unserved. It also leans on the code-standards dial, which is out of scope.

**R6. Hub and spokes (a mix).**
- What goes where:
  - A new small sibling holds only three things: the stakes key grammar with its run-open read and user ruling, the band principle (never legs, never `**TEST:**`, never probe breadth, never gates), and the trail line.
  - The review-form invariants go into sound-loop leg 2, where the non-author review already lives.
  - Each step's own thinning stays in its owning skill, keyed on `stakes` declared `entry-derived`.
- How implement keys on it: the run-open condition, one floor `pointer:` to the hub, and `when: stakes=…` on its must-rules.
- What it contradicts: only unconditional must-rules get re-keyed. Sound-loop floors gain a clause but lose no leg.
- Cost: one new skill of about 5 rules, two or three spoke amendments, and the command re-key.
- Failure mode: readers make two hops, and the hub and spokes can disagree on stakes values. That needs one shared value set, which the hub owns.

**R7. Fold the key into `patterns-model-tiering`.**
- What goes where:
  - Its thesis is already "rides the lowest tier where its result can be trusted". Extend it to verification depth.
  - Stakes becomes a third key beside the class key and the seat default key.
  - The band becomes a depth ladder modeled on `dispatch-ladder`.
  - The trail rides `seat-roster-disclosure`.
  - The invariants already partly live in `seat-deviation-bounds`: "A grader or reviewer never runs below the tier its producer ran at."
- How implement keys on it: as in R2, with the pointer going to tiering.
- What it contradicts: tiering's scope line, "dispatch tier and the seat default", which has to widen. It also rubs against `class-key-session-tier`, which says judgment legs are "never tiered down". Skipping a leg is not tiering, but it reads close.
- Cost: no new primitive and about 5 rules. The description is at 1216 of 1536 characters, so it is tight.
- Failure mode: "which model" and "whether the step runs" get conflated, and the skill becomes a grab bag.

**R8. The stakes read rides an independent seat (a variant that pairs with R2, R6 or R7).**
- What goes where: the stakes proposal comes from the entry grading seat, `review-sufficiency` in implement, which already reads spec, store and baselines per row, including sensitivity and brownfield exposure. The user rules it at run-open. The key grammar still lives in the hub.
- How implement keys on it: one more field in the sufficiency report, plus the run-open ruling.
- What it contradicts: `impl.seat-sufficiency-independence` is fine. Scope creep into "Never reads code" needs care.
- Cost: one amendment to `review-sufficiency`.
- Failure mode: it only helps commands that have an entry grader. Other commands fall back to a lead read, which carries the under-classification risk.

## Top three
1. **R6, hub and spokes.** It gives the key, band and trail one home without restating spokes, review invariants sit where non-author review already lives, and floors keep every leg.
2. **R7, fold into model-tiering.** It is the same "lowest rung you can trust" economy with no new primitive. It is the best cheap-but-shared road if description headroom holds.
3. **R5, reuse depth and scope.** It is the floor-safe baseline at near-zero cost, and it shows exactly what a per-run stakes key adds over a project-wide dial.
