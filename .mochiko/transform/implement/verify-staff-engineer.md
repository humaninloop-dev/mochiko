# Verify — `agents/staff-engineer.md` (P2)

**Skill:** `mochiko:verify-output` · **Grader:** independent `validator` (did NOT author the artifact) · **Date:** 2026-07-01
**Disposition graded:** `port-with-edits × standalone` (producer of the implement team)
**Bar:** the transform done-condition (5 conventions + sound-loop placement + kernel-free + complete trace) and the task's 5 grading points; Seam 3 (reconcile) is the fix-mode-split spec.

```
VERIFY: staff-engineer (P2)
Evidence read: plugins/mochiko/agents/staff-engineer.md (all 89 lines);
               .mochiko/transform/implement/transform-staff-engineer.md (producer claim — audited, not trusted);
               .mochiko/transform/implement/reconcile.md (Seam 3 spec);
               cross-checks: plugins/mochiko/agents/requirements-analyst.md + qa-engineer.md (house convention + independence);
               plugins/mochiko/commands/implement.md (moved-to-lead landing spot-check).
Deny-list greps: run BY THE VALIDATOR on the real artifact (not the producer's self-scan).

Conformance:
  1 Classification            PASS — frontmatter L37 `skills: executing-tdd-cycle, brownfield-integration`; agent = model-invoked by dispatch (no disable-model-invocation needed). Matches peer `requirements-analyst`.
  2 Discoverability           DEFERRED (documented Wave-2) — router index + plugin.json not this pass; agent is NOT orphaned — cast in `implement.md` lead (L8/L19/L39/L49). Non-blocking (see note).
  3 Reliable model-invocation PASS — 3 work-context `<example>` blocks (L9-34); no dispatch/context-file/mode. Same trigger mechanism as ported peers.
  4 Composition & DECOUPLING  PASS — persona carries self-sufficient craft+judgment; procedure lives in the 2 mounted skills. Deny-list grep = ZERO on all four surfaces (below).
  5 Producer↔validator pair   PASS — grading skill `testing-end-user` mounted ONLY on qa-engineer (L35), NEVER on staff (grep-confirmed). Different agents, disjoint skills. Verdict lead-owned (implement.md L8/L18/L80).
  6 Sound-loop placement      PASS — the loop it sits in (implement.md) invokes loop-discipline (L10), default-FAIL done-condition (L18), independent qa validation, bounded iteration/caps (L20), named human gate G5 (L18/L55).
  7 Kernel-free               PASS — no DAG/catalog/MCP/hil-dag/humaninloop: token (grep CLEAN).
  8 Altitude/single-source    PASS — cycle-report FORMAT + TDD/brownfield PROCEDURE referenced to skills (L46/L47/L65), schema/steps not restated.

Trace audit:
  Completeness      PASS — 18-row realized trace mirrors the assess trace; every P2 responsibility carries a realized tag.
  Justified drops   PASS — 5 drops (#5,#7-label,#8-label,#10,#11), each reasoned, all in the human-gate bundle (reconcile §9 P2).
  Landing integrity PASS — moved-to-lead items VERIFIED on implement.md (fix-pass trigger L49; "unconstrained by cycle boundaries / may touch files from any cycle" L49; max-3 L20/L49; casting L8/L19).
  No capability loss PASS — fix-mode ROUTING moved to the lead (exercised there); fix CRAFT kept on staff; nothing lost.

VERDICT: PASS
Failing items: none
Required fixes: none (one documented Wave-2 follow-up + one cosmetic note — neither blocks)
```

---

## 1. Decoupling — the crux (validator-run grep, not the producer's self-scan)

I ran the deny-list greps myself against the real file. **ZERO residual on all four surfaces.**

| Surface | Pattern class grepped | Result |
|---------|----------------------|--------|
| **Mode-section** | `two execution modes` · `cycle mode` · `fix mode` · `after validation failure` · `unconstrained` · `may touch` · `any cycle` · `cycle's scope` · `max 3` · `3 passes` · `3 retries` · `cycle boundar` · `normal)` | **CLEAN (0)** |
| **Context-file / dispatch** | `context.md` · `.workflow` · `supervisor` · `dispatch` · `read your instructions` · `your instructions` · `specs/001` · `locations specified` · `workflow-agnostic` | **CLEAN (0)** |
| **Sibling-agents + kernel** | `qa-engineer` · `state-analyst` · `task-architect` · other agent names · `humaninloop:` · `hil-dag` · `catalog` · `DAG` · `MCP` | **CLEAN (0)** |
| **Independence** | `testing-end-user` · `verify-*` · `validation-*` · `self-grade` | **CLEAN (0)** |

The HIL `## Two Execution Modes / Cycle Mode / Fix Mode` section is **GONE** — no skeleton, no mode-labels, no "After Validation Failure" trigger, no "unconstrained by cycle boundaries," no max-3 bounds. The 3 `<example>` blocks are reframed to pure work-context (task list to implement / reported failures to reproduce-then-fix / brownfield EXTEND) — no context-file mechanism, no "Supervisor dispatches," no sibling names, no injected paths/phases.

