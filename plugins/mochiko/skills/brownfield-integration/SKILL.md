---
name: brownfield-integration
description: This skill MUST be invoked when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file on disk: reading the whole file first, following its patterns, and preserving its interface. SHOULD also invoke when extending an existing file, modifying existing behavior, or following prior patterns. Consumes the extend/modify classification from the card's brownfield exposure — NOT the cycle execution that co-fires (mochiko:executing-tdd-cycle).
---

# Brownfield Integration

## Overview

Craft for implementing a task that touches existing code — read the existing code first,
follow what is already there, preserve the interface, and surface conflicts rather than
silently resolving them.

**The existing code is not wrong until proven otherwise.** It has consumers, tests, and patterns that evolved for reasons not immediately visible.

## Rules — load the schema first

Your first action, before any read or write in the existing file: **Read `schema.yaml`
(this skill's own directory) raw, in full** — the small families ship no common file, so
the pair's own schema is the whole first action. The schema is the source of truth for
this craft's binding rules, nested in six sections, each addressable by its section ID:
`brownfield-integration.sec.independence` · `brownfield-integration.sec.scope` ·
`brownfield-integration.sec.inputs` · `brownfield-integration.sec.verdict` ·
`brownfield-integration.sec.output` · `brownfield-integration.sec.reserved`. Interpret it
live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
of `class: floor` is always read and always delivered; a `pointer:` rule binds you to that
file's or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 6 rules of `class: floor`
are non-waivable. Before the first read step, state the floor count back — a skipped or
partial read leaves that count blank: halt and surface it, and halt likewise if the
schema's `class: floor` count disagrees with the pin.

## When NOT to Use

- Greenfield tasks creating entirely new files
- Tasks with no reference to existing code
- Refactoring work — out of scope for an extend/modify task

## Common Mistakes

| Mistake | What goes wrong | Fix |
|---------|-----------------|-----|
| Not reading the full file | Duplicated functionality, mismatched conventions, conflicts with unseen code | Read the entire file first — skimming is not sufficient for brownfield work |
| Silently rewriting when asked to extend | Consumers break on the changed interface; unrelated tests fail; unexplained modifications | EXTEND means extend; if you cannot extend, flag it |
| Ignoring existing error handling | Raw exceptions beside Result types — inconsistency confuses consumers | Match the file's error-handling pattern exactly |
| Adding "better" patterns | Two patterns in one file; the next developer can't tell which to follow | Follow existing patterns; note the improvement opportunity in your report |

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "This file is small, I can just skim it" | Small files have hidden conventions. Read the full file. The read-first floor exists because skimming misses patterns. |
| "My pattern is better than what exists" | Consistency is more valuable than local improvement. Two patterns in one file is worse than one imperfect pattern. |
| "The existing code doesn't follow best practices" | Existing code has consumers. Introducing a second convention creates confusion. Note it, follow it. |
| "I need to refactor to make my extension work" | If EXTEND doesn't fit, flag it. Silent refactoring breaks existing consumers. |
| "The interface is obviously wrong" | It made sense to someone with context not visible now. Read more before judging. |
| "I'll just fix this one small thing while I'm here" | Scope creep starts with "just one thing." Note it in the report, don't act on it. |

## Red Flags — STOP and Reconsider

If any of these thoughts arise, stop — the Rationalizations table above rebuts each:

- "This existing code is messy, I'll clean it up"
- "I'll use a better pattern here"
- "The existing tests don't cover this" — a pre-existing gap, not a problem to fix now
- "I need to refactor this to make my change work"
- "This interface doesn't make sense"
