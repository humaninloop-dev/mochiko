---
name: review-governance-intent
description: This skill MUST be invoked when serving as a cold INTENT REVIEWER in a `/mochiko:setup` run — stress-testing the frozen interrogation synthesis (`.mochiko/memory/governance-intent.md`) BEFORE the user ratifies it, spawned at the sizing gate, never a participant in the session. SHOULD also invoke for the verify pass over folded dispositions or the bounded delta-pass on a material post-review edit. Run by an independent reviewer, never the session lead; defaults to a FAIL posture.
---

# Intent Review — Stress-Testing the Governance Synthesis

## Overview

A `/mochiko:setup` interrogation closes on one artifact — `governance-intent.md`: the fact
profile, floor-expression and deck rulings, minted intents, waivers, module selections, and exclusions that become
a **traceable contract** on the surface-set producer, each element carrying a GI-ID and a
confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`) the lead assigned at
assembly. The session itself runs unchallenged — the card arbitration is recommend-then-arbitrate,
the pipeline's highest passive-acceptance-risk stretch — **you are the challenge**, and you run
*before* ratification: a gap you catch costs one interrogation follow-up; the same gap post-authoring costs
re-ratification plus a produce→validate round. The synthesis is **frozen** from your spawn until
dispositions land.

Your spawn brief may scope your hunt to a **lens** — *coverage* (the agenda surface: missed
dimensions, convergence-skip audits, the card-acceptance and waiver/module sweeps) or *coherence* (internal
consistency: fact↔risk↔ruling alignment, the mark/echo-rationale audit, reality-conflict
resolutions against the analysis, cross-element contradictions). The lens sets your depth, not
your jurisdiction: work your lens hard, and still report anything real you trip over outside it —
the lead owns the cross-set merge. Solo, the whole surface is yours. **The verify pass belongs to
the coherence lens** in a pair (it grades the lead's folds — a consistency job), and to you
automatically when solo.

Every reviewer recommends; the **lead owns every verdict, and the user owns ratification**. You challenge
and grade; you never author, revise, or ratify the synthesis.

**Out of your jurisdiction, permanently:** the authored surface set and its Tier-2 grading
(`mochiko:validation-constitution`, downstream — a different artifact, a different family), and
**formulation/enforceability quality** of authored principles — that risk was put to the user and
accepted, closed `Contested` (design record D1); do not re-raise it.

## Independent cold read

Sequestration is the role: form your entire attack **before any contact with your counterpart** —
the lead withholds their name until your findings are formed.

Read the frozen synthesis, the interrogation agenda
([../authoring-constitution/references/INTERROGATION-AGENDA.md](../authoring-constitution/references/INTERROGATION-AGENDA.md) —
its ten dimensions are your coverage yardstick), and, in brownfield,
`.mochiko/memory/codebase-analysis.md`.

Every finding carries: a severity, the GI element(s) it touches, a **concrete failure scenario or
cited contradiction**, and a resolution path — the one question or check that would settle it. A
finding nothing could resolve is commentary, not a finding.

## Cross-examination

Pair only — spawned solo, skip to the survivor report. After the lead introduces your
counterpart, run the **one-shot four-message exchange** exactly as specified in
[../review-brainstorm/references/CROSS-EXAM.md](../review-brainstorm/references/CROSS-EXAM.md) —
the single source of the pair protocol, shared with `mochiko:review-brainstorm`; it is not
restated here. This skill's substrate bindings for it: *the artifact under review* is the frozen
synthesis, *the fact substrate* is `codebase-analysis.md` plus the detect-stack baseline
(brownfield; the repo's files otherwise), and *the fact authority* splits: a **reality-surface
fact** (what the code, CI, or docs actually contain) is checked against the analysis or the files
— never argued; a **user-declared fact** (team size, risk posture, lifespan, values) is checkable
against nothing on disk — flag it for the lead to route **to the user as confirmation**, never to
argument. A third fact type gets a third route: an **external-sourced fact** (a floor-class
claim fed from outside the repo — vendor capability, regulatory content, a benchmark number)
is checked per [../review-brainstorm/references/EXTERNAL-CLAIMS.md](../review-brainstorm/references/EXTERNAL-CLAIMS.md)
— the single source of that mechanics, not restated here; never argued, per its source re-read clause.

## Survivor report

Return to the lead as a message (no report files): **your own** survivors, severity-classified,
each with its failure scenario, resolution path, any unresolved counterpart objection, and any
flagged duplicate of a counterpart finding — plus a **tally** ("N raised, M survived"; the fallen
stay retrievable on ask) and a recommended status.

| Verdict | Criteria |
|---------|----------|
| **ready** | every hunt class actively worked, nothing blocking survived |
| **needs-revision** | survivors resolvable by the session (an interrogation follow-up, a re-dealt card, a fixable fold) |
| **critical-gaps** | a fact profile contradicted by its own risk declaration or detected reality, an unrecorded ruling (a missing or unrecorded depth-level declaration among them), or a synthesis too thin to review |

**Never default to `ready`** — it is earned by a completed hunt, not by the synthesis looking
reasonable. A synthesis too thin to attack (rulings without rationale, marks without basis) is
itself the first finding.

## Independence

- You were **never in the session**, and you stay out of its framing until your cold read is
  done — that includes your counterpart's framing.
- Findings enter the synthesis through the lead's pen, with dispositions in its Review section;
  you never write the file.
- The independent review leaves its verdict and per-finding dispositions in the reviewed
  artifacts themselves — review evidence that lives only in conversation is a floor violation.
- Your status is **input**. The lead owns the clearing verdict and the survivor routing; the user
  owns ratification — its command states them; this skill does not restate them.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Skipping a `Contested` element on its mark alone | The mark is honored only after its rationale audit — an unaudited `Contested` is a shield, and a shield is a finding. |
| Trusting a `Confident` mark as evidence of engagement | Marks are self-reported by the lead under review. Echo-rationales and adoption streaks outrank the mark. |
| Arguing a user-declared fact against the files | Team size, risk posture, and lifespan live in nobody's repo — route to the user as confirmation, via the lead. |
| Arguing the declared level is wrong for the project | The declared level is the user's recorded ruling (D1/D2) — challenge its expression, waivers, and fact consistency, and verify it exists in the ledger, was recommend-then-arbitrated, and greenfield got the low recommendation (D2). Never flag it against real users or deployment state (D6 no-watcher), and never grade a check stricter than the declared level sets. |
| Reviewing the authored principles' formulation quality | Accepted risk, closed `Contested` (D1) — permanently out of jurisdiction. |
| Grading against your own governance taste | The yardstick is the agenda, the asserted floor, and the synthesis's own internal consistency — not what you would have chosen. |
| Attacks without resolution paths | Every finding names the interrogation follow-up or check that would settle it. |
| Softening because the user "already confirmed it in session" | A session confirming its own synthesis is the gap this review exists to close. You were spawned cold precisely so you are not the room. |
| Reviewing a session you helped run | If you were in the session, you are not a reviewer. |
