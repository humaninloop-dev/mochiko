---
report: review
stage: cold-read
skill: mochiko:review-brainstorm
seat: review-seat (devils-advocate, cold, solo, map drawn at front)
reviewed: .mochiko/brainstorms/implement-brainstorm-led/record.md (frozen 2026-10-11, 918 lines)
read_also: [reports/angle-map.md, research/second-list-engine.md]
date: 2026-10-11
recommended_status: needs-revision
raised: 18
survived: 15
critical: 0
important: 8
minor: 7
coverage_findings: 2
cross_exam: none sent; every finding rests on the frozen record and repo files read first-hand
fence: index never opened; ~/.claude/projects never read; no session artifact read beyond the three named above
---

## Failure narrative

Status `needs-revision`. Nothing Critical: no load-bearing claim is broken, every decision has an
owner, and the record is thick enough to review. Eight Important survivors block `ready`. Three
are contradictions a builder cannot build around: D8 lets seats add to drafts after drafting,
while D1 and the fail default forbid it (S2); the kept Common Mistakes rows disagree with the
engine inside one skill once the precedence clauses leave (S3); and D2 adopts the road it rejects
(S4). Two are decisions that reach past their facts: D6 grows the run outside the sufficiency check
and the spec (S1), and D7 has the lead frame the technical pieces the design seats own (S8). The
other three are builder asks: how unchanged callers pick a mode (S5), the missing supersessions
(S6, coverage), and no build-time proof that specify and brainstorm do not weaken (S7).

## Survivors

Each survivor gives its severity, the decisions it touches, the scenario or cited contradiction,
and a resolution path. `[coverage]` marks a diff against the blind map.

**S1 · Important · D6-outside-need-asked (and the *Sizing and skips* default).** A user-added gap
skips every check the original gaps passed. D6 says the gap list "grows by one, marked as
user-added in `sufficiency-report.md`", and the design review then grades against the grown list.
Scenario, using the record's own Q6 example: a Refund table comes up mid-stage and the user says
"this run".
- The report is the sufficiency seat's binding verdict (`impl.sufficiency-report`), and that seat
  "will not design or build this batch" (`impl.seat-sufficiency-independence`). The record does
  not say who writes the mark. If the lead writes it, the grader's file gets a second writer.
- The new gap's clauses are never graded. Yet `review-sufficiency.every-applicable-clause`
  (floor) says "a clause that cannot be graded is a gap, never a pass".
- If refunds are new behaviour, `spec.md` has no story or acceptance criteria for them. Cards need
  "acceptance criteria by ID" (`impl.card-contents`), and final validation's gap-finder works from
  the spec. So the run designs something nothing downstream can verify.
- The *No size gate* default lets the stage run on lane runs, and D6 has no lane exception.
  `impl.lane-never-widens` is a floor.
- The second list named a road the user never saw: "A candidate outside the gap list is scope
  escalation to the user, never a widened gap" (`research/second-list-engine.md`, *How implement
  plugs in*). Q6's trail shows no blind list and no difference put to the user
  (`brainstorm.blind-second-list`).

Resolution: put Q6 again with that road's case. A need that is new behaviour goes to specify, or
later. A design-only need inside an existing story is re-graded by the sufficiency seat before it
joins, so the report keeps one writer. Lane runs never add.

**S2 · Important · D1-engine-asking-contract, D8-checkpoint-shows-landing, the *Implement — the
fail* default.** Three texts disagree about seat additions.
- D1: "anything off the set goes back to the user before it is written".
- The fail default: "A design-phase write with no ruled set behind it, or outside it, fails the
  run."
- D8: the landing table shows "every addition the seats made on their own, each needing the
  user's OK", and each ruled item's status is "kept, or changed with why". That approval comes at
  the checkpoint, after drafting.

Scenario: the technical-analyst adds an `audited_at` field the user never ruled. Under D1 it
should have gone to the user first. Under the fail, the run has already failed. Under D8, it waits
for an OK at the checkpoint. D8 also gives the meaning-kept grade to "the design review pair", but
neither `review-plan-artifacts` nor `review-feasibility` carries that clause today, and the record
does not say which one gains it.

