# TRANSFORM: patterns-technical-decisions (P6)

> Role: **apply + wire only.** Disposition was finalized by assess + reconcile; this file applies it and updates the trace. **Not graded here** — `verify-output` runs independently, on a different agent.
> Disposition (finalized): **`port-with-edits × standalone`** — the producer's decision-technique skill (ADR).
> Source: `human-in-loop/plugins/humaninloop/skills/patterns-technical-decisions/` (SKILL.md + references/DECISION-RECORD.md + references/EVALUATION-MATRIX.md; no scripts).

```
TRANSFORM: patterns-technical-decisions
Applied:   port-with-edits × standalone + wiring-pass (all 6)
Artifacts: plugins/mochiko/skills/patterns-technical-decisions/SKILL.md
           plugins/mochiko/skills/patterns-technical-decisions/references/DECISION-RECORD.md
           plugins/mochiko/skills/patterns-technical-decisions/references/EVALUATION-MATRIX.md
New partners: none (standalone; the producer↔validator pairing is a cross-cluster wiring at plan-loop build, not a P6-body product)
Wiring:    classification=model-invoked  router=NOT-edited (entry noted below — shared router)
           triggers=work-context graded RFC-2119, de-collided vs P5
           rebinds=constitution→.mochiko/memory/constitution.md; "spec or plan"/constraints-and-decisions.md framing→role language; escalation "iteration 2"→human gate
           single-source=references mochiko:authoring-technical-requirements (artifact) + mochiko:loop-discipline (loop); RFC-2119 kept local
```

---

## SHARED SEAM BOUNDARY (P5 ↔ P6) — as applied

Per `reconcile.md` §RQ6 (P5↔P6 = **boundary + dedupe, not merge**; handoff edge **P5 → P6**):

- **P6 (this skill) owns the TECHNIQUE:** the evaluation/decision matrix, full ADR record depth, criteria weighting, trade-offs + consequences + dependency/impact chains, the ≥2-alternatives discipline, brownfield-alignment scoring, and the `NEEDS CLARIFICATION` **marker**.
- **P5 owns the ARTIFACT:** the `constraints-and-decisions.md` template, the Section-2 `D-XXX` field schema, and `C-XXX`↔`D-XXX` / `IP-XXX` traceability.
- **Applied dedupe:** the HIL SKILL.md `## constraints-and-decisions.md Output` (lines ~92–119) **and** the HIL `DECISION-RECORD.md` `## constraints-and-decisions.md Structure` (lines ~40–86) both reproduced P5's artifact. **Both** were replaced with a reference to `mochiko:authoring-technical-requirements` (single-source requires no restated copy survives anywhere in P6).

### P5-reference phrasing used (so P5's port matches the seam)

1. SKILL.md · Overview: *"This skill owns the decision-making **technique** … The `constraints-and-decisions.md` **artifact** the decisions land in is owned by `mochiko:authoring-technical-requirements`; this skill references that artifact rather than restating it."*
2. SKILL.md · *Where decisions are recorded*: *"Its file structure, its Section-2 Technology-Decisions **`D-XXX`** field schema, and the constraint↔decision / infrastructure-planning traceability are owned by `mochiko:authoring-technical-requirements`. Do not restate the artifact template here."* — plus an explicit two-row **ownership boundary table** (artifact = P5 / technique = P6) and the line *"place it into the `D-XXX` slots that `mochiko:authoring-technical-requirements` defines."*
3. DECISION-RECORD.md · *Where these records land*: *"Its file structure — the Summary table, the per-decision `D-XXX` sections, the Dependencies table, and the Open Questions / escalation list — together with the `D-XXX` field schema and `C-XXX`↔`D-XXX` / `IP-XXX` traceability are owned by `mochiko:authoring-technical-requirements`. … place it into that artifact's `D-XXX` slots — do not restate the artifact shape here."*

> Reciprocal expectation for the P5 port: P5 **owns** the `constraints-and-decisions.md` template + `D-XXX` schema + traceability and **references `mochiko:patterns-technical-decisions`** for the decision technique / ADR depth that fills each `D-XXX` slot. Bare-name (`mochiko:<skill>`) cross-reference form used throughout, matching the ported `authoring-requirements` convention (no fragile relative-path link to the not-yet-ported P5).

