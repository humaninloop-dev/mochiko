# `@`-reference recovery superseded — the platform bug it named is resolved

**Status:** ruled · **Date:** 2026-08-01

## Context

Five commands carried an empty-`$ARGUMENTS` recovery clause that named a *cause*: the Claude Code
`@`-reference drop bug, where an `@`-referenced argument was silently dropped from `$ARGUMENTS`.
The recovery — "ask the user to re-enter it, or confirm the detected feature" — is a hard-won fix,
protected by `command-succinctness-strip` record §7's retrofit-regression set and restored under
independent-audit pressure (check 14, preserved responsibilities) at the v0.34.0 plan pilot after
a first draft silently dropped it.

The bug is now resolved platform-side: `$ARGUMENTS` reliably carries what the user typed, so the
*named cause* no longer exists and the re-enter workaround it justified is dead surface.

## Decision (user-ruled, 2026-08-01)

- **The `@`-reference drop-bug attribution and the re-enter workaround are superseded** across
  `specify`, `plan`, `implement`, `slice`, and `brainstorm` — recorded by supersession-by-ruling
  strip entries at v0.37.0, never by silent omission.
- **The legitimate empty-args behavior is preserved, not dropped:**
  - `specify` / `brainstorm` (entry commands, no feature to fall back to): empty still asks for the
    description / topic — only the "known bug" framing is removed.
  - `plan` / `implement` / `slice` (feature-operating): empty resolves to the most-recent
    in-progress feature under `.mochiko/specs/`, **confirmed with the user before the run opens** —
    the protected "confirm the detected feature" half survives; the re-enter half retires.

## Rationale

The recovery's protection premise was "a platform bug silently corrupts `$ARGUMENTS`, and the
verbose recovery encodes the fix." With the bug fixed, that premise dissolves; keeping the
re-enter prompt would guard a failure mode that no longer occurs. The detected-feature confirm on
the feature-operating commands is retained on its own merit — silently proceeding on a *guessed*
feature would risk a full, expensive run against the wrong one.

## Alternatives considered

- **Remove only the parenthetical attribution, keep the full two-option prompt everywhere:**
  rejected — the re-enter option exists solely for the now-resolved bug; keeping it is dead surface.
- **Silent resolution on plan/implement/slice (no confirm):** rejected by the user — it drops the
  protected "confirm the detected feature" responsibility check 14 guards, and risks an expensive
  run on a mis-detected feature.

**Kept deliberately:** the empty-args ask on `specify` / `brainstorm`; the detected-feature confirm
on `plan` / `implement` / `slice`; `implement.md`'s most-recent-feature resolution clause.

**Ripple:** `command-succinctness-strip` record §7's protected-set membership for the `@`-reference
recovery is now spent — noted in each strip entry so a future auditor reads the supersession, not
an accidental drop.
