# ASSESSMENT: P1 — `tasks` command

**Source:** `human-in-loop/plugins/humaninloop/commands/tasks.md` (613 lines)
**Cluster:** tasks (core-only port) · **Assessed:** 2026-07-01 · ROLE: assess/diagnose ONLY (no transform, no grade, no cross-primitive resolution)
**Branch context:** **4th command port**, and the **2nd net-new command since the 2026-06-30 altitude fix** (after `plan`). Must come out thin by construction (target ≤ ~90 lines, cf. `plan.md` at 82). Gold-standard precedent read and matched: `plugins/mochiko/commands/plan.md`.

---

## Class / Branch

- **Class:** command → **IS-a-loop** branch.
- The HIL `tasks` command IS a loop: a prose **Supervisor** that owns the loop, routes on agent verdicts, manages state via `.workflow/tasks-context.md`, and runs two nested human loops (Clarification + Exit-Early) across two produce→review phases (Mapping → Tasks). What matters most for this branch: who drives the loop, the done-condition, where validation + human gates sit (or are missing), **and the altitude split** — generic loop-discipline (→ `dedupe`) vs. workflow-specific orchestration (→ `moved-to-lead`).
- **Two structural facts distinguish it from `plan`:** (1) it is a **single-reviewer** loop (`task-architect` produces, `devils-advocate`+`validation-task-artifacts` grades) — the **`specify` shape**, NOT `plan`'s two-form; there is no reviewer-architecture question here. (2) both phases share **one producer and one reviewer**, and the two phases produce **overlapping** artifacts (`tasks.md` re-carries the mapping table) — the seed of **RQ-A**.

## Triage (3 gates)

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — it *is* a markdown supervisor; drives 2 agents, owns all routing/state via `.workflow/tasks-context.md`, gated on an upstream plan-workflow-complete prerequisite. |
| 2 | Multi-responsibility / fans out? | **YES** — 2 phases, 2 agent roles, 2 nested human loops (clarification + exit-early), completion-report assembly, entry gate + resume detection + state recovery. |
| 3 | Emits artifact whose correctness is NOT machine-checkable? | **YES** — `task-mapping.md` (vertical-slice quality, story→cycle) and `tasks.md` (TDD cycle structure, file-path specificity) are model-judgment artifacts; no version/schema assert grades them → a **real** producer↔validator pairing is needed, not a degenerate one. |

**gate1=y gate2=y gate3=y → full-lens.** (All three trip; the maximal case, as with `plan`.)

## Disposition

**`redesign` × `absorb-into-lead`** — for the command itself — **+ flag-for-reconcile** on the relational sub-decisions (RQ-A artifact shape · P6 tasks-context-template absorb · the producer-report home).

- **Body = `redesign`** (not `port-with-edits`). Minimalism governor considered and **rejected**: the body assumes a markdown-supervisor orchestration model with a `.workflow/tasks-context.md` state-carrier (a ~30-row placeholder table in Phase 1), inlines every `Task()` / `AskUserQuestion()` payload and `**Phase**: …` `supervisor_instructions` block (§2.1, 2.5, 3.1, 3.5, Clarification Loop), declares **"Use judgment for iteration limits (no hard caps)"** (Important Notes), reads the agent's **verdict field** and routes on it instead of Reading the artifact, and ships **no acceptance gate** on either deliverable. An edit cannot turn "inline everything + trust the verdict field" into "reference doctrine + fill a contract artifact + lead-owned verdict"; the whole orchestration model changes and new gates are added — the identical situation `plan` (613→82), `specify` (329→66), and `setup` (385→78) were redesigned out of. → `redesign`.
- **Structural = `absorb-into-lead`** — a command IS its own lead; the workflow-specific orchestration stays in the thinned supervisor, the generic discipline is *referenced* (`loop-discipline` + `workflow-contract` + `agent-dispatch`), and the `.workflow/tasks-context.md` state-carrier dissolves into `.mochiko/specs/<feature>/` workspace-as-state + in-session (same absorb as `context-template` / `plan-context-template`, 3× confirmed). No orphan skill created.
- The command's **own** placement (absorb-into-lead) is not sibling-dependent and is proposed here. The **relational** sub-decisions (RQ-A, P6, report-home) ARE sibling-dependent and are flagged below, not decided.

---

## Responsibility trace (COMPLETE — altitude rule applied)

