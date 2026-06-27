# Assessment — `templates/constitution-context-template.md` (P8)

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/constitution-context-template.md`
Described role: the **OLD supervisor's context-handoff artifact** — written by the setup command, read by `principal-architect` across iterations.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, and path coupling.

But this template is **not an output artifact** — it is **supervisor scaffolding**. Its own Usage Notes state it plainly: *"Supervisor owns this artifact — creates, modifies, and deletes it; Agent reads only; context is ephemeral, removed when workflow finishes."* It exists solely to carry state (project context, supervisor instructions, clarification history, iteration count) from the markdown supervisor to the dispatched agent and back, round after round. It is the **state-carrying mechanism of the dissolving supervisor**, not a deliverable.

**Drift finding (important):** the *current* `commands/setup.md` does **not** reference this template by name. setup.md **inlines its own, richer** context block (`type: brownfield-setup`, with Detection Results / phase / constitution_mode), while this template is a simpler, earlier `type: constitution-setup` (`mode: create|amend`) shape. The two have **drifted**: this template represents a pre-brownfield-multi-phase version of the handoff. So it is effectively a **stale/parallel** copy of a responsibility the command now carries inline.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = YES (strongly)` — the template has **no reason to exist** outside the supervisor-drives-agent loop. It is pure markdown-supervisor orchestration scaffolding: the supervisor populates `## Project Context`, injects iteration-specific `## Supervisor Instructions`, appends `## Clarification Log` rounds, bumps `iteration` in frontmatter, and deletes the file at the end. This is **orchestration-coupling**, not content-coupling — and **not kernel**: it is the markdown supervisor's state file.
2. **Multi-responsibility / fans out?** `gate2 = YES` — two consumers (setup command = writer/owner; principal-architect = reader) and multiple responsibilities bundled (context-transport, supervisor-instruction injection, conversation-history/clarification log, iteration tracking, ephemeral lifecycle).
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` (moot) — free-form supervisor prose, not schema-checkable; but it is *plumbing*, not a reviewable deliverable, so no producer↔validator pairing is implied by its content.

gate1 + gate2 trip → **full lens.** This is the cluster's pivotal "how does the loop carry state" question.

## Step 3 — The lens (weighted for artifact branch + orchestration)

**Check 1 — Orchestration test.** Orchestrated by the **setup command (markdown supervisor)** — content-coupling to kernel = **NONE**, orchestration-coupling = **TOTAL**. When the supervisor dissolves onto a mochiko-native home (command lead / Workflow), the template's responsibilities must land somewhere explicit. They are **moved-to-lead** candidates, exactly the "input/prerequisite wiring → explicit handoff edges in the contract" rehome that `reconcile-cluster`'s rehome-orchestration owns. **This template cannot be assessed standalone** — its fate is a function of the rehomed supervisor's state-carrying design.

**Check 2 — Role (two altitudes).** Skill-role analog = **carries instructions + context** (a transport, not a producer or validator). Team-role conferred = it makes the supervisor the **lead** (owner of iteration instructions + loop state) and the agent a **reader**. It confers no producer/validator role. Its entire value is loop-state transport.

**Check 3 — Independence.** No produce+grade leak — it grades nothing. Neutral. (Note: it does *carry* the supervisor's loop-driving instructions, which is a lead concern, not an independence concern.)

**Check 4 — Verdict-sink / loop-driver.** This template **is the substrate the loop-driver writes to.** The supervisor's `## Supervisor Instructions` + `## Clarification Log` + `iteration` frontmatter are precisely *how* the FAIL/clarification loop is driven across rounds. These are the biggest things the supervisor owned — and the ones most easily dropped silently. Naming them here is how they get re-homed onto the mochiko lead's **bounded loop** (round cap + no-progress exit) in reconcile.

**Check 5 — Sibling / overlap.**
- **Drift vs `commands/setup.md` (P1):** the command inlines a *different, richer* context block and does not use this file. This is a **dedupe/supersede signal** — the live responsibility already lives inline in P1. Decide with P1's reassessment whether mochiko keeps a separate template or folds the handoff into the command.
- **Family:** HIL has many `*-context-template.md` (plan-context, tasks-context, techspec-context, context-template) — a whole genre of supervisor state files. mochiko's stance on supervisor state-carrying is a cluster/architecture decision that this template is the setup-cluster instance of.

