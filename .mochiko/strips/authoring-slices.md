# Strip notes — `skills/authoring-slices`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25; band
10–40 per the v0.14.0 zero-strip assessment below); [v0.14.0] entries — the slice cluster wave.
Slice-cluster-only skill (the decomposition producer craft, mounted on `task-architect`) — strips
ruled in-wave at v0.14.0. Zero strips proposed then: the skill sits at altitude (matching the
specify-wave finding on authoring skills). One contested keep (below). The library-wide
"letter/spirit" aphorism (L8) was **raised, not ruled** there — ruled library-wide at R4b
(2026-07-25, keep-and-survivor-log; this skill's copy is anchored by the Overview's discipline
framing).

## [v0.58.0] Skill retired whole — graduation slices dissolve into the feature map (supersession by ruling)
- **Disposition:** superseded → `skills/authoring-feature-map/SKILL.md` — the directory `plugins/mochiko/skills/authoring-slices/` is deleted; the feature is the pipeline unit and the slicing invariants re-key onto features there.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md` — **D4** graduation slices retire, the feature selection is the delivery unit; **D18** the Graduation contract re-keys to features verbatim plus the cross-spec extend reach; **D22** foundation softens to an ordering role)
- **Content (the full SKILL.md at deletion, verbatim):**

  ````markdown
  ---
  name: authoring-slices
  description: This skill MUST be invoked when decomposing a feature specification's user stories into graduation slices — ordered, named groups of stories that graduate through the design→build→verify pipeline as independent units — authoring the spec's **Delivery Slices section**: story→slice assignment (every story exactly one home), dependency-closed ordering, foundation-slice designation, cross-cutting extend obligations, the Feature-Done map (SC-XXX coverage + cross-slice seams), and the Graduation contract. SHOULD also invoke when the work involves "slice the spec", "decompose the spec", "graduation slices", "delivery slices", "story slices", "foundation slice", or grouping user stories into independently graduating increments. Includes the single-slice exit: a spec that would not yield at least two slices with distinct value seams gets the one-line "Single slice — whole spec." section, never a forced decomposition. Boundary: this slices STORIES into pipeline-unit groups at spec level (which stories advance together) — NOT stories into TDD cycles within one slice's implementation (cycle structuring is mochiko:patterns-vertical-tdd, downstream); it authors the section as an index of the stories, never rewrites them, and never grades its own output.
  ---

  # Authoring Graduation Slices

  **Violating the letter of the rules is violating the spirit of the rules.**

  ## Overview

  A whole-spec pipeline moves every user story through design, tasking, and implementation together — the highest-value stories cannot reach verified working code until the entire spec has crossed each stage, and every stage's artifacts spread attention across all stories at once. A **graduation-slice decomposition** fixes both: the spec's **Delivery Slices section** groups its user stories into ordered, named slices, each of which graduates through the downstream stages as its own unit.

  This skill is the slicing judgment plus the section authoring. The output is the Delivery Slices section of `spec.md`, in the shape [`spec-template.md`](../../templates/spec-template.md) defines — the slice table, Feature-Done, and the Graduation contract. The section **indexes** the spec's stories by ID; it never restates, splits, or amends them — stories, requirements, and success criteria stay single-sourced in their own sections. The slicing shape follows the spec's confirmed **Intent** section (the delivery ruling): decompose when the intent asks for increments, take the single-slice exit when it asks for the whole. Density per the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md)): rationale ≤ 3 lines per slice, seams and obligations one line each — the section is an index, not an essay.

  ## Vocabulary — two "slices", two levels

  | Term | Level | What it groups | Owner |
  |------|-------|----------------|-------|
  | **Graduation slice** | Spec / pipeline | User stories that advance through design→build→verify together | **this skill** |
  | **Vertical slice (cycle)** | Implementation, within one slice's scope | A test-first increment delivering one observable behavior | `mochiko:patterns-vertical-tdd` (downstream) |

  Same ethos — smaller, independently verifiable units — at two different altitudes. Never conflate them: a graduation slice will later be *cut into* vertical-slice cycles by the downstream structuring.

  ## When to Use

  - Authoring the Delivery Slices section during specification authoring
  - Designating the foundation slice and ordering the rest
  - Placing cross-cutting stories and recording their extend obligations
  - Authoring the Feature-Done map (SC coverage + seams) at decomposition time
  - Revising the section after a reviewer's gap list

  ## When NOT to Use

  - **Structuring cycles** — cutting one slice's scope into cycle cards is `mochiko:patterns-vertical-tdd`
  - **Grading a decomposition** — the Delivery Slices section is graded with the spec by `mochiko:review-specifications`, run by an independent reviewer, never the author
  - **Editing stories, requirements, or success criteria** — the section indexes them by ID; a story that cannot be placed is a finding to surface, not a story to rewrite
  - **Small or whole-delivery specs** — when the single-slice exit fires (below), write the one-line form and stop

  ## The invariants (hard rules)

  1. **Exactly one home.** Every user story is assigned to exactly one slice. An unassigned story means the decomposition is incomplete; a story in two slices means the boundary is wrong.
  2. **Dependency closure.** A slice must be designable and buildable given **only** the slices ordered before it. No forward dependencies — if S2 needs something S4 delivers, the ordering is invalid.
  3. **Foundation legitimacy.** The first slice — the **foundation slice** — must both (a) establish the shared design core (the entities, contracts, and constraints most other slices depend on) and (b) deliver a testable user journey of its own. A pure-plumbing slice is forbidden: infrastructure without observable value belongs in foundation *cycles* inside a slice, downstream — not in a valueless first slice that delays every user-visible outcome.
  4. **Ordering: dependency beats priority.** Order by dependency closure first; among independent slices, higher-priority stories (P1 before P2 before P3) graduate earlier.
  5. **Complete coverage.** The union of all slices is exactly the spec's story set — no orphans, no inventions.
  6. **The section indexes, never rewrites.** It references story IDs (`US-#`) and success-criterion IDs (`SC-#`); the stories' own sections remain the single source of their content.

  ## Sizing

  Soft target **2–4 stories per slice**. A single-story slice is fine when the story is genuinely independent. A slice above 4 stories must carry an explicit justification in its rationale — an oversized slice recreates the diluted, everything-at-once stage work the decomposition exists to fix.

  ## Cross-cutting stories

  A story that touches several slices' territory (the classic: "audit logging on every action") still gets exactly one home:

  - **Earliest meaningful home.** Assign it to the earliest slice where its independent test can pass meaningfully — where the story is demonstrably delivered for that slice's scope.
  - **Explicit extend obligations.** Every later slice the story touches gets a recorded **extend obligation** in the section: when that slice graduates, its design must extend the story's behavior to its scope. Obligations are named and visible — silent spread is scope leak.
  - **The un-homeable escalation.** If the story's independent test cannot pass meaningfully in *any* single slice, the story is mis-specified — usually a non-functional requirement wearing a story costume. Surface it as a spec-amendment finding to the lead; do not force an assignment.

  ## The single-slice exit

  Decompose only when the spec yields **at least two slices with distinct value seams** — separable user journeys whose independent graduation is worth per-slice pipeline overhead — and the Intent section's delivery ruling asks for increments. Otherwise the section body is the single line **"Single slice — whole spec."** and nothing else. A forced decomposition of a small spec is overhead without the focus benefit; the one-liner is the honest record that the depth call was made, and it is what the reviewer's depth second-guess grades.

  ## The Feature-Done map — declared now, verified at feature-close

  Independent graduation destroys the free whole-feature ending a single pipeline had, so the feature-level done-condition must be **declared at decomposition time** — before any slice graduates — not invented after the last slice ships:

  - **SC coverage map.** Every success criterion (`SC-#`) in the spec maps to the slice(s) whose verification covers it, with a one-line *how*. An SC that no slice can verify is a decomposition gap — fix the slicing now, while it is cheap.
  - **Cross-slice seams.** Name each integration point where one slice's stories interact with another's, and what must hold across it. Per-slice verification covers each side; the seam itself is exactly what it misses.

  The map is **executed** later, at feature-close, when every slice has shipped — it is authored here so the done-condition exists before the loop it governs.

  ## Process

  1. **Read the spec's confirmed Intent section** — the delivery ruling decides decompose vs the single-slice exit; the scope and constraints rulings bound the slicing.
  2. **Read the stories** — IDs, priorities, independent tests — plus functional requirements, success criteria, and edge cases. The section indexes these by ID; you must hold the real set.
  3. **Sketch the dependency structure** — which stories presuppose which (data another story creates, capability another story establishes).
  4. **Check the single-slice exit** — fewer than two distinct value seams, or a whole-delivery intent ruling → write the one-liner, stop.
  5. **Designate the foundation slice** — the minimal story set satisfying invariant 3.
  6. **Group the remainder** — by user-journey cohesion within the dependency structure; apply sizing.
  7. **Place cross-cutting stories** — earliest meaningful home + extend obligations; flag the un-homeable.
  8. **Order the slices** — dependency closure first, then priority; verify invariant 2 holds for every slice.
  9. **Author the Feature-Done map** — the SC coverage map (close any uncovered SC by re-slicing, not by dropping it) and the seams — and the section per `spec-template.md`'s Delivery Slices shape, Graduation contract included.

  ## Quality checklist

  Before handing off:

  - [ ] Every spec story assigned to exactly one slice; no orphans, no inventions
  - [ ] Every slice designable/buildable from earlier slices only (no forward dependencies)
  - [ ] Foundation slice establishes the shared design core AND delivers a testable journey
  - [ ] Ordering respects dependencies; priority breaks ties
  - [ ] Slice sizes within the soft target, or explicitly justified
  - [ ] Cross-cutting stories homed earliest-meaningful, obligations recorded on every touched slice
  - [ ] Every SC-# mapped to a verifying slice; cross-slice seams named
  - [ ] The slicing shape follows the Intent section's delivery ruling
  - [ ] Rationale recorded per slice; no status/stage fields (state is derived from the workspace)
  - [ ] Story/requirement/criterion sections untouched — the section only indexes them

  ## Red Flags — STOP and re-slice

  - "We'll figure out the shared design when we get there" — foundation designation exists precisely so you don't
  - "This story kind of belongs in both slices" — exactly-one-home plus an extend obligation, or the boundary is wrong
  - "The spec is small, but slicing can't hurt" — it costs a full per-slice pipeline pass; take the single-slice exit
  - "I'll add a stage column so progress is visible" — state is workspace-derived; a stored field goes stale and lies
  - "That SC is really an end-state thing, no slice covers it" — then the decomposition is wrong; re-slice until every SC has a verifying home
  - "I'll just tweak the story wording so it fits" — the section never rewrites the spec's stories; surface the finding instead

  ## Common Rationalizations

  | Excuse | Reality |
  |--------|---------|
  | "Dependency order and priority order are basically the same" | Until they are not — and a forward dependency blocks a slice mid-pipeline. Check closure explicitly. |
  | "The foundation slice doesn't need user value, it's foundation" | A valueless first slice delays every observable outcome — the exact cost this decomposition exists to remove. Plumbing goes in foundation cycles, downstream. |
  | "Extend obligations are obvious from the story text" | Obvious scope is silent scope. Unrecorded obligations become unplanned work in a later slice's design. |
  | "We can map the SCs after the slices ship" | A done-condition invented at the end is post-hoc. Declared-before-the-loop is the point of authoring it here. |
  | "The intent said whole delivery, but the spec is big, so I'll slice anyway" | The delivery ruling is the user's; a slicing the intent didn't ask for is a silent goal change. Surface the tension instead. |

  ## Related

  - [`spec-template.md`](../../templates/spec-template.md) — owns the Delivery Slices section shape this skill fills, Graduation contract included
  - `mochiko:review-specifications` — grades the section with the spec (independent reviewer, never the author)
  - `mochiko:patterns-vertical-tdd` — downstream: cuts one slice's scope into cycle cards (the vocabulary table's other "slice")
  - `mochiko:authoring-user-stories` — upstream: the story quality (IDs, priorities, independent tests) this decomposition indexes
  ````
