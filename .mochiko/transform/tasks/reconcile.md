# RECONCILE — cluster `tasks` (core-only, 6 primitives)

**Run:** `/mochiko:transform-cluster tasks` · **Phase 2 (reconcile, whole cluster, one producer)** · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:reconcile-cluster` · **Date:** 2026-07-01
**Consumes:** the 6 assessments (`assess-tasks-command.md` P1 · `assess-task-architect.md` P2 · `assess-patterns-vertical-tdd.md` P3 · `assess-validation-task-artifacts.md` P4 · `assess-tasks-template.md` P5 · `assess-tasks-context-template.md` P6) + `contract.md` + `context.md`.
**Reference models read this run:** `plugins/mochiko/commands/plan.md` (the thin-command model for P1) · `skills/mochiko/SKILL.md` (router — **no Tasks section exists**) · `agents/devils-advocate.md` (re-mount stub at frontmatter L25–28) · `agents/technical-analyst.md` + `templates/techanalyst-report-template.md` + `templates/analyst-report-template.md` (producer-report precedent) · `templates/advocate-report-template.md` (scaffolding-dedupe target) · `.claude-plugin/plugin.json` (agents = explicit array).

**Reconcile decides; it does not edit.** Applying these dispositions is `transform-recipes`; grading is `verify-output` (a different agent). RQ-A and the producer-report home are **human-gated** (contract §4) — reconcile assigns a verdict + recommendation and names the gate owner; it does not unilaterally close the two gated calls.

**Done-condition of this reconcile:** every `flag-for-reconcile` carries a concrete relational move, the dissolving supervisor + carrier have one coherent rehome map, every dropped/orphaned responsibility has a landing place, and **Open flags = NONE.** Met (see close).

---

## Independence — the test that overrides convenience (checked first, per skill)

Confirmed **structurally intact** across the cluster; **no split anywhere**, no new validator agent required:

- **Producer** `task-architect` (P2) mounts `patterns-vertical-tdd` (P3) — an authoring/producer skill **only**; no grader. `skills ∩ grading-skills = ∅`.
- **Validator** `devils-advocate` mounts `analysis-specifications` + `validation-plan-artifacts` + (this run) `validation-task-artifacts` (P4) — all reviewer skills; no authoring skill.
- The re-mount lands on the **reviewer** agent, never the producer. **Guard for transform:** `validation-task-artifacts` MUST NOT be mounted on `task-architect`. `task-architect` ≠ `devils-advocate` (disjoint agents). Produce + grade never co-reside → nothing to re-resolve as a split.

The tasks pair is the **single-reviewer `specify` shape** (contract §2), NOT plan's two-form — no homeless feasibility gate to place, so no `pair`/`split` is owed. The only genuinely new primitive is the producer report template (Flag 2, gated).

---

# JOB 1 — Relational verdicts (every flag → a concrete move)

## Flag 1 — RQ-A · artifact shape (THE pivotal call — human-gated; recommendation only)

**Signal:** HIL runs two phases producing two artifacts — `task-mapping.md` (story→cycle + slice **rationale**, authored **freehand**; **HIL has no `task-mapping-template.md`**) then `tasks.md` (cycle→TDD-tasks). But `tasks.md` already carries a "Story → Cycle Mapping" table + per-cycle `> Stories:` traceability → the *result* content is duplicated. This is genuinely cross-primitive: it touches P1's phase structure, P2's artifact menu, P4's checklist structure, P5's mapping section, and P6's mapping row.

**Move:** `rehome` (artifact-shape → the lead) **+ `dedupe`** (the duplication, resolved *without* collapse) — **RECOMMENDATION, gated at the Phase-2 human gate. Not closed here.**

### RECOMMENDATION → **Branch A: keep two phases / two artifacts**, and dissolve the duplication by source-of-truth

- **`task-mapping.md` = single source of truth** for slice rationale + story→cycle decisions (freehand, template-free — matching HIL, which never had a mapping template).
- **`tasks.md`'s Story→Cycle table = an explicitly-derived echo** — a read-only view for the implementer, regenerated from `task-mapping.md`, **not** an independent second source. One source, one derivation → the "duplication reads as a dedupe smell" objection (the collapse camp's one strong point) dissolves **without** collapsing.

**Why A (three reasons, from P1/P5 assessments):** (1) the **early mapping-review gate** — reviewing slicing quality *before* the expensive full TDD breakdown — carries the same real value plan's feasibility-once gate does (cheap rework avoidance); (2) **P4 and P2 are already structured for two phases** (separate Mapping / Tasks / Cross-Artifact checklists), so keep-two is the *least-disruptive* move — minimalism governs the thin **command** (thin either way), it must **not** govern erasing a validator's proven structure; (3) the **cumulative cross-check** (`tasks.md` → `task-mapping.md`) is native to two artifacts. Independence is unaffected either way (architect ≠ advocate under both).

### Consequences for BOTH branches — explicit per primitive (so the lead can decide cleanly)

| Primitive | **Branch A — keep two (RECOMMENDED)** | **Branch B — collapse into one `tasks.md`** |
|---|---|---|
| **P1** command | Two loops: **Phase 1 Mapping** (early mapping-review gate) → **Phase 2 Tasks** (cumulative review back to `task-mapping.md`). Two produce→review sequences. | One loop: single Tasks loop; mapping becomes a `tasks.md` section reviewed intra-artifact. **Loses the early slicing gate** (bad slicing surfaces only after the full breakdown). Cumulative cross-check degenerates to intra-artifact. |
| **P2** task-architect | Persona artifact-menu lists **TWO** artifact-types (a story→cycle mapping; a cycle-based task list). Authors `task-mapping.md` freehand, then `tasks.md`. | Persona menu lists **ONE** artifact-type (a cycle-based task list with an embedded mapping section). Menu re-scoped. |
| **P4** validation-task-artifacts | **Keep** separate Mapping / Tasks / Cross-Artifact checklists (already built) — two review passes + cross-artifact review. | **REDESIGN** the review structure: fold Mapping Review into a single-pass Tasks Review; Cross-Artifact Review collapses to intra-artifact. **Wider blast radius than the thin command** — a validator redesign. |
| **P5** tasks-template | Story→Cycle Mapping section (L191–198) = **deduped derived echo** of `task-mapping.md` (summary view; `task-mapping.md` authoritative). | Story→Cycle Mapping section = **authoritative absorbed mapping** (it IS the mapping). Simpler for P5 — no dedupe. |
| **P6** tasks-context-template (absorbed) | Lead's `Task Mapping` state slice + `phase: mapping` value re-home as **workspace file presence** (`task-mapping.md` present). | That slice **dissolves** (no separate mapping artifact / no `mapping` phase). |

**Net blast radius:** A = one extra produce→review round + a derived-echo note on P5 (the command is thin either way). B = a P4 **redesign** + a P2 re-scope + a P1 phase-collapse — **wider than the thin command**, and it discards P4's already-built two-phase structure. **→ A recommended; the lead owns the final pick at the Phase-2 gate.**

**Downstream disposition robustness:** all six body×structural dispositions are **stable under both branches** (P4's `phase → artifact-type` reframe is safe regardless; P3's content is `kept` regardless — only *framing* shifts; P5 stays `standalone`; P6 stays `absorb`). Only the **conditional slices** in the table above flip on the human's pick — reconcile has resolved each slice under *both* branches, so no slice is left open.

---

## Flag 2 — Producer-report home (`task-architect`'s inline "Planner Report") — human-gated; decided with a firm recommendation

**Signal:** HIL bakes an inline "Planner Report" (P2 L109–152) into the persona, and it **self-asserts a clearing verdict** (`Completion: complete/partial` L122; `Ready for Review …Devil's Advocate` L149–151). Two homes on the table: inline persona self-disclosure vs. a new `taskarchitect-report-template`.

