# Phase 2 Reconciliation — cluster `implement`

**Run:** `/mochiko:transform-cluster implement` · **Lead/referee:** transform-cluster supervisor (this session)
**Skill:** `mochiko:reconcile-cluster` · **Date:** 2026-07-01 · **Consumes:** the 7 Phase-1 assessments + `context.md` + `contract.md`
**Precondition met:** all 7 primitives assessed (Phase-1 log entry). This runs once, cluster-wide.

> **Reconcile decides; it does not edit.** No artifact is produced here (that is Phase-3 `transform-recipes`).
> Output = finalized dispositions + the single authoritative rehome map + closed traces, with **zero open flags.**

---

## 0. Verdict at a glance

- **Open flags: ZERO.** Every `flag-for-reconcile` from all 7 assessments is resolved to a concrete move (§Job 1 ledger).
- **New primitives required: NONE** — no new agent, no new skill, no `loop-discipline` edit. The thin `implement` lead absorbs the orchestration; the producer↔validator pair already exists. (§New primitives, stated explicitly.)
- **The rehome map** lands constraints (a)–(g) + both gate placements **once** on the thin lead, split `dedupe-to-loop-discipline` (reference) vs `rehome-to-lead` (workflow-specific), with every P7 survivor **merged into** its P1 twin (no double-home). (§Job 2.)
- **Finalized dispositions:** P1 `redesign × absorb-into-lead`; P2/P3/P4/P5/P6 `port-with-edits × standalone`; P7 `drop` → realized `absorb-into-lead` (survivors→lead) + `dedupe` (doctrine→loop-discipline), no residual skill.
- **Report-format call (human-gate item):** **keep** cycle-report + verification-report formats in their skill `references/` — do **not** extract to `templates/`. (§Seam 6, recommended + justified.)

---

## Job 1 — Relational verdicts (every flag → a concrete move)

Master ledger. Each row is a `flag-for-reconcile` (or relational signal) from an assessment, with its resolved structural move and partner. Nothing below stays open.