Keystone test on every responsibility: *true of any sound loop → `dedupe` into `loop-discipline`; only true of THIS workflow → `moved-to-lead`.*

### A. Generic loop-discipline mechanics → `dedupe` (NEVER `moved-to-lead`)

True of any sound mochiko loop; already single-sourced in `loop-discipline` (+ `workflow-contract`, `agent-dispatch`). The command **references** them; copying them into the body is the altitude defect `verify-output` fails.

- The produce→review→revise **iteration structure** (the two phase-loops + the Clarification Loop *as loops*) → `dedupe`.
- **Default-FAIL done-condition mechanics** (the artifact starts FAILing) → `dedupe`. *(HIL lacks this — §E; only params move to lead.)*
- **Producer↔validator independence doctrine** ("validation is a different agent + different skill, grading from the artifact; the lead owns the verdict") → `dedupe`. *(HIL states it as prose "AGENTS (independent, no workflow knowledge)".)*
- **Validator trustworthiness tiers** · **tamper-proofing** (no PASS without evidence Read from the real artifact) → `dedupe`.
- **The four iteration guards** (hard cap / no-progress / kill-switch / escalate-don't-die) as *requirements* → `dedupe`. *(HIL's "no hard caps" violates this — §E adds the deterministic versions; the requirement is doctrine.)*
- **Gap-type routing** (knowledge→research · preference→human · scope→halt) → `dedupe`. HIL's Clarification-Loop **"Research this"** branch (Explore / WebSearch / context7 for a factual unknown) is a concrete instance of knowledge-gap→research → `dedupe`.
- **Anti-rationalization** ("Always give the user the choice—never force-terminate without consent"; exhaustion≠done) → `dedupe`.
- **Briefing-each-dispatch** mechanics (what a good brief carries) → `dedupe` into `agent-dispatch`. *(HIL transliterates full prompt bodies inline — §C drop.)*
- Standing supervisor footer "Do NOT modify git config or push to remote" · "Always use Task tool to invoke agents" · "Agents have NO workflow knowledge—all context via context file" → `dedupe` (the standard lead-footer lines; not tasks-specific).

### B. Workflow-specific orchestration → `moved-to-lead` (only true of THIS workflow)

Only these stay as the thinned supervisor's body. Several depend on **RQ-A** for their final shape — noted.

