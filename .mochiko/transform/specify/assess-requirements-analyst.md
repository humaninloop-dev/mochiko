# Assessment — `agents/requirements-analyst.md`

Run: `specify` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source: `human-in-loop/plugins/humaninloop/agents/requirements-analyst.md`
Class: **agent → PLAYS-a-role branch**
Cluster role: **PRODUCER** of the adversarial spec loop (analyst authors `spec.md`; `devils-advocate` grades; the `specify` command supervisor is lead/referee).

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting per branch: persona-vs-procedure split, the **team-role it
confers** on its caller (here: producer), its **`skills:`-list independence** (does it both produce
AND grade?), and **persona decoupling** (no sibling-agent names / "dispatch" / workflow
modes-paths-phases / "workflow-agnostic" meta-labels). Loop-ownership is NOT this primitive's
concern — that sits on the orchestrator (HIL: the DAG/`state-analyst`/Supervisor; mochiko: the
`specify` command supervisor, P1).

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** In HIL, `requirements-analyst` is a **dispatched leaf node**
   of the `specify` DAG: the Supervisor delegates DAG mechanics to `state-analyst`, which drives
   nodes from `specify-catalog.json` via the `hil-dag` MCP server, assembles the analyst's prompt,
   injects inputs/paths, and loops the analyst↔advocate convergence. The agent only *functions*
   because that machinery dispatches and feeds it. **Important:** this is **orchestration-coupling,
   not content-coupling** — the agent *body* contains no DAG/catalog/MCP/kernel reference (verified
   by grep, see Check 1). The coupling re-homes onto the lead (P1); it leaves **no** loop-driving on
   this agent.
2. **Multi-responsibility / fans out?** **YES.** Two distinct authoring responsibilities across two
   mounted skills (`authoring-requirements` → FR-XXX/SC-XXX/edge cases; `authoring-user-stories` →
   P1/P2/P3, Given/When/Then) plus the analyst persona/judgment. (They converge into one `spec.md`
   artifact, so fan-out to consumers is narrow — but responsibility-count > 1.)
3. **Non-machine-checkable artifact?** **YES.** The spec (requirements + user stories) is
   model-judgment output — no version-equality/schema assert can grade it. A **real grounded
   validator partner is required** — and it already exists as the **`devils-advocate`** (adversarial
   critic), so the pairing is satisfied by existing structure, not newly created.

At least one (all three) **yes → full 7-check lens.** Gates tripped: 1, 2, 3.

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer (HIL):** a **DAG kernel** — `state-analyst` drives nodes from
  `specify-catalog.json` through the `hil-dag` MCP server; the Supervisor sits on top. This agent is
  a dispatched node (analyst author / analyst-review). This is the kernel/DAG the cluster thesis
  sheds.
