---
report: review
pass: cold
pairing: solo
lenses: decision-quality + record-integrity
reviewer: review-seat (mochiko:devils-advocate, persona tier opus, no override)
reviewed: .mochiko/brainstorms/specify-brainstorm-discovery/record.md (frozen 2026-10-10)
blind_map: .mochiko/brainstorms/specify-brainstorm-discovery/reports/angle-map.md (drawn at frame hardening, record_contact none)
recommended_status: needs-revision
status_criterion: >-
  No critical-gaps criterion holds. Every load-bearing fact checked out, both open questions
  have owners (OQ1 by D7-dogfood-criteria-deferred, OQ2 by a later session with a BACKLOG
  item), no Critical coverage gap was found, and the record is thick enough to review. But eight
  Important survivors stand. They include a contradiction inside the confirmed defaults (S1), a
  slate home that rewrites the index the record says needs no change (S2), and a build
  touch-set that is larger than D5-specify-own-rules names (S5). A builder reading only the
  record would still have to ask the user about each of them.
tally: 20 raised, 13 survived (0 Critical, 8 Important, 5 Minor)
cross_exam:
  Q1_defaults_engagement: >-
    Lead's answer: the batch was shown once as one block with Q4, each default in one line, with
    the offer "say pull X to open any default on its own turn". It was shown again on the accept
    screen and confirmed as one block. The user pulled none and commented on none. Effect: S7
    drops from Important to Minor, because the pull offer reduces the passive-acceptance harm.
    The offer itself is not in the record.
  Q2_test_line: >-
    Lead's answer: not ruled either way. The lead's intent was a user-facing line that the
    analyst expands into the Independent Test field without changing its meaning. The record
    does not say this, and the relation to authoring-user-stories.independent-test-required was
    never put to the user. Effect: S4 stands as Important.
  Q3_index_columns: >-
    Lead's answer: not ruled. The column list came from the second-list seat's description of
    the index plus a test column, and the template's own column grammar was not read. The intent
    was additive: keep Feature, add test, and widen Disposition. The collision between the two
    disposition vocabularies was not seen in the session. Effect: S2 stands as Important, with
    the additive intent folded into its resolution.
card_parts:
  D1-story-discovery-stage: all six parts present
  D2-ruled-unit-line-plus-test: all six parts present (card sits after the Freeze heading, S13)
  D3-lead-holds-story-stage: all six parts present (card sits after the Freeze heading, S13)
  D4-blind-journey-list: all six parts present
  D5-specify-own-rules: all six parts present
  D6-two-confirm-screens: all six parts present
  D7-dogfood-criteria-deferred: all six parts present
fact_verification:
  F-S1_to_F-S8_F-S10_F-S11_F-S13: verified against my own renders of the specify, authoring-user-stories, authoring-feature-map and patterns-sound-loop rules (binary 0.4.0, plugin 0.118.0)
  F-S9: verified (spec template render, lines 25–27, 115 and 117)
  F-S12: verified (architecture-brainstorm-interview record.md, lines 7 and 75)
  F-D1: verified (.mochiko/decisions/2026-08-28-near-dup-convergence.md, lines 36–40)
  D19_precedent: verified (brainstorm-target-state record.md, lines 559–562; the ellipsis is faithful)
  F12_correction: verified (brainstorm-target-state record.md, lines 149–154)
  ai-fileops_lines_57-59: verified (F4, F3 and F2 quotes at lines 57, 58 and 59)
  F-H1_to_F-H3: >-
    Not live-checked. They stopped being load-bearing at F3, because D3-lead-holds-story-stage
    rests on the user's word and would stand if they were false. They are disclosed as quotes
    returned by a seat, with URLs.
  index_lines_11_and_17: not checked (fenced from index.md)
