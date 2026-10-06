---
report: review
round: 1
pass: cold
pairing: solo
lenses: decision-quality + record-integrity
reviewer: cold-reviewer — mochiko:devils-advocate seat, mochiko:review-brainstorm, opus
reviewed: .mochiko/brainstorms/brainstorm-target-state/record.md (frozen 2026-10-04, message 2; 799 lines)
blind_map: .mochiko/brainstorms/brainstorm-target-state/reports/angle-map.md (63 angles, 25 load-bearing; record_contact none at build)
recommended_status: needs-revision
status_criterion: >-
  No Critical survivor. Seven Important survivors stand. Five are builder questions the record
  leaves open (S1-S5), even though its Destination promises a build "without asking the user a
  design question", so the record fails the builder test it adopts itself (D16). One record-fitness
  item is partial (S14), and any unchecked fitness item blocks ready.
tally: 26 raised, 16 survived (0 Critical, 7 Important, 9 Minor; 4 of them coverage)
routing_disclosure: >-
  A third Explore haiku dispatch, for the index and record counts, again ended without a
  SubagentHandback report. Every fact check below was run first-hand on the seat tier with grep,
  sed and awk.
fact_verification:
  - F1 holds. The render has 5+5+7+8+1+4 = 30 rules, 8 floors and 4 moments. No rule states when questioning ends.
  - F2 holds. analysis-iterative SKILL.md is 66 lines. "The adaptive flow above" is at line 52. Output points at SYNTHESIS.md. The KEPT dogfood quote is at strips/analysis-iterative.md:44-50.
  - F3 holds. final-verdict.md:63-64 and :75-76 match the quotes word for word.
  - F4 holds. Each of the three index quotes occurs once. A grep of cold-review-gap-challenge/record.md for front, in-session, agenda or prevent finds no front-map road.
  - F5 is partly wrong. The raised-to-survivors pairs hold, and "verify round 1 NOT CLEAN" occurs 23 times (claim "at least twenty"). The string critical-gaps occurs 22 times in the index; only the backticked form occurs 7 times. See S9.
  - F6 holds. setup-product-agnostic record lines 528-554 and 402-417 match the answers, the six Confident marks, S2/S7/S15 and the 3 coverage survivors, so 13 of 16 holds.
  - F7 is partly over-read. Q3 and Q7 at hook-enforcement-field-review/record.md:677-692 hold, but the facts were already in that record (F11, F12, F16); see S11. Orchestrator G1 and G3-G5 hold (record:144-148). The "eleven" count is 10 records excluding this one; see S8.
  - F8 holds. 23,283 lines; 1,536 and 1,375 are the largest; 558/545/244. The index has 78 entries today, 77 without this session.
  - F9 holds at agent-skills@1401c8b. Each named skill has 1 eval in evals/cases. evals/skill-impact.md (the rejected-changes ledger) is a header-only table.
  - F10 holds at pocock-skills@d81f3a1 (docs/engineering/wayfinder.md:69-80), except the item "an agent choosing a prototype variant for the user". It is not found in wayfinder.md or prototype.md; see S10.
  - F12 holds. setup.interrogation-inline names mochiko:analysis-iterative. The specify render has 0 references. Prose references are in review-specifications, the router, and authoring-constitution/references/INTERROGATION-AGENDA.md.
  - F13 holds against the Agent tool contract ("continue a previously spawned agent with its context intact").
fitness:
  self_contained: pass. The topic, F1-F13, the stage table, D-cards with rationale, rejected roads and risk, and the Q-trail read without the conversation.
  decisions_attackable: pass. Every D-card and D12-D18 carries a concrete statement and evidence.
  decision_trail_present: pass. There is a how-it-was-decided line per card, Q1-Q14 with answers verbatim, and three streak notes.
  confidence_marks_honest: pass. The ratified decisions say "ratified" (D1, D3, D5, D6, D8, D10, D11, D12-D18). D9 and D20 are Contested against the lead's lean. OQ1 is Assumed.
  rejected_roads_recorded: partial. D12-D18 name no rejected road and give no reason it lost, except D15 (predict-three) and D14 (clickable prototype). See S14.
  honest_about_the_open: pass. OQ1-OQ3 are listed, plus "Out of scope and not done" with the lead's unprompted drops named.
  provenance_stated: pass. The status header and Review section name the seat, tier, dispatch and attestation.
