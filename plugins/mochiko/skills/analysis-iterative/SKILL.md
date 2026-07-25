---
name: analysis-iterative
description: This skill MUST be invoked when enriching a sparse feature description into the Who / Problem / Value triad (plus explicit out-of-scope and success criteria) before a specification is authored, or when running a collaborative think-through / brainstorm that explores a problem through adaptive, one-question-at-a-time discovery and ends in a structured synthesis. SHOULD also invoke when a feature description lacks Who/Problem/Value clarity and needs conditioning before it reaches a requirements producer, or when deliberately working through a complex, multi-decision trade-off space. A general / shared skill — useful across clusters, not tied to a single workflow. Distinct from review-specifications, which reviews an already-drafted spec for gaps: this skill conditions raw input BEFORE a spec exists.
---

# Iterative Analysis

## Overview

Guide collaborative thinking by adapting questioning style, depth, and output to the complexity that emerges during conversation. Each question builds on the previous answer, and the format of each question adapts to the user's current state — confident, uncertain, or somewhere in between. Conclude with a structured synthesis document whose depth matches the conversation's depth.

## When to Use

- Brainstorming sessions that need structured exploration
- Deep analysis of complex problems with multiple decision points
- Thinking through feature requirements before specification
- Enriching sparse feature descriptions before specification authoring
- Exploring trade-offs that require deliberate choices
- When the user is unsure and needs help discovering what they think

## When NOT to Use

- **Quick clarifications** — simple questions do not need iterative questioning
- **Implementation details** — use planning skills instead
- **Specification review** — reviewing an already-drafted spec for gaps is post-draft work; use `mochiko:review-specifications` instead (this skill conditions raw input *before* a spec exists)
- **When user has clear direction** — use confirmations to verify, then wrap up fast. Do not slow down decisive users with unnecessary exploration.
- **Time-sensitive decisions** — iterative questioning takes time

## Adaptive Flow

Opening → Discovery → Adaptive Questioning → Conclusion.

**Opening** establishes the topic and asks the first question. Do not recite a fixed introduction script. Adapt tone and framing to the topic — a migration planning session opens differently than a notification brainstorm.

**Discovery** happens through the first 1-2 questions. Their purpose is dual: advance the conversation AND reveal how deep the exploration needs to go. See the Discovery section below.

**Adaptive Questioning** is the core loop. Each turn: read the answer, pick the right question format, ask one question. Continue until convergence signals appear or the user signals completion. See Question Format Adaptation and Reading Confidence Signals below.

**Conclusion** generates the synthesis artifact. See Smart Wrap-up below.

How long each phase lasts adapts to complexity. A crisp 3-question brainstorm may spend one turn in discovery. A complex architecture decision may spend three.

## Discovery

The first 1-2 questions serve a dual purpose: advance the conversation toward substance AND calibrate complexity.

**Principle:** Ask a question that is genuinely useful on its own merit — not a meta-question about "how deep should we go." Read the answer for complexity signals.

**What to look for in the first answer:**

- **Crisp, specific, opinionated** → User knows the domain. Move faster, lean toward structured options.
- **Vague, hedging, "I'm not sure"** → Exploration needed. Lean toward open probes. Do not default to a recommendation when the user has not yet formed a view.
- **Long, detailed, multi-dimensional** → Complex problem space. Plan for more questions and a comprehensive synthesis.

**"Unsure" is a first-class signal.** When the user says "I don't know" or "I'm not sure," it means probe deeper from a different angle — not fill in a default recommendation and move on. After 3+ consecutive "unsure" answers, reframe the question from a different angle or suggest narrowing the scope to a concrete sub-problem.

## Question Format Adaptation

Four formats, selected based on the user's current state:

### Structured Options + Recommendation

Use when: a genuine decision point exists and the user has enough context to evaluate options.

```
[Brief context connecting to previous answer]

**Question [N]**: [Clear, focused question]

**Options:**
- **A) [Option]**: [What it means and its implications]
- **B) [Option]**: [What it means and its implications]
- **C) [Option]**: [What it means and its implications]

**Recommendation**: Option [X] because [reasoning based on what is known so far]
```

### Open-Ended Probes

Use when: the user is uncertain, the problem space is not yet mapped, or an answer revealed unexpected complexity. No options — ask a question that helps the user externalize their thinking.

```
[Acknowledge what was shared]

[Open question that helps the user articulate what they know,
or surfaces information needed for the next decision]
```

### Confirmations

Use when: the user has clearly decided and the remaining question is just verification. Do not belabor obvious conclusions.

```
[Acknowledge the decision and its implications]

That seems settled. [Brief implication or transition to next topic]
```

### Recommend-then-Arbitrate

Use when: the user lacks domain expertise for the question at hand — or says so outright ("this is where I'd like you to recommend"). Structured options still force the user to *generate* an evaluation they don't have the domain knowledge for; an open probe produces silence. Instead, **supply the domain content yourself, then have the user sort and arbitrate it**: domain knowledge is a knowledge gap (yours or research's to fill); which of it to adopt, and how strictly, is a preference gap (only theirs).

```
[Supply a concrete recommended set — actual domain content, not option labels,
 with the reasoning and the strongest counter-consideration for each piece]

[Ask the user to sort/arbitrate: keep, drop, tighten, or re-rank —
 not to generate alternatives]
```