Resolution: one rule for additions. Either they halt to the user through the lead before they are
written, or the checkpoint signature cures them and the fail narrows to "unsigned at the
checkpoint". Also name the skill that grades meaning kept.

**S3 · Important · D2-iterative-becomes-engine, the *Precedence* and *What analysis-iterative
keeps* defaults.** Two texts that disagree end up inside one skill with no precedence between
them.
- The defaults keep "the common-mistakes list" in the skill that becomes the engine, and remove
  `brainstorm.own-rules-win` and `spec.own-rules-win`.
- `.mochiko/strips/analysis-iterative.md` lines 49–78 record why those rows were superseded for
  brainstorm (`brainstorm-target-state` D19-brainstorm-only-reach): "One question per turn —
  always", where brainstorm batches defaults, and "always-structured options listed as a mistake",
  where the options rule fixes the form.
- The engine now owns both the defaults batch ("confirmed on one screen") and the options rule,
  beside the kept row "Multiple questions per turn … One question per turn — always".

Scenario: at implement's defaults batch, the lead reads the engine's own anti-pattern table
telling it never to put more than one question per turn.

Resolution: rule how each row stands against the engine's mechanics: reworded, scoped to the
probe-only callers, or yielding by a stated clause. Record it as a supersession.

**S4 · Important · D2-iterative-becomes-engine.** As amended at Q4, D2 adopts a road its own card
still rejects. The card's rejected road reads: "(B) as first put, with setup and specify's intent
still reading the skill — every engine edit reaches them, and floors reach them whatever a mode
setting says (F23)". After Q4, setup and specify's intent do still read the skill (D4, frame
change 2).
- The accepted risk covers the contract not binding them. It does not cover engine floors being
  delivered into setup's interrogation, which is out of scope.
- Q4's case against was only "two commands keep their own asking for a while, a second build
  later". So the user's "as recommended" was given without the reason the lead had used at Q2.
- The card title still says the dependencies "move out".
- The *Change control* default audits only "the four users' renders", not the two probe-only
  callers.

Resolution: put the floor-delivery exposure to the user plainly for one confirm. `when:` gates
whether a floor applies, never whether it is delivered. Then fix the card's rejected road, its
title, and the audit scope.

**S5 · Important · the *Probe-only callers*, *Slots declared where* defaults; D4-shape-six-build-four.**
Builder ask: how does a caller whose pointer is unchanged select probe-only mode? The default
gives setup and specify's intent "a caller setting that gates the stage rules and the contract off
for them". D4 says "until then setup's and specify-intent's pointers stay as they are".
- The precedent condition, `review-brainstorm`'s `lens`, is "entry-derived" from a spawn brief.
  These callers invoke inline, with no brief.
- A `common.*` stub inherits `text` only (`.claude/rules/mochiko/primitive-edits.md:215-219`).
  So four users cannot set different modes from one shared pointer line without a var (the
  `${explore_model}` precedent, `0001-genesis.yaml:9793`) or local text.

Resolution: rule the condition's default value (probe-only when the caller names none) and how
each of the four users names its own mode. Neither should touch setup's rules.

**S6 · Important · [coverage — blind map J3, taken but not answered in depth] · D2, D3-all-four-move-now,
the *The build* default.** The supersessions this record owes are only partly named. The record
names F18, specify's D5-specify-own-rules with its rejected road, and the two own-rules-win
clauses. It does not name:
- (a) `architecture-brainstorm-interview`'s confirmed default *Where the rules live* (that record,
  line 1375): the desk's own schema, plus a precedence clause that the desk's rules win over
  `analysis-iterative`. *The build* replaces it with "item A's focus stage written straight onto
  the engine".
- (b) `brainstorm-target-state` D19-brainstorm-only-reach. Brainstorm's mechanics now reach three
  more commands.
- (c) The protected `[v0.25.0] KEPT:` entry and the v0.63.0 keep-set that protect `## Two output
  shapes, one engine` as "the caller-facing dispatch seam" (`.mochiko/strips/analysis-iterative.md:93,123`).
  D2 strips that section.
- (d) D6-desk-stays-engine's hand-off line and the BACKLOG trigger the user overrode.