---

## Wiring pass — all six conventions

| # | Convention | Applied |
|---|------------|---------|
| 1 | **Classification** | **model-invoked** (no `disable-model-invocation`). Agent-consumed producer skill, mounted on the `technical-analyst` producer (a `skills:`-list fact wired when P2 ports — not a P6-body edit). |
| 2 | **Router registration** | **NOT edited** (shared router / `plugin.json`, per instruction). Entry text provided below for the router owner. The `mochiko` router has **no Plan cluster block yet** — the entry lands when the plan cluster is registered. |
| 3 | **Trigger phrasing** | `description` rewritten from HIL's literal *"when the user says '…'"* to **work-context graded RFC-2119** (MUST = evaluate ≥2 alternatives + capture trade-offs/consequences + brownfield scoring + record an ADR; SHOULD = the strong-noun triggers). **De-collided vs P5**: the description cedes the artifact + `D-XXX` schema + traceability to P5 and claims only the technique side (evaluate / alternatives / trade-offs / consequences / ADR / rationale / brownfield / NEEDS CLARIFICATION). |
| 4 | **Path rebinding** | Constitution reference made concrete-mochiko: `.mochiko/memory/constitution.md` (DECISION-RECORD Constitution Alignment). No `.humaninloop/` literals existed in source. `constraints-and-decisions.md` / "spec or plan" framing softened to role language (When-to-Use). Escalation example "iteration 2" → "the human gate". |
| 5 | **Decouple persona/skill** | Source was already free of agent names / "dispatch" / "workflow-agnostic" (assess confirmed clean). The only coupling — **injected workflow paths/phases** — was keystone-tested and softened: "When spec or plan requires…" → "When a technology choice needs a documented justification"; the `constraints-and-decisions.md`-NEEDS-CLARIFICATION line → role language. Keystone-true lines (anyone making a documented decision) kept. |
| 6 | **Single-source / de-duplicate** | (a) P5 artifact: referenced, not restated — deduped in **both** SKILL.md and DECISION-RECORD.md. (b) Loop language: the `NEEDS CLARIFICATION` resolution loop references `mochiko:loop-discipline` and is stated to belong to the command supervisor, not this skill. (c) RFC-2119: **kept local** in DECISION-RECORD.md (matches the sibling `authoring-requirements`, which keeps its own `references/RFC-2119-KEYWORDS.md`; assess trace #8 "keep local"). |

### Router entry needed (for the shared-router owner — NOT applied here)

To be added under a new **Plan cluster** block of `plugins/mochiko/skills/mochiko/SKILL.md` when the plan cluster registers:

```
| `patterns-technical-decisions` | making/documenting a technology or architecture decision well — evaluating ≥2 alternatives against weighted criteria, capturing trade-offs + consequences, brownfield-alignment scoring, ADR record depth, marking NEEDS CLARIFICATION; owns the decision *technique* while `authoring-technical-requirements` owns the `constraints-and-decisions.md` artifact the decisions land in |
```

Agent-mount note (for P2 `technical-analyst` port): add `patterns-technical-decisions` to the producer's `skills:` list (with P5/P7/P8). **Independence guard:** never co-mount a grading skill (`validation-plan-artifacts`, `validation-feasibility`) on `technical-analyst`.

---

## Realized responsibility trace (every responsibility flipped to its final tag — no silent loss)

| # | Responsibility | assess tag | **realized tag** | Where it landed |
|---|----------------|-----------|------------------|-----------------|
| 1 | Evaluation-of-alternatives technique (criteria, weighting, scoring matrix) | kept | **kept** | EVALUATION-MATRIX.md (verbatim); SKILL Phase 1 |
| 2 | ≥2–3-alternatives discipline / anti-single-option | kept | **kept** | SKILL Phase 1 + "Single Option" mistake; encoded in `description` MUST clause |
| 3 | ADR / decision-record format (Status/Context/Decision/Rationale/Alternatives/Consequences) | kept | **kept** | SKILL Phase 3 + DECISION-RECORD "Decision Record Format" (verbatim) |
| 4 | Consequence documentation (positive / negative / neutral) | kept | **kept** | DECISION-RECORD "Consequence Documentation" (verbatim) |
| 5 | Decision-dependency & impact-chain tracking (D2 depends on D1) | kept | **kept** | DECISION-RECORD "Dependency Documentation" + "Impact Chain" (verbatim) — the *reasoning technique*, distinct from P5's artifact Dependencies table |
| 6 | Trade-offs-explicit + non-vague-rationale discipline | kept | **kept** | SKILL Phase 2 + "Vague Rationale"/"Missing Trade-offs" mistakes; DECISION-RECORD "Rationale Best Practices" |
| 7 | Brownfield-alignment scoring | kept | **kept** | SKILL "Brownfield Alignment" + EVALUATION-MATRIX "Brownfield Considerations / Alignment Scoring" |
| 8 | RFC 2119 keywords table | kept (keep local) | **kept (local)** | DECISION-RECORD "RFC 2119 Keywords" — NOT deduped; sibling keeps RFC-2119 local |
| 9 | `NEEDS CLARIFICATION` **marking** capability | kept | **kept** | SKILL Phase 2 marking note + When-to-Use + `description` |
| 9b | `NEEDS CLARIFICATION` **escalation/resolution loop** (loop-driving + human gate) | moved-to-lead | **moved-to-lead** | SKILL Phase 2: marking is the skill's job; *driving* the loop / human escalation belongs to the command supervisor (references `mochiko:loop-discipline`) |
| 10 | `constraints-and-decisions.md` output structure (Summary / records / Dependencies) | flag → dedupe | **dedupe** | Deduped in **both** SKILL "Where decisions are recorded" + DECISION-RECORD "Where these records land" → reference `mochiko:authoring-technical-requirements` (P5 owns artifact) |
| 11 | Constitution-alignment check | kept-but-rebind | **kept-but-rebind** | DECISION-RECORD "Constitution Alignment" → concrete `.mochiko/memory/constitution.md` |
| 12 | When-to-use / when-NOT boundary | kept; "spec or plan"→rebind | **kept + decoupled** | SKILL When-to-Use; plan-workflow framing softened to role language; When-NOT kept verbatim |
| 13 | Trigger `description` intent | kept-but-rebind | **kept-but-rebind** | `description` rewritten to work-context graded RFC-2119; de-collided vs P5 |
| 14 | Quality checklist (producer self-check) | kept | **kept** | SKILL "Quality Checklist" — stays author-side self-check, **not** a validator rubric (independence preserved) |
| 15 | Technology-category example tables (auth / storage / API) | kept | **kept (+ note)** | EVALUATION-MATRIX "Common Technology Categories" + light note: illustrative only; design-domain ownership routes to `patterns-api-contracts` / `patterns-entity-modeling` |
| 16 | Classification convention | kept-but-rebind | **kept-but-rebind** | model-invoked (no `disable-model-invocation`) |
| 17 | Router registration | kept-but-rebind | **noted (not applied)** | Shared router NOT edited per instruction; entry text supplied above for the router owner |
| 18 | Independent validation of the decisions artifact | flag → moved-to-validator | **moved-to-validator** | Cross-cluster pairing (reconcile RQ1/§250–258): graded by `devils-advocate`·`validation-plan-artifacts` (completeness) + `principal-architect`·`validation-feasibility` (feasibility), on independent validator agents — **never** on `technical-analyst`. Wired at plan-loop build; not a P6-body edit. |

**Drops: none.** P6 carried no kernel/DAG/catalog/MCP content. Every responsibility is `kept`, `kept-local`, `kept-but-rebind`, `dedupe`, `moved-to-lead`, `moved-to-validator`, or `noted`.

**Independence note (non-negotiable):** P6 confers a **producer** team-role only. Its Quality Checklist is a producer *self*-check, not a grading rubric. Nothing in this transform mounts a validator capability on P6 or on its producer agent. The decisions artifact's independent grade lives on separate validator agents (trace #18).

---

## Handoff to `verify-output`

Artifact + this trace are ready for an **independent** grade (a different agent). Expected check surface: the five conventions (classification / router-entry-noted / triggers / rebinds / single-source), kernel-free, sound-loop placement (producer-side only; loop rehomed to lead), the P5 seam (no restated `constraints-and-decisions.md` artifact survives in P6), and the trace audit (no silent loss).
