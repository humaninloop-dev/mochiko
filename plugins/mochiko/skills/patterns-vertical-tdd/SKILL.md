---
name: patterns-vertical-tdd
description: This skill MUST be invoked when structuring a feature's implementation into vertical slices under test-first discipline — mapping user stories to TDD cycles (Simple / Split / Merge cases), classifying foundation versus feature cycles, ordering each cycle's tasks test-first (failing test → implement → refactor → verify against real infrastructure), and producing the story→cycle mapping (task-mapping.md, the source of truth) and the cycle-based tasks structure (tasks.md) with per-task file paths and brownfield markers. SHOULD also invoke when the work involves "create task mapping", "structure implementation", "define cycles", "vertical slice", "TDD", "test-first", "cycle structure", "red-green-refactor", "story→cycle mapping", "testable increment", or "implementation tasks". Structures the implementation at design time — the slicing, the test-first ordering, and the tasks.md structure — not the runtime execution of those cycles (writing code through red/green/refactor against real infrastructure), which is downstream.
---

# Vertical Slicing with TDD

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform requirements into implementation tasks organized as vertical slices with strict TDD discipline. Each slice (called a "cycle") delivers observable, testable value and follows test-first principles.

This skill structures **two artifacts**: `task-mapping.md` (the story→cycle mapping and slice rationale — the source of truth for how stories become cycles) and `tasks.md` (the cycle-based TDD task list the mapping expands into). It works at **design time** — it structures the tasks; it does not execute them.

This is a discipline-enforcing skill. The test-first structure exists because tests written after code verify implementation, not requirements. Skipping or reordering undermines the entire purpose.

## When to Use

- Transforming user stories into implementation tasks as the task-structuring producer
- Creating task-mapping.md from a specification
- Structuring tasks.md with proper TDD ordering
- Breaking down large features into vertically sliced, testable increments

## When NOT to Use

- **Bug fixes** - Single-task fixes don't need cycle structure
- **Documentation-only tasks** - No TDD needed for docs
- **Spike/research tasks** - Exploration doesn't follow TDD
- **Refactoring without behavior change** - Existing tests suffice
- **When tests already exist** - Don't duplicate test-first for covered code
- **Executing the cycles** - This skill structures tasks (design-time). Running them — writing code through red/green/refactor against real infrastructure and producing a cycle report — is downstream execution, a separate runtime concern from task structuring.

## Core Principles

### 1. Vertical Over Horizontal

**Wrong** (horizontal slicing):
```
Phase 1: All models
Phase 2: All services
Phase 3: All endpoints
Phase 4: All tests
```

**Right** (vertical slicing):
```
Cycle 1: User creation (model + service + endpoint + test)
Cycle 2: User authentication (model + service + endpoint + test)
Cycle 3: User profile management (model + service + endpoint + test)
```

### 2. Test-First at Task Level

Every cycle structures tasks so tests come before implementation — the failing test opens the cycle, the `**TEST:**` real-infrastructure verification closes it (see *Cycle Structure* below).

### 3. Foundation + Parallel

Foundation cycles run sequentially and establish what every feature depends on; feature cycles follow, parallel-eligible unless they depend on another feature cycle (see *Foundation vs Feature Cycles* below).

### 4. Layered Testability

Each cycle must be testable at multiple levels:
- **Automated tests**: Unit, integration, and/or E2E tests
- **Demonstrable behavior**: Observable by stakeholders
- **Contract verification**: Meets acceptance criteria from spec

## Identifying Vertical Slices

See [SLICE-IDENTIFICATION.md](references/SLICE-IDENTIFICATION.md) for detailed heuristics on identifying good vertical slices from requirements.

### Quick Heuristics

A good vertical slice:
1. **Delivers user value**: Something a user could observe or use
2. **Touches all layers**: Model, service, API, UI (as applicable)
3. **Is independently testable**: Can verify it works without other slices
4. **Is sized appropriately**: Completable in 1-3 implementation sessions

### Slice Boundaries

| Boundary Signal | Action |
|-----------------|--------|
| Distinct user action | New cycle |
| Different acceptance scenario | May be new cycle or same cycle |
| Shared infrastructure need | Foundation cycle |
| Optional enhancement | Feature cycle (can parallelize) |

## Cycle Structure

The canonical `tasks.md` skeleton the producer fills is [`tasks-template.md`](../../templates/tasks-template.md) — the single source of the cycle / `tasks.md` structure: the `### Cycle N` header with Stories/Dependencies/Type lines, the `TN.X` task list, the closing `**Checkpoint**:`. [CYCLE-STRUCTURE.md](references/CYCLE-STRUCTURE.md) teaches how to fill it — cycle anatomy, task-ID format, file-path conventions, worked examples. Consult those two sources; do not restate the format.

The final task of every cycle is a **`**TEST:**` verification task** — a real-infrastructure gate validating the cycle's acceptance criteria, not a re-run of the automated tests. Its full grammar (fields, action modifiers, assert patterns, classification) lives in [TEST-GRAMMAR.md](references/TEST-GRAMMAR.md).

### Markers

| Marker | Meaning |
|--------|---------|
| `[P]` | Parallel-eligible (no dependencies blocking) |
| `[US#]` | Maps to user story number |
| `[EXTEND]` | Extends existing file (brownfield) |
| `[MODIFY]` | Modifies existing code (brownfield) |

## Foundation vs Feature Cycles

**Foundation cycles** establish infrastructure that ALL features depend on — platform infrastructure (IP-XXX items from constraints-and-decisions.md), data models, auth, API framework, error handling — and complete sequentially before any feature cycle starts. **Identification**: Ask "Could ANY user story work **in production** without this?" If no, it's foundation.

