---
name: loop-discipline
description: This skill MUST be invoked when designing, authoring, or reviewing any mochiko workflow, command supervisor, or agent loop — any time a loop's done-condition, validation gate, iteration bound, or human gate is being decided or checked, or a workflow-contract is being filled or audited. SHOULD also invoke whenever a primitive iterates (produce → check → repeat), dispatches a validator, or claims an output is "done". The doctrine skill behind every mochiko loop.
---

# Loop Discipline

## Overview

Every mochiko workflow is a **constrained loop**, never freeform generation. Mochiko has no kernel to enforce this — the discipline is carried by this skill (the rules) plus a [`workflow-contract`](../../templates/workflow-contract.md) that each workflow fills in. A loop that does not satisfy all four requirements below is not a mochiko workflow; it is unbounded generation wearing a loop costume.

**Violating the letter of these rules is violating the spirit of them.** "The loop basically validates" or "the lead will stop when it's clearly done" is not loop discipline — it is the exact failure this skill exists to prevent.

```
A LOOP IS ONLY SOUND WHEN:
  its done-condition was written BEFORE the loop ran and DEFAULTS TO FAIL,
  its validator is a DIFFERENT agent running a DIFFERENT skill than the producer,
  its iteration is BOUNDED by a deterministic cap, not by the model feeling done,
  and a HUMAN GATE is named in its contract.
You cannot rationalize your way out of any of the four.
```

## When to Use

- Authoring or reviewing any `commands/*.md` supervisor or agent-team workflow
- Deciding whether a produce → check → repeat pattern is safe to ship
- Filling in or auditing a `workflow-contract`
- Whenever an agent is about to grade its own output, or a loop has no visible stop condition
- Whenever a primitive claims "done" — check it against a pre-declared condition, not a vibe

## When NOT to Use

- For a single, non-iterative skill invocation that emits no reviewable artifact (no loop, no contract needed — but it still follows the library conventions: classification, router registration, triggers)
- For purely informational queries

## The Four Requirements

Derived from the sound-loop technique cluster (`pre-declared-done-condition`, `external-grounded-validation`, `bounded-agent-loop`) plus open question #11 (the human gate). Each maps to one axis of goal / validation / loop / human.

### 1. Pre-declared done-condition — default FAIL

The success condition is written **before** the loop runs, and the result artifact **starts in the failing state**. Absence of proof reads as *not done* — never as done.

A done-condition has three parts:

| Part | What it states | Bad | Good |
|------|----------------|-----|------|
| **Measurable end state** | The observable fact that means "done" | "the output is good" | "the validator returns PASS and every required section is present and traced" |
| **Stated check** | *How* completion is proven, named in advance | "I'll know it when I see it" | "the validator reads the artifact and confirms all five conventions present" |
| **Constraints** | What must not be violated on the way | (none) | "no original responsibility silently dropped" |

The contract ships the result as **FAIL** and it only flips on real evidence. An LLM-controlled exit ("I think that's done") is not a done-condition.

### 2. External, independent validation

**Never let the producer grade its own output.** Validation is run by a **different agent** using a **different skill**, working from the artifact itself — not from the producer's say-so. The lead/referee owns the verdict.

Rank validators by trustworthiness; prefer the highest the artifact allows:

1. **Deterministic ground truth** — a test result, exit code, JSON-schema check, file diff, version equality. Strongest.
2. **Separate-context grounded LLM** — a fresh agent grading *from real evidence it Read*, not from a summary. Middle.
3. **LLM-as-judge** — ungrounded judgment. Weakest; a fallback, never the only gate where (1) or (2) is possible.

**Tamper-proof even deterministic checks.** A PASS is invalid unless the evidence was actually Read from the real artifact. "It looks like it passes" is not a PASS. (Tests get gamed; so does reading a summary instead of the file.)

When **nothing is machine-checkable**, the human is the cheapest external ground truth — which is requirement 4, not an excuse to let the producer self-grade.

### 3. Bounded iteration

An LLM-controlled exit can run forever — burning budget, repeating failed actions, or grinding the validator until it games it. Every loop ships **all four** guards:

1. **Hard round cap** — a deterministic ceiling counted by the supervisor (e.g. `max 3 produce↔validate rounds`), not judged by the model. The model's "I'll stop now" is itself LLM-controlled; the cap is the backstop.
2. **No-progress exit** — stop when a round changes nothing, so the loop doesn't spin on stuck state.
3. **Budget / kill-switch** — stop when the token or cost ceiling is hit; provide an out-of-band halt.
4. **Escalate, don't silently die** — on hitting a guard instead of the goal, hand off to the human gate with failure context. Never report "done" because you ran out of rounds. Escalation is not uniform — route each finding by gap type (see *Routing a FAIL by gap type* below): only genuine judgment calls reach the human.

### 4. Defined human gate

Mochiko is the successor to *human-in-loop*: the human is the framework's primary external validator, present by design. Every contract **names where the human validates**. *Presence* is non-negotiable; *placement* is a per-workflow call:

