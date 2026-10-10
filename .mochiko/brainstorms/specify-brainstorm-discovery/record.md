# Specify reimagined around brainstorm's discovery — decision record

Status: **accepted** 2026-10-10 (user: "accepte", after one re-ask of a vaguer word) · cold read `needs-revision` (13 survivors, 13/13 dispositioned by the user: "as recommded, all inline"), verify round 1 NOT CLEAN (6 fold-introduced Minor, lead-repaired), verify round 2 **`ready`** (three cite-format residuals fixed on sight) · **built** 2026-10-10 at v0.119.0 (branch `specify-story-stage`; `build-log.md` · `wave1-story-stage.md` · `reports/audit.md`) · frozen for cold review 2026-10-10 after the accept screen ("as recommded") · opened 2026-10-10 · lead: Claude (session) · rulings: deepeshBodh

## Topic (user's words)

> based on the context from dogfooding project, ai-fileops, i want to reimagine the specify workflow. I want the specify workflow to leverage what brainstorm does to find out what the user wants.

## Orientation

- Session index read 2026-10-10 (`.mochiko/brainstorms/index.md`). No earlier session on specify's
  questioning. The nearest prior ruling: `brainstorm-target-state` D19-brainstorm-only-reach
  ("The target applies to `/mochiko:brainstorm` alone … any other command that might follow is a
  separate concern, out of this session's scope", `.mochiko/brainstorms/brainstorm-target-state/record.md`
  lines 559–562). This session is a separate concern of that kind, for specify.
- `brainstorm-target-state` F12 (same record, lines 149–154): "No rule in `specify`'s render names the
  skill or a blind map; `skills/review-specifications/SKILL.md` and the router skill mention the
  skill in prose."
- The dogfood repo the topic names: `../ai-fileops` (read 2026-10-10). Two commits
  (`9acddc6 Add ai-fileops shared-plugin design record`, `49bb763 Add governance and operating docs
  scaffold`); one accepted brainstorm (`ai-fileops-shared-plugin`, D1–D9 + DF1–DF12, accepted
  2026-10-09); one setup run (governance v1.0.0, depth `low`); `FEATURES.md` has no rows; no
  `.mochiko/specs/` and no `.mochiko/features/` directory.
- From that record (`../ai-fileops/.mochiko/brainstorms/ai-fileops-shared-plugin/record.md`), three
  places where the user's want only surfaced through the session's questioning:
  - Line 59 (F2): "I dont think we need to fixate on kinako deeply … I was imagining a rich text
    editor like notion. using https://github.com/udecode/plate" — "Contested against the lead's lean
    to block-level source editing".
  - Line 58 (F3): "This opens a really good dimenion. i want git backend source control on files" —
    "a scope addition by the user's word".
  - Line 57 (F4): "what i think i mean is, git is central to how the changes are managed" — the
    answer to an intent probe.
  Lead's reading (marked as reading): in ai-fileops the brainstorm, not a spec, found what the user
  wanted, and two of the three wants contradicted or widened the lead's own frame.
- Artifact home rendered 2026-10-10 (`mochiko-cli home --plugin-root plugins/mochiko
  .mochiko/brainstorms/specify-brainstorm-discovery`): closed set `record.md`, `synthesis.md`,
  `build-log.md`, `wave<n>-<slug>.md`; sub-directories `inputs`, `research`, `referents`; reports
  under `reports/`. (The bare `mochiko-cli home <path>` form failed: "the migration log cannot be
  read: No such file or directory"; it needs `--plugin-root`.)
- Knowledge-management file present (`.mochiko/memory/knowledge-management.md`): the close ritual
  applies.

## Facts from the user

- F1 (user, 2026-10-10, first reply to the frame card): "i think the problem what you have mentioned in
  not specific to dogfood project, i feel the specify workflow requires tighter mapping to understand
  what the user wants, currently it takes what user gives and goes ahead and build a lot of user
  stories which may or may not be related to what user wanted. doesnt mean they are wrong, just not
  tight to what user may want. specify should be like brainstorm session for user stories" — the
  Problem line's dogfood framing struck; the problem is specify's own: stories are built in volume
  from what the user gave, loosely tied to what the user wants. Lead's reading: the want is not
  "specify asks more questions" but "the stories themselves are arrived at the way brainstorm
  arrives at decisions" — put to the user, ruled, tight.
- F2 (user, 2026-10-10, second reply, "staying with problem and destination"): "i think the analyst
  and the user need to work a lot tigthly in terms of expectation of what user stories will be
  worked on. the workflow needs to brainstomr with user before generating a large amoubnt of user
  stories" — two things named: the pair that must agree is the analyst seat and the user; what they
  agree on is the expectation of which stories will be worked on; and the agreement comes before
  the volume of stories is generated, not after. Lead's reading: the want is a story-level frame —
  which journeys, for whom, which ones matter and which are out — ruled with the user before
  authoring, and the analyst seat (not only the lead) in that conversation. *(Reading corrected by
  F3: "the analyst" in F2 meant the lead.)*
- F3 (user, 2026-10-10, at Q3): "i am confused, we use agent teams, not subagents" and then "when i
  said analyst and me, i meant the lead. the lead is the manager of the teammates." — the user's
  runs seat teammates, never subagents; and the pair that works tightly is the lead and the user.
  Lead's reading: F2's "analyst" was the lead wearing the analyst's craft; the harness facts F-H2
  (subagents) are moot for the user's runs, and F-H1/F-H3 (teammates) no longer gate the "who"
  question.

## Facts from specify's rendered rules (`mochiko-cli rules specify --section <id> --plugin-root plugins/mochiko`, 2026-10-10, binary 0.4.0 · plugin 0.118.0)

- F-S1 (`spec.intent-stage-first`, section `spec.sec.tools`, `pointer: mochiko:analysis-iterative`):
  "The intent stage runs before any authoring: the adaptive-probe agenda via mochiko:analysis-iterative
  — scope boundary · delivery intent · depth-rigor expectation · UX-bearing (does the feature carry a
  user-facing surface to prototype) · constraints · out-of-scope." Lead's reading: specify already
  runs the shared questioning skill — on an envelope agenda (scope, delivery, depth, UX, constraints),
  not on what the stories should be. This corrects `brainstorm-target-state` F12's "No rule in
  `specify`'s render names the skill" for the current render.
- F-S2 (`spec.intent-probe-discipline`, section `spec.sec.ways-of-working`): "Intent runs as probes,
  never a questionnaire; it closes in a one-screen synthesis the user confirms; flag ratification
  streaks before treating adoptions as engagement."
- F-S3 (`spec.capability-frame-at-intent`, section `spec.sec.roles`): the product-manager seat states
  the capability frame "as nouns + verbs only, never enumerating stories, agreed with the user."
- F-S4 (`spec.frame-hypothesis-not-anchor`, section `spec.sec.ways-of-working`): "The capability frame
  is a hypothesis, not an anchor: it dictates neither story boundaries nor journeys (stories stay
  journey-driven), and stories win any conflict with it".
- F-S5 (`spec.reserved-to-user`, section `spec.sec.reserved`): the user's reserved set is "the feature
  framing when `$ARGUMENTS` is empty · intent confirmation (the capability frame included) ·
  clarification answers only they can settle · clicking each story's prototype screens as they land
  (their reactions fold back into story and screen together) · ruling on an escalated filter
  disagreement." Lead's reading: no rule puts a story, or the set of stories, to the user as a choice
  before it is authored; the user meets stories after authoring, through the prototype click-through
  (UX-bearing only) and acceptance.
- F-S6 (`spec.lockstep-prototyping`, `when: ux_bearing=yes`): "each story's screens and flows land
  while that story is under discussion; the user clicks through while the story is wet, never a batch
  render after the text settles." Lead's reading: the one place specify already works story-by-story
  with the user is gated on a UX surface.
- F-S7 (`plugins/mochiko/skills/authoring-user-stories/SKILL.md`, lines 2–3): "This skill MUST be
  invoked when transforming a feature description into prioritized user stories — assigning P1/P2/P3
  priority levels, authoring Given/When/Then acceptance scenarios, and specifying an independent test
  for each story." Lead's reading: the story skill's input is "a feature description"; it transforms,
  it does not ask.
- F-S8 (`authoring-user-stories.story-structure`, rendered 2026-10-10): "Generate 2–5 user stories
  per feature, landing in `spec.md`". Lead's reading: with the capability frame naming several
  capabilities, the authored set can run to ten or more stories per run — the volume the user names.
- F-S9 (`mochiko-cli template spec --plugin-root plugins/mochiko`, 2026-10-10): "`## Intent` — 10
  lines" (line 117 of the render); "An undeclared `##` heading is denied." (line 115); "`## User
  Stories` — An INDEX only. Story content lives in per-story files: stories/US-*.md" (lines 25–27).
  Lead's reading: a ruled story slate has a home today in the User Stories index; a new section
  means a template migration. *(At review, S2: the index's column grammar — "ID (linked to
  stories/US-N.md), Story (one breath), Priority, Feature (FEAT-XXX-<slug> or —), Disposition (homed |
  rejected — why)" and the line "The only story-native status is `rejected`" — was quoted by the
  review seat from its own render; the lead had not read the columns first-hand. The build reads the
  template first-hand.)*
