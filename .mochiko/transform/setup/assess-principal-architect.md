# Assessment — `agents/principal-architect.md`

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source: `human-in-loop/plugins/humaninloop/agents/principal-architect.md`
Class: **agent → PLAYS-a-role branch**

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting per branch: persona-vs-procedure split, the team-role it
confers on its caller, and `skills:`-list independence carry the most weight. Loop-ownership is
NOT this primitive's concern (that sits on the supervisor `commands/setup.md`, P1).

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** It only functions when the **markdown supervisor**
   `commands/setup.md` drives it: the supervisor writes a context file, dispatches the agent via
   `Task(subagent_type: principal-architect, …)`, selects which of the agent's many hats to wear
   per phase, loops on clarifications (max 3), and presents the human checkpoints. (Supervisor-
   coupling, NOT kernel — setup is already kernel-free.)
2. **Multi-responsibility / fans out?** **YES.** Seven distinct responsibilities across six mounted
   skills + a baked-in feasibility-review procedure; feeds multiple consumers (constitution,
   analysis, roadmap, CLAUDE.md, feasibility verdict).
3. **Non-machine-checkable artifact?** **YES.** Constitutions, codebase analyses, roadmaps, and
   feasibility verdicts are all model-judgment outputs — no version-equality/schema assert can
   stand in for grading them. A real grounded validator partner is required (not a degenerate
   deterministic assert).

All three **yes → full 7-check lens.** Gates tripped: 1, 2, 3.

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer:** the **markdown supervisor** `commands/setup.md` (Phases 0–5). It owns
  sequencing, mode routing (brownfield/greenfield/amend), the clarification loop, checkpoint gates,
  context-file injection, and state recovery. No kernel/DAG/MCP/catalog anywhere in this path.
- **Content-coupling:** **minimal/none.** The agent body references no kernel, DAG, catalog JSON, or
  MCP tools. It says "Write outputs to the locations specified in your instructions" — paths are
  injected by the supervisor's context file, not hardcoded in the persona. Body is mochiko-clean.
- **Orchestration-coupling:** the agent is a **dispatched leaf**. It relies on the supervisor for
  which-hat-per-phase, path injection, the FAIL/clarification loop, and the checkpoints. Those are
  the supervisor's (P1) responsibilities; when the supervisor re-homes onto the mochiko lead, they
  land on the lead — **not** on this agent. This agent owns **no** loop-driving to re-home.

### 2. Role at two altitudes
- **Skill-role:** artifact-emitter (wraps several emits-artifact skills). Every output is a
  reviewable artifact → needs an independent producer↔validator partner.
- **Team-role conferred:** **conflicted.** authoring-constitution + brownfield-constitution +
  analysis-codebase + (cross-cutting) syncing-claude-md/authoring-roadmap confer **producer**; but
  validation-constitution confers **validator** of its own output, and the baked-in feasibility
  review confers **referee**. One agent currently confers producer AND validator AND referee — a
  mochiko agent should confer one.

### 3. Independence — CENTRAL FINDING
- **Self-grade leak: YES.** The `skills:` list mounts both the producing skills
  (`authoring-constitution`, `brownfield-constitution`) **and** the grading skill
  (`validation-constitution`). The same agent writes the constitution and grades it — the textbook
  leak that is invisible at the skill level and visible only in the agent's `skills:` list
  (synthesis point 7: "the controller must never grade its own ungrounded sensor").
- **Second smell:** the baked-in Feasibility Review (a referee/validation procedure) lives inside
  the producer persona too.
- **Fix signal (relational):** **split the agent** — move `validation-constitution` to a NEW
  independent peer validator agent (`moved-to-validator`). → `flag-for-reconcile`.

### 4. Verdict-sink / loop-driver
- **Consumer of output:** the setup supervisor (P1) consumes `architect-report.md`, presents
  checkpoints, and decides iterate-vs-finalize; the `constitution.md` flows downstream to the wider
  workflow (`specify`/`plan`).
- **What loops on FAIL today:** only the supervisor's *clarification* loop (max 3) and the user's
  Edit/Reject at checkpoints. There is **no independent-validation FAIL loop** — the agent
  "validates" itself. That missing gate is the rehome-orchestration item for reconcile.