- The **2-phase Mapping→Tasks sequence** (Phase 2 = `task-mapping.md` story→cycle + slice rationale; Phase 3 = `tasks.md` cycle→TDD-tasks) → `moved-to-lead`. **Final shape rides on RQ-A** (two phases/artifacts vs. one).
- The **CUMULATIVE review** — Phase 3's tasks review is **not isolated**: it cross-checks `tasks.md` back to `task-mapping.md` (mapping-tasks alignment, story→cycle→tasks traceability chain, cycle/dependency consistency) → `moved-to-lead`. This is the tasks analogue of `plan`'s Phase-2 dual-mode incremental review. **The lead owns the MODE SELECTION and supplying BOTH artifacts to the reviewer**; the CHECKS themselves live in `validation-task-artifacts` (P4, "Cross-Artifact Review" section — §D). **Silent-drop risk** (§F) — trivially flattened to "review tasks.md" in a thin rewrite.
- The **early mapping-review gate** — the advocate reviews slicing quality (vertical vs horizontal, foundation separation, IP-XXX coverage, cycle sizing) *before* the architect expands every cycle into a full TDD breakdown → `moved-to-lead`. The exact "how expensive is bad slicing to redo" rationale mirrors `plan`'s feasibility-once-before-completeness. **Tied to RQ-A** (§F).
- **Team casting (single-reviewer, the `specify` shape)** — `task-architect` produces **both** phases; `devils-advocate` (+ `validation-task-artifacts`) grades **both** phases → `moved-to-lead`. Independence is structurally sound in HIL (architect ≠ advocate) — preserve it. *(NOT `plan`'s two-form; no feasibility reviewer.)*
- **Entry-gate prerequisite (plan-workflow-complete)** → `moved-to-lead`: block unless `plan.md` present AND the plan workflow is accepted. HIL reads `plan.md` existence + `.workflow/plan-context.md` `status == completed`; **rebind** the status read to workspace evidence (plan is now workspace-as-state — §C, transliteration risk §F). On fail → block/point to `/mochiko:plan`.
- **Constitution as governing context** → `moved-to-lead`: carried into the producer's brief (Phase 1 `{{constitution_principles}}`; the architect Reads `constitution.md`). NOTE the distinction from `plan`: tasks' *hard* prerequisite is plan-completion, **not** the constitution — the constitution is governing context here, not a blocking Phase-0 gate. (Softer than `plan`'s constitution prerequisite; keep it that way unless reconcile decides to strengthen.)
- **Brownfield-from-plan routing** → `moved-to-lead`: "Brownfield context comes from plan artifacts, not re-analyzed" (Important Notes) — tasks does NOT re-run codebase analysis; it inherits brownfield context from plan's artifacts. *(The task-architect's `evolution-roadmap.md` / `[GAP:XXX]` read is a **documented stub** — roadmap track deferred, §D / deferred-by-reference.)*
- **This loop's done-condition parameters** (measurable end state: both artifacts present + validated; the cap *number*; the named gate placements) → `moved-to-lead` (filled into the `workflow-contract` artifact, not inlined).
- **Phase 4 completion** — assemble the completion report: cycle counts, foundation/feature counts, `[P]` parallel-opportunity count (extracted from `task-mapping.md` / `tasks.md`); next-step pointer → `moved-to-lead`, **rebound** to `/mochiko:implement` (implement-cluster, not yet ported — a documented forward reference, not a live command).
- **Empty-input `@`-reference recovery** (the known Claude Code bug; re-enter / continue) → `moved-to-lead` (same workflow-specific input handling as `specify` G1 / `plan` Phase-0).
- **Resume detection + state recovery** — resume an interrupted run → `moved-to-lead`, **rebound** to workspace evidence (HIL resumes from `phase`+`status` in tasks-context.md; mochiko resumes from workspace-as-state — §C). The HIL State-Recovery table (7 phase/status→resume-point rows) collapses into a workspace-evidence resume table like `plan`'s / `specify`'s.
- **Existing mid-loop human gates** → `moved-to-lead` (must survive alongside the NEW acceptance gate — §E, §F):
  - **Clarification Loop** gate (present advocate gaps; user answers; the "Research this" option) — preference-gap + the knowledge-gap research branch.
  - **"When to Exit Early"** offering (continue-refining / accept-current / stop-and-review). *(The generic "escalate, don't force-terminate" half is `dedupe` §A; only the tasks-specific offering placement is `moved-to-lead`.)*
- **Operational handling** — verify-agent-output (confirm each expected file was written — `task-mapping.md`, `tasks.md`, the round reports; on missing → log + ask retry/abort) and Agent-Failure messaging → `moved-to-lead`. *(Generic supervisor hygiene, but the specific file lists are tasks-specific.)*

### C. Content / path / state-carrier couplings

- `.humaninloop/memory/constitution.md` → **`kept-but-rebind`** (`.humaninloop/` → `.mochiko/`).
- `specs/{feature-id}/…` workspace + `${CLAUDE_PLUGIN_ROOT}/templates/…` refs → **`kept-but-rebind`** to `.mochiko/specs/<feature>/…` (matches `specify`/`plan` workspace-as-state). The round reports `architect-report.md` / `advocate-report.md` survive as per-round artifacts under the workspace (the `.workflow/` subdir dissolves).
- **Entry-gate read of plan's `.workflow/plan-context.md` `status == completed`** → **`kept-but-rebind`** — rebind to "`plan.md` present and accepted" workspace evidence; do **not** transliterate the status-field read (mochiko `plan` no longer writes a context-file `phase`/`status`). **Transliteration risk** (§F).
- `.workflow/tasks-context.md` **state-carrier** (the **P6** template: all `{{…}}` placeholders, the ~30-row File-Paths + status table, `supervisor_instructions`, `phase`/`status`/`iteration` fields, Clarification Log) → **`dropped + reason`**: kernel-adjacent markdown state-carrier; mochiko uses workspace-as-state + in-session (same absorb as `context-template` / `plan-context-template`). **The P6 absorb is FLAGGED for reconcile** (§ Reconcile) — 3×-confirmed precedent, so a *confirm*, not an open question.
- Inlined `Task(...)` / `AskUserQuestion(...)` payload bodies and the full `**Phase**: …` `supervisor_instructions` prose blocks (§2.1, 2.5, 3.1, 3.5, Clarification Loop, Exit-Early) → **`dropped + reason`**: transliterated mechanics; `agent-dispatch` briefing replaces inline prompt-body transliteration, and inlining them is the altitude defect `verify-output` now fails. The briefing *content* (which inputs to Read, where to Write, which skills to hint) → folded into concise `agent-dispatch`-style params (`moved-to-lead`, §B).
- HIL `iteration` counter spelled out inline + **"Use judgment for iteration limits (no hard caps)"** → **`dropped + reason`**: replaced by the contract's deterministic cap (§E); a hand-coded LLM-judged counter is the transliteration `verify-output`'s altitude floor rejects.
- The **Architecture Overview** ASCII diagram + **Communication Pattern** file-tree + **Agents Used** table (§40–89) → **`dropped + reason`**: doctrine/illustration restated in the body; the team + dispatch model is referenced via `agent-dispatch`, not drawn inline (altitude).

### D. Producer / validator content embedded in the supervisor → **cluster siblings** (NOT command-body)

These ride inside HIL's `supervisor_instructions` / `**Check**:` blocks but are **producer** (P2 `task-architect` + P3 `patterns-vertical-tdd`) or **validator** (P4 `validation-task-artifacts`) responsibilities, not command-level orchestration. They must NOT be carried into the thin command, and must NOT be silently lost — unlike `plan` (whose producer/validator skills were `[ ]` other-cluster), here **the receiving siblings P2/P3/P4 are ported THIS run**, so the content lands in-cluster.

- **Vertical-slice identification + TDD cycle structure** (Phase 2/3 "Use Skills: `patterns-vertical-tdd`") → belongs to **P3 `patterns-vertical-tdd`** + **P2 `task-architect`** (`folded-into-skill` / producer-persona). The command only *hints the skill*; it does not restate the procedure.
- **The advocate's phase checks** — Mapping Review (story coverage, vertical-slice check, foundation separation, IP-XXX coverage, platform-app ordering, sizing), Tasks Review (TDD test-first ordering, file-path specificity, `**TEST:**` verification tasks, `[EXTEND]`/`[MODIFY]` markers), and the **Cross-Artifact Review** (mapping-tasks alignment, story→cycle→tasks traceability) → belong to **P4 `validation-task-artifacts`** (`moved-to-sibling-skill` — the checks already live in P4's `PHASE-CHECKLISTS.md`). The command's Phase-3 `**Check**:` list is a restatement → dropped from the command; the capability is P4's. **Highest-volume silent-drop surface** (§F): the thin command drops the inline instructions wholesale, so these survive ONLY if P4 (re-mounted on `devils-advocate`) carries them.
- **IP-XXX infrastructure coverage through mapping→tasks** (every IP-XXX item mapped to a foundation cycle, then to ≥1 task) → P3/P4 content; a real capability, must survive in the ported skills. **Silent-drop risk** (§F).
- The task-architect's brownfield **`evolution-roadmap.md` / `[GAP:XXX]`** read → **deferred-by-reference** (roadmap track `authoring-roadmap` + `evolution-roadmap-template` deferred, per context/contract; a documented P2 stub, not a live mount — as setup stubbed `syncing-claude-md`).

