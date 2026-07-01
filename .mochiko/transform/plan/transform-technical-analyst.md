# Transform — `agents/technical-analyst.md`

Run: `plan` cluster transform · Phase 3 (transform) · Producer: `transform-producer`
Skill: `mochiko:transform-recipes`
Source: `human-in-loop/plugins/humaninloop/agents/technical-analyst.md`
Target: `plugins/mochiko/agents/technical-analyst.md`  ← **written**
Disposition (finalized, human-gated): **`port-with-edits × standalone`** — the plan workflow's PRODUCER.
Inputs consumed: `assess-technical-analyst.md`, `reconcile.md` (§Independence attestation, §RQ6, P2 row),
HIL source, sibling target shape `agents/requirements-analyst.md`.

> ROLE: apply the decision + run the convention-wiring pass. **Not** grading — `verify-output` is a
> separate phase run by a different agent (the independent `validator`). This file is the realized
> trace that `verify-output` audits.

---

## Applied

**`port-with-edits × standalone` + convention-wiring pass.**

- **Body = `port-with-edits`:** the persona is mochiko-sound craft (no kernel/DAG/catalog, no sibling
  names, no hardcoded paths). Three localized caller-injection edits + a namespace rebind + a light
  dedupe; section structure, headings, and voice preserved. Not `keep-verbatim` (3 decouple edits +
  rebind are mandatory); not `redesign` (no mechanism to shed).
- **Structural = `standalone`:** placed as-is at `agents/technical-analyst.md`. No self-grade leak →
  no split; complementary reviewers (not variants) → no merge; not a check → no promote; real reusable
  producer body → not absorb-into-lead. The DAG→agent-team rewiring of the *cluster* is P1's move, not
  this agent's. The producer↔validator pairing (RQ1) is settled in `reconcile.md` (graded by
  `principal-architect`+`validation-feasibility` and `devils-advocate`+`validation-plan-artifacts`);
  it changes this agent's **partners**, never its placement — so nothing structural lands on the agent
  body here.

## The 3 caller-injection decouple hits — FIXED (the run headline)

| # | HIL source (coupled) | Mochiko port (decoupled) | Tag |
|---|----------------------|--------------------------|-----|
| L76 | "Your work produces two categories of artifacts. **Your instructions will specify which artifacts to produce and where to write them.**" | "Your work spans two categories of artifacts. **You produce the ones the task in front of you calls for**—a full analysis-to-design pass, or a focused subset—**and when the brief is thin you proceed on the most reasonable reading rather than stalling.**" | `moved-to-lead` |
| L88 | "**Write outputs to the locations specified in your instructions.**" | *(sentence cut entirely — the persona names no write-location)* | `moved-to-lead` |
| L136 | "**When your instructions indicate brownfield context**, you value:" | "**When you are working against an existing codebase**, you value:" | `port-with-edits` (kept values) |

All three are the same class (caller-injection of inputs/paths/modes). The fix is self-contained
professional idiom + graceful degradation on a thin brief; the *which-artifacts / where-to-write*
context re-homes **caller-side** (P1 dispatch / `agent-dispatch.md`), never into the persona. The
brownfield **values** are unchanged — only the injected-mode *trigger* was coupling; the keystone-clean
craft (existing-patterns-over-invention, extension classification, convention consistency, collision-risk
transparency, priority alignment) survives intact.

## Light dedupe — literal procedure deferred to the skills (single source)

