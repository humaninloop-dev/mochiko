# Obligation map — W1, the shared thinking stage

Seat: P1 schema (W1 producing seat), 2026-10-11. Written at build from the approved plan chain —
`w1-p1-plan.md` (sha256 `6d696b41…cf9a6dd`) → `w1-p1-plan-rev1.md` (`4d456da3…4efd1`) →
`w1-p1-plan-rev2.md` (`6c768d82…a52f`), the later revision winning — plus the build-time changes in
the last section. Before-texts: the plugin 0.119.0 renders. After-texts: migrations 0058–0062 as
stamped (binary 0.4.0 · grammar 2). Brief: `wave1-engine.md` §3.1.

## Scope and method

**What is mapped.** Every rule in the 0.119.0 renders that governs how the user is asked, how the
user's rulings are taken, or the register the user is addressed in: brainstorm (35 of 55 rules),
specify's story and intent stages (the 11 ids §3.1 names, `spec.story-stage`, and the untouched
stage rules, `spec.register` among them), setup's interrogation (`setup.interrogation-inline`, the
11 rules it leans on, and `INTERROGATION-AGENDA.md`'s default-on offers), and the prose skill's own
obligations.

**Excluded** — seat wiring, review mechanics, bookkeeping and gates that ask the user nothing (20
brainstorm ids): `staffing-latitude` · `record-review-independence` · `blind-map-dispatch` ·
`pair-maps-independent` · `small-session-review` · `index-bookkeeping` · `km-close-ritual` ·
`next-step-offer` · `model-tiering` · `plan-approval-producers` · `author-grader-default-fail` ·
`reopen-born-verify` · `no-git-mutations` · `artifact-home` · `fix-one-card-verify-once` ·
`transport-floor` · the four `brainstorm.fail.*`; and the prose skill's enrichment shape (no caller
asks through it, F27; stripped by D2-iterative-becomes-engine).

**Method.** Each engine rule split into its clauses; each clause read against each caller's
before-text. A clause the caller carried is `kept` or `reworded` on the caller's row; a clause it did
not carry is a `gained` row; a caller's word no carrier keeps is a `lost` row.

**Statuses.** `kept` — same obligation and words; the carrier may move. `reworded` — same
obligation, words changed or split between engine and command. `lost` — a caller's word no carrier
keeps. `gained` — a clause the caller takes from the engine and did not carry. "Engine `<slug>`"
abbreviates `analysis-iterative.<slug>`.

## Brainstorm