**Feature cycles** deliver user value incrementally — mapping directly to user stories, independently testable, often parallel-eligible once foundation is complete. **Identification**: Ask "Does this deliver value a user could observe?" If yes, it's a feature.

## TDD Task Sequence

Each cycle follows the red-green-refactor pattern (the task lines themselves are template-defined):

- **Red — write the failing test first.** The test expresses the acceptance criteria, is specific about expected behavior, and must FAIL when run — nothing is implemented yet.
- **Green — implement to pass.** Minimal code (just enough to pass), but complete across all necessary layers (model, service, endpoint).
- **Refactor and verify.** Improve code quality without changing behavior; all tests still pass.
- **TEST — verify against real infrastructure.** Exercise the slice against **real** files, DBs, and APIs — never mocks — producing a tangible, observable outcome that verifies the slice against its **spec acceptance criteria** and gates cycle completion. This gate is what makes the vertical slice actually vertical; author it in the `**TEST:**` grammar (see *Cycle Structure* above).

## Mapping Stories to Cycles

The story→cycle decisions and the slice rationale behind them are authored into **`task-mapping.md`** — the source of truth for how stories become cycles. `tasks.md`'s Story → Cycle table is a **derived echo** of `task-mapping.md`, regenerated from it, never an independent second source.

### task-mapping.md — canonical compact shape

`task-mapping.md` follows the deliverable envelope
([`artifact-format.md`](../../templates/artifact-format.md)); it is a **mapping, not an
essay** — the story→cycle table plus per-cycle rationale lines, nothing more:

```markdown
# Task Mapping: {feature_id}

> Source of truth for story→cycle decisions and slice rationale. Expanded into tasks.md.

## Story → Cycle Mapping  *(the ID index)*

| Story | Priority | Cycle(s) | Case |
|-------|----------|----------|------|
| US-1 | P1 | C1 | Simple |
| US-2 | P1 | C2, C3 | Split — [why, one line] |
| US-3, US-4 | P2 | C4 | Merge — [why, one line] |

## Cycles

| Cycle | Title | Type | Depends on | Stories | Rationale (≤ 2 lines) |
|-------|-------|------|-----------|---------|-----------------------|
| C1 | [title] | Foundation | — | US-1 | [why this is a vertical slice; what it establishes] |
| C2 | [title] | Feature [P] | C1 | US-2 | [why these tasks graduate together] |

## Slicing notes  *(only when a decision needs more than its rationale cell — ≤ 3 lines each; omit when empty)*

- **C2/C3 split:** [the judgment that didn't fit the cell]
```

Cite spec/plan content by ID (`US-#`, `FR-#`, `C-#`) — never re-quote it. A mapping that
needs pages of prose is hiding slicing uncertainty that belongs in the producer's
disclosure report, not the artifact.

### Case column: Simple / Split / Merge

- **Simple** — story = cycle: a well-scoped story becomes one cycle (US-1 create-a-task → C1).
- **Split** — story > cycle: a too-large story splits across cycles (US-2 manage-tasks → create / edit / delete / complete cycles).
- **Merge** — stories < cycle: too-small stories share one cycle (US-3 task-count + US-4 completed-count → one statistics cycle).

Size calibration — when to split or merge — is in [SLICE-IDENTIFICATION.md](references/SLICE-IDENTIFICATION.md).

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "I'll write tests after the code works" | Tests written after verify implementation, not requirements. Test-first verifies behavior. No exceptions. |
| "This is too simple to need tests first" | Simple code becomes complex. Tests document intent. Write them first anyway. |
| "Tests slow down development" | Debugging untested code is slower. Tests catch bugs immediately. Faster overall. |
| "I'm just prototyping" | Prototypes become production code. Start with tests or mark SPIKE explicitly. |
| "Horizontal slicing is more efficient" | Horizontal slicing defers integration. Bugs surface late. Vertical finds issues early. |
| "Foundation doesn't need tests" | Foundation is tested by feature cycles. But foundation cycles still follow TDD internally. |
| "Manual verification is sufficient" | Manual testing doesn't scale. Automated tests enable confident refactoring. |
| "The client wants it fast, skip tests" | Skipped tests create technical debt. Bugs cost more than tests. Push back. |

## Red Flags - STOP and Restart Properly

If any of these thoughts arise, STOP immediately:

- "Let me just get the code working first"
- "This feature is straightforward, tests can come after"
- "We'll add tests in a later cycle"
- "The horizontal approach makes more sense here"
- "Foundation setup doesn't need the full TDD ceremony"
- "It's faster to write all models, then all services, then all tests"

**All of these mean:** Rationalization is occurring. Return to test-first discipline.

**No exceptions** — not for "simple" features, tight deadlines, "just the foundation", "we'll refactor later", nor even if the user says "just write the code".

## Quality Checklist

Before finalizing task mapping or task list:

- [ ] Every P1/P2 story maps to at least one cycle
- [ ] Cycles are vertical slices (not horizontal layers)
- [ ] Foundation cycles identified and sequenced
- [ ] Feature cycles marked [P] where appropriate
- [ ] Each cycle has TDD structure (failing test first)
- [ ] Each cycle ends with a `**TEST:**` verification task (real-infrastructure gate)
- [ ] Every task has a specific file path
- [ ] Brownfield tasks carry `[EXTEND]` / `[MODIFY]` markers
- [ ] Dependencies are minimal and explicit
- [ ] Cycles are independently testable
- [ ] Story→cycle decisions and rationale recorded in task-mapping.md (tasks.md's Story → Cycle table derives from it)
