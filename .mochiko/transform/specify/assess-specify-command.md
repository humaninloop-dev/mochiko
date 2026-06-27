# ASSESSMENT: `specify` command (P1)

**Primitive:** `human-in-loop/plugins/humaninloop/commands/specify.md`
**Run:** transform-cluster · cluster `specify` · Phase 1 assessment
**Assessed by:** `mochiko:transform-producer` (assess-primitive) · 2026-06-27
**This is the loop itself** — weighted per the IS-a-loop branch: loop-driver, done-condition, validation + human gate placement.

---

## Class / branch

- **Class:** command → **branch IS-a-loop**.
- A command *is* a loop; it does not *play a role in* one. So the checks that carry weight are: **who drives the loop**, **the done-condition** (default-FAIL? bounded? human gate?), and **where validation + human gates sit or are missing**. Classification/persona-decoupling checks (which dominate for agents) are secondary here.

---

## Triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — depends wholly on the `hil-dag` MCP/DAG kernel, `specify-catalog.json`, and the `state-analyst` agent to function. The command body is a delegation harness. |
| 2 | Multi-responsibility / fans out? | **YES** — drives the entire spec loop, dispatches multiple agents (analyst, advocate, Explore, skills), owns argument-parsing, completion, human input, escalation. |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **YES (as a loop)** — the loop exists precisely because its deliverable `spec.md` is judged by model judgment (the advocate), not a schema/version equality. The producer↔validator pairing is real, not degenerate. |

**Result: full 7-check lens** (all three gates trip — expected for the loop itself).

---

## The 7-check lens (weighted IS-a-loop)

### Check 1 — Orchestration test (content vs orchestration coupling)

**Orchestrating layer:** a **Python/MCP/DAG kernel** (`hil-dag` MCP server + `humaninloop-brain`) driven through the `state-analyst` agent, fed by `specify-catalog.json`. The command is a *thin markdown supervisor over a kernel* — it delegates ALL mechanics ("zero direct CLI usage; all `hil-dag` operations delegated to the State Analyst"). Nominally the Supervisor "decides," but the `state-analyst` computes next-node, recommendations, advance-action, freezing, and gate/milestone evaluation — so the kernel+analyst *actually* drive the loop and the Supervisor rubber-stamps.

**Content-coupling (body work → `redesign`):** the body is saturated — `description: "DAG-based workflow execution"`, the "DAG Vocabulary" section (4 node types / 6 edge types / pass lifecycle / gate verdicts), catalog + context-template resource paths, "Two Outbound Verbs" (Ask-the-Analyst / Dispatch), `create-new-feature.sh` + `hil-dag` MCP install block, `mkdir .workflow/dags`, the entire Supervisor-Loop (brief-and-assemble / parse-and-advance / update-and-advance / re-brief), dispatch_mode routing, Lifecycle Rules 1–7, Context-Protection rules, and the Supervisor-vs-Analyst Responsibility-Boundaries table. None of this survives a localized edit.

**Orchestration-coupling (cluster work → `rewire-cluster` + rehome):** the command only "works" because the kernel sequences it and the analyst feeds it. When the kernel+analyst dissolve, the loop-driving, verdict-evaluation, convergence-watch, pass-freezing, prompt-construction, briefing, and report-parsing must re-home — onto **this command as the lead** (the mochiko `setup.md` precedent: "you, executing this command, are the supervisor").

**Conclusion:** body = `redesign`; placement = `rewire-cluster`; a large rehome list (below) lands on the lead.

### Check 2 — Role at two altitudes

- **Skill-role:** the command is a **consumed orchestration procedure** (a user-invoked slash command). It does not itself emit a reviewable artifact; it *orchestrates the production* of `spec.md`.
- **Team-role it confers:** running it makes the executing session the **lead / referee** — it owns the loop, the verdict, and the human gates. In mochiko this is explicit (no separate lead agent); in HIL it was split between the Supervisor (this file) and the `state-analyst` (the kernel driver).

