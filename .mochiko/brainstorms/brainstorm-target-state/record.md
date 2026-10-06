# Brainstorm target state — decision record

**Status:** **accepted 2026-10-06** (user: "happy for you to accept and start building", said
while the verify ran and taken as the acceptance once the verify came back with nothing blocking
and the user had ruled its one open point) — review round 1 folded (16/16 survivors
dispositioned: S1–S7 user-ruled one by one, S8–S16 in one user-confirmed batch); verify round 1
NOT CLEAN, nothing blocking (V1, V2, V4, V5, V6 lead-repaired, V3 user-ruled; closed
seat-unverified, disclosed) · **Landed:** `DECISIONS.md` row 2026-10-06, five prior rows
annotated · `BACKLOG.md` *Brainstorm target-state build* section (the build item and the dogfood
watch); the cold-review first-live-run watch annotated · `ROADMAP.md` folded into the
Cold-review gap-challenge Now row (cap held at 5) · both indexes updated (this entry; five prior
entries and their records annotated) · **built 2026-10-06 at v0.117.0** — wave 1
(`wave1-target-state.md`, `build-log.md`, `reports/gate-audit.md`) on branch
`brainstorm-target-state`: migrations 0044–0045 (`brainstorm` 30 → 55 rules, `review-brainstorm`
30 → 34, no floor or fail moved; the lead dropped one approved re-anchor when the crate's
provenance test forbade it), prose, strips, budgets (the `review-brainstorm` +1,398 overage
ruled HOLDS), crate pins and both eval kits; three producer seats under the sound loop (P1 and
P2 each one failed plan round, P2's second round granted by the user with the Q23 ruling folded
into D16; P3 clean); 8/8 gate audits PASS round 1; crate gates, views ≡ replay, audit and secret
scan green; the contract suite 97/97, none skipped; the build item → trail, the dogfood watch
open ·
**Opened:** 2026-10-04 · **Lead:** session lead (inline questioning via
`mochiko:analysis-iterative`) · **Review:** solo cold review, complete — one
`mochiko:devils-advocate` seat on `mochiko:review-brainstorm` at its persona default tier (`opus`,
no deviation), both lenses, blind two-message dispatch (message 1: topic and goal line only;
this session's directory and the index fenced; `record_contact: none` attested; one fence
disclosure, weighed in the Review section) ·
**Transport (`mochiko:patterns-transport-floor`):** one reviewer seat, lead-relayed messaging, no
cross-seat mesh, so the message lane binds; one writer per file — the record is the lead's,
`reports/angle-map.md` and `reports/review.md` are the reviewer's — so no shared write surface;
the record holds frozen from message 2 until the verdict

## Topic

Driver ask (user, 2026-10-04, across the conversation that opened the session): "i want you to
critically analyse https://github.com/addyosmani/agent-skills and mochiko brainstorm to help me
rebuild mochiko brainstorm" · "help me imagine what would be a flow that will look like if we had
full freedom to rewrite brainstorming" · "https://github.com/mattpocock/skills/…/wayfinder/SKILL.md
get context from it too and see what is worth contemplating in the flow" · "lets design a target
state, basically adding a column and go one by one" · at command entry: "My plan is to create a
target state. I like the format in which the information is being presented."

**Goal line:** decide, stage by stage, the target-state flow of `/mochiko:brainstorm` — what the
session does at each stage, set beside the current flow and the two outside flows — leaving one
hardened decision record.

