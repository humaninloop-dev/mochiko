# Assessment — `templates/plan-context-template.md` (P11)

Run: `plan` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/plan-context-template.md`
ROLE: assess / diagnose ONLY — no transform, no grade, no cross-primitive resolution.
Described role: the **plan workflow's context/state carrier** — created by the `plan` command at
Phase 1.1 from this template, written to `specs/{feature-id}/.workflow/plan-context.md`, updated by
the supervisor across phases/passes, read by the domain agents (`technical-analyst`,
`principal-architect`, `devils-advocate`) because they hold no workflow knowledge. The brain-prose-era
handoff document between the markdown supervisor and separately-contexted agents. Shared genre with
specify/tasks/implement.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, path coupling.

But — exactly like setup's twin (P8) and specify's twin (P14) — this template is **not an output
artifact; it is the dissolving supervisor's STATE CARRIER.** Every field is loop/phase state, a
per-artifact status registry, a path registry, a supervisor→agent dispatch channel, a cross-cluster
input transport, or human-gate history. It exists solely to ferry state between the markdown-supervisor
and the separately-contexted agents, phase after phase, pass after pass. It is **plumbing of the
dissolving supervisor**, not a reviewable deliverable.

**Consumption is confirmed (live carrier, no drift):** `commands/plan.md` references it by name as the
template (L232) and creates `plan-context.md` from it at "Phase 1.1: Create Plan Context" (L230-271,
~40 placeholder values). The supervisor updates its frontmatter (`phase`/`status`/`iteration`/
`analysis_status`/`design_status`) at every routing step (L314-321, L377-380, L438-441, L508-515,
L589-592, L813-824), injects per-phase `## Supervisor Instructions` (L276-312, L345-375, L404-436,
L475-506, L538-587), appends `## Clarification Log` rounds (L639-650, L736-749), and dispatches every
agent with the same prompt — *"Read your instructions from: specs/{feature-id}/.workflow/plan-context.md"*
(L327-331, L383-389, L443-450, L519-525, L594-601). The defining coupling is L906: **"Agents have NO
workflow knowledge—all context via context file."** That on-disk handoff is the precise coupling mochiko
sheds by running the loop in one session. A second defining coupling is the **State Recovery** table
(L884-898): resume is keyed entirely on the carrier's `phase`+`status` frontmatter — exactly the
context-file `phase` field mochiko replaces with workspace evidence.