- **Kept deliberately:** none — invariants re-keyed into `authoring-feature-map` (D4/D18/D22): exactly-one-home and complete disposition (invariants 1/2 there, now story→feature), dependency closure (invariant 3), foundation as an ordering role with the legitimacy test demoted to guidance (D22), cross-cutting extend obligations → entry obligations lines (D21/R2), the un-homeable escalation → the filter's reject-with-reasons + user escalation, the Feature-Done SC map → derivation-time SC re-homing (D21), cross-slice seams → later-landing-feature ownership (D13) + one-sided-seam entry obligations, the Graduation contract → D18's per-feature re-key (landing in plan/implement, wave 3), the single-slice exit's honesty → selection is explicit and user-ruled every run, the two-level vocabulary table (the [v0.14.0] KEPT below) → `authoring-feature-map`'s own vocabulary table (feature vs story vs vertical-slice cycle), the "letter/spirit" aphorism → present in `authoring-feature-map`. The [v0.14.0] KEPT survivor is discharged by this ruling, not silently dropped: its point-of-use disambiguation duty now lives in the successor skill's vocabulary table and `patterns-vertical-tdd`'s boundary line (re-keyed in wave 3).
- **Consumers assessed (grep at deletion, every referencing file):**
  - `plugins/mochiko/commands/specify.md` — edited this wave (binding replaced by `authoring-feature-map`).
  - `plugins/mochiko/templates/spec-template.md` — edited this wave (Delivery Slices section out).
  - `plugins/mochiko/skills/mochiko/SKILL.md` (router) — edited this wave (row removed).
  - `plugins/mochiko/skills/patterns-vertical-tdd/SKILL.md` line 28 (upstream-boundary bullet) — **wave-3 producer's file**, hand-off noted, not edited here.
  - `plugins/mochiko/commands/plan.md` line 68 + `plugins/mochiko/commands/implement.md` line 91 (Delivery Slices / Graduation-contract scope blocks) — **wave-3 producer's files**, hand-off noted.
  - `plugins/mochiko/templates/tasks-template.md` (cycle-card slice references) — **wave-3 producer's file**, hand-off noted.
  - `plugins/mochiko/skills/authoring-prototype/SKILL.md` lines 105/156 (slice-tag pass + Related bullet) — owned by this wave's skill/templates producer per the build plan, not this seat; hand-off noted.
  - `plugins/mochiko/templates/artifact-format.md` line 82 (skill-name list) + `ARCHITECTURE.md` (multiple rows/diagrams) — ripple-wave territory (wave 4).
  - `BACKLOG.md` / `DECISIONS.md` / `.mochiko/strips/*` / `.mochiko/brainstorms/*` — record layer, frozen or self-describing; no edits owed.