- **Where loop-driving re-homes:** onto the mochiko command supervisor/lead (ported P1), never onto
  this agent.

### 5. Sibling / overlap ("look sideways")
- Only agent in this cluster, so no agent-sibling collision. But its *mounted* skills overlap among
  themselves (`authoring-constitution` ⊃ `brownfield-constitution` thin-variant) — that merge signal
  belongs to those skills' own assessments (P3/P4), not here.
- The sideways signal originating HERE: this agent **needs a new sibling validator agent** to
  receive the grading responsibility (pairing), and its orphaned feasibility procedure needs a home
  (new skill, likely in a different cluster). → `flag-for-reconcile`.

### 6. Coupling audit
- **Hardcoded paths in body:** none. Output paths come from the supervisor's injected instructions;
  the `.humaninloop/`→`.mochiko/` rebind therefore lives in P1 + the skills, not this persona. The
  "write where instructed" handoff itself is `kept-but-rebind`.
- **`skills:` references:** 4 in-scope rebind `humaninloop:`→`mochiko:` (one of which — 
  `validation-constitution` — actually leaves this agent, see trace); 2 cross-cutting
  (`syncing-claude-md`, `authoring-roadmap`) → other-cluster.
- **Upstream prerequisites:** brownfield/roadmap phases assume `codebase-analysis.md` already exists
  — a handoff edge owned by the supervisor's contract, not the agent.
- **Determinism boundary:** all of this agent's work is model judgment (governance authoring,
  analysis narrative, feasibility reasoning). No deterministic script in the agent
  (`detect-stack.sh` lives in the analysis-codebase skill). ⇒ the validator partner is a **real
  grounded validator**, not a degenerate assert.

### 7. Conventions + loop placement
- **Persona-vs-procedure split:** mostly clean persona (Three-Part Rule, essential-floor philosophy,
  reject-vague taste, opinionated defaults = CARES → keep) **except** the **Feasibility Review**
  section (a concrete HOW: hunt-list + verdict vocabulary + write-location) baked into the persona →
  must `folded-into-skill`; and the **Essential Floor Knowledge** table duplicates reference content
  the skills already carry → `dedupe`.
- **Classification/`skills:` list:** present; will be cleaned in the wiring pass.
- **Router registration:** absent (mochiko router is new) → wiring-pass item.
- **Producer↔validator pairing:** **MISSING** — the agent grades itself. Loop gap → reconcile.
- **Sound-loop:** the setup loop has a done-condition + human checkpoints but **no independent
  validation gate**. That gap is the rehome-orchestration item for reconcile (lands on P1's loop +
  the new validator).

---

## Step 4 — Disposition

- **Body treatment (proposable now): `port-with-edits`.** The persona is mochiko-clean (no
  kernel/DAG/catalog). The edits are localized and structure-preserving: remove
  `validation-constitution` from the `skills:` list, lift the Feasibility Review procedure out of the
  persona, dedupe the essential-floor table, rebind skill namespaces. Not `redesign` (no kernel
  assumption to rewrite around); not `keep-verbatim` (self-grade leak + orphaned procedure must be
  fixed).
- **Structural move: `flag-for-reconcile` (signal: `split` — producer↔validator pairing).** The
  split depends on a sibling that does not yet exist (a new independent constitution-validator
  agent) and on where the orphaned feasibility procedure lands. Per the skill, do not guess a
  relational move solo — reconcile decides with full cluster context.

---

