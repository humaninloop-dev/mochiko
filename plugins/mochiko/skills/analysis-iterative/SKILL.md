---
name: analysis-iterative
description: This skill MUST be invoked when enriching a sparse feature description into the Who / Problem / Value triad before a specification is authored, or when running a collaborative think-through / brainstorm through adaptive, one-question-at-a-time discovery. SHOULD also invoke when working through a complex, multi-decision trade-off space. Distinct from review-specifications, which reviews an already-drafted spec for gaps: this skill conditions raw input BEFORE a spec exists.
---

# Iterative Analysis

## Overview

Guide collaborative thinking by adapting questioning style, depth, and output to the complexity that emerges during conversation. Each question builds on the previous answer, and the format of each question adapts to the user's current state — confident, uncertain, or somewhere in between. Conclude with a structured synthesis document whose depth matches the conversation's depth.

## When NOT to Use

- **Quick clarifications** — simple questions do not need iterative questioning
- **Implementation details** — use planning skills instead
- **Specification review** — reviewing an already-drafted spec for gaps is post-draft work; use `mochiko:review-specifications` instead (this skill conditions raw input *before* a spec exists)
- **When user has clear direction** — use confirmations to verify, then wrap up fast. Do not slow down decisive users with unnecessary exploration.
- **Time-sensitive decisions** — iterative questioning takes time

## Output

Generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md).

**Floor (non-waivable)** — surface every elicited unknown as an open question in the produced artifacts; a vague zone the principal could not resolve is never silently omitted.

**Confidence indicators** — assign based on conversation observation:

| Indicator | Meaning |
|-----------|---------|
| `Confident` | Clear, reasoned choice with no hesitation |
| `Assumed` | Inferred from context, never explicitly confirmed |
| `Contested` | User disagreed with recommendation; deliberate choice |
| `Unsure` | Expressed uncertainty; decided provisionally |
| `Deferred` | Explicitly postponed — not enough information now |

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

The enriched description is plain output: whoever requested the enrichment reads the artifact directly. How a caller asks for this shape — and tells you which triad elements it already knows are missing — is a caller-side dispatch concern, carried in the caller's own brief, not something this skill parses or owns.

## Reference

See [ADAPTIVE-EXAMPLES.md](references/ADAPTIVE-EXAMPLES.md) for annotated conversations showing these principles in action.
