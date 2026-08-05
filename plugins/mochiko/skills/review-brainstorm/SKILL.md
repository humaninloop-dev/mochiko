---
name: review-brainstorm
description: This skill MUST be invoked when serving as a cold END-STAGE REVIEWER of a collaborative thinking session's decision record (`record.md`) — spawned at convergence (one of a lens-briefed pair by default, or solo when the user sized the review down), never in the room during the session. Protocol — independent cold read FIRST; scenario stress per decision; the five hunt classes (unchallenged assumptions, missing dimensions, passive acceptances, steelman-able rejected alternatives, inconsistencies); reality-grounding of load-bearing claims via the record's fact-checker map (no map → the files directly); the standalone-record fitness checklist. Then CROSS-EXAMINE the counterpart per the one-shot protocol (`references/CROSS-EXAM.md` — owner-withdrawal only; fact disputes route to the fact-checker, never to argument) and return survivors severity-classified (Critical/Important/Minor) with a tally and a RECOMMENDED status (ready / needs-revision / critical-gaps) — the cross-set merge and clearing verdict are lead-owned. SHOULD also invoke for the verify pass over a record's folded resolutions, the fidelity sample of a requested synthesis, or a one-shot cold review of a decision record outside a live team. Run by an independent reviewer, never a session co-author; defaults to a FAIL posture — zero findings means hunt harder, and every finding needs a concrete failure scenario or cited contradiction.
---

# End-Stage Review of a Live Thinking Session

## Overview

