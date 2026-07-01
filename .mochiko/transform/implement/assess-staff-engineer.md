# Assessment — `agents/staff-engineer.md` (P2)

Run: `implement` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source: `human-in-loop/plugins/humaninloop/agents/staff-engineer.md` (95 ln)
Class: **agent → PLAYS-a-role branch**
Cluster role: **PRODUCER** of the implement workflow (writes code via TDD red/green/refactor; updates
`tasks.md`; writes `cycle-report.md`). Its independent counterpart is the **`qa-engineer` (P3)** validator
driven by `testing-end-user` (P5) — a **single-reviewer, Tier-1 deterministic** loop (verifies against
real infrastructure + quality-gate exit codes; contract §2). The thin sequential lead (P1 command
supervisor) is lead/referee. **Net-new to mochiko** (no existing mochiko `staff-engineer` — fresh port,
`standalone`). Declared `skills:` = `executing-tdd-cycle` (P4) + `brownfield-integration` (P6), **both
ported this run**.

> ROLE NOTE: assess/diagnose ONLY. No transform, no grading. The fix-pass **routing/bounds** are
> cross-primitive (shared with P1 + the dissolving P7) → emitted as `flag-for-reconcile`, not resolved
> here. The producer↔validator pairing with P3/P5 is settled cluster wiring (the mount lives on P3),
> noted not fired.

**Headline — the fix-mode call is the crux.** This persona is **moderately coupled, and cleaner than
`task-architect` on every worst axis**: grep confirms **ZERO sibling-agent names**, **ZERO self-asserted
clearing verdict**, **ZERO kernel/DAG/catalog/MCP tokens**, **ZERO `.humaninloop/` artifact paths**,
**ZERO "workflow-agnostic" meta-label**. Its coupling is concentrated in exactly two surfaces: (B) the
**"Two Execution Modes / Cycle Mode / Fix Mode"** skeleton (L73–81) and (C) the **context-file +
"Supervisor dispatches"** framing in all three `<example>` blocks (L8/9, 16/18, 25/27) plus the L64
output-location line. The decisive judgment is **separating the two threads inside the modes section**:
the *disposition* to scope a fix tightly to the reported failures and reproduce a bug with a failing test
before fixing it is **intrinsic craft (keep)**; the *mode-labels, the trigger ("After Validation
Failure"), the "unconstrained by cycle boundaries" vocabulary, and "read your instructions from ...
context.md"* are **this-workflow machinery (cut → rehome to the lead)**. Body treatment is a **moderate
`port-with-edits`** — decouple the two surfaces, keep the craft — well short of `redesign` (the body
assumes no kernel; the craft is preserved, not re-conceived).

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting: persona-vs-procedure split, the **team-role it confers** (here:
PRODUCER), its **`skills:`-list independence** (does it write AND grade?), and **persona decoupling** (no
sibling-agent names / "dispatch" / injected workflow modes-paths-phases / "workflow-agnostic" labels).
Loop-ownership is NOT this primitive's concern — it sits on the orchestrator (HIL: the `implement`
DAG-Supervisor + State-Analyst + `strategy-implementation`; mochiko: the thin `implement` command
supervisor, P1).

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** All three `<example>` blocks have a **Supervisor "dispatch"** the
   engineer and instruct it to **"Read your instructions from: specs/001-feature/.workflow/context.md"**
   (L8–9, 16–18, 25–27). The persona is organized around being **dispatched in a mode** (Cycle vs Fix,
   L73–81) and reads its output locations "from your instructions (context.md)" (L64). It only *functions*
   because a supervisor sequences cycles, feeds it a task list / failure report, and loops execute→verify.
   **Orchestration-coupling** (the supervisor drives it) **+ content-coupling** (the persona bakes the
   mode-framing + context-file contract into its body).
2. **Multi-responsibility / fans out?** **YES.** Writes code under TDD, updates `tasks.md`, produces
   `cycle-report.md`, and handles brownfield EXTEND/MODIFY. Feeds the **validator** (P3/P5), the **lead**
   (execute→verify pairing + targeted retry + fix pass). (Most of the *procedure* lives in the two mounted
   skills — the persona is thinner than `task-architect` — but it still holds >1 responsibility and >1
   consumer.)
3. **Non-machine-checkable artifact?** **Mixed → YES on the judgment axis.** The *code* it emits is
   deterministically checkable (tests pass/fail, quality-gate exit codes) — which is exactly why P3 is a
   **Tier-1** validator. But whether TDD was genuinely followed (a real red phase), whether scope was
   held, and whether the cycle report is honest are **model judgment**. ⇒ a **real grounded validator
   partner is required and is settled**: `qa-engineer` (P3) + `testing-end-user` (P5), a separate agent
   (contract §2).

