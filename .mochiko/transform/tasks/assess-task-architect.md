# Assessment — `agents/task-architect.md`

Run: `tasks` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source: `human-in-loop/plugins/humaninloop/agents/task-architect.md`
Class: **agent → PLAYS-a-role branch**
Cluster role: **PRODUCER** of the tasks workflow (authors `task-mapping.md` + `tasks.md`). Reviewed by
the **existing** `devils-advocate` + `validation-task-artifacts` — a **single-reviewer** loop (the
`specify` shape, not `plan`'s two-form; contract §2). The `tasks` command supervisor (P1) is
lead/referee. **Net-new to mochiko** (no existing `task-architect.md` — fresh port, `standalone`).

> ROLE NOTE: assess/diagnose ONLY. No transform, no grading. Relational moves (producer-report home,
> RQ-A artifact shape) are emitted as `flag-for-reconcile`, not resolved here. `patterns-vertical-tdd`
> (P3) is being ported in parallel this run — the mount is noted, not fired; it resolves when P3 lands.

**Headline:** this is the **highest-risk decoupling surface of the run** (contract §1). Unlike
`technical-analyst` (3 hits, all one benign class, worst categories clean), this persona is **coupled
on the worst categories**: it names **two sibling agents** (Devil's Advocate ×1, qa-engineer ×4), it is
organized *around* an injected **phase-mode** skeleton, it "reads its instructions from a **context
file**," and its inline report **self-asserts a clearing verdict**. The intrinsic craft (vertical
slicing, test-first discipline, foundation+parallel, traceability, reject/embrace, brownfield markers)
is mochiko-sound and survives; the this-loop machinery goes. Body treatment is a **heavy
`port-with-edits`** (whole-section excisions) — but still short of `redesign`, because the body assumes
**no kernel/DAG/catalog** and the craft is preserved, not re-conceived.

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting: persona-vs-procedure split, the **team-role it confers** (here:
PRODUCER), its **`skills:`-list independence** (does it author AND grade?), and **persona decoupling**
(no sibling-agent names / "dispatch" / injected workflow modes-paths-phases / "workflow-agnostic"
labels). Loop-ownership is NOT this primitive's concern — it sits on the orchestrator (HIL: the `tasks`
markdown supervisor + `state-analyst`/DAG; mochiko: the `tasks` command supervisor, P1).

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** The persona explicitly reads a **context file** (L29, L342–351)
   that a supervisor writes — it is told which **phase** to run (L30), where its inputs live
   (`file_paths`, L349), and where to write its report (L111). It only *functions* because a supervisor
   sequences mapping→tasks and feeds it. This is **orchestration-coupling** (the supervisor drives it)
   **plus content-coupling** (the persona bakes the context-file/phase contract and two sibling-agent
   names into its body).
2. **Multi-responsibility / fans out?** **YES.** Two authoring artifacts across two phases
   (`task-mapping.md`, then `tasks.md`), a systems/architect persona, ~130 lines of verification-task
   grammar, and brownfield-marker craft. Feeds the reviewer (`devils-advocate`), the lead, and
   downstream `implement`.
3. **Non-machine-checkable artifact?** **YES.** Vertical-slice integrity, TDD ordering, story→cycle
   traceability, foundation-vs-feature classification, and "is this slice truly vertical" are **model
   judgment**, not a schema/version assert. ⇒ a **real grounded validator partner is required**
   (`validation-task-artifacts` on `devils-advocate` — cluster wiring; contract §2).

All three **yes → full 7-check lens.** Gates tripped: 1, 2, 3.

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer (HIL):** a **markdown supervisor + DAG** — `commands/tasks.md` (with
  `state-analyst`/`hil-dag`) writes the `.workflow/tasks-context.md` carrier, sequences
  Mapping→Tasks, injects inputs/paths, and loops the architect↔advocate convergence. This agent is a
  dispatched producer node.