| # | Rule | Obligation | Carrier after W1 | Status |
|---|---|---|---|---|
| B1 | `brainstorm.lead-inline-questioning` | lead plans the run; runs the questioning inline via the skill | itself (the deciding is the thinking stage) + engine `lead-holds-conversation` ("You run the stage yourself, inline") + `brainstorm.thinking-stage` | reworded (L6; G2, G3) |
| B2 | `brainstorm.own-rules-win` | brainstorm wins over the skill | engine `engine-precedence` (flip ruled: default *Precedence*) | reworded — ruled flip |
| B3 | `brainstorm.blind-second-list` | costly fork: fresh seat, frame + question, not your options; differences shown before ruling; standard/too-big only | engine `blind-second-list` + itself (cadence: none on small; the frame as opening) | reworded (L16; G4) |
| B4 | `brainstorm.coverage-survivor-routing` | coverage survivor: user rules explore now (re-enter the skill) / inline / defer | itself | kept |
| B5 | `brainstorm.user-pen-boundary` | amendments and new decisions are the user's | itself + engine `user-rules` | kept |
| B6 | `brainstorm.frame-changes-users-word` | frame lines change only by the user's word, stamped | itself | kept |
| B7 | `brainstorm.size-users-ruling` | size proposed by the lead, ruled by the user | itself | kept |
| B8 | `brainstorm.user-steers` | user may edit the map, pick the next question, pull a default, stop | itself (map edit, next question, stop; out of scope is a frame change), pointing at engine `user-rules`, which carries the pull-a-default limb verbatim | reworded (at build — last section) |
| B9 | `brainstorm.deliverable-record` | `record.md` is the deliverable | itself (fills the engine's deliverable slot) | kept |
| B10 | `brainstorm.record-as-you-go` | record written as the session goes | itself | kept |
| B11 | `brainstorm.synthesis-on-request` | synthesis only on request, after acceptance | itself | kept |
| B12 | `brainstorm.decision-cards` | the ruled unit is a decision card | itself (unit slot) | kept |
| B13 | `brainstorm.facts-quoted` | a fact enters as a quoted source line; reading apart | itself | kept |
| B14 | `brainstorm.non-coverage-survivors` | a real-choice survivor gets its own turn in the question form; repairs in one named batch | itself ("the question form" is now engine `question-form`) | kept |
| B15 | `brainstorm.acceptance-plain-text` | rulings are plain blocking text | itself | kept |
| B16 | `brainstorm.light-orient` | read the index and named records first; no sweep | itself | kept |
| B17 | `brainstorm.frame-card` | first message: six-line frame card, guesses marked | itself (opening slot) | kept |
| B18 | `brainstorm.frame-hardens` | no deciding before the frame hardens | itself | kept |
| B19 | `brainstorm.decision-map` | three-part map; each later turn one status line `decided · askable now · in fog` | itself (map) + engine `status-line` | reworded (L1, L2) |
| B20 | `brainstorm.blind-map-fold` | fold the blind angle map into the map | itself | kept |
| B21 | `brainstorm.fact-before-decision` | fact first, source, found while other questions go on; user-only fact labelled; unknown carried open | engine `fact-before-fork` (every clause) | reworded (L5) |
| B22 | `brainstorm.own-turn-or-batch` | own turn for a real or costly choice; the rest shown by name in one line, confirmed in one batch at wrap-up; a hanging default first | engine `own-turn-or-batch` (every clause; "only" added). "Never by silence" is D1's statement, carried by engine `user-rules`; brainstorm's "confirmed in one batch" already required the user's word — kept in substance | reworded (L5, L13) |
| B23 | `brainstorm.options-rule` | two real options, the cheaper shape, the rejected road with its reason | engine `options-rule` (verbatim) | kept |
| B24 | `brainstorm.ratified-yes` | a yes stands, recorded as ratified in the card, no new mark; three ratified rulings in a row: say so, change the form | itself (card record) + engine `ratified-streak` (every clause, words kept) | reworded (G7) |
| B25 | `brainstorm.vague-answer-reasked` | "whatever you think" re-asked as two options; convention draws one probe | engine `vague-answer` (verbatim) | kept |
| B26 | `brainstorm.dependency-order` | no question before what it hangs on; each says what it unblocks | itself (D1 does not list it — moving it would add an obligation to setup and intent) | kept |
| B27 | `brainstorm.question-form` | plain words, names, one sentence + one example per option, case against, one-line pick, one screen; open question allowed | engine `question-form` | reworded (L4) |
| B28 | `brainstorm.show-before-asking` | rough artifact inside the message; never a file or code; heavier only on request | engine `mock-in-message`, verbatim. Under the base `engine-precedence` wording ("hold in every command whose thinking stage runs on this skill") the engine's rules hold wherever brainstorm asks, so the survivor turns after cold review (`brainstorm.non-coverage-survivors`) and the after-review screen keep this rule and every other moved one, as before W1 | kept |
| B29 | `brainstorm.stop-rule` | map empty (fog ruled, deferred by the user's word or handed on; no open frame line; defaults confirmed; blind map folded); a stopped session records what is open and says the destination was not reached; three questions without shrink: offer the frame | itself (what "empty" means, the deferral path, the destination line, the stall to the frame) + engine `stop-rule` (close rule, "left as open", three-turn stall). A deferral by the user's word is a ruling out of this session, inside the engine's "ruled in or out" — a domain fill, not a loosening | reworded (L3, L14) |
| B30 | `brainstorm.accept-screens` | one screen before freeze: decisions, defaults batch, out of scope, open questions, bets; acceptance explicit | itself (brainstorm's one-screen batch) | kept |
| B31 | `brainstorm.size-shapes` | too-big: rule the askable forks, hand fog to named later sessions | itself | kept |
| B32 | `brainstorm.user-record-acceptance` (floor) | record acceptance is the user's word, never yours | itself | kept |
| B33 | `brainstorm.user-review-waiver` | a review waiver is the user's | itself | kept |
| B34 | `brainstorm.user-survivor-challenge` | a survivor challenging a user ruling is the user's to dispose | itself | kept |
| B35 | `brainstorm.register` | user-facing prose follows `templates/output-style.md` | itself (beside engine `question-form`'s plain words — compatible) | kept |

## Specify — story and intent stages

| # | Rule | Obligation | Carrier after W1 | Status |
|---|---|---|---|---|
| S1 | `spec.story-forks` | one fork/turn, fact first, two options incl. the do-less shape (fewer stories or none), a one-row mock, own turn only for real or costly, the rest named and confirmed on the slate screen, status line over journeys | engine `one-fork-per-turn` · `fact-before-fork` · `options-rule` · `mock-in-message` · `own-turn-or-batch` · `status-line` + itself (do-less option, the **one-row** mock, slate screen, journeys) | reworded (L7, L8; G8–G12) |
| S2 | `spec.story-stage-stop-rule` | close when every candidate, the blind list's differences included, is ruled and nothing is open; three-turn stall back to the intent synthesis | engine `stop-rule` + itself (blind-list differences; stall to the intent synthesis) | reworded (L12; G15) |
| S3 | `spec.intent-probe-discipline` | intent as probes, one-screen synthesis; "in the intent and story stages alike, flag ratification streaks before treating adoptions as engagement"; story-stage vague re-ask | itself (probes; synthesis the user confirms before any authoring) + engine `ratified-streak` (both stages) + `vague-answer` (both stages) | reworded (L9; G13, G14, G31, G32) |
| S4 | `spec.lead-holds-story-stage` | lead runs the story stage inline, frame in hand; no teammate converses; analyst authors | engine `lead-holds-conversation` (words kept: "run … yourself, inline in the main conversation"; "no teammate holds a conversation with the user") + itself | reworded (G19) |
| S5 | `spec.analyst-brief-and-pen` | slate transcribed into `spec.md` at confirm, not authoring, before the analyst opens it; pen passed; brief: rows with the user's words, seeds, `out` rows with whys; a later change as exact row text, mechanical | engine `contract-pen` (transcription, pen, quoted words, exact text, mechanical) + itself (`spec.md`, slate confirm, the next writer's brief: seeds, `out` rows) | reworded (L10) |
| S6 | `spec.slate-binds-authoring` | `in` rows only; waiver-pending waits; the meaning of line and seed kept; the slate's count is the feature's; a missing story proposed as a row, never authored before ruled in | itself, reworded to specify's part (`in` rows · waiver wait · count · proposed as a row) + engine `contract-inside` (meaning kept; anything off the set back to the user before it is written) | reworded (L15; G18) |
| S7 | `spec.stage-inputs-in-hand` | a named record or stories in hand open the stage as facts; only open asked; no size gate; decisive user fast; nothing in hand, no skip | engine `inputs-in-hand` (specify's words) + itself (which inputs; no size gate; decisive user) | reworded |
| S8 | `spec.slate-provenance` | one mark per row after its line in the Story cell; the four-word vocabulary and its assignment | engine `provenance-marks` + itself (Story-cell position; a brainstorm-record row is `handed in` whatever the source) | reworded (L11; G16) |
| S9 | `spec.blind-journey-list` | once per run before the slate closes: fresh opus teammate, intent synthesis + frame, never slate or conversation; differences are candidate rows | itself, untouched (fills the engine list's cadence, inputs, job) | kept |
| S10 | `spec.intent-stage-first` | intent before authoring; adaptive-probe agenda via the skill | itself, pointer unchanged (now reaches the engine) | kept |
| S11 | `spec.own-rules-win` | specify wins over the skill | engine `engine-precedence` (ruled flip) | reworded — ruled flip |
| S12 | `spec.story-stage` | no story fork before intent confirmed; story set worked with the user into a ruled slate; slate ≠ selection | itself (+ on the thinking stage, contract `on`, slate = ruled set, analyst = next writer) + `spec.thinking-stage` | reworded |
| S13 | `spec.capability-frame-at-intent` | PM states the frame at intent, agreed with the user | itself + engine `lead-holds-conversation` | kept (+ G21) |
| S14 | `spec.slate-unit` · `spec.slate-home` · `spec.handed-in-slate` · `spec.gate-slate-confirm` · `spec.slate-rulings-users` · `spec.floor-obligated-rows` · `spec.slate-not-the-frame` · `spec.pm-vets-slate-once` · `spec.lockstep-prototyping` · `spec.reserved-to-user` · `spec.intent-synthesis-governs` · `spec.acceptance-plain-text` · `spec.register` | specify's unit, home, skip, slate gate, reservation, floor rows, frame line, PM vet, click-back amendments, intent confirm, synthesis governs, plain-text rulings, register | themselves, untouched | kept |
| S15 | — | story stage G8–G20 · intent stage G21–G35 | the foot | gained |

## Setup's interrogation

| # | Rule | Obligation | Carrier after W1 | Status |
|---|---|---|---|---|
| U1 | `setup.interrogation-inline` | lead runs it inline; agenda dimensions adaptively via the skill; then the deck card by card, recommend-then-arbitrate | itself, untouched + engine `lead-holds-conversation` · `options-rule` · `question-form` (a pick with the case against) | kept (+ G36–G50) |
| U2 | `setup.interrogation-inputs` | agenda, deck, domain dependencies; brownfield analysis | itself | kept |
| U3 | `setup.user-card-rulings` | every card, module and waiver ruling is the user's | itself + engine `user-rules` | kept |
| U4 | `setup.user-mode-ruling` | ambiguous mode ruled by the user | itself | kept |
| U5 | `setup.user-conflict-rulings` | brownfield conflicts confronted in the open | itself | kept |
| U6 | `setup.gate-synthesis-ratification` (floor) | ratification is the user's | itself | kept |
| U7 | `setup.synthesis-artifact` | `governance-intent.md` is the deliverable | itself (fills the deliverable slot) | kept |
| U8 | `setup.stress-test-cold-seat` | cold seat stress-tests the synthesis, or a waiver | itself (independent-leg slot) | kept |
| U9 | `setup.blind-map-dispatch` (floor) | stress-test seat's blind map, two messages | itself | kept |
| U10 | `setup.coverage-survivor-routing` | coverage survivor: explore now (re-enter the skill) / inline / defer | itself | kept |
| U11 | `setup.acceptance-plain-text` (floor) | rulings plain blocking text | itself | kept |
| U12 | `setup.register` | user-facing prose follows `templates/output-style.md` | itself | kept |
| U13 | `INTERROGATION-AGENDA.md` dimension 7 (lines 39, 44–54) and dimension 8's `release-gates` offer (line 40), read through `setup.interrogation-inputs` | dimension 7: the knowledge-management module "offered default-on … Recommend adoption — the user must actively decline" (line 45). Dimension 8: `release-gates` "default-on for a deployed product, recorded either way" — no active-decline clause | the agenda + `setup.user-card-rulings` + engine `user-rules` | gained — **ruled** (Q5): adopting either default-on module takes the user's yes; P2 edits lines 39–40 and 44–45 |
| U14 | — | G36–G50 | the foot | gained |

## The prose skill's obligations (read before W1 by setup, specify's intent, and brainstorm where no own rule differed)

| # | Obligation (`SKILL.md` at 0.119.0) | Carrier after W1 | Status |
|---|---|---|---|
| P1 | Overview: adapt style, depth and output to the user's state; each question builds on the last | `SKILL.md` Overview (P2) + engine `question-form` (open probe for an unsure user) | kept |
| P2 | When NOT to Use: quick clarifications · implementation details · spec review · clear direction → confirm and wrap up fast · time-sensitive | `SKILL.md` (P2; "Implementation details" rewritten — ruled V5/F21) | reworded |
| P3 | Output: the caller's deliverable, else `SYNTHESIS.md` | engine `deliverable` | kept |
| P4 | Floor: every elicited unknown surfaced as open, never silently omitted | engine `open-unknowns` (`class: floor`) | kept |
| P5 | Confidence indicators table | `SKILL.md` (P2) | kept |
| P6 | Mistake row "always using structured options" | `SKILL.md` row reworded (P2, ruled S3) + engine `question-form` | reworded |
| P7 | Mistake row "multiple questions per turn — one question per turn, always" | engine `one-fork-per-turn` (defaults batch the one exception) + `SKILL.md` row reworded (P2) | reworded |
| P8 | Mistake rows: unsure signals · disconnected questions · premature synthesis · rigid opening · padded synthesis | `SKILL.md` rows (P2); premature synthesis also engine `stop-rule` | kept |

## The foot — the list for the user

### `lost` rows — a caller's word that no carrier keeps

| # | Caller · rule | Before → after |
|---|---|---|
| L1 | brainstorm · B19 | status-line words "decided · askable now" → "ruled · askable" |
| L2 | brainstorm · B19 | "Each later turn carries one status line" → "Each turn" — the first turn now also carries a line beside the map |
| L3 | brainstorm · B29 | stall "three questions running" → "three turns running"; "the map has not shrunk" → "the candidates have not shrunk" |
| L4 | brainstorm · B27 | "an open question to an unsure user" → "an open probe" |
| L5 | brainstorm · B21, B22 | "decision" → "fork" four times ("No decision is put", "dressed as a decision", "the decision carries it", "A decision gets its own turn"); "a default other questions hang on" → "other forks" |
| L6 | brainstorm · B1 | "run the questioning yourself" → "run the stage yourself" / "its deciding is the run's thinking stage" |
| L7 | specify · S1 | "a candidate gets its own turn" → "a fork"; "the rest named in one line as they arise" → "shown by name in one line when it arises" |
| L8 | specify · S1 | "one the do-less shape" → engine "the cheaper shape — do less, reuse, or do nothing"; specify keeps "do-less option" |
| L9 | specify · S3 | "flag ratification streaks" → "flagging the streak" |
| L10 | specify · S5 | "you write the slate into `spec.md`" → "the slate is transcribed into `spec.md`"; "before the requirements-analyst seat opens it" → "before the next writer opens it"; "its brief" (the requirements-analyst seat's) → "the next writer's brief" (build); "a row change … reaches the seat as the exact row text" → "a change … reaches the writer as the exact text" |
| L11 | specify · S8 | "story lines the user handed in" → "inputs in hand"; "a row the blind journey list added" → "one the blind list added"; "any other row, ruled through the forks" → "otherwise" |
| L12 | specify · S2 | "the candidate set has not shrunk" → "the candidates have not shrunk" |
| L13 | brainstorm · B22 | "A decision gets its own turn when" → "A fork gets its own turn only when" |
| L14 | brainstorm · B29 | "a session the user stops records" → "a stage the user stops records" |
| L15 | specify · S6 | "each keeping the meaning of its row's line and seed" → engine "keeping each item's meaning"; "never authored before the user rules it in" → engine "goes back to the user before it is written" |
| L16 | brainstorm · B3 | "never the review seat" → "never a review seat"; "but not your options" → "never your options" |

### `gained` rows — a clause the caller takes from the engine that it did not carry

Brainstorm:

| # | Engine rule | Clause(s) gained |
|---|---|---|
| G1 | `one-fork-per-turn` | one fork per turn as a rule; the defaults batch the one exception |
| G2 | `lead-holds-conversation` | no teammate holds a conversation with the user |
| G3 | `lead-holds-conversation` | transcribing rulings takes no sound loop — a latitude; the record was already the lead's pen and its cold review stays ("the caller's own independent leg still runs") |
| G4 | `blind-second-list` | the wider fence: the seat never gets the conversation or the ruled set (before: "not your options") |
| G5 | `inputs-in-hand` | inputs in hand open as facts, never re-asked; only what they leave open is asked; a skip never skips the close |
| G6 | `provenance-marks` | one mark per ruled decision, the four-word vocabulary, shown as ruled, saved in `record.md` (before: frame lines marked; a ratified pick noted in the card) |
| G7 | `ratified-streak` | the streak flagged before adoptions are treated as engagement (specify's clause) |

Specify story stage:

| # | Engine rule | Clause(s) gained |
|---|---|---|
| G8 | `fact-before-fork` | the source attached; you find what the repo, platform or docs answer while other questions go on; a user-only fact asked as a labelled fact question; an unfound fact stated unknown and carried open (before: "its fact in hand first") |
| G9 | `options-rule` | the rejected road named with its reason |
| G10 | `question-form` | plain words; decisions by name, never bare ids; one sentence and one example per option; the case against and a one-line recommendation; one screen |
| G11 | `mock-in-message` | never a file or code; anything heavier only on the user's request. No collision with specify's prototype: the rule governs the artifact that comes with a question about how something looks or behaves; specify's prototype is a deliverable its own rules require (`spec.lockstep-prototyping`, `spec.whole-feature-prototype`), not a heavier artifact for a question |
| G12 | `own-turn-or-batch` | a default other forks hang on is confirmed before them |
| G13 | `ratified-streak` | the streak counted at three; the next fork changes form (before: flagged, no count, no form change) |
| G14 | `vague-answer` | a convention-sounding answer draws one probe |
| G15 | `stop-rule` | a stage the user stops records what is left as open |
| G16 | `provenance-marks` | each mark shown as its row is ruled (before: written in the Story cell) |
| G17 | `user-rules` | the user may pull a default out to its own turn ("silence never rules" kept in substance by the slate confirm) |
| G18 | `contract-inside` | the wider bar: anything off the slate that changes a ruling or that the user would notice goes back to the user before it is written (before: a story the slate lacks). No latitude: specify's rules grant the later-sign-off wait by name nowhere, so `spec.gate-acceptance` opens none — no loosening (R12) |
| G19 | `lead-holds-conversation` | transcription takes no sound loop — a latitude matching `spec.analyst-brief-and-pen`'s former "transcription, not authoring" |
| G20 | `blind-second-list` | the list's seat is never a review seat (before: "one fresh teammate") |

Specify intent stage (it asked through the prose skill only):

| # | Engine rule | Clause(s) gained |
|---|---|---|
| G21 | `lead-holds-conversation` | the lead runs intent inline; no teammate converses — the PM's capability frame reaches the user through the lead; transcription takes no sound loop |
| G22 | `engine-precedence` | the engine's asking rules hold over specify's intent text where they differ |
| G23 | `blind-second-list` | a blind list on an intent fork costly to undo (settled: verify round 2's W1 repair) |
| G24 | `fact-before-fork` | all five clauses |
| G25 | `inputs-in-hand` | the description and a named record open as facts, never re-asked; a skip never skips the confirm |
| G26 | `options-rule` | all three clauses, wherever options are put |
| G27 | `question-form` | plain words, names, one sentence + one example per option, the case against, a one-line pick, one screen (the open probe kept) |
| G28 | `mock-in-message` | all four clauses |
| G29 | `own-turn-or-batch` | all four clauses |
| G30 | `status-line` | a status line each turn |
| G31 | `ratified-streak` | the count at three and the next fork's form change (before: "flag ratification streaks" only) |
| G32 | `vague-answer` | both limbs (before: story stage only) |
| G33 | `stop-rule` | close only with nothing open; a user stop records what is open; the three-turn stall back to the opening |
| G34 | `provenance-marks` | intent's marks kept in the conversation — **ruled** (Q3, "In the chat only"); no edit |
| G35 | `user-rules` | silence never rules; the user may pull a default out |

Setup's interrogation:

| # | Engine rule | Clause(s) gained |
|---|---|---|
| G36 | `lead-holds-conversation` | no teammate converses; transcription takes no sound loop (the synthesis stays "your pen"; its cold stress-test still runs) |
| G37 | `engine-precedence` | the engine's asking rules hold over the agenda's and deck's text where they differ (what makes G50 bite) |
| G38 | `blind-second-list` | all clauses (V1) |
| G39 | `fact-before-fork` | no fork before its fact; source attached; user-only fact labelled; unfound fact carried open (the agenda's read-from-code-first lines keep "find what the repo answers") |
| G40 | `inputs-in-hand` | a skip never skips the close (the agenda keeps "covers only what the mode leaves unknown" and "skip what an answer has already settled") |
| G41 | `options-rule` | the rejected road named; for agenda forks also two real options with a cheaper one (deck cards already offer keep · tighten · drop · re-rank) |
| G42 | `question-form` | plain words, names, one sentence + one example per option, the case against, one screen (the recommendation already there) |
| G43 | `mock-in-message` | all four clauses |
| G44 | `own-turn-or-batch` | agenda items not worth a turn named as defaults and confirmed on one screen; a hanging default first. The deck stays card by card — **ruled** (Q2, "Keep card by card"): setup's own cadence, a slot fill already in `setup.interrogation-inline`'s text; no edit |
| G45 | `status-line` | a status line each turn |
| G46 | `ratified-streak` | all clauses |
| G47 | `vague-answer` | both limbs (the agenda's "probing deeper where answers are vague" kept) |
| G48 | `stop-rule` | all three clauses |
| G49 | `provenance-marks` | marks kept in the conversation (`governance-intent.md` has no place for them) |
| G50 | `user-rules` | the user may pull a default out; "silence never rules" meets dimension 7's active-decline offer (line 45) and dimension 8's default-on offer (line 40) — **ruled** (Q5, "Need my explicit yes"): adoption takes the user's yes; carrier engine `user-rules` plus P2's edit of the agenda; no P1 op |

### The questions — ruled by the user, 2026-10-11 (never re-asked)

1. Lost and gained rows — **ruled "Accept all"**: every lost and gained row shown accepted, none
   restored, none made a slot.
2. Setup's deck — **ruled "Keep card by card"**: its own cadence, a slot fill already in
   `setup.interrogation-inline`'s text; no edit.
3. Intent's provenance marks — **ruled "In the chat only"**: they stay in the conversation; no edit.
4. *(Struck, R4.)* Settled by verify round 2's W1 repair (record lines 1022–1023, 1207–1209,
   1222–1223): the *Slots declared where* default covers the story stage only; the intent stage
   inherits the costly-fork default (G23).
5. Default-on modules — **ruled "Need my explicit yes"**: adopting a default-on module takes the
   user's yes; carrier `analysis-iterative.user-rules` plus P2's edit of `INTERROGATION-AGENDA.md`
   lines 39–40 and 44–45; no P1 op.

**Not shown to the user before "Accept all":** L15, L16, the L10 and L11 widenings, and the
build-time word changes below. The lead puts them to the user or records its reading, as it did for
L13–L14 (R15).

**Hand-offs (P2's):** `INTERROGATION-AGENDA.md:166` ("Use the recommend-then-arbitrate format from
`analysis-iterative`" — a format template the skill dropped at v0.63.0; after W1 the engine's
`options-rule` + `question-form` carry a recommendation) and the Q5 lines 39–40 and 44–45 (R6, R16).

## Build-time changes — plan text → built text

`migrate validate --report` run 1 reported four similar-rule clusters and re-run 1 one; re-run 2
reported none. Each was cleared by a text fix inside the P1 write set, the obligation kept; no
allowlist row was added (plan rev2 §8: an engine rule against a surviving rule restating the same
mechanic is fixed in the text).

| Rule | Plan → built | Edge cleared | Map effect |
|---|---|---|---|
| `analysis-iterative.user-rules` | "the user may pull any default out to its own turn" → "the user may pull a default out to its own turn" | with `brainstorm.user-steers` | G17, G35, G50 read "a default"; the meaning is unchanged |
| `brainstorm.user-steers` (B8; the plan left it untouched) | reworded in 0060: the pull-a-default limb leaves it — engine `user-rules` carries it verbatim, so the caller restating it was a duplicate — and the text points there: "In any reply the user may edit the decision map, pick which askable question comes next, or stop the session — the thinking stage's other steering is in analysis-iterative.user-rules; moving anything out of scope is a frame change and takes the user's word." | with engine `user-rules`; then, once shortened, with `brainstorm.size-users-ruling`, `brainstorm.frame-changes-users-word` and `patterns-design-direction.replace-reserved` | B8 kept → reworded; no lost word — "pull a default out to its own turn" is verbatim in the engine, whose limb holds in every turn as "In any reply" did |
| `spec.intent-probe-discipline` (S3) | "a one-screen synthesis the user confirms." → "a one-screen synthesis the user confirms before any authoring." | with `review-governance-intent.no-in-session-confirmation` | the words restate `spec.intent-stage-first`'s order; no new obligation, no lost word |
| `spec.analyst-brief-and-pen` (S5) | "the requirements-analyst seat's brief" → "the next writer's brief" | with `spec.lead-holds-story-stage` | L10 widened; `spec.story-stage` names the requirements-analyst seat the next writer |
| `spec.stage-inputs-in-hand` (S7) | "In the story stage, a brainstorm record named in `$ARGUMENTS`, or stories already in story form, are the inputs in hand;" → "Specify's inputs in hand when its story stage opens: a brainstorm record named in `$ARGUMENTS`, or stories already in story form;" | with `spec.slate-binds-authoring` | no before-text clause dropped beyond the plan's moves to engine `inputs-in-hand` |
