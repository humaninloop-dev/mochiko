---
description: Think a problem through with the user and harden the record at the end — the session is just the lead and the user (plus a fact-checker teammate, seated from the start whenever the topic touches existing code, that maps the reality surface into the record and verifies claims against the files); at convergence the user sizes the review at a named gate — a lens-split cold pair by default (independent reads, one four-message cross-examination, only survivors return for rulings), a single reviewer for lean records, or a recorded waiver. Deliverable is one decision record, plus a fidelity-checked synthesis on request after acceptance; pipeline entry is an offer, never a default. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one hardened decision record
behind. Empty topic → ask what we are thinking through.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and
`mochiko:loop-discipline` before anything else; brief every dispatch per
`templates/agent-dispatch.md`; at the sizing gate Read
`templates/sized-end-stage-review.md`. This file carries only brainstorm's parameters. You
run the questioning inline via `mochiko:analysis-iterative` — one question per turn, format
adapted to the user's state. **First-spawn probe:** the fact-checker at start where its seat
fills, otherwise the reviewers at convergence.

## Goal

`.mochiko/brainstorms/<slug>/record.md` exists, each decision carrying statement + rationale +
confidence mark; the sized review ran per the user's ruling with **every survivor
dispositioned** and the verify
pass confirming the folds landed — or the ruling was **none** and the waiver is recorded; the
session's index entry names where the outcome landed; the KM close ritual ran; and the user
has accepted the record. Zero survivors is vacuously clean — the tally is still reported.

**Not done:** an unreviewed record with no recorded waiver · an undispositioned survivor · an
unrun verify pass · a reviewer's status taken as the disposition without your read · no user
acceptance · a synthesis shipped without its fidelity check.

## Seats & checks

| seat | agent × skill | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| fact-checker | a neutral empiricist, no skill mounted | produces the reality map, and settles the reviewers' fact disputes; reports what is, never argues what should be, volunteering file-grounded facts that cut either way; never grades the record | at start, conditional on the topic having a reality surface; **probe seat** when filled, its announcement naming that surface | you only, one send per fact; the reviewers' fact disputes reach it through you |
| reviewer(s) | `mochiko:devils-advocate` × `mochiko:review-brainstorm` in the **end-stage reviewer role**; a pair splits the hunt by lens — one **decision-quality**, one **record-integrity** | grades the frozen record; never authors it | cold at convergence only, never in the room before it; count per the sizing ruling | withheld from each other until findings are formed; one cross-exam |

**Validation model:** the sized end-stage review of `record.md`; there is no in-loop critique
seat.

**Seat lifecycle:** the fact-checker meets the governed criterion but counts no loop unit —
**cadence-exempt**, recycled only on the user's gate-time order.

## Constraints

- **Review sizing** *(at convergence)* — evidence: convergence signals — answers turning
  confirmatory, no new dimensions, the wrap confirmed with the user · rules: the user, on your
  weight statement (decision count · confidence-mark mix · reality-surface load) · decides:
  pair / single / none, a heavyweight record defaulting to the full pair; **none** records a
  waiver. A single reviewer gets the whole hunt surface and no cross-examination — its
  findings arrive undebated, the trade this gate priced.
- **Survivor rulings** — evidence: a survivor in user territory — a challenge to a user
  ruling, or a user-declared fact offered as confirmation · rules: the user · decides: its
  disposition. Theirs to answer, not a tie-break.
- **Tie-break** — evidence: a lead↔reviewer argument unresolved at the two-exchange cap ·
  rules: the user, on both positions plus your recommendation · decides: the disposition, and
  whether the element marks `Contested`.
- **Acceptance** — evidence: every survivor dispositioned and the verify pass recorded, or the
  waiver · rules: the user · decides: done. Then offer, don't push: if the record is honestly
  the shape of a next stage (e.g. a feature description for `/mochiko:specify`), name it as an
  option and stop.
- **Bounds:** per reviewer one cold read, plus (pair only) the one-shot four-message
  cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor**,
  you count them; one fact-checker dispatch per fact. No kill-switch and no no-progress exit —
  the human-attended session is the escalation surface, not a substitute for the caps.
- **Invariants:** **no standing challenger** — beyond these two seats the conversation is you
  and the user: the v2 standing episodic advocate generated 3:1 machine-to-user traffic and
  folded amendments into user-ruled decisions without consent
  (`.mochiko/brainstorms/brainstorm-v2-revision/record.md`). A **reality surface** is existing
  code, docs, or a system under redesign; that call at the start fills or leaves empty the
  fact-checker seat. The checker's map lands **verbatim**, a checker-authored section you write
  around and never restate — the first completed run's headline finding was an over-claim
  living in the lead's paraphrase of the map, not in the map. Both reviewer briefs name that
  map as the fact substrate: reviewers do **not** re-read the reality surface it already covers
  (the first completed run read it three times over — the pair's dominant cost); the lens split
  lives in those briefs, never in the skill, which stays one document. (Both rules'
  evidence: `.mochiko/brainstorms/brainstorm-v2-2-revision/record.md`.) The **synthesis is
  on request only, after acceptance** — never auto-generated; write it beside the record
  stamped ***derived — record canonical***, and before it ships the reviewer that ran the
  verify pass — still seated — sample-checks its fidelity (every ruling present, no confidence
  mark inflated, no rejected alternative resurrected). Under a waiver it is stamped
  **"derived, unchecked"** instead — the same recorded-absence discipline as the waiver.
  Governance context is native: the CLAUDE.md governance region loads with the session; read
  `.mochiko/memory/governance-ledger.md` only when a decision needs waiver or amendment
  detail — never a blocking gate. `KEPT:` the no-fallback transport bet stays `Contested`, its
  provenance this command's own v2 design record
  (`.mochiko/brainstorms/brainstorm-command-rewrite/record.md`, D9).

## Bindings

- **Artifacts:** `.mochiko/brainstorms/<slug>/record.md` in one decision namespace (D1…) — the
  deliverable, kept in place at acceptance; a conditional `synthesis.md` beside it. Derive the
  kebab-case `<slug>` at the start.
- **Uncertainty carrier:** the lead-penned record.
- **Fact route:** the fact-checker seat; an `Explore` subagent when it is unfilled, or for a
  one-off fetch with no standing-perspective value.
- **Verify-pass owner:** the record-integrity reviewer, or the sole reviewer in single mode.
- **KM landing:** `.mochiko/brainstorms/index.md` is the session index — read it before
  opening, enter this session on open (status: open), and at acceptance or supersession update
  it with where the outcome landed (a `DECISIONS.md` row, or an explicit no-graduation). Run
  the open and close invariants — at close, the subtractive landing ritual — from the
  project-pinned `.mochiko/memory/knowledge-management.md` under fix-on-sight. No index and no
  module → skip; the layer was declined.

## Recovery

No resume table — the record is the whole state. Note resume state on its `Status` line; to
resume, re-read `record.md` and continue from the last decision or the survivor queue,
respawning the fact-checker mid-session or the reviewers per the sizing ruling (the frozen
record makes a cold re-read cheap).
