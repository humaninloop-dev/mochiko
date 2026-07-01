# TRANSFORM: patterns-api-contracts (P8)

> Role: **apply + wire only.** Disposition was finalized by assess + reconcile; this file applies it and updates the trace. **Not graded here** — `verify-output` runs independently, on a different agent.
> Disposition (finalized): **`port-with-edits × standalone`** — the producer's API-contract design skill. **Canonical home for x-integration (RQ6b) — a SUBSTANTIVE content ADD, not just wiring.**
> Source: `human-in-loop/plugins/humaninloop/skills/patterns-api-contracts/` (SKILL.md + references/ERROR-PATTERNS.md + references/PAGINATION-PATTERNS.md + references/OPENAPI-TEMPLATE.yaml + scripts/validate-openapi.py).
> x-integration substance absorbed from P5 `authoring-technical-requirements` (SKILL.md §4 + references/ARTIFACT-TEMPLATES.md §5 — the INT-XXX field schema: system / protocol / API version / criticality / failure-modes [failure·detection·impact·fallback] / auth).

```
TRANSFORM: patterns-api-contracts
Applied:   port-with-edits × standalone + wiring-pass (all 6)
Artifacts: plugins/mochiko/skills/patterns-api-contracts/SKILL.md            (port + ADD Integration Boundaries section)
           plugins/mochiko/skills/patterns-api-contracts/references/ERROR-PATTERNS.md       (verbatim)
           plugins/mochiko/skills/patterns-api-contracts/references/PAGINATION-PATTERNS.md  (verbatim)
           plugins/mochiko/skills/patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml   (port + ADD x-integration example on /auth/login)
           plugins/mochiko/skills/patterns-api-contracts/scripts/validate-openapi.py        (port + rebind usage path + ADD x-integration well-formedness check)
New partners: none (standalone). The artifact's independent grade is a cross-cluster pairing (P9 validation-plan-artifacts completeness + validation-feasibility), wired at plan-loop build — never co-mounted on the producer.
Wiring:    classification=model-invoked  router=NOT-edited (entry noted below — shared router)
           triggers=work-context graded (design API / map endpoints / define schemas / API contract / OpenAPI / integration boundaries), de-collided vs P7
           rebinds=humaninloop:→mochiko: (2 sibling cross-refs); linter-invocation + contracts/ dir → .mochiko/specs/<feature>/contracts/api.yaml
           decouple=ZERO deny-list hits in source (assess-confirmed); kept clean — added content names no agent/workflow
           single-source=type vocabulary referenced to mochiko:patterns-entity-modeling (not restated); x-integration is now the canonical single source P5 references
```

---

## The load-bearing move — x-integration is now PRESENT (RQ6b repaired)

**Before (HIL):** `patterns-api-contracts` contained **ZERO** x-integration content. P5 forward-referenced it ("see `humaninloop:patterns-api-contracts` for details") — a **dangling pointer** (P5 said "see P8", P8 had nothing). P2's persona promised it; P9 grades it **Critical**.

**After (this port):** the target is now real. P8 contains a dedicated **`## Integration Boundaries`** section (SKILL.md L173–242) with the full per-endpoint **`x-integration` OpenAPI-extension format**, plus a worked example on `/auth/login` in OPENAPI-TEMPLATE.yaml, plus a deterministic well-formedness check in the linter. The dangling pointer is repaired by making the pointed-at content exist.

**The `x-integration` format as authored (all six fields canonical):**

| Field | Required | Format |
|-------|----------|--------|
| `system` | Yes | external system name + version |
| `protocol` | Yes | REST / GraphQL / gRPC / webhook / SDK / queue |
| `api_version` | Yes | the external contract version targeted |
| `criticality` | Yes | Critical / Important / Optional |
| `auth` | Yes | outbound-call auth mechanism + secret source |
| `failure_modes` | Yes | non-empty array; each entry = **failure · detection · impact · fallback** |

Carries P5's doctrine verbatim-in-spirit: *"Optimistic integration maps are incomplete: every external dependency fails eventually — document what happens when it does."* **Cannot be dropped** — P9 grades it Critical; dropping it would fail the plan loop's own validation gate.

### Phrasing P5 should reference (so P5's port matches the seam)

P5 keeps a **thin INT-XXX analysis declaration** (flag *which* external systems are integration concerns, as a requirement) and **references P8** for the per-endpoint design format. Replace HIL's dangling pointer with:

