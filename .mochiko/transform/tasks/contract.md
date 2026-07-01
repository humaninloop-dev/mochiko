# Workflow Contract — transform-cluster (run: tasks)

**Workflow:** transform-cluster · **Carrier:** `commands/transform-cluster.md` supervisor (this run) · **Filled:** 2026-07-01

## 1. Done-condition (DEFAULTS TO FAIL)

- **Measurable end state:** For **every** primitive in the resolved `tasks` cluster (6, see `context.md`), `verify-output` returns **PASS** (five-convention conformance + sound-loop placement + command-altitude scan + trace audit), AND every original responsibility carries a realized trace tag with no silent drops, AND the human gate has accepted any `redesign` / `absorb` / `promote` / `dropped` dispositions — **including the artifact-shape decision (RQ-A) named below**.
- **Stated check (how it is proven):** the `mochiko:validator` agent **Reads each transformed artifact + its realized responsibility trace** and confirms conformance; the supervisor reconciles per-primitive verdicts against this done-condition. Running out of rounds is FAIL-and-escalate, never "done."
- **Constraints (must not be violated):**
  - No original responsibility silently dropped — **especially the tasks command's supervisor orchestration**: the two-phase sequence (Mapping → Tasks), the **cumulative review** (when reviewing `tasks.md`, cross-check back to `task-mapping.md`), the entry-gate (plan-workflow-complete prerequisite), and the clarification/exit-early routing must be **rehomed to the lead**, not dropped with the brain-prose supervisor.
  - Kernel-free maintained — no Python/MCP/DAG/catalog dependency reintroduced; no brain scripts; HIL `.workflow/tasks-context.md` state carrier absorbed into the lead (in-session + workspace-as-state), not transliterated.
  - **Command altitude held** (the 2026-06-30 rule, `COMMAND-ALTITUDE-SYNTHESIS.md`) — the ported `tasks` command *references* `loop-discipline` + `workflow-contract` + `agent-dispatch`, never restates them; no inlined filled contract, no transliterated `Task()`/`AskUserQuestion()` payloads, no "Supervisor behaviors" doctrine footer. This is the **fourth** command port and the **second net-new command since the altitude fix** — it must come out thin by construction (target ≤ ~90 lines, cf. `plan.md` at 82). `verify-output`'s altitude gate (item 8) must PASS.
  - **Decoupling doctrine holds** (this run continues the empirical test) — no ported persona or skill acquires a deny-list token (sibling-agent name, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-label). Highest-risk surface this run: the **`task-architect`** persona (rich, TDD-discipline-heavy, carries brownfield markers) and its "phase: mapping | tasks" language. `verify-output`'s decoupling scan + keystone test must catch any that slip in.
  - Deferred primitives rebound **by reference only**, not ported this run (user-selected core-only): the roadmap track (`authoring-roadmap` + `evolution-roadmap-template`) stays deferred — the task-architect's brownfield `evolution-roadmap.md` / `[GAP:XXX]` read is a **documented stub**, not a live mount (as setup stubbed `syncing-claude-md`); `executing-tdd-cycle`, `staff-engineer`, `qa-engineer`, `testing-end-user`, `brownfield-integration` all stay `implement`-cluster.
- **Initial state:** `FAIL`

## 2. Producer ↔ Validator (independence on two axes)

This contract governs the **transform run's** loop (the meta-loop). The *tasks workflow's own* producer↔reviewer pair (`task-architect` ↔ `devils-advocate`+`validation-task-artifacts`) is a **single-reviewer** loop (the `specify` shape, not `plan`'s two-form) and is realized inside the run, gated by the human at reconcile.

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
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/transform/tasks/STOP` (checked before each dispatch batch); plus a run ceiling — if cumulative produce↔validate rounds across the cluster exceed **2× the primitive count (≥12)**, halt and escalate. Either trigger is an escalation, never a "done."
- **On hitting a guard:** escalate to the human gate via `AskUserQuestion` with the failing items and failure context. Never report done on cap/no-progress/budget exhaustion.

## 4. Human gate

- **Placement:** **on gated dispositions + escalations** (the `loop-discipline` default; user-confirmed at intake; a ROADMAP Key Decision).
- **Where it fires:**
  - **(a) Phase 2 reconcile — the structural decisions**, before applying any `redesign` / `absorb-into-lead` / `promote` / `dropped` decision. The gated bundle this run centers on one named reconciliation question:
    - **RQ-A — artifact shape (one deliverable or two).** HIL tasks produces **two** artifacts across two phases: `task-mapping.md` (story→cycle mapping + slice rationale) then `tasks.md` (cycle→TDD-tasks). But `tasks.md` already carries a "Story → Cycle Mapping" table and per-cycle `> Stories:` traceability. Decide the mochiko shape: **keep two phases / two artifacts** (matches HIL + `plan`'s two-phase precedent; preserves the mapping-review gate + slice rationale as a distinct reviewed artifact) vs. **collapse** the mapping into `tasks.md` (one artifact, one review; mapping becomes a section). The two-phase-vs-one and the `task-architect`'s phase-behaviors ride on this.
    - Plus the routine gated items: P1 command `redesign` (thin sound-loop), P6 `tasks-context-template` expected `absorb-into-lead`, the producer-report home (inline persona self-disclosure vs. a `taskarchitect-report-template`), the `devils-advocate` `validation-task-artifacts` re-mount, and all dropped-responsibility reasons.
  - **(b) any Phase 4 escalation** — round-cap exhaustion, no-progress exit, or budget/kill-switch trip.
- **What the human decides:** accept / override / send-back each gated disposition (RQ-A first); accept or reject any dropped-responsibility reason; clear the gated dispositions before the run is declared DONE. The supervisor owns arbitration and the final verdict.

---

**Contract version:** v1 · **Governed by:** `loop-discipline`