This is the same kind as P14 (specify) and the **richest instance yet**: a two-phase loop (`phase`,
`analysis_status`, `design_status`), six per-artifact statuses, three reviewer-report paths, a
constitution-principles transport, and a brownfield codebase-context transport with a staleness field.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = YES (strongly)` — it has **no reason to exist** outside the
   supervisor-drives-agent loop. The supervisor creates it (Phase 1.1), bumps its phase/status/iteration
   at every routing step, injects per-phase dispatch instructions, appends clarification rounds, and
   deletes it on "Start fresh" (L224). The agents read it *because they hold no workflow state* (L906).
   **Orchestration-coupling = total**; content-coupling = the state vocab in the body (`type:
   plan-request`, `phase`, `status`, `iteration`, `analysis_status`/`design_status`, the per-artifact
   status registry, the path registry).
2. **Multi-responsibility / fans out?** `gate2 = YES` — many bundled responsibilities (phase position,
   within-phase status, iteration, two-phase aggregate statuses, feature identity, six per-artifact
   statuses, three report paths, constitution-principles transport, project-type/codebase-context
   transport with staleness, supervisor-instruction dispatch channel, clarification log) feeding four
   consumers (supervisor + technical-analyst + principal-architect + devils-advocate).
3. **Emits a non-machine-checkable artifact?** `gate3 = YES (moot)` — free-form supervisor/agent prose,
   but it is *plumbing*, not a graded deliverable; **no producer↔validator pairing implied by its
   content.** (The graded deliverables are the six artifacts + the sibling templates P10/P12/P13.)

gate1 + gate2 trip → **full lens.** This is the plan cluster's "how does the loop carry state" pivot,
and the third cluster's test of the now-confirmed memory model (ROADMAP Key Decision, OQ#3).

## Step 3 — The lens (weighted for artifact branch + orchestration)

**Check 1 — Orchestration test.** Orchestrated by the **markdown supervisor** (`commands/plan.md`) —
NOT a kernel (plan is DAG in some HIL workflows, but this carrier is driven by the prose supervisor
inline). Content-coupling to kernel = **NONE**; orchestration-coupling = **TOTAL**. When the supervisor
dissolves onto the mochiko command lead (P1: `redesign × absorb-into-lead`), the carrier's
responsibilities must land somewhere explicit. They are **moved-to-lead** candidates — the
"input/prerequisite wiring → explicit handoff" rehome that `reconcile-cluster`'s rehome-orchestration
(Job 2) owns. Its fate is a *function of the rehomed lead's state design*, shared with P1. **This
carrier cannot be assessed standalone** — it is one slice of the **one** dissolving supervisor's
state machinery.

**Check 2 — Role (two altitudes).** Skill-role analog = **transport** (carries phase state + per-artifact
status + dispatch instructions + clarification history); neither producer nor validator. Team-role
conferred = it makes the supervisor the **lead** (owner of per-phase dispatch + loop state) and the
agents **readers**. Its entire value is loop-state transport → it emits **no reviewable artifact** →
needs **no validator partner**.

**Check 3 — Independence.** No produce+grade leak — it grades nothing, holds no `skills:`. Neutral. (It
*carries* the supervisor's per-phase dispatch instructions — a lead concern, not an independence one.)

**Check 4 — Verdict-sink / loop-driver.** This template **is the substrate the loop-driver writes to.**
`status` + `iteration` + `phase` frontmatter and the `## Supervisor Instructions` + `## Clarification
Log` sections are *how* the produce→feasibility→completeness→revise loop is driven across passes and
across the two phases (plan.md §2.2/2.5/2.7/2.8, §3.2/3.5/3.6, Feasibility Rejection Loop, Clarification
Loop). The **State Recovery** table (L884-898) reads this state to resume. These are the biggest things
the supervisor owned and the easiest to drop silently. Naming them re-homes them onto the mochiko lead's
**bounded** loop (contract §3: round cap 3 + no-progress + STOP). Note: HIL's `iteration` was an
**unbounded** counter — plan.md L905 states **"Use judgment for iteration limits (no hard caps)"** —
so re-homing it into the bounded lead loop is an **upgrade**, not a like-for-like move (same shape as
setup's done-condition upgrade and specify's iteration-bound upgrade).

**Check 5 — Sibling / overlap ("look sideways").**
- **vs deliverable templates P10/P12/P13** (plan-template, techanalyst-report, architect-report,
  advocate-report): those carry reviewable **content** and survive **standalone**; this carries
  **state** and does not. The template family splits along the deliverable/state-carrier line —
  identically to setup and specify.
- **vs setup's `constitution-context-template` (P8)** and **specify's `context-template` (P14)** — the
  **cross-cluster twins**: same kind, same field genre, same "supervisor owns/creates/updates/deletes;
  agents read only; ephemeral" doctrine. Both resolved to **absorb-into-lead (body drop; not produced)**.
  Strong, two-cluster precedent.
- **vs P1 (the plan command).** P1's assessment already tags this carrier `dropped + reason` (§C) and
  **flags the P11 absorb for reconcile** (§F, flag #2). The carrier's state/loop/dispatch
  responsibilities re-home onto the **same lead** P1 redesigns into. They are slices of the **one**
  dissolving supervisor → the rehome must be coordinated, not decided in isolation.
- **vs RQ1 (reviewer architecture — UNRESOLVED).** P11 carries **three** report paths
  (`analyst_report_path`, `architect_report_path`, `advocate_report_path`) — one more than specify's
  twin, reflecting plan's **two** reviewers (principal-architect feasibility + devils-advocate
  completeness). The rehome shape of the architect + advocate report slices **depends on RQ1**. This is
  P11's distinctive reconcile coupling that specify's twin (fixed analyst+advocate) did not have.
- **Cross-workflow:** the same `.workflow/`-context-carrier genre is shared by specify/plan/tasks/
  implement → absorbing it here continues the precedent for the peers (cross-cluster note for reconcile).

**Check 6 — Coupling audit.**
- **Path coupling:** `## File Paths` is a registry of workspace artifact paths (spec / requirements /
  constraints-and-decisions / nfrs / data-model / contracts/ / quickstart) + three `.workflow/` report
  paths + `constitution_path` + `codebase_analysis_path`. These do **not** rebind in place (the carrier
  is absorbed); they re-home as **workspace-as-state** on the lead (fixed layout, e.g.
  `.mochiko/specs/<feature>/{requirements.md, …, data-model.md, contracts/api.yaml, quickstart.md}` with
  per-round reports under the workspace). The `.humaninloop/memory/…` references (`constitution_path`,
  `codebase_analysis_path`) rebind `.humaninloop/`→`.mochiko/` wherever the handoff lands.
