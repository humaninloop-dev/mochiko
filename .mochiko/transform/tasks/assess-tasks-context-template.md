# Assessment — `templates/tasks-context-template.md` (P6)

Run: `/mochiko:transform-cluster tasks` · core-only · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/tasks-context-template.md` (49 lines)
ROLE: assess / diagnose ONLY — no transform, no grade, no cross-primitive resolution.
Assessed: 2026-07-01

**Described role:** the **tasks workflow's context/state carrier** — the direct analog of specify's
`context-template` (P14) and plan's `plan-context-template` (P11). Created by the `tasks` command at
Phase 1 from this template, written to `specs/{feature-id}/.workflow/tasks-context.md`, updated by the
markdown supervisor across the two phases (mapping → tasks), and read by the domain agents
(`task-architect`, `devils-advocate`) because they hold no workflow knowledge. The brain-prose-era
on-disk handoff between the supervisor and separately-contexted agents.

**Precedent this run makes binding:** setup (`constitution-context-template` P8) → absorb-into-lead;
specify (`context-template` P14) → absorb-into-lead; plan (`plan-context-template` P11) → absorb-into-lead
(the 3rd confirmation, a ROADMAP Key Decision / memory-model). **This is the 4th instance and the 4th
confirmation.** P1 (the `tasks` command, already assessed this run) drops this carrier (§C) and flags
its absorb (flag #2: "Confirm, not open — 3×-confirmed precedent").

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, path coupling.

But — exactly like its three cross-cluster twins — this template is **not an output artifact; it is the
dissolving supervisor's STATE CARRIER.** Every field is loop/phase state, a per-artifact status +
path registry, a supervisor→agent dispatch channel, a cross-cluster input transport, a constitution
transport, or human-gate history. It exists solely to ferry state between the markdown-supervisor and
the separately-contexted agents, phase after phase, pass after pass. It is **plumbing of the dissolving
supervisor**, not a reviewable deliverable.

**Consumption is confirmed first-hand (live carrier, no drift), grounded in `commands/tasks.md`:**
- Referenced **by name** as the template (L158) and instantiated to `specs/{feature-id}/.workflow/tasks-context.md` at Phase 1 (L158-193, ~15 placeholder values).
- Supervisor **updates its frontmatter** at every routing step — `phase: mapping` (L231, 273) → `phase: tasks` (L342, 381) → `phase: completed` (L526); `status: awaiting-architect` (L232, 343) / `awaiting-advocate` (L280) / `awaiting-user` (State Recovery); `iteration` incremented (L303, L487).
- Supervisor **injects per-phase `## Supervisor Instructions`** (L201, L312) and dispatches **every** agent with the identical prompt — *"Read your instructions from: specs/{feature-id}/.workflow/tasks-context.md"* (L241, 287, 353).
- **State Recovery** table (L589-600): resume is keyed entirely on the carrier's `phase`+`status` frontmatter (6 rows, `(mapping|tasks) × (awaiting-architect|awaiting-advocate|awaiting-user)` → resume point) — exactly the context-file `phase`/`status` field the mochiko memory model replaces with **workspace evidence**.
- **Ephemeral lifecycle**: resume-or-restart prompt offers "Start fresh → Delete task artifacts and restart" (L134-142).

