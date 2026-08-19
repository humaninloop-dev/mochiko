---
name: authoring-architecture
description: This skill MUST be invoked when authoring or updating `ARCHITECTURE.md` at a plan/implement landing that changed structure, and to run the **landing diff** (built topology vs the approved architecture delta) whenever an approved delta existed. Owns the In-flight pointer list. SHOULD also invoke on 'update the architecture doc', 'architecture drift', or 'built vs approved architecture'. Current state only; distinct from mochiko:patterns-system-design. No structural change → no update.
---

# Authoring Architecture

## Overview

Keep `ARCHITECTURE.md` a current-state map of the system a maintainer can read in one sitting —
what exists now, not how it got here. Dispatched at a landing, this skill carries **two distinct
duties on two distinct triggers** — do not conflate them.

## Two duties at a landing

| Duty | Fires when | Output |
|------|-----------|--------|
| **Landing diff** | an **approved architecture delta existed** for the feature — independent of what was built | a built-vs-approved topology report: "built as approved", or the named divergence |
| **`ARCHITECTURE.md` fold** | the built work **changed structure** (components, boundaries, data flow, cross-component contracts) | the updated current-state map |

The triggers are deliberately different. An approved delta that was **descoped or silently dropped**
built nothing structural, so the fold does not fire — but the **diff does**, and it is the only thing
that catches the missing change. A landing with no prior approved architecture fires neither. Run
each duty on its own trigger; **never gate the diff on whether structure was built.**

## Duty 1 — the landing diff

When an approved `architecture.md` delta existed for this feature, run this **first**:

1. Read the approved `architecture.md` (the target signed off at design time) and the built reality
   (the code plus the feature's landed artifacts).
2. Compare box-by-box and arrow-by-arrow: was each approved add / remove / redirect / boundary-move
   actually built? Was anything structural built that the approved target did not carry?
3. Report **built-as-approved** or the **divergence** — each difference named (component or
   interaction, approved vs built). The diff **reports**; it does not silently reconcile.

This is a **built-vs-approved topology diff** taking the approved artifact as input — a capability
this duty owns, not an assumed pre-existing one. It runs whether or not the fold (Duty 2) fires.

## Duty 2 — the `ARCHITECTURE.md` fold

**Fires when** the landing's work changed **components, boundaries, data flow, or cross-component
contracts**. Internal refactors, cosmetic moves, and feature work inside an existing component do
not fire it. **No structural change → no update.**

1. Read the current `ARCHITECTURE.md` — update in place, never wholesale rewrite.
2. Reflect the change where it lives: **Components** (name — responsibility — boundary),
   **Data flow**, **External integrations**. Add, retire, or redraw only what changed.
3. Present tense, current system only. No history narration, no rationale prose — link the
   `DECISIONS.md` row that ruled the change.
4. Keep it one read: a component earns a line, not a chapter; depth lives in the feature
   artifacts.

**Feature-scoped landing → two targets, each on its own trigger.** The feature-root
`architecture.md` (`.mochiko/features/FEAT-XXX/architecture.md`) accumulates the feature's
**approved** delta — firing on **approved-delta-existed** (Duty 1's trigger shape),
independent of whether structure was built: it is the **accumulated feature target**, not
built state, so a descoped or partially built feature's approved delta still lands, and
later landings' deltas seed from it (never from scratch). Repo `ARCHITECTURE.md` keeps
Duty 2's **built-change** trigger and takes only the built change, staying
current-shipped-state per branch. Same in-place-update discipline for both; the
"no structural change → no update" rule above scopes to repo `ARCHITECTURE.md` only. A
non-feature-scoped landing folds into repo `ARCHITECTURE.md` alone.

## The In-flight pointer list

Repo `ARCHITECTURE.md` carries a thin **In flight** list making in-progress structural work
visible from the current branch without duplicating topology — one line per active feature:

```markdown
## In flight
- FEAT-XXX → .mochiko/features/FEAT-XXX/architecture.md
```

- **Added** when plan's architecture sign-off lands (the feature-root target now exists).
- **Removed** at feature close — before removal, the feature-close diff (shipped code vs the
  accumulated feature-root target) is the parked hook; until a feature-close workflow owns
  it, removal alone applies.
- The section is created lazily by the first pointer write — never scaffolded empty.
- Pointer lines only, never topology; the linked feature-root file carries the content.

## Quality checks

- **Diff (Duty 1):** every approved add / remove / redirect / boundary-move is accounted for as built or diverged; nothing structural was built unreported. The diff ran because an approved delta existed, not because structure was built.
- **Dual-target (feature-scoped):** the feature-root `architecture.md` accumulated the approved delta (fires on approved-delta-existed, even when nothing was built); repo `ARCHITECTURE.md` took the built change only where structure was built.
- **In-flight list:** every pointer targets an open feature and resolves; the feature's sign-off added its line, its close removed it; no topology in the list.
- Every named component exists in the code; every pointer resolves.
- No past-tense narration; no rationale restated from the decisions layer.
- A reader new to the repo can place any file in a component from this doc alone.