## Step 5 — Responsibility trace (complete; no silent drops)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Governance persona & judgment (Three-Part Rule, reject-vague, opinionated defaults, essential-floor philosophy, "What You Produce/Reject/Embrace") | `kept` |
| 2 | Greenfield constitution authoring (via `authoring-constitution`) | `kept-but-rebind` (`humaninloop:`→`mochiko:authoring-constitution`) |
| 3 | Brownfield constitution authoring (via `brownfield-constitution`) | `kept-but-rebind` (rebind skill ref) |
| 4 | Codebase analysis (via `analysis-codebase`) | `kept-but-rebind` (rebind skill ref) |
| 5 | Constitution quality grading / validation (via `validation-constitution`) | `moved-to-validator` (self-grade leak → new independent constitution-validator agent; relational — assess flags, reconcile assigns) |
| 6 | CLAUDE.md governance sync (via `syncing-claude-md`) | `moved-to-other-cluster` (cross-cutting; not ported this run — reference rebind deferred) |
| 7 | Evolution roadmap authoring (via `authoring-roadmap`) | `moved-to-other-cluster` (cross-cutting; not ported this run) |
| 8 | Cross-artifact feasibility review (baked-in procedure + verdict vocabulary, body lines 127–148) | `folded-into-skill` (orphaned HOW lifted out of persona) — **reconcile must decide its home cluster/role** (operates over specify/plan artifacts + confers a referee role → likely belongs to a peer cluster, not setup-core) |
| 9 | Essential Floor Knowledge table (body lines 111–126) | `dedupe` (canonical home = `authoring-`/`brownfield-`/`validation-constitution` skills, not duplicated in persona; relational — reconcile confirms) |
| 10 | "Write outputs to locations specified in instructions" (path-injection handoff) | `kept-but-rebind` (`.humaninloop/`→`.mochiko/` rebind lives in P1's handoff; agent keeps "write where instructed") |

**Orchestration note:** sequencing, mode-routing, the clarification loop, and the human checkpoints
are NOT this agent's responsibilities — they belong to the supervisor (P1). No `moved-to-lead` tag
originates from this primitive. The independence/validation-gate gap (below) does feed P1's loop.

No responsibility is dropped. Trace is complete → assessment done-condition met.

---

## Reconcile flags (for `reconcile-cluster`)

1. **Self-grade leak → split / producer↔validator pairing.** Move `validation-constitution`
   (responsibility #5) off this agent onto a **new independent constitution-validator agent**. Never
   resolve by leaving produce + grade on one agent. Coordinates with P5's own assessment
   (`validation-constitution` is expected to flag `promote` into exactly this validator). Partner
   required: new `constitution-validator` agent + ported `validation-constitution` skill.
2. **Orphaned feasibility-review procedure (#8).** Decide: new standalone skill vs
   `moved-to-other-cluster`. It reads requirements/constraints/NFRs/decisions (specify/plan
   artifacts, not the constitution) and confers a referee/validator role → strong signal it is out
   of setup-core scope. Reconcile assigns the final home.
3. **Essential-floor duplication (#9).** Confirm `dedupe` — canonical home is the constitution
   skills; persona should reference, not duplicate.
4. **Missing independent-validation gate (rehome-orchestration).** The setup loop never had an
   external validation gate. Reconcile's rehome-orchestration must add it: the new validator (from
   flag 1) + the bounded FAIL-loop on P1's supervisor. (Originates from this agent's self-grade
   structure; lands on P1 + the new validator.)

---

## Output block

```
ASSESSMENT: principal-architect
Class:        agent → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × flag-for-reconcile: split (producer↔validator pairing — needs new validator agent)
Trace:
  - governance persona & judgment                       → kept
  - greenfield constitution authoring                   → kept-but-rebind (mochiko:authoring-constitution)
  - brownfield constitution authoring                   → kept-but-rebind (mochiko:brownfield-constitution)
  - codebase analysis                                   → kept-but-rebind (mochiko:analysis-codebase)
  - constitution quality grading (validation-constitution) → moved-to-validator (new constitution-validator agent)
  - CLAUDE.md governance sync (syncing-claude-md)       → moved-to-other-cluster
  - evolution roadmap authoring (authoring-roadmap)     → moved-to-other-cluster
  - cross-artifact feasibility review (baked-in proc)   → folded-into-skill (home cluster/role → reconcile)
  - essential-floor knowledge table                     → dedupe (canonical home = constitution skills)
  - "write to locations specified in instructions"      → kept-but-rebind (.humaninloop/ → .mochiko/, via P1 handoff)
Reconcile flags:
  - split: move validation-constitution to a new independent constitution-validator agent (pairs with P5 promote)
  - feasibility-review procedure: new-skill vs moved-to-other-cluster (likely peer cluster; referee role)
  - dedupe: essential-floor table → constitution skills, not persona
  - rehome-orchestration: add the missing independent-validation gate to P1's loop + new validator
```
