# TRANSFORM (wiring pass) — cluster `tasks`, Wave 2 (shared-file registrations)

**Run:** `/mochiko:transform-cluster tasks` · **Phase 3 (transform), wiring-only pass** · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` (convention-wiring step) · **Date:** 2026-07-01
**Consumes:** `reconcile.md` **Flag 7** (lines ~132–141) — the shared-file registrations the Wave-1 content transforms deliberately deferred to one pass.
**Scope:** exactly three shared files. The Wave-1 content artifacts (`commands/tasks.md`, `agents/task-architect.md`, `skills/patterns-vertical-tdd/`, `skills/validation-task-artifacts/`, `templates/tasks-template.md`, `templates/taskarchitect-report-template.md`) are done and were NOT touched (tasks-template internal content handled by a parallel agent).
**Grading:** NOT self-graded. This artifact + the three edits are handed to `verify-output` (a different agent).

`Applied: (wiring-pass only) — no body/structural move; the six content primitives already carry theirs (see reconcile.md JOB 3).`

---

## Wiring items (Flag 7) — each → applied, with before/after

### Item 1 — `devils-advocate` re-mount: `validation-task-artifacts` stub → live  ·  APPLIED
**File:** `plugins/mochiko/agents/devils-advocate.md`
**Convention-wiring axes:** #1 classification (agent `skills:` list) · #6 single-source (mount the one landed skill, don't restate it).

**1a · frontmatter `skills:` + deferred-stub comment — BEFORE (old L24–29):**
```
color: red
# analysis-specifications + validation-plan-artifacts mounted (specify + plan slices).
# Deferred — NOT dropped — re-mount here when its cluster ports:
#   validation-task-artifacts  (tasks cluster — task-artifact review)
skills: analysis-specifications, validation-plan-artifacts
---
```
**AFTER (now L24–26):**
```
color: red
skills: analysis-specifications, validation-plan-artifacts, validation-task-artifacts
---
```
Deferred-stub comment block deleted (all three comment lines removed — the top line was now stale, since three slices are mounted, not "specify + plan"); `validation-task-artifacts` added to the mount list. YAML re-parses; skills = 3.

**1b · Skills-Available bullet — added after the `validation-plan-artifacts` bullet (now L36):**
```
- **`mochiko:validation-task-artifacts`**: Guidance on the completeness/gap review of task artifacts
  (`task-mapping.md` / `tasks.md`) — vertical-slice integrity, TDD test-first structure, and
  story→cycle→task traceability — with severity classification and the structured verdict;
  the clearing verdict is lead-owned.
```
Existing two bullets kept verbatim.

**Independence check:** mount lands on the **reviewer** (`devils-advocate`), never the producer. `task-architect` (mounts only `patterns-vertical-tdd`) ≠ `devils-advocate` — disjoint agents; produce + grade never co-reside. Guard honored: `validation-task-artifacts` is NOT on `task-architect`.
**Realized tag:** re-mount `kept-but-rebind` (stub → live).

---

### Item 2 — Router registration: NEW "Tasks cluster" section + entry-point + agents rows  ·  APPLIED
**File:** `plugins/mochiko/skills/mochiko/SKILL.md` (the user-invoked router)
**Convention-wiring axis:** #2 router registration (a primitive absent from the router fails discoverability by construction). Verified precondition: the router had **no** Tasks section before this edit.

**2a · New skill table — inserted after the plan-cluster note, before `### Entry point` (now L85–91):**
`### Tasks cluster (model-invoked — auto-reached during a /mochiko:tasks run)` with four rows + when-to-reach-each:
- `patterns-vertical-tdd` — producer's vertical-slice + TDD structuring skill (teaches the craft; fills `tasks-template`).
- `validation-task-artifacts` — reviewer's task-artifact completeness/gap review; **carries the boundary note**: grades **task artifacts** vs `validation-plan-artifacts` grading **plan artifacts** — disjoint artifacts, disjoint checks.
- `tasks-template` (template) — the `tasks.md` deliverable skeleton.
- `taskarchitect-report-template` (template) — the producer's per-round non-verdict report (gated-in per Flag 2; the template landed, so registered).

**2b · Entry-point row — added after the `/mochiko:plan` row (now L100):**
`| /mochiko:tasks | … structure an accepted plan into implementation tasks — task-mapping.md + tasks.md — via the task-architect→devils-advocate loop with a human acceptance gate on tasks.md; next step /mochiko:implement |`