- **Quality Standards** gained the target-shape framing ("this is the *taste* you bring, not the format
  spec … single source of truth") + a closing deferral paragraph naming the literal procedures that live
  in P5–P8: the **IP-XXX** infrastructure-provisioning format → `authoring-technical-requirements`; the
  **at-least-two-alternatives ADR evaluation** with criteria weighting → `patterns-technical-decisions`;
  the **entity attribute/relationship/state-machine schema** → `patterns-entity-modeling`; the OpenAPI
  request/response structure → `patterns-api-contracts`.
- The **Infrastructure-aware** standard softened "a corresponding IP-XXX item" → "a corresponding
  infrastructure-provisioning requirement" (taste kept; the IP-XXX *format* now deferred, not restated).
- The **Normalized** standard softened the literal entity sub-field enumeration to taste ("clear
  boundaries with well-defined relationships, explicit validation rules, and complete lifecycles").
- **What You Embrace** "Thorough exploration of alternatives" reworded from the literal "evaluates at
  least 2 alternatives" rule to the value ("you never ship a single-option 'decision'; weighed against
  real alternatives with explicit trade-offs") — the count/format defers to P6.
- **Dropped one intra-persona duplicate:** the What-You-Embrace "Traceability from requirements to
  design" bullet was a verbatim restatement of the Quality-Standards "Traceable" standard → removed
  (the taste survives once, in Quality Standards).

**Scope discipline (what dedupe did NOT touch):** the **What You Produce** 6-artifact menu is
output-shape the disposition says to keep — its content descriptions (including the "(IP-XXX)" content
label in item 2, the entity sub-field list in item 4, OpenAPI in item 5) are kept as the *deliverable
menu*. The dedupe targets only the **Quality-Standards / What-You-Embrace** *standards* (per assess
trace #10), not the menu. Core Identity's war-story "at least two alternatives" is kept verbatim
(taste/motivation, assess trace #1 `kept`); the *procedure* — not the value — is what defers to P6.

## Convention-wiring pass (all six)

1. **Classification** — model-invoked producer agent (a dispatched author, not a user-invoked router
   entry). Frontmatter `skills:` list kept **bare** (`authoring-technical-requirements,
   patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts`), per the
   ported-agent convention (matches `requirements-analyst`). Persona-vs-procedure split honored: persona
   holds taste/judgment/output-menu; procedure lives in the four mounted skills.
2. **Router registration** — **NOT applied here** (final wiring pass owns the shared router). See the
   **plugin.json / router NOTE** below.
3. **Trigger phrasing** — the 4 `<example>` blocks in `description` kept verbatim; they are craft-clean
   (work-context: "the spec", "Stripe", "SendGrid"; the `assistant` lines name only the agent itself,
   **zero** sibling agents). Added one independence-by-role clause to the description summary ("Authors
   the technical artifacts; does not grade its own output." — mirrors the target shape).
4. **Path rebinding** — body "Skills Available" refs (×4) `humaninloop:` → `mochiko:`; the deferral
   paragraph names the same four `mochiko:`-namespaced skills. No `.humaninloop/` literal paths existed
   (the write-location was phrasing, removed in L88). Frontmatter `skills:` stays bare.
5. **Decouple persona** — the 3 caller-injection hits fixed (above); independence stated by **role**
   (producer authoring artifacts), not by agent name. Confirmed ZERO: sibling-agent names · "dispatch" ·
   injected workflow modes/paths/phases · "workflow-agnostic"/independence-by-declaration meta-labels ·
   kernel/DAG/catalog · hardcoded paths. (Two grep matches are the assess-cleared false positives:
   "systematic cataloguing" inside an `<example>` commentary = ordinary English; "Technology-agnostic in
   analysis" = a craft standard, not the forbidden self-label.)
6. **Single-source / de-duplicate** — the persona **references** its four skills as the single source of
   truth for the literal formats/procedures (the Quality-Standards deferral paragraph) rather than
   restating them. (No `loop-discipline`/`workflow-contract` reference is owed — that floor is for
   `command`/`workflow` primitives, not an agent.)

## Realized responsibility trace (every responsibility → realized tag; no silent loss)

| # | Responsibility | Assess tag | Realized tag (this transform) |
|---|----------------|-----------|-------------------------------|
| 1 | Systems-engineer persona & judgment (Core Identity 8 war-stories; What You Reject; Quality-Standards + Embrace taste) | `kept` | **`kept`** — Core Identity + What You Reject ported **verbatim**; Quality Standards / Embrace kept as taste (light dedupe only) |
| 2 | TR/constraints/NFR/IP-XXX/data-sensitivity authoring (P5) | `kept-but-rebind` | **`kept-but-rebind`** — body ref → `mochiko:authoring-technical-requirements`; **stays mounted** (frontmatter) |
| 3 | Technology-decision / ADR authoring (P6) | `kept-but-rebind` | **`kept-but-rebind`** — `mochiko:patterns-technical-decisions`; **stays mounted** |
| 4 | Entity-modeling / data-model authoring (P7) | `kept-but-rebind` | **`kept-but-rebind`** — `mochiko:patterns-entity-modeling`; **stays mounted** |
| 5 | API-contract / OpenAPI authoring (P8) | `kept-but-rebind` | **`kept-but-rebind`** — `mochiko:patterns-api-contracts`; **stays mounted** |
| 6 | Integration Guide authoring (no 1:1 skill) | `kept` | **`kept`** — menu item 6 retained; graceful-degradation framing added to the menu intro |
| 7 | PRODUCER team-role (authors 6 artifacts) | `kept` | **`kept`** — producer-only; independence-by-role clause added to description; **no grading skill mounted** (validators = PA + advocate, per `reconcile.md`) |
| 8 | Caller-injection of artifact-selection + write-location (L76, L88) | `moved-to-lead` | **`moved-to-lead`** — L76 reframed, L88 cut; which-artifacts/where context owned caller-side (P1 dispatch / `agent-dispatch.md`) |
| 9 | Brownfield-awareness disposition (L134–142) | `kept` (port-with-edits) | **`kept`** — L136 trigger reframed to self-contained; "Roadmap alignment" → "Priority alignment"; 5 values unchanged |
| 10 | Quality-Standards / Embrace literal procedure (≥2 alternatives, IP-XXX format, entity sub-field list) | `dedupe` | **`dedupe`** — deferred to P5–P8 via the single-source paragraph; "≥2" reworded to value; duplicate traceability bullet dropped; **taste kept** |
| 11 | "Constitution alignment" (L132) | `kept` | **`kept`** — reworded to "Alignment with project governance … established principles" (light generic label) |
| 12 | Classification + router/discoverability (`<example>` already present) | `kept-but-rebind` | **`kept-but-rebind`** — 4 examples kept verbatim; classification = model-invoked; **router registration deferred to final wiring pass** (NOTE below) |

**Orchestration note (unchanged from assess):** no loop-driving tag originates from this agent. Being a
dispatched node, fed inputs/paths, and looped on the reviewer verdict are HIL-DAG/`state-analyst`/
Supervisor responsibilities that re-home onto the **`plan` command supervisor (P1)** — listed in
`reconcile.md` Job 2A, not here. The one caller-side context this agent sheds (#8) re-homes to P1's
dispatch brief.

**No responsibility dropped.** Every responsibility carries a realized tag → transform done-condition met.

## plugin.json / router NOTE (for the final wiring pass — NOT edited here)

Per instruction, the shared router (`plugins/mochiko/skills/mochiko/SKILL.md`) and the manifest
(`plugins/mochiko/.claude-plugin/plugin.json`) are **owned by a final wiring pass** and were **not
edited**. Two registrations remain outstanding for `technical-analyst`:

1. **`plugin.json` agents array** — currently lists `principal-architect, transform-producer, validator,
   requirements-analyst, devils-advocate` (5). **Add** `"./agents/technical-analyst.md"`. Without it the
   agent is not loaded by the plugin.
2. **Router `SKILL.md`** — has no **Plan cluster** section yet (only Doctrine / Transformer / Setup /
   Specify). The final pass should add a Plan-cluster block and an **Agents** table row:
   `technical-analyst | plan-cluster PRODUCER — authors the 6 analysis+design artifacts (skills:
   authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling,
   patterns-api-contracts)`. A primitive absent from the router fails discoverability (a `verify-output`
   Part-A item) — flagged so the final pass closes it.

## Decouple-residual scan (mechanical confirmation that the edits landed — not a grade)

- 3 caller-injection hits → **0 residual** (`your instructions` / `instructions will specify` /
  `locations specified in your` / `when your instructions indicate` all absent).
- sibling-agent names → **0** · `dispatch` → **0** · `workflow-agnostic` → **0** · kernel/DAG/phase →
  **0** · hardcoded paths / `humaninloop:` namespace → **0**.
- False positives (assess-cleared, no action): "systematic **catalogu**ing" (example commentary,
  ordinary English) · "**Technology-agnostic** in analysis" (craft standard).
- 4 `mochiko:` skill refs in Skills Available + 4 in the deferral paragraph; frontmatter `skills:` bare;
  independence-by-role clause present (YAML line-wrapped).

## Independence attestation (re-confirmed; non-negotiable)

Producer `technical-analyst` mounts `{authoring-technical-requirements, patterns-technical-decisions,
patterns-entity-modeling, patterns-api-contracts}` — **all authoring/patterns (producer) skills; not one
grader.** Producer skills ∩ grading skills = **∅**. No `validation-plan-artifacts`, no
`validation-feasibility`, no `verify-*`. The agent authors; it never grades. Its artifacts are graded by
**different** agents (per `reconcile.md` §Independence attestation: `principal-architect` +
`validation-feasibility` for feasibility, `devils-advocate` + `validation-plan-artifacts` for
completeness; the `plan` lead referees). Port hazard recorded: **never co-mount a review/validation skill
on this agent.**

## Handoff

Artifact + this realized trace → `verify-output`, run by a **different** agent (the independent
`validator`). This producer does not grade its own output.

---

## Output block (transform-recipes format)

```
TRANSFORM: technical-analyst
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/agents/technical-analyst.md (created)
           .mochiko/transform/plan/transform-technical-analyst.md (this trace)
New partners: none from this primitive (producer↔validator pairing settled in reconcile.md:
           validators = principal-architect+validation-feasibility, devils-advocate+validation-plan-artifacts;
           created by their own transforms, not here)
Wiring:    classification=model-invoked producer (skills: list bare; persona/procedure split honored)
           router=NOT-registered (deferred to final wiring pass — NOTE: add to plugin.json agents array + router Plan-cluster)
           triggers=4 <example> blocks kept verbatim (craft-clean; zero sibling names) + independence-by-role clause added
           rebinds=4 body skill refs humaninloop:→mochiko: (kept-but-rebind); frontmatter skills: bare; no .humaninloop/ paths
           decouple=3 caller-injection hits FIXED (L76 reframed / L88 cut / L136 reframed); 0 residual deny-list tokens
           single-source=literal IP-XXX/ADR-≥2/entity-schema/OpenAPI procedures deferred to P5–P8 (not restated)
Trace (realized):
  - systems-engineer persona & judgment            → kept (Core Identity + Reject verbatim; standards = taste)
  - TR/constraints/NFR/IP-XXX (P5)                  → kept-but-rebind (mochiko:; stays mounted)
  - technology-decisions/ADR (P6)                   → kept-but-rebind (mochiko:; stays mounted)
  - entity-modeling/data-model (P7)                 → kept-but-rebind (mochiko:; stays mounted)
  - API-contracts/OpenAPI (P8)                      → kept-but-rebind (mochiko:; stays mounted)
  - Integration Guide authoring (no 1:1 skill)      → kept (menu item 6; graceful-degradation framing)
  - PRODUCER team-role (authors 6 artifacts)        → kept (no grading skill mounted; independence ∩=∅)
  - caller-injection artifact-selection/write-loc   → moved-to-lead (L76 reframed, L88 cut → agent-dispatch.md)
  - brownfield-awareness disposition                → kept (L136 reframed; Roadmap→Priority; values unchanged)
  - Quality-Standards/Embrace literal procedure     → dedupe (deferred to P5–P8; duplicate traceability bullet dropped)
  - "Constitution alignment"                        → kept (light generic reword → project governance)
  - classification + router/discoverability         → kept-but-rebind (examples kept; router NOTED for final pass)
Independence: producer skills ∩ grading skills = ∅ (no self-grade; graders are different agents)
3 hits: FIXED (0 residual) | plugin.json+router: NOTED (final wiring pass) | grade: deferred to verify-output (different agent)
```