### Check 3 — Independence

**The produce/grade boundary is HELD in HIL specify — preserve it.** `requirements-analyst` (produces `spec.md`) ≠ `devils-advocate` (grades it via the advocate report). This is exactly the adversarial pair the mochiko thesis wants. **No self-grade leak in the command.**

**Subtlety (not a leak):** the `state-analyst` touches *both* sides — it constructs the producer's brief AND parses the validator's verdict/evaluates the gate. But it neither *authors* nor *grades* `spec.md`; it orchestrates. When it dissolves, the **lead** inherits both touchpoints — which is correct: a referee is allowed to brief the producer and read the validator's verdict. Independence is structural (producer agent writes, validator agent grades, lead never does either) and stays intact. One real fix is required: the gate **verdict** currently is evaluated *autonomously by the analyst* ("evaluate-gate → Analyst evaluates autonomously"); in mochiko the **lead must own the verdict**, routing on the advocate's grounded report — `moved-to-lead`.

### Check 4 — Verdict-sink / loop-driver (the heart of this assessment)

- **Consumer of the loop's output:** currently the `state-analyst`'s `parse-and-advance` consumes the advocate report and emits an `advance.action_taken` the Supervisor routes on. **What loops on FAIL:** advocate verdict `needs-revision` → `freeze_and_new_pass` → new pass (Lifecycle Rule 1). Verdict `critical-gaps` → `escalate` to user (Rule 3). Verdict `ready` → `completion` (Rule 2).
- **Where loop-driving must re-home:** onto **the lead** (this command), as a bounded produce→validate→repeat loop. This is the single biggest thing the kernel/analyst owned and the thing most at risk of silent loss.

### Check 5 — Sibling / overlap ("look sideways")

This command is the hub the whole cluster wires through. Relations (all → reconcile):
- pairs **`requirements-analyst` (P2, producer)** ↔ **`devils-advocate` (P3, validator)** as the loop's independent pair;
- **`state-analyst` (P4)** dissolves *into* this command — its non-DAG capabilities re-home here;
- **`strategy-specification` (P9)** + **`strategy-core` (P10)** supply spec-loop patterns that re-home into the lead / dedupe vs `loop-discipline`;
- **`context-template` (P14)** is absorbed into the lead (setup precedent);
- templates **`spec-template` (P11) / `analyst-report-template` (P12) / `advocate-report-template` (P13)** are consumed via lead-wired handoff edges.
- **Deferred, confirmed by reading the body:** the specify command references **no** design track (no `ui-designer` / screenshot / design-system) — deferral is safe; and the advocate's `validation-plan-artifacts` / `validation-task-artifacts` are plan/tasks-only — stub.

### Check 6 — Coupling audit

- **Hardcoded paths:** `${CLAUDE_PLUGIN_ROOT}/catalogs/specify-catalog.json` → **drop**; `${CLAUDE_PLUGIN_ROOT}/templates/context-template.md` → **absorb/rebind** (P14); `${CLAUDE_PLUGIN_ROOT}/scripts/create-new-feature.sh` → **drop** (script) / capability **moved-to-lead**; `specs/{feature-id}/.workflow/dags/...` → **drop** (DAG path); feature workspace `specs/{feature-id}/.workflow/` → **rebind** to a mochiko workspace (path decision for reconcile/transform); `.humaninloop/memory/constitution.md` → **rebind** `.mochiko/memory/constitution.md`.
- **Upstream prerequisite:** a constitution must exist (HIL did this via INV-002 `carry_forward` auto-resolution inside the analyst). Re-home as an explicit **prerequisite handoff edge** the lead checks (`.mochiko/memory/constitution.md` present), like setup's deterministic input checks.
- **Determinism boundary:** `spec.md` correctness = **model judgment** → grounded validation by `devils-advocate` is **real** (confirms the pair is warranted). Only file-existence is deterministic (spec present) — a Tier-1 sub-check, not the quality gate.

