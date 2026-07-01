# Assessment — P1 `implement` command

**Run:** `transform-cluster implement` · **Primitive:** P1 `commands/implement.md` (303 ln) · **Assessed:** 2026-07-01
**Producer:** `mochiko:transform-producer` · **Skill:** `assess-primitive` (branch → IS-a-loop)

> The pivotal, heaviest assessment of the run. HIL `/implement` is the only fully DAG-mediated
> workflow: a **hollow Supervisor** that delegates its entire orchestration brain to the
> State-Analyst over a `hil-dag` MCP kernel + `implement-catalog.json` DAG. The real work of this
> assessment is a **complete responsibility trace that reaches into the delegated brain** — the
> command body alone under-counts what this command is responsible for, because most of its
> orchestration lives in the State-Analyst's implement-specific slice.

---

## Output (skill format)

```
ASSESSMENT: implement command
Class:        command → branch IS-a-loop
Triage:       gate1=YES gate2=YES gate3=n/a  → full-lens (gates 1 & 2 trip emphatically)
Disposition:  redesign × absorb-into-lead → thin sound-loop command
              (structural verdict decidable solo; the rehome-map SPECIFICS are flagged FR-1..FR-4)
Reconcile flags: FR-1 (P7 strategy-implementation + dissolved State-Analyst slice — the (a)-(g) rehome split)
                 FR-2 (P2 staff-engineer / P3 qa-engineer — producer/validator casting + lead-owned verdict)
                 FR-3 (P2 staff-engineer — fix-mode workflow-routing vs. intrinsic fix craft split)
                 FR-4 (P5 testing-end-user — confidence-based gate wiring: classification lives in P5)
```

---

## Step 1 — Class & branch

**Class:** `command` → **branch IS-a-loop.** This IS a loop, not a role-player. What carries weight
(per the branch): who drives the loop, the done-condition, where validation + human gates sit, and —
decisively for this port — **which responsibilities are generic loop-discipline (→ `dedupe`) vs.
workflow-specific (→ `moved-to-lead`)**. The altitude rule is the spine of this assessment: it is
what keeps the ported command thin instead of re-inlining the shed kernel as prose.

## Step 2 — Fast-path triage gate

| Gate | Answer | Evidence |
|------|--------|----------|
| 1. Orchestration-coupled? | **YES (maximal)** | Depends on a `hil-dag` MCP kernel, a catalog JSON DAG, a markdown State-Analyst agent, AND a `check-prerequisites.sh` script — the fullest coupling stack of any primitive in the migration. |
| 2. Multi-responsibility / fans out? | **YES** | Holds entry-gate, scaffolding, cycle sequencing, execute→verify pairing, targeted retry, fix-pass, convergence detection, completion; fans out to staff-engineer, qa-engineer, State-Analyst. |
| 3. Emits a non-machine-checkable artifact? | **n/a** | A command is not an artifact-emitter; it drives the loop. (The workflow it drives produces code graded by qa — partly Tier-1 deterministic, partly judgment. That is P2–P5's concern.) |

Gates 1 & 2 trip emphatically → **full 7-check lens.**

---

## Step 3 — The lens (weighted for IS-a-loop)

### Check 1 — Orchestration test
**Layer:** a **DAG/MCP kernel** (`hil-dag` + `implement-catalog.json`) mediated by a **markdown
Supervisor** (this command) that **delegates its whole brain to a markdown State-Analyst agent**.
This is the deepest orchestration coupling in the run: kernel + supervisor + delegated analyst.

- **Content-coupling** (body references the mechanism): DAG vocabulary (4 node / 6 edge types, pass
  lifecycle, gate-verdict-as-status), catalog + context-template paths, the "Two Outbound Verbs",
  `hil-dag` CLI, State-Analyst delegation verbs, INV-002/004 references, `check-prerequisites.sh`,
  `.workflow/context.md` + `.workflow/dags/` paths, carry_forward mechanics.
- **Orchestration-coupling:** the command IS the orchestrator, but a **hollow** one — it cannot
  function without the State-Analyst, which owns all DAG mechanics, briefings, report-parsing,
  recommendations, and gate evaluation. **The command's real orchestration responsibilities are
  delegated into the State-Analyst's implement-specific slice** (Briefing Rule 9 cycle-awareness;
  the `cycle-report`/`verification-report` parsing patterns; the `execute-cycle`/`verify-cycle` NL
  prompt construction; the deterministic evaluation of `cycle-checkpoint`/`final-validation`). The
  trace MUST reclaim these, or the port silently drops the command's actual job. → **FR-1.**

