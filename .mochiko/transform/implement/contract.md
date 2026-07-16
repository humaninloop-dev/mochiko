# Workflow Contract — transform-cluster (run: implement)

**Workflow:** transform-cluster · **Carrier:** `commands/transform-cluster.md` supervisor (this run) · **Filled:** 2026-07-01

## 1. Done-condition (DEFAULTS TO FAIL)

- **Measurable end state:** For **every** primitive in the resolved `implement` cluster (7, see `context.md`), `verify-output` returns **PASS** (five-convention conformance + sound-loop placement + command-altitude scan + trace audit), AND every original responsibility carries a realized trace tag with no silent drops, AND the human gate has accepted every gated disposition (`redesign` / `absorb-into-lead` / `dissolve` / `dropped`) — **including the correct realization of the two intake design decisions** (sequential orchestration; confidence-based+final impl-gate).
- **Stated check (how it is proven):** the `mochiko:validator` agent **Reads each transformed artifact + its realized responsibility trace** and confirms conformance; the supervisor reconciles per-primitive verdicts against this done-condition. Running out of rounds is FAIL-and-escalate, never "done."
- **Constraints (must not be violated):**
  - **No original responsibility silently dropped — especially the implement command's orchestration**, which is the largest of any port. All of the following must be **rehomed to the thin lead**, not dropped with the dissolving DAG-Supervisor + State-Analyst + `strategy-implementation`: (a) **sequential cycle sequencing** (foundation cycles before feature cycles; current cycle = first with unchecked `tasks.md` tasks); (b) **execute-then-verify pairing** (every staff-engineer cycle is followed by a qa-engineer verification in the same round — never skipped); (c) **targeted retry** (trace a checkpoint failure to its tasks, re-open only those, **max 3 attempts per cycle**); (d) **fix-pass mode** (after final-validation failure: unconstrained by cycle boundaries, scoped strictly to reported failures, max 3 fix passes); (e) **convergence-stall detection** (same failure pattern 2+ rounds → surface, don't silently continue); (f) the **entry gate** (tasks-workflow-complete prerequisite: `tasks.md` present/complete before implementing); (g) **project scaffolding** (ignore-file creation from detected stack).
  - **Kernel-free maintained** — no Python/MCP/DAG/catalog/invariant dependency reintroduced; `implement-catalog.json` stays excluded; no brain scripts (`check-prerequisites.sh`); the HIL `.workflow/context.md` state carrier is absorbed into the lead (in-session + workspace-as-state under `.mochiko/specs/<feature>/`), not transliterated; no `hil-dag` verbs.
  - **Orchestration decision honored (user, this run):** **sequential-first.** Cycles execute in dependency order under a thin prose lead with Task-subagent dispatch. **No** native-Workflow-`pipeline()`/`parallel()`, **no** artifact-DAG. Parallelism is a documented `deliberate-shortcut-ledger` deferral, not a silent drop. Realized in the P1 redesign; verified by the validator.
  - **Impl human-gate placement honored (user, this run):** **confidence-based + final acceptance.** The redesigned command's designed-in gate = deterministic CLI verifications that 100% pass auto-approve; GUI / subjective / any-failure always checkpoint; named final acceptance gate before the workflow reports done. (Distinct from §4 — that gate governs *this transform run*; this constraint governs the *artifact being produced*.)
  - **Command altitude held** (the 2026-06-30 rule, `.mochiko/brainstorms/command-altitude/synthesis.md`) — the ported `implement` command *references* `loop-discipline` + `workflow-contract` + `agent-dispatch`, never restates them; no inlined filled contract, no transliterated `Task()`/`AskUserQuestion()` payloads, no "Supervisor behaviors" doctrine footer. This is the **fifth** command port. **Nuance:** `implement` legitimately carries the most workflow-specific machinery of any command (entry gate + scaffolding + sequential cycle loop + per-cycle execute→verify + targeted retry + fix-pass + two agents + a designed-in multi-tier gate), so a modestly higher line count than `tasks`/`plan` (77/82) may be genuinely workflow-specific rather than restated doctrine — `verify-output`'s altitude item 8 (grep-floor + keystone-ceiling) adjudicates line-by-line, not by raw count.
  - **Decoupling doctrine holds** (5th empirical test) — no ported persona/skill acquires a deny-list token (sibling-agent name, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-label). **Highest-risk surfaces this run:** `staff-engineer`'s **"Two Execution Modes / Cycle Mode / Fix Mode"** section (fix-mode is this-workflow machinery — decouple to intrinsic craft or rehome to the lead) and its `context.md`-reading examples; `qa-engineer`'s "dispatched as part of an implementation cycle verification" framing; `strategy-implementation`'s entire body (dissolves). `verify-output`'s decoupling scan + keystone test must catch any that slip in.
  - **Boundary integrity** — the `patterns-vertical-tdd` (design-time task *structuring*) ↔ `executing-tdd-cycle` (runtime cycle *execution*) split must hold: shared discipline may be referenced, substance must not be duplicated (`grep cycle-report` in patterns-vertical-tdd stays 0; the `**TEST:**` grammar stays owned by patterns-vertical-tdd, *consumed* by testing-end-user).