- **Content-coupling: PRESENT and heavier than `technical-analyst`.** The body bakes the **context-file
  dispatch contract** ("You read your instructions from a **context file**," L29; the entire "Reading
  the Context" section, L342–351, incl. `phase` L345, `supervisor_instructions` L346, `file_paths`
  L349), the **phase-mode skeleton** ("How You Operate" L27–35 → "Phase Behaviors" L38–107), and
  **two sibling-agent names** (Devil's Advocate L151; qa-engineer L248/L274/L279/L337). No
  Python/MCP/DAG/catalog *token* in the body (grep `dag|mcp|brain|catalog|hil-dag|node` → none) — the
  coupling is to a **markdown supervisor and sibling agents**, not a kernel. That keeps it out of
  `redesign` territory but makes the decouple substantial.
- **Orchestration-coupling:** dispatched producer node — relies on the supervisor for invocation, for
  which-phase/which-inputs/where-to-write injection, and for the FAIL/convergence loop on the advocate
  verdict, **plus** the Mapping→Tasks **sequencing** and the entry-gate (plan-complete prerequisite).
  Those re-home onto the **`tasks` command supervisor (P1)** (contract §1 names them explicitly as
  must-rehome-not-drop). This agent owns **no loop-driving** of its own to re-home; the sequencing it
  *implied* via the phase-mode is `moved-to-lead`.

### 2. Role at two altitudes
- **Skill-role:** **artifact-emitter** — wraps `patterns-vertical-tdd` (P3) and emits two reviewable
  artifacts. → needs an independent producer↔validator partner.
- **Team-role conferred:** **PRODUCER — single and unconflicted.** Its one mounted skill
  (`patterns-vertical-tdd`) is an authoring/patterns (producer) skill. **No grading/validation skill on
  this agent.** A mochiko agent should confer exactly one team-role; this one does (producer).
- **Partner is SETTLED (unlike plan's RQ1):** the reviewer is the **existing `devils-advocate`** driven
  by **`validation-task-artifacts`** (P4) — a single-reviewer loop (contract §2). The re-mount of
  `validation-task-artifacts` onto `devils-advocate` is **cluster wiring** (the stub is already in the
  agent — plan report follow-up), **not** a move on this primitive.

### 3. Independence — CONFIRMED CLEAN (no self-grade leak)
- **Self-grade leak: NO.** `skills:` (L6) = `patterns-vertical-tdd` — an authoring (producer) skill;
  **not a grader.** No `validation-task-artifacts`, no `verify-*`. The agent authors; it never grades.
- **Producer skills ∩ grading skills = ∅.** Independence is **structural and intact on the producer
  side** — the same textbook-correct shape as `technical-analyst`. **No split needed; no
  `flag-for-reconcile` for an independence *leak*.**
- **One independence hazard lives in the *report*, not the skills:** the inline "Planner Report"
  **self-asserts a clearing verdict** ("Completion: complete/partial" L122; "Ready for Review: …ready
  for Devil's Advocate review" L149–151). A producer must **not** self-assert clearing — the lead owns
  the verdict (cf. the plan port dropping `techanalyst-report`'s `completion_status`). → those fields
  are `dropped + reason`; the report *home* itself is `flag-for-reconcile`.

### 4. Verdict-sink / loop-driver
- **Consumers of output:** `task-mapping.md` + `tasks.md` are consumed by the **reviewer**
  (`devils-advocate`+`validation-task-artifacts` — grades, incl. the **cumulative** cross-check of
  `tasks.md` back to `task-mapping.md`, contract §1), by the **lead** (converge-vs-iterate), and
  downstream by `implement`.
- **What loops on FAIL today:** the HIL `tasks` supervisor/DAG drives the architect↔advocate
  convergence and the Mapping→Tasks sequencing. **None of that lives in this agent.**
- **Where it re-homes:** onto the **`tasks` command supervisor (P1)** as a bounded produce→review loop
  (round cap 3 + no-progress + human gate) — the `specify` single-reviewer shape. Originates from P1 /
  the dissolving supervisor, listed here for the rehome map's completeness.

### 5. Sibling / overlap ("look sideways")
- **Agent siblings:** `devils-advocate` is a **complementary role (validator), not a variant** — no
  merge. `qa-engineer` / `staff-engineer` / `state-analyst` are **out-of-scope / dissolved** (context.md)
  — the qa-engineer references in this body are coupling to a downstream cluster, not a sibling to
  reconcile with here.
- **Skill overlap with P3 (the real signal):** large tracts of this persona **restate procedure P3
  owns** — the Cycle Structure template (L214–235 ≈ P3 CYCLE-STRUCTURE / Standard Cycle Format),
  slice-identification and foundation-vs-feature tests (Success Criteria L61–69/L97–106 ≈ P3
  SLICE-IDENTIFICATION + Quality Checklist), the brownfield marker table (L195–200 ≈ P3 Markers), and
  the reject/embrace lists (≈ P3 Common Rationalizations/Mistakes). → a **`dedupe`/`folded-into-skill`
  signal**: canonical home is P3; the persona keeps *taste*, defers *literal procedure*.
- **Mapping vs Tasks = NOT an agent-split signal** — one architect authoring two artifacts (like
  `technical-analyst` holding analysis+design). Whether they stay two artifacts or collapse to one is
  **RQ-A** (artifact shape), a reconcile decision — but it never splits the *agent*.

### 6. Coupling audit
- **Hardcoded paths:** **L188** `.humaninloop/memory/codebase-analysis.md` → `kept-but-rebind`
  (`.mochiko/`; real mochiko artifact, produced by setup's `analysis-codebase`). **L189**
  `.humaninloop/memory/evolution-roadmap.md` → **`deferred-by-reference`** (roadmap track deferred
  core-only, contract §1 / context.md; documented stub, path noted, **not a live mount** — as setup
  stubbed `syncing-claude-md`). Input-file lists (L43–50, L76–83) are **injected inputs**, not literal
  hardcoded paths — caller-side dispatch context → `moved-to-lead`.
- **`skills:` references:** 1 (`patterns-vertical-tdd`, P3) — **in-scope, ported in parallel this run**
  → `kept-but-rebind` (frontmatter L6 stays bare per the ported-agent convention; body ref L15 gets the
  `mochiko:` prefix). **Do NOT mount unported skills** — P3 is in-cluster, so the mount is valid once
  P3 lands.
- **Upstream prerequisites:** assumes the lead handed it the plan-workflow outputs (spec/plan/
  constraints/data-model/contracts) and, on the tasks phase, `task-mapping.md` — **handoff edges owned
  by P1's contract** (incl. the plan-complete entry-gate), not the agent.
- **Determinism boundary:** essentially **all model judgment** (slice identification, TDD ordering,
  traceability, foundation classification). ⇒ the validator partner (`validation-task-artifacts`) is a
  **real grounded validator**, not a degenerate assert.

### 7. Conventions + loop placement — DECOUPLING SCAN (run headline)

**Persona-vs-procedure split.** Persona **keep** (taste/judgment): Core Identity (L19–25 war stories:
oversized/misordered tasks, horizontal-slicing pain, tasks not mapping to value, integration
nightmares), Quality Standards (L154–166 as *taste*), What You Reject/Embrace (L168–183), the
brownfield-marker *disposition*, and the **menu** of what it produces (a story→cycle mapping and a
cycle-based task list — output-shape is allowed, like `technical-analyst`'s menu). Procedure **→ P3**
(already factored, or to fold): the literal Cycle Structure template, slice-identification heuristics,
foundation-vs-feature test, TDD task sequence, verification-task grammar, marker table.

**Decoupling scan — DENY-LIST HITS BY CLASS (every hit + its fix). Worst categories are PRESENT here.**

**A. Sibling-agent names (the worst category — coupled, unlike `technical-analyst`):**
| # | Line | Token | Fix |
|---|------|-------|-----|
| A1 | L151 | "Devil's Advocate" ("ready for **Devil's Advocate** review") | Drop the sibling name **and** the whole self-asserted "Ready for Review" line (independence: lead owns the verdict; reviewer identity is caller-side, in P1's dispatch). |
| A2 | L248 | "qa-engineer" ("The qa-engineer will determine whether to auto-approve…") | Drop. Runtime-classification (auto-approve vs human checkpoint) is **downstream implement/qa** orchestration → `moved-to-other-cluster`. |
| A3 | L274 | "qa-engineer" ("The qa-engineer will classify the task at runtime") | Same → drop / `moved-to-other-cluster`. |
| A4 | L279 | "qa-engineer" ("the qa-engineer handles the 'when to involve human' decision") | Same → drop / `moved-to-other-cluster`. |
| A5 | L337 | "qa-engineer" ("the qa-engineer also accepts…") | Same → drop / `moved-to-other-cluster`. |

**B. Injected workflow modes / phases (the organizing skeleton):**
| # | Line | Token | Fix |
|---|------|-------|-----|
| B1 | L30 | "Which **phase** you're in (mapping or tasks)" | Cut. A decoupled producer authors the artifact-type in its brief, not a self-selected phase. |
| B2 | L35 | "Based on the **phase**, you produce the appropriate artifact…" | Cut. |
| B3 | L38–107 | "## Phase Behaviors / ### Phase: Mapping / ### Phase: Tasks" (the phase-mode skeleton) | Re-express as **artifact-types the producer authors** (a story→cycle mapping; a cycle-based task list) — like `technical-analyst`'s "What You Produce" menu — **not** injected phases. Content (success criteria, produce lists) survives as producer responsibilities; the *phase framing* is dropped. **Rides on RQ-A** (one artifact vs two). |
| B4 | L123 | "# Planner Report: {phase}" (phase interpolation) | Cut the `{phase}` interpolation (report-home is a reconcile flag). |
| B5 | L345 | "`phase`: Current phase (mapping/tasks)" | Cut (part of the context-file section, C-class). |

**C. "Reads instructions from a context file" / caller-injection / supervisor (markdown-supervisor carrier):**
| # | Line | Token | Fix |
|---|------|-------|-----|
| C1 | L29 | "You read your instructions from a **context file**…" | Cut. mochiko briefs via in-session dispatch (`agent-dispatch`), not a context file. |
| C2 | L111 | "write a report to the **report path specified in your context file**" | Cut. Report path is caller-side dispatch context → `moved-to-lead`. |
| C3 | L342–351 | "## Reading the Context" whole section (context-file fields) | **Cut entirely** — the whole section is the HIL dispatch contract baked into the persona. |
| C4 | L346 | "`supervisor_instructions`: Specific guidance for this iteration" | Drop (supervisor-carrier coupling; the deny-list token the contract §1 names by hand). |
| C5 | L349 | "`file_paths`: Locations of all input artifacts" | Cut; inputs are caller-side → `moved-to-lead`. |
| C6 | L351 | "Always start by **reading the context file**…" | Cut. |
| C7 | L43–50, L76–83 | "Read: spec.md / plan.md / constraints-and-decisions.md / data-model.md / contracts/ / Constitution" (enumerated inputs) | `moved-to-lead` — the dispatch brief supplies inputs; the persona doesn't self-enumerate them. |

**D. Hardcoded paths:** L188 `.humaninloop/…codebase-analysis.md` → `kept-but-rebind`; L189
`.humaninloop/…evolution-roadmap.md` → `deferred-by-reference` (roadmap stub). (See Check 6.)

**E. Self-asserted verdict (independence, not a persona-name but a decouple/drop):** L122
"Completion: complete/partial"; L149–151 "Ready for Review: yes/no" → `dropped + reason` (producer must
not self-assert clearing; lead owns it; mirrors the plan port's `completion_status` drop).

**FALSE POSITIVES / keystone-survivors (KEEP — do not over-scrub):**
- "vertical slice", "TDD", "test-first", "red-green-refactor", "foundation + parallel", "[US#]",
  "traceability", "file paths for every task" — **intrinsic craft** (true of any TDD architect).
- **Brownfield markers** `[NEW]`/`[EXTEND]`/`[MODIFY]`/`[GAP:XXX]` (L195–207) — **craft** (keystone:
  true of any architect touching existing code; brownfield-the-concept legitimately survived the port
  in mochiko `technical-analyst`/`principal-architect`). The `[GAP:XXX]` marker's *roadmap read* is
  `deferred-by-reference`; the **marker itself stays**.
- The **verification-task concept** — each vertical slice ending in a real-integration task that gates
  cycle completion (L237–246: "what makes vertical TDD actually vertical") — **craft**; only the
  qa-engineer handoff (A2–A5) and the heavy `TEST:` grammar (procedure → P3) are coupling.
- "Constitution alignment" (L50/L83/L142/L348) — **craft** (align work to governance, like
  `technical-analyst`); the *principles* arrive via the lead's brief (injection → `moved-to-lead`); the
  disposition to align stays (optional light generic reword).
- "Planner Report" label (L109/L123) — a **mislabel** (this is the task-architect, not a planner); note
  for the report-home reconcile flag.

**Convention wiring (present/missing):**
- **Classification:** model-invoked producer agent (set in wiring pass; agents aren't user-invoked
  router entries).
- **Discoverability / description:** the HIL description (L2–3) is a **bare two-sentence blurb with NO
  `<example>` blocks** — *lighter than* `technical-analyst` (which shipped 4). → **heavier wiring on
  this axis**: author `<example>` blocks describing work-context (turn planning artifacts into a
  cycle-based task list), and register in the `mochiko` router + `plugin.json` (cluster wiring).
- **Reliable model-invocation:** N/A at agent frontmatter (skill-level concern; P3 owns its triggers).
- **Producer↔validator pairing:** **REQUIRED and SETTLED** — `devils-advocate` +
  `validation-task-artifacts` (P4), guaranteed structurally (separate reviewer agent), never in this
  persona. Re-mount is cluster wiring.
- **Sound-loop:** done-condition / independent validation / human gate land on **P1's supervisor loop**
  (rehome-orchestration), not on this agent.

---

## Step 4 — Disposition

- **Body treatment: `port-with-edits`** — but the **heaviest port of the run** (whole-section
  excisions, not line edits). The craft core (Core Identity, Quality Standards as taste, Reject/Embrace,
  brownfield markers, the artifact menu) is **kept/lightly-edited**; the coupling scaffolding is
  **excised**: the "How You Operate" + "Phase Behaviors" phase-mode skeleton (B1–B3), the "Reading the
  Context" section (C1–C6), the two sibling-agent names (A1–A5), the inline report's self-verdict
  (E), the ~130-line `TEST:` grammar (folded to P3 / moved to implement-cluster). Rebinds: the P3 body
  ref → `mochiko:`; `codebase-analysis.md` → `.mochiko/`.
  - **Why not `keep-verbatim`:** the decouple/excise edits + rebinds are mandatory and extensive.
  - **Why not `redesign`:** the body assumes **no kernel/DAG/catalog**; the coupling is to a markdown
    supervisor + sibling agents, and the craft is **preserved, not re-conceived**. We carry the
    responsibilities and strip mechanism — the definition of `port-with-edits`, at heavy weight. The
    minimalism governor forbids redesigning what excision-plus-edit fixes.
- **Structural move: `standalone`** — decidable solo: no self-grade leak → **no `split`**; complementary
  reviewer, not a variant → **no `merge`**; not a check → **no `promote`**; real reusable producer body
  → **not `absorb-into-lead`**. Lands `standalone` in `agents/` as a **net-new** mochiko agent. The
  supervisor→agent-team rewiring of the *cluster* is **P1's** move, not this agent's.
- **Disposition: `port-with-edits × standalone`** — **WITH two `flag-for-reconcile` items** (human-gated
  per contract §4): (a) the **producer-report home**, and (b) **RQ-A artifact shape**, on which the
  phase-behaviors decouple (B3) rides. The disposition itself is stable under both RQ-A branches; only
  the artifact-menu content (one item or two) changes.

---

## Step 5 — Responsibility trace (complete; no silent drops)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Task-architect persona & judgment — oversized/misordered-task scars, horizontal-slice pain, tasks-must-map-to-value, integration-nightmare avoidance (Core Identity L19–25) | `kept` (keystone-clean intrinsic craft) |
| 2 | Vertical-slicing + TDD-discipline **procedure** via `patterns-vertical-tdd` (P3) | `kept-but-rebind` (frontmatter L6 bare; body L15 → `mochiko:`; **stays mounted** — P3 in-cluster, ported in parallel) |
| 3 | Author a **story→cycle mapping** (map P1/P2 stories to cycles; slice rationale; foundation/feature identification; IP-XXX→foundation-cycle; inter-cycle dependencies; traceability) | `kept` (producer authors this artifact-type; expressed as craft, **not** an injected "phase") + literal HOW `dedupe`→P3; **artifact-distinctness rides on RQ-A** |
| 4 | Author a **cycle-based `tasks.md`** (TDD cycles; foundation sequential + feature `[P]`; file path per task; `[US#]` traceability; brownfield markers) | `kept` (producer artifact-type) + literal HOW `dedupe`→P3 |
| 5 | **Phase-mode framing** ("read which phase, produce the appropriate artifact"; How You Operate + Phase Behaviors *as injected modes*, L27–35/L38–107) | `dropped + reason` (injected-workflow-mode coupling; a decoupled producer authors the artifact-type in its brief, not a self-selected phase — content survives as #3/#4). **Human-gate accept.** |
| 6 | Mapping→Tasks **sequencing** the phase-mode encoded | `moved-to-lead` (P1 dispatches mapping, then tasks; the producer never self-sequences) |
| 7 | **Context-file mechanism** ("reads instructions from a context file"; the "Reading the Context" section; `supervisor_instructions`; `file_paths`; "always start by reading the context file" — L29/L111/L342–351) | `dropped + reason` (markdown-supervisor carrier; mochiko uses in-session dispatch briefs + workspace-as-state, not a context file). **Human-gate accept.** |
| 8 | **Injected input list** (spec/plan/constraints/data-model/contracts/constitution — L43–50, L76–83, `file_paths` L349) | `moved-to-lead` (caller-side dispatch brief / `agent-dispatch`; producer consumes what it's handed, doesn't self-enumerate paths) |
| 9 | Constitution/governance **alignment** disposition (L50/L83/L142) | `kept` (craft: align work to governance, like `technical-analyst`); the *principles injection* → `moved-to-lead`; optional light generic reword |
| 10 | Success-Criteria + Quality-Standards quality bars (L61–69/L97–106; L154–166) | `kept` (as producer **taste**) + `dedupe` (literal criteria → P3 Quality Checklist author-side; the review-side mirror is **P4** `validation-task-artifacts`) |
| 11 | What You Reject / What You Embrace (L168–183) | `kept` (persona taste, keystone-clean) + light `dedupe` where it restates P3 Common Rationalizations/Mistakes |
| 12 | **Cycle Structure template** (`### Cycle N` format, TN.1→TN.4, checkpoint — L214–235) | `folded-into-skill` → P3 (P3 CYCLE-STRUCTURE + Standard Cycle Format is canonical; persona shouldn't carry the literal template) |
| 13 | **Verification-task concept** — a real-integration task that gates cycle completion ("what makes vertical TDD actually vertical", L237–246) | `kept` (producer taste: the architect specifies how each slice is verified) + `dedupe`→P3 |
| 14 | **`TEST:` verification-task grammar** (unified format, field reference, action modifiers, assert patterns, examples, legacy-format support — L260–341) | `folded-into-skill` → P3 (verification-task authoring *procedure* belongs in the skill, not the persona) |
| 15 | **qa-engineer runtime-classification handoff** (CLI auto-approve vs GUI/subjective human checkpoint; "who decides when to involve a human" — L248/L274–279/L337) | `moved-to-other-cluster` (implement/qa — `qa-engineer`/`testing-end-user`, **out of scope** this run) **+ the 4 sibling-name tokens `dropped + reason`** (deny-list) |
| 16 | Producer's inline **"Planner Report"** self-disclosure (Summary, What Was Produced, Key Outputs, Vertical-Slice Rationale, TDD Structure, Constitution Alignment, Open Questions — L109–152) | **`flag-for-reconcile`** — producer-report home: inline persona self-disclosure vs a `taskarchitect-report-template` (contract §4 / context.md gated item). Non-verdict descriptive sections are legitimate self-disclosure regardless of home. |
| 17 | Report's **self-asserted verdict** ("Completion: complete/partial" L122; "Ready for Review …Devil's Advocate" L149–151) | `dropped + reason` (independence: lead owns the clearing verdict; mirrors the plan port's `completion_status` drop; also carries the A1 sibling name). **Human-gate accept.** |
| 18 | Report path injection (L111) | `moved-to-lead` (caller-side dispatch context) |
| 19 | Brownfield **task markers** `[NEW]`/`[EXTEND]`/`[MODIFY]`/`[GAP:XXX]` + plan-marker→task-marker translation (L191–213) | `kept` (keystone-clean craft) + light `dedupe` of the marker table vs P3 |
| 20 | Brownfield **codebase-analysis read** (`.humaninloop/memory/codebase-analysis.md`, L188) | `kept-but-rebind` (`.mochiko/`; real mochiko artifact from setup's `analysis-codebase`) |
| 21 | Brownfield **evolution-roadmap read** (`.humaninloop/memory/evolution-roadmap.md`, L189; the `[GAP:XXX]` source) | **`deferred-by-reference`** (roadmap track deferred core-only; documented stub, path noted, **not a live mount** — the `[GAP:XXX]` *marker craft* stays via #19) |
| 22 | PRODUCER **team-role** (authors both artifacts; reviewed by `devils-advocate`+`validation-task-artifacts` single-reviewer loop) | `kept` (producer role; **validator partner settled**; independence intact — no grading skill mounted) |
| 23 | Classification + router/discoverability + `<example>` blocks (description L2–3 is bare — **no examples**, unlike `technical-analyst`) | `kept-but-rebind` (convention-wiring pass — **heavier**: author `<example>` blocks + router/`plugin.json` registration) |
| 24 | Agent frontmatter (`model: opus` L4, `color: green` L5) | `kept` |

**Orchestration note (no loop-driving tag originates here).** Being a dispatched node, fed
inputs/paths, told its phase, and looped on the advocate verdict — plus the Mapping→Tasks sequencing,
the plan-complete entry-gate, the cumulative `tasks.md`→`task-mapping.md` cross-review, and
clarification/exit-early routing — are **HIL `tasks` supervisor / `state-analyst`** responsibilities
that re-home onto the **`tasks` command supervisor (P1)** (contract §1 names them as must-rehome). This
agent contributes `moved-to-lead` only for the *sequencing/inputs/report-path* it currently injects
(#6, #8, #18); it never owned the loop.

**No responsibility is dropped silently.** Every drop (#5, #7, #15, #17) carries a reason for the human
gate to accept. Every responsibility carries a tag → assessment done-condition met.

---

## Reconcile flags (for `reconcile-cluster`)

1. **Producer-report home (GATED; contract §4).** The inline "Planner Report" (L109–152) → keep as
   **inline persona self-disclosure** (like `technical-analyst`, whose report lives in a lightweight
   template, non-verdict) **vs** extract a **`taskarchitect-report-template`**. **Hard constraint
   regardless of home:** the self-asserted verdict fields (`Completion`, `Ready for Review …Devil's
   Advocate`) are **dropped** — the lead owns the clearing verdict (independence; mirrors the plan
   port's `completion_status` drop). Leaning: inline self-disclosure (matches the `specify`/single-
   reviewer shape and the plan precedent of a light producer report), but **do not guess** — human-gated.
2. **RQ-A — artifact shape (GATED; the cluster headline, contract §4a).** One `tasks.md` (mapping
   collapses to a section) **vs** two artifacts/phases (`task-mapping.md` → `tasks.md`). The
   task-architect's **phase-behaviors (B3) and responsibilities #3/#4 ride on this**: two artifacts →
   the decoupled persona lists **two** artifact-types in its menu; one artifact → **one**. The
   disposition (`port-with-edits × standalone`) is stable either way; only the menu content changes.
   **Do not guess** — human-gated at reconcile.
3. **`dedupe` / `folded-into-skill` → P3 (trace #3,4,10,11,12,13,14,19).** Confirm canonical home =
   `patterns-vertical-tdd`: the persona keeps *taste*, defers the literal Cycle Structure / TEST:
   grammar / slice heuristics / marker table / quality checklist to P3. Executable in the
   `transform-recipes` decouple/dedupe step; coordinate with P3's assessment so the fold has a landing
   place (no orphan procedure).
4. **`moved-to-other-cluster` → implement/qa (trace #15).** The qa-engineer runtime-classification
   (auto-approve vs human-checkpoint) belongs to the deferred implement/qa cluster; confirm it is
   **parked, not dropped** (the 4 sibling-name *tokens* are dropped; the *capability* re-homes).
5. **Decouple/excise edits (A/B/C/E classes; NOT structural — noted for completeness).** The
   sibling-name, phase-mode, context-file, and self-verdict excisions are `port-with-edits` wiring
   actions. `verify-output`'s decoupling scan + keystone test must confirm **zero residual** deny-list
   token on the port (this is the run's highest-risk surface, contract §1).

**Cluster-wiring (NOT this primitive's move; noted so nothing is dropped):** re-mount
`validation-task-artifacts` on `devils-advocate` (stub already present); register `task-architect` in
the `mochiko` router + `plugin.json`; P1 owns the supervisor→agent-team rewiring, the Mapping→Tasks
sequencing, the plan-complete entry-gate, and the cumulative-review requirement.

**Independence — CONFIRM, NO ACTION.** `skills:` `{patterns-vertical-tdd}` ∩ grading skills = ∅. No
self-grade leak. **No `split`, no new persona** for this primitive — only the heavy `port-with-edits`
plus the two gated reconcile decisions around it.

**Decoupling-scan result for the run goal:** **the coupled case the run was built to test.** Unlike
`technical-analyst` (worst categories clean), this persona is **coupled on the worst categories**: **5
sibling-agent-name hits** (Devil's Advocate ×1, qa-engineer ×4), a **phase-mode skeleton** (B1–B5), a
**context-file section** (C1–C7), and a **self-asserted verdict** (E). All are cut/reframed by the
heavy `port-with-edits`; the intrinsic craft (vertical slicing, TDD, foundation+parallel, traceability,
brownfield markers, verification-task concept) survives keystone-clean. Empirical read: the doctrine's
worst failure mode (sibling-naming) **does occur here** — this is the primitive that proves the
decoupling pass earns its keep.

---

## Output block

```
ASSESSMENT: task-architect
Class:        agent → branch PLAYS-a-role  (team-role conferred: PRODUCER; net-new to mochiko)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (heaviest port of the run — whole-section excisions)
              + flag-for-reconcile: (a) producer-report home  (b) RQ-A artifact shape
Trace:
  - task-architect persona & judgment (Core Identity)                        → kept (keystone-clean craft)
  - vertical-slice+TDD procedure (patterns-vertical-tdd P3)                   → kept-but-rebind (mochiko:; stays mounted, P3 in-cluster)
  - author story→cycle mapping (artifact-type)                               → kept (+ HOW dedupe→P3; distinctness rides on RQ-A)
  - author cycle-based tasks.md (artifact-type)                              → kept (+ HOW dedupe→P3)
  - phase-mode framing (How You Operate / Phase Behaviors as injected modes) → dropped + reason (injected-workflow-mode coupling)
  - Mapping→Tasks sequencing                                                 → moved-to-lead (P1 sequences)
  - context-file mechanism (Reading the Context; supervisor_instructions)    → dropped + reason (markdown-supervisor carrier)
  - injected input list (spec/plan/constraints/data-model/contracts)         → moved-to-lead (dispatch brief / agent-dispatch)
  - constitution/governance alignment                                        → kept (craft; principles-injection → moved-to-lead)
  - Success-Criteria / Quality-Standards bars                                → kept (taste) + dedupe (literal → P3; review-mirror = P4)
  - What You Reject / What You Embrace                                       → kept (taste) + light dedupe→P3
  - Cycle Structure template (### Cycle N, TN.1→TN.4)                        → folded-into-skill→P3
  - verification-task concept (gates cycle; "makes vertical TDD vertical")   → kept (taste) + dedupe→P3
  - TEST: verification grammar (format/modifiers/asserts/legacy)             → folded-into-skill→P3
  - qa-engineer runtime-classification handoff (auto-approve vs checkpoint)  → moved-to-other-cluster (implement/qa) + 4 sibling-name tokens dropped+reason
  - inline "Planner Report" self-disclosure (L109–152)                       → flag-for-reconcile (report home)
  - report self-asserted verdict (Completion; Ready for Review…Devil's Adv)  → dropped + reason (independence: lead owns verdict)
  - report path injection                                                    → moved-to-lead
  - brownfield task markers [NEW]/[EXTEND]/[MODIFY]/[GAP:XXX]                 → kept (craft) + light dedupe→P3
  - codebase-analysis.md read (.humaninloop→.mochiko)                        → kept-but-rebind
  - evolution-roadmap.md read (the [GAP:XXX] source)                         → deferred-by-reference (roadmap track deferred; stub, not a live mount)
  - PRODUCER team-role (reviewed by devils-advocate+validation-task-artifacts)→ kept (validator settled; independence intact)
  - classification + router + <example> blocks (description bare, no examples)→ kept-but-rebind (wiring pass; HEAVIER — author examples)
  - frontmatter (model: opus, color: green)                                  → kept
Reconcile flags:
  - (a) producer-report home: inline self-disclosure vs taskarchitect-report-template — GATED; self-verdict fields dropped regardless
  - (b) RQ-A artifact shape (one tasks.md vs two mapping+tasks): phase-behaviors ride on it — GATED; disposition stable either way
  - dedupe/fold → P3 (Cycle Structure, TEST: grammar, slice heuristics, marker table, quality checklist); coordinate with P3 assess
  - moved-to-other-cluster → implement/qa (qa-engineer runtime-classification): parked, not dropped
  - decouple/excise edits (A sibling-names ×5 / B phase-mode / C context-file / E self-verdict): port-with-edits wiring, not structural
Independence: CONFIRM-NO-ACTION (skills {patterns-vertical-tdd} ∩ grading skills = ∅; no self-grade leak)
Decoupling scan: COUPLED on worst categories — 5 sibling-agent names (Devil's Advocate L151; qa-engineer L248/274/279/337),
                 phase-mode skeleton (L30/35/38-107/123/345), context-file section (L29/111/342-351), self-verdict (L122/149-151),
                 hardcoded paths (L188 rebind, L189 deferred-by-reference). Craft survives keystone-clean.
```
