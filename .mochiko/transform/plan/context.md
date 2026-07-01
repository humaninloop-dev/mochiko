# Transform Run Context — cluster `plan`

**Command:** `/mochiko:transform-cluster` · **Started:** 2026-06-30 · **Lead/referee:** the `transform-cluster` supervisor (this session)

## User request

> i want to transform plan from human in loop into mochiko

## Scope (confirmed via human gate, Phase 0)

- **Breadth:** plan-**core** (the unified analysis→design loop the `/humaninloop:plan` command actually orchestrates: technical-analyst producer + principal-architect feasibility reviewer + devils-advocate completeness reviewer + lead referee). Matches the setup + specify core-only precedent. Confirmed via `AskUserQuestion`.
- **`techspec` merge → RESOLVED (excluded).** HIL already merged `techspec` into `plan` (ADR-008); `techspec.md` is a hard-deprecated stub. The unified `plan` command IS the merged form — techspec's old `integrations.md` / `data-sensitivity.md` outputs are folded into `data-model.md` (sensitivity annotations) + `contracts/api.yaml` (x-integration docs). `techspec` stays `[-]` in REGISTRY.
- **Design / roadmap / tasks track → DEFERRED** (REGISTRY loosely files them under "plan" but `/plan` never invokes them): `patterns-flow-mapping`, `patterns-interface-design`, `ui-designer` (design-track); `authoring-roadmap` + `evolution-roadmap-template` (setup-brownfield); `patterns-vertical-tdd` (tasks). Rebind by reference only.

## Resolved primitive list (15)

HIL source root: `human-in-loop/plugins/humaninloop/`