fitness:
  self_contained: >-
    Pass. The topic, F1–F3, F-S1–F-S13, F-H1–F-H3, F-D1, the Q1–Q7 trail and the cards carry
    every decision. Caveat: card placement is out of order (S13).
  decisions_attackable: pass (each card's Why is concrete; D1's Why ties to F2 and to F-S1/F-S5)
  decision_trail: pass (each card names its Q; the ratified streak is counted 1, 2, 0, 1, 2, 3; Changed at review is visibly "—")
  confidence_marks: >-
    Pass. Confident on explicit yeses is the ruled practice ("Ratified" in how-it-was-decided,
    no new mark: brainstorm-target-state record line 428). D3 is Contested because the user ruled
    against the lead's lean, and D7 is Deferred.
  rejected_roads: pass (every card names its roads and reasons; the second list is saved verbatim in research/)
  honest_about_open: >-
    Pass with caveat. OQ1, OQ2, the three untested bets and D7 are listed as open. But two
    threads ride the defaults as if settled and are not listed as open: what "test: open"
    becomes at authoring (S4), and where a slate-deferred row resurfaces (S3).
  provenance: pass (the header names the lead and the ruler; the seat roster gives tiers; the session-tier switch and freeze line are stamped)
coverage_diff:
  folded_and_answered: >-
    A2 (ruled: road 2 rejected in D1), B1, B2, B4, B5, B6, B7, C1, C2 (mooted by F3), C4, C5,
    D1, D2, D3, D4, D5, D6, E1, E5, F1, F2, F3, G2, plus the defaults batch.
  folded_but_answer_defective: B3 (S6), E4 (S8), G1 (S1)
  partially_folded: E2 (S9), E3 (S3), C3 (S12), F4 (S5)
  deferred_by_users_word: A1, A3, A4 (D7, OQ1); these are rulings, not gaps
  dropped: >-
    C6, C7, E6, F5, G3, G4 and G5, all marked non-load-bearing in the map. None re-weighs to
    load-bearing against the decisions. G5's build mechanics (the contract suite's frozen
    specify floor set) are folded into S5.