**Survivors judged (keystone test — all KEEP):**
- `cycle` ×3 (L37/L46/L65) = the mounted skill name `executing-tdd-cycle` and its own artifact `cycle-report.md`, *referenced*. Not "cycle mode / boundaries / scope." True of this engineer on any TDD job.
- `brownfield` ×4 (L32/L37/L47/L73) = intrinsic craft (working existing codebases) + the skill name. Not an injected greenfield/brownfield/amend run-scope. Reading before modifying is universal senior craft.
- `fix` ×4 (L23/L57/L58/L87) = intrinsic fix craft only. No mode-label, trigger, or bound.
- `phase` ×1 (L54) = the red **phase** of red/green/refactor (TDD craft), not a workflow phase.

**Corroboration (not the basis of the PASS):** the removed machinery is where it belongs — on the caller. `implement.md` L49 carries the fix-pass trigger and "unconstrained by cycle boundaries (may touch files from any cycle)"; L20/L49/L80 carry the max-3 caps. Callers (commands) are allowed to name agents and own workflow knowledge; personas are not. Clean split.

## 2. Fix-mode split correct (Seam 3)

**Intrinsic craft KEPT** (all three Seam-3 KEEP items present):
- Scope a fix tightly to reported failures — L58 "keep the change scoped to that failure"; L87 "keeping the fix scoped to that failure rather than a wider refactor"; example 2 (L21) "scoped to the reported failures, not a refactor of the surrounding code."
- Reproduce a bug with a failing test **before** fixing — L58 "reproduce each one with a failing test before touching the code"; L87 "Reproducing a reported failure with a failing test before fixing it."
- A fix follows the failure, **not** a refactor — L58 "follow the failure wherever it leads rather than treating it as a license to refactor."

**Machinery ABSENT** — trigger / bounds / mode-labels all zero (grep Check 1). **Keystone test:** what remains ("reproduce with a failing test before fixing, scope tightly, follow the failure not a refactor") is true of a senior engineer on **any** job. PASS.

## 3. Independence (convention 5)

- `skills:` = `executing-tdd-cycle, brownfield-integration` **only** (L37). No grading skill mounted.
- `testing-end-user` / any `verify-*` **NOT** mounted on staff — grep across `plugins/mochiko/agents/` shows `testing-end-user` on **qa-engineer only** (L35). Structural independence (staff produces ≠ qa grades; different agents, disjoint skills), not a self-declaration.
- Cycle-report framed as honest self-disclosure, NOT a clearing verdict — L65: "a truthful record of what actually happened… **not a verdict on whether the result passes**." The only `grade`/`verdict` tokens are this disclaimer and "does not grade its own output" (L6, independence-by-role, house-style shared with requirements-analyst/task-architect). No verdict leak.

## 4. Conventions

- **Classification** — agent + `skills:` list present (L37). PASS.
- **Description** — well-formed, declares role + producer boundary ("Produces the implementation; does not grade its own output," L6) + 3 work-context `<example>` triggers. Matches the house convention exactly (peer `requirements-analyst` L3-7/L37 is byte-for-pattern identical). Note on "RFC-2119": agents realize the model-invocation convention via `<example>` blocks, **not** literal MUST/SHOULD in the description — confirmed against peers (RFC-2119 keywords appear only in body skill-descriptions, never the agent description). Conformant at agent altitude. PASS.
- **Producer craft intact** — TDD discipline (Core Identity L54/L63, Quality Standards L71 "TDD rigor — every task… No exceptions"); Quality Standards (L68-74); What You Reject (L77-81); What You Embrace (L84-89). PASS.

## 5. Trace audit

- **Completeness** — 18-row realized trace covers every P2 responsibility with a realized tag; mirrors the assess trace + reconcile Job-3 P2. No silent drop.
- **Justified drops** — 5 drops (#5 output-location context.md; #7 Cycle-Mode label+injection; #8 Fix-Mode label/trigger; #10 context-file mechanism in examples; #11 "Supervisor dispatches"+mode/trigger). Each carries a reason; all enumerated in the human-gate bundle (reconcile §9 P2). Accepted, not bare.
- **Landing integrity** — spot-checked the receiving primitive (`implement.md`): the fix-pass trigger, "unconstrained by cycle boundaries / may touch files from any cycle," and the max-3 caps all landed on the lead (L20/L49/L80); the producer↔validator casting landed (L8/L19). Real destination, responsibility received.
- **No capability loss** — the fix-mode ROUTING/trigger/bounds were never the agent's own capability (HIL Supervisor briefed them); they moved to the lead and are exercised there. The fix CRAFT stays on staff. Union of tags covers original behavior. No loss.

---

## Notes (non-blocking — do NOT flip the verdict)

1. **Router index + `plugin.json` registration are a DOCUMENTED Wave-2 deferral** (transform report L23/L124-125), not a silent gap. It is outside the agent file, outside the task's 5 graded points, and mooted for reachability because the agent is already cast in its `implement.md` lead (agents are reached by command dispatch, not the router). Track it as a Wave-2 follow-up; it does not block the P2 artifact's PASS.
2. **Cosmetic: the producer's trace line-citations drifted** vs. the actual file (e.g. the reproduce-with-a-failing-test war-story is at **L58**, the trace cites L57; the Embrace fix bullet is at **L87**, the trace cites L82). Every cited item **is** present in the artifact at the substance level — only the producer's line bookkeeping is off (likely counted against an intermediate draft). The artifact itself has no defect; this affects neither conformance nor trace completeness/capability.

**Verify version:** v1 · **Governed by:** `verify-output` · defaults to FAIL; flipped to PASS only on evidence Read + grepped from the real artifact by an independent grader.