**Destination (the lead's reading, confirmed by the user in the wrap-up batch, 2026-10-04):** a target
column ruled for every row of the stage table below, complete enough that the build — which the
user ruled happens in this same session (Q12; wording corrected at review, S13) — can be planned
without asking the user a design question. The build surface itself (which
primitives change, migrations, strips, audits) is not designed here — see OQ1.

**Session format (user, 2026-10-04: "I like the format in which the information is being
presented"):** one stage per turn; the stage's row shown across the columns with the proposed
target; one question in plain words; each option with one concrete example; the case against the
lead's pick; a one-line recommendation.

**Prior-session relations.** `brainstorm-command` (v1, superseded) · `brainstorm-command-rewrite`
(v2, v0.4.0) · `brainstorm-v2-revision` (v2.1, v0.5.0) · `brainstorm-v2-2-revision` (v2.2, v0.6.0)
· `cold-review-gap-challenge` (v0.60.0) · `validator-scope-and-verbosity` (the guardrails-vs-detail
benchmark, v0.63.0) · `skill-succinctness-strip` (v0.25.0). What each ruled is in F2–F4.

## Ground facts (F)

Every fact was read first-hand by the lead on 2026-10-04. Counts are greps over
`.mochiko/brainstorms/`; where a figure is an estimate or a partial read, the fact says so.

- **F1 — The command today.** `plugins/mochiko/commands/brainstorm.md` carries the identity, the
  goal protocol and seven `mochiko-cli rules` slots; the render (binary 0.3.0, grammar 2, plugin
  0.116.0) delivers 30 rules in six sections (roles 5 · reserved 5 · tools 7 · ways of working 8 ·
  boundaries 1 · fail conditions 4), eight of them floors, and four moments: session-open,
  cold-review, acceptance, close. The only rule on how questions are asked is
  `brainstorm.lead-inline-questioning`: "one question per turn, format adapted to the user's
  state". No rule states when questioning is finished.
- **F2 — The questioning skill today.** `skills/analysis-iterative/SKILL.md` is 66 lines. The
  v0.63.0 guardrails cut removed its Adaptive Flow, Discovery, the four question formats, the
  confidence-signals table and the Smart Wrap-up section (`.mochiko/strips/analysis-iterative.md`).
  The body still says "The adaptive flow above is the single questioning engine", and no flow
  stands above that line. The skill's Output section says to generate the synthesis document,
  while the command makes `record.md` canonical and a synthesis on-request only. The strip file's
  v0.25.0 `KEPT` entry records dogfood evidence for the removed ratification-streak doctrine
  ("streak flagged after 4× 'yes a'; user re-engaged with genuine reads").
- **F3 — What the benchmark behind that cut measured.**
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`: twelve runs over `setup` and
  `specify` only (caveat C-4: "plan/implement/brainstorm untested"), all "single-agent simulations
  with a model-played principal" (C-3). Cross-cutting finding 2: "Full-detail's failure mode is
  over-production … detail did not buy honesty; it bought weight."
- **F4 — Brainstorm's own history** (index entries). v1 was superseded after its first dogfood —
  "adversarial substance kept, phase/gate ceremony killed". v2 seated a standing advocate in the
  room; v2.1 retired it after transcript forensics "measured 3:1 machine-to-user traffic and
  consent-free folds into user-ruled decisions" and moved all adversarial pressure to the end.
  v2.2 made review sizing a human gate after a run measured ≈654k output tokens.
  `cold-review-gap-challenge` added the blind angle map and the two-message dispatch; a grep of
  its record finds no sign that an angle map at the front of the session was weighed.
- **F5 — Review outcomes across sessions** (index). Seven entries state a raised → survivors
  count: 21→16, 25→17, 24→16, 25→18, 22→15 plus 8 coverage, 30→25, 29→20. As a
  backticked status, `critical-gaps` appears seven times and `ready` three; the bare string
  occurs 22 times (corrected at review, S9). Where an entry states its coverage findings, the
  counts are 3, 6, 4, 3 and 8 (added at review, S12). At least twenty entries carry a
  distinct "verify round 1 NOT CLEAN" note, most of them naming fold-propagation defects, stale
  echoes or fold-introduced contradictions.
- **F6 — One session read closely** (`setup-product-agnostic`, question trail and review table
  read in full). Q1 was answered by the user narrowing the topic; Q2–Q6 were answered "out",
  "yes", "yes", "confiremd", "yes"; all six decisions are marked `Confident`. Of the 16 review
  survivors, three are roads never put to the user (S2, S7, S15), four are unlisted consumers,
  an incomplete supersession chain or wrong counts (S6, S8, S9, S16), and five touch the build
  surface (S1, S6, S8, S9, S11). The ruling that reversed an earlier one — forward-only, no legacy
  accommodation — came from the user unprompted during the delta-check (Q8).
- **F7 — Three more sessions skimmed** (question trails and survivor lists only). In
  `hook-enforcement-field-review`, at Q3 and Q7 the user asked why the thing in question existed
  at all; the answer was already in that record's facts (its F11, F12, F16) and each question was
  re-put in a different shape — the user challenging a question's premise, not a missing fact
  (corrected at review, S11; the lead had written that the user "asked a fact back"). One
  Critical finding there was put three times and reversed by the user. In
  `orchestrator-model-selection`, several survivors are unchecked platform facts (G1, G3, G4, G5).
  Ten other records match `never (put|dealt|weighed|considered)`; the reviewer read them and
  found seven are review findings about a road never put to the user and three are not
  (`cold-review-gap-challenge`, `model-tiered-seats`, `security-depth-scoping`) (corrected at
  review, S8; the lead had written "eleven").
- **F8 — Size.** 77 index entries; records total 23,283 lines; the two largest are 1,536 and
  1,375 lines; the index is 789 lines. `setup-product-agnostic` alone is a 558-line record, a
  545-line review report and a 244-line angle map.
- **F9 — addyosmani/agent-skills at `1401c8b`.** `interview-me`: a one-sentence hypothesis with a
  confidence figure on the first turn, one question per turn with the agent's guess attached, a
  "what would you want if you didn't have to justify it" probe, a restate (outcome · user · why
  now · success · constraint · out of scope), an explicit-yes gate that names what is not a yes,
  the stop test "can I predict the user's reaction to the next three questions", a stuck floor.
  `idea-refine`: variation lenses, two or three distinct directions, assumptions each with a way
  to test it, a Not Doing list — and three to five sharpening questions asked as a batch.
  `doubt-driven-development`: a fresh reviewer given the artifact and its contract but never the
  claim, four reconcile classes, a three-cycle bound. Its evals carry one dialogue case per
  skill, and the rejected-changes ledger is empty.
- **F10 — mattpocock/skills at `d81f3a1`.** `wayfinder`: the destination named first; a map that
  is "an index, not a store"; decision tickets typed research (agent alone), prototype, grilling
  and task; fog of war ("don't chart what you can't yet see") with a Not yet specified and an Out
  of scope section; "refer by name", never a bare id; one ticket per session; "plan, don't do".
  `grilling`: the whole frontier asked in one round with recommended answers; "finding facts is
  your job, never the user's". `domain-modeling`: an ADR only when hard to reverse, surprising
  without context, and the result of a real trade-off. `codebase-design/DESIGN-IT-TWICE.md`:
  three or more parallel designs under different constraints. `docs/engineering/wayfinder.md`
  reports its own field failures: 27 tickets charted and stale by the thirteenth, an agent
  licensing itself through notes it wrote, questions three paragraphs long, an agent choosing a
  prototype variant for the user (`docs/engineering/wayfinder.md:75`: "building three UI
  variations, choosing one itself, and closing the ticket"; source line added at review, S10). None of these skills carries an independent review.
- **F11 — The user's standing feedback on question form** (2026-09-19, during
  `author-grader-consolidation`; held in the lead's session memory, not re-read in that record):
  questions in plain words, short, one concrete example per option; rule ids stay in the record.

- **F12 — Reach beyond brainstorm** (rendered rules of `setup` and `specify`, 2026-10-04).
  `setup.interrogation-inline` points at `mochiko:analysis-iterative`, the same questioning skill
  brainstorm uses. `setup.blind-map-dispatch` and `setup.coverage-survivor-routing` carry the
  same two-message blind dispatch and coverage routing for setup's review seat. No rule in
  `specify`'s render names the skill or a blind map; `skills/review-specifications/SKILL.md` and
  the router skill mention the skill in prose.

- **F13 — Resuming a seat** (the harness's own tool contract, read 2026-10-04; not probed over a
  long idle). A seat spawned earlier in a session is continued "with its context intact" by
  sending it a message under its name or id. Today's two-message blind dispatch already relies on
  this over a short pause. Nothing was found that carries a seat across separate sessions, which
  is the case D10's fresh-seat fallback covers.

## Stage table — the working surface

The first three columns are the comparison the user asked for and are descriptive. The Target
column is ruled one row at a time; an unruled cell reads "open".

| Stage | Current | + Interview | + Interview + Wayfinder | Target |
|---|---|---|---|---|
| Open | Read index, enter session | Same | Same | Light orient: the index, the records the topic names, the user's earlier rulings on it. No code sweep, no user turn (D1) |
| Frame | None; the first question starts deciding | Lead states its read and what is missing; restate card with out of scope | Same, plus a named destination | One card in one turn — problem · destination · must not break · betting on · out of scope, plus a size line — each marked as the user's words or the lead's guess; the user strikes or corrects before deciding starts (D2). It hardens at the user's first reply; a line the user cannot answer yet is marked open and becomes an early decision; later changes go one line at a time, by the user's word (D3) |
| Decision map | None; the lead picks the next question as it goes | None | Askable-now decisions, fog, out of scope | A named map in three parts — askable now · in fog · out of scope — shown with the first question, never a gate, editable in any reply; one status line per turn after (D4) |
| Order | Lead judgment | Lead judgment | Dependency order; each question says what it unblocks | No question before what it hangs on is settled; which askable item goes first is the lead's judgment, and the user may pick; each question says what it unblocks (D12, as changed by D22) |
| Facts | Sometimes put to the user, then re-asked | Same | The lead finds them; never asked of the user | Fact first: a decision waits for the fact it hangs on; the lead finds repo, platform and doc facts and attaches the source; a fact only the user holds is asked as a labelled fact question (D5) |
| Asking | One per turn, options plus a recommendation | One per turn, guess attached; the "need not justify" probe | By type: fact (no turn), probe (react to an artifact), talk (one per turn) | A decision gets its own turn when there is a real choice or a wrong pick is costly to undo; everything else is a named one-line default confirmed in one batch (D6). The lead writes a fork's options under a rule — two real options, the cheaper shape among them, the rejected road named — and on a costly fork a fresh seat returns a blind second list (D7). The question form (D13); show before asking (D14) |
| Bare yes | Counts as `Confident` | "Sounds good" and "whatever you think" are not a yes; re-ask | Same | A yes that follows the lead's pick stands and is recorded as ratified; after three in a row the lead says so and changes the form of the next fork; "whatever you think" is re-asked as a two-option choice (D8) |
| Stop rule | None | The lead can predict the next three answers; stuck floor | The way is clear: nothing left to decide before the destination | Finished when the map is empty and any blind map that was started has landed; the user may stop at any time; a map that has not shrunk for three questions sends the lead back to the frame (D15) |
| Record | Long prose; one fact in several places | Same | One home per decision; the index holds a gist and a link; names, not ids | Inside the record each decision is one named card with fixed parts, and everything else points to it by name with at most a one-line gist; the cross-session index keeps its long entry (D9) |
| Cold review | Blind map, cold read, six hunt classes | Same | Same, plus "what would a builder still ask" | The blind angle map moves to the front and feeds the decision map without holding the first question; the same seat is resumed at the end for the cold read; no second map (D10). The end reviewer first checks that every card has its parts, reads ratified decisions first and applies the builder test (D16) |
| Fix and verify | Batch dispositions, a verify round | Same | Same; a fix edits one place | Findings sorted by the two tests; a fix edits one card; one verify pass over the changed cards, a second failure to the user (D17) |
| Accept | The user's word | An explicit yes on the restated record | Same | One screen before review, one after; the user's explicit word (D18) |
| Size | One depth, one session | Same | No fog: skip the map. Too big: chart now, resolve the rest later | Three sizes — small · standard · too big — proposed by the lead on the frame card and ruled by the user in the same reply; raised freely, never lowered without the user's word (D11) |

Two decisions sit outside the table because they are not stages of the flow: the target reaches
brainstorm only (D19), and the blind second list is built untested and tried in the dogfood repo
(D20).

## Lead's reading of the evidence (not ruled)

Three causes, offered as the lead's diagnosis and carried `Assumed`: (1) the session collects
ratification more than it collects what only the user knows (F6, F7); (2) the lead leaves roads
unput and blast radius unlisted, and those arrive as review findings (F6, F7); (3) the record
restates one fact in several places, so every fold leaves stale echoes (F5).

## On the table from the opening conversation (proposals, not rulings)

- A decision card written before the question is asked: options including the cheaper shape,
  what breaks under each with an example, consumers touched, the case against the lead's pick.
- Design-it-twice on shape-defining forks: two or three seats under different constraints.
- The blind angle map moved to the front of the session — **parked to the Cold review row**.
- The lead's earlier "doubt in flight" proposal is **withdrawn by the lead**: it repeats v2's
  standing advocate (F4).

## Constraints carried in (confirmed by the user in the wrap-up batch, 2026-10-04)

1. The cold review's core stays — blind dispatch, the hunt classes, the user's pen. Only its
   inputs and its load change.
2. The flow is designed freely here; mochiko form (rules in the migration log, strips, audits)
   is build work.
3. The evidence is the user's own sessions, so the target fits that pattern, not an unknown
   plugin user's.

## Decisions (D)

### D1 — Open: the lead orients lightly before its first message — `Confident`

**Statement.** Before its first message the lead reads the index, the records the topic names,
and the user's earlier rulings on the topic. It takes no user turn and makes no sweep of code or
consumers; fact work waits until the session knows which facts its decisions need.

**Rationale.** A deep sweep before the frame is paid for on ground the user may then cut: in
`setup-product-agnostic` the user narrowed the topic at the first answer (F6), so a sweep of the
rehoming half would have been wasted reading.

**Rejected road.** Deep ground first — the light orient plus a sweep of code and consumers
before the frame. It loses on cost spent before the user has confirmed what the session is
about.

**Accepted risk.** A frame built on thin reading may misstate the repo and cost the user one
correction turn. Put to the user with the question as the case against the lead's pick.

**How it was decided.** Q1; user: "yes A" — the lead's recommendation, ratified.

### D2 — Frame: one card in one turn, guesses marked — `Confident`

**Statement.** Before any deciding starts, the lead's first message is a frame card of five
lines: problem · destination · must not break · betting on · out of scope — and, since the
review (S2), a sixth line proposing the session's size (D11). The lead drafts it
from the user's opening words and the light orient (D1), and marks every line as the user's
words or the lead's guess. The user strikes or corrects. A line the lead cannot fill even as a
guess is asked on its own before the card is shown. The card carries no confidence percentage;
the guess marks do that job.

**Rationale.** Today nothing pins the session's subject before the first design question, so a
constraint the lead silently assumed can shape several decisions before it surfaces: in
`setup-product-agnostic` the forward-only ruling arrived unprompted during the delta-check and
reversed an earlier ruling (F6). A card with the guess shown would have put that line in front
of the user on the first turn.

**Rejected road.** Interview first — who, why now, success and limit asked one per turn, the
card shown afterwards. It loses on turns: the user's opening ask usually answers most of those
lines already, as it did on the setup topic.

**Accepted risk.** A card is easy to wave through with one "yes", so a wrong but plausible guess
can survive. The guard belongs to the Bare yes row and is not designed here.

**Detail, confirmed in the wrap-up batch.** The problem line names who has the problem. The lead added this
while walking the user's billing example, at first without a question.

**How it was decided.** Q2; user: "yes A", followed by the user's own question on when the card
is created and when it hardens (answered in the conversation, lifecycle put as Q3).

### D3 — Frame: hardened at the first reply, amended only by the user's word — `Confident`

**Statement.** The frame hardens at the user's first reply to the card: every guessed line is
confirmed, corrected or struck before deciding starts. A line the user cannot answer yet is
marked open and becomes an early decision instead of a silent guess. After hardening, the frame
changes one line at a time and only by the user's word — when a fact breaks a line, when the
user changes their mind, or when a decision rules something out — and each change is stamped in
the record.

**Rationale.** A guess confirmed only when a decision first touches it can shape the decisions
before that one. That is the path the legacy assumption took in `setup-product-agnostic` (F6).

**Rejected road.** Progressive hardening — the card is shown, deciding starts at once, and each
guessed line is confirmed when a decision first touches it. It loses because an unconfirmed
guess steers earlier decisions unseen.

**Accepted risk.** The user must read and react to six lines (five before the review added the
size line; corrected at verify, V2) before any design question is
asked.

**Detail, confirmed in the wrap-up batch.** A changed destination re-checks the decisions already made against
the new one. Said in the lead's walk-through, at first without a question.

**How it was decided.** Q3; user: "as recommeded" — the lead's recommendation, ratified.

### D4 — Decision map: visible, never a gate — `Confident`

**Statement.** Once the frame has hardened, the lead shows a decision map in three parts:
askable now (the decisions it can state sharply, each by name), in fog (loosely stated, with
what each hangs on), and out of scope. The map arrives in the same message as the first
question. The user is not asked to approve it and may edit it in any reply. After that, each
turn carries one status line (decided · askable now · in fog), and the full map is shown again
when it changes. Only a question that can be stated sharply is listed; the rest stays as fog and
graduates as decisions land.

**Rationale.** The user's own read: "yes A definitely matches what i like". A wrong map is
cheaper than a wrong frame, because a bad question is cut when it is put. This session is itself
evidence that the user works well against a visible list — the user asked to go through the
stage table "one by one".

**Rejected roads.** A gated map, confirmed or edited before the first question — one more round
trip per session for the same content. No map, as today — the user learns that a question
exists only when it is asked.

**Accepted risk.** An ungated map can be skimmed past, so a missing decision may be noticed
late. A full list up front is a separate trap the statement avoids: wayfinder's field report is
27 tickets charted and stale by the thirteenth (F10).

**Detail, confirmed in the wrap-up batch.** The map is the lead's working plan: the lead may add or graduate
items, always visibly. Ruling something out of scope is a frame change and takes the user's word
(D3). Told to the user when they asked how the two options differ, at first without a question.

**How it was decided.** Q4, put with a weak lean after the streak note; user: "yes A definitely
matches what i like, however, what would be difference between A and B?" — the difference was
answered in the conversation.

### D5 — Facts: the fact is in hand before the decision is put — `Confident`

**Statement.** A decision that hangs on a fact is not put to the user until the fact is in hand.
The lead finds what the repo, the platform or the docs can answer — itself or through a cheap
explorer seat — and puts the decision with the fact and its source attached. Other askable
questions proceed while fact work runs. A fact only the user holds is asked plainly as a fact
question and labelled as one, never dressed as a decision.

**Rationale.** In `orchestrator-model-selection` several review survivors were platform facts
nobody had checked (F7) — the cost of putting a decision ahead of its fact. (Corrected at
review, S11: the lead had also cited two re-put questions in `hook-enforcement-field-review`.
There the facts were already in hand and the user was challenging the question's premise. The
user was told and did not reopen the ruling.)

**Rejected road.** Ask first, check after — the decision is put on the lead's best belief and
verified later. It loses because a ruling made on a wrong belief comes back to the user.

**Accepted risk.** Fact work costs tokens and time, some of it on decisions the user later cuts.

**Detail, confirmed in the wrap-up batch.** A fact that cannot be found is stated as unknown, and the decision
carries it as an open question. First stated by the lead without a question.

**How it was decided.** Q5; user: "yes A" — the lead's recommendation, ratified.

**Changed at review (S6, a coverage finding; Q20, user: "as recommended").** Added: in the new
brainstorm a fact enters the record as a quoted source line with its file and line number, and
the lead's reading of it is written separately and marked as a reading. The session had never
looked at how facts are written down. The history it missed: `brainstorm-v2-2-revision` F9 found
that an over-claim "lived in the paraphrase" of the fact-checker's map, and its D2 made that map
land verbatim; no fact-checker sits in a session like this one, and this record's own F section
repeated the fault three times (S8, S9, S11). Rejected: leaving it open until after dogfood;
exploring it as its own topic, for instance a fact-checking seat that writes the facts section.
Accepted: quoted lines make a record longer, and a quote can still be chosen selectively. This
record is not rewritten to the new form; its wrong facts are repaired and sources added where
the reviewer could not verify.

### D6 — Asking: what earns its own turn — `Confident`

**Statement.** A decision gets its own turn when there is a real choice, or when a wrong pick is
costly to undo. Every other decision is a default: shown by name in one line when it arises, and
confirmed or changed by the user in one batch. The user can pull any default out to its own
turn.

**Rationale.** The user's sessions already end in a wrap-up batch (`hook-enforcement-field-review`
Q10, `delta-files-vs-direct-baseline-edits` Q6); this names the rule for what may go into it.
One turn per obvious answer spends the user's attention where it adds nothing.

**Rejected roads.** Every decision one per turn, as today's rule says — turns spent on obvious
answers. The whole askable set in one round with a recommendation on each, as `grilling` does —
fast, and an invitation to answer yes to all of them.

**Accepted risk.** The lead sorts forks from defaults, so a real fork can hide in the batch. Two
guards: each default is shown by name when it arises, and the user can promote any of them.

**Detail, confirmed in the wrap-up batch.** The batch is confirmed at wrap-up; a default that other questions
hang on is confirmed before them. First stated by the lead without a question.

**How it was decided.** Q6; user: "as recommded" — the lead's recommendation, ratified. This
session already runs that way: the Order row was shown as a default and held (B1).

### D7 — Asking: the lead writes the options, a blind second list on costly forks — `Confident`

**Statement.** On every fork the lead writes the options under one rule: at least two real
options, one of them the cheaper shape (do less, reuse, or do nothing), and the rejected road
named with its reason. On a fork that is costly to undo, one fresh seat is also given the frame
and the question — never the lead's options — and returns the roads it sees; the lead shows the
user where the two lists differ before the user rules.

**Rationale.** Seven records carry a review finding that a road was never put to the user (F7, as corrected
at review, S8);
in `setup-product-agnostic` three of sixteen survivors were of that kind (F6). The rule makes the
cheaper shape and the rejected road present on every fork; the blind list is aimed at the roads
a lead with a favourite does not think to put.

**Rejected roads.** The lead alone under the rule, with no second list (level 1) — the rule can
be met with a strawman option, and only the end review would catch it. Full designs by two or
three seats under different constraints, on the user's request (level 3) — not chosen; the user
ruled level 2.

**Accepted risk.** One extra seat per costly fork. The premise is untested here: nothing yet
shows that a blind list finds roads the lead misses (OQ2).

**History check.** v2's standing advocate was retired for machine traffic and for folds into the
user's rulings (F4). This seat works before the user rules and returns one list, so nothing is
folded into a ruling.

**How it was decided.** Q7, put with the lead's uncertainty stated; user: "lets do level 2".

### D8 — Bare yes: the yes stands, a run of them changes the next question — `Confident`

**Statement.** A yes that follows the lead's pick stands as the user's ruling and is recorded as
ratified. After three ratified rulings in a row the lead says so and puts the next fork in a
different form — the case against first, or a weak lean in place of a firm pick. "Whatever you
think" is not a ruling; it is re-asked as a choice between two concrete options. An answer that
sounds like convention and not a want draws one probe: what would you want if you need not
justify it. The cold reviewer aims first at the decisions recorded as ratified.

**Rationale.** In `setup-product-agnostic` six decisions were marked `Confident` on answers such
as "yes" and "confiremd" (F6), which hides from the reviewer which rulings the user reasoned
through. The removed streak doctrine carried dogfood evidence that flagging a run of yeses
brought the user back to genuine reads (F2).

**Rejected roads.** Push on costly forks — a bare yes on a costly fork draws one follow-up asking
for the user's reason. It slows a decisive user and adds machine traffic, the fault v2's advocate
was retired for (F4). Leave it — a yes is a yes, as today.

**Accepted risk.** The flag can become noise the user learns to skip, and a ratified costly fork
still passes. This session shows one result each way: at Q4 the changed form drew a reasoned
answer; at Q8 it drew a bare yes.

**Detail, confirmed in the wrap-up batch.** "Ratified" is written in the decision's how-it-was-decided line;
no new confidence mark is added. First stated by the lead without a question.

**How it was decided.** Q8, put with a weak lean after the second streak note; user: "as
recommded" — ratified, the fourth in a row. The user did not say whether the flag had helped.

### D9 — Record: one card per decision, the long index kept — `Contested`

**Statement.** Inside the record, a decision's card is its only home. The card is titled by the
decision's name and has fixed parts: statement, why, rejected roads, accepted risk, how it was
decided, and changed at review. Everything else in the record — the at-a-glance list, the status
line, the question trail — points to the card by name and carries at most a one-line gist. The
cross-session index keeps a long entry per session, as today.

**Rationale.** At least twenty sessions had verify round 1 come back not clean, mostly because a
fix was made in one place and its copies went stale (F5); one home inside the record removes
those copies. The user ruled the index stays long. It is what made cross-session lookups cheap
in this very session: F4 and F5 were read from it without opening a record.

**Rejected roads.** A thin index as well — one line and a pointer per session. This was the
lead's weak lean; the user ruled against it. Today's form with names added — a review fix would
still touch the decision, the review table, the status header and the index.

**Accepted risk.** A change made after acceptance touches two places, the card and the index
entry.

**Detail, confirmed in the wrap-up batch.** The long index entry is written at acceptance, after the review's
changes are in — as today's index rule already has it — so a fix made during review touches the
card and its one-line gist only. First stated by the lead without a question.

**How it was decided.** Q9, put with the case against first. The user first asked how wayfinder
and the target differ and was answered with the billing example, then ruled: "I like the target
format. lets keep the long index" — against the lead's weak lean.

### D10 — Cold review: the blind map at the front, the same seat reading at the end — `Confident`

**Statement.** When the frame hardens in a standard or too-big session, a seat starts drawing a
blind angle map from the frame's problem, destination and out-of-scope lines — never from the
session or the record. The first question goes out without waiting for it. When the map lands,
the lead folds the load-bearing angles into the decision map, each marked as from the blind map; the
angles it drops stay listed for the end check. At the end the same seat is resumed and reads the
frozen record cold: it runs today's hunt, checks that each folded angle was answered in depth,
and checks the angles the lead dropped. No second map is drawn. A fresh seat reading the saved
map takes the end read only when the first seat is gone.

**Rationale.** The last five reviewed sessions each had three to eight coverage gaps (F5). Each
arrives after the record is written, and a decision reopened that way gets a bounded verify,
never a full cold read (`brainstorm.reopen-born-verify`, F1). Drawn at the front, the same
angles become ordinary decisions with the full treatment. The end read stays because most
findings are about the decisions themselves, which do not exist at the front (13 of 16 in F6),
and because no record is cleared by its author. The seat that drew the map has seen neither the
room nor the record, so resuming it keeps the independence today's reviewer has.

**Rejected roads.** The map at the end, as today. A second, fresh map at the end on top of the
front one — the only road that attacks the first mapper's blind spots, at the cost of two maps.
Always a fresh seat for the end read — one more seat start, in exchange for a reader with no
stake in the angles and a second pair of eyes (worded wrongly before the review as "no gain in
independence"; see the S7 note below).

**Accepted risk.** The lead sees the checklist it is graded on, so an angle can be covered
thinly. The resumed seat reads the record through its own map, so its blind spots survive to the
end, and an angle neither it nor the lead thought of goes unfound. A grep of
`cold-review-gap-challenge` finds no sign this move was weighed there (F4).

**Detail, confirmed in the wrap-up batch.** The seat that draws a blind second list on a costly fork (D7) is
never the review seat: a fork's question shows the session's direction, and the review seat must
not see it. First stated by the lead without a question.

**How it was decided.** Q10; user: "i like the idea of at the front and end too. however i want
to understand if we need fresh cold seat read the the end?" — answered in the conversation. Q10b;
user: "lets do 1" — the lead's recommendation, ratified.

**Changed at review (S2; Q16, user: "yes a").** The statement said the map is drawn "right
after the frame hardens" and feeds the decision map, while D4 has the decision map arrive with
the first question and D11 had the size — which decides whether a blind map is drawn at all —
proposed only once the map existed. Now the seat starts at hardening, for standard and too-big
only, and nothing waits for it. Added risk: an angle can land after the user has ruled on
something it bears on; that ruling then comes back to the user.

**Changed at review (S7; Q21, user: "keep 1").** The ruling on who reads at the end stands, on a
corrected account. The lead had told the user that a fresh seat brings no gain in independence
and that the resumed seat is "the same shape as today's reviewer with a pause". Both were
inaccurate: under this decision the seat's angles enter the session as map items, and the same
seat then judges how they were answered. It wrote none of the answers but has a stake in the
questions, which today's reviewer does not. The rationale's last sentence is to be read with
that correction. Re-put with the trade stated plainly — one seat saved per session against a
reader with nothing to defend — the user kept the same seat. Added to the accepted risk: the
end reader may be attached to its own angles.

**Changed at review (S16, a coverage finding; review batch, user: "confirmed").** The lead had
put a review pair out of scope without asking. Ruled inline: a pair stays possible; each seat
draws its own front map, as the live rule `brainstorm.pair-maps-independent` has it, and each is
resumed at the end. "No second map" in the statement means none at the end.

### D11 — Size: three sizes, ruled by the user — `Confident`

**Statement.** The lead proposes one of three sizes on the frame card and the user rules it in
the reply that confirms the frame. Small — no fog and a few forks: no front map and no blind
second lists; the review is today's, the review seat drawing its blind map at the end before it
sees the record — or the user's waiver. Standard — everything as designed. Too big — fog dominates: the session rules
the askable forks and hands the fog to named later sessions. The size can be raised
mid-session; it is never lowered without the user's word.

**Rationale.** Records of 1,536 and 1,375 lines (F8) show topics that outgrew one session, and
one depth makes a small topic pay for the full machinery. Today the only lever is waiving the
review.

**Rejected roads.** Standard plus the too-big split, with no small path — a small topic still
pays for a blind map. One size always, the review waivable as today.

**Accepted risk.** The small path is where rigour erodes: a lead can under-size a topic to save
effort. The guard is that the user rules the size.

**Detail, confirmed in the wrap-up batch.** Fog handed to a later session is written where the project keeps
its open threads, with the fog's own wording. First stated by the lead without a question.

**How it was decided.** Q11; user: "yes A" — the lead's recommendation, ratified.

**Changed at review (S2; Q16, user: "yes a").** The size was to be proposed "when the map is
first drawn". It could not be: the size decides whether the blind map is drawn, and the blind map
feeds that same map. It is now a line on the frame card (D2).

**Changed at review (S3; Q17, user: "yes A").** Small said "no blind map". That collided with
the confirmed constraint that the review's blind dispatch stays and with the protected floor
`review-brainstorm.blind-map-before-record-contact`. A small session now keeps today's review
and skips only the two new pieces. Rejected: a small review with no blind map at all, the floor
formally changed for small sessions. Accepted: a small session costs as much to review as a
session does today.

### D19 — Reach: brainstorm only — `Confident`

**Statement.** The target applies to `/mochiko:brainstorm` alone. `/mochiko:setup` keeps today's
questioning and today's review, and any other command that might follow is a separate concern,
out of this session's scope. Brainstorm's new rules for asking live in brainstorm's own rules,
not in the shared questioning skill, so setup is not touched.

**Rationale.** The user's words: "if other commands needs to change, thats a separate concern."
Every session read for this record was a brainstorm session; no evidence on how setup's
questioning goes was looked at (F6, F7).

**Rejected road.** Both commands change — setup's interrogation and its review follow the new
design. A bigger build, resting on evidence that was never gathered for setup.

**Accepted risk.** Two commands that ask questions will behave differently until someone takes
up the separate concern.

**What this settles about the earlier cut.** The v0.63.0 guardrails cut ruled on the text of
skill bodies (F2, F3). With the new asking rules going into the command's rules and the shared
skill's body left as it is, the cut itself is not reversed. One part of that ruling is
superseded, for brainstorm only (reworded at verify, V4, to agree with the S5 note below): the
Common Mistakes rows the cut kept deliberately (`.mochiko/strips/analysis-iterative.md`, the
v0.63.0 entry's kept-deliberately line) are overridden by brainstorm's precedence clause, and
that supersession is recorded at landing. Its finding
that detail buys weight, not honesty, still binds the form the new rules take: short and
checkable, not prose walkthroughs.

**How it was decided.** Q13, put in plain words; user: "yes A, if other commands needs to
change, thats a separate concern."

**Changed at review (S5; Q19, user: "as recommended", ratified on a weak lean).** Brainstorm's
rule still points at the shared skill, whose Common Mistakes rows say "One question per turn —
always" and list always-structured options as a mistake, while D6 and D17 batch and D13 fixes a
form. Brainstorm's own rule now states that where its rules and the skill differ, brainstorm's
rules apply. The skill's rows stay as they are for setup. Those rows were kept deliberately at
the v0.63.0 cut (`.mochiko/strips/analysis-iterative.md`, the v0.63.0 entry; the source was
first cited as F2, which does not carry it — corrected at verify, V4), so the override is
written down as a supersession for brainstorm at
landing. Rejected: rewording the shared skill's rows, which changes text setup reads. Accepted:
two texts that disagree stay in the plugin.

### D20 — The blind second list is built untested and tried in the dogfood repo — `Contested`

**Statement.** The blind second list (D7) is built without a prior test. Whether it finds roads
the lead misses is judged in use, in the dogfooding repo the user has set up, once the
implementation is done.

**Rationale.** The user's words: "build it , i have setup a dogfooding repo where we can use it
in a clean way after the implementation is done." A clean repo gives real sessions to watch,
where one replay of an old question would prove little either way.

**Rejected road.** Test first — one fresh seat given a past question word for word, checked
against the option the review later found missing, the piece dropped on a fail. This was the
lead's lean; the user ruled against it.

**Accepted risk.** A piece that turns out not to work is built before anyone knows. What the
dogfood runs are judged on was undecided when this was ruled; it is now D23 (corrected at
verify, V5).

**How it was decided.** Q14, put in plain words; user's ruling as quoted, against the lead's
lean.

### D21 — Shared questioning skill: two repairs, no change in behaviour — `Confident`

**Statement.** During the build, two faults in `skills/analysis-iterative/SKILL.md` are repaired.
The sentence "The adaptive flow above is the single questioning engine" is reworded so that it
no longer points at a flow that is not there. The output line that tells every caller to
generate the synthesis document is reworded to defer to the calling command's own deliverable,
because brainstorm's deliverable is the record and a synthesis is on request only. Setup's
output and behaviour stay as they are.

**Rationale.** Both faults are in F2. The first is a pointer to deleted text. The second is
wrong for brainstorm only: setup does conclude with a synthesis, so the repair must not replace
"synthesis" with "record" — it makes the line defer to the caller.

**Rejected road.** Leave the shared skill untouched and note the faults as a separate concern.
The user chose the repair.

**Accepted risk.** This is the one place the build edits text that setup also reads. D19's
"setup is not touched" is to be read as: setup's behaviour is not touched.

**How it was decided.** Proposed by the lead after Q13 as a default awaiting a word either way.
The lead described the second fault to the user as "names the wrong output file", which is
looser than the statement above. User: "yes fix the two broken sentences". (This part was
split from its card when D22 and D23 were inserted; put back at verify, V6.)

### D22 — The target and the August ruling: guardrails without a fixed order, three counts kept — `Confident`

**Statement.** The target is written into brainstorm's rules as guardrails — what must be true
before something else happens — and never as a sequence of steps: nothing in the command says
"first do this, then that". Three counted limits stay as limits: three ratified answers in a row
(D8), three questions without the map shrinking (D15), and a second failed verify going to the
user (D17). At landing, `command-architecture-realignment` D1 is annotated as amended in part
for brainstorm: counted limits return for these three cases; choreography and default pipelines
stay deleted.

**Rationale.** D1 of that session (`Contested`, `DECISIONS.md:108`) deleted stage and seat
choreography, default pipelines and counted bounds from commands, on the user's reasoning that
"the volume of encoded detail is itself the defect". Most of the target can be said as a
guardrail — the frame is confirmed before deciding starts, a fact is in hand before its decision
is put — which the August ruling keeps. The counts are the exception; a later ruling already
brought one back, the gate loop that sends a second failure to the user
(`author-grader-consolidation` D6).

**Rejected roads.** Brainstorm as an exception that carries a fixed order and the counts — it
re-admits the pipeline the August ruling deleted. The August ruling whole, order and counts left
to the lead — the faults found in this session, no stop rule and unremarked runs of yeses,
arose under lead judgment alone (F1, F6).

**Accepted risk.** Counted limits are back in a command, which is the direction the August
ruling moved away from.

**Consequence for D12.** Its dependency clause is a guardrail and stands: no question is put
before what it hangs on is settled. Its "a shape-defining fork goes first" clause is an ordering
preference; under this ruling it is the lead's judgment, not a rule. Told to the user with the
next question.

**How it was decided.** Review finding S1, a coverage finding put to the user as Q15; user:
"yes C" — the lead's lean, ratified.

### D23 — What the dogfood runs are judged on — `Confident`

**Statement.** The rebuilt brainstorm is judged in the dogfood repo on three counts, each set
against today's figures: the number of findings a review returns (today 15 to 25 survivors per
session, F5); the number of coverage findings that still arrive at the end (today 3 to 8, F5);
and whether the first verify pass comes back clean (today, in at least twenty sessions, it did
not, F5). The blind second list (D7) is judged inside the same runs: whether it names a road
the lead had not listed.

**Rationale.** The session had ruled a design and no measure for it; the review's blind map
carried that as a load-bearing angle and S15 found the open question covered only D7.

**Rejected road.** None was put to the user; the three counts were the lead's proposal.

**Accepted risk.** The figures being compared against come from sessions about mochiko itself,
and the dogfood repo's topics will differ, so a change in the counts may not be the design's
doing.

**How it was decided.** OQ3, put in the review batch; user: "confirmed , can we implement" —
ratified in a batch.

## Decisions confirmed in the wrap-up batch (D12–D18)

Rows the lead read as defaults, not forks. Each was shown to the user by name when its row came
up, under a working label (B1–B7), and all seven were confirmed in one batch — user: "confirmed,
lets do the open questions" — ratified. Each is marked `Confident`. They sit in the same `D…`
namespace as the cards above. The same answer confirmed the nine details the lead had carried
`Assumed` on D2, D3, D4, D5, D6, D8, D9, D10 and D11, and the four frame assumptions.

**What was rejected, per decision (added at review, S14).** None of the seven was put to the
user with alternatives, so none has a road the user rejected; what the lead had in view is
stated here so it is not silently blessed. D12 — order left wholly to the lead's judgment, as
today. D13 — no alternative considered; it is the form the session itself used. D14 — a runnable
prototype by default, as wayfinder's prototype ticket has it (F10); left to the user's request.
D15 — interview's "predict the next three answers" test, dropped in the decision's own text; and
no stop rule at all, as today (F1). D16 — a tool check of card parts at write time, rejected at
review (S4). D17 — today's unsorted hand-off of findings. D18 — no alternative considered.

- **D12 — Order (shown as B1).** Dependency order: a question is put only when what it hangs on is settled.
  Among the askable ones, a shape-defining fork goes first because it clears the most fog. The
  user may pick any askable item instead. Each question says in one line what it unblocks.
  *Changed at review (S1, D22; carried here at verify, V1):* the "shape-defining fork goes
  first" clause is no longer a rule; which askable item goes first is the lead's judgment. The
  dependency clause stands as a guardrail.
- **D13 — Question form (shown as B2).** The form this session has used, which the user said they like (session
  format, Topic) and which matches F11: plain words; decisions by name, never a bare id; each
  option one sentence plus one concrete example; the case against the lead's pick; a one-line
  recommendation; the whole question fits one screen. *Changed at review (S5; Q19):* this form
  applies when options are put. It does not forbid an open question to a user who is unsure,
  which is what the shared skill advises.
- **D14 — Show before asking (shown as B3).** When a question is about how something looks or behaves, the lead
  shows a rough artifact inside the message — a mock, a worked scenario with real numbers, or a
  replay on a past case — and the user reacts to it. No files and no code. Anything heavier, such
  as a clickable prototype, is built only on the user's request. Evidence in this session: the
  mock frame card and mock map drew the user's firmest answers, and the user asked for a worked
  billing example before ruling on the frame.
- **D15 — Stop rule (shown as B4).** Deciding is finished when the map is empty: nothing askable, every patch
  of fog ruled, deferred by the user's word or handed to a later session, no open line on the
  frame, and the defaults batch confirmed. The lead does not call it finished with an item still
  on the map. The user may stop at any time; what is left is recorded as open and the record
  says the destination was not reached. If the map has not shrunk for three questions running,
  the lead says so and offers to step back to the frame. The "predict the next three answers"
  test is not used, because nobody but the lead can check it. *Changed at verify (V3; Q22, user:
  "yes"):* one more condition — any blind map that was started has landed, and each of its angles
  is folded into the decision map or listed as dropped. Without it, a session whose first
  question no longer waits for the map (D10, S2) could finish before the map arrived, and its
  angles would reach the user as end-of-session findings. Accepted: a quick session may wait a
  few minutes for the seat.
- **D16 — What the end reviewer checks besides today's hunt (shown as B5).** Three additions. The seat's
  first step is a checklist that every card has its fixed parts, reported before it reads for
  judgment. The seat aims first at decisions recorded as ratified (D8). And it runs the
  builder test against the frame's destination: reading only the record, what would a builder
  still have to ask the user. *Changed at review (S4; Q18, user: "as recommeded", ratified on a
  weak lean):* the first addition was worded as "a mechanical check" with no mechanism named,
  and had been batched as a default although it was a real choice. It is now the seat's own
  checklist, with no new tooling. Rejected: a tool check at write time, which would give the
  record a fixed template it deliberately lacks today and needs a migration; to be revisited if
  dogfood runs show cards keep arriving incomplete. Accepted: a checklist run by a seat can skip
  or misjudge, and an incomplete card still reaches the review. *Changed at build (Q23;
  2026-10-06, user: "2A"):* a missing card part is reported and does not itself block `ready`;
  what blocks `ready` stays with the fitness items of `references/RECORD-FITNESS.md`. Put to the
  user when the build found that the new checklist, appended to that file, would otherwise sit
  under its "any unchecked item blocks `ready`" line and gain a consequence this decision never
  ruled. Rejected: any missing part blocks `ready` — it would make the checklist a gate this
  decision called a report.
- **D17 — Fix and verify (shown as B6).** Findings reach the user sorted by D6's two tests: a finding that
  offers a real choice, is costly to get wrong, or challenges a ruling of the user's gets its own
  turn in the question form (B2); lead repairs of facts, counts and wording go in one named
  batch; a finding that changes nothing is listed with its reason. A fix edits the one card and
  adds a line under changed at review (D9). The review seat then verifies once, reading only the
  changed cards and their gists; a second failure goes to the user. A coverage finding at the end
  keeps today's routing — explore now, rule inline, or defer, by the user's word.
- **D18 — Accept (shown as B7).** Before the record freezes for review, one screen: the decisions by name with
  their gists, the defaults batch, what is out of scope, the open questions, and each bet still
  untested with how it would be tested. The user confirms or changes it. After review, one screen
  of what changed. Acceptance is the user's explicit word; anything vaguer is asked once more.

## Open questions

- **OQ1 — Does this record carry a build surface?** Put as Q12. The user answered that the build
  happens in this same session. The lead's reading, not yet confirmed in so many words
  (`Assumed`): the record carries the decisions a builder would otherwise have to ask about —
  setup's reach (Q13), the replacement of the earlier cut, and the paused review seat, which
  proved to be a fact and is settled by F13 (wording corrected at review, S13) — and the build plan itself is written after acceptance as a
  separate `wave<n>-<slug>.md` file in this session's home.
- **OQ2 — Does a blind second list find roads the lead misses?** Still unanswered as a fact. When
  it gets answered is ruled: in use, in the dogfood repo (D20).
- **OQ3 — What are the dogfood runs judged on?** Closed → D23.

## For the landing (added at review, S1 and S15)

Prior rulings this record touches, each to be annotated when the record lands:

- `command-architecture-realignment` D1 — amended in part for brainstorm: three counted limits
  return (D22).
- `cold-review-gap-challenge` D2 and D6 — the blind map and its two-message dispatch move to the
  front of a standard or too-big brainstorm session (D10); setup's review is unchanged (D19).
- `brainstorm-v2-revision` — "all adversarial pressure moved to convergence" gains one
  exception: a blind second list before the user rules on a costly fork (D7).
- The v0.63.0 guardrails cut (`validator-scope-and-verbosity`) — its kept Common Mistakes rows
  in the shared skill are superseded for brainstorm only, by a precedence clause (D19, S5).
- `brainstorm-v2-2-revision` D2 — the verbatim rule for facts is extended to sessions with no
  fact-checker seated (D5, S6).

Open backlog items this record affects:

- **The first-live-run watch on the blind map** (`BACKLOG.md`, Cold-review gap-challenge
  residuals) — for brainstorm it moves to the front map; setup's half is unchanged.
- **The brainstorm plan-only eval kit** (`BACKLOG.md`, command plan-only eval kits) — its
  expectations are keyed to today's rules and are re-keyed in the build.
- **Closed — two repairs in the shared questioning skill.** Agreed by the user → D21.

## Out of scope and not done

- **Other commands.** Setup and any other command that asks questions (D19).
- **Full designs by several seats** on a shape-defining fork — level 3 of Q7, not chosen (D7).
- **A thin cross-session index** (D9).
- **The build plan.** Written after acceptance as a separate file in this session's home (OQ1).
- **A review pair under the front map** was listed here by the lead without a ruling; it is now
  ruled in D10's changed-at-review note (S16).
- **Dropped by the lead without a question to the user:** the glossary and term checks of
  `domain-modeling`, and `idea-refine`'s variation lenses (F9, F10). Interview's confidence
  percentage is dropped in D2 and its predict-the-next-three test in D15.

## Review (cold, round 1 — `reports/angle-map.md`, `reports/review.md`)

The blind map came back on 2026-10-04, before any record contact: 63 angles, 25 of them
load-bearing, in nine classes (premise and evidence 7/4 · collisions with prior rulings 9/5 ·
stage coverage 15/8 · design choices within a stage 6/1 · review and integrity 4/1 · stress
scenarios and exit paths 8/1 · fit to mochiko machinery 7/1 · landing and evaluation 4/3 ·
excess watch 3/1). The seat attested `record_contact: none`.

**Fence disclosure, weighed by the lead.** While grounding one angle the seat opened a file
outside the repo: the lead's session memory note on question form. Its 2026-10-04 addendum names
this session's Q12 and three labels the lead had coined; it holds no decision text. The lead
judges the map's blindness intact — the note says that a question about how to build was asked
and was hard to read, not what was ruled. The memory directory was not named in the fence; that
omission is the lead's. Disclosed to the user.

**Routing disclosure.** The seat's two haiku Explore dispatches ended without handing back, so
it took those reads itself, on its own tier.

The record was declared frozen and sent to the seat as message 2 once this section was written.

### Verdict, round 1

Recommended status **`needs-revision`**. Tally: 26 raised, 16 survived — no Critical, 7
Important, 9 Minor; four of the sixteen are coverage findings (S1, S6, S15, S16). The seat's
reason for the status: the Destination promises a builder can plan without asking the user a
design question, and its builder test (D16) run against this record turns up at least five such
questions (S1–S5). Facts: 11 of 13 checked first-hand, F1–F4, F6, F8, F9, F12 and F13 hold.
Fitness: six of seven items pass, one partial (S14). A third haiku Explore dispatch failed to
hand back; the seat took all reads itself.

The lead re-read the sources behind S1, S3, S8, S10 and S11 before proposing anything:
`command-architecture-realignment` D1 and `DECISIONS.md:108` say what S1 quotes; the blind-map
floor's protection is in `.mochiko/strips/review-brainstorm.md` (v0.100.0); S8's count is right;
S11's reading of `hook-enforcement-field-review` Q3 and Q7 is right and the lead's was wrong;
S10's quote does exist, at `docs/engineering/wayfinder.md:75` of the clone at `d81f3a1`.

| # | Severity | Touches | Finding, short | Proposed disposition | Ruling |
|---|---|---|---|---|---|
| S1 | Important, coverage | D8, D12, D15, D17, D19, constraint 2 | The August ruling that commands are goal plus harness — choreography, default pipelines and counted bounds deleted, not relocated (`command-architecture-realignment` D1, `Contested`) — was never weighed. The target brings back an order of stages and three counts | Candidate topic, put to the user on its own | user: "yes C" → D22 (guardrails without a fixed order, the three counts kept; the August ruling annotated amended in part at landing; D12's "shape fork first" becomes the lead's judgment) |
| S2 | Important | D4, D10, D11 | The blind map must exist before the decision map is first shown, yet whether to draw it hangs on a size proposed only when the map is first drawn; the size ruling is a user turn while the map is "never a gate"; the wait for the seat is unpriced | Put to the user: the size is proposed on the frame card and ruled in the user's first reply; the seat is spawned at hardening for standard and too-big only; the first question goes out while the map is drawn and its angles are added to the decision map when they land | user: "yes a" — folded into D2, D10 and D11, each with a changed-at-review line or wording |
| S3 | Important | D11, constraint 1 | Small means "no blind map", while constraint 1 keeps the blind dispatch and `review-brainstorm.blind-map-before-record-contact` is a protected floor | Put to the user: a small session keeps today's review — the blind map drawn at the end, before record contact — or the user's waiver; it skips only the front map and the blind second lists | user: "yes A" — folded into D11 with a changed-at-review line |
| S4 | Important | D6, D9, D16 | The "mechanical check" of card parts names no mechanism. A tool check changes the record's home, which today has no template; a seat check is not mechanical. By D6's own test this was a fork, batched as a default | Put to the user: the review seat runs it first as a checklist; a tool check waits until dogfood runs show missing parts recurring | user: "as recommeded" — folded into D16 with a changed-at-review note |
| S5 | Important | D6, D13, D19, D21 | Brainstorm's rule still points at the shared skill, whose Common Mistakes rows say "One question per turn — always" and warn against always-structured options; D6 and D17 batch, D13 fixes one form | Put to the user: brainstorm's own rule states that its rules win where the two differ; the skill's rows stay as they are for setup; the kept-deliberately text gets a recorded supersession for brainstorm at landing | user: "as recommended" — folded into D19 and D13, each with a changed-at-review note |
| S6 | Important, coverage | D5, the F section | Facts entered this record as the lead's paraphrase and three were wrong (S8, S9, S11) — the failure the fact-checker's verbatim map was introduced to stop | Candidate topic; recommend rule inline: a fact enters the record as a quoted source line with its path and line, and the lead's reading of it is written separately | user ruled the path inline: "as recommended" — folded into D5 with a changed-at-review note |
| S7 | Important, challenges a user ruling | D10 | The lead told the user a fresh end seat brings "no gain in independence". Inaccurate: under D10 the seat's angles enter the room as map items and the same seat then grades how they were answered, which gives it a stake | Reserved to the user: who reads at the end is re-put with the trade stated accurately | user: "keep 1" — D10 stands; its rejected-road and risk lines corrected, with a changed-at-review note |
| S8 | Minor | F7, D7 | "Eleven records" overstates: ten other records match, three of them not findings about an unput road — seven genuine | Lead repair | user: "confirmed" (review batch) — applied |
| S9 | Minor | F5 | "`critical-gaps` occurs seven times" is wrong: 22 occurrences, seven of them backticked | Lead repair | user: "confirmed" (review batch) — applied |
| S10 | Minor | F10 | The prototype-variant report was not found by the seat | No change to the claim; the source line is added (`docs/engineering/wayfinder.md:75`) | user: "confirmed" (review batch) — applied |
| S11 | Minor | F7, D5 | D5's evidence from `hook-enforcement-field-review` is misread: at Q3 and Q7 the facts were already in that record and the user challenged the premise of the question | Lead repair of F7 and of D5's rationale; D5's statement untouched; the user is told the recommendation leaned on a misread | user: "confirmed" (review batch) — applied |
| S12 | Minor | F5, D10 | D10 cites F5 for "three to eight coverage gaps", which F5 does not carry | Lead repair: F5 gains the coverage counts | user: "confirmed" (review batch) — applied |
| S13 | Minor | Destination, OQ1 | "A later build session" against the user's same-session ruling; the paused seat called "a fact to test" in one place and closed in another | Lead repair | user: "confirmed" (review batch) — applied |
| S14 | Minor | D12–D18 | No rejected roads on the seven batch decisions | Lead repair: each states what was considered, or says plainly that no alternative was put to the user | user: "confirmed" (review batch) — applied |
| S15 | Minor, coverage | D10, OQ3 | The backlog's first-live-run watch on the blind map and the brainstorm eval kit are both affected and unmentioned; OQ3 covers only D7 | Candidate topic; recommend rule inline: both named for the landing, OQ3 widened to the whole target | user: "confirmed" (review batch) — applied |
| S16 | Minor, coverage | D10, Out of scope | The review pair was put out of scope by the lead, never ruled; `brainstorm.pair-maps-independent` is live and collides with "no second map" | Candidate topic; recommend rule inline: a pair stays possible, each seat drawing its own front map; "no second map" means none at the end | user: "confirmed" (review batch) — applied |

### Verify round 1 (`reports/review.md`, entry "Verify round 1") — NOT CLEAN, nothing blocking

Eighteen items graded: the sixteen folds, plus D22 and D23 on the bounded grade. Thirteen clean;
S1, S2, S5, S15 and D23 not clean, for six defects the folds introduced. The seat withdrew S10
as its own error — its grep had cut the line short, and F10 held all along. No fold reopened a
decision. The seat's closing line: once V1–V6 land, nothing in the review stands against
acceptance and no further cold read is owed.

| # | Defect the folds introduced | Disposition |
|---|---|---|
| V1 | D22's change had not reached D12 or the Order cell | lead-repaired |
| V2 | D3's accepted risk still said five lines | lead-repaired |
| V3 | The stop rule could fire before a started blind map had landed | user: "yes" (Q22) — D15 gains the condition |
| V4 | D19's paragraph on the earlier cut contradicted the S5 note, and F2 was cited for a fact it does not hold | lead-repaired; the strip file is cited |
| V5 | D20's risk still called the dogfood measure undecided | lead-repaired |
| V6 | D21's how-it-was-decided part had been split onto D23 | lead-repaired |

The six repairs were made after the verify and were not read again by the seat: closed
seat-unverified, disclosed.

## Question trail (Q)

- **Q1 — Open: how much the lead reads before its first message.** Light orient — the index,
  the records the topic names, the user's earlier rulings on the topic, with fact work waiting
  for the map (A, recommended) · deep ground first — A plus a sweep of code and consumers before
  the frame (B). **Answer:** "yes A" → D1.
- **Q2 — Frame: how the session pins down what it is about before deciding starts.** One card in
  one turn — five lines (problem · destination · must not break · betting on · out of scope),
  each marked as the user's words or the lead's guess, the user strikes or corrects; a line the
  lead cannot fill even as a guess is asked on its own (A, recommended) · interview first — who,
  why now, success and limit asked one per turn, the card shown after (B). **Answer:** "yes A"
  → D2. The user then asked, with a billing-system example: "at what point frame will be created
  or harden". The lead answered with a walk-through — drafted in the lead's first message from
  the opening context, hardened by the user's reply — and put the lifecycle as Q3.
- **Q3 — Frame: when it hardens and how it changes afterwards.** Hardened at the user's first
  reply — no line left as an unconfirmed guess; a line the user cannot answer yet is marked open
  and becomes an early decision; later changes go one line at a time, by the user's word only,
  each stamped in the record (A, recommended) · progressive — the card is shown, deciding starts
  at once, and each guessed line is confirmed only when a decision first touches it (B). **Answer:** "as
  recommeded" → D3.
- **Streak note (lead, after Q3).** Three decisions in a row followed the lead's recommendation
  (Q1 "yes A", Q2 "yes A" with a question of the user's own, Q3 "as recommeded"). The lead flagged
  the streak to the user and put Q4 with a weak lean in place of a firm recommendation, because
  Q4 turns on how the user likes to work.
- **Q4 — Decision map: does the user see what is coming, and must they confirm it.** A visible
  map that is not a gate — askable-now decisions by name, fog, out of scope, shown with the first
  question after the frame hardens, editable at any time, one status line per turn after (A, weak
  lean) · a visible map the user confirms or edits before the first question (B) · no map, as
  today (C). Only what can be stated sharply is listed; the rest stays as fog. **Answer:** "yes A
  definitely matches what i like, however, what would be difference between A and B?" → D4. The
  difference was answered in the conversation: same content, B adds one round trip and makes the
  list something the user approved, A leaves it the lead's visible working plan.
- **Order row** — shown to the user as a proposed default and held for the wrap-up batch (B1),
  not put as a question.
- **Q5 — Facts: must a fact be in hand before the decision that hangs on it is put.** Fact
  first — the lead finds what the repo, the platform or the docs can answer and puts the
  decision with the fact and its source attached; other askable questions proceed meanwhile; a
  fact only the user holds is asked as a fact question, labelled as one (A, recommended) · ask
  first, check after — the decision is put on the lead's best belief and verified later (B).
  **Answer:** "yes A" → D5.
- **Asking row** — split by the lead into three questions (what earns its own turn · where the
  options come from · when the lead shows instead of asks); the question form shown as a
  proposed default (B2).
- **Q6 — Asking, part 1: what earns its own turn.** Two tests — a decision gets its own turn
  when there is a real choice or a wrong pick is costly to undo; everything else is a named
  one-line default confirmed in one batch, and the user can pull any default out to its own turn
  (A, recommended) · every decision one per turn, as today's rule says (B) · the whole askable
  set in one round, each with a recommendation, as `grilling` does (C). **Answer:** "as
  recommded" → D6.
- **Q7 — Asking, part 2: where the options on a fork come from.** Three levels, each including
  the one before. Level 1 — the lead writes them under a rule: at least two real options, one of
  them the cheaper shape, the rejected road named with its reason. Level 2 — on a fork that is
  costly to undo, one fresh seat is given the frame and the question but not the lead's options
  and returns the roads it sees; the lead shows where the two lists differ (recommended, with
  the lead's uncertainty stated). Level 3 — on the user's request only, two or three seats each
  draft a full design under a different constraint and the lead compares them. **Answer:** "lets
  do level 2" → D7; the untested premise carried as OQ2.
- **Asking, part 3** — show before asking: shown to the user as a proposed default and held for
  the wrap-up batch (B3), not put as a question.
- **Streak note (lead, after Q7).** Three more rulings in a row followed the lead's pick (Q5,
  Q6, Q7). Flagged to the user; Q8 is the rule for exactly this and is put with a weak lean.
- **Q8 — Bare yes: how the lead treats a run of answers that follow its pick.** Flag and change
  form — the yes stands and is recorded as ratified; after three in a row the lead says so and
  puts the next fork with the case against first or a weak lean (A, weak lean) · push on costly
  forks — A, plus a bare yes on a costly fork draws one follow-up asking for the user's reason
  in a line (B) · leave it — a yes is a yes, as today (C). In all three, "whatever you think" is
  not a ruling and is re-asked as a two-option choice, and an answer that sounds like convention
  draws one "what would you want if you need not justify it" probe. **Answer:** "as recommded"
  → D8.
- **Stop rule row** — shown to the user as a proposed default and held for the wrap-up batch
  (B4), not put as a question.
- **Q9 — Record: how far the one-home rule goes.** Put with the case against first, per D8. The
  card is the only home — a decision's card holds its statement, why, rejected roads, risk, how
  it was decided and any review change; everything else, the index included, points by name and
  carries at most a one-line gist (A, weak lean) · cards and names inside the record, the index
  kept as today's long digest (B) · today's form with names added (C). Put 2026-10-04. The user
  asked: "i am confused how is wayfinder and target different, use the billign example again".
  The lead answered that on this row the target is wayfinder's one-home rule carried into
  mochiko's files — the home is a card in `record.md`, not a tracker ticket; the card has fixed
  parts where a ticket holds a question and a free-form answer; and the card takes review
  changes, which wayfinder has no review to produce. Q9 re-put as the one choice left: whether
  the cross-session index also goes thin. **Answer:** "I like the target format. lets keep the
  long index" → D9, `Contested` (against the lead's weak lean for a thin index).
- **Cold review row** — the end reviewer's three added checks shown to the user as a proposed
  default and held for the wrap-up batch (B5).
- **Q10 — Cold review: where the blind angle map sits.** At the front — a seat maps angles from
  the hardened frame's problem, destination and out-of-scope lines; the lead folds the
  load-bearing ones into the decision map, marked as from the blind map; at the end a fresh cold
  seat checks each angle was answered in depth and checks the angles the lead dropped; no second
  map (A, recommended) · at the end, as today — coverage gaps arrive as review findings and the
  user rules explore now, inline or defer (B) · both — the front map feeds the session and the
  end reviewer builds its own fresh map as well (C). Put 2026-10-04. **Answer so far:** "i like
  the idea of at the front and end too. however i want to understand if we need fresh cold seat
  read the the end?" — the front map is wanted and something at the end is wanted; whether the
  end read needs a fresh seat, and whether "end too" means a second map, is not yet ruled. The
  lead answered: an end read is needed because most findings are about the decisions themselves,
  which do not exist at the front (13 of 16 survivors in F6), and because no record is cleared by
  its author; but the reader need not be fresh — the seat that drew the front map has seen
  neither the room nor the record, which is the same shape as today's reviewer with a pause in
  the middle. The lead's "fresh cold seat" wording in option A was its own addition, not a
  requirement.
- **Q10b — Cold review: who reads at the end.** The front-map seat, resumed — no second map; a
  fresh seat with the saved map only when the first seat is gone (1, recommended) · always a
  fresh seat, reading the saved map and the record, no second map (2) · a fresh seat that also
  draws its own second blind map before it reads (3 — option C of Q10). **Answer:** "lets do 1"
  → D10.
- **Fix and verify row, Accept row** — each shown to the user as a proposed default and held for
  the wrap-up batch (B6, B7).
- **Q11 — Size: one depth, or scaled to the topic.** Three sizes, proposed by the lead when the
  map is first drawn and ruled by the user — small (no fog, a few forks: no blind map, no blind
  lists, one end read or the user's waiver), standard (everything as designed), too big (fog
  dominates: this session rules the askable forks and hands the fog to named later sessions); the
  size can be raised mid-session and is never lowered without the user's word (A, recommended) ·
  standard plus the too-big split, no small path (B) · one size always, the review waivable as
  today (C). **Answer:** "yes A" → D11. Every row of the stage table now has a ruled target or
  a held default.
- **Wrap-up batch, put 2026-10-04.** One screen for the user to confirm or change: the seven row
  defaults (B1–B7), the eight details the lead carried `Assumed` on D2, D3, D4, D5, D6, D8, D9,
  D10 and D11, and the four frame assumptions (the destination and constraints 1–3). The lead
  marked three for a second look: the stop rule (B4), the rule that the blind-list seat is never
  the review seat (D10), and constraint 3 — the evidence is the user's own mochiko-design
  sessions, and the billing walk-throughs are the only check on product topics. **Answer:**
  "confirmed, lets do the open questions" → B1–B7 ruled as D12–D18; the nine details and the
  four frame assumptions confirmed.
- **Streak note (lead, after the batch).** Three ratified in a row again (Q10b, Q11, the batch);
  Q12 is put with the case against first, per D8.
- **Q12 — OQ1: does this record carry a build surface.** A named hand-off — the record lists the
  primitives the target touches and three hard points, each handed to the build session as an
  open item and none ruled here: the reach into setup (F12), the partial reversal of the v0.63.0
  guardrails cut (F2, F3), and a review seat paused for a whole session and resumed (B, weak
  lean) · no build surface at all (A) · rule the three hard points now, in this session (C).
  A full plan by wave is not offered: five of sixteen survivors in F6 hit the build surface. Put
  2026-10-04. The user answered: "ask in simple language" — the question as put used the lead's
  own terms ("build surface", "named hand-off", "hard points") and ran long, against F11 and
  D13. Re-put in plain words as: should this record also say how to build the new brainstorm —
  no (A) · only a short list of what changes and three things the builder must ask the user
  about (B, the lead's lean) · decide those three things now (C). **Answer:** "i plan to build in
  this same session." The lead reads this as: no later session exists to hand anything to, so
  the points the builder would have had to ask about are decided here, before the review, and
  the how-to-build goes into a separate plan file after acceptance (see OQ1). The lead's lean for
  a hand-off is dropped.
- **Q13 — Reach: does setup change too.** Brainstorm only — setup keeps today's questioning and
  today's review; brainstorm's new asking rules live in brainstorm's own rules, not in the shared
  skill (A, the lead's lean) · both — setup's interrogation and its review follow the new design
  (B). **Answer:** "yes A, if other commands needs to change, thats a separate concern." → D19.
- **The other two points from Q12, closed without a question.** The earlier cut: not reversed
  under D19, so nothing to replace (D19). The paused review seat: resuming a seat inside one
  session is the harness's stated behaviour and today's dispatch already uses it; across
  sessions D10's fallback applies (F13).
- **Proposed to the user, awaiting a word either way:** during the build, repair two faults in
  the shared questioning skill that change no behaviour — the sentence pointing at a flow that
  is no longer there, and the output line that names the synthesis where the command names the
  record (F2).
- **Q14 — OQ2: test the blind second list before building it, or build it and watch.** Test
  first — one fresh seat is given a real past question, word for word as it was put, and the
  lead checks whether it names the option the review later found missing; a fail drops that
  piece before it is built (A, the lead's lean) · build it and watch the first real sessions
  (B). **Answer:** "build it , i have setup a dogfooding repo where we can use it in a clean way
  after the implementation is done." → D20, `Contested`.
- **Map empty, 2026-10-04.** Every row is ruled, both open questions that were to be put are
  answered, and what is left is listed under Open questions. The lead opened the cold review:
  message 1 (topic and goal line only) sent to the review seat for its blind angle map; the
  record goes to the seat only after the map is back.
- **The two repairs** — user, while the map was being drawn: "yes fix the two broken sentences"
  → D21. Written before the record was sent to the review seat.
- **Review round 1 back, 2026-10-04** — `needs-revision`, 16 survivors (Review section). The lead
  told the user the verdict, that three of its facts were wrong (S8, S9, S11) and that one of
  them had been used to recommend D5, and that seven findings need the user's ruling one at a
  time while nine go in one batch.
- **Q15 — S1: the August ruling on commands.** Put in plain words. Keep the stages as guardrails
  with no fixed order, and keep the three counts as limits; the August ruling is marked amended
  in part for brainstorm (C, the lead's lean) · brainstorm becomes an exception and carries the
  flow and the counts; the August ruling is marked replaced in part (A) · the August ruling
  stands whole: the design ships as guardrails only, order and counts left to the lead (B).
  **Answer:** "yes C" → D22.
- **Q16 — S2: the timing clash between size, blind map and first question.** Put in plain words.
  The size is proposed on the frame card and ruled in the reply that confirms the frame; for
  standard and too-big the blind seat starts then; the first question goes out without waiting,
  and the seat's angles are added to the decision map when they land, marked as from the blind
  map (A, the lead's lean and the reviewer's fix) · no question goes out until the blind map is
  back (B). **Answer:** "yes a" → folded into D2, D10, D11.
- **Q17 — S3: small sessions against the protected blind-map rule.** Put in plain words. A small
  session keeps today's review — the reviewer draws its blind map at the end, before it sees the
  record — or the user waives the review; small skips only the front map and the blind second
  lists (A, the lead's lean) · a small session's reviewer reads the record with no blind map at
  all, and the protected rule is formally changed for small sessions (B). **Answer:** "yes A" →
  folded into D11.
- **Streak note (lead, after Q17).** Three ratified in a row (Q15, Q16, Q17); Q18 is put with the
  cases against first and a weak lean, per D8.
- **Q18 — S4: what does the "mechanical check" of card parts.** Put in plain words. The review
  seat does it first, as a checklist, with no new tooling (A, weak lean) · a tool does it — the
  CLI checks the record when it is written and refuses a card with a missing part, which gives
  the record a fixed template it deliberately lacks today and needs a migration (B).
  **Answer:** "as recommeded" → folded into D16.
- **Q19 — S5: the shared skill's "one question per turn — always".** Put in plain words, cases
  against first (the streak stands at four). Brainstorm's own rule says its rules win where the
  two differ; the shared skill stays as it is for setup; the fixed question form applies when
  options are put and does not forbid an open question to an unsure user (A, weak lean) · the
  shared skill's rows are reworded too, which changes text setup reads (B). **Answer:** "as
  recommended" → folded into D19 and D13.
- **Q20 — S6, a coverage finding: facts quoted, not paraphrased.** Put in plain words, cases
  against first (the streak stands at five). Rule it now — in the new brainstorm a fact enters
  the record as a quoted line with its file and line number, and the lead's reading of it is
  written separately and marked as a reading; in this record the lead repairs the wrong facts
  and adds source lines where the reviewer could not verify (A, the lead's lean) · defer it as
  an open item until after dogfood (B) · explore it now as its own topic, for instance a
  fact-checking seat that writes the facts section itself (C). **Answer:** "as recommended" →
  folded into D5.
- **Q21 — S7, which challenges the user's ruling on who reads at the end.** Re-put in plain words
  with the trade stated accurately (the streak stands at six): the lead's earlier claim that a
  fresh seat brings no gain in independence was wrong — the resumed seat judges how its own
  angles were answered, so it has a stake in the questions though it wrote none of the answers.
  Keep the same seat — cheaper, and it knows what its angles meant (1, the lead's lean, weaker
  than before) · a fresh seat reads the saved map and the record — no attachment, a second pair
  of eyes, one more seat start (2) · a fresh seat that also draws its own second map (3).
  **Answer:** "keep 1" → D10 stands, corrected.
- **Review batch, put 2026-10-04.** Seven lead repairs (S8–S14), the lead's proposed inline
  rulings on the two minor coverage findings (S15: the backlog's first-live-run watch moves to
  the front map and the brainstorm eval kit is named for the landing; S16: a review pair stays
  possible, each seat drawing its own front map, "no second map" meaning none at the end), and a
  proposed answer to OQ3 — the dogfood runs are judged on three counts set against today's
  figures: findings per review, coverage findings that still arrive at the end, and whether the
  first verify comes back clean. **Answer:** "confirmed , can we implement" → S8–S16 applied,
  OQ3 → D23. All sixteen survivors are dispositioned. The user's "can we implement" is read as a
  wish to start the build, not as acceptance of the record; acceptance is asked for after the
  verify pass.
- **Verify pass opened, 2026-10-04.** The record was declared frozen again and the review seat
  resumed to grade the folds.
- **The user on acceptance, while the verify ran:** "happy for you to accept and start
  building". The lead does not accept on the user's behalf. It told the user it records those
  words as the user's acceptance, to take effect when the verify comes back clean or with
  repairs the reviewer itself marks non-blocking, and that anything blocking goes back to the
  user first.
- **Verify round 1 back, 2026-10-04 — NOT CLEAN, nothing blocking.** 18 items graded, 13 clean,
  5 not clean; six fold-introduced defects V1–V6. The reviewer withdrew S10 as its own error
  (its grep had cut the line short; F10 held all along). V1, V2, V4, V5 and V6 are lead repairs
  and are applied: D12 and the Order cell carry D22's change (V1); D3's risk says six lines
  (V2); D19's paragraph on the earlier cut agrees with the S5 note and cites the strip file
  (V4); D20's risk points at D23 (V5); D21's how-it-was-decided part is back on its card (V6).
  The reviewer's nits with no action: the 22 in F5 counts every occurrence of the bare string,
  backticked ones included; D22's "told to the user with the next question" refers to the
  lead's message that put Q16.
- **Q22 — V3, which amends the stop rule and so needs the user's word.** Put in plain words.
  With the first question no longer waiting for the blind map, the stop rule could fire before
  a started map has landed; its angles would then arrive after the record is frozen. Proposed:
  the stop rule also requires that any blind map that was started has landed, with its angles
  folded into the map or listed as dropped (yes, the lead's and the reviewer's proposal) · leave
  the stop rule as it is (no). **Answer (2026-10-06):** "yes" → D15 gains the condition.
- **Q23 — At build (2026-10-06).** Two questions put together when the prose seat's plan failed
  its peer grade after the wave's one shared re-plan round was spent: whether the seat may
  revise once more (A), be replaced (B) or have its scope narrowed (C); and whether a missing
  card part bars the reviewer from `ready` (A: no, reported only; B: yes). **Answer:** "1A, 2A"
  — the revision granted; the ruling written into D16's changed-at-build note.
- **Accepted, 2026-10-06.** With the verify back, nothing in it blocking, and V3 ruled, the
  user's earlier words — "happy for you to accept and start building" — take effect as the
  acceptance, on the terms the lead had stated to the user. The lead ran the close: this
  record's status, the index, `DECISIONS.md`, `BACKLOG.md`, `ROADMAP.md`, and the five prior
  rulings named under "For the landing".
