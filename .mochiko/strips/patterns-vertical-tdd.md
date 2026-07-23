# Strip notes — `skills/patterns-vertical-tdd`

Entry formats: `strips/README.md`. Wave context: the token-reduction build, wave 2 (tasks cluster,
v0.20.0). The task-structuring producer skill, mounted on the `task-architect` producer (the tasks
cluster's authoring skill). D4 (reference-by-ID) edits the story→cycle **mapping** guidance only — the
`**TEST:**` grammar in `references/CYCLE-STRUCTURE.md` and everything the implement cluster consumes are
untouched (the D6a grammar split is the implement wave's, not this one).

## [v0.20.0] D4 — reference-by-ID in the story→cycle mapping (de-modeled examples + added rule)

Token-reduction epic **D4** (reference-by-ID; generalizes `authoring-slices` invariant 6 — cite IDs,
one-line gloss max, never re-quote). **Edited in place — de-modeled examples + one added rule; no strip
of existing behavior:**
- The **"Mapping Stories to Cycles" preamble** gained the explicit rule: the mapping cites `US-#` story
  IDs, never re-quotes story text from `spec.md` (one-line gloss only where a bare ID is unreadable),
  and the story→cycle table is named the **coverage surface** the reviewer grades story coverage
  against.
- The **Quality Checklist** gained one item: the mapping cites story `US-#` IDs (one-line gloss max),
  never re-quoted story text — the Story → Cycle table is the coverage surface. (No second size-guidance
  item: unlike wave-1's data-model / api-contract skills, this skill has no verbose per-block structure
  — e.g. Sensitivity Details — for a compactness rule to attach to, so the cite-by-ID line carries the
  whole D4 concern. Not forced.)
- The **three case examples** (Simple / Split / Merge) were **de-modeled** — each restated the full
  story sentence, modeling the re-quote D4 forbids; rewritten to ID + short gloss so they model the
  rule. The Simple / Split / Merge teaching is intact.

**Case examples — verbatim before→after (the changed lines only; the `→ Cycle …` targets are unchanged):**
- Simple: `US-1: As a user, I can create a task with a title` / `  → Cycle 1: Task creation` → `US-1 (task creation) → Cycle 1: Task creation`
- Split: `US-2: As a user, I can manage my tasks (create, edit, delete, complete)` → `US-2 (task management: create / edit / delete / complete)`
- Merge: `US-3: As a user, I can see task count` / `US-4: As a user, I can see completed count` → `US-3 (task count)` / `US-4 (completed count)`

**Consumer assessment (what was checked — `grep -rn patterns-vertical-tdd` + `grep -rn CYCLE-STRUCTURE`
across `plugins/mochiko`):**
- **Mounting agent — `task-architect` only** (`agents/task-architect.md:49` `skills: patterns-vertical-tdd,
  authoring-slices`; no other agent mounts it). It is the producer that reads this SKILL body; the
  edited mapping section is producer-facing authoring guidance.
- **`review-task-artifacts`** mirrors the producer's rules (its `references/PHASE-CHECKLISTS.md` and
  SKILL.md state the reviewer checks exactly what the producer is taught to author). The mirror row is
  added this wave — a **Story reference discipline** check in the Mapping Review table plus one Key
  Question — keeping producer and reviewer in lock-step. Logged in `strips/review-task-artifacts.md`.
- **`testing-end-user` / `executing-tdd-cycle` / `brownfield-integration`** consume this skill's
  `**TEST:**` grammar and its `[EXTEND]`/`[MODIFY]` marker vocabulary via `references/CYCLE-STRUCTURE.md`
  (grammar-owner references verified at `testing-end-user/SKILL.md:16,47,63,73,263` +
  `testing-end-user/references/TASK-PARSING.md:3,19,56`; marker ownership at
  `brownfield-integration/SKILL.md:10,33`). This wave **does not touch** `CYCLE-STRUCTURE.md`, the
  `**TEST:**` grammar, or the marker table — the mapping-section edits are disjoint from those surfaces,
  so implement-cluster consumption is unaffected.
- **`authoring-slices` / `review-slices` / the `skills/mochiko` router** name the skill only —
  relationship pointers (`authoring-slices/SKILL.md:21,35,149` "downstream", `review-slices/SKILL.md:24`
  boundary note, `skills/mochiko/SKILL.md:85` router index row). None consume the mapping preamble or
  the case examples.

No `command-shape.md` touch this wave → no cross-command re-audit.