The decision map after D8 claims J3 is "carried by a named default". None of the defaults names
these. GI-005: protected content leaves only by recorded ruling. Item (a) plausibly changes a
ruling, since the desk chose to keep its rules winning.

Resolution: one supersession list on the accept screen, (a) put to the user on its own.

**S7 · Important · D3-all-four-move-now, frame *Must not break*.** No build-time check proves the
moved stages do not weaken. The frame promises "specify's story stage as built at v0.119.0; no
command's ruled behaviour weakens". The only measures named are "ships once" (no duplicated text)
and dogfood counts after the build.
- Scenario: `spec.story-forks` requires "a one-row mock of the story as it would read". The engine
  says "a rough mock inside the message". If specify's domain part does not restate the one-row
  form, specify has weakened, and nothing at the build sees it.
- The second list's point 3, a render-diff partition as in the near-dup wave, was never put.
- Its point 4, whether the pointer stub is `class: floor` or `must`, was never ruled.

Resolution: rule the check. Every pre-move obligation of brainstorm and specify maps to a
post-move rule, and the user rules each difference. Also rule the stub's class.

**S8 · Important · D7-direction-then-pieces.** Unchallenged assumption plus a rejected-road
steelman. Under D7 the lead "breaks it into its pieces — for a data gap the fields, states and
rules; for an API gap the inputs, outputs and errors … suggests each". The design is then framed
by the lead, not by the personas whose skills own it (`patterns-entity-modeling`,
`patterns-api-contracts`, `patterns-system-design`; CLAUDE.md axis 4).
- Scenario: the lead suggests `status` as free text and the user ratifies. The technical-analyst
  would model a state machine, but under D1 and D8 that is a change needing the user's OK at the
  checkpoint. The seat's expertise arrives only as a correction.
- The desk's own unit stops at direction (D2-part-direction-know). D7 goes down to field level.
- Never considered: the design seat drafts candidate pieces read-only, in its existing plan-only
  dispatch (`impl.plan-approval-producers`), and the lead puts them to the user as forks.

Resolution: put that road to the user, or bound the lead's suggestions to what the user would
notice and leave the pieces to the seat.

**S9 · Minor · [coverage — blind map D2, depth] · D5-stage-after-runopen.** On an epic the design
phase "always fires, for the joint spine … joint architecture and seam design, and batch ordering"
(`impl.epic-design-always-fires`). D6, however, fixes the stage's agenda to "the gap list". Seams
and ordering are not gaps, so the epic's largest design is drafted by seats with no stage: the
problem line's own shape. The delta scope (per-card clauses) is also unstated. Resolution: say
whether spine items join the agenda.

**S10 · Minor · D4-shape-six-build-four, frame change 2.** This is an unverifiable claim. The
destination now promises the engine "shaped so setup, feature and specify's intent stage can join
it later without reopening it". D4's own Why says the feature desk's "unit and set home are new
design work (F29)", and D4's slot list is the same as D1's. Nothing the build does can show
"shaped for six". Resolution: soften the promise to "slots named; fit checked at the follow-up",
or mark the feature half `Assumed`.

**S11 · Minor · D3, D4 headings.** Two slugs break the joined-ID rule. `D3-all-four-move-now` and
`D4-shape-six-build-four` carry four slug words. `templates/artifact-format.md` IDs: "exactly
three lowercase ASCII words … a hyphen run that is not three such words is not a slug. The
three-word count binds new mints." Resolution: re-slug both before landing, with `ids rename`.

**S12 · Minor · Q1 facts (record line 316).** The reading "implement's stage needs a new section"
(from F17) collides with the command six-set: `primitive-edits.md` criterion 2, "all six present
in every schema". A seventh section fails the audit. The ceiling is per section render, and the
binding rules fit an existing small section (`impl.sec.ways-of-working`, 9 rules). Resolution:
correct the reading so the builder does not follow it.

