# Transform (realized) — `skills/executing-tdd-cycle/` (P4)

**Run:** `/mochiko:transform-cluster implement` · Phase 3 (transform) · **Producer:** `mochiko:transform-producer`
**Skill applied:** `mochiko:transform-recipes` · **Date:** 2026-07-01
**Consumes:** `assess-executing-tdd-cycle.md` (P4 trace + boundary finding) + `reconcile.md` (§Job-3 P4, §Seam 3 fix/retry split, §Seam 5a `**TEST:**` boundary, §Job 1.3 P4↔P5 disjoint, §Seam 6 keep-report-format, §1.6 trigger de-collision)
**Finalized disposition (reconcile §Finalized dispositions):** **`port-with-edits × standalone`** — HOW-craft kept/decoupled; routing→lead; grammar boundary holds; zero open flags.

> ROLE NOTE: transform APPLIES the finalized decision; it does not re-decide and does not grade its own output. The independent PASS/FAIL is `verify-output`, run by a different agent (`mochiko:validator`).

---

## TRANSFORM output

```
TRANSFORM: executing-tdd-cycle (P4)
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/skills/executing-tdd-cycle/SKILL.md                         (HIL 164 ln → 133 ln)
           plugins/mochiko/skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md (kept in references — Seam 6)
           plugins/mochiko/skills/executing-tdd-cycle/references/TASK-PARSING.md
           plugins/mochiko/skills/executing-tdd-cycle/references/TDD-ANTI-RATIONALIZATION.md
New partners: NONE — standalone. Producer↔validator pair pre-exists (staff-engineer P2 ↔ qa-engineer P3);
              P4 mounts on staff-engineer ONLY (the mount lives on P2; never co-mounted with testing-end-user P5).
              P4↔P5 are opposite runtime sides of the same gate, disjoint (reconcile §Job 1.3).
Wiring:    classification = model-invoked skill (no disable-model-invocation; staff-engineer leans on it as producer procedure)
           router         = DEFERRED to Wave-2 (mochiko router index — Implement cluster row)
           triggers       = graded RFC-2119 (MUST/SHOULD), runtime-execution-scoped, de-collided from patterns-vertical-tdd
           rebinds        = cycle-report consumers (checkpoint gate/State Analyst/carry_forward) → the lead + the verifier;
                            feature dir → .mochiko/specs/<feature>/; agent names (Staff Engineer/qa-engineer) → role
           single-source  = **TEST:** grammar NOT restated (owned by patterns-vertical-tdd, consumed by testing-end-user);
                            cycle-report.md FORMAT kept HERE (its runtime home); upward When-NOT-to-Use cross-ref added
Trace (realized): every P4 responsibility → final tag (below); no silent loss.
```

**Body treatment realized (`port-with-edits`).** HIL structure + voice preserved; only the specific coupled lines edited. HIL SKILL.md 164 ln → 133 ln — the shrink is the removed DAG "When NOT to Use" line + the de-coupled retry/fix framing, not dropped craft. All three references ported (rebind/decouple only; no craft removed).

---

## The four reconcile edits — as realized

### A. Rebind brain/DAG references (consumers → role-neutral; paths → `.mochiko/specs/<feature>/`; drop the DAG-state line)

| HIL source | Realized | Anchor |
|-----------|----------|--------|
| `cycle-report.md` frontmatter "for the checkpoint gate" (SKILL 86) | "the lead reads them when deciding the cycle checkpoint, and verifies independently" | SKILL.md Progress Tracking |
| "context for the State Analyst and carry_forward" (SKILL 87) | "the human-readable context the lead and the next cycle's implementation need" — `State Analyst` + `carry_forward` vocab **dropped** | SKILL.md Progress Tracking |
| CYCLE-REPORT-FORMAT.md "for the checkpoint gate (deterministic evaluation) and … the State Analyst (briefings and carry_forward context)" (ref L3) | "structured YAML frontmatter the lead reads when it decides the cycle checkpoint, and prose sections … for the lead and the next cycle" | CYCLE-REPORT-FORMAT.md L1 |
| `checkpoint_criteria_met` "(checkpoint gate verifies independently)" | "the lead verifies independently — this is a self-report, not the verdict" | CYCLE-REPORT-FORMAT.md schema + field table |
| "context for the State Analyst to understand the state of the codebase" (ref L51) | "for the lead and the next cycle to understand the state of the codebase" | CYCLE-REPORT-FORMAT.md → What Was Done |
| "the next cycle's Staff Engineer dispatch needs to know" (ref L65) | "the next cycle's implementation needs to know" | CYCLE-REPORT-FORMAT.md → Notes for Next Cycle |
| feature-dir artifacts (`tasks.md`, `cycle-report.md`) | anchored to `.mochiko/specs/<feature>/` | SKILL.md When-to-Use + step 1 |
| **"Modifying DAG state or pass lifecycle — handled by the graph operations layer" (SKILL 29)** | **DROPPED** (no mochiko referent). Role-neutral intent preserved: "Managing loop or orchestration state — this skill executes one cycle or one rework and produces one report; it neither drives the loop nor tracks cross-cycle state." | SKILL.md When NOT to Use |

