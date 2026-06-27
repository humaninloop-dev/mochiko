# Assessment — `templates/context-template.md` (P14)

Run: `specify` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/context-template.md`
Described role: the **workflow context/state carrier** — created by the `specify` command from this
template, written/updated by the dissolving `state-analyst`, read by the domain agents
(`requirements-analyst`, `devils-advocate`) across passes. Shared genre across specify/plan/tasks/implement.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, path coupling.

But — exactly like setup's twin — this template is **not an output artifact; it is the dissolving
orchestration's STATE CARRIER.** Every field is loop/DAG state, a path registry, a supervisor→agent
handoff channel, or an input the lead can assemble directly. It exists solely to ferry state between
the brain/DAG-driven supervisor and separately-contexted agents, pass after pass. It is **plumbing of
the dissolving DAG/supervisor**, not a reviewable deliverable.

**Consumption is confirmed (no drift):** `commands/specify.md` references it by name as a resource
(L51) and creates `context.md` from it at "Initialize Workflow Structure" (L94: *"Create initial
`context.md` from `…/templates/context-template.md` with detected project context, user input, and
file paths"*). `feature_id` is `BRANCH_NAME` from the **excluded** `create-new-feature.sh` (L86).
Completion flips its status to `completed` (L195). The defining coupling is L243: **"Domain agents
have NO workflow knowledge — all context via files on disk."** That on-disk handoff is the precise
coupling mochiko sheds by running the loop in one session.

This is a sharper case than setup's P8, which had *drifted* (the setup command inlined its own context
block and did not name the template). Here the template is the **live, in-use** carrier.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = YES (strongly)` — it has no reason to exist outside the
   DAG/supervisor-drives-agent loop. The brain scaffolds it (`create-new-feature.sh`), the
   `specify-catalog.json` DAG sequences the nodes that write it, the `state-analyst` populates/updates
   it, agents read it because they hold no workflow state. **Orchestration-coupling = total**;
   content-coupling = the DAG vocab in the body (`type: specification-request`, `iteration`,
   `feature_id`, status lifecycle, the path registry).
2. **Multi-responsibility / fans out?** `gate2 = YES` — many bundled responsibilities (status,
   iteration, feature-id, input transport, enrichment transport, project-context transport,
   constitution-principles transport, file-path registry, supervisor-instruction channel,
   clarification log) feeding multiple consumers (supervisor, state-analyst, both domain agents) —
   and shared across four workflows.
3. **Emits a non-machine-checkable artifact?** `gate3 = YES (moot)` — free-form supervisor/agent
   prose, but it is *plumbing*, not a graded deliverable; **no producer↔validator pairing implied by
   its content.** (The graded deliverables are the sibling templates P11/P12/P13.)

gate1 + gate2 trip → **full lens.** This is the cluster's "how does the loop carry state" pivot, and
the explicit BACKLOG OQ#3 memory-model test against setup.

## Step 3 — The lens (weighted for artifact branch + orchestration)

**Check 1 — Orchestration test.** Orchestrated by the **brain/DAG + markdown supervisor + state-analyst**,
not the body alone. Content-coupling = DAG state vocab in frontmatter/sections (body work, but the
body does not survive). Orchestration-coupling = **total**: when the DAG sheds and the state-analyst
(P4) dissolves onto the mochiko command lead, the carrier's responsibilities must land somewhere
explicit. They are **moved-to-lead** candidates — the "input/prerequisite wiring → explicit handoff"
rehome that `reconcile-cluster`'s rehome-orchestration owns. Its fate is a *function of the rehomed
lead's state design*, shared with P1/P4/P9/P10.

**Check 2 — Role (two altitudes).** Skill-role analog = **transport** (carries state + instructions);
neither producer nor validator. Team-role conferred = it makes the supervisor the **lead** (owner of
per-pass instructions + loop state) and the agents **readers**. Its entire value is loop-state
transport → it emits **no reviewable artifact** → needs **no validator partner**.