### E. Missing loop-discipline gates to ADD (HIL lacks them)

Requirement = `dedupe` (mechanics live in `loop-discipline`); placement/params = `moved-to-lead` (filled into the contract).

- **Default-FAIL done-condition** — HIL has none (the run self-declares done on a `ready` verdict at pass 1). ADD: both artifacts start FAIL; clear only on independent validation + (new) human acceptance. Requirement `dedupe`; params `moved-to-lead`.
- **Lead-OWNED verdict** — HIL's supervisor reads the advocate's **verdict field** and routes on it (§2.6, §3.5 "extract verdict"; trusts the report). ADD: the lead **Reads** `task-mapping.md` / `tasks.md` + the advocate report and owns the clearing verdict; the advocate's status is **input, not the gate** (the reversal `specify`/`plan` applied). Doctrine `dedupe`; the reversal note is a workflow-specific design note `moved-to-lead`.
- **Hard bound + kill-switch** — HIL explicitly "no hard caps" + only a model-judged "gaps aren't resolving" heuristic. ADD: deterministic round cap (number `moved-to-lead`; contract sets cap **3/phase**), no-progress exit (unchanged gap set round-over-round), kill-switch file. Requirements `dedupe`; params `moved-to-lead`.
- **NEW human ACCEPTANCE gate on the deliverable** — HIL has mid-loop human gates (clarification, exit-early) but **no final acceptance gate** on `tasks.md` (Phase 4 just writes status + reports). ADD a named acceptance gate (accept → done / amend → bounded re-enter / reject → abort), exactly like `specify` G3 / `plan` G5. Requirement (named human gate) `dedupe`; placement `moved-to-lead`. **Must not displace** the existing mid-loop gates (§B, §F).

