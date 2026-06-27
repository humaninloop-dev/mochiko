# Transform Run Report — cluster `specify`

**Date:** 2026-06-27 · **Workflow:** `/mochiko:transform-cluster specify` · **Scope:** core-only (user-selected) · **Run state:** **DONE** (done-condition met)

## Done-condition: MET

- `verify-output` returned **PASS** for every produced primitive — **12 verify dispatches, all PASS in round 1, zero required fixes.** The bounded loop converged in a single round.
- Every original responsibility carries a realized trace tag — trace audits PASS on all primitives, **no silent drops** (the highest-risk dissolution, `state-analyst`, fully traced: kernel mechanics dropped-with-reason, all non-kernel judgment `moved-to-lead`/`dedupe`).
- Human gate cleared all gated dispositions (Phase 2) — the P1 redesign, the P4 + P14 absorb-into-lead, the P9/P10 strategy dissolution + the `loop-discipline` edit, the P5 marker drop, and every §E dropped reason.

## The core transformation (the inverse of `setup`)

`setup` had to **construct** an independent validator (split `principal-architect` + promote a new `constitution-validator`). `specify` already **ships** one: `requirements-analyst` (producer, authoring skills) ↔ `devils-advocate` (critic, `analysis-specifications`) — disjoint skills, different agents, no self-grade leak. So **no new primitive was created this run.** The real work was two things HIL's brain/DAG hid:

1. **Re-landing the dissolving DAG orchestration onto the command lead.** HIL delegated all orchestration to a `state-analyst` agent driving `specify-catalog.json` via the `hil-dag` MCP. Kernel-free mochiko has no such agent — the `specify` command supervisor owns the loop directly. `state-analyst` (P4) + both strategy skills (P9/P10) + `context-template` (P14) all dissolved onto that single lead via one coherent rehome map.
2. **ADDING the four `loop-discipline` gates HIL never had.** HIL's done-condition was orchestrator-evaluated (the analyst auto-completed the gate; the loop could declare itself done on pass 1), the bound was soft, and there was **no human acceptance gate** on the happy path. The redesign installed: default-FAIL done-condition, **lead-owned verdict** (the advocate's status is input), hard round cap + no-progress + STOP kill-switch + escalate-on-cap, and the **NEW human acceptance gate (G3)**.

## Per-primitive outcome

| # | Primitive | Disposition (resolved) | Verdict | Landed as |
|---|-----------|------------------------|---------|-----------|
| P1 | specify command | redesign × rewire-cluster | **PASS** | `commands/specify.md` — kernel-free adversarial sound loop |
| P2 | requirements-analyst | port-with-edits × standalone | **PASS** | `agents/requirements-analyst.md` (producer; skills authoring-requirements + authoring-user-stories) |
| P3 | devils-advocate | port-with-edits × standalone | **PASS** | `agents/devils-advocate.md` (critic; skill analysis-specifications; 2 plan/tasks skills stubbed) |
| P4 | state-analyst | drop × absorb-into-lead | **PASS** | **dissolved** into P1 lead — non-kernel judgment rehomed/deduped; kernel mechanics dropped |
| P5 | analysis-iterative | port-with-edits × standalone | **PASS** | `skills/analysis-iterative/` (dual-mode kept; HARD decouple hits scrubbed; DAG markers dropped) |
| P6 | analysis-specifications | port-with-edits × standalone | **PASS** | `skills/analysis-specifications/` (L19 "Devil's Advocate" → role; stays a gap-finder, no verdict added) |
| P7 | authoring-requirements | port-with-edits × standalone | **PASS** | `skills/authoring-requirements/` (+refs +script; dangling patterns-* links scrubbed) |
| P8 | authoring-user-stories | port-with-edits × standalone | **PASS** | `skills/authoring-user-stories/` (+refs +script; flag resolved → standalone; kept-distinct) |
| P9 | strategy-specification | absorb-into-lead + dedupe | **PASS** | **dissolved** — generic → loop-discipline; survivors → lead |
| P10 | strategy-core | drop × dedupe-into-loop-discipline | **PASS** | **dissolved** — 6/9 patterns → loop-discipline; Gap Classification folded in |
| P11 | spec-template | port-with-edits × standalone | **PASS** | `templates/spec-template.md` (8 slots verbatim; header → workspace-as-state) |
| P12 | analyst-report-template | port-with-edits × standalone | **PASS** | `templates/analyst-report-template.md` (iteration→round; counts demoted) |
| P13 | advocate-report-template | keep-verbatim × standalone | **PASS** | `templates/advocate-report-template.md` (verdict verbatim; iteration→round) |
| P14 | context-template | drop × absorb-into-lead | **PASS** | **absorbed** into P1 lead — workspace-as-state; not produced |
| — | loop-discipline | EDIT (additive) | **PASS** | `skills/loop-discipline/SKILL.md` — Gap Classification taxonomy folded in (universal, keystone-clean) |
| — | router | EDIT (wiring) | **PASS** | `skills/mochiko/SKILL.md` — specify cluster registered; no dissolved/deferred primitive registered |

