---
record_contact: none
report: review
phase: blind-angle-map
reviewer: review-seat (cold reviewer; pairing unstated in the first brief; map timing front)
built_from: frame problem, destination and out-of-scope lines + free repo grounding only
angle_count: 40
load_bearing_count: 30
class_count: 7
classes:
  A: diagnosis and the want — what is broken and how "tight" is measured
  B: conversation shape — what of brainstorm's way carries over, what is excess
  C: seats and transport — who sits in the conversation and how it physically runs
  D: placement in specify's flow — intent, capability frame, lockstep, filter, selection
  E: what the ruling binds downstream — authoring, grading, struck candidates
  F: where the rules live — rule homes, brainstorm's fence, done condition
  G: scaling and entry variants — size, skip paths, inputs already in hand
angles:
  - id: A1
    class: A
    load_bearing: true
    angle: >-
      Volume and fit are separable harms. The problem line says both "many" stories and "not
      tight to the want". Is the harm the count (over-generation past the skill's 2–5 bound, or
      2–5 per capability times several capabilities), the fit (wrong stories at a fine count), or
      ownership (correct stories the founder never made their own)? Each points to a different
      fix: enforce the bound, elicit the content, or mark provenance. A record that treats them
      as one harm may build the wrong fix.
    grounding: authoring-user-stories.story-structure ("Generate 2–5 user stories per feature"); frame problem line
  - id: A2
    class: A
    load_bearing: true
    angle: >-
      Why the confirmed intent synthesis did not bind story authoring. The intent agenda covers
      the envelope (scope, delivery, depth, UX-bearing, constraints, out of scope) and has no line
      for content, meaning the journeys or jobs the founder has in mind. Is the miss in the
      agenda, in the 10-line Intent budget, or in the hand-off to the author? The cheaper shape
      (one more probe and one more Intent line) has to be ruled against before heavier machinery.
    grounding: spec.intent-stage-first agenda; spec template Intent budget 10 lines; spec.intent-synthesis-governs
  - id: A3
    class: A
    load_bearing: true
    angle: >-
      A testable definition of "tight to the want". Candidates: every story traces to a
      founder-stated want or a founder-ratified inference; zero stories the founder would strike
      at acceptance; story count within a stated band. Without one, the build item cannot be
      graded and an eval cannot tell better from merely different.
    grounding: frame destination line; review-brainstorm.builder-test
  - id: A4
    class: A
    load_bearing: true
    angle: >-
      Evidence base. How many specify runs show the problem, which ones, with what story counts,
      and which stories the founder would have struck? "Not specific to any one dogfood project"
      still needs at least one quoted case, or the diagnosis carries Assumed. A replay of a past
      run under the proposed shape is the cheap test brainstorm itself names.
    grounding: brainstorm.facts-quoted; brainstorm.show-before-asking ("a replay on a past case"); `pm-role-and-feature-derivation` D11-evidence-success-probe n=1 honesty precedent (DECISIONS.md 2026-08-13)
  - id: B1
    class: B
    load_bearing: true
    angle: >-
      Elicit first, or propose then prune. A slate the analyst drafts and the founder approves
      repeats the failure the frame names, because the intent synthesis was also "confirmed".
      Options: the founder names the stories and the analyst sharpens them; the analyst proposes
      one-breath candidates and the founder strikes and adds; a hybrid. Brainstorm's guards
      against passive approval (ratified-yes streak flag, vague answer re-asked, convention
      probe) bear here; specify already carries only the streak flag.
    grounding: brainstorm.ratified-yes; brainstorm.vague-answer-reasked; spec.intent-probe-discipline ("flag ratification streaks")
  - id: B2
    class: B
    load_bearing: true
    angle: >-
      The unit the founder rules: capability-level journeys, one-breath story lines (the User
      Stories index already has a "Story (one breath)" column), priorities, acceptance
      scenarios, or which stories build now. A founder's want often lives at scenario level
      ("what happens when..."). Ruling titles only may push the volume one layer down, into
      scenarios, FRs and SCs that are still authored in volume.
    grounding: spec template User Stories index columns, FR/SC sections; authoring-user-stories.scenario-count-bound
  - id: B3
    class: B
    load_bearing: true
    angle: >-
      Keep or drop, part by part, with the cheaper shape named: frame card, decision map
      (askable / fog / out of scope), options rule with a do-less option, one question per turn,
      show-before-asking, blind second list, stop rule, confidence marks, decision cards, record
      file, cold review, blind angle map. A full brainstorm per spec is heavy, and specify
      already has its own stress-test. The record should say which parts earn their cost at the
      story step.
    grounding: brainstorm.sec.ways-of-working (25 rules); brainstorm.sec.roles; review-brainstorm.excess-names-cheaper-shape
  - id: B4
    class: B
    load_bearing: false
    angle: >-
      Provenance mark per candidate story, "founder's words" or "analyst's guess", the way the
      frame card marks its lines. Assumed stories are exactly the "may or may not be wanted"
      ones, so the mark makes the gap visible at the moment of choice.
    grounding: brainstorm.frame-card ("each marked as the user's words or your guess"); analysis-iterative confidence indicators (Assumed)
  - id: B5
    class: B
    load_bearing: true
    angle: >-
      Fidelity against completeness. Some stories the founder never names are still needed:
      failure recovery, empty states, permissions, admin. Who surfaces them, and how, without
      re-inflating the set? Brainstorm uses a blind second list for missed roads. Specify's
      stress-test hunts edge cases per main flow and may force a late reopen. The frame says the
      old stories were "not wrong", so part of the old volume may have been necessary ground.
    grounding: review-specifications coverage duty ("edge cases hunted per main flow"); brainstorm.blind-second-list; requirements-analyst "Consider the edges"
  - id: B6
    class: B
    load_bearing: false
    angle: >-
      Stop rule for the story step: every candidate in, out or deferred, nothing open, plus a
      stall rule when the set stops shrinking.
    grounding: brainstorm.stop-rule
  - id: B7
    class: B
    load_bearing: false
    angle: >-
      Show-before-asking for stories. Brainstorm uses a rough in-message mock or worked scenario
      and builds heavier artifacts only on request. Specify obligates a clickable prototype for
      UX-bearing work. Which artifact the story conversation uses, and when.
    grounding: brainstorm.show-before-asking; spec.lockstep-prototyping
  - id: C1
    class: C
    load_bearing: true
    angle: >-
      Who holds the conversation: the lead inline (brainstorm's shape), the requirements-analyst
      seat (the frame's words), the product-manager, or a pair. Every later angle (hand-off,
      transport, sound loop, PM boundary) changes with this choice. Today specify does not say
      who runs the intent stage either; it is under lead latitude.
    grounding: frame destination ("the analyst seat and the user"); brainstorm.lead-inline-questioning; spec.lead-latitude
  - id: C2
    class: C
    load_bearing: true
    angle: >-
      Platform fact, to verify and not assume: can a teammate seat hold a live conversation with
      the user in Claude Code agent teams, or does every user turn route through the lead? A
      subagent cannot converse at all. If turns are relayed, "the analyst works with the user"
      becomes lead-relayed messaging under the transport floor's message legs, with race risk on
      overlapping questions.
    grounding: spec.transport-floor; patterns-transport-floor; review-brainstorm references/EXTERNAL-CLAIMS.md
  - id: C3
    class: C
    load_bearing: true
    angle: >-
      Context hand-off. If the conversation holder is not the story author, the nuance travels
      in a brief. A lossy brief (the analyst authoring from a one-screen synthesis) may be the
      original root cause. Seat continuity from conversation into authoring has to be weighed
      against independence.
    grounding: spec.intent-synthesis-governs; requirements-analyst "Your Judgment" ("Make reasonable defaults for minor details")
  - id: C4
    class: C
    load_bearing: true
    angle: >-
      The PM / analyst boundary is a floor: the PM owns "which" (capabilities, the story filter,
      selection advice) and the analyst owns "how well". Ruling which stories exist is a "which"
      question. Giving it to the analyst crosses authoring-user-stories.pm-frame-boundary and the
      PM persona's remit. The record must say who owns the story set's "which", and amend the
      floor by ruling if it moves.
    grounding: authoring-user-stories.pm-frame-boundary (floor); product-manager "Where Your Remit Ends"; spec.pm-recommends-never-selects
  - id: C5
    class: C
    load_bearing: true
    angle: >-
      The sound loop and plan approval against a conversational seat. Specs are a governing
      surface: leg 1 keeps production off the lead, and spec.plan-approval requires a plan-only
      dispatch with a graded plan before a writing seat works. A live conversation does not fit
      plan-then-work. If the lead runs it inline and writes the set, is that transcription of
      user decisions (exempt) or judgment authorship?
    grounding: patterns-sound-loop.leg-1-seat-produces, exemptions-exactly-three; spec.plan-approval
  - id: C6
    class: C
    load_bearing: false
    angle: >-
      Seat tier. requirements-analyst pins sonnet. A founder-facing conversation is
      judgment-heavy, and only the lead may deviate a seat's tier, disclosed.
    grounding: plugins/mochiko/agents/requirements-analyst.md frontmatter (model sonnet); mochiko:patterns-model-tiering
  - id: C7
    class: C
    load_bearing: false
    angle: >-
      Persona posture. The analyst persona leans toward filling gaps with stated defaults rather
      than asking. Personas carry judgment and no workflow (axis 4), and persona edits take an
      advisory eval grid.
    grounding: requirements-analyst.md "Your Judgment"; CLAUDE.md skill-library axis 4; .claude/rules/mochiko/primitive-edits.md persona grid
  - id: D1
    class: D
    load_bearing: true
    angle: >-
      Placement against intent: inside the intent stage (one conversation covering envelope and
      content), right after intent confirms, or replacing part of it. Two back-to-back
      confirmations risk fatigue. Merging them risks the 10-line Intent budget.
    grounding: spec.intent-stage-first; spec template Intent budget
  - id: D2
    class: D
    load_bearing: true
    angle: >-
      Ordering against the capability frame, which is a ruled frame-first discipline: frame
      before any story, nouns and verbs only, stories win conflicts. A set ruled before the frame
      lets story shape anchor capabilities, the disease `pm-role-and-feature-derivation` D1-story-feature-mirroring named. A set ruled inside the
      frame lets the frame anchor stories, against spec.frame-hypothesis-not-anchor. Does the
      post-stories confirm step keep any work?
    grounding: authoring-feature-map.frame-first; spec.capability-frame-at-intent; spec.frame-hypothesis-not-anchor; DECISIONS.md 2026-08-13 `pm-role-and-feature-derivation` D5-pm-specify-front
  - id: D3
    class: D
    load_bearing: true
    angle: >-
      Story set against selection. "Which user stories will be worked on" can mean which stories
      the spec carries, or which build now. Selection is a floor gate after derivation, at
      work-row level. The whole-feature prototype and the completeness view assume the spec
      carries more than build-now. Ruling "worked on" up front either duplicates selection (two
      gates, one question) or has to be scoped to spec content only.
    grounding: spec.gate-selection (floor); spec.selection-card; spec.whole-feature-prototype; spec template Intent "Delivery (whole vs subset now)"
  - id: D4
    class: D
    load_bearing: true
    angle: >-
      Total founder gates end to end: intent confirm with frame, story-set ruling, per-story
      lockstep clicks, filter escalations, selection, acceptance. Each addition costs attention.
      Which existing gate shrinks or merges?
    grounding: spec.reserved-to-user; spec.sec.reserved
  - id: D5
    class: D
    load_bearing: true
    angle: >-
      The filter after a ruled set. A PM rejection of a story the founder ruled in challenges a
      user ruling and escalates. Does the filter shrink to map fit only, or move ahead of the
      ruling so the PM vets candidates first? The complete-disposition floor covers drafted
      stories only.
    grounding: spec.filter-rejections-recorded; spec.filter-disagreement-escalates; authoring-feature-map.complete-disposition (floor)
  - id: D6
    class: D
    load_bearing: true
    angle: >-
      Lockstep prototyping on UX-bearing work. The skeleton nav frame comes first and implies
      the story set. Per-story clicks fold the founder's reactions back into story and screen,
      a second and later ruling channel that can change the set after it was ruled.
    grounding: spec.lockstep-prototyping; spec.reserved-to-user ("clicking each story's prototype screens as they land")
  - id: E1
    class: E
    load_bearing: true
    angle: >-
      Fidelity during authoring. May the author add a story the set lacks (found necessary
      mid-authoring), split one, merge two, or drop one? Silent additions bring the volume back.
      Each change either routes back to the founder or is barred.
    grounding: frame destination ("ruled with the user, not authored in volume"); brainstorm.frame-changes-users-word as analog
  - id: E2
    class: E
    load_bearing: true
    angle: >-
      Grading against the ruled set. The stress-test grades stories for completeness and hunts
      edge cases. Nothing grades stories against a ruled set (no adds, no drops, no inflation),
      so a drift back to volume would pass unseen. The review's completeness duty pushes toward
      more, while its no-scope-creep floor only bars new features.
    grounding: review-specifications coverage duty; review-specifications.no-scope-creep (floor); spec.stress-test-one-pass
  - id: E3
    class: E
    load_bearing: true
    angle: >-
      Where struck and deferred candidates are recorded. Complete disposition covers drafted
      stories, so a candidate struck before drafting has no story file to hold its why. Options:
      the Out-of-scope line, Open Questions, map pending rows or stubs, nowhere. A deferred
      remainder needs a named return path under the stacking ruling.
    grounding: authoring-feature-map.complete-disposition; DECISIONS.md 2026-08-10 `pm-requirements-stacking` D1-across-round-phasing (named return path)
  - id: E4
    class: E
    load_bearing: true
    angle: >-
      Floor-derived stories. The target is production-only with breadth invariant at both depth
      levels. May the founder strike a story the floor obligates, such as auth, data deletion or
      error recovery? Floor obligations are never excess, so a strike needs a waiver path.
    grounding: CLAUDE.md target line (production floor, breadth invariant); review-brainstorm.never-excess
  - id: E5
    class: E
    load_bearing: true
    angle: >-
      The 2–5 stories per feature bound against a founder-ruled count: which wins, and is
      "feature" the spec or each capability? The same rule says stories land in spec.md, while
      the spec template makes User Stories an index with content in stories/US-*.md. That
      standing dual home is touched by any change here.
    grounding: authoring-user-stories.story-structure, authoring-user-stories.artifact-home; spec template User Stories section
  - id: E6
    class: E
    load_bearing: false
    angle: >-
      Mid-run changes to the set from lockstep reactions, frame-confirm discoveries, or
      stress-test findings: how a changed story re-enters, one at a time and stamped.
    grounding: brainstorm.frame-changes-users-word
  - id: F1
    class: F
    load_bearing: true
    angle: >-
      Rule home options: specify's own schema; analysis-iterative as a third output shape beside
      general analysis and specification-input; a discovery section in authoring-user-stories;
      the analyst persona; a new skill; a common block shared with brainstorm. Single source
      (GI-017-pointer-only-region) and axis-4 decoupling constrain the choice, and personas carry no workflow.
    grounding: analysis-iterative "Two output shapes, one engine"; CLAUDE.md axes 1 and 4; GI-017-pointer-only-region
  - id: F2
    class: F
    load_bearing: true
    angle: >-
      Carrying brainstorm's way over without touching brainstorm's rules. Much of that way lives
      in brainstorm's own rules, which override analysis-iterative where they differ. An extends
      common block needs a 3+ command family and turns brainstorm's rules into stubs, which edits
      brainstorm and is out of scope. A copy restates. A pointer from one command into another
      command's rule ids is not an existing pattern.
    grounding: brainstorm.own-rules-win; .claude/rules/mochiko/primitive-edits.md criterion 11 (3+ command extraction bar); frame out-of-scope line
  - id: F3
    class: F
    load_bearing: true
    angle: >-
      Convergence with the sibling session architecture-brainstorm-interview, seen by directory
      name only. If two commands import brainstorm's way at the same time, two shapes can
      diverge. With brainstorm, three commands might meet the 3+ extraction bar, which changes
      the home question.
    grounding: gitStatus untracked .mochiko/brainstorms/architecture-brainstorm-interview/; ls of .mochiko/brainstorms/
  - id: F4
    class: F
    load_bearing: true
    angle: >-
      Specify's fixed done condition and fail set. A ruled story set joins the Goal and needs a
      kind fail rule (set never ruled, or story authored outside it). Its ruling needs a home in
      spec.md, whose heading set is closed (an undeclared heading is denied) and whose Intent
      section is capped at 10 lines. A template change is a migration.
    grounding: plugins/mochiko/commands/specify.md Goal step; spec template Conformance; spec.artifact-home (floor)
  - id: F5
    class: F
    load_bearing: false
    angle: >-
      Build cost: a primitive-edit landing with migrations, audits, strips, the specify eval kit
      grid, a persona grid if the analyst is edited, and char budgets.
    grounding: .claude/rules/mochiko/primitive-edits.md; BACKLOG.md specify kit lines (~330–344)
  - id: G1
    class: G
    load_bearing: true
    angle: >-
      Adaptive size. A small extension and a new product area should not pay the same session.
      A decisive founder wants confirmations and a fast wrap-up; an unsure one needs probes. A
      size gate or skip path keyed to what: depth level, brainstorm-style size, or the founder's
      word? Without one, every spec pays a full session or the founder routes around it.
    grounding: analysis-iterative "When user has clear direction"; brainstorm size condition; CLAUDE.md depth level low/high
  - id: G2
    class: G
    load_bearing: true
    angle: >-
      Entry with stories already in hand, or from an accepted brainstorm record that brainstorm
      offers to specify. Re-asking ruled content is waste, and the story skill already says not
      to duplicate story-form input.
    grounding: brainstorm.next-step-offer; authoring-user-stories.already-story-form
  - id: G3
    class: G
    load_bearing: false
    angle: >-
      Brownfield inputs. Pending work rows and parked stubs in the territory are candidate
      stories already. Stubs are never derivation anchors, but maturation routes through specify.
      Can the conversation use them as prompts?
    grounding: spec.unrefined-stubs; spec.selection-card completeness view
  - id: G4
    class: G
    load_bearing: false
    angle: >-
      Large or multi-capability specs: a set of 15 or more stories. Split the spec, propose an
      epic, or rule in rounds.
    grounding: spec.epic-proposal-optional
  - id: G5
    class: G
    load_bearing: false
    angle: >-
      Testability of a conversational stage. The specify eval kit and contract suite run
      headless, so the story step needs a scripted founder. Evidence that the change works rides
      that.
    grounding: BACKLOG.md specify kit lines; evals/commands/specify/
---

## Notes of note

- Fence: no file under the session directory was opened; `index.md` was not opened. Two
  wildcard listings across every session's `reports/` directory included this session's path
  at the shell level; output was filtered by name before display and nothing from it showed.
- Grounding: specify and brainstorm rules (all sections), specify.md and brainstorm.md, the
  analysis-iterative skill and its specification-input variant, authoring-user-stories rules,
  both personas, authoring-feature-map scope/inputs/story rules, the spec template,
  review-specifications story rules, sound-loop discipline, DECISIONS/BACKLOG/ROADMAP greps.
- Tiering deviation: one haiku Explore spawn for the DECISIONS/BACKLOG locate returned no
  report; the same small greps then ran on the seat tier. Every other read was interpretive.
- C2 is an outside-repo platform claim, unverified here; it needs a source or a probe.
- Load-bearing means a gap there would plausibly or likely have changed a ruling.
