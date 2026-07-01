# Verify (independent grade) — `skills/executing-tdd-cycle/` (P4)

**Skill run:** `mochiko:verify-output` · **Validator:** `mochiko:validator` (independent — did NOT author the artifact)
**Disposition graded:** `port-with-edits × standalone` (staff-engineer runtime-execution skill)
**Producer claim audited:** `.mochiko/transform/implement/transform-executing-tdd-cycle.md` (audited, not trusted)
**Date:** 2026-07-01

```
VERIFY: executing-tdd-cycle (P4)
Evidence read (this run):
  plugins/mochiko/skills/executing-tdd-cycle/SKILL.md                          (169 ln)
  plugins/mochiko/skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md (101 ln)
  plugins/mochiko/skills/executing-tdd-cycle/references/TASK-PARSING.md        (95 ln)
  plugins/mochiko/skills/executing-tdd-cycle/references/TDD-ANTI-RATIONALIZATION.md (35 ln)
  plugins/mochiko/skills/patterns-vertical-tdd/SKILL.md                        (sibling, boundary check)
  .mochiko/transform/implement/transform-executing-tdd-cycle.md               (producer claim)

VERDICT: PASS
```

---

## Conformance checklist (binary, quoted evidence, greps re-run by validator)

### CHECK 1 — Boundary vs `patterns-vertical-tdd` (the load-bearing check) — **PASS**

- **`**TEST:**` grammar absent from P4 dir** — `grep -rn '\*\*TEST:\*\*' plugins/mochiko/skills/executing-tdd-cycle/` → **0 matches** (exit 1), per-file count `ALL ZERO`. The TN.4 real-infra grammar is owned by `patterns-vertical-tdd` (which defines it: `TN.4: **TEST:** - [What to verify with real infrastructure]`, sibling L127) and is neither defined nor referenced here. P4 executes TN.1–TN.3 only.
- **Upward cross-reference present** — SKILL.md L26 `When NOT to Use`: *"**Structuring the cycles** … is design-time work owned by `patterns-vertical-tdd` (upstream). This skill executes an already-structured cycle; it does not create, split, or reorder tasks."* Boundary symmetry confirmed against the sibling's downward pointer (sibling L34: *"Executing the cycles … is downstream execution"*).
- **`patterns-vertical-tdd` NOT edited** — `grep -rin 'cycle-report' plugins/mochiko/skills/patterns-vertical-tdd/` → **0 matches** (exit 1). The runtime cycle-report artifact never leaked into the design-time skill; boundary invariant holds in both directions.

### CHECK 2 — Routing stays lead-owned — **PASS**

- **Routing/bounds tokens absent** — `grep -rinE 'max 3|3 attempts|3 fix|convergence|no-progress|round cap|fix mode|two execution modes|cycle mode|round.?limit|bounded iteration'` → **0 matches** (exit 1).
- **Mode/dispatch/supervisor absent** — `grep -rinE '\bdispatch|\bmode\b|supervisor|when dispatched|instructions provided'` → **0 matches** (exit 1).
- **HOW-craft kept, routing explicitly ceded** — the retry craft (SKILL L90–101 `Reworking Specific Failed Tasks`) and the fix craft (SKILL L104–112 `Fixing a Reported Failure`, incl. *"Reproduce each one with a failing test before changing any code"*) are present, but each section closes by handing the WHEN/HOW-MANY back to the lead: L101 *"Whether to rework, how many attempts are permitted, and when to stop are the lead's routing decisions — not this skill's."*; L112 *"how many are permitted, is the lead's routing decision — not this skill's."* Craft in; routing out.

### CHECK 3 — Kernel-free + decoupling — **PASS**

- **Kernel/brain tokens absent** — `grep -rinE 'hil-dag|DAG|State.?Analyst|carry_forward|\.workflow|humaninloop:|catalog|INV-0|\.humaninloop|graph operations|MCP|analysis layer|checkpoint gate'` → **0 matches** (exit 1).
- **Sibling agent names / dispatch absent** — `grep -rinE 'Staff Engineer|qa-engineer|staff-engineer|principal-architect|…|devils-advocate'` → **0 matches** (exit 1). Consumers rebound to roles: *"the implementer" / "the lead" / "the verifier (`testing-end-user`)" / "you"*.
- **Feature-dir rebound** — `grep -rn '\.mochiko/specs'` → 3 hits (SKILL L3, L18, L40), all `.mochiko/specs/<feature>/`. Stray HIL paths (`.humaninloop|specs/[0-9]|/\.workflow/`) → **0 matches** (exit 1).

### CHECK 4 — Conventions — **PASS**

