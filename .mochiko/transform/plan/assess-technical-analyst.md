# Assessment — `agents/technical-analyst.md`

Run: `plan` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source: `human-in-loop/plugins/humaninloop/agents/technical-analyst.md`
Class: **agent → PLAYS-a-role branch**
Cluster role: **PRODUCER** of the plan analysis→design loop (the technical-analyst authors all six
artifacts — technical requirements, constraints+decisions, NFRs, data-model, API contracts,
integration guide; the reviewer architecture that grades them is **RQ1, undecided**; the `plan`
command supervisor (P1) is lead/referee).

> ROLE NOTE: this is assess/diagnose ONLY. No transform, no grading. Relational moves (the
> producer↔validator pairing architecture) are emitted as `flag-for-reconcile`, not resolved here.

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting per branch: persona-vs-procedure split, the **team-role it
confers** (here: PRODUCER), its **`skills:`-list independence** (does it both author AND grade?), and
**persona decoupling** (no sibling-agent names / "dispatch" / injected workflow modes-paths-phases /
"workflow-agnostic" meta-labels). Loop-ownership is NOT this primitive's concern — it sits on the
orchestrator (HIL: the `plan` DAG / `state-analyst` / Supervisor; mochiko: the `plan` command
supervisor, P1).

This is the **highest-risk decoupling surface of the run** — the richest persona being ported (4
mounted skills, 6 artifacts across two categories, the deepest war-story Core Identity yet). The
decoupling scan (Check 7) is the run headline.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** In HIL `plan`, the technical-analyst is a **dispatched DAG
   node** — `state-analyst` drives nodes from the plan catalog via the `hil-dag` MCP server,
   assembles the analyst's prompt, injects which-artifacts/where-to-write, and loops the
   analyst↔reviewer convergence. The agent only *functions* because that machinery dispatches and
   feeds it. **This is orchestration-coupling, AND there is a residue of content-coupling** — unlike
   `requirements-analyst`, this body bakes caller-injection of inputs/paths into the persona ("Your
   instructions will specify which artifacts to produce and where to write them"; see Checks 1 & 7).
   The DAG/dispatch re-homes onto P1; the baked caller-injection is a `port-with-edits` decouple.
2. **Multi-responsibility / fans out?** **YES, strongly** — the most of any plan-cluster producer.
   Four distinct authoring responsibilities across four mounted skills (TR/constraints/NFR/IP-XXX +
   ADR decisions + entity modeling + API contracts) **plus** a sixth artifact (integration guide)
   with no 1:1 skill, **plus** the systems-engineer persona/judgment. Fans out across **two artifact
   categories** (analysis + design) feeding multiple downstream consumers (the reviewer[s], then
   `implement`).
3. **Non-machine-checkable artifact?** **YES.** TR-to-FR traceability, NFR measurability, failure-mode
   coverage, entity normalization, and schema-to-entity alignment are **model judgment**. Even the
   structured parts (OpenAPI schema) are graded on *correctness-against-intent*, not a version/schema
   assert. ⇒ a **real grounded validator partner is required** — and *which* validator(s) is the open
   reviewer-architecture question (RQ1).

At least one (all three) **yes → full 7-check lens.** Gates tripped: 1, 2, 3.

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer (HIL):** a **DAG kernel** — `state-analyst` drives plan nodes through the
  `hil-dag` MCP server; the Supervisor sits on top. This agent is a dispatched producer node. This is
  the kernel/DAG the cluster thesis sheds.
- **Content-coupling: MINIMAL but PRESENT (unlike requirements-analyst).** Grep of the body for
  `phase|node|catalog|dag|mcp|brain|hil-dag` returns only **false positives** — "systematic
  **catalogu**ing" (description example, line 32) is ordinary English for documenting dependencies,
  not catalog JSON. No DAG/MCP/kernel reference. **BUT** the persona does carry **caller-injection
  content-coupling**: lines 76, 88, 136 bake "your instructions will specify / locations specified in
  your instructions / when your instructions indicate" into the body (Check 7 / hits below). That is
  a content-coupling residue requiring a `port-with-edits` decouple — not a kernel reference, but a
  baked dependency on a dispatcher.
- **Orchestration-coupling:** the agent is a **dispatched producer node** — it relies on the
  orchestrator for invocation, for which-artifacts/path injection, and for the FAIL/convergence loop
  on the reviewer verdict(s). In mochiko those become the **`plan` command supervisor's (P1)**
  responsibilities. **This agent owns no loop-driving to re-home** — no `moved-to-lead` for
  sequencing/looping originates from the agent body. (The one thing that DOES re-home to the caller
  is the artifact-selection / write-location context the persona currently injects — see trace #8.)

### 2. Role at two altitudes
- **Skill-role:** **artifact-emitter** — it wraps four emits-artifact authoring/patterns skills and
  produces six reviewable artifacts. → needs an independent producer↔validator partner.
- **Team-role conferred:** **PRODUCER — single and unconflicted.** All four mounted skills
  (`authoring-technical-requirements`, `patterns-technical-decisions`, `patterns-entity-modeling`,
  `patterns-api-contracts`) confer *producer*. There is **no** grading / validation / referee skill on
  this agent. A mochiko agent should confer exactly one team-role; this one does (producer).
- **Partner is UNDECIDED (RQ1).** In HIL plan the validator team-role is split across **two** sibling
  reviewers (principal-architect = feasibility/contradiction; devils-advocate = completeness via
  `validation-plan-artifacts`). The mochiko reviewer architecture is the central reconcile question —
  the pairing is required, the partner identity is not settled here. → `flag-for-reconcile`.

### 3. Independence — CENTRAL FINDING (confirmed CLEAN, as the run asked)
- **Self-grade leak: NO.** The `skills:` list (line 46) is `authoring-technical-requirements,
  patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts` — **all four are
  authoring/patterns (producer) skills; not one is a grader.** No `validation-plan-artifacts`, no
  `cross-artifact-checklist`-as-grader, no `verify-*`. The agent authors the artifacts; it never
  grades them.
- **Producer skills ∩ grading skills = ∅.** Independence is **structural and intact on the producer
  side**: this agent cannot self-grade because it holds no grading skill. This is the textbook-correct
  producer shape — the opposite of the setup-cluster `principal-architect` self-grade leak.
- **The independence *question* that remains is the partner**, not this agent: who reviews the six
  artifacts (RQ1). That is a pairing/architecture decision for `reconcile-cluster`, **not** an
  independence violation on the technical-analyst. **No split needed for THIS primitive; no
  `flag-for-reconcile` for an independence *leak*** — only for the pairing *architecture*.
- **Run confirmation:** all four declared skills are in-scope and ported this run (context P5–P8) →
  they **stay mounted** on the ported producer. Correct as-is.

### 4. Verdict-sink / loop-driver
- **Consumers of output:** the six artifacts are consumed by the **reviewer(s)** (grade them — RQ1
  shape), by the **lead** (decides converge-vs-iterate), and downstream by `implement`.
- **What loops on FAIL today:** the HIL plan DAG / `state-analyst` / Supervisor drives the
  analyst↔reviewer convergence — including the **architect-feasibility-once-then-advocate-completeness
  ordering** and the **skip-architect-re-review-unless-structural-change** routing (context/contract
  §1). **None of that loop-driving lives in this agent.**
- **Where it re-homes:** onto the **mochiko `plan` command supervisor (P1)** as a bounded
  produce→review loop (round cap + no-progress + human gate). Originates from P1 / the dissolving DAG,
  **not** from this primitive — listed for the rehome map's completeness only.

### 5. Sibling / overlap ("look sideways")
- **Agent siblings:** the reviewers (`principal-architect`, `devils-advocate`) are **complementary
  roles (validators), not variants** — no merge with this producer. `state-analyst` is pure
  orchestration being dissolved — unrelated to this body.
- **Analysis vs design = NOT a split signal.** The technical-analyst deliberately holds **both**
  analysis (TR/constraints/NFR) and design (data-model/contracts/integration) artifacts. This
  unification is **upstream-decided** (HIL ADR-008 merged `techspec` into `plan`; "same agent
  maintains full context", source example line 42; context.md confirms techspec-merge RESOLVED). Do
  **not** invent an analysis/design split — the two-category producer is the intended merged form.
- **No agent split across the four skills.** TR-authoring, ADRs, entity-modeling, API-contracts are
  one systems engineer's toolkit (procedure correctly factored into P5–P8). One producer, four skills.
- **Intra-primitive overlap:** "Quality Standards" + "What You Embrace" restate procedure the four
  skills own (the "≥2 alternatives" rule, the IP-XXX format, the entity sub-field list) → a **light
  `dedupe`** signal, canonical home = the skills (Check 7). Low-stakes.

### 6. Coupling audit
- **Hardcoded paths in body:** **NONE** literal (grep of `.humaninloop|.mochiko|specs/|.workflow|
  /memory/` → none). The write-location coupling is **phrasing, not a literal path** — "the locations
  specified in your instructions" (line 88). The `.humaninloop/`→`.mochiko/` rebind therefore lives in
  P1's context handoff, not as a string in this persona.
- **`skills:` references:** 4, all **in-scope & ported** (P5–P8) → `kept-but-rebind` (body "Skills
  Available" lines 55–58 get the `mochiko:` namespace prefix; frontmatter `skills:` line 46 stays bare
  per the ported-agent convention).
- **Upstream prerequisites:** the analyst assumes the lead handed it the business `spec.md` (and, on a
  design pass, the locked analysis artifacts; on a review pass, the reviewer's prior gaps) — **handoff
  edges owned by P1's contract**, not the agent.
- **Determinism boundary:** essentially **all model judgment** (translation, measurability,
  failure-mode analysis, entity extraction, contract design). The structured OpenAPI output is
  format, not a correctness oracle. ⇒ the validator partner is a **real grounded validator**, not a
  degenerate assert — reinforcing that RQ1 must land a genuine reviewer, not a schema check.

### 7. Conventions + loop placement — DECOUPLING SCAN (run headline)

**Persona-vs-procedure split.** Persona (keep): Core Identity (war-story judgment), Quality Standards
(taste), What You Reject / What You Embrace, Brownfield Awareness (the *values*), and the **menu** of
six artifact *types* in "What You Produce" (output-shape is allowed). Procedure (lives in skills,
already factored): the concrete HOW of TR-authoring, ADRs, entity-modeling, API-contracts → P5–P8.
**Procedure currently in the persona that belongs in a skill** = the literal rules restated in Quality
Standards / What You Embrace (the "≥2 alternatives" rule, "IP-XXX" format, the entity field list) →
**light `dedupe`** (keep the taste, defer the literal procedure to the skills as single source).

**Decoupling scan — 3 DENY-LIST HITS, all one class (caller-injection); worst categories CLEAN.**
Evidence-backed (grep results):

- sibling-agent names (`principal-architect|devil|advocate|supervisor|state-analyst|ui-designer|…`)
  → **NONE.** The richest persona this run names **zero** siblings — the most dangerous category is
  clean.
- `dispatch` / loop-position (`dispatch|the lead|round|fix-list|cap`) → **NONE.**
- `workflow-agnostic` / meta-labels → **NONE.** (Grep hit "**Technology-agnostic** in analysis", line
  94, is a **FALSE POSITIVE** — a craft standard meaning "requirements describe what not how", not the
  forbidden "workflow-agnostic" self-label.)
- kernel/DAG/phase (`phase|node|catalog|dag|mcp|brain`) → **NONE.** (Grep hit "systematic
  **catalogu**ing", line 32, is a **FALSE POSITIVE** — ordinary English, not catalog JSON.)
- hardcoded paths → **NONE.**
- **caller-injection of inputs/paths/modes → 3 HITS (the only real coupling):**
  1. **Line 76** — `Your work produces two categories of artifacts. Your instructions will specify
     which artifacts to produce and where to write them.` → the "your instructions will provide"
     pattern (forbidden, O1). The persona must describe **no** inputs; which-artifacts + write-location
     are caller-side context (P1 dispatch / `agent-dispatch.md`).
  2. **Line 88** — `Write outputs to the locations specified in your instructions.` → same
     caller-injection (write-location). Cut from persona; lives caller-side.
  3. **Line 136** — `When your instructions indicate brownfield context, you value:` → same
     "your instructions indicate" injection, here gating the Brownfield Awareness section on a
     caller-supplied mode.

**Brownfield — the subtle profession-trait vs loop-position line (keystone-tested precisely).** Grep
flags "brownfield" at lines 115, 134, 136. Verdict: **the brownfield *disposition* is CRAFT — keep**;
only the *injected-mode framing* is coupling.
- Keystone test of the values (existing-patterns-over-invention, explicit extension classification,
  convention consistency, collision-risk transparency, priority alignment): **true of a systems
  engineer on ANY job touching existing code → craft.** "Brownfield" is industry-standard terminology
  for existing-codebase context, **not** a mochiko/HIL workflow mode — and it legitimately survived
  the port in the mochiko `principal-architect` (which authors a "brownfield codification"). So
  "brownfield-the-concept" stays (line 115 What-You-Reject item and the line-134 section content are
  `kept`).
- The **coupling** is the *trigger* "When your instructions indicate brownfield context" (line 136) —
  a caller-injected mode gate — which `port-with-edits` reframes to self-contained ("When working
  against an existing codebase, you value…"). The values are unchanged; the dispatcher dependency is
  removed. "Roadmap alignment" → light generic reword to "respect established priorities" (drops the
  workflow-ish "roadmap" noun; the disposition is craft).

**Constitution alignment (line 132)** — "Every artifact references how it aligns with project
principles" — **keep** (aligning work to project governance is craft true on any governed project; the
mochiko `principal-architect` authors the constitution, so the concept is legitimate). Optional very
light reword of the "Constitution" label to generic "project principles/governance"; minor, not a
deny-list hit.

- **Classification:** model-invoked by nature (a producer agent dispatched by the lead); set in the
  wiring pass. (Agents are not user-invoked router entries themselves.)
- **Discoverability / description:** the HIL description **already carries four `<example>` blocks**
  (richer than `requirements-analyst`, which had none) and is **craft-clean** (the examples reference
  "the spec", "Stripe", "SendGrid" — work-context, no coupling). → keep the examples; the description
  is already in mochiko agent shape. **Lighter wiring than requirements-analyst on this axis.**
- **Producer↔validator pairing:** **REQUIRED but UNHOMED here** — the artifacts are reviewable and
  non-machine-checkable, so an independent validator must exist; *which* (the RQ1 architecture) is a
  reconcile decision. The pairing is guaranteed structurally (separate reviewer agent[s]), never in
  this persona.
- **Sound-loop:** the done-condition / independent validation / human gate the loop needs land on
  **P1's supervisor loop** (rehome-orchestration), not on this agent.

---

## Step 4 — Disposition

- **Body treatment: `port-with-edits`** (matches the run hypothesis; **heavier than
  requirements-analyst's** because of 3 real decouple edits vs that agent's 0). The persona is clean
  on the worst categories (no kernel/DAG, no sibling names, no "dispatch", no "workflow-agnostic", no
  hardcoded path) but carries **3 caller-injection deny-list hits** plus a **light dedupe**. The earned,
  structure/voice-preserving edits:
  (a) **decouple** — cut the two caller-injection sentences (lines 76, 88); the persona keeps the
      six-artifact *menu* (output-shape) but stops injecting which-to-produce/where-to-write, which
      re-homes to the caller (P1 dispatch / `agent-dispatch.md`);
  (b) **decouple** — reframe the brownfield trigger (line 136) to self-contained; keep the values;
      light reword "roadmap" → "established priorities";
  (c) **rebind** — the four body skill refs (lines 55–58) to the `mochiko:` namespace;
  (d) **dedupe** — the Quality-Standards / What-You-Embrace literal procedure (≥2 alternatives,
      IP-XXX, entity field list) → defer to P5–P8 as single source; keep the taste.
  It is **not** `keep-verbatim` (the decouple edits + rebind are mandatory) and **not** `redesign`
  (the body assumes no kernel/DAG/catalog; the Core Identity, standards, and artifact menu are
  mochiko-sound craft to preserve).
- **Structural move: `standalone`** — for the agent's **own placement**. Decidable solo on placement:
  no self-grade leak → **no `split`**; complementary reviewers, not variants → **no `merge`**; not a
  check → **no `promote`**; real reusable producer body → **not `absorb-into-lead`**. The agent lands
  `standalone` in `agents/`. The DAG→agent-team rewiring of the *cluster* is **P1's** structural move,
  not this agent's.
- **Disposition: `port-with-edits × standalone`** (matches the run hypothesis) — **WITH a
  `flag-for-reconcile` on the producer↔validator pairing architecture (RQ1)**: the agent's placement
  is standalone, but *which validator(s) grade its six artifacts* is a sibling-dependent decision that
  must NOT be guessed here.

---

## Step 5 — Responsibility trace (complete; no silent drops)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Systems-engineer persona & judgment — measurability-obsession, failure-mode awareness, data-sensitivity classification, constraint-surfacing, FR-to-TR traceability, alternatives-evaluation, entity-extraction discipline, API-realism (Core Identity; What You Reject/Embrace; Quality Standards taste) | `kept` (keystone-clean intrinsic craft; decoupled by absence on the worst tokens) |
| 2 | TR + constraints + NFR + IP-XXX + data-sensitivity authoring via `authoring-technical-requirements` (P5) | `kept-but-rebind` (`humaninloop:`→`mochiko:`; **stays mounted** — in-scope, ported P5) |
| 3 | Technology-decision / ADR / alternatives authoring via `patterns-technical-decisions` (P6) | `kept-but-rebind` (`humaninloop:`→`mochiko:`; **stays mounted** — in-scope, ported P6) |
| 4 | Entity-modeling / data-model authoring via `patterns-entity-modeling` (P7) | `kept-but-rebind` (`humaninloop:`→`mochiko:`; **stays mounted** — in-scope, ported P7) |
| 5 | API-contract / OpenAPI / integration-boundary authoring via `patterns-api-contracts` (P8) | `kept-but-rebind` (`humaninloop:`→`mochiko:`; **stays mounted** — in-scope, ported P8) |
| 6 | Integration Guide authoring (artifact 6 — common flows, auth sequences, error patterns; **no 1:1 skill**, produced via persona competence + P8) | `kept` (graceful-degradation craft; competent without a dedicated skill) |
| 7 | PRODUCER team-role in the plan analysis→design loop (authors all six artifacts when dispatched) | `kept` (producer role preserved; **validator partner = RQ1**, reconcile; independence holds — no grading skill mounted) |
| 8 | Caller-injection of artifact-selection + write-location into the persona (lines 76, 88: "your instructions will specify which artifacts… where to write them"; "locations specified in your instructions") | `moved-to-lead` (decouple-by-absence: cut from persona; the which-artifacts + write-location context is owned by P1's dispatch / `agent-dispatch.md`, rebinding `.humaninloop/`→`.mochiko/` there) |
| 9 | Brownfield-awareness disposition (existing-patterns, extension classification, convention consistency, collision-risk transparency, priority alignment) | `kept` (keystone-clean craft) **via `port-with-edits`** — reframe the line-136 "when your instructions indicate brownfield context" trigger to self-contained; reword "roadmap"→"established priorities"; values unchanged |
| 10 | Quality-Standards / What-You-Embrace literal procedure restating the four skills (≥2 alternatives rule, IP-XXX format, entity sub-field list) | `dedupe` (canonical home = P5–P8; persona keeps taste, defers literal procedure — relational: assess flags, transform assigns) |
| 11 | "Constitution alignment — artifacts align to project principles" (line 132) | `kept` (craft: align to governance; optional light generic reword of the "Constitution" label — minor, not a deny-list hit) |
| 12 | Classification + router/discoverability (description **already** carries `<example>` blocks; model-invoked producer agent) | `kept-but-rebind` (convention-wiring pass; lighter than requirements-analyst — examples already present) |

**Orchestration note (no loop-driving tag originates here).** Being a dispatched DAG node, fed
inputs/paths, and looped on the reviewer verdict(s) — including the feasibility-once-then-completeness
ordering and the skip-re-review routing — are **HIL DAG / `state-analyst` / Supervisor**
responsibilities that re-home onto the **mochiko `plan` command supervisor (P1)**. This agent
contributes **no `moved-to-lead` for sequencing/loop-driving** (it never owned the loop). The one
caller-side context it *does* shed (trace #8) re-homes to P1's dispatch, not as loop-driving but as a
briefing input.

**No responsibility is dropped.** Every responsibility carries a tag → assessment done-condition met.

---

## Reconcile flags (for `reconcile-cluster`)

1. **Producer↔validator PAIRING ARCHITECTURE — RQ1 (HEADLINE FLAG; do NOT guess).** The
   technical-analyst is the **settled PRODUCER half** (authors six reviewable, non-machine-checkable
   artifacts; carries no grading skill → independence intact). The **validator half is undecided**:
   HIL plan splits review across **two** reviewers (principal-architect = feasibility/cross-artifact
   contradiction; devils-advocate = completeness via `validation-plan-artifacts`). Reconcile must pick
   the mochiko shape — **(i)** two distinct validators (a *checklist* advocate + an
   *adversarial-critique* architect — the convention-5 two-form case, plan being the first cluster to
   exercise it); **(ii)** fold feasibility into the advocate (the `specify` one-reviewer shape); or
   **(iii)** rehome feasibility onto the generic `validator`. Human-gated. The technical-analyst's own
   placement (`standalone`) does **not** change under any option; only its partner does.
2. **RQ2 — principal-architect as feasibility reviewer (partner-side; noted for the pair).** The
   already-ported (producer-only) `principal-architect` would, under RQ1 option (i)/(iii)-adjacent
   readings, **review** the technical-analyst's output. That is principal-architect's reconcile
   concern (produce-here/review-there for one decoupled persona; which skill drives the feasibility
   review). Flagged here only because **the technical-analyst's artifacts are the input that pairing
   grades** — the producer side is settled; the reviewer side rides on RQ2.
3. **Light `dedupe` (trace #10).** Quality-Standards / What-You-Embrace literal procedure duplicates
   P5–P8. Confirm canonical home = the four mounted skills; persona references/defers rather than
   restates. Low-stakes; executable in the `transform-recipes` decouple/dedupe step. Persona *taste*
   (measurability, failure-awareness, reject-vague) stays.
4. **Decouple-by-absence edits (the 3 caller-injection hits; NOT structural — noted for completeness).**
   Lines 76, 88, 136 are `port-with-edits` decouple actions in the wiring pass (cut/reframe
   caller-injection; push the which-artifacts/where context to `agent-dispatch.md`). `verify-output`'s
   decoupling scan + keystone test must confirm **zero residual** on the port.

**Independence — CONFIRM, NO ACTION (as the run asked).** Producer skills `{authoring-technical-
requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts}` ∩ any
grading/validation skill = **∅**. No self-grade leak. Independence is structural on the producer side;
the only open independence-adjacent item is the *partner identity* (RQ1), not a leak on this agent.
**No `split`, no new persona** required for this primitive — only an edit to the ported producer plus
the RQ1 pairing decision around it.

**Decoupling-scan result for the run goal:** **3 deny-list hits — all one class** (caller-injection,
lines 76/88/136: "your instructions will specify / locations specified / your instructions indicate").
**Worst categories CLEAN:** zero sibling-agent names, zero "dispatch", zero "workflow-agnostic", zero
kernel/DAG/phase, zero hardcoded paths (two grep hits — "Technology-agnostic" L94, "cataloguing" L32 —
are false positives). The richest persona of the run is **cleaner than feared on the dangerous tokens
but carries the caller-injection pattern `requirements-analyst` lacked** — a bounded `port-with-edits`
fixes it. Empirical read: the doctrine's worst failure mode (sibling-naming) did not occur; the
residual is the dispatcher-dependency O1 explicitly targets.

---

## Output block

```
ASSESSMENT: technical-analyst
Class:        agent → branch PLAYS-a-role  (team-role conferred: PRODUCER)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   + flag-for-reconcile: producer↔validator pairing architecture (RQ1)
Trace:
  - systems-engineer persona & judgment (Core Identity, standards, reject/embrace) → kept (keystone-clean craft)
  - TR/constraints/NFR/IP-XXX/data-sensitivity (authoring-technical-requirements P5) → kept-but-rebind (mochiko:; stays mounted)
  - technology-decisions/ADR (patterns-technical-decisions P6)                       → kept-but-rebind (mochiko:; stays mounted)
  - entity-modeling/data-model (patterns-entity-modeling P7)                          → kept-but-rebind (mochiko:; stays mounted)
  - API-contracts/OpenAPI/integration-boundaries (patterns-api-contracts P8)          → kept-but-rebind (mochiko:; stays mounted)
  - Integration Guide authoring (no 1:1 skill; persona + P8)                          → kept (graceful-degradation craft)
  - PRODUCER team-role (authors 6 artifacts)                                          → kept (validator partner = RQ1; no self-grade leak)
  - caller-injection of artifact-selection + write-location (L76, L88)                → moved-to-lead (decouple; P1 dispatch / agent-dispatch.md)
  - brownfield-awareness disposition (L134-142)                                       → kept (port-with-edits: reframe L136 trigger; values unchanged)
  - Quality-Standards/What-You-Embrace literal procedure (≥2 alt, IP-XXX, fields)     → dedupe (canonical home = P5-P8)
  - "Constitution alignment" (L132)                                                   → kept (craft; optional light generic reword)
  - classification + router/discoverability (<example> already present)               → kept-but-rebind (wiring pass; lighter — examples present)
Reconcile flags:
  - RQ1 producer↔validator pairing ARCHITECTURE: producer settled (technical-analyst authors only);
    validator side = two distinct (checklist advocate + adversarial-critique architect) vs fold-into-advocate
    vs generic validator — DO NOT GUESS; human-gated
  - RQ2 (partner-side): principal-architect produce-here/review-there + driving skill — rides on the pairing
  - dedupe: Quality-Standards / What-You-Embrace literal procedure → P5-P8 (light)
  - decouple edits (3 caller-injection hits L76/88/136): port-with-edits wiring action, not structural
Independence: CONFIRM-NO-ACTION (producer skills ∩ grading skills = ∅; no self-grade leak)
Decoupling scan: 3 deny-list hits, all caller-injection (L76/88/136); 0 sibling names / 0 dispatch /
                 0 workflow-agnostic / 0 kernel-DAG-path (L94 "Technology-agnostic" + L32 "cataloguing" = false positives)
```