**When the layer dissolves:** kernel/DAG/catalog/`hil-dag`/invariants → `dropped` (kernel-free);
the State-Analyst's *workflow-specific* intelligence → `moved-to-lead`; its *generic* loop mechanics
→ `dedupe`→`loop-discipline`; the `.workflow/context.md` carrier → `absorbed-into-lead`.

### Check 2 — Role at two altitudes
Not a skill; as a workflow it is the **loop-driver**. The team-role it confers on its runner is
**lead / supervisor / referee** — the redesigned command becomes the thin lead that owns the loop,
the verdict, and the human gates (exactly the tasks/plan shape). It emits no reviewable artifact of
its own; the code + reports are produced by staff/qa.

### Check 3 — Independence (the strong case — unusual for this migration)
Unlike setup/specify (which **lacked** independent validation and reconcile had to *add* it),
implement **already has it, structurally**: **staff-engineer produces** (`execute-cycle`, TDD) and
a **different agent, qa-engineer, grades** (`verify-cycle`, TEST: tasks + quality gates) — a
**Tier-1 deterministic validator** (real infrastructure, captured evidence, quality-gate exit
codes). Reconcile's job here is therefore **not** "add the missing validation gate" but "preserve
the strong pairing, cast it correctly, and move verdict-ownership from the dissolved Analyst onto
the lead." The one hazard: HIL's `cycle-checkpoint`/`final-validation` verdicts are evaluated
*autonomously by the State-Analyst*; on dissolution the lead must own them, and the `final-validation`
verdict must never collapse into a staff self-grade. → **FR-2.**

### Check 4 — Verdict-sink / loop-driver (the biggest thing the kernel owned)
Today the **State-Analyst** parses reports and computes `advance.action_taken`; the Supervisor
routes on it. The FAIL-loops: `cycle-checkpoint` needs-revision → **targeted retry** (max 3/cycle,
INV-004); `final-validation` needs-revision → **fix pass** (staff fix-mode, max 3, INV-004);
convergence stall (same failure 2+ passes) → surface; 3 retries → surface. On shed, the
**verdict-ownership + FAIL-loop-driving** must rehome onto the thin lead's bounded loop. The
altitude split is the crux:
- **Generic (→ `dedupe`):** that a loop iterates on FAIL, is bounded by a deterministic cap, exits
  on no-progress, escalates rather than dies; that validation is independent; default-FAIL.
- **Workflow-specific (→ `moved-to-lead`):** the *shapes/parameters* — targeted retry scoped to
  failed tasks (cap 3/cycle), fix-pass scoped to reported failures (cap 3), convergence-stall =
  same failure pattern 2+ rounds, execute→verify pairing every round, foundation-before-feature
  sequencing, the confidence-based+final gate placement.

### Check 5 — Sibling / overlap ("look sideways")
- **strategy-implementation (P7)** — IS the cycle-sequencing / execute-then-verify / targeted-retry
  / fix-pass-scoping / escalate-before-stall doctrine the command's Lifecycle Rules 5/6/8/9 lean on
  (via the State-Analyst that mounts P7). P7 dissolves into `loop-discipline`; its workflow-specific
  survivors rehome to the lead — and **this command is the primary consumer of those survivors**.
  The dedupe-vs-lead split cannot be decided until P7 is assessed. → **FR-1.**
- **dissolved State-Analyst** — the command's delegated brain (Check 1). Its implement-slice must be
  reclaimed onto the lead; the split overlaps P7's doctrine and P2/P3's casting. → **FR-1** (+ FR-2/FR-3).
- **staff-engineer (P2)** — HIL fix-pass routing lives partly here (Rule 9) and partly in
  staff-engineer's **"Two Execution Modes / Cycle Mode / Fix Mode"** persona (contract §1's
  highest-risk decoupling surface). The split — fix *routing* → lead; fix *craft* → staff — is
  relational. → **FR-3.**