---

## Reconcile flags (relational — NOT decided here)

1. **RQ-A — artifact shape (one deliverable or two).** *The named cluster-scope question; PRIMARY flag.* HIL runs two phases producing two artifacts — `task-mapping.md` (story→cycle + slice **rationale**) then `tasks.md` (cycle→TDD-tasks) — but `tasks.md` already carries a "Story → Cycle Mapping" table + per-cycle `> Stories:` traceability, so the *result* content is duplicated. **This is genuinely cross-primitive** (it decides P2's phase behaviors, P4's Mapping/Tasks/Cross-Artifact checklist structure, and P5's template shape) → flag, do not guess.
   - **(a) keep two phases / two artifacts** — matches HIL + `plan`'s two-phase precedent; preserves the **early mapping-review gate** (catch bad slicing before the expensive full task breakdown) and the **cumulative cross-check** as native cross-artifact review; P4 is *already* built for it (separate Mapping/Tasks/Cross-Artifact checklists). Cost: one extra produce→review round, and the mapping-table duplication reads as a dedupe smell.
   - **(b) collapse mapping into `tasks.md`** — one artifact, one review, no duplication; thinner. Cost: loses the early gate (bad slicing surfaces only after the full breakdown), forces a **redesign of P4** (fold Mapping Review into a single-pass Tasks Review) and P2's phase behaviors — a wider blast radius than the command — and dissolves the cross-artifact cumulative check into an intra-artifact one.
   - **RECOMMENDATION → (a) keep two phases / two artifacts**, for three reasons: the early slicing gate has the same real value `plan`'s feasibility-once gate does (cheap rework avoidance); P4 and P2 are *already structured for two phases* so (a) is the least-disruptive move (minimalism governs the thin *command*, which is thin either way — it must not govern erasing a validator's proven structure); and the cumulative cross-check is native to two artifacts. **Resolve the collapse camp's one strong point (duplication) without collapsing:** make `task-mapping.md` the **source of truth** for slicing rationale + story→cycle decisions (it is already template-free — **no `task-mapping-template.md` exists in HIL** — i.e. the lighter reasoning artifact), and make `tasks.md`'s Story→Cycle table an explicitly-**derived echo** for the implementer, not an independent second source. One source, one derivation → the sync-risk objection dissolves. *(Independence is unaffected either way — both options keep architect≠advocate.)*
2. **P6 — `tasks-context-template` absorb.** Disposition signal: `absorb-into-lead` (workspace-as-state + in-session), template `dropped`. It is a separate primitive; its absorb is a cross-primitive structural call → flagged. **Confirm, not open** — 3×-confirmed precedent (`context-template`, `constitution-context-template`, `plan-context-template`).
3. **Producer-report home.** The command writes `architect-report.md` (task-architect) + `advocate-report.md`. Advocate side is **settled** — `advocate-report-template` is reused as-is (already in mochiko). Open: does the task-architect get its own **`taskarchitect-report-template`** (mirroring `techanalyst-report-template`) or inline persona self-disclosure? Touches P2 + a possible net-new template → reconcile decides.

**Explicitly NOT flags (settled — recorded so the reader knows they were weighed):**
- **Reviewer architecture** — settled **single-reviewer** (`specify` shape). Unlike `plan`'s RQ1, tasks has one reviewer (`devils-advocate` + `validation-task-artifacts`); there is no homeless feasibility gate to place. No two-form question.
- **Team casting / independence** — `task-architect` produces, `devils-advocate` grades; disjoint agents, structurally independent in HIL. Preserve; nothing to resolve.
- **`validation-task-artifacts` re-mount on `devils-advocate`** — cluster wiring (the stub is already present on mochiko `devils-advocate` per the plan-cluster follow-up), executed in the convention-wiring/transform pass — not a command-body decision.

*(Cluster dependencies, not command-body flags: P2 `task-architect`, P3 `patterns-vertical-tdd`, P4 `validation-task-artifacts`, P5 `tasks-template`. The §D producer/validator content lands in these — assessed separately.)*

## Silent-drop risks (for the lead/human to accept)

