# Workflow Contract — transform-cluster (run: plan)

**Workflow:** transform-cluster · **Carrier:** `commands/transform-cluster.md` supervisor (this run) · **Filled:** 2026-06-30

## 1. Done-condition (DEFAULTS TO FAIL)

- **Measurable end state:** For **every** primitive in the resolved `plan` cluster (15, see `context.md`), `verify-output` returns **PASS** (five-convention conformance + sound-loop placement + command-altitude scan + trace audit), AND every original responsibility carries a realized trace tag with no silent drops, AND the human gate has accepted any `redesign` / `absorb` / `promote` / `dropped` dispositions — **including the reviewer-architecture decision (RQ1) and the principal-architect role decision (RQ2)** named below.
- **Stated check (how it is proven):** the `mochiko:validator` agent **Reads each transformed artifact + its realized responsibility trace** and confirms conformance; the supervisor reconciles per-primitive verdicts against this done-condition. Running out of rounds is FAIL-and-escalate, never "done."
- **Constraints (must not be violated):**
  - No original responsibility silently dropped — **especially the plan command's supervisor orchestration**: the two-phase sequence (analysis → design), the **architect-feasibility-once-then-advocate-completeness** ordering, the *skip-architect-re-review-unless-structural-change* routing, and the Phase-3 *incremental* review (full design review + 2-3 min consistency check) must be **rehomed to the lead**, not dropped with the brain-prose supervisor.
  - Kernel-free maintained — no Python/MCP/DAG/catalog dependency reintroduced; no brain scripts; HIL `.workflow/`-state carriers absorbed, not transliterated.
  - **Command altitude held** (the 2026-06-30 rule, `.mochiko/brainstorms/command-altitude/synthesis.md`) — the ported `plan` command *references* `loop-discipline` + `workflow-contract`, never restates them; no inlined filled contract, no transliterated `Task()`/`AskUserQuestion()` payloads, no "Supervisor behaviors" doctrine footer. This is the **third** command port and the **first net-new command since the altitude fix** — it must come out thin by construction. `verify-output`'s altitude gate (item 8) must PASS.
  - **Decoupling doctrine holds** (this run continues the empirical test) — no ported persona or skill acquires a deny-list token (sibling-agent name, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-label). Highest-risk surface this run: the **`technical-analyst`** persona (rich, multi-artifact). `verify-output`'s decoupling scan + keystone test must catch any that slip in.
  - Deferred primitives rebound **by reference only**, not ported this run (user-selected core-only): design-track (`patterns-flow-mapping`, `patterns-interface-design`, `ui-designer`), `authoring-roadmap` + `evolution-roadmap-template` (setup-brownfield), `patterns-vertical-tdd` (tasks). `techspec` stays `[-]` (deprecated; merged into `plan` upstream).
- **Initial state:** `FAIL`

## 2. Producer ↔ Validator (independence on two axes)

This contract governs the **transform run's** loop (the meta-loop). The *plan workflow's own* producer↔validator pair (technical-analyst ↔ its reviewer[s]) is the subject of RQ1/RQ2 and is decided inside the run, gated by the human.

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | `mochiko:transform-producer` | `assess-primitive`, `reconcile-cluster`, `transform-recipes` | emits assessments, the rehome map, and the transformed artifacts |
| **Validator** | `mochiko:validator` | `verify-output` | grades from each artifact itself, never the producer's report |

- **Independence check:** producer agent (`transform-producer`) ≠ validator agent (`mochiko:validator`) ✓ **AND** producer skills `{assess-primitive, reconcile-cluster, transform-recipes}` ∩ validator skill `{verify-output}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the artifact allows). Five-convention conformance, sound-loop placement, altitude, and trace completeness are judged by reading markdown. Tier-1 deterministic sub-checks are layered where they exist: file exists, frontmatter parses, `grep` finds zero leftover brain/DAG/MCP references, declared `skills:` resolve to ported skills, decoupling deny-list `grep` is clean, altitude grep-floor (no inlined contract / restated doctrine).
- **Tamper-proofing:** no PASS unless the validator **Read the real transformed artifact file this run** and cites evidence quoted from it. A summary-read or "looks like it passes" is an automatic FAIL. Validator defaults to FAIL.

## 3. Bounded iteration

- **Hard round cap:** **3** produce↔validate rounds **per primitive**, counted by the supervisor.
- **No-progress exit:** a round whose set of failing items is unchanged from the previous round (same primitive, same conformance/altitude/trace failures) — stop and escalate that primitive.
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/transform/plan/STOP` (checked before each dispatch batch); plus a run ceiling — if cumulative produce↔validate rounds across the cluster exceed **2× the primitive count (≥30)**, halt and escalate. Either trigger is an escalation, never a "done."
- **On hitting a guard:** escalate to the human gate via `AskUserQuestion` with the failing items and failure context. Never report done on cap/no-progress/budget exhaustion.

## 4. Human gate

- **Placement:** **on gated dispositions + escalations** (the `loop-discipline` default; confirmed by setup + specify, now a ROADMAP Key Decision).
- **Where it fires:**
  - **(a) Phase 2 reconcile — the structural decisions**, before applying any `redesign` / `absorb-into-lead` / `promote` / `dropped` decision. The gated bundle this run centers on two named reconciliation questions:
    - **RQ1 — reviewer architecture.** HIL plan has **two** independent reviewers (principal-architect = *feasibility / cross-artifact contradiction*; devils-advocate = *completeness*, via `validation-plan-artifacts`). Decide the mochiko shape: keep two distinct validators (a **checklist** advocate + an **adversarial-critique** architect — the convention-5 two-form case, which `plan` would be the first cluster to exercise), fold feasibility into the advocate (one reviewer, the `specify` shape), or rehome feasibility onto the generic `validator`. The producer↔validator convention for `plan` rides on this.
    - **RQ2 — principal-architect's role.** It is already ported **producer-only** (setup: authors the constitution). `plan` uses it as a **reviewer**. Decide whether one workflow-agnostic persona legitimately produces in one workflow and reviews in another (decoupling-by-absence says yes), and what skill — if any — drives the feasibility review (a new reviewing skill, or persona-only "competent when no skill fits").
    - Plus the routine gated items: P1 command `redesign`, P11 `plan-context-template` expected `absorb-into-lead`, P14 `cross-artifact-checklist` fate (standalone vs fold into `validation-plan-artifacts`), P15 `advocate-report-template` reuse, and all dropped-responsibility reasons.
  - **(b) any Phase 4 escalation** — round-cap exhaustion, no-progress exit, or budget/kill-switch trip.
- **What the human decides:** accept / override / send-back each gated disposition (RQ1 + RQ2 first); accept or reject any dropped-responsibility reason; clear the gated dispositions before the run is declared DONE. The supervisor owns arbitration and the final verdict.

---

**Contract version:** v1 · **Governed by:** `loop-discipline`
