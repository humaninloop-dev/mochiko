# VERIFY (independent) — cluster integration — `implement`

**VALIDATE:** cluster-level integration wiring of the ported `implement` cluster
**Checklist run:** `mochiko:verify-output` — cluster-conformance portion (convention 2 discoverability, convention 4 composition, convention 5 producer↔validator pairing, kernel-free, no-dangling). Per-primitive body conformance (conventions 1/3/6/8) + Part-B trace audit are out of scope here (they are the separate per-primitive `verify-output` gate).
**Independence:** graded by an agent that did NOT author the wiring. The Wave-2 wiring agent's `transform-wiring.md` claims were audited against the real files, not trusted.
**Date:** 2026-07-01

**Evidence read (this run):**
- `plugins/mochiko/.claude-plugin/plugin.json` (Read + `json.load` parse)
- `plugins/mochiko/skills/mochiko/SKILL.md` (Read + occurrence greps)
- `plugins/mochiko/commands/implement.md` (Read + ref-resolution greps)
- `plugins/mochiko/agents/staff-engineer.md`, `plugins/mochiko/agents/qa-engineer.md` (Read + `^skills:` audit)
- `plugins/mochiko/skills/{executing-tdd-cycle,testing-end-user,brownfield-integration}/SKILL.md` (Read frontmatter + dir listing)
- `.mochiko/transform/implement/transform-wiring.md` (audited claim)

---

## Conformance

### 1. Router completeness (convention 2) — **PASS**
`skills/mochiko/SKILL.md` indexes all 6 new primitives with when-to-reach guidance:
- **`/mochiko:implement`** Entry-point row — line 108, with a full when-to-reach clause ("turn an accepted `tasks.md` into working, verified code … staff-engineer→qa-engineer producer→verifier loop … named final-acceptance gate").
- **"Implement / execute cluster" section** present — line 93 (`model-invoked — auto-reached during a /mochiko:implement run`), indexing the 3 model-invoked skills:
  - `executing-tdd-cycle` (line 96) **with the required boundary hint**: *"for design-time cycle *structuring* / test-first ordering, that's `patterns-vertical-tdd` (Tasks)"*.
  - `testing-end-user` (line 97) with *"mounted on qa **only**, never on staff"*.
  - `brownfield-integration` (line 98).
- **Agents** — `staff-engineer` (line 119, "implement-cluster PRODUCER") + `qa-engineer` (line 120, "implement-cluster VALIDATOR"), each with role guidance and skills list.
- Occurrence counts: `mochiko:implement` ×3 · `staff-engineer` ×4 · `qa-engineer` ×3 · `executing-tdd-cycle` ×2 · `testing-end-user` ×2 · `brownfield-integration` ×2 (matches the wiring agent's claim).

### 2. plugin.json (convention 2) — **PASS**
`python3 json.load` → **JSON PARSES: OK**. `agents` is a list of **9** entries; `'./agents/staff-engineer.md'` and `'./agents/qa-engineer.md'` both present (last two). `commands` = `'./commands/'` and `skills` = `'./skills/'` left as directory globs (correctly NOT enumerated) → the 3 new skill dirs (each has `SKILL.md`) + `commands/implement.md` auto-register.

### 3. Independence structure (convention 5, cluster level) — **PASS**
- `staff-engineer` skills = `{executing-tdd-cycle, brownfield-integration}` (line 37); `qa-engineer` skills = `{testing-end-user}` (line 35). **Intersection = ∅.**
- `grep -rln testing-end-user plugins/mochiko/agents/` → **`qa-engineer.md` ONLY** (1 file). The verification skill is mounted on the validator alone.
- `verify-*` anywhere in agents → **only `validator.md`** (the transform-cluster generic grader — a different cluster); **not** on `staff-engineer`. staff carries zero validator skill.
- Producer ≠ validator (different agent + different skill). Single-reviewer shape (`specify`/`tasks` pairing, not `plan`'s two-form) — the command even states "No G2 — implement has a single verifier." Independence is structural, not self-declared.

### 4. Mounts resolve (convention 4) — **PASS**
- Both agents' `skills:` entries resolve to real landed dirs: `executing-tdd-cycle/SKILL.md`, `brownfield-integration/SKILL.md`, `testing-end-user/SKILL.md` all present.
- Command's 5 team references all resolve: `staff-engineer`(×5), `qa-engineer`(×6), `executing-tdd-cycle`(×2), `testing-end-user`(×3), `brownfield-integration`(×2) → all point to landed artifacts.
- Doctrine refs resolve: `loop-discipline` → `skills/loop-discipline/SKILL.md`; `workflow-contract` → `templates/workflow-contract.md`; `agent-dispatch` → `templates/agent-dispatch.md`.
- All `mochiko:` refs in the command resolve to real primitives (`loop-discipline`, `qa-engineer`, `setup`, `staff-engineer`, `tasks`).

### 5. No dangling / dissolved refs — **PASS** (all counts = 0)
- `grep -ri "strategy-implementation" plugins/mochiko/` → **COUNT=0** (P7 fully dissolved, incl. after router edits).
- `grep -rin "humaninloop:"` across the 6 artifacts → **COUNT=0** (namespaces rebound to `mochiko:`).
- Kernel/DAG/dissolved token scan across the 6 artifacts, each **COUNT=0**: `hil-dag`, `catalog`, `INV-`, `.workflow/`, `state.analyst`, `State-Analyst`, `capability catalog`, `DAG` (case-insensitive).

---

## Audit of the wiring agent's claims (`transform-wiring.md`)
Every claim independently re-derived and confirmed: agents array 7→9 (now 9, both new present); router occurrence counts exact match; `testing-end-user` on qa only; all mounts + team + doctrine refs resolve; all dissolved/kernel greps = 0; JSON parses. No claim was taken on trust; each was re-run.

## Non-blocking observations (not failures)
- The command references `deliberate-shortcut-ledger` once (line 35) as a forward-looking deferral concept (parallel-cycle execution pending dogfooding). It is a concept/ledger reference, not a skill/template/agent mount, and not a dissolved/kernel token — outside the resolution set and not a dangling-ref failure. Flagged only for cluster-owner awareness.
- The regex co-mount probe (`staff-engineer` + `testing-end-user` on one line) hit lines 8 and 19, both of which state the **opposite** ("the verification skill is **never** mounted on staff") — false positives describing the disjoint pairing, not a co-mount.

---

## VERDICT: **PASS**

The `implement` cluster is correctly wired and discoverable at the cluster level: 2 agents registered in `plugin.json` (valid JSON); command + 3 skills + 2 agents indexed in the router with when-to-reach guidance and the required boundary hint; producer/validator independence holds by construction (disjoint agents + disjoint skills, verification skill on qa only); every mount and team/doctrine reference resolves; P7 (`strategy-implementation`) fully dissolved; no HIL-namespace, kernel, DAG, or State-Analyst tokens survive in the artifacts.

**Issues requiring fix:** none (cluster-conformance scope).

**Out of scope (still owed by the separate gate):** the per-primitive body grade — conventions 1 (classification), 3 (trigger phrases), 6 (sound-loop placement / filled contract), 8 (command altitude / single-source), and the Part-B responsibility-trace audit — run per primitive by `verify-output`.