## [v0.49.0] Re-scoped overlay → spec section
- **Disposition:** superseded → same skill, section form: output is `spec.md`'s Delivery Slices section per `spec-template.md`; keyed to the spec's confirmed Intent delivery ruling
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D5+D6+D7)
- **Content:** "The spec stamp (staleness guard)" section · invariant 6's overlay-never-edits-spec framing · the null-exit-writes-nothing rule ("absence of this file IS the whole-spec state — never write a stub") · Process step 9's slices-template fill + stamp · overlay-purity checklist rows · review-slices as the named grader. Full text: git history at v0.48.0.
- **Kept deliberately:** all six invariants (6 reworded to section-indexes-never-rewrites) · sizing · cross-cutting/un-homeable escalation · Feature-Done declared-now-verified-at-close · the two-slices vocabulary table · single-slice exit (inverted from write-nothing to write-the-one-line, now gradeable).
- **Consumers assessed:** specify (new binder) · review-specifications (new grader) · spec-template (new shape home) · router.

## [v0.46.0] loop-discipline Related bullet out
- **Disposition:** superseded → loop ownership is the command's
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** Related bullet "`loop-discipline` — the loop, bounds, and gates around this authoring are the lead's; referenced, never restated" deleted.
- **Consumers assessed:** slice command briefs unchanged.

