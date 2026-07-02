---
name: validation-brainstorm
description: This skill MUST be invoked to adversarially review the artifacts of a collaborative thinking session — EITHER challenging a frozen decision digest (`digest.md` — the session's decisions, rationales, confidence marks, and rejected alternatives) for unchallenged assumptions, missing dimensions, passively-accepted decisions, internal inconsistencies, and steelmanned rejected alternatives, emitting each surviving challenge as a live question; OR pressure-testing a destination-shaped synthesis/finding on three lenses — scenario stress ("what breaks this?"), reality-grounding (verifying its load-bearing factual claims against the actual codebase), and destination-readiness (grading it against the named checklist: feature-description / specification-equivalence / task-derivability / direct-execution). Emits a severity-classified report (Critical/Important/Minor) and a RECOMMENDED 3-state verdict (ready / needs-revision / critical-gaps) — the clearing verdict is lead-owned. SHOULD also invoke whenever a thinking loop's challenge or pressure-test step needs an independent grade of the session's artifacts, or when re-reviewing after a revision. The caller names the branch and, for a pressure test, the destination. Boundary: does NOT review a specification authored as a spec through its own authoring loop (that is mochiko:analysis-specifications — this skill certifies a session conclusion's fitness to ENTER a next stage of work, not a full product-gap review); does NOT grade plan or task artifacts (mochiko:validation-plan-artifacts / mochiko:validation-task-artifacts). Defaults to FAIL; run by an independent reviewer, never a co-author of the session under review.
---

# Reviewing Brainstorm Artifacts

## Overview

A collaborative thinking session produces two reviewable artifacts at two moments: a **decision
digest** frozen when the conversation converges, and a **destination-shaped synthesis** drafted
once the conclusions have a consumer. Both are co-authored inside the session's own momentum —
which is exactly why they need a reviewer who was not in the room. This skill is that review, in
two branches:

- **Branch 1 — digest challenge**: adversarial critique (judgment, not a checklist) of the frozen
  decision set, *while the thinking is still fluid*. Its findings are returned as **live
  questions** the session can still answer cheaply.
- **Branch 2 — finding pressure-test**: hybrid critique + checklist over the drafted synthesis —
  does the conclusion survive contact with reality, and is it fit for the destination it claims?

The output of either branch is a **report plus a RECOMMENDED 3-state verdict** (ready /
needs-revision / critical-gaps) — an *input* to the loop, **not** a clearing PASS/FAIL. The lead
owns the clearing decision and routes any revision; this skill recommends, the lead routes.

**Violating the letter of the rules is violating the spirit of the rules.** Softening a challenge
because the session's participants "clearly thought about it," or ticking a readiness item off a
skim, is the exact captured-by-the-room failure this review exists to correct.

## Scope — session artifacts, and the boundaries

| Lens | Artifact | Owner |
|------|----------|-------|
| **Digest challenge / finding pressure-test** | `digest.md`, the destination-shaped synthesis | **this skill** |
| **Product-gap review of a drafted spec** | a `spec.md` in its own authoring loop | `mochiko:analysis-specifications` |
| **Plan-artifact completeness / feasibility** | requirements, NFRs, data-model, contracts | `mochiko:validation-plan-artifacts` / `mochiko:validation-feasibility` |
| **Task-artifact quality** | `task-mapping.md`, `tasks.md` | `mochiko:validation-task-artifacts` |

The nearest boundary is `analysis-specifications`: where that skill hunts product gaps in a spec
*being authored as a spec*, this skill's specification-equivalence check certifies a session
conclusion's **fitness to stand in** for one — shape conformance plus load-bearing substance.
Destination-readiness is an *entry* certificate, never a substitute for the destination's own
review loop, which still grades everything downstream.

## When to Use

- Grading a frozen `digest.md` at a session's convergence point — before conclusions harden
- Pressure-testing a destination-shaped synthesis before it is handed to its consumer
- Re-reviewing either artifact after a revision driven by a prior round's findings

## When NOT to Use

- **Mid-conversation** — there is nothing frozen to grade; wait for the digest
- **Reviewing a spec inside its own authoring loop** — use `mochiko:analysis-specifications`
- **Grading plan or task artifacts** — use their reviewers (see *Scope*)
- **Authoring or fixing the digest or synthesis** — you surface findings and hand them back;
  revision belongs to the session, never to its reviewer

## Branch 1 — Digest challenge

Read `digest.md` itself — never a summary of the session, never a participant's account. The
digest must carry each decision with its rationale, its confidence mark, and the alternatives
rejected. **A digest too thin to challenge is itself the first finding** (Important): a decision
whose rationale cannot be attacked cannot be trusted either.

Hunt each class in turn; do not stop at the first clean one:

| # | Class | The question |
|---|-------|--------------|
| 1 | **Unchallenged assumption** | What does this decision silently presume that nobody tested? |
| 2 | **Missing dimension** | What angle (cost, failure mode, actor, timescale) did the session never visit? |
| 3 | **Passive acceptance** | Which decision was adopted on "sounds good" — thin rationale, no pushback recorded? |
| 4 | **Rejected-road steelman** | Take the strongest discarded alternative and argue it seriously — does the chosen road still win? |
| 5 | **Internal inconsistency** | Do two decisions, each fine alone, undercut each other? |

