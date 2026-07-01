# Transform (realized) — `agents/staff-engineer.md` (P2)

Run: `implement` cluster transform · Phase 3 (transform) · Producer: `transform-producer`
Skill applied: `mochiko:transform-recipes` · Date: 2026-07-01
Consumes: `assess-staff-engineer.md` (P2 trace + 2 decouple surfaces) + `reconcile.md` (Seam 3 fix-mode split, Seam 2 casting/independence, Job-3 P2 finalized trace)
Finalized disposition (reconcile §Finalized dispositions): **`port-with-edits × standalone`** — fix-mode split resolved (craft kept, routing→lead); zero open flags.

> ROLE NOTE: transform APPLIES the finalized decision; it does not re-decide and does not grade its own output. The independent PASS/FAIL is `verify-output`, run by a different agent.

---

## Output block

```
TRANSFORM: staff-engineer (P2)
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/agents/staff-engineer.md   (net-new mochiko agent; HIL source 95 ln → 87 ln)
New partners: NONE — standalone. No split/promote/merge. Producer↔validator pair already exists
              (qa-engineer P3 + testing-end-user P5); the mount lives on P3, never here (reconcile §Seam 2).
Wiring:    classification = model-invoked producer agent (no disable-model-invocation; skills: list present, persona-vs-procedure split honored)
           router          = DEFERRED to Wave-2 (mochiko router index)
           plugin.json     = DEFERRED to Wave-2 (agent registration)
           triggers        = N/A at agent frontmatter; description ships 3 role-based <example> blocks (work-context, no dispatch/context-file/mode)
           rebinds         = humaninloop:executing-tdd-cycle → mochiko:executing-tdd-cycle (body L46);
                             humaninloop:brownfield-integration → mochiko:brownfield-integration (body L47);
                             cycle-report format pointer → mochiko:executing-tdd-cycle (body L65)
           single-source   = cycle-report.md FORMAT referenced (mochiko:executing-tdd-cycle), schema NOT restated;
                             TDD/retry/fix PROCEDURE referenced (executing-tdd-cycle), not restated;
                             EXTEND/MODIFY + read-before-write PROCEDURE referenced (brownfield-integration), not restated
Trace (realized): every P2 responsibility → final tag (below); no silent loss.
```

---

## The fix-mode split as realized (Seam 3, line-by-line)

The HIL `## Two Execution Modes` section (source L73–81) is **dissolved**. Its surviving craft rehomed into the persona as intrinsic craft; its routing/trigger/bounds are **not named** here (they live on the lead — P1).

### KEPT as intrinsic craft (rehomed within the persona — no mode framing)
| Seam-3 KEEP item | Realized home in artifact | Line |
|------------------|---------------------------|------|
| reproduce a bug with a failing test **before** fixing it | Core Identity war-story ("…you reproduce each one with a failing test before touching the code…") + What You Embrace bullet | L57, L82 |
| scope a fix tightly to the reported failures | Core Identity ("…keep the change scoped to that failure…") + Embrace ("…keeping the fix scoped to that failure…") + example 2 | L57, L82, desc |
| a fix follows the failure, **not** a refactor opportunity | Core Identity ("…follow the failure wherever it leads rather than treating it as a license to refactor") + Embrace ("…rather than a wider refactor") | L57, L82 |
| scope discipline (stay tightly scoped to the work given) | Quality Standards → Scope discipline ("…stay tightly scoped to it") + Core Identity ("implement exactly what the task describes, nothing more") | L72, L56 |

