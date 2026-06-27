# ASSESSMENT: state-analyst

**Primitive:** `human-in-loop/plugins/humaninloop/agents/state-analyst.md`
**Run:** transform-cluster / `specify` · **Producer:** mochiko:transform-producer · **Assessed:** 2026-06-27
**Scope note:** specify-CORE only. `state-analyst` is shared across specify/plan/tasks/implement — its plan/tasks/implement slices are tagged `moved-to-other-cluster` and handled when those clusters port.

```
ASSESSMENT: state-analyst
Class:        agent → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=n  [full-lens]
Disposition:  drop (body) × absorb-into-lead (structural)  +  flag-for-reconcile (rehome placement)
```

This is the **central dissolution** of the `specify` port and the **highest silent-drop risk in the cluster**. The agent *is* the DAG/MCP orchestration brain that mochiko removes. The transform's whole hazard is here: dropping the agent's **mechanism** (correct) while silently dropping the **judgment responsibilities** it carried (forbidden). The trace below exists to make that impossible.

---

## Step 1 — Class & branch

**AGENT → PLAYS-a-role.** Loop-ownership is not this primitive's to hold in mochiko; it plays the role of "the engine the Supervisor delegates all DAG mechanics + state analysis to." Weighted checks: persona-vs-procedure split, team-role conferred, `skills:` independence, and (run goal) **persona decoupling**.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — maximally. Depends on a kernel (`hil-dag` MCP), a catalog (`specify-catalog.json`), a DAG (`*-strategy.json`), AND a markdown supervisor (`commands/specify.md`) that drives it via a 4-action JSON envelope. |
| 2 | Multi-responsibility / fans out? | **YES** — briefing, report-parse, gap-classify, recommend/rank, convergence-watch, node assembly, pass freeze, status update, prompt construction. Feeds the Supervisor + every domain agent. |
| 3 | Emits a non-machine-checkable reviewable artifact? | **NO** — its outputs are ephemeral coordination (briefing/recommendation JSON to the Supervisor) and machine-checkable DAG JSON. It authors no reviewable artifact needing a producer↔validator partner (it is forbidden from writing source artifacts). |

At least one YES → **full 7-check lens.**

---

## Step 3 — The 7-check lens (PLAYS-a-role weighting)

**1 — Orchestration test.** Orchestrated by a **kernel** (`hil-dag` MCP + catalog + DAG) *and* a **markdown supervisor** (`specify.md`), which together drive it. This is the rare HIL primitive that is **both** content-coupled and orchestration-coupled to the maximum:
- *Content-coupling:* the body references MCP tools, catalog JSON, node ids, pass numbers, edge/invariant vocabulary throughout → not `port-with-edits`-fixable; it is the mechanism mochiko sheds → **body = drop**.
- *Orchestration-coupling:* it only exists because the Supervisor sequences it and loops on its `advance` verdicts. In mochiko **there is no separate orchestration agent** — the command supervisor owns the loop directly → the agent shell dissolves; its responsibilities re-home → **structural = absorb-into-lead**.
- *Inherited-by:* the **lead** (command supervisor) inherits every non-kernel responsibility; the **excluded kernel** absorbs every DAG/MCP/catalog mechanic.

**2 — Role at two altitudes.** Skill-role: a *consumed procedure* (the Supervisor "runs" it per pass), not an artifact-emitter. Team-role conferred on its caller: it makes the Supervisor the **lead/referee** by doing the lead's thinking for it (what's next, are we converging, escalate?). In mochiko that thinking belongs **on** the lead, not in a delegate — which is exactly the `absorb-into-lead`. No validator partner needed (it grades nothing, authors nothing reviewable).

**3 — Independence.** No self-grade leak today (it neither produces nor grades source artifacts; the read/write boundary in Operational Rules explicitly bars it from writing `spec.md`). **Forward-looking guard:** `absorb-into-lead` must land the routing/convergence/recommendation judgment on the lead **as referee only** — the lead must NOT also acquire producer (`requirements-analyst`) or validator (`devils-advocate`) skills. Loop-routing/verdict-ownership *is* a referee responsibility (loop-discipline req 2), so this absorb is sound by construction — flagged to reconcile to keep it that way.

