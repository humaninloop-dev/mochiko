---
name: authoring-architecture
description: This skill MUST be invoked when authoring or updating `ARCHITECTURE.md` — the knowledge-management module's living system view (components, boundaries, data flow, external integrations) — at a plan or implement landing whose work changed the system's structure. SHOULD also invoke on "update the architecture doc", "system view", "architecture drift", or "does ARCHITECTURE.md still match the code". Records the RESULTING system, present tense, current state only — decision rationale lives in the decisions layer (`DECISIONS.md` + `.mochiko/decisions/`, technique in mochiko:patterns-technical-decisions) and is linked, never restated; feature-scope design artifacts (data-model.md, contracts/) stay in their specs. No structural change → no update.
---

# Authoring Architecture

## Overview

Keep `ARCHITECTURE.md` a current-state map of the system a maintainer can read in one sitting —
what exists now, not how it got here.

## When it fires

A landing whose work changed **components, boundaries, data flow, or cross-component
contracts**. Internal refactors, cosmetic moves, and feature work inside an existing component
do not fire it.

## Procedure

1. Read the current `ARCHITECTURE.md` — update in place, never wholesale rewrite.
2. Reflect the change where it lives: **Components** (name — responsibility — boundary),
   **Data flow**, **External integrations**. Add, retire, or redraw only what changed.
3. Present tense, current system only. No history narration, no rationale prose — link the
   `DECISIONS.md` row that ruled the change.
4. Keep it one read: a component earns a line, not a chapter; depth lives in the feature
   artifacts.

## Quality checks

- Every named component exists in the code; every pointer resolves.
- No past-tense narration; no rationale restated from the decisions layer.
- A reader new to the repo can place any file in a component from this doc alone.