For every finding, record the evidence (quote the digest), the severity, and — this is the branch's
distinctive output — the **live question**: the single question the session's human should answer
to resolve it. A challenge without an answerable question is commentary, not a finding. A steelman
the human overrules is *resolved* — recommend the digest mark that decision `Contested`, and do not
re-raise it in a later round.

## Branch 2 — Finding pressure-test

The caller names the **destination** the synthesis claims to be shaped for. Read the synthesis
itself, then work all three lenses — a finding on any lens blocks `ready`:

1. **Scenario stress.** Attack the substance: what input, actor, failure, scale, or sequence breaks
   the conclusion? Probe the edges the way the digest's decisions said the world works — then probe
   the ways it didn't say.
2. **Reality-grounding.** List the synthesis's **load-bearing factual claims** about existing code
   and systems ("X already handles retries", "the schema has a status column") and verify each one
   directly against the codebase — Read/search the actual files and cite them. A claim you could
   not verify is a finding, not a benefit of the doubt; "the session said so" is not evidence.
3. **Destination-readiness.** Run the named checklist from
   [references/READINESS-CHECKLISTS.md](references/READINESS-CHECKLISTS.md), citing evidence from
   the synthesis for every item. If the work demands more than the destination provides (its
   depth-downgrade rule trips), report **wrong depth** and recommend the shallower destination —
   that is a scope-type finding for the lead to route, not a reason to stretch the checklist.

## Report shape and verdict

Both branches write the report in the shared `mochiko:advocate-report-template` shape — findings
as the severity-classified gap table, live questions as the Clarifications section (each with
concrete options where they exist), the recommended verdict with rationale, and genuine strengths.

| Verdict | Branch 1 criteria | Branch 2 criteria |
|---------|-------------------|-------------------|
| **ready** | zero Critical, zero Important challenges outstanding | all three lenses hunted clean **and** every checklist item evidenced |
| **needs-revision** | 1–3 Important, all answerable in one round of live questions | resolvable findings, right destination |
| **critical-gaps** | 1+ Critical, or the digest is too thin to review | a broken load-bearing claim, or wrong depth |

**Never default to `ready`.** It is earned by a completed hunt — every class (branch 1) or every
lens plus every checklist item (branch 2) actively worked with nothing surfacing — never by the
artifact looking reasonable or the session having been thorough.

## Independence (stated by role)

- You review a session you were **not part of**. The digest and synthesis are co-authored by the
  session's participants; your value is precisely that their momentum has no hold on you. You never
  author, revise, or complete the artifacts you grade.
- Your verdict is **input**, not the gate. The lead reads the artifacts and your report and owns
  the clearing verdict, the revision routing, and the human gate — see `loop-discipline`; this
  skill does not restate or own them.
- Live questions are gate fuel: preference-shaped challenges belong to the human; a factual unknown
  belongs to investigation. Route by kind per `loop-discipline`'s gap routing — never send a
  "should we" to research or a "does the code do X" to the human.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Challenging the transcript instead of the digest | The digest is the frozen artifact. If it under-records the session, that thinness is the finding. |
| Findings without live questions (branch 1) | Every challenge carries the one question that resolves it. Unanswerable commentary is noise. |
| Re-raising an overruled steelman | A `Contested` mark means the human decided with eyes open. Once is a challenge; twice is nagging. |
| Trusting the session's claims about the code | Reality-grounding means Reading the actual files and citing them. Unverified = finding. |
| Stretching a checklist to avoid a depth downgrade | Wrong depth is a scope finding with a shallower-destination recommendation — say it plainly. |
| Full product-gap review under the equivalence check | Certify fitness to enter the next stage; the destination's own reviewers still grade the work. |
| Grading your own session | If you were in the room, you are a co-author. A different reviewer runs this skill. |

## Red Flags — STOP and re-hunt

- "The session clearly discussed this thoroughly." *(the captured-by-the-room tell)*
- "Nothing jumped out, so it's ready."
- "The claim about the codebase is probably right."
- "The steelman would just annoy them — the choice is made."
- "It's slightly past the depth rule, but close enough." *(the checklist-stretching tell)*
- "I'll skip the lenses and just run the checklist."

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Challenging settled decisions wastes the session's time" | One live question now is the cheapest moment this gap will ever have. That is the branch's whole purpose. |
| "The participants know their codebase" | Sessions routinely build on stale beliefs about existing code. Verify against the files, cite the evidence. |
| "A checklist pass is enough for ready" | Branch 2 is three lenses. A finding-free checklist over a conclusion that breaks under scenario stress is a failed review. |
| "Recommending a shallower destination feels like blocking" | Wrong depth shipped anyway is the expensive version of the same news. |
| "Zero findings means it was a good session" | Zero findings means go back and hunt harder — some challenge, strength, or unverified claim is always worth surfacing. |

## Related

- `mochiko:advocate-report-template` — the shared report shape both branches fill
- `mochiko:analysis-specifications` — the nearest boundary: product-gap review of a spec in its own authoring loop
- `mochiko:validation-plan-artifacts` / `mochiko:validation-task-artifacts` / `mochiko:validation-feasibility` — the downstream reviewers a readiness certificate never replaces
- `loop-discipline` — the loop, bounds, gap routing, and human gate are the lead's; referenced, never restated
- [references/READINESS-CHECKLISTS.md](references/READINESS-CHECKLISTS.md) — the four destination checklists + depth-downgrade rules (the single source for what each stamp means)