survivors:
  - id: S1
    severity: Important
    decisions: [defaults "Size and skip", defaults "Slate home and binding", D1-story-discovery-stage]
    kind: inconsistency inside the confirmed defaults
    finding: >-
      Two confirmed defaults contradict each other. "Skipping the stage is the user's word,
      recorded in the spec's Intent as a line" sits beside the new kind fail "a story authored
      with no slate ruled" and the new stress-test check "every story file maps to an `in` row".
      A skipped stage rules no slate, so every story authored after a skip trips both.
    failure_scenario: >-
      A decisive founder says "skip it, I have my three stories." The lead writes the skip line
      into Intent, which already holds six rulings plus the frame against a 10-line cap. The
      analyst authors three stories. Then the new fail stands, and the stress-test raises a
      blocking gap for three unmapped files. Read literally, the run cannot finish without the
      stage.
    resolution: >-
      Rule one of two. Either the fail and the check carve out a recorded skip, or a skip is the
      G2 inputs-in-hand path, where the user's stories are transcribed as `in` rows so a slate
      exists. Say where the skip line fits within the Intent budget.
  - id: S2
    severity: Important
    decisions: [D1-story-discovery-stage, defaults "Slate home and binding"]
    kind: inconsistency with a cited fact (F-S9) and with the template's floor restatement
    finding: >-
      D1 places the slate in the existing `## User Stories` index on F-S9's reading that "a
      ruled story slate has a home today". The default then gives the index different columns:
      "ID · one-breath line · priority · independent test · disposition (in / out / deferred)".
      The template's columns are "ID (linked to stories/US-N.md), Story (one breath), Priority,
      Feature (FEAT-XXX-<slug> or —), Disposition (homed | rejected — why)". Feature is gone, a
      test column is new, and the disposition vocabulary is replaced. `out` and `deferred` rows
      carry IDs that link to story files that are never authored, which makes dead pointers.
      The template also says "The only story-native status is `rejected`", restating
      authoring-feature-map's single-home status floor, and the slate adds two more
      story-level dispositions beside it.
    failure_scenario: >-
      A builder implements the default as written. Derivation's Feature re-tag has no column,
      and review-specifications' disposition check reads "homed | rejected" against
      "in/out/deferred". The KM dead-pointer invariant flags every struck row's link.
    resolution: >-
      Rule the index shape as the lead intended (cross-exam Q3): additive, keeping Feature,
      adding the test column. Resolve the two disposition vocabularies, in/out/deferred before
      authoring and homed/rejected after, as two fields or two phases of one field. Either
      `out`/`deferred` rows carry no file link, or they sit in a `###` sub-table. Record the
      template migration this needs, and the reading that pre-authoring dispositions are not
      story statuses under the floor. Read the template's column grammar first-hand, not
      through the second list's paraphrase.
  - id: S3
    severity: Important
    coverage: true
    decisions: [D1-story-discovery-stage, defaults "Slate home and binding"]
    kind: coverage, E3 partially folded (the return-path limb was dropped)
    finding: >-
      The default makes the `out`/`deferred` row the home of a struck candidate. But a
      slate-`deferred` row has no return path. It is never authored, never a pending map row,
      never in the completeness view, and outside the PM's re-surfacing scope. It also reads as
      "later", which is selection's question. D1 says the slate never rules build-now, and
      selection's deferral comes with that machinery (pending rows, deferred-SC list). Two
      meanings of "deferred" now share one spec.
    failure_scenario: >-
      The founder defers "bulk CSV invite" at the slate. The spec is accepted, and the row lives
      only in a frozen spec index. The territory is re-specified months later, and nothing
      brings the candidate back. `pm-requirements-stacking` D1-across-round-phasing (DECISIONS.md
      2026-08-10) requires every deferred remainder to carry a named return path.
    resolution: >-
      Either keep the slate to in/out only, so that "later" means `in` and unselected at the
      selection card (which already makes a pending row), or give slate-deferred a return path,
      such as a parked stub staged for the acceptance batch.
  - id: S4
    severity: Important
    decisions: [D2-ruled-unit-line-plus-test]
    kind: inconsistency with the story skill (authoring-user-stories)
    finding: >-
      D2 defines the test as "the one sentence that says how the user would know the story
      works" and allows "test: open". authoring-user-stories.independent-test-required requires
      the Independent Test to name how QA verifies the story in isolation, what data or setup it
      needs, and what passes or fails. authoring-user-stories.rationalization-stop (floor) bars
      incomplete stories. So the analyst either rewrites the ruled test, which breaks untested
      bet 3 ("zero rewrites of a story's line or test at acceptance"), or the story file fails
      the skill. A "test: open" row forces the analyst to invent the test, which is the "same
      guess problem" D2's accepted risk names.
    failure_scenario: >-
      The user rules "test: I can see the invitee in my member list". The analyst must add setup
      (seed workspace, invite token) and a fail condition, so the acceptance screen shows a
      rewritten test. Bet 3 fails on every story by construction.
    resolution: >-
      Put to the user what the lead intended but never ruled (cross-exam Q2): the slate test is
      a user-facing seed that the analyst expands into the Independent Test (setup or data, plus
      pass/fail) without changing its meaning. Alternatively, the test is a separate "user's
      check" line carried beside the Independent Test. Restate bet 3 as "no change of meaning",
      not "zero rewrites". Rule what "test: open" becomes at authoring, for example an Open
      Questions entry plus a provisional test marked as such.
  - id: S5
    severity: Important
    decisions: [D5-specify-own-rules, defaults "Slate home and binding", defaults "Filter"]
    kind: missing intra-decision dimension (the build touch-set) and builder test
    finding: >-
      D5 says the stage is written into specify's own schema and gives a precedence clause over
      analysis-iterative only. The confirmed defaults need edits elsewhere: authoring-user-stories
      (the 2–5 bound yields, the Independent Test meaning per S4, and "landing in `spec.md`"),
      review-specifications (the new traceability check), the spec template (S2),
      authoring-feature-map (the filter partly moves into the stage, against its flow of "frame
      at intent, stories authored inside it, filter and selection after"), specify.md's fixed
      Goal step (F4), and the contract suite's frozen specify floor set, because spec.fail.* are
      class floor. A precedence clause over analysis-iterative does not reach
      authoring-user-stories. That skill's letter-is-spirit floor keeps the 2–5 rule binding on
      an analyst handed a ruled count of seven.
    failure_scenario: >-
      The build edits specify's schema only. The analyst seat, bound to a seven-row slate, also
      loads authoring-user-stories, whose rendered rules still say 2–5 per feature under a floor
      that says the letter is the spirit. The seat either stops or trims the set.
    resolution: >-
      D5 (or a build-surface line) names the full touch-set. For each conflicting skill rule, it
      names either a precedence clause in specify or a ruled skill edit, and it names the
      specify.md Goal clause for the slate.
  - id: S6
    severity: Important
    decisions: [D1-story-discovery-stage, defaults "Mechanisms carried"]
    kind: missing intra-decision dimension (B3 keep/drop list incomplete)
    finding: >-
      The mechanisms default keeps "one fork per turn" and a status line. It never says whether
      brainstorm's own-turn-or-batch carries over (real choices get a turn, the rest are batched
      defaults), or the stop rule's stall clause ("if the map has not shrunk for three questions
      running, say so"). Those two decide the stage's length, and the stage's length is the
      record's first untested bet.
    failure_scenario: >-
      Take a three-capability spec with twelve candidates. Each line plus test is ruled one per
      turn, four blind-list differences are ruled one by one (D4), and the PM is consulted for
      map fit on each fork. That is over twenty turns before any authoring, with no rule letting
      the lead batch the uncontroversial rows.
    resolution: >-
      Rule own-turn-or-batch as carried: a candidate gets its own turn only when it is a real
      choice or costly to undo, and the rest are default rows confirmed on the slate screen. Add
      the stall clause.
  - id: S7
    severity: Minor
    decisions: [defaults batch]
    kind: passive acceptance (downgraded at cross-exam Q1)
    finding: >-
      The defaults batch settles at least three load-bearing blind-map angles that carry real
      choices. They are the frame ordering (D2: carve the slate out as not-the-frame, or
      supersede F-S3/F-S10), the filter moving into the stage (D5), and the floor-strike route
      (E4). All three were confirmed by one "as recommded" at the accept screen, right after a
      three-ratification streak. brainstorm.own-turn-or-batch gives a real choice its own turn.
      The lead offered "pull X" (cross-exam Q1) and the user pulled none. That offer is not in
      the record, and S1 and S8 show the batch was not tested from inside.
    failure_scenario: >-
      The build lands a PM-vetting-in-stage flow the user never weighed on its own. That flow
      changes authoring-feature-map's filter timing and adds a seat round trip per fork (S6).
    resolution: >-
      Record the pull offer and the user's non-pull. When folding S1, S5 and S8, put the
      filter-timing default to the user on its own turn, since it changes a skill's flow.
  - id: S8
    severity: Important
    decisions: [defaults "Floor-obligated stories"]
    kind: unchallenged assumption against the precedent it reuses
    finding: >-
      "A strike of such a story is a waiver on the governance ledger's existing path, never a
      bare `out`." The precedent arch.floor-precedence routes a true drop to a ledger waiver
      through /mochiko:setup, but it also allows "n-a — handled elsewhere" with its required
      pointer, and a narrowing. The default offers only waiver or `in`. A ledger waiver is a
      MINOR governance amendment through /mochiko:setup amend mode
      (.mochiko/memory/governance-ledger.md, amendment policy). The record does not say what the
      specify run does while that is pending.
    failure_scenario: >-
      A brownfield product already delivers auth under an existing capability. A new spec's
      candidate "user resets password" is floor-obligated and already covered. The founder
      strikes it as covered, and the default turns a non-drop into a governance amendment that
      halts a specify run into setup, which this session lists out of scope.
    resolution: >-
      Add "out — handled elsewhere: <pointer>" and "narrowed" as legal strikes, and keep the
      waiver for a true drop. Add the routing rule (dispatch to /mochiko:setup, as the
      architecture rules do), and say whether the run pauses or continues with the row held.
  - id: S9
    severity: Important
    coverage: true
    decisions: [D2-ruled-unit-line-plus-test, defaults "Slate home and binding"]
    kind: coverage, E2 partially folded (inflation limb dropped)
    finding: >-
      The new stress-test check grades presence: every story file maps to an `in` row and every
      `in` row has a file. Nothing grades content. Nothing checks that a file carries the ruled
      line and test, or that scenarios, FRs and SCs stay "inside the ruled lines", as D2
      requires. Blind map B2 named this as the volume moving one layer down. Bet 3 rests on the
      user's eye at acceptance.
    failure_scenario: >-
      Four rows are ruled. The analyst authors four files, each with three scenarios, and FRs
      covering admin flows nobody ruled. The check passes and the volume is back under a tight
      story list.
    resolution: >-
      Extend the check so the story line and test match the row, or the expansion is shown per
      S4, and so each FR traces to an `in` row's story.
  - id: S10
    severity: Minor
    decisions: [frame card, D3-lead-holds-story-stage]
    kind: record integrity (frame line changed without a stamp)
    finding: >-
      F3 ruled that the pair is the lead and the user. The frame card's Destination row (line
      152) still reads "the analyst seat and the user" with no stamp.
      brainstorm.frame-changes-users-word requires a one-line stamped change and a re-check of
      the decisions against it. F2 carries an inline correction note, but a builder reading the
      destination alone would build a seat-held conversation.
    resolution: Stamp the Destination change citing F3, and note that the re-check finds D1–D7 consistent.
  - id: S11
    severity: Minor
    decisions: [D4-blind-journey-list]
    kind: builder test
    finding: >-
      D4 names neither the seat's persona nor its alias. "Rides the session tier" is not an
      alias: a rostered persona pins its own tier, and a persona-less seat needs an explicit
      `model:` per mochiko:patterns-model-tiering. The provenance vocabulary (user's words /
      lead's guess, ratified) also has no value for blind-list origin, so bet 2 ("counting the
      rows it added that the user ruled in") cannot be read off the slate.
    resolution: Name the seat type and its alias, and add a "blind list" provenance value.
  - id: S12
    severity: Minor
    coverage: true
    decisions: [D3-lead-holds-story-stage, D2-ruled-unit-line-plus-test]
    kind: coverage, C3 folded only implicitly
    finding: >-
      The record never says what the analyst's authoring brief carries: the slate rows alone, or
      also the user's own words per row. The one-breath line is the lead's compression. It also
      never says who writes the slate into spec.md, or when. Record-as-you-go left with the
      record file, and a lead-written index in a seat-authored spec.md makes the file a
      multi-writer surface under the transport floor's single-writer leg.
    resolution: Add one default naming the brief's contents and the slate's writer and write timing.
  - id: S13
    severity: Minor
    decisions: [D2-ruled-unit-line-plus-test, D3-lead-holds-story-stage, record-wide]
    kind: record integrity and joined IDs
    finding: >-
      (a) D2 and Q3/D3 sit after `## Freeze` (lines 584–667), D4–D7 sit under `## Decision map —
      reshown after D3`, and `## Decisions` holds D1 alone. (b) The new mints
      `D2-ruled-unit-line-plus-test` (five words) and `D3-lead-holds-story-stage` (four words)
      break the joined form: "exactly three lowercase ASCII words … a hyphen run that is not
      three such words is not a slug" (templates/artifact-format.md, IDs). (c) `mochiko-cli ids
      --check` reports 20 bare. The real bare decision cites are at lines 294, 385, 394, 429,
      482, 557, 558, 572 and 613, where lists of up to three must be joined. The rest are the
      blind map's D-class labels, whose letter collides with the session's D namespace, which
      is my map's naming.
    resolution: >-
      Move the cards under `## Decisions` in number order, re-slug D2 and D3 to three words
      (sweeping every cite), and join the bare cites.
---

## Failure narrative

The record is not ready. The seven cards are sound as rulings, and every load-bearing fact
checked out. The defects sit in the confirmed defaults batch, which the build will carry
verbatim. One default contradicts another (S1). The slate home rewrites an index the record
treats as unchanged (S2), and the deferred rows it creates have no way back (S3). The test line
has no stated relation to the story skill's Independent Test (S4). The build touches more
primitives than D5 names (S5). How the stage keeps its turn count down is unsaid (S6). The
floor-strike route overrides a precedent it means to reuse (S8). The traceability check grades
presence, not content (S9). Cross-exam confirmed S2 and S4 were never ruled in the session and
downgraded S7. Each Important survivor needs a disposition from the user, and most fold as
defaults.

## Notes of note

- Fence: my blind map was drawn before any record contact (record_contact none). The record
  and the second list were first opened on the lead's message-two signal.
- Dropped at the hunt, 7: Confident on ratified yeses (ruled practice) · F-S13's transcription
  reading (holds by the intent-synthesis precedent, where the lead writes a user-confirmed
  synthesis) · F-H1–F-H3 unverified (not load-bearing after F3) · D4 per run even for tiny
  specs (risk accepted in its card) · D1 Confident on an Assumed diagnosis (D1 rests on F1/F2,
  the user's want; OQ1 holds the diagnosis) · D4's seat seeing the frame (frame-first already
  rules stories inside the frame) · OQ2's trigger tied to the sibling (consistent with the 3+ bar).
- Contested D3: nothing raised against the ruling itself. S12 is a missing dimension
  (brief contents, slate writer), not a re-raise.
- Tiering deviation: two haiku Explore spawns (one per message) returned no report; the same
  targeted reads ran on the seat tier. Cross-exam answers and effects: frontmatter `cross_exam`.

## Verify round 1

- Graded: record.md snapshot taken at 14:51:30 on 2026-10-10 (817 lines, sha256 81ae8371ae4a…),
  still identical to the live file when this block was written. The record changed during the
  verify read (816 to 817 lines at 14:51; the D5 cite in *Slate home and binding* and the
  survivors-table cites were joined) after the lead said "writers hold". I asked the lead to stop
  edits (quiesce-before-cold-grade). The edits I saw were cite joins only. The lead confirmed in
  two messages that the complete list of edits after the fold is ten cite joins, made in one
  batch before the snapshot, with nothing moved since. The snapshot therefore covers the whole
  fold. The lead owns the breach and will disclose it in the record's run notes after this
  verdict. The record was re-checked as identical to the snapshot after that confirmation. At
  14:57, after this verdict went out, a lead repair of N1 landed in *Size and skip*. It falls
  outside this round and goes to the next verify.
- Read: only the changed parts (`brainstorm.fix-one-card-verify-once`): the changed-at-review
  parts of D1–D5, D6 and D7 for cites, *Defaults batch — confirmed*, the stamped Destination row,
  the F-S9 note, the reshown map's pointer, the label note, and the `## Cold review` table.
- Recommended status: **needs-revision**. All 13 folds are CLEAN, but the folds introduced six
  Minor defects (N1–N6). Three of them (N1, N2, N5) fail the builder test because a builder
  would still have to ask the user. One more verify round follows the repair, and a second
  failed verify goes to the user.

| Survivor | Verdict | Evidence |
|---|---|---|
| S1 | CLEAN | *Size and skip*: skip means handed-in stories become `in` rows with provenance `handed in`; the fail and the checks are unchanged; the skip is recorded in the slate header, not Intent |
| S2 | CLEAN | Columns are additive (Feature kept, Test added); Disposition is one field in two phases; `out` rows are unlinked in `### Struck candidates`; the floor reading is stated; the migration is named; F-S9 is annotated |
| S3 | CLEAN | No slate `deferred`; "later" is `in` and unselected, which gives a pending row; D4 and the stop rule read in/out |
| S4 | CLEAN | D2-line-plus-test changed-at-review: the seed is expanded per the skill without changing its meaning and is shown beside the expansion; `test: open` becomes Open Questions plus a provisional test; bet 3 is restated; check (2) agrees |
| S5 | CLEAN | D5-specify-own-rules changed-at-review names six touches as ruled primitive edits, including the specify.md Goal clause and the contract suite (but see N1, N3) |
| S6 | CLEAN | *Mechanisms carried* adds own-turn-or-batch and the stall clause |
| S7 | CLEAN | Filter timing ruled on its own turn (PM vets once at slate close); the pull offer is recorded in the reshown map and the table |
| S8 | CLEAN | `out — handled elsewhere: <pointer>` and `narrowed` are added; a true drop goes to `/mochiko:setup` amend; the waiver-pending behaviour is stated (but see N2) |
| S9 | CLEAN | Checks (2)–(4) are added (meaning kept, FR trace, scenario bound) (but see N3) |
| S10 | CLEAN | The Destination row is stamped with F3 and re-checked against D1–D7 |
| S11 | CLEAN | Persona-less `general-purpose` seat with explicit `model: opus`, disclosed; `blind list` provenance value (but see N4) |
| S12 | CLEAN | *Hand-off and slate writing*: the slate is written at confirm, the pen passes explicitly, and the brief contents are named (but see N5) |
| S13 | CLEAN | Cards under `## Decisions` in order; three-word re-slugs, cites swept; trail gathered; label note added. The 18 bare flags left are the blind map's report labels, bare by rule, apart from N6 |

New defects introduced by the folds (all Minor):

- **N1** (S1 fold against D4-blind-journey-list and D6-two-confirm-screens): a skip now yields a
  slate without a stage. The record does not say whether the blind list ("one list per spec
  run"), the PM's slate-close vet and the slate-confirm screen still run on a skip. The slate
  header line holding the skip is a template element that D5's spec-template touch does not
  list. Fix: one line in *Size and skip* saying which slate-close steps run, plus the header in
  the template touch.
- **N2** (S8 fold against the S2 fold and check (1)): `narrowed — <what remains>` and `in — waiver
  pending` are missing from the two-value Disposition (`in` | `out — <why>`). A waiver-pending
  row is `in`, so its story file gets authored. If the waiver lands, the row leaves `in` and its
  file breaks check (1), and that transition is unstated. Fix: list the full slate vocabulary in
  one place, and say what happens to the row and its file when the waiver lands, or that
  authoring waits for the waiver.
- **N3** (S9 fold against the S5 fold): check (3), "every FR traces to an `in` row's story", has no
  carrier. FR lines are "one line each" with no story reference in the template or in
  authoring-requirements, and neither is in D5's touch-set. Fix: say the trace is the
  reviewer's judgment, or add a trace form and name the touch.
- **N4** (S11 fold against F3): D4's seat is "teammate or subagent at the lead's call", but F3
  records "the user's runs seat teammates, never subagents". Fix: make it a teammate, or record
  why a disposable seat that only returns text may be a subagent.
- **N5** (S12 fold against *Slate home and binding* E1, *UX-bearing work* and the S8 fold): once the
  pen passes to the analyst, three defaults still change slate rows: author-proposed rows the
  user rules in, a click amendment to a row's line or test, and a waiver outcome. If the analyst
  writes them, that is the transcription the hand-off default rejected as needing grading. If
  the lead writes them, the pen must pass back. Fix: name the writer for changes after the
  confirm.
- **N6** (S3 fold, joined IDs): line 696 cites "(pm-requirements-stacking D1)" bare. The
  definition is joined as `D1-across-round-phasing`, so the cite needs the owner code span and
  the slug. It was copied from my own report, which carried the same bare cite; I fixed it there
  and swept five bare cross-record cites in my angle map (`ids --check`: 0 bare on both reports).

Commentary, no finding: under in/out, a founder's "maybe later" is either `in` (authored now) or
`out` (no return path). The lead may want the slate screen to say so when a candidate is struck
"for later".

## Verify round 2

- Graded: record.md snapshot (859 lines, sha256 dc99fefcc0759df5, matching the lead's pin),
  still identical to the live file when this block was written. A diff against the round-1
  snapshot shows changes only at the spans the lead listed: D4's changed-at-review, D5's
  touch-set, *Slate home and binding*, *Floor-obligated stories*, *Size and skip*, *Hand-off and
  slate writing*, and `## Verify pass`. Nothing else moved, and no write landed during this read.
- Recommended status: **ready**. N1–N5 are CLEAN, and the three builder-test failures from round
  1 (N1, N2, N5) are now answered in the record. All 13 cold-read survivors stayed CLEAN, since
  the repairs touched none of their substance, and every fitness item passes as graded at the
  cold read. What is left is three cite-format repairs (below). They change no content and need
  no ruling. They are fix-on-sight integrity repairs (the patterns-sound-loop exemption, the KM
  invariant), which the lead can apply without another verify. If the lead rules that they need
  a verify, this round's status reads needs-revision instead, and it goes to the user.

| Item | Verdict | Evidence |
|---|---|---|
| N1 | CLEAN | *Size and skip*: "a skip skips the forks, not the close". The blind list, PM vet and slate confirm run over the handed-in rows. The header line is in the template migration, and D5's touch-set gains the template line |
| N2 | CLEAN | Narrowing is an `in` row rewritten by the user's word, not a value. `in — waiver pending` is not authored while pending, and check (1) treats it as not yet due. On landing, the row becomes `out — waived: <ledger reference>` and is never authored, or the user restores it to `in` and it is authored. *Slate home and binding* lists the slate-phase values `in` · `in — waiver pending` · `out — <why>` |
| N3 | CLEAN | Check (3)'s carrier is a `US-n` reference on each FR line. `authoring-requirements` and the template's FR guidance join D5's touch-set |
| N4 | CLEAN | D4-blind-journey-list changed-at-review: a persona-less `general-purpose` teammate, never a subagent (F3) |
| N5 | CLEAN | After the pen passes, row changes are ruled by the user with the lead and sent as the exact row text. That fits `patterns-transport-floor.content-pinned-supersession` ("An order quotes the exact text it lands"). The analyst writes them as mechanical execution of a ruling (F-S13), so there is one writer and no clash with the rejected brief-transcription road |
| N6 | NOT CLEAN (format only) | It now reads "(pm-requirements-stacking D1-across-round-phasing)". The slug is joined, but the owner is not in a code span, which the IDs section's Owners rule requires for a session decision cited outside its record: `` `pm-requirements-stacking` D1-across-round-phasing `` |

New defects from the repairs (Minor, cite format only):

- **R1**: the `## Verify pass` log adds two bare session-decision cites, "(default *Size and
  skip*; D5)" on line 846 and "N4 — D4's seat" on line 853. They should read
  D5-specify-own-rules and D4-blind-journey-list (`ids --check` flags both). The other 17
  flags are the blind map's report labels, bare by rule.
- **R2**: N6's owner code span, as in the table.

Commentary, no finding: *Floor-obligated stories* says a narrowing is "noted in the row's why",
but the slate-phase `in` value carries no why. The builder can put the note in the Disposition
text or the provenance cell without asking the user.