- **Initial state:** `FAIL`

## 2. Producer ↔ Validator (independence on two axes)

This contract governs the **transform run's** loop (the meta-loop). The *implement workflow's own* producer↔validator pair (`staff-engineer` ↔ `qa-engineer`) is a **single-reviewer, objective/mirror-checklist** loop (the `specify`/`tasks` shape, not `plan`'s two-form) whose validator is unusually strong — **Tier 1 (deterministic ground truth):** qa verifies against real infrastructure with captured evidence + quality-gate exit codes. That pair is *designed* inside this run and gated by the human at reconcile; it is not the loop this contract bounds.

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | `mochiko:transform-producer` | `assess-primitive`, `reconcile-cluster`, `transform-recipes` | emits assessments, the rehome map, and the transformed artifacts |
| **Validator** | `mochiko:validator` | `verify-output` | grades from each artifact itself, never the producer's report |

- **Independence check:** producer agent (`transform-producer`) ≠ validator agent (`mochiko:validator`) ✓ **AND** producer skills `{assess-primitive, reconcile-cluster, transform-recipes}` ∩ validator skill `{verify-output}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the transform artifact allows; markdown conformance is not machine-decidable). Tier-1 deterministic sub-checks are layered where they exist: file exists, frontmatter parses, `grep` finds zero leftover brain/DAG/MCP/catalog/`hil-dag` references, declared `skills:` resolve to ported skills, decoupling deny-list `grep` is clean, altitude grep-floor (no inlined contract / restated doctrine), `grep cycle-report` boundary check.
- **Tamper-proofing:** no PASS unless the validator **Read the real transformed artifact file this run** and cites evidence quoted from it. A summary-read or "looks like it passes" is an automatic FAIL. Validator defaults to FAIL.

## 3. Bounded iteration

- **Hard round cap:** **3** produce↔validate rounds **per primitive**, counted by the supervisor.
- **No-progress exit:** a round whose set of failing items is unchanged from the previous round (same primitive, same conformance/altitude/decoupling/trace failures) — stop and escalate that primitive.
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/transform/implement/STOP` (checked before each dispatch batch); plus a run ceiling — if cumulative produce↔validate rounds across the cluster exceed **2× the primitive count (≥14)**, halt and escalate. Either trigger is an escalation, never a "done."
- **On hitting a guard:** escalate to the human gate via `AskUserQuestion` with the failing items and failure context. Never report done on cap/no-progress/budget exhaustion.

## 4. Human gate

- **Placement:** **on gated dispositions + escalations** (the `loop-discipline` default; a ROADMAP Key Decision; ran cleanly across setup + specify + plan + tasks).
- **Where it fires:**
  - **(a) Phase 2 reconcile — the structural decisions**, before applying any `redesign` / `absorb-into-lead` / `dissolve` / `dropped`. The two heaviest design calls were **pre-answered at intake** (orchestration=sequential; impl-gate=confidence-based+final), so the reconcile gate **confirms their correct realization** rather than re-litigating them. The gated bundle this run centers on:
    - **The command redesign + the rehome map** — the largest orchestration rehome of any port: the dissolving DAG-Supervisor + State-Analyst + `strategy-implementation` must land constraints (a)–(g) from §1 onto the thin lead, sequential-only, with the confidence-based+final gate wired in. This is the run's pivotal decision.
    - **`strategy-implementation` dissolution** into `loop-discipline` + which survivors rehome to the lead vs. are already covered by the doctrine.
    - **`staff-engineer` fix-mode decoupling** — how the two-execution-modes machinery is decoupled (intrinsic craft kept; workflow-routing rehomed to the lead).
    - **`approved-domain-deps` disposition** — deferred / reference-stub (scope=core-only).
    - all dropped-responsibility reasons.
  - **(b) any Phase 4 escalation** — round-cap exhaustion, no-progress exit, or budget/kill-switch trip.
- **What the human decides:** accept / override / send-back each gated disposition; accept or reject any dropped-responsibility reason; confirm the two intake decisions are correctly realized; clear the gated dispositions before the run is declared DONE. The supervisor owns arbitration and the final verdict.

---

**Loop-discipline four-requirement audit (Phase 0 confirmation):**
1. **Pre-declared done-condition, default FAIL** ✓ — §1: measurable end state (all 7 PASS + trace + gate) + stated check (validator Reads artifact) + constraints; initial state FAIL.
2. **External, independent validation** ✓ — §2: producer agent ≠ validator agent; producer skills ∩ validator skill = ∅; Tier 2 + layered Tier-1 sub-checks; tamper-proofed on evidence-Read.
3. **Bounded iteration** ✓ — §3: hard cap 3/primitive; no-progress exit; STOP sentinel + 14-round ceiling; escalate-don't-die.
4. **Defined human gate** ✓ — §4: placement (gated dispositions + escalations) + where it fires + what the human decides.

No open brackets. The loop is sound per `loop-discipline`.

**Contract version:** v1 · **Governed by:** `loop-discipline`