- **The CUMULATIVE review** (§B) — tasks review cross-checks back to `task-mapping.md`; the single most tasks-specific orchestration nuance and trivially flattened to "review tasks.md." Must survive as an explicit lead routing param (cumulative mode: supply BOTH artifacts to the reviewer); the CHECKS live in P4's Cross-Artifact Review. Contract calls this out by name.
- **Entry-gate plan-workflow-complete prerequisite** (§B/§C) — must rebind the `plan-context.md` `status` read to "plan.md present and accepted" workspace evidence, NOT transliterate a status field mochiko `plan` no longer writes.
- **The early mapping-review gate / two-phase sequence** (§B) — tied to RQ-A; if collapse wins, ensure slicing is still reviewed *before* the full task breakdown, else bad slicing is caught expensively late.
- **§D producer/validator content** — vertical-slice + TDD structure (P3/P2), the phase checklists incl. **IP-XXX infra coverage** and **`**TEST:**` verification-task discipline** (P4). The thin command drops the inline instructions; these capabilities survive ONLY if P2/P3/P4 carry them. Highest-volume silent-drop surface. *(Boundary note for the P4 assessment/reconcile — not decided here: P4 checks for `**TEST:**` real-infrastructure verification tasks, but the TEST-authoring capability `testing-end-user` is deferred to implement-cluster; confirm the ported validator's TEST demand still has an author, or scope the check.)*
- **Existing mid-loop human gates** (clarification / exit-early) — must not be displaced by the new acceptance gate; ADD the acceptance gate, KEEP these.
- **"Research this" knowledge-gap→research branch** — a real capability; `dedupe`'d to loop-discipline's gap-routing, but the lead must actually route knowledge gaps to `Explore` (don't lose it in the dedupe).

---

## Output block

```
ASSESSMENT: P1 — tasks command
Class:        command → branch IS-a-loop
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  redesign × absorb-into-lead  (+ flag-for-reconcile: RQ-A artifact shape · P6 tasks-context-template absorb · producer-report home)
Trace:        complete — every responsibility tagged (see §A–§E)
  altitude split → dedupe (generic): iteration structure · default-FAIL mechanics · independence doctrine ·
                   validator tiers · tamper-proofing · the four guards (requirements) · gap-type routing
                   (incl. "Research this") · anti-rationalization · briefing mechanics · git/Task footer
                → moved-to-lead (workflow-specific): 2-phase Mapping→Tasks sequence (RQ-A) · CUMULATIVE
                   review (mode + supply both artifacts) · early mapping-review gate (RQ-A) · single-reviewer
                   team casting (specify shape) · plan-workflow-complete entry gate (rebound) · constitution-as-
                   context · brownfield-from-plan · done-condition params (cap #, gates) · Phase-4 completion
                   report + /implement pointer · @-input recovery · resume/state-recovery (rebound) ·
                   clarification + exit-early human gates · op handling (verify-output, agent-failure)
                → kept-but-rebind: .humaninloop→.mochiko · specs/→.mochiko/specs · round reports ·
                   entry-gate plan status→workspace evidence
                → dropped+reason: tasks-context.md state-carrier (P6, FLAG-confirm) · inline Task/AskUserQuestion
                   payloads + supervisor_instructions · LLM-judged "no hard caps" counter · Architecture/
                   Communication/Agents-Used diagrams (restated doctrine)
                → cluster-siblings (NOT command-body): vertical-slice+TDD structure → P3/P2 · phase checklists
                   incl. IP-XXX coverage + TEST: discipline + Cross-Artifact review → P4 · evolution-roadmap read
                   → deferred-by-reference (roadmap track)
  ADD (HIL gaps): default-FAIL done-condition · lead-owned verdict (status=input) · hard cap + kill-switch ·
                   NEW human acceptance gate on tasks.md
Reconcile flags: RQ-A artifact shape [REC: keep two, task-mapping.md=source-of-truth, tasks.md table=derived echo] ·
                 P6 tasks-context-template absorb (confirm) · producer-report home (taskarchitect-report-template?)
  NOT flags (settled): single-reviewer architecture (specify shape) · team casting/independence · P4 re-mount (wiring)
Silent-drop risks: CUMULATIVE review · entry-gate status rebind · early mapping gate (RQ-A) · §D producer/validator
                   content (slice/TDD/IP-XXX/TEST:) · existing mid-loop gates · "Research this" branch
```
