---
name: patterns-model-tiering
description: This skill MUST be invoked when dispatching exploration or fact-finding work in any mochiko run — routing each read by the class key. locate/enumerate/targeted-read gaps go to a native `Explore` subagent spawned with an explicit `model: haiku` override; interpretive reads, decision-driving absences, and completeness-sensitive enumerations stay on the session tier. SHOULD also invoke on 'model tiering', 'cheap explorer', 'which model', 'explore the code', 'targeted read', or 'fact-find dispatch'. Governs dispatch tier only — rostered seats never change model (model-tiered-seats D5); third sibling of patterns-sound-loop and patterns-transport-floor.
---

# Model Tiering — The Class-Keyed Dispatch Floor

**Every read rides the lowest tier where its answer can be trusted.**

## Overview

Rostered mochiko personas run on the strong tier and stay there; this floor governs the
*reads they and the lead dispatch along the way*. The economics are documented, not assumed:
Haiku is ~5× cheaper than Opus and ~10× cheaper than Fable per token both directions, and on
subscription seats cheap-model work preserves Opus-cap headroom (model-tiered-seats D1).
The native `Explore` agent inherits the session model when spawned bare, so a bare spawn is
not cheap — the cheap rung is native `Explore` **with an explicit `model: haiku` override on
the Agent tool call** (D4 as amended by the 2026-08-19 retarget ruling: a plugin-scoped
seat cannot be spawned from agent-team teammates, so the native agent is the only rung that
resolves from every dispatcher).

## The class key

**Cheap tier — dispatch native `Explore` with `model: haiku`, disposable per gap:**

- locate a file, symbol, config key, or line
- enumerate a bounded, **spot-checkable** set (files, frontmatter values, pattern matches)
- targeted read of a named span, quoted verbatim
- deterministic checks (stack detection, presence tests, exact-command output)

**Session tier — the dispatching seat reads it itself, or a strong seat does:**

- interpretive reading — what a design implies, whether patterns conflict, what quality a
  surface has
- any gap where **absence would drive a decision** — a weak negative that would mislead a
  producer stays on the strong tier
- an enumeration whose **completeness** would drive a decision (the F2 guide-line) — the
  cheap tier takes only spot-checkable enumerations
- all producing, reviewing, and grading work — never tiered down (D5)

## The dispatch ladder

Direct tool call → cheap `Explore` (haiku) → session-tier read. Each gap sits on the lowest
rung where the answer can be trusted: a one-file read the dispatcher already knows the path
of is a direct tool call, not a spawn; a sweep across many files is the cheap `Explore`; a
judgment read is session tier. A spawn that costs more than the read it saves has failed
the ladder.

## Dispatch shape

- **Disposable per gap** — spawn the native `Explore` agent via the Agent tool, one gap per
  dispatch, discard after. Never a standing "librarian" seat: a standing seat re-pays its
  transcript across gate pauses (D4/F5).
- **The override is the pin** — every cheap dispatch MUST pass `model: haiku` explicitly on
  the Agent tool call; a spawn without the override inherits the session tier and has
  failed this floor.
- **Fact-finder brief** — the dispatch brief states the one gap and constrains the return:
  terse spot-checkable facts with `file:line` provenance, verbatim quotes never paraphrase,
  absence reported method-scoped, no interpretation. The brief carries this because the
  native agent has no persona pinning it.
- **Terse return, provenance attached** — the explorer returns the smallest decisive facts
  with `file:line` provenance; the bulk read stays inside the disposable context
  (the context-health test, D1).
- **Weak-negative watch** — a cheap "not found" is method-scoped ("not found by `<method>`
  over `<scope>`"), never a verdict of nonexistence; if that absence would drive a decision,
  the class key already routed it session-tier — re-route on sight.

## The brief obligation

The lead MUST carry this rule into **every seat brief**: one line directing the seat to
route its own locate/enumerate reads to a native `Explore` subagent spawned `model: haiku`
per this skill. Spawned teammates never load `skills:` frontmatter, so the brief line is
the only channel that reaches them — a run whose seat briefs omit it has not applied this
floor.

## When NOT to Use

- **A read the dispatcher does cheaper inline** — known path, one file, small span: rung 1,
  direct tool call.
- **Seat-roster model choices** — rostered personas keep their declared models; retiering a
  rostered seat is deferred to its own reliability-gated brainstorm (D5), never done ad hoc.
- **Review, grading, or production dispatches** — never tiered down; the sound-loop floor
  governs those seats.

## Quality Checklist

- [ ] Every exploration/fact-finding dispatch classified against the class key before spawn
- [ ] Locate/enumerate/targeted-read gaps dispatched to native `Explore` with the
      `model: haiku` override passed explicitly, one gap per spawn
- [ ] Interpretive, absence-driven, and completeness-sensitive reads kept session-tier
- [ ] Every seat brief carries the one-line routing rule
- [ ] No standing cheap seat; explorers discarded after their gap
- [ ] Cheap negatives treated as method-scoped, re-routed when absence drives a decision