> **Integrations** — Document each external-system dependency as an **`x-integration`** extension on the operation that wraps it, in `contracts/api.yaml`, during design. The per-endpoint `x-integration` format (system, protocol, API version, criticality, failure modes [failure / detection / impact / fallback], auth) and the integration-boundary authoring procedure are owned by **`mochiko:patterns-api-contracts`** — see that skill for the format. This requirements layer only declares *which* dependencies are integrations and their business criticality; it does not restate the per-endpoint format.

> Reciprocal expectation already satisfied here: P8 **contains** the format (no forward-ref back to P5), so the pointer resolves. Bare-name (`mochiko:<skill>`) cross-reference form, matching the ported `authoring-requirements` / `patterns-technical-decisions` convention.

---

## SHARED SEAM BOUNDARIES — as applied

Per `reconcile.md` §RQ6b and the P7↔P8 / P5↔P8 rows of the relational-verdicts table:

- **P8 (this skill) OWNS:** API contracts (endpoint mapping, method/idempotency, REST naming, request/response schemas, type-mapping, error design, pagination, OpenAPI spec) **AND** the **x-integration boundary authoring** + per-endpoint extension format (ADDED — was ZERO in HIL P8).
- **P5 keeps** a thin INT-XXX analysis declaration and **references P8** (phrasing above). P5's INT-XXX *authoring* → `moved-to-sibling-skill` → P8.
- **P7 owns the data-model.** P8 **consumes** P7's conceptual types at the schema seam and **references** them — it does **not** restate P7's type vocabulary. Applied at SKILL.md L111: the Type-Mapping table is prefaced *"The conceptual types come from the data model (`mochiko:patterns-entity-modeling`) … reference the data model's types, don't redefine them here."* The schema-model **consistency check** is **P9-owned**; the data-model→schema **handoff edge** is **command/lead-owned**. **Kept distinct — NOT a merge** (peer producers of distinct artifacts; factoring the type table would manufacture cross-skill coupling against kernel-free minimalism).
- **Shared `validate-*.py` scaffold (P7/P8)** → kept distinct (minimalism); each linter checks its own artifact.

---

## Wiring pass — all six conventions

| # | Convention | Applied |
|---|------------|---------|
| 1 | **Classification** | **model-invoked** (no `disable-model-invocation`). Agent-consumed producer skill; mounts on the `technical-analyst` producer (a `skills:`-list fact wired when P2 ports — not a P8-body edit). |
| 2 | **Router registration** | **NOT edited** (shared router / `plugin.json`, per instruction). Entry text provided below. The `mochiko` router has **no Plan cluster block yet** — the entry lands when the plan cluster registers. |
| 3 | **Trigger phrasing** | `description` rewritten from HIL's literal *"when the user says 'design API' / 'OpenAPI spec' …"* to **work-context** triggers describing the design work. Required tokens present: design API / map endpoints / define schemas / API contract / OpenAPI / **integration boundaries**. **De-collided vs P7**: P8 claims *endpoint / API contract / request-response schema / OpenAPI / HTTP / integration boundary*; data-model/entities/relationships are explicitly ceded to P7 (named only as the upstream type seam). |
| 4 | **Path rebinding** | (a) `humaninloop:patterns-entity-modeling` + `humaninloop:patterns-technical-decisions` → `mochiko:` (When-NOT-to-Use). (b) Linter invocation `scripts/validate-openapi.py path/to/openapi.yaml` → `.mochiko/specs/<feature>/contracts/api.yaml` (SKILL Validation + script docstring). (c) "Creating contracts/ directory artifacts" → "Creating `contracts/` directory artifacts" under the `.mochiko/specs/<feature>/` convention. No `.humaninloop/` literals existed in source (assess grep-clean). |
| 5 | **Decouple persona/skill** | Source had **ZERO** deny-list hits (assess-confirmed: no agent names, no "dispatch", no injected workflow modes/paths/phases, no "workflow-agnostic" label). **Kept clean** — the ADDED Integration-Boundaries content states its job by *work* (authoring boundaries for external-system endpoints), names no agent and no workflow. The only cross-refs are skill-namespace references (`mochiko:patterns-entity-modeling`/`-technical-decisions`) — a `kept-but-rebind`, not a coupling violation; both targets are in plan-core so they resolve. |
| 6 | **Single-source / de-duplicate** | (a) **Type vocabulary**: referenced to `mochiko:patterns-entity-modeling`, not restated (P7 owns the conceptual types). (b) **x-integration**: P8 is now the **canonical single source**; P5 references it (no duplicate authoring survives in P5 after its port). (c) PLAYS-a-role skill → no `loop-discipline`/`workflow-contract` obligation (that floor is the command's, not a producer skill's). |

