---
name: brownfield-integration
description: This skill MUST be invoked when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file already on disk: reading the whole file before writing, following its established patterns, preserving its interface, and detecting conflicts before adding code. SHOULD also invoke when extending an existing file, modifying existing behavior, integrating against an established interface, or following patterns set by prior work in the codebase. Consumes the `[EXTEND]`/`[MODIFY]` markers that patterns-vertical-tdd defines at design time; this is the implement-time, read-before-write craft of making that one modification safely — NOT the execution of the cycle the task belongs to (that is executing-tdd-cycle, which co-fires on the same brownfield task and drives red/green/refactor).
---

# Brownfield Integration

## Overview

Craft for implementing a task that touches existing code. Brownfield tasks arrive already tagged `[EXTEND]` or `[MODIFY]`: the marker **vocabulary** is defined by `patterns-vertical-tdd`, which stamps those markers onto tasks at design time. This skill does not redefine the markers — it is the implement-time discipline of **consuming** one safely: read the existing code first, follow what is already there, preserve the interface, and surface conflicts rather than silently resolving them.

**The existing code is not wrong until proven otherwise.** It has consumers, tests, and patterns that evolved for reasons not immediately visible.

**Violating the letter of the rules is violating the spirit of the rules.**

## When to Use

- A task carries an `[EXTEND]` marker — you are adding to a file that already exists
- A task carries a `[MODIFY]` marker — you are changing behavior in a file that already exists
- Any task that references a file already on disk
- When following patterns established by prior work in the codebase

## When NOT to Use

- Greenfield tasks creating entirely new files
- Tasks with no reference to existing code
- Refactoring work — out of scope for an extend/modify task; note the opportunity, do not act on it

## Core Process

### EXTEND vs. MODIFY: interface impact

`patterns-vertical-tdd` owns what the `[EXTEND]` and `[MODIFY]` markers *mean*. What each one is allowed to do to an existing interface — the implement-time consumption rule this skill enforces — is below:

| Marker | Scope of change | Interface impact |
|--------|-----------------|------------------|
| `[EXTEND]` | New functions, new methods, new exports — alongside what exists | MUST NOT change existing function signatures, exports, or type contracts |
| `[MODIFY]` | The specified sections only | MAY change function internals; MUST NOT change signatures unless the task explicitly says so |

**Never treat an `[EXTEND]` task as a `[MODIFY]`.** If you believe the existing code cannot support the extension without changing its interface, surface it as a blocker — do not silently rewrite.

### Read-Before-Write Checklist

Before writing any code in an existing file, complete all five steps:

1. **Read the full file** — not just the section you plan to change. Understand the complete context.
2. **Identify naming conventions** — variable naming (camelCase, snake_case), file naming, function naming patterns. Follow them exactly.
3. **Identify error handling patterns** — how does existing code handle errors? Try-catch, Result types, error callbacks? Match the pattern.
4. **Identify import style** — relative vs. absolute imports, named vs. default exports, import ordering. Follow the same style.
5. **Identify test patterns** — if the file has tests, how are they structured? Match describe/it nesting, assertion style, fixture patterns.

### Interface Preservation

When extending existing code:

- Do NOT change function signatures (parameter order, types, return types)
- Do NOT change export surfaces (what is exported, export names)
- Do NOT rename existing variables, functions, or classes
- Do NOT change the file's public API unless the task explicitly says `[MODIFY]`
- DO add new exports alongside existing ones
- DO follow the file's established patterns for new code

### Conflict Detection

Before adding new code, check for:

- **Name collisions** — search the file for the function/class/variable name you plan to add
- **Import collisions** — verify your new imports don't shadow existing ones
- **Test file alignment** — if adding to `user.ts`, check that `user.test.ts` exists and follow its patterns
- **Circular dependencies** — verify your new imports don't create circular reference chains

### When to Flag

Surface these as blockers rather than silently resolving them — they belong in the cycle report the run produces (owned by `executing-tdd-cycle`), not in a quiet workaround:

- Existing code contradicts the task description
- The file's patterns are inconsistent (multiple conflicting conventions)
- The task carries `[EXTEND]` but the existing interface cannot support the addition without modification
- Existing tests would break from the addition (interface leak)
- The file has no tests but the task expects test-first development

## Common Mistakes

### Mistake: Not Reading the Full File

**What goes wrong:** You add code that duplicates existing functionality, uses different naming conventions, or conflicts with code you didn't see.

**Fix:** Always read the entire file before making any changes. Skim is not sufficient for brownfield work.

### Mistake: Silently Rewriting When Asked to Extend

**What goes wrong:** Existing consumers break because the interface changed. Tests fail for unrelated code. Your report doesn't explain why unrelated files were modified.

**Fix:** EXTEND means extend. If you cannot extend, flag it. Never silently rewrite.

### Mistake: Ignoring Existing Error Handling

**What goes wrong:** Your new code throws raw exceptions while the rest of the file uses Result types. Or your code returns null while existing code throws. Inconsistency confuses consumers.

**Fix:** Step 3 of the Read-Before-Write Checklist. Match the existing error handling pattern exactly.

### Mistake: Adding "Better" Patterns

**What goes wrong:** You introduce a "better" pattern alongside the existing one. Now the file has two patterns. The next developer doesn't know which to follow. Consistency is more valuable than local improvement.

**Fix:** Follow existing patterns, even if you'd prefer different ones. Note the improvement opportunity in your report.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "This file is small, I can just skim it" | Small files have hidden conventions. Read the full file. Step 1 exists because skimming misses patterns. |
| "My pattern is better than what exists" | Consistency is more valuable than local improvement. Two patterns in one file is worse than one imperfect pattern. |
| "The existing code doesn't follow best practices" | Existing code has consumers. Introducing a second convention creates confusion. Note it, follow it. |
| "I need to refactor to make my extension work" | If EXTEND doesn't fit, flag it. Silent refactoring breaks existing consumers. |
| "The interface is obviously wrong" | It made sense to someone with context not visible now. Read more before judging. |
| "I'll just fix this one small thing while I'm here" | Scope creep starts with "just one thing." Note it in the report, don't act on it. |

## Red Flags — STOP and Reconsider

- "This existing code is messy, I'll clean it up" — Not the current scope. Note it, don't fix it.
- "I'll use a better pattern here" — Consistency beats local optimization. Follow what exists.
- "The existing tests don't cover this" — That's a pre-existing gap, not a problem to fix now.
- "I need to refactor this to make my change work" — Flag it — don't silently refactor.
- "This interface doesn't make sense" — It made sense to someone. Read more context before judging.

**No exceptions:**
- Not for "obviously broken" code
- Not for "trivially better" patterns
- Not for "quick cleanup while I'm here"
- Not even if the existing code has no tests
- Not even if the existing naming is inconsistent