- **every cycle** — high-stakes or low-automation-trust loops
- **low validator-confidence only** — the validator can grade most cases but flags the uncertain ones
- **preference-gap only** — the loop converges autonomously and only escalates genuine judgment calls (a *preference gap*; knowledge and scope gaps route elsewhere — see *Routing a FAIL by gap type* below)

A workflow with no named human gate is incomplete, even if it never fires.

## Routing a FAIL by gap type — a refinement of requirements 3 and 4, not a fifth requirement

A FAIL is not one thing. When validation returns a finding — or the loop stalls short of its done-condition — *what you do with the finding* depends on the **kind of gap** it is. Sending every finding to the human (requirement 4) spends the human on questions a machine could settle; iterating the loop on every finding (requirement 3) grinds on gaps no round will ever close. Route each finding by its type:

- **Knowledge gap** — a factual unknown an investigation could settle (e.g. "which protocol does the existing system use?", "what does this artifact already contain?"). **Route to research** — a native `Explore` pass or equivalent investigation — not to the human. The loop resolves it and continues; this is the cheapest gap to close.
- **Preference gap** — a judgment or taste call only the human can make (e.g. "opt-in or opt-out?", "what threshold is acceptable?"). **Route to the human gate.** This is the genuine-judgment escalation that requirement 4's *preference-gap only* placement is named for — research cannot manufacture an answer to "should we".
- **Scope gap** — the work is bigger or different than it was framed (e.g. "this spans three separate concerns", "the goal conflicts with a stated constraint"). **Halt or split** — do not keep iterating. A round cap will never converge on a target the loop was never scoped to hit; surface it through requirement 3.4's escalation, with the failure context.

**The corollary — never cross the wires.** A preference question sent to research comes back empty: investigation cannot answer "should we". A knowledge question sent to the human burns the scarcest validator you have on something an investigation would have settled. Routing a gap to the wrong sink is a defect, not a shortcut — the wasted round is exactly the one requirement 3's bound exists to catch.

## How to apply: fill in the contract

Instantiate [`workflow-contract`](../../templates/workflow-contract.md) for the workflow. The filled-in contract is the inspectable proof that all four requirements are met — a reviewer can read it and see whether the validator is genuinely independent and where the human gate sits. A workflow without a filled contract has not met this skill's bar.

## Briefing the agents — a guide, not a fifth gate

The four requirements above govern the **loop**. Dispatching an agent *inside* the loop is a separate concern: it is **briefing**, not gating. A mochiko agent is a self-contained professional that **degrades gracefully** — a thin brief yields a worse result, not a broken one. So [`agent-dispatch`](../../templates/agent-dispatch.md) is a caller-side *briefing guide* that raises loop quality; it is **not** a fifth requirement, and an under-filled brief does not by itself make a loop unsound.

The one place briefing and loop-soundness meet is **requirement 2**: independence is carried by *who the caller dispatches* — a different agent running a different skill to grade than to produce — never by a line in a persona. A persona that *declares* itself independent or workflow-agnostic has told you nothing; the structure either separates producer from validator or it does not.

**Keep personas decoupled with the keystone test.** For every line of an agent persona (or skill): *would this be true of this professional on **any** job? → craft, keep. Does it only make sense inside **one** workflow? → coupling, cut.* Intrinsic traits survive (a reviewer *is* independent and skeptical; an author does not grade their own work — integrity, true everywhere). This-loop machinery — sibling-agent names, "dispatch," modes/paths/phases, "workflow-agnostic" meta-labels — goes. Decoupling is proven by that **absence**, not by a declaration of it. (Full doctrine + the grep-checkable deny-list: `.mochiko/brainstorms/agent-decoupling/synthesis.md`.)

## Red Flags — STOP and fix the loop

If you notice yourself thinking any of these, the loop is unsound. Stop and repair it:

- "The producer can just check its own work — it knows the output best"
- "We'll add the validator later / a round cap later"
- "It'll obviously stop when it's done"
- "Done means the agent stopped emitting tool calls"
- "No need for a human gate, the validator is good enough"
- "The done-condition is implied"
- "It passed last time, so it passes"

**All of these mean:** you are rationalizing away one of the four requirements. None are optional.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Producer self-check is faster" | A controller grading its own ungrounded sensor is the documented failure mode. Speed is not soundness. |
| "Separate agent is overkill here" | If an artifact is reviewable, independence is cheap insurance against confident-wrong output. |
| "The model will stop when it's done" | LLM-controlled exits never reliably fire. The deterministic cap is the only backstop. |
| "We don't need a default-FAIL, we'll see if it works" | Absence of proof must read as not-done. Default-PASS hides every silent failure. |
| "Human gates slow us down" | The whole framework is named for the human in the loop. Placement is negotiable; presence is not. |
| "This loop is too simple to need a contract" | Then the contract takes 2 minutes to fill. Simple loops with no contract are how unsound loops ship. |

## Related

- [`workflow-contract` template](../../templates/workflow-contract.md) — the fill-in form this skill governs
- [`agent-dispatch` template](../../templates/agent-dispatch.md) — the caller-side briefing guide for each dispatch inside the loop (not a fifth gate)
- See `ROADMAP.md` ("The sound-loop doctrine") and `agent-skills-research/synthesis/my-framework.md` for the source techniques
