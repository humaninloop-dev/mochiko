---
description: Think a problem through with the user and harden the record at the end — the session is just the lead and the user (plus a fact-checker teammate, seated from the start whenever the topic touches existing code, that maps the reality surface into the record and verifies claims against the files); at convergence the lead sizes the review at a named gate, under the weight card the user ruled at run start — a lens-split cold pair by default (independent reads, one four-message cross-examination, only survivors return for rulings), a single reviewer for lean records, or, on the user's recorded waiver alone, none. Deliverable is one decision record, plus a fidelity-checked synthesis on request after acceptance; pipeline entry is an offer, never a default. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one hardened decision record
behind. Empty topic → ask what we are thinking through.

**You are the lead**: you compose the run and own its counters, every verdict, every
escalation, every human gate, and the user-facing conversation — agents produce and review,
you adjudicate. Brief every dispatch per `templates/agent-dispatch.md`; at the sizing gate
Read `templates/sized-end-stage-review.md`. This file is self-contained: brainstorm's whole
contract lives here. You run the questioning inline via `mochiko:analysis-iterative` — one
question per turn, format adapted to the user's state. **First-spawn probe:** the
fact-checker at start where its seat fills, otherwise the reviewers at convergence.

## Goal

`.mochiko/brainstorms/<slug>/record.md` exists, each decision carrying statement + rationale +
confidence mark, and its Review section carrying the sizing ruling, **every survivor's
disposition**, and the verify outcome quoting the evidence the folds landed — or, in their
place, the recorded waiver; the tally is on the record even at zero survivors, which is
vacuously clean; the session's index entry names where the outcome landed; the KM close ritual
ran; and the user has accepted the record.

**Not done:** an unreviewed record with no recorded waiver · an undispositioned survivor · a
survivor dispositioned by a reviewer's status alone · folds with no verify outcome recorded · a
departure with no trail line · no user acceptance · a synthesis shipped without its fidelity
check.

## Seats & checks

| seat | agent × skill | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| fact-checker | a neutral empiricist, no skill mounted | produces the reality map, and settles the reviewers' fact disputes; reports what is, never argues what should be, volunteering file-grounded facts that cut either way; never grades the record | at start, conditional on the topic having a reality surface; **probe seat** when filled, its announcement naming that surface | you only, one send per fact; the reviewers' fact disputes reach it through you |
| reviewer(s) | `mochiko:devils-advocate` × `mochiko:review-brainstorm` in the **end-stage reviewer role**; a pair splits the hunt by lens — one **decision-quality**, one **record-integrity** | grades the frozen record; never authors it | cold at convergence only, never in the room before it; count per the sizing ruling | withheld from each other until findings are formed; one cross-exam |