**Check 3 — Independence.** No produce+grade leak — it grades nothing, holds no `skills:`. Neutral.
(It *carries* the supervisor's loop-driving instructions — a lead concern, not an independence one.)

**Check 4 — Verdict-sink / loop-driver.** This template **is the substrate the loop-driver writes to.**
`status` + `iteration` frontmatter and the `## Supervisor Instructions` + `## Clarification Log`
sections are *how* the produce→validate/clarify loop is driven across passes (specify.md Rules 1/5/6).
These are the biggest things the supervisor/DAG owned and the easiest to drop silently. Naming them
re-homes them onto the mochiko lead's **bounded** loop (contract §3: round cap 3 + no-progress + STOP).
Note: HIL's `iteration` was an **unbounded** LLM/DAG counter (specify.md Rule 6 "5 passes → surface",
soft). Re-homing it is an **upgrade**, not a like-for-like move — same shape as setup's
done-condition upgrade.

**Check 5 — Sibling / overlap ("look sideways").**
- **vs deliverable templates P11/P12/P13** (spec, analyst-report, advocate-report): those carry
  reviewable **content** and survive **standalone**; this carries **state** and does not. The
  template family splits along the deliverable/state-carrier line — identically to setup
  (constitution-template + codebase-analysis-template standalone; constitution-context-template
  absorbed).
- **vs setup's `constitution-context-template` (P8)** — the **cross-cluster twin**: same kind, same
  field genre, same "supervisor owns/creates/deletes; ephemeral" doctrine. Setup resolved it to
  **absorb-into-lead**. Strong precedent.
- **vs P1 (command) + P4 (state-analyst) + P9/P10 (strategy)** — all re-home state/loop/dispatch onto
  the **same lead**. This carrier is one slice of the **one** dissolving DAG-state machinery → its
  rehome must be coordinated, not decided in isolation.
- **Cross-workflow:** the same `context-template` genre is shared by specify/plan/tasks/implement →
  absorbing it here sets precedent for the peers (cross-cluster note for reconcile).

**Check 6 — Coupling audit.**
- **Path coupling (RUN-GOAL hit):** `## File Paths` is a registry of DAG-node artifact paths
  (spec/context/analyst-report/advocate-report) + `constitution_path`. The self-referential
  `context_path` points the file at itself. These do **not** rebind (the carrier is absorbed); they
  re-home as **workspace-as-state** on the lead (fixed layout, e.g. `.mochiko/specs/<feature>/{spec.md,
  analyst-report.md, advocate-report.md}`).
- **DAG / feature-id / pass-number assumptions (RUN-GOAL hit):** `type: specification-request`
  (DAG node-kind), `status` (DAG node status lifecycle), `iteration` (pass/round number), `feature_id`
  (DAG feature id from `create-new-feature.sh` BRANCH_NAME). All DAG-state assumptions — handled in
  the trace (identity → lead; numbering scheme → dropped; iteration → bounded lead counter).
- **References to excluded primitives (RUN-GOAL hit):** created by the excluded `create-new-feature.sh`;
  sequenced by the excluded `specify-catalog.json` DAG; written/read by the dissolving `state-analyst`
  (P4). **A carrier whose every producer and consumer is excluded or dissolving cannot stand alone.**
- **Prerequisite/handoff:** `constitution_path` + `## Constitution Principles` assume setup's
  constitution exists first → real cross-cluster handoff edge to setup (rebind to `.mochiko/memory/`).
- **Determinism boundary:** none — free-form transport.

**Check 7 — Conventions + loop placement.**
- **Classification / discoverability:** inert state-carrier; not user/model-invoked, not router-
  registered. Moot under absorb (no standalone artifact to classify or register).
- **Producer↔validator pairing:** not a deliverable → none.
- **Decoupling scan:** `## Supervisor Instructions` is a literal supervisor→agent dispatch channel
  (dispatch coupling baked into the artifact). Under absorb it becomes the lead's **in-session
  dispatch** (caller-side context → `agent-dispatch.md`, per the decouple convention), not a file
  field. No persona to scrub (it is a template), but the dispatch-channel responsibility is precisely
  what the decouple convention says belongs on the caller/lead, not the primitive.
- **Sound-loop placement (the crux):** this template is the *carrier* of the supervisor's loop state —
  done-condition signaling (status), iteration bound, human-gate inputs (clarification log). In
  mochiko these are **loop-discipline obligations of the lead**, carried **in-session + workspace**,
  not by an ephemeral file.

## Step 4 — Disposition

**Body × Structural = `drop` × `absorb-into-lead`.**