**S13 · Minor · D1 rejected road D10.** The road is rejected because "a subagent cannot ask the
user". That holds for subagents, but a facilitator seat could be a teammate. Verified at
https://code.claude.com/docs/en/agent-teams: "You can also talk to any teammate directly without
going through the lead." The sound reason for implement is on the same page: "No session
resumption with in-process teammates: `/resume` and `/rewind` do not restore in-process
teammates". Implement runs stop and resume (`impl.gate-design-checkpoint`). Resolution: correct
the reason. The decision stands. This survives pending the lead's cold re-read of the cited source
(EXTERNAL-CLAIMS source re-read clause).

**S14 · Minor · Q6–Q8.** The blind second list is missing and the gap is not disclosed. Q6 (run
scope), Q7 (the unit) and Q8 (a floor's presentation) are forks costly to undo. None got a list,
and the record neither says they were judged cheap nor discloses the skip. The reuse at Q2 and Q5
was disclosed. Resolution: disclose in the run notes. Q6 is covered in S1.

**S15 · Minor · the record as a build input.** It has no touch set. Specify's D5 listed one. Two
gate consequences go unnamed:
- `evals/contract/README.md:370` lists `analysis-iterative` among the prose skills that "need no
  stub" (gate 6).
- `evals/plan/brainstorm/observable.yaml:103` observes `brainstorm.own-rules-win`, which leaves.

Resolution: one touch-set line at landing, naming `review-plan-artifacts` (S2), the sufficiency
report's writer (S1), the new home `design-stage.md`, and the router and description edits.

Tally: 18 raised, 15 survived.

## Dismissed

- The heredoc channel breach (lead disclosure 1): the content was checked. `mochiko-cli check` on
  the record returns allow, and there are no `$` characters or empty code spans, so no sign of
  expansion damage. The disclosure stands as written. Commentary.
- The lead holds implement's conversation in a long run (blind map F1): specify's
  D3-lead-holds-stage is `Contested`. The new angle, the long resumable run, resolves in favour of
  the lead (S13's verified resume limitation). It is not raised.
- F27's grep wording: the cited line 8455 matches "enrichment of a sparse idea", not the quoted
  pattern. The claim's direction (no live caller) holds either way.

## Card parts and fitness

- Card parts: all eight cards carry statement · why · rejected roads · accepted risk · how it was
  decided · changed at review (all eight read "—"). Headings are in `### D<n>-<slug> — <name>`
  form. The slug defects are in S11.
- Self-contained: yes. Orientation F1–F9, per-fork facts and a Q-trail per decision (lines 305–815).
- Decisions attackable: yes. Each Why cites a fact or the user's word (e.g. D5 cites `implement.md:78`).
- Decision trail present: yes. Rulings quoted with streak counts; untouched cards show "—".
- Confidence marks honest: mostly. D2 is `Contested` correctly and OQ1 carries `Assumed`. D4's
  feature half is `Confident` on an unverifiable claim (S10).
- Rejected roads recorded: present, but D2's rejected road contradicts its amended statement (S4).
- Honest about the open: OQ1–OQ3 and five untested bets (lines 880–901). There is no build-time
  check for "no weakening" (S7).
- Provenance stated: the status line, seats and tiers in the run notes, and the freeze disclosure
  (lines 3, 149–160, 912–917).

## Builder test

Reading only the record, a builder would still have to ask:
- who writes the user-added mark, and whether that gap is graded (S1);
- what happens to a seat addition (S2);
- which kept anti-pattern rows yield (S3);
- how unchanged callers pick a mode (S5);
- the full supersession list (S6);
- how "no weakening" is proven (S7);
- whether epic spine items enter the stage (S9);
- where implement's new rules sit (S12);
- which review skill grades meaning kept (S2).

## Coverage diff

- Front map: 45 angles, 33 load-bearing. Thirty-one load-bearing angles were taken in D1–D8 or a
  named default and read in depth.
- Two were taken but answered thin: J3 (S6) and D2's epic half (S9).
- Dropped F5, G3, H3 and J5 were checked: each drop gives a reason and stands as a ruling. The
  build consequences of H3 and J5 are listed in S15.
- I2's lane half is folded into S1.

## Notes of note

- Tiering: the haiku `Explore` locate route failed for this seat at the map pass, and every
  attempt in the last build failed too (BACKLOG seat-transport finding). This pass ran its locate
  greps on the seat tier. Disclosed.
- External: one claim was live-checked (S13), with quotable text cited. The lead's cold re-read is
  owed before S13 survives.
- The second list was read in full. Its two unput points (precedence handled; partition and stub
  class not ruled) are in S7.

## Verify round 1

Pass: verify, solo, 2026-10-11. I read only the changed parts and their gists, as the brief lists
them:
- the cold-review section and its 15/15 dispositions table (record lines 1088–1138);
- the changed parts of D1–D8;
- frame changes 2 and 3;
- Q1's corrected reading and the run-notes disclosure;
- the two new sections and the changed defaults.

There was no fresh cold read and no new coverage hunt. The locate greps (the old-slug sweep and the
cite anchors) ran on the seat tier, because the haiku `Explore` route already failed for this seat
this session. Disclosed.

Result: all 15 survivors are CLEAN. Round 1 is still NOT CLEAN, on six defects the fixes
introduced: one Important (V1) and five Minor repairs (V2–V6).

### Per survivor

- **S1 CLEAN.** D6's statement (780–786) now splits outside needs by kind:
  - new behaviour is never added to the run; it goes to specify, or later;
  - a design piece inside an existing story is re-graded by the sufficiency seat, and "the report
    keeps its one writer";
  - "A lane run never adds".

  The second list's road was shown at the re-put (806–807). Nit: the *Sizing and skips* default
  still reads "No size gate" with no lane pointer; D6's changed-at-review line carries the
  narrowing.
- **S2 CLEAN.** The three texts now agree. D8 (889–893) splits additions by size and names
  `review-plan-artifacts` as the meaning-kept grader. D1's changed-at-review line (395–398)
  narrows "before it is written" to rulings and noticeable changes. The fail default (1029–1032)
  is narrowed to match.
- **S3 CLEAN.** *What analysis-iterative keeps* (1008–1013) rewords both conflicting rows against
  the engine and records it as a supersession. V5 names the list this supersession is missing
  from.
- **S4 CLEAN.**
  - D2's title now reads "its setup- and specify-shaped content moves out".
  - The rejected road that contradicted the card is replaced by the dropped-switch road (512–515).
  - The floor exposure was put plainly and ruled by the user (538–549).
  - *Change control* now audits "every caller's render" (989–991).
  - Frame change 3 is stamped.

  The widening creates V1 and V4.
- **S5 CLEAN.** *Slots declared where* (995–999): the contract switch defaults to off. The three
  contract users name it on in their own rule text, "(a stub inherits text only, so the mode never
  rides the shared line)".
- **S6 CLEAN.** *Supersessions owed at the build* (922–943) names all four missing items:
  - the desk's *Where the rules live*, flipped by the user's word;
  - D19 with `brainstorm.own-rules-win`;
  - the KEPT and keep-set entry;
  - D6's hand-off line and the BACKLOG trigger.
- **S7 CLEAN.** D3's statement (576–582) adds a before-and-after obligation map, with every
  mismatch going to the user before the bump. The pointer is `class: floor`, and the grid is held
  in reserve.
- **S8 CLEAN.** D7's statement (826–830): the design seat that owns the surface proposes the pieces
  read-only in its plan-only dispatch, and the lead puts them to the user. This creates V2.
- **S9 CLEAN.** D5 (722–724): on an epic, the joint spine's seams, joint architecture and batch
  ordering join the agenda. This creates V3.
- **S10 CLEAN.** Frame change 2's destination is softened (648–651), and D4 marks feature's fit
  `Assumed` (687).
- **S11 CLEAN.** `D3-four-move-now` and `D4-shaped-for-six` have three slug words each. Inside the
  record the old slugs survive only in the two re-slug history lines (604, 698). V6 covers the old
  slugs in this report.
- **S12 CLEAN.** Q1's reading is corrected (321–325): the rules spread across the existing six
  sections, never a seventh.
- **S13 CLEAN.** D1's rejected road D10 (377–383) carries both quoted lines, and the lead's cold
  re-read is recorded (1096–1100). Nit: Q1's trail (334–335) keeps the old reason as history,
  unmarked.
- **S14 CLEAN.** The run notes (161–165) name Q6–Q8 as a miss.
- **S15 CLEAN.** *Touch set for the build* (945–972) lists the primitives, `review-plan-artifacts`,
  `review-sufficiency`, the declared home and both gate files. V5 names an item it misses.

### Fold-introduced defects

- **V1 · Important · frame change 3, D2 and D4 as changed, *Slots declared where*.** Nothing says
  what a caller that fills no slot gets.
  - The fixes: setup's interrogation and specify's intent stage now "follow the engine's asking
    half". They "name nothing and need no edit", and their "own rules and content are not edited"
    (663).
  - D1's asking half includes "a fenced blind second list from a fresh seat" and "a provenance mark
    per item" (353–354).
  - D4's slots include "the blind list's cadence and job" (670–671).

  So it is open whether setup's interrogation now spawns a blind second list, at what cadence and
  with what inputs. It is also open where its provenance marks land, since `governance-intent.md`
  has no field for them. D3's obligation map lists obligations *lost*, not *gained*, so it will not
  catch an added one. Resolution: rule which asking mechanics are slot-free, and each slot's value
  when a caller names none (for example, no blind list unless the caller's own rules name one).
  The obligation map should also list what each caller gains.
- **V2 · Minor · D5 against D7.** D5 places the stage "before any design seat's plan-only
  dispatch" (720–721; its title says "before any design seat plans"). D7, as changed, runs the
  seat's read-only proposal "riding its existing plan-only dispatch" inside the stage. Resolution:
  D5 reads "before any design seat's plan is finished and graded".
- **V3 · Minor · D6 against D5.** D6 still opens "The stage's agenda is the gap list the
  sufficiency check graded" (778), while D5 now adds the joint spine's design items on an epic.
  Resolution: D6's first sentence adds "on an epic, plus the spine items (D5-stage-after-runopen)".
- **V4 · Minor · lines left stale by frame change 3.**
  - OQ2 (1053–1055) still says setup and specify's intent "join the engine" at the follow-up. They
    follow the asking half from this build; the follow-up rules only the contract half (D4, frame
    change 3).
  - The decision map after D8 (978) still reads "Out of scope: as frame change 2".
  - D4's rejected road (i) gives "setup … moved untested" as its reason. That now applies to the
    asking half D4 accepts (687–688). Narrow the reason to the full stage and contract.
- **V5 · Minor · the supersession list and the touch set.**
  - S3's reword of the Common Mistakes rows supersedes protected content. The v0.63.0 keep-set
    names `## Common Mistakes`, and the [v0.117.0] strip entry keeps "every row byte for byte, for
    setup and every other caller" (`.mochiko/strips/analysis-iterative.md` lines 49–78). *Supersessions
    owed at the build* omits it.
  - The touch set's `analysis-iterative` line omits `## When NOT to Use`'s "**Implementation
    details** — use planning skills instead" (F21, record lines 446–447). The engine's new use in
    implement's design phase contradicts that line.
- **V6 · Minor · this report.** The cold-read section above cites the slugs from before the rename
  (S5, S6, S7, S10, S11), as they stood at the cold read. I left them as written. If the `ids`
  check reads reports, an `ids rename` sweep at landing covers them.

Recommended status after round 1: `needs-revision` until V1 is ruled; V2–V6 are lead repairs. With
V1 ruled and V2–V6 repaired, nothing this pass found blocks `ready`. The next step is the lead's,
per `brainstorm.fix-one-card-verify-once`: a second failed verify goes to the user.

## Verify round 2

Pass: verify, solo, 2026-10-11. I read only the spots the brief names:
- the record's *Verify pass › Round 1* (lines 1166–1196);
- D1's V1 line (404–408);
- the *Slots declared where* and *Setup and specify's intent* defaults (1009–1032);
- D3's statement (578–592);
- D5's and D6's first statement sentences (729–733, 789–791);
- OQ2 (1078–1081), the decision-map line (994), and D4's rejected road (i) (690–692);
- the analysis-iterative bullets in the supersession list and the touch set (948–951, 963–968);
- the two nit spots (1057–1059, 334–336).

Locate reads ran on the seat tier, disclosed as in round 1. I also ran `mochiko-cli ids --check`
(advisory) over the record and this report.

Result: five of V1–V6 are CLEAN. V2 is NOT CLEAN on its title only (Minor). One Minor wording
defect came in with the V1 fold (W1).

### Per item

- **V1 CLEAN.** The user's ruling is recorded in the trail (1189–1194) and landed in four places
  that agree:
  - D1's changed-at-review: "one default for every slot, inherited by every caller with no
    per-caller carve-out; a command customises only in its own rules";
  - *Slots declared where* names the three defaults: contract off, the blind list "on a fork costly
    to undo", provenance "kept in the conversation where that output has no place for them (setup's
    `governance-intent.md`)";
  - the setup and intent default inherits them;
  - D3's obligation map now sends "every obligation a caller gains from the engine's defaults" to
    the user.
- **V2 NOT CLEAN · Minor.** The statement is fixed (729–733): the stage runs "before any design
  seat drafts", and the proposal rides the plan-only dispatch inside it. The card's title (727)
  still reads "before any design seat plans", which is the placement the statement now
  contradicts. Fix: retitle the card "… before any design seat drafts …". The slug is unchanged.
- **V3 CLEAN.** D6 opens "the gap list … — on an epic, with the joint spine's design items beside
  it (D5-stage-after-runopen)" (789–790).
- **V4 CLEAN.** All three stale spots are updated:
  - OQ2 (1078–1081) names feature's join and "whether setup's interrogation and specify's intent
    stage take the contract half";
  - the decision-map line reads "as frame change 3" (994);
  - D4's rejected road (i) is narrowed to "six full users now", with setup's asking change noted
    (690–692).
- **V5 CLEAN.** The supersession bullet adds the keep-set's anti-pattern rows. I checked its cite
  at strip line 89, which reads "**Kept deliberately (the guardrails keep-set):** … the
  anti-patterns". The touch set rewrites the F21 *When NOT to Use* line (965–967).
- **V6 CLEAN, with a note.** No change. `mochiko-cli ids --check` over this report shows 0 drift on
  the old D3 and D4 slugs, and the record maps old to new at lines 604 and 698. The stated basis
  covers less than claimed: `human-readable-ids` D15's quote rule covers text inside quotes and
  code spans, while the S5, S6, S7 and S10 headings cite the old slugs bare. The outcome is
  harmless.
- **Nits CLEAN.** The *Sizing and skips* default gains "A lane run never widens
  (D6-outside-need-asked …)" (1059). Q1's trail marks the D10 reason "corrected at review, S13"
  (334–336).

### Introduced by the repairs

- **W1 · Minor · the *Slots declared where* default.** "specify sets once per run" can be read as
  covering the whole specify command, intent stage included. The setup and intent default gives
  that same intent stage "the blind list on a costly fork". The once-per-run list is the story
  stage's (`spec.blind-journey-list`, "One list per spec run"). Fix: "specify's story stage sets
  once per run".

### Own disclosure

- This report's cold-read section carries one bare durable ID: `GI-005` at line 144 (the joined
  form is `GI-005-record-layer-integrity`), flagged by the advisory check. It is my defect; I leave
  it for the lead's word or the landing's `ids` sweep.
- The advisory check over the record also lists my angle map's class-D labels (`bm D2`…) and the
  second list's road labels (D1–D10) as bare session IDs, plus cross-record decision cites as
  drift. These predate the fixes and are outside this bound. A later map should not use `D` as a
  class letter.

### Final recommended status

`ready`, once the two one-line wording repairs land: V2's title and W1. Both are transcription that
needs no ruling and no further verify. Evidence for `ready`:
- all eight Important survivors closed in the cards with evidence (rounds 1 and 2);
- the one fold defect the user had to rule (V1) is ruled and landed consistently;
- the fitness items over the changed cards still hold: every changed card adds its
  changed-at-review line, D2 stays `Contested`, and D4 marks feature's fit `Assumed`.

Strictly, round 2 is NOT CLEAN on V2's title. Under `brainstorm.fix-one-card-verify-once` that goes
to the user, and the clearing verdict is the lead's.