### Check 7 — Conventions + loop placement

- **Classification:** user-invoked slash command. `description` must be rebound (drop "DAG-based workflow execution").
- **Discoverability:** register in the `mochiko` router (user-invoked → hinted).
- **Triggers:** N/A (command, not a model-invoked skill).
- **Decoupling scan (the run's headline test) — for THIS primitive: PASS by construction.** The deny-list (sibling-agent names, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-labels) governs **personas and skills**, not the **command supervisor**. A command legitimately names its agents, says "dispatch," and has phases — `setup.md` does all three correctly. The keystone test confirms it: these are true of *this lead on this workflow* because the lead *is* the workflow. **The real decoupling risk is downstream (P2/P3/P4 personas, P5–P10 skills); the redesigned command must not push its workflow vocabulary INTO those agents' personas** — that is their decoupling check, flagged for the run.
- **Producer↔validator pairing:** structurally guaranteed (analyst agent ≠ advocate agent, disjoint skills). Preserve; the redesign must inline a filled `workflow-contract` (setup precedent) as inspectable proof.
- **Sound-loop placement — gaps to fill (feed reconcile rehome-orchestration):**

| Sound-loop requirement | HIL specify today | Verdict |
|------------------------|-------------------|---------|
| Pre-declared done-condition, **default-FAIL** | "verdict `ready` + milestone `achieved`", **evaluated by the analyst**, no pre-declared contract artifact, milestone self-certified | **NOT sound** — verdict-*gated* (good: it does not auto-complete on exhaustion, it escalates) but **not a default-FAIL contract** and **orchestrator-evaluated**, not lead/human-owned. ADD a default-FAIL contract + lead-owned verdict. |
| Independent validation | `devils-advocate` reviews `spec.md` (separate agent) | **Sound — preserve.** Move only the *verdict ownership* to the lead. |
| Bounded iteration | 5-pass cap (Rule 6) + convergence stall (Rule 5), both "surface to user"; **computed by the analyst from DAG history**; no kill-switch | **Partially sound** — a cap and a no-progress check exist and *escalate* (not auto-done), but they are soft, analyst-computed, not lead-owned, and there is no kill-switch/budget. ADD lead-owned hard cap + no-progress + kill-switch. |
| Defined **human gate** | human only on clarification decision-nodes + `critical-gaps` / stall / cap / error escalations | **Acceptance gate MISSING** — on the happy path (verdict `ready` first pass) the loop **auto-completes to a summary with no human acceptance**. ADD a human acceptance gate (setup G3 precedent). |

**The real transformation is not "shed the DAG" — it is ADD the default-FAIL contract, the lead-owned verdict, the lead-owned bound, and the human ACCEPTANCE gate the original never had**, while *preserving* the genuine analyst↔advocate independence HIL already got right.

---

## DISPOSITION

```
Disposition: redesign × rewire-cluster
```

- **Body = `redesign`:** the DAG/catalog/MCP/state-analyst body is rewritten around the mochiko sound loop (the `setup.md` shape: pre-declared default-FAIL done-condition, inlined `workflow-contract`, lead-owned produce→validate→repeat, named human gates, state-recovery-from-workspace). Carry the *responsibilities*, not the mechanism. `port-with-edits` is impossible — there is no clean body under the kernel scaffolding.
- **Structural = `rewire-cluster`:** the command is the **lead** that the dissolving `state-analyst` re-homes into; it wires the analyst↔advocate pair, absorbs `context-template`, re-homes the strategy patterns, and adds the missing gates. The command itself is not split/merged/promoted (it *is* the lead — `absorb-into-lead` is a category error for it), but its transformation is inseparable from the cluster rewire.
- **Why not `flag-for-reconcile` for the whole disposition:** the body treatment is firmly proposable now. The structural *map* (which sibling responsibility lands exactly where) depends on P4/P9/P10/P14 and is carried as reconcile flags below — per the skill, the body is proposed and the relational specifics are flagged.

---

## RESPONSIBILITY TRACE (complete — nothing silently lost)

Mechanism and capability are separated where a kernel mechanism is shed but the capability it served must survive. **Every `dropped` carries a reason for the lead/human gate to accept.**

### A. The command's own responsibilities (from `specify.md`)

| # | Responsibility | Tag |
|---|----------------|-----|
| A1 | Argument parsing + lost-input (`@`-bug) recovery via `AskUserQuestion` | `kept` (rebuilt on lead) |
| A2 | Goal: produce a validated `spec.md` (the deliverable) | `kept` |
| A2′ | Done-condition *definition* ("verdict `ready` + milestone `achieved`", analyst-evaluated) | `kept-but-rebind` → redesigned to **default-FAIL** (advocate verdict `ready` **AND** human acceptance), **lead-owned** |
| A3 | DAG Vocabulary: node types / edge types / pass lifecycle | `dropped + reason` (DAG kernel concepts; mochiko is kernel-free — no DAG) |
| A3′ | Gate-verdict taxonomy: `ready` / `needs-revision` / `critical-gaps` | `kept-but-rebind` (becomes the advocate's three-way verdict the lead routes on) |
| A4 | Catalog resource path | `dropped + reason` (catalog excluded, kernel-free) |
| A4′ | Context-template resource path | `kept-but-rebind` → absorbed (P14; in-session + workspace) |
| A5 | "Ask the Analyst" outbound verb (delegate to state-analyst) | `dropped + reason` (state-analyst dissolves; no delegation layer) |
| A5′ | "Dispatch the agent" outbound verb | `kept` (lead dispatches producer, then validator) |
| A6 | "Zero direct CLI / all hil-dag delegated" | `dropped + reason` (no hil-dag, no CLI delegation) |
| A7 | Resolve project root (`git rev-parse`) | `kept` |
| A8 | `create-new-feature.sh` invocation (script) | `dropped + reason` (brain script, excluded) |
| A8′ | Feature-workspace creation + feature-id derivation + seed `spec.md` from template (capability) | `moved-to-lead` (lead does it inline, kernel-free, setup `mkdir -p` precedent) |
| A9 | Verify `hil-dag` MCP available + install instructions | `dropped + reason` (MCP/brain install; kernel-free) |
| A10 | Parse script JSON for `BRANCH_NAME`/`SPEC_FILE`/`FEATURE_NUM` | `dropped + reason` (parses the dropped script) — feature-id capability folds into A8′ |
| A11 | `mkdir .workflow/dags`, set `dag_path` | `dropped + reason` (DAG directory/file; kernel-free) |
| A12 | Create `context.md` from context-template (handoff mechanism) | `dropped + reason` (context-handoff file absorbed; P14) |
| A12′ | Carry run state across the loop (capability) | `moved-to-lead` (in-session + `.mochiko/` workspace, setup precedent) |
| A13 | Seed `spec.md` from `spec-template`, Edit placeholders | `kept-but-rebind` (producer authors from `spec-template` (P11); lead wires it; rebind path) |
| A14 | Absolute-paths-rooted-at-`PROJECT_ROOT` convention | `kept-but-rebind` (rebind to mochiko workspace) |
| A15 | Supervisor Loop — brief-and-assemble delegation (start of pass) | `dropped + reason` (no analyst/DAG assembly) — capabilities split to B1/B8 |
| A16 | Override / `re-brief` (override analyst's auto-selected recommendation) | `dropped + reason` (no node-menu/recommendations in a fixed produce→validate loop) |
| A17 | Per-Node dispatch_mode routing (agent/skill/supervisor-owned/auto-resolved) | `dropped + reason` (DAG dispatch-mode machinery) |
| A17′ | Actual dispatch capability (dispatch producer/validator agent; invoke a skill; collect input) | `moved-to-lead` (lead's fixed produce→validate dispatch) |
| A18 | Supervisor-owned `collect-input` via `AskUserQuestion` | `kept` (human clarification sub-gate, rebuilt on lead) |
| A18′ | Supervisor-owned `evaluate-gate` (analyst evaluates autonomously) | `moved-to-lead` (**lead owns the verdict**, routing on advocate evidence — the independence fix) |
| A18″ | Supervisor-owned `verify-milestone` (analyst verifies prerequisites) | `moved-to-lead` (lead checks the done-condition) |
| A19 | `parse-and-advance` delegation (mandatory after every agent) | `dropped + reason` (no analyst/DAG record/advance) — capabilities split to B4/B6 |
| A20 | `update-and-advance` delegation (after supervisor-owned node) | `dropped + reason` — capture-answers capability `moved-to-lead` |
| A21 | Evaluate-and-Route on `advance.action_taken` | `dropped + reason` (routing on analyst's field) — routing logic `moved-to-lead` (lead's loop control) |
| A22 | Lifecycle Rule 1 — `needs-revision` → freeze + new pass | `moved-to-lead` (lead loops back to produce); freeze mechanism → A24 |
| A23 | Lifecycle Rule 2 — `ready` → completion | `moved-to-lead` + redesigned (lead declares done **only** after human acceptance — A26/G-accept) |
| A24 | Lifecycle Rule 3 — `critical-gaps` → escalate to user | `moved-to-lead` (lead escalates to human gate) |
| A25 | Lifecycle Rule 5 — convergence stall (same gap count 2+ passes → surface) | `moved-to-lead` (**lead's no-progress guard** — loop-discipline req 3) |
| A26 | Lifecycle Rule 6 — 5-pass cap → surface options | `moved-to-lead` + `kept-but-rebind` (lead-owned **hard round cap**; "surface" → "escalate, never silently continue/declare done"; cap value a design choice) |
| A27 | Lifecycle Rule 7 — unexpected/error → present to user | `moved-to-lead` (lead's escalate-don't-silently-die) |
| A28 | Pass lifecycle / freezing (created/executed/frozen in StrategyGraph) | `dropped + reason` (DAG pass lifecycle) — round demarcation/history → `moved-to-lead` (round counter + workspace round artifacts) |
| A29 | Completion: output summary (feature, passes, verdict, milestone, artifacts) | `kept` (lead outputs summary) |
| A29′ | Completion: "update context status to `completed`" | `dropped + reason` (context-file status field absorbed; state-recovery-from-workspace instead) |
| A29″ | Next-steps pointer → `/humaninloop:plan` | `kept-but-rebind` (→ `/mochiko:plan` **reference stub** — plan not ported; setup's "not ported yet" precedent) |
| A30 | Context-Protection: **NEVER read domain agent reports directly** | `dropped + reason` (kernel context-isolation rule; in mochiko the lead **reads `spec.md` + the advocate report directly** to own the verdict — the middle-man that required this isolation is gone. Deliberate reversal; reason logged for the human gate.) |
| A31 | Context-Protection: NEVER call `hil-dag` / zero CLI; ALWAYS parse-and-advance / brief-and-assemble | `dropped + reason` (analyst/kernel protocol) |
| A32 | Responsibility-Boundaries table (Supervisor vs State-Analyst split) | `dropped + reason` (the division dissolves; lead owns all — individual capabilities traced in B) |
| A33 | Note: do NOT modify git config / push | `kept` |
| A34 | Note: always use Task tool to invoke agents | `kept` |
| A35 | Note: "Domain agents have NO workflow knowledge — all context via files on disk" | `kept-but-rebind` (the **decoupling/agent-self-containment convention** survives; rebind "files on disk" to the mochiko workspace + in-session brief) |

### B. `state-analyst` orchestration re-homed via this command's redesign

Per the run mandate: every orchestration responsibility the dissolving `state-analyst` owns is traced here to a rehome target on the lead — **`moved-to-lead`, never dropped** for the *capability*; only the *kernel mechanism* is `dropped + reason`. (P4 is assessed separately; this command is its rehome destination — coordinate in reconcile, flag RF1.)

| # | State-analyst responsibility | Tag |
|---|------------------------------|-----|
| B1 | Produce briefing (state_summary, outcome_trajectory, pass_context, relevant_patterns) | `moved-to-lead` (lead tracks round state in-session) |
| B2 | Gap classification (knowledge / preference / scope) | `moved-to-lead` *(tentative)* — may instead fold into the advocate's `analysis-specifications`; **RF3** decides |
| B3 | Ranked recommendations / next-node selection from catalog | `dropped + reason` (DAG node-menu selection; mochiko's loop is a fixed produce→validate — no node catalog) |
| B4 | Report parsing (extract verdict/gaps/summary from advocate & analyst reports) | `moved-to-lead` (lead reads reports directly; structured by report templates P12/P13) |
| B5 | Node assembly via `hil-dag` | `dropped + reason` (DAG/MCP) |
| B6 | Pass freezing via `hil-dag` | `dropped + reason` (mechanism) — round demarcation `moved-to-lead` (= A28) |
| B7 | Status updates via `hil-dag` | `dropped + reason` (DAG/MCP) |
| B8 | NL prompt construction (point agents at workspace artifacts, minimal brief) | `moved-to-lead` (lead writes producer/validator dispatch prompts; the decoupled-prompt convention survives — `agent-dispatch` guide) |
| B9 | Graph invariant validation | `dropped + reason` (DAG kernel) |
| B10 | Constitution prerequisite (INV-002 `carry_forward` auto-resolution) | mechanism `dropped + reason`; capability `moved-to-lead` (lead checks `.mochiko/memory/constitution.md` exists — prerequisite handoff edge) |
| B11 | **Convergence-watch** (outcome_trajectory / gaps trend across passes) | `moved-to-lead` (lead's no-progress guard — = A25) |
| B12 | 5-pass cap awareness (`pass_context`) | `moved-to-lead` (lead's hard round cap — = A26) |
| B13 | Escalation surfacing on non-resolvable error | `moved-to-lead` (lead's escalate-don't-silently-die — = A27) |
| B14 | Strategy-skill consumption (`strategy-core`, `strategy-specification`, `strategy-implementation`) | **RF3** — `strategy-specification`/`strategy-core` patterns dedupe vs `loop-discipline` + residual `moved-to-lead`; `strategy-implementation` is out of specify-core scope (other cluster) |

### C. Gaps the rehome must ADD (not relocate — these never existed in HIL)

| # | New responsibility (loop-discipline) | Tag |
|---|--------------------------------------|-----|
| C1 | Pre-declared **default-FAIL** done-condition + inlined `workflow-contract` | `moved-to-lead` (NEW — rehome-orchestration, RF6) |
| C2 | Lead **owns the verdict** (replaces analyst-autonomous gate evaluation) | `moved-to-lead` (NEW posture; see A18′) |
| C3 | Lead-owned **kill-switch / budget** (sentinel file, checked before each dispatch) | `moved-to-lead` (NEW — HIL had none) |
| C4 | **Human ACCEPTANCE gate** on the validated spec (setup G3 precedent) | `moved-to-lead` (NEW — the headline missing gate) |

> **Drops requiring human-gate acceptance (summary for the lead):** all `dropped + reason` items are kernel/DAG/catalog/MCP/brain-script plumbing (A3, A4, A5, A6, A8, A9, A10, A11, A12, A15, A16, A17, A19, A20, A21, A28[mech], A29′, A30, A31, A32, B3, B5, B6[mech], B7, B9, B10[mech]) — **no domain capability among them is lost**; each capability they carried is re-homed (A8′, A12′, A17′, A18′, B-series, C-series). The one judgment-call drop is **A30** (reversing "never read agent reports"): justified because the analyst middle-man that required context-isolation is gone and the lead must read the artifact to own the verdict (loop-discipline req 2). Recommend the Phase-2 reconcile human gate accept these.

---

## Reconcile flags (relational signals for `reconcile-cluster`)

- **RF1 — P4 `state-analyst` dissolution → this command (lead).** Confirm dissolve/absorb-into-lead; reconcile P4's own trace against Section B so every SA capability lands exactly once on the lead (no double-drop, no orphan). This command is P4's rehome destination.
- **RF2 — P2↔P3 pairing.** Confirm `requirements-analyst` (producer) ↔ `devils-advocate` (validator) as the loop's structurally-independent pair; record **verdict ownership moving to the lead** (A18′/C2) in the rehome map. Independence holds — do **not** collapse onto one agent.
- **RF3 — P9 `strategy-specification` + P10 `strategy-core` (+ B2 gap-classification).** Dedupe vs `loop-discipline`; decide which residual spec-loop patterns (knowledge-gap-before-preference, severity ordering, stall heuristic, gap-classification) become **lead heuristics** vs the **advocate's `analysis-specifications` skill** vs `dropped` (loop-discipline duplicate). `strategy-implementation` is out of specify-core scope (peer cluster).
- **RF4 — P14 `context-template` absorb-into-lead.** Confirm in-session + `.mochiko/` workspace, no standalone context-handoff file (setup memory-model precedent); A30's "never read reports" isolation drops with it.
- **RF5 — templates P11/P12/P13 handoff edges.** Wire `spec-template` (producer authoring), `analyst-report-template` (producer report), `advocate-report-template` (validator report) as lead-pointed handoff edges; confirm ported.
- **RF6 — rehome-orchestration / MISSING gates (the real transformation).** Reconcile Job 2 must **add**, not relocate: C1 default-FAIL contract, C2 lead-owned verdict, C3 kill-switch, C4 human acceptance gate, plus the lead-owned hard round cap + no-progress (A25/A26). These are the loop gaps from Check 7.
- **RF7 — path/workspace rebinding.** Decide the mochiko feature-workspace path (HIL `specs/{feature-id}/.workflow/` → mochiko equivalent) and the constitution prerequisite rebind (`.humaninloop/` → `.mochiko/memory/constitution.md`).

**Run-goal notes (for the lead, not blocking this assessment):**
- *Decoupling-doctrine test:* the command body is **clean by construction** (a lead may name agents, say "dispatch," and have phases). The empirical risk lives in P2/P3/P4 personas + P5–P10 skills — verify the redesigned command does **not** inject its workflow vocabulary into those personas.
- *Empirical structural confirmation:* specify's loop **confirms setup's calls** — human-gate placement (acceptance + escalations) and the memory model (in-session + `.mochiko/` workspace, context-handoff absorbed into lead). Eligible for ROADMAP promotion if reconcile agrees.

---

## Output-format summary

```
ASSESSMENT: specify command (P1)
Class:        command → branch IS-a-loop
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  redesign × rewire-cluster
              (body redesign firm now; structural rehome map depends on siblings → flags below)
Trace:        complete — 35 command responsibilities (A) + 14 state-analyst orchestration
              responsibilities (B) + 4 added loop-discipline gaps (C); every item tagged.
              Capabilities re-home (moved-to-lead / kept / kept-but-rebind); only kernel/DAG/
              catalog/MCP/brain-script MECHANISM is dropped+reason. No domain capability lost.
Reconcile flags: RF1 state-analyst→lead · RF2 analyst↔advocate pair + verdict-to-lead ·
              RF3 strategy dedupe vs loop-discipline · RF4 context-template absorb ·
              RF5 template handoff edges · RF6 ADD default-FAIL/verdict/kill-switch/acceptance-gate ·
              RF7 path rebinding
```
