# Assessment — `patterns-vertical-tdd` (P3, tasks cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:assess-primitive` · **Date:** 2026-07-01
**Source:** `human-in-loop/plugins/humaninloop/skills/patterns-vertical-tdd/` (SKILL.md + `references/CYCLE-STRUCTURE.md` + `references/SLICE-IDENTIFICATION.md`; no bundled script)
**Run:** `/mochiko:transform-cluster tasks` · core-only (6 primitives) · **Status:** ASSESS ONLY — no edits made; relations emitted as `flag-for-reconcile`.

---

## ASSESSMENT (skill output format)

```
ASSESSMENT: patterns-vertical-tdd
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]   (≥1 yes → full 7-check lens)
Disposition:  port-with-edits × standalone
Trace:
  # ── SKILL.md body ──────────────────────────────────────────────────────────
  - Discipline-framing (the "letter = spirit" anchor; "tests-after verify implementation, not requirements") → kept   [discipline-enforcement core]
  - When to Use / When NOT to Use (bug fixes / docs / spikes / refactor / tests-exist exclusions)             → kept   [BUT L23 "when the task architect agent generates…" → decouple, below]
  - Core Principle 1 — Vertical over Horizontal                                                                → kept
  - Core Principle 2 — Test-First at Task Level                                                                → kept
  - Core Principle 3 — Foundation + Parallel                                                                   → kept
  - Core Principle 4 — Layered Testability                                                                     → kept
  - Identifying Vertical Slices (quick heuristics + slice-boundary table + ref → SLICE-IDENTIFICATION.md)      → kept
  - Cycle Structure (Standard Cycle Format + task numbering T{c}.{n} + markers table + ref → CYCLE-STRUCTURE.md) → kept   [F1: cycle-format ↔ P5 tasks-template.md alignment — reconcile]
  - Foundation vs Feature Cycles (purpose / characteristics / "could ANY story work in production?" test)      → kept   [cycle-TYPE taxonomy = craft, not a workflow-phase injection; keystone passes]
  - TDD Task Sequence (Red write-failing-test / Green implement / Refactor / Demo-validate task templates)      → kept   [authors the task TEMPLATE — design-time; NOT red/green/refactor runtime — B1 boundary vs executing-tdd-cycle]
  - Mapping Stories to Cycles (Simple = / Split > / Merge < cases)                                             → kept   [RQ-A: two-artifact vs one-artifact FRAMING rides on the reconcile gate; content kept either way]
  - Common Rationalizations (anti-rationalization table, 8 excuse→reality rows)                                → kept   [discipline-enforcement core]
  - Red Flags — STOP and Restart (the STOP thought-list + "No exceptions")                                     → kept   [discipline-enforcement core]
  - Common Mistakes (anti-pattern catalogue: tests-after / fake-vertical / missing-test-task / …)              → kept
  - Quality Checklist (pre-finalize producer self-check, 8 items)                                              → kept   [producer SELF-check, NOT an independent grade → not promote material]
  # ── references/CYCLE-STRUCTURE.md (progressive disclosure) ──────────────────
  - Cycle anatomy / task-ID format / file-path conventions / foundation+feature examples / brownfield markers / checkpoint guidelines / worked "Complete Example" tasks.md → ported-with-skill (kept)   [F1: "Complete Example" full tasks.md ↔ P5 template dedupe — reconcile]
  - Verification Task Requirements (TEST: / Setup·Action·Assert format the architect AUTHORS into the final cycle task) → ported-with-skill (kept)   [authoring the verification task = task-structuring; the RUNTIME consumer is deferred qa-engineer/testing-end-user — B1]
  - qa-engineer runtime-classification prose (L228, L332) + "downstream verification agent" (L236)             → kept-but-rebind   [DECOUPLE: name the downstream verification STEP/consumer by role, not the agent; qa-engineer is deferred/out-of-scope]
  # ── references/SLICE-IDENTIFICATION.md (progressive disclosure) ─────────────
  - Value-stream test / extraction-from-user-stories / size calibration / dependency analysis / domain examples / anti-patterns / decision matrix → ported-with-skill (kept)
  - constraints-and-decisions.md IP-XXX platform-foundation input (L40/L101/L118; also SKILL.md L153)          → kept   [upstream plan-cluster ARTIFACT input the architect reads; command-owned handoff edge, not skill coupling]
  # ── conventions / wiring floor ─────────────────────────────────────────────
  - classification tag (absent in HIL)                                                                          → assigned model-invoked (default; agent-consumed) via wiring pass
  - model-invocation trigger `description` (HIL "when the user says…" user-utterance form)                      → kept-but-rebind   [user-utterance → WORK-CONTEXT; retain graded MUST/SHOULD; match patterns-entity-modeling shape — the primary body edit]
  - router registration (absent — NO tasks-cluster section exists in the mochiko router; verified)             → added via wiring pass (new Tasks-cluster router row + when-to-reach-it)
  - "the task architect agent" sibling-agent name (SKILL.md L23)                                                → kept-but-rebind   [DECOUPLE: state by role — "when authoring implementation task artifacts"]
Reconcile flags:
  F1 — P3 patterns-vertical-tdd ↔ P5 tasks-template.md : the skill's Standard Cycle Format + CYCLE-STRUCTURE.md worked "Complete Example" (a full tasks.md) vs P5's template → confirm ONE canonical cycle/tasks structure + decide if the worked example lives in the skill or references P5 (dedupe). NOT a merge.
  RQ-A dependency — the skill teaches BOTH story→cycle mapping AND cycle→TDD-tasks; whether it frames output as TWO artifacts (task-mapping.md → tasks.md) or ONE (tasks.md w/ a mapping section) rides on the contract's RQ-A, human-gated at reconcile. Body-FRAMING dependency; content kept regardless; not a structural move.
  B1 — cross-cluster boundary vs `executing-tdd-cycle` (implement, DEFERRED/out-of-scope): boundary CONFIRMED CLEAN (structuring vs execution). No in-cluster dedup possible this run; reconcile records the forward-watch for the implement port.
```

