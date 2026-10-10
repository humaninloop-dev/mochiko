---
report: review
stage: cold-read
skill: mochiko:review-brainstorm
seat: review-seat (devils-advocate, cold, solo, map drawn at front)
reviewed: .mochiko/brainstorms/architecture-brainstorm-interview/record.md (frozen 2026-10-11, 1311 lines)
read_also: [reports/angle-map.md, research/second-list-shape.md]
date: 2026-10-11
recommended_status: critical-gaps
raised: 29
survived: 24
critical: 1
important: 15
minor: 8
coverage_findings: 4
cross_exam: 5 questions to team-lead, answered by message 2026-10-11; record untouched
fence: index never opened; no session artifact read beyond the three named above
---

## Failure narrative

Status `critical-gaps` rests on one broken load-bearing claim (S1). D7 accepts a second home for data-model facts because "the `Draws:` line and the stale flag" hold it in check, and both key on "data-model entities by their joined IDs". No entity ID family exists: the IDs section of `plugins/mochiko/templates/artifact-format.md` lists none, and kinako's `data-model.md` heads entities by name ("### Corpus — the `meta` table"). The lead confirmed this in cross-exam and called the fix "a ruling, not a wording repair".

The fifteen Important survivors share one shape. The decisions are sound in direction, but several statements reach into implement's and the store's floors without naming the rules they change. A builder could not start from the record alone (see Builder test). Five things the lead holds as intent are absent from the record: the row gate (S2), the diagram marker (S3), the batch scope (S11), the two-key rule (S3) and D6's exact exchange (S14).

## Survivors

Severity · decisions touched · scenario or contradiction · resolution path. `[coverage]` marks a blind-map diff.

**S1 · Critical · D7-diagram-set-index-points, D8-draw-before-code.** The ER and state views cite IDs that do not exist. Scenario: a delta-scope bug fix adds a state to kinako's `JournalEntry.state`. No ID changes, so no compare can fire, and agents read a stale state machine as truth. That is exactly the risk D7 accepted on the strength of the mechanism. Contradiction: D7 says "data-model entities by their joined IDs"; artifact-format's families list no entity family (cross-exam answer 2 agrees). Resolution: a user ruling among (a) minting an entity ID family, which takes a template migration and touches `patterns-entity-modeling` and so needs a frame change, (b) citing `data-model.md § <heading>` with its rename drift accepted, or (c) keeping ER and state views out of the set, or marked unchecked, until a key exists.

**S2 · Important · D1, D2-part-direction-know, D3-visit-file-home.** The record never says where the user rules the architect's concrete row text. D2: "the stance word, rationale and as-built fields are authored afterwards". D3: tech-lead grades "for sameness of meaning". Scenario: the direction is "Google or email sign-in", with "a new user signs up in a minute" as the know-sentence. The architect writes "decided: Auth0", the sameness grade passes, and nothing shown to the user names Auth0. Read literally, the stage moves the architect's guess past the last user gate, which is the problem's own failure one layer down. Cross-exam answer 1 says the existing gate is meant to stand (`arch.user-reserved-rulings`, `arch.sound-loop-floor`), unwritten. Resolution: one sentence on D3 saying the stance-batch user gate after tech-lead's grade is unchanged and the sameness grade is additive to it. D1's "Two agreed screens per visit" then becomes three or more, which bears on the "not heavier" bet.