- **Structural = `absorb-into-lead` (the absorb-vs-standalone question is RESOLVED — see Finding).**
  Standalone is **refuted**; the carrier folds into the command lead (in-session state +
  workspace-as-state + `.mochiko/memory/` reads), leaving no orphan template. `absorb-into-lead` is a
  move onto the lead (not a sibling-relational move), so it is properly callable at assess. What is
  **deferred to reconcile is the rehome COORDINATION, not the absorb decision** — the responsibilities
  land on the *same* lead as P1/P4/P9/P10 and must be homed once and coherently (rehome-orchestration,
  Job 2), plus the cross-workflow implication and the contract-§4 human gate. See Reconcile flags.
- **Body = `drop`** — the artifact file is **not produced**; nothing of the body survives as template
  *content* (it is placeholder scaffolding for DAG state, not reusable prose). This is an artifact-level
  drop; at the *responsibility* level almost everything is `moved-to-lead` (see trace), with a few
  genuine drops (each with a reason). Matches setup P8's realized outcome ("absorb-into-lead (body
  drop) … not produced"). The **only** world where body becomes `port-with-edits` is the refuted
  `standalone` path (rebind `.mochiko/` paths) — recorded for completeness, not proposed.

## Step 5 — Responsibility trace (every responsibility tagged — no silent loss)

