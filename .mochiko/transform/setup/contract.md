# Workflow Contract — transform-cluster (run: setup)

**Workflow:** transform-cluster · **Carrier:** `commands/transform-cluster.md` supervisor (this run) · **Filled:** 2026-06-27

## 1. Done-condition (DEFAULTS TO FAIL)

- **Measurable end state:** For **every** primitive in the resolved `setup` cluster, `verify-output` returns **PASS** (five-convention conformance + sound-loop placement + trace audit), AND every original responsibility carries a trace tag with no silent drops, AND the human gate has accepted any `redesign`/`absorb`/`promote` dispositions and any `dropped` reasons.
- **Stated check (how it is proven):** the `transform-validator` agent **Reads each transformed artifact + its realized responsibility trace** and confirms conformance; the supervisor reconciles the per-primitive verdicts against this done-condition. Running out of rounds is FAIL-and-escalate, never "done."
- **Constraints (must not be violated):** no original responsibility silently dropped; kernel-free maintained (no Python/MCP/DAG/catalog dependency reintroduced); cross-cutting skills `syncing-claude-md` + `authoring-roadmap` are **rebound by reference only**, not ported this run; `brownfield-integration` is **deferred** (out of scope this run).
- **Initial state:** `FAIL`

## 2. Producer ↔ Validator (independence on two axes)

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | `mochiko:transform-producer` | `assess-primitive`, `reconcile-cluster`, `transform-recipes` | emits assessments, the rehome map, and the transformed artifacts |
| **Validator** | `mochiko:transform-validator` | `verify-output` | grades from each artifact itself, never the producer's report |

- **Independence check:** producer agent (`transform-producer`) ≠ validator agent (`transform-validator`) ✓ **AND** producer skills `{assess-primitive, reconcile-cluster, transform-recipes}` ∩ validator skill `{verify-output}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the artifact allows). The bulk of the done-condition — five-convention conformance, sound-loop placement, trace completeness — is judged by reading markdown, not machine-checkable. Tier-1 deterministic sub-checks are layered in where they exist: file exists, frontmatter parses, `grep` finds zero leftover brain/DAG/MCP references, declared `skills:` resolve to ported skills.
- **Tamper-proofing:** no PASS unless the validator **Read the real transformed artifact file this run** and cites evidence quoted from it. A summary-read or "looks like it passes" is an automatic FAIL. Validator defaults to FAIL.

## 3. Bounded iteration

- **Hard round cap:** **3** produce↔validate rounds **per primitive**, counted by the supervisor.
- **No-progress exit:** a round whose set of failing items is unchanged from the previous round (same primitive, same conformance/trace failures) — stop and escalate that primitive.
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/transform/setup/STOP` (checked before each dispatch batch); plus a run ceiling — if cumulative produce↔validate rounds across the cluster exceed **2× the primitive count (≥20)**, halt and escalate. Either trigger is an escalation, never a "done."
- **On hitting a guard:** escalate to the human gate via `AskUserQuestion` with the failing items and failure context. Never report done on cap/no-progress/budget exhaustion.

## 4. Human gate

- **Placement:** **on gated dispositions + escalations** (user-selected; the `loop-discipline` default).
- **Where it fires:** (a) Phase 2 reconcile — before applying any `redesign` / `absorb-into-lead` / `promote` / `dropped` decision; (b) any Phase 4 escalation — round-cap exhaustion, no-progress exit, or budget/kill-switch trip.
- **What the human decides:** accept / override / send-back each gated disposition; accept or reject any dropped-responsibility reason; clear the gated dispositions before the run is declared DONE. The supervisor owns arbitration and the final verdict.

---

**Contract version:** v1 · **Governed by:** `loop-discipline`