**S3 · Important · D7, D8.** Diagram hunks are unmarked, and two open keys collide on one file. D8 edits `diagrams/*` in place inside `.mochiko/product/`. `impl.fail.unmarked-baseline-write` (floor) fails any hunk "outside an entry marked for the run's key", and `impl.baseline-entry-grammar` requires a `**Lifecycle:**` field. D7's file shape (one block, a caption, a `Draws:` line) carries none, so D8's "serialized by the lifecycle marker" asserts a marker that does not exist. Two features of one epic touching the overview flowchart has no rule ("not thought through", cross-exam answer 3). Resolution: rule each diagram file as one entry with its own `**Lifecycle:**` field. Rule one open key per file, with a second run treating it as an in-flight collision (the lead's fold option i). Or rule per-key delta classes.

**S4 · Important · D8, D7.** The stale safety net has no trigger, no executor and two homes for its flag.
- Trigger: the index regenerates only on store writes (`authoring-architecture-store.index-regenerated-every-write`), so a `data-model.md` write never runs the compare.
- Executor: "a mechanical compare the store skill runs" is a model, unless code does it. Code would raise a kernel-admission question under GI-019-kernel-tooling-admission.
- Undefined phrase: "with no drawing behind it" is never defined.
- Flag home: D8 puts the flag in the index's health view, while D8's last line and the Readers default have agents read it with the diagram file.
- Count rule: `health-view-binding` fixes "five counts".

Resolution: name the trigger moment for baseline writes, the executor, one home for the flag, and the sixth count.

**S5 · Important · D3, D5-visit-file-design-input.** The visit file sits inside the store directory, written by the lead. `arch.single-writer-store`: "one seat holds the pen per visit". `authoring-architecture-store.one-home-one-writer` (floor): "every write to it goes through this skill". D3 restates this as "one writer per file" and records no amendment. Scenario from D5's own text: at an implement checkpoint "the run pauses for a desk visit". The visit file then lands under `.mochiko/product/` inside the open run's pinned-base diff, an unmarked hunk, so `impl.fail.unmarked-baseline-write` fails the run. Resolution: home visit files outside `.mochiko/product/`, as the feature desk's `.mochiko/features/desk/` sits outside the map's truth files. Or record both amendments plus a run-pause exception.

**S6 · Important · D8.** The record does not say what the user signs for a small change in a large system. `patterns-system-design` makes the container delta diagram "the sign-off surface", scoped to the delta neighborhood, with "~12 rendered nodes" before the full view is "linked, never inlined". D8 shrinks the run's drawing to a table plus links and signs the store's diagram files. D7 keeps the container view in `spine.md`, not `diagrams/`, so it is unclear whether D8 edits it. Kinako's spine container diagram runs about 90 lines. A 2-node change then means signing a wall, or signing no container view at all. Resolution: name the files D8 edits, and keep or retire the neighborhood view by named supersession.

**S7 · Important · D8.** The deviation gate's reach widens to every diagram kind. `impl.deviation-gate` (floor) stops a cycle that "adds or removes a box … an arrow" of the signed delta. D8 makes the gate read the store diagrams, whose kinds are open (ER, state, flowchart). Scenario: a builder adds a retry state the signed state view lacks. The cycle stops for a user amendment, and every entity or transition tweak becomes a stop. Resolution: name the kinds the gate holds the build to, for example the container and flow views. The rest go to the landing diff.

**S8 · Important · D8, D9-render-where-viewed, the frame.** Implement touches exceed the stamped frame change. The stamp (record line 1078) allows three implement lines. D8 and D9 also change:
- `impl.design-outputs-home` (the run's drawing shrinks);
- `impl.deviation-gate`;
- the built-vs-signed diff;
- the landing flip;
- gap-finding's first message, which carries "the signed architecture.md drawing";
- `impl.gate-design-checkpoint` ("signed on a rendered diagram"), changed by D9.

`brainstorm.frame-changes-users-word`: a line changes only by the user's word. Resolution: put the full implement touch list to the user as one frame-change stamp, or narrow D8.

**S9 · Important · D9.** D9 names neither the rule texts it supersedes nor who checks "must parse".
- Floors carrying "rendered": `authoring-architecture-store.sign-off-flips-proposed` ("on the rendered diagram") and `impl.gate-design-checkpoint`, which already has a degrade path. Plus `patterns-system-design`'s "must render" and its checklist. D9 names only `architecture-design-primitive` D8-rendered-diagram-signoff.
- "Must parse": D9 makes the parse check optional ("the build decides"), so "must parse" can go unenforced. A user could sign text that neither parses nor was seen.
- Publishing: the named render surface, a published page, sends product architecture to a hosted service at each sign-off.

Resolution: list the superseded rule texts; rule who parses and when, or soften to "should"; ask the user before any publish.

**S10 · Important · D1-focus-stage-then-author.** D1 says the breadth floor "stands" while changing its order clause. `arch.breadth-invariant` (floor): "the walk order is by retrofit cost". `patterns-architecture-shelves.retrofit-cost-walk-order`: "retrofit cost is the whole ordering principle". D1: "focus is depth and order inside the full walk". The second list's road 5 flagged "fights retrofit-cost order", and D1's accepted risk dropped it. The batch also shows only "default and cost", while `patterns-architecture-shelves.recommend-then-arbitrate` (floor) requires "what would argue against it". D1 rejected road B for amending that same floor. Resolution: walk focus items and batch rows each in retrofit-cost order, and give each batch row its case-against. Or record supersessions.

**S11 · Important · [coverage] · D1 · angles 1 and 21.** The batch's row set on later visits is undefined. "The rest of the shelf scope in play" could re-deal `decided` rows as shelf defaults on an amendment visit and flip a ruled stance by batch confirm. Diff: the map asked "once per baseline or on every visit"; the record took angle 1 but not this sub-question, and dropped angle 21. Materiality: likely changes D1's text. Cross-exam answer 5 gives the intent, unwritten: no-stance rows only, ruled rows reopen only as pulled focus items. Resolution: write that sentence into D1 and the batch default.

**S12 · Important · [coverage] · D1, D4-blind-list-multi-part · angles 2 and 9.** The context the focus is "chosen from" is never bound. D4's blind seat gets the goal, the `Scope:` line, the derived index and any handed-in record. It never gets the product's specs, FEATURES.md entries or codebase analysis. On a greenfield baseline the index is a scaffold, so the seat sees a goal and one line. Kinako's off-shelf rows (the basis of D4's Why) came from product context. The record also never lists what the lead reads to propose a focus, per visit kind. Materiality: plausibly changes D4's inputs. Resolution: name the inputs per visit kind for both the focus proposal and the blind list.

**S13 · Important · D1, D3, D4.** Machinery fires on visits that rule nothing. Every visit gets a directory (D3) and the stage (D1), and any multi-part set takes a blind seat (D4). A store query writes a file for nothing ruled. A drift visit with three findings is multi-part and pays a seat over a closed three-way disposition (`authoring-architecture-store.drift-user-disposition`). The defaults cover first visits only. Cheaper shape: the stage, the visit file and the blind list fire only on visits that rule a stance, baseline, amendment or structure. A query writes nothing. A drift visit records its dispositions without a blind list.

**S14 · Important · D6-desk-stays-engine-shared.** The record does not quote the decisive exchange, and the implement half rides a bundled yes. The record quotes neither the message that introduced "thinking engine" nor the re-put "question 1". The lead supplied both. In the original message the lead says "(1) … (my pick)" and also "Recommendation: (2)". The re-put "Yes" bundled "keep it" with "one shared thinking stage … brainstorm, specify, the architecture command, and implement all use". The user ruled the keep. Adding implement to the engine was never put as its own choice. Resolution: quote both messages in Q5b's trail. Put the implement widening as its own one-line confirm, or mark that half `Assumed`.

**S15 · Important · D6.** The hand-off target's charter cannot carry the work. BACKLOG.md lines 37–41: a `common.<slug>` block "under near-dup R1–R6 … boilerplate only, never judgment". D6 hands it an engine design that gives implement new behavior. That is judgment, not convergence of existing near-identical text. Resolution: open a new BACKLOG item for the shared thinking-stage design, keeping near-dup convergence separate. Or rewrite the item's charter by the user's word.

**S16 · Important · [coverage] · untested bets, dogfood default · angle 16.** The main bet's test cannot run where it is aimed. "The focus stage does not make a visit heavier than today's walk — tested by the first ai-fileops baseline run". ai-fileops is `**Scope**: desktop` (its `spine.md` line 3), and no desktop shelf is seeded (`patterns-architecture-shelves.honest-gaps`). So there is no walk to compare against and no shelf batch, and falsifier F5a cannot fire. The record dropped angle 16. Materiality: plausibly changes the dogfood plan. Resolution: name a comparison that holds on an unseeded surface, such as turns or time to close, or add a backend-scoped run.

**S17 · Minor · D5.** The desk writes the feature entry's architecture link "at the visit's close". `architecture.md` line 19 says the two desks "neither writes the other's truth", and `impl.design-map-assertion` has the design phase fill that link "when a store delta exists". Resolution: drop the desk-side write and rely on the slug, or let implement fill the link.

**S18 · Minor · defaults (`kind: fail`).** The fail lists "delta" among desk store writes, yet D5 says the desk writes no store delta. Read beyond desk visits, it would fail every implement run that has no visit file. Resolution: scope it to desk visits and drop "delta".

**S19 · Minor · D2.** The know-sentence becomes "trigger or target", but an upgrade trigger exists only on `not-now`, and Targets are `NFR-XXX` rows with a measurement point. A `decided` row with no NFR has no field for it. Turning a sentence into a number adds meaning. Resolution: say where the sentence lands for a decided row, and that numbers are the user's to give.

**S20 · Minor · D5.** A visit file keyed to a feature has no freshness rule. A design phase months later reads a focus set the store has since moved past. Resolution: the design phase reads it against its own base and lists conflicts at the checkpoint.

**S21 · Minor · defaults (outside claims).** The default says "tech-lead's grade re-checks the claims", but `arch.seat-tech-lead-grader` binds no grading contract, and no desk skill appears among `EXTERNAL-CLAIMS.md`'s consumers. Resolution: name the contract the re-check rides on.

**S22 · Minor · D4, D2, D9, D1's batch.** Some confidence marks are inflated, and the batch mitigation is untested. D4 was "ratified on a weak lean" with the lead's doubt stated, yet it is marked `Confident`. D2 and D9 were ratified with no engagement, after four plain-words requests. This session's own 16-row batch drew zero pulls. That is the very user behaviour D1's batch mitigation counts on being broken. Resolution: mark D4 `Assumed` or record why it is `Confident`. Add "pulls per batch" to the untested bets.

**S23 · Minor · [coverage] · defaults (taste) · angle 29.** BACKLOG.md line 907 queues a security-depth "trust-boundary diagram convention". The taste section also lands in `DIAGRAM-CONVENTIONS.md`, and nothing reconciles the two. Materiality: not likely to change a ruling. Resolution: name `DIAGRAM-CONVENTIONS.md` the one home that build extends.

**S24 · Minor · IDs (`review-brainstorm.joined-ids`).** `mochiko-cli ids --check` on the record prints "2 bare · 8 drift". Cross-session cites lack owner code spans, so they key to this record's numbers:
- `specify-brainstorm-discovery` D2-line-plus-test / D3-lead-holds-stage at lines 128, 132, 350, 610, 629 and 1201;
- `architecture-design-primitive` D8-rendered-diagram-signoff at lines 888 and 1035;
- bare D8 at lines 1080 and 1134.

Separately, five new mints carry four-word slugs: D1-focus-stage-then-author, D4-blind-list-multi-part, D5-visit-file-design-input, D6-desk-stays-engine-shared, D7-diagram-set-index-points. The rule says "exactly three lowercase ASCII words"; the tool did not flag these. Resolution: add owner spans; rename the slugs before the `DECISIONS.md` rows cite them.

## Card parts and fitness

Card parts: 9 of 9 cards carry all six parts (grep count 9 per part); every header leads with its joined ID and name.

- Self-contained: FAIL. D6's deciding exchange needed the lead's message to reconstruct (S14), and the row gate is intent, not text (S2).
- Decisions attackable: PASS. Each Why cites facts; for example D3 cites the F32 precedent and the git analysis, and D5 cites F35.
- Decision trail present: PASS. Every How-it-was-decided quotes the user and the streak; Changed at review reads "—" on all nine.
- Confidence marks honest: FAIL, minor (S22).
- Rejected roads recorded: PASS. All nine; D1 lists the second list's roads not taken.
- Honest about the open: FAIL. Several unweighed questions sit in settled prose instead of the open list: the entity key (S1), two keys on one file (S3), the batch scope (S11) and the stale trigger (S4).
- Provenance stated: PASS. Run notes, lines 313–336 (lead tier, seats, models, dates); freeze, lines 1306–1311.

## Builder test

Reading only the record, a builder would still have to ask the user:
- which key an ER or state view cites (S1);
- where the user rules the authored rows (S2);
- what marks a diagram hunk, and what happens with two open keys (S3);
- what triggers the stale compare, who runs it, and where the flag lives (S4);
- whether the visit file belongs to the store (S5);
- which files D8 signs (S6) and which kinds the deviation gate reads (S7);
- the full list of implement touches (S8) and the rule texts D9 supersedes (S9);
- the batch's row set (S11) and the context inputs (S12);
- which visit kinds run the stage (S13).

## Coverage diff

Load-bearing angles taken in depth: 3 (D5), 6 (D1, road D), 8 (defaults), 15 (D3), 23 (D7), 26 (F36, F37, OQ3). Taken but partial: 1 and 21 (S10, S11), 2 and 9 (S12), 11 (S21), 18 (S22), 22 (S1), 24 (S3, S4), 30 (S16). Dropped angles checked: 4 bit (S10) · 7 covered by D6 and the own-rules default · 10 not material · 12 covered (cold review dropped; tech-lead is the independent leg) · 16 bit (S16) · 17 residual in S2 · 20 covered by D3 · 21 bit (S11) · 29 bit (S23) · 31 bit (S8).

## Notes of note

- Raised 29, survived 24 (1 Critical · 15 Important · 8 Minor; 4 coverage). Five dropped after checking: the lead's visit-file writes as unlooped (they are transcription, as specify ruled); the streak rule (applied as written); a D6 versus D5 offer clash (staged, not in conflict); D3's rulings in two places (an accepted risk is a ruling); the sampled facts (all held).
- Cross-exam changed S2 from a Critical candidate to Important, because the lead's intent keeps the floors and nothing supersedes them. It confirmed S1, S3, S11 and S14.
- Verified first-hand: renders of the architecture, store, implement, review-sufficiency, feature-map and spine rules; `mochiko-cli home` of `.mochiko/product` and of the feature desk; artifact-format's IDs section and `mochiko-cli ids --check`; kinako `spine.md` line 8, its `concerns.md` (21 cites) and its `data-model.md` headings; ai-fileops `spine.md` line 3; BACKLOG.md lines 37–41, 793 and 907; `architecture.md` line 19; the diagram-design README (F19) and the GitHub diagrams page (F37), both fetched live.
- Taken from the record unverified: F36 (Mermaid docs), F18, F20, F24, F25, F31, and F9–F17's line references.

## Verify round 1

Verify pass, 2026-10-11, same seat. It read only the nine changed-at-review paragraphs with their cards' statements and accepted risks, the defaults batch, the open questions, the untested bets, both frame-change stamps, and the dispositions and repair-batch tables (`brainstorm.fix-one-card-verify-once`). There was no fresh cold read and no new coverage hunt. Recommended status: **needs-revision**. S1 is CLEAN, so no Critical item remains. Three survivors are NOT CLEAN, and three fold-introduced defects are Important.

### Grades

CLEAN, 21: S1 · S2 · S3 · S7 · S8 · S9 · S10 · S11 · S12 · S13 · S14 · S15 · S16 · S17 · S18 · S19 · S20 · S21 · S22 · S23 · S24. S24 was checked first-hand: `mochiko-cli ids --check` on the record prints "0 bare · 0 drift", and the five renamed slugs are three words each.

NOT CLEAN, 3:
- **S4.** The executor, the flag's home and the sixth count are fixed. The trigger still misses the original scenario.
  - The compare is called at index regeneration, at a design-phase checkpoint and "at a landing fold".
  - The fold runs only when structure was built: `authoring-architecture-store.fold-duty` carries `when: structure_built=present`, and `impl.store-landing` fires on "a built structural change".
  - A build-time decision is "written directly in that post-sign form" with no checkpoint (`impl.baseline-entry-grammar`).
  - So a delta-scope fix adding a state under `data-model.md § JournalEntry`, with no structural change, still flags nothing.
  - Fix: call the compare at every landing that writes a product baseline, or at any `data-model.md` write.
- **S5.** See V3. The paragraph and the row put the new home in different places.
- **S6.** See V1. The paragraph and the row disagree on what is signed.

### Fold-introduced defects

- **V1 · Important · D8-draw-before-code (S6).** The disposition row says "the run keeps the neighborhood view as the sign-off picture". The paragraph instead signs "the diagram files in `diagrams/` the change touches, plus the spine's container view when a container changes". It calls the neighborhood view "drawn from the same elements as the sign-off picture", then says "a two-node change signs a neighborhood, never a wall". Problems:
  - If the spine's whole container view is signed, a two-node container change signs a wall.
  - The deviation gate and the built-vs-signed diff need one signed object.
  - Two drawings of one delta are what the frame's bet ("without two copies that drift") rules out.

  Fix: one sentence naming the signed object and the picture shown, and saying which one the gate reads.
- **V2 · Important · D8-draw-before-code (S3 × S4).** The new flag home collides with the unmarked-hunk floor. The store skill writes a `Stale:` line into a diagram file at index regeneration, at the checkpoint or at the landing. Inside an open run, a file the run never marked for its key then gets an unmarked hunk in `.mochiko/product/`, which `impl.fail.unmarked-baseline-write` fails. Scenario: a feature run's design phase edits `data-model.md § JournalEntry`. At its checkpoint the compare writes `Stale:` into `diagrams/journal-state.md`, which that run never marked, and the run fails. Before the fold, the flag lived in the root index, outside `.mochiko/product/`. Fix, one of three:
  - the design phase redraws and marks any diagram whose cited target it changes, before sign-off;
  - the `Stale:` line is excepted from the unmarked-hunk test;
  - the flag goes back outside `.mochiko/product/`, with the health count.
- **V3 · Important · D3-visit-file-home (S5).** The paragraph says "a new home beside `.mochiko/product/architecture/`", which still sits inside `.mochiko/product/`. S5's option (a) and its row say "outside `.mochiko/product/`". D3's title and statement still read "under the store". Built from the card, the paused-run unmarked hunk comes back. Fix: write "outside `.mochiko/product/`" into the paragraph, and mark the title and statement as superseded by it.
- **V4 · Minor · the defaults batch, *Who draws*.** It still says "from the data-model baseline's entity IDs" and "the IDs its `Draws:` line cites", against S1's file-and-heading citation. Fix: say entities are cited by file and heading.
- **V5 · Minor · OQ2.** It reads "the convergence session's, widened to implement's design phase", against S15's new BACKLOG item and the convergence item left as it is. Fix: point OQ2 at the new item.
- **V6 · Minor · stale text on cards the fixes did not touch:**
  - D1's accepted risk still says "Two agreed screens per visit", against S2's three;
  - D3's statement still says "Each desk visit gets its own directory", against S13;
  - D3's statement still says "the sentence expanded into trigger or target", against S19;
  - D8's S8 line says "The implement touches this card and D9-render-where-viewed reach are nine", but two of the nine are D5-visit-file-input's.

  Fix: one pointer line per card saying the later paragraph governs.

### Verify notes

- Blocking: S4, S5 and S6 NOT CLEAN, plus V1, V2 and V3. V4 to V6 are repairs that do not block.
- Seat disclosure: `mochiko-cli ids --check` over `reports/` found the kernel-tooling governance ID written without its slug once in each of this seat's two reports. Both now read GI-019-kernel-tooling-admission, an ID-form change only. No Explore spawn was used, and the index was not opened.

## Verify round 2

Bounded verify, 2026-10-11, same seat. It covered only the nine repairs from round 1, reading the spots the lead named: D8-draw-before-code's changed-at-review paragraph (its S4 and S6 parts), D3-visit-file-home's title, statement and S5 paragraph, D1-focus-stage-first's accepted risk, the *Who draws* default, OQ2, the S4 disposition row, and the record's new `## Verify pass` section. Recommended status: **needs-revision**. One repair is NOT CLEAN (Minor), and one repair-introduced defect is Important and needs the user's word.

### Grades

CLEAN, 8:
- **S4 and V2.** The compare's trigger is every index regeneration. The index now regenerates at "every design checkpoint and every landing — structural or not — of any run that wrote a product baseline". That covers the delta-card `data-model.md` scenario. The flag lives only in the health view, outside `.mochiko/product/`, and "a diagram file carries no flag".
- **S5 and V3.** D3's title, statement and paragraph all put the visit directories outside `.mochiko/product/`, at the working path `.mochiko/architecture/desk/<date-slug>/`.
- **S6 and V1.** One signed object, the run's `architecture.md`: a neighborhood view plus a table of the edited store diagram files. The deviation gate anchors to it.
- **V5.** OQ2 now names the new shared-stage BACKLOG item.
- **V6.** D1 now says three screens. D3 now says "each visit that rules something" and points to D2-part-direction-know for where the sentence lands. D8 now says the nine touches come from "this card, D5-visit-file-input and D9-render-where-viewed".

NOT CLEAN, 1:
- **V4 · Minor.** The *Who draws* default fixed one of the two phrases round 1 named. It still ends "tech-lead grades a drawing against the IDs its `Draws:` line cites" (record lines 1401–1402), yet a data-model entity is cited by file and heading, not by ID. Exact repair: "against the targets its `Draws:` line cites". This is the item's second NOT CLEAN, so it goes to the user.

### Repair-introduced defects

- **V7 · Important · D8-draw-before-code (S4 repair) against the S8 frame stamp.** The repair has "implement's checkpoint and landing each call the store skill's regeneration" for every run that wrote a product baseline, structural or not. That is a new implement duty: today implement's rules mention regeneration only inside `impl.store-landing`, which fires on "a built structural change". The S8 stamp fixes implement's exception at "the nine touches listed under S8", so this tenth touch needs the user's word, `brainstorm.frame-changes-users-word`. Exact repair: extend the S8 stamp by one line, on the user's word, naming the regeneration call at implement's checkpoint and landing.
- **V8 · Minor · the *Readers* default (record line 1404).** It still has agents read "the diagram file — text, `Draws:` line, stale flag". The S4 repair says "a diagram file carries no flag". Exact repair: "the diagram file — text and `Draws:` line — with the health view's stale list".
- **V9 · Minor · D8-draw-before-code, accepted risk.** It still records amending `architecture-design-primitive` D5-plan-run-artifact's "the drawing the user signs" because "the signed surface becomes the store's diagram files". After the V1 repair, the signed object is the run's `architecture.md` again. Exact repair: restate the supersession's scope as a change to the drawing's content (a neighborhood view plus a table of the edited store diagram files), not to the signed surface.
- **V10 · Minor · the record's new `## Verify pass` section.** It cites two decision numbers and one governance ID without their slugs (record lines 1675–1676). `mochiko-cli ids --check` on the record went from round 1's "0 bare" to "3 bare". Exact repair: join them as D5-visit-file-input, D9-render-where-viewed and GI-019-kernel-tooling-admission.

### Verify notes

- Blocking: V7, which needs the user's one-line stamp extension. V4 (Minor NOT CLEAN) goes to the user under `brainstorm.fix-one-card-verify-once`. V8 to V10 are mechanical repairs.
- Outside this round's bound and not graded: the S4 compare reads "entries and headings changed since the last regeneration", but nothing records where the last regeneration happened. Round 1's "touched entries" had the same gap. It is listed here so it is not lost.
- No Explore spawn was used, and the index was not opened.