- F-S10 (`authoring-feature-map.frame-first`, rendered 2026-10-10): "Derivation is frame-first:
  before any story is drafted, the product frame names the capabilities the territory touches as a
  hypothesis — nouns and verbs only, never a list of stories". Lead's reading: with F-S3, two rules
  keep story lists out of the frame; a story slate before authoring must be declared as not the
  frame, or those lines are superseded.
- Note: the raw render could not be kept under `research/` by shell copy (the artifact-home hook
  denies shell writes of non-`.md` files there); the rule ids above are the citable source, and the
  render is reproducible from the command line given.

## Facts about the sibling session, the story-skill floor and the sound loop

- F-S11 (`authoring-user-stories.pm-frame-boundary`, `class: floor`, rendered 2026-10-10): "Authored
  inside the product-manager's frame — the PM owns which capabilities (features, the story filter,
  selection advice — map machinery: `mochiko:authoring-feature-map`); this craft owns how well the
  stories are written. Neither edits the other's verdicts." Lead's reading: "which stories" sits on
  the PM side of a floor; the user's F2 pairing (analyst + user on which stories) either reads the
  user as the one ruling "which" with the analyst sharpening, or moves the floor by ruling.
- F-S12 (`.mochiko/brainstorms/architecture-brainstorm-interview/record.md`, line 7, read 2026-10-10):
  "i want the architecture interview to be the brainstorm workflow for architecture. similar to how
  brainstorm works on an abstract problem, architecture workflow should be architecture". Its
  out-of-scope line (line 75) names "specify (the open sibling)". Lead's reading: three commands —
  brainstorm, specify, architecture — may carry the same way of thinking with the user; the plugin's
  3+ extraction bar for a common block (`.claude/rules/mochiko/primitive-edits.md`, criterion 11)
  becomes reachable, which bears on where the rules live.
- F-S13 (`patterns-sound-loop.exemptions-exactly-three`, rendered 2026-10-10): "Only three kinds of
  write never trip the trigger: mechanical execution of an existing ruling · transcription of user
  decisions · fix-on-sight integrity repairs". Lead's reading: the lead writing the ruled slate —
  the user's rulings, row by row — is transcription and trips no sound loop; the stories authored
  from it are judgment writes and keep the loop (a seat, plan-first, graded).

## Facts from the harness docs (`claude-code-guide` seat `harness-fact-c2`, 2026-10-10; quotes as returned, URLs the seat's)

