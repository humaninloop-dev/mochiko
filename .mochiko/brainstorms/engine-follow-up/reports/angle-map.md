---
report: review
stage: blind-angle-map
seat: blind-map (devils-advocate; cold end-stage reviewer, never in the room)
map_timing: end
record_contact: none
built_from: the frame's problem, destination and out-of-scope lines as dispatched, plus free repo grounding only
angle_count: 57
load_bearing: 39
class_count: 11
classes:
  A: diagnosis and evidence — is "two ways of being asked" felt, and what it costs
  B: the inventory and the sorting test — every ruling place, engine against plain sign-off, scope edges
  C: /mochiko:feature on the engine — visit kinds, the unit, weight, next writer, the twin desk
  D: feature's ruled set — its home, lifetime and readers
  E: setup — the contract half, and the asking half it already follows
  F: specify's intent stage — the contract half
  G: implement's card confirm and the seed's hand-off to the TEST cases
  H: implement's cycle-checkpoint batches
  I: implement's final validation and acceptance
  J: the founder across the whole chain — gate count, carry-forward, altitude, who holds the talk
  K: build and landing — size, supersessions, neighbours, tests
angles:
  - id: A1
    class: A
    load_bearing: true
    angle: >-
      Evidence base of n=0, stacked. The frame says no real run has felt the problem; the
      predecessor session's OQ1 said the same of implement; the first ai-fileops runs of the
      specify story stage, the desk's focus stage and implement's design stage are all still open,
      each "judged from that log, never before it". Moving four more ruling places onto an engine
      that has not run once stacks a second unproven layer on the first. The cheaper road — wait
      for the three logs, or build only what does not hang on them — has to be ruled against in
      writing, not passed over.
    grounding: >-
      frame problem line ("Whether a real run has felt any of it: unknown"); BACKLOG.md:36-42,
      :78-84, :87-97; .mochiko/brainstorms/implement-brainstorm-led/record.md:1079 (OQ1)
  - id: A2
    class: A
    load_bearing: true
    angle: >-
      Which harm is being fixed. The problem names two: inconsistency (the founder learns two
      asking styles) and passive acceptance ("sign this finished draft"). They imply different
      fixes — inconsistency is cured by feature joining the engine; passive acceptance by the
      sign-off form (a landing table, as D8 built) whether or not the engine runs. Without naming
      which harm each moment carries, "stay a plain sign-off, with the reason" has no yardstick.
    grounding: >-
      frame problem line; implement-brainstorm-led D8-checkpoint-shows-landing (record.md:897);
      impl.gate-design-checkpoint
  - id: A3
    class: A
    load_bearing: false
    angle: >-
      No falsifier per moment. The predecessor named untested bets with tests (turns, time, pulled
      defaults). Each moment moved here needs one named, or an explicit deferral to the first run's
      log; a moment moved with neither cannot later be shown worse.
    grounding: >-
      implement-brainstorm-led record.md:1088-1099 (Untested bets)
  - id: A4
    class: A
    load_bearing: true
    angle: >-
      Feature's asking has never been seen in a live run either. The first live feature-surface run
      watch from v0.61.0 is still open, so "feature asks its own way" is a reading of its rules, and
      its engine fit was marked Assumed at the predecessor. Designing feature's unit and home on
      paper may fit a desk no one has used; deferring feature until a logged visit is a real road.
    grounding: >-
      BACKLOG.md:670-676; implement-brainstorm-led D4-shaped-for-six accepted risk ("feature's fit
      is Assumed")
  - id: B1
    class: B
    load_bearing: true
    angle: >-
      The inventory is wider than the problem's list. "Every place a user command rules something"
      also covers: implement's run-open rulings, scope escalation and design-first-write; specify's
      selection gate, post-stories frame confirm, acceptance and filter-disagreement escalation;
      setup's mode ruling, ratification, brownfield conflict rulings, coverage-survivor routing and
      final acceptance; feature's merge/retire, route overrides, selection and epic mint; the
      architecture desk's drift dispositions, baseline confirmation, shelf-scope override and
      waiver; brainstorm's frame hardening and acceptance. Only the built stages are out of scope.
      Either each is placed, or the destination's "every place" is narrowed by ruling.
    grounding: >-
      impl.user-runopen-rulings, impl.scope-escalation-fail, impl.design-first-write;
      spec.gate-selection, spec.confirm-frame-post-stories, spec.gate-acceptance,
      spec.filter-disagreement-escalates; setup.user-mode-ruling, setup.gate-synthesis-ratification,
      setup.user-conflict-rulings, setup.coverage-survivor-routing, setup.gate-final-acceptance;
      feat.user-reserved; arch.user-reserved-rulings; brainstorm moments frame-hardened, acceptance
  - id: B2
    class: B
    load_bearing: true
    angle: >-
      A stated sorting test. Candidates: is there a next writer the ruling should hold; does the
      ruling have two or more real options; is the thing ruled content still to be written or an
      outcome already built; is a wrong pick costly to undo (the engine's own own-turn-or-batch
      test). Without one test, each moment is ruled ad hoc, and the next command to add a ruling
      place has nothing to apply.
    grounding: >-
      analysis-iterative.own-turn-or-batch, .contract-pen; frame destination ("with the reason")
  - id: B3
    class: B
    load_bearing: true
    angle: >-
      Steelman for the plain sign-off. The engine is built for rulings before writing — the pen
      passes "before the next writer opens it". Accepting built code, or a finished verification,
      has no next writer to hold; putting it as forks may only add turns. The honest test is
      whether a sign-off moment hides a pre-write ruling (then move that ruling earlier) or is a
      true outcome check (then keep it plain, perhaps in landing-table form).
    grounding: >-
      analysis-iterative.contract-pen; impl.gate-final-acceptance; impl.gate-design-checkpoint
      (landing-table form)
  - id: B4
    class: B
    load_bearing: true
    angle: >-
      The edge with "the engine's own asking rules" (out of scope). Engine precedence says a
      difference a command needs "becomes a slot by ruling" — an engine edit. If fitting a moment
      needs a new slot (a batch cadence for checkpoints, a visit-kind skip for the desk) and a slot
      is an asking rule, every such moment is forced to "plain sign-off" by the frame, not by its
      merits. Whether adding a slot is in scope needs ruling up front.
    grounding: >-
      analysis-iterative.engine-precedence, .caller-slots; frame out-of-scope line
  - id: B5
    class: B
    load_bearing: true
    angle: >-
      The edge with "verification mechanics" (out of scope). Final validation's user-facing part is
      finding dispositions, kind disputes and bound exhaustion; but the BACKLOG item names "final
      validation's gap-finder reading the ruled set", which changes the gap-finder's fence — a
      verification mechanic. In or out has to be ruled, or I1 is decided by default.
    grounding: >-
      BACKLOG.md:65-71; impl.gap-finding-blind-dispatch; frame out-of-scope line
  - id: B6
    class: B
    load_bearing: true
    angle: >-
      The middle forms. The destination reads binary — on the engine, or a plain sign-off — but
      three middle forms already exist: the asking half only (setup and intent today, contract
      off), the contract half only, and the landing-table sign-off without a stage (D8's form).
      Each moment's placement should name which of the four it takes.
    grounding: >-
      plugins/mochiko/skills/analysis-iterative/SKILL.md Overview ("the contract off");
      implement-brainstorm-led D4-shaped-for-six; impl.gate-design-checkpoint
  - id: C1
    class: C
    load_bearing: true
    angle: >-
      Which feature visits run a stage. A desk visit can be a health check or map query, a demand
      routed out to specify, a bug delta card, an improvement card, growth-door rows, a grooming
      ruling (merge, retire, status), epic mint/membership/close, or an architecture intake. The
      twin architecture desk runs its focus stage only on visit_kind ruling or delta; feature has no
      visit-kind condition at all.
    grounding: >-
      feat.capability-write-test, feat.growth-door, feat.delta-cards, feat.dm-epic-stewardship,
      feat.architecture-intake; arch.focus-stage (when: visit_kind=ruling|delta); feature preamble
      conditions (km_file, seats only)
  - id: C2
    class: C
    load_bearing: true
    angle: >-
      Feature's unit. One command rules several kinds of thing: the visit goal and done condition,
      a delta card's 1-3 acceptance criteria, a bug's reproduction (a fact, not a fork), a growth
      row's acceptance criteria, a route override, a merge or retire, epic membership. The engine's
      slot "the unit the user rules" is singular. One unit across kinds (the twin desk's part,
      direction, "how I'd know" — where "how I'd know" is the acceptance criterion), or one per
      kind?
    grounding: >-
      analysis-iterative.caller-slots; feat.delta-cards; feat.growth-door; feat.user-reserved;
      arch.focus-unit
  - id: C3
    class: C
    load_bearing: true
    angle: >-
      Weight at the lightest door. feature.md promises "a crisp demand converges in a single
      exchange" and "never imposes brainstorm ceremony on a clear ask". The engine wants two or more
      options with the cheaper one on every fork, a status line every turn, a blind list on costly
      forks, a defaults batch and the stop rule. For a one-line bug report: does inputs-in-hand's
      skip ("skips the forks, never the close") carry it, or does feature need a size or skip rule —
      and is the feature.md line itself superseded?
    grounding: >-
      plugins/mochiko/commands/feature.md:57-63; analysis-iterative.options-rule, .status-line,
      .blind-second-list, .inputs-in-hand; arch.focus-blind-list (skip on a single-row ask)
  - id: C4
    class: C
    load_bearing: true
    angle: >-
      Feature's next writer and independent leg. After the ruled set, who writes: the desk's
      card-producing seat, the PM seat (grooming proposals), or the dispatched implement run? The
      desk already runs a sound-loop review leg on every delta card, but nothing grades "meaning
      kept against what the user ruled" — the twin desk has tech-lead's sameness grade, implement
      has review-plan-artifacts' meaning-kept grade.
    grounding: >-
      feat.author-grader, feat.sound-loop-floor, feat.delta-cards; arch.rows-from-visit-file;
      impl.design-review-pair
  - id: C5
    class: C
    load_bearing: false
    angle: >-
      Feature's floor and fail. All four built callers carry a `<cmd>.thinking-stage` floor; feature
      carries none, and its only kind fail is feat.fail.no-verdict. Joining with the contract on
      implies a write-outside-the-ruled-set fail like arch.fail.write-outside-focus, enforcing a
      live local rule.
    grounding: >-
      feature preamble floors; arch.thinking-stage, spec.thinking-stage, impl.thinking-stage,
      brainstorm.thinking-stage; arch.fail.write-outside-focus; impl.fail.design-outside-ruled-set
  - id: C6
    class: C
    load_bearing: false
    angle: >-
      The PM verdict: fact or fork. The PM's extend-vs-mint verdict decides the door (extend stays
      at the desk, mint goes to specify), while route overrides are the user's. Under facts-first
      the verdict is a sourced fact the fork hangs on; under the user's reservation the door itself
      is a fork. Which one changes how a routing visit is asked.
    grounding: >-
      feat.pm-seat, feat.user-reserved, feat.growth-routes-to-specify;
      analysis-iterative.fact-before-fork
  - id: C7
    class: C
    load_bearing: false
    angle: >-
      Feature's blind list slot. Unfilled, it inherits the per-costly-fork default. Inputs (the
      demand, the map, the entries in territory), cadence (per visit, skipped on a single card?) and
      job need ruling; a blind list may add nothing to a bug intake.
    grounding: >-
      analysis-iterative.blind-second-list, .caller-slots; arch.focus-blind-list
  - id: C8
    class: C
    load_bearing: true
    angle: >-
      The twin desk as the cheaper shape. The architecture desk shares feature's spine — health
      first, converge the goal, close with a verdict, dispatch only — and already has a built focus
      stage (visit file, focus unit, own-screen confirm, fail). Mirroring it is the do-less road.
      Where they differ must be named: feature writes no capability truth (those writes go to
      specify), so its ruled set can govern only rows, cards and grooming rulings.
    grounding: >-
      feat.dm-health-first, feat.dm-converge-goal, feat.dm-close-verdict, feat.no-delivery-harness,
      feat.capability-writes-sacred; arch.focus-stage, arch.visit-file, arch.gate-focus-confirm
  - id: D1
    class: D
    load_bearing: true
    angle: >-
      Where feature's ruled set lives, with each road's touch set. The feature-desk home declares
      one deliverable, derivation.md (300 lines whole file), and its own description reads "One
      specify-desk sitting"; a new file (like the twin's visit.md) needs a migration-declared home;
      the card lives "on the entry", a capability entry file next to sacred writes; or the
      dispatched run's home. Homes are migration-declared and an undeclared name is refused.
    grounding: >-
      `mochiko-cli home .mochiko/features/desk/<date-slug>` (feature-desk: derivation.md only);
      feat.artifact-home; feat.delta-cards ("card on the entry"); arch.visit-file
  - id: D2
    class: D
    load_bearing: true
    angle: >-
      Lifetime and readers. A desk ruling may be read days later by the dispatched run, whose delta
      entry is "the desk-confirmed card". Is the card the ruled set (it travels as is), or does the
      run read a separate file? implement's inputs in hand list a visit file, the spec's slate and
      Intent, and a named brainstorm record — no feature ruled set.
    grounding: >-
      feat.dispatch-scope-split; impl.design-stage-inputs; impl.design-inputs
  - id: D3
    class: D
    load_bearing: false
    angle: >-
      Map integrity at close. The desk must leave no dangling entries or orphaned deltas; a ruled-set
      file that outlives an abandoned card, or a card dispatched without its set, is a new kind of
      orphan.
    grounding: >-
      feat.dm-map-integrity; feat.no-silent-map-mutations
  - id: E1
    class: E
    load_bearing: true
    angle: >-
      What setup's contract half would add. Setup already holds its writer through a floor
      ratification gate, a kind fail on an unclosed trace from intent to surfaces, a validate step
      by a fresh seat, and flagged-proposal acceptance. Contract on may be additive, duplicative, or
      a replacement for one of these; "off, with the reason" is the cheaper road to rule against.
    grounding: >-
      setup.gate-synthesis-ratification, setup.fail.unclosed-trace, setup.validate-seat-form,
      setup.gate-final-acceptance
  - id: E2
    class: E
    load_bearing: true
    angle: >-
      Flagged proposals against contract-inside. Setup's final acceptance goes "flagged proposal by
      flagged proposal" — writer additions shown after writing. Contract-inside sends anything that
      changes a ruling or that the user would notice back before it is written; only a small
      technical extra waits, and only where the caller grants that by name (implement's grant is the
      precedent). Turning it on moves setup from batch-after to halt-before, touching a floor gate.
    grounding: >-
      setup.gate-final-acceptance (floor); analysis-iterative.contract-inside;
      impl.design-seat-additions
  - id: E3
    class: E
    load_bearing: true
    angle: >-
      Synthesis or transcription. Setup says the synthesis is the lead's pen, so its stress-test is
      always a cold seat (and the two-message blind map is a floor). The engine says transcribing the
      user's rulings is not authoring and takes no sound loop. With contract on, is
      governance-intent.md a ruled set (transcription) or an authored synthesis (cold-reviewed)? The
      two framings disagree on what the cold review is for.
    grounding: >-
      setup.stress-test-cold-seat, setup.blind-map-dispatch (floor);
      analysis-iterative.lead-holds-conversation, .contract-pen
  - id: E4
    class: E
    load_bearing: true
    angle: >-
      A live conflict in setup's asking today. Setup works the catalog deck "card by card,
      recommend-then-arbitrate"; the engine gives a fork its own turn only when it is a real choice
      or costly to undo, batches the rest, and flags a three-yes streak. Setup already follows the
      engine's asking through its unchanged pointer, and engine precedence forbids loosening a
      mechanic — so either the deck already breaks the engine, or the deck is exempt by an unstated
      carve-out. Independent of the contract question.
    grounding: >-
      setup.interrogation-inline; analysis-iterative.engine-precedence, .own-turn-or-batch,
      .ratified-streak; plugins/mochiko/skills/authoring-constitution/references/catalog/
  - id: E5
    class: E
    load_bearing: true
    angle: >-
      Slots inherited silently by setup and intent. An unfilled slot takes the engine's default,
      "inherited by every caller with no per-caller carve-out". Neither setup's rules nor specify's
      intent rules fill any slot, so today both inherit the per-costly-fork blind list, provenance
      marks, the status line and the stall step-back. Whether those run, and whether each slot is
      ruled for them, is owed contract aside; the predecessor covered this only by a no-weakening
      check.
    grounding: >-
      analysis-iterative.caller-slots, .blind-second-list, .provenance-marks, .status-line;
      setup.interrogation-inline; spec.intent-stage-first; implement-brainstorm-led D4-shaped-for-six
      accepted risk
  - id: E6
    class: E
    load_bearing: false
    angle: >-
      Setup's engine tie is a must-pointer only. The four built callers each carry a floor
      `<cmd>.thinking-stage`; setup's floors list none. Whether setup gains the floor is a ruling.
    grounding: >-
      setup preamble floors; arch.thinking-stage, spec.thinking-stage, impl.thinking-stage,
      brainstorm.thinking-stage
  - id: E7
    class: E
    load_bearing: false
    angle: >-
      Two blind mechanisms in setup: the engine's per-costly-fork blind list and setup's blind-map
      stress-test seat. Overlap, cadence, and whether one stands for the other.
    grounding: >-
      setup.blind-map-dispatch; analysis-iterative.blind-second-list
  - id: E8
    class: E
    load_bearing: false
    angle: >-
      Setup's modes. In amend, governance-intent.md is the durable baseline carrying every past GI
      ruling: does a contract hold the writer to the whole set or the amend's delta? Brownfield
      conflict rulings (reality against declared intent) are rulings with no next writer.
    grounding: >-
      setup.synthesis-artifact ("the durable amend baseline"); setup.user-conflict-rulings;
      setup.user-mode-ruling
  - id: E9
    class: E
    load_bearing: false
    angle: >-
      Reach into this repo. mochiko amends its own governance through /mochiko:setup, so a change to
      setup changes this repo's amendment ceremony too; the ledger's amendment policy may cite
      setup's steps.
    grounding: >-
      CLAUDE.md Governance operations ("Amend via /mochiko:setup");
      .mochiko/memory/governance-ledger.md
  - id: F1
    class: F
    load_bearing: true
    angle: >-
      Who is intent's next writer. The confirmed intent lands verbatim (the lead transcribes). After
      it come the PM's capability frame (inside intent), the lead's own story stage (a second engine
      stage with its own contract), then the analyst (already held by the slate), the prototype and
      technical requirements. If the next writer is the lead's story stage, contract-on binds the
      lead to itself and is inert; if it is the analyst, two contracts bind one writer.
    grounding: >-
      spec.intent-synthesis-governs, spec.capability-frame-at-intent, spec.story-stage,
      spec.analyst-brief-and-pen, spec.lead-holds-story-stage
  - id: F2
    class: F
    load_bearing: true
    angle: >-
      Precedence between two ruled sets. Intent rules scope, out-of-scope, constraints, depth and
      UX-bearing; the slate rules the stories. Stories already win a conflict with the capability
      frame. Does the slate also beat an intent line, or does an intent out-of-scope line strike a
      slate row?
    grounding: >-
      spec.confirm-frame-post-stories, spec.slate-not-the-frame, spec.intent-stage-first
  - id: F3
    class: F
    load_bearing: true
    angle: >-
      Do-less for intent. Intent already binds: a kind fail if unconfirmed before authoring, and
      "governs the run". Contract-on adds only halt-before-writing-off-it and a meaning-kept grade —
      and specify's spec reviewer gives gap input, not a clearing verdict, so no seat would carry
      that grade. "Off, with the reason" must be ruled against.
    grounding: >-
      spec.fail.intent-unconfirmed, spec.intent-synthesis-governs; mochiko:review-specifications
      description ("gap-finding INPUT, not a clearing PASS/FAIL verdict")
  - id: F4
    class: F
    load_bearing: false
    angle: >-
      Intent's style against the engine's options rule. Intent runs "as probes, never a
      questionnaire"; the engine wants two real options on every fork but allows an open probe to an
      unsure user. Whether intent's agenda dimensions are forks or probes decides which rule bites.
    grounding: >-
      spec.intent-probe-discipline; analysis-iterative.options-rule, .question-form
  - id: G1
    class: G
    load_bearing: true
    angle: >-
      What a founder can rule at card confirm. Slicing (simple, split, merge, walking skeleton
      first) is seat craft; the founder-visible part is what works after each cycle and in which
      order. An engine unit could be "after cycle n you can do X", the cheaper option fewer, bigger
      cycles — or the gate stays a plain sign-off because the founder has nothing to fork.
    grounding: >-
      impl.gate-card-confirm (floor); impl.card-contents; mochiko:patterns-vertical-tdd
  - id: G2
    class: G
    load_bearing: true
    angle: >-
      Seeds to TEST cases. The slate's seed test (expanded into the Independent Test) and the design
      stage's "I'd know it works if…" seed both plausibly seed the **TEST:** cases qa-engineer
      writes. Under contract-inside a TEST case would have to keep its seed's meaning — the card
      review grades quality and buildability, not that. And an `open` seed carried as an open
      question: does it block a card, or ride as open?
    grounding: >-
      spec.slate-unit; impl.design-stage-unit; impl.seat-card-author-independence (qa-engineer
      authors TEST cases); impl.card-review-before-confirm
  - id: G3
    class: G
    load_bearing: true
    angle: >-
      Card confirm right after the design checkpoint. With gaps, the founder signs the landing table
      and then confirms cards: merge into one screen, or keep two. On the zero-gap path no stage
      runs, so card confirm is the founder's first ruling after run-open — the two paths may want
      different forms.
    grounding: >-
      impl.gate-design-checkpoint, impl.gate-card-confirm, impl.design-stage ("on the zero-gap path
      no stage runs")
  - id: G4
    class: G
    load_bearing: false
    angle: >-
      Card authors as next writers. A design-class seat writes cards, never the builder; qa-engineer
      writes TEST cases within its slicing. A ruled slicing set would bind both; the existing fail
      guards independence, not writing outside a ruled set.
    grounding: >-
      impl.seat-card-author-independence; impl.fail.card-independence; impl.cards-template
  - id: H1
    class: H
    load_bearing: true
    angle: >-
      Batch against one fork per turn. implement lands reserved questions as one batch at the cycle
      checkpoint; the engine puts one fork per turn and batches only defaults on one screen. Five
      real escalations become five turns while seats wait — or the batch becomes a defaults batch
      with real forks pulled out. Which rule bends is either an engine slot (B4) or implement's
      cadence rule.
    grounding: >-
      impl.escalation-batching; analysis-iterative.one-fork-per-turn, .own-turn-or-batch
  - id: H2
    class: H
    load_bearing: true
    angle: >-
      Escalations are not one kind. An infeasible card is a business-level scope fork; a requirement
      ambiguity is often a fact only the user holds (a labelled fact question under facts-first); a
      producer's judgment call is a fork; an Important-or-above finding is a disposition. Each may
      want a different treatment; a disposition may be best plain.
    grounding: >-
      impl.infeasible-card-escalation, impl.ambiguity-escalation, impl.finding-severity-routing;
      analysis-iterative.fact-before-fork
  - id: H3
    class: H
    load_bearing: true
    angle: >-
      Mid-cycle interrupts. A build-blocking question interrupts now and must never be sat on; a full
      fork (fact found, options, mock) adds latency to a blocked builder. The design re-fire already
      runs a short stage mid-build. Is a blocking escalation a mini-stage, or one plain question?
    grounding: >-
      impl.escalation-batching ("Never sit on a build-blocking one"); impl.midrun-refire
  - id: H4
    class: H
    load_bearing: false
    angle: >-
      Rulings reaching the builder. With a contract on, a ruling after the pen passes reaches the
      writer as the exact text to land. Without one, checkpoint rulings travel by the lead's
      paraphrase into a builder's brief.
    grounding: >-
      analysis-iterative.contract-pen
  - id: H5
    class: H
    load_bearing: false
    angle: >-
      Streaks across a long run. Ten checkpoints of "as recommended" is the passive acceptance the
      problem names. Does the streak rule count per checkpoint or across the run?
    grounding: >-
      analysis-iterative.ratified-streak
  - id: H6
    class: H
    load_bearing: false
    angle: >-
      Epic runs. Checkpoint batches across members, and the epic member halt, interleave several
      members' questions in one conversation.
    grounding: >-
      impl.epic-member-halt; impl.epic-card-sequence
  - id: I1
    class: I
    load_bearing: true
    angle: >-
      The gap-finder reading the ruled set against its blindness. The seeds in design-stage.md are
      promised behaviour, so adding the file to the finder's first message looks right. But if
      qa-engineer wrote TEST cases from the same seeds, the finder inherits the test author's framing
      — the shared assumption gap-finding exists to escape. Today the fence excludes TEST cases and
      every `proposed` entry; design-stage.md is not listed.
    grounding: >-
      impl.gap-finding-blind-dispatch; impl.seat-gap-finder-blind; mochiko:testing-gap-finding
  - id: I2
    class: I
    load_bearing: true
    angle: >-
      Finding dispositions. Fix-now, backlog or accepted-as-designed, a disputed kind, and gap-rework
      bound exhaustion are the user's rulings. Engine form (options, backlog as the cheaper road) or
      a plain list? "Accepted as designed" also claims the ruled set covers the behaviour — checked
      against design-stage.md, or not?
    grounding: >-
      impl.gates-fold; impl.finding-kinds; impl.finding-kind-disputed; impl.gap-rework-bound
  - id: I3
    class: I
    load_bearing: true
    angle: >-
      Acceptance's form. Accept, amend or reject over a whole built run. D8's pattern suggests a
      landing table over the chain — each slate seed and design seed beside the evidence that it
      holds — or it stays plain. "Amend" is undefined in engine terms: what reopens — a fork in which
      stage, a new cycle, a new run? The built-vs-signed diff already exists at landing and may be
      the table's half.
    grounding: >-
      impl.gate-final-acceptance (floor); implement-brainstorm-led D8-checkpoint-shows-landing;
      impl.landing-verifier-folds
  - id: I4
    class: I
    load_bearing: false
    angle: >-
      design-first-write at acceptance is already a per-item signature (the proposed system-part
      write and each critique-advisory finding); on the engine or plain.
    grounding: >-
      impl.design-first-write
  - id: J1
    class: J
    load_bearing: true
    angle: >-
      The founder's net gate count over a full chain: setup (interrogation, ratification,
      acceptance), a feature visit, specify (intent, slate confirm, frame confirm, selection,
      acceptance), the desk (focus confirm), implement (run-open, stage close, design checkpoint,
      card confirm, n cycle checkpoints, acceptance). Moving more moments onto the engine adds turns;
      gate fatigue drives the rubber-stamping the engine exists to stop. The net count, with merges
      or retirements, is a ruling.
    grounding: >-
      moments blocks of `mochiko-cli rules setup|feature|specify|architecture|implement`
  - id: J2
    class: J
    load_bearing: true
    angle: >-
      Carry-forward. A fork ruled upstream must arrive as a fact. Specify's story stage takes a named
      brainstorm record or stories in story form — not a desk sitting; implement's design stage takes
      a visit file, slate and Intent, a brainstorm record — not a feature ruled set; implement sends
      an outside need to "a feature-desk card", which the desk would then re-ask. Every new ruled set
      needs its readers declared, or the founder is asked twice.
    grounding: >-
      spec.stage-inputs-in-hand; impl.design-stage-inputs; impl.design-stage-agenda ("a feature-desk
      card"); feat.dispatch-scope-split
  - id: J3
    class: J
    load_bearing: true
    angle: >-
      Altitude. Card slicing, cycle escalations and finding dispositions are technical. implement's
      design stage puts pieces as "plain consequences and a worked example with real values, never a
      schema"; whether that rule extends to every implement moment that joins. The user has asked
      for simpler questions mid-session before.
    grounding: >-
      impl.design-stage-unit; .mochiko/brainstorms/architecture-brainstorm-interview/record.md:1000
  - id: J4
    class: J
    load_bearing: false
    angle: >-
      Who holds the talk in a long run. The engine has the lead hold it inline; implement's lead also
      orchestrates many seats over a run that may stop and resume. Many checkpoint stages in the main
      pane load its context.
    grounding: >-
      analysis-iterative.lead-holds-conversation; impl.gate-design-checkpoint ("may stop at the
      checkpoint and resume the build later")
  - id: K1
    class: K
    load_bearing: true
    angle: >-
      One build's size. Four commands (feature, setup, specify, implement), possibly engine slots, a
      new home and template, kind fails, contract cases. "Decisions one build can carry" needs a size
      test and a cut line — what ships in this build and what waits.
    grounding: >-
      frame destination line; CHANGELOG.md 0.120.0; implement-brainstorm-led D3-four-move-now
  - id: K2
    class: K
    load_bearing: true
    angle: >-
      Supersessions owed. D4-shaped-for-six's changed-at-review clause and frame change 3 ("their own
      rules and content are not edited") for setup and intent; feature.md's "never imposes brainstorm
      ceremony"; setup.interrogation-inline's card-by-card; setup.gate-final-acceptance (floor) if
      flagged proposals move; impl.escalation-batching. Each leaves only by recorded ruling, with
      strip entries per primitive.
    grounding: >-
      .mochiko/brainstorms/implement-brainstorm-led/record.md:666-711; CLAUDE.md landing ritual;
      .mochiko/strips/README.md
  - id: K3
    class: K
    load_bearing: true
    angle: >-
      Neighbours and order. The three-command convergence session (boilerplate outside the stage)
      touches specify; the three pending ai-fileops first runs would test a moving target if setup,
      intent or implement change first; the engine-desk residual items touch the same files. The
      build's order against each is a ruling.
    grounding: >-
      BACKLOG.md:98-108, :36-42, :78-97, :43-64
  - id: K4
    class: K
    load_bearing: false
    angle: >-
      Tests. Each new floor or fail and each changed gate needs contract-suite cases; a
      conversational stage needs a scripted founder run headless, and setup's interrogation is long.
    grounding: >-
      CLAUDE.md quality gates (gate 6)
  - id: K5
    class: K
    load_bearing: false
    angle: >-
      The engine skill's own text. Its description lists the callers and its Overview says setup and
      intent run "the contract off"; any change touches the 1,536-character description budget and
      Overview lines.
    grounding: >-
      plugins/mochiko/skills/analysis-iterative/SKILL.md description and Overview; CLAUDE.md axis 3
---

## Notes of note

- Fence: no file under the session directory was opened; `index.md` was not opened. One `find`
  over `.mochiko/brainstorms` walked the session directory's listing with that path excluded from
  output; nothing from it was printed or read.
- Grounding read: the rendered rules of `analysis-iterative`, `feature`, `setup`, `specify`,
  `implement`, and the architecture desk's focus rules (plugin 0.120.0); `feature.md`; BACKLOG
  lines 20-115 and 668-676; the predecessor record's facts F28-F29, Q4, frame changes 2-3, D4,
  open questions and untested bets; the predecessor's angle map for shape; the feature-desk home
  and the design-stage template.
- Tiering: one haiku `Explore` locate spawn ended with no report (the seat-transport finding
  again); the locates ran on the seat tier.
- Load-bearing means a gap there would plausibly or likely have changed a ruling.
