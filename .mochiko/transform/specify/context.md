# Transform Run Context — cluster `specify`

**Command:** `/mochiko:transform-cluster` · **Started:** 2026-06-27 · **Lead/referee:** the `transform-cluster` supervisor (this session)

## User request

> i want to transform the human-in-loop specify workflow and all its related agents, skills etc

## Scope (confirmed via human gate, Phase 0)

- **Breadth:** specify-**core** (adversarial spec loop: analyst producer + advocate critic + lead referee). Matches ROADMAP "rebuild as adversarial 2-member agent team" + setup's core-only precedent.
- **Analyst authoring skills:** **INCLUDED** — `authoring-requirements` + `authoring-user-stories` ported as part of specify-core (they are the producer's only skills; REGISTRY mis-files them under the plan cluster — re-file plan→specify on finalize).
- **Design track:** **DEFERRED** — `ui-designer` + design skills (analysis-screenshot, authoring-design-system, patterns-*) not ported this run (specify catalog never invokes them).

## Resolved primitive list (14)

HIL source root: `human-in-loop/plugins/humaninloop/`

| # | Primitive | Path | Expected disposition (hypothesis — producer decides) |
|---|-----------|------|------------------------------------------------------|
| P1 | `specify` command | `commands/specify.md` | redesign → sound adversarial loop, kernel-free; rehome state-analyst's orchestration |
| P2 | `requirements-analyst` | `agents/requirements-analyst.md` | port-with-edits (producer); mount authoring-requirements + authoring-user-stories |
| P3 | `devils-advocate` | `agents/devils-advocate.md` | port-with-edits (adversarial validator); specify slice = analysis-specifications; defer plan/tasks validation skills (stub) |
| P4 | `state-analyst` | `agents/state-analyst.md` | **assess → dissolve / absorb-into-lead** (kernel-free); trace + rehome non-DAG briefing/convergence/parse responsibilities |
| P5 | `analysis-iterative` | `skills/analysis-iterative/` | port (input enrichment / Who-Problem-Value) |
| P6 | `analysis-specifications` | `skills/analysis-specifications/` | port (gap analysis — advocate's skill) |
| P7 | `authoring-requirements` | `skills/authoring-requirements/` | port (analyst's skill — FR-XXX) |
| P8 | `authoring-user-stories` | `skills/authoring-user-stories/` | port (analyst's skill — P1/P2/P3, G/W/T) |
| P9 | `strategy-specification` | `skills/strategy-specification/` | reconcile — rehome spec-loop patterns into lead; dedupe vs loop-discipline |
| P10 | `strategy-core` | `skills/strategy-core/` | reconcile — dedupe vs loop-discipline; rehome residual |
| P11 | `spec-template.md` | `templates/spec-template.md` | port |
| P12 | `analyst-report-template.md` | `templates/analyst-report-template.md` | port |
| P13 | `advocate-report-template.md` | `templates/advocate-report-template.md` | port |
| P14 | `context-template.md` | `templates/context-template.md` | expected absorb-into-lead (setup memory-model precedent) |

**Pre-decided excluded (kernel-free, already `[-]` in REGISTRY):** `specify-catalog.json`; brain scripts (`create-new-feature.sh`, `check-prerequisites.sh`, `common.sh`, `setup-plan.sh`). Native `Explore` agent (targeted-research node) — no port.

**Deferred (out of specify-core):** `ui-designer` + design track; `devils-advocate`'s `validation-plan-artifacts` + `validation-task-artifacts`.

## Run goals beyond the port (from BACKLOG)

1. **Empirical decoupling-doctrine test** — confirm no new persona/skill acquires a deny-list token; confirm verify-output's decoupling scan + keystone test catch slips.
2. **Confirm two empirical structural calls** (promote to ROADMAP if specify agrees with setup): human-gate placement (gated dispositions + escalations) and the memory model (in-session + `.mochiko/memory/`, workspace-as-state, context-handoff absorbed into lead).

## Running log

- **2026-06-27 — Phase 0:** Intake complete. Traced cluster from specify-catalog.json + agent frontmatter + REGISTRY. Scope confirmed via AskUserQuestion (authoring skills IN, design track DEFERRED). Workspace + contract.md created. loop-discipline check PASS (all four requirements, no open brackets).
- **2026-06-27 — Phase 1:** All 14 primitives assessed (one producer per primitive, parallel). Assessments in `assess-*.md`. Results below. Next: Phase 2 reconcile.

### Phase 1 — assessment results

| # | Primitive | Class | Disposition (proposed) | Reconcile flag? |
|---|-----------|-------|------------------------|-----------------|
| P1 | specify-command | command/IS-a-loop | **redesign × rewire-cluster** | YES — RF1–RF7 (the lead absorbs the dissolving orchestration + MUST ADD default-FAIL contract / lead-owned verdict / kill-switch / human acceptance gate) |
| P2 | requirements-analyst | agent | port-with-edits × standalone | light — **no new validator owed** |
| P3 | devils-advocate | agent | port-with-edits × standalone | light — defer `validation-plan-artifacts` + `validation-task-artifacts` (stub/rebind) |
| P4 | state-analyst | agent | **drop × absorb-into-lead** + flag | YES — central dissolution; (a) kernel mechanics dropped, (b) judgment work → moved-to-lead/dedupe |
| P5 | analysis-iterative | skill | port-with-edits × standalone | F1 mount-point · F2 dual-mode scope · F3 trigger de-collision w/ P6 |
| P6 | analysis-specifications | skill | port-with-edits × standalone | confirm-pairing · verdict-ownership→lead · trigger de-collision |
| P7 | authoring-requirements | skill | port-with-edits × standalone | overlap w/ P8 (dedupe candidate, **not merge**) |
| P8 | authoring-user-stories | skill | port-with-edits × **flag-for-reconcile** | overlap w/ P7 (keep distinct + dedupe substrate) |
| P9 | strategy-specification | skill | **redesign × flag-for-reconcile** | YES — 3-way dedupe (loop-discipline × strategy-core); thin spec slice survives → lead |
| P10 | strategy-core | skill | **drop × flag-for-reconcile** | YES — near-total dedupe into loop-discipline; only Gap Classification survives |
| P11 | spec-template | template | port-with-edits × standalone | F-1 header identity model · F-2 lead seeds spec.md |
| P12 | analyst-report-template | template | port-with-edits × standalone | edit-depth (gated on P4) · header consistency w/ P13 |
| P13 | advocate-report-template | template | **keep-verbatim × standalone** | soft — verdict→loop-driver rehomes to lead |
| P14 | context-template | template | **drop × absorb-into-lead** | YES — confirms+strengthens memory-model; one coherent rehome map w/ P1/P4 |

### Phase 1 — key cross-cutting findings

1. **Independence is ALREADY SOUND in HIL specify** — `requirements-analyst` (producer, authoring skills) ≠ `devils-advocate` (validator, `analysis-specifications`); skill sets disjoint, no self-grade leak. **Sharp contrast with `setup`** (which needed a validator split). So **no new partner agent is created this run** — the pair is confirmed, not constructed. The only fix is **verdict ownership → lead** (was orchestrator/analyst-autonomous).
2. **The real mochiko upgrade is the LOOP, not the agents.** HIL's done-condition is orchestrator-evaluated (not default-FAIL), the bound is soft, and the **human ACCEPTANCE gate is MISSING**. P1's redesign must ADD all four loop-discipline requirements (the agents/skills are largely portable as-is).
3. **state-analyst dissolves cleanly** — every non-kernel judgment responsibility (briefing, convergence-watch, report-parse, recommendation, escalation, cap, input-assessment) → `moved-to-lead`, most also `dedupe` vs loop-discipline. Kernel mechanics (DAG/MCP/catalog/freeze/JSON) → `dropped + reason: kernel-free`. Highest silent-drop risk; fully traced.
4. **strategy-core is near-total dedupe into loop-discipline** (6 of 9 patterns map 1:1 onto the four requirements) — only the **Gap Classification taxonomy** (knowledge→research / preference→user / scope→halt) is additive. strategy-specification's survivors: **Input Assessment** (sparse/rich triage) + targeted-revision + spec done-condition content. Both strategy skills' fate: dissolve, rehome residual.
5. **context-template absorbs-into-lead** — CONFIRMS + STRENGTHENS setup's memory-model decision (in-session + `.mochiko/` workspace-as-state). BACKLOG OQ#3 ready to promote.
6. **DECOUPLING DOCTRINE HOLDS (empirical win).** Agent/skill BODIES are clean by absence; coupling lives in the dissolving orchestration layer. Only genuine deny-list hits: `analysis-specifications` L19 names "Devil's Advocate" (the canonical grep-catchable case → decouple to role); strategy skills name "State Analyst" (moot under dissolution). requirements-analyst, authoring-requirements, authoring-user-stories, all templates: ZERO body hits. Strong support for the doctrine + verify-output's scan.

- **2026-06-27 — Phase 2:** Reconcile complete (`reconcile.md`), **zero open flags**. Single coherent rehome map (§B); no new partner primitive; producer↔validator pair CONFIRMED (analyst↔advocate). **Human gate (contract §4a) — both questions answered:**
  - **Q1 (Gap routing) → FOLD into `loop-discipline`** (additive edit; record as ROADMAP Key Decision). Highest-blast-radius edit, accepted.
  - **Q2 (rest of gated bundle) → ACCEPT ALL, proceed.** Ratified: P1 redesign (+ NEW human acceptance gate), P4 `state-analyst` + P14 `context-template` absorb-into-lead (incl. A30 read-reports reversal + a9 silent-recovery removal), P9/P10 strategy dissolution, P5 6a marker drop, soft P11 feature_id rebind, all §E dropped-responsibility reasons.
  - Net Phase-3 plan: CREATE P1 command + P2/P3 agents + P5/P6/P7/P8 skills + P11/P12/P13 templates; EDIT loop-discipline (additive Gap Classification) + router; DISSOLVE P4/P9/P10/P14; NEW primitives none.
  - Next: Phase 3 transform.
- **2026-06-27 — Phase 3:** All 12 Phase-3 artifacts written to `plugins/mochiko/` (parallel producers). 3 agents hit a session-limit on their RETURN message only — disk verified: **all artifacts + traces present and complete** except P1's trace. Artifacts: `commands/specify.md` (P1, 329 lines, sound), `agents/requirements-analyst.md` (P2), `agents/devils-advocate.md` (P3), `skills/analysis-iterative/` (P5, full dir), `skills/analysis-specifications/` (P6), `skills/authoring-requirements/` (P7, +refs +script), `skills/authoring-user-stories/` (P8, +refs +script), `templates/{spec,analyst-report,advocate-report}-template.md` (P11/P12/P13), `skills/loop-discipline/SKILL.md` (EDIT — Gap Classification landed L83/91-95), `skills/mochiko/SKILL.md` (router EDIT — specify cluster registered). Realized traces in `transform-*.md`. **Gap:** `transform-specify-command.md` (P1 trace) not written (agent cut off post-artifact) → reconstructing. **Cleanup note:** stray `authoring-user-stories/scripts/__pycache__/` from a smoke-test → remove in Phase 5.
  - Next: reconstruct P1 trace, then Phase 4 verify (12 independent validations).
- **2026-06-27 — P1 trace reconstructed** (`transform-specify-command.md`): every §B/§E responsibility confirmed landed; zero silent drops; 2 benign deviations matching the setup precedent. Phase 3 fully complete.
- **2026-06-27 — Phase 4:** 12 independent `mochiko:validator` runs, **ALL PASS in round 1, zero required fixes.** Bounded loop converged in one round. Trace audits clean. Done-condition MET. Non-blocking housekeeping fixed at finalize: P13 orphan code fence removed; `__pycache__` stray removed.
- **2026-06-27 — Phase 5 (DONE):** REGISTRY updated (specify `[x]`; dissolved/deferred marked; authoring skills re-filed plan→specify). `report.md` written. ROADMAP Key Decisions += human-gate placement + memory model (both promoted from OQ#1/OQ#3) + Gap Classification fold; Decision Trail += specify subsection. BACKLOG: memory-model + 2 specify-confirmation items closed; specify-port follow-ups section added. **Cluster-level manifest fix (caught at finalize):** `plugin.json` `agents` array is explicit (not a glob) — added `requirements-analyst` + `devils-advocate` (would have been undiscoverable otherwise); JSON validated, all 5 agents resolve. marketplace.json needs no change. **RUN COMPLETE — done-condition MET.**