### B. Decouple (agent names → role; "dispatched" → craft; no injected phases/modes)

| HIL source | Realized |
|-----------|----------|
| "The Staff Engineer produces / uses these rules" (SKILL, CYCLE-REPORT-FORMAT L3, TASK-PARSING L3) | "the implementer" / "you" — role, not agent name |
| "verified by the qa-engineer, not the Staff Engineer" (TASK-PARSING L95) | "executed by the verifier (`testing-end-user`), not by this skill" — exclusion kept, independence stated by role |
| "the qa-engineer's verification" (TDD-ANTI-RATIONALIZATION L23) | "the verifier's independent verification" |
| `### Retry Handling` / "When **dispatched** after a checkpoint failure" (SKILL 89–91) | retitled **`### Reworking Specific Failed Tasks`**; "When particular tasks in a cycle come back as failing" — no "dispatch," no trigger |
| `### Fix Mode` / "When **dispatched** after final-validation failure" (SKILL 100–102) | retitled **`### Fixing a Reported Failure`**; "When a failure is reported against already-working code" — no "mode," no "dispatch," no trigger |

Keystone-tested every retitled line: reworking only the failed tasks, and reproducing a reported bug with a failing test before fixing it, are true of this engineer on **any** job → kept as craft. The workflow trigger ("after checkpoint / final-validation failure") and the mode-labels are what dropped.

### C. KEEP (runtime craft — carried intact)

