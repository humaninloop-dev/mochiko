# Workflow Contract — transform-cluster (run: specify)

**Workflow:** transform-cluster · **Carrier:** `commands/transform-cluster.md` supervisor (this run) · **Filled:** 2026-06-27

## 1. Done-condition (DEFAULTS TO FAIL)

- **Measurable end state:** For **every** primitive in the resolved `specify` cluster (14, see `context.md`), `verify-output` returns **PASS** (five-convention conformance + sound-loop placement + trace audit), AND every original responsibility carries a realized trace tag with no silent drops, AND the human gate has accepted any `redesign` / `absorb` / `promote` / `dropped` dispositions.
- **Stated check (how it is proven):** the `mochiko:validator` agent **Reads each transformed artifact + its realized responsibility trace** and confirms conformance; the supervisor reconciles per-primitive verdicts against this done-condition. Running out of rounds is FAIL-and-escalate, never "done."
- **Constraints (must not be violated):**
  - No original responsibility silently dropped — **especially `state-analyst`'s non-DAG responsibilities** (briefing, recommendation, convergence-watching, report-parsing) must be **rehomed to the lead**, not dropped with the DAG machinery.
  - Kernel-free maintained — no Python/MCP/DAG/catalog dependency reintroduced; `specify-catalog.json` + brain scripts stay **excluded**.
  - **Decoupling doctrine holds** (this run is its empirical test) — no ported persona or skill acquires a deny-list token (sibling-agent name, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-label). `verify-output`'s decoupling scan + keystone test must catch any that slip in.
  - Deferred primitives rebound **by reference only**, not ported this run: `ui-designer` + design track (user-selected core-only); `devils-advocate`'s `validation-plan-artifacts` + `validation-task-artifacts` skills (stub, re-mount when plan/tasks port).
- **Initial state:** `FAIL`

## 2. Producer ↔ Validator (independence on two axes)

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | `mochiko:transform-producer` | `assess-primitive`, `reconcile-cluster`, `transform-recipes` | emits assessments, the rehome map, and the transformed artifacts |
| **Validator** | `mochiko:validator` | `verify-output` | grades from each artifact itself, never the producer's report |

- **Independence check:** producer agent (`transform-producer`) ≠ validator agent (`mochiko:validator`) ✓ **AND** producer skills `{assess-primitive, reconcile-cluster, transform-recipes}` ∩ validator skill `{verify-output}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the artifact allows). Five-convention conformance, sound-loop placement, and trace completeness are judged by reading markdown. Tier-1 deterministic sub-checks are layered where they exist: file exists, frontmatter parses, `grep` finds zero leftover brain/DAG/MCP references, declared `skills:` resolve to ported skills, decoupling deny-list `grep` is clean.
- **Tamper-proofing:** no PASS unless the validator **Read the real transformed artifact file this run** and cites evidence quoted from it. A summary-read or "looks like it passes" is an automatic FAIL. Validator defaults to FAIL.

## 3. Bounded iteration

- **Hard round cap:** **3** produce↔validate rounds **per primitive**, counted by the supervisor.
- **No-progress exit:** a round whose set of failing items is unchanged from the previous round (same primitive, same conformance/trace failures) — stop and escalate that primitive.
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/transform/specify/STOP` (checked before each dispatch batch); plus a run ceiling — if cumulative produce↔validate rounds across the cluster exceed **2× the primitive count (≥28)**, halt and escalate. Either trigger is an escalation, never a "done."
- **On hitting a guard:** escalate to the human gate via `AskUserQuestion` with the failing items and failure context. Never report done on cap/no-progress/budget exhaustion.

## 4. Human gate

- **Placement:** **on gated dispositions + escalations** (the `loop-discipline` default; the placement BACKLOG asks `specify` to confirm against `setup`).
- **Where it fires:** (a) Phase 2 reconcile — before applying any `redesign` / `absorb-into-lead` / `promote` / `dropped` decision (notably the **`state-analyst` dissolution**, the **`strategy-core` / `strategy-specification` dedupe vs `loop-discipline`**, and the **`context-template` absorb**); (b) any Phase 4 escalation — round-cap exhaustion, no-progress exit, or budget/kill-switch trip.
- **What the human decides:** accept / override / send-back each gated disposition; accept or reject any dropped-responsibility reason; clear the gated dispositions before the run is declared DONE. The supervisor owns arbitration and the final verdict.

---

**Contract version:** v1 · **Governed by:** `loop-discipline`
