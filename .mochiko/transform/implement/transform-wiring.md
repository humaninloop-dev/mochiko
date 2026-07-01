# Phase 3 Wave-2 — cluster-wide wiring pass — cluster `implement`

**Run:** `/mochiko:transform-cluster implement` · **Skill:** `mochiko:transform-recipes` (step 6 single-source + registration) · **Date:** 2026-07-01
**Scope:** the SHARED-file wiring all 6 primitive transforms deferred (to avoid concurrent edits). This pass **wires + verifies only** — it does NOT re-transform any primitive body, and it does NOT grade them (that is Phase-4 `verify-output`, run independently).
**Consumes:** `reconcile.md` (finalized dispositions + rehome map) + the 6 landed artifacts.

The cluster's 6 landed artifacts:
- command `plugins/mochiko/commands/implement.md`
- agents `plugins/mochiko/agents/staff-engineer.md`, `plugins/mochiko/agents/qa-engineer.md`
- skills `plugins/mochiko/skills/{executing-tdd-cycle,testing-end-user,brownfield-integration}/`
- `strategy-implementation` — **DROPPED** (P7 dissolved; no artifact — reconcile §Job 1.4 / §2).

---

## Part A — What was registered

### A1. `plugins/mochiko/.claude-plugin/plugin.json`

Read first to learn the discovery model:
- `"commands": "./commands/"` — **directory glob → auto-discovered** (the `implement` command needs no explicit entry).
- `"skills": "./skills/"` — **directory glob → auto-discovered** (the 3 new skills need no explicit entries).
- `"agents": [ … ]` — **explicit array → NOT auto-discovered** (new agents MUST be listed).

**Registered (only what is not auto-discovered):** appended the two new agents to the `agents` array, matching the existing `"./agents/<name>.md"` entry style:
- `"./agents/staff-engineer.md"`
- `"./agents/qa-engineer.md"`

Agents array grew 7 → 9. No skills/commands entries added (they are globs). JSON re-validated (parses clean).

### A2. `plugins/mochiko/skills/mochiko/SKILL.md` (the user-invoked router / library index)

The router indexes cluster skills in per-cluster tables, user-invoked commands in an **Entry point** table, and agents in an **Agents** table (the tasks/plan/specify/setup pattern). Matched that format exactly:

1. **New section "Implement / execute cluster"** (inserted after the Tasks cluster section, header `(model-invoked — auto-reached during a /mochiko:implement run)`), indexing the 3 model-invoked skills with when-to-reach-each guidance:
   - `executing-tdd-cycle` — runtime red/green/refactor cycle **EXECUTION** + task parsing + targeted rework + `cycle-report.md` schema; **boundary hint included**: *for design-time cycle structuring / test-first ordering, that's `patterns-vertical-tdd` (Tasks)*.
   - `testing-end-user` — runtime verification against real infrastructure + evidence + quality-gate exit-code classification + **CLI/GUI/SUBJECTIVE** confidence classification + report/checkpoint; noted *mounted on qa only, never on staff*.
   - `brownfield-integration` — safely **EXTEND/MODIFY** existing code at implement-time (read-before-write, interface preservation, conflict detection); noted the `[EXTEND]`/`[MODIFY]` marker vocabulary is owned by `patterns-vertical-tdd`.
2. **Entry point table** — added the `/mochiko:implement` user-invoked row: *turn an accepted `tasks.md` into working, verified code — cycle-by-cycle (foundation → feature) via the staff-engineer→qa-engineer producer→verifier loop, confidence-based per-cycle gate + named final-acceptance gate.*
3. **Agents table** — added `staff-engineer` (implement PRODUCER; skills: executing-tdd-cycle, brownfield-integration; verification skill never mounted here) and `qa-engineer` (implement VALIDATOR; skills: testing-end-user; grades a different agent's work, mounts no producer skill), placed with the other cluster producers, above the generic `validator`.

Convention-wiring step 6 (single-source) note: no doctrine was restated in the router; each entry points at the primitive that owns the procedure. The command body already references `loop-discipline` / `agent-dispatch` / the `workflow-contract` artifact rather than inlining them (a Phase-4 altitude concern, not re-checked here).

---

## Part B — Verification checks (each with result + grep counts)

### B1. Mounts resolve — **PASS**
Agent `skills:` frontmatter vs. skill directories on disk:
- `staff-engineer` → `executing-tdd-cycle, brownfield-integration` — both dirs present (`skills/executing-tdd-cycle/SKILL.md`, `skills/brownfield-integration/SKILL.md`). OK
- `qa-engineer` → `testing-end-user` — dir present (`skills/testing-end-user/SKILL.md`). OK
- No agent mounts a skill that does not exist (swept all 9 agents' `^skills:` lines; every mount resolves to a real skill dir).

### B2. Independence intact — **PASS**
- `grep -rln testing-end-user plugins/mochiko/agents/` → **`qa-engineer.md` ONLY** (count = 1 file).
- `staff-engineer` skills line = `executing-tdd-cycle, brownfield-integration` — contains **no** `testing-end-user` and **no** `verify-*`.
- Only `verify-*` mount anywhere = `verify-output` on `validator.md` (the transform-cluster generic grader) — expected, not on staff.
- Verdict: produce + grade are never co-mounted on one agent. Independence holds by construction.

### B3. No dangling / dissolved refs — **PASS** (all counts = 0)
- `grep -ri "strategy-implementation" plugins/mochiko/` → **COUNT=0** (P7 fully dissolved; re-run after the router edits, still 0).
- `grep -rin "humaninloop:"` across the 6 new artifacts → **COUNT=0** (all namespaces rebound to `mochiko:`).
- Kernel/DAG token scan across the 6 new artifacts, each **COUNT=0**: `hil-dag`=0, `catalog`=0, `INV-`=0, `.workflow/`=0, `state.analyst` (regex, case-insensitive)=0.

### B4. Router completeness — **PASS**
Every new primitive is indexed in `skills/mochiko/SKILL.md` (occurrence counts): `mochiko:implement` ×3, `staff-engineer` ×4, `qa-engineer` ×3, `executing-tdd-cycle` ×2, `testing-end-user` ×2, `brownfield-integration` ×2. "Implement / execute cluster" header present.
Command team references all resolve to landed artifacts:
- `mochiko:staff-engineer` → `agents/staff-engineer.md` OK
- `mochiko:qa-engineer` → `agents/qa-engineer.md` OK
- `executing-tdd-cycle` / `testing-end-user` / `brownfield-integration` → skill dirs present OK
- Doctrine refs also resolve: `loop-discipline` (`skills/loop-discipline/SKILL.md`), `workflow-contract` (`templates/workflow-contract.md`), `agent-dispatch` (`templates/agent-dispatch.md`).

### B5. plugin.json integrity — **PASS**
Valid JSON after edit; `agents` array = 9 entries incl. the 2 new ones; `commands`/`skills` left as globs (correctly not enumerated).

---

## Result

All wiring applied; **all five verification checks PASS**. The `implement` cluster is registered (2 agents in plugin.json; command + 3 skills + 2 agents in the router) and discoverable. No dissolved/kernel refs, independence intact, all mounts and team references resolve.

**Not done here (by design):** grading the 6 primitive bodies — that is Phase-4 `verify-output`, run by an independent validator.