- **Cycle Execution Sequence** (Parse → Red → Green → Refactor → Mark-complete → Write-report), strict-order framing → kept verbatim (SKILL Core Process).
- **Red/green/refactor execution mechanics** (write test, run it, verify it fails *for the right reason*, minimal green, cycle-only refactor, re-run) → kept verbatim.
- **Runtime task-parsing** (TASK-PARSING.md: task pattern, cycle = first with unchecked tasks, file-path extraction, `[EXTEND]`/`[MODIFY]`, multi-line rule, checkpoint & quality-gate *recognition*) → kept (agent-name decouple only).
- **`cycle-report.md` format** — full frontmatter schema (incl. `attempt` increments-on-retry, `cycle: "fix"`) + prose sections + example → **kept in `references/CYCLE-REPORT-FORMAT.md`** (human-gate-confirmed keep-in-references, reconcile §Seam 6). NOT extracted to `templates/`.
- **TDD anti-rationalization prose** — SKILL Red Flags + TDD-ANTI-RATIONALIZATION.md table → kept verbatim (execution-scoped; distinct altitude from patterns-vertical-tdd's structuring rationalizations).
- **retry/fix HOW-craft** — trace → re-open (`[x]`→`[ ]`) → TDD only the re-opened tasks → leave passing code → updated-`attempt` report; and for a fix: **reproduce the bug with a failing test, then green it**, scoped strictly, `cycle: fix` report → kept (decoupled).
- **`[EXTEND]`/`[MODIFY]` runtime handling + `brownfield-integration` (P6) invocation** → kept (proper sibling-skill reference, same cluster).
- **shared discipline banner** ("Violating the letter of the rules is violating the spirit of the rules.") → kept (references patterns-vertical-tdd's discipline; no restated substance; context.md "banner OK").

### D. Boundary vs `patterns-vertical-tdd` (single-source; reference, don't restate)

- **Upward cross-reference ADDED** (the assessment's recommended wiring nicety): `When NOT to Use → Structuring the cycles … is design-time work owned by patterns-vertical-tdd (upstream)`, mirroring the downward pointer patterns-vertical-tdd already carries ("Executing the cycles … is downstream"). Boundary symmetry restored; discipline single-sourced.
- **`**TEST:**` grammar stays ABSENT** — **0 matches** across SKILL + all 3 refs (grep-confirmed). P4 executes TN.1–TN.3 (failing-test / implement / refactor); the TN.4 real-infrastructure verification gate is the verifier's (`testing-end-user`), stated by *function/role* (not the `**TEST:**` marker literal) so the grammar is neither defined nor referenced here.
- **`patterns-vertical-tdd` NOT edited** — `grep cycle-report` there stays **0** (re-confirmed post-write). The cycle-report remains a runtime artifact owned HERE, un-leaked into the design-time skill.
- **`grep cycle-report` boundary invariant intact** in both directions.

---

## Convention-wiring pass (all six ran)

1. **Classification** — **model-invoked** skill (no `disable-model-invocation`); `staff-engineer` (P2) leans on it as its core producer procedure. Frontmatter is `name` + `description` only (skills carry no `skills:` list). ✓
2. **Router registration** — **DEFERRED to Wave-2** (the cluster-wide `mochiko` router index pass). Noted, not silently skipped — see "Wave-2 wiring needed." ✓
3. **Trigger phrasing** — graded RFC-2119 (`MUST` / `SHOULD`), describing the **runtime work context** (not "when the user says…"). **De-collided** from `patterns-vertical-tdd`: this skill's quoted triggers are execution-only ("execute cycle", "execute the cycle tasks", "implement the cycle task list", "write the cycle report", rework-failed-tasks, reproduce-a-failure-with-a-test); it does **not** list `patterns-vertical-tdd`'s design-time phrases ("TDD", "test-first", "red-green-refactor", "vertical slice", "cycle structure", "define cycles", "structure implementation"). The closing MUST/boundary sentence cedes *structuring* to `patterns-vertical-tdd` explicitly. ✓
4. **Path rebinding** — feature dir → `.mochiko/specs/<feature>/`; no `.humaninloop/` / `.workflow` / catalog / MCP / `hil-dag` paths (grep-confirmed 0). ✓
5. **Decouple** — agent names → role; "dispatch(ed)" → craft; no injected workflow modes/phases; keystone-tested. Deny-list grep = **0**. ✓
6. **Single-source / de-duplicate** — the `**TEST:**` grammar is referenced-by-boundary (owner `patterns-vertical-tdd`), never restated; the cycle-report FORMAT is single-sourced HERE (its runtime home — the sibling `staff-engineer` already points to `mochiko:executing-tdd-cycle` for the schema rather than copying it); shared discipline banner referenced, not re-derived. ✓

**Mechanical grep-floor (self-check, not a grade):**

| Check | Result |
|-------|--------|
| `**TEST:**` grammar (SKILL + 3 refs) | **0** — boundary integrity holds |
| deny-list: `dispatch` / `Staff Engineer` / `qa-engineer` / `State Analyst` / `carry_forward` / `DAG` / `graph operations` / `analysis layer` / `checkpoint gate` / `workflow-agnostic` | **0** |
| kernel/brain paths: `humaninloop:` / `hil-dag` / `catalog` / `INV-00` / `.humaninloop` / `.workflow` / `MCP` | **0** |
| retry/fix **routing & bounds** (`max 3` / `3 attempts` / `3 fix` / `convergence` / `no-progress` / `round cap`) | **0** — routing stays lead-owned |
| residual mode framing (`Fix Mode` / `Cycle Mode` / `Two Execution Modes` / `supervisor` / `when dispatched` / `instructions provided`) | **0** |
| `patterns-vertical-tdd` `grep cycle-report` (untouched sibling) | **0** — unchanged |

---

## Realized responsibility trace (every P4 responsibility → final tag; artifact anchors given)

Mirrors `assess-executing-tdd-cycle.md`'s trace + reconcile §Job-3 P4, tags flipped to **realized**. No responsibility dropped silently.

### KEPT — runtime execution craft (9)

| # | Responsibility | Assess tag | Realized | Anchor |
|---|----------------|-----------|----------|--------|
| 1 | Cycle Execution Sequence (Parse→Red→Green→Refactor→Mark→Report), strict order | kept | **kept** | SKILL Core Process 1–6 |
| 2 | Red/Green/Refactor phase mechanics (fail-for-right-reason; minimal green; cycle-only refactor; re-run) | kept | **kept** | SKILL steps 2–4 |
| 3 | Runtime task-parsing (pattern, current-cycle rule, path extraction, markers, multi-line, checkpoint/quality-gate recognition) | kept (minus decouple) | **kept** | TASK-PARSING.md (agent-name decouple applied) |
| 4 | `cycle-report.md` **format** (frontmatter schema incl. `attempt`/`cycle:"fix"` + prose + example) | kept (minus consumer rebind) | **kept — stays in `references/`** (Seam 6); consumers rebound | CYCLE-REPORT-FORMAT.md |
| 5 | TDD anti-rationalization prose (Red Flags + table) | kept | **kept** (execution-scoped) | SKILL Red Flags; TDD-ANTI-RATIONALIZATION.md |
| 6 | retry HOW-craft (trace→re-open→TDD-only-reopened→leave-passing→updated `attempt`) | kept (decoupled) | **kept** | SKILL → Reworking Specific Failed Tasks |
| 7 | fix HOW-craft (reproduce-with-a-failing-test → narrowest green → scoped → `cycle: fix`) | kept (decoupled) | **kept** | SKILL → Fixing a Reported Failure |
| 8 | `[EXTEND]`/`[MODIFY]` runtime handling + `brownfield-integration` (P6) invocation | kept | **kept** (sibling ref, same cluster) | SKILL steps 3.4–3.5; When-to-Use |
| 9 | shared discipline banner | kept | **kept** (references, no restated substance) | SKILL L3 |

### KEPT-BUT-REBIND — content-coupling fixed by edit (3)

| # | Responsibility | Realized | Anchor |
|---|----------------|----------|--------|
| 10 | `cycle-report.md` consumer references (`checkpoint gate` / `State Analyst` / `carry_forward`) | **kept-but-rebind** → the lead (reads for its cycle-checkpoint verdict) + the verifier; DAG/State-Analyst/carry_forward vocab dropped; **format unchanged** | SKILL Progress Tracking; CYCLE-REPORT-FORMAT.md L1 + field table |
| 11 | feature-dir artifacts (`tasks.md`, `cycle-report.md`) | **kept-but-rebind** → `.mochiko/specs/<feature>/` | SKILL When-to-Use + step 1 |
| 12 | description trigger phrases ("when the user says 'execute cycle'…") | **kept-but-rebind** → runtime work-context, RFC-2119, de-collided from patterns-vertical-tdd | SKILL frontmatter description |
| 12b | "Parsing/evaluating checkpoint/validation reports — handled by the analysis layer" (SKILL 27) | **kept-but-rebind** — exclusion kept ("this skill produces its own report; it does not grade one"); "analysis layer" → the lead Reads reports (no State-Analyst) | SKILL When NOT to Use |

### DROPPED — kernel-only, no mochiko referent (1)

| # | Responsibility | Realized | Reason |
|---|----------------|----------|--------|
| 13 | "Modifying DAG state or pass lifecycle — handled by the graph operations layer" (SKILL 29) | **dropped + reason** | DAG state / pass lifecycle do not exist in mochiko (kernel-free). Role-neutral intent **preserved**: "this skill … neither drives the loop nor tracks cross-cycle state." Grep-confirmed absent. **Human-gate accepted** (reconcile §Gated bundle item 9 → P4: 1 drop). |

### DECOUPLE — agent-name / "dispatched" scrub (2)

| # | Responsibility | Realized | Anchor |
|---|----------------|----------|--------|
| 14 | agent names `Staff Engineer` / `qa-engineer` (SKILL + all 3 refs) | **kept-but-rebind / decouple** → "the implementer" / "the verifier" / "this skill" / "you" | all four files |
| 15 | "When **dispatched** after a checkpoint / final-validation failure" (SKILL 91, 102) | **decoupled** → craft framing ("When particular tasks come back as failing" / "When a failure is reported"); the WHEN is not this skill's | SKILL Reworking / Fixing sections |

### MOVED-TO-LEAD / DEDUPE — retry/fix ROUTING (confirmation, not extraction) (1)

| # | Responsibility | Realized | Note |
|---|----------------|----------|------|
| 16 | retry/fix **routing/WHEN** — max-3-attempts/cycle, when-retry-vs-fix, max-3-fix-passes, convergence-stall | generic backbone **dedupe → `loop-discipline`** (Req 3); THIS-workflow params **moved-to-lead (P1)** | Never in P4's body in HIL (the DAG-Supervisor held it). Realized as **explicit non-inclusion**: each retry/fix section states "how many … and when to stop are the lead's routing decisions — not this skill's." Grep for bounds = **0**. Nothing extracted from P4; routing simply confirmed absent. |

**Realized bucket counts:** kept 9 · kept-but-rebind 4 (10, 11, 12, 12b) · dropped 1 · decouple 2 · routing-confirmed-out 1 → **every responsibility tagged, zero silent loss.**

---

## Independence (HARD rule honored — reconcile §Seam 2)

- P4 makes `staff-engineer` (P2) a **PRODUCER** (executes cycles, emits `cycle-report.md`); it acquires **no** grading responsibility. The quality-gate / real-infrastructure verification exclusion is **kept** (SKILL When NOT to Use; TASK-PARSING L95) and stated by *role* — that work is the verifier's (`testing-end-user` / P5), on `qa-engineer` (P3), a **different agent**.
- P4 is mounted on `staff-engineer` **only**; **never** co-mounted with `testing-end-user`. Produce + grade on one agent is wrong by construction.
- The clearing **verdict is lead-owned** — the report's `checkpoint_criteria_met` is framed explicitly as a self-report, "not the verdict." No self-grade leak.
- **P4 ↔ P5 disjoint** (reconcile §Job 1.3): P4 = producer-side runtime (write code, emit report); P5 = validator-side runtime (verify real infra, capture evidence, classify). No duplicated substance; `**TEST:**` = 0 in P4.

---

## Wave-2 wiring needed (DEFERRED — flagged, not silently skipped)

The skill's **local** wiring (frontmatter classification + description triggers + refs + boundary cross-ref + all rebinds/decouples) is complete. Cluster-scoped items belong to the Wave-2 pass:

1. **`mochiko` router SKILL.md** — add the **Implement cluster** section (model-invoked row): `executing-tdd-cycle` — "reach when executing an already-structured cycle at runtime: parse the cycle's tasks, drive red/green/refactor, rework failed tasks test-first, write the `cycle-report.md`; **not** for structuring the cycles (→ `patterns-vertical-tdd`)." Registered alongside `testing-end-user` + `brownfield-integration` + the `/mochiko:implement` entry. (A primitive not in the router fails discoverability.)
2. **`plugin.json`** — no edit required for the skill itself (skills auto-register via the `plugins/mochiko/skills/` directory glob); the Wave-2 pass only hand-adds the agent-array entries (P2/P3) and the router rows.
3. **`REGISTRY.md`** (Phase-5 delta) — L107 `executing-tdd-cycle` `[ ]` → `[x]` (Ported, implement, 2026-07-01: runtime cycle-execution skill; `cycle-report` format kept in references; `**TEST:**` grammar single-sourced to `patterns-vertical-tdd`, absent here; retry/fix HOW-craft kept, routing → lead).
4. **Cross-reference resolution** — the sibling `staff-engineer` (P2, landed) already references `mochiko:executing-tdd-cycle` by name for the execution procedure + report schema; those references now resolve (this artifact landed this run).

---

## Handoff

**Artifact:** `plugins/mochiko/skills/executing-tdd-cycle/` (SKILL.md + 3 references).
**Trace:** this file.
**Next:** independent grade by `verify-output` (a **different** agent, `mochiko:validator`) — the five conventions, sound-loop placement (producer half; no self-grade), kernel-free, the decoupling scan, the `**TEST:**`=0 boundary check, `grep cycle-report` in `patterns-vertical-tdd` still 0, and the responsibility-trace audit for silent loss.

**Transform version:** v1 · **Governed by:** `transform-recipes` · defaults to no self-grade.