| # | Responsibility (field / section) | Tag |
|---|----------------------------------|-----|
| R1 | Workflow status lifecycle (`status`: specification-request → … → completed) | **moved-to-lead** — in-session loop state on the supervisor |
| R2 | Iteration / pass counting (`iteration`) | **moved-to-lead** — in-session round counter, now **bounded** by contract §3 (round cap 3 + no-progress + STOP). *Upgrade*, not like-for-like (HIL's was unbounded). |
| R3a | Feature identity ("which feature/workspace") (`feature_id`) | **moved-to-lead** — lead tracks the active feature/workspace dir in-session (e.g. `.mochiko/specs/<feature>/`) |
| R3b | DAG feature-numbering scheme + `create-new-feature.sh` scaffolding | **dropped + reason** — excluded brain script; kernel-free. No capability lost (lead derives the workspace dir). Lead must accept. |
| R4 | Input-source provenance (`input_source`, `original_input`) | **moved-to-lead** — lead holds the raw user request in-session |
| R5 | State-file timestamps (`created`, `updated`) + node-kind marker (`type`) | **dropped + reason** — DAG state-file bookkeeping; workspace mtime / git history covers any need; node-typing has no kernel-free meaning. Lead must accept. |
| R6 | Carry user input to the agent (`## User Input`) | **moved-to-lead** — held in-session; assembled into the producer's dispatch prompt |
| R7 | Carry input-enrichment output (`## Input Enrichment`) | **moved-to-lead** — `analysis-iterative` (P5) still *produces* enrichment; it lands in-session/workspace and the lead assembles it, not this carrier |
| R8 | Carry detected project context (`## Project Context`: name/stack/constitution) | **moved-to-lead** — workspace-as-state: lead reads project context from `.mochiko/memory/` (setup memory-model precedent) |
| R9 | Inline constitution principles (`## Constitution Principles`, `constitution_path`) | **moved-to-lead** (cross-cluster handoff) — lead points the agent at `.mochiko/memory/constitution.md` (setup's output home) instead of copying principles into a context file; rebind `.humaninloop/`→`.mochiko/` |
| R10 | File-path registry (`## File Paths`: spec/analyst-report/advocate-report) | **moved-to-lead** — workspace-as-state: lead knows the fixed `.mochiko/specs/<feature>/` layout |
| R11 | Self-referential `context_path` | **dropped + reason** — no separate context file exists to point at under absorb. Lead must accept. |
| R12 | Per-pass supervisor→agent dispatch instructions (`## Supervisor Instructions`) | **moved-to-lead** — in-session dispatch: the lead builds each agent's prompt directly (agent-dispatch pattern, not a file field) |
| R13 | Multi-round human Q&A history (`## Clarification Log`) | **moved-to-lead** — human-gate history held in-session by the lead; persisted to workspace if durability needed; the carrier-field is absorbed |
| R14 | Cross-context state HANDOFF (the file's reason to exist — separate-context agents read shared state from disk, specify.md L243) | **moved-to-lead** — mochiko runs the loop in one session; the lead holds state and gives each agent its slice via the dispatch prompt; durable bits in workspace |
| R15 | Ephemeral lifecycle (supervisor creates/updates/deletes; "removed when workflow finishes") | **dropped + reason** — in-session + workspace state needs no ephemeral file to manage. Lead must accept. |
| R16 | Shared state schema across specify/plan/tasks/implement | **moved-to-other-cluster** (note) — absorbing here sets the workspace-as-state precedent for the peers; they inherit it rather than re-port a context-template. Reconcile records the cross-cluster implication. |

No responsibility left untagged. Drops (R3b, R5, R11, R15) carry reasons for human-gate acceptance;
all non-dropped responsibilities route to the lead. R2/R3a/R12/R13/R14 feed reconcile's rehome map.

## Reconcile flags (relational / rehome signals — for reconcile-cluster)

- **F-CTX-1 (rehome coordination; PAIRS WITH P1 + P4 + P9/P10).** The carrier's state/loop/dispatch
  responsibilities (R1, R2, R3a, R10, R12, R13, R14) re-home onto the **same lead** as the
  `state-analyst` dissolution (P4) and the strategy survivors (P9/P10). These are slices of the **one**
  dissolving DAG-state machinery. Rehome-orchestration (Job 2) must build **one** coherent map so each
  responsibility is homed exactly once — `iteration`→bounded round-counter, `status`→loop-state,
  `## Supervisor Instructions`→in-session dispatch, `## File Paths`→workspace-as-state,
  `## Clarification Log`→in-session human-gate history — with **no double-home and no drop between
  primitives.** Resolve P14 together with P1 (lead), P4 (state-analyst), P9/P10 (strategy).
- **F-CTX-2 (cross-workflow / cross-cluster).** `context-template` is shared by
  specify/plan/tasks/implement. Absorbing it in specify-core removes the carrier and sets the
  workspace-as-state precedent for the peers. Record the cross-cluster implication: **plan/tasks/
  implement inherit in-session-lead + workspace-as-state, NOT a re-ported context-template.**
- **F-CTX-3 (human gate; per contract §4a).** The absorb and its accepted drops (R3b, R5, R11, R15)
  are an explicitly human-gated disposition at Phase 2 reconcile (the contract names "the
  context-template absorb"). The lead must accept the drops before the run is declared DONE.

## The absorb-vs-standalone finding (with the setup comparison)

**FINDING: `absorb-into-lead` — CONFIRMED. `standalone` is refuted. The setup memory-model decision
(BACKLOG OQ#3) is CONFIRMED, not refuted — and specify strengthens it.**

**Why absorb, not standalone.** context-template is a **state carrier, not a deliverable.** Every
field is one of: (a) DAG/loop state (`type`, `status`, `iteration`, `feature_id`, `created`/`updated`);
(b) a path registry derivable from a fixed workspace layout (`## File Paths`); (c) a supervisor→agent
handoff channel mochiko replaces with in-session dispatch (`## Supervisor Instructions`); (d) inputs
the lead assembles directly or reads from `.mochiko/memory/` (`## User Input`, `## Input Enrichment`,
`## Project Context`, `## Constitution Principles`); (e) human-gate history (`## Clarification Log`).
**None is reusable body content.** specify.md confirms the coupling is structural and total — the
carrier exists *only* because HIL ran agents in separate contexts mediated by a DAG (L243), and it is
created by the excluded `create-new-feature.sh`, sequenced by the excluded `specify-catalog.json`, and
driven by the dissolving `state-analyst`. A carrier whose every producer and consumer is excluded or
dissolving has nothing to stand on as a standalone artifact.

**Setup comparison — same kind, cleaner case.** This is the cross-cluster **twin** of setup's
`constitution-context-template` (P8): same genre (ephemeral supervisor/brain-driven state carrier),
same field shape (type/iteration/created frontmatter; User Input; Project Context table; Supervisor
Instructions; Clarification Log), same doctrine ("Supervisor owns… creates/modifies/deletes… context
is ephemeral"). Setup resolved P8 to **absorb-into-lead (body drop; not produced)** (setup report L23).
specify is a **stronger** confirmation on two counts:
1. **No drift.** Setup's P8 had drifted off the live command (the command inlined a different context
   block and never named the template), so its assessor could not even confirm use → it landed at
   `flag-for-reconcile` with absorb as the leading candidate. specify.md **uses context-template
   directly and by name** (L51, L94) — it is the live carrier, so the absorb rests on observed
   consumption, and the structural call can be made at assess.
2. **More state, still dissolves.** specify carries *more* state than setup — per-feature
   `feature_id`, a multi-agent file-path registry, an explicit produce→validate iteration loop,
   analyst + advocate reports — yet **all of it** still dissolves cleanly into in-session lead state +
   workspace-as-state + `.mochiko/memory/` reads. The harder case passing is the stronger evidence for
   the memory model.

**Same split in both clusters.** Templates divide along the deliverable/state-carrier line identically:
setup = 2 standalone deliverables (constitution-template, codebase-analysis-template) + 1 absorbed
carrier; specify = 3 standalone deliverables (spec P11, analyst-report P12, advocate-report P13) + 1
absorbed carrier (P14).

**Recommendation to the lead:** with two clusters now agreeing, the run goal's structural call —
*memory model = in-session + `.mochiko/memory/` + workspace-as-state; no separate context-handoff
artifact* — is ready to **promote to ROADMAP Key Decisions** and close BACKLOG OQ#3 (lead/human-gate
to confirm). One upgrade to surface at the gate: re-homing `iteration` into the lead **binds** the
previously-unbounded loop (contract §3) — an improvement, traced as `moved-to-lead`, not a drop.

---

### Summary block

```
ASSESSMENT: context-template.md (P14)
Class:        template → branch artifact  (functionally = dissolving DAG/supervisor state carrier)
Triage:       gate1=y gate2=y gate3=y(moot)  [full-lens]
Disposition:  drop (body) × absorb-into-lead (structural)   [standalone REFUTED; setup precedent CONFIRMED]
              — absorb decision RESOLVED at assess; rehome COORDINATION deferred to reconcile (F-CTX-1)
Trace:
  - R1  status lifecycle                         → moved-to-lead (in-session loop state)
  - R2  iteration/pass counter                   → moved-to-lead (in-session round counter; now BOUNDED — upgrade)
  - R3a feature identity (which feature/workspace)→ moved-to-lead (lead tracks workspace dir)
  - R3b DAG numbering + create-new-feature.sh     → dropped + reason (excluded brain script)
  - R4  input-source provenance                  → moved-to-lead
  - R5  created/updated timestamps + type marker  → dropped + reason (DAG state-file bookkeeping)
  - R6  user input transport                     → moved-to-lead (assembled into dispatch prompt)
  - R7  input-enrichment transport               → moved-to-lead (analysis-iterative still produces it)
  - R8  project-context transport                → moved-to-lead (workspace-as-state; .mochiko/memory/)
  - R9  constitution-principles transport        → moved-to-lead (cross-cluster: read .mochiko/memory/constitution.md)
  - R10 file-path registry                       → moved-to-lead (workspace-as-state layout)
  - R11 self-referential context_path            → dropped + reason (no context file to point at)
  - R12 supervisor→agent dispatch instructions   → moved-to-lead (in-session dispatch)
  - R13 clarification-log human Q&A history       → moved-to-lead (in-session human-gate history)
  - R14 cross-context state handoff (raison d'être)→ moved-to-lead (single-session loop)
  - R15 ephemeral create/update/delete lifecycle  → dropped + reason (in-session+workspace needs no file)
  - R16 shared schema across specify/plan/tasks/implement → moved-to-other-cluster (note; F-CTX-2)
Reconcile flags:
  - F-CTX-1 rehome coordination — PAIRS WITH P1(lead)+P4(state-analyst)+P9/P10(strategy); one coherent
            rehome map (Job 2); home each state/loop/dispatch responsibility exactly once
  - F-CTX-2 cross-workflow — context-template shared by specify/plan/tasks/implement; peers inherit
            in-session-lead + workspace-as-state, NOT a re-ported carrier
  - F-CTX-3 human gate (contract §4a) — accept the absorb + drops R3b/R5/R11/R15 before DONE
Finding:      absorb-into-lead CONFIRMED; standalone refuted; setup OQ#3 memory model CONFIRMED &
              STRENGTHENED (cleaner case: no drift + more state, still dissolves) → promote to ROADMAP
```