All three **yes → full 7-check lens.** Gates tripped: 1, 2, 3.

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer (HIL):** a **DAG + markdown supervisor** — `commands/implement.md` (with the
  DAG-Supervisor, `state-analyst`/`hil-dag`, and `strategy-implementation`) writes the `.workflow/context.md`
  carrier, sequences foundation→feature cycles, dispatches this producer, then loops execute→verify /
  targeted-retry / fix-pass. This agent is a **dispatched producer node**.
- **Content-coupling: PRESENT, on two surfaces only.** The body bakes (i) the **mode skeleton** ("## Two
  Execution Modes" → "### Cycle Mode / ### Fix Mode (After Validation Failure)", L73–81), and (ii) the
  **context-file dispatch contract** ("Read your instructions from: ... context.md" ×3 in the examples,
  L9/18/27; "Write outputs to the locations specified in your instructions (context.md)" L64). **No
  Python/MCP/DAG/catalog/`hil-dag`/`node` token anywhere** (grep D → none) — the coupling is to a
  **markdown supervisor + mode-framing**, not a kernel. That keeps it firmly out of `redesign` territory.
- **Orchestration-coupling:** dispatched producer node — relies on the supervisor for invocation, for
  which-mode / which-inputs / where-to-write injection, and for the execute→verify loop, targeted-retry
  (max 3/cycle), fix-pass (max 3), and convergence-stall detection. Those **re-home onto the thin
  `implement` command supervisor (P1)** (contract §1 constraints (a)–(g) name them as must-rehome-not-drop).
  This agent owns **no loop-driving** of its own; the mode-routing it *implies* is `moved-to-lead` /
  `flag-for-reconcile`.

### 2. Role at two altitudes
- **Skill-role:** **artifact-emitter** — wraps `executing-tdd-cycle` (P4) + `brownfield-integration` (P6)
  and emits reviewable artifacts (implemented code, updated `tasks.md`, `cycle-report.md`). → needs an
  independent producer↔validator partner.
- **Team-role conferred:** **PRODUCER — single and unconflicted.** Both mounted skills are
  implementation/authoring (producer) skills. **No grading/validation skill on this agent.** A mochiko
  agent should confer exactly one team-role; this one does (producer).
- **Partner is SETTLED (like `task-architect`, unlike plan's RQ1):** the reviewer is the **`qa-engineer`
  (P3)** driven by **`testing-end-user` (P5)** — a single-reviewer, **Tier-1 deterministic** loop
  (contract §2). The mount of `testing-end-user` lives on **P3**, not on this agent — cluster wiring, not
  a move on this primitive.

### 3. Independence — CONFIRMED CLEAN (no self-grade leak)
- **Self-grade leak: NO.** `skills:` (L35) = `executing-tdd-cycle, brownfield-integration` — both
  implementation (producer) skills; **neither grades.** No `testing-end-user`, no `verify-*`. The agent
  writes code; it never grades it.
- **Producer skills ∩ grading skills = ∅.** Independence is **structural and intact on the producer
  side** — the textbook-correct shape (same as `task-architect`). **No split needed; no
  `flag-for-reconcile` for an independence *leak*.**
- **No independence hazard in the output, either (cleaner than `task-architect`):** the `cycle-report.md`
  is **honest self-disclosure** of what happened, incl. difficulties/deviations (L71, L95) — it does
  **not** self-assert a clearing verdict ("ready for QA" / "complete/partial" / PASS). Grep E returns only
  false positives ("implementing to pass", "fix the specific issues"). The clearing verdict belongs to
  P3. **Nothing to drop here** — unlike `task-architect`'s "Ready for Review / Completion" fields.
- **HARD INDEPENDENCE RULE (carry into transform + reconcile):** never mount `testing-end-user` or any
  `verify-*`/grading skill on `staff-engineer`. Produce + grade on one agent is wrong by construction.

### 4. Verdict-sink / loop-driver
- **Consumers of output:** the code + `tasks.md` + `cycle-report.md` are consumed by the **validator**
  (`qa-engineer`+`testing-end-user` — verifies each cycle against real infrastructure + gates), and by the
  **lead** (execute→verify pairing, targeted-retry decision, fix-pass decision, convergence-stall).
- **What loops on FAIL today:** the HIL `implement` DAG-Supervisor + State-Analyst + `strategy-implementation`
  drive execute→verify, targeted-retry (max 3/cycle), fix-pass (max 3), and convergence-stall. **None of
  that lives in this agent.**
- **Where it re-homes:** onto the **thin `implement` command supervisor (P1)** as a bounded, **sequential**
  produce→verify loop with the confidence-based + final-acceptance gate (intake decisions 1 & 2). Originates
  from P1 / the dissolving supervisor + P7; listed here for the rehome map's completeness.

### 5. Sibling / overlap ("look sideways")
- **Agent siblings:** `qa-engineer` (P3) is the **complementary role (validator), not a variant** — no
  merge; it is the pairing partner. **This persona does not name it** (grep A → none), so there is **no
  sibling-name to scrub** (unlike `task-architect`, which named qa-engineer ×4). The pairing is settled
  cluster wiring.
- **Skill overlap (the real signal):** the persona lightly restates procedure P4/P6 own — the
  red/green/refactor sequence, task parsing, retry/fix procedure (→ **P4** `executing-tdd-cycle`), and
  read-before-write / EXTEND-MODIFY semantics (→ **P6** `brownfield-integration`). → a
  `dedupe`/persona-keeps-taste signal: canonical home is P4/P6; the persona keeps *taste/disposition*,
  defers *literal procedure*. Coordinate with P4/P6 assessments so the fold has a landing place.
- **Cross-primitive fix-mode routing (→ reconcile):** the fix-pass **routing/trigger/bounds** overlap
  **P1 (command) + P7 (`strategy-implementation`, dissolving)** — genuinely cross-primitive →
  `flag-for-reconcile` (see Step 4 + flags).
- **Boundary note (P4↔P3):** the design-time *structuring* (`patterns-vertical-tdd`, tasks cluster) vs
  runtime *execution* (`executing-tdd-cycle`, P4) split is a **P4** concern (context §45); noted, not this
  agent's move.

### 6. Coupling audit
- **Hardcoded paths:** `.workflow/context.md` (L9/18/27/64) → **`dropped`** (context-file carrier; mochiko
  has **no** `context.md` — it is absorbed into the lead + workspace-as-state under `.mochiko/specs/<feature>/`,
  context §34; **not** a `.mochiko/` rebind). `specs/001-feature/...` (L9/18/27) → example path, dropped
  with the example reframe. **No `.humaninloop/memory/...` literal artifact path** in this body (this agent
  does not read `codebase-analysis.md` — that is `task-architect`).
- **`skills:` references:** 2 (`executing-tdd-cycle` P4, `brownfield-integration` P6) — **both in-scope,
  ported this run** → `kept-but-rebind` (frontmatter L35 stays bare per the ported-agent convention; body
  refs L44/L45 get the `mochiko:` prefix). The P6 mount **resolves the setup-run REGISTRY mis-file** — its
  home is `implement` (context §27). **Do NOT mount unported skills** — both are in-cluster, so the mounts
  are valid once P4/P6 land.
- **Upstream prerequisites:** assumes the lead handed it a cycle task list (`tasks.md`, present/complete —
  the **entry gate**, constraint (f)) and, in fix scope, a validation report of failures. **Handoff edges
  owned by P1's contract**, not the agent.
- **Determinism boundary:** the agent's **output** is validated deterministically by P3 (Tier-1: tests +
  gate exit codes + captured evidence); the agent's **craft** (genuine red phase, scope discipline, honest
  reporting) is model judgment. ⇒ the validator partner is a **real Tier-1 grounded validator**, not a
  degenerate assert. The pairing is real and load-bearing.