**2c · Agents section — `devils-advocate` row UPDATED (now L108):**
BEFORE: "specify-cluster critic **and** plan-cluster completeness reviewer … (skills: analysis-specifications, validation-plan-artifacts)"
AFTER: "specify-cluster critic, plan-cluster completeness reviewer, **and** tasks-cluster **task-artifact reviewer** (vertical-slice integrity, TDD structure, traceability over `task-mapping.md`/`tasks.md`) … (skills: analysis-specifications, validation-plan-artifacts, **validation-task-artifacts**)" — cross-workflow, skills list kept in sync with the agent frontmatter (Item 1).

**2d · Agents section — `task-architect` row ADDED (now L110), between `technical-analyst` and `validator`:**
`| task-architect | tasks-cluster PRODUCER — structures an accepted plan into implementation tasks (task-mapping.md story→cycle mapping + slice rationale, then tasks.md cycle-based TDD task list); never grades its own output (skills: patterns-vertical-tdd) |`

**Realized tag:** router registrations = wiring (discoverability floor). Markdown tables intact; new section mirrors the plan-cluster section's shape.

---

### Item 3 — `plugin.json`: register the new agent  ·  APPLIED
**File:** `plugins/mochiko/.claude-plugin/plugin.json`
**Convention-wiring axis:** #2 registration (manifest). Skills glob-discover (`"skills": "./skills/"` → P3, P4 auto-register); commands glob-discover (`"commands": "./commands/"` → `tasks.md` auto-registers); templates need no manifest key (P5, `taskarchitect-report-template`). **Agents are an explicit array** → the one manual add.

**BEFORE (agents array tail):**
```
    "./agents/devils-advocate.md",
    "./agents/technical-analyst.md"
  ]
```
**AFTER:**
```
    "./agents/devils-advocate.md",
    "./agents/technical-analyst.md",
    "./agents/task-architect.md"
  ]
```
Added `"./agents/task-architect.md"` exactly as `./agents/technical-analyst.md` is listed. No skill/command/template entries added (they auto-discover). **JSON re-validated** (`json.load` OK); all 7 agent paths resolve to files.
**Realized tag:** manifest registration = wiring.

---

## Deferred / no-op wiring items (Flag 7.4, 7.5) — recorded, correctly out of scope for this pass
- **7.4 — qa `**TEST:**` runtime classification → `defer-by-reference` (implement/qa).** Not a shared-file edit; the 4 sibling-name tokens were `dropped` inside the Wave-1 content artifacts (P2/P3), not here. No wiring action in this pass.
- **7.5 — boundary vs `executing-tdd-cycle` (implement, deferred) → forward-watch.** No in-cluster resolution; nothing to wire this run. Recorded for the implement port.

Neither touches the three shared files; both are content/forward-watch concerns already homed in reconcile.md — noted so nothing is dropped.

---

## Dangling-mount audit — every registered skill/agent/template resolves to a landed file

| Registration (this pass) | Resolves to | On disk |
|---|---|---|
| `devils-advocate` mount `validation-task-artifacts` | `skills/validation-task-artifacts/SKILL.md` | OK |
| router skill `patterns-vertical-tdd` | `skills/patterns-vertical-tdd/SKILL.md` | OK |
| router skill `validation-task-artifacts` | `skills/validation-task-artifacts/SKILL.md` | OK |
| router template `tasks-template` | `templates/tasks-template.md` | OK |
| router template `taskarchitect-report-template` | `templates/taskarchitect-report-template.md` | OK |
| router entry-point `/mochiko:tasks` | `commands/tasks.md` | OK |
| router agent + `plugin.json` `task-architect` | `agents/task-architect.md` | OK |

**No dangling mounts.** `plugin.json` valid JSON; `devils-advocate.md` frontmatter valid YAML (skills = 3).

---

## Realized trace (wiring responsibilities → final tag)
- `devils-advocate` `validation-task-artifacts` re-mount (stub → live) → **`kept-but-rebind`** (deferred stub resolved; reviewer agent; independence preserved)
- deferred-stub comment block (3 lines) → **`dropped + reason`** (bookkeeping for a now-resolved deferral; the live `skills:` line is self-describing)
- router: Tasks-cluster skill section (4 entries) + `/mochiko:tasks` entry-point + `task-architect` agent row + `devils-advocate` row update → **wiring / discoverability floor** (every new primitive now discoverable in the router)
- router boundary note (`validation-task-artifacts` vs `validation-plan-artifacts`) → **wiring** (mutual-boundary record, per Flag 4's disjoint-not-merge finding)
- `plugin.json` `task-architect` agent registration → **wiring / manifest**

**Open items:** NONE. All Flag 7 shared-file registrations applied; the two `defer-by-reference` items (7.4, 7.5) are correctly out of scope for this pass.
**Next:** `verify-output` (independent `validator`, a different agent) grades the three edited files + this trace.