- **Two-phase / status / pass assumptions:** `type: plan-request` (state node-kind), `phase`
  (analysis/design/completed), `status` (within-phase dispatch lifecycle), `iteration` (unbounded pass
  counter), `analysis_status`/`design_status` (Phase-1/Phase-2 aggregate flags), six per-artifact
  `*_status` flags. All state-machine assumptions — handled in the trace (phase position → in-session +
  workspace evidence; per-artifact statuses → workspace file presence; iteration → bounded lead counter;
  node-kind/timestamps → dropped).
- **References to excluded / dissolving primitives:** created and driven entirely by the markdown
  supervisor that P1 dissolves; read by `technical-analyst` (`[ ]` not yet ported), the setup-ported
  `principal-architect`, and the specify-ported `devils-advocate`. **A carrier whose every producer is
  the dissolving supervisor and whose every consumer reads it only for lack of in-session state cannot
  stand alone.**
- **Prerequisite/handoff (cross-cluster edges):** `spec_status: present` assumes specify's `spec.md`
  exists first (entry-gate edge to specify); `constitution_path` + `## Constitution Principles` assume
  setup's constitution exists first; `codebase_analysis_path` + `codebase_analysis_age` assume setup's
  brownfield `codebase-analysis.md`. Real cross-cluster handoff edges → rebind to `.mochiko/memory/`
  reads on the lead, not copied into a context file.
- **Determinism boundary:** none — free-form transport. (The one near-deterministic bit, the
  `codebase_analysis_age` >14d staleness check, becomes a lead-side workspace-mtime computation, not a
  carried field.)

**Check 7 — Conventions + loop placement.**
- **Classification / discoverability:** inert state-carrier; not user/model-invoked, not router-
  registered. Moot under absorb (no standalone artifact to classify or register).
- **Producer↔validator pairing:** not a deliverable → none.
- **Decoupling scan:** `## Supervisor Instructions` is a literal supervisor→agent **dispatch** channel
  (the deny-list "dispatch" coupling baked into the artifact). Under absorb it becomes the lead's
  **in-session dispatch brief** (caller-side context → `agent-dispatch`, per the decouple convention),
  not a file field. No persona to scrub (it is a template), but the dispatch-channel responsibility is
  precisely what the decouple convention says belongs on the caller/lead, not the primitive. **Note
  (silent-drop surface):** the per-phase `## Supervisor Instructions` blocks physically host
  **producer-side and reviewer-side content** — the analyst's skill hints + IP-XXX infra-planning + the
  advocate's focus-area checklists (FR-coverage, sensitivity-contract / integration-contract /
  infrastructure-design alignment, etc.). That content is NOT command orchestration; it belongs to the
  producer/validator **skills**, and must not vanish with the dropped carrier field (see F-P11-3).
- **Sound-loop placement (the crux):** this template is the *carrier* of the supervisor's loop state —
  done-condition signaling (`status`/`analysis_status`/`design_status`), iteration bound (`iteration`),
  human-gate inputs (`## Clarification Log`), and resume state (the State Recovery table). In mochiko
  these are **loop-discipline obligations of the lead**, carried **in-session + workspace-as-state**,
  not by an ephemeral file. The contract §3 round cap + no-progress + STOP **binds** what HIL left
  unbounded.

## Step 4 — Disposition

**Body × Structural = `drop` × `absorb-into-lead` — emitted as `flag-for-reconcile`.**

