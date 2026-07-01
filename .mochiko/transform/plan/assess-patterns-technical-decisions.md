# ASSESSMENT: patterns-technical-decisions (P6)

> Role: **assess/diagnose only.** No transform, no grade. Relational moves emitted as `flag-for-reconcile`.
> Run context: porting the HIL `plan` cluster into mochiko, core-only. P6 is one of the **technical-analyst producer (P2)** skills.
> Source read: `human-in-loop/plugins/humaninloop/skills/patterns-technical-decisions/` — `SKILL.md`, `references/DECISION-RECORD.md`, `references/EVALUATION-MATRIX.md` (no scripts present).

```
ASSESSMENT: patterns-technical-decisions
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone  (+ relational signals → flag-for-reconcile)
```

---

## Step 1 — Class / branch

**skill → PLAYS-a-role.** What carries weight on this branch: consumed-procedure vs emits-artifact, trigger reliability, **sibling overlap** (the task's headline check vs P5), and decoupling (no agent names; independence by *role*).

A skill is *a role in* a loop, not the loop. So loop-ownership questions (done-condition, FAIL-loop, human gate) are scored as "where do they re-home," not "does this skill own them."

---

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled (depends on kernel / markdown supervisor / command / DAG to function)?** **No.** The core procedure (evaluate alternatives → decide → document an ADR) is a self-contained professional procedure; it functions without any orchestrator. The HIL plan DAG only *sequences* when it runs. **Caveat:** there is *content*-coupling to the plan workflow (the `constraints-and-decisions.md` artifact, "spec or plan", "NEEDS CLARIFICATION") — caught by checks 1/6/7, not an orchestration dependency.
2. **Multi-responsibility / fans out?** **Yes.** Holds the evaluation-matrix technique, the ADR/decision-record format, consequence + dependency tracking, brownfield-alignment scoring, the ≥2-alternatives discipline, and the `constraints-and-decisions.md` output shaping. Feeds the technical-analyst producer (and is genuinely general to anyone documenting a tech decision).
3. **Emits an artifact whose correctness is NOT machine-checkable?** **Yes.** It shapes decision records / the decisions section of `constraints-and-decisions.md`. "Are two alternatives *genuinely* weighed? Is the rationale honest? Are trade-offs real?" is model judgment, not a schema/version assert.

Two gates "yes" → **full 7-check lens** (gate1 n, but content-coupling noted).

---

## Step 3 — The 7-check lens (weighted for PLAYS-a-role)

### 1. Orchestration test — *content vs orchestration coupling*
- **Orchestrating layer:** the HIL plan DAG + the technical-analyst agent that mounts this skill. The DAG dissolves in mochiko; the technical-analyst producer consumes the skill directly (a `skills:` mounting, decided at agent level — not in this body).
- **Content-coupling (in the body, → `port-with-edits`):** "When `constraints-and-decisions.md` requires NEEDS CLARIFICATION resolution" (line 17); "When spec or plan requires technology choice justification" (line 19); the full `constraints-and-decisions.md` output template (lines 92–119); the implied constitution path. These are plan-workflow artifact/phase references — soften/rebind, don't drop.
- **Orchestration-coupling:** **none in the body.** The skill drives no loop and is driven by no loop internally. The only re-home is "consumed by the technical-analyst producer," which is a `skills:`-list fact, not a body responsibility.
- **Kernel-free scan (task-requested):** **clean.** No brain / MCP / DAG / catalog references anywhere in `SKILL.md`, `DECISION-RECORD.md`, or `EVALUATION-MATRIX.md`. The only couplings are workflow *paths/brand* (`constraints-and-decisions.md`, `/humaninloop:plan`, constitution path) → `kept-but-rebind`.

### 2. Role at two altitudes
- **Skill-role:** a **consumed-procedure that shapes a reviewable artifact** (decision records / the D-XXX decisions in `constraints-and-decisions.md`). It is an artifact-shaper, so its product wants an independent validator.
- **Team-role conferred on its caller:** **producer.** It is authoring guidance, not a grading rubric. Its "Quality Checklist" is a producer *self*-check (author-side discipline) — it does **not** confer a validator team-role.
- **Consequence:** the artifact needs independent validation, but that validator is the **plan cluster's** validator (`validation-plan-artifacts`), not a per-skill split of P6. → relational → **flag-for-reconcile** (the pairing is wired at plan-loop build).

### 3. Independence
- **No self-grade leak in the skill itself.** It does not grade.
- **Cluster-scoped risk:** the leak would appear only if the **technical-analyst** producer were mounted with both this authoring skill *and* the decisions-grading skill (`validation-plan-artifacts`). That must not happen — grading mounts on an independent validator agent. → note for reconcile / plan-cluster `skills:`-list casting. **Independence non-negotiable.**

### 4. Verdict-sink / loop-driver
- **Consumer of output:** the plan validation gate (`validation-plan-artifacts`) + ultimately the human acceptance gate.
- **What loops on FAIL:** the plan **command supervisor** owns the produce→validate→revise loop — *not* this skill.
- **`NEEDS CLARIFICATION`:** P6 only *emits the marker*; resolving it is an **escalation / human-gate** concern that re-homes to the plan lead. → loop-driving + escalation are plan-cluster rehome-orchestration, surfaced as reconcile signals (not this skill's body).

### 5. Sibling / overlap — "look sideways" (the headline check)
**P5 `authoring-technical-requirements` ↔ P6 `patterns-technical-decisions` share `constraints-and-decisions.md`.**
- **P5 is the artifact-owner.** `constraints-and-decisions.md` is one of P5's three analysis artifacts; P5 defines **Section 2: Technology Decisions (D-XXX)** fields (Context / Options / Choice / Consequences / Rationale → references C-XXX) plus the C↔D traceability and IP-XXX.
- **P6 is the technique/depth skill.** It owns *how* to make and document the decision well: the evaluation matrix (criteria weighting + scoring), the full ADR record (Status / Context / Decision / Rationale / Alternatives / Consequences), consequence buckets, decision-dependency/impact chains, brownfield-alignment scoring, and the ≥2-alternatives discipline. **AND** P6 also restates the `constraints-and-decisions.md` Summary/records/Dependencies structure (lines 92–119; `DECISION-RECORD.md`).
- **Relationship = shared-core / handoff, NOT a merge.** P6 is **not** a thin variant of P5 — it has a large unique slice (evaluation matrix, ADR depth, brownfield scoring, anti-patterns). Neither is it independent — both describe the same artifact and the same D-XXX decisions.
- **Signal → `flag-for-reconcile`:** set the boundary (P5 = the artifact + C↔D/IP traceability; P6 = the evaluation/ADR technique) and **`dedupe`** the `constraints-and-decisions.md` structure so P6 *references* P5's artifact instead of restating it. **Do not merge; do not decide solo.**
- **Other plan siblings:** the technology-category example tables in `EVALUATION-MATRIX.md` (auth / data storage / API style) touch the domains of `patterns-api-contracts` / `patterns-entity-modeling`, but only as *illustrative decision categories* — not ownership overlap. No merge signal; light note that they aren't the authoritative source for those design domains.

### 6. Coupling audit
- **Hardcoded paths / brand:**
  - `constraints-and-decisions.md` (plan artifact, owned by P5) → `kept-but-rebind` (mochiko plan workspace, and reference P5).
  - constitution alignment → rebind `.humaninloop/memory/constitution.md` → `.mochiko/memory/constitution.md`.
  - `/humaninloop:plan`, "spec or plan" → soften to role-language / rebind to mochiko plan.
- **Upstream prerequisites / handoffs:** assumes a plan/spec context exists and that P5's `constraints-and-decisions.md` is the home for the decisions. Handoff edge: **P5 → P6** (P5 owns the artifact P6 contributes decision records to).
- **Determinism boundary:** **almost entirely model judgment** (weighing criteria, writing rationale, assessing trade-offs). Structural facts (≥2 options present, a trade-offs section exists) are checkable, but *quality* is judgment → a validator partner is a **real grounded grader, not a degenerate assert** (confirms gate3).

### 7. Conventions + loop placement
- **Classification:** none in HIL. This is **agent-consumed + model-invoked** → assign **model-invoked** (no `disable-model-invocation`). *Missing → assign.*
- **Discoverability / router:** **not in the mochiko router** (no plan-cluster section exists yet). *Missing → register at transform under a new Plan cluster block.*
- **Trigger phrasing:** HIL uses the literal **"when the user says '…'"** style. Per mochiko convention (and the ported `authoring-requirements` model), rewrite to **work-context graded RFC-2119** describing the *work* (evaluating alternatives, capturing trade-offs/consequences, ≥2 alternatives, NEEDS CLARIFICATION), not "when the user says." Keep strong nouns (alternatives, trade-offs, decision record, ADR, rationale, NEEDS CLARIFICATION) as SHOULD-phrases. → `port-with-edits`.
- **Sibling trigger relationship (P5):** not a hard *literal-phrase* collision (P5 triggers on constraints/NFR/IP/integration/data; P6 on alternatives/trade-offs/decision-record). The real risk is **artifact-ownership routing ambiguity** over the D-XXX / `constraints-and-decisions.md` surface — once both are model-invoked, "document the technology decision" reads as both. → reconcile sets the boundary; wiring then phrases each description to its own side.
- **Decoupling scan:** **no agent names, no "dispatch," no "workflow-agnostic" meta-label** (clean). The only coupling is **injected workflow paths/phases** ("spec or plan", `constraints-and-decisions.md`, NEEDS-CLARIFICATION-of-plan) → keystone-test and soften (true-of-anyone-making-a-documented-decision stays; only-in-plan framing is cut/softened).
- **Producer↔validator pairing:** absent in-skill → the plan-cluster validator owns it → **flag-for-reconcile** (see check 2).
- **Sound-loop placement:** producer-side procedure only; done-condition / independent validation / human gate live in the (not-yet-built) plan command supervisor; `NEEDS CLARIFICATION` is the escalation hook → re-homes to the plan lead. → reconcile rehome-orchestration at plan-loop build.

---

## Step 4 — Disposition

**`port-with-edits` × `standalone`** (+ relational signals → `flag-for-reconcile`).

- **Body = `port-with-edits`** (not `keep-verbatim`, not `redesign`): the body is mochiko-clean in substance (no kernel infra; the evaluation matrix, ADR format, brownfield scoring, and anti-patterns are sound and worth keeping verbatim), but **localized edits are required**: (a) rewrite the `description` to work-context graded triggers; (b) soften/rebind the plan-workflow path/phase references; (c) resolve the `constraints-and-decisions.md` overlap with P5 by *referencing* rather than restating (pending reconcile).
- **Structural = `standalone`**: the skill survives as itself, mounted on the technical-analyst producer in the plan cluster. It is **not** split/merged/promoted. The relational items below are *wiring/pairing* decisions for reconcile, not a structural dissolution of P6.

---

## Step 5 — Responsibility trace (complete; no silent loss)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Evaluation-of-alternatives technique (criteria set, weighting, scoring matrix; `EVALUATION-MATRIX.md`) | `kept` |
| 2 | ≥2–3-alternatives discipline / anti-single-option (Single-Option, Shiny-Object mistakes) | `kept` |
| 3 | ADR / decision-record format (Status/Context/Decision/Rationale/Alternatives/Consequences) | `kept` |
| 4 | Consequence documentation (positive / negative / neutral) | `kept` |
| 5 | Decision-dependency & impact-chain tracking (D2 depends on D1) | `kept` |
| 6 | Trade-offs-explicit + non-vague-rationale discipline | `kept` |
| 7 | Brownfield-alignment scoring (existing dep / same eco / different eco / conflicting) | `kept` |
| 8 | RFC 2119 keywords for constraints (`DECISION-RECORD.md` table) | `kept` (small local table; mild `dedupe` candidate vs the shared RFC-2119 doctrine — keep local) |
| 9 | `NEEDS CLARIFICATION` *marking* capability | `kept` |
| 9b | `NEEDS CLARIFICATION` *escalation/resolution loop* (driving + human gate) | `moved-to-lead` → plan command supervisor (at plan-loop build) |
| 10 | `constraints-and-decisions.md` output structure (Summary / records / Dependencies; lines 92–119) | **`flag-for-reconcile`** → likely `dedupe` / reference P5 (P5 owns the artifact) |
| 11 | Constitution-alignment check (Constitution-Blindness mistake; `DECISION-RECORD.md` section) | `kept-but-rebind` (`.humaninloop/…/constitution.md` → `.mochiko/memory/constitution.md`; cross-cluster reference) |
| 12 | When-to-use / when-NOT boundary (trivial/already-documented/hotfix/reversible) | `kept`; the "spec or plan" trigger line → `kept-but-rebind` (decouple plan framing to role-language) |
| 13 | Trigger `description` intent (literal "when the user says" phrases) | `kept-but-rebind` (rewrite to work-context graded RFC-2119; de-collide with P5) |
| 14 | Quality checklist (producer self-check) | `kept` (author-side self-check; **not** a validator rubric) |
| 15 | Technology-category example tables (auth / storage / API style) | `kept` (illustrative; not authoritative ownership of those design domains) |
| 16 | Classification convention | `kept-but-rebind` (assign **model-invoked**) |
| 17 | Router registration | `kept-but-rebind` (register in router — Plan cluster section; currently MISSING) |
| 18 | Independent validation of the decisions artifact (absent in-skill) | **`flag-for-reconcile`** → `moved-to-validator` (plan-cluster `validation-plan-artifacts`, on an independent validator agent — never on technical-analyst) |

**Drops:** **none.** P6 carries no kernel/DAG/catalog/MCP content to shed; every responsibility is kept, rebound, re-homed, or flagged.

---

## Reconcile flags (relational — for `reconcile-cluster` at plan-cluster build)

1. **P5↔P6 shared `constraints-and-decisions.md` ownership (D-XXX decisions).** Overlap/handoff, **not** a merge (both hold large unique slices). Resolve as a **boundary + `dedupe`**: P5 owns the artifact template + C↔D/IP traceability; P6 owns the evaluation/ADR *technique* and **references** P5's artifact rather than restating it (trace #10). Handoff edge **P5 → P6**.
2. **P5↔P6 routing ambiguity over the D-XXX surface.** Not a literal-phrase collision, but an ownership ambiguity once both are model-invoked. After reconcile sets the boundary, **de-collide in the wiring pass** (phrase each `description` to its own side: P5 = constraints/NFR/IP authoring + traceability; P6 = evaluate/ADR/alternatives/trade-offs/NEEDS CLARIFICATION technique).
3. **Producer↔validator pairing for the decisions artifact.** P6 emits a reviewable, non-machine-checkable artifact with no in-skill validator. Pair with the plan-cluster validator (`validation-plan-artifacts`, not yet ported) on an **independent validator agent**; **never** mount it on the technical-analyst producer (independence is non-negotiable) (trace #18).
4. **`NEEDS CLARIFICATION` escalation rehome.** Marker stays in P6; the escalation/resolution loop-driving + human gate re-home to the plan command supervisor when the plan loop is built (trace #9b).

> Several of these resolve only when their partner primitives port (P5, `validation-plan-artifacts`, the plan command). Given core-only incremental plan porting, reconcile should record them now and finalize at plan-loop build.