- **Content-coupling: NONE.** Grep of the body for `dag|catalog|hil-dag|mcp|brain|\.humaninloop|
  node|phase|specify(workflow)` returns only ordinary-English false positives ("**specify** the
  requirements", "verified as **pass**/fail", "happy **path**"). The persona references no kernel,
  DAG, catalog JSON, or MCP tool. **Body is mochiko-clean.**
- **Orchestration-coupling:** the agent is a **dispatched leaf** — it relies on the orchestrator for
  being invoked, for input/path injection, and for the FAIL/convergence loop on the advocate's
  verdict. In mochiko those become the **`specify` command supervisor's (P1)** responsibilities
  (sequencing, the bounded produce→critique loop, the human gate). **This agent owns no
  loop-driving to re-home** — no `moved-to-lead` tag originates here.

### 2. Role at two altitudes
- **Skill-role:** **artifact-emitter** — it wraps two emits-artifact authoring skills and produces a
  reviewable `spec.md`. → needs an independent producer↔validator partner.
- **Team-role conferred:** **PRODUCER — single and unconflicted.** Both mounted skills
  (`authoring-requirements`, `authoring-user-stories`) confer *producer*. There is **no** grading /
  validation / referee skill on this agent. A mochiko agent should confer exactly one team-role;
  this one does (producer). Contrast the setup-cluster `principal-architect`, which conferred
  producer + validator + referee at once.
- **Partner already exists:** the validator team-role lives on the **sibling `devils-advocate`**
  agent — the pairing is structural, not declared in this persona.

### 3. Independence — CENTRAL FINDING (confirmed CLEAN)
- **Self-grade leak: NO.** The `skills:` list is `authoring-requirements, authoring-user-stories` —
  **both producing skills, neither a grader.** The agent writes the spec; it never grades it.
- **Where the grading lives:** entirely on the peer **`devils-advocate`** (`skills:
  analysis-specifications` for the specify slice; `validation-plan-artifacts` /
  `validation-task-artifacts` deferred-stub). Producer skills ∩ validator skills = ∅.
- **Independence on two axes holds by construction:** producer agent (`requirements-analyst`) ≠
  validator agent (`devils-advocate`) **AND** their skill sets are disjoint. This is the textbook
  *correct* structure — the exact opposite of the `principal-architect` self-grade leak. **No split
  needed; no `flag-for-reconcile` for independence.**
- **Confirmation requested by the run:** both declared skills are in-scope and being ported (context
  P7/P8) → they **stay mounted** on the ported producer. Correct as-is.

### 4. Verdict-sink / loop-driver
- **Consumer of output:** the `spec.md` is consumed by the **`devils-advocate`** (grades it) and by
  the **lead** (decides converge-vs-iterate); downstream it feeds `plan`.
- **What loops on FAIL today:** the HIL DAG / `state-analyst` / Supervisor drives the
  analyst↔advocate convergence loop. **None of that loop-driving lives in this agent.**
- **Where it re-homes:** onto the **mochiko `specify` command supervisor (P1)** as a bounded
  produce→critique loop (round cap + no-progress + human gate). Originates from P1/`state-analyst`
  (P4), **not** from this primitive.

### 5. Sibling / overlap ("look sideways")
- **Agent siblings:** `devils-advocate` is a **complementary role (validator), not a variant** — no
  merge. `state-analyst` (P4) is pure orchestration being dissolved — unrelated to this body.
- **No trigger collision:** this agent is dispatched by role, not auto-fired; its description does
  not collide with siblings.
- **Skill overlap (intra-primitive):** the persona's **Quality Standards** restate format content
  the two mounted skills already own (see Check 7) → a **light `dedupe`** signal, canonical home =
  the skills. This is the only sideways signal and it is low-stakes.
- **No new sibling agent required** (unlike setup, the validator partner already exists).

### 6. Coupling audit
- **Hardcoded paths in body:** **NONE.** The agent does not even carry a "write outputs to the
  location specified" handoff (`principal-architect` did) — it is pure persona. The
  `.humaninloop/`→`.mochiko/` rebind therefore lives in P1's context handoff, not in this persona.
- **`skills:` references:** 2, both **in-scope & ported** → `kept-but-rebind` (body "Skills
  Available" descriptions get the `mochiko:` namespace prefix, matching the ported
  `principal-architect` body convention; frontmatter `skills:` stays bare).
- **Upstream prerequisites:** the analyst assumes the lead handed it the feature request / context
  (and, on a review pass, the advocate's prior gaps) — a **handoff edge owned by P1's contract**,
  not the agent.
- **Determinism boundary:** all of this agent's work is **model judgment** (requirements elicitation,
  story authoring, assumption-flagging). No deterministic script. ⇒ the validator partner
  (`devils-advocate`) is a **real grounded validator**, not a degenerate assert.

### 7. Conventions + loop placement — DECOUPLING SCAN (run headline)
- **Persona-vs-procedure split:** **clean.** "Core Identity", "What You Produce/Reject/Embrace",
  "Your Judgment" are pure persona (what the analyst *cares about*). "Your Process" (extract core
  need → identify actors → map happy path → consider edges → define success) is a generic analytical
  method that **passes the keystone test** (true of a requirements analyst on *any* job) → keep.
  The **"Quality Standards"** block, however, restates the skills' concrete format templates
  (`"As a [role], I want [capability], so that [benefit]"`; "must/shall/will" + quantify;
  testable acceptance) → **`dedupe`** (keep the taste, defer the literal templates to the skills as
  single source of truth).
- **Decoupling scan — CLEAN (decoupled by absence).** Deny-list grep of the body:
  - sibling-agent names (`devil|advocate|state-analyst|supervisor|ui-designer|principal|…`) → **none**
  - `dispatch` → **none**
  - injected workflow modes/paths/phases (`phase|node|catalog|dag|mcp|brain|\.humaninloop|workflow|greenfield/brownfield`) → **none** (only false positives "specify the requirements", "pass/fail")
  - `workflow-agnostic` / independence-by-declaration meta-labels → **none**
  - hardcoded write-location/paths → **none** (only "happy **path**", a user-flow term)
  The persona states its independence **by role, never by naming a sibling or a workflow** — the
  doctrine's ideal. **Strong empirical support for decoupling-by-absence; cleaner than
  `principal-architect`** (which carried a baked-in feasibility procedure + an essential-floor table +
  a write-location handoff). **Zero deny-list hits to fix.**
- **Classification:** model-invoked by nature (an agent dispatched by the lead); will be set in the
  wiring pass. (Agents are not user-invoked router entries themselves.)
- **Discoverability / description:** the HIL description is a single prose line with **no `<example>`
  blocks**; mochiko convention (cf. ported `principal-architect`, `transform-producer`) gives agent
  descriptions `<example>` blocks → **wiring-pass add** (convention conformance, not a body redesign).
- **Producer↔validator pairing:** **PRESENT and sound** — guaranteed structurally by the separate
  `devils-advocate`, not declared in this persona. (The gap that dominated `setup` is *absent* here.)
- **Sound-loop:** the loop this agent sits in needs a done-condition / independent validation / human
  gate — those land on **P1's supervisor loop** (rehome-orchestration), not on this agent. The
  independent-validation gate already has a home (the advocate); the bounded-loop + human gate are
  P1's to install.

---

## Step 4 — Disposition

- **Body treatment: `port-with-edits`.** The persona is mochiko-clean (no kernel/DAG/catalog, zero
  deny-list tokens), so this is the *light* end of `port-with-edits`, not `redesign`. The earned
  edits are localized and structure/voice-preserving: (a) **rebind** the two body skill references to
  the `mochiko:` namespace (wiring); (b) **dedupe** the "Quality Standards" literal format templates
  → defer to the two skills as single source of truth (keep the analyst's taste). It is *not*
  `keep-verbatim`: the skill-ref rebind is mandatory and the format-template duplication is a real
  (if low) drift hazard, matching the `principal-architect` essential-floor `dedupe` precedent.
- **Structural move: `standalone`.** Decidable **solo** — it does **not** depend on a sibling:
  - no self-grade leak → **no `split`**;
  - validator partner already exists (`devils-advocate`) → **no `pair`/new-agent**;
  - complementary sibling, not a variant → **no `merge`**;
  - not a check → **no `promote`**;
  - real reusable producer body → **not `absorb-into-lead`**.
  The DAG→agent-team rewiring of the *cluster* is real, but it is **P1's** structural move (the
  command supervisor), not this agent's. This agent simply lands `standalone` in `agents/`.
- **Disposition: `port-with-edits × standalone`** (matches the run hypothesis). **No
  `flag-for-reconcile` for the structural move.**

---

## Step 5 — Responsibility trace (complete; no silent drops)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Analyst persona & judgment — transform ambiguity→clarity; state assumptions explicitly; flag critical gaps; never guess on security/data/user-facing; "What You Produce/Reject/Embrace", "Your Judgment" | `kept` (keystone-clean, decoupled by absence) |
| 2 | "Your Process" 5-step analytical method (core need → actors → happy path → edges → success) | `kept` (passes keystone test — generic to the profession; not workflow-injected) |
| 3 | Functional-requirements authoring (FR-XXX / SC-XXX / edge cases) via `authoring-requirements` | `kept-but-rebind` (`humaninloop:`→`mochiko:authoring-requirements`; **stays mounted** — in-scope, ported P7) |
| 4 | User-story authoring (P1/P2/P3, Given/When/Then) via `authoring-user-stories` | `kept-but-rebind` (`humaninloop:`→`mochiko:authoring-user-stories`; **stays mounted** — in-scope, ported P8) |
| 5 | PRODUCER team-role in the spec loop (authors `spec.md` when dispatched) | `kept` (producer role preserved; paired with the existing `devils-advocate` validator — independence holds, no leak) |
| 6 | "Quality Standards" literal format templates ("As a [role]…", must/shall + quantify, testable AC) that duplicate the two mounted skills | `dedupe` (canonical home = the two skills; persona keeps taste, defers templates — relational: assess flags, reconcile/transform assigns) |
| 7 | Classification + router/discoverability (`<example>` blocks per mochiko agent convention) | `kept-but-rebind` (set in the convention-wiring pass; was implicit in HIL) |

**Orchestration note (no tag originates here):** being dispatched, fed inputs, and looped on the
advocate's verdict are **not this agent's responsibilities** — they belong to the HIL
DAG/`state-analyst`/Supervisor and re-home onto the **mochiko `specify` command supervisor (P1)**.
This agent contributes **no `moved-to-lead`** responsibility (it never owned the loop). The
independent-validation gate this loop needs is already homed on the `devils-advocate`; the bounded
loop + human gate are P1's rehome-orchestration items.

**No responsibility is dropped.** Every responsibility carries a tag → assessment done-condition met.

---

## Reconcile flags (for `reconcile-cluster`)

1. **Independence — CONFIRM, NO ACTION.** Producer↔validator pairing is **already satisfied**:
   `requirements-analyst` (producer; `authoring-requirements` + `authoring-user-stories`) ↔
   `devils-advocate` (validator; `analysis-specifications`), different agents, disjoint skills, **no
   self-grade leak**. Reconcile should confirm the pair holds and that the advocate's specify-slice
   skill ports while plan/tasks validation skills stay deferred-stub. **No `split`, no new validator
   agent** required for this primitive (contrast `setup`/`principal-architect`).
2. **Light `dedupe` (responsibility #6).** "Quality Standards" format templates duplicate
   `authoring-requirements` / `authoring-user-stories`. Confirm canonical home = the two mounted
   skills; persona references/defers rather than duplicates. Low-stakes; can be executed in the
   `transform-recipes` decouple/dedupe step. Persona *taste* (quantify, reject-vague, testable) stays.
3. **Rehome-orchestration (originates elsewhere, noted for completeness).** The analyst↔advocate
   convergence loop, input/path handoff, and human gate re-home onto **P1** (the `specify` command
   supervisor) — not onto this agent. Listed so reconcile's rehome map accounts for the loop this
   producer feeds; **this primitive emits nothing to re-home.**

**Decoupling-scan result for the run goal:** **0 deny-list hits.** The HIL `requirements-analyst`
persona is already decoupled by absence (no sibling names, no "dispatch," no workflow
modes/paths/phases, no "workflow-agnostic" label, no write-location). Empirical support for the
decoupling doctrine; `verify-output`'s decoupling scan should likewise come back clean on the port.

---

## Output block

```
ASSESSMENT: requirements-analyst
Class:        agent → branch PLAYS-a-role  (team-role conferred: PRODUCER)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (no flag-for-reconcile on the structural move)
Trace:
  - analyst persona & judgment                          → kept
  - "Your Process" 5-step analytical method             → kept (keystone-clean)
  - functional-requirements authoring (authoring-requirements) → kept-but-rebind (mochiko:authoring-requirements; stays mounted)
  - user-story authoring (authoring-user-stories)       → kept-but-rebind (mochiko:authoring-user-stories; stays mounted)
  - PRODUCER team-role (authors spec.md)                → kept (paired w/ existing devils-advocate validator; no self-grade leak)
  - "Quality Standards" literal format templates        → dedupe (canonical home = the two skills)
  - classification + router/discoverability (<example>) → kept-but-rebind (wiring pass)
Reconcile flags:
  - independence: CONFIRM-NO-ACTION (pair already exists; producer skills ∩ validator skills = ∅)
  - dedupe: Quality Standards templates → the two mounted skills (light)
  - rehome-orchestration: convergence loop + handoff + human gate land on P1 (not this agent)
Decoupling scan: 0 deny-list hits (decoupled by absence; cleaner than principal-architect)
```