- **Structural = `absorb-into-lead`, FLAGGED for reconcile.** The carrier folds into the command lead
  P1 (in-session state + workspace-as-state under `.mochiko/specs/<feature>/` + `.mochiko/memory/`
  reads), leaving no orphan template. **Standalone is refuted** (see Finding) and the absorb is the
  strongly-evidenced expectation (two-cluster precedent, richest carrier, all state dissolves). It is
  emitted as **`flag-for-reconcile`** — not unilaterally resolved at assess — for three reasons this
  run makes binding:
  1. **Relational rehome coordination (PAIRS WITH P1).** The carrier's state/loop/dispatch
     responsibilities re-home onto the **same lead** P1 redesigns into, alongside P1's dissolving
     supervisor orchestration (2-phase sequence, skip-re-review routing, Phase-2 incremental review,
     state-recovery, mid-loop gates). One coherent rehome map (reconcile Job 2) must home each
     responsibility exactly once — no double-home, no drop between primitives.
  2. **RQ1-coupled report-path shape.** P11's three report paths depend on the unresolved dual-reviewer
     architecture (RQ1). The architect + advocate report slices cannot be finalized until RQ1 lands.
  3. **Human gate (contract §4a / RQ5).** The contract names "P11 plan-context-template expected
     absorb-into-lead" as a gated reconcile decision; the lead/human must accept the absorb + its drops
     before the run is DONE.
  `absorb-into-lead` is a move onto the lead (not a sibling-relational move), so the *recommendation* is
  callable at assess; what is **deferred to reconcile is the structural confirmation + rehome
  coordination + RQ1-coupled shape + human gate**, per this run's convention and P1's flag #2.
- **Body = `drop`** — the artifact file is **not produced**; nothing of the body survives as template
  *content* (it is placeholder scaffolding for supervisor state, not reusable prose). This is an
  artifact-level drop; at the *responsibility* level almost everything is `moved-to-lead` (see trace),
  with a few genuine drops (each with a reason). Matches setup P8 + specify P14's realized outcome
  ("absorb-into-lead (body drop); not produced"). The **only** world where body becomes
  `port-with-edits` is the refuted `standalone` path (rebind `.mochiko/` paths) — recorded for
  completeness, not proposed.

## Step 5 — Responsibility trace (every responsibility tagged — no silent loss)