## [v0.28.0] R4b item 3 executed + homed mistake rows stripped (body 147 → 127, −14%, in 10–40 band)
- **Disposition:** **R4b item 3 (owed since wave-1 open, ruled for this wave):**
  `templates/slices-template.md`'s Graduation contract confirmed canonical (its own principle:
  "commands reference it rather than restating it") — the SKILL's two downstream-restatement
  clauses dropped (the Overview's "…that tells downstream consumers how to honor the
  decomposition" and the Related bullet's "…including the Graduation contract downstream
  consumers honor"); the surviving mention is the Process fill-instruction, which names the
  section without restating its rules. Width-only — the clauses sat on long single lines, so
  the measured delta is the mistakes deletion's · **Common Mistakes deleted whole** (all 6 rows
  homed in-file: horizontal-grouping → process step 5's journey-cohesion rule;
  priority-only-ordering → invariant 4 verbatim; everything-foundation → invariant 3 +
  rationalizations row 2; silent-spread → the cross-cutting section's extend-obligation rule;
  tracking-fields → the checklist's no-status-fields row; stub-on-null-exit → the null-exit
  section's "never write a stub") · densified (width-only): the review-slices and vertical-tdd
  Related bullets (their boundary content is in the description, When-NOT-to-Use, and the
  vocabulary table)
- **Tier failed:** 1 throughout (every cut had a verified in-file or template home)
- **Content:** two restatement clauses, six mistake subsections
- **Consumers assessed:** wave-open enumeration — 6 citing files (task-architect, slice,
  review-slices, slices-template, artifact-format, mochiko router); none links a section
  anchor. Kept: the two-level vocabulary table (the [v0.14.0] KEPT below, BACKLOG item 207 —
  untouched), Red Flags (**deliberately not compressed** — unlike the batch-2 precedent these
  bullets are not the rationalizations table's excuse column; they are distinct recognition
  tripwires, and the skill passed the v0.14.0 altitude assessment), invariants, sizing,
  cross-cutting, null exit, Feature-Done, spec stamp, process, checklist, rationalizations.
  Session ruling: wave-4 batch-3 ratified 2026-07-25.

## [v0.14.0] KEPT: the two-level vocabulary table (Graduation slice vs Vertical slice / cycle)
- **Tier-2 evidence:** the graduation-slice ≠ vertical-slice conflation is a named, watched risk
  (BACKLOG item 207); the point-of-use disambiguation table is what `review-slices`' "wrong
  altitude" check keys on. The boundary is also in the skill's `description:` and in `review-slices`
  (a two-homes challenge), but single-sourcing it would force a load-coupling (one skill loading the
  other); the deliberate paired anchor is the better design. Both REGISTRY rows (137/138) name the
  two-level boundary a designed feature.