**Move:** `promote → new partner primitive (template)` — **extract the report out of the persona into a standalone template.**

### DECISION (reconcile verdict) → **create `taskarchitect-report-template`** (new primitive), mirroring `techanalyst-report-template`

**Cluster-context finding that settles the "lean":** the actual mochiko codebase has **zero inline-report producers**. **Both** prior single-producer ports use a *separate* lightweight, non-verdict report template — specify's `analyst-report-template` and plan's `techanalyst-report-template`. The assessment's "lean inline" text is internally self-contradictory ("inline … whose report *lives in a lightweight template*") — read against the realized precedent, it points **at a template**, not at persona-inlining. `task-architect` is the direct structural analog of `technical-analyst` (one producer authoring the loop's artifacts) → **mirror plan.** This also keeps the just-excised persona clean (the heaviest port of the run — P2) and gives P6's `architect_report_path` slice a concrete home.

- **New primitive:** `taskarchitect-report-template` at `plugins/mochiko/templates/taskarchitect-report-template.md`.
- **Shape:** mirror `techanalyst-report-template` — `> Feature / Round / Generated` header; **What Was Produced · What Changed This Round · Constitution Alignment · Open Questions · Handoff to Review (a *pointer*, not a claim) · Artifacts Produced (optional)** — plus the two tasks-specific descriptive sections that are legitimate non-verdict self-disclosure: **Vertical-Slice Rationale** and **TDD Structure**.
- **HARD CONSTRAINT (holds under EITHER home):** the self-verdict fields `Completion: complete/partial` and `Ready for Review …Devil's Advocate` are **`dropped`** — the producer must not self-assert clearing; the lead owns the verdict (mirrors the plan port dropping `techanalyst-report`'s `completion_status`, and the template's own Usage Note: "deliberately no 'Completion' / done field").
- **Output location:** `.mochiko/specs/<feature>/taskarchitect-report.md`, seeded/collected by the lead.

**Alternative recorded for the gate:** inline persona self-disclosure (fewer primitives). Rejected as unprecedented and re-bloating the excised persona — but the human may override at the gate. **Either way the self-verdict drop stands.** Gated (contract §4) → carried in the bundle.

---

## Flag 3 — P2 → P3 dedupe (persona keeps care-abouts; skill keeps procedure)