survivors:
  - id: S1
    severity: Important
    kind: coverage (map B1, load-bearing; the record never mentions it)
    decisions: D19 Reach (what it "settles"), D12 Order, D8 Bare yes, D15 Stop rule, D17 Fix and verify, constraint 2
    evidence: >-
      command-architecture-realignment D1 is Contested and user-ruled (record lines 70-79). It says
      "Stage/seat choreography, default pipelines, recovery tables, and procedural detail are
      deleted, not relocated", and its rationale is "the volume of encoded detail is itself the
      defect". D6 adds "no stage or gate vocabulary". The DECISIONS.md row 108 lists "counted
      bounds" among what v8 dropped from commands. The target puts a default pipeline (D12) and
      three counted bounds (three yeses in D8, three questions in D15, a second failure in D17)
      into brainstorm's own rules (D19). A grep of the record for realign, choreograph, v8 and
      counted finds nothing. D19's "what this settles" section weighs only the v0.63.0 benchmark,
      which governed skill bodies, and misses the ruling that governs command rules.
    failure_scenario: >-
      The build writes the counts into brainstorm's rules. At landing, the KM ritual needs a
      supersession annotation on row 108 that no ruling backs, or the audit's
      preserved-responsibilities check reads the counts as a regression against a standing
      user ruling. Either way, the user has reversed their own Contested ruling without being shown it.
    resolution: >-
      Put it to the user as a candidate topic (coverage routing). Option one supersedes D1/D6 in
      part for brainstorm and records which parts. Option two states the counts as lead guardrails
      ("say so when a run of yeses builds up") with no fixed number. Either way, list the prior
      rulings this record supersedes in part, so the landing can annotate DECISIONS.md.
  - id: S2
    severity: Important
    kind: inconsistency (sequencing) and missing dimension
    decisions: D4 Decision map, D10 Cold review, D11 Size
    evidence: >-
      D10: right after the frame hardens, a seat draws the blind map, and the lead folds its
      load-bearing angles into the decision map. D4: the map arrives with the first question.
      D11: the size is proposed "when the map is first drawn", ruled by the user, and Small means
      "no blind map". So the blind map has to exist before the map is shown, but whether to draw
      it depends on a size ruled only when the map is shown. D4 also says "never a gate", while
      D11's size ruling needs a user turn. Nothing prices the wait while the seat builds its map
      before the first question. This seat's own 63-angle map took a long tool-call run.
    failure_scenario: >-
      The builder must pick one of three paths, each with a different user experience. It can
      spawn the map before the size is ruled, which wastes a seat on Small. It can add a size turn
      before the map, which is a gate. Or it can show the map first and fold angles in later, which
      breaks D10's "folded into the decision map" at the first display. That is a design question
      the Destination says no builder should have to ask.
    resolution: >-
      Put to the user as one fork. Recommended: the size rides in the frame card's turn (a sixth
      line), the blind map spawns at hardening for Standard and Too big only, and the first
      question goes out while the map is drawn. Its angles fold in at the next status line, under
      D5's "other askable questions proceed".
  - id: S3
    severity: Important
    kind: inconsistency (decision against carried-in constraint and floor)
    decisions: D11 Size (Small path), constraint 1
    evidence: >-
      Constraint 1, confirmed by the user, says "The cold review's core stays — blind dispatch,
      the hunt classes, the user's pen". D11 Small says "no blind map ... one end read or the
      user's waiver". review-brainstorm.blind-map-before-record-contact is class floor ("Phase 0
      blind angle map ... before record contact"). Strips v0.100.0 record its protection
      transfer: the blind-map floor is protected since v0.60.0 and v0.88.0. An end read with no map
      breaks the floor and constraint 1. Only a waiver is allowed to skip the review.
    failure_scenario: >-
      A Small session ships with a cold read that has no coverage hunt. Its survivors include
      none of the gaps that F5 shows arrive in every reviewed session. The build has to strip a
      protected floor that no ruling names.
    resolution: >-
      The user rules one of two things. Small keeps a blind map at the end, cheap and as today.
      Or Small drops the floor by recorded supersession, with constraint 1 amended to say so.
  - id: S4
    severity: Important
    kind: missing dimension, and the record's own D6 test misapplied
    decisions: D16 end-reviewer checks, D9 Record card, D6 Asking
    evidence: >-
      D16 says "A mechanical check that every card has its fixed parts runs before the seat
      reads", and D9 fixes the card's parts. The record never says what runs the check. If it is
      mochiko-cli or a hook, the record.md home must change: mochiko-cli home reads "record.md ·
      no template · no size bound — ... the cold review grades it". That is a migration and a
      conformance-gate question under the kernel-class line. If it is a seat, it is not
      mechanical. By D6's own test (a real choice, or costly to undo), D16 is a fork. It was
      batched as a default (shown as B5) and confirmed by "confirmed, lets do the open questions".
    failure_scenario: >-
      The builder must ask whether record.md gains a template schema and a hook check, or whether
      the check is a checklist line for the seat. The two differ in kernel-class admission,
      migration count and contract-suite re-freeze.
    resolution: >-
      Put to the user as a fork in the D13 form. Options: a template schema for record.md cards
      checked by mochiko-cli (advisory or gating), or a seat checklist item with no tooling. The
      cheaper shape is the checklist.
  - id: S5
    severity: Important
    kind: inconsistency left for the builder
    decisions: D6 Asking, D13 Question form, D19 Reach, D21 Shared-skill repairs
    evidence: >-
      brainstorm.lead-inline-questioning points at mochiko:analysis-iterative, and its text says
      "one question per turn, format adapted to the user's state". The skill's Common Mistakes
      table (SKILL.md:40-42) says "Always using structured options ... Open probes help them
      discover what they think" and "Multiple questions per turn ... One question per turn —
      always". D6 and D17 batch defaults into one turn. D13 fixes one option form for every fork.
      D19 keeps the skill unchanged so that setup is not touched, and D21 repairs only two other
      sentences.
    failure_scenario: >-
      After the build, brainstorm's rule sends the lead to a skill that forbids the batching and
      fixed form brainstorm's own rules require. The lead gets two contradicting instructions per
      question. The builder either edits the shared table rows, which touches setup (against D19),
      or writes an override clause no decision rules.
    resolution: >-
      The user rules one of two options. Brainstorm's rule states that its asking rules take
      precedence over the skill's one-per-turn and options rows. Or D21 widens to scope those two
      rows to the general shape and leave setup's behaviour alone. Either way, the reworded text of
      brainstorm.lead-inline-questioning, a v0.48.0 kept-deliberately line, gets a recorded supersession.
  - id: S6
    severity: Important
    kind: coverage (map B5, load-bearing; fact-checker history absent)
    decisions: D5 Facts, the Ground facts section
    evidence: >-
      The record never mentions the fact-checker seat (2026-07-05). That seat's map landed
      verbatim because "the first completed run's headline finding was an over-claim living in the
      lead's paraphrase of the map" (strips/brainstorm.md, v0.48.0 verbatim block). D5 rules that
      the lead finds facts and attaches them, so facts stay lead-paraphrased. This record shows the
      failure that precedent warned of: S8 (D7's "eleven records carry a review finding" against 7
      genuine), S9 (a count off by 15) and S11 (evidence read against its source).
      review-brainstorm.verify-load-bearing-claims still binds "the record's fact-checker map",
      which no target stage produces.
    failure_scenario: >-
      A decision's premise is stated in the lead's paraphrase of a grep. The end reviewer has to
      re-derive it from the files, the cost the fact-checker map was built to remove. An
      unverified over-claim, like S8, reaches the user as the rationale they rule on.
    resolution: >-
      Candidate topic. The user rules whether D5's facts land as quoted source lines (path:line plus
      the matched text, in place of a lead paraphrase), and whether the review binding's
      "fact-checker map" wording is renamed to the record's fact list.
  - id: S7
    severity: Important
    kind: rejected-road steelman (premise of a user ruling misstated); reserved to the user
    decisions: D10 Cold review (Q10b ruling)
    evidence: >-
      D10 rejects "Always a fresh seat for the end read" as "one more seat start for no gain in
      independence". In Q10, the lead told the user the resumed seat is "the same shape as today's
      reviewer with a pause in the middle". It is not the same shape. Today's blind map never
      enters the room. Under D10 the seat's angles are folded into the decision map and shape the
      decisions, and at the end the same seat "checks that each folded angle was answered in
      depth". The grader then judges how the lead handled its own output. A fresh seat reading the
      saved map has no such stake. That is the independence option 2 buys, against the seat's
      deeper grasp of its own map.
    failure_scenario: >-
      The resumed seat rates thinly covered angles as answered because the decision map adopted
      its framing. The thin-coverage risk D10 accepts then lands on a grader with a stake in that
      framing.
    resolution: >-
      Re-put Q10b to the user with the trade stated accurately: option 1 is deeper context with a
      stake in the folded angles, option 2 is no stake for one seat start. Or keep D10 and correct
      its rejected-road reason and accepted risk to name the trade.
  - id: S8
    severity: Minor
    kind: unverifiable claim and over-claim (lead paraphrase)
    decisions: D7 Asking (blind second list), F7
    evidence: >-
      D7 says "Eleven records carry a review finding that a road was never put to the user (F7)".
      Grepping "never (put|dealt|weighed|considered)" over the other sessions' record.md files
      gives 10 files; 11 counts this record. Three are not such findings:
      cold-review-gap-challenge (a hunt-class definition), model-tiered-seats ("never put the
      discipline surface ... on a weak model"), and security-depth-scoping ("never dealt the card",
      a shelf mechanic). Seven are genuine: architect-role, domain-dependency-allowlist,
      lead-owned-process-flexibility R8, orchestrator G6, plan-stage-utility S1,
      setup-product-agnostic S2/S7/S15, validator-worktree-isolation S10.
    failure_scenario: D7's premise still stands on 7 records plus F6, but the user ruled level 2 on an inflated count.
    resolution: Lead repair. F7 and D7 say "7 of the other records carry such a finding (grep of 10, 3 false positives named)".
  - id: S9
    severity: Minor
    kind: fact error
    decisions: F5
    evidence: >-
      F5 says the string critical-gaps "occurs seven times". It occurs 22 times in
      .mochiko/brainstorms/index.md (22 lines). Only the backticked form occurs 7 times.
    failure_scenario: A reader takes the ratio of critical-gaps to ready (7:3) as the review base rate. The real figure is much higher.
    resolution: Lead repair. Write "22 times (7 as a backticked status)".
  - id: S10
    severity: Minor
    kind: unverifiable external claim (EXTERNAL-CLAIMS floor)
    decisions: F10
    evidence: >-
      F10 lists "an agent choosing a prototype variant for the user" among wayfinder.md's field
      failures. At pocock-skills@d81f3a1, neither docs/engineering/wayfinder.md nor
      docs/engineering/prototype.md carries it. The closest text is wayfinder.md:44, "an agent that
      answers its own grilling questions has broken it". The claim is not load-bearing, and F10
      carries no disclosure line.
    failure_scenario: A later reader cites a field failure that does not exist at the pinned commit.
    resolution: Lead re-reads the source (source re-read clause). Replace it with the wayfinder.md:44 text, or strike it.
  - id: S11
    severity: Minor
    kind: evidence misread
    decisions: D5 Facts (rationale), F7
    evidence: >-
      D5 cites hook-enforcement-field-review Q3 and Q7 as "the cost of putting a decision ahead of
      its fact". At record:677-692 both were answered "from F11/F12" and "from F16", which were
      facts already in that record. The user's question was a premise challenge: why is evidence
      generated at all, and why is a run log required. That is an unput cheaper road, which is
      D7's ground, not D5's.
    failure_scenario: D5's rationale overstates its evidence. Only the orchestrator half (G1, G3-G5) shows facts nobody checked.
    resolution: Lead repair. D5 cites the orchestrator survivors only, and the hook-enforcement case moves to D7's rationale.
  - id: S12
    severity: Minor
    kind: citation gap
    decisions: D10 Cold review (rationale)
    evidence: >-
      D10 says "The last five reviewed sessions each had three to eight coverage gaps (F5)". F5
      gives only "22→15 plus 8 coverage". The claim is true per the index (3, 6, 4, 3, 8 coverage
      across setup-product-agnostic, delta-files, hook-enforcement, author-grader-consolidation and
      orchestrator-model-selection), but F5 does not carry it.
    failure_scenario: The end reviewer cannot check D10's premise from the record alone.
    resolution: Lead repair. Add the five counts to F5.
  - id: S13
    severity: Minor
    kind: inconsistency (record wording against later rulings)
    decisions: Destination, OQ1, the Q12 trail closure
    evidence: >-
      The Destination says "a later build session can plan the rebuild", but Q12's answer is "i plan
      to build in this same session". OQ1 calls the paused seat "a fact to test, not a ruling",
      while the Q-trail closes it "without a question" as the harness's stated behaviour (F13).
    failure_scenario: The builder reads the paused seat as untested and owed a probe, or as closed. These lead to different wave plans.
    resolution: Lead repair. Reword the Destination to "the build, in this session", and give OQ1 one status for the paused seat.
  - id: S14
    severity: Minor
    kind: record fitness (rejected roads)
    decisions: D12 Order, D13 Question form, D17 Fix and verify, D18 Accept
    evidence: >-
      The batch decisions name no rejected road and give no reason one lost. The stage-table
      columns hold the alternatives (lead judgment, interview, wayfinder), but no reason is given.
      D9's own target card requires rejected roads.
    failure_scenario: A future reader re-derives why wayfinder's "each question says what it unblocks" was kept but its frontier-in-one-round was not, and so on.
    resolution: Lead repair. One rejected-road line per batch decision, drawn from the table's columns.
  - id: S15
    severity: Minor
    kind: coverage (map E4/H1)
    decisions: D10 Cold review, D20 and OQ3, landing
    evidence: >-
      BACKLOG.md:544 "First-live-run watch, both carriers" asks whether the blind map produces
      material coverage findings. F5 answers that, and D10 moves the mechanism it watches.
      Separately, BACKLOG.md:270-290 records a brainstorm plan-only eval kit, built and audited
      2026-09-19, that has never run a grid. OQ3 limits the measure to D7's blind list, while the
      other new pieces are equally untested. The record names neither item.
    failure_scenario: The watch item stays open against a moved mechanism. The one existing instrument for judging the rebuild goes unused.
    resolution: Candidate topic, or rule inline. The landing closes or annotates BACKLOG:544 with F5, and OQ3 widens to the whole target, with the eval kit's pre/post grid named as one candidate measure.
  - id: S16
    severity: Minor
    kind: coverage (builder question), out of scope but not ruled
    decisions: D10 Cold review, Out of scope ("a review pair under the front map")
    evidence: >-
      "How two reviewers would share a front map was not designed." brainstorm.staffing-latitude
      and brainstorm.pair-maps-independent ("both seats build their Phase 0 angle maps
      independently") stay live, and they conflict with D10's "No second map is drawn". The lead
      listed the item as out of scope. The user never ruled it.
    failure_scenario: The lead seats a pair under the new flow and gets two maps, which D10 rules out, or none, which breaks the pair rule.
    resolution: Rule inline. The pair path is closed for the front-map flow (solo only), or left to a later session with the pair rules marked inactive.
dropped:
  - A7, whose session the flow serves. Constraint 3 is the user's ruling, and a dismissed angle is a ruling.
  - B3, reversing the benchmark. D19's "what this settles" section addresses it on the record.
  - B4, the standing challenger. D7's history check and the withdrawal of doubt-in-flight address it.
  - The index entry's stale artifacts line (F1-F11, Q1 put). The status agrees (open), and brainstorm.index-bookkeeping rewrites the entry at acceptance. Commentary.
  - The thin coverage of folded angles. D10 already accepts this risk.
  - D18's pre-review screen repeats D15/D6's batch. No failure scenario. Commentary.
  - The name "decision map", which wayfinder.md itself dropped as jargon. Commentary only, since D13 governs question words, not internal names.
  - The upstream advice that single-session work is cheaper with grilling than with a map. D4's map is a light, ungated list, and the 27-ticket trap is cited and avoided.
  - Licensing (MIT notices). Nothing verbatim is adopted.
  - F9's "asked as a batch". idea-refine collects 3-5 questions through AskUserQuestion, so the reading is fair.
verify_round_1:
  reviewed: record.md as re-frozen 2026-10-04 at the verify-pass opener (1,060 lines)
  scope: >-
    The 16 folds plus D22 and D23 (born at review). D22 and D23 get the bounded grade only:
    internal consistency and record fitness. New surface is limited to contradictions the folds
    themselves introduced. No fresh cold read and no coverage hunt.
  result: >-
    18 graded — 13 CLEAN, 5 NOT CLEAN (S1, S2, S5, S15, D23); 6 fold-introduced defects (V1-V6),
    all non-blocking; V3 needs the user's word.
  reviewer_correction: >-
    S10 was this seat's error. The quote exists at pocock-skills@d81f3a1
    docs/engineering/wayfinder.md:75: "an agent has been reported building three UI variations,
    choosing one itself, and closing the ticket". The seat's grep printed that line cut off at 300
    characters, before the match. So the cold pass's F10 fact_verification line above is wrong, and
    F10 held all along. The lead's fold (claim unchanged, source line added) is correct.
  folds:
    - >-
      S1, NOT CLEAN (non-blocking; V1). D22 itself is sound (see D22). The fold did not reach D12 or
      the stage table, which still state the clause D22 turns into the lead's judgment.
    - >-
      S2, NOT CLEAN (non-blocking; V2, V3). D2 (sixth line), D10 ("The first question goes out
      without waiting for it"), D11 ("on the frame card ... in the reply that confirms the frame")
      and the Frame, Cold review and Size cells all landed, and the loop is gone. Two new seams are
      listed under V2 and V3.
    - >-
      S3, CLEAN. D11 Small reads "no front map and no blind second lists; the review is today's, the
      review seat drawing its blind map at the end before it sees the record", which agrees with
      constraint 1 and the floor.
    - >-
      S4, CLEAN. D16 reads "The seat's first step is a checklist that every card has its fixed
      parts". The note rejects the tool check, and the Cold review cell matches.
    - >-
      S5, NOT CLEAN (non-blocking; V4). The precedence clause landed in the D19 note and the D13
      note ("does not forbid an open question"). D19's own "What this settles" paragraph still says
      the cut "is not reversed and needs no replacement".
    - >-
      S6, CLEAN. The D5 note reads "a fact enters the record as a quoted source line with its file
      and line number". v2.2 F9 and D2 are cited accurately (brainstorm-v2-2-revision/record.md:25,
      :36). Nit, no action: the resolution's second limb (renaming review-brainstorm's
      "fact-checker map" wording) is not carried. It is build-level, and the binding's "or the files"
      still works.
    - >-
      S7, CLEAN. The D10 rejected road now reads "a reader with no stake in the angles and a second
      pair of eyes", and Q21 "keep 1" is on an accurate premise. Nit, no action: the rationale's last
      sentence stands, with a read-with-correction pointer.
    - >-
      S8, CLEAN. F7 reads "Ten other records match ... seven are review findings ... three are not",
      and D7 reads "Seven records" with a pointer to S8.
    - >-
      S9, CLEAN. F5 reads "As a backticked status, critical-gaps appears seven times ... the bare
      string occurs 22 times". Nit, no action: "bare" can be read as un-backticked (15). 22 is all
      occurrences.
    - >-
      S10, CLEAN. F10 now carries docs/engineering/wayfinder.md:75 with the quote (see
      reviewer_correction).
    - >-
      S11, CLEAN. F7 reads "the user challenging a question's premise, not a missing fact". D5's
      rationale cites only the orchestrator survivors, plus the correction.
    - >-
      S12, CLEAN. F5 reads "the counts are 3, 6, 4, 3 and 8 (added at review, S12)", so D10's "three
      to eight" is now carried.
    - >-
      S13, CLEAN. The Destination reads "the build — which the user ruled happens in this same
      session (Q12...)". OQ1 says the paused seat "proved to be a fact and is settled by F13".
    - >-
      S14, CLEAN. The "What was rejected, per decision" paragraph names a road or "no alternative
      considered" for each of D12-D18.
    - >-
      S15, NOT CLEAN (non-blocking; V5). "For the landing" names the BACKLOG watch and the eval kit,
      and OQ3 is closed into D23. D20's accepted risk still says the measure "has not been decided
      (OQ3)".
    - >-
      S16, CLEAN. The D10 note reads "a pair stays possible; each seat draws its own front map ...
      'No second map' in the statement means none at the end". The Out of scope bullet points to it.
    - >-
      D22, CLEAN on the bounded grade. It carries statement, rationale, rejected roads, risk and
      how-decided. It cites DECISIONS.md:108 and author-grader-consolidation D6 ("a second FAIL stops
      and goes to the user") accurately, and agrees with "For the landing". Nit, no action: "Told to
      the user with the next question" (the D12 consequence) has no trace in Q16's trail entry.
    - >-
      D23, NOT CLEAN (non-blocking; V6). Its content is consistent: 15-25 survivors, 3-8 coverage and
      "at least twenty" not-clean all match F5. Rejected road "None was put to the user" is honest.
      But the card ends with D21's displaced "How D21 was decided" paragraph (record:665-667).
  new_surface:
    - >-
      V1, non-blocking, lead repair. D22 says "a shape-defining fork goes first ... is the lead's
      judgment, not a rule". The D12 bullet (record:687) still reads "a shape-defining fork goes
      first because it clears the most fog", and the Order cell (record:152) reads "Dependency
      order; a shape-defining fork first". Repair: a changed-at-review line on D12 and the table
      cell reworded.
    - >-
      V2, non-blocking, lead repair. D3's accepted risk (record:258) reads "The user must read and
      react to five lines", but the card is now six lines (D2).
    - >-
      V3, non-blocking, needs the user's word. S2's "nothing waits for it" lets D15's "Deciding is
      finished when the map is empty" fire before a started blind map has landed. The angles would
      then arrive after freeze as end-of-session coverage findings, the failure D10 exists to
      prevent. Repair: add to D15's empty-map conditions "and any blind map started has landed, its
      angles folded or listed as dropped". It touches a user-ruled decision, so it can ride D18's
      pre-review screen.
    - >-
      V4, non-blocking, lead repair. D19's "What this settles" (record:553-557) says the v0.63.0 cut
      "is not reversed and needs no replacement". The S5 note and "For the landing" (record:753-754)
      supersede the cut's kept Common Mistakes rows for brainstorm. Also, the S5 note cites F2 for
      "kept deliberately", but F2 does not carry that fact; strips/analysis-iterative.md v0.63.0
      "Kept deliberately" does. Repair: reword the paragraph to "not reversed for skill bodies; its
      keep-set superseded in part for brainstorm (S5)" and cite the strip.
    - >-
      V5, non-blocking, lead repair. D20's accepted risk (record:585-586) "What the dogfood runs are
      judged on has not been decided (OQ3)" is now decided by D23.
    - >-
      V6, non-blocking, lead repair. D22 and D23 were inserted between D21's accepted risk and its
      how-decided paragraph. D21's card (record:591-608) lacks its how-it-was-decided part, and D23's
      card carries it (record:665-667). Repair: move the paragraph back under D21.
  recommended_status: >-
    needs-revision, non-blocking. Five lead repairs (V1, V2, V4, V5, V6) and one item that needs the
    user's word (V3, can ride the D18 screen). No fold reopens a decision. Once those lines land,
    nothing in this review stands against acceptance, and no further cold read is owed.
---

## Failure narrative

The record's Destination is a target column "complete enough that a later build session can plan
the rebuild without asking the user a design question". Running D16's builder test against that
line, a builder would still have to ask at least five design questions. S1: does the target
supersede the user's own Contested goal-and-harness ruling (realignment D1/D6), whose counted
bounds and default pipeline it brings back? S2: when is the size ruled, given the blind map and
the first map display? S3: does the Small path drop a protected review floor that constraint 1
keeps? S4: is the card check a tool or a seat? S5: how does brainstorm's rule coexist with the
shared skill's one-per-turn row? S7 asks the user to re-rule Q10b on an accurately stated
premise. S6 asks for a ruling on the fact route, given that this record holds three lead-paraphrase
fact defects (S8, S9, S11). None of these breaks a decision's core. The stage-by-stage design is
coherent, well evidenced and honestly marked. The gap is the distance between the design and the
build-readiness the record claims.

## Notes of note

- Verification was first-hand. 11 of 13 facts were sampled against files and both clones.
  F1-F4, F6, F8, F9, F12 and F13 hold. F5, F7 and F10 each carry one defect (S9, S8/S11, S10).
- S1 and S6 are coverage gaps against load-bearing map angles B1 and B5. Under
  brainstorm.coverage-survivor-routing they go to the user as candidate topics. S15 and S16 are
  Minor coverage and can be ruled inline.
- S7 challenges a user ruling (Q10b), so it is reserved to the user (brainstorm.user-survivor-challenge).
- The ratified decisions were read first, per D8/D16's own aim. Of the seven Important
  survivors, five touch ratified or batch-confirmed decisions: D10, D11, D12-D17 and D19.
- External source re-read owed by the lead (EXTERNAL-CLAIMS re-read clause): S10, against
  pocock-skills@d81f3a1 docs/engineering/wayfinder.md:44 and :69-80.

## Verify round 1

Verify round 1 is NOT CLEAN, and nothing in it blocks. Of 18 items graded (16 folds, plus D22 and
D23 on the bounded grade), 13 are CLEAN and 5 are NOT CLEAN (S1, S2, S5, S15, D23). Every
substantive fold landed, and no decision reopens. The six fold-introduced defects (V1-V6) are
stale echoes and one misplaced paragraph. The exception is V3: the folded timing lets the stop rule
fire before a late blind map lands, which adds a condition to D15 and needs the user's word.
Correction on the cold pass: S10 was this seat's error. The quote is at wayfinder.md:75, and the
record's original F10 claim was right. Per-item evidence is in the frontmatter's verify_round_1
block. Recommended status: needs-revision, non-blocking. Once V1-V6 land, no further cold read is
owed.