- **Classification = model-invoked** — `grep 'disable-model-invocation'` → **0** (exit 1); frontmatter is `name` + `description` only (skills carry no `skills:` list). Consumed by `staff-engineer` (P2) as producer procedure.
- **Triggers graded RFC-2119 + runtime-scoped** — `grep -oE 'MUST be invoked|SHOULD also invoke'` → both present. Description opens *"This skill MUST be invoked when executing an already-structured cycle at runtime …"* — work-context framed, not "when the user says".
- **De-collided from `patterns-vertical-tdd`** — P4's quoted SHOULD triggers are `"execute cycle"`, `"execute the cycle tasks"`, `"implement the cycle task list"`, `"write the cycle report"` (verified via `grep -oE '"[^"]+"'`). The design-time collision phrases (`"TDD"`, `"test-first"`, `"red-green-refactor"`, `"vertical slice"`, `"cycle structure"`, `"define cycles"`, `"structure implementation"`, `"testable increment"`) → **0 matches** as P4 triggers (exit 1). Quoted-trigger sets are disjoint; P4 fires on *executing* a cycle. ("red→green→refactor" appears once as descriptive mechanism — "the red→green→refactor execution sequence", L3 — not as a quoted trigger phrase.)
- **`cycle-report.md` format kept in references (not extracted)** — `references/CYCLE-REPORT-FORMAT.md` carries the full frontmatter schema (`cycle`/`attempt`/`tasks_total`/`tasks_completed`/`files_created`/`files_modified`/`brownfield_tasks`/`checkpoint_criteria_met`, incl. `cycle: "fix"` and `attempt` increments-on-retry), the field-definition table, prose sections, and a worked example. Present in `references/`, not promoted to `templates/`.

### CHECK 5 — Trace audit (Part B) — **PASS**

Audited the producer's realized trace (16 responsibilities) against the artifact:

| Audit dimension | Result | Evidence |
|---|---|---|
| **Completeness** | PASS | All 16 responsibilities tagged (kept 9 · kept-but-rebind 4 · dropped 1 · decouple 2 · routing-confirmed-out 1). Every kept item spot-checked present in the artifact (Core Process 1–6; red/green/refactor mechanics L49–73; TASK-PARSING.md; CYCLE-REPORT-FORMAT.md; Red Flags + TDD-ANTI-RATIONALIZATION.md; rework L90–101; fix L104–112; `[EXTEND]`/`[MODIFY]` + `brownfield-integration` L21/L63–64; banner L8). |
| **Justified drops** | PASS | Exactly **one** drop (#13, "Modifying DAG state or pass lifecycle — handled by the graph operations layer"). Reason logged (no mochiko referent — kernel-free), role-neutral intent preserved at SKILL L30 (*"Managing loop or orchestration state … it neither drives the loop nor tracks cross-cycle state"*), and **human-gate accepted** (reconcile §Gated bundle item 9). Grep-confirmed absent. Matches the single accepted drop the grade anticipated. |
| **Landing integrity** | PASS | `moved-to`/rebind destinations are real: cycle-report consumers → the lead + the verifier (roles, structurally present in both files); feature-dir → `.mochiko/specs/<feature>/` (grep-confirmed); retry/fix routing backbone → `loop-discipline` + params → lead (P1), realized as explicit non-inclusion (never in P4's HIL body — DAG-Supervisor held it — so confirmation-of-absence, not silent extraction). |
| **No capability loss** | PASS | Union of tags covers the original runtime behavior — parse → red/green/refactor → mark → report → rework → fix → brownfield handling all intact. The only removal is kernel plumbing with no mochiko referent. Nothing a user could do before is now impossible. |

---

## Sound-loop placement (Part A items 5–6) — **PASS**

- **Producer↔validator independence is structural, by role.** P4 makes the implementer a **producer** (executes cycles, emits `cycle-report.md`); grading is ceded to the **verifier** (`testing-end-user`, P5 on `qa-engineer` — a different agent), stated by function. SKILL L27–28 exclude quality-gate/verification work and report-grading. `checkpoint_criteria_met` is framed as *"a self-report, not the verdict — the lead verifies independently"* (CYCLE-REPORT-FORMAT.md L34). No self-grade leak.
- P4 correctly does **not** own the done-condition/bounds/human-gate (those are the command lead's + `loop-discipline`'s) — consistent with a producer-side skill sitting in a sound loop.

## Altitude (Part A item 8) — **PASS (N/A restatement)**

P4 is a skill, not a command. It **references** `patterns-vertical-tdd` for the boundary rather than restating the `**TEST:**` grammar (grep-confirmed absent), and legitimately keeps its own runtime procedure (skills run agent-less and own their procedure). No shared doctrine duplicated.

---

## Noted (transparently deferred — NOT a silent gap, out of scope for this local grade)

- **Router discoverability (verify-output Part A item 2)** is **DEFERRED to Wave-2** by the producer (§Wave-2 wiring needed #1: add the Implement-cluster row to the `mochiko` router). This is the one full-checklist item not yet satisfied for P4; it is flagged, not silently skipped, and is cluster-scoped rather than part of this skill's local wiring. Flagging here for the Wave-2 pass — a primitive absent from the router fails discoverability once that pass runs. It does not gate the five checks under grade.

---

## Failing items

**None.** All five graded checks PASS; the trace audit is clean (one accepted, human-gated drop; no silent loss).

## Required fixes

**None for acceptance.** Wave-2 follow-up (not a fix): register `executing-tdd-cycle` in the `mochiko` router (Implement cluster, model-invoked row) with when-to-reach-it guidance and the `→ patterns-vertical-tdd` not-for-structuring pointer.

**Verify version:** v1 · **Graded by:** `mochiko:validator` via `mochiko:verify-output` · independent of the producer.