**Signal:** large tracts of the `task-architect` persona **restate procedure `patterns-vertical-tdd` (P3) owns** (P2 assessment trace #10–14, #19; P3 check 5). Canonical home is P3 (grep-confirmed to already carry this content) → the fold has a landing place, **no orphan procedure**.

**Move:** `dedupe` / `folded-into-skill` → **target P3 `patterns-vertical-tdd`** (NOT a merge — P2 agent and P3 skill stay distinct; the standard persona-vs-procedure split, as `technical-analyst` keeps taste while the IP-XXX/ADR/entity/OpenAPI *procedures* live in its skills).

### The exact split — persona-intrinsic (P2 keeps) vs skill-procedure (→ P3)

**P2 KEEPS — care-abouts / taste (the WHY):**
- Core Identity war-stories (L19–25): oversized/misordered tasks · horizontal-slicing pain · tasks-not-mapping-to-value · integration nightmares → `kept` (keystone-clean).
- Quality Standards **as taste** (L154–166); What You Reject / What You Embrace (L168–183) → `kept` as taste (literal criteria dedupe below).
- The **verification-task *concept*** — a real-integration task that gates cycle completion, "what makes vertical TDD actually vertical" (L237–246) → `kept` as taste (the architect *cares* that each slice is verified).
- Brownfield **marker disposition** + the artifact-menu (the output-shape it produces) → `kept`.
- Constitution/governance **alignment** disposition (L50/L83/L142) → `kept` (principles arrive via the lead's brief).

**P2 FOLDS → P3 — literal procedure (the HOW):**
- **Cycle Structure template** (`### Cycle N` format, `TN.1→TN.4`, checkpoint — L214–235) → `folded-into-skill` → P3 (P3 CYCLE-STRUCTURE + Standard Cycle Format is canonical).
- **`**TEST:**` verification-task grammar** (unified format · field reference · action modifiers · assert patterns · examples · legacy-format — L260–341) → `folded-into-skill` → P3.
- **Slice-identification heuristics** (Success Criteria L61–69 / L97–106) + the **foundation-vs-feature test** → `dedupe` → P3 (SLICE-IDENTIFICATION + Quality Checklist).
- **Marker table** (L195–200) → light `dedupe` → P3 (the *markers themselves* stay as craft; the literal table is P3's).
- **Quality-Checklist literal** → `dedupe` → P3 author-side; the **review-side mirror lives in P4** (`validation-task-artifacts`), not P2.

**Split rule (state for transform):** *persona = what the professional cares about + its taste; skill = the step-by-step procedure.* Anything that reads as a fill-in template, a grammar, a heuristic list, or a checklist → P3. Anything that reads as a scar, a standard, or a disposition → stays P2.

---

## Flag 4 — Scaffolding dedupe (P4 embedded report templates → shared `advocate-report-template`)

**Signal:** P4's `ISSUE-TEMPLATES.md` embeds **two full report templates** ("Mapping Review Report" / "Tasks Review Report") — redundant with the already-ported shared `advocate-report-template` (which `devils-advocate` already fills for spec + plan reviews).

**Move:** `dedupe` → **reference `mochiko:advocate-report-template`; do not re-embed.** Keep only the task-specific **"Checks Executed" tables**, the **issue-ID conventions** (`TM-`/`TT-`/`TX-`), and the **verdict decision tree** (mechanical 3-state derivation). Confirmed against the ported `advocate-report-template` (Gaps-Found table + Clarifications + `ready/needs-revision/critical-gaps` verdict + What's-Strong) — it already carries the report shape P4 embedded.

**Division of labor vs `validation-plan-artifacts` — CONFIRMED DISJOINT, NO structural merge:**
- **Artifacts reviewed are disjoint:** PLAN {requirements · constraints-and-decisions · nfrs · data-model · contracts · quickstart} vs TASK {task-mapping.md · tasks.md}.
- **Checks are disjoint:** design coverage/measurability/schema-consistency vs vertical-slice quality · TDD test-first ordering · TEST-task presence · story→cycle→task traceability · task-ID/file-path/marker presence.
- **Only shared surface = scaffolding** (severity taxonomy · 3-state verdict · issue-doc format · report shape) — all already single-sourced (severity + 3-state are the mochiko validation-skill convention; the assembled deliverable is `advocate-report-template`). → **cleanly complementary.** Record the mutual boundary in both skills' `When-NOT` / descriptions (the sibling already points here at its L45). **No merge.**

---

## Flag 5 — F1 / mirror alignment (alignment contracts each transform MUST honor — NOT merges)

Two sideways relationships. Neither is a structural move; each is a **contract** handed to `transform-recipes` and audited by `verify-output`.

### 5a · P3 ↔ P5 (F1) — one canonical `tasks.md` structure

**Move:** `dedupe`/`align` (same class as plan's F1 P7↔P5 `data-model.md`; **NOT a merge** — teach-the-craft vs fill-the-skeleton are distinct primitives, both `standalone`).

**Alignment contract:** **`tasks-template.md` (P5) is the single canonical `tasks.md` *structure*** — the fill-target. `patterns-vertical-tdd` (P3) keeps the teaching/rationale, and its **Standard Cycle Format** + the CYCLE-STRUCTURE.md worked "Complete Example" `tasks.md` **must conform to P5** (byte-aligned on: the `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` markers, the `TN.X` task-ID scheme, the checkpoint line, and the `**TEST:**` block). **Recommended mechanism** (from P3's lean, mirroring plan's "collapse to one canonical template"): P3 *references* P5 for the skeleton and retains the worked example as *illustration pointing at the template*; if kept in full, it must be byte-aligned so the two cannot drift. Under **RQ-A Branch A** the canonical structure additionally shows the Story→Cycle table as a **derived echo** of `task-mapping.md`.

### 5b · P3 ↔ P4 — producer↔validator mirror

**Move:** `align` (the healthy producer↔validator split; **NOT a merge**).

**Alignment contract:** every task-review check in P4's `PHASE-CHECKLISTS.md` corresponds to an authoring rule P3 teaches — **the reviewer checks exactly what the producer is told to author** (vertical-slice quality · TDD test-first ordering · cycle definition/sizing · `**TEST:**` task presence · story→cycle→task traceability · marker/file-path/task-ID presence). No orphan check (P4 grading something P3 never teaches) and no unchecked rule (P3 teaching something P4 never verifies), **within the core-only scope** — the parked IP-XXX / brownfield / roadmap checks are dormant on **both** sides in lock-step (see Flag 7 defer). Transform must keep the two edited together.

---

## Flag 7 — Wiring (convention-wiring pass items + defer-by-reference; handed to `transform-recipes`)

Not structural moves — recorded so nothing is dropped:

1. **`devils-advocate` re-mount (stub → live).** Confirmed the stub sits exactly at `agents/devils-advocate.md` frontmatter L25–28. Wiring: add `validation-task-artifacts` to the `skills:` list (L28), add a Skills-Available bullet (after L38), **delete** the deferred comment (L26–27). Trace `kept-but-rebind` (stub→live). Independence preserved (reviewer agent).
2. **Router registration — NEW "Tasks cluster" section** (verified **absent**: the router has doctrine/transform/setup/specify/plan sections, no tasks). Add a Tasks-cluster skill table (`patterns-vertical-tdd`, `validation-task-artifacts`, `tasks-template`, `taskarchitect-report-template` [if gated-in], with when-to-reach-each + the boundary vs `validation-plan-artifacts`); add a **`/mochiko:tasks`** row under **Entry point**; add a **`task-architect`** row under **Agents**; **update** the `devils-advocate` agent row to "**cross-workflow** — specify critic + plan completeness reviewer + **tasks task-artifact reviewer**." A primitive absent from the router fails discoverability by construction.
3. **`plugin.json`.** Skills auto-discover via the `"skills": "./skills/"` glob (P3, P4 auto-register); commands auto-discover via `"commands": "./commands/"` (the ported `tasks.md` auto-registers); templates have **no** manifest key (P5, `taskarchitect-report-template` need none). **Agents are an explicit array** — exactly how plan's producer was registered (`./agents/technical-analyst.md` is listed). Wiring: **add `"./agents/task-architect.md"` to the `agents` array.**
4. **qa `**TEST:**` runtime classification → `defer-by-reference` (implement/qa).** The `qa-engineer` runtime-classification (CLI auto-approve vs GUI/subjective human checkpoint; "who decides when to involve a human") is downstream implement/qa orchestration. The **4 sibling-name tokens are `dropped`** (P2 L248/274/279/337; P3 CYCLE-STRUCTURE L228/L332 → role phrasing); the **capability re-homes to implement/qa, parked not dropped**. **Boundary confirm (resolves the P1/P6 silent-drop note):** P4's `**TEST:**` check is a **design-time presence/structure** check on `tasks.md` — its author is `task-architect`/P3 (which author the `**TEST:**` task template). The deferred `testing-end-user`/`qa-engineer` is the **runtime executor**, which P4 does **not** need. → the ported P4 TEST check **has an author (P2/P3) this run**; do **not** scope it out. Runtime execution is the deferred consumer.
5. **Boundary vs `executing-tdd-cycle` (implement, deferred) → `defer-by-reference` / forward-watch (CLEAN).** P3 owns task-**structuring** (design-time: mapping + test-first *ordering* + `tasks.md` structure); `executing-tdd-cycle` owns cycle-**execution** (runtime red/green/refactor + code + `cycle-report.md`). `grep cycle-report` across P3 = zero → P3 does not bleed into execution. No in-cluster resolution possible this run; **record the forward-watch:** when implement ports, confirm the structure→execute handoff stays distinct (shared discipline banner + vocabulary are an acceptable anchor, not duplicated substance). NOT a merge, NOT a this-run move.

---

# JOB 2 — Rehome-orchestration (Flag 6 · P1 + P6 as ONE coherent map)

The tasks cluster's real transformation is **not "shed a kernel"** — HIL had a markdown supervisor + a `.workflow/tasks-context.md` carrier, no kernel. It is **"absorb the supervisor's orchestration onto a thin lead AND add the four gates HIL lacked."** P1 (`redesign × absorb-into-lead`) and P6 (`drop × absorb-into-lead`) dissolve into the **same** lead (the ported `tasks.md` command). One map homes each responsibility **exactly once** — no double-home, no drop between primitives (F-P6-1).

## Altitude split (prevents re-inlining the supervisor) — honored

- **Generic loop-discipline → `dedupe` (referenced, NEVER copied into the command body):** the produce→review iteration structure · default-FAIL *mechanics* · producer↔validator independence doctrine · validator tiers · tamper-proofing · the four iteration guards *as requirements* · gap-type routing (incl. the "Research this" knowledge→research branch) · anti-rationalization · briefing mechanics (→ `agent-dispatch`) · the git/Task standing-footer. The command **references** `loop-discipline` + `workflow-contract` + `agent-dispatch`; it fills a `workflow-contract` **artifact** at runtime. Restating any of this is the altitude defect `verify-output` fails.
- **Workflow-specific orchestration → `moved-to-lead`:** only *this* loop's params, sequence, casting, prerequisites, and gate placements (the map below).

## The rehome map (P1 supervisor orchestration + P6 carrier state → the lead)

| Responsibility (source) | Re-homes to |
|---|---|
| Two-phase **Mapping → Tasks** sequence (P1; P6 `phase`) | **lead** — Phase 1 → Phase 2 (Branch A). `phase` value backed by workspace evidence (`task-mapping.md` present ⇒ past mapping; `tasks.md` present ⇒ past tasks) |
| **CUMULATIVE review** — reviewing `tasks.md` cross-checks back to `task-mapping.md` (P1; the single most tasks-specific nuance) | **lead** — Phase-2 mode-select **+ supply BOTH artifact sets** {`tasks.md`}/{`task-mapping.md`} to the reviewer; the *checks* live in P4's Cross-Artifact Review. Must NOT be flattened to "review `tasks.md`" |
| **Early mapping-review gate** — slicing reviewed before the full TDD breakdown (P1) | **lead** — Phase-1 review step (Branch A). The tasks analogue of plan's feasibility-once gate |
| **Team casting** — single-reviewer specify shape (P1) | **lead** — producer `task-architect`; reviewer `devils-advocate`(`validation-task-artifacts`); disjoint |
| **Entry gate: plan-workflow-complete** (P1; P6 `plan_status` R7) | **lead** — Phase-0 gate on **WORKSPACE EVIDENCE**: `.mochiko/specs/<feature>/plan.md` present (+ its backing artifacts) = plan accepted. **NOT** a `plan-context.md` `status` field (mochiko `plan` no longer writes one). Missing → block, point to `/mochiko:plan` |
| **Constitution as governing context** (P1; P6 R8) | **lead** — Phase-0 read of `.mochiko/memory/constitution.md` into the producer's brief. **Governing context, not a blocking gate** (softer than plan — do not strengthen) |
| **Brownfield-from-plan** (P1) | **lead** — inherit brownfield context from plan's artifacts; do NOT re-run codebase analysis. `evolution-roadmap.md`/`[GAP:XXX]` = **documented stub** (roadmap track deferred) |
| **Upstream INPUT reads** ×N (P6 R9: spec/requirements/constraints-and-decisions/nfrs/data-model/contracts) | **lead** — Phase-0 workspace-as-state reads of plan's design outputs → the producer's brief (no registry field) |
| **Within-phase dispatch status / iteration** (P6 R3/R4) | **lead** — in-session loop state + **bounded** round counter (contract §3). HIL's "no hard caps" → cap **3/phase** = an **upgrade**, not like-for-like |
| **Per-artifact statuses** (P6 R10/R11) | **lead** — **workspace file presence** (`task-mapping.md`/`tasks.md` exist + cleared validation). R10 RQ-A-coupled |
| **Producer report path** (P6 R12 `architect_report_path`) | **lead** — workspace-as-state per-round report → **`taskarchitect-report-template`** (Flag 2) |
| **Reviewer report path** (P6 R13 `advocate_report_path`) | **lead** — workspace-as-state → **`advocate-report-template`** (SETTLED, reused) |
| **Supervisor→agent dispatch channel** (P6 R14 `## Supervisor Instructions`) | **lead** — in-session dispatch brief per `agent-dispatch`; NOT a file field |
| **Producer/reviewer CONTENT inside Supervisor Instructions** (P6 R15 — slice+TDD structure, skill hints, mapping/tasks/cross-artifact checklists incl. IP-XXX + `**TEST:**`) | **`moved-to-sibling-skill`** — producer content → P2/P3; reviewer content → P4. **In-cluster this run.** Highest-volume silent-drop surface (F-P6-3) — survives ONLY because P2/P3/P4 port now |
| **Clarification history / "Research this"** (P6 R16 `## Clarification Log`) | **lead** — in-session human-gate history (G3); knowledge-gap → native `Explore` |
| **State recovery / resume** (P1; P6 R17 State Recovery table) | **lead** — **REBOUND to workspace evidence** (artifact presence), NOT a `phase`/`status` field |
| **Cross-context state handoff** (P6 R18 — the carrier's raison d'être) | **lead** — single-session loop; the lead holds state and briefs each agent with its slice |
| **@-input recovery / feature identity** (P1; P6 R5) | **lead** — Phase-0 `@`-reference-drop recovery (G1); tracks the workspace dir |
| **Completion report + next-step** (P1 Phase-4) | **lead** — Finalize: cycle/foundation/feature/`[P]` counts from `task-mapping.md`+`tasks.md`; next-step → `/mochiko:implement` (forward reference, not-yet-ported) |
| **Verify-agent-output / agent-failure** (P1) | **lead** — confirm each expected file was written (`task-mapping.md`/`tasks.md`/round reports); missing → log + ask retry/abort |

### The gates HIL LACKED — rehome-orchestration ADDS them (not just relocation)

| Missing gate | Home |
|---|---|
| **Default-FAIL done-condition** (HIL self-declared done on a pass-1 `ready` verdict) | contract param: both artifacts start FAIL; clear only on independent validation **+** human acceptance |
| **Lead-OWNED verdict** (HIL routed on the advocate's verdict *field*) | the lead **Reads** `task-mapping.md`/`tasks.md` + the advocate report and owns the clearing verdict; **advocate status = input, not the gate** |
| **Hard cap + no-progress + kill-switch** (HIL "no hard caps") | cap **3/phase** (lead counts) · no-progress exit (unchanged gap set round-over-round) · kill-switch **`.mochiko/specs/<feature>/TASKS_STOP`** (mirrors plan's `PLAN_STOP`; distinct from this transform run's meta kill-switch `.mochiko/transform/tasks/STOP`) |
| **NEW human ACCEPTANCE gate on `tasks.md`** (HIL had mid-loop gates but no final acceptance) | Phase-3 named gate (accept → done / amend → bounded re-enter / reject → abort) — **must not displace** the existing mid-loop clarification/exit-early gates |

## The lead's Phase structure + human gates (modeled 1:1 on `plan.md`; Branch A)

Tasks is **single-reviewer** → it uses plan's gate slots **minus G2** (no feasibility reviewer, so no feasibility-rejection gate). Gates: **G1 input-recovery · G3 clarification · G4 exit-early/escalation · G5 `tasks.md` acceptance** (the G2 slot is intentionally empty). The ported command fills its **own** `workflow-contract` artifact at runtime — it does not inline these.

- **Phase 0 — Prerequisites & entry triage *(G1)*:** capture `$ARGUMENTS`, resolve `<feature>`, empty-input `@`-drop recovery (G1); **entry gate = plan-workflow-complete** by workspace evidence (`plan.md` present; missing → block → `/mochiko:plan`); constitution read as governing context (missing → surface, offer `/mochiko:setup`, don't auto-resolve); brownfield-from-plan (inherit; roadmap read = stub); read plan's design outputs as the producer's inputs.
- **Phase 1 — Mapping loop *(you own the round counter + verdict)*:** produce `task-mapping.md` (freehand story→cycle + slice rationale; **source of truth**) + `taskarchitect-report.md`; **early mapping-review** by `devils-advocate`(`validation-task-artifacts` Mapping checklist) → `advocate-report.md`; **verdict (you):** Read `task-mapping.md` + report; `ready` + no blocking gap → Phase 2; else route per `loop-discipline` gap-routing (→ G3), apply bounds (cap/no-progress/`TASKS_STOP` → G4/escalate), loop.
- **Phase 2 — Tasks loop *(cumulative review; you own the round counter + verdict)*:** produce `tasks.md` (cycle→TDD tasks; foundation sequential + feature `[P]`; file path per task; `[US#]`; markers; **Story→Cycle table = derived echo** of `task-mapping.md`) + `taskarchitect-report.md`, briefed with `task-mapping.md` as input; **cumulative review** — dispatch `devils-advocate` in cumulative mode (full `tasks.md` review **+** cross-check back to `task-mapping.md`: mapping↔tasks alignment, story→cycle→task chain, cycle/dependency consistency — P4's Cross-Artifact Review); **you** select the mode and supply BOTH artifact sets → `advocate-report.md`; **verdict (you):** `ready` + no blocking gap → Phase 3; else route (→ G3), apply bounds (→ G4/escalate), loop.
- **Mid-loop gates (both phases):** **G3** clarification (present gaps, log answers in-session, feed next produce; "Research this" → native `Explore`, never the user) · **G4** exit-early/escalation (on a guard trip or stalled gaps: continue-refining / accept-with-noted-gaps / stop-and-review — stays FAIL unless the human accepts). Neither ends the loop on its own.
- **Phase 3 — `tasks.md` acceptance *(human gate G5)*:** reachable only after your clearing verdict — present the validated deliverables (cycle/foundation/feature/`[P]` counts, any noted gaps); accept → Phase 4 / amend → bounded re-enter with the changes as the gap list / reject → abort (drafts remain under `.mochiko/specs/<feature>/`).
- **Phase 4 — Finalize:** report deliverables (`task-mapping.md` · `tasks.md` · the round reports), per-phase round counts, the completion counts, a suggested commit (`docs: tasks <feature>`), next-step `/mochiko:implement`; lightweight retain/clean for round reports; never offer to delete `task-mapping.md`/`tasks.md`.
- **State recovery — from workspace evidence** (no context-file `phase`/`status`): `no plan.md` → Phase 0 (blocked) · `plan.md` present, no `task-mapping.md` → Phase 1 (produce) · `task-mapping.md` present, no advocate mapping report this round → Phase 1 (review) · mapping not cleared, within cap → Phase 1 (loop control) · mapping cleared, no `tasks.md` → Phase 2 (produce) · `tasks.md` present, advocate not `ready`, within cap → Phase 2 (loop control) · both cleared, not yet accepted → Phase 3 · accepted → Phase 4 · `TASKS_STOP` present → escalate (G4).
- **"What you own (not the agents)" footer:** the two-phase sequence + per-phase loop (round counter/no-progress/cap/kill-switch/escalation); the verdict (Read the artifacts, decide against the default-FAIL done-condition — reviewer status is input); the cumulative-mode selection + supplying both artifact sets; the human gates (G1/G3/G4/G5); the plan-complete entry gate + constitution/brownfield prerequisites; verify-agent-output; never let the producer grade its own output. Stay kernel-free; brief per `agent-dispatch`; dispatch via the Task tool; no git/push. Full rules: `mochiko:loop-discipline`.

> **Under RQ-A Branch B** this collapses to a single Tasks loop (Phase 1+2 merge; mapping = a `tasks.md` section, reviewed intra-artifact; the early gate and cumulative cross-check are lost), and **P4 requires a review-structure redesign** — the wider-blast-radius branch. The lead structure above is the recommended (A) form.

---

# JOB 3 — Re-emitted per-primitive traces (relational tags now ASSIGNED)

Only the relational tags reconcile owns are re-emitted here (the assessments hold the full per-responsibility traces; all non-relational tags carry forward unchanged). Every relational signal now resolves to a concrete tag with a target.

**P1 — `tasks` command** · `redesign × absorb-into-lead` (FINAL)
- generic loop-discipline mechanics → `dedupe` (→ `loop-discipline`/`workflow-contract`/`agent-dispatch`; referenced, not copied)
- two-phase sequence · cumulative review (mode + both artifacts) · early mapping gate · team casting · plan-complete entry gate (workspace evidence) · constitution-as-context · brownfield-from-plan · done-condition params · completion report · @-recovery · resume · mid-loop gates · verify-output → `moved-to-lead` (see rehome map)
- ADDED gates (default-FAIL · lead-owned verdict · cap+kill-switch · G5 acceptance) → `moved-to-lead` (params) over `dedupe` (mechanics)
- `tasks-context.md` carrier · inline Task/AskUserQuestion payloads · "no hard caps" counter · Architecture/Communication/Agents-Used diagrams → `dropped + reason`
- §D producer/validator content → **`moved-to-sibling-skill`** (P2/P3 producer; P4 reviewer)

**P2 — `task-architect`** · `port-with-edits × standalone` (FINAL; heaviest port)
- inline "Planner Report" (L109–152) → **`promote` → new `taskarchitect-report-template`** (Flag 2, gated)
- report self-verdict fields (`Completion`; `Ready for Review …Devil's Advocate`) → `dropped + reason` (lead owns verdict)
- Cycle Structure template · `**TEST:**` grammar → **`folded-into-skill` → P3**; slice heuristics · foundation/feature test · marker table · quality-checklist literal → **`dedupe` → P3** (Flag 3)
- qa-engineer runtime-classification handoff → **`moved-to-other-cluster`** (implement/qa, parked) **+ 4 sibling-name tokens `dropped`** (Flag 7.4)
- phase-mode framing · context-file mechanism → `dropped + reason`; Mapping→Tasks sequencing · injected inputs · report path → `moved-to-lead`
- PRODUCER team-role (reviewed by `devils-advocate`+`validation-task-artifacts`) → `kept` (independence intact); artifact-menu = **2 types (A) / 1 type (B)** per RQ-A

**P3 — `patterns-vertical-tdd`** · `port-with-edits × standalone` (FINAL)
- Cycle Structure template · `**TEST:**` grammar · slice heuristics · marker table · quality checklist → **canonical home (receives P2's `folded-into-skill`/`dedupe`)** — no orphan procedure (Flag 3)
- Standard Cycle Format + worked "Complete Example" → **`dedupe`/align to P5 canonical structure** (Flag 5a) — NOT a merge
- authoring criteria → **mirror-aligned with P4's review checklist** (Flag 5b)
- `SKILL.md:23` "the task architect agent" · `CYCLE-STRUCTURE.md:228/332` qa-engineer → `kept-but-rebind` (decouple to role); description → work-context; router row + classification → wiring
- boundary vs `executing-tdd-cycle` → **`defer-by-reference`** (forward-watch, Flag 7.5)

**P4 — `validation-task-artifacts`** · `port-with-edits × standalone` (FINAL)
- embedded Mapping/Tasks Review report templates → **`dedupe` → `advocate-report-template`** (keep Checks-Executed tables + `TM`/`TT`/`TX` IDs + verdict tree) (Flag 4)
- verdict-as-clearing + "Return to Task Architect" → `moved-to-lead`; "Devil's Advocate"/"Task Architect"/"Used by the Devil's Advocate" names → `dropped + reason` (decouple to role)
- Mapping/Tasks/Cross-Artifact checklists → `kept`; **mirror-aligned with P3** (Flag 5b); row-count = **two-phase (A) / single-pass redesign (B)** per RQ-A
- IP-XXX/brownfield/roadmap checks → `kept (dormant/parked)` (Flag 7.4 defer — dormant in lock-step with P3)
- devils-advocate re-mount (stub→live) · router · `plugin.json` → wiring (Flag 7)
- boundary vs `validation-plan-artifacts` → **confirmed disjoint, NO merge** (Flag 4)

**P5 — `tasks-template.md`** · `port-with-edits × standalone` (FINAL)
- Story→Cycle Mapping section (L191–198) → **`dedupe`: derived echo of `task-mapping.md` (A) / authoritative absorbed mapping (B)** per RQ-A
- cycle-format/markers/task-ID/checkpoint/TEST block → **canonical structure; P3 conforms** (Flag 5a)
- provenance L7 · sample-comment `/humaninloop:tasks` L42 + list L44 → `kept-but-rebind` (`.mochiko/specs/<feature>/`; scrub command name); frontmatter description → `dropped + reason` (redundant)

**P6 — `tasks-context-template.md`** · `drop × absorb-into-lead` (FINAL — 4th memory-model confirmation; no artifact produced)
- all state/loop/dispatch slices (R2–R14, R16–R18, R20) → `moved-to-lead` (rehome map; homed once each)
- **R15 producer/reviewer content in `## Supervisor Instructions` → `moved-to-sibling-skill`** (P2/P3 producer; P4 reviewer; in-cluster) — the highest-volume silent-drop surface, now landed
- R12 `architect_report_path` → `moved-to-lead` → `taskarchitect-report-template` (Flag 2); R13 `advocate_report_path` → `advocate-report-template` (settled)
- R10 mapping row + `phase: mapping` → `moved-to-lead` as workspace presence (A) / dissolves (B) per RQ-A
- R1 node-kind · R6 timestamps · R19 ephemeral lifecycle → `dropped + reason` (human-gate accept)
- R21 shared carrier schema → `moved-to-other-cluster` (note: implement inherits workspace-as-state, not a re-ported carrier)

---

# New primitives required

- **`taskarchitect-report-template`** (template) — `plugins/mochiko/templates/taskarchitect-report-template.md`. The `task-architect` producer's per-round **non-verdict** self-disclosure report, mirroring `techanalyst-report-template` (+ tasks-specific Vertical-Slice Rationale / TDD Structure sections), self-verdict fields dropped. Gives P6's `architect_report_path` slice a home. **Gated** (Flag 2) — created only if the human confirms the template home over inline; the self-verdict drop holds either way.

*(No new validator agent/skill — the `task-architect` ↔ `devils-advocate`(`validation-task-artifacts`) pair already exists and is independent. No `pair`/`split`/`promote`-into-validator owed.)*

---

# Finalized dispositions (per primitive — body × structural, stable under both RQ-A branches)

| # | Primitive | Disposition | Relational moves it carries |
|---|-----------|-------------|-----------------------------|
| P1 | `tasks` command | **`redesign × absorb-into-lead`** (thin, ≤ ~90 lines, model `plan.md`) | rehome map; §D content → sibling skills |
| P2 | `task-architect` | **`port-with-edits × standalone`** (heaviest) | `promote`→new report template; `dedupe`/`fold`→P3; qa handoff→other-cluster |
| P3 | `patterns-vertical-tdd` | **`port-with-edits × standalone`** | canonical fold-target; `dedupe`/align→P5; mirror-align↔P4; boundary defer |
| P4 | `validation-task-artifacts` | **`port-with-edits × standalone`** | `dedupe` embedded templates→`advocate-report-template`; mirror-align↔P3; re-mount |
| P5 | `tasks-template.md` | **`port-with-edits × standalone`** | canonical structure; mapping section `dedupe` (A) per RQ-A |
| P6 | `tasks-context-template.md` | **`drop × absorb-into-lead`** (no artifact) | all state→lead; R15→sibling skills |

---

# GATED-DECISIONS BUNDLE (the lead takes these to the human — Phase-2 reconcile gate)

1. **RQ-A — artifact shape (PIVOTAL).** **RECOMMENDATION: Branch A — keep two artifacts** (`task-mapping.md` → `tasks.md`), duplication dissolved by making `task-mapping.md` the source of truth and `tasks.md`'s Story→Cycle table a derived echo. **Tradeoff:** A costs one extra produce→review round but preserves the early slicing gate, the native cumulative cross-check, and P4/P2's already-built two-phase structure; **B** is one artifact/one review but loses the early gate, degrades the cross-check to intra-artifact, and forces a **P4 redesign + P2 re-scope + P1 phase-collapse** (wider blast radius than the thin command). Both branches fully mapped across P1/P2/P4/P5/P6 above. **Human decides; not closed by reconcile.**
2. **Producer-report home.** **RECOMMENDATION: create `taskarchitect-report-template`** (new partner primitive, mirror `techanalyst-report-template`) — the precedent-consistent choice (both prior producers use a separate lightweight non-verdict template; zero inline precedent). **Alternative:** inline persona self-disclosure. **Hard constraint under EITHER:** no self-asserted clearing verdict (`Completion`/`Ready for Review` dropped — lead owns it).
3. **P1 command `redesign`** → the thin sound-loop lead above (contract §4 routine gated item).
4. **P6 `tasks-context-template` → `absorb-into-lead` (body drop; not produced).** The 4th confirmation of the memory-model Key Decision; a **new coupling shape** (cross-workflow plan-complete entry gate) still dissolves cleanly. One upgrade to surface: re-homing `iteration` **binds** HIL's explicitly-unbounded loop ("no hard caps") — an improvement, not a drop.
5. **`devils-advocate` re-mount** of `validation-task-artifacts` (stub → live) — routine gated (contract §4).
6. **Dropped-responsibility reasons needing acceptance (no capability lost — mechanism/plumbing, or re-homed & parked):**
   - **P2:** phase-mode framing · context-file mechanism → `dropped` (markdown-supervisor coupling; content survives as artifact-menu + rehome). Report self-verdict fields → `dropped` (independence). 4 qa-engineer sibling-name tokens → `dropped` (capability re-homes to implement/qa, **parked**).
   - **P1:** `tasks-context.md` carrier · inline `Task()`/`AskUserQuestion()` payloads + `supervisor_instructions` · "no hard caps" counter · Architecture/Communication/Agents-Used diagrams → `dropped` (transliterated mechanism / restated doctrine).
   - **P4:** "Devil's Advocate"/"Task Architect"/"Used by the Devil's Advocate" agent-name references → `dropped` (decouple to role).
   - **P5:** frontmatter `description` → `dropped` (redundant with the H1; deliverable-template precedent carries none).
   - **P6:** `type` node-kind (R1) · `created`/`updated` timestamps (R6) · ephemeral create/delete lifecycle (R19) → `dropped` (state-file bookkeeping; in-session + workspace needs no carrier file).

---

# Open flags

**NONE.** Every `flag-for-reconcile` from the 6 assessments is resolved: RQ-A (rehome/dedupe → recommend A, gated) · producer-report home (promote → new template, gated) · P6 absorb (rehome, gated) · P2→P3 dedupe (split assigned) · P4 scaffolding dedupe (→ `advocate-report-template`) · F1 P3↔P5 (dedupe/align) · mirror P3↔P4 (align) · re-mount + router + `plugin.json` (wiring) · qa `**TEST:**` classification (`defer-by-reference` → implement/qa) · executing-tdd-cycle boundary (`defer-by-reference`/forward-watch). The dissolving supervisor (P1) + carrier (P6) have one coherent rehome map with every responsibility homed exactly once and the four missing gates added. Two calls (RQ-A, producer-report home) carry recommendations to the human gate — that is a gate hand-off, not an open reconcile flag.

**Next:** Phase-2 human gate (RQ-A first) → `transform-recipes` applies the finalized dispositions + rehome map → `verify-output` (independent `validator`, a different agent) grades each artifact + realized trace.