Follow-through matters: a user's sort of unfamiliar material can misfire on oversight rather than judgment — walk salient demotions back past them once ("you demoted X — deliberate?") before treating the sort as settled.

**Meta-principle:** Pick the format that serves the user's current state. If uncertain which format to use, prefer the open probe — it gathers information without forcing premature commitment. But when the user has explicitly handed you the domain expert's seat, the open probe is an abdication — recommend, then let them arbitrate.

## Reading Confidence Signals

Recalibrate after every answer. The user's state can shift mid-conversation.

| Signal | Meaning | Response |
|--------|---------|----------|
| Crisp, specific, immediate | High confidence | Move faster. Structured options. |
| Hedging, "it depends," "I think maybe" | Moderate uncertainty | Slow down. Mix of probes and options. |
| "I don't know," "I'm not sure" | Low confidence | Switch to open probes. Help discover before deciding. |
| Disagrees with recommendation | Deliberate preference | Explore reasoning, push back once, then respect the choice. |
| Quick agreement without elaboration | Possible passive acceptance | Light challenge: "Just to pressure-test — what makes you prefer this over [alternative]?" |

**Disagreement handling:** When the user picks differently than recommended — explore their reasoning, present counterarguments respectfully but directly, and if they maintain their choice after the challenge, accept it and integrate. Mark the decision as `Contested` in the synthesis.

**Ratification streaks:** ~three consecutive unelaborated adoptions ("go with your recommendation") are a passive-acceptance signal even when each answer reads as decisive. Flag the streak to the user plainly, then pose the next genuine fork with steelmanned options and **no marked recommendation** — forcing a genuine read. Validated across three design sessions: each time, the flagged user promptly re-engaged (once by reversing a fresh ruling).

## Smart Wrap-up

**Convergence signals** — watch for these:
- Answers are becoming confirmatory, not exploratory
- Key trade-offs have been explicitly addressed
- No new dimensions are emerging
- User's confidence in direction is increasing

**Nudge format:** Suggest wrap-up, but always offer an exit:

```
The core decisions feel settled. Should the synthesis be generated, or is there
another dimension to explore?
```

**Asymmetry principle:** Wrapping up too early is worse than wrapping up too late. An extra question costs one turn. A missing decision costs rework. When in doubt, ask one more question.

Never force synthesis. The skill may nudge, but the user always has final say on when to conclude.

## Output

Generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md).

**Confidence indicators** — assign based on conversation observation:

| Indicator | Meaning |
|-----------|---------|
| `Confident` | Clear, reasoned choice with no hesitation |
| `Assumed` | Inferred from context, never explicitly confirmed |
| `Contested` | User disagreed with recommendation; deliberate choice |
| `Unsure` | Expressed uncertainty; decided provisionally |
| `Deferred` | Explicitly postponed — not enough information now |

**Output scales with conversation.** A 3-question brainstorm produces Problem Statement, Key Decisions, and Next Steps. A 10-question deep exploration produces all sections including Decision Trail and Risks. Never pad a lean conversation with filler sections.

## Common Mistakes

| Mistake | Why it fails | Fix |
|---------|--------------|-----|
| Always using structured options | Options force premature decisions on an unsure user | Open probes help them discover what they think |
| Ignoring "unsure" signals | Uncertainty read as indifference → recommendations passively accepted, never owned | Probe deeper from a different angle |
| Multiple questions per turn | Fractured attention, shallow answers across all of them | One question per turn — always |
| Questions that don't connect | Topic-jumping breaks collaborative momentum | Show how the previous answer shapes the direction |
| Premature synthesis | Forced wrap-up leaves core trade-offs unaddressed | Nudge, never force; the user has final say |
| Rigid opening script | A recited introduction ignores the topic's tone and energy | Adapt the opening to both |
| Padding the synthesis | Filler sections dilute a lean conversation's record | Output depth matches conversation depth |

---

## Two output shapes, one engine

The adaptive flow above is the single questioning engine. It carries two output shapes; the engine is identical and only the agenda and the concluding artifact differ. Pick the shape that matches the work in front of you.

### General analysis (default)

Open exploration — brainstorming, trade-off analysis, thinking a problem through. Converges when the decisions are settled and concludes with a synthesis document. Use [SYNTHESIS.md](SYNTHESIS.md) for the output.

### Specification-input enrichment

When the work is conditioning a sparse feature description before a specification is authored — the description is thin on the **Who / Problem / Value** triad — run the focused variant in [SPECIFICATION-INPUT.md](SPECIFICATION-INPUT.md). It fills the missing triad elements plus scope and success, and concludes with the [ENRICHMENT.md](ENRICHMENT.md) artifact, handed to whoever authors the specification next (the requirements producer).

The enriched description is plain output: whoever requested the enrichment reads the artifact directly. How a caller asks for this shape — and tells you which triad elements it already knows are missing — is a caller-side dispatch concern (see the agent-dispatch briefing), not something this skill parses or owns.

## Reference

See [ADAPTIVE-EXAMPLES.md](references/ADAPTIVE-EXAMPLES.md) for annotated conversations showing these principles in action.
