---
name: authoring-architecture
description: This skill MUST be invoked when authoring or updating `ARCHITECTURE.md` — the knowledge-management module's living system view (components, boundaries, data flow, external integrations) — at a plan or implement landing whose work changed the system's structure. MUST also run the **landing diff** when dispatched at a landing where an approved architecture delta existed for the feature — diffing the built topology against the approved target and reporting built-vs-approved divergence — which fires on approved-delta-existed even when nothing structural was built, distinct from the `ARCHITECTURE.md` fold (which fires only on built structural change). SHOULD also invoke on "update the architecture doc", "system view", "architecture drift", "built vs approved architecture", or "does ARCHITECTURE.md still match the code". Records the RESULTING system, present tense, current state only — decision rationale lives in the decisions layer (`DECISIONS.md` + `.mochiko/decisions/`, technique in mochiko:patterns-technical-decisions) and is linked, never restated; feature-scope design artifacts (data-model.md, contracts/) stay in their specs. No structural change → no `ARCHITECTURE.md` update.
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

## Quality checks

- **Diff (Duty 1):** every approved add / remove / redirect / boundary-move is accounted for as built or diverged; nothing structural was built unreported. The diff ran because an approved delta existed, not because structure was built.
- Every named component exists in the code; every pointer resolves.
- No past-tense narration; no rationale restated from the decisions layer.
- A reader new to the repo can place any file in a component from this doc alone.