**4 — Verdict-sink / loop-driver.** This is the heart of the silent-drop risk. The agent owns: consuming the advocate gate verdict and **driving the FAIL-loop** (`needs-revision`→new pass), the **done-signal** (`ready`→completion), and **escalation** (`critical-gaps`→escalate). These were "the biggest things the kernel/supervisor owned." They MUST re-home to the lead's bounded loop, not vanish with the DAG → `moved-to-lead` + `dedupe(loop-discipline)`.

**5 — Sibling / overlap ("look sideways").** Strong overlap signal → **reconcile**. The agent's `skills:` are `strategy-core` (P10) + `strategy-specification` (P9) + `strategy-implementation` (other-cluster). Its briefing JUDGMENT *is literally those skills executed* — gap classification (= strategy-core "Gap Classification"), pass/convergence evolution (= strategy-core "Pass Evolution"), halt/escalation (= strategy-core "Halt Escalation"), input assessment + produce-then-validate + gap-informed revision (= strategy-specification). So each (b) responsibility lands somewhere that **overlaps a ported strategy skill AND dedupes against `loop-discipline`** — a placement decision that needs full cluster context.

**6 — Coupling audit.**
- *Hardcoded paths:* `.humaninloop/memory/constitution.md`, `{feature_dir}/.workflow/...`, `${CLAUDE_PLUGIN_ROOT}/catalogs/specify-catalog.json`, `dags/{workflow}-strategy.json`. Spec-relevant ones → `kept-but-rebind` (to `.mochiko/`); catalog/DAG/MCP paths → `dropped` (kernel-free).
- *Prerequisites/handoffs:* assumes constitution exists (INV-002), and assumes upstream artifacts on disk (enriched-input, spec.md, advocate-report.md) → these become explicit handoff-edge checks on the lead.
- *Determinism boundary:* the **deterministic** slice (file-exists / frontmatter gate checks; constitution-gate) → can survive as a deterministic assert on the lead. The **judgment** slice (briefing, ranking, convergence, gap-classification) → grounded model judgment the lead owns. The DAG graph-algorithmic slice (assembly, edge inference, invariant validation, topo sort) → **dropped with the kernel**.

**7 — Conventions + loop placement + DECOUPLING SCAN (run goal).**
- *Classification / router / triggers:* N/A on the realized side — the agent is dropped, so it gets no mochiko classification tag or router entry; the lead (P1) carries placement.
- *Loop placement:* the agent currently *is* the loop driver. Post-port the loop lives in the lead's contract (done-condition default-FAIL, independent validator = devils-advocate, bounded cap, human gate). The 5-pass cap (INV-004) and convergence-stall watch → `dedupe(loop-discipline)` bounded-iteration.
- **DECOUPLING SCAN — saturated, expected, RECORDED.** The persona is maximally workflow-coupled. Deny-list tokens present in force:
  - **Sibling-agent names:** `requirements-analyst`, `devils-advocate`, `staff-engineer`, `qa-engineer`, `Explore`, `state-analyst`, and the `humaninloop:<agent>` naming rule.
  - **"dispatch" / "dispatch_mode":** pervasive.
  - **Injected workflow modes/paths/phases:** pass numbers ("Pass 2 of 5"), node ids (`analyst-review`, `advocate-review`, `targeted-research`, `human-clarification`, `constitution-gate`, `spec-complete`…), action verbs (`brief-and-assemble`, `parse-and-advance`, `update-and-advance`, `re-brief`), workflow names (`specify`, `implement`).
  - **Defined entirely relative to "the Supervisor."**
  - **Verdict of the scan:** there is **no keystone-surviving "true on any job" professional core** to decouple — the agent's whole identity *is* "the engine that drives this DAG workflow." So the scan does **not** yield a `port-with-edits decouple` action (as it would for a survivable agent); it yields **confirming evidence for `drop`**: once the workflow machinery is removed, nothing decoupleable remains. This is the expected outcome for the cluster's dissolving primitive, and it is the empirical decoupling-doctrine datapoint the run wanted.