**Distinctive vs P11 (plan's twin) — this is a *different-shaped*, not merely simpler, carrier:**
1. **Plan-completion ENTRY GATE.** `plan_status` (Feature Context L18 + File Paths L27) is tasks' *hard upstream prerequisite*: L104-108 blocks unless the plan workflow's `status == completed` ("Tasks workflow requires a completed plan"). Distinctive cross-cluster edge (plan had no such peer-workflow gate; its prereq was the constitution).
2. **The File-Paths table is mostly upstream INPUT reads.** tasks *consumes* plan's whole design output set (spec, requirements, constraints-and-decisions, nfrs, data-model, contracts) as inputs; only `Task Mapping` + `Tasks` are tasks' own outputs. plan's twin registered its own produced artifacts.
3. **Single reviewer → two report paths, not three.** `architect_report_path` = the **producer** (task-architect) report; `advocate_report_path` = the **reviewer** (devils-advocate) report. This is the `specify` shape — **NOT** RQ1-coupled (no reviewer-architecture question; settled single-reviewer). The report slice's only coupling is the milder producer-report-home flag (P1 #3), not plan's F-P11-2.
4. **No two-phase aggregate flags, no brownfield codebase transport.** tasks carries a single `phase` field (not plan's `analysis_status`/`design_status`) and inherits brownfield context from plan's artifacts rather than transporting a `codebase_analysis_age` staleness field. **Leaner in the loop-state dimension** — and still it all dissolves.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = YES (strongly)` — it has **no reason to exist** outside the supervisor-drives-agent loop. The supervisor creates it (L158), bumps phase/status/iteration at every routing step, injects per-phase dispatch instructions, appends clarification rounds, resumes from its fields (L589-600), and deletes it on "Start fresh" (L142). The agents read it *because they hold no workflow state*. **Orchestration-coupling = total**; content-coupling = the state vocab in the body (`type: tasks-request`, `phase`, `status`, `iteration`, the per-artifact status + path registry).
2. **Multi-responsibility / fans out?** `gate2 = YES` — many bundled responsibilities (phase position, within-phase dispatch status, iteration, feature identity, the plan-completion entry gate, an 11-row path+status registry of upstream inputs and own outputs, two report paths, a constitution-principles transport, a supervisor-instruction dispatch channel, a clarification log) feeding three consumers (supervisor + `task-architect` + `devils-advocate`).
3. **Emits a non-machine-checkable artifact?** `gate3 = YES (moot)` — free-form supervisor/agent prose, but it is *plumbing*, not a graded deliverable; **no producer↔validator pairing implied by its content.** (The graded deliverables are `task-mapping.md` / `tasks.md` + the sibling template P5.)

gate1 + gate2 trip → **full lens.** This is the tasks cluster's "how does the loop carry state" slice, and the 4th cluster's test of the now-thrice-confirmed memory model.

## Step 3 — The lens (weighted for artifact branch + orchestration)

**Check 1 — Orchestration test.** Orchestrated by the **markdown supervisor** (`commands/tasks.md`) — NOT
a kernel. Content-coupling to kernel = **NONE**; orchestration-coupling = **TOTAL**. When the supervisor
dissolves onto the mochiko command lead (P1: `redesign × absorb-into-lead`), the carrier's
responsibilities must land somewhere explicit. They are **moved-to-lead** candidates — the
"input/prerequisite wiring → explicit handoff" rehome that `reconcile-cluster`'s rehome-orchestration
(Job 2) owns. Its fate is a *function of the rehomed lead's state design*, shared with P1. **This carrier
cannot be assessed standalone** — it is one slice of the **one** dissolving supervisor's state machinery
(P1 §C drops it, §Reconcile flag #2 flags the absorb).

**Check 2 — Role (two altitudes).** Skill-role analog = **transport** (carries phase state + per-artifact
status/paths + dispatch instructions + clarification history); neither producer nor validator. Team-role
conferred = it makes the supervisor the **lead** (owner of per-phase dispatch + loop state) and the agents
**readers**. Its entire value is loop-state transport → it emits **no reviewable artifact** → needs **no
validator partner**.

**Check 3 — Independence.** No produce+grade leak — it grades nothing, holds no `skills:`. Neutral. (It
*carries* the supervisor's per-phase dispatch instructions — a lead concern, not an independence one.)

**Check 4 — Verdict-sink / loop-driver.** This template **is the substrate the loop-driver writes to.**
`status` + `iteration` + `phase` frontmatter and the `## Supervisor Instructions` + `## Clarification Log`
sections are *how* the produce→review→revise loop is driven across passes and across the two phases
(mapping / tasks). The **State Recovery** table (L589-600) reads this state to resume. These are the
biggest things the supervisor owned and the easiest to drop silently — naming them re-homes them onto the
mochiko lead's **bounded** loop (contract §3: round cap 3 + no-progress + STOP). Note: HIL's `iteration`
is an **unbounded** counter — tasks.md L608 states **"Use judgment for iteration limits (no hard caps)"** —
so re-homing it into the bounded lead loop is an **upgrade**, not a like-for-like move (same shape as
setup's done-condition upgrade, specify's + plan's iteration-bound upgrade).

**Check 5 — Sibling / overlap ("look sideways").**
- **vs the deliverable template P5** (`tasks-template.md`): P5 carries reviewable **content** (the filled
  `tasks.md`) and survives **standalone** (`port-with-edits × standalone`, already assessed); this carries
  **state** and does not. The tasks template family splits along the deliverable/state-carrier line —
  identically to setup, specify, and plan.
- **vs the three cross-cluster twins** — setup's `constitution-context-template` (P8), specify's
  `context-template` (P14), plan's `plan-context-template` (P11): same genre (ephemeral supervisor-driven
  state carrier), same field shape (type/phase/status/iteration/created frontmatter; File Paths; Constitution
  Principles; Supervisor Instructions; Clarification Log), same doctrine ("supervisor owns/creates/updates/
  deletes; agents read only; ephemeral"). All three resolved to **absorb-into-lead (body drop; not
  produced)**. Strong, **three-cluster** precedent.
- **vs P1 (the tasks command).** P1's assessment already tags this carrier `dropped + reason` (§C) and
  **flags the P6 absorb for reconcile** (flag #2, "Confirm, not open — 3×-confirmed precedent"). The
  carrier's state/loop/dispatch responsibilities re-home onto the **same lead** P1 redesigns into. They are
  slices of the **one** dissolving supervisor → the rehome must be coordinated, not decided in isolation.
- **vs the producer/validator siblings P2/P3/P4 (IN-CLUSTER this run).** The `## Supervisor Instructions`
  blocks physically host producer-side content (vertical-slice + TDD cycle structure → `task-architect` P2
  + `patterns-vertical-tdd` P3) and reviewer-side content (mapping/tasks/cross-artifact phase checklists,
  IP-XXX coverage, `**TEST:**` discipline → `validation-task-artifacts` P4). **Unlike plan** (whose
  producer/validator skills were `[ ]` other-cluster), here the receiving siblings **are ported this run** →
  the content lands in-cluster (`moved-to-sibling-skill`, not `moved-to-other-cluster`). This is P6's
  highest-volume silent-drop surface.
- **vs RQ-A (artifact shape — UNRESOLVED, the cluster's primary flag).** P6's `Task Mapping` file-path row
  (`mapping_path`/`mapping_status`) and the `phase: mapping` value exist only if RQ-A keeps two artifacts;
  if it collapses to one, that slice dissolves. This is P6's distinctive reconcile coupling (parallels
  P11's RQ1 coupling, but on RQ-A).
- **Cross-workflow:** the same `.workflow/`-context-carrier genre is shared by specify/plan/tasks/implement.
  Absorbing it here confirms the workspace-as-state precedent for the **last** remaining peer (implement).

**Check 6 — Coupling audit.**
- **Path coupling:** `## File Paths` is a registry of workspace artifact paths — **upstream inputs** (spec,
  plan, requirements, constraints-and-decisions, nfrs, data-model, contracts) + tasks' **own outputs**
  (task-mapping, tasks) + two `.workflow/` report paths (architect, advocate) + `constitution_path`. These
  do **not** rebind in place (the carrier is absorbed); they re-home as **workspace-as-state** on the lead
  (fixed layout `.mochiko/specs/<feature>/{…}` with per-round reports under the workspace). The
  `.humaninloop/memory/…` reference (`constitution_path`) rebinds `.humaninloop/`→`.mochiko/` wherever the
  handoff lands.
- **Status / phase / pass assumptions:** `type: tasks-request` (state node-kind), `phase` (mapping / tasks /
  completed), `status` (within-phase dispatch lifecycle), `iteration` (unbounded pass counter), nine
  per-artifact `*_status` flags. All state-machine assumptions — handled in the trace (phase position →
  in-session + workspace evidence; per-artifact statuses → workspace file presence; iteration → bounded lead
  counter; node-kind/timestamps → dropped).
- **References to excluded / dissolving primitives:** created and driven entirely by the markdown supervisor
  that P1 dissolves; read by `task-architect` (P2, ported this run) and the specify-ported `devils-advocate`.
  **A carrier whose every producer is the dissolving supervisor and whose every consumer reads it only for
  lack of in-session state cannot stand alone.**
- **Prerequisite / handoff (cross-cluster edges):** `plan_status: completed` is a real **entry-gate edge to
  the plan workflow** (L104-108) → rebind to workspace evidence ("plan.md present AND the plan workflow
  accepted"), **do not transliterate the status-field read** (mochiko `plan` no longer writes a context-file
  `phase`/`status`). The spec/requirements/constraints/nfrs/data-model/contracts paths are **cross-cluster
  INPUT reads** from plan's outputs. `constitution_path` + `## Constitution Principles` assume setup's
  constitution exists — but note (P1 §B) the constitution is **governing context here, not a blocking
  Phase-0 gate** (softer than plan). Real cross-cluster handoff edges → rebind to workspace + `.mochiko/memory/`
  reads on the lead, not copied into a context file.
- **Determinism boundary:** none — free-form transport. (The one near-deterministic bit, the entry-gate
  plan-completion check, becomes a lead-side workspace-evidence check, not a carried status field.)

**Check 7 — Conventions + loop placement.**
- **Classification / discoverability:** inert state-carrier; not user/model-invoked, not router-registered.
  Moot under absorb (no standalone artifact to classify or register).
- **Producer↔validator pairing:** not a deliverable → none.
- **Decoupling scan:** `## Supervisor Instructions` is a literal supervisor→agent **dispatch** channel (the
  deny-list "dispatch" coupling baked into the artifact). Under absorb it becomes the lead's **in-session
  dispatch brief** (caller-side context → `agent-dispatch`, per the decouple convention), not a file field.
  No persona to scrub (it is a template), but the dispatch-channel responsibility is precisely what the
  decouple convention says belongs on the caller/lead, not the primitive. **Note (silent-drop surface):**
  the per-phase `## Supervisor Instructions` blocks physically host **producer-side and reviewer-side
  content** — the architect's skill hints (`patterns-vertical-tdd`) + vertical-slice/TDD cycle structure,
  and the advocate's focus-area checklists (story coverage, vertical-slice check, foundation separation,
  IP-XXX coverage, TDD test-first ordering, file-path specificity, `**TEST:**` verification tasks,
  cross-artifact traceability). That content is NOT command orchestration; it belongs to the producer/
  validator **skills** (P2/P3/P4, ported this run), and must not vanish with the dropped carrier field
  (see F-P6-3).
- **Sound-loop placement (the crux):** this template is the *carrier* of the supervisor's loop state —
  done-condition signaling (`status` / per-artifact `*_status`), iteration bound (`iteration`), human-gate
  inputs (`## Clarification Log`), and resume state (the State Recovery table). In mochiko these are
  **loop-discipline obligations of the lead**, carried **in-session + workspace-as-state**, not by an
  ephemeral file. The contract §3 round cap + no-progress + STOP **binds** what HIL left unbounded.

## Step 4 — Disposition

**Body × Structural = `drop` × `absorb-into-lead` — emitted as `flag-for-reconcile`.**
**(The expected result — 4th confirmation of the memory model. Nothing in this template resists absorption.)**

- **Structural = `absorb-into-lead`, FLAGGED for reconcile.** The carrier folds into the command lead P1
  (in-session state + workspace-as-state under `.mochiko/specs/<feature>/` + `.mochiko/memory/` reads),
  leaving no orphan template. **Standalone is refuted** (see Finding) and the absorb is the
  strongly-evidenced expectation (three-cluster precedent; every field dissolves). It is emitted as
  **`flag-for-reconcile`** — not unilaterally resolved at assess — for four reasons:
  1. **Relational rehome coordination (PAIRS WITH P1).** The carrier's state/loop/dispatch responsibilities
     re-home onto the **same lead** P1 (`redesign × absorb-into-lead`) dissolves the supervisor into. One
     coherent rehome map (reconcile Job 2) must home each responsibility exactly once — no double-home, no
     drop between primitives.
  2. **RQ-A-coupled slices.** The `Task Mapping` file-path row and the `phase: mapping` value depend on the
     unresolved artifact-shape question (RQ-A). If RQ-A collapses to one artifact, that slice dissolves.
  3. **Producer-report-home coupling.** The `architect_report_path` slice depends on P1 flag #3 (does
     `task-architect` get its own `taskarchitect-report-template`, or inline self-disclosure?). The
     `advocate_report_path` is settled (`advocate-report-template` reused).
  4. **Human gate (contract — gated dispositions).** The intake gate set "Gated dispositions + escalations";
     the lead/human must accept the absorb + its drops before the run is DONE.
  `absorb-into-lead` is a move onto the lead (not a sibling-relational move), so the *recommendation* is
  callable at assess; what is **deferred to reconcile is the structural confirmation + rehome coordination +
  RQ-A/report-home coupled slices + human gate**, per this run's convention and P1's flag #2.
- **Body = `drop`** — the artifact file is **not produced**; nothing of the body survives as template
  *content* (it is placeholder scaffolding for supervisor state, not reusable prose). This is an
  artifact-level drop; at the *responsibility* level almost everything is `moved-to-lead` (see trace), with a
  few genuine drops (each with a reason) and one sibling-skill rehome. Matches setup P8 + specify P14 + plan
  P11's realized outcome ("absorb-into-lead (body drop); not produced"). The **only** world where body
  becomes `port-with-edits` is the refuted `standalone` path (rebind `.mochiko/` paths) — recorded for
  completeness, not proposed.

## Step 5 — Responsibility trace (every field / section tagged — no silent loss)

| # | Responsibility (field / section, line) | Tag |
|---|----------------------------------------|-----|
| R1 | State node-kind marker (`type: tasks-request`, L2) | **dropped + reason** — supervisor state-file bookkeeping; node-typing has no kernel-free meaning. Lead must accept. |
| R2 | Loop position (`phase`: mapping → tasks → completed, L3) | **moved-to-lead** — in-session loop state; backed by workspace evidence (task-mapping.md present ⇒ past mapping; tasks.md present ⇒ past tasks). *2-phase; the `mapping` value is RQ-A-coupled (F-P6-2).* |
| R3 | Within-phase dispatch status (`status`: awaiting-architect / awaiting-advocate / awaiting-user / completed, L4) | **moved-to-lead** — in-session loop state (which agent the lead is awaiting). |
| R4 | Iteration / pass counting (`iteration`, L5) | **moved-to-lead** — in-session round counter, now **bounded** by contract §3 (round cap 3 + no-progress + STOP). *Upgrade*, not like-for-like — HIL was explicitly unbounded ("no hard caps", tasks.md L608). |
| R5 | Feature identity (`feature_id`, L6) | **moved-to-lead** — lead tracks the active feature/workspace dir in-session (e.g. `.mochiko/specs/<feature>/`). |
| R6 | State-file timestamps (`created`, `updated`, L7-8) | **dropped + reason** — state-file bookkeeping; workspace mtime / git history covers any need. Lead must accept. |
| R7 | Entry-gate plan-completion prerequisite (`plan_status`: Feature Context L18 + File Paths L27; enforced L104-108) | **moved-to-lead** (cross-cluster handoff) — tasks' **hard upstream gate**; rebind the `status == completed` read to workspace evidence ("plan.md present AND plan accepted"); the lead reads the file, **not a status field**. *Distinctive to tasks; transliteration risk (§ Silent-drop).* |
| R8 | Governing-context constitution (`## Constitution Principles` L38-40 + `{{constitution_path}}` L20) | **moved-to-lead** (cross-cluster handoff) — lead points the agent at `.mochiko/memory/constitution.md` (setup's output home) instead of copying principles into a context file; rebind `.humaninloop/`→`.mochiko/`. *Governing context here, not a blocking gate (softer than plan).* |
| R9 | Cross-cluster INPUT path reads ×5 (`spec`, `requirements`, `constraints_decisions`, `nfrs`, `datamodel`, `contracts` paths+statuses, File Paths L26/28-32) | **moved-to-lead** — workspace-as-state reads of **plan's design outputs**; `task-architect` consumes them. The lead points the producer at the workspace layout; no registry field needed. *These are real input dependencies (§ Silent-drop).* |
| R10 | Own-output path+status: Task Mapping (`mapping_path`/`mapping_status`, L33) | **moved-to-lead** — workspace file presence — **RQ-A-coupled** (exists only if two-artifact; dissolves if collapsed). See F-P6-2. |
| R11 | Own-output path+status: Tasks (`tasks_path`/`tasks_status`, L34) | **moved-to-lead** — workspace file presence (the deliverable ⇒ produced; cleared validation ⇒ complete). The run-goal's "per-artifact statuses → workspace file presence", made literal. |
| R12 | Producer report path (`architect_report_path`, L35) | **moved-to-lead** — workspace-as-state per-round **producer** report — **coupled to producer-report-home** (P1 flag #3): own template vs inline self-disclosure. See F-P6-4. |
| R13 | Reviewer report path (`advocate_report_path`, L36) | **moved-to-lead** — workspace-as-state per-round **reviewer** report; **settled** — `advocate-report-template` reused as-is (already in mochiko). |
| R14 | Per-phase supervisor→agent dispatch channel (`## Supervisor Instructions` L42-44; updated L201, L312) | **moved-to-lead** — in-session dispatch: the lead builds each agent's brief directly (`agent-dispatch` pattern, not a file field). **Producer/reviewer CONTENT inside these blocks → see R15 + F-P6-3.** |
| R15 | Producer-/reviewer-side content riding inside `## Supervisor Instructions` (vertical-slice + TDD structure; skill hints; advocate mapping/tasks/cross-artifact checklists incl. IP-XXX coverage + `**TEST:**` discipline) | **moved-to-sibling-skill** — belongs to the producer (`task-architect` P2 + `patterns-vertical-tdd` P3) and validator (`validation-task-artifacts` P4) **skills**, NOT the carrier. **In-cluster this run** (siblings ported now — unlike plan's other-cluster). Highest-volume silent-drop surface (F-P6-3). *Relational → reconcile assigns.* |
| R16 | Multi-round human Q&A history (`## Clarification Log` L46-48; the Clarification Loop + "Research this" knowledge-gap branch) | **moved-to-lead** — human-gate history held in-session by the lead; persisted to workspace if durability needed; the carrier field is absorbed. |
| R17 | State recovery / resume (the State Recovery table keyed on `phase`+`status`, L589-600) | **moved-to-lead**, **REBOUND** to workspace evidence — mochiko resumes from artifact presence, NOT a context `phase`/`status` field. *Exactly the context-file `phase` field the ROADMAP memory model rebinds.* |
| R18 | Cross-context state HANDOFF (the file's reason to exist — separate-context agents read shared state from disk; dispatch prompt L241/287/353) | **moved-to-lead** — mochiko runs the loop in one session; the lead holds state and briefs each agent with its slice via the dispatch brief; durable bits in workspace. |
| R19 | Ephemeral lifecycle (supervisor creates at Phase 1 L158; "Start fresh → delete task artifacts" L142) | **dropped + reason** — in-session + workspace state needs no ephemeral file to manage. Lead must accept. |
| R20 | Intra-template duplication (Feature Context table L15-20 re-surfaces `feature_id` / `phase` as human-readable summary rows) | **dedupe** — under absorb the lead holds each fact once (in-session or workspace); the duplicated summary rows collapse. (`plan_status` + `constitution` rows are the load-bearing ones → R7/R8, not dedupe.) |
| R21 | Shared `.workflow/`-context-carrier schema across specify/plan/tasks/implement | **moved-to-other-cluster** (note) — absorbing here confirms the workspace-as-state precedent; **implement** (last peer) inherits in-session-lead + workspace-as-state, NOT a re-ported context-carrier. Reconcile records the cross-cluster implication (F-P6-5). |

No responsibility left untagged. Drops (R1, R6, R19) carry reasons for human-gate acceptance; all
non-dropped state responsibilities route to the lead (R15 routes to sibling skills; R21 is a cross-cluster
note); R2/R3/R4/R7/R9/R10/R12/R14/R16/R17 feed reconcile's rehome map.

## Reconcile flags (relational / rehome signals — for `reconcile-cluster`)

- **F-P6-1 (rehome coordination; PAIRS WITH P1).** The carrier's state/loop/dispatch responsibilities
  (R2, R3, R4, R9, R10, R11, R12, R13, R14, R16, R17) re-home onto the **same lead** that P1
  (`redesign × absorb-into-lead`) dissolves the supervisor into (P1 §C drops the carrier; §Reconcile flag #2
  flags this absorb). Rehome-orchestration (Job 2) must build **one** coherent map so each responsibility is
  homed exactly once — `phase`→in-session loop position + workspace evidence; per-artifact `*_status`→
  **workspace file presence**; `status`→in-session loop state; `iteration`→**bounded** round-counter
  (contract §3); `## File Paths`→workspace-as-state layout; `## Supervisor Instructions`→in-session dispatch
  (`agent-dispatch`); `## Clarification Log`→in-session human-gate history; State Recovery→workspace-evidence
  resume — with **no double-home and no drop between primitives.** Resolve P6 **together with P1**.
- **F-P6-2 (RQ-A-coupled slice) — P6's distinctive dependency.** The `Task Mapping` file-path row (R10) and
  the `phase: mapping` value (R2) exist only if RQ-A keeps two artifacts. If RQ-A collapses mapping into
  `tasks.md`, that slice dissolves; if it keeps two, it re-homes as workspace file presence. Resolve **with
  RQ-A** (the cluster's primary flag; P1 recommends keep-two, `task-mapping.md` = source-of-truth).
- **F-P6-3 (producer/reviewer content must not vanish with the carrier) — highest-volume silent-drop
  surface.** HIL's `## Supervisor Instructions` blocks physically host producer/reviewer briefing content
  (R15): vertical-slice + TDD cycle structure, `patterns-vertical-tdd` skill hints, and the advocate's
  mapping/tasks/cross-artifact checklists (story coverage, foundation separation, **IP-XXX coverage**, TDD
  test-first ordering, file-path specificity, `**TEST:**` verification tasks, story→cycle→tasks
  traceability). When the carrier drops, this content survives ONLY if the producer (`task-architect` P2 +
  `patterns-vertical-tdd` P3) and validator (`validation-task-artifacts` P4) **skills** carry it. **Unlike
  plan** (skills were other-cluster, unported), here the receiving siblings **are ported this run** →
  reconcile confirms the in-cluster landing place exists. Pairs with the P2/P3/P4 ports.
- **F-P6-4 (producer-report-home coupling).** The `architect_report_path` slice (R12) couples to P1 flag #3:
  does `task-architect` get its own `taskarchitect-report-template` (mirroring `techanalyst-report-template`)
  or inline persona self-disclosure? The `advocate_report_path` (R13) is **settled** (advocate-report-template
  reused). Milder than plan's F-P11-2 — **single-reviewer, so NOT RQ1-coupled** (no reviewer-architecture
  question). Resolve with the producer-report-home flag.
- **F-P6-5 (cross-workflow / cross-cluster).** `tasks-context-template` is the 4th instance of the
  `.workflow/`-context-carrier genre shared by specify/plan/tasks/implement. Absorbing it continues the
  workspace-as-state precedent set by setup + specify + plan. Record the cross-cluster implication:
  **implement (the last peer) inherits in-session-lead + workspace-as-state, NOT a re-ported context-carrier.**
- **F-P6-6 (human gate; contract — gated dispositions).** The absorb and its accepted drops (R1, R6, R19)
  are an explicitly human-gated disposition at Phase 2 reconcile. The lead must accept the absorb + the drops
  before the run is declared DONE.

## State that risks SILENT LOSS (call out for the lead/human to accept)

1. **Entry-gate plan-completion (R7).** tasks' defining hard prerequisite. If transliterated, the lead would
   look for a plan `status == completed` field that mochiko `plan` no longer writes. Must rebind to workspace
   evidence ("plan.md present AND plan accepted"). **Risk: medium-high** — distinctive to tasks; the P1
   assessment flags the identical transliteration risk.
2. **Cross-cluster INPUT reads (R9).** `task-architect` consumes plan's full design output set
   (requirements / constraints-and-decisions / nfrs / data-model / contracts). When the path registry
   dissolves, the lead must still point the producer at plan's workspace outputs, or the producer loses its
   inputs. **Risk: medium.**
3. **Producer/reviewer content in Supervisor Instructions (R15).** Vertical-slice + TDD structure (P3),
   phase checklists incl. **IP-XXX coverage**, **`**TEST:**` discipline**, cross-artifact traceability (P4).
   The thin command drops the instruction blocks wholesale; these capabilities survive ONLY if P2/P3/P4 carry
   them. **Risk: high** — highest-volume silent-drop surface (F-P6-3); mitigated because the siblings are
   ported this run (in-cluster). *(Boundary note, not decided here: P4's `**TEST:**` demand's author,
   `testing-end-user`, is deferred to implement-cluster — confirm the ported validator's TEST check still has
   an author or is scoped.)*
4. **Per-artifact statuses → workspace file presence (R10/R11).** The run-goal's literal requirement. The
   mapping (`*_status` field ⇒ "artifact file exists + cleared validation") must be made explicit so the
   lead's state-recovery reads workspace evidence. **Risk: medium.**
5. **State-recovery rebind (R17).** If transliterated, the lead would look for a `phase`/`status` field that
   no longer exists. Must rebind to workspace evidence (artifact presence). **Risk: medium.**
6. **Clarification Log incl. "Research this" (R16).** Multi-round human Q&A history + the knowledge-gap→
   research branch. If dropped, the loop loses its preference-gap memory and its research escape hatch. Must
   be held in-session (persisted to workspace if durability needed). **Risk: low-medium.**

## The absorb-vs-standalone finding (with the setup + specify + plan comparison)

**FINDING: `absorb-into-lead` is the strongly-evidenced disposition; `standalone` is refuted. The ROADMAP
memory-model Key Decision is CONFIRMED a FOURTH time. Nothing in this template resists absorption. The
structural call is FLAGGED for reconcile (F-P6-1/2/4/6), not unilaterally resolved at assess.**

**Why absorb, not standalone.** tasks-context-template is a **state carrier, not a deliverable.** Every
field is one of: (a) loop/phase state (`type`, `phase`, `status`, `iteration`, `created`/`updated`); (b) a
per-artifact status + path registry derivable from workspace file presence (nine `*_status`/path rows + the
`## File Paths` registry); (c) a supervisor→agent dispatch channel mochiko replaces with in-session dispatch
(`## Supervisor Instructions`); (d) cross-cluster inputs the lead reads from the workspace + `.mochiko/memory/`
(`plan_status` entry gate, the plan design-output reads, `## Constitution Principles`); (e) human-gate history
(`## Clarification Log`); (f) resume state mochiko reads from workspace evidence (the State Recovery table).
**None is reusable body content.** tasks.md confirms the coupling is structural and total — the carrier exists
*only* because HIL ran agents in separate contexts (dispatch prompt L241/287/353) and resumes from its
`phase`+`status` fields (L589-600). A carrier whose every producer is the dissolving supervisor has nothing to
stand on as a standalone artifact. **I checked each field for a slice that must survive as a standalone
deliverable; none does** — the producer/reviewer *content* riding in Supervisor Instructions (R15) survives,
but it re-homes to the sibling **skills**, not to a re-ported context file.

**Setup + specify + plan comparison — same kind, 4th confirmation.** This is the cross-cluster **twin** of
setup's `constitution-context-template` (P8 → absorb), specify's `context-template` (P14 → absorb), and
plan's `plan-context-template` (P11 → absorb, the ROADMAP Key Decision). tasks confirms on four counts:
1. **Live, no drift.** Like specify/plan (and unlike setup's drifted P8), tasks.md **uses the template
   directly and by name** (L158) and resumes from its fields (L589-600) — the absorb rests on observed
   consumption.
2. **A *different-shaped* carrier, still dissolves.** tasks is not merely "plan minus fields": it adds a
   **peer-workflow entry gate** (`plan_status`, the first context-carrier to block on another *workflow's*
   completion), and its File-Paths table is dominated by **upstream INPUT reads** of plan's outputs rather
   than its own produced artifacts — yet **all of it** still dissolves cleanly into in-session lead state +
   workspace-as-state + `.mochiko/memory/` reads. A new coupling shape (cross-workflow prerequisite) passing
   is independent evidence, not a repeat.
3. **The State Recovery table is again the clean "phase-field → workspace-evidence" case** (L589-600, keyed
   on `phase`+`status`) — the exact field the ROADMAP memory model replaces with workspace evidence.
4. **Simpler in the loop-state dimension, still needs no carrier** — no two-phase aggregate flags, no
   brownfield staleness transport. The leaner case dissolving as cleanly as the richest (plan) brackets the
   decision from both ends.

**Same split in all four clusters.** Templates divide along the deliverable/state-carrier line identically:
setup = 2 standalone deliverables + 1 absorbed carrier; specify = 3 + 1; plan = 4 standalone deliverables + 1
absorbed carrier; tasks = 1 standalone deliverable (`tasks-template` P5) + 1 absorbed carrier (P6).

**Recommendation to the lead:** with four clusters now agreeing, the memory-model Key Decision holds
unshaken; tasks-context-template's absorb is the expected fate. Confirm at reconcile **alongside P1** (one
coherent rehome map), **with RQ-A** (the mapping row's shape) and the **producer-report-home** flag (the
architect report slice), and accept the drops (R1/R6/R19) at the human gate. The one upgrade to surface:
re-homing `iteration` into the lead **binds** HIL's explicitly-unbounded loop ("no hard caps", L608) — an
improvement, traced as `moved-to-lead`, not a drop.

---

### Summary block

```
ASSESSMENT: tasks-context-template.md (P6)
Class:        template → branch artifact  (functionally = dissolving-supervisor state carrier)
Triage:       gate1=y gate2=y gate3=y(moot)  [full-lens]
Disposition:  drop (body) × absorb-into-lead (structural) — emitted as FLAG-FOR-RECONCILE
              [standalone REFUTED; setup+specify+plan precedent CONFIRMED a 4th time; nothing resists absorption]
              — absorb is the strongly-evidenced expectation; flagged because the rehome is RELATIONAL
                (pairs with P1, one coherent map), RQ-A-COUPLED (mapping row), REPORT-HOME-COUPLED
                (architect slice), and HUMAN-GATED
Trace:
  - R1  type: tasks-request node-kind         → dropped + reason (state-file bookkeeping)
  - R2  phase (mapping→tasks→completed)        → moved-to-lead (in-session + workspace evidence) [mapping value RQ-A-coupled]
  - R3  within-phase status                    → moved-to-lead (in-session loop state)
  - R4  iteration/pass counter                 → moved-to-lead (in-session; now BOUNDED — upgrade vs "no hard caps" L608)
  - R5  feature identity (feature_id)          → moved-to-lead (lead tracks workspace dir)
  - R6  created/updated timestamps             → dropped + reason (state-file bookkeeping)
  - R7  plan_status ENTRY GATE                 → moved-to-lead (workspace evidence; cross-cluster→plan; NOT a status field) [distinctive]
  - R8  constitution principles + path         → moved-to-lead (read .mochiko/memory/constitution.md; rebind; governing context)
  - R9  5 upstream INPUT path reads            → moved-to-lead (workspace-as-state; task-architect's inputs from plan's outputs)
  - R10 Task Mapping path/status               → moved-to-lead (workspace file presence) — RQ-A-COUPLED
  - R11 Tasks path/status                       → moved-to-lead (WORKSPACE FILE PRESENCE — the deliverable)
  - R12 architect_report_path (producer)        → moved-to-lead (workspace-as-state) — PRODUCER-REPORT-HOME-COUPLED (P1 #3)
  - R13 advocate_report_path (reviewer)         → moved-to-lead (workspace-as-state) — SETTLED (advocate-report-template reused)
  - R14 ## Supervisor Instructions dispatch     → moved-to-lead (in-session dispatch / agent-dispatch)
  - R15 producer/reviewer content inside R14    → moved-to-sibling-skill (P2 task-architect + P3 patterns-vertical-tdd + P4 validation-task-artifacts; IN-CLUSTER) [SILENT-DROP HIGH]
  - R16 ## Clarification Log human Q&A history   → moved-to-lead (in-session human-gate history; incl. "Research this")
  - R17 State Recovery (phase+status resume)     → moved-to-lead, REBOUND to workspace evidence (L589-600)
  - R18 cross-context state handoff (raison d'être) → moved-to-lead (single-session loop; brief per agent-dispatch)
  - R19 ephemeral create/delete lifecycle         → dropped + reason (in-session+workspace needs no file; "Start fresh" L142)
  - R20 intra-template duplicated summary rows     → dedupe (lead holds each fact once)
  - R21 shared schema across specify/plan/tasks/implement → moved-to-other-cluster (note; implement is the last peer; F-P6-5)
Reconcile flags:
  - F-P6-1 rehome coordination — PAIRS WITH P1; one coherent rehome map (Job 2); home each state/loop/
           dispatch responsibility exactly once; no double-home, no drop between primitives
  - F-P6-2 RQ-A-coupled slice — Task Mapping row + phase:mapping value depend on artifact-shape (RQ-A);
           dissolves if collapsed, workspace-presence if kept-two; resolve WITH RQ-A
  - F-P6-3 producer/reviewer content (slice/TDD structure, IP-XXX, TEST:, cross-artifact checklists) in
           ## Supervisor Instructions must land on sibling skills P2/P3/P4 (IN-CLUSTER this run), not vanish
  - F-P6-4 producer-report-home coupling — architect_report_path slice couples to P1 #3 (own template vs
           inline); advocate slice SETTLED; single-reviewer → NOT RQ1-coupled (unlike plan F-P11-2)
  - F-P6-5 cross-workflow — carrier genre shared by specify/plan/tasks/implement; implement (last peer)
           inherits in-session-lead + workspace-as-state, NOT a re-ported carrier
  - F-P6-6 human gate — accept the absorb + drops R1/R6/R19 before DONE (gated dispositions)
Silent-drop risks: plan-completion ENTRY GATE rebind (R7, med-high) · cross-cluster INPUT reads (R9, med) ·
           producer/reviewer content in Supervisor Instructions (R15, HIGH) · per-artifact statuses→workspace
           evidence (R10/R11, med) · state-recovery rebind (R17, med) · Clarification Log incl. "Research this" (R16)
Finding:      absorb-into-lead = strongly-evidenced (4th cluster; nothing resists; a NEW coupling shape —
              cross-workflow entry gate — still dissolves); standalone refuted; memory-model Key Decision
              CONFIRMED a 4th time; structural call FLAGGED for reconcile (coordination + RQ-A + report-home +
              human gate), not resolved solo
```