| # | Primitive | Path | Status | Expected disposition (hypothesis — producer decides) |
|---|-----------|------|--------|------------------------------------------------------|
| P1 | `plan` command | `commands/plan.md` | NEW | **redesign × rewire-cluster** → thin sound loop (altitude); rehome the 2-phase + dual-reviewer + skip-rereview + incremental orchestration to the lead; ADD default-FAIL contract / lead-owned verdict / bound + kill-switch / human acceptance gate |
| P2 | `technical-analyst` agent | `agents/technical-analyst.md` | NEW | port-with-edits (producer); mount P5–P8 (the 4 authoring/patterns skills); decouple-by-absence (rich persona = highest deny-list risk) |
| P3 | `principal-architect` agent | `agents/principal-architect.md` | **PORTED** (setup, producer-only) | **reconcile (RQ2)** — `plan` uses it as a *feasibility reviewer*; current mochiko port is producer-only. Add reviewer capability / skill, or rehome feasibility. No new persona, an edit to the ported one |
| P4 | `devils-advocate` agent | `agents/devils-advocate.md` | **PORTED** (specify) | re-mount `validation-plan-artifacts` (P9) — currently stubbed comment-only in its `skills:`. Light edit |
| P5 | `authoring-technical-requirements` skill | `skills/authoring-technical-requirements/` | NEW | port (analyst's skill — TR / constraints / NFR / IP-XXX / data-sensitivity authoring) |
| P6 | `patterns-technical-decisions` skill | `skills/patterns-technical-decisions/` | NEW | port (analyst's skill — ADR / alternatives / decision records) |
| P7 | `patterns-entity-modeling` skill | `skills/patterns-entity-modeling/` | NEW | port (analyst's skill — data model, DDD entities, state machines) |
| P8 | `patterns-api-contracts` skill | `skills/patterns-api-contracts/` | NEW | port (analyst's skill — REST / OpenAPI / integration boundaries) |
| P9 | `validation-plan-artifacts` skill | `skills/validation-plan-artifacts/` | NEW | port (the advocate's plan-gate checklist; P1/P2 phases). **Pairs with RQ1** — validator-side skill of the plan producer↔validator pair |
| P10 | `plan-template.md` | `templates/plan-template.md` | NEW | port (the `plan.md` summary deliverable) |
| P11 | `plan-context-template.md` | `templates/plan-context-template.md` | NEW | expected **absorb-into-lead** (setup + specify memory-model precedent; this is a large `.workflow/` state carrier) |
| P12 | `techanalyst-report-template.md` | `templates/techanalyst-report-template.md` | NEW | port (producer's report format) |
| P13 | `architect-report-template.md` | `templates/architect-report-template.md` | NEW | port (feasibility-reviewer's report format; fate tied to RQ1) |
| P14 | `cross-artifact-checklist.md` | `templates/cross-artifact-checklist.md` | NEW | **reconcile** — used by the Phase-3 consistency check; decide standalone template vs **fold into `validation-plan-artifacts`** (P9) |
| P15 | `advocate-report-template.md` | `templates/advocate-report-template.md` | **PORTED** (specify) | **reconcile** — reuse the specify-ported version as-is (same 3-state verdict) vs extend for plan; expected keep/reuse |

**Pre-decided excluded (kernel-free / deprecated / already `[-]`):** `techspec` command + `techspec-context-template`; any brain scripts; native `Explore` agent (research node) — no port.

**Deferred (out of plan-core, rebind by reference):** design-track (`patterns-flow-mapping`, `patterns-interface-design`, `ui-designer`); `authoring-roadmap` + `evolution-roadmap-template`; `patterns-vertical-tdd`.

## Central reconciliation questions (the gated bundle — see contract §4a)

- **RQ1 — reviewer architecture.** Two reviewers in HIL plan (architect = feasibility/contradiction; advocate = completeness/`validation-plan-artifacts`). Mochiko shape options: **(i)** keep two distinct validators — a *checklist* advocate + an *adversarial-critique* architect (the convention-5 two-form case; plan would be the **first** cluster to exercise it); **(ii)** fold feasibility into the advocate (one reviewer, the specify shape); **(iii)** rehome feasibility onto the generic `validator`. Decided at reconcile, human-gated.
- **RQ2 — principal-architect role.** Already ported producer-only (setup). Plan uses it as a reviewer. Decoupling-by-absence permits produce-here/review-there for one workflow-agnostic persona; decide the driving skill (new reviewing skill vs persona-only).
- **RQ3 — cross-artifact-checklist (P14)** standalone vs fold into `validation-plan-artifacts` (P9).
- **RQ4 — advocate-report-template (P15)** reuse the specify port as-is vs extend.
- **RQ5 — plan-context-template (P11)** absorb-into-lead (expected) vs keep.

## Run goals beyond the port (from BACKLOG + ROADMAP)

1. **First post-altitude net-new command.** P1 must come out thin by construction (the fixed recipe + `verify-output`'s altitude gate). plan is the most complex command yet (2 phases, 2 reviewers, feasibility loop + clarification loop) — the altitude stress test. Watch the "what you own" footer if it grows.
2. **Resolve the dual-reviewer / two-form-validator question (RQ1)** — does convention 5's "*adversarial-critique* skill for judgment artifacts" + "*mirror-checklist* skill for objective criteria" pairing produce TWO validators here? First cluster to test it.
3. **Decoupling-doctrine empirical test continues** — `technical-analyst` is the richest persona ported so far; confirm zero new deny-list tokens; confirm `verify-output`'s scan + keystone test catch any slip.
4. **Close BACKLOG `plan` scoping item** — techspec-merge decision (excluded) + technical-analyst/task-architect overlap (task-architect is `tasks`-cluster, not pulled in).

## Running log

- **2026-06-30 — Phase 0:** Intake complete. Traced cluster from `commands/plan.md` (the orchestrated agents + skills + templates) cross-checked against REGISTRY + `techspec.md` (confirmed deprecated/merged). Scope confirmed via `AskUserQuestion` (core-only; design/roadmap/tasks track deferred). Workspace + `contract.md` created; `loop-discipline` check PASS (all four requirements, no open brackets). Next: Phase 1 assess (15 primitives).
- **2026-06-30 — Phase 1:** All 15 primitives assessed (one producer per primitive, two parallel batches). Assessments in `assess-*.md`. Results below. Next: Phase 2 reconcile.

### Phase 1 — assessment results

| # | Primitive | Class | Disposition (proposed) | Reconcile flag? |
|---|-----------|-------|------------------------|-----------------|
| P1 | plan-command | command/IS-a-loop | **redesign × absorb-into-lead** | YES — RQ1; ADD 4 missing gates (default-FAIL, lead-owned verdict, bound+kill-switch, human acceptance gate); altitude split done |
| P2 | technical-analyst | agent | port-with-edits × standalone | RQ1 (validator partner undecided); 3 decouple edits (all caller-injection class); light dedupe (literal procedure→P5–P8) |
| P3 | principal-architect | agent | **port-with-edits × flag-for-reconcile** | YES — RQ1 + RQ2; **CRITICAL: current mochiko port explicitly DISCLAIMS feasibility ("not your responsibility") → feasibility review is HOMELESS in mochiko** |
| P4 | devils-advocate | agent | port-with-edits × standalone (light) | RQ1; re-mount P9 at lines 25–29 (frontmatter + stub + body); valid in ALL RQ1 options |
| P5 | authoring-technical-requirements | skill | port-with-edits × standalone | P5↔P6 D-XXX/constraints-and-decisions.md boundary; P5↔P7 DS overlap; P5↔P8 INT overlap; P5↔P9 latent pair |
| P6 | patterns-technical-decisions | skill | port-with-edits × standalone | P5↔P6 shared constraints-and-decisions.md (boundary+dedupe, not merge); D-XXX trigger de-collision |
| P7 | patterns-entity-modeling | skill | port-with-edits × standalone | **RQ6a — data-sensitivity orphan**: P5 declares DS-XXX + delegates to P7, full taxonomy lives in P5's ARTIFACT-TEMPLATES §4, P7 has only light PII markers → broken delegation + 2 divergent data-model.md templates; P7↔P8 handoff |
| P8 | patterns-api-contracts | skill | port-with-edits × standalone | **RQ6b — x-integration orphan**: P5 forward-refs "see patterns-api-contracts" but P8 has ZERO x-integration content → dangling; P9 grades it Critical → cannot drop; P7↔P8 handoff |
| P9 | validation-plan-artifacts | skill | port-with-edits × standalone | RQ1 (P9 ALREADY overlaps feasibility checks → fold-low-friction); RQ3 (P14 fold-in); incremental-mode split (procedure→skill, mode-select→lead) |
| P10 | plan-template | template | port-with-edits × standalone | content-consistency w/ P5–P8 outputs; lead (P1) assembles plan.md; keep `[...]` bracket style (not `{{}}`) |
| P11 | plan-context-template | template | **drop × absorb-into-lead** | YES — memory-model **3rd confirmation** (richest carrier yet, all dissolves); F-P11-2 report-paths RQ1-coupled |
| P12 | techanalyst-report-template | template | port-with-edits × standalone | edit-depth gated on P11 absorb; cross-report header consistency gated on RQ1; carries NO verdict (lead-owned) |
| P13 | architect-report-template | template | **port-with-edits × flag-for-reconcile** | YES — RQ1 (fate rides on it); 3-state `feasible/needs-revision/infeasible` vs validator's binary PASS/FAIL tension; `infeasible`=business escalation (distinctive) |
| P14 | cross-artifact-checklist | template | **port-with-edits × flag-for-reconcile** | YES — RQ3; producer lean **fold into P9** (orphan — grep finds no consumer; near-total dup of P9; tasks "shared" is false — tasks validator self-contains its own) |
| P15 | advocate-report-template | template | **keep-verbatim × standalone (REUSE)** | RQ4; producer lean **reuse as-is** (same 3-state verdict + severity; plan gaps fold into existing taxonomy); NO new file — just REGISTRY re-file specify→shared |

### Phase 1 — key cross-cutting findings (input for reconcile)

1. **RQ1 (reviewer architecture) is THE central question** — flagged by P1/P2/P3/P4/P9/P13. Converged producer input:
   - mochiko `devils-advocate` = completeness reviewer (mirror-checklist via P9) **already exists**; the P9 re-mount is valid in ALL three options (P4 not blocked on RQ1).
   - **Feasibility review is HOMELESS** — the mochiko `principal-architect` port (setup, producer-only) *actively disclaims* it. "ALREADY PORTED" is a trap: reusing it as-is silently drops plan's entire cross-artifact-contradiction gate.
   - **P9 already overlaps feasibility** (its P1/P3 checks include TR↔constraint contradictions, NFR↔constraint conflicts, NFR-design feasibility, constraint-design alignment) → option **(ii) fold feasibility into the advocate is low-friction**.
   - Generic `validator` is **binary PASS/FAIL**; both reviewer reports are **3-state adversarial** → option **(iii) rehome-to-validator has structural friction** (binary-vs-3-state; would flatten `infeasible`→escalate).
   - Convention-5 two-form: ONE `devils-advocate` persona can mount BOTH forms (checklist P9 + critique `analysis-specifications`) → evidence the two forms are two *skills* under one persona, not necessarily two agents.
   - **Three options for reconcile:** (i) two distinct validator agents (checklist advocate + adversarial-critique architect — first cluster to run convention-5 two-form); (ii) fold feasibility into the advocate (one reviewer, specify shape; low-friction given P9 overlap); (iii) rehome feasibility onto generic `validator` (binary-vs-3-state friction).
2. **RQ2 (principal-architect role)** — produce-here/review-there is structurally legitimate (decoupling-by-absence + independence permit it; different artifacts, different workflows). Re-coupling risk LOW. **Guardrail G1: re-broadening PA must NOT re-mount `validation-constitution`** (would recreate the original constitution self-grade). Entangled with RQ1 (PA only gains a reviewer role if RQ1 picks option (i)).
3. **RQ3 (cross-artifact-checklist P14) → strong fold-into-P9 lean.** P14 is an ORPHAN (no file consumer — grep empty), near-total dup of P9; the plan+tasks "shared source" argument is FALSE (tasks validator self-contains a different checklist). Human-gated.
4. **RQ4 (advocate-report-template P15) → strong reuse-as-is lean.** Same verdict contract; produces NO new file; reuse survives all 3 RQ1 branches.
5. **RQ5 (plan-context-template P11) → drop × absorb-into-lead.** 3rd memory-model confirmation. RQ1-coupled report-path slice resolves after RQ1.
6. **NEW — RQ6 producer-skill content homes** (the highest silent-drop surface, from P11/P5/P7/P8): the producer-side content physically living in the dissolving plan-context Supervisor-Instructions blocks (IP-XXX infra, **data-sensitivity taxonomy**, **x-integration boundaries**, advocate focus-area checklists) survives ONLY if the producer/validator skills carry it. Two concrete orphans: **RQ6a** data-sensitivity (P5↔P7 broken delegation; P7-lean: P7 absorbs taxonomy + canonical data-model.md template) and **RQ6b** x-integration (P5↔P8 dangling ref; P8-lean: add to P8 per ADR-008). Neither can drop (P9 grades both Critical). Plus the P5↔P6 constraints-and-decisions.md ownership boundary.
7. **Decoupling doctrine HOLDS again (empirical win).** Richest persona (technical-analyst) = 3 hits, all ONE caller-injection class ("your instructions will specify…"), ZERO dangerous tokens (no sibling names / dispatch / workflow-agnostic / kernel-DAG-phase). Skills: P8 zero hits; P5/P6/P9 only injected-workflow-phases; P7 one stale "Generated by Domain Architect" stamp. All bounded edits. `verify-output`'s scan + keystone test will confirm at Phase 4.
8. **Altitude split (P1) clean.** Generic loop-discipline → `dedupe` (references); workflow-specific orchestration → `moved-to-lead` (2-phase analysis→design, architect-feasibility-ONCE-then-advocate ordering, skip-architect-unless-structural routing, Phase-3 incremental review, team casting, done-condition params, plan.md assembly, entry/brownfield prerequisites). Thin command must ADD the 4 missing gates.

- **2026-06-30 — Phase 2:** Reconcile complete (`reconcile.md`), **zero open flags**. Single coherent rehome map (Job 2, P1 ⊕ P11); independence attested by construction; **one new primitive** (`validation-feasibility` skill — feasibility reviewer's driver on a re-broadened `principal-architect`; **zero new agents**). **Human gate (contract §4a) — both questions answered:**
  - **RQ1 (reviewer architecture) → option (i) TWO DISTINCT VALIDATORS, accepted.** `technical-analyst` (producer, authoring-only) → graded by `principal-architect` (feasibility, NEW `validation-feasibility` skill, 3-state incl. `infeasible`, once after analysis) + `devils-advocate` (completeness, re-mount `validation-plan-artifacts`, both phases); the plan command lead referees + owns the verdict. Plan is the first cluster to run convention-5's two-form. Forcing fact: feasibility review was HOMELESS (current PA port disclaims it). RQ2 ratified with it: produce-here(setup)/review-there(plan) is decoupling-legitimate; driver = new `validation-feasibility` skill (not persona-only); **G1 honored** — `validation-constitution` NOT re-mounted on PA.
  - **Supporting dispositions → ACCEPT ALL.** P11 `drop × absorb-into-lead` (3rd memory-model confirmation); P14 `drop × merge-into-sibling` → fold into P9 (orphan + near-total dup); P15 `keep-verbatim` reuse (no new file, REGISTRY re-file specify→shared); RQ6a data-sensitivity → P7 (canonical taxonomy + data-model template); RQ6b x-integration → P8 (added content); P5↔P6 boundary+dedupe; kernel-free/independence drops (P12 `completion_status`, P10 producer stamp, P11 timestamps/node-kind, P13 DAG `type`) all accepted.
  - **Net Phase-3 plan:** CREATE — P1 command (thin) + P2 technical-analyst + P5/P6/P7/P8/P9 skills + `validation-feasibility` skill + P10/P12/P13 templates (P13 renamed `feasibility-report`); EDIT — P3 principal-architect (re-broaden + mount validation-feasibility) + P4 devils-advocate (re-mount P9) + router + plugin.json; DISSOLVE (trace only, no artifact) — P11 (→P1), P14 (→P9); REUSE (no file) — P15. **Sequencing:** P9 + validation-feasibility must exist before P4/P3 verify (handle by writing all in Phase 3, verify in Phase 4).
  - Next: Phase 3 transform.
- **2026-07-01 — Phase 3:** All 16 artifacts written to `plugins/mochiko/` in two waves (Wave 1: 10 new — P2 agent + P5/P6/P7/P8/P9 + `validation-feasibility` skills + P10/P12/P13 templates; Wave 2: P3/P4 edits + P1 thin command). **Seams verified reciprocal** during Wave 1: P5's DS→P7 / x-int→P8 reference phrasing matches what P7/P8 now contain (broken delegations repaired); the P9↔`validation-feasibility` check boundary mirrored both sides (4 feasibility checks removed from P9 → homed in the new skill; `infeasible` preserved; G1 honored). P13 renamed `feasibility-report-template.md`. P11 absorbed into P1; P14 folded into P9; P15 reused (no file). **2 agents (P3, P1) hit a session-limit on their RETURN message only — disk verified: artifacts complete** (P3 re-broadened + `validation-feasibility` mounted + G1; P1 command 82 lines); their traces (`transform-principal-architect.md`, `transform-plan-command.md`) reconstructed from the on-disk artifacts. **P1 altitude confirmed (referee read):** 82 lines (specify 66 < setup 78 < plan 82; delta = workflow-specific, not restated doctrine); 18/18 moved-to-lead landed; 4 gates added; loop-discipline/workflow-contract/agent-dispatch referenced not restated; state recovery reads workspace evidence.
  - **2 non-blocking flags for the verify pass:** (a) P3 `principal-architect` L126 uses "its own validator" as by-role independence language (reconstructor read: acceptable role-language, not a dispatch ref); (b) router/`plugin.json` registration deferred to the Wave-3 wiring pass (must land before verify or discoverability FAILs spuriously).
  - Next: Wave 3 wiring (router + plugin.json), then Phase 4 verify (16 primitives + the 2 edits).
- **2026-07-01 — Wave 3 wiring:** router (`skills/mochiko/SKILL.md`) gained a Plan-cluster section (`/mochiko:plan`, technical-analyst, the 6 skills, 3 templates; broadened principal-architect + devils-advocate; advocate-report re-filed shared); `plugin.json` gained `technical-analyst` (commands/skills are globs — auto-register); JSON validated; **zero dangling mounts**. Wired before verify so discoverability wouldn't FAIL spuriously.
- **2026-07-01 — Phase 4 (independent verify):** 14 independent `mochiko:validator` runs (13 artifacts + the cluster wiring; P11/P14/P15 folded into P1/P9/wiring). **ALL PASS.** 2 hit transient connection errors mid-response (validation-feasibility, feasibility-report-template) → re-dispatched → both PASS. **Bounded loop converged in a single round, zero required fixes.** Highlights independently confirmed: P1 altitude scan clean (thin-by-construction); P3 G1 held (no `validation-constitution`; L126 "its own validator" = by-role language, accepted); P7 DS taxonomy genuinely absorbed; P8 x-integration genuinely added; P9↔validation-feasibility boundary **clean from both sides** (no overlap, no gap); `infeasible` preserved end-to-end; P14 folded (no standalone file); P15 git-confirmed unchanged. **Done-condition MET.**
- **2026-07-01 — Phase 5 (DONE):** Quickstart/Integration-Guide label aligned on `technical-analyst` (coupling-free finalize housekeeping). REGISTRY updated (plan `[x]`; technical-analyst + 5 skills + validation-feasibility + 3 templates marked; principal-architect/devils-advocate notes broadened; P11/P14 `[-]`; advocate-report re-filed shared; deferrals flagged). ROADMAP += Decision-Trail subsection (altitude recipe confirmed by construction; convention-5 two-form first exercised; produce-here/review-there decoupling; decoupling doctrine 3rd pass; techspec excluded). BACKLOG: `plan` scoping item closed; re-mount + altitude follow-ups marked `[~]`; plan-port follow-ups section added. `report.md` written. **RUN COMPLETE — done-condition MET.**