- F-H1 (https://code.claude.com/docs/en/agent-teams): "You can also talk to any teammate directly
  without going through the lead." and, of the agent panel, "Enter: open the selected teammate's
  transcript and message it directly". Of permissions: "Teammate permission prompts appear in the
  lead session, so approve them there yourself."
- F-H2 (https://code.claude.com/docs/en/sub-agents): the tool filter "removes these tools, even
  when listed in the `tools` field:" — `AskUserQuestion` among them; "In an interactive session,
  only the top-level subagent's summary returns to you and the intermediate output stays out of
  your main conversation".
- F-H3 (https://code.claude.com/docs/en/tools-reference): AskUserQuestion — "Questions stay open
  until you answer them." Whether a teammate can call it, and where such a question surfaces, is
  not documented (seat's finding: "not documented").
- Lead's reading: a subagent cannot ask the user anything; a teammate can be messaged by the user
  directly, but a teammate putting a free-form question to the user and blocking on it is
  undocumented — the only documented live path is the user opening the teammate's transcript. So
  "the analyst seat works with the user" in a run is either lead-relayed messaging (message lane
  of the transport floor) or the lead holding the conversation inline with the analyst's craft;
  a teammate-held conversation would rest on undocumented behaviour and would need a probe before
  the build counts on it. *(Not live-checked by the review seat: not load-bearing after F3.)*

## Facts from the decisions layer

- F-D1 (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`, lines 36–40): "R1 — Convergence
  licensed. A family spanning 3+ commands with near-identical wording (same responsibility;
  differences confined to phrasing, illustration, or unit nouns) may converge to one
  `common.<slug>` block. This is a narrow widening of the ontology D8 extraction bar; D8's other
  limbs (per-command default, stub-carried binding, block = boilerplate never judgment) stand
  unchanged." Lead's reading: convergence is licensed, not obliged; a common block carries
  boilerplate, never judgment; and extracting one turns brainstorm's rules into stubs — an edit to
  brainstorm, out of this session's scope.
- Precedent: `brainstorm-target-state` D19-brainstorm-only-reach — brainstorm's asking rules live in
  brainstorm's own rules, not the shared questioning skill, with a precedence clause
  (`brainstorm.own-rules-win`); the skill's body was left as it is for setup.

## Frame card — hardened 2026-10-10 ("okay")

Problem corrected and Destination amended by the user's first two replies (F1, F2); Must not break,
Betting on, Out of scope and Size confirmed by "okay". The fact question put with the card (where
the user saw loose stories) was not answered — carried as unknown; a replay case, if one is needed,
is taken from the ai-fileops record instead.

| Line | Text | Source |
|---|---|---|
| Problem | `/mochiko:specify` takes what the user gives, confirms a one-screen intent synthesis about the envelope (scope, delivery, depth, UX, constraints — F-S1), then builds many user stories that may or may not be what the user wanted — not wrong, but not tight to the want (F1). The person with the problem is the founder running specify on their own products. Not specific to the dogfood project. | user's words (F1), rewritten by the lead against F-S1; dogfood framing struck by the user |
| Destination | Specify works "like a brainstorm session for user stories" (F1): the lead and the user "work a lot tightly in terms of expectation of what user stories will be worked on", and that brainstorm happens "before generating a large amount of user stories" (F2) — the story set is ruled with the user, not authored in volume from the intent. Session ends with a ruled target a build item can carry: what of brainstorm's way carries over to the story step, who sits in that conversation, what specify keeps, where the rules live. | user's words (F1, F2) + guess (the session's end). **Changed 2026-10-10 by the user's word (F3): "the analyst seat and the user" → "the lead and the user"; stamped at review (S10). Re-check against the decisions: D1–D7 were ruled with the lead as the conversation holder (D3-lead-holds-stage) — consistent.** |
| Must not break | specify's outputs and their downstream readers (the spec, the feature map rows, Screens & Flows, implement's sufficiency check); brainstorm's accepted target state (`brainstorm-target-state` D1–D23); setup's questioning; the non-waivable floors (author≠grader, sound loop, plan approval). | guess |
| Betting on | Brainstorm's discovery carries over to specify's narrower question space (one capability, its stories and acceptance criteria) without making specify much slower or heavier. | guess |
| Out of scope | Specifying any ai-fileops feature in this session; brainstorm's own rules; setup, plan, implement; the spec's content shape except where discovery changes it; specify's grading side except where discovery hands to it. | guess |
| Size | standard | proposed by the lead, ruled by the user ("okay") |

## Run notes

- Floor read-back — model tiering, 8: class-key-session-tier · seat-default-key · override-is-the-pin ·
  seat-deviation-bounds · persona-less-grader-pin · worker-return-is-a-claim · brief-obligation ·
  worker-seat-set-reserved.
- Floor read-back — transport floor, 11: message-lane-trigger · topology-lane-trigger ·
  neither-lane-waivable · composition-steer · single-writer-per-surface · mesh-hold ·
  content-pinned-supersession · quiesce-before-cold-grade · no-ritual-sends · fan-in-confirmation ·
  version-floor. Message lane fires (the review seat is resumed later with the record); topology lane
  does not (the lead is the sole writer of this record; seats return text, or write only their own
  report file). Claude Code 2.1.294 ≥ 2.1.224 (version floor).
- Session tier: the user switched the session model to Fable 5.1 after the frame card (the `/model`
  command, 2026-10-10). Lead on `claude-fable-5-1`.
- Seats (2026-10-10, at frame hardening):
  - Blind-map review seat `review-seat` — `mochiko:devils-advocate`, persona default tier (`opus`),
    no override; given the problem, destination and out-of-scope lines only, never this record or
    the index; invokes `mochiko:review-brainstorm` for the blind angle map; writes only
    `reports/angle-map.md`. Resumed with the record for the cold read (message two).
  - Blind second-list seat `second-list-shape` — persona-less `general-purpose`, spawned
    `model: opus` (disclosed: the ruled strong alias for an interpretive seat; not a grader); given the
    frame and the shape question only, never the lead's options or the record; returns text.
  - Harness fact seat `harness-fact-c2` — native `claude-code-guide`, one question (F-H1–F-H3).
  - Every brief carries the model-tiering routing line.
- Fan-in expected: 2 returns (blind map · second list on the shape fork). Second list arrived
  2026-10-10 (`research/second-list-shape.md`, six roads); blind map arrived 2026-10-10
  (`reports/angle-map.md`, 40 angles / 30 load-bearing / 7 classes, `record_contact: none`). Fan-in
  complete. Seat disclosures: two wildcard listings over every session's `reports/` directory touched
  this session's path at shell level, filtered before display, nothing shown — weighed by the lead as
  no contact (the record was not opened); one haiku Explore spawn returned no report and the same
  greps ran on the seat tier.
- Concurrent sibling session: `.mochiko/brainstorms/architecture-brainstorm-interview/` is open in
  another session of the user's at the same time (its index entry names this session as "the open
  sibling"). `.mochiko/brainstorms/index.md` is therefore a shared write surface across two sessions —
  a transport the floor says to cite only when in scope (`patterns-transport-floor.cross-session-cited-in-scope-only`);
  it is in scope here as a write-collision risk. Lead's discipline: this session edits only its own
  index entry, re-reads the index immediately before each write, and never rewrites the file whole.
  Both entries verified present 2026-10-10 (lines 11 and 17).
- Record edits: the artifact-home hook denied two shell writes (a raw render copy into `research/`;
  a `sed -i` on this file); every later edit went through Write/Edit. The review-fold restructure
  (S13) was one whole-file Write by the lead.

## Decision map — first shown with Q1 (2026-10-10)

**Askable now**
- Q1 — Where the story brainstorm sits in specify and what it rules (the shape fork; costly to undo;
  own turn; blind second list taken).

**In fog** (what each hangs on) — blind-map angles folded 2026-10-10, each marked `(blind map <id>)`
- Who holds the conversation with the user — the analyst seat (F2's words), the lead inline as
  brainstorm does, the PM, or a pair `(blind map C1)`; hangs on Q1 and on a harness fact still to
  find: whether a spawned seat can put a question to the user directly, or only through the lead
  `(blind map C2)`. Folded in: who owns the story set's "which" — a floor puts "which" on the PM side
  (F-S11), so the analyst owning it moves a floor by ruling `(blind map C4)`; a live conversational
  seat against plan-then-work and sound-loop leg 1 — is the lead writing a ruled set transcription or
  authorship `(blind map C5)`; the hand-off from the conversation holder to the story author, a lossy
  brief being a candidate root cause `(blind map C3)`.
- Which of brainstorm's mechanisms carry over, part by part with the cheaper shape named (options with
  the do-less option · fact before decision · show before asking · decision map · ratified-yes streak ·
  vague-answer re-ask · stop rule · blind second list · provenance mark per candidate) `(blind map B3,
  B1, B4, B6, B7)`; hangs on Q1.
- The unit the user rules — journeys, one-breath story lines, priorities, acceptance scenarios, or
  build-now — ruling titles only may push the volume one layer down into scenarios, FRs and SCs
  `(blind map B2)`; hangs on Q1.
- Where the ruled slate lives and how authoring is bound to it: the User Stories index exists (F-S9);
  a `kind: fail` rule for a set never ruled or a story authored outside it `(blind map F4)`; whether
  the author may add, split, merge or drop mid-authoring, routed back or barred `(blind map E1)`;
  whether anything grades stories against the ruled set — today nothing does `(blind map E2)`; the 2–5
  bound (F-S8) against a ruled count, and "feature" meaning the spec or each capability `(blind map E5)`;
  hangs on Q1.
- Fidelity against completeness — stories the user never names but the floor or the edge hunt
  requires (failure recovery, empty states, permissions): who surfaces them without re-inflating
  the set `(blind map B5)`; whether a floor-obligated story can be struck, and the waiver path
  `(blind map E4)`; hangs on Q1 and the binding item.
- The two "never a list of stories" lines (F-S3, F-S10) — carve the slate out as not-the-frame, or
  supersede — and the ordering against the capability frame: a set before the frame lets story
  shape anchor capabilities, a set inside it lets the frame anchor stories `(blind map D2)`; hangs
  on Q1.
- The gate count end to end — intent confirm, story ruling, lockstep clicks, filter escalations,
  selection, acceptance — which existing gate shrinks or merges `(blind map D4, D1)`; the PM filter
  after a ruled set — map-fit only, or ahead of the ruling `(blind map D5)`; hangs on Q1.
- Where struck and deferred candidates are recorded, with a named return path `(blind map E3)`;
  hangs on the binding item.
- An existing brainstorm record, or stories already in hand, as specify's input without re-asking
  (the ai-fileops case) `(blind map G2)`; likely a default, after Q1.
- A size or skip path — a decisive user wants confirmations and a fast wrap-up, an unsure one needs
  probes; keyed to what `(blind map G1)`; hangs on the mechanisms item.
- Acceptance scenarios — ruled with the user or authored after the slate; hangs on Q1 (see also B2).
- The UX-bearing lockstep loop (F-S6) beside the story brainstorm — the skeleton nav implies the set,
  and per-story clicks are a second ruling channel `(blind map D6)`; hangs on Q1.
- Where the rules live — specify's own rules, the shared questioning skill as a third output shape,
  the story skill, a new skill, or a common block; carrying brainstorm's way over without editing
  brainstorm's rules `(blind map F1, F2)`; with the sibling architecture session (F-S12) three
  commands may meet the 3+ extraction bar `(blind map F3)`; hangs on Q1 and the mechanisms item.
- A blind angle map inside specify; hangs on the mechanisms question.
- What "tight to the want" means, testably — every story traces to a stated want or a ratified
  inference; zero strikes at acceptance; a count band — and whether the harm is count, fit or
  ownership `(blind map A3, A1)`; the evidence base — no quoted case yet, so the diagnosis carries
  `Assumed` until a run is replayed `(blind map A4)`; what a dogfood run is judged on; last.

**Dropped from the map, listed for the end read** (not load-bearing per the seat): C6 seat tier of the
analyst (`sonnet`) in a judgment-heavy conversation · C7 the analyst persona's fill-the-gap posture ·
E6 mid-run changes to the set re-entering one at a time · F5 build cost of the landing · G3 pending
rows and stubs as prompts · G4 specs of fifteen or more stories · G5 headless testability of a
conversational stage. A2 (the cheaper shape: one content probe in the intent agenda) rides Q1 as the
second list's road 2. (The blind map's class letters A–G and its angle ids such as `D1`–`D6` are the
seat's map labels, not this record's decision ids; every `(blind map …)` tag names the map.)

**Out of scope** — brainstorm's own rules · setup, plan, implement · specifying ai-fileops features ·
spec shape and grading beyond the story-discovery touch.

## Decision map — reshown after D3-lead-holds-stage (2026-10-10)

**Decided.** D1-story-discovery-stage · D2-line-plus-test · D3-lead-holds-stage.

**Askable now**
- Q4 — A blind second list of journeys before the slate closes: yes, no, or on request (blind map
  B5; real choice — a seat per spec run).
- Q5 — Where the rules live (specify's own rules · a third output shape in the questioning skill ·
  a common block once three commands carry the way); hangs on Q4 only for its size.
- Q6 — Two confirmed screens (intent, slate) or one (blind map D1, D4).

**Defaults — named now, confirmed in one batch at wrap-up.** Named here by heading, each one line in
the Q4 message, with the offer "say pull X to open any default on its own turn"; the user pulled none
(cross-exam Q1, S7). The living text of every default, as confirmed and as amended at review, is the
section *Defaults batch — confirmed* below: mechanisms carried and dropped · slate home and binding ·
ordering against the capability frame · filter · floor-obligated stories · inputs already in hand ·
size and skip · hand-off and slate writing (added at review, S12) · UX-bearing work · acceptance
scenarios.

**In fog.** What "tight" means testably and what the dogfood run is judged on (blind map A1, A3, A4)
— last, after Q4–Q6.

**Out of scope.** Unchanged.

**End state (after Q7).** Nothing askable; the one fog item deferred by the user's word
(D7-dogfood-criteria-deferred); no open frame line; the blind map folded, its dropped angles listed;
the defaults batch confirmed at the accept screen.

## Question trail

### Q1 — Where does the story brainstorm sit, and what does it rule? (put 2026-10-10)

Lead's options, written before the second list was read: (A) a story-discovery stage after intent
confirm, run like a brainstorm, closing in a ruled slate that binds authoring · (B) the story
expectation folded into the intent screen · (C) story by story, each put before it is drafted · (D)
nothing new in specify — a brainstorm record as the input. Second list (`research/second-list-shape.md`):
six roads. Differences shown to the user: the seat added "change nothing — prune at the selection
card" and split "one core-journey probe in the intent agenda" from the slate; the seat's grounding
supplied F-S8–F-S10 and the fact that unselected stories land as pending map rows. Lead's pick: A;
cheaper shape: B. Before the ruling, three blind-map angles were shown to the user as bearing on Q1:
C4 (the "which" floor, F-S11), B1 (propose-then-prune repeats the failure), D3 ("worked on" means
spec content, not build-now). **User: "A, as recommended"** → D1-story-discovery-stage.

### Q2 — What unit does the user rule in the story stage? (put 2026-10-10)

Hangs on D1-story-discovery-stage only; unblocks the binding item (what authoring is bound to), the
acceptance-scenarios item and the fidelity-vs-completeness item. Options: (a) one-breath story lines
with priority — who, does what, why — scenarios, requirements and success criteria authored after,
inside the ruled lines (cheaper shape) · (b) the story line plus its independent test — the one
sentence that says how the user would know it works — edge scenarios authored after · (c) full story
content ruled live, every scenario a fork. Lead's pick: (b). Case against (b): more per-story turns;
an unsure user may not have a test sentence yet. The harness fact for the "who" question (blind map
C2) dispatched to a `claude-code-guide` seat in parallel; the "who" question waits for it. **User:
"b, as recommended"** → D2-line-plus-test.

### Q3 — Who holds the story conversation with the user, and who owns "which"? (put 2026-10-10)

Facts in hand: F-H1–F-H3 (a subagent cannot ask the user; a teammate's blocking question is
undocumented; the user can open a teammate's transcript); F-S11 (the "which" floor: PM owns which
capabilities, the filter and selection advice; the analyst owns how well); F-S13 (transcription of
user decisions trips no sound loop). Hangs on D1-story-discovery-stage. Unblocks the rules-home item,
the hand-off item (blind map C3) and the gate-count item.

Options: (A) the lead holds the conversation inline, as brainstorm does, with the story skill's
form as the unit and the PM's capability frame as input; the slate is transcribed; the analyst seat
then authors stories bound to the slate, plan-first (cheaper shape — reuses brainstorm's wiring) ·
(B) the analyst seat is a teammate holding the conversation, the lead relaying forks and answers
(message lane; the direct path undocumented) · (C) the analyst drafts each fork's options and facts
from a seat, the lead puts them to the user — the analyst's craft in the room, no live seat ·
"which" stays the user's under every option; the PM floor is read as untouched (the user sits above
it) and is not amended. Lead's pick: (A). Case against (A): the user asked for the analyst in the
room, and under (A) the analyst is not — the lead is, wearing the analyst's skill; persona judgment
("consider the edges") enters only at authoring.

User's first reply: "i am confused, we use agent teams, not subagents" — the lead restated plainly
with the subagent fact dropped and re-put the three options as teammate shapes, leaning (B) with a
dogfood probe as its condition. User's second reply (F3): the lead holds it — "when i said analyst
and me, i meant the lead" → D3-lead-holds-stage. Ratified streak reset to 0 (the user ruled against
the lead's lean).

### Q4 — A blind second list of journeys before the slate closes? (put 2026-10-10)

Hangs on D1-story-discovery-stage, D3-lead-holds-stage. Unblocks Q5's size and the completeness
default. Options: (a) none — the lead's "what the floor needs" fact, the PM's map-fit vetting and the
stress-test's edge hunt are the completeness checks (cheaper) · (b) one per spec run, right before
the slate closes: a fresh seat gets the intent synthesis and the capability frame, never the slate,
and returns the journeys it sees; the user sees where it differs, each difference a candidate row to
rule · (c) on the user's request only ("what am I missing?"). Lead's pick: (b). Case against (b): a
seat per spec run; new candidates late can re-inflate the set, and the user has to rule them one by
one. **User: "b, as recommended"** → D4-blind-journey-list.

### Q5 — Where do the stage's rules live? (put 2026-10-10)

Hangs on D1-story-discovery-stage, D3-lead-holds-stage, D4-blind-journey-list. Unblocks the build
item's shape. Facts: F-D1 and the D19 precedent (section *Facts from the decisions layer*).

Options: (A) specify's own rules — a new moment and the stage's rules written in specify's schema,
with a precedence clause over the shared questioning skill as brainstorm has; the sibling
architecture session rules its own; convergence to a common block handed to a later session once
three commands carry the way (cheaper now; drift accepted) · (B) a third output shape in
`mochiko:analysis-iterative` — "story discovery", with the agenda, the unit and the slate — pointed
at by one specify rule; the shared skill's body grows, and setup and brainstorm read the same file ·
(C) extract the common block now — brainstorm's rules become stubs; touches brainstorm's schema
(out of scope) and pre-empts the sibling. Lead's pick: (A). Case against (A): three near-identical
rule sets drift until someone converges them, and R1's licence invites the convergence that D19
already deferred once. **User: "a, as recommended"** → D5-specify-own-rules.

### Q6 — Two confirmed screens, or one? (put 2026-10-10)

Hangs on D1-story-discovery-stage, D3-lead-holds-stage. Unblocks nothing further; closes the
gate-count item (blind map D1, D4). Facts: F-S2 (intent "closes in a one-screen synthesis the user
confirms"); `spec.fail.intent-unconfirmed` (a floor: the Intent section confirmed before authoring —
authoring, not the story stage); the capability frame is agreed with the user at intent (F-S3).
Options: (a) two — the intent screen confirmed as today, then the story stage, then the slate
confirmed at its close (do-nothing-new shape; the envelope hardens before the forks, as brainstorm's
frame does) · (b) one — the intent synthesis stays provisional through the story stage and intent
plus slate are confirmed together at the stage's close. Lead's pick: (a). Case against (a): two
confirmations back to back; the gate count grows by one, the fatigue the blind map named. **User:
"a, as recommended"** → D6-two-confirm-screens. Ratified streak reaches 3: said to the user; Q7 is
put in another form (the case against first, a weak lean in place of a pick).

### Q7 — What is a dogfood run judged on, and what does "tight" mean? (put 2026-10-10, case-against-first form after three ratified rulings)

Hangs on D1–D6. The last fog item (blind map A1, A3, A4). No quoted case of the problem is in the
record (the frame-card fact question went unanswered); the diagnosis carries `Assumed` until a run.
Lead's weak lean: three counts — (1) traceability, every story file maps to an `in` slate row and no
`in` row lacks a file (mechanical; the stress-test check already a default); (2) zero strikes — at
the acceptance screen the user strikes or rewrites no story's line or test; (3) the user's word on
tightness per run, with any story they would not have asked for named. No count band (the ruled
count is the band); no replay required (the dogfood runs are the evidence). Alternatives: the user's
word alone (cheapest, subjective); a replay of a past kinako run under both shapes (most evidence,
costs a run). **User: "i think we can ignore the dogfooding for now. I plan to change the specify
workflow based on this brainstomr and then run specify in the dogfood project."** →
D7-dogfood-criteria-deferred.

## Decisions

### D1-story-discovery-stage — A story-discovery stage after intent, run like a brainstorm — `Confident`

**Statement.** `/mochiko:specify` gains a story-discovery stage that runs after the intent synthesis
is confirmed and before any story is authored. In it the story set is worked with the user the way
brainstorm works decisions: forks put one per turn, each with at least two real options of which one
is the do-less shape (fewer stories, or none for that journey), the facts a fork hangs on in hand
before it is put, closing in a ruled slate. The slate is written into the spec's `## User Stories`
index (F-S9) and story authoring is bound to it. The selection gate is untouched: the slate rules
what the spec carries, never what builds now (blind map D3).

**Why.** The user's own words — "brainstorm with user before generating a large amount of user
stories" (F2). The intent screen is already confirmed by the user and still does not bind the
stories (F-S1, F-S5; blind map A2), so a second confirmed screen of the analyst's guesses would
repeat the failure (blind map B1). The cheapest roads either spend the volume first (prune at
selection) or rule one anchor story, not the set (one core-journey probe).

**Rejected roads.** B, a slate on the intent screen — the analyst's list pruned, not a brainstorm;
collides with the two "never a list of stories" lines (F-S3, F-S10); the Intent section's 10-line
budget (F-S9). C, story at a time — no whole-set view, so stories cannot be traded against each
other; turns grow with the count. D, nothing new in specify — pruning at the selection card after
authoring, prototyping and stress-testing; or `/mochiko:brainstorm` first, whose full ceremony is
heavy for a story list and whose record binds nothing in specify today. The second list's road 2
(one content probe in the intent agenda) rules the anchor story only.

**Accepted risk.** The slowest road: two confirmed screens (intent, slate) — ruled so at
D6-two-confirm-screens. The most new rules — a moment, a reservation line, a fail condition.
Brainstorm's mechanisms are restated into specify's rules; convergence to a common block handed on
(D5-specify-own-rules), with the sibling architecture session counted (F-S12).

**How it was decided.** Q1, its own turn; the blind second list taken (`research/second-list-shape.md`)
and its differences shown; the case against the pick given; user: "A, as recommended" — ratified
(streak: 1).

**Changed at review.** The two "open" pointers in Accepted risk (gate count; rules home) now cite
the cards that closed them, D6-two-confirm-screens and D5-specify-own-rules (record-fitness repair at
the fold, no change of substance).

### D2-line-plus-test — The user rules one-breath story lines with priority and an independent test — `Confident`

*(Re-slugged at review, S13: was `D2-ruled-unit-line-plus-test`, five words; every cite swept.)*

**Statement.** In the story stage the unit the user rules is the story line — who, does what, why —
with its priority and its independent test: the one sentence that says how the user would know the
story works. Acceptance scenarios, functional requirements and success criteria are authored
afterwards by the analyst, inside the ruled lines; they are not forks of the stage. Which rows build
now stays the selection gate's.

**Why.** A want often lives at "how I would know it works" (blind map B2); one test line pins what
a story means without opening every scenario. A line alone leaves the author free to read a story
several ways — the looseness the problem names, one layer down.

**Rejected roads.** (a) line and priority only — cheaper, but the meaning stays open to the author's
reading. (c) full story content ruled live, every scenario a fork — the volume moves into the
conversation; turns multiply.

**Accepted risk.** More turns per story than (a). An unsure user may have no test sentence yet;
forcing one would be the same guess problem — the stage must allow "test: open" on a line and carry
it as an open question rather than invent one.

**How it was decided.** Q2, its own turn, three options with the cheaper shape named and the case
against the pick given; user: "b, as recommended" — ratified (streak: 2).

**Changed at review (S4; user: "as recommded", ratified).** The slate's test is a seed, not the
story file's `Independent Test` verbatim: the analyst expands it into that field — setup or data,
plus the pass and fail conditions, per `authoring-user-stories.independent-test-required` — without
changing its meaning, and the acceptance screen shows the seed beside the expansion. A `test: open`
row becomes an Open Questions entry plus a provisional test marked provisional, never a silent
invention. Rejected: a separate "user's check" line carried beside a freshly written Independent
Test — two tests per story that drift apart. Untested bet 3 is restated from "zero rewrites" to "no
change of meaning between seed and expansion".

### D3-lead-holds-stage — The lead holds the story conversation in the main pane — `Contested`

*(Re-slugged at review, S13: was `D3-lead-holds-story-stage`, four words; every cite swept.)*

**Statement.** The story stage is run by the lead, inline, in the main conversation — the same seat
and pane as the intent stage and as brainstorm's questioning. The lead carries the story skill's unit
(D2-line-plus-test) and the PM seat's capability frame as inputs, puts the forks, and transcribes the
user's rulings into the slate (transcription — no sound loop, F-S13). The analyst seat then authors
the stories from the ruled slate, plan-first and graded as today. No teammate holds a conversation
with the user; the user never leaves the main pane for this stage. "Which stories" is the user's; the
PM floor on which capabilities, the filter and selection advice (F-S11) is untouched.

**Why.** The user's word: "the lead is the manager of the teammates"; F2's "analyst" meant the lead.
The documented harness path for a teammate-held conversation is user-initiated only (F-H1, F-H3) and
unprobed in a run; the lead-inline shape is the one brainstorm already uses.

**Rejected roads.** (B) the analyst teammate holding the conversation in its own pane, the user
switching to it — the lead's lean, rejected by the user's word; it would also have rested on unprobed
behaviour. (C) the analyst drafting each fork for the lead to relay — one hop each way, message-lane
legs on every question.

**Accepted risk.** The analyst persona's judgment ("consider the edges") enters only at authoring,
after the slate is ruled; the completeness of the slate rests on the lead's forks and the blind
journey list (D4-blind-journey-list).

**How it was decided.** Q3, its own turn, put twice — first with a subagent fact the user found
confusing, then restated plainly as teammate shapes; the lead leaned (B); user: "when i said analyst
and me, i meant the lead. the lead is the manager of the teammates." — the user's own ruling against
the lead's lean.

**Changed at review.** The hand-off this card leaves implicit — what the analyst's brief carries,
who writes the slate into `spec.md` and when — is ruled as the default *Hand-off and slate writing*
(S12, coverage, user: "all inline"); the card's statement is unchanged.

### D4-blind-journey-list — One blind second list of journeys per spec run, before the slate closes — `Confident`

**Statement.** Before the slate closes, the lead dispatches one fresh seat with the confirmed intent
synthesis and the capability frame — never the slate or the conversation — and it returns the
journeys it sees for that territory. The user sees where its list and the slate differ; each
difference is a candidate row the user rules in or out. One list per spec run; the seat is
disposable; an interpretive dispatch, so it never rides haiku.

**Why.** The old stories were "not wrong" (F1): some journeys the user never names are still needed
(blind map B5). A blind list surfaces them as rulings for the user, not as the analyst's volume —
the same fence brainstorm uses on costly forks.

**Rejected roads.** (a) no list — completeness left to the floor fact, the PM's map-fit vetting and
the stress-test's late edge hunt, which reopens the set after authoring. (c) on request only — the
user who most needs it is the one who does not know what is missing.

**Accepted risk.** A seat per spec run. Late candidates can re-inflate the set, and the user rules
each one; the do-less option on each keeps the default at out.

**How it was decided.** Q4, its own turn, three options with the cheaper shape named and the case
against the pick given; user: "b, as recommended" — ratified (streak: 1).

**Changed at review.** (S11, batch) The seat is a persona-less `general-purpose` teammate (the
user's runs seat teammates, never subagents — F3; verify round 1, N4) spawned with an explicit `model: opus` (the strong alias; a
persona-less interpretive seat pins its alias on the spawn per `mochiko:patterns-model-tiering`),
disclosed in the run's roster line; "rides the session tier" was not an alias and is struck. Rows
the list adds carry the provenance value `blind list` (default *Mechanisms carried*), so bet 2 can be
read off the slate. (S3, coverage, user: "all inline") "in, out or deferred" became "in or out": the
slate carries no `deferred` value (default *Slate home and binding*).

### D5-specify-own-rules — The stage's rules live in specify's own schema; convergence handed on — `Confident`

**Statement.** The story stage is written into specify's own rules: a new moment, the stage's
duties and bindings, the reservation line, the fail condition, and a precedence clause stating that
where specify's rules and `mochiko:analysis-iterative` differ, specify's rules win (as
`brainstorm.own-rules-win` does). The shared questioning skill is not edited for this. Convergence of
the three commands' ways (brainstorm, specify, the architecture desk) into a `common.<slug>` block is
handed to a later session, opened when the sibling architecture session lands, under near-dup R1–R6;
until then the three rule sets are allowed to drift.

**Why.** Precedent D19-brainstorm-only-reach: a command's asking rules live in its own schema. R1
licenses convergence, never obliges it, and a block carries boilerplate never judgment (F-D1).
Extracting now would edit brainstorm's schema (out of scope) and pre-empt the open sibling (F-S12).

**Rejected roads.** (B) a third output shape in the shared questioning skill — grows a file setup
and brainstorm read, and the mechanisms are brainstorm's rules not the skill's, so they would be
restated anyway. (C) extract the common block now — out of scope, pre-empts the sibling.

**Accepted risk.** Three near-identical rule sets drift until converged; the convergence item is
written at landing with its trigger (the sibling's landing) so it is not lost.

**How it was decided.** Q5, its own turn, three options with the cheaper shape named and the case
against the pick given; user: "a, as recommended" — ratified (streak: 2).

**Changed at review (S5, batch; user: "as recommded, all inline").** The build's touch-set is wider
than specify's schema, and each touch is a ruled primitive edit under the landing ritual, not a
precedence clause — a clause over `mochiko:analysis-iterative` reaches no other skill:
- `specify` schema and `plugins/mochiko/commands/specify.md` — the new moment, the stage's rules, the
  slate-confirm reservation, the `kind: fail` rule, the precedence clause, and a Goal-step clause for
  the ruled slate (blind map F4);
- `authoring-user-stories` — the 2–5-per-feature bound (F-S8) yields to a ruled slate's count; the
  Independent Test is the expansion of the row's seed without change of meaning (D2-line-plus-test as
  changed); "landing in `spec.md`" reads as the index row plus the `stories/US-*.md` file;
- `review-specifications` — the slate checks of the default *Slate home and binding*;
- the `spec` template — the Test column and the struck-candidates sub-table (default *Slate home and
  binding*), a template migration;
- `authoring-feature-map` — the filter's judgment half runs once at slate close (default *Filter*);
- `authoring-requirements` and the `spec` template's Functional Requirements guidance — each FR
  line carries its story reference (`US-n`), the carrier of slate check (3) (verify round 1, N3);
- the `spec` template, additionally — the slate header line that records a skip (verify round 1, N1);
- the plugin contract suite's frozen specify floor set — the new `spec.fail.*` rule is `class: floor`.
The architecture sibling's rules are not touched.

### D6-two-confirm-screens — Intent confirmed first, then the story stage, then the slate confirmed — `Confident`

**Statement.** The intent synthesis is confirmed on its own screen, as today, before the story stage
opens; the story stage closes on a second confirmed screen — the slate: every candidate row with
its disposition and why, the blind list's differences ruled, nothing open. `spec.fail.intent-unconfirmed`
is unchanged (confirmed before authoring); the slate confirm is a new gate reserved to the user.

**Why.** Forks run against a hardened envelope, as brainstorm's deciding runs only after its frame
hardens; a scope or depth line changing mid-stage would reopen forks. Two short screens cost less
than that.

**Rejected roads.** (b) one screen at the stage's close with the intent provisional throughout —
one gate fewer, forks against a moving envelope.

**Accepted risk.** Two confirmations back to back; the gate count grows by one (blind map D4).

**How it was decided.** Q6, its own turn, two options with the do-nothing-new shape named and the
case against the pick given; user: "a, as recommended" — ratified (streak: 3).

**Changed at review.** —

### D7-dogfood-criteria-deferred — What a run is judged on is deferred to the first run in ai-fileops — `Deferred`

**Statement.** No criteria for "tight" or for judging a dogfood run are fixed in this session. The
build lands the stage from D1–D6; the first `/mochiko:specify` run in the ai-fileops repo is the
test, judged by the user in use. The three counts of the lead's weak lean (traceability · zero
strikes · the user's word) stay in this card as candidates for that run, not as rulings.

**Why.** The user's word: "ignore the dogfooding for now … run specify in the dogfood project." The
record holds no quoted case of the problem, so criteria fixed now would be fixed against an
`Assumed` diagnosis.

**Rejected roads.** Fixing the three counts now; the user's word alone; a replay of a past kinako
run.

**Accepted risk.** The first run has no agreed yardstick; whether the change worked will be the
user's judgment after the fact, and a regression (slower specify, re-inflated set) is found in use.
The slate checks still ship as stress-test defaults, so the mechanical counts exist even without a
ruling on them.

**How it was decided.** Q7, put in the case-against-first form after three ratified rulings; the
user deferred by their own word.

**Changed at review.** —

## Defaults batch — confirmed

Named by name in the Q4 message with the pull offer; confirmed as one block at the accept screen
2026-10-10 ("as recommded"), together with the decisions D1–D7 by name and gist, the out-of-scope
line, the open questions and the untested bets. Amended at review where marked; every amendment was
ruled by the user — S1, S4 and the filter timing on their own turns, S3, S9 and S12 inline as
coverage candidates, the rest in one confirmed repair batch ("as recommded, all inline").

- **Mechanisms carried** from brainstorm into the stage: one fork per turn · at least two real
  options, one the do-less shape · fact in hand before the fork · a rough in-message mock for a story
  (one row as it would read) · a visible status line (ruled · askable · in fog) over the candidate
  journeys · ratified-yes streak flag (specify already carries it) · vague answer re-asked as two
  concrete options · stop rule: every candidate in or out, nothing open · provenance mark per slate
  row, one of `user's words` · `lead's guess, ratified` · `blind list` · `handed in` · own-turn-or-batch:
  a candidate journey gets its own turn only when it is a real choice or costly to undo; the rest are
  default rows named in one line as they arise and confirmed together on the slate screen · stall
  clause: if the candidate set has not shrunk for three turns running, the lead says so and offers
  to step back to the intent synthesis. Dropped: a frame card (the intent screen is it) · decision
  cards and a record file (the spec is the record) · a blind angle map and cold review (the
  stress-test exists). *Changed at review: own-turn-or-batch and the stall clause added (S6, batch);
  `blind list` and `handed in` provenance values added (S11, batch; S1); "in, out or deferred" →
  "in or out" (S3).*
- **Slate home and binding**: the slate is the spec's `## User Stories` index (F-S9), whose column
  grammar the build reads first-hand from `mochiko-cli template spec`. Its existing columns stay —
  ID (linked to `stories/US-N.md`), Story (one breath), Priority, Feature (`FEAT-XXX-<slug>` or —),
  Disposition — and one column is added: Test, the user's seed (D2-line-plus-test). Disposition is one
  field in two phases: before authoring it carries the slate value — `in`, `in — waiver pending`
  (default *Floor-obligated stories*) or `out — <why>`; after derivation an `in` row carries the
  existing `homed | rejected — why`. `out` rows carry no file link
  (the ID is unlinked) and sit in a `### Struck candidates` sub-table under `## User Stories` — the
  home of a struck candidate (blind map E3); slate values are not story statuses under
  `authoring-feature-map`'s single-home floor ("the only story-native status is `rejected`" stands
  for authored stories). The slate has no `deferred` value: "later" is `in`, authored, and unselected
  at the selection card, which already yields a pending work row with its named return path
  (`pm-requirements-stacking` D1-across-round-phasing). This is a `spec` template migration, named in
  D5-specify-own-rules's touch-set. Story
  files are authored for `in` rows only. A new `kind: fail`: a story authored with no slate ruled, or
  outside the ruled slate. An author who finds a story the slate lacks proposes it as a row; the user
  rules; never a silent add (blind map E1). The stress-test gains the slate checks, each a blocking
  gap on failure: (1) every story file maps to an `in` row and no `in` row lacks a file; (2) each
  file's story line and Independent Test keep the meaning of its row's line and seed, the seed shown
  beside the expansion; (3) every functional requirement traces to an `in` row's story — carried by
  a story reference (`US-n`) on each FR line, which `authoring-requirements` and the `spec`
  template's Functional Requirements guidance gain in D5-specify-own-rules's touch-set; (4) the
  scenario count per story stays within the story skill's existing bound (blind map E2). *(Verify
  round 1, N3: the carrier for check (3) named; lead-repaired.)* The 2–5
  bound (F-S8) yields to the ruled count (blind map E5). *Changed at review: columns additive, two
  disposition phases, unlinked `out` rows in a sub-table, the template migration named (S2, batch);
  no `deferred` value (S3, coverage, inline); checks (2)–(4) added (S9, coverage, inline).*
- **Ordering against the capability frame**: the PM's frame (nouns and verbs) still comes first, at
  intent; the story stage runs inside it; the slate is declared not-the-frame, so F-S3 and F-S10
  stand unamended; stories still win conflicts at the post-stories confirm (blind map D2).
- **Filter**: the PM seat vets the slate once, at its close — one pass over every candidate row for
  map fit (duplicates of pending rows or stubs, extend-vs-mint), returned alongside the blind journey
  list before the user confirms the slate; after authoring, the filter keeps only its homing and
  dedup mechanics, and a rejection of an `in` row then is an escalation to the user (blind map D5).
  *Changed at review (S7's pull of the filter-timing default, S5; put on its own turn; user: "as
  recommded", ratified): the earlier text had the PM vetting per fork during the stage — a seat
  round trip on every candidate; the as-today road (filter after authoring, escalation on bounce)
  was rejected as churn.*
- **Floor-obligated stories** (auth, deletion, recovery under the production floor): the lead names
  the floor as the fact the fork hangs on. Legal strikes: `out — handled elsewhere: <pointer to the
  capability or row that covers it>`, and a narrowing — the row stays `in`, its line rewritten to
  what remains by the user's word (provenance `user's words`), what was cut noted in the row's
  Story cell after the line (an `in` row carries no why).
  A true drop is a waiver on the governance ledger's existing path, dispatched to `/mochiko:setup`
  amend mode as the architecture rules do; the run continues with the row held `in — waiver pending`:
  the row is not authored while pending and slate check (1) treats it as not yet due; acceptance
  blocks until the waiver lands — the row then becomes `out — waived: <ledger reference>`, never
  authored — or the user restores it to `in`, after which it is authored (blind map E4). So the
  Disposition field's slate phase carries `in`, `in — waiver pending`, `out — <why>` (its whys
  including `handled elsewhere: <pointer>` and `waived: <ledger reference>`). *(Verify round 1, N2:
  lead-repaired — the earlier "narrowed" value and the unstated waiver transition.)* *Changed at review (S8, batch): the earlier
  text allowed only a waiver or `in`; "handled elsewhere" and "narrowed" added from the
  `arch.floor-precedence` precedent, and the run's behaviour while a waiver is pending stated.*
- **Inputs already in hand**: a brainstorm record named in `$ARGUMENTS`, or stories already in story
  form, open the stage as facts; only what they leave open is asked (blind map G2).
- **Size and skip**: no size gate; a decisive user gets confirmations and a fast wrap-up (the skill's
  own line). Skipping the stage means handing in stories: the user's own one-breath lines are
  transcribed as `in` rows with provenance `handed in`, so a slate always exists and the new fail and
  the slate checks stand unchanged; the skip is recorded in the slate's header line, never in the
  Intent section (its 10-line budget, F-S9). A skip with nothing in hand is not a skip — the stage
  runs (blind map G1). A skip skips the forks, not the close: the blind journey list
  (D4-blind-journey-list), the PM's slate-close vet (default *Filter*) and the slate confirm
  (D6-two-confirm-screens) still run over the handed-in rows, so a handed-in set is still checked for
  what it misses and for map fit before it binds authoring. The slate header line is part of the
  `spec` template migration named in D5-specify-own-rules's touch-set. *(Verify round 1, N1:
  lead-repaired.)* *Changed at review (S1; own turn; user: "as recommded", ratified): the earlier
  text "skipping the stage is the user's word, recorded in the spec's Intent as a line" contradicted
  the new fail and the check; the carve-out road (exempting a recorded skip) was rejected as
  reopening the loose path on one word.*
- **Hand-off and slate writing** *(added at review — S12, coverage, inline)*: at the slate confirm the
  lead writes the slate into `spec.md`'s `## User Stories` index and its struck sub-table —
  transcription of user decisions (F-S13) — before the analyst seat opens `spec.md`; the pen then
  passes to the analyst seat explicitly, so `spec.md` has one writer at any moment
  (`patterns-transport-floor.single-writer-per-surface`). The analyst's authoring brief carries the
  slate rows, the user's own words per row quoted verbatim (the one-breath line is the lead's
  compression), each row's seed, and the `out` rows with their whys, so the author neither loses the
  nuance nor re-proposes a struck candidate (blind map C3). Rejected: the analyst transcribing the
  slate from the brief — a transcription that would itself need grading. After the pen has passed,
  every later row change — an author-proposed row the user rules, a UX click amendment, a waiver
  outcome — is ruled by the user with the lead and sent to the analyst seat as the exact row text
  to land (content-pinned, `patterns-transport-floor.content-pinned-supersession`); the analyst
  writes it as mechanical execution of an existing ruling (F-S13's first exemption), so `spec.md`
  keeps one writer and the slate's judgment stays the user's. *(Verify round 1, N5:
  lead-repaired.)*
- **UX-bearing work**: the skeleton nav frame is built from the ruled slate; a per-story click that
  changes a story's line or test goes back to the user as a one-line amendment to its row (blind
  map D6).
- **Acceptance scenarios**: authored after the slate, inside the ruled lines (D2-line-plus-test).

## Open questions

- OQ1 — The evidence base: no specify run showing the problem is quoted in this record (the frame
  card's fact question went unanswered; the user placed the problem in specify generally, F1). The
  diagnosis carries `Assumed`; the first ai-fileops run (D7-dogfood-criteria-deferred) is where it is
  tested.
- OQ2 — Convergence of the three commands' ways into a common block (D5-specify-own-rules): handed to
  a later session, opened when `architecture-brainstorm-interview` lands; written as a BACKLOG item at
  landing.

## Untested bets (shown at the accept screen)

- The story stage does not make specify much slower — tested by the user's first ai-fileops run
  (D7-dogfood-criteria-deferred).
- The blind journey list catches needed-but-unnamed stories without re-inflating the set — tested by
  counting the slate rows carrying provenance `blind list` that the user ruled `in`.
- The line-plus-test unit pins a story's meaning for the analyst — tested by no change of meaning
  between the ruled line and seed and the authored story and its expanded Independent Test, read
  at acceptance. *(Restated at review, S4; was "zero rewrites of a story's line or test".)*

## Freeze

Frozen for the cold read 2026-10-10 after the accept screen. The review seat (`review-seat`, the
blind-map seat) was resumed with this record for message two. Writers held until the verdict
(`patterns-transport-floor.quiesce-before-cold-grade`).

## Cold review — landed 2026-10-10 (`reports/review.md`)

Solo cold read by `review-seat` (`mochiko:devils-advocate`, persona tier `opus`, no override), the
same seat that drew the blind map; cross-examination of the lead on three points (defaults
engagement · D2-line-plus-test's test line · the index columns — answers in the report's `cross_exam` block).
Recommended status **needs-revision**: 20 raised, 13 survived — 0 Critical · 8 Important · 5 Minor;
coverage-marked S3, S9, S12. Every load-bearing fact verified by the seat's own renders; F-H1–F-H3
not live-checked (not load-bearing after F3). Seat disclosure: two haiku Explore spawns returned no
report; the reads ran on the seat tier. The lead's reading of the survivors: the seven cards stand
as rulings; the defects sit in the defaults batch and in what the cards leave unsaid.

Routing (per `brainstorm.non-coverage-survivors` and `brainstorm.coverage-survivor-routing`): own
turns for S1 (skip vs the new fail — a real choice), S4 (the test line's relation to the story
skill — clarifies D2-line-plus-test) and the filter-timing default (S7's ask, S5 — changes a skill's
flow); coverage candidates S3, S9, S12 put to the user for a path each (explore now · rule inline ·
defer), each with the lead's lean and the case against it first (the ratified streak stood at 3);
S2, S5, S6, S8, S10, S11, S13 as one named repair batch with the lead's proposed fix per item.
**User: "as recommded, all inline"** — the three coverage candidates ruled inline on the lead's
leans; the batch confirmed.

| # | Sev | Finding (gist) | Disposition |
|---|---|---|---|
| S1 | Important | skip default contradicts the new fail and the traceability check | own turn; user: "as recommded" — skip = stories handed in and transcribed as `in` rows; fail and checks unchanged; skip line in the slate header (default *Size and skip* amended) |
| S2 | Important | slate-home default rewrites the User Stories index; two disposition vocabularies; dead links | batch, confirmed — columns additive (Feature kept, Test added); Disposition one field in two phases; `out` rows unlinked in a `### Struck candidates` sub-table; slate values not story statuses under the single-home floor; template migration named in D5-specify-own-rules (default *Slate home and binding*; F-S9 annotated) |
| S3 | Important, coverage | slate-`deferred` has no return path; duplicates selection's deferral | candidate topic; user: inline — slate values are `in`/`out` only; "later" = `in` and unselected at selection, which yields the pending row and its return path (default *Slate home and binding*; D4-blind-journey-list changed-at-review) |
| S4 | Important | D2-line-plus-test's test line vs `authoring-user-stories.independent-test-required`; bet 3 fails by construction | own turn; user: "as recommded" — seed expanded without change of meaning, shown beside; `test: open` → Open Questions + provisional test; bet 3 restated (D2-line-plus-test changed-at-review) |
| S5 | Important | build touch-set larger than D5-specify-own-rules names | batch, confirmed — D5-specify-own-rules changed-at-review names the six touches as ruled primitive edits |
| S6 | Important | own-turn-or-batch and the stall clause not carried | batch, confirmed — both added to default *Mechanisms carried* |
| S7 | Minor | one-block defaults; pull offer not in the record; filter timing never weighed alone | filter timing put on its own turn; user: "as recommded" — PM vets once at slate close (default *Filter* amended). The pull offer: made in the Q4 message ("say pull X to open any default on its own turn") and again at the accept screen; the user pulled none — recorded here and in the reshown map |
| S8 | Important | floor-strike default allows only waiver or `in` | batch, confirmed — `out — handled elsewhere: <pointer>` and `narrowed` legal; true drop = ledger waiver via `/mochiko:setup` amend; row held `in — waiver pending`, acceptance blocks until it lands (default *Floor-obligated stories*) |
| S9 | Important, coverage | the stress-test check grades presence, not content | candidate topic; user: inline — checks (2) line and seed meaning kept, (3) every FR traces to an `in` row's story, (4) scenario count within the skill's bound (default *Slate home and binding*) |
| S10 | Minor | Destination line unstamped after F3 | batch, confirmed — stamped in the frame card; D1–D7 re-checked consistent |
| S11 | Minor | D4-blind-journey-list's seat type and alias unnamed; no blind-list provenance value | batch, confirmed — persona-less `general-purpose`, explicit `model: opus`, disclosed; provenance value `blind list` (D4-blind-journey-list changed-at-review; default *Mechanisms carried*) |
| S12 | Minor, coverage | the analyst's brief contents, the slate's writer and timing unsaid | candidate topic; user: inline — new default *Hand-off and slate writing*: the lead writes the slate at confirm, pen passes explicitly; the brief carries rows, the user's words verbatim, seeds and `out` rows (D3-lead-holds-stage changed-at-review) |
| S13 | Minor | card placement; the D2-line-plus-test and D3-lead-holds-stage slugs not three words; bare cites | batch, confirmed — cards under `## Decisions` in order; `D2-line-plus-test`, `D3-lead-holds-stage` (cites swept); the question trail gathered; pairs and short lists of decision cites joined, ranges left bare; the blind map's `D`-class labels disambiguated in the first map |

## Verify pass

Round 1 (2026-10-10, `reports/review.md` § Verify round 1; graded snapshot: the record at 14:51:30,
817 lines): **13/13 survivors CLEAN**; six fold-introduced Minor defects N1–N6, status
`needs-revision` (N1, N2, N5 failed the builder test). Quiesce breach, disclosed: the lead landed
ten cite joins after the verify dispatch and before the seat's snapshot; the seat's snapshot
included them; no edit landed after it until this repair. Lead repairs, each one default or card:
- N1 — a skip skips the forks, not the close: blind list, PM vet and slate confirm still run over
  handed-in rows; the slate header line joins the template touch (default *Size and skip*;
  D5-specify-own-rules).
- N2 — "narrowed" is a rewritten `in` line, not a disposition value; `in — waiver pending` rows are
  not authored while pending, check (1) treats them as not yet due, and the waiver's landing turns
  the row `out — waived: <ledger reference>` or the user restores it to `in` (defaults
  *Floor-obligated stories* and *Slate home and binding*).
- N3 — check (3)'s carrier: a `US-n` reference on each FR line; `authoring-requirements` and the
  template's FR guidance join D5-specify-own-rules's touch-set.
- N4 — D4-blind-journey-list's seat is a teammate, never a subagent (F3).
- N5 — post-hand-off row changes are ruled by the user with the lead and landed by the analyst as
  content-pinned mechanical execution (default *Hand-off and slate writing*).
- N6 — the cross-record cite joined: pm-requirements-stacking D1-across-round-phasing.

Round 2 (2026-10-10, `reports/review.md` § Verify round 2; graded snapshot 859 lines, sha256
prefix dc99fefcc0759df5, identical to the live file throughout; diff against round 1 confined to the
listed spans): **N1–N5 CLEAN; N6 NOT CLEAN on cite format only**; one new cite-format defect (R1:
two bare cites in this section); status **`ready`** — the residuals are fix-on-sight integrity
repairs under the sound-loop exemption, needing no ruling and no further verify. Lead's clearing
verdict: **ready**. Fix-on-sight after the verdict, seat-unverified, disclosed: N6's owner put in a
code span; R1's two cites joined; the seat's commentary on "noted in the row's why" for a narrowing
(an `in` row has no why) repaired to "noted in the row's Story cell".

## Acceptance and landing

Accepted 2026-10-10 — user: "accepte" (the first reply, "as recommded", was re-asked once as not
an acceptance word, per `brainstorm.accept-screens`). Landed the same day under the KM close ritual
(`.mochiko/memory/knowledge-management.md`): one `DECISIONS.md` row (2026-10-10) pointing here ·
`BACKLOG.md` section *Specify story-stage build* — the build item and the three-command
convergence session (OQ2) · `ROADMAP.md` — folded into the PM-role & capability-map Now row, caps
held (Now 5 · Next 7) · the index entry updated to accepted with this landing. The open-item count
watch stood tripped before this session (last groom 2026-09-03, baseline 83; 101 open before this
landing, 103 after) — a groom was offered to the user at close, not run here. No git mutation run;
a commit suggested. Next-step offer made, not defaulted: the build item is the next move, the
first ai-fileops specify run its test.

Built 2026-10-10 at v0.119.0 on branch `specify-story-stage`, the same day, on the user's word
("open the build now"): one wave (`wave1-story-stage.md`; the log `build-log.md`; the gate report
`reports/audit.md`). Migrations 0051–0057 carry D1–D6 and the confirmed defaults (0057 a unit-1
gate fix restoring `${pm_seat}` and `${spec_schema}` bindings, the render unchanged); the prose,
five strips and four ledger rows ruled HOLDS; the crate census tests, the contract suite's specify
floor set and the specify kit re-keyed. Three persona-less producers on peer-graded plans — P1 and
P2 revised once, P2's revision the user's grant past the wave's spent re-plan bound, and the user's
widening of P2's set to `validate-requirements.py` — one gate grader for the wave, nine units PASS.
Gates: validate 0 rejecting · views ≡ replay · `cargo test --all` 765 · the full similarity sweep ·
gitleaks clean · ids 0 bare · contract suite 97/97, none skipped. Landed: the `DECISIONS.md` row
stamped built · the build item to the trail, the first ai-fileops run (D7's test) and the
convergence session left open · the ROADMAP row touched · the index entry stamped built. D7 stays
`Deferred`: what a run is judged on waits for that first run's log.