---

## Step 4 — Disposition

**`drop` (body) × `absorb-into-lead` (structural)` + `flag-for-reconcile`.**

- **Body = drop:** the content is kernel/DAG/catalog/MCP mechanism end-to-end; there is no portable body. (`drop` still emits a full, auditable trace — below.)
- **Structural = absorb-into-lead:** pure orchestration with no reusable standalone body; the non-kernel responsibilities move **onto the command supervisor**, leaving no orphan agent. Mochiko has no separate orchestration agent by design.
- **flag-for-reconcile:** `absorb-into-lead` is proposable solo, but **where each (b) judgment responsibility actually lands** is a cross-primitive call (lead-prose vs carried-by-a-ported-strategy-skill vs `dedupe` into `loop-discipline`). That depends on P1 (`specify` command/lead), P9 (`strategy-specification`), P10 (`strategy-core`), P2/P3 (the report-parse boundary) — none visible from this primitive alone.

---

## Step 5 — Responsibility trace (DONE-CONDITION; the surgical (a)/(b) split)

### (a) Pure DAG / MCP / kernel / catalog mechanics — `dropped` (dissolve WITH the excluded kernel)

| # | Responsibility | Tag |
|---|----------------|-----|
| a1 | `hil-dag` MCP mount + all MCP calls (`assemble`, `record`, `status`, `freeze`, `validate`, `sort`, `catalog_validate`) | dropped + reason: **kernel-free (MCP excluded)** |
| a2 | Node assembly — capability-tag resolution, edge inference from consumes/produces, invariant validation, `carry_forward` gate auto-add | dropped + reason: **kernel-free (DAG excluded)** |
| a3 | Pass freezing + pass lifecycle (create/execute/freeze within StrategyGraph) | dropped + reason: **kernel-free (DAG excluded)** |
| a4 | Node status updates / graph mutation / direct DAG-JSON writes | dropped + reason: **kernel-free (DAG excluded)** |
| a5 | Catalog-driven resolution — read `specify-catalog.json`; `capability_tags`, `node_type`, contract `consumes`/`produces`, `dispatch_mode` mapping | dropped + reason: **kernel-free (catalog excluded)** |
| a6 | DAG file as state-of-record (read single DAG for history) | dropped + reason: **kernel-free (DAG excluded)** — *purpose* (track prior passes/outcomes) re-homes to workspace-as-state on the lead (see b2/state) |
| a7 | Recording Protocol — evidence/trace JSON, `EV-{node}-{pass}-{seq}` ids, derived `duration_ms` | dropped + reason: **kernel-free (DAG excluded)** |
| a8 | Graph invariant validation, edge inference, topological concerns, `resolution_failed` retries | dropped + reason: **kernel-free (DAG excluded)** |
| a9 | **Silent** `carry_forward` auto-resolution ("the Supervisor is never informed") | dropped + reason: **kernel-free AND mochiko forbids silent recovery** (good riddance) |
| a10 | The **agent shell itself** — frontmatter (name/model/color), the "separate orchestration agent" identity, the 4-action protocol (`brief-and-assemble`/`parse-and-advance`/`update-and-advance`/`re-brief`), the JSON request/response envelope to the Supervisor | dropped + reason: **mochiko has NO separate orchestration agent; the command supervisor owns the loop directly** |

### (b) Non-kernel JUDGMENT the lead still needs — `moved-to-lead` / `dedupe` (MUST NOT drop with the kernel)

> Contract constraint §1 names four that must survive: **briefing, recommendation, convergence-watching, report-parsing.** All four are below, re-homed — none dropped.

| # | Responsibility | Tag |
|---|----------------|-----|
| b1 | **State briefing** (state_summary, pass_context, relevant_patterns synthesis) | **moved-to-lead** [reconcile: placement vs strategy skills] |
| b2 | **Outcome-trajectory / convergence-watching** (count gaps/pass, detect flat/growing/recurring trends, stall signal) | **moved-to-lead** + **dedupe** (loop-discipline no-progress exit; strategy-core "Pass Evolution") [reconcile] |
| b3 | **Report parsing into structured findings** (analyst-report, advocate-report, enriched-input, research-findings → verdict / gaps / summary extraction) | **moved-to-lead** [reconcile: lead-parses vs producer/validator emits lead-ready structured output — P2/P3] |
| b4 | **Gap classification** (knowledge / preference / scope) | **moved-to-lead** + **dedupe** (strategy-core "Gap Classification") [reconcile] |
| b5 | **Ranked recommendation / auto-selection** (intent+rationale+priority; rank by impact; knowledge-before-preference, gap-resolving-before-production) | **moved-to-lead** + **dedupe** (strategy-core / strategy-specification) [reconcile] |
| b6 | **Verdict-sink + FAIL-loop driving** (ready→complete, needs-revision→new pass, critical-gaps→escalate) | **moved-to-lead** + **dedupe** (loop-discipline req 2 verdict ownership + req 3.4 escalate-don't-die) |
| b7 | **Escalation triggering** (critical-gaps; "report ALL non-auto-resolvable errors to Supervisor — never silently recover") | **moved-to-lead** + **dedupe** (loop-discipline escalate) |
| b8 | **5-pass cap logic** (INV-004 "after 5 passes, surface to user with options") | **moved-to-lead** + **dedupe** (loop-discipline bounded-iteration hard cap) — note the *workflow's* own pass cap is the lead's bounded loop, distinct from the transform-run's cap=3 |
| b9 | **Input assessment / first-pass handling** (sparse vs detailed input; enrichment-or-not decision) | **moved-to-lead** + **dedupe** (strategy-specification "Input Assessment") [reconcile] |
| b10 | **Produce-then-validate sequencing** (analyst → advocate gate ordering; never skip the gate) | **moved-to-lead** + **dedupe** (strategy-specification / loop-discipline external validation) |
| b11 | **Dispatch-prompt construction / domain-agent briefing** ("point agents at artifacts on disk, minimal instructions, let the agent's own system prompt guide behavior" — already a decoupled-dispatch principle in HIL) | **moved-to-lead** + **dedupe** (mochiko `agent-dispatch.md` briefing guide) [reconcile] |
| b12 | **Decision-node answer persistence** (write user clarification answers to disk) | **moved-to-lead** + **kept-but-rebind** (`.workflow/` → `.mochiko/` workspace) |
| b13 | **Deterministic gate evaluation — constitution-gate** (constitution exists before spec; INV-002) | **moved-to-lead** (deterministic assert) + **kept-but-rebind** (path) |
| b14 | **Input-availability / prerequisite checks** ("expected artifact missing"; contract satisfiability for spec artifacts) | **moved-to-lead** (handoff-edge check) |
| b15 | **Read/write boundary principle** ("analyst never writes source artifacts; domain agents do") | **moved-to-lead** as referee boundary (lead doesn't author `spec.md`; producer does) — partial **dedupe** (independence) |

### Path rebinds (convention-wiring; recorded so the lead's rehome inherits them)

| # | Responsibility | Tag |
|---|----------------|-----|
| r1 | `.humaninloop/memory/constitution.md` | **kept-but-rebind** → `.mochiko/memory/constitution.md` (lands on lead) |
| r2 | `{feature_dir}/.workflow/...` spec artifacts (spec.md, analyst-report, advocate-report, research-findings, clarification-answers, enriched-input, raw-input, context.md) | **kept-but-rebind** → mochiko workspace paths (lead) |
| r3 | Catalog / DAG / MCP paths (`specify-catalog.json`, `dags/*-strategy.json`) | **dropped** + reason: kernel-free |

### Other-cluster (shared agent; out of specify-core scope)

| # | Responsibility | Tag |
|---|----------------|-----|
| o1 | `strategy-implementation` skill mount | **moved-to-other-cluster** (implement) |
| o2 | Cycle awareness (Briefing Rule 9 — `tasks.md` cycles, checkpoint/validation content) | **moved-to-other-cluster** (implement) |
| o3 | `cycle-report.md` / `verification-report.md` parsing patterns | **moved-to-other-cluster** (implement) |
| o4 | `execute-cycle` (staff-engineer) / `verify-cycle` (qa-engineer) NL prompt patterns | **moved-to-other-cluster** (implement) |
| o5 | `tasks-complete` / `cycle-checkpoint` / `final-validation` deterministic gates | **moved-to-other-cluster** (tasks/implement) |
| o6 | Plan/implement artifact paths (tasks.md, plan.md, data-model.md, contracts/, tasks-context.md, …) | **moved-to-other-cluster** |

### Skill mounts (overlap signal → reconcile)

| # | Responsibility | Tag |
|---|----------------|-----|
| s1 | `strategy-core` mount (P10) — specify-relevant patterns overlap b2/b4/b5/b6/b7/b8 | **reconcile** (placement; strategy-core assessed separately) |
| s2 | `strategy-specification` mount (P9) — overlaps b5/b9/b10 | **reconcile** (placement; strategy-specification assessed separately) |

**Trace completeness:** every responsibility above carries a tag. The four contract-named survivors (briefing b1, recommendation b5, convergence-watching b2, report-parsing b3) are all `moved-to-lead`, **none dropped**. ✓

---

## Reconcile flags (for `reconcile-cluster`, full cluster context)

1. **(primary) Rehome placement of the (b) judgment set.** `absorb-into-lead` is chosen, but each (b) responsibility's *final* home is cross-primitive: **lead-prose** vs **carried by a ported strategy skill (P9/P10)** vs **`dedupe` into `loop-discipline`**. Reconcile must (a) decide each placement with P1/P9/P10 in view, and (b) **confirm the lead's rehome map actually receives every (b) item** so nothing is dropped with the DAG. This is the run's #1 silent-drop guard.
2. **Report-parse boundary (with P2/P3).** With the kernel gone and no separate analyst, does the **lead parse** domain reports (b3), or do the **producer (`requirements-analyst` P2) / validator (`devils-advocate` P3) emit lead-ready structured findings**? HIL's "Supervisor NEVER reads reports directly" was a context-protection artifact of the kernel era; mochiko's lead reads the validator's verdict directly. Reconcile sets this boundary.
3. **Independence guard on the absorb.** Reconcile must confirm `absorb-into-lead` lands routing/convergence/recommendation judgment on the lead **as referee only** — the lead must not also mount producer/validator skills (would recreate a self-grade). Sound by construction (these are referee duties), but explicitly ratify it.
4. **strategy-skill dedupe vs `loop-discipline` (feeds P9/P10).** Many (b) dedupe-candidates (b2/b4/b5/b6/b7/b8/b9/b10) overlap both the strategy skills and `loop-discipline`. Reconcile coordinates this with the P9/P10 dedupe verdicts so the same doctrine isn't authored in three places.

(Note: the `moved-to-other-cluster` rows o1–o6 are clean handoffs, **not** flags for this cluster — they are deferred to the plan/tasks/implement ports of the shared `state-analyst`.)

---

## Human-gate note (for the supervisor)

This disposition is **gated** (contract §4a): the `state-analyst` **dissolution** (`drop`/`absorb-into-lead`) and its dropped-responsibility reasons (the a1–a10 + a9 silent-recovery removal) require human acceptance at Phase-2 reconcile before any edit. Every `dropped` row carries a reason for that acceptance.