- **qa-engineer (P3)** — the independent validator; casting + independence (qa must not mount
  staff's skills) is a cluster decision. → **FR-2.**
- **testing-end-user (P5)** — the confidence-based gate the command names (auto-approve vs.
  checkpoint) is *driven by* P5's reclaimed `**TEST:**` runtime classification (CLI/GUI/SUBJECTIVE).
  The gate placement lives on the lead; the classification lives in P5. → **FR-4.**

### Check 6 — Coupling audit
**Paths that break on port:**
| HIL path | Fate |
|----------|------|
| `${CLAUDE_PLUGIN_ROOT}/catalogs/implement-catalog.json` | `dropped` (catalog shed) |
| `${CLAUDE_PLUGIN_ROOT}/templates/context-template.md` | `dropped` (carrier absorbed by prior runs) |
| `${CLAUDE_PLUGIN_ROOT}/scripts/check-prerequisites.sh` | `dropped` (brain script `[-]`) |
| `$PROJECT_ROOT/specs/{id}/.workflow/dags/implement-strategy.json` | `dropped` (DAG shed) |
| `{FEATURE_DIR}/.workflow/tasks-context.md` (frontmatter `status`) | `kept-but-rebind` → entry gate reads **workspace evidence** (`.mochiko/specs/<feature>/tasks.md` present/complete), no context-file status |
| `{feature_dir}/.workflow/context.md` | `absorbed-into-lead` (in-session + workspace-as-state) |
| `tasks.md · plan.md · task-mapping.md · data-model.md · contracts/ · constraints-and-decisions.md` | `kept-but-rebind` → `.mochiko/specs/<feature>/` |
| `.gitignore` / `.dockerignore` / lint-ignore | `kept` (scaffolding outputs, project-relative) |

**Prerequisite/handoff:** upstream is the **tasks** workflow (entry gate); inputs are the plan+tasks
design outputs. Rebound to workspace-as-state (matching tasks' plan-complete gate, plan's spec-complete gate).

**Determinism boundary (this drives the confidence-based gate = intake decision #2):**
- *Deterministic (Tier-1 assert, qa-owned):* quality gates (lint/build/test exit codes),
  TEST:VERIFY/TEST:CONTRACT CLI checks, entry-gate file existence, `all tasks [x]`.
- *Model judgment (needs lead + human gate):* TDD quality, TEST:GUI/SUBJECTIVE, checkpoint
  "all-complete" call, final-validation traceability + constitution alignment, retry failure-tracing,
  fix-pass scoping. The gate maps exactly to this line: CLI 100%-pass → auto-approve; GUI/subjective/
  any-failure → checkpoint.

### Check 7 — Conventions + loop placement
- **Classification:** user-invoked → add `disable-model-invocation: true` (HIL has only `description`).
- **Discoverability:** register in the `mochiko` router (user-invoked = hinted).
- **Model-invocation reliability:** n/a (user-invoked command).
- **Decoupling:** the command body legitimately names agents and "dispatches" — the deny-list targets
  *personas/skills*, not the lead. The command's altitude risk is **restated doctrine**, not coupling
  tokens: it must *reference* `loop-discipline`/`workflow-contract`/`agent-dispatch`, never re-inline
  the DAG vocabulary, lifecycle rules, or context-protection as prose.
- **Producer↔validator pairing:** present (staff ≠ qa, Tier-1), but must be re-expressed as the
  lead's explicit casting + lead-owned verdict → FR-2.
- **Sound-loop gaps the redesign must fill** (feed reconcile's rehome-orchestration):
  1. **default-FAIL done-condition + a filled `workflow-contract` artifact** — HIL routed on the gate
     *verdict field* and declared "no hard caps"; it could self-declare done at pass 1 (violates
     loop-discipline reqs 1 & 3). *(NEW)*
  2. **A deterministic overall round cap + no-progress exit** — HIL had per-cycle INV-004 but no
     mochiko overall round-cap/no-progress + STOP kill-switch. *(partial → complete)*
  3. **A named final-acceptance human gate** before "done." *(NEW)*
  4. The **confidence-based gate made explicit + named** (present but implicit via testing-end-user).

---

## Step 4 — Disposition

**`redesign × absorb-into-lead` → a thin sound-loop command.**

- **Body = `redesign`** (not `port-with-edits`): the body assumes a kernel/DAG/catalog/State-Analyst
  *throughout* — the two verbs, DAG vocabulary, 10 Lifecycle Rules, Context Protection are the
  mechanism itself, wrong for mochiko. Almost nothing survives as-is; carry the *responsibilities*,
  not the mechanism. Not `drop` (the responsibilities are load-bearing), not `keep-verbatim`.
- **Structural = `absorb-into-lead`** (decidable solo): pure orchestration, no reusable body →
  workflow-specific orchestration into the thin supervisor; *reference* (don't copy) the doctrine
  `loop-discipline`/`workflow-contract`/`agent-dispatch`; the State-Analyst + strategy-implementation
  + DAG dissolve, leaving no orphan skill. This is the tasks/plan pattern, at greater scale.

**Redesign target honors the two LOCKED intake decisions:**
1. **Sequential-only.** Cycles execute in dependency order (foundation-before-feature) under a thin
   prose lead with Task-subagent dispatch. **No** native-Workflow `pipeline()`/`parallel()`, **no**
   artifact-DAG. Parallelism is a documented `deliberate-shortcut-ledger` deferral, not a silent drop.
2. **Confidence-based + final-acceptance gate.** Deterministic CLI verifications that 100% pass
   auto-approve; GUI/subjective/any-failure always checkpoint; a named final acceptance gate before
   the workflow reports done.

**Why the structural verdict is emitted solo but the rehome *specifics* are flagged:**
`absorb-into-lead` is not a sibling-dependent move (unlike split/merge/promote/pair), so it is
decided here. But *which* of constraints (a)–(g) land on the lead vs. are already covered by
`loop-discipline` (dedupe), the producer/validator casting, the fix-mode split, and the
confidence-gate wiring are **cross-primitive** and belong to Phase-2 reconcile with full context.
→ FR-1..FR-4. The rehome map must be built ONCE, cluster-wide; this assessment does not
unilaterally assign shared orchestration.

---

## Step 5 — Responsibility trace (COMPLETE — 43 responsibilities, each in exactly one bucket)

> Enumerated by walking the command body top-to-bottom AND reaching into the State-Analyst's
> delegated implement-slice (Check 1). Every original orchestration responsibility carries a tag; no
> silent drops. `(a)`–`(g)` map to contract §1 constraints. `→ FR-n` marks cross-primitive overlap.

### Bucket (i) — `dropped + reason` · brain-plumbing that dissolves (kernel-free) — **14**
| # | Responsibility | Reason |
|---|----------------|--------|
| D1 | DAG vocabulary (4 node / 6 edge types; pass lifecycle; gate-verdict-as-status) | kernel-free; DAG shed |
| D2 | Catalog path + catalog-driven capability-based node assembly | `implement-catalog.json` `[-]` excluded |
| D3 | Context-template path | carrier absorbed by prior runs; no carried artifact |
| D4 | "Ask the Analyst" verb + the 4 State-Analyst actions (brief-and-assemble / parse-and-advance / update-and-advance / re-brief) | State-Analyst dissolves |
| D5 | `hil-dag` CLI/MCP operations + "zero direct CLI usage" discipline | MCP kernel shed |
| D6 | `check-prerequisites.sh --json --require-tasks` prerequisite script | brain script `[-]` excluded |
| D7 | DAG paths + `mkdir .workflow/dags` | DAG storage shed |
| D8 | dispatch_mode routing (agent/skill/supervisor-owned/auto-resolved) + node-type dispatch | DAG dispatch machinery shed |
| D9 | INV-001/002/003/004/005 references + assembly-time/runtime enforcement | invariant engine shed |
| D10 | carry_forward auto-resolution (Rule 10: tasks-complete auto-resolved, Supervisor never told) | DAG carry_forward plumbing |
| D11 | Pass lifecycle (create/execute/freeze; `freeze_and_new_pass`; `pass_number`) | pass engine shed |
| D12 | Context Protection ("NEVER read domain agent reports directly; all content via parse-and-advance") | brain-era token device; mochiko lead **Reads artifacts directly** — deliberate reversal, required by loop-discipline req 2 tamper-proofing |
| D13 | State-Analyst-side Responsibility-Boundaries (briefings / parsing / status / freezing / prompt-construction / invariant-validation via hil-dag) | State-Analyst dissolves |
| D14 | `re-brief` override mechanism (`override_recommendation`) | Analyst verb; the lead decides directly |

### Bucket (ii) — `dedupe` → `loop-discipline` / `agent-dispatch` · generic loop discipline (do NOT re-inline) — **6**
| # | Responsibility | Single-source |
|---|----------------|---------------|
| DD1 | default-FAIL done-condition *mechanics* + "how completion is proven" | `loop-discipline` req 1 |
| DD2 | produce→check→repeat *pattern* + generic routing (continue-next / loop-on-FAIL / done / escalate — Rules 1/2/3/7 mechanics) | `loop-discipline` |
| DD3 | the four iteration-guard *doctrines* — that a hard cap, no-progress exit, kill-switch, escalate-don't-die exist at all | `loop-discipline` req 3 |
| DD4 | independent-validation *principle* (self-report unreliable → grade with a different agent) | `loop-discipline` req 2 *(implement realizes it structurally via staff≠qa — see L0/L7)* |
| DD5 | gap-routing (knowledge→research, preference→human, scope→halt) | `loop-discipline` |
| DD6 | agent briefing / prompt-construction as caller-side context ("domain agents have NO workflow knowledge — all context via files") | `agent-dispatch` *(the decoupling principle, correct — not a defect)* |

### Bucket (iii) — `moved-to-lead` · workflow-specific orchestration (constraints a–g + gates + phases) — **18**
| # | Responsibility | Note |
|---|----------------|------|
| L0 | Lead role + ownership of the loop and the **clearing verdict** (lead Reads artifacts + qa verification, decides against default-FAIL; qa status is *input*) | → FR-2 (casting) |
| L1 | `$ARGUMENTS` capture + empty-input recovery (the `@`-reference drop bug) | G1-analogue |
| L2 | done-condition **end-state parameters** — all cycles complete, `final-validation` ready, all `tasks.md` `[x]`, implementation-complete | contract param |
| L3 | **(f)** entry gate: tasks-workflow-complete prerequisite | rebind → workspace evidence `tasks.md` present/complete |
| L4 | load design inputs (tasks.md, plan.md, task-mapping.md, data-model.md, contracts/, constraints-and-decisions.md) as producer inputs | briefed per `agent-dispatch` |
| L5 | **(g)** project scaffolding — ignore-file creation from detected stack | |
| L6 | **(a)** sequential cycle sequencing — foundation before feature; current cycle = first with unchecked tasks | → FR-1 (P7 + SA-slice) |
| L7 | **(b)** execute-then-verify pairing — every staff cycle followed by qa verification in the same round, never skipped | → FR-1 (P7), FR-2 (casting) |
| L8 | **(c)** targeted retry — trace checkpoint failure to its tasks, re-open only those, **max 3/cycle** | → FR-1 (P7) |
| L9 | **(d)** fix-pass mode — after final-validation failure: unconstrained by cycle boundaries, scoped strictly to reported failures, **max 3 passes** | → FR-1 (P7), FR-3 (P2 split) |
| L10 | **(e)** convergence-stall detection — same failure pattern 2+ rounds → surface, don't silently continue | implement parameterization of no-progress (doctrine dedupes at DD3); → FR-1 |
| L11 | overall-loop cap/no-progress **parameters** + kill-switch instantiation (lead counts; no-progress = unchanged failure set; STOP sentinel) | the mochiko round-cap HIL lacked at the overall level |
| L12 | mid-loop clarification gate (HIL `user-clarification` — collect input when staff flags ambiguity) | G3-analogue |
| L13 | **confidence-based human-gate placement** — CLI 100%-pass → auto-approve; GUI/subjective/any-failure → checkpoint | intake #2; → FR-4 (classification in P5) |
| L14 | **named final-acceptance human gate** before "done" | intake #2; NEW (HIL had none) |
| L15 | completion / finalize — status→completed; summary (cycle/task/fix-pass counts, quality gates, artifacts, next steps) | tasks/plan Phase-finalize analogue |
| L16 | dispatch via Task tool, never inline agent behavior; do-not-modify-git | kept lead invariants (present in tasks/plan footers) |
| L17 | **State-Analyst implement-slice reclaimed**: direct Read of `cycle-report`/`verification-report` as verdict input (replaces `parse-and-advance`); deterministic gate evaluation (cycle-checkpoint = criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + constitution alignment) as the lead's Tier-1 asserts feeding its verdict | → FR-1 / FR-2 |

### Bucket — `kept-but-rebind` · paths — **3**
| # | Responsibility |
|---|----------------|
| R1 | artifact input paths → `.mochiko/specs/<feature>/` (tasks/plan/task-mapping/data-model/contracts/constraints-and-decisions) |
| R2 | entry-gate evidence → `.mochiko/specs/<feature>/tasks.md` present/complete (was `{FEATURE_DIR}/.workflow/tasks-context.md` frontmatter status) |
| R3 | scaffolding output paths (.gitignore/.dockerignore/lint-ignore) — project-relative, kept |

### Bucket — `absorbed-into-lead` · state carrier — **1**
| # | Responsibility |
|---|----------------|
| A1 | `.workflow/context.md` creation + status updates → in-session lead state + workspace-as-state under `.mochiko/specs/<feature>/`; no carried artifact (confirmed by prior runs — not re-litigated) |

### Bucket — `moved-to-sibling-skill` · reclaim — **1**
| # | Responsibility |
|---|----------------|
| S1 | `**TEST:**` runtime task-classification (CLI/GUI/SUBJECTIVE → auto-approve vs. human-checkpoint) → reclaimed by **P5 testing-end-user** (already in its HIL body); the command *consumes* it to drive L13 | → FR-4 |

**Bucket counts:** dropped **14** · dedupe **6** · moved-to-lead **18** · kept-but-rebind **3** ·
absorbed-into-lead **1** · moved-to-sibling-skill **1** → **43 total, every responsibility tagged.**

---

## Reconcile flags (build the rehome map ONCE, cluster-wide, in Phase 2)

**FR-1 — the (a)–(g) rehome split [overlaps P7 `strategy-implementation` + the dissolved State-Analyst].**
Cycle-sequencing / execute-then-verify / targeted-retry / fix-pass-scoping / escalate-before-stall
doctrine lives in P7 and is executed by the State-Analyst's implement-slice. Reconcile must decide,
per constraint, the **`dedupe`→`loop-discipline` vs. `moved-to-lead`** split — the *doctrine* of caps/
no-progress/escalation dedupes; the *implement parameters* (foundation-before-feature, max 3/cycle,
max 3 fix passes, same-failure-pattern-2+-rounds, execute→verify-every-round) rehome to the lead.
This command is the primary consumer of the survivors; cannot resolve until P7 is assessed.

**FR-2 — producer/validator casting + lead-owned verdict [overlaps P2 `staff-engineer` + P3 `qa-engineer`].**
Assign: staff-engineer produces `execute-cycle`; qa-engineer (a *different* agent, not mounting
staff's skills) grades `verify-cycle`; the **lead owns** the cycle-checkpoint + final-validation
verdict (never a staff self-grade, never a dissolved-Analyst autonomous gate). Honor the intake-locked
sequential decision. The independence structure is load-bearing.

**FR-3 — fix-mode split [overlaps P2 `staff-engineer`].**
HIL fix-pass routing is split between the command (Rule 9) and staff-engineer's "Two Execution Modes"
persona (contract §1's highest-risk decoupling surface). Reconcile must split: fix-pass
**workflow-routing** (when to enter fix mode, the max-3 bound, scoping to reported failures) →
`moved-to-lead`; **intrinsic fix craft** (trace a failure to code, fix via TDD) → stays in
staff-engineer as ordinary debugging, decoupled. Cannot decide until P2 is assessed.

**FR-4 — confidence-based gate wiring [overlaps P5 `testing-end-user`].**
The designed-in gate (auto-approve vs. checkpoint) is *driven by* P5's reclaimed `**TEST:**` runtime
classification (CLI/GUI/SUBJECTIVE). Place consistently: **classification** lives in testing-end-user
(qa's skill); **gate placement + named final-acceptance** live on the lead (L13/L14). The command
consumes the classification.

---

## Altitude note (for `verify-output` item 8 — grep-floor + keystone-ceiling, not raw count)

This is the **5th** command port. `implement` legitimately carries the **most workflow-specific
machinery of any command**: entry gate (f) + scaffolding (g) + sequential cycle loop (a) + per-cycle
execute→verify (b) + targeted retry (c) + fix-pass (d) + convergence-stall (e) + two agents + a
designed-in multi-tier gate (L13/L14). Its 18 `moved-to-lead` responsibilities vs. tasks/plan's
smaller sets mean a thin `implement` may run **modestly longer than tasks (77) / plan (82)** and
still be correctly thin. That extra length must be **genuine workflow-specific orchestration**
(the a–g contract params + the two gates), **never** restated doctrine (DAG vocabulary, the four
rules, validator tiers, gap-routing, a filled contract). The altitude adjudication is line-by-line:
grep-floor = zero inlined contract / zero restated `loop-discipline`; keystone-ceiling = every line
is true only of *this* workflow. Do not pad; do not over-cut to hit 82.

## Handoff

Disposition + complete trace emitted; **no artifact produced** (that is `transform-recipes`, post-reconcile).
The four flags feed Phase-2 `reconcile-cluster`, which must return zero open flags and a rehome map
landing constraints (a)–(g) on the thin lead — sequential-only, confidence-based+final gate wired —
before any edit begins.
