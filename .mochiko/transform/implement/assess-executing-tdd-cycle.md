# Assessment — P4 `executing-tdd-cycle` skill

**Run:** `transform-cluster implement` · **Producer:** `mochiko:transform-producer` · **Date:** 2026-07-01
**Source:** `human-in-loop/plugins/humaninloop/skills/executing-tdd-cycle/` (SKILL.md 164 ln + 3 refs: CYCLE-REPORT-FORMAT.md, TASK-PARSING.md, TDD-ANTI-RATIONALIZATION.md)
**Role:** `staff-engineer`'s (P2) core RUNTIME execution procedure — the producer half of the implement workflow's produce→verify loop.

---

```
ASSESSMENT: executing-tdd-cycle
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone
              (+ reconcile flag on the retry/fix routing seam — cross-primitive P1+P2+P7)
```

## Triage (all three tripped → full lens)

1. **Orchestration-coupled? YES.** Content-coupled to the brain in specific lines: cycle-report consumers named as `checkpoint gate` / `State Analyst` / `carry_forward` (DAG artifacts); "When NOT to Use" delegates to the "analysis layer" and "graph operations layer" (= State-Analyst + hil-dag); retry/fix framed as "When dispatched…". The procedure was driven by the DAG-Supervisor.
2. **Multi-responsibility / fans out? YES.** Holds ≥5 responsibilities (execution sequence, runtime task-parsing, cycle-report generation, retry HOW, fix HOW, anti-rationalization) and feeds multiple consumers (the lead's cycle gate, qa verification, the next cycle).
3. **Non-machine-checkable artifact? YES.** Emits `cycle-report.md` (honest "What Was Done" / "Decisions Made" is judgment), and the TDD execution itself is model judgment.

## Lens (weighted for a PLAYS-a-role skill)

**1 — Orchestration test.** Separate the two couplings:
- *Content-coupling* (in the body, fixable by edit): the cycle-report consumer references (`checkpoint gate`, `State Analyst`, `carry_forward`), and the two "When NOT to Use" DAG lines (`analysis layer`, `graph operations layer`, `DAG state / pass lifecycle`). These rebind/drop — kernel-free.
- *Orchestration-coupling* (driven from outside): the DAG-Supervisor decided WHEN to dispatch this skill and WHEN to loop on FAIL. That inherits to the **thin lead (P1)**, not to this skill. The skill's own body survives without the brain — it is a self-contained runtime craft procedure.

**2 — Role (two altitudes).** Skill-role = **consumed-procedure AND emits-artifact** (a runtime execution procedure staff-engineer consumes; emits `cycle-report.md`). Team-role conferred on its caller = **PRODUCER** (produces code + report; never grades).

**3 — Independence.** CLEAN. This skill makes staff-engineer (P2) a producer; grading is qa-engineer (P3) via `testing-end-user` (P5) — a different agent, a different skill. No produce+grade on one agent. **Guard for the port:** the skill must NOT acquire a grading responsibility — running quality gates / the `**TEST:**` real-infra verification is explicitly qa's (`When NOT to Use` line 26; TASK-PARSING.md line 95). Keep that exclusion.

**4 — Verdict-sink / loop-driver.** `cycle-report.md` is consumed by the HIL DAG "checkpoint gate"; on FAIL the DAG-Supervisor drove retry/fix. In mochiko: the report's verdict-sink **rebinds** to the qa verification + the thin lead's cycle gate; the FAIL-loop **routing** moves to the lead (contract §1 constraints b/c/d/e). The skill keeps only the *execution* of one report / one rework.

**5 — Sibling / overlap.** Two sibling relations:
- vs **`patterns-vertical-tdd`** (design-time): boundary holds — see finding below. No merge.
- **Trigger collision:** both descriptions currently claim `red green refactor` / `TDD` (patterns-vertical-tdd MUST list has "red-green-refactor"; P4 MUST list has "red green refactor"). De-collide in the wiring pass: design-time *structuring / test-first ordering* → patterns-vertical-tdd; runtime *executing the cycle* → this skill. (Phrasing de-collision, not a structural move.)
- Sibling-skill reference to **`brownfield-integration`** (P6) by skill-name is proper (P6 lands in this same cluster) — keep.

**6 — Coupling audit.** No `.humaninloop/`/`.workflow`/absolute paths in the body (confirmed by grep) — `tasks.md` / `cycle-report.md` are feature-dir-relative → rebind the feature dir to `.mochiko/specs/<feature>/`. Upstream prereq: `tasks.md` must exist (produced by the tasks run) — an **entry-gate handoff owned by the lead** (contract §1 (f)), not this skill. Determinism boundary: TDD execution is model judgment; running the test command to read pass/fail is deterministic — fine as-is.

**7 — Conventions + loop placement.** Model-invoked, agent-consumed (default classification; no `disable-model-invocation`). Description triggers are RFC-2119-graded but **mis-scoped** — "when the user says 'execute cycle'…" is wrong for an agent-consumed runtime skill (wiring-pass item 3: agent-consumed skills describe the *work context*, not user utterances). This is a producer skill — it supplies the loop's producer half; done-condition / independent-validation / human-gate belong to the loop (P1), not here.

---

## BOUNDARY FINDING — `patterns-vertical-tdd` (design-time) ↔ `executing-tdd-cycle` (runtime)

**The boundary HOLDS. No duplicated substance. Resolvable alone — no reconcile flag on the boundary.**

| Axis | Result |
|------|--------|
| `grep cycle-report` in `patterns-vertical-tdd` | **0 matches** ✓ — the cycle-report is a runtime artifact owned HERE, not leaked into the design-time skill. |
| `**TEST:**` grammar ownership | **Owned by `patterns-vertical-tdd`** (SKILL.md lines 113/127/136/218/227/332 + CYCLE-STRUCTURE.md), **consumed by `testing-end-user` (P5)** at runtime. **Absent from P4** entirely (0 matches across P4's SKILL + 3 refs). P4 executes only the test/implement/refactor tasks (TN.1–TN.3); the final `**TEST:**` real-infra task (TN.4) is qa's (P5). **The overlap question the run flagged resolves cleanly — P4 neither defines nor references the grammar, so nothing to reconcile here.** |
| red/green/refactor concept | Shared *discipline*, different *altitude*, no restated substance. patterns-vertical-tdd = **ORDERING** (task order in `tasks.md`: TN.1 write failing test → TN.2 implement → TN.3 refactor → TN.4 TEST). P4 = **EXECUTION** (write the test file, run it, verify it fails *for the right reason* not a ModuleNotFoundError, minimal green, cycle-only refactor). Distinct work. |
| shared discipline banner | Both carry "*Violating the letter of the rules is violating the spirit of the rules.*" — **allowed** (context.md: "shared discipline banner OK"). |
| anti-rationalization prose | **Distinct altitude, not duplicated.** patterns-vertical-tdd's table is *structuring*-scoped (horizontal-vs-vertical, "foundation doesn't need tests", prototyping). P4's Red Flags + TDD-ANTI-RATIONALIZATION.md are *execution*-scoped ("the test already passes so I'll skip the red run", "I'll refactor existing code while I'm here", EXTEND-rewrite, "the checkpoint will catch it"). One line ("write tests after code") is thematically shared but each serves its own altitude — no restatement. |

**Recommended wiring nicety (port-with-edits, not a flag):** add a `When NOT to Use → structuring the cycles / the tasks.md order = patterns-vertical-tdd (design-time)` cross-reference, mirroring the pointer patterns-vertical-tdd already carries downward ("Executing the cycles … is downstream"). Restores boundary symmetry and single-sources the discipline.

---

## Responsibility trace (every responsibility tagged — this is the done-condition)

**KEPT — runtime execution craft (survives without the brain):**
- Cycle Execution Sequence (Parse → Red → Green → Refactor → Mark-complete → Write-report), strict order → **kept**
- Red/Green/Refactor phase *mechanics* (run test; verify it fails for the right reason; minimal green; refactor cycle-only; re-run) → **kept**
- Runtime task-parsing (TASK-PARSING.md: task pattern, cycle = first with unchecked tasks, file-path extraction, `[EXTEND]`/`[MODIFY]` markers, multi-line rules, checkpoint & quality-gate *recognition*) → **kept** (minus agent-name decouple)
- `cycle-report.md` **format** — frontmatter schema (incl. `attempt` increments-on-retry, `cycle: "fix"`) + prose sections → **kept** (runtime artifact OWNED HERE; minus consumer-reference rebind)
- TDD anti-rationalization prose (Red Flags list + TDD-ANTI-RATIONALIZATION.md table) → **kept** (execution-scoped; distinct from patterns-vertical-tdd's structuring rationalizations)
- retry **HOW-craft**: trace a checkpoint failure to its task → re-open only that task (`[x]`→`[ ]`) → TDD only the re-opened tasks → leave passing code untouched → write updated-attempt report → **kept** (decoupled from "dispatched" framing)
- fix **HOW-craft**: trace a reported failure → fix scoped strictly to it, unconstrained by cycle boundaries → write `cycle: fix` report → **kept** (decoupled from "dispatched" framing)
- `[EXTEND]`/`[MODIFY]` runtime handling + `brownfield-integration` (P6) invocation → **kept** (proper sibling-skill reference, same cluster)
- shared discipline banner → **kept** (references patterns-vertical-tdd's discipline, doesn't restate substance)

**KEPT-BUT-REBIND — content-coupling fixed by edit (kernel-free):**
- `cycle-report.md` consumer references: `checkpoint gate` (SKILL.md 86), `State Analyst` + `carry_forward` (SKILL.md 87; CYCLE-REPORT-FORMAT.md 3, 51, 65) → **kept-but-rebind** consumers to the thin lead + qa verification; drop DAG/State-Analyst/carry_forward vocab (format stays)
- feature-dir artifacts (`tasks.md`, `cycle-report.md`) → **kept-but-rebind** to `.mochiko/specs/<feature>/`
- description trigger phrases ("when the user says 'execute cycle'…") → **kept-but-rebind** to runtime-work-context, RFC-2119, de-collided from patterns-vertical-tdd (wiring item 3)

**DROPPED — kernel-only, no mochiko referent:**
- "Modifying DAG state or pass lifecycle — handled by the graph operations layer" (SKILL.md 29) → **dropped + reason:** DAG state / pass lifecycle do not exist in mochiko (kernel-free constraint). The intent ("this skill does not manage orchestration state") is preserved role-neutrally, but the DAG referent is removed.
- "Parsing/evaluating checkpoint/validation reports — handled by the analysis layer" (SKILL.md 27) → **kept-but-rebind** (keep the exclusion — this skill doesn't evaluate reports — but rebind "analysis layer" → the lead reads reports; no State-Analyst).

**DECOUPLE (port-with-edits, wiring item 5) — agent-name / dispatch scrub:**
- agent names `Staff Engineer` / `qa-engineer` across SKILL + TASK-PARSING + CYCLE-REPORT-FORMAT + TDD-ANTI-RATIONALIZATION → **kept-but-rebind** to role ("the implementer" / "the verifier") or describe the work; independence stated by ROLE, not agent-name
- "When **dispatched** after a checkpoint / final-validation failure" (SKILL.md 91, 102) → **decouple** the deny-list token "dispatched"; reframe as craft ("when reworking a reported failure…"); the WHEN is not this skill's to decide

**MOVED-TO-LEAD / DEDUPE — retry/fix ROUTING (the seam — see flag):**
- retry/fix **routing/WHEN**: max-3-attempts-per-cycle, when to retry vs. enter fix mode, max-3-fix-passes, convergence-stall surfacing (contract §1 (c)(d)(e)) → generic bounded-iteration/round-cap/no-progress mechanics **dedupe → `loop-discipline`**; the THIS-workflow parameters (3/cycle, 3 fix passes, stall-surface) **moved-to-lead (P1)**.
  *Note:* this routing is **not currently in P4's body** — HIL kept it in the DAG-Supervisor. P4 owns only the mechanics of one rework + the report's `attempt`/`cycle:fix` fields. So the "move" is really a confirmation that routing stays out of P4 and lands on the lead — not an extraction from P4.

---

## Reconcile flag (1) — retry/fix routing seam (cross-primitive: P1 + P2 + P7)

**Signal:** the retry/fix concept is spread across four primitives and the WHEN↔HOW seam must be drawn once, coherently, so nothing is double-owned or dropped:
- **P4 (this skill)** — owns retry/fix **HOW-craft** (trace→re-open→TDD-only-reopened→updated report; scoped fix + `cycle: fix`). Proposed: **kept**, decoupled from "dispatched" framing.
- **P1 (`implement` command)** — owns retry/fix **routing/WHEN** (max-3-attempts, when-retry-vs-fix, max-3-fix-passes, convergence-stall). Proposed: **moved-to-lead**, generic mechanics **dedupe→`loop-discipline`**.
- **P2 (`staff-engineer` agent)** — its "Two Execution Modes / Cycle Mode / Fix Mode" persona section (contract §4 decoupling hotspot) must **decouple** without re-stating P4's HOW-craft (avoid double-ownership).
- **P7 (`strategy-implementation`)** — its "execute-then-verify / targeted retry / fix pass / implementation escalation" patterns **dissolve into `loop-discipline`** + lead; must not leave a second copy of the routing or the craft.

**Why flagged (not resolved solo):** the exact seam placement depends on P2's fix-mode decoupling and P7's dissolution landing coherently — a cross-primitive call `reconcile-cluster` must make with full context. **Proposed resolution (for reconcile to confirm):** HOW-craft stays in P4 (decoupled); routing→P1 lead with generic mechanics deduped to `loop-discipline`; P2's "modes" decouples to reference the craft, not restate it; P7's patterns land in `loop-discipline`/lead. Zero double-ownership, zero drop.

**No other open flags.** The `patterns-vertical-tdd` boundary and the `**TEST:**`-grammar-ownership question are **resolved in-assessment** (boundary holds; grammar owned by patterns-vertical-tdd, consumed by testing-end-user, absent from P4).