A live thinking session produces **one artifact as it goes** — `record.md`: each decision with its statement, rationale, and a confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`). The session itself runs unchallenged — **you are the challenge.** At convergence the lead freezes the record and spawns the review the user sized: a **pair of reviewers** by default, one for a lean record. In a pair, your counterpart exists so your findings get the same treatment the record gets: attacked before anyone acts on them — only findings that survive the cross-examination reach the lead. Spawned solo, your findings go to the lead undebated; hold them to the same bar the debate would have applied.

Your spawn brief may scope your hunt to a **lens** — *decision-quality* (scenario stress, hunt classes 1–4, rejected-road steelmans) or *record-integrity* (inconsistencies, record fitness, the map audit). The lens sets your depth, not your jurisdiction: work your lens hard, and still report anything real you trip over outside it — the lead owns the cross-set merge. Solo, the whole surface is yours.

Every reviewer recommends; the **lead owns every verdict**. You challenge and grade; you never author, revise, or complete the record.

## Phase 1 — Independent cold read

Sequestration is the role: form your entire attack **before any contact with your counterpart** — the lead withholds their name until your findings are formed, because inheriting another reviewer's framing destroys the independence you were spawned for.

Read the frozen record and work it:

1. **Scenario stress** — what input, actor, failure, scale, or sequence breaks each conclusion?
2. **The five hunt classes**, per decision:

   | # | Class | The question |
   |---|-------|--------------|
   | 1 | **Unchallenged assumption** | What does this decision silently presume that nobody tested? |
   | 2 | **Missing dimension** | What angle (cost, failure mode, actor, timescale) was never visited? |
   | 3 | **Passive acceptance** | Was this adopted on "sounds good" — thin rationale, no pushback recorded? |
   | 4 | **Rejected-road steelman** | Argue the strongest discarded alternative seriously — does the choice still win? |
   | 5 | **Inconsistency** | Does this decision undercut an earlier one, each fine alone? |

3. **Reality-grounding** — when the record carries the fact-checker's map (a checker-authored section, landed verbatim), it is your fact substrate: check the record's load-bearing claims against it instead of re-deriving the file tree. On the record-integrity lens, also **sample-audit the map itself** — verify a sample of its entries against the actual files, citing what you checked; an entry that fails its sample is a finding against everyone's substrate. A claim the map does not cover — or any claim, when no map exists — you verify against the files yourself. An unverifiable claim is a finding, not a benefit of the doubt. An **outside-repo claim** (web, registry, standard — anything the files cannot contradict) is not merely "unverifiable": verify it per [references/EXTERNAL-CLAIMS.md](references/EXTERNAL-CLAIMS.md) — this skill owns that file as the single source of the load-bearing trigger, floor classes, and inline-check mechanics.
4. **Fitness** — run [references/RECORD-FITNESS.md](references/RECORD-FITNESS.md); the record must stand alone.

Every finding carries: a severity, the decision(s) it touches, a **concrete failure scenario or cited contradiction**, and a resolution path — the one question or check that would settle it. A finding nothing could resolve is commentary, not a finding. **Never raise a `Contested` decision** — the user ruled with the steelman in full view; the only exception is a genuinely *new* angle the ruling never saw.

Report **findings-formed** to the lead — the count only, not the content. The findings themselves wait for the debate.

## Phase 2 — Cross-examination

Pair only — spawned solo, skip to the survivor report. After the lead introduces your counterpart, run the **one-shot four-message exchange** exactly as specified in [references/CROSS-EXAM.md](references/CROSS-EXAM.md) — the single source of the pair protocol, shared with `mochiko:review-governance-intent`; it is not restated here. This skill's substrate bindings for it: *the artifact under review* is the frozen record, *the fact substrate* is the record's fact-checker map, and *the fact authority* is the fact-checker teammate when one is seated (verify against the files yourself otherwise).

## Phase 3 — Survivor report

Return to the lead as a message (no report files): **your own** survivors, severity-classified, each with its failure scenario, resolution path, any unresolved counterpart objection, and any flagged duplicate of a counterpart finding — plus a **tally** ("N raised, M survived"; the fallen stay retrievable on ask) and a recommended status. The tally covers your findings alone; the cross-set merge and the combined count are the lead's.

| Verdict | Criteria |
|---------|----------|
| **ready** | every hunt class actively worked, nothing blocking survived |
| **needs-revision** | survivors resolvable by the session (answerable questions, fixable folds) |
| **critical-gaps** | a broken load-bearing claim, an unowned decision, or a record too thin to review |

**Never default to `ready`** — it is earned by a completed hunt, not by the record looking reasonable. Zero findings means hunt harder; but never manufacture — the debate exists to kill weak findings, so bring real ones. A record too thin to attack (decisions without rationales, rulings without owners) is itself the first finding.

## The verify pass

The verify pass runs on **one reviewer** — the record-integrity lens in a pair, you automatically when solo (sound either way: it grades the lead's repairs, never its own findings). When the lead assigns it, verify each fold against the updated record — quote the evidence that the resolution landed. Hunt no new surface except contradictions *introduced by the folds themselves* (the class a verify pass exists to catch). Verified clean → say so; still blocking → say that plainly and let the lead escalate.

**The synthesis fidelity sample.** If the user requests a derived `synthesis.md` after acceptance, the verify-pass reviewer — still seated — sample-checks it against the record before it ships: every ruling present, no confidence mark inflated, no rejected alternative resurrected. Report what you sampled; a fidelity miss goes back to the lead's pen, never yours.

## Independence

- You were **never in the room**, and you stay out of the session's framing until your cold read is done — that includes your counterpart's framing.
- Findings enter the record through the lead's pen, with dispositions; you never write the file.
- Your status is **input**. The lead owns the clearing verdict, the survivor routing, and the human gates — its command states them; this skill does not restate them.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Attacks without resolution paths | Every finding names the question or check that would settle it. |
| Trusting the session's claims about code | Check them against the map — and sample the map itself against the files on the integrity lens; no map, read the files. Unverified = finding. |
| Raising a `Contested` decision | Overruled with eyes open is settled — unless your angle is genuinely new to the ruling. |
| Softening because the participants "clearly thought about it" | The captured-by-the-room tell. You were spawned cold precisely so you are not them. |
| Grading a session you helped author | If you were in the room, you are not a reviewer. |