### REMOVED (rehomed to the lead — appears NOWHERE in the artifact)
| Seam-3 REMOVE item | HIL source | Realized |
|--------------------|-----------|----------|
| `## Two Execution Modes` skeleton | L73 | deleted (no two-mode framing) |
| `### Cycle Mode (Normal)` mode-label | L75 | deleted (scope-discipline disposition survives as craft, not a mode) |
| `### Fix Mode` mode-label | L79 | deleted (fix craft survives as craft, not a mode) |
| **"After Validation Failure"** trigger | L79 | deleted — persona does **not** name when fix-scoped work fires |
| "Unconstrained by cycle boundaries / may touch files from any cycle" | L81 | deleted (routing the lead makes when it briefs a fix-scoped run) |
| max-3-passes / max-3-retries bounds | (command/skill-side in HIL) | **never introduced** — persona names no bound |
| skill-blurb "retry handling, **fix mode**" mode-naming | L44 | softened → "targeted rework when specific tasks fail" (describes the skill's *work*, not a workflow-mode) — L46 |

### DEDUPED (procedure — referenced, not restated)
- fix/retry HOW (trace→re-open→TDD-only-reopened→updated `cycle-report`) → `mochiko:executing-tdd-cycle` (P4); the artifact references the skill as single source of truth, restates no steps.
- bounded-iteration backbone (a cap exists; escalate-don't-die) → `loop-discipline` Req 3 (lead-owned; the persona carries none of it).

---

## Decoupling self-scan (grep-anchored — ZERO residual on both named surfaces)

Ran the decoupling scan the way `verify-output` will. Result: **ZERO** deny-list tokens on the two named highest-risk surfaces + the general deny-list.

| Scan | Pattern class | Result |
|------|---------------|--------|
| **Surface 1 — mode-section** | `two execution modes`, `cycle mode`, `fix mode`, `after validation failure`, `unconstrained by cycle`, `may touch files/any cycle`, `the cycle's scope`, `in your instructions`, `max 3 / 3 passes / 3 retries` | **CLEAN (0)** |
| **Surface 2 — context-file / dispatch** | `context.md`, `.workflow`, `supervisor`, `dispatch`, `read your instructions`, `specs/001`, `locations specified`, `instructions (context…)` | **CLEAN (0)** |
| **General** | sibling-agent names (`qa-engineer`, `state-analyst`, `task-architect`, …), `workflow-agnostic`, `humaninloop:`, `hil-dag`, `catalog`, `DAG` | **CLEAN (0)** |
| **Independence** | any mounted grading/`verify-*`/`testing-end-user`/`validation-*` skill | **CLEAN (0 mounts)** — the single `grade` occurrence is the description's "does not grade its own output" (independence-by-role, house-style on both landed producer siblings), NOT a mounted grading skill |

Positive wiring confirmed: `mochiko:executing-tdd-cycle` + `mochiko:brownfield-integration` body refs present (L46/L47); cycle-report format pointer → `mochiko:executing-tdd-cycle` (L65); frontmatter `skills: executing-tdd-cycle, brownfield-integration` bare per the ported-agent convention.

---

## Mounts (Seam 2 — HARD independence rule honored)

| Field | Value | Rationale |
|-------|-------|-----------|
| `skills:` (frontmatter) | `executing-tdd-cycle, brownfield-integration` | both producer/authoring skills; both ported this run (in-cluster); bare per convention (`task-architect` precedent) |
| body skill refs | `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration` | rebound from `humaninloop:` |
| **NEVER mounted** | `testing-end-user` / any `verify-*` / grading skill | produce + grade on one agent is wrong by construction; the P5 grading mount lives on **P3 `qa-engineer` only** (reconcile §Seam 2). Verdict is **lead-owned**; qa's status is input, never a staff self-grade. |

Producer↔validator independence: `{executing-tdd-cycle, brownfield-integration}` ∩ grading skills = ∅; `staff-engineer` (producer) ≠ `qa-engineer` (validator) — different agents. Confirmed clean, no restructure (the pair pre-exists — the inverse-of-setup nuance).

---

## Realized responsibility trace (every P2 responsibility → final tag)

Mirrors the assess trace (18 rows) + reconcile §Job-3 P2, with tags flipped to their **realized** state. No responsibility is dropped silently.

| # | Responsibility | Assess tag | Realized |
|---|----------------|-----------|----------|
| 1 | Persona & judgment (Core Identity: genuine-red / simplest-impl / read-before-write / no-scope-creep / flag-honestly) | `kept` | **kept** — Core Identity 6 war-stories (L52–58), keystone-clean; +1 new fix-craft war-story (L57) |
| 2 | TDD red/green/refactor **procedure** via `executing-tdd-cycle` (P4) | `kept-but-rebind` | **kept-but-rebind** — frontmatter bare (L37); body `mochiko:executing-tdd-cycle` (L46); stays mounted |
| 3 | Brownfield EXTEND/MODIFY **procedure** via `brownfield-integration` (P6) | `kept-but-rebind` | **kept-but-rebind** — frontmatter bare (L37); body `mochiko:brownfield-integration` (L47); stays mounted; resolves setup-run REGISTRY mis-file (home = implement) |
| 4 | Output menu — implemented code (TDD); updated `tasks.md`; honest `cycle-report.md` | `kept` | **kept** — What You Produce 1–3 (L63–65); filenames kept as artifact-type names |
| 5 | Output-**location** "locations specified in your instructions (context.md)" (HIL L64) | `dropped + reason` | **dropped + reason** — context-file carrier; output location is caller-side (`moved-to-lead`). Line removed. **Human-gate accepted** (bundle §9 P2). |
| 6 | Quality Standards — TDD rigor / scope / brownfield / honest reporting | `kept` + light `dedupe`→P4 | **kept** as taste (L69–73) with single-source pointer to skills; literal red/green/refactor step deduped to P4 |
| 7 | **Cycle-Mode framing** (HIL L75–77) | scope-disposition `kept`; mode-label+"cycle's scope"+injection `dropped`; seq-proc `dedupe`→P4 | **realized SPLIT** — scope-discipline **kept** as craft (Quality Standards L72 + Core Identity L56); mode-label + "the cycle's scope" + "in your instructions" **dropped** (removed with the section); sequential-execution procedure **dedupe**→P4. **Human-gate accepted.** |
| 8 | **Fix-Mode framing** (HIL L79–81) — THE CRUX | SPLIT: craft `kept`; routing/label/trigger/bounds `flag-for-reconcile`→lead; proc `dedupe`→P4 | **realized SPLIT (Seam 3)** — fix craft **kept** (Core Identity L57 + Embrace L82); mode-label + "After Validation Failure" trigger + cycle-boundary vocabulary + max-passes bound **moved-to-lead** (absent from artifact); procedure **dedupe**→P4. **Human-gate accepted** for dropped mode-labels. |
| 9 | **Retry disposition** (targeted rework of only failed tasks; don't re-implement passing code) | disposition `kept` + proc `dedupe`→P4 + routing/trigger `moved-to-lead` | **realized** — disposition **kept** ("narrowest change… scoped to the reported failures" — example 2; Embrace L82); procedure **dedupe**→P4; routing/trigger ("after checkpoint failure", max-3/cycle) **moved-to-lead** (not named here) |
| 10 | **Context-file mechanism** in 3 `<example>` blocks (HIL L9/18/27) | `dropped + reason` | **dropped + reason** — no mochiko `context.md`; examples reframed to work-context. **Human-gate accepted.** |
| 11 | **"Supervisor dispatches" + mode/trigger naming** in 3 examples (HIL L8/16/25, L17/26) | `dropped + reason` | **dropped + reason** — 3 examples **reframed to work-context** (task list to implement; reported failures to reproduce-then-fix; brownfield EXTEND). No dispatch / supervisor / mode / trigger tokens. **Human-gate accepted.** |
| 12 | Skills-Available descriptions (2 skills as assets) | `kept-but-rebind` (soften "fix mode" blurb) | **kept-but-rebind** — `humaninloop:`→`mochiko:` (L46/47); "retry handling, fix mode" softened → "targeted rework when specific tasks fail" (describes work, not mode) |
| 13 | What You Reject (no unasked code; no skipped failing test; no interface-mod without `[MODIFY]`; no silent workarounds) | `kept` | **kept** — L77–80, keystone-clean; `[MODIFY]` is brownfield craft |
| 14 | What You Embrace (follow patterns; behavior-not-impl tests; flag discrepancies; honest reports) | `kept` | **kept** — L82–86; +1 fix-craft bullet (reproduce-then-fix, scoped) L82 |
| 15 | **Honest cycle-report self-disclosure** (reflects what actually happened, incl. difficulties/deviations) | `kept` (self-disclosure, NOT a self-asserted verdict) | **kept** — What You Produce #3 (L65) explicitly frames it as "a truthful record… not a verdict on whether the result passes"; **no self-asserted clearing verdict** — no independence leak |
| 16 | PRODUCER **team-role** (verified by independent `qa-engineer` P3 + `testing-end-user` P5 — Tier-1) | `kept` (validator settled; independence intact) | **kept** — single producer role; no grading skill mounted; description states "does not grade its own output"; validator settled on P3 (not named here) |
| 17 | Classification + router/discoverability + `<example>` reframe | `kept-but-rebind` (wiring pass) | **realized** — classification = model-invoked (done); all 3 examples rewritten to work-context (done); router + `plugin.json` registration **DEFERRED to Wave-2** |
| 18 | Agent frontmatter (`model: opus`, `color: green`) | `kept` | **kept** — L33/34 (identical) |

**Orchestration note (unchanged):** this agent never owned the loop. Sequential cycle sequencing, execute→verify pairing, targeted-retry (max-3/cycle), fix-pass (max-3), convergence-stall, the entry gate, and project scaffolding are HIL DAG-Supervisor + State-Analyst + `strategy-implementation` (P7) responsibilities that re-home onto the **thin `implement` lead (P1)** (reconcile §Job 2). This artifact contributes only the `moved-to-lead` / dropped items for the mode-routing / output-location / injected-instructions it formerly referenced (#5, #7-label, #8-label, #10, #11).

**No silent loss.** Every drop (#5, #7-label, #8-label, #10, #11) carries a reason and is in the human-gate bundle (reconcile §9 P2 — 5 drops). Every responsibility carries a realized tag → transform done-condition met.

---

## Wave-2 wiring needed (DEFERRED — not this pass)

1. **Router index** — register `staff-engineer` in the `mochiko` router with when-to-reach-it guidance (producer of the implement team; writes code via TDD, integrates brownfield, produces the cycle report).
2. **`plugin.json`** — add the agent to `plugins/mochiko/.claude-plugin/plugin.json` agent registration.
3. **Cluster wiring (P1's move, noted for completeness):** the thin `implement` lead owns supervisor→agent-team casting, the sequential cycle loop, execute→verify pairing, targeted-retry, fix-pass, convergence-stall, the entry gate, project scaffolding, and both gate placements. `qa-engineer` (P3) mounts `testing-end-user` (P5). None of these are staff-engineer's move.

---

## Handoff

Artifact: `plugins/mochiko/agents/staff-engineer.md`
Trace: this file.
Next: independent grade by `verify-output` (a **different** agent) — the five conventions, sound-loop placement, kernel-free, decoupling scan on the two named surfaces, and the trace audit for silent loss.

**Transform version:** v1 · **Governed by:** `transform-recipes` · defaults to no self-grade.
