# Assessment — P6 `brownfield-integration` (skill)

**Run:** transform-cluster `implement` · **Producer:** `mochiko:transform-producer` · **Date:** 2026-07-01
**HIL source:** `human-in-loop/plugins/humaninloop/skills/brownfield-integration/SKILL.md` (130 ln)
**Role:** `staff-engineer`'s (P2) SECOND skill — EXTEND/MODIFY semantics, read-before-write checklist, interface preservation, conflict detection when implementing against an existing codebase.
**Diagnose-only.** No edits made. Transform is `transform-recipes`; grading is `verify-output`.

---

## ASSESSMENT: brownfield-integration

```
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=n  → full-lens (gate2 tripped: 4 responsibilities)
Disposition:  port-with-edits × standalone
              (+ 1 flag-for-reconcile: cross-cluster marker-grammar single-source vs patterns-vertical-tdd)
```

**Disposition rationale.** Refines the anticipated `port × standalone` to **`port-with-edits × standalone`**.
Not `keep-verbatim` — three localized edits are earned: (a) re-anchor the description's `MUST`
triggers from "when the user says…" to the agent-consumed **work context** (wiring step 3); (b) soften
soft workflow-phase vocabulary (`cycle execution`, `previous cycles`, `cycle report`) to intrinsic craft;
(c) single-source the `[EXTEND]`/`[MODIFY]` marker *vocabulary* to its canonical owner. Not `redesign` —
the substance (EXTEND/MODIFY consumption semantics, the 5-step read-before-write checklist, interface
preservation, conflict detection, the anti-rationalization spine) is mochiko-clean craft; an edit fixes
it, so minimalism governs. Structural `standalone` — self-contained, one home (staff-engineer's 2nd skill),
emits no artifact (no split/pair), confers producer role not validator (no promote), reusable craft not
orchestration (no absorb-into-lead), disjoint from siblings (no merge).

---

## Home confirmation — REGISTRY correction rides on this

**CONFIRMED: home = `implement`** (not `setup`). The setup run filed it under the Setup cluster but
flagged it a **mis-file** and deferred the call to "when the implement workflow is scoped." That is now.
Four converging evidences:

1. **Mounting.** `agents/staff-engineer.md` frontmatter: `skills: executing-tdd-cycle, brownfield-integration`.
   staff-engineer is the **implement producer** (P2). This skill is its 2nd skill by construction.
2. **Craft is implement-time.** EXTEND/MODIFY semantics, read-before-write, interface preservation, and
   conflict detection are all *code-writing-against-existing-code* disciplines — exercised while implementing,
   not while governing.
3. **Pure implement-time consumer of upstream markers.** It CONSUMES `[EXTEND]`/`[MODIFY]` markers that the
   **tasks/design-time** author (`patterns-vertical-tdd`) stamps onto `tasks.md`. It reads what tasks already
   carry; it produces no governance artifact.
4. **Setup never invokes it.** The setup command/agent produce a constitution; this skill touches no
   constitution, no `.mochiko/memory/`, and is never dispatched by the setup lead. (Setup-run finding,
   independently reconfirmed here.)

**Phase-5 REGISTRY action (flag for transform/verify).** `REGISTRY.md` currently lists
`[ ] brownfield-integration` under **`### Setup workflow cluster`** (line 80: "Deferred — out of setup-core
scope (REGISTRY mis-file; reads as implement-time)"). Phase 5 MUST **move the row to `### Implement / execute
cluster`** (alongside `executing-tdd-cycle`, `testing-end-user`) and mark it `[x]` when landed. This is the
REGISTRY correction the setup follow-up requested; do not leave the row under Setup.

---

## Disjointness check — vs `analysis-codebase` (already ported): CONFIRMED DISJOINT

| Axis | `analysis-codebase` (setup, ported) | `brownfield-integration` (implement, this port) |
|------|-------------------------------------|--------------------------------------------------|
| **Concern** | DETECT a codebase's stack/architecture/conventions + Essential-Floor status **to inform governance** | Safely **EXTEND/MODIFY** existing code during implementation |
| **Phase** | setup (before a constitution exists) | implement (during cycle execution) |
| **Consumer** | constitution author / `principal-architect` | `staff-engineer` (implement producer) |
| **Output** | an artifact — `.mochiko/memory/codebase-analysis.md` | shaped code + surfaced flags (no artifact of its own) |
| **"Read existing code" scope** | breadth — read the **whole project** once to *characterize* it | depth — read the **one file about to be touched** to *preserve its interface* |

The only surface overlap is the word "brownfield" and the verb "read existing code." Underneath they are
opposite acts: analysis-codebase reads to **characterize/govern** (project-wide, one-time, governance);
brownfield-integration reads to **safely modify** (file-scoped, per-task, implementation). No shared core,
no merge, no trigger collision (analysis-codebase fires on "analyze codebase / detect stack / brownfield
governance"; brownfield-integration fires on EXTEND/MODIFY / implementing against existing code). **Fully
disjoint.** No structural relation between them.

---

## The 7-check lens (weighted PLAYS-a-role)

**1 — Orchestration test.** Consumed procedure; the staff-engineer invokes it via the Skill tool on
brownfield markers. **Zero content-coupling to kernel/DAG/catalog/MCP** — `grep -niE
'\.humaninloop|catalog|DAG|MCP|hil-dag|brain|\.workflow/'` → **none**. HIL's `implement` was DAG-based
(`implement-catalog.json`), but this skill body carries no brain dependency. Only couplings are the
implement-workflow *vocabulary* (`cycle report`, `cycle execution`, `previous cycles`) — soft, not kernel.
Nothing to rehome. Clean body.

**2 — Role (two altitudes).** Skill-role = **consumed procedure** (staff-engineer runs it as
implementation guidance). It **emits no reviewable artifact** → needs **no producer↔validator partner**.
Team-role conferred = **producer** (implementation craft), NOT validator → **not `promote` material**.

**3 — Independence.** No self-grade leak. Pure producer-side craft, entirely on the producer
(`staff-engineer`); it has no grading function. The implement loop's independence is guaranteed by
`staff-engineer` (produce) ≠ `qa-engineer` (grade); this skill sits wholly on the produce side and does not
disturb that boundary.

**4 — Verdict-sink / loop-driver.** Owns no verdict and no FAIL-loop. Its one downstream edge is the
"flag, don't silently resolve" discipline, which routes discovered blockers/conflicts into the **cycle
report** (owned by `executing-tdd-cycle`/P4), consumed downstream by qa/lead. Loop-driving (targeted retry,
fix-pass) is the lead's, not this skill's. Nothing to rehome from here.

**5 — Sibling / overlap ("look sideways").** Two signals, one reconcile-worthy:
- **Marker-grammar boundary vs `patterns-vertical-tdd` (tasks cluster, ported) → flag-for-reconcile.**
  `patterns-vertical-tdd` is the canonical **author/owner** of the `[EXTEND]`/`[MODIFY]` marker vocabulary
  (its Markers table stamps brownfield tasks). brownfield-integration **CONSUMES** those markers at
  implement-time and currently **restates their one-line meaning** (its "EXTEND vs. MODIFY Semantics" table:
  EXTEND="Add new code alongside existing", MODIFY="Change existing behavior"). That is a duplication of the
  *vocabulary definition* — flagged per instruction. The two are genuinely distinct by altitude (design-time
  *tagging* vs implement-time *consumption/interface-impact*), so the resolution is **reference/dedupe, not
  merge**: brownfield-integration references `patterns-vertical-tdd` (or `tasks-template`) as the canonical
  marker owner and keeps only the implement-time consumption semantics (the interface-impact rules
  patterns-vertical-tdd does NOT carry). Direct analog of the contract's P4 boundary-integrity clause.
- **Trigger co-fire vs `executing-tdd-cycle` (P4) → convention-wiring de-collide (NOT structural).** Both
  claim the `[EXTEND]`/`[MODIFY]` trigger; both legitimately co-fire on a brownfield cycle (they do different
  work — P4 drives red/green/refactor, P6 is the read-before-write craft). De-collide by scoping descriptions;
  no merge. Hand to the wiring pass.

**6 — Coupling audit.** **No hardcoded paths** (grep clean — no `.humaninloop/`, no catalog/script paths).
Prerequisite/handoff edges (consumed inputs, not kernel coupling): (a) tasks arrive already carrying
`[EXTEND]`/`[MODIFY]` markers, authored upstream at design-time (`patterns-vertical-tdd`/tasks); (b) a
"cycle report" exists as the flag sink (owned by `executing-tdd-cycle`/P4). Determinism boundary: pure
model-judgment craft (read, match patterns, preserve interfaces); a couple of sub-steps are mechanically
checkable (conflict detection = "search the file for the name" = a grep) but the skill is judgment, not a
deterministic script — and it emits no artifact, so no degenerate-validator concern.

**7 — Conventions + loop placement.**
- **Classification:** model-invoked (default; no `disable-model-invocation`). Correct — an agent-consumed
  producer-craft skill. ✓
- **Discoverability:** register in the `mochiko` router with when-to-reach guidance (agent-consumed entry).
  Not yet present → wiring-pass item.
- **Reliable model-invocation:** graded RFC-2119 triggers **PRESENT** — `MUST be invoked when the user says
  "brownfield integration", "extend existing code", or "modify existing file". SHOULD also invoke when
  encountering tasks with [EXTEND] or [MODIFY] markers, implementing against existing codebases, or
  integrating with established interfaces.` Graded (MUST/SHOULD) with exact phrases ✓. **Refinement earned:**
  the `MUST` clause leads with "when the user says…"; for an agent-consumed skill the wiring convention wants
  triggers framed as **work context** (encountering `[EXTEND]`/`[MODIFY]` markers, implementing against an
  existing codebase), not user utterances → re-anchor in the port (port-with-edits).
- **Agent↔skill composition / decoupling scan:** **No hard deny-list tokens** — `grep -niE
  'staff-engineer|qa-engineer|supervisor|state-analyst|dispatch|workflow-agnostic|principal-architect'` →
  **none**. Pure procedure (persona lives on staff-engineer). Soft workflow-phase vocabulary present and
  located: `previous cycles` (L21), `during cycle execution` (L27), `cycle report` (L38, 72, 90, 104, 122).
  Keystone test: the *substance* is intrinsic craft (a professional extending existing code on ANY job reads
  before write, preserves interfaces, and surfaces blockers rather than silently rewriting); only the
  workflow-phase *labels* are coupling → soften to intrinsic phrasing (`during a scoped extend/modify task`,
  `prior work`, `surface it / flag rather than silently resolve`). Soft, edit-level — no redesign.
- **Producer↔validator pairing:** N/A — emits no artifact. Correctly producer-side.
- **Sound-loop:** not a loop itself; a good citizen inside the implement loop. Supplies no
  done-condition/validation/human-gate of its own (correctly — those are the lead's + qa's), and its
  "flag, don't silently resolve" discipline feeds the loop's FAIL surface. No loop gap at the skill level.

---

## Responsibility trace (done-condition — every responsibility tagged, no silent drop)

```
TRACE: brownfield-integration
  - EXTEND/MODIFY consumption semantics (interface-impact rules per marker)  → kept
  - [EXTEND]/[MODIFY] marker VOCABULARY definition (they exist + 1-line gloss) → dedupe → patterns-vertical-tdd
        (canonical owner; reconcile-assigned; cross-cluster single-source — NOT a merge)
  - read-before-write checklist (5 steps: full file, naming, error-handling, imports, tests) → kept
  - interface preservation (no signature/export/name changes; add alongside)  → kept
  - conflict detection (name / import / test-file alignment / circular deps)  → kept
  - "flag, don't silently resolve" blocker-surfacing discipline              → kept
  - the flag SINK reference "the cycle report" (×5)                          → kept-but-rebind
        (soften to intrinsic "surface in your report"; cycle-report artifact owned by executing-tdd-cycle/P4,
         bound caller-side)
  - soft workflow-phase vocabulary ("cycle execution", "previous cycles")     → kept-but-rebind
        (soften to intrinsic craft: "scoped extend/modify task", "prior work")
  - anti-rationalization spine (Common Mistakes / Rationalizations / Red Flags / "No exceptions") → kept
        (code-modification SCOPE discipline — craft-specific; NOT loop-discipline's loop-gaming doctrine, so
         NOT dedupe→loop-discipline)
  - when-to-use / when-NOT-to-use scoping (EXTEND/MODIFY tasks; not greenfield/refactor) → kept
  - model-invocation triggers (graded MUST/SHOULD, exact phrases)            → kept-but-rebind
        (grammar already graded ✓; re-anchor "when the user says…" → work-context per wiring step 3)
  - classification (model-invoked) + router registration                    → kept + wiring-pass (new router entry)
```

**No `dropped`.** Every original responsibility lands. The two `kept-but-rebind` softenings and the single
`dedupe` are the entire delta from verbatim — consistent with `port-with-edits × standalone`.

*(Home cluster setup→implement is a REGISTRY re-file — metadata, not a responsibility — tracked in the
Home-confirmation section above, not as a trace tag.)*

---

## Reconcile flags (for `reconcile-cluster`, full-cluster context)

1. **flag-for-reconcile — marker-grammar single-source (cross-cluster).** `[EXTEND]`/`[MODIFY]` vocabulary
   is owned/authored by `patterns-vertical-tdd` (tasks cluster, ported); brownfield-integration **consumes**
   it at implement-time and currently restates the one-line meanings. Resolve as **dedupe/reference** (point
   at the canonical owner; keep only the implement-time consumption/interface-impact semantics here). NOT a
   merge — the skills are disjoint by altitude and cluster. Confirms the P4↔patterns-vertical-tdd
   boundary-integrity principle extends to the marker grammar.

## Wiring-pass notes (not structural — for `transform-recipes`)

- **Trigger de-collide with `executing-tdd-cycle` (P4)** on `[EXTEND]`/`[MODIFY]`: scope the two descriptions
  so both can co-fire cleanly (P4 = cycle execution / red-green-refactor; P6 = read-before-write EXTEND/MODIFY
  craft). No merge.
- **Re-anchor MUST-triggers** from "when the user says…" to agent-consumed work-context (wiring step 3).
- **Soften soft workflow-phase vocabulary** (`cycle execution`, `previous cycles`, `cycle report`) to
  intrinsic craft (decouple step 5); bind the cycle-report handoff caller-side.
- **Register** in the `mochiko` router (agent-consumed entry, when-to-reach guidance).
- **NO `[GAP:XXX]` reference present** (grep confirmed only `[EXTEND]`/`[MODIFY]`) — the GAP grammar
  (loop-discipline's) is **not** duplicated here; no GAP boundary flag.

---

**Assessment complete.** Every responsibility carries a tag; disposition is `port-with-edits × standalone`;
one cross-cluster reconcile flag (marker single-source) open for `reconcile-cluster`; home confirmed =
`implement` with the Phase-5 REGISTRY move noted; disjointness vs `analysis-codebase` confirmed.
