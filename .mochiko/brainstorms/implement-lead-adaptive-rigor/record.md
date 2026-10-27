# Implement — lead-held adaptive rigor: which steps run, and how deep — decision record

Status: **accepted** 2026-10-27 (user's word: "accept") · frame hardened 2026-10-25 (size standard, reach universal), three lines amended by the user's word · deciding finished 2026-10-27 — map empty · ruled 9 (D1–D9; D2 amended in session; D3, D4, D5, D6, D7, D8 and five slugs changed at review) · defaults DF1–DF11 confirmed · cold review: 14 survivors and 4 verify-born inconsistencies, every one dispositioned by the user's word; verify bound exhausted after one pass and one bounded re-verify · landed: `DECISIONS.md` row 2026-10-27, `BACKLOG.md` build item and OQ2 open thread, `ROADMAP.md` Now touched, index entry updated

## Topic (user's words)

> i want to target overall mochiko philosophy via implement workflow restructure. i am concerned
> that lead doest make active decisions on when it can call on a particular step of the implement.
> I found the level of verification is not adaptive. I want lead to make active decisions in terms
> of what needs to followed, and how deeply. it needs to pragmatic. I need to target the core
> skills of the mochiko

## Orientation

Session index read 2026-10-25 (`.mochiko/brainstorms/index.md`); newest entry `engine-follow-up`
(accepted 2026-10-15, built 2026-10-18 at v0.121.0). Working tree on `trim-textbook-references`
at 0.122.0. Facts below are quoted source lines; readings are marked.

### Facts

- F1 — `DECISIONS.md` line 126 (2026-08-01, `lead-owned-process-flexibility`): "commands keep a
  **stated default pipeline** the lead departs from at will, departures recorded … run-start
  weight card user-ruled … rigor = cost-of-being-wrong … | ruled (… the inlined
  floor/bounds/trail/departure-record content itself superseded later the same day — v8
  goal+harness rebuild drops weight card, counted bounds, and departure trail from commands,
  command-architecture-realignment D2-harness-keep-drop, deliberately ratified reversals)".
- F2 — `.mochiko/brainstorms/command-architecture-realignment/record.md` lines 95–103
  (D2-harness-keep-drop, 2026-08-02): "Drop, dying in place: counted bounds, round caps,
  kill-switch tokens · seat tables and fixed agent×skill assignments · recovery tables · seat
  lifecycle/recycling, the run-start weight card, the departure trail · KM landing steps. Both
  flagged reversals (departure trail — reverses a v7 ruling two days old; counted bounds — removes
  the postmortem-paid brake) were put to the user plainly and ratified deliberately. Control moves
  from procedural rails to **native plan approval**".
- F3 — `.mochiko/brainstorms/index.md` lines 608–611 (`command-architecture-realignment`, About):
  "the user's stance — the volume of encoded detail in commands is itself the defect; it encodes
  distrust of the lead … shape v8 anatomy Goal · Harness · Bindings, ~30–50 lines per command".
- F4 — `.mochiko/brainstorms/production-floor-adaptive-depth/record.md` line 73 (2026-08-11):
  "### D8-dial-standards-scope — The dial governs asserted code standards only; pipeline process
  rigor stays uniform — `Confident`"; line 75: "D5-low-retrofit-principle's retrofit-cost principle
  argues for uniform process rigor — a bad dependency or an under-reviewed governance synthesis is
  exactly the expensive-to-fix-later class."
- F5 — `DECISIONS.md` line 84 (2026-08-13, `charter-ritual-balance`): "**kind-keyed sound-loop
  floor** — a judgment-authored write to a governing surface … obliges the loop … no size
  threshold — kind-keyed, not scale-keyed (D1–D2) … staffing-freedom clause narrowed 'below the
  floor' (D3–D4)".
- F6 — `DECISIONS.md` line 32 (2026-09-19, `author-grader-consolidation`): "independence stays at
  all five binding sites — the cost cut comes from the grader's *form*, the surface-value tiering
  road recorded-rejected (D1)".
- F7 — `mochiko-cli rules implement --section <id> --plugin-root plugins/mochiko` (binary 0.4.0 ·
  grammar 2 · plugin 0.122.0), end lines: `impl.sec.roles · 18 rules` · `impl.sec.reserved · 15
  rules` · `impl.sec.tools · 50 rules` · `impl.sec.ways-of-working · 15 rules` ·
  `impl.sec.boundaries · 16 rules` · `impl.sec.fail-conditions · 17 rules`; preamble pins:
  "kind: fail · 17 rules" · "class: floor · 40 rules".
- F8 — same render: the only rules carrying `kind: latitude` are `impl.staffing-latitude` and
  `impl.design-seats-staffing` ("[class: advisory · kind: latitude · labels: seats] Design-seat
  staffing is your call"). Neither touches which steps run or how deep a grade goes.
- F9 — same render, preamble `conditions`: `scope` (selection|delta|epic|lane) · `depth`
  (low|high, "the governance depth level, read from the governance region at entry") · `km_file` ·
  `governance_region` · `baseline` · `seats` · `ux_bearing`. `when: depth=high` appears once:
  `impl.mutation-lens` — "The mutation lens runs on the verification seat … skips are disclosed".
  `impl.gap-finding-scope`: "The gap-finding pass runs on selection-scope and epic runs only".
- F10 — same render, `impl.sec.boundaries` opening line: "Floor obligations no seat and no in-run
  ruling can waive."; `impl.attempt-per-grade`: "A cycle consumes an attempt every time a
  verification seat grades it … Default 3 per cycle, redeclarable only at run-open."
- F11 — `.mochiko/strips/implement.md` line 2083 (`[v0.48.0]` strip): "the entire v7-form file
  superseded — … every G-numbered gate, the run-start weight card, floor-gate set, counted
  bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration +
  departure trail + per-run contract file".
- F12 — `.mochiko/brainstorms/index.md` line 20 (`implement-brainstorm-led`, Landed): "migrations
  0058–0062 · 0065–0070 · 0075–0084 (1,196 → 1,243 rules)".
- F13 — `crates/mochiko-cli/src/model.rs` lines 202–214: `pub enum Class { Floor, Must, Advisory,`
  … `"floor" => Class::Floor, "must" => Class::Must, "advisory" => Class::Advisory`; lines
  227–251: `pub enum RuleKind` carries `Reservation` and `Latitude` among its values. Reading
  (lead's): three rule classes exist; a fourth "departable default" class would be a grammar
  change and meets the CLI-grammar fence — `class: must` with a `when:` condition or
  `kind: latitude` are the in-grammar shapes.

- F14 — implement render, preamble `legend`: "class: floor is always delivered whatever its when:;
  when: gates when the obligation applies, never whether it reaches you." Boundaries section
  heading: "Boundaries — the non-waivable floor · Floor obligations no seat and no in-run ruling
  can waive." (surfaced by the blind second-list seat; lead-verified against F7's render.)
  Reading (lead's): the `floor` tag does two jobs today — delivery guarantee and non-waivability —
  and floor-tagged rules sit in four sections, not only Boundaries.
- F15 — implement render: `impl.fail.skip-unstated` "[when: scope=delta|lane] A run whose
  final-validation report does not state its skipped gap-finding pass." is the only skip-trail rule
  in the command; `impl.attempt-per-grade` and `impl.gap-rework-bound` carry `class: floor`
  (counted bounds are floor again after the 2026-08-02 drop). (Blind seat's fact, lead-verified.)

- F16 — implement render, `impl.staffing-latitude` "[class: must · kind: latitude · labels: seats]
  Beyond this schema's `class: floor` rules, everything is your per-run judgment: how you staff,
  sequence, and run the cycles; teammates or subagents per seat is your call." (blind map A1;
  lead-verified). Reading (lead's): latitude over *how* exists on paper; *which* steps run and how
  deep is pinned by the 40 floors and the 17 fail conditions (`impl.fail.design-skipped`,
  `impl.fail.gap-finding-missing`, `impl.fail.card-independence`, `impl.fail.regression`…), so the
  frame's "zero latitude over steps" overstates by the word "steps" alone — corrected below by the
  user's word.
- F17 — `mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.trigger`:
  `patterns-sound-loop.two-part-trigger` "[class: floor · kind: gate] … There is no size threshold:
  a small judgment-authored write on a governing surface trips the floor as surely as a 10-file
  rewrite — magnitude never gates it."; `sec.discipline`: `patterns-sound-loop.leg-2-non-author-review`
  "[class: floor · when: trigger=fired] … the user's ruling alone never substitutes for the review
  leg."; `patterns-sound-loop.exemptions-exactly-three` "Only three kinds of write never trip the
  trigger: mechanical execution of an existing ruling · transcription of user decisions ·
  fix-on-sight integrity repairs"; `patterns-sound-loop.no-delta-card-exemption` is `class: floor`.
- F18 — `BACKLOG.md` line 35: "**First ai-fileops implement run — the design stage's test**
  (2026-10-11 …) — after the plugin is upgraded to v0.120.0, run `/mochiko:implement` on an
  ai-fileops batch whose sufficiency check names gaps"; lines 444, 483, 501: "Sufficiency-check
  first-live-run watch" · "QA gap-finding first-live-run watch … n=0" · "Test-case-anchor
  first-live-run watch … n=0". Reading (lead's): no implement run has executed under the current
  rule set; the complaint's driver is a rule read or an older run — asked of the user as a fact.
- F19 — blind map A2 (lead-verified against F7's render): adaptivity exists in four places today —
  `impl.gap-finding-scope` (scope type), `impl.mutation-lens` (`depth=high`),
  `impl.delta-reverification` (re-grade scoped to the changed surface), and the sound loop's three
  exemptions (F17).

### Readings (lead's, marked)

- R-a — This is the third visit to "lead judgment versus encoded rigor". 2026-08-01 ruled the lead
  composes the process under a floor with a stated rigor key (F1). 2026-08-02 kept the lead's
  freedom but deleted its instruments — weight card, departure trail, counted bounds — on the
  ground that encoded detail is itself the defect (F2, F3). Since then the rule layer regrew
  through schemas: implement now delivers 131 rules, 40 of them floor, 17 fail conditions, and two
  latitude rules about staffing only (F7, F8). The lead's freedom is nominal: there is no rule
  that lets it not run a step.
- R-b — Verification depth today keys on two facts the lead does not set — `scope` and the
  governance `depth` level — and `depth` gates one thing, the mutation lens (F9). The per-cycle
  grade, final validation, cold snapshot, regression sweep and gap-finding run the same at every
  stakes level. The user's "not adaptive" matches the render.
- R-c — Three accepted rulings stand against adaptive process rigor and would be amended or
  superseded by the destination as I read it: D8 of `production-floor-adaptive-depth` (process
  rigor uniform, F4), D1–D2 of `charter-ritual-balance` (kind-keyed, not scale-keyed, F5), D1 of
  `author-grader-consolidation` (independence universal, F6). Each must be named at the frame, not
  found at review.

## Frame card (hardened 2026-10-25; three lines changed by the user's word, stamped below — heading groomed at review, S10)

Card as put (first message), with the user's one correction applied:

- **Problem** *(user's words + lead's count)* — the implement lead makes no active decision on
  when a step runs; verification depth is not adaptive; the lead should rule what is followed and
  how deeply, pragmatically. Who has it: the user running implement, and the lead seat under 131
  delivered rules with zero latitude over steps.
- **Destination** *(lead's guess, amended by the user's first reply)* — the implement lead holds a
  ruled, visible choice over which steps run and how deep each verification goes, keyed on stakes
  read at run-open; **author≠grader stays as a concept — adversarial review exists — and the lead
  rules its form per run**; the change lands in the core skills (`patterns-sound-loop`,
  `patterns-model-tiering`, `review-*`, `testing-*`), implement's rules pointing at them, not more
  command rules.
- **Must not break** *(lead's guess; author≠grader struck from this line by the user; **confirmed
  by the user 2026-10-25: "confirm all five"**)* — user gates stay the user's ·
  real-infrastructure evidence wherever verification runs · an honest trail of what the lead
  skipped · the engine rulings built at 0.120/0.121 · clone-only install and CLI delivery. Items
  1–3 are the floors the 2026-08-01 ruling kept at maximum lead freedom; 4–5 are scope fences.
- **Betting on** *(lead's guess; **confirmed by the user 2026-10-25: "confirm all three"**)* —
  (A) a stated stakes key beats both uniform rigor and free judgment — if wrong, the lead's read
  drifts under context pressure and the key becomes ceremony; (B) 2026-08-02 deleted the lead's
  instruments but not the appetite, and rules regrew to 131 — if wrong, the regrowth was right and
  this session re-adds ceremony; honest mark: no live implement run since the 0.120 engine rebuild,
  n=0 either way; (C) the lever belongs in skills so every command inherits it — if wrong, skill
  text is weaker delivery than a command rule for a lever that must fire at run-open.
- **Out of scope** *(lead's guess; **confirmed by the user 2026-10-25: "confirm all five"**)* —
  the other five commands' own flows (inherit by pointer only — a skill change they point at is
  inheritance, command surgery is out) · the Rust CLI grammar (a new condition key fits the
  existing `when:` form) · the code-standards low/high dial itself (asserted code rules, a
  separate axis from process depth) · the feature map · the architecture desk.
- **Size** *(lead's proposal; **ruled by the user 2026-10-25: "as recommded"**)* — standard.
  Fog at ruling: which stakes key · which steps the lead may depart · skill vs command home ·
  author≠grader form · fate of three prior rulings — five patches, inside implement plus the skills
  it points at.

User's first reply (2026-10-25, verbatim): "i want to question author not grader too. I want to
keep that as a concept, which is adversirial review, however, lead has flexilibty to define how it
wants that done." Applied as the destination amendment and the must-not-break strike above; every
other line was then confirmed, corrected or ruled in the same reply chain (stamps on each line).

- **Reach** *(open line on the card; **ruled by the user 2026-10-25: "yes, universal ."**, put
  with the case against the lead's lean first after a four-ruling ratified streak)* — a standing
  principle ruled here: the lead holds process depth under the floor, landing in
  `patterns-sound-loop` and its siblings; every command pointing at them inherits on its next run;
  implement is the first build, no command surgery (out-of-scope fence 1). Rejected: implement-only
  with a scope clause inside the shared skill — a skill saying two things to two callers.

**Frame hardened 2026-10-25** — every line confirmed, corrected or ruled by the user's word
(must-not-break "confirm all five" · betting-on "confirm all three" · out-of-scope "confirm all
five" · size "as recommded" · reach "yes, universal ."). From here a line changes only by the
user's word, one line at a time, each change stamped below.

Frame changes:

1. **Problem line, 2026-10-27** — "the lead seat under 131 delivered rules with zero latitude
   over steps" is struck; the line now ends: "the lead seat under 131 delivered rules has
   latitude over *how* — staffing, sequencing, teammate or subagent — and none over *which steps
   run* or *how deep*." Why: the blind map's A1 against F16 (`impl.staffing-latitude`) showed
   "zero latitude" overstated. User's word: "1 yes".
2. **Out-of-scope fence 1, 2026-10-27** — "the other five commands' own flows (inherit by pointer
   only — a skill change they point at is inheritance, command surgery is out)" is reworded to:
   "the other five commands' own flows — implement's floors are re-keyed in this session; another
   command's floors are re-keyed on its next touch, the shared-skill change reaching it by pointer
   until then." Why: the blind map's F1 and F2 showed that where a command pins a step with its
   own floor, inheritance by pointer is nominal; the reach ruling stays universal as the
   principle. User's word: "2 (a)". Rejected: widening to all six commands' floors now (a bigger
   build on the same ruling); leaving fence 1 as it stood ("universal" meaning nothing until
   someone else re-keys). The destination is unchanged, so no decisions need a re-check.
3. **Reach line, 2026-10-27 (at review, S1)** — one clause added: "the lever binds the
   input-job reviews of product work in any command; the two gate jobs — the primitive-edit
   audit, setup's governance validation — keep their fixed forms, and brainstorm's blind-map cold
   review, an input job, keeps its kept conditions; a stakes key for non-product surfaces is a
   named later session (OQ2)." *(wording corrected at the verify pass, N1)* Why: the cold review's
   S1 showed the five invariants, placed library-wide, would reach reviews
   `author-grader-consolidation` D2, D3, D7 fixed on purpose, and that the harm classes have no
   meaning for a spec, a governance file or a record. User's word: "i will go with
   recommdation" (A with the later session named, over B widening now and A plain). The
   destination is unchanged; D4 and D8 carry the scoping.

- F20 — *(fact only the user holds, asked 2026-10-25 as a labelled fact question, answered
  2026-10-27)* — what drives the "not adaptive" concern. User's words: "3 other repo run". Reading
  (lead's): the driver is an implement run in another repo, under an earlier plugin version than
  the 0.120 engine rebuild; the repo, the run, and what felt heavy are not yet named — asked as a
  one-line fact question beside Q1 and carried into Q9 (evidence plan) as the replay case.
- F21 — implement render (`mochiko-cli rules implement --section impl.sec.ways-of-working
  --plugin-root plugins/mochiko`, 2026-10-25 scratch copy): "Run-open stays one confirmation";
  `impl.sec.reserved`: "Reserved to the user at run-open: each store trip (ruled there, or
  deferred on the record) · each in-flight conflict · any sufficiency clause the grader could not
  clear."; `impl.user-runopen-rulings` is listed "Writer held: … the run-open rulings
  (impl.user-runopen-rulings), written into the sufficiency report and binding the run." Reading
  (lead's): a user confirmation already exists at run-open, with a home for its rulings — a
  stakes read put there adds no gate.
- F22 — `mochiko:review-sufficiency` description (plugin skill listing, 2026-10-27): "grading a
  capability-batch's guidance sufficiency at `/mochiko:implement` entry — the ten-clause check
  per selected work row … over the spec, the architecture store, and the product baselines,
  emitting a binding per-row `sufficient` verdict or the gap list … Never reads code, `tasks.md`,
  `**TEST:**` cases, or cycle reports. Defaults to FAIL … run by a seat that authored none of the
  graded sources." Reading (lead's): the entry grader already reads every surface the harm
  class is read from, before the lead's run-open line; it does not read the diff, so it cannot
  read the undo residue — which D3 dropped as an axis anyway.

## Decision map

Opened 2026-10-25 at frame hardening. Seats: review seat `review-seat` (`mochiko:devils-advocate`
on `mochiko:review-brainstorm`, persona pin, no deviation) spawned blind with the frame's problem,
destination and out-of-scope lines only, drawing its angle map to `reports/angle-map.md`; blind
second-list seat `second-list-floor` (persona-less `general-purpose`, `model: opus`) on Q1, the
frame as its opening, writing `research/second-list-floor.md`. Each seat writes its own file; the
lead holds the record pen. Installed plugin 0.120.0 seats the skills; current rules rendered from
`plugins/mochiko` (0.122.0).

**Blind map landed 2026-10-25** (`reports/angle-map.md`, seat `review-seat`: 66 angles, 18
load-bearing, 10 classes, `record_contact: none`; one disclosed incidental exposure — the index
About line, frame text only). Fold below; items marked *[blind map X]* came from it. Three
load-bearing angles bear on rulings already made — the frame's problem line (A1), its reach and
fence 1 (F1, F2) — and one asks a fact only the user holds (A3); all four went back to the user
before Q1 was ruled. **Ruled 2026-10-27**: A1 landed as frame change 1, F1/F2 as frame change 2,
A3 as F20 (see the frame card's "Frame changes" and F20).

**Ruled or folded, in the order put** *(groomed at review, S10: every item below reached a card
or a default; nothing askable, nothing in fog)*
- Q1 — **The floor test.** → `D1-floor-four-questions` (2026-10-27). The inventory of the 40 *[blind map
  D1]* is produced under Q3 by applying D1; the misread-low check *[blind map G1]* is D1's
  non-author read; a floor the read may touch is re-classed, never left floor *[blind map B9]*.

- Q4 — **Adversarial review's form.** → `D4-review-form-invariants` (2026-10-27): five
  invariants ruled once for the library, the form ruled per run and named in the run-open line
  *[blind map B3 answered; E1's axes are the lead's per run]*.
- Q2 — **The stakes key.** → `D3-harm-band-smoke` (2026-10-27). Blind second list
  (seat `second-list-stakes`, opus, frame + question only; `research/second-list-stakes-key.md`,
  53 lines, 15 roads; disclosure: no session file read, the directory name seen in `git status`
  and one `ls`, no Explore dispatch made). The user's smoke-probe idea was outside both lists.
  *[blind map C2 answered: inputs are the spec's harm line and the built thing; B2 answered: no
  count enters the key]*.
- Q3 — **Departable set in implement.** → `D5-band-step-latitude` (2026-10-27)
  *[blind map D2 answered in the worked example; C4: the sufficiency check never moves]*. The
  residual-coverage rule carried from Q2 is folded into the trail's grammar (DF5: each lowering
  names what still covers the failure kind).
- Q5 — **The lever's home.** → `D8-home-sound-loop` (2026-10-27) *[blind map F1, F2
  answered: implement re-keys its own rules; the skill edit reaches every pointer on its next
  touch]*. Blind second list in hand
  2026-10-27 (seat `second-list-home`, opus, frame + question only;
  `research/second-list-home.md`, 107 lines, 8 roads; disclosure: no session file read, no
  Explore dispatch, every read a known-path direct call). Its top three: R6 hub and spokes (a
  small new sibling holds the key, the band principle and the trail; review invariants into
  sound-loop leg 2; each step's thinning in its owning skill) · R7 fold into
  `patterns-model-tiering` · R5 reuse the `depth` and `scope` conditions. Facts it adds: a
  run-open condition fits the command grammar today (`crates/mochiko-cli/src/model.rs:283-292`,
  `user-ruled` and `moment-resolved(<moment>)` resolutions; a skill receives the value
  `entry-derived`, as `testing-gap-finding` does with `depth`) — fence 2 confirmed; sound-loop's
  headline "The entry door never lowers the review" (`patterns-sound-loop/SKILL.md:9`) and
  three hard floors (`testing-end-user.strict-order`, `testing-gap-finding.probe-kit-breadth-invariant`,
  `review-seat-plan.fresh-peer-grader`) need a clause or stay out of the band. Its R8 — the
  stakes read proposed by the entry grading seat (`review-sufficiency`), not the lead — bears on
  D2 (already ruled) and goes back to the user as a possible amendment. A command's own floors
  outrank skill latitude, so implement's floors are re-keyed in implement's rules whatever the
  home *[blind map F1]*; a skill edit reaches brainstorm's cold review and setup's governance
  gate *[blind map F2]*. Unblocks Q8.
- Q7 — **Fate of prior rulings.** → `D6-prior-rulings-fate` (2026-10-27) *[blind map B1, B2,
  B3 answered]*.
- Q10 — **The third-swing card.** → `D7-third-swing-card` (2026-10-27) *[blind map B4
  answered]*.
- Q9 — **Evidence plan.** → `D9-evidence-users-trials` (2026-10-27): no watch item; the user's
  own trials on other projects *[blind map J1 answered by the user's word: declined]*.

- Q2b-amend — **Who proposes the harm class.** → D2 amended (2026-10-27): the entry grading
  seat proposes where the command has one, the lead where it does not.
- Q6 — **The lead's read made visible.** Folded to default DF5 (2026-10-27): D2 fixes the home,
  so only a line grammar remained — not a real choice; Q9 and Q10 hung on DF5's confirmation.
- Q2b — **Who rules the read.** → `D2-stakes-read-ruling` (2026-10-27): the lead proposes,
  the user rules at the existing run-open confirmation *[blind map C1 answered]*. Q6 now hangs on
  Q2 only.

- Q8 — **Rule class for departable steps.** Folded to default DF7 (2026-10-27) once D8 fixed
  the home: `must` + `when:` for the steps, one widened `latitude` rule, floors in sound-loop for
  the never-thins rule and the invariants.

**In fog** — none.

**Map empty 2026-10-27** — nothing askable, no fog, no open frame line; the blind map landed and
was folded (18 load-bearing angles) or listed as dropped (the end read keeps them); the defaults
batch (DF1–DF7, with DF8–DF11 added at review) confirmed.

**Defaults candidates from the blind map** (each shown by name when it arises, confirmed in the
batch): granularity per run / card / step (C3) · stakes change mid-run, ratchet-up only (C5) ·
unclear read reads high (C6) · vocabulary kept apart from the `depth` dial (C7) · where the choice
lives (C8) · skip and thin as separate axes (D3) · attempt economy stays at run-open (D4) ·
`impl.fail.gap-finding-missing` re-keyed on the `skip-unstated` model (D5) · one grade over several
cards (D6) · lowest form at lowest stakes (E2) · plan-QA leg in the menu (E3) · gap-finder stays
blind (E4) · epic members' mixed stakes (G5) · the ruling in a durable file (G7) · the dial also adds
rigor (G8) · rework depth after a regression (G9) · the disclosure line's grammar (J3) · the
promises-kept table shows skips (J4) · the build that ships this uses no adaptive rigor (J5).

**Dropped for the end read** (not folded; the review seat keeps them): A4, A5, A6, B5, B6, B7,
B8, B10, E5, F3, F4, F5, F6, G2, G3, G4, G6, H1–H6, I1–I4, J2, J6.

**Out of scope** — per the frame's confirmed line.

## Session trail

### Q1 — the floor test (put 2026-10-25; blind second list in hand: 15 roads,
`research/second-list-floor.md`, seat `second-list-floor`; its haiku Explore dispatch returned no
report, section reads done on the seat tier — disclosed)

Options put: **A** four-question test (does departing let the lead take someone else's call ·
misstate a result · hide a skip · break a machine check) — floor = user gates, outcome honesty,
trail, one non-author read of the built thing before acceptance, machine-checked record layer;
**B** trail-only minimal (blind road 14) — gates, no self-clear, trail, real infra when verifying;
**C** end-state floor (blind road 5) — the `kind: fail` set is the floor, every enforcing rule
departable. Rejected: keep the 40 floors as they stand (blind road 1 — accreted, almost no
latitude) · minimum-depth per skill (blind road 10 — the tier ladder 2026-08-01 rejected). Lead's
pick: A, with blind road 7 (stakes-invariance) as its one-line principle and road 15 (upward
departure free) carried as a default. Blind roads differing from the lead's list and shown to the
user: 5, 7, 8, 12, 10.

Re-put 2026-10-27 in plain words after the three frame items landed (user's reply "1 yes, 2 (a),
3 other repo run"); the options and the lead's pick unchanged. Ruled "as recommended" → D1.

### Q2b — who rules the stakes read (put 2026-10-27; streak form — case against the lead's lean
first, weak lean in place of a firm pick, after four yes-to-pick rulings in a row)

Options put: **(i)** the lead reads alone and states the read in the run-open line, departures
trailed; **(ii)** the lead proposes, the user rules — the read and the planned skips on the
existing run-open confirmation (F21), no new gate, recorded with the run-open rulings; **(iii)**
the user declares stakes up front, like the `depth` dial, and the lead never reads. Rejected: a
new gate of its own (run-open stays one confirmation, F21). Lead's weak lean: (ii). Case against
(ii) put first: the user asked for the lead to decide; a user who always says yes makes the
screen a stamp, and this session's own streak shows that tendency; (i) is what the words asked
for. Blind road 12 (three-tier authority) bears here and was shown. Ruled "yes as recommded" →
D2.

### Q2 — the stakes key (put 2026-10-27; blind second list in hand: 15 roads,
`research/second-list-stakes-key.md`; streak form again — case against the lead's lean first,
weak lean — after five yes-to-pick rulings)

Options put: **A** harm kind by undo cost (blind road 15, the seat's top) — two questions per
run, each answered from a cited line: the worst kind of thing that breaks if this is wrong, from a
closed list (money · data loss or corruption · security or trust · availability · wrong answer
shown · cosmetic), and whether a revert fully undoes it (residue in stored data, external systems,
or published contracts; the retrofit-cost categories of build-vs-off-the-shelf D4 and
adaptive-depth D5 as the cited residue list); the two together size each non-floor step; **B**
retrofit-cost categories alone (blind road 1, the cheap reuse) — deep where the run touches a
category already ruled expensive to undo, shallow elsewhere, one pointer to an existing ruled
line. Rejected: free judgment with a disclosure line (blind road 14) — the frame's bet A already
bets against it, and bet B says the appetite regrows; scope type and `ux_bearing` as the key
(blind road 13) — scope names the door, not the harm; the sufficiency gap count (road 5's
failure) — magnitude by proxy; per-promise stakes tags (road 10) — specs carry none today. Carried
to Q3 as a candidate rule, not a key: residual-check coverage (road 8) — a step is thinned only
where a later independent check covers the same failure kind. Lead's weak lean: A, with B's
category list as A's undo-axis input. Case against A put first: two judgments per run, both
arguable, and "worst plausible failure" drifts into blast radius, which is magnitude; it needs one
cited line per axis to stay honest, and the lead judges reversibility optimistically. Blind roads
differing from the decision map's C2 input list and shown to the user: 2/3/15 (undo cost, harm
kind, their grid), 8 (residual-check coverage), 10 (per-promise), 12 (escalation on in-run
evidence, now DF2), and the seat's not-inputs list (not magnitude · not the `depth` dial · not
scope type alone · not lead familiarity). Defaults shown by name here: DF3, DF4.

Re-put 2026-10-27 after the user asked for more options ("are there more options? think harder.
sort of like lead gets omeone to do sort of smoke test"). The user's idea opens a second family —
keys read from the running code, not from documents — which neither the lead's list nor the blind
list carried (provenance of the family: `user's words`). Menu widened to three families: read the
documents (A harm by undo · B risky-area list) · poke the code (C smoke probe after the build, the
plan ruled at run-open so the thinning is mechanical under D2 · D existing-checks coverage — thin
where an independent check of the same failure kind already exists on the touched surface · E
defect history — territory with past escaped defects reads high) · stage it (F full depth first,
thin after the first clean grade). Each with its failure: a clean smoke is a weak negative that
misses silent data corruption (`patterns-model-tiering.weak-negative-watch`); coverage says
nothing about new behaviour; new territory has no history; the first cycle is the skeleton and
proves little. Lead's lean changed to a pairing: A's first question (worst kind of harm) sets
which steps may thin at all — money, data, security never — and C's smoke probe decides inside
that band. Case against put first. Ruled "as recommended" → D3.

Fence disclosure (lead's): the stakes-key blind seat's brief carried one line of the ruled set —
D1's test and its upward-free default — as part of the question's constraints, which the
blind-list rule fences out ("never your options, the conversation or the ruled set"). The seat
saw no option of the lead's and no other ruling; the exposure is noted here for the cold read.
The departable-set seat's brief carries the frame and the question only.

### Q3 — the departable set (groundwork 2026-10-27; blind second list drawing, seat
`second-list-departable`, opus, frame + question only)

Facts before the fork: the lead applied D1's test to the 40 floors —
`research/floor-inventory.md`. Result: 39 stay floor (17 by question 1, 9 by question 2, 4 by
question 3, 6 by question 4, 3 invariants with no depth dimension), 1 re-classes
(`impl.fail.gap-finding-missing`, re-keyed on the `skip-unstated` model). Reading: the depth the
user feels is in the `class: must` layer, not the floors; the departable set is a re-keying of
must-class steps. The lead's candidate list of nine thinnable steps is in the same file, drawn
before the blind list lands; the fork is put when it does.

Blind list landed 2026-10-27 (`research/second-list-departable.md`, seat
`second-list-departable`, opus, frame + question only; disclosure: no fence exposure, the
directory seen as a name only; no Explore dispatch, a few inline greps instead — a small
departure from the tiering routing, stated by the seat). Its content: 16 steps with verdicts
(SKIP · THIN · NEVER) and the defect each prevents, cited to strips and `DECISIONS.md` rows; a
floor split keeping 31 and re-classing 9 under its own test ("a decision the user owns or an end
state the user is owed" stays floor; "a procedure toward such an end" is a means); three cheap
roads; seven prior rulings contradicted.

Where the blind list and the lead's list agree: the shape — shrink the graders' form, rarely
skip; record-layer steps never move; the regression sweep thins to touched territory and never
skips; gap-finding is the one large step that may be skipped stated; widening
`impl.fail.skip-unstated` to every skipped or thinned step is the whole trail (its cheap road 2 =
the lead's inventory note). Where they differ, shown to the user: (1) floors — it re-classes
eight more than the lead (`baseline-diff-review`, `fail.unreviewed-baseline-diff`,
`artifact-home`, `no-progress-stop`, `minimalism-advisory`, `design-audit-advisory`,
`fail.sufficiency-unrecorded`, `fail.card-unchecked`); the lead keeps them — six change nothing
in practice (hook-enforced or advisory ceilings), and `fail.sufficiency-unrecorded` is where D2's
run-open rulings land (F21); shown as default DF6. (2) Cold verification — the lead had the smoke
replace it; the blind list says the cold run must be the one final gate run, skipped only when no
build or dependency file is in the diff and CI gates the pull request; the lead adopts this: the
smoke runs cold. (3) Steps the lead's list lacked: the code-minimalism lens (SKIP, stated), the
design-stage blind list at one or two gaps (SKIP; a precedent exists in the rule), the landing
verifier (THIN by a `sonnet` worker on the mechanical checks, never skipped), the card-review /
design-pair double grade of card quality on the gap path (fold). (4) A sharper principle than
the lead's: a code revert undoes code, never a shared record (baselines, store, `gates.md`, the
map) or a ruling taken from the user — adopted as the never-move test. (5) Four prior rulings the
lead's map lacked, added to Q7: `producer-plan-enforcement` D6 (no size carve, `DECISIONS.md:53`),
`validator-worktree-isolation` D0 (always-cold, `DECISIONS.md:122`),
`qa-gap-finding-verification` D2 (pass keyed on scope, `DECISIONS.md:75`),
`command-architecture-realignment` D1/D2 (the trail it killed, `DECISIONS.md:115`).

Options put: **A** shrink, rarely skip — every verification step keeps running in a smaller
form in the band; three may be skipped outright, stated: gap-finding, the code-minimalism lens,
the design-stage blind list at one or two gaps; record-layer steps never move; **B** skip freely
in the band — any must-class verification step may be skipped stated, D1's floors standing;
**C** do nothing new (blind cheap road 1) — route low-stakes changes as delta cards through
`/mochiko:feature`, which already skips gap-finding stated and re-verifies thin. Rejected: B walks
the surface-value tiering road `author-grader-consolidation` D1 recorded-rejected, and a
regression in another feature goes unseen until after acceptance; C keys on shape, not stakes —
a cosmetic change in a selection run still pays full. Lead's lean: A; case against first: it is
the most rules to write (a verdict per step) and it contradicts seven prior rulings that Q7 must
then supersede one by one. The user asked "A vs B?"; the lead compared and revised its lean to B
with two edits; ruled "as recommded" → D5.

### Q7 — fate of prior rulings (put 2026-10-27 as one screen; rows verified against
`DECISIONS.md` 2026-10-27)

Seven rows, each with the lead's proposed fate; the user confirms, edits or strikes rows:

1. `production-floor-adaptive-depth` D8-dial-standards-scope (2026-08-11, `DECISIONS.md:91`) —
   "pipeline process rigor stays uniform". Proposed: **superseded in part** by D3 and D5 — process
   depth keys on stakes; the `depth` dial keeps code standards only (DF4).
2. `charter-ritual-balance` D1–D2 (2026-08-13, `DECISIONS.md:84`), the kind-keyed sound-loop
   trigger, "magnitude never gates it" (F17). Proposed: **kept, one clause added** — the trigger
   stays kind-keyed and stakes is a harm kind, not a size; what stakes keys is the depth of the
   legs after the trigger fires, never whether it fires.
3. `author-grader-consolidation` D1 universal independence, surface-value tiering
   recorded-rejected (2026-09-19, `DECISIONS.md:32`; record line 202). Proposed: **amended in
   part** — independence stays universal at run grain (one non-author read before acceptance,
   floor); the grade's grain and form are the lead's (D4, D5); the rejection stands for whether a
   grade happens, not its form.
4. `producer-plan-enforcement` D6 no size carve (2026-09-03, `DECISIONS.md:53`). Proposed:
   **amended in part** — plan-QA once per seat per run in the band; still no carve by size, the
   carve is by stakes band.
5. `validator-worktree-isolation` D0 always-cold (2026-08-01, `DECISIONS.md:122`). Proposed:
   **amended in part** — the cold run is the one final run; skipped only when no build or
   dependency file is in the diff and CI gates the pull request, stated (D5).
6. `qa-gap-finding-verification` D2 gap-finding keyed on scope (2026-08-19, `DECISIONS.md:75`).
   Proposed: **amended in part** — keyed on scope and on the stakes band; the skip stated in either
   case.
7. `command-architecture-realignment` D1–D2 (2026-08-02, `DECISIONS.md:115`), which dropped the
   departure trail, the weight card and the counted bounds. Proposed: **superseded in part** — the
   trail returns as one widened fail rule and one line per lowering (DF5), never as a weight card;
   counted bounds stay dropped. `lead-owned-process-flexibility` (2026-08-01, `DECISIONS.md:126`)
   D2 and D5 revive in part — Q10's card says what differs this time.

Ruled "confirm all seven" → D6.

### Q6 — the lead's read made visible (shown as a default 2026-10-27, DF5)

With D2 fixing the home, what remains is a line grammar — a default, not a fork.

### Q10 — the third-swing card (put 2026-10-27 as a drafted card for the user's word; DF5
pulled forward for confirmation in the same turn, since Q9 hangs on it)

Drafted: six lines on what differs from 2026-08-01 and 2026-08-02 — (1) one question plus
evidence in place of a four-factor paper key and a weight card; (2) the user rules each plan at
run-open, so the trail is not the only control; (3) the trail revives as its job (a stated skip),
not its instrument (a card) — one widened fail rule and one line per lowering; (4) the lever
lands in skills and the command only keys and points, where the 131 grew by adding floors and
fails to the command; (5) counted bounds stay dropped; (6) the two ways this swing fails the same
way, named as Q9's watches — the run-open line growing into a card, and D4's examples hardening
into a menu. Rejected: reviving the four factors (a paper read the user called for pragmatism
against); no card (the cold review's first question). Ruled "as recommded" with DF5 → D7.

### Q2b-amend — who proposes the harm class (put 2026-10-27; late road R8 from the home blind
list bearing on D2, so back to the user)

Options put: **(i)** amend D2 — where the command has an entry grading seat (implement's
sufficiency seat, F22) that seat proposes the harm class with its cited line as one more field
of its report; the lead carries it to the run-open line and may raise it, never lower it; the
user rules; where no entry grader exists the lead reads, as D2 says; **(ii)** keep D2 as ruled —
the lead reads. Lead's lean: (i). Case against (i): one more hop and one more field; the
sufficiency seat grades guidance, not risk, and never reads the diff. For (i): the lead reading
the stakes of its own run is the under-classification risk the sound-loop floor already names
one level down (`patterns-sound-loop.only-real-seats`, per the blind list); the entry grader
already reads every surface the class is read from, authored nothing, and defaults to FAIL.
Ruled "as recommended" → D2 amended.

### Q5 — the lever's home (put 2026-10-27; blind second list in hand: 8 roads,
`research/second-list-home.md`)

Options put: **A** hub and spokes (blind road R6; the lead's lean) — one small new sibling skill
holds the stakes key (the harm classes, the run-open read and ruling), the band rule (what never
thins), the trail line grammar and the worked example; the five review invariants (D4) go into
`patterns-sound-loop`'s leg 2, where non-author review already lives; each step's lighter form
stays in the skill that owns the step, keyed on `stakes` declared `entry-derived`; implement
declares `stakes` as a `moment-resolved(run-open)` condition, one floor `pointer:` to the hub,
`when: stakes=…` on its thinnable must-rules, widens `impl.staffing-latitude` to which and how
deep inside the band, and widens `impl.fail.skip-unstated`; **B** fold into
`patterns-model-tiering` (blind road R7) — stakes as a third key beside the class key and the
seat default key, the band as a ladder, the trail on the roster disclosure, no new primitive;
**C** amend `patterns-sound-loop` alone (blind road R1; the lead's earlier option (a)). Rejected:
reuse the `depth` and `scope` conditions (R5 — `depth` is project-wide, set at setup, not a
per-run read; bet A unserved; leans on the out-of-scope dial); command rules only (R4 — against
the destination and bet C); split every part to its nearest owner with no hub (R3 — five skills
with no single definition of the band). Case against A first: a fifth sibling in a library the
frame says regrew, and two hops to read. Blind roads the lead's list lacked, shown: R6's hub
shape itself, R7, R8 (now D2's amendment), R5. The user asked "why cant we fit in already
existing skills"; the lead measured the hosts (sound-loop 14 rules, 509 description chars;
model-tiering 22 rules, 1,215 chars — corrected at review, S11) and revised its lean to sound-loop; ruled "as recommded"
→ D8. Q8 folded to DF7.

### Q9 — the evidence plan (put 2026-10-27; the last askable item)

Options put: **A** one watch item in `BACKLOG.md` on the existing n=0 first-live-run pattern
(F18, lines 444/483/501) — observable: seats and rounds on band runs against the full-depth
count for the same run class, read from the run report's roster line and promises-kept table
(no new field), and escaped defects after acceptance on band runs; three triggers with a
one-rule-edit revert each: an escaped defect a thinned step would have caught → that step
returns to unconditional `must`; the run-open line exceeds its five fields → the key is growing
a card, groom; a rule naming a review form as required → the menu is back, groom; horizon: the
first five band runs, read at the next grooming; dogfood target: the first ai-fileops implement
run (F18, `BACKLOG.md:35`) and the user's other-repo run class as the replay (F20, OQ1); **B**
the trail alone — no watch item, the trails read when something goes wrong; **C** A plus a
per-run counter field in the final-validation report. Rejected: C — the roster line already
carries the count; B — trails go unread until a defect, and D7's two watches never get counted.
Lead's pick: A. Case against: a fourth n=0 watch in a file with size bounds. The user declined
all three ("i think we can ignore this, i will try on other projects") → D9. Map empty.

## Decisions

### D1-floor-four-questions — The four-question floor test, under stakes-invariance

**Statement.** A rule stays `class: floor` — non-departable at any stakes level, by any seat, by
any in-run ruling — only if departing from it would let the lead (1) take a call that is someone
else's, (2) misstate a result, (3) hide a skip, or (4) break a machine check. Every other rule now
carried as floor is re-classed so the lead may depart it under a trail. At the family level the
test keeps five things floor: the user's gates · outcome honesty, including no output cleared by
its author · the trail of what was skipped or thinned · one non-author read of the built thing
before acceptance · the machine-checked record layer. One principle governs the test: a floor is
whatever holds the same at every stakes level. One default rides with it (confirmed in the batch):
raising rigor is always free; only lowering it is trailed. The check that survives a misread-low
is the non-author read before acceptance (question 2) — no stakes level removes it.

**Why.** The user's words: "I want lead to make active decisions in terms of what needs to
followed, and how deeply. it needs to pragmatic." Implement delivers 40 floors, and the lead's
latitude today is over how, never over which steps or how deep (F7, F16, frame change 1). The four
questions name the harms encoded rigor exists to prevent — usurped gates, false results, hidden
skips, broken machine checks — and nothing else; the rest of the 40 are means toward those ends,
and means are what a pragmatic lead sizes. Stakes-invariance is the tie-breaker: a rule that would
sensibly differ for a one-line fix and a payments rewrite is not a floor.

**Rejected roads.** B, trail-only minimal (blind road 14) — smaller and harder to game, but it
drops the non-author read and the machine-check line from the floor, both inside the frame's
must-not-break (engine rulings, CLI delivery). C, end-state floor (blind road 5) — makes the 17
fail conditions the floor, but a fail condition is the contrapositive of a rule and several enforce
means (`impl.fail.gap-finding-missing` enforces a step, not an outcome); it re-encodes the same 40
by the back door and leaves the fail set's own regrowth ungoverned. Keep all 40 (blind road 1) —
the status quo this session exists to change. Minimum depth per skill (blind road 10) — a tier
ladder, rejected 2026-08-01. The detection test (blind road 8) — folded inside question 3 (a skip a
reviewer cannot detect is a hidden skip), not the test itself. Three-tier authority (blind road
12: nobody · the user at run-open · the lead by trail) — not rejected; handed to Q2b and Q3, which
rule who departs the rest; D1 rules only what is floor.

**Accepted risk.** Question 1 is arguable rule by rule, so the inventory of 40 could drift toward
keeping most of them. Mitigation: the inventory is produced under Q3 as a list, each kept floor
citing the question it passes, and each checked against stakes-invariance for the smallest and
the largest run. Second: the trail (question 3) is itself only honesty — a lead under context
pressure can skip and trail falsely; the non-author read (question 2) is the detection, and no
cheaper detection exists.

**How it was decided.** Lead's pick A, ratified by the user ("as recommended", 2026-10-27) — the
fourth yes-to-pick in a row (reach, frame change 1, frame change 2, Q1), flagged and the next fork
put in another form. Blind second list of 15 roads drawn before the fork
(`research/second-list-floor.md`); roads 5, 7, 8, 10, 12 shown to the user as differing from the
lead's list. Provenance: `lead's guess, ratified`; the stakes-invariance principle (road 7) and
the upward-free default (road 15) are `blind list`.

**Changed at review.** —

Confidence: Confident.

### D2-stakes-read-ruling — The lead proposes the stakes read; the user rules it at the existing run-open confirmation

**Statement.** *(amended 2026-10-27 by the user's word, see how-it-was-decided)* At run-open the
harm class (D3) is proposed by the command's entry grading seat where it has one — in implement
the sufficiency seat (F22), as one more field of its report, the class with its cited line —
and by the lead where no entry grader exists; the lead carries it to the run-open line and may
raise it, never lower it. The lead puts one line on the run-open confirmation implement already
has (F21): the read and its source, the inputs it cites, and the steps it plans to skip or thin. The user's word binds the run; the line
is recorded with the run-open rulings in the sufficiency report. No new gate — run-open stays one
confirmation. After run-open the lead raises rigor freely (DF1); a lowering after run-open is a
checkpoint ruling of the user's, never the lead's alone, since the read it changes was the user's.

**Why.** The lead sizing how much checking its own run gets is itself an author-and-grader
question *[blind map C1]*; the user's gate at run-open already exists, so one line there costs
nothing and makes the run's rigor a visible ruling of the user's — must-not-break item 1 (user
gates stay the user's) and the destination's "ruled, visible choice" both land on it. The lead
still does the active reading the user asked for; the user only confirms or raises.

**Rejected roads.** (i) The lead reads alone, trails departures — the user's words point here,
but the read then has no check at the moment it matters and the trail reports only after the
fact. (iii) The user declares stakes up front like the `depth` dial — the user holds fewer
surfaces at run-open than the lead does, and adaptive-depth D1's dial is a standing governance
level, not a per-run read. A new gate of its own — run-open stays one confirmation (F21). Blind
road 12 (three-tier authority) is taken in part: floors nobody waives (D1); the rest proposed by
the lead and ruled by the user at run-open; a "lead by trail" tier exists only for raising (DF1).

**Accepted risk.** Default-accept fatigue: the line becomes a stamp — the case the blind seat's
road 11 and this session's own ratified streak both make. Mitigation: one line on an existing
screen, never a screen of its own; a read the user never raises shows only on the user's own
trials (D9 — nothing in this repo watches it);
a stamped wrong read is still the user's ruling, which is the design, and the non-author read (D1)
stays as the detection at every stakes level.

**How it was decided.** Lead's weak lean (ii), put in streak form — the case against it first —
after four yes-to-pick rulings; ratified by the user ("yes as recommded", 2026-10-27), the fifth
in a row, flagged again. The blind stakes-key list's road 11 was shown before the ruling as the
same case against. Provenance: `lead's guess, ratified`. **Amended 2026-10-27**: the home blind
list's R8 (`research/second-list-home.md`) bore on this card after it was ruled, so it went
back to the user as Q2b-amend; the user ruled "as recommended" for the entry grader proposing
the class — provenance `blind list`. The rejected road for the amendment: keep the lead as the
sole reader (one hop fewer, but the self-rating risk the sound-loop floor names one level down).

**Changed at review.** S2 (Important, 2026-10-27): the accepted-risk line no longer leans on
the declined Q9 watch; the user's own trials (D9) are where a never-raised read shows.

Confidence: Confident.

### D3-harm-band-smoke — The harm class sets the band; a smoke probe decides inside it

**Statement.** The stakes key has two parts. (1) At run-open the lead answers one question from a
cited line: the worst kind of thing that breaks if this run is wrong, from a closed list — money ·
data loss or corruption · security or trust · availability · wrong answer shown · cosmetic. The
class sets the band: for money, data and security no non-floor step thins, whatever any probe
says; for availability, wrong answer and cosmetic, the steps Q3 names may thin on a clean smoke.
(2) The smoke probe: at the point the ruled plan names — by default after the build, before final
validation — an independent seat that wrote no product code in the run builds the thing, runs the
suite, starts it on real infrastructure and probes the changed behaviour by hand, and holds the
diff against the ruled harm class — a diff touching a money, data or security surface voids the
thinning and the run goes full depth (DF2) *(added at review, S5)*; it runs under
`mochiko:testing-end-user` by the run's QA seat (`mochiko:qa-engineer`, which wrote no product
code in the run — it authors `**TEST:**` cases, never the code they test — on its persona tier
*(wording corrected at the verify pass, N3)*) and lands as a `verification` report carrying a `smoke: true` field,
the envelope admitting extra fields *(added at review, S8)*; its form within that is Q4's and
the model-tiering floor's. The lead's run-open line (D2) proposes the harm class
with its cited line, the smoke's scope, and the named steps that thin on a clean smoke; the user
rules it; a clean smoke then thins those steps as mechanical execution of that ruling, and any red
runs full depth. A clean smoke is a weak negative, reported method-scoped — "clean by <probes>
over <surface>" — never a verdict of safety; the band exists because of that. The key is never
magnitude: no count of files, cards, rows, gaps or users enters either part (DF4 keeps its
vocabulary apart from the `depth` dial).

**Why.** The user's idea: real evidence from the built thing beats a paper read, and it is the
pragmatic shape they asked for. A paper read alone asks the lead for judgment calls it will make
optimistically under context pressure; a probe alone misses the failures that do not show on a
smoke — silent corruption, a wrong charge — so the harm class fences those out of thinning. One
question at run-open plus one bounded probe keeps the lead's active decision and gives it
evidence to decide on.

**Rejected roads.** A alone, harm by undo cost — two paper judgments per run and no evidence;
its undo question is dropped as a separate axis, its residue list (stored data, outward sends,
published contracts) surviving as the cited line that makes a run read "data" or "money". B
alone, the risky-area list — misses visible harm with cheap retrofit. C alone with a fixed
never-thin rule — the cheapest shape, but the fixed rule still needs the harm class read to apply
it, so it is the same question unnamed. D, existing-checks coverage — old checks say nothing about
new behaviour; carried to Q3 as the rule for *which* steps may thin (a step thins only where
another independent check covers the same failure kind). E, defect history — new territory reads
low exactly when it should not. F, full depth first then thin — the first cycle is the skeleton
and proves little. Free judgment with a disclosure line — the frame's bet A. Scope type — the
door, not the harm. Gap count — magnitude by proxy. Per-promise stakes tags — specs carry none.

**Accepted risk.** Two mechanisms, and the smoke seat is one more seat on every run that might
thin — on a small run the spawn can cost more than the steps it saves (the dispatch ladder's own
test). Mitigation: the run-open plan may say "no smoke, full depth" where nothing would be saved,
and the smoke's scope is bounded to build · suite · start · hand probes. Second: a clean smoke in
the availability, wrong-answer or cosmetic band can miss a defect the thinned step would have
caught; the non-author read before acceptance (D1) and gap-finding where it runs remain; the
revert trigger is the user's word after their own trials (D9) — nothing in this repo fires on its
own, which the user re-confirmed at review (S2). Third: adaptive-depth D8 ("process rigor stays
uniform") is superseded in part by this card — Q7 records it. Cost, *Assumed* (added at review,
S14): on a cosmetic three-cycle run the band saves two of three per-cycle grades, the separate
cold-verification run and the gap-finding pass against one smoke dispatch added, which doubles
as the cold run; on a money, data or security run it adds the smoke and saves nothing — the
plan may then say "no smoke, full depth".

**How it was decided.** The lead put A and B; the user asked for more ("are there more options?
think harder. sort of like lead gets omeone to do sort of smoke test") and so opened the
poke-the-code family; the lead's lean moved to the pairing, put with its case against first;
ratified "as recommended" 2026-10-27. Sixth yes-to-pick in a row, but on an option the user
shaped — read as engagement, the streak watch reset. Provenance: `user's words` for the smoke
family; `lead's guess, ratified` for the pairing; the harm-class list is `blind list` (roads 3
and 15, `research/second-list-stakes-key.md`).

**Changed at review.** S2 (2026-10-27): the revert trigger is the user's word after their own
trials, re-confirmed ("confirmed"). S5: the smoke seat holds the diff against the ruled harm
class; a fenced surface in the diff voids the thinning. S8: the smoke runs under
`mochiko:testing-end-user` by the QA seat and lands as a `verification` report with `smoke: true`.
Verify pass (2026-10-27, N3): the QA seat described as having written no product code, since it
authors `**TEST:**` cases; round 3 (user: "confirmed"): the sentence's opening clause changed
to match.

Confidence: Confident.

### D4-review-form-invariants — Adversarial review keeps five invariants; the lead defines the form per run and names it

**Statement.** Adversarial review stays as a concept on every run: before acceptance, something
built or authored in the run is read by a seat that did not author it. *(Scope, added at review
— S1.)* This card binds the input-job reviews of a run's built product work — implement's cycle
grades, its final validation, the design phase's drafts — and the same job in any command that
reviews product work. Named here so nobody reads them in: the two gate jobs
`author-grader-consolidation` D2, D3 and D7 fixed — the primitive-edit audit (GI-004) and
setup's governance validation — keep their fixed forms; brainstorm's blind-map cold review, an
input job, keeps the conditions consolidation D8 kept (the blind angle map first, the completed
hunt); and those D8 conditions stay for every input job they name *(wording corrected at the
verify pass, N1)*. Within that scope the form is the lead's to define per run, bound by five
invariants — plus, for the input jobs consolidation D8 names, its kept conditions — and
nothing else *(reworded at round 3, N1)*: (1) the reader never authored what it
reads; (2) it reads the surface itself, never the author's report; (3) it starts at FAIL; (4) it
runs at or above the tier the work was produced at, its model named on the spawn (the
model-tiering floor's persona-less pin); (5) where the form is a blind hunt, the reader stays
blind — the two-message dispatch, nothing of the author's reasoning in its brief. The form is
named in the run-open line (DF5) so the user rules on it with the rest of the plan (D2). Named
forms — a cold pair review by a blind persona seat · one blind seat reading solo · a delta read
of only the changed surface · the smoke probe (D3) · a plan-QA leg — live in the skill as
examples only, never as a closed menu; a form the lead invents is legal when it meets the five
and is named in the line.

**Why.** The user's words: "I want to keep that as a concept, which is adversirial review,
however, lead has flexilibty to define how it wants that done." The five invariants are what
"adversarial" means structurally — independence, the surface, the default verdict, the tier, the
fence; everything beyond them is shape, and shape is where a pragmatic lead sizes. Naming the
form in the run-open line, not a menu, is what keeps drift visible: the user sees "delta read"
three runs running and can raise it. A menu would add rules to a layer this session says regrows
(frame bet B).

**Rejected roads.** (a) A short fixed menu ruled once here — the lead's lean; it gives the user
a known cost and coverage to rule on, but it is more rules, and its forms survive as examples
anyway. (b) Two forms only, full or light — cheapest, but no middle shape, so a medium run pays
full. (c) as first put with three invariants — too few: the tier pin and the blindness fence
already bind from the model-tiering and gap-finding skills and would have bound silently.

**Accepted risk.** Under context pressure the lead's inventions converge on the lightest shape
that passes the five. Mitigation: the form is named where the user rules (D2, DF5), the
non-author read before acceptance is floor (D1) whatever the form, and the forms chosen show in
each run's trail, read on the user's own trials (D9 — no watch in this repo). Second: invariant 5 ("where the form is a blind hunt") leaves the lead
to say whether a form is a hunt; a delta read is not, a gap-finding pass is; a form the lead
calls "not a hunt" to skip the fence is a hidden skip under D1's question 3.

**How it was decided.** The lead leaned (a); the user asked "why not c?"; the lead gave the
case both ways and recommended (c) with two added invariants; the user ruled "c as amended"
(2026-10-27). Provenance: `user's words` for the concept and the freedom of form; `lead's
guess` for the five invariants and the naming in the line. The ratified streak ended here.

**Changed at review.** S1 (Critical, 2026-10-27): scoped to the input-job reviews of product
work; the three gate jobs and consolidation D8's conditions named as kept. The user ruled the
scoping over widening the freedom library-wide ("i will go with recommdation"); the widening's
gap — a stakes key for non-product surfaces — is a named later session (OQ2). S2 (2026-10-27):
the accepted-risk line no longer leans on the declined Q9 watch. Verify pass (2026-10-27, N1):
brainstorm's cold review re-described as an input job with kept conditions, not a gate job;
"nothing else" scoped to "within this scope"; round 3 (N1, user: "confirmed"): the invariants
line now carries the kept conditions explicitly — "five invariants, plus the kept conditions
for the input jobs that have them, and nothing else".

Confidence: Contested.

### D5-band-step-latitude — In the band any must-class check may thin and four may be skipped, stated; record-guarding steps never move; the standard thin plan is the worked example

**Statement.** In the thinnable band (D3: availability, wrong answer shown, cosmetic), with a
clean cold smoke, the lead may thin any `class: must` verification step of the run — the
per-cycle verification grades, the card review before confirm, the per-cycle full suite, the
regression sweep's breadth, the design review pair, the plan-QA leg — and may skip only the
four non-leg steps — the gap-finding pass, the design audit, the code-minimalism lens, the
design-stage blind list — *(amended at review, S3: a review leg thins, never vanishes — the
sound-loop floor keeps every leg, and stakes keys their depth, never whether they fire, D6 row
2)* — on three conditions: each
thinning or skip is proposed in the run-open line and ruled by the user (D2); each is recorded in
the trail (DF5); and D1's floors stand — one non-author read of the built thing before
acceptance, no output cleared by its author, gates never triaged, real-infrastructure evidence,
the trail, the machine-checked record layer. Never move, as a rule, not a judgment: every step
that guards a shared record — the baseline diff review's mechanical legs (unmarked-write test,
duplicate-id check, landing flip), the built-vs-signed store diff, the landing verifier on any
run that wrote a baseline, the store or `gates.md`, and the sufficiency check at entry — because
a code revert undoes code, never a shared record or a ruling taken from the user. The cold run is
the one final gate run: the smoke runs in a clean snapshot, never a warm run followed by a cold
one; it always runs, on the exact head the user accepts, and repeats when rework changes code
after the smoke, within the existing gap-rework bound *(amended at review, S7: the former
CI-gated skip clause is dropped — implement never pushes, so no CI result exists at acceptance,
and `impl.cold-verification` makes the cold results acceptance evidence)*.
`impl.fail.skip-unstated` widens from the skipped gap-finding pass to every
thinned or skipped step at every scope; `impl.fail.gap-finding-missing` takes its stated-skip
form (`research/floor-inventory.md`). The standard thin plan lives in the skill as the worked
example the lead cites by name in the run-open line instead of inventing one: one non-author
grade over several small cycles · affected tests per cycle, the full suite once in the cold
smoke · regression gates of touched territories only · design review by one seat running
`mochiko:review-plan-artifacts`, whose meaning-kept grade fills the design checkpoint's landing
table (`impl.gate-design-checkpoint`), with the feasibility lens folded in where a store delta
exists · plan-QA once
per seat per run · gap-finding, the code-minimalism lens and the design-stage blind list (at one
or two gaps) skipped, stated. Outside the band nothing thins (D3); raising is always free (DF1).

**Why.** The user's words: the lead decides. D2 makes each run's plan the user's ruling, so the
lead's freedom is bounded per run by the user's word rather than by a table in the rulebook —
the shape D4 chose for review forms (invariants as rules, forms as examples), taken here for
depth. The never-move rule exists because the one harm a revert cannot undo is damage to a
shared record or a ruling taken from the user — the blind list's line, adopted. The worked
example exists so a lead under pressure has a tested default to cite rather than "smoke, then
accept".

**Rejected roads.** A, shrink-rarely-skip as rules — sixteen verdicts written into a layer the
frame bets regrows, and a lead executing a table rather than deciding; its content survives as
the worked example. B as first put — let the lead skip the record-guarding steps too; a revert
does not undo those. *(Added at review, S4 — the two objections the lead raised against B at Q3
and dropped on switching to it, answered here.)* First, that B walks the surface-value tiering
road `author-grader-consolidation` D1 recorded-rejected: with S3's narrowing no grade is skipped
by stakes — grades thin; the three skippable review-shaped steps are the code-minimalism lens
and the design audit, both advisory by floor (`impl.minimalism-advisory`,
`impl.design-audit-advisory`: findings never block at any severity), and the gap-finding hunt,
which is not advisory — a spec-required finding fails the final validation
(`impl.finding-kinds`) and `impl.fail.spec-gap-unresolved` is floor — so its stated skip is the
one blocking check the band gives up, with the smoke's hand probes as its lighter form and the
non-author read before acceptance (D1) still standing; D6 row 3 is restated to bend D1 for
exactly those three *(corrected at the verify pass, S4 second round; the blocking claim
corrected again at round 3, N4; user: "confirmed")*. Second, that a regression
in another feature goes unseen until after acceptance: the cold smoke runs the full automated
suite, so only hand-run real-infrastructure gates of other features go unrun, and only in the
band — the accepted risk below names it. C, do nothing new (route low-stakes work as delta cards through
`/mochiko:feature`) — keys on shape, not stakes; a cosmetic change in a selection run still pays
full. Thinning the smoke's suite run to touched modules — the full suite is what covers the
automated gates of every other delivered feature, and it runs once.

**Accepted risk.** Plans converge on the lightest shape under pressure; the user rules each
plan, the example anchors it, and the trail shows the forms and skips, read on the user's own
trials (D9 — nothing in this repo fires on its own, which the user re-confirmed at review, S2).
Non-automated
real-infrastructure `**TEST:**` gates of other delivered features go unrun when the sweep thins
to touched territory — accepted in the band only, where no money, data or security harm is in
play; `impl.fail.regression` stays floor as the end state. The per-cycle grade's grain moves from
artifact to run, bending `author-grader-consolidation` D1's "every reviewable artifact" — Q7.
Seven prior rulings are superseded in part by this card together with D3 — Q7 names each.
Cost, *Assumed* (added at review, S14): the worked thin plan on a three-cycle cosmetic run spends
one grade, one cold smoke and one solo design review where the full run spends three grades, a
pair, a separate cold run, a full regression sweep and a gap-finding pass — roughly half the
seat-rounds; the pair-to-solo thinning carries the overlap risk `author-grader-value-tiering`'s
F4 measured (two of six Important findings caught by both reviewers), accepted in the band only.

**How it was decided.** The lead leaned A; the user asked "A vs B?"; the lead compared them and
revised its lean to B with two edits (the never-move rule, the worked example); the user ruled
"as recommded" (2026-10-27). Provenance: `lead's guess, ratified` for the shape; `blind list` for
the never-move principle, the cold-run clause and four of the example's rows
(`research/second-list-departable.md`); `user's words` for the latitude itself.

**Changed at review.** S2 (2026-10-27): the accepted-risk line no longer leans on the declined
Q9 watch; the user re-confirmed the acceptance knowing nothing in this repo fires on its own
("confirmed"). S3: the skip set narrowed to the four non-leg steps; review legs thin only. S4:
the two objections to B from Q3 recorded and answered in the rejected roads. S7 (2026-10-27,
user: "as recommended"): the CI-gated cold-run skip dropped; the cold run always runs on the
accepted head and repeats after post-smoke code changes. Rejected: keep the skip fenced to the
band with a CI result present at acceptance. S12: re-slugged from `D5-band-latitude-record-never`
(four words, one a strength word). S13: the mark changed from Confident to Contested — the lean
reversed on the user's "A vs B?", the same pattern D4 and D8 carry. S14: cost line added.
Verify pass (2026-10-27, user: "as recommended, yes to all five"): S4 second round — the
rejected-roads answer names all three skippable advisory steps, not the hunt alone; N2 — the
heading now says "thin any, four may be skipped"; residual — the one-seat design review names
`mochiko:review-plan-artifacts`. Round 3 (N4, user: "confirmed"): the rejected-roads answer no
longer calls the gap-finding hunt advisory — it blocks when it runs, and its stated skip is the
one blocking check the band gives up.

Confidence: Contested *(was Confident; S13)*.

### D6-prior-rulings-fate — Seven prior rulings: two superseded in part, four amended in part, one kept with a clause

**Statement.** The rulings D1–D5 bend, each with its fate, to be annotated on its `DECISIONS.md`
row at landing: (1) `production-floor-adaptive-depth` D8-dial-standards-scope (2026-08-11,
`DECISIONS.md:91`) — superseded in part by D3 and D5: process depth keys on stakes; the `depth`
dial keeps code standards only (DF4). (2) `charter-ritual-balance` D1–D2 (2026-08-13,
`DECISIONS.md:84`) and the sound-loop trigger floor's "magnitude never gates it" — kept, one
clause added: stakes is a harm kind, not a size; it keys the depth of the legs after the trigger
fires, never whether it fires. (3) `author-grader-consolidation` D1 (2026-09-19,
`DECISIONS.md:32`) — amended in part: independence stays universal at run grain (one non-author
read before acceptance, floor); the grade's grain and form are the lead's (D4, D5); the
recorded rejection of surface-value tiering stands for grades — none is skipped by stakes, they
thin (D5 as amended at review) — and is superseded in part for the three steps D5 lets the band
skip stated: the code-minimalism lens and the design audit, advisory by floor and never
blocking, and the gap-finding hunt, whose spec-required findings do block when it runs
(`impl.fail.spec-gap-unresolved` is floor) — the one blocking check the band gives up, its
stated skip the trail's first line *(restated at review, S4; corrected at the verify pass to
name all three; the blocking claim corrected at round 3, N4; user: "confirmed")*.
(4) `producer-plan-enforcement` D6 (2026-09-03, `DECISIONS.md:53`) — amended in part: plan-QA
once per seat per run in the band; no carve by size, the carve is by stakes band. (5)
`validator-worktree-isolation` D0 (2026-08-01, `DECISIONS.md:122`) — **kept**, one clause
added *(restated at review, S7; was "amended in part" with a CI-gated skip, dropped)*: the cold
run doubles as the smoke's run, is never skipped, and repeats when rework changes code after
it. (6) `qa-gap-finding-verification` D2 (2026-08-19,
`DECISIONS.md:75`) — amended in part: gap-finding keyed on scope and on the stakes band, the skip
stated either way. (7) `command-architecture-realignment` D1–D2 (2026-08-02, `DECISIONS.md:115`)
— superseded in part: the departure trail returns as one widened fail rule and one line per
lowering (DF5), never as a weight card; counted bounds stay dropped.
`lead-owned-process-flexibility` D2 and D5 (2026-08-01, `DECISIONS.md:126`) revive in part; Q10's
card says what differs this time.

**Why.** Each row was named at the frame (R-c) or by the blind lists, not found at review; a
ruling bent silently is the record-layer corruption the KM invariants forbid. The fates are the
narrowest that make D1–D5 consistent with the index: nothing is struck whole, and the two
superseded rows lose only the clause the new cards contradict.

**Rejected roads.** Strike the seven whole — more than D1–D5 need, and each row's other
decisions stand. Leave them unannotated and let the new cards speak — a reader of the index
would find two rulings in force that contradict each other. Fold the annotations into one
umbrella row — the KM invariants want status agreement per row.

**Accepted risk.** The landing touches seven rows plus this session's own; a missed row is a
dead pointer, caught by the KM dead-pointer scan at landing. The clause added to the sound-loop
trigger floor (row 2) is a shipped-primitive edit and takes the audit and strip ritual at build,
not here.

**How it was decided.** One screen of seven rows with proposed fates; rows 1–3 from the frame's
R-c and the blind map (B1–B3), rows 4–7 from the departable-set blind list; the user ruled
"confirm all seven" (2026-10-27). Provenance: `lead's guess, ratified` for the fates; `blind
list` for rows 4–7.

**Changed at review.** S1 (Critical, 2026-10-27): `author-grader-consolidation` D2, D3, D7
(the gate jobs' fixed forms) and D8 (the input-job conditions), and GI-004, are **not bent** —
named as kept by D4 and D8's scoping, so a reader of the index finds no silent contradiction.
S4 (2026-10-27): row 3 restated — the rejection stands for grades, bent only for the
gap-finding hunt in the band. S7 (2026-10-27): row 5 restated — `validator-worktree-isolation`
D0 is kept, not amended; the cold run is never skipped. Verify pass (2026-10-27): row 3 names
all three steps the band may skip (S4 second round, user: "as recommended"); round 3 (N4,
user: "confirmed"): the row no longer calls all three advisory — the gap-finding hunt blocks
when it runs.

Confidence: Confident.

### D7-third-swing-card — What differs from 2026-08-01 and 2026-08-02, and how this swing could fail the same way

**Statement.** This is the third ruling on lead judgment versus encoded rigor. What differs: (1)
one question plus evidence in place of a paper key — 2026-08-01 gave the lead four rigor factors
and a weight card to fill per run, and 2026-08-02 found the encoded detail was itself the defect;
this time the read is one harm question on one line (D3), and a smoke probe does the work the
card tried to do. (2) The user rules each plan — 2026-08-01's departures were the lead's alone,
recorded after; now the plan is ruled at run-open (D2), so the trail is not the only control. (3)
The trail returns as a job, not an instrument — one widened fail rule (a skip not stated fails
the run) and one line per lowering (DF5); no weight card. (4) The lever lands in skills — the
131 rules grew by adding floors and fails to the command; the floor inventory found 39 of 40
floors right (`research/floor-inventory.md`), and what changes is the must layer gaining
conditions, not new rules (Q5, Q8). (5) Counted bounds stay dropped — 2026-08-02 stands there.
(6) How this swing could fail the same way, the things to look for on the user's own trials
(D9 — no watch in this repo): the run-open line grows into
a card, or D4's review-form examples harden into a menu.

**Why.** The cold review's first question of a third swing is "what is different"; a card that
answers it in the record beats an answer reconstructed at review. Each line names the instrument
the earlier swing used, what replaced it, and why the earlier reason for dropping it does not
apply to the replacement.

**Rejected roads.** Revive 2026-08-01's four factors — a paper read, against the user's call for
pragmatism. No card — the question gets asked anyway, and silence reads as a swing that did not
know its history.

**Accepted risk.** Line 6 is a prediction; if those signs never show the card stays unproven,
and if they show the session's instrument is the problem again. D9 leaves the revert to the
user's word; D5 names its shape — one rule edit returns a step to unconditional `must`.

**How it was decided.** Drafted by the lead from F1–F3, R-a, and D1–D6; put as a card for the
user's word with DF5 pulled forward in the same turn; the user ruled "as recommded"
(2026-10-27), confirming both. Provenance: `lead's guess, ratified`.

**Changed at review.** S2 (2026-10-27): line 6 and the accepted risk no longer lean on the
declined Q9 watch; the things to look for are read on the user's own trials (D9). Verify pass
(2026-10-27, residual note): the stale word "watches" in the accepted risk replaced.

Confidence: Confident.

### D8-home-sound-loop — The lever lands in `patterns-sound-loop`; no new skill; each step's lighter form in the skill that owns the step

**Statement.** `patterns-sound-loop` is the lever's home. It gains: the stakes key — the harm
classes of D3, the run-open read (proposed by the entry grading seat where the command has one,
else the lead; the lead may raise, never lower; the user rules — D2 as amended), declared as a
condition a caller feeds (`entry-derived`, as `testing-gap-finding` receives `depth`); the band
rule — which classes may thin and what never thins (D5: the floors, every step that guards a
shared record, `**TEST:**` execution, probe breadth); the trail as one more segment of its
pinned disclosure line (DF5's grammar); and D4's five review invariants in its leg 2, where
non-author review already lives. Its headline, "The entry door never lowers the review", and
the trigger floor's "magnitude never gates it" take one clause each: the trigger fires by kind;
stakes, also a kind, keys the depth of the legs after it fires, never whether they fire (D6 row
2). Each step's lighter form stays in the skill that owns the step, keyed on `stakes` — gap
finding's stated skip in `testing-gap-finding`, the plan leg's once-per-seat form in
`review-seat-plan`, and in `testing-end-user` the smoke procedure itself — build · suite · start
on real infrastructure · hand probes of the changed behaviour · the diff held against the ruled
harm class — with the cold-run clause *(widened at the verify pass, residual note)* — none
restated in sound-loop. The
worked thin plan (D5) is implement-specific prose, not a rule: it lands in implement's own text
or a sound-loop reference file. Implement declares `stakes` as a `moment-resolved(run-open)`
condition (the grammar already carries that resolution, `crates/mochiko-cli/src/model.rs:283-292`),
keeps its one floor `pointer:` to sound-loop, keys its thinnable must-rules `when: stakes=…`,
widens `impl.staffing-latitude` to which steps and how deep inside the band, and widens
`impl.fail.skip-unstated` to every thinned or skipped step. *(Added at review, S8.)* The touch
set also carries `review-sufficiency` — the harm-class field and duty of D2 as amended — and
the contract suite's frozen floor set (`evals/contract/expected-skills.json`, gate 6), re-frozen
for sound-loop's new floors. Every other command inherits on its next touch by the same pointer
(frame change 2). *(Scope, added at review — S1.)* The key and
the band bind product-run reviews only: where the reviewed object is not product work — a
plugin primitive, a governance surface, a brainstorm record — no harm class applies, there is no
band, and the review runs in its fixed form; sound-loop's new text says so beside the key, and
the gate jobs named in D4 keep their forms.

**Why.** The user's question — "why cant we fit in already existing skills" — and the frame's
own bet B: a new primitive in a library this session says regrew is the cost the session exists
to avoid. Every part the hub would have held has a fitting home in the ritual floor: the key
beside the kind trigger, the band as the floor's own content, the trail on the existing
disclosure line, the invariants in the leg they bind. Sound-loop carries 14 rules (the section
totals its render prints) and a 509-character description against the 1,536 cap — about 1,000
of headroom — so it absorbs six to eight rules; model-tiering carries 22 rules and a
1,215-character description, about 320 of headroom *(corrected at review, S11: the lead's
first count had added each skill's floor-pin line to its section total; measured by
`mochiko-cli rules <skill> --section preamble --plugin-root plugins/mochiko` and an awk over
the parsed `description:` value)*, and would mix "which model" with "whether the step runs".

**Rejected roads.** A new sibling skill, hub and spokes (blind road R6; the lead's first lean) —
buys a name and nothing else at the price of a new primitive. Fold into `patterns-model-tiering`
(R7) — description nearly full; conflates tier with presence. Reuse the `depth` and `scope`
conditions (R5) — project-wide and shape-keyed, not a per-run read. Command rules only (R4) —
against the destination and bet C. Split to nearest owners with no single band definition (R3)
— five skills drift apart; D8 keeps one definition in sound-loop and only the lighter forms in
the owners.

**Accepted risk.** Sound-loop's remit widens from "who produces, who reviews" to "who, and how
deep"; its headline clause is the place a cold reader will probe for a softener arguing with
its floor — the clause says depth, never presence, and D6 row 2 records it. The edit is a
shipped-primitive edit under the audit and strip ritual, plus the owning skills' edits and the
implement re-key — one audit per edited unit at build.

**How it was decided.** The lead leaned A (new sibling); the user asked why the parts could not
fit existing skills; the lead measured the hosts and revised its lean to sound-loop; the user
ruled "as recommded" (2026-10-27). Provenance: `lead's guess` for the placement, shaped by the
user's push; `blind list` for the grammar fact and the hosts' floors to carve
(`research/second-list-home.md`).

**Changed at review.** S1 (Critical, 2026-10-27): the key and band scoped to product-run
reviews; non-product surfaces have no band; the gate jobs keep their forms (see D4). User's
word: "i will go with recommdation". S8 (2026-10-27): the touch set gains `review-sufficiency`
and the contract suite's frozen floor set. Verify pass (2026-10-27, residual note): the
`testing-end-user` touch names the smoke procedure's hand-probe and diff-hold duties.

Confidence: Contested.

### D9-evidence-users-trials — No watch item; the user's own trials on other projects are the evidence

**Statement.** The session lands no `BACKLOG.md` watch item and no counter for this change. The
evidence that the lever works, and the trigger to undo any part of it, is the user's own
trials on other projects and the user's word afterwards. D7's two named failure modes (the
run-open line growing into a card; the review-form examples hardening into a menu) and D5's
revert shape (a thinned step returns to unconditional `must` by one rule edit) stay in this
record as the things to look for, tracked nowhere else. The trail (DF5) is what the user reads
on each trial run.

**Why.** The user's words: "i think we can ignore this, i will try on other projects." The
driver was a run in another repo (F20), and the user is the one who will run the next ones; a
watch item in this repo would count runs that happen elsewhere.

**Rejected roads.** A watch item on the n=0 first-live-run pattern (the lead's pick) — would
have counted seats, rounds and escapes on the first five band runs with three one-rule reverts;
declined. A per-run counter field — the roster line already carries the count.

**Accepted risk.** Nothing in this repo fires if the lever misbehaves; the trigger is the user
noticing on another project and saying so. The two D7 watches go uncounted. Bet A (a stated
stakes key beats uniform rigor and free judgment) stays untested in this repo until the user
reports back; bet B (regrowth) is testable at the build without runs — implement's rule count
before and after; bet C (inheritance) is tested on the next touch of another command.

**How it was decided.** The lead put A, B, C with A as its pick; the user declined all three
in favour of their own trials (2026-10-27). Provenance: `user's words`. OQ1 (which repo, what
felt heavy) closes with this ruling — the replay is the user's own.

**Changed at review.** —

Confidence: Confident.

## Defaults batch

**Confirmed as a batch 2026-10-27** — the accept-screen listed DF1–DF7 by name; the user's word
"confirm" covered all seven (DF5 had been confirmed earlier on its own).

- DF1 — **Upward departure is free** *(blind list, road 15; arose at Q1, carried in D1)* —
  raising rigor above what the stakes read sizes needs no trail line; only lowering is trailed.
  Confirm in the batch.
- DF2 — **The read ratchets up only, mid-run** *(blind map C5; arose at Q2b, carried in D2)* —
  after run-open the read moves up on in-run evidence (a consumed attempt, a failed gate, a
  deviation, or — *added at review, S5* — the smoke seat finding the diff touches a money, data
  or security surface, which voids the thinning outright) by the lead's own hand; it never moves
  down except by the user's checkpoint ruling. Confirm in the batch.
- DF3 — **An unclear read reads high** *(blind map C6; arose at Q2; reworded at review, S10,
  after D3 dropped the undo axis)* — where the proposer cannot cite a line for the harm class,
  the class reads as the never-thins band (money, data or security). Confirm in the batch.
- DF4 — **"Stakes", never "depth"** *(blind map C7; arose at Q2)* — the key's vocabulary stays
  apart from the governance `depth` dial, which keys code standards; the run-open line says
  "stakes", and no rule reads the dial to size process. Confirm in the batch.
- DF5 — **The run-open line and the trail, their grammar** *(Q6 folded to a default 2026-10-27;
  lead's guess; blind map C8, J3, J4)* — the run-open line, recorded with the run-open rulings in
  the sufficiency report: `stakes: <class> — <cited line> · smoke: <scope> at <point> · thins on
  clean: <steps> · never: <steps>`; the trail: one line per lowering written in the
  final-validation report file itself — a durable home `impl.fail.skip-unstated` can read — and
  shown in the promises-kept table *(amended at review, S6: the table is composed in
  conversation, no new file, per `impl.promises-kept-table`, so it cannot be the home)*, naming
  the step, the smoke result it rode on, and what still covered the failure kind; a raise needs
  no line (DF1). **Confirmed 2026-10-27** — pulled
  forward because Q9 hung on it; the user's word "as recommded" covered it with D7.
- DF6 — **Eight floors the blind list would re-class stay floor** *(lead's guess against blind
  road; arose at Q3)* — `baseline-diff-review` · `fail.unreviewed-baseline-diff` ·
  `artifact-home` · `no-progress-stop` · `minimalism-advisory` · `design-audit-advisory` ·
  `fail.sufficiency-unrecorded` · `fail.card-unchecked` keep `class: floor`: each passes D1's
  question 3 or 4 or is an invariant with no depth dimension, six change nothing in practice
  either way, and the sufficiency report is where D2's run-open rulings land. Confirm in the
  batch, or pull out to its own turn.
- DF7 — **Rule class for the departable steps** *(Q8 folded to a default 2026-10-27; lead's
  guess; F13 — no fourth class)* — implement's thinnable steps stay `class: must`, each gated
  `when: stakes=<thinnable class>` and worded "per the ruled plan, on a clean cold smoke";
  `impl.staffing-latitude` stays the one `kind: latitude` rule, widened to which steps and how
  deep inside the band; in sound-loop the never-thins rule and the five invariants are
  `class: floor` (they pass D1's test), the key and the trail segment `class: must`. No new
  class, no CLI change (out-of-scope fence 2). Confirm in the batch.

**Added at review (S9, four candidates the batch had absorbed silently); confirmed by the
user 2026-10-27 ("confirmed"):**

- DF8 — **Stakes on an epic** *(blind map G5)* — the harm class is read per member from each
  member's own spec lines; the epic's band is the highest class among its members; one plan
  line per member, one cold smoke per epic (one snapshot already covers a whole epic,
  `impl.cold-verification`).
- DF9 — **Rework after a regression in a band run** *(blind map G9)* — any red after a clean
  smoke raises the read (DF2); the rework is graded at full depth for the touched cycle and the
  band does not return for that run; the cold run repeats (D5 as amended, S7).
- DF10 — **Attempts under one grade over several cycles** *(blind map D4, D6)* — a shared grade
  charges one attempt to each cycle it covers (`impl.attempt-per-grade`: a cycle consumes an
  attempt every time a verification seat grades it); a FAIL on one cycle charges only that
  cycle's rework.
- DF11 — **How this change itself is built** *(blind map J5)* — the plugin edits that build it
  ship under the existing primitive-edit audit (GI-004, the locked gate form), never under their
  own lever; consistent with S1's scoping — plugin edits are not product work.

## Open questions

- OQ1 — *(fact only the user holds, asked 2026-10-27 beside Q1, never answered; **closed
  2026-10-27 by D9**)* — which repo the driver run (F20) was in, and what felt heavy in it. No
  longer needed: the user runs the trials themselves (D9); nothing ruled hangs on it.
- OQ2 — *(opened at review 2026-10-27, S1; handed to a later session by the user's word)* — a
  stakes key for non-product surfaces: what "stakes" means for a plugin-primitive edit, a
  governance surface or a brainstorm record, and whether the gate jobs
  (`author-grader-consolidation` D2, D3, D7; GI-004) may ever thin under it. Lands in
  `BACKLOG.md` at the close ritual as an open thread in this wording.

## Review

**Accept-screen confirmed 2026-10-27** ("confirm"): the nine decisions by name with gists, the
defaults batch DF1–DF7, the out-of-scope line as amended, no open questions, the three bets each
with its test. The record froze on that word.

**Cold read dispatched 2026-10-27** to `review-seat` (`mochiko:devils-advocate`, persona-pinned
opus, spawned blind at frame hardening 2026-10-25; its blind angle map at
`reports/angle-map.md`, 66 angles / 18 load-bearing). The dispatch names the frozen record and
the report home `reports/cold-review.md`; the lead holds the pen and writes nothing in the
record until the verdict lands (quiesce before cold grade). Survivors are dispositioned by the
user's word; a survivor challenging a user ruling is the user's alone; fixes edit one card each
with a line under its changed-at-review part; one verify round.

**Cold review landed 2026-10-27** — `reports/cold-review.md` (report: review, round 1,
record_contact: read; confirmed on disk, 222 lines). Status recommended: critical-gaps (input,
not a clearing). 17 raised, 14 survived: Critical 1 (S1) · Important 8 (S2–S9) · Minor 5
(S10–S14); three withdrawn on the seat's own checks. All nine cards carry all six parts;
fitness 3 pass, 4 partial. Coverage: every one of the 66 map angles placed. No cross-examination
run before the disposition rounds: the lead checked each survivor's cited facts against the
sources instead (sound-loop's trigger section, `impl.promises-kept-table`,
`impl.cold-verification`, the report-envelope template, `evals/contract/expected-skills.json`,
`templates/artifact-format.md` IDs section, the host skills' renders) and found them as the
seat stated, except two counts in S11 and S12 (the host rule counts are 14 and 22 as the seat
says; the description values measure 509 and 1,215 by the lead's command against the seat's
505 and 1,209 — the headroom argument holds either way; five slugs, not six, break the
three-word rule). Disposition rounds: S1 on its own turn (a real choice that challenges D4 and
D8) · S2–S6 and S8 as one batch of one-fix survivors touching ruled cards · S7 and S9 on their
own turns (real choices) · S10–S14 as the repairs batch.

**Dispositions**
- S1 (Critical) — **accepted, scoped** (user: "i will go with recommdation", 2026-10-27, after
  asking for A against B twice): D4 and D8 scoped to product-run input-job reviews; the gate jobs
  and consolidation D8 named as kept (D4, D6); non-product surfaces have no band (D8); frame
  change 3 stamped; OQ2 opened for the later session. Rejected: widen now (B).
- S2–S6, S8 (Important) — **accepted as one batch** (user: "confirmed", 2026-10-27): S2 — the
  five accepted-risk lines reworded off the declined watch, D3 and D5 re-confirmed; S3 — D5's
  skip set narrowed to the four non-leg steps; S4 — the two Q3 objections to B recorded and
  answered on D5, D6 row 3 restated; S5 — the smoke seat holds the diff against the ruled
  class, DF2 amended; S6 — the trail's home is the final-validation report file, DF5 amended;
  S8 — the smoke's seat, skill and report type named on D3, D8's touch set widened.
- S7 (Important) — **accepted, option (a)** (user: "as recommended", 2026-10-27): the CI-gated
  cold-run skip dropped from D5; the cold run always runs on the accepted head and repeats after
  post-smoke code changes; D6 row 5 restated to "kept". Rejected: the skip fenced to the band
  with a CI result present at acceptance.
- S9 (Important) — **accepted** (user: "confirmed", 2026-10-27): the four absorbed candidates
  ruled as DF8–DF11 (epic stakes · rework after a regression · attempts under a shared grade ·
  this change built under the locked gate form).
- S10–S14 (Minor) — **accepted as the repairs batch** (user: "confirmed", 2026-10-27): S10 —
  frame heading and reply paragraph groomed, the map collapsed to one ruled-or-folded list with
  "In fog — none", Q6's spliced text removed, DF3 reworded; S11 — D8's host counts corrected to
  14 and 22 rules, 509 and 1,215 description characters, the command cited; S12 — five IDs
  re-slugged to three plain words (`D1-floor-four-questions` · `D2-stakes-read-ruling` ·
  `D3-harm-band-smoke` · `D4-review-form-invariants` · `D5-band-step-latitude`; the cold-review
  report keeps the old slugs it graded); S13 — D5 marked Contested; S14 — cost lines marked
  Assumed added to D3 and D5.

**Fixes complete 2026-10-27** — every survivor dispositioned by the user's word (S1 scoped,
S2–S9 accepted, S10–S14 repaired; none declined). Verify round dispatched to `review-seat`:
the changed cards and their gists only, once.

**Verify landed 2026-10-27** — `reports/verify.md` (round 2, record_contact: read; confirmed on
disk, 139 lines). Status recommended: needs-revision. 14 checked · 13 fixed · 1 not fixed ·
3 new Minor inconsistencies. Not fixed: S4 — D5's rejected roads and D6 row 3 now claim the old
rejection is bent "only for the gap-finding hunt", but D5's skip set also carries the
code-minimalism lens and the design audit, which are grades under `author-grader-consolidation`
D1's site 5; the claim is narrower than the card. New: N1 — D4 and frame change 3 call
brainstorm's cold review a "gate job" (it is an input job, kept via consolidation D8), and
"five invariants and nothing else" sits beside "D8's conditions stay"; N2 — D5's heading still
says "thin or skip any must-class check"; N3 — D3 says the QA seat "authored nothing in the
run", but the QA seat authors `**TEST:**` cases. Residual notes, not failing: the one-seat
design review should name `review-plan-artifacts`; D8's `testing-end-user` touch omits the
smoke's hand-probe and diff-hold duties; D7 line "if the watches never fire". The seat
corrected its own S12 count to five. Per the bound, the failed verify goes to the user: S4's
corrected claim is theirs to rule; N1–N3 and the residual notes are wording repairs put in the
same screen.

**Second-round dispositions 2026-10-27** (user: "as recommended, yes to all five"): S4 — option
(a), the bend of `author-grader-consolidation` D1 names all three advisory steps the band may
skip (D5 rejected roads, D6 row 3); rejected: (b) shrink the skip list so the two lenses thin.
N1 — D4 and frame change 3 re-describe brainstorm's cold review as an input job with kept
conditions; "nothing else" scoped. N2 — D5's heading. N3 — D3's QA-seat wording. Residual
notes — the one-seat design review names `mochiko:review-plan-artifacts` (D5); D8's
`testing-end-user` touch names the smoke's hand-probe and diff-hold duties; D7's "watches"
replaced. A second, bounded verify pass goes to `review-seat` over these seven places only; if
it fails, the user rules with both fix lists.

**Round 3 re-verify landed 2026-10-27** — appended to `reports/verify.md` (round_3 fields and
a "Round 3" section; the seat disclosed it appended by shell heredoc and ran `mochiko-cli check`
on the final file: allow). Status recommended: needs-revision. Fixed: places 3, 5, 6, 7. Still
failing: N4 (Important, introduced by the lead's S4 fix) — D5's rejected roads and D6 row 3 call
all three skippable steps advisory and say none can block, citing two floors for three steps;
the gap-finding hunt is not advisory — `impl.finding-kinds` fails final validation on a
spec-required finding and `impl.fail.spec-gap-unresolved` is floor; the bend's scope (three
steps) is right, the claim about blocking is wrong, and D6 row 3 is the text that annotates
`DECISIONS.md` row 32 at landing. N1 (Minor, partly fixed) — "five invariants and nothing else
within this scope" still sits against "D8's conditions stay". N3 (Minor, not fixed) — D3's
sentence still opens "an independent seat that authored nothing in the run" before the
corrected parenthetical. The verify bound is exhausted: both fix lists go to the user — the
seat's and the lead's agree on all three.

**Round-3 dispositions 2026-10-27** (user: "confirmed"): N4 — D5's rejected roads and D6 row 3
now say the two lenses are advisory by floor and the gap-finding hunt blocks when it runs, its
stated skip the one blocking check the band gives up; N1 — D4's invariants line reads "five
invariants — plus, for the input jobs consolidation D8 names, its kept conditions — and nothing
else"; N3 — D3's sentence opens "a seat that wrote no product code in the run". The verify bound
is exhausted (one pass plus one bounded re-verify); no further pass runs. The seat's and the
lead's fix lists agreed, so the user's word on the three fixes closes the review.
`brainstorm.fail.survivor-undispositioned`: none left — 14 survivors and 4 verify-born
inconsistencies all dispositioned by the user's word.