### 7. Conventions + loop placement — DECOUPLING SCAN (the crux)

**Persona-vs-procedure split.** Persona **keep** (taste/judgment): Core Identity (L50–56 war stories —
genuine red phase, simplest impl, read-before-write, no scope creep, flag-honestly), Quality Standards
(L67–71 as *taste*), What You Reject/Embrace (L83–95), the **output menu** (implemented code, updated task
checkboxes, honest report — output-shape is allowed, like `task-architect`'s menu), and — critically — the
**fix disposition** (scope tightly to reported failures; reproduce a bug with a failing test before
fixing; a fix is not a refactoring opportunity). Procedure **→ P4/P6** (already factored): red/green/refactor
steps, task parsing, retry/fix procedure, EXTEND/MODIFY semantics, read-before-write.

**Decoupling scan — DENY-LIST HITS BY CLASS (grep-anchored). Worst axes are CLEAN; two surfaces are coupled.**

**A. Sibling-agent names — NONE (grep A clean).** Notable: unlike `task-architect`, this producer names
no sibling. The "Supervisor" tokens (L8/16/25) are loop-position words (class C), not sibling-agent names.

**B. Injected workflow modes / phases (the organizing skeleton — the crux):**
| # | Line | Token | Fix |
|---|------|-------|-----|
| B1 | L73 | "## Two Execution Modes" (the skeleton header) | Cut the two-modes framing. A decoupled producer does not self-select a mode; it is briefed on a task. |
| B2 | L75 | "### Cycle Mode (Normal)" | Cut the mode-label. The *scope-discipline disposition* ("stay tightly scoped to the work you're given") survives as craft. |
| B3 | L77 | "Bound to the scope defined **in your instructions**... Stay within **the cycle's scope**." | Keep the intrinsic "stay tightly scoped to what you're asked"; drop "in your instructions" (injection) + "the cycle's scope" (workflow unit). Sequential-execution procedure `dedupe`→P4. |
| B4 | L79 | "### Fix Mode **(After Validation Failure)**" | Cut the mode-label **and** the trigger. **The persona must not name the trigger** — the lead decides *when* to brief fix-scoped vs cycle-scoped. |
| B5 | L81 | "**Unconstrained by cycle boundaries**... **may touch files from any cycle**... write a failing test for the bug, then fix it." | **SPLIT.** Keep the craft ("scope to the specific failures; reproduce with a failing test before fixing; follow the failure wherever it leads, not an excuse to refactor"). Drop the **cycle-boundary vocabulary** + "identified in your instructions". The **routing/max-passes bound** → lead (`flag-for-reconcile`). |
| B6 | L44 | skill-blurb: "...retry handling, **fix mode**, and cycle report generation" | Soften the mode-naming in the asset description (describe the skill's *work*, not its workflow-mode). Light. |

**C. "Reads instructions from a context file" / "Supervisor dispatches" / caller-injection:**
| # | Line | Token | Fix |
|---|------|-------|-----|
| C1 | L8, L16, L25 | "**Supervisor dispatches** Staff Engineer for/in ..." (example Context lines) | Cut. Deny-list "dispatch" + supervisor loop-position + names the loop driver. Reframe the examples to **work-context** (like the landed `task-architect`). |
| C2 | L9, L18, L27 | "Read your instructions from: **specs/001-feature/.workflow/context.md**" (×3) | Cut. mochiko briefs via in-session dispatch (`agent-dispatch`) + workspace-as-state, not a context file. |
| C3 | L17, L26 | example mode/trigger: "in **fix mode** after **final-validation failure**"; "for **retry after checkpoint failure**" | Cut in the example reframe (workflow-mode + trigger vocabulary). |
| C4 | L64 | "Write outputs to the **locations specified in your instructions (context.md)**." | Cut the context-file ref; output location is caller-side dispatch context → `moved-to-lead`. |

**D. Hardcoded paths:** `.workflow/context.md` (L9/18/27/64) → `dropped` (no mochiko `context.md`; absorbed
into lead). Skill-ref prefixes `humaninloop:` (L44/L45) → `kept-but-rebind` (`mochiko:`). No `.humaninloop/`
artifact path.

**E. Self-asserted verdict — NONE (grep E clean of real hits).** The `cycle-report` is honest
self-disclosure, not a clearing verdict. **Nothing to drop** (cleaner than `task-architect`).

**FALSE POSITIVES / keystone-survivors (KEEP — do not over-scrub):**
- "TDD", "red/green/refactor", "write genuinely failing tests first", "verify they fail for the right
  reason", "the simplest implementation that passes", "don't over-engineer" — **intrinsic craft** (true of
  any TDD engineer).
- "read the full file first", "follow existing patterns", "EXTEND/MODIFY", "[MODIFY] marker",
  "preserve interfaces" — **brownfield craft** (keystone-clean; brownfield-the-concept legitimately
  survived in mochiko `task-architect`/`technical-analyst`).
- "implement exactly what the task describes, nothing more", "note opportunities, don't act on them",
  "stay within scope" — **scope-discipline craft** (this is the intrinsic thread of the modes section).
- **"write a failing test for the bug, then fix it"** (L81) + **"scoped to specific failures"** (L81) —
  **reproduce-then-fix + scope-tightly craft. KEEP.** This is the disposition the fix-mode section is
  *about*; only the mode-label/trigger/cycle-boundary machinery around it is coupling.
- "flag blockers honestly", "silent workarounds", "honest reporting", cycle reports "reflect what actually
  happened, including difficulties and deviations" — **honesty craft**.
- The **output menu** (implemented code via TDD; updated task checkboxes; an honest implementation report)
  — allowed output-shape; the literal `cycle-report.md`/`tasks.md` filenames are artifact-type names (light
  reword at most, as `task-architect` kept `tasks.md`).

**Convention wiring (present/missing):**
- **Classification:** model-invoked producer agent (set in wiring pass; agents aren't user-invoked router
  entries).
- **Discoverability / description:** the HIL description ships **three `<example>` blocks**, but all three
  are **coupled** (Supervisor-dispatch + context.md + mode-trigger). → **rewrite all three to work-context**
  (a cycle task list ready to implement via TDD; a set of reported failures to fix by reproducing-then-fixing;
  a brownfield EXTEND/MODIFY task) and register in the `mochiko` router + `plugin.json` (cluster wiring).
- **Reliable model-invocation:** N/A at agent frontmatter (skill-level concern; P4/P6 own their triggers).
- **Producer↔validator pairing:** **REQUIRED and SETTLED** — `qa-engineer` (P3) + `testing-end-user` (P5),
  guaranteed structurally (separate reviewer agent), never in this persona. Mount lives on P3.
- **Sound-loop:** done-condition / execute→verify gate / targeted-retry / fix-pass / convergence-stall /
  confidence-based+final human gate all land on **P1's supervisor loop** (rehome-orchestration), not on
  this agent.

---

## Step 4 — Disposition

- **Body treatment: `port-with-edits`** — **moderate weight** (two decouple surfaces + rebinds; lighter
  than `task-architect`'s heaviest-of-run port). The craft core (Core Identity, Quality Standards as taste,
  Reject/Embrace, the output menu, Skills-as-assets, the scope/reproduce disposition) is **kept/lightly
  edited**; the coupling is **excised on two surfaces**: the "Two Execution Modes" skeleton (B1–B5) and the
  "Supervisor dispatches / context.md" example framing + L64 (C1–C4). Rebinds: 2 skill body-refs →
  `mochiko:`.
  - **Why not `keep-verbatim`:** the two decouple surfaces + rebinds are mandatory.
  - **Why not `redesign`:** the body assumes **no kernel/DAG/catalog** (grep D clean); the coupling is to a
    markdown supervisor + mode-framing, and the craft is **preserved, not re-conceived**. Carry the
    responsibilities, strip the mechanism — the definition of `port-with-edits`. The minimalism governor
    forbids redesigning what decouple-plus-edit fixes.
- **Structural move: `standalone`** — decidable solo: no self-grade leak → **no `split`**; complementary
  validator, not a variant → **no `merge`**; not a check → **no `promote`**; real reusable producer body →
  **not `absorb-into-lead`**. Lands `standalone` in `agents/` as a **net-new** mochiko agent. The
  supervisor→agent-team rewiring of the *cluster* is **P1's** move, not this agent's.
- **Disposition: `port-with-edits × standalone`** — **WITH one `flag-for-reconcile`** (human-gated per
  contract §4): the **fix-pass routing/bounds** (cross-primitive with P1 + P7). The disposition itself is
  stable regardless of how reconcile lands the routing; only *where the routing lives* is deferred.

---

## Step 5 — Responsibility trace (complete; no silent drops)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Staff-engineer persona & judgment — genuine-red-phase, simplest-impl, read-before-write, no-scope-creep, flag-honestly (Core Identity L50–56) | `kept` (keystone-clean intrinsic craft) |
| 2 | TDD red/green/refactor execution **procedure** via `executing-tdd-cycle` (P4) | `kept-but-rebind` (frontmatter L35 bare; body L44 → `mochiko:`; **stays mounted** — P4 in-cluster) |
| 3 | Brownfield EXTEND/MODIFY **procedure** via `brownfield-integration` (P6) | `kept-but-rebind` (frontmatter L35 bare; body L45 → `mochiko:`; **stays mounted** — P6 in-cluster; **resolves the setup-run REGISTRY mis-file**, home = implement) |
| 4 | Output menu — implemented code (TDD); updated `tasks.md` checkboxes; honest `cycle-report.md` (What You Produce L58–63) | `kept` (producer output-shape; filenames are artifact-type names, light reword at most) |
| 5 | Output-**location** instruction "locations specified in your instructions (context.md)" (L64) | `dropped + reason` (context-file carrier; mochiko output location is caller-side + workspace-as-state) — location itself `moved-to-lead`. **Human-gate accept.** |
| 6 | Quality Standards — TDD rigor, scope discipline, brownfield respect, honest reporting (L67–71) | `kept` (producer **taste**) + light `dedupe` (literal "every task red/green/refactor" step → P4) |
| 7 | **Cycle-Mode framing** ("### Cycle Mode (Normal): bound to the scope in your instructions; execute sequentially through TDD; stay within the cycle's scope" L75–77) | *scope-discipline disposition* `kept` (craft: stay tightly scoped to the work given); **mode-label + "the cycle's scope" + "in your instructions" injection** `dropped + reason` (injected-workflow-mode coupling); sequential-execution procedure `dedupe`→P4. **Human-gate accept.** |
| 8 | **Fix-Mode framing** ("### Fix Mode (After Validation Failure): unconstrained by cycle boundaries; scoped to specific failures in your instructions; may touch files from any cycle; still write a failing test for the bug, then fix it" L79–81) | **SPLIT** — (i) *intrinsic craft* (scope tightly to reported failures; reproduce a bug with a failing test before fixing; follow the failure wherever it leads, not a refactor opportunity) → `kept` (keystone-clean); (ii) *fix-pass **routing** / mode-label / trigger "After Validation Failure" / cycle-boundary vocabulary / max-passes bound* → **`flag-for-reconcile`** (re-homes to the thin lead; cross-primitive with P1 + P7); (iii) *fix/retry **procedure*** → `dedupe`→P4. **Human-gate accept** for the dropped mode-labels. |
| 9 | **Retry disposition** (targeted rework of only the failed tasks; don't re-implement passing code — examples L28–31) | *disposition* `kept` (craft: minimal targeted rework) + *procedure* `dedupe`→P4 + *routing/trigger ("after checkpoint failure", max-3/cycle)* `moved-to-lead` (P1 constraint (c); folds into the fix-pass reconcile flag) |
| 10 | **Context-file mechanism** in the 3 `<example>` blocks ("Read your instructions from: specs/001-feature/.workflow/context.md" L9/18/27) | `dropped + reason` (markdown-supervisor `context.md` carrier; mochiko uses in-session dispatch briefs + workspace-as-state, context §34). **Human-gate accept.** |
| 11 | **"Supervisor dispatches" + mode/trigger naming** in the 3 examples (L8/16/25 "Supervisor dispatches …"; "normal cycle" / "fix mode … final-validation failure" / "retry … checkpoint failure" L17/26) | `dropped + reason` (deny-list: "dispatch" + supervisor loop-position + workflow-mode/trigger names) — examples **reframed to work-context** in the wiring pass. **Human-gate accept.** |
| 12 | Skills-Available descriptions (2 skills as assets, L40–47) | `kept-but-rebind` (`humaninloop:`→`mochiko:`; soften the "retry handling, fix mode" mode-naming in the P4 blurb, L44) |
| 13 | What You Reject (no unasked code; no skipped failing test; no interface-mod without `[MODIFY]`; no silent workarounds — L83–88) | `kept` (keystone-clean craft; `[MODIFY]` marker is brownfield craft) |
| 14 | What You Embrace (follow existing patterns; test behavior-not-implementation; flag discrepancies; honest cycle reports — L90–95) | `kept` (keystone-clean craft) |
| 15 | **Honest cycle-report self-disclosure** (reflects what actually happened, incl. difficulties/deviations — L71, L95) | `kept` (honesty craft; **NOTE: self-disclosure, NOT a self-asserted clearing verdict** — no independence leak, unlike `task-architect`; the verdict is P3's) |
| 16 | PRODUCER **team-role** (writes code; verified by the independent `qa-engineer` P3 + `testing-end-user` P5 — Tier-1 deterministic; contract §2) | `kept` (producer role; **validator partner settled**; independence intact — no grading skill mounted; **must not** mount `testing-end-user`/`verify-*`) |
| 17 | Classification + router/discoverability + `<example>` reframe (3 examples all coupled) | `kept-but-rebind` (convention-wiring pass: model-invoked; router + `plugin.json` registration; **rewrite all 3 examples to work-context**) |
| 18 | Agent frontmatter (`model: opus` L33, `color: green` L34) | `kept` |

**Orchestration note (no loop-driving tag originates here).** Being a dispatched node, fed
instructions/paths, told its mode, and looped on the qa verdict — plus the **sequential cycle sequencing**
(foundation-before-feature), the **execute→verify pairing**, **targeted-retry** (max-3/cycle), the
**fix-pass** (max-3), **convergence-stall detection**, the **entry gate** (`tasks.md` complete), and
**project scaffolding** — are HIL DAG-Supervisor + State-Analyst + `strategy-implementation` (P7)
responsibilities that re-home onto the **thin `implement` command supervisor (P1)** (contract §1
constraints (a)–(g) name them as must-rehome-not-drop). This agent contributes `moved-to-lead` /
`flag-for-reconcile` only for the *mode-routing / output-location / injected-instructions* it currently
references (#5, #8, #9, #10, #11); **it never owned the loop.**

**No responsibility is dropped silently.** Every drop (#5, #7-label, #8-label, #10, #11) carries a reason
for the human gate to accept. Every responsibility carries a tag → assessment done-condition met.

---

## Reconcile flags (for `reconcile-cluster`)

1. **Fix-pass routing / bounds (GATED; contract §4 — THE crux of this primitive).** The fix-mode
   **routing** — deciding *when* to brief the engineer fix-scoped vs cycle-scoped, the "unconstrained by
   cycle boundaries" decision, and the **max-3-fix-passes** bound (constraint (d)) — re-homes to the **thin
   lead**, and is **shared across P1 (command supervisor), P7 (`strategy-implementation`, dissolving), and
   this agent**. Its precise landing is cluster-scoped → **reconcile decides** (coordinate with P1's
   redesign + P7's dissolution; some of it is already generic loop-discipline, some is
   workflow-specific-to-the-lead). **HARD CONSTRAINT regardless of landing:** the intrinsic *craft* (scope
   tightly to reported failures; reproduce-with-a-failing-test; don't refactor opportunistically) **stays
   in the persona**; only the routing/trigger/bounds move. **The persona must NOT name the trigger** ("after
   final-validation failure") **or the mode.** Leaning: the *when/how-scoped* is the lead's brief; the
   *how-to-fix-well* is the engineer's craft (+ P4 procedure) — but **do not guess the exact P1/P7 split**;
   human-gated at reconcile.
2. **`dedupe` / persona-keeps-taste → P4/P6 (trace #2,3,6,7,8,9).** Confirm canonical home:
   `executing-tdd-cycle` (P4) owns the red/green/refactor + retry + fix *procedure*; `brownfield-integration`
   (P6) owns EXTEND/MODIFY + read-before-write. The persona keeps *taste/disposition*, defers *literal
   procedure*. Coordinate with P4/P6 assessments so the fold has a landing place (no orphan procedure).
   Boundary note: the P4↔P3 (`patterns-vertical-tdd`) design↔runtime split (context §45) is a P4 concern.
3. **Producer↔validator pairing — CONFIRM, SETTLED (no action on this primitive).** `qa-engineer` (P3) +
   `testing-end-user` (P5), separate agent, structural (contract §2). Mount lives on **P3**. **HARD
   INDEPENDENCE RULE:** never mount a grading/`verify-*` skill on `staff-engineer` — produce + grade on one
   agent is wrong by construction.
4. **Decouple/excise edits (B modes-skeleton / C context-file+"dispatch" + example-reframe; NOT structural
   — noted for completeness).** These are `port-with-edits` wiring actions. `verify-output`'s decoupling
   scan + keystone test must confirm **ZERO residual** deny-list token on the port — the two coupled
   surfaces (mode-framing + context-file examples) are the run's named highest-risk surface for this
   primitive (contract §1/§15).

**Cluster-wiring (NOT this primitive's move; noted so nothing is dropped):** register `staff-engineer` in
the `mochiko` router + `plugin.json`; **P1** owns the supervisor→agent-team rewiring, the **sequential**
cycle sequencing, the execute→verify pairing, targeted-retry, fix-pass, convergence-stall, the entry gate,
and project scaffolding (constraints (a)–(g)); **P3** mounts `testing-end-user`.

**Independence — CONFIRM, NO ACTION.** `skills:` `{executing-tdd-cycle, brownfield-integration}` ∩ grading
skills = ∅. No self-grade leak, no self-asserted verdict. **No `split`, no new persona** for this primitive
— only the moderate `port-with-edits` plus the one gated reconcile decision (fix-pass routing).

**Decoupling-scan result for the run goal:** **the *subtle-line* case the doctrine warns about — cleaner
than `task-architect` on every worst axis, coupled on exactly two surfaces.** Grep confirms **ZERO
sibling-agent names, ZERO self-asserted verdict, ZERO kernel/DAG/catalog tokens, ZERO `.humaninloop/`
paths, ZERO meta-labels.** The coupling is concentrated in (B) the **mode skeleton** (Two Execution Modes /
Cycle / Fix, L73–81) and (C) the **"Supervisor dispatches / read from context.md" examples** (L8–27) + the
L64 output-location line. The empirical lesson this primitive teaches: the fix-mode section is precisely
the **profession-trait vs loop-position** line — the *disposition* to scope-tightly-and-reproduce-with-a-test
is craft that survives; the *mode-label, the trigger, and the cycle-boundary vocabulary* are loop-position
machinery that must rehome to the lead. Separating those two threads cleanly is the whole assessment.

---

## Output block

```
ASSESSMENT: staff-engineer (P2)
Class:        agent → branch PLAYS-a-role  (team-role conferred: PRODUCER; net-new to mochiko)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (moderate — two decouple surfaces; lighter than task-architect)
              + flag-for-reconcile: fix-pass routing/bounds (cross-primitive with P1 command + P7 strategy)
Trace:
  - staff-engineer persona & judgment (Core Identity: genuine red / simplest-impl / read-before-write / no-creep / flag-honestly) → kept (keystone-clean craft)
  - TDD red/green/refactor procedure (executing-tdd-cycle P4)                 → kept-but-rebind (mochiko:; stays mounted, P4 in-cluster)
  - brownfield EXTEND/MODIFY procedure (brownfield-integration P6)            → kept-but-rebind (mochiko:; stays mounted; resolves setup-run mis-file → home=implement)
  - output menu (impl code / tasks.md checkboxes / honest cycle-report.md)    → kept (producer output-shape)
  - output-location "in your instructions (context.md)" (L64)                 → dropped + reason (context-file carrier) + location moved-to-lead
  - Quality Standards (TDD rigor / scope / brownfield / honest reporting)     → kept (taste) + light dedupe→P4
  - Cycle-Mode framing (L75-77)                                              → scope-disposition kept; mode-label + "cycle's scope" + injection dropped+reason; sequential proc dedupe→P4
  - Fix-Mode framing (L79-81)  [THE CRUX — SPLIT]                            → craft (scope-to-failures + reproduce-with-test) kept; routing/mode-label/trigger/cycle-boundary/max-passes flag-for-reconcile; proc dedupe→P4
  - Retry disposition (targeted rework; don't re-impl passing code)           → disposition kept + proc dedupe→P4 + routing/trigger moved-to-lead (P1 constraint c)
  - context-file mechanism in 3 examples (read from .workflow/context.md)     → dropped + reason (markdown-supervisor carrier)
  - "Supervisor dispatches" + mode/trigger naming in 3 examples              → dropped + reason (dispatch + supervisor loop-position + mode names); examples reframed to work-context
  - Skills-Available descriptions (2 skills as assets)                        → kept-but-rebind (humaninloop:→mochiko:; soften "fix mode" mode-naming L44)
  - What You Reject / What You Embrace                                       → kept (keystone-clean craft; [MODIFY] is brownfield craft)
  - honest cycle-report self-disclosure                                       → kept (honesty craft; NOT a self-asserted verdict — no independence leak)
  - PRODUCER team-role (verified by qa-engineer P3 + testing-end-user P5)     → kept (validator settled; independence intact — no grading skill mounted)
  - classification + router + <example> reframe (3 examples coupled)          → kept-but-rebind (wiring pass; rewrite all 3 examples to work-context)
  - frontmatter (model: opus, color: green)                                   → kept
Reconcile flags:
  - fix-pass routing/bounds: rehome to thin lead; cross-primitive P1 + P7 — GATED. Craft (scope-tightly + reproduce-with-test) stays; persona must NOT name the trigger/mode. Do not guess P1/P7 split.
  - dedupe/persona-keeps-taste → P4/P6 (red-green-refactor, retry/fix proc, EXTEND/MODIFY): coordinate so the fold has a landing place
  - producer↔validator pairing CONFIRM-SETTLED (qa-engineer P3 + testing-end-user P5, mount on P3); HARD RULE: never mount a grading skill on staff-engineer
  - decouple/excise edits (B mode-skeleton / C context-file+dispatch examples + L64): port-with-edits wiring, not structural
Independence: CONFIRM-NO-ACTION (skills {executing-tdd-cycle, brownfield-integration} ∩ grading skills = ∅; no self-grade leak; no self-asserted verdict)
Decoupling scan: CLEANER than task-architect — ZERO sibling-names / ZERO self-verdict / ZERO kernel-DAG-catalog / ZERO .humaninloop paths / ZERO meta-labels.
                 Coupled on TWO surfaces only: (B) mode-skeleton L73-81 (Two Execution Modes/Cycle/Fix); (C) "Supervisor dispatches"+context.md examples L8-27 + L64.
                 Craft survives keystone-clean. The subtle profession-trait-vs-loop-position line — separating it in the fix-mode section is the assessment.
```
