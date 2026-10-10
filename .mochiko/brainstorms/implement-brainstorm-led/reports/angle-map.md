---
report: review
stage: blind-angle-map
seat: review-seat (devils-advocate; solo cold reviewer, never in the room)
map_timing: front
record_contact: none
built_from: the frame's problem, destination and out-of-scope lines (hardened 2026-10-11) plus free repo grounding only
angle_count: 45
load_bearing: 33
class_count: 10
classes:
  A: diagnosis and evidence — is the problem real, and what "ships once" means
  B: the shared core against the local parts — what the engine is
  C: home and delivery — where the engine lives, precedence, grammar
  D: implement's design phase fit — gap scope, firing, visit file, checkpoint, refire
  E: the ruled unit and the founder's altitude in technical design
  F: seats — who holds the conversation, blind lists, pen pass, the sameness grade
  G: homes and state of the ruled set
  H: gates and fail conditions — the founder's gate count
  I: entry variants — carry-forward across the chain, sizing, skips
  J: build and landing — sequencing with A and B, the sibling session, supersessions
angles:
  - id: A1
    class: A
    load_bearing: true
    angle: >-
      Evidence base of n=0 at every instance. The specify story stage is built (v0.119.0) but its
      first run is still owed; the desk's focus stage is unbuilt (build item A); implement's
      problem is marked Assumed; brainstorm's target-state dogfood watch is open. A shared engine
      abstracted from four instances none of which has run risks fixing the wrong common shape.
      The cheaper road has to be ruled against in writing: build the desk and implement stages
      locally, converge after the first-run logs.
    grounding: >-
      BACKLOG.md:57 (first desk run), :66 (first specify run), :128 (target-state dogfood watch);
      frame problem "Not yet shown in a real implement run (unknown — Assumed)";
      architecture-brainstorm-interview D6-desk-stays-engine accepted risk
  - id: A2
    class: A
    load_bearing: true
    angle: >-
      Which design decisions the founder does not own today. The design phase drafts data model,
      contracts, constraints and decisions, quickstart, the design baseline's truth part, the
      store's proposed elements and the signed drawing. Which of these does the founder want to
      think through, and which want defaulted? Without that split the stage has no agenda and
      either asks everything or nothing.
    grounding: >-
      impl.design-outputs-home; impl.gate-design-checkpoint (the founder signs a rendered diagram,
      AX row changes and a pinned-base diff)
  - id: A3
    class: A
    load_bearing: true
    angle: >-
      A testable meaning of "ships once". One source text all four point at; one procedure with
      per-command parameters; or one block in the migration log with local stubs. analysis-iterative
      already claims "Two output shapes, one engine", yet brainstorm and specify each overrode it
      by a precedence clause. Without a measure (a net-reduction test, a detector, a single file
      count) the build cannot be graded and the copies drift back.
    grounding: >-
      plugins/mochiko/skills/analysis-iterative/SKILL.md:50; brainstorm.own-rules-win;
      spec.own-rules-win; near-dup ruling R5 (.mochiko/decisions/2026-08-28-near-dup-convergence.md)
  - id: A4
    class: A
    load_bearing: true
    angle: >-
      Change control of the shared stage. The existing engine split because each later session
      kept its reach local (brainstorm-only reach; specify's own rules) to avoid touching siblings.
      Once shared, a change for one command reaches four. Who may amend it, what a command may
      override locally and how narrowly, and whether a local override is itself a recorded ruling,
      decide whether the stage stays shared.
    grounding: >-
      brainstorm-target-state D19-brainstorm-only-reach (DECISIONS.md:91 annotation);
      specify-brainstorm-discovery D5-specify-own-rules; extends local class: precedence
      (.claude/rules/mochiko/primitive-edits.md:215-220)
  - id: A5
    class: A
    load_bearing: false
    angle: >-
      What would show the shared stage worse than the local copies. Brainstorm has D23 counts,
      specify deferred its criteria to the first run, the desk deferred them too (OQ1). A fourth
      deferral leaves the engine with no falsifier; a regression in brainstorm's counts after the
      move is the obvious one to name.
    grounding: >-
      DECISIONS.md:16 (D23); specify-brainstorm-discovery D7-dogfood-criteria-deferred;
      BACKLOG.md:57-62 (OQ1)
  - id: B1
    class: B
    load_bearing: true
    angle: >-
      The candidate core, and where the four already differ. Shared today in wording: fact in hand
      before a fork, one fork per turn with a do-less option, the ratified-yes streak, a vague
      answer re-asked as two options, a mock shown before asking, a status line, a defaults batch
      on one screen, a stop rule with a three-turn step-back, provenance marks, a seed sentence.
      Different today: brainstorm's blind second list is per costly fork, specify's and the desk's
      is one per run or visit; brainstorm has a size the user rules, specify has no size gate, the
      desk skips on a single-row ask. Each difference is a ruling: unify, or make it a parameter.
    grounding: >-
      brainstorm.fact-before-decision, .options-rule, .ratified-yes, .vague-answer-reasked,
      .show-before-asking, .stop-rule, .blind-second-list; spec.story-forks,
      spec.story-stage-stop-rule, spec.blind-journey-list, spec.stage-inputs-in-hand;
      architecture-brainstorm-interview D1, D4
  - id: B2
    class: B
    load_bearing: true
    angle: >-
      Brainstorm does not fit "think, then author inside the ruled set". Its deciding produces
      decision cards in the lead's own record with no author seat after it, and its frame card,
      decision map, front blind angle map and cold review are session-level, not stage-level. Is
      brainstorm a consumer of the engine or the engine's source? If the source, extracting it
      re-touches 55 rules rebuilt at v0.117.0.
    grounding: >-
      brainstorm.decision-cards, .frame-card, .decision-map, .blind-map-dispatch; DECISIONS.md:16
      (brainstorm 30 to 55 rules at v0.117.0)
  - id: B3
    class: B
    load_bearing: true
    angle: >-
      The parameter interface each command must supply: the ruled unit, the home of the ruled set,
      who authors after it, the blind list's inputs and fence, the confirm gate, the stop
      condition, the skip or handed-in condition, the provenance set. Declared where: per-command
      schema rules, the dispatching command's brief (caller-side context rides the brief), or a new
      grammar construct?
    grounding: >-
      CLAUDE.md axis 4 ("Caller-side context rides the dispatching command's own brief");
      spec.slate-unit, spec.slate-home, spec.handed-in-slate; architecture-brainstorm-interview D2, D3
  - id: B4
    class: B
    load_bearing: false
    angle: >-
      Shaped for four or for more. Setup's questioning, specify's intent stage and implement's card
      slicing (handed to the follow-up session) are the next likely consumers. Building only what
      the four need is the plan-minimalism default; the follow-up session should not have to reopen
      the engine to plug in.
    grounding: >-
      mochiko:patterns-plan-minimalism ladder; frame out-of-scope line (named follow-up session)
  - id: B5
    class: B
    load_bearing: true
    angle: >-
      The seam with specify's intent stage. Intent runs through analysis-iterative's probe agenda
      immediately before the story stage. If the story stage moves to a new engine, one run has two
      questioning regimes back to back; if the engine replaces analysis-iterative's conduct, intent
      changes too, and intent is not named in the destination.
    grounding: >-
      spec.intent-stage-first; spec.intent-probe-discipline; plugins/mochiko/skills/mochiko/SKILL.md:66
  - id: C1
    class: C
    load_bearing: true
    angle: >-
      Where the engine lives, with each road's touch set. (a) Widen analysis-iterative: a prose
      skill outside the migration log whose "When NOT to Use" excludes "Implementation details —
      use planning skills instead", and which setup.interrogation-inline also points at, so the
      edit reaches setup's questioning (out of scope). (b) A new model-invoked skill pair delivered
      by mochiko-cli, each command pointing at it. (c) A common block by extends, inside the
      near-dup charter "boilerplate never judgment". (d) A pointer from three commands into one
      command's rule ids, which no existing pattern does.
    grounding: >-
      plugins/mochiko/skills/analysis-iterative/SKILL.md:15; setup.interrogation-inline;
      brainstorm.lead-inline-questioning; spec.intent-stage-first; `mochiko-cli rules
      analysis-iterative` returns "no command or skill named 'analysis-iterative' in the log";
      BACKLOG.md:77-84
  - id: C2
    class: C
    load_bearing: true
    angle: >-
      Precedence flips. brainstorm.own-rules-win and spec.own-rules-win make local rules beat the
      engine. A shared stage needs the reverse, or local overrides named one by one. Both clauses
      are ruled content and leave only by recorded supersession.
    grounding: >-
      brainstorm.own-rules-win; spec.own-rules-win; CLAUDE.md "Protected content ... leaves only as
      a recorded supersession-by-ruling"
  - id: C3
    class: C
    load_bearing: true
    angle: >-
      The skill-to-command line. User gates, reservations, kind fail rules, moments and conditions
      are command grammar; skills carry procedure. A stage that closes on a confirm gate (the slate
      confirm) cannot live wholly in a skill. Where the engine ends and each command's gates begin
      has to be drawn, or the gates get restated four times.
    grounding: >-
      spec.gate-slate-confirm; CLAUDE.md axis 4 (persona judgment, skill procedure); command
      architecture realignment D2 (harness = goal + user-reserved decisions), DECISIONS.md:112
  - id: C4
    class: C
    load_bearing: true
    angle: >-
      Grammar and binary cost. A block shared between a skill and commands, a new pointer form, or
      shared moments or conditions are grammar changes: crate code under independent review, a
      mochiko-cli release before the plugin bump that needs it (a log outside the binary's grammar
      range halts a run), while mochiko-cli 0.4.0 is itself still unpublished. The cheapest road
      stays inside grammar 2.
    grounding: >-
      CLAUDE.md GI-020-plugin-install-model and GI-012 tag gates; .claude/rules/mochiko/rust-cli.md;
      BACKLOG.md:95 (0.4.0 unpublished); render header "grammar 2"
  - id: C5
    class: C
    load_bearing: false
    angle: >-
      Description budget and trigger collision. A new model-invoked skill needs graded MUST/SHOULD
      trigger phrases within 1,536 characters, must not steal analysis-iterative's triggers
      ("brainstorm", "think-through"), and needs a router entry in the mochiko skill.
    grounding: >-
      CLAUDE.md axis 3; plugins/mochiko/skills/mochiko/SKILL.md:66, :124
  - id: D1
    class: D
    load_bearing: true
    angle: >-
      Gap scope against the founder's direction. Design seats author exactly the named gaps, and
      the design review auto-fails material divergence past the gap list. A founder who rules a
      direction wider than the sufficiency gap list trips that blocking check. Is the stage's
      agenda exactly the gap list, or may the founder widen it, and if so how the gap list re-keys.
    grounding: >-
      impl.design-gaps-only; mochiko:review-plan-artifacts description ("nothing materially past
      the gap list; material divergence auto-FAILs — BLOCKING")
  - id: D2
    class: D
    load_bearing: true
    angle: >-
      When the stage fires. The design phase fires only on gaps, so a fully sufficient batch goes
      straight to cards with no thinking stage; is that intended (spec and desk already ruled) or
      a hole? On epic scope the design phase always fires over the joint spine: one stage over the
      joint design, or one per member?
    grounding: >-
      impl.design-phase-fires-on-gap; impl.epic-design-always-fires; impl.zero-gap-map-assertion
  - id: D3
    class: D
    load_bearing: true
    angle: >-
      Placement inside the run. After the sufficiency check, the run-open confirmation already
      reserves rulings (store trips, in-flight conflicts, uncleared clauses). Does the stage merge
      with run-open, follow it, or sit after the design seats' plan-only dispatch? Does the ruled
      set become the scope input the peer plan grade reads?
    grounding: >-
      impl.user-runopen-rulings; impl.design-gaps-only ("each on a plan a fresh peer passed");
      impl.plan-approval-producers
  - id: D4
    class: D
    load_bearing: true
    angle: >-
      The visit file's interplay. Where a desk visit file keyed to the feature exists it is a
      design input; the stage runs itself "when no visit file exists". A visit file covering
      structure while the gaps also name data model or contracts is partial: skip, run on the
      remainder (like specify's handed-in source), or run anyway? Who decides that the file covers
      a gap?
    grounding: >-
      architecture-brainstorm-interview D5-visit-file-input, D6-desk-stays-engine; spec.handed-in-slate
  - id: D5
    class: D
    load_bearing: true
    angle: >-
      The design checkpoint after the stage. The checkpoint is a floor and its skip a kind fail:
      the founder still signs drafts after the seats author. With thinking upstream, is the
      checkpoint a second sign-off, or does it become a check that the drafts kept the ruled set's
      meaning? The frame names sign-offs over finished drafts as the problem; leaving the checkpoint
      unchanged keeps part of it, and changing a floor is a supersession.
    grounding: >-
      impl.gate-design-checkpoint (floor); impl.fail.design-skipped; frame problem line
  - id: D6
    class: D
    load_bearing: true
    angle: >-
      Mid-run refire. A builder hitting undesigned structure halts the cycle and re-fires the
      design phase. Does the stage re-run mid-build, a founder conversation in the middle of a
      cycle, against the rule that reserved questions batch at the cycle checkpoint and only a
      build-blocking one interrupts? Cycle-checkpoint batches are out of scope; refire belongs to
      the design phase.
    grounding: >-
      impl.midrun-refire; impl.escalation-batching; frame out-of-scope line
  - id: D7
    class: D
    load_bearing: true
    angle: >-
      Which design surfaces the stage covers, and where the design phase ends. Staffing names
      technical-analyst for baselines, principal-architect for the store and drawing, qa-engineer
      for TEST cases. TEST cases sit at the card-slicing border (out of scope). The stage's reach
      over each surface, and the line against card slicing, need drawing.
    grounding: >-
      impl.design-seats-staffing; impl.design-outputs-home; mochiko:patterns-vertical-tdd owns the
      TEST grammar
  - id: E1
    class: E
    load_bearing: true
    angle: >-
      The ruled unit at implement. Specify rules a one-breath line, priority and seed test; the desk
      rules a part, the user's direction and a "how I'd know" sentence or open; brainstorm rules a
      decision card. At implement: per gap, per design element (entity, endpoint, store element,
      constraint), or per consequence? Is "direction plus how I'd know" the shared unit?
    grounding: >-
      spec.slate-unit; architecture-brainstorm-interview D2-part-direction-know; brainstorm.decision-cards
  - id: E2
    class: E
    load_bearing: true
    angle: >-
      The founder's altitude in technical design. Forks on schemas, endpoints or topology put to a
      non-technical founder invite a ratified-yes streak: the same passive acceptance the problem
      names, one layer down. Options: rule consequences the founder would see, default technical
      forks in a batch, show worked scenarios with real numbers. The user has already asked for
      simpler questions mid-session.
    grounding: >-
      brainstorm.question-form, .show-before-asking, .ratified-yes;
      .mochiko/brainstorms/architecture-brainstorm-interview/record.md:1000 ("i am confused ask
      question again in simople language")
  - id: E3
    class: E
    load_bearing: false
    angle: >-
      Where implement's seed sentence goes next. The founder's "how I'd know" plausibly seeds the
      TEST cases, which belong to card slicing (out of scope). Name the hand-off so the follow-up
      session can consume it rather than re-ask.
    grounding: >-
      mochiko:patterns-vertical-tdd; frame out-of-scope line
  - id: F1
    class: F
    load_bearing: true
    angle: >-
      Who holds the conversation at implement. Specify ruled the lead holds the stage inline
      (Contested), and the desk's lead holds the visit file's pen. Implement's lead also
      orchestrates seats across a long run that may stop and resume; an inline founder conversation
      adds main-pane context load. Implement's long run is new ground for that Contested ruling.
    grounding: >-
      specify-brainstorm-discovery D3-lead-holds-stage (Contested); spec.lead-holds-story-stage;
      impl.gate-design-checkpoint ("may stop at the checkpoint and resume the build later")
  - id: F2
    class: F
    load_bearing: true
    angle: >-
      The blind list at implement and the three blind mechanisms. Inputs and fence (spec,
      sufficiency report, store index, visit file; never the ruled set or the conversation), how
      many per run, and the seat (persona-less, model opus). Brainstorm's per-fork second list,
      specify's one-per-run journey list and the desk's one-per-visit list differ; one mechanism
      or three?
    grounding: >-
      spec.blind-journey-list; architecture-brainstorm-interview D4-blind-list-visits;
      brainstorm.blind-second-list
  - id: F3
    class: F
    load_bearing: true
    angle: >-
      The pen pass and the sameness grade. Specify's analyst expands the seed without change of
      meaning; the desk's tech-lead grades rows against the visit file for sameness. Implement's
      design review grades conformance to the gap list and feasibility, not sameness to the
      founder's ruled set. Who grades "authored inside the ruled set, meaning kept" at implement?
      And does lead transcription of the ruled set stay outside the sound loop, as at the desk?
    grounding: >-
      spec.analyst-brief-and-pen; architecture-brainstorm-interview D3-visit-file-home;
      impl.design-review-pair; impl.sound-loop-floor
  - id: F4
    class: F
    load_bearing: false
    angle: >-
      Parallel seats during the conversation. The blind list, the sufficiency grader and plan-only
      dispatches run while the founder talks; fan-in before the set closes rides the transport
      floor's message legs.
    grounding: >-
      impl.transport-floor; mochiko:patterns-transport-floor
  - id: F5
    class: F
    load_bearing: false
    angle: >-
      "Facts first" at implement leans on cheap fact reads. The haiku Explore locate route returned
      nothing in every seat attempt of the last build (and again for this map); a stage whose forks
      wait on facts needs a route that delivers.
    grounding: >-
      BACKLOG.md:117-121; brainstorm.fact-before-decision; mochiko:patterns-model-tiering
  - id: G1
    class: G
    load_bearing: true
    angle: >-
      The home of implement's ruled set. A new deliverable in the feature home, the run folder, or
      the desk's visit file reused? Homes are migration-declared and an undeclared name is refused.
      The set must survive a stopped and resumed run. One shared template for the four ruled sets,
      or a home per command?
    grounding: >-
      impl.artifact-home (floor); impl.gate-design-checkpoint (stop and resume);
      architecture-brainstorm-interview D3-visit-file-home; spec.slate-home
  - id: G2
    class: G
    load_bearing: false
    angle: >-
      One provenance vocabulary. Specify and the desk already share "user's words · lead's guess,
      ratified · blind list · handed in"; brainstorm records "ratified" in how-it-was-decided.
    grounding: >-
      spec.slate-provenance; architecture-brainstorm-interview D3-visit-file-home; brainstorm.ratified-yes
  - id: G3
    class: G
    load_bearing: false
    angle: >-
      Downstream readers of implement's ruled set. Final validation's blind gap-finder reads the
      sufficiency report and design deltas as the promised surface; a founder-ruled design set is
      promised behavior too. Final validation is out of scope, but the stage changes what it reads.
    grounding: >-
      impl.gap-finding-blind-dispatch; mochiko:testing-gap-finding fence
  - id: G4
    class: G
    load_bearing: false
    angle: >-
      IDs for ruled items. Slate rows are US-n, focus items cite store IDs, decisions are joined
      D-ids; a new ID family for implement's items would need minting to the IDs section.
    grounding: >-
      templates/artifact-format.md IDs section; review-brainstorm.joined-ids
  - id: H1
    class: H
    load_bearing: true
    angle: >-
      The founder's gate count per implement run: run-open, the stage's close, the design
      checkpoint, card confirm, cycle checkpoints, final acceptance. Adding a confirm without
      merging or retiring one raises the gate fatigue that drives rubber-stamping; specify already
      added one. The net count is a ruling.
    grounding: >-
      implement moments (run-open, design-checkpoint, card-confirm, cycle-checkpoint, acceptance);
      specify-brainstorm-discovery D6-two-confirm-screens
  - id: H2
    class: H
    load_bearing: true
    angle: >-
      Fail conditions. Each instance needs a kind fail (specify's story outside the slate; item
      A's planned desk fail; implement's new one, a design authored outside the ruled set or the
      stage skipped). A kind fail must enforce a live local rule, so a shared fail cannot enforce a
      skill's rule: one fail per command, each enforcing its local pointer rule?
    grounding: >-
      spec.fail.story-outside-slate; BACKLOG.md:28-31 (item A's kind fail);
      .claude/rules/mochiko/primitive-edits.md:215-217 ("each listed ID resolving to a live local rule")
  - id: H3
    class: H
    load_bearing: false
    angle: >-
      Testability. Each new floor or fail needs contract-suite cases, and a conversational stage
      needs a scripted founder headless, as the specify kit did.
    grounding: >-
      CLAUDE.md gate 6; DECISIONS.md:14 (contract suite 97/97 at the specify build)
  - id: I1
    class: I
    load_bearing: true
    angle: >-
      Carry-forward along the chain. A founder going brainstorm, specify, desk, implement meets the
      engine four times; a fork ruled upstream must arrive as a fact and never be re-asked. Which
      ruled sets implement's stage reads as handed in (the spec's Intent and slate, the visit file,
      a brainstorm record), and the "only what they leave open is asked" rule at implement.
    grounding: >-
      spec.stage-inputs-in-hand; architecture-brainstorm-interview D5-visit-file-input;
      brainstorm.next-step-offer
  - id: I2
    class: I
    load_bearing: true
    angle: >-
      Sizing and skips. Brainstorm sizes by the user's ruling, specify has no size gate, the desk
      skips on a single-row ask; implement has scopes (selection, delta, epic, lane) and a depth
      level, and a lane never widens. A one-gap delta or a lane run paying a full stage, or a
      decisive founder: one size rule for the engine, or per command?
    grounding: >-
      brainstorm.size-users-ruling; spec.stage-inputs-in-hand; architecture-brainstorm-interview
      D4; implement conditions scope and depth; impl.lane-never-widens (floor)
  - id: I3
    class: I
    load_bearing: false
    angle: >-
      Greenfield with no baseline. Every surface is open and the seed is reconstructed and
      confirmed at the checkpoint: the stage's heaviest case, kept apart from the feature's own
      design.
    grounding: >-
      impl.design-absent-baseline-seed
  - id: J1
    class: J
    load_bearing: true
    angle: >-
      Sequencing with build items A and B. The BACKLOG item's trigger says "build items A and B
      landed"; the destination says built together with them. Building the desk's focus stage as
      an engine instance changes item A's touch set (its moment, duties, kind fail and precedence
      clause). Built local then refactored, or built on the engine? The trigger change is itself
      an operating-doc change to record.
    grounding: >-
      BACKLOG.md:47-50; BACKLOG.md:28-35 (item A); frame destination line
  - id: J2
    class: J
    load_bearing: true
    angle: >-
      The line against the three-command convergence session. That session converges near-identical
      wording across 3+ commands into a common block, boilerplate only, beside and never merged
      with this one. Converging the fork, streak and stop rules is exactly near-identical wording
      across three commands. Which rules are judgment (here) and which boilerplate (there), so
      neither double-touches nor leaves a gap?
    grounding: >-
      BACKLOG.md:77-84; near-dup ruling R1 ("boilerplate never judgment");
      architecture-brainstorm-interview D6 changed at review (S15)
  - id: J3
    class: J
    load_bearing: true
    angle: >-
      Supersessions owed. brainstorm-target-state D19-brainstorm-only-reach and its precedence
      clause, specify D5-specify-own-rules, D6-desk-stays-engine's hand-off wording, and the
      analysis-iterative lines the brainstorm watch already flags. Each leaves only by recorded
      ruling with strip entries per primitive.
    grounding: >-
      DECISIONS.md:91 annotation; DECISIONS.md:14 (D5); BACKLOG.md:128-140 (analysis-iterative
      Overview lines); CLAUDE.md landing ritual
  - id: J4
    class: J
    load_bearing: true
    angle: >-
      Regression to freshly built stages. Brainstorm (v0.117.0) and the specify story stage
      (v0.119.0) would be re-homed onto the engine before either has run once in the dogfood repo.
      The pending ai-fileops specify run would then test the engine, not the built stage; which
      version that run tests must be decided.
    grounding: >-
      BACKLOG.md:66-76; BACKLOG.md:128; DECISIONS.md:14, :16
  - id: J5
    class: J
    load_bearing: false
    angle: >-
      Build cost: migrations across four commands and a skill, author-not-grader audits per unit,
      strips, contract cases, description budgets, CHANGELOG and marketplace sync.
    grounding: >-
      .claude/rules/mochiko/primitive-edits.md; CLAUDE.md release gates
---

## Notes of note

- Fence: nothing under the session directory was opened, listed or grepped, other than this
  file's write; `index.md` was not opened; nothing under `~/.claude/projects/` was read. The
  `ls .mochiko/brainstorms/` listing was filtered to exclude the session directory's name.
- Grounding read: the full rendered rules of `brainstorm`, `specify`, `architecture` and
  `implement` (plugin 0.119.0); the `analysis-iterative` skill; BACKLOG lines 1-140; DECISIONS
  rows for the three sibling sessions and plan-stage retirement; the architecture session's
  D1-D6 cards and OQ list; the specify session's angle map for shape; primitive-edits extraction
  bar; the near-dup ruling's R1-R6 headings.
- Tiering: one haiku `Explore` locate spawn returned no report (the BACKLOG seat-transport
  finding again); the same greps ran on the seat tier.
- Load-bearing means a gap there would plausibly or likely have changed a ruling.