| # | Responsibility (field / section) | Tag |
|---|----------------------------------|-----|
| R1 | State node-kind marker (`type: plan-request`) | **dropped + reason** — supervisor state-file bookkeeping; node-typing has no kernel-free meaning. Lead must accept. |
| R2 | Two-phase loop position (`phase`: analysis → design → completed) | **moved-to-lead** — in-session loop state; the lead tracks which of the 2 phases is active. Backed by workspace evidence (Phase-1 vs Phase-2 artifacts present). *Distinctive to plan (2-phase).* |
| R3 | Within-phase dispatch status (`status`: awaiting-analyst/architect/advocate/user/completed) | **moved-to-lead** — in-session loop state (which agent the lead is awaiting). |
| R4 | Iteration / pass counting (`iteration`) | **moved-to-lead** — in-session round counter, now **bounded** by contract §3 (round cap 3 + no-progress + STOP). *Upgrade*, not like-for-like — HIL was explicitly unbounded ("no hard caps", plan.md L905). |
| R5 | Feature identity (`feature_id`) | **moved-to-lead** — lead tracks the active feature/workspace dir in-session (e.g. `.mochiko/specs/<feature>/`). |
| R6 | State-file timestamps (`created`, `updated`) | **dropped + reason** — state-file bookkeeping; workspace mtime / git history covers any need. Lead must accept. |
| R7 | Two-phase aggregate statuses (`analysis_status`, `design_status`) | **moved-to-lead** — via **workspace evidence**: a phase is "complete" when its artifacts exist and have cleared validation, not via a status field. *Distinctive to plan.* |
| R8 | Per-artifact statuses ×6 (`requirements_status`, `constraints_decisions_status`, `nfrs_status`, `datamodel_status`, `contracts_status`, `quickstart_status`) | **moved-to-lead** — via **workspace file presence** (artifact exists ⇒ produced; cleared validation ⇒ complete). The run-goal's "per-artifact statuses → workspace file presence", made literal. |
| R9 | Entry-gate spec handoff (`spec_status: present`, `spec_path`) | **moved-to-lead** (cross-cluster handoff) — rebind to workspace evidence: `spec.md` present + specify accepted; the lead reads the file, not a status field. |
| R10 | Workspace artifact path registry (`## File Paths`: spec/requirements/constraints/nfrs/data-model/contracts/quickstart) | **moved-to-lead** — workspace-as-state: the lead knows the fixed `.mochiko/specs/<feature>/` layout; no registry field needed. |
| R11 | Reviewer report paths ×3 (`analyst_report_path`, `architect_report_path`, `advocate_report_path`) | **moved-to-lead** — workspace-as-state per-round reports — **BUT the SET of report homes is RQ1-dependent** (two reviewers → two report homes; folded → one; generic-validator → its report). See F-P11-2. |
| R12 | Inline constitution principles (`## Constitution Principles`, `constitution_path`) | **moved-to-lead** (cross-cluster handoff) — lead points the agent at `.mochiko/memory/constitution.md` (setup's output home) instead of copying principles into a context file; rebind `.humaninloop/`→`.mochiko/`. |
| R13 | Brownfield codebase handoff (`project_type`, `codebase_analysis_path`, `codebase_analysis_age` staleness) | **moved-to-lead** (cross-cluster handoff) — lead reads `.mochiko/memory/codebase-analysis.md` and computes staleness from workspace mtime; the >14d warning is a lead-side check, not a carried field. |
| R14 | Free-form codebase context transport (`{{codebase_context}}`, filled by analyst if brownfield) | **moved-to-lead** — `technical-analyst` still produces brownfield context; it lands in-session/workspace and the lead assembles it, not this carrier. |
| R15 | Per-phase supervisor→agent dispatch instructions (`## Supervisor Instructions`: analyst P1/P2, architect, advocate P1/P2-incremental, revision) | **moved-to-lead** — in-session dispatch: the lead builds each agent's brief directly (`agent-dispatch` pattern, not a file field). **Producer/reviewer CONTENT inside these blocks → see R16 + F-P11-3.** |
| R16 | Producer-/reviewer-side content riding inside `## Supervisor Instructions` (skill hints, IP-XXX infra planning, advocate focus-area checklists) | **moved-to-other-cluster** — belongs to the producer (`technical-analyst` + `authoring-technical-requirements`/`patterns-*`) and validator (`validation-plan-artifacts`) **skills**, NOT the carrier. Highest-volume silent-drop surface (F-P11-3). |
| R17 | Multi-round human Q&A history (`## Clarification Log`: feasibility iterations + clarification iterations) | **moved-to-lead** — human-gate history held in-session by the lead; persisted to workspace if durability needed; the carrier field is absorbed. |
| R18 | State recovery / resume (the State Recovery table keyed on `phase`+`status`, plan.md L884-898) | **moved-to-lead**, **REBOUND** to workspace evidence — mochiko resumes from artifact presence, NOT a context `phase`/`status` field. *Distinctive to plan (explicit resume table); exactly the context-file `phase` field the ROADMAP memory model rebinds.* |
| R19 | Cross-context state HANDOFF (the file's reason to exist — separate-context agents read shared state from disk, plan.md L906) | **moved-to-lead** — mochiko runs the loop in one session; the lead holds state and briefs each agent with its slice via the dispatch brief; durable bits in workspace. |
| R20 | Ephemeral lifecycle (supervisor creates at Phase 1.1; deletes on "Start fresh" L224) | **dropped + reason** — in-session + workspace state needs no ephemeral file to manage. Lead must accept. |
| R21 | Intra-template duplication (Feature Context + Codebase Context tables re-surface `phase`/`feature_id`/`project_type`/`constitution_path`/`codebase_analysis_path` as human-readable summaries) | **dedupe** — under absorb the lead holds each fact once (in-session or workspace); the duplicated summary rows collapse. |
| R22 | Shared `.workflow/`-context-carrier schema across specify/plan/tasks/implement | **moved-to-other-cluster** (note) — absorbing here continues the workspace-as-state precedent; tasks/implement inherit in-session-lead + workspace-as-state, NOT a re-ported context-carrier. Reconcile records the cross-cluster implication (F-P11-4). |

No responsibility left untagged. Drops (R1, R6, R20) carry reasons for human-gate acceptance; all
non-dropped state responsibilities route to the lead (R16 + R22 route to other-cluster skills/peers);
R2/R3/R4/R7/R8/R11/R15/R17/R18 feed reconcile's rehome map.

## Reconcile flags (relational / rehome signals — for reconcile-cluster)

- **F-P11-1 (rehome coordination; PAIRS WITH P1).** The carrier's state/loop/dispatch responsibilities
  (R2, R3, R4, R7, R8, R10, R11, R15, R17, R18) re-home onto the **same lead** that P1
  (`redesign × absorb-into-lead`) dissolves the supervisor into. These are slices of the **one**
  dissolving supervisor's state machinery (P1 §C drops the carrier and §F flags this absorb).
  Rehome-orchestration (Job 2) must build **one** coherent map so each responsibility is homed exactly
  once — `phase`→in-session loop position + workspace evidence; `analysis_status`/`design_status` + six
  `*_status`→**workspace file presence**; `status`→in-session loop state; `iteration`→**bounded**
  round-counter (contract §3); `## File Paths`→workspace-as-state layout; `## Supervisor Instructions`→
  in-session dispatch (`agent-dispatch`); `## Clarification Log`→in-session human-gate history; State
  Recovery→workspace-evidence resume — with **no double-home and no drop between primitives.** Resolve
  P11 **together with P1**.
- **F-P11-2 (RQ1-coupled report-path shape) — P11's distinctive dependency.** The carrier's three report
  paths (R11) include `architect_report_path` + `advocate_report_path`, reflecting plan's **two**
  reviewers. Their rehome shape **depends on RQ1** (reviewer architecture, unresolved): (a) keep two
  distinct validators → two reviewer-report homes under the workspace; (b) fold feasibility into the
  advocate → one report home; (c) rehome feasibility onto the generic `validator` → its report home.
  The carrier's report slice **cannot be finalized until RQ1 lands**. Unlike specify's twin (fixed
  analyst+advocate, no such coupling). Resolve **after RQ1**.
- **F-P11-3 (producer-side content must not vanish with the carrier) — highest-volume silent-drop
  surface.** HIL's `## Supervisor Instructions` blocks physically host producer/reviewer briefing
  content (R16): the analyst's skill hints + **IP-XXX infrastructure planning** + the advocate's
  **focus-area checklists** (FR-coverage, orphan-TRs, NFR-measurability, constraint-actionability,
  sensitivity-contract / integration-contract / infrastructure-design alignment). When the carrier
  drops, this content survives ONLY if the producer (`technical-analyst` + `authoring-technical-
  requirements` / `patterns-*`) and validator (`validation-plan-artifacts`) **skills** carry it — all
  `[ ]` not-yet-ported. Reconcile must confirm the landing place exists. Pairs with P1 §D + the producer/
  validator skill ports. Mirrors P1's flagged silent-drop risk; the carrier is where the content
  physically lives, so naming it here keeps it from being read as a silent drop of the carrier.
- **F-P11-4 (cross-workflow / cross-cluster).** `plan-context-template` is one instance of the
  `.workflow/`-context-carrier genre shared by specify/plan/tasks/implement. Absorbing it continues the
  workspace-as-state precedent set by setup + specify. Record the cross-cluster implication:
  **tasks/implement inherit in-session-lead + workspace-as-state, NOT a re-ported context-carrier.**
- **F-P11-5 (human gate; contract §4a / RQ5).** The absorb and its accepted drops (R1, R6, R20) are an
  explicitly human-gated disposition at Phase 2 reconcile (the contract names "P11 plan-context-template
  expected absorb-into-lead"; RQ5). The lead must accept the absorb + the drops before the run is
  declared DONE.

## State that risks SILENT LOSS (call out for the lead/human to accept)

1. **Two-phase state (R2/R7) — `phase` / `analysis_status` / `design_status`.** Plan's defining shape is
   the 2-phase analysis→design loop. A thin rewrite that treats plan as single-phase would flatten it.
   Must survive as the explicit 2-phase sequence on the lead (P1 §B already tags this `moved-to-lead`;
   the carrier's slice must land on the same map). **Risk: medium** — named in P1, but the carrier holds
   the per-phase aggregate flags, so the workspace-evidence mapping must be explicit.
2. **Six per-artifact statuses (R8) → workspace file presence.** The run-goal's literal requirement. The
   mapping (status field ⇒ "artifact file exists + cleared validation") must be made explicit so the
   lead's state-recovery actually reads workspace evidence. **Risk: medium.**
3. **Three report paths / RQ1 coupling (R11).** If RQ1 folds to one reviewer and nobody re-homes
   feasibility's report, the architect-report slice is silently lost. Tied to RQ1. **Risk: medium** —
   explicitly flagged (F-P11-2).
4. **Producer-side content in Supervisor Instructions (R16).** IP-XXX infra planning, data-sensitivity,
   integration boundaries, advocate focus-area checklists. The thin command drops the instruction blocks
   wholesale; these capabilities survive ONLY if the producer/validator skills carry them. **Risk: high**
   — highest-volume silent-drop surface (F-P11-3); the skills are not yet ported.
5. **State-recovery rebind (R18).** If transliterated, the lead would look for a `phase`/`status` field
   that no longer exists. Must rebind to workspace evidence (artifact presence). **Risk: medium.**
6. **Clarification Log (R17).** Multi-round human Q&A history (feasibility + clarification). If dropped,
   the loop loses its preference-gap memory. Must be held in-session (persisted to workspace if
   durability needed). **Risk: low-medium.**

## The absorb-vs-standalone finding (with the setup + specify comparison)

**FINDING: `absorb-into-lead` is the strongly-evidenced disposition; `standalone` is refuted. The
ROADMAP memory-model Key Decision (ex-OQ#3) is CONFIRMED a THIRD time — and plan provides the strongest
"more state, still dissolves" evidence yet. The structural call is FLAGGED for reconcile (F-P11-1/2/5),
not unilaterally resolved at assess.**

**Why absorb, not standalone.** plan-context-template is a **state carrier, not a deliverable.** Every
field is one of: (a) loop/phase state (`type`, `phase`, `status`, `iteration`, `analysis_status`,
`design_status`, `created`/`updated`); (b) a per-artifact status registry derivable from workspace file
presence (six `*_status` + the `## File Paths` registry); (c) a supervisor→agent dispatch channel
mochiko replaces with in-session dispatch (`## Supervisor Instructions`); (d) cross-cluster inputs the
lead reads from `.mochiko/memory/` (`## Constitution Principles`, project-type/codebase-analysis/age);
(e) human-gate history (`## Clarification Log`); (f) resume state mochiko reads from workspace evidence
(the State Recovery table). **None is reusable body content.** plan.md confirms the coupling is
structural and total — the carrier exists *only* because HIL ran agents in separate contexts (L906) and
resumes from its `phase`+`status` fields (L884-898). A carrier whose every producer is the dissolving
supervisor has nothing to stand on as a standalone artifact.

**Setup + specify comparison — same kind, richest case.** This is the cross-cluster **twin** of setup's
`constitution-context-template` (P8) and specify's `context-template` (P14): same genre (ephemeral
supervisor-driven state carrier), same field shape (type/iteration/created frontmatter; project/codebase
context; Supervisor Instructions; Clarification Log), same doctrine ("supervisor owns… creates/updates/
deletes… agents read only"). Setup resolved P8 → **absorb-into-lead (body drop)**; specify resolved P14 →
**absorb-into-lead (body drop)**. plan is the **strongest confirmation yet** on three counts:
1. **Live, no drift.** Like specify (and unlike setup's drifted P8), plan.md **uses the template directly
   and by name** (L232) and resumes from its fields (L884-898) — the absorb rests on observed
   consumption.
2. **Most state, still dissolves.** plan carries *more* state than specify — a **two-phase** loop
   (`phase`/`analysis_status`/`design_status`), **six** per-artifact statuses, **three** reviewer-report
   paths, a constitution-principles transport, and a brownfield codebase-context transport with a
   staleness field — yet **all of it** still dissolves cleanly into in-session lead state +
   workspace-as-state + `.mochiko/memory/` reads. The harder case passing is the stronger evidence.
3. **The explicit State Recovery table is the cleanest "phase-field → workspace-evidence" case.** plan is
   the only twin with a formal resume table keyed on the context `phase`+`status` — the exact field the
   ROADMAP memory model replaces with workspace evidence. Its clean rebind is direct confirmation of the
   Key Decision.

**Same split in all three clusters.** Templates divide along the deliverable/state-carrier line
identically: setup = 2 standalone deliverables + 1 absorbed carrier; specify = 3 standalone deliverables
+ 1 absorbed carrier; plan = standalone deliverables (plan-template P10, techanalyst-report P12,
architect-report P13, advocate-report P15) + 1 absorbed carrier (P11).

**Recommendation to the lead:** with three clusters now agreeing, the memory-model Key Decision holds
unshaken; plan-context-template's absorb is the expected fate. Confirm at reconcile **alongside P1** (one
coherent rehome map), **after RQ1** (the report-path shape), and accept the drops (R1/R6/R20) at the
human gate (RQ5). The one upgrade to surface: re-homing `iteration` into the lead **binds** HIL's
explicitly-unbounded loop ("no hard caps", L905) — an improvement, traced as `moved-to-lead`, not a drop.

---

### Summary block

```
ASSESSMENT: plan-context-template.md (P11)
Class:        template → branch artifact  (functionally = dissolving-supervisor state carrier)
Triage:       gate1=y gate2=y gate3=y(moot)  [full-lens]
Disposition:  drop (body) × absorb-into-lead (structural) — emitted as FLAG-FOR-RECONCILE
              [standalone REFUTED; setup+specify precedent CONFIRMED a 3rd time]
              — absorb is the strongly-evidenced expectation; flagged because the rehome is RELATIONAL
                (pairs with P1, one coherent map), RQ1-COUPLED (report-path shape), and HUMAN-GATED (RQ5)
Trace:
  - R1  type: plan-request node-kind              → dropped + reason (state-file bookkeeping)
  - R2  phase (analysis→design→completed)         → moved-to-lead (in-session + workspace evidence) [2-phase]
  - R3  within-phase status                       → moved-to-lead (in-session loop state)
  - R4  iteration/pass counter                    → moved-to-lead (in-session; now BOUNDED — upgrade vs "no hard caps")
  - R5  feature identity (feature_id)             → moved-to-lead (lead tracks workspace dir)
  - R6  created/updated timestamps                → dropped + reason (state-file bookkeeping)
  - R7  analysis_status/design_status             → moved-to-lead (workspace evidence) [2-phase aggregate]
  - R8  six per-artifact statuses                 → moved-to-lead (WORKSPACE FILE PRESENCE)
  - R9  entry-gate spec handoff (spec_status)     → moved-to-lead (workspace evidence; cross-cluster→specify)
  - R10 workspace path registry (## File Paths)   → moved-to-lead (workspace-as-state layout)
  - R11 three report paths (analyst/architect/advocate) → moved-to-lead (workspace-as-state) — SET IS RQ1-DEPENDENT
  - R12 constitution principles transport         → moved-to-lead (read .mochiko/memory/constitution.md; rebind)
  - R13 brownfield codebase handoff + staleness   → moved-to-lead (read .mochiko/memory/; lead computes age; cross-cluster→setup)
  - R14 free-form codebase_context transport      → moved-to-lead (analyst still produces it)
  - R15 ## Supervisor Instructions dispatch       → moved-to-lead (in-session dispatch / agent-dispatch)
  - R16 producer/reviewer content inside R15       → moved-to-other-cluster (technical-analyst + producer/validator skills) [SILENT-DROP HIGH]
  - R17 ## Clarification Log human Q&A history     → moved-to-lead (in-session human-gate history)
  - R18 State Recovery (phase+status resume table) → moved-to-lead, REBOUND to workspace evidence
  - R19 cross-context state handoff (raison d'être)→ moved-to-lead (single-session loop; brief per agent-dispatch)
  - R20 ephemeral create/delete lifecycle          → dropped + reason (in-session+workspace needs no file)
  - R21 intra-template duplicated summary rows      → dedupe (lead holds each fact once)
  - R22 shared schema across specify/plan/tasks/implement → moved-to-other-cluster (note; F-P11-4)
Reconcile flags:
  - F-P11-1 rehome coordination — PAIRS WITH P1; one coherent rehome map (Job 2); home each
            state/loop/dispatch responsibility exactly once; no double-home, no drop between primitives
  - F-P11-2 RQ1-coupled report-path shape — three report paths' rehome depends on the dual-reviewer
            architecture (RQ1); resolve AFTER RQ1 (P11's distinctive dependency vs specify's twin)
  - F-P11-3 producer-side content (IP-XXX, sensitivity, integration, advocate checklists) in
            ## Supervisor Instructions must land on producer/validator SKILLS, not vanish with the carrier
  - F-P11-4 cross-workflow — context-carrier genre shared by specify/plan/tasks/implement; peers inherit
            in-session-lead + workspace-as-state, NOT a re-ported carrier
  - F-P11-5 human gate (contract §4a / RQ5) — accept the absorb + drops R1/R6/R20 before DONE
Silent-drop risks: 2-phase state (R2/R7) · six per-artifact statuses→workspace evidence (R8) ·
            three report paths/RQ1 (R11) · producer content in Supervisor Instructions (R16, HIGH) ·
            state-recovery rebind (R18) · Clarification Log (R17)
Finding:      absorb-into-lead = strongly-evidenced (3rd cluster, richest carrier, all state dissolves);
              standalone refuted; memory-model Key Decision CONFIRMED & STRENGTHENED (most state +
              explicit phase-field State Recovery table, still dissolves); structural call FLAGGED for
              reconcile (coordination + RQ1 + human gate), not resolved solo
```