---

## Step 1 — Class / branch

**skill → PLAYS-a-role.** It is `task-architect`'s (P2) core task-structuring authoring procedure — the producer skill the agent invokes to turn stories/plan-artifacts into the tasks artifacts (per `context.md`: producer `task-architect` ↔ reviewer `devils-advocate`+`validation-task-artifacts`, the single-reviewer `specify` shape). Weighting per branch: consumed-procedure vs emits-artifact, trigger reliability, **sibling overlap (P5 template + the executing-tdd-cycle boundary — load-bearing here)**, decoupling.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (kernel / md-supervisor / command / DAG to function)? | **no** — self-contained vertical-slicing + TDD-structuring guidance; survives without the brain (ROADMAP test passes). In HIL the DAG sequences the *agent* (task-architect) + the 2-phase `/tasks` loop drives revision — not this skill. `grep` of SKILL.md + both refs = **zero** `brain`/`hil-dag`/`catalog`/`mcp`/`StrategyGraph`/`DAG`/`.humaninloop`/`.workflow` tokens. |
| 2 | Multi-responsibility / fans out? | **yes** — slice identification + cycle structure + foundation/feature classification + TDD task-sequencing + story→cycle mapping + verification-task authoring; the emitted `task-mapping.md`/`tasks.md` fans out to **executing-tdd-cycle** (downstream execution) and **validation-task-artifacts** (the reviewer). |
| 3 | Emits artifact whose correctness is NOT machine-checkable? | **yes** — whether slices are genuinely *vertical*, whether stories map soundly to cycles, whether test-first ordering is correct, whether cycles are appropriately sized — all irreducibly model judgment. **No bundled linter here at all** (unlike P7's `validate-model.py`) → pure model judgment, which *strengthens* gate 3 and makes the P4 validator genuinely grounded (not degenerate). |

≥1 yes → **full lens**. Gates 2 + 3 tripped.

## Step 3 — The 7-check lens (weighted for PLAYS-a-role / skill)

1. **Orchestration test.** Content-coupling: **none** (grep clean — no kernel/DAG/catalog/MCP in body or either reference). Orchestration-coupling: the skill is *consumed* by the `task-architect` agent via the Skill tool; in HIL the DAG sequences the agent and the 2-phase (Mapping → Tasks) loop drives revision. **Nothing orchestration-bound lives *in* this skill → nothing re-homes *from* it.** The two-phase sequence, the cumulative Mapping↔Tasks review, the entry-gate, and the FAIL-loop belong to the **command (P1)** and are rehomed to the lead per the contract — not carried by this skill.

2. **Role (two altitudes).** Skill-role = **emits-artifact** (the `task-mapping.md`/`tasks.md` structure; also runs as a consumed design step, but its output is a reviewable artifact). Team-role conferred = **producer** — running it makes the architect a producer. The embedded **Quality Checklist is a producer self-check, NOT an independent grade → not promote material** (the independent grade is P4 `validation-task-artifacts`).

3. **Independence.** Producer↔grader boundary is **structural at cluster level**: `task-architect` mounts this producer skill; the grader `validation-task-artifacts` (P4) mounts on **`devils-advocate` — a different agent** (per `context.md`: "re-mount validation-task-artifacts on devils-advocate"). No self-grade leak at skill or agent level; this skill is unambiguously the **producer half**. *(Definitive confirmation rides on P2/P4's own assessments — flagged there, not here — but the producer identity holds regardless.)*

4. **Verdict-sink / loop-driver.** This skill emits **no verdict**. Verdict-sink for the task artifacts = the reviewer (P4 `validation-task-artifacts` on `devils-advocate`) + the lead's owned verdict. The FAIL-loop (revise-on-gaps, the Mapping→Tasks two-phase sequence, the cumulative cross-check back to `task-mapping.md`) is driven by the **command supervisor (lead)** per the contract's rehome list. **No loop owned here → nothing to rehome from this skill.**

5. **Sibling / overlap ("look sideways").** THE load-bearing check → detail in the dedicated sections below.
   - **In-cluster P5** (`tasks-template.md`): the skill teaches a Standard Cycle Format and ships a full worked "Complete Example" `tasks.md` (CYCLE-STRUCTURE.md L337–471); P5 is the actual template. Divergence/dedupe risk — **exactly the plan-cluster F1 (P7↔P5 data-model.md template) shape** → **flag F1**. NOT a merge.
   - **Cross-cluster `executing-tdd-cycle`** (implement, deferred/out-of-scope): the boundary check → **B1**, confirmed clean, forward-watch.
   - **In-cluster P4** (`validation-task-artifacts`) is the producer↔**validator pairing**, not an overlap/merge (correct, expected).
   → **flag-for-reconcile** (F1 + B1; RQ-A framing dependency noted).

6. **Coupling audit.**
   - **Hardcoded paths:** **none** `.humaninloop/`/`.workflow/`. **No `humaninloop:` namespace cross-refs to rebind** (simpler than P7). Artifact references (`task-mapping.md`, `tasks.md`, `spec.md`, `constraints-and-decisions.md`/IP-XXX, US-X) are **craft inputs/outputs** — the run brief's explicit carve-out: "references the task-architect's artifacts (task-mapping.md, tasks.md) as the things it structures — that's the craft, not coupling." Example source/test paths (`src/models/task.py`, `tests/e2e/test_[feature].py`) are craft illustrations of the "every task needs a file path" rule. Keystone test passes on all.
   - **Prerequisites / handoffs:** assumes upstream `spec.md` (user stories US-X) and plan artifacts (`constraints-and-decisions.md` IP-XXX for platform-foundation cycles) exist; emits `task-mapping.md`/`tasks.md` consumed downstream by `executing-tdd-cycle` (execution) + `validation-task-artifacts` (review). All **command-owned handoff edges**, not skill-owned.
   - **Determinism boundary:** **no bundled script** — correctness is *entirely* model judgment (vertical slicing, story→cycle mapping, test-first ordering, sizing, foundation/feature classification). The producer↔validator pairing (P3→P4) is therefore **real, not degenerate**: P4 must apply grounded model judgment, not a deterministic assert.

7. **Conventions + loop placement.**
   - **Classification:** **absent** → assign **model-invoked** (default; agent-consumed) via wiring.
   - **Discoverability:** **not in the mochiko router** — verified: the router has doctrine/transform/setup/specify/plan sections but **no Tasks-cluster section at all** → register via wiring (new Tasks-cluster row + when-to-reach-it guidance).
   - **Reliable model-invocation:** current `description` is the HIL **"This skill MUST be invoked when the user says …"** user-utterance pattern. For an **agent-consumed** skill mochiko describes the **work context** → `port-with-edits` rephrase (retain graded MUST/SHOULD; re-anchor to task-structuring / vertical-slice-TDD authoring work). **The primary body edit.** Match the ported `patterns-entity-modeling` shape.
   - **Agent↔skill composition / decoupling scan:** **TWO deny-list hits** (detail below) — `SKILL.md:23` "the task architect agent" (sibling-agent name) and `CYCLE-STRUCTURE.md:228/332` `qa-engineer` (deferred downstream agent) → **DECOUPLE (`kept-but-rebind`)**. Otherwise clean: no "dispatch," no "workflow-agnostic" meta-label, no injected workflow modes/paths/phases (Foundation-vs-Feature is a *cycle-type domain taxonomy*, and brownfield `[EXTEND]`/`[MODIFY]` markers are a *domain concern* — both pass the keystone test; the "phase: mapping | tasks" risk the contract names lives in the **P2 persona**, not this skill).
   - **Producer↔validator pairing:** the task artifacts' independent validator is **P4 `validation-task-artifacts`** (different agent — `devils-advocate` — different skill), guaranteed structurally at cluster level. This skill is the producer half; **no bundled self-check** exists, so no degenerate-validator confusion.
   - **Sound-loop:** the enclosing tasks loop's done-condition / independent validation / human gate are the **command's** concern (P1 + contract), not this skill's. **No loop gap owned here.**

## Step 4 — Disposition

**`port-with-edits × standalone`** (matches the `context.md` precedent for the pattern-skills, and the plan-cluster P7 precedent).

- **Body = port-with-edits** (not keep-verbatim, not redesign): substance is high-quality and mochiko-clean (slicing heuristics, cycle structure, TDD task sequencing, story-mapping, anti-rationalization table, red flags, Common Mistakes, checklists — all `kept`; both references kernel-free). Edits are **localized**: (a) rephrase the `description` from user-utterance triggers to **work-context** triggers; (b) **DECOUPLE** "the task architect agent" (SKILL.md L23) → role phrasing; (c) **DECOUPLE** the `qa-engineer` runtime references (CYCLE-STRUCTURE.md L228/L332) → downstream-verification-step by role. **No** namespace rebinds needed (zero `humaninloop:` cross-refs). Not `keep-verbatim` because those fixes are required; not `redesign` because the approach assumes no kernel/DAG/catalog and is right for mochiko.
- **Structural = standalone**: the architect's distinct task-structuring producer procedure, mounted on `task-architect`. Its **placement** does not depend on a sibling merge/split/promote. F1 (P5 template alignment) is a dedupe/content-alignment question that may add a reference or align the worked example, but does **not** dissolve P3 into P5 or split it into a validator. RQ-A affects the skill's output **framing** (two artifacts vs one), not its placement. B1 is a cross-cluster forward-watch. Hence structural stays `standalone`; F1/RQ-A/B1 are flagged, not resolved.

## Step 5 — Responsibility trace

Complete trace in the ASSESSMENT block above. **No `dropped` responsibilities** → no silent loss, nothing requiring a lead-accepted drop reason. Tag distribution: most `kept`; both reference files `ported-with-skill` (grep-confirmed kernel-free); four `kept-but-rebind` (description triggers, the task-architect agent name, the qa-engineer references); two new wiring assignments (classification, router). Two `kept` items carry cross-primitive relational signals reconcile must weigh: the **cycle-format/tasks.md template alignment with P5** (F1) and the **artifact-shape framing** (RQ-A). One `kept` item (TDD Task Sequence + Verification Task authoring) sits on the **executing-tdd-cycle boundary** (B1) — owned here as *authoring*, not *execution*.

---

## SIBLING-OVERLAP FINDING F1 → flag-for-reconcile  (in-cluster: P3 ↔ P5 template alignment)

**Pair:** `patterns-vertical-tdd` (P3, the producer skill) ↔ `tasks-template.md` (P5, the emitted-artifact template).

**The overlap.** P3 teaches the target structure in two places and P5 *is* that structure:
- **P3 SKILL.md "Standard Cycle Format"** (L114–127) — the cycle skeleton (`> Stories / > Dependencies / > Type`, `TN.1..TN.4`, `**Checkpoint**`).
- **P3 CYCLE-STRUCTURE.md** — a full "Complete Example: Task Management Feature" `tasks.md` (L337–471), plus cycle anatomy, task-ID format, and the TEST: verification block.
- **P5 `tasks-template.md`** — the canonical `tasks.md` the architect fills.

**Why this is the plan-cluster F1 shape (not a merge).** Same defect class as plan's P7↔P5 divergent `data-model.md` templates: a *producer skill* carrying an artifact structure that a *template primitive* also owns → risk of two divergent sources. It is **not** a thin-variant-over-shared-core (distinct primitives, distinct jobs: teach-the-craft vs fill-the-skeleton).

**Resolution menu for `reconcile-cluster` (do not decide here):**
- **(i)** P5 `tasks-template.md` is the single canonical `tasks.md` structure; P3 *references* it and keeps only the teaching/rationale (heuristics, anti-rationalization, worked example as *illustration* pointing at the template). **My lean** — mirrors plan's "collapse to one canonical template."
- **(ii)** Keep the worked "Complete Example" in P3 as pedagogy, but reconcile MUST confirm P3's cycle-format and P5's template are **byte-aligned on structure** (markers, task-ID scheme, checkpoint line, TEST: block) so they cannot drift.
Whichever wins, reconcile confirms **one** canonical cycle/`tasks.md` structure and repairs any drift. **Also gated here: RQ-A** (below) sets whether that canonical structure is one artifact or two.

**Flag payload:** `P3 patterns-vertical-tdd ↔ P5 tasks-template.md : the skill's Standard Cycle Format + the CYCLE-STRUCTURE.md worked "Complete Example" tasks.md vs P5's template — same divergence class as plan F1 (P7↔P5 data-model.md). Confirm ONE canonical cycle/tasks.md structure; decide if the worked example lives in P3 (illustration) or references P5. NOT a merge.`

## RECONCILE DEPENDENCY (RQ-A) → the artifact-shape decision touches this skill's body framing

The contract's **RQ-A** (one deliverable vs two) is the reconcile question this skill most directly informs. P3 teaches **both** halves — story→cycle **mapping** ("Mapping Stories to Cycles", Simple/Split/Merge cases) **and** cycle→TDD-**tasks** (the cycle structure). Its output *framing* therefore rides on RQ-A:
- **Two artifacts** (`task-mapping.md` → `tasks.md`, HIL + plan two-phase precedent): P3's overview/description names two deliverables; the mapping section describes `task-mapping.md`.
- **One artifact** (`tasks.md` with a Story→Cycle mapping section): P3 frames a single deliverable; mapping becomes a section.
**The content is `kept` either way** — only the naming/overview framing shifts. This is a **body-framing dependency, not a structural move**; the disposition (`port-with-edits × standalone`) is unaffected. Surfaced here per the contract ("To be surfaced by the assessments and gated at reconcile").

## BOUNDARY FINDING B1 → flag-for-reconcile  (cross-cluster: P3 vs `executing-tdd-cycle`, DEFERRED)

**Pair:** `patterns-vertical-tdd` (P3, tasks cluster) vs `executing-tdd-cycle` (implement cluster — **deferred / out-of-scope this run**, per `context.md`).

**Boundary CONFIRMED CLEAN — distinct cores, distinct artifacts, distinct altitude:**

| | `patterns-vertical-tdd` (P3) — **task-STRUCTURING** | `executing-tdd-cycle` (deferred) — **cycle-EXECUTION** |
|---|---|---|
| **When** | Design-time (authoring the plan of tasks) | Build-time (running the tasks) |
| **Owns** | Story→cycle mapping, vertical-slice identification, **test-first task ORDERING**, foundation/feature classification, the cycle/task *template* | **Red/green/refactor RUNTIME** — parse `tasks.md`, write the failing test, implement to pass, refactor, mark complete |
| **Emits** | `task-mapping.md` / `tasks.md` **structure** | Implemented **code** + `cycle-report.md` |
| **Shared** | Vocabulary only ("cycle", "red/green/refactor", "TDD") + the "letter = spirit" discipline banner | — |

**Hard evidence for the boundary:** `grep` for `cycle-report` across all three P3 files returns **zero** — P3 stops at `tasks.md` and never touches the execution-side artifact. P3's "TDD Task Sequence" and CYCLE-STRUCTURE.md's Verification block *author the task template* (what the task text says); they do **not** execute red/green/refactor. **P3 does NOT bleed into cycle execution.** ✓

**Why it is nonetheless flagged (a forward-watch, not a live move):** `executing-tdd-cycle` is deferred/out-of-scope, so **reconcile-cluster (tasks) has no in-cluster sibling to dedup against and cannot resolve a cross-cluster move this run.** The note reconcile should *record*: when the implement cluster ports, confirm the boundary still holds — P3's red/green/refactor **task template** vs executing-tdd-cycle's red/green/refactor **execution discipline** must stay distinct (clean structure→execute handoff), not duplicate into a dedupe defect. The shared discipline banner and vocabulary are acceptable (a meme/anchor, not substantive content).

**Flag payload:** `P3 patterns-vertical-tdd vs executing-tdd-cycle (implement, DEFERRED): boundary confirmed CLEAN — P3 owns task-structuring (design-time mapping + test-first ORDERING + tasks.md structure), executing-tdd-cycle owns cycle-execution (runtime red/green/refactor + code + cycle-report.md); zero cycle-report reference in P3 (grep-confirmed). No in-cluster dedup possible this run; forward-watch for the implement port. NOT a merge, NOT a this-run resolution.`

---

## Trigger notes (model-invocation reliability)

- **Form:** current `description` is the HIL **"This skill MUST be invoked when the user says 'create task mapping', 'structure implementation', 'define cycles', 'vertical slice', 'TDD', 'test first', 'cycle structure', or 'testable increment'…"** — the **user-utterance** pattern. For an **agent-consumed** skill mochiko describes the **work context** → `port-with-edits` rephrase. Target shape (match `patterns-entity-modeling`): *"This skill MUST be invoked when structuring a feature's implementation into vertical slices with test-first discipline — mapping user stories to cycles, ordering each cycle's tasks test-first (failing test → implement → refactor → verify), classifying foundation vs feature cycles, and authoring the task-mapping/tasks structure. SHOULD also invoke when the work involves 'vertical slice', 'TDD', 'test-first', 'cycle structure', 'red-green-refactor', 'story→cycle mapping', or 'testable increment'."* Retain graded MUST/SHOULD; re-anchor to the authoring work. **The primary body edit.**
- **Sibling collision (de-collide in wiring):** within the tasks cluster this skill is the sole task-structuring producer (P4 is the validator, P5/P6 templates) → **no in-cluster trigger collision**. Cross-cluster: `executing-tdd-cycle`'s deferred triggers overlap on `"TDD" / "red green refactor"`; work-context phrasing (P3 = *structuring/ordering* tasks; executing = *running* them) separates them — note for the wiring pass when implement ports, not a structural problem now.

## Decoupling-scan hits (run-goal: empirical decoupling-doctrine test)

**Two deny-list hits, both `kept-but-rebind` / decouple:**
1. **`SKILL.md:23`** — *"When the task architect agent generates implementation artifacts"* — a **sibling-agent name** (the P2 `task-architect`) in the skill body. → restate by **role**: *"When authoring implementation task artifacts."* Independence stated by role, not agent name.
2. **`references/CYCLE-STRUCTURE.md:228` + `:332`** — *"The qa-engineer classifies tasks at runtime…"* / *"the qa-engineer accepts these legacy markers…"* — a **deferred downstream agent name** (`qa-engineer`, implement cluster, out-of-scope). → restate by **role**: *"the downstream verification step classifies tasks at runtime…"*. (`:236` "the downstream verification **agent**" is already role-phrased — borderline; optional tidy to "verification step" for zero ambiguity.)

**Everything else CLEAN:** grep + manual read found **no** "dispatch," **no** "workflow-agnostic"/independence-by-declaration meta-label, **no** injected workflow modes/paths/phases in the body, and **no** `.humaninloop/`/`.workflow/` paths. Foundation-vs-Feature is a **cycle-type domain taxonomy**; brownfield `[EXTEND]`/`[MODIFY]` markers are a **domain concern** — both pass the keystone test (true of this craft on any job → keep). The artifact references (`task-mapping.md`/`tasks.md`/`spec.md`/`constraints-and-decisions.md`) are the deliverables the craft structures — explicitly carved out as craft, not coupling. The contract's flagged high-risk "phase: mapping | tasks" language lives in the **P2 persona**, not here. `verify-output`'s decoupling scan + keystone test should confirm post-transform.

## Reference-file confirmation (per run brief)

Both reference files **come along with the skill** and are tagged **`ported-with-skill`** in the trace:
- `references/CYCLE-STRUCTURE.md` — **kernel-free** (grep: zero brain/DAG/MCP/catalog tokens). Carries the two `qa-engineer` decouple hits (rebind to role) and the TEST: verification-authoring format (task-structuring; runtime consumer deferred — B1). Also the F1 worked-example alignment with P5.
- `references/SLICE-IDENTIFICATION.md` — **kernel-free** (grep: zero brain/DAG/MCP/catalog tokens). Carries the upstream `constraints-and-decisions.md` IP-XXX platform-foundation input (craft handoff edge, kept).

---

**Done-condition (this assessment):** every responsibility tagged ✓ (SKILL.md body + both reference files + description-triggers + router-registration wiring, no silent drops) · no untagged responsibility ✓ · relational moves flagged (F1 P5-template alignment, RQ-A framing dependency, B1 executing-tdd-cycle boundary), not guessed ✓ · description-convention + router + boundary/dedup + decoupling checks explicit ✓ · ASSESS-only, no edits ✓.