| # | Primitive | Flag (as emitted) | Resolved move | Partner(s) | Where resolved |
|---|-----------|-------------------|---------------|------------|----------------|
| 1 | P1 | **FR-1** — the (a)–(g) rehome split [overlaps P7 + dissolved State-Analyst] | **rehome-orchestration**: workflow params `rehome-to-lead` + doctrine `dedupe`→`loop-discipline`; **MERGE P7 survivors in** | P7, State-Analyst-slice | §Job 2 (the map) |
| 2 | P1 | **FR-2** — producer/validator casting + lead-owned verdict [overlaps P2/P3] | **confirm pairing + wire** (no restructure): staff produces ≠ qa grades; verdict `moved-to-lead` | P2, P3 | §Seam 2 |
| 3 | P1 | **FR-3** — fix-mode routing vs. fix craft split [overlaps P2] | **split**: routing/trigger/bounds `rehome-to-lead`; craft `kept` in P2; procedure `dedupe`→P4 | P2, P4, P7 | §Seam 3 |
| 4 | P1 | **FR-4** — confidence-based gate wiring [overlaps P5] | **three-way split**: classification→P5; judgment→P3; placement→lead | P5, P3 | §Seam 4 |
| 5 | P2 | fix-pass routing/bounds (cross-primitive P1+P7) | same as #3 — routing→lead, craft stays, procedure→P4 | P1, P4, P7 | §Seam 3 |
| 6 | P2 | dedupe / persona-keeps-taste → P4/P6 | **dedupe**: persona keeps disposition; literal procedure stays canonical in P4 (`executing-tdd-cycle`) + P6 (`brownfield-integration`); no orphan | P4, P6 | §Job 1.1 |
| 7 | P2 | producer↔validator pairing CONFIRM-SETTLED | same as #2 — confirm; mount stays on P3; **never** mount a grading skill on staff | P3 | §Seam 2 |
| 8 | P2 | decouple/excise edits (B mode-skeleton / C context-file+dispatch) | **wiring** (not structural) → handed to `transform-recipes` decouple pass | — | §Job 1.2 |
| 9 | P3 | **F1** — preserve producer↔validator pairing; rehome execute→verify wiring to lead | same as #2 — confirm pair (different agents, disjoint skills); pairing declared by lead casting, not persona | P2 | §Seam 2 |
| 10 | P3 | **F2** — confidence-gate three-way split; confirm the seam | same as #4 | P5, P1 | §Seam 4 |
| 11 | P3 | **F3** — FAIL-loop driving belongs to the lead | **rehome-to-lead**: targeted-retry / fix-pass / convergence-stall; qa persona carries none (emits verdict only) | P1 | §Job 2 (c/d/e) |
| 12 | P4 | retry/fix routing seam (cross-primitive P1+P2+P7) | same as #3 — HOW-craft `kept` in P4 (decoupled from "dispatched"); routing→lead; generic mechanics `dedupe`→`loop-discipline` | P1, P2, P7 | §Seam 3 |
| 13 | P5 | grammar dedupe — P5 restates `**TEST:**` grammar owned by `patterns-vertical-tdd` | **dedupe / single-source** (NOT merge): P5 references the legal set; keeps execution/parse semantics | patterns-vertical-tdd | §Seam 5(a) |
| 14 | P5 | reciprocal classification table (CYCLE-STRUCTURE.md L250–254 reproduces P5's CLI/GUI/SUBJECTIVE) | **dedupe (minor, low-priority)**: slim to a pointer back to P5 when patterns-vertical-tdd is next touched; non-blocking (direction already correct — it cedes ownership) | patterns-vertical-tdd | §Seam 5(a) |
| 15 | P5 | P4↔P5 runtime boundary (confirm disjoint) | **confirm** — disjoint (P4 producer-side write-code; P5 validator-side verify; P4 has 0 `**TEST:**` matches, no evidence/checkpoint/result-classification dup); shared discipline referenced, not duplicated | P4 | §Job 1.3 |
| 16 | P5 | independence mount (confirm) | **confirm** — P5 mounts on qa-engineer **only**; never co-mounted with P4 on staff | P2, P3 | §Seam 2 |
| 17 | P6 | marker-grammar single-source — P6 restates `[EXTEND]`/`[MODIFY]` owned by `patterns-vertical-tdd` | **dedupe / reference** (NOT merge): P6 references the vocabulary owner; keeps implement-time consumption/interface-impact semantics | patterns-vertical-tdd | §Seam 5(b) |
| 18 | P7 | **FR-A** — assign dedupe + accept DROP of standalone skill (TRACE rule 3) | **dedupe** DD1–DD4→`loop-discipline` (no edit — already covered); human gate accepts the **DROP** | loop-discipline | §Job 1.4 + gated bundle |
| 19 | P7 | **FR-B** — resolve jointly with P1 FR-1; survivors are the SAME responsibilities | **MERGE into FR-1** — land each survivor once on the lead; do not double-home | P1 | §Job 2 (merge column) |
| 20 | P7 | **FR-C** — cross-workflow shared-substrate / close the family | **confirm** — no `strategy-*` skill survives in mochiko for any workflow (core + specification already gone; this is the third/last) | — | §Job 1.5 |

### 1.1 — Dedupe/persona-keeps-taste (P2 flag #6) — resolved

No structural move; a `dedupe` with an already-existing canonical home. The `staff-engineer` persona keeps only **taste/disposition** (genuine red phase, simplest impl, read-before-write, no scope creep, honest reporting, scope-tightly, reproduce-with-a-test); the **literal procedure** stays canonical in the two mounted skills — `executing-tdd-cycle` (P4: red/green/refactor + retry/fix HOW) and `brownfield-integration` (P6: EXTEND/MODIFY + read-before-write). Both skills land in this same cluster this run, so the fold has a landing place — **no orphan procedure**. The P4↔`patterns-vertical-tdd` design↔runtime boundary (context §45) is a P4 concern, confirmed holding in §Job 1.3.

### 1.2 — Decouple/excise edits (P2 flag #8) — routed to transform, not structural

The "Two Execution Modes" mode-skeleton (B1–B5) and the "Supervisor dispatches / read from context.md" example framing + L64 (C1–C4) are `port-with-edits` **wiring actions**, not structural moves. They belong to Phase-3 `transform-recipes` (decouple pass, wiring step 5) and are audited by `verify-output`'s decoupling scan. Logged here so nothing is dropped; the *content* of the split (which lines are craft vs. routing) is settled in §Seam 3.

### 1.3 — P4↔P5 runtime boundary (P5 flag #15) — confirmed disjoint

Complementary opposite sides of the same `**TEST:**` gate: **P4** is the producer-side runtime (write code red/green/refactor, emit `cycle-report.md`); **P5** is the validator-side runtime (verify against real infra, capture evidence, classify, present checkpoint). Confirmed non-overlapping: P4's assessment records **0 `**TEST:**` matches** across its SKILL + 3 refs and no re-implementation of P5's evidence-capture / checkpoint / result-classification substance. Shared TDD discipline is a **referenced banner**, not duplicated body. No action.

### 1.4 — P7 dedupe assignment (FR-A) — resolved

DD1 (execute-then-verify *principle*), DD2 (iterate-on-FAIL *backbone*), DD3 (escalate-before-stall *doctrine*), DD4 (convergence-signals / no-progress) all **`dedupe`→`loop-discipline`** (Req 2 / Req 3 / Req 3.4+Req 4 / Req 3.2 respectively) — **all already present; NO `loop-discipline` edit this run.** The standalone `strategy-implementation` skill is **DROPPED** (no `strategy-implementation.md` in mochiko); human gate accepts the drop (contract §4a names it).

### 1.5 — Strategy family closed (FR-C) — resolved

`strategy-implementation` was `state-analyst` substrate; its dissolution is cluster-global. Confirmed: **no `strategy-*` skill survives in mochiko for any workflow** — `strategy-core` and `strategy-specification` dissolved in the specify run; this is the third and final. Any future workflow's "strategy" = `loop-discipline` (doctrine) + that workflow's lead (parameters), never a carried strategy skill.

### 1.6 — Trigger de-collisions (wiring → transform, not structural)

Two trigger collisions handed to the Phase-3 convention-wiring pass (they are phrasing de-collisions, not merges):
- **P4 vs `patterns-vertical-tdd`** on `red green refactor` / `TDD`: design-time *structuring / test-first ordering* → `patterns-vertical-tdd`; runtime *executing the cycle* → P4.
- **P6 vs P4** on `[EXTEND]`/`[MODIFY]`: both legitimately co-fire on a brownfield cycle (P4 = cycle execution/red-green-refactor; P6 = read-before-write EXTEND/MODIFY craft). Scope the two descriptions so both fire cleanly; no merge.

---

## Job 2 — THE REHOME MAP (the run's centerpiece)

The dissolving orchestrators — the **DAG-Supervisor (P1)** + the **State-Analyst implement-slice** + **`strategy-implementation` (P7, dropped)** — land their survivors **once** on the thin `implement` lead. This is the largest orchestration rehome of any port.

### 2.0 — The inverse-of-setup nuance (STATED EXPLICITLY)

In `setup`/`specify`, reconcile had to **construct a missing independent validator** — the producer graded its own work, so the rehome *added* a validator agent + `verify-*` skill and built the pair.

**`implement` is the inverse.** The independent producer↔validator pair **already exists, structurally**: `staff-engineer` produces (skills `executing-tdd-cycle` + `brownfield-integration`) and a **different agent**, `qa-engineer`, grades (skill `testing-end-user`) — disjoint skills, a Tier-1 deterministic validator. Therefore this rehome:

1. **ADDS the missing GATES** that HIL never had at the mochiko bar — the default-FAIL done-condition, the confidence-based per-cycle gate made explicit, the **named final-acceptance gate** (HIL had none), and the overall round-cap / no-progress / STOP kill-switch (HIL had per-cycle INV-004 but no overall mochiko bound); and
2. **MOVES VERDICT-OWNERSHIP** from the dissolved State-Analyst (which evaluated `cycle-checkpoint` / `final-validation` autonomously) **onto the lead** — the lead Reads the reports directly and owns the clearing verdict; qa's verdict is *input*, never a staff self-grade.

It does **NOT** construct a missing validator. The pair is cast, not created.

### 2.1 — The authoritative map

Each orchestration responsibility → **`dedupe-to-loop-discipline`** (generic; the lead *references* it, never restates) or **`rehome-to-lead`** (workflow-specific; lives in the thin command). Keystone test applied per row: *true of any sound loop → dedupe; only true of THIS workflow → lead.* The **Merge** column marks each P7 survivor as the SAME responsibility P1 already enumerated (`=P1 Lx`) — landed **once**, never double-homed.

#### Constraints (a)–(g)

| Constraint | Slice | Keystone | Lands | Merge |
|-----------|-------|----------|-------|-------|
| **(a)** sequential cycle sequencing — foundation-before-feature; current cycle = first with unchecked `tasks.md` tasks | whole | only THIS workflow (cycles + `tasks.md` checkboxes) | **rehome-to-lead** | `=P1 L6 = P7 L-a` + SA Briefing-Rule-9 |
| **(b)** execute-then-verify | *independent-verification principle* (distrust self-report; systematic blind spots) | any sound loop | **dedupe → `loop-discipline` Req 2** (+ tamper-proofing) | `=P7 L-b(principle) = DD1` |
| | *pairing shape* — every staff cycle followed by a qa verification, same round, never skipped | only THIS workflow | **rehome-to-lead** (+ casting → §Seam 2) | `=P1 L7 = P7 L-b(shape)`; P2, P3 F1 |
| **(c)** targeted retry | *iterate-on-FAIL bounded backbone* | any sound loop | **dedupe → `loop-discipline` Req 3** | `=P7 L-c(backbone) = DD2` |
| | *scoping tactic* — re-open only failed tasks, don't rewrite working code; **max 3/cycle** param | only THIS workflow | **rehome-to-lead** | `=P1 L8 = P7 L-c(tactic)` |
| | *HOW-craft* — trace→re-open (`[x]`→`[ ]`)→TDD-only-reopened→updated-attempt report | producer craft | **kept in P4** (`executing-tdd-cycle`); disposition kept in P2 | P4, P2 |
| **(d)** fix-pass mode | *bounded backbone* (a cap exists; escalate-don't-die) | any sound loop | **dedupe → `loop-discipline` Req 3** | `=P7 L-d(backbone) = DD3` |
| | *routing/WHEN* — enter after final-validation failure; "unconstrained by cycle boundaries"; scoped strictly to reported failures; **max 3 fix passes** | only THIS workflow | **rehome-to-lead** (→ §Seam 3) | `=P1 L9 = P7 L-d`; P2 fix-mode |
| | *fix craft* — reproduce-with-a-failing-test before fixing; scope tightly; follow the failure, not a refactor license | producer craft | **kept in P2** persona; *procedure* → **P4** | P2, P4 |
| **(e)** convergence-stall detection | *no-progress-exit doctrine* — stop when a round changes nothing; escalate | any sound loop | **dedupe → `loop-discipline` Req 3.2** | `=P7 DD4` |
| | *implement parameter* — same failure pattern **2+ rounds** → surface | only THIS workflow | **rehome-to-lead** | `=P1 L10` |
| **(f)** entry gate — tasks-workflow-complete prerequisite (`tasks.md` present/complete before implementing) | whole | only THIS workflow (handoff from the tasks workflow) | **rehome-to-lead** as an explicit **handoff edge**; path rebind → `.mochiko/specs/<feature>/tasks.md` present/complete (workspace evidence, not context-file status) | `=P1 L3 / R2`; P7 L-f ordering-guard rides with done-condition ordering |
| **(g)** project scaffolding — ignore-file creation from detected stack | whole | only THIS workflow | **rehome-to-lead**; output paths kept project-relative (`.gitignore`/`.dockerignore`/lint-ignore) | `=P1 L5 / R3` |

#### The two gate placements (intake decision #2)

| Gate | Slice | Keystone | Lands |
|------|-------|----------|-------|
| **Confidence-based per-cycle gate** | *classification procedure* — CLI (measurable, 100%-pass) ⇒ auto-approve; GUI/SUBJECTIVE/any-fail ⇒ checkpoint; default SUBJECTIVE | validator-side runtime rule | **P5 `testing-end-user`** (kept — the reclaimed Task-Classification table; = P1 S1) → §Seam 4 |
| | *conservative judgment/disposition* — uncertain ⇒ default to human; "ambiguity is a reason to escalate, never to auto-approve" | validator persona | **P3 `qa-engineer`** persona (kept) → §Seam 4 |
| | *placement doctrine* — "human gate at low validator-confidence" | any sound loop | **dedupe → `loop-discipline` Req 4** ("low validator-confidence only" placement) |
| | *gate placement + auto-approve/checkpoint wiring* | only THIS workflow | **rehome-to-lead** (`=P1 L13`) → §Seam 4 |
| **Named final-acceptance gate** | *human-gate presence* | any sound loop | **dedupe → `loop-discipline` Req 4** |
| | *the placement* — a named final acceptance before the workflow reports done | only THIS workflow (NEW; HIL had none) | **rehome-to-lead** (`=P1 L14`) |

#### Residual dissolving-supervisor / State-Analyst-slice orchestration

| Responsibility | Keystone | Lands |
|----------------|----------|-------|
| Lead role + **clearing-verdict ownership** — lead Reads artifacts + qa verification, decides against default-FAIL; qa status is *input* | *verdict-ownership principle* → any sound loop | **dedupe → `loop-discipline` Req 2**; the workflow-specific evaluation → below (`=P1 L0`) |
| **Verdict evaluation params** (State-Analyst reclaim) — cycle-checkpoint = criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + constitution alignment | only THIS workflow | **rehome-to-lead** as the lead's Tier-1 asserts feeding its verdict (`=P1 L17`) |
| **Direct-Read** of `cycle-report`/`verification-report` (replaces `parse-and-advance`; reverses HIL Context-Protection) | tamper-proofing → any sound loop | **dedupe → `loop-discipline` Req 2 tamper-proof** (lead Reads the real artifact) (`=P1 L17 / D12`) |
| Overall loop guards | *four-guards doctrine* | **dedupe → `loop-discipline` Req 3** (`=P1 L11`) |
| Overall loop **params** — cap number, no-progress = unchanged failure set, `STOP` sentinel | only THIS workflow / its contract | **rehome-to-lead** (fills the workflow-contract artifact) |
| Done-condition | *default-FAIL mechanics + "how proven"* | **dedupe → `loop-discipline` Req 1** (`=P1 L2 = P7 L-e`) |
| Done-condition **end-state params** — all cycles complete; final-validation `ready`; all `tasks.md` `[x]`; implementation-complete | only THIS workflow | **rehome-to-lead** + the filled **workflow-contract artifact** (`=P1 L2 = P7 L-e`; DAG gate/milestone labels drop) |
| Design-input loading — `tasks.md`, `plan.md`, `task-mapping.md`, `data-model.md`, `contracts/`, `constraints-and-decisions.md` | *brief agents with files* → doctrine | **dedupe → `agent-dispatch`** (briefing) + paths **rehome-to-lead** (kept-but-rebind → `.mochiko/specs/<feature>/`) (`=P1 L4 / R1`) |
| `$ARGUMENTS` capture + empty-input recovery | only THIS workflow entry | **rehome-to-lead** (`=P1 L1`) |
| Mid-loop clarification gate — collect input when staff flags ambiguity | *preference-gap routing* = doctrine; the gate = workflow | **dedupe → `loop-discipline` (gap routing)** + placement **rehome-to-lead** (`=P1 L12`) |
| Completion / finalize — status → complete; summary (cycle/task/fix-pass counts, quality gates, artifacts, next steps) | only THIS workflow | **rehome-to-lead** (`=P1 L15`) |
| Dispatch-via-Task; do-not-modify-git | *dispatch* = doctrine; the git invariant = lead | **dedupe → `agent-dispatch`** (dispatch) + **rehome-to-lead** (git invariant) (`=P1 L16`) |
| `.workflow/context.md` state carrier | — | **absorbed-into-lead** — in-session + workspace-as-state under `.mochiko/specs/<feature>/`; no carried artifact (`=P1 A1`; 4× prior-run confirmed) |

#### Dropped (kernel-free; NOT rehomed — reasons go to the gated bundle)

- **P1 — 14 brain-plumbing drops** (DAG vocabulary; catalog path/assembly; context-template path; State-Analyst 4 verbs; `hil-dag` CLI/MCP; `check-prerequisites.sh`; DAG paths/`mkdir`; dispatch_mode routing; INV-001…005; carry_forward; pass lifecycle; Context Protection *device*; State-Analyst boundaries; `re-brief` override).
- **P7 — 5 framing drops** (frontmatter trigger/briefing `description`; Overview+When-to-Use consumption framing; When-NOT-to-Use sibling/DAG routing; DAG vocabulary *labels*; advisory/"may-deviate" stance). Substance under the labels survives at the dedupe/rehome rows above; only the labels drop.
- **P2 — 5 drops** (output-location "context.md" ref L64; Cycle-Mode label + injection; Fix-Mode label/trigger; context-file mechanism in 3 examples; "Supervisor dispatches" + mode/trigger naming — examples reframed to work-context).
- **P4 — 1 drop** ("Modifying DAG state or pass lifecycle — graph operations layer" — no mochiko referent; the role-neutral intent "this skill does not manage orchestration state" is preserved).

### 2.2 — Altitude confirmation

The lead **references** `loop-discipline` / `workflow-contract` / `agent-dispatch` for every `dedupe` row above and **fills a `workflow-contract` artifact** for the done-condition/params — it never restates the four rules, the validator tiers, gap-routing, or a filled contract inline. Only the `rehome-to-lead` rows (the a–g params + the two gate placements + the workflow-unique steps) become command-body prose. `implement` legitimately carries the most workflow-specific machinery of any command (entry gate + scaffolding + sequential cycle loop + per-cycle execute→verify + targeted retry + fix-pass + two agents + a multi-tier gate), so a modestly higher line count than `tasks` (77) / `plan` (82) may be genuine — `verify-output` item 8 adjudicates line-by-line (grep-floor = zero inlined contract / zero restated doctrine; keystone-ceiling = every line true only of THIS workflow), not by raw count.

---

## Seam 2 — Producer↔validator casting + independence

**Resolution: confirm + wire. No restructure — the pair already exists (the inverse-of-setup nuance, §2.0).**

| Role | Agent | Skill(s) | Team-role |
|------|-------|----------|-----------|
| **Producer** | `staff-engineer` (P2) | `executing-tdd-cycle` (P4) + `brownfield-integration` (P6) | writes code via TDD; emits `cycle-report.md`; **never grades** |
| **Validator** | `qa-engineer` (P3) | `testing-end-user` (P5) | verifies against real infra + quality-gate exit codes (Tier-1); emits verification report + checkpoint |

- **Independence — CONFIRMED CLEAN.** `staff` skills `{executing-tdd-cycle, brownfield-integration}` ∩ `qa` skill `{testing-end-user}` = ∅; **different agents.** This is already the mochiko-correct arrangement (contract §2).
- **HARD INDEPENDENCE RULE (carry into transform + verify):** `testing-end-user` / any `verify-*` / grading skill is **NEVER** mounted on `staff-engineer`. Produce + grade on one agent is wrong by construction. The mount for P5 lives on **P3 only**; P4 lives on **P2 only** — never co-mounted.
- **Verdict is lead-owned.** The **cycle-checkpoint** and **final-validation** verdicts are the lead's (it Reads `cycle-report`/`verification-report` + qa's evidence and decides against default-FAIL). Never a staff self-grade; never a dissolved-Analyst autonomous gate. qa's status is *input.*
- **Execute-then-verify pairing wired to the lead** (§Job 2 constraint (b), *pairing shape*): every staff cycle is followed by a qa verification in the same round, never skipped. Declared by the lead's casting, **not** in either persona (neither names the other — grep-confirmed clean in both assessments).

---

## Seam 3 — Fix-mode split (P2), line-by-line

Resolves FR-3 / P2-flag / P4-seam jointly. The split is drawn **once**: intrinsic craft stays in the persona; routing/trigger/bounds rehome to the lead; procedure is canonical in P4; the bounded backbone dedupes to `loop-discipline`.

**STAYS as intrinsic craft in the `staff-engineer` persona (kept):**
- L77 the scope-discipline *disposition* — "stay tightly scoped to the work you're given" (drop the "in your instructions" injection + "the cycle's scope" workflow unit).
- L81 the fix *craft* — "scope to the specific failures; **reproduce the bug with a failing test before fixing it**; follow the failure wherever it leads, not a refactor opportunity."
- The retry *disposition* — targeted minimal rework; don't re-implement passing code.

**REHOMES to the lead as routing/trigger/bounds (`rehome-to-lead`):**
- L73 "## Two Execution Modes" — the mode-skeleton itself: *that there are two modes and WHEN to enter each* is the lead's briefing decision, not a persona self-selection.
- L75 "### Cycle Mode (Normal)" and L79 "### Fix Mode **(After Validation Failure)**" — the mode-labels **and the trigger**.
- L81 "**Unconstrained by cycle boundaries** … may touch files from any cycle" — the routing decision the lead makes when it briefs a fix-scoped run.
- The **max 3 fix passes** bound (constraint (d)) and **max 3 retries/cycle** bound (constraint (c)).

**DEDUPES to P4 `executing-tdd-cycle` (procedure):** the fix/retry HOW (trace→re-open→TDD-only-reopened→`cycle: fix` / updated-`attempt` report). **DEDUPES to `loop-discipline`:** the bounded-iteration backbone (Req 3).

**HARD CONSTRAINT (audited by `verify-output`'s decoupling + keystone scan):** the persona must **NOT** name the trigger ("After Validation Failure") **or** the max-passes bound. The L44 skill-blurb "retry handling, fix mode, and cycle report generation" softens to describe the skill's *work*, not the workflow-mode. Zero residual deny-list token on the two named highest-risk surfaces (the mode-skeleton + the context.md examples).

---

## Seam 4 — Confidence-gate three-way split

Resolves FR-4 / P3-F2. All three pieces land; none is double-owned or dropped.

| Piece | Home | Trace tag | Detail |
|-------|------|-----------|--------|
| **Classification *procedure*** (the reclaimed Task-Classification table) | **P5 `testing-end-user`** (SKILL L111–122) | **kept** (canonical runtime home; = P1 S1 `moved-to-sibling-skill`) | CLI + measurable + 100%-pass ⇒ auto-approve; GUI/SUBJECTIVE/any-fail ⇒ human checkpoint; **default SUBJECTIVE** (safe fallback) |
| **Conservative *judgment/disposition*** | **P3 `qa-engineer`** persona | **kept** | default-to-human-when-uncertain; "ambiguity is never a reason to auto-approve — it's a reason to escalate"; unexpected output ⇒ checkpoint |
| **Gate *placement + final-acceptance wiring*** | **the lead** (P1 L13 + L14) | **rehome-to-lead** | where the gate sits; the auto-approve-vs-checkpoint routing; the named final-acceptance gate before "done" |
| *(placement doctrine)* | `loop-discipline` Req 4 | **dedupe** | the lead *references* "low validator-confidence only" placement; does not restate it |

Confirmed: the classification rule lives in exactly one place (P5), the disposition in exactly one (P3), the wiring in exactly one (lead). This is the procedure ÷ judgment ÷ placement seam — no piece leaks across.

---

## Seam 5 — Single-source dedupes vs `patterns-vertical-tdd` (BOTH grammars)

`patterns-vertical-tdd` (tasks cluster, ported) is confirmed the **canonical author/owner of both grammars** (Markers table SKILL.md L150–151 for `[EXTEND]`/`[MODIFY]`; SKILL.md L113/127/136/218/227/332 + `CYCLE-STRUCTURE.md` for `**TEST:**`; REGISTRY L102: "canonical home of the `**TEST:**` verification-task grammar"). Both are resolved as **dedupe / reference, NOT merge** — the skills are disjoint by **altitude** (design-time authoring vs runtime consumption) **and cluster** (tasks vs implement).

### (a) `**TEST:**` grammar in P5 `testing-end-user`

- **P5 REFERENCES (dedupe → `CYCLE-STRUCTURE.md`, the legal set):**
  - SKILL.md L50–55 — the supported-marker list (`**TEST:**` / `TEST:VERIFY` / `TEST:CONTRACT` / `HUMAN VERIFICATION`).
  - SKILL.md L81–86 — the action-modifier *catalog* (`(background)` / `(timeout Ns)` / `(in path)`).
  - SKILL.md L93–99 — the assert-pattern *catalog* (`Console contains` / `File exists` / `Response status`).
  - `TASK-PARSING.md` field-marker + modifier + assert *tables* (the same legal set re-encoded as regexes) → point at the canonical set.
- **P5 RETAINS (execution/parse semantics — its canonical half):** how to *detect* a TEST task and *extract* its fields; how to *run* a `(background)` action + track its PID and enforce `(timeout Ns)`; how to *evaluate* `Console contains` against captured output; sequential fail-fast setup; evidence capture; result classification (PASS/FAIL/PARTIAL/TIMEOUT/ERROR); the **runtime CLI/GUI/SUBJECTIVE classification** (P5's own — the §Seam 4 reclaim, not patterns-vertical-tdd's); report generation; checkpoint presentation; anti-rationalization.
- **Reciprocal (minor, low-priority):** `CYCLE-STRUCTURE.md` L250–254 reproduces the CLI/GUI/SUBJECTIVE table "for context" though it cedes ownership to P5. Slim it to a pointer back to P5 when patterns-vertical-tdd is next touched — **non-blocking** (direction already correct; drift risk low). Not required for this cluster's PASS.
- **Boundary integrity holds:** `grep cycle-report` in `patterns-vertical-tdd` stays **0**; the `**TEST:**` grammar stays *owned* by `patterns-vertical-tdd`, *consumed* by P5 (contract §Boundary-integrity satisfied).

### (b) `[EXTEND]`/`[MODIFY]` marker grammar in P6 `brownfield-integration`

- **P6 REFERENCES (dedupe → `patterns-vertical-tdd` Markers table):** the marker **vocabulary definition** — that `[EXTEND]`/`[MODIFY]` exist + their one-line gloss (P6's "EXTEND vs. MODIFY Semantics" table's *vocabulary* line: EXTEND = "add new code alongside existing", MODIFY = "change existing behavior").
- **P6 RETAINS (implement-time consumption semantics — the half patterns-vertical-tdd does NOT carry):** the **interface-impact rules** per marker; the 5-step read-before-write checklist (full file, naming, error-handling, imports, tests); interface preservation (no signature/export/name changes; add alongside); conflict detection (name / import / test-file alignment / circular deps); the "flag, don't silently resolve" blocker-surfacing discipline; the code-modification-scope anti-rationalization spine.

Direct analog of the P4 boundary-integrity clause, extended to the marker grammar: design-time *tagging* (patterns-vertical-tdd) vs implement-time *consumption/interface-impact* (P6).

---

## Seam 6 (open question) — Report-format homing

**Question:** do the `cycle-report.md` format (P4 `references/CYCLE-REPORT-FORMAT.md`) and the verification-report format (P5 `references/REPORT-TEMPLATES.md`) stay in skill `references/`, or extract to `plugins/mochiko/templates/`?

**RECOMMENDATION: KEEP both in their skill `references/`. Do NOT extract to `templates/`.** (Human-gate item — the lead presents this call.)

**Justification:**
1. **These are the skills' own runtime-artifact formats, tightly coupled to execution semantics** — not a hand-off contract layered on top. The `cycle-report` frontmatter schema (`attempt` increments-on-retry, `cycle: "fix"`) *is* part of the retry/fix execution the skill performs; the verification-report's minimal/rich/PARTIAL/TIMEOUT/ERROR variants + `quality_gates` YAML *are* the shape of the result-classification + evidence-capture the skill performs. Extracting the format from the skill that produces it splits format from how-to — the exact drift hazard single-sourcing guards against, here in reverse.
2. **This is the opposite of the plan/tasks precedent.** In `plan`/`tasks`, the extracted-to-`templates/` reports (`techanalyst-report-template`, `taskarchitect-report-template`) are **separate producer self-disclosure reports layered on top of** the deliverable (`plan.md` / `tasks.md`) — lead-facing hand-off contracts with a deliberately dropped verdict field. The implement reports are different in kind: the `cycle-report` **is** staff's primary self-disclosure output (there is no separate "staff-engineer report" on top of it), and the verification-report **is** qa's evidence output (ephemeral — REPORT-TEMPLATES.md L316: "not persisted to disk by default").
3. **`templates/` in mochiko houses lead-filled deliverables** (`plan-template`, `tasks-template`, `constitution-template`) and lead-facing report contracts. The cycle-report/verification-report are **agent-runtime outputs**, not lead-filled deliverables — their home is with the skill.
4. **The lead Reads these as verdict input (§Job 2, L17) but does not own their format.** Reading ≠ owning. The lead references them; the producing skill owns the schema.
5. **Minimalism governs.** No workflow needs either format shared across skills today. If a future workflow does, *then* extract — do not pre-extract. Keeping both in skill references also keeps the two report formats **symmetric** and co-located with the execution that emits them.

---

## New primitives required — NONE (stated explicitly)

- **No new agent.** The producer↔validator pair (`staff-engineer` ↔ `qa-engineer`) already exists and is disjoint (§Seam 2). The dissolving State-Analyst does **not** become a new agent — its implement-slice is `absorbed-into-lead`.
- **No new skill.** P7 `strategy-implementation` **drops** with **no residual skill** — its survivors `rehome-to-lead` (already in P1's build) and its doctrine `dedupe`s into the existing `loop-discipline`.
- **No `loop-discipline` edit.** Nothing is genuinely-universal-but-missing (the family's one universal gap, Gap Classification, was folded in during specify). This is the cleanest strategy dissolution: zero additions, zero residual.

**The thin `implement` lead absorbs the orchestration.** That is the whole structural move.

---

## Job 3 — Re-emitted per-primitive traces (relational tags assigned)

Every responsibility carries a realized relational tag; nothing is orphaned. Phase 3 targets these.

### P1 `implement` command — `redesign × absorb-into-lead`
- 14 brain-plumbing responsibilities → **dropped + reason** (kernel-free; reasons → gated bundle).
- 6 generic loop responsibilities → **dedupe → `loop-discipline`/`agent-dispatch`** (DD1–DD6; the lead references, never restates).
- 18 workflow-specific orchestration responsibilities → **moved-to-lead** (L0–L17; the a–g params + both gate placements + phases; see §Job 2 for the dedupe/rehome split within each).
- 3 paths → **kept-but-rebind** (→ `.mochiko/specs/<feature>/`; entry-gate evidence; scaffolding outputs).
- 1 state carrier → **absorbed-into-lead** (A1).
- 1 classification reclaim → **moved-to-sibling-skill** (S1 `**TEST:**` runtime classification → **P5**; the command *consumes* it to drive the confidence gate → **paired-with P5**).
- **Relational:** **paired-with** P2 (producer) + P3 (validator); **absorbs** State-Analyst implement-slice + `.workflow/context.md`; **merges** P7's 6 survivors (no double-home).

### P2 `staff-engineer` — `port-with-edits × standalone`
- Persona/judgment, output menu, Quality Standards (taste), Reject/Embrace, honest cycle-report self-disclosure, PRODUCER role, frontmatter → **kept**.
- TDD procedure (P4), brownfield procedure (P6) → **kept-but-rebind** (`mochiko:`; **stays mounted**; **deduped-to** P4/P6 for literal procedure).
- Fix-Mode framing → **split**: craft **kept**; routing/trigger/mode-label/max-passes **moved-to-lead**; procedure **dedupe → P4** (§Seam 3).
- Cycle-Mode label + injection, output-location context.md ref, context-file mechanism (3 examples), "Supervisor dispatches" + mode/trigger naming → **dropped + reason** (reframed to work-context).
- **Relational:** **paired-with** P3 (independent validator); **deduped-to** P4/P6 (procedure); **HARD RULE** — never mount a grading skill here.

### P3 `qa-engineer` — `port-with-edits × standalone`
- Tier-1 validator craft (evidence-first, reproducibility, honesty, completeness, distrust-inferred, **conservative disposition**, escalate-ambiguous, human-oversight-as-final-gate, real-infra-first, deterministic quality-gate verdict, output menu, reporting judgment) → **kept**.
- Parse/execute `**TEST:**` + evidence, **task-classification procedure**, quality-gate mechanics → **folded-into-skill (P5)** (persona references, does not restate).
- `skills: testing-end-user` → **kept-but-rebind** (→ ported P5); frontmatter → **kept**.
- "cycle" tokens (desc L5, ex-3 L26/L27) → **kept-but-rebind** (decouple the "cycle" binding; gating responsibility survives as craft).
- Execute→verify pairing, FAIL-loop driving, gate placement → **moved-to-lead** (P1).
- **Relational:** **paired-with** P2 (producer); conservative judgment is the §Seam 4 middle piece; carries no loop-driving. **Dropped: none.**

### P4 `executing-tdd-cycle` — `port-with-edits × standalone`
- Cycle execution sequence, red/green/refactor mechanics, runtime task-parsing, `cycle-report.md` **format**, anti-rationalization (execution-scoped), retry HOW-craft, fix HOW-craft, `[EXTEND]`/`[MODIFY]` runtime handling + P6 invocation, shared discipline banner → **kept** (retry/fix HOW decoupled from "dispatched" framing).
- cycle-report consumer refs (`checkpoint gate`/`State Analyst`/`carry_forward`), feature-dir artifacts, description triggers → **kept-but-rebind** (consumers → lead + qa; paths → `.mochiko/specs/<feature>/`; triggers → work-context, de-collided from patterns-vertical-tdd).
- "DAG state / pass lifecycle" line → **dropped + reason**; "analysis layer" exclusion → **kept-but-rebind** (lead reads reports; no State-Analyst).
- agent names, "dispatched" → **kept-but-rebind / decouple** (role-neutral).
- retry/fix routing (max-3, when-retry-vs-fix, max-3-fix, stall) → **dedupe → `loop-discipline`** (backbone) + **moved-to-lead** (params) — confirmation that routing stays out of P4, not an extraction.
- **Relational:** **paired-with** P5 (opposite runtime side, disjoint — §Job 1.3); **deduped-to** `patterns-vertical-tdd` (shared discipline referenced, boundary holds).

### P5 `testing-end-user` — `port-with-edits × standalone`
- Detection/parsing, setup/action/assert execution, evidence capture, **CLI/GUI/SUBJECTIVE runtime classification (the reclaim)**, result classification, **quality-gate deterministic auto-resolution (Tier-1, un-diluted)**, report generation, checkpoint presentation, anti-rationalization, legacy normalization, test provenance → **kept**.
- `**TEST:**` grammar catalog (marker list, action-modifier catalog, assert-pattern catalog, field tables) → **dedupe → `patterns-vertical-tdd`** (reference the legal set; keep execution/parse) (§Seam 5a).
- evidence-path filenames embedding `qa-engineer` → **kept-but-rebind** (role-neutral / mochiko scratch); "dispatched … cycle" + "cycle-checkpoint gate" phrasing → **kept-but-rebind** (role-neutral); the dispatch/pairing → **moved-to-lead** (constraint (b)); loop-on-reject (targeted retry) → **moved-to-lead** (constraint (c)).
- **Relational:** **paired-with** P3 (mounts on qa **only**); **deduped-to** `patterns-vertical-tdd` (grammar). **Dropped: none.**

### P6 `brownfield-integration` — `port-with-edits × standalone`
- EXTEND/MODIFY consumption semantics (interface-impact), read-before-write checklist, interface preservation, conflict detection, "flag don't silently resolve", anti-rationalization spine, when-to/when-NOT scoping → **kept**.
- `[EXTEND]`/`[MODIFY]` marker vocabulary definition → **dedupe → `patterns-vertical-tdd`** (canonical owner; cross-cluster single-source — NOT a merge) (§Seam 5b).
- "the cycle report" flag-sink (×5), soft workflow-phase vocabulary (`cycle execution`, `previous cycles`) → **kept-but-rebind** (soften to intrinsic craft; cycle-report bound caller-side to P4).
- model-invocation triggers, classification + router → **kept-but-rebind** (re-anchor "when the user says…" → work-context) + wiring-pass.
- **Relational:** **deduped-to** `patterns-vertical-tdd` (marker vocabulary); **home corrected** setup→implement (REGISTRY, §Phase-5 delta). **Dropped: none.**

### P7 `strategy-implementation` — `drop` (realized: `absorb-into-lead` + `dedupe`, no residual)
- DD1–DD4 (execute-then-verify principle; iterate-on-FAIL backbone; escalate-before-stall doctrine; convergence signals) → **dedupe → `loop-discipline`** (Req 2 / 3 / 3.4+4 / 3.2; all present, no edit).
- L-a…L-f (cycle sequencing; execute→verify pairing shape; targeted-retry tactic + max-3/cycle; fix-pass scoping + max-3; done-condition content; ordering guard) → **moved-to-lead**, each **MERGED into its P1 twin** (`=P1 L6/L7/L8/L9/L2`) — landed once (FR-B).
- D1–D5 (frontmatter trigger/briefing desc; consumption framing; sibling/DAG routing; DAG vocabulary labels; advisory stance) → **dropped + reason**.
- **Relational:** **deduped-to** `loop-discipline`; survivors **rehomed-to** P1 lead (merged, not double-homed); **family closed** (FR-C). **The standalone skill does not exist in mochiko.**

---

## Finalized dispositions

| Primitive | Body × Structural | Open flags |
|-----------|-------------------|-----------|
| P1 `implement` command | **`redesign × absorb-into-lead`** → thin sound-loop; absorbs State-Analyst slice + context.md carrier; merges P7 survivors | none |
| P2 `staff-engineer` | **`port-with-edits × standalone`** (fix-mode split resolved: craft kept, routing→lead) | none |
| P3 `qa-engineer` | **`port-with-edits × standalone`** (confirm+wire; 3 "cycle" decouples) | none |
| P4 `executing-tdd-cycle` | **`port-with-edits × standalone`** (HOW-craft kept/decoupled; routing→lead; grammar boundary holds) | none |
| P5 `testing-end-user` | **`port-with-edits × standalone`** (`**TEST:**` grammar → reference; classification reclaim kept) | none |
| P6 `brownfield-integration` | **`port-with-edits × standalone`** (marker vocab → reference; home corrected → implement) | none |
| P7 `strategy-implementation` | **`drop`** → realized `absorb-into-lead` (survivors→P1) + `dedupe` (doctrine→loop-discipline); no residual skill, no loop-discipline edit | none |

---

## Phase-5 REGISTRY delta

**Primary (task-specified) — `brownfield-integration` re-file:**
- **REMOVE** from `### Setup workflow cluster` (currently L80): `[ ] brownfield-integration — Deferred — out of setup-core scope (REGISTRY mis-file; reads as implement-time)`.
- **ADD** to `### Implement / execute cluster` (alongside `executing-tdd-cycle`, `testing-end-user`): `[x] brownfield-integration — Ported (implement, 2026-07-01) — staff-engineer's 2nd skill (EXTEND/MODIFY consumption + read-before-write + interface preservation + conflict detection); REGISTRY mis-file corrected setup→implement; [EXTEND]/[MODIFY] marker vocabulary deduped → patterns-vertical-tdd (canonical owner), keeps implement-time consumption/interface-impact semantics.` Mark `[x]` when the artifact lands in Phase 3.

**Companion deltas that also land in Phase 5 (noted so the row-set is complete; not the task's focus):**
- L38 `[ ] implement` command → `[x]` (thin sequential sound-loop; confidence-based+final gate).
- L54 `[ ] staff-engineer` → `[x]`; L55 `[ ] qa-engineer` → `[x]`.
- L107 `[ ] executing-tdd-cycle` → `[x]`; L110 `[ ] testing-end-user` → `[x]`.
- L68 `[ ] strategy-implementation` → `[-]` **Dissolved** — into `loop-discipline` (doctrine) + implement lead (params); strategy family closed (3rd/final; no strategy-* survives).
- L141 `[ ] approved-domain-deps.md` → **stays `[ ]`** (deferred / reference-stub; scope=core-only).

---

## Gated-disposition bundle for the human gate (contract §4a) — enumerated

The lead presents these before any Phase-3 edit. The two heaviest calls were pre-answered at intake (sequential; confidence-based+final), so the gate **confirms correct realization** rather than re-litigating.

1. **The command redesign + the rehome map (P1 `redesign × absorb-into-lead`)** — the largest orchestration rehome of any port. Constraints (a)–(g) + both gate placements land **once** on the thin lead, split dedupe-vs-rehome per §Job 2. **Confirm the two intake decisions are correctly realized:** sequential-only (no Workflow `pipeline()`/`parallel()`, no artifact-DAG); confidence-based + named final-acceptance gate.
2. **`strategy-implementation` dissolution (P7 `drop`)** — accept the **DROP of the standalone skill**; survivors merged into the lead (no double-home), doctrine deduped into `loop-discipline` (no edit); strategy family closed.
3. **`staff-engineer` fix-mode decoupling (P2)** — accept the split: intrinsic craft kept in the persona; the two-execution-modes routing/trigger/max-passes rehomed to the lead; **the persona must not name the trigger or the bound** (§Seam 3).
4. **Absorbs** — the dissolved **State-Analyst implement-slice** and the **`.workflow/context.md` carrier** are absorbed into the lead (in-session + workspace-as-state); no new agent, no carried artifact.
5. **Producer↔validator casting** — confirm `staff` produces ≠ `qa` grades (disjoint skills, different agents); **verdict is lead-owned** (cycle-checkpoint + final-validation); `testing-end-user`/`verify-*` **never** mounted on staff.
6. **Report-format call (§Seam 6)** — recommendation: **keep** `cycle-report` (P4) + verification-report (P5) formats in skill `references/`; do **not** extract to `templates/` (runtime-artifact formats coupled to execution; unlike plan/tasks producer self-disclosure). Human confirms.
7. **`approved-domain-deps` disposition** — deferred / reference-stub (scope=core-only).
8. **Parallelism deferral** — sequential-first is a documented `deliberate-shortcut-ledger` entry (parallel + kernel deferred pending dogfooding), **not a silent drop**. Human accepts the deferral.
9. **All dropped-responsibility reasons to accept** (kernel-free; every drop is auditable):
   - **P1 — 14** brain-plumbing drops (DAG vocabulary; catalog path/assembly; context-template path; State-Analyst 4 verbs; `hil-dag` CLI/MCP; `check-prerequisites.sh`; DAG paths/`mkdir`; dispatch_mode routing; INV-001…005; carry_forward; pass lifecycle; Context-Protection device; State-Analyst boundaries; `re-brief` override).
   - **P2 — 5** drops (output-location "context.md" ref; Cycle-Mode label + injection; Fix-Mode label/trigger; context-file mechanism in 3 examples; "Supervisor dispatches" + mode/trigger naming).
   - **P4 — 1** drop ("DAG state / pass lifecycle" line — role-neutral intent preserved).
   - **P7 — 5** framing drops (frontmatter trigger/briefing desc; consumption framing; sibling/DAG routing; DAG vocabulary labels; advisory/"may-deviate" stance).
   - **P3, P5, P6 — 0** drops (their "cycle"/grammar items are rebind/dedupe, not drops — capability survives).

---

## Open flags

**NONE.** All 20 ledger rows resolved; every orphaned/dropped responsibility has a landing place; every P7 survivor is merged into its P1 twin. Reconcile's done-condition is met.

**Reconcile version:** v1 · **Governed by:** `reconcile-cluster` · **Next:** Phase-3 `transform-recipes` applies these finalized dispositions + the rehome map; Phase-4 `verify-output` grades independently.