**Net:** CREATE 1 command + 2 agents + 4 skills + 3 templates; EDIT loop-discipline + router; DISSOLVE 4 primitives; NEW primitives: **none**.

## Accepted drops (human-gated, Phase 2)

- HIL's orchestrator-evaluated done-condition (analyst auto-completes `ready` + self-certified milestone; no human acceptance) — **dropped** (replaced by default-FAIL + lead-owned verdict + G3). Reason: loop-discipline req 1.
- `state-analyst` kernel mechanics (hil-dag MCP, node assembly, pass-freezing, catalog resolution, DAG-as-state, the agent shell) — **dropped** (kernel-free); every non-kernel judgment rehomed, none lost.
- `state-analyst` silent `carry_forward` auto-resolution — **dropped** (mochiko forbids silent recovery; the constitution prerequisite survives as an explicit deterministic handoff).
- HIL's "Supervisor NEVER reads agent reports" context-isolation rule — **reversed** (the state-analyst middle-man is gone; the lead must read `spec.md` + the advocate report to own the verdict). A deliberate posture improvement.
- DAG parse-markers in `analysis-iterative` (`ENRICHMENT_COMPLETE`/`_OUTPUT_END`), the DAG outcome-trajectory metric in the analyst report, the feature-numbering scheme in the spec template, `context-template`'s DAG state-file bookkeeping — all **dropped** (kernel-free; capabilities preserved by workspace-as-state + direct reads).
- strategy-core's "advisory / may-deviate" stance — **dropped** (superseded by loop-discipline's non-negotiable framing).

Every dropped row carried a reason and a "capability lost? **No**" judgment (except the two deliberate improvements above). All accepted at the Phase-2 gate.

## Run-goal results (both confirmed)

1. **Agent↔workflow decoupling doctrine — HELD empirically.** Agent/skill bodies were clean by absence; coupling lived in the dissolving orchestration layer. The only genuine deny-list hits were the canonical grep-catchable case (`analysis-specifications` L19 "Devil's Advocate" → decoupled to role) and the dissolving strategy skills (moot). `requirements-analyst`, `authoring-requirements`, `authoring-user-stories`, and all templates: **zero body hits**. `verify-output`'s decoupling scan + keystone test caught and confirmed every case. The Gap Classification fold into `loop-discipline` passed the universality grep (zero specify tokens). → **Doctrine proven; no deny-list refinement needed.**
2. **Two empirical structural calls — specify confirms setup.** Human-gate placement (gated dispositions + escalations + a new acceptance gate) and the memory model (in-session + `.mochiko/` workspace-as-state, context-handoff absorbed into the lead) both worked cleanly. P14 is a *stronger* confirmation than setup's twin (no drift; more state, still dissolves). → **Ready to promote to ROADMAP Key Decisions + close BACKLOG OQ#3.**

## Deviations & non-blocking notes (recorded, validator-PASSED)

1. **P1 trace reconstructed.** The Phase-3 P1 producer hit a session limit after writing `commands/specify.md` but before its trace; a fresh producer reconstructed `transform-specify-command.md` from the written artifact + reconcile rehome map (confirmed every responsibility landed). Two benign deviations from the literal assessment tags, both matching the all-PASS setup precedent: A7 `git rev-parse` → cwd-relative `.mochiko/` paths; the `disable-model-invocation` classification flag is a skill convention, not a command one (absent in setup.md too).
2. **Housekeeping fixed at finalize:** an orphan trailing code fence in `advocate-report-template.md` (wrapper hygiene; flagged by two validators) and a stray `__pycache__/` from a script smoke-test were removed.
3. **Cosmetic, no fix:** `authoring-requirements`'s `validate-requirements.py` `BANNED_TERMS` heuristic can false-positive on prose (faithful verbatim port of HIL's Tier-1 self-check); the router-edit trace narration says "eight names" where seven are listed (the substantive zero-hits claim is correct).

## Open follow-ups (for ROADMAP.md / BACKLOG.md)

- **Re-mount `devils-advocate`'s deferred skills when plan/tasks port.** `validation-plan-artifacts` + `validation-task-artifacts` are stubbed (comment-only in `skills:`, not live mounts) with re-mount edges logged. Re-mount when those clusters land.
- **`strategy-implementation` will likely dissolve the same way** when `implement` ports (dedupe into `loop-discipline`) — REGISTRY notes this.
- **`ui-designer` + design track** remain deferred (out of specify-core; the catalog never invokes them). Scope with a design cluster or `plan`.
- **Dogfood `/mochiko:specify` for real (behavioral validation).** The port passed *structural* verification; it has not run end-to-end. Run it on a real feature to confirm the produce→critique loop converges, the advocate's gap list flows back as a targeted revision, the gap-classification routing (Explore / clarification / halt) fires sensibly, and the G3 acceptance gate works. Pairs with the still-open dogfood-`/mochiko:setup` check.
- **Promote to ROADMAP / close BACKLOG** the two confirmed structural calls + the decoupling-doctrine result + the Gap Classification fold (done in this finalize).