### Router entry needed (for the shared-router owner — NOT applied here)

To be added under a new **Plan cluster** block of `plugins/mochiko/skills/mochiko/SKILL.md` when the plan cluster registers:

```
| `patterns-api-contracts` | designing the API-contract layer — mapping user actions to REST endpoints (method/idempotency/naming), request/response schemas (mapping data-model types to OpenAPI), error design, list pagination, and the per-endpoint `x-integration` boundary for endpoints wrapping external systems; assembles `contracts/api.yaml`. Owns the API contract + x-integration format; consumes `patterns-entity-modeling`'s conceptual types at the schema seam |
```

Agent-mount note (for P2 `technical-analyst` port): add `patterns-api-contracts` to the producer's `skills:` list (with P5/P6/P7). **Independence guard:** never co-mount a grading skill (`validation-plan-artifacts`, `validation-feasibility`) on `technical-analyst`.

---

## Realized responsibility trace (every responsibility flipped to its final tag — no silent loss)

| # | Responsibility | assess tag | **realized tag** | Where it landed |
|---|----------------|-----------|------------------|-----------------|
| 1 | Endpoint mapping (user action → HTTP method + path pattern) | kept | **kept** | SKILL "Endpoint Mapping" (verbatim) |
| 2 | Method selection (POST/PUT/PATCH/GET/DELETE + idempotency) | kept | **kept** | SKILL "Method Selection" (verbatim) |
| 3 | Resource naming conventions (plural, kebab-case, path/query/nesting) | kept | **kept** | SKILL "Resource Naming Conventions" (verbatim) |
| 4 | Endpoint documentation format | kept | **kept** | SKILL "Endpoint Documentation Format" (verbatim) |
| 5 | Request/response schema definition (OpenAPI + examples) | kept | **kept** | SKILL "Schema Definition" (verbatim) |
| 6 | Type mapping from data model | kept (dedupe candidate → reconcile) | **kept + seam-reference** | SKILL "Type Mapping from Data Model" — table kept; prefaced with a reference to `mochiko:patterns-entity-modeling` (consume P7's types, don't restate). Reconcile resolved **keep-distinct** |
| 7 | Error-response design + ERROR-PATTERNS.md | kept | **kept** | SKILL "Error Response Design" + references/ERROR-PATTERNS.md (verbatim) |
| 8 | List endpoints: pagination/filtering/sorting + PAGINATION-PATTERNS.md | kept | **kept** | SKILL "List Endpoints" + references/PAGINATION-PATTERNS.md (verbatim) |
| 9 | Brownfield API alignment (base path, auth, error, pagination; reuse/rename/new) | kept | **kept** | SKILL "Brownfield Considerations" (verbatim) |
| 10 | OpenAPI structure + OPENAPI-TEMPLATE.yaml | kept | **kept + ADD** | SKILL "OpenAPI Structure" (kept); OPENAPI-TEMPLATE.yaml kept **+ x-integration example added on /auth/login** to demonstrate the canonical format |
| 11 | Traceability (endpoint → FR/US) | kept | **kept** | SKILL "Traceability" (verbatim) |
| 12 | Deterministic OpenAPI linter (validate-openapi.py) | kept (Tier-1 assert; determinism boundary) | **kept-but-rebind + ADD** | scripts/validate-openapi.py — usage path rebound to `.mochiko/...`; **narrow `check_integration_boundaries` added** (format-only well-formedness *when present*; presence NOT required — that stays model/P9 judgment). Still a Tier-1 self-check, NOT the substantive gate |
| 13 | Quality checklist (producer self-check) | kept | **kept + ADD** | SKILL "Quality Checklist" — kept; **+ x-integration item**. Stays author-side self-check, NOT a validator rubric (independence preserved) |
| 14 | Common Mistakes | kept | **kept + ADD** | SKILL "Common Mistakes" — kept; **+ "Optimistic Integration Boundaries"** mistake |
| 15 | **x-integration boundary AUTHORING + per-endpoint extension format** | **FLAG-FOR-RECONCILE** | **ADDED (canonical home)** | **SKILL "Integration Boundaries" (L173–242) — NEW.** The substantive ADD: when-to-use, the `x-integration` format (system/protocol/api_version/criticality/auth/failure_modes[failure·detection·impact·fallback]), field tables, criticality levels, the "every dependency fails" doctrine. Absorbed from P5 §4 + ARTIFACT-TEMPLATES §5. **Cannot drop (P9 Critical).** |
| 16 | model-invocation trigger `description` | kept-but-rebind | **kept-but-rebind** | `description` rewritten to work-context triggers; de-collided vs P7 |
| 17 | "When NOT to Use" sibling cross-refs (entity-modeling / technical-decisions) | kept-but-rebind | **kept-but-rebind** | `humaninloop:` → `mochiko:`; both targets in plan-core → resolve, don't dangle |
| 18 | "When NOT to Use" scope exclusions (GraphQL / WebSocket / internal microservice) | kept | **kept** | legitimate scope boundaries (verbatim) |
| 19 | Example/invocation paths (linter arg; contracts/ dir) | kept-but-rebind | **kept-but-rebind** | → `.mochiko/specs/<feature>/contracts/api.yaml` |
| 20 | Classification tag (absent in HIL) | assigned model-invoked | **assigned** | model-invoked (no `disable-model-invocation`) |
| 21 | Router registration (absent in HIL) | added via wiring | **noted (not applied)** | Shared router NOT edited per instruction; Plan-cluster entry text supplied above for the router owner |
| 22 | INT-XXX authoring substance (provenance) | — (P5's trace: moved-to-sibling-skill→P8) | **received here** | The authoring that moved out of P5 lands as #15 above; P5's reference phrasing supplied above so the seam closes both ways |
| 23 | Independent validation of the contracts artifact | flag → (reconcile) | **moved-to-validator** | Cross-cluster pairing: graded by `devils-advocate`·`validation-plan-artifacts` (completeness, incl. "every integration boundary has failure modes + fallbacks") + `principal-architect`·`validation-feasibility` (feasibility), on independent validator agents — **never** on `technical-analyst`. Wired at plan-loop build; not a P8-body edit |

**Drops: none.** P8 carried no kernel/DAG/catalog/MCP content (assess grep-clean). Every responsibility is `kept`, `kept + ADD`, `kept-but-rebind`, `assigned`, `noted`, `moved-to-validator`, or the canonical **ADD** (#15). The one previously claimed-but-absent responsibility (x-integration) is now genuinely present, not silently lost.

**Determinism-boundary note (for the verifier to audit):** the linter's new `check_integration_boundaries` is deliberately **presence-optional** and **format-only** — it validates the *shape* of an `x-integration` block when one exists (required fields, well-formed failure modes, valid criticality enum) but never requires presence and never judges whether the documented failure modes are *realistic*. Deciding *which* endpoints wrap external systems, and whether the failure modes are genuine, remains model judgment owned by the independent plan reviewer (P9, Critical). The check sits in the same Tier-1 tier as the existing format checks; it does not become the substantive gate. (Verified: catches missing api_version/auth, invalid criticality, and failure-mode missing fallback; passes a well-formed block; no-op when x-integration absent. End-to-end run on the ported template: all 11 checks present, `integration_boundaries` PASS; the only 2 failing checks are pre-existing illustrative-template traits — action endpoints without request bodies + schemas without examples — identical to HIL, not introduced by this port.)

**Independence note (non-negotiable):** P8 confers a **producer** team-role only. Its Quality Checklist and its linter are producer *self*-checks, not grading rubrics. Nothing in this transform mounts a validator capability on P8 or on its producer agent. The contracts artifact's independent grade lives on separate validator agents (trace #23).

---

## Handoff to `verify-output`

Artifact + this trace are ready for an **independent** grade (a different agent). Expected check surface: the five conventions (classification / router-entry-noted / triggers / rebinds / single-source), kernel-free, sound-loop placement (producer-side only; loop rehomed to lead), the seams (x-integration now present and self-contained; P7 type vocabulary referenced not restated; no duplicate INT-XXX authoring expected to survive in P5 after its port), the determinism-boundary call on the linter's new check, and the trace audit (no silent loss; x-integration flipped from flag → ADD, not dropped).