**Check 6 — Coupling audit.**
- **Path references (rebind, if kept):** the `## Supervisor Instructions` examples write to `.humaninloop/memory/constitution.md` (L59, L61) and reference `.humaninloop/memory/constitution.md` in `## Existing Constitution` → `.mochiko/...`.
- **Prerequisite/handoff:** this *is* the handoff edge between supervisor and agent (Phase 1↔3 of setup.md). In mochiko it becomes an explicit contract handoff edge (reconcile owns it).
- **Determinism boundary:** none — free-form.
- **Frontmatter:** `type / mode / iteration / created` — iteration is loop-bound metadata (→ lead's round cap).

**Check 7 — Conventions + loop placement.**
- **Classification:** inert artifact; convention-wiring floor = path rebind + (if it survives) a router/template entry. No trigger phrasing.
- **Loop placement (the crux):** this template is the *carrier* of the supervisor's loop state — done-condition signaling (clarifications resolved), iteration bound (`iteration` ≤ 3 / max-rounds), and the human-gate inputs (user answers appended to the log). In mochiko these are **loop-discipline obligations of the lead**, not of an ephemeral template. Whether they need a written file at all depends on whether mochiko's setup loop carries state in-session, in the `workflow-contract`, or in a rebound context file — **a rehome-orchestration decision.**

## Step 4 — Disposition

**Body × Structural = `port-with-edits` (tentative, "if kept") × `flag-for-reconcile`.**

- **Structural = `flag-for-reconcile`** — the move is **entirely sibling/supervisor-dependent** and must not be guessed solo. Candidate moves, all needing cluster context: **(a) `absorb-into-lead`** — fold the handoff into the mochiko setup command lead / `workflow-contract`, leaving no orphan template (most likely, given P1 already inlines its own context and mochiko is kernel-free/Workflow-first); **(b) `drop`** — if mochiko carries setup state in-session and needs no written handoff file (reason: ephemeral plumbing the framework now provides); **(c) `standalone` (port-with-edits)** — keep a rebound `.mochiko/` context template if the rehomed supervisor still wants an explicit on-disk handoff.
- **Body = `port-with-edits` (tentative)** — *only if it survives as a file*: rebind the `.humaninloop/memory/constitution.md` example paths, align the `type:`/`mode:` frontmatter to the mochiko setup loop, and reconcile its shape with setup.md's currently-inlined block. If reconcile chooses `absorb-into-lead` or `drop`, body becomes **`drop`** with the reason recorded below. The body proposal is conservative; the real choice is reconcile's.

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Transport project context (name / stack / CLAUDE.md-exists) to the agent | **flag-for-reconcile** → likely **moved-to-lead** (handoff edge in the contract) |
| R2 | Carry per-iteration `## Supervisor Instructions` (what to produce, where to write, format) | **moved-to-lead** — supervisor loop-driving / dispatch instructions live on the mochiko command lead |
| R3 | Maintain `## Clarification Log` conversation history across rounds | **flag-for-reconcile** → **moved-to-lead** (how mochiko carries multi-round state: in-session vs contract vs rebound file) |
| R4 | Iteration tracking (`iteration` frontmatter) | **moved-to-lead** — bounded-loop round cap (round cap + no-progress exit) is a loop-discipline obligation of the lead |
| R5 | Ephemeral lifecycle (supervisor creates / modifies / deletes; "delete after completion") | **moved-to-lead** (supervisor owns artifact lifecycle) **or dropped + reason** (in-session state needs no file) — reconcile decides |
| R6 | Specify output location for the produced constitution (`.humaninloop/memory/constitution.md`) | **kept-but-rebind** → `.mochiko/...` (this binding survives wherever the handoff lands) |
| R7 | `mode: [create\|amend]` routing signal | **moved-to-lead** (the create/amend branch is a supervisor decision; folds into the lead's dispatch) |
| R8 | "Flexible structure — supervisor may add custom sections" affordance | **dropped + reason**: this is a property of an editable supervisor state file; if the handoff is `absorb-into-lead`/in-session, the affordance is provided by the lead's own context, not a template (lead to accept) |
| R9 | Discoverability / placement (router/template entry if it survives) | **kept-but-rebind** *iff* structural = standalone; else **dropped + reason** (no standalone artifact to register) |

No responsibility left untagged. Drops carry reasons for lead acceptance; all non-dropped supervisor responsibilities route to the lead via reconcile.

## Reconcile flags

- **F-P8-1 (BLOCKING, structural) — state-carrying fate of the dissolving supervisor's handoff.** `constitution-context-template.md` is the setup supervisor's ephemeral context file. Decide in **rehome-orchestration** how mochiko's setup loop carries state: **absorb-into-lead** (fold into the command lead / `workflow-contract`, no template) · **drop** (in-session state; reason: framework-provided, ephemeral plumbing) · or **standalone** (keep a rebound `.mochiko/` context template). Most responsibilities (R1–R5, R7) are **moved-to-lead** candidates. Partner primitives: **setup command (P1)** — which already inlines a richer context block (drift/supersede), and the cluster's `workflow-contract`.
- **F-P8-2 (note) — drift vs P1's inlined context.** The live handoff responsibility already exists inside `commands/setup.md` (a different, richer shape). Reconcile should resolve P7/P8 against P1 so the cluster ends with **one** state-carrying mechanism, not a template duplicating what the lead inlines.

---

### Summary block

```
ASSESSMENT: constitution-context-template.md (P8)
Class:        template → branch artifact (BUT functionally = dissolving-supervisor state scaffolding)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits (tentative, "if kept") × flag-for-reconcile  [candidates: absorb-into-lead | drop | standalone]
Trace:
  - R1 transport project context to agent              → flag-for-reconcile → moved-to-lead (handoff edge)
  - R2 per-iteration supervisor instructions           → moved-to-lead (loop-driving/dispatch)
  - R3 clarification-log conversation history          → flag-for-reconcile → moved-to-lead (multi-round state)
  - R4 iteration tracking (frontmatter)                → moved-to-lead (bounded-loop round cap)
  - R5 ephemeral create/modify/delete lifecycle        → moved-to-lead OR dropped+reason (in-session state)
  - R6 output location binding for constitution.md     → kept-but-rebind (.mochiko/)
  - R7 mode create|amend routing signal                → moved-to-lead (dispatch branch)
  - R8 "supervisor may add custom sections" affordance → dropped + reason (provided by lead, not a template)
  - R9 discoverability/placement                       → kept-but-rebind iff standalone; else dropped+reason
Reconcile flags:
  - F-P8-1 (BLOCKING) state-carrying fate: absorb-into-lead | drop | standalone — partner P1 + workflow-contract; R1–R5/R7 = moved-to-lead
  - F-P8-2 (note) drift/supersede vs setup.md's inlined context — resolve P7/P8 against P1 to one mechanism
```
