# Transform Run Context — cluster `tasks`

**Command:** `/mochiko:transform-cluster tasks` · **Lead/referee:** the transform-cluster supervisor · **Started:** 2026-07-01

## User request

> i want to transform the tasks cluster from human in loop. ask any questions you may have during the process

## Intake decisions (Phase 0 human gate)

- **Scope:** Core-only (6 primitives). The roadmap track (`authoring-roadmap` + `evolution-roadmap-template`) stays deferred; the task-architect's brownfield `evolution-roadmap.md` / `[GAP:XXX]` read is a documented stub, not a live mount.
- **Gate placement:** Gated dispositions + escalations (the `loop-discipline` default).

## Resolved primitive list (6 — HIL sources under `human-in-loop/plugins/humaninloop/`)

| # | HIL source | Kind | Assess status | Disposition (proposed) | Verdict |
|---|-----------|------|---------------|------------------------|---------|
| P1 | `commands/tasks.md` | command | ✅ assessed | `redesign × absorb-into-lead` (thin) | — |
| P2 | `agents/task-architect.md` | agent | ✅ assessed | `port-with-edits × standalone` (heaviest) | — |
| P3 | `skills/patterns-vertical-tdd/` (+ `CYCLE-STRUCTURE.md`, `SLICE-IDENTIFICATION.md`) | skill | ✅ assessed | `port-with-edits × standalone` | — |
| P4 | `skills/validation-task-artifacts/` (+ `ISSUE-TEMPLATES.md`, `PHASE-CHECKLISTS.md`) | skill | ✅ assessed | `port-with-edits × standalone` | — |
| P5 | `templates/tasks-template.md` | template | ✅ assessed | `port-with-edits × standalone` | — |
| P6 | `templates/tasks-context-template.md` | template | ✅ assessed | `drop × absorb-into-lead` (no artifact) | — |

**Cross-cutting flags surfaced by assessment (for reconcile):** RQ-A artifact-shape (all 6; P1/P5 rec = keep 2 artifacts) · producer-report home inline-vs-template (P1/P2/P6) · scaffolding dedupe P4 report-templates → `advocate-report-template` (P4) · P2→P3 dedupe (literal cycle/TEST grammar folds into the skill) · F1/mirror alignment P3↔P5 + P3↔P4 · router registration (new Tasks section, absent) · devils-advocate re-mount (P4) · qa `**TEST:**` classification parked→implement (P2) · boundary vs executing-tdd-cycle CLEAN forward-watch (P3).

