---
description: Think a problem through with the user and leave one hardened, cold-reviewed decision record behind.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one hardened decision record
behind. Empty topic → ask what we are thinking through.

## Goal

`.mochiko/brainstorms/<slug>/record.md` exists, each decision carrying statement + rationale +
confidence mark (`Confident` / `Assumed` / `Contested` / `Unsure` / `Deferred`); the record was
cold-reviewed and every surviving finding dispositioned — or the user's waiver of the review is
recorded on it; the session's entry in `.mochiko/brainstorms/index.md` is updated with where the
outcome landed; and the user accepted the record.

**Not done — default FAIL:** an unaccepted record · an unreviewed record with no recorded
waiver · an undispositioned review survivor · an index entry missing or contradicting the
record's status.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; run the questioning
  yourself, inline, via `mochiko:analysis-iterative` — one question per turn, format adapted to
  the user's state. Teammates or subagents per seat is your call.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author — the record is yours, so its review
  seat is always someone else, reading the frozen record cold from the file, default FAIL.
- **Blind-map dispatch:** a review seat is spawned in two messages — first the topic statement
  and goal line only, *never* the record path, so it builds its Phase 0 angle map with no
  sight of what the session decided; its map returns before you send the record path and the
  cold read begins. The anchoring fence is structural, not a trust ask. In a pair, both seats
  build their maps independently.
- **Coverage-survivor routing:** a surviving coverage finding is a candidate that questions the
  topic itself, not a fold — present each gap as a candidate topic; **the user** rules the path: **explore now**
  (re-enter `mochiko:analysis-iterative` on that angle; the resulting decision lands in the
  record's same `D…` namespace), **rule inline**, or **defer**. Non-coverage survivors keep the
  ordinary fold / repair / ruling path and may be dispositioned in batches.
- **Reopen-born verify:** a decision born from a coverage-survivor reopen gets one bounded verify
  round — internal consistency and record-fitness, no fresh cold read, no blind-map coverage
  hunt against it, and no second reopen off it.
- **Reserved to the user:** record acceptance · the disposition of any review survivor that
  challenges a user ruling · the waiver, if the review is to be skipped · any amendment to a
  user-ruled decision, and any new decision — their word, never yours.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/brainstorms/<slug>/record.md` — kebab-case `<slug>` derived at the
  start, decisions in one `D1…` namespace, written as the session progresses, never
  reconstructed at the end.
- **Index:** `.mochiko/brainstorms/index.md` — read before opening; enter the session on open
  (status: open); update at acceptance or supersession with where the outcome landed. Where
  `.mochiko/memory/knowledge-management.md` exists, run its close ritual.
- **Synthesis:** on request only, after acceptance — beside the record, stamped
  *derived — record canonical*; under a review waiver, stamped *derived, unchecked*.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** pipeline entry (e.g. `/mochiko:specify` when the record is honestly a
  feature description) is an offer after acceptance, never a default.