**Validation model:** the sized end-stage review of `record.md`; there is no in-loop critique
seat. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset
→ stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first
spawn is the authoritative probe, and there is no teamless fallback. A seat is spawned with
**`name:`** — a nameless spawn is a one-shot subagent, the forbidden transport; every later
send is a `SendMessage` to that same named seat. Verify from the roster: the `members` array
in `~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the
session ID) must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an
agent team; failing again stops the run. Teammates don't load `skills:` frontmatter — every
spawn prompt names the skill and role itself. Tell the user up front they can watch or
message any teammate; announce each seat in one line when filled; never narrate or reply to
teammate housekeeping.

**Seat lifecycle:** the fact-checker meets the standing multi-unit criterion but counts no
loop unit — **cadence-exempt**, recycled only on the user's gate-time order. The reviewers
are cold end-stage seats, exempt by nature. A respawn is a reset: briefed from the on-disk
record alone, versioned successor name, never the dead seat's bare name. End-of-need
shutdown; no ritual sends.

## Constraints

- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  topic — **reversibility** (rework cost if the record is wrong) · **blast radius** (how much
  downstream work will read it as authoritative) · **precedent** (first-of-kind, or mirroring an
  audit-cleared pattern) · **input confidence** (scored on the artifact under review; a user
  ruling discounts ambiguity risk only, and one introducing new surface raises consistency
  risk) — plus the process you compose from it — the stated default below, or your departures
  from it · rules: the user · decides: the run's composed process. Rigor scales with
  cost-of-being-wrong, never task size.
- **Review sizing** *(at convergence)* — evidence: convergence signals — answers turning
  confirmatory, no new dimensions, the wrap confirmed with the user · rules: you, on your own
  weight statement (decision count · confidence-mark mix · reality-surface load), sizing under
  the user's weight card and never around it · decides: pair / single, a heavyweight record
  defaulting to the full pair and any size below that default costing one trail line. **None is
  not yours to take** — `record.md` is lead-penned, so shipping it uncold-read needs the user's
  recorded waiver at the weight card. A single reviewer gets the whole hunt surface and no
  cross-examination — its findings arrive undebated, the trade this gate priced.
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
- **Floor gates:** the weight card · survivor rulings · tie-break · acceptance — each reading
  `rules: the user`, none of them yours to compose away. **Review sizing is the one lead-ruled
  gate here**, so it is deliberately out of the set: it sizes under the weight card the user
  already ruled, never around it. Survivor rulings and tie-break fire only when their evidence
  exists; that bounds when they open, never who rules them. **`record.md` is lead-penned, so it
  always takes the cold grade** — the sized review and the verify pass over your folds are
  non-discretionary wherever a review runs, and it ships with zero cold reads only on the user's
  recorded waiver at the weight card, never on your sizing.
- **Bounds:** per reviewer one cold read, plus (pair only) the one-shot four-message
  cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor**,
  you count them; one fact-checker dispatch per fact. No kill-switch and no no-progress exit —
  the human-attended session is the escalation surface, not a substitute for the caps. Any
  bound this run declares — including a declared cost range — has you as its named counter,
  **rises only at a user checkpoint**, and is re-declared only on the record; busting a bound
  escalates, never silently continues.
- **Invariants:** **no standing challenger** — beyond these two seats the conversation is you
  and the user: the v2 standing episodic advocate generated 3:1 machine-to-user traffic and
  folded amendments into user-ruled decisions without consent. A **reality surface** is existing
  code, docs, or a system under redesign; that call at the start fills or leaves empty the
  fact-checker seat. The checker's map lands **verbatim**, a checker-authored section you write
  around and never restate — the first completed run's headline finding was an over-claim
  living in the lead's paraphrase of the map, not in the map. Both reviewer briefs name that
  map as the fact substrate: reviewers do **not** re-read the reality surface it already covers
  (the first completed run read it three times over — the pair's dominant cost); the lens split
  lives in those briefs, never in the skill, which stays one document. The **synthesis is
  on request only, after acceptance** — never auto-generated; write it beside the record
  stamped ***derived — record canonical***, and before it ships the reviewer that ran the
  verify pass — still seated — sample-checks its fidelity (every ruling present, no confidence
  mark inflated, no rejected alternative resurrected). Under a waiver it is stamped
  **"derived, unchecked"** instead — the same recorded-absence discipline as the waiver.
  Governance context is native: the CLAUDE.md governance region loads with the session; read
  `.mochiko/memory/governance-ledger.md` only when a decision needs waiver or amendment
  detail — never a blocking gate. `KEPT:` the no-fallback transport bet stays `Contested`.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a
  timed prompt. The record is written as the session progresses, never reconstructed at the
  end; it reads standalone as the review surface — review findings and dispositions live in
  its closing Review section, never interleaved — and your pen covers your own formulation
  only: nothing amends a user-ruled decision, and no new decision exists, without the user's
  word. Every departure from the stated default is one trail line — by record, never by
  silence — and rulings batch into the fewest checkpoints that respect the floor gates.

## Bindings

- **Artifacts:** `.mochiko/brainstorms/<slug>/record.md` in one decision namespace (D1…) — the
  deliverable, kept in place at acceptance; a conditional `synthesis.md` beside it. Derive the
  kebab-case `<slug>` at the start.
- **Uncertainty carrier:** the lead-penned record.
- **Fact route:** the fact-checker seat; an `Explore` subagent when it is unfilled, or for a
  one-off fetch with no standing-perspective value.
- **Verify-pass owner:** the record-integrity reviewer, or the sole reviewer in single mode.
- **Run-start declaration:** one line on `record.md`'s `Status` line — the surface Recovery
  already keeps — for a default run; a departing run, or one declaring non-default bounds,
  instantiates `templates/workflow-contract.md` as
  `.mochiko/brainstorms/<slug>/brainstorm-contract.md` beside the record. Counted unit: the
  lead↔reviewer **exchange per survivor**, the bound you count; the cold reads, the cross-exam
  and the verify pass are one-shot.
- **Departure trail:** one line per departure under that same declaration as it is taken,
  part of what the user accepts — a review sized below the default included
  (`sized-end-stage-review.md` defers that line here).
- **KM landing:** `.mochiko/brainstorms/index.md` is the session index — read it before
  opening, enter this session on open (status: open), and at acceptance or supersession update
  it with where the outcome landed (a `DECISIONS.md` row, or an explicit no-graduation). Run
  the open and close invariants — at close, the subtractive landing ritual — from the
  project-pinned `.mochiko/memory/knowledge-management.md` under fix-on-sight. No index and no
  module → skip; the layer was declined.

## Recovery

No resume table — the record is the whole state. Note resume state on its `Status` line, with
the run's counter state — exchanges consumed · bounds declared · departures taken. Sessions
and teams do not survive `/resume`; resume from the workspace, never a context `phase` field:
re-read `record.md` and continue from the last decision or the survivor queue, respawning the
fact-checker mid-session or the reviewers per the sizing ruling (the frozen record makes a
cold re-read cheap, and a respawn is cold by design).