**Plus cluster wiring (the dissolving supervisor's orchestration):** re-mount `validation-task-artifacts` on `devils-advocate` (stub already present in the agent); register P2–P5 in the router (`mochiko` skill) + `plugin.json`; decide the producer-report home.

**Reused, not re-ported (already in mochiko):** `devils-advocate` agent, `advocate-report-template`, `loop-discipline`, `workflow-contract`, `agent-dispatch` template.

**Out of scope (precedent — not consumed by the two-agent tasks workflow):** `strategy-implementation`, `executing-tdd-cycle`, `staff-engineer`, `qa-engineer`, `testing-end-user`, `brownfield-integration`, `state-analyst` (dissolved).

## Open reconciliation question (for Phase 2 human gate)

- **RQ-A — artifact shape:** keep two artifacts/phases (`task-mapping.md` → `tasks.md`, HIL + plan precedent) vs. collapse mapping into `tasks.md` (one artifact). To be surfaced by the assessments and gated at reconcile.

## Precedent applied (settled — not re-litigated)

- Core-only scope; thin/altitude command shape (4th command port); `.mochiko/specs/<feature>/` memory model; round cap 3; single completeness reviewer (the `specify` shape); `tasks-context-template` → absorb-into-lead (3× confirmed).

---

## Running log

- **2026-07-01 — Phase 0 complete.** Cluster resolved to 6 primitives (core-only). `contract.md` filled (default-FAIL done-condition; meta-loop producer `transform-producer` ↔ validator `validator`; cap 3; kill-switch `.mochiko/transform/tasks/STOP`; gate on dispositions + escalations). Intake human gate cleared (scope + gate placement). `loop-discipline` confirmed the contract (all 4 reqs, no open brackets).
- **2026-07-01 — Phase 1 complete.** All 6 primitives assessed (P6 re-dispatched after an API mid-response drop; fresh run, no partial work). Dispositions recorded above; complete responsibility traces in each `assess-*.md`. Zero silent drops flagged. RQ-A surfaced by all 6 as the pivotal cluster-scope call. Next: Phase 2 reconcile (one producer, whole cluster) → human gate.
- **2026-07-01 — Phase 2 complete (reconcile + human gate).** `reconcile.md` closed with ZERO open flags: rehome map homes every P1+P6 responsibility once + adds the 4 gates HIL lacked; single-reviewer specify shape confirmed (no split owed). **Human gate cleared — all 3 recommendations accepted:** (1) **RQ-A = Branch A** (keep two artifacts; task-mapping.md source of truth, tasks.md table derived echo); (2) **producer-report = new `taskarchitect-report-template`** (mirror techanalyst-report-template, self-verdict dropped); (3) **routine bundle accepted** (P1 thin redesign, P6 absorb, devils-advocate re-mount, all enumerated drops — mechanism/decouple/independence, no capability lost). Next: Phase 3 transform (Wave 1 = 5 content primitives parallel; Wave 2 = wiring pass).
- **2026-07-01 — Phase 3 Wave 1 complete + 1 referee arbitration.** All 5 content primitives landed: P1 `tasks.md` (77 lines), P2 `task-architect.md` (109 lines, zero residual deny-list tokens) + new `taskarchitect-report-template.md` (80 lines), P3 `patterns-vertical-tdd/` (3 files), P4 `validation-task-artifacts/` (3 files), P5 `tasks-template.md`. **P3↔P5 alignment defect** surfaced by the P3 producer (correctly handed up, not self-graded) and **independently confirmed by the lead** (Read both files): P3's canonical cycle ends in a `**TEST:**` verification block; P5 kept HIL's deprecated "Demo and validate" final task with no `**TEST:**`. **Arbitration (correctness call, no human gate): fix P5 UP to `**TEST:**`** — it's mandated by P2, is P3's reconcile-directed canonical grammar, is what P4 checks, and is a Flag-5a byte-align point. Dispatching: P5 alignment fix + Wave-2 wiring (parallel, independent files). Then Phase 4 verify.
- **2026-07-01 — Phase 3 complete.** P5 alignment fix landed (all 5 sample cycles now end in a `**TEST:**` block matching CYCLE-STRUCTURE.md; Cycle Format bullet + TDD-Discipline note updated). Wave-2 wiring landed: devils-advocate re-mount live (`validation-task-artifacts` in `skills:`, stub comment deleted, bullet added; independence preserved — mounted on reviewer only), router Tasks-cluster section + `/mochiko:tasks` + `task-architect` rows + updated devils-advocate row, `plugin.json` agents array += `task-architect`. No dangling mounts. All 6 realized traces written (`transform-*.md`). Next: Phase 4 independent verify (6 parallel `validator` dispatches, default FAIL).
- **2026-07-01 — Phase 4 complete — ALL PASS (single round, zero fixes).** 6 independent `validator` dispatches (each Read the artifact itself, default FAIL): P1 command PASS · P2 task-architect + report-template PASS (decoupling scan: **zero deny-list tokens** on the run's hardest surface) · P3 patterns-vertical-tdd PASS (P3↔P5 `**TEST:**` alignment confirmed HOLDS from both files) · P4 validation-task-artifacts PASS (verdict lead-owned, dedup + disjoint boundary confirmed) · P5 tasks-template PASS (the `**TEST:**` fix holds across all 5 cycles) · wiring PASS (re-mount live, independence structural, router complete, plugin.json valid, no dangling mounts). Every trace audit PASSED (no silent drops). Non-blocking: 3 traces have stale "deferred to Wave-2" lines (validators confirmed everything landed against live files — bookkeeping only, as the plan run noted).
- **2026-07-01 — Phase 5 complete — RUN DONE.** Done-condition MET (verify-output PASS ×6 + complete traces + human gate cleared). `report.md` written; `REGISTRY.md` updated (tasks command + task-architect + patterns-vertical-tdd + validation-task-artifacts + tasks-template → `[x]`; tasks-context-template → `[-]` absorbed; taskarchitect-report-template new `[x]`; devils-advocate row → cross-workflow); `BACKLOG.md` updated (tasks scoping DONE; both validation-task-artifacts re-mount follow-ups closed; altitude-pass tasks DONE; new Tasks-port follow-ups section). **Verdict: ✅ DONE.**
