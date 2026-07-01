# Assessment — `patterns-api-contracts` (P8, plan cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:assess-primitive` · **Date:** 2026-06-30
**Source:** `human-in-loop/plugins/humaninloop/skills/patterns-api-contracts/` (SKILL.md + `references/ERROR-PATTERNS.md` + `references/PAGINATION-PATTERNS.md` + `references/OPENAPI-TEMPLATE.yaml` + `scripts/validate-openapi.py`)
**Role this run:** ASSESS / DIAGNOSE ONLY — no edits, no grading. Relational moves emitted as `flag-for-reconcile`.
**Run context:** plan-core port; one of `technical-analyst` (P2)'s DESIGN-phase skills.

---

## ASSESSMENT (skill output format)

```
ASSESSMENT: patterns-api-contracts
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]   (≥1 yes → full 7-check lens)
Disposition:  port-with-edits × standalone
Trace:
  - Endpoint mapping (user action → HTTP method + path pattern)                                  → kept
  - Method selection (POST/PUT/PATCH/GET/DELETE + idempotency)                                   → kept
  - Resource naming conventions (plural nouns, kebab-case, path/query params, nesting)           → kept
  - Endpoint documentation format (description, source reqs, request/response, error cases)       → kept
  - Request/response schema definition (OpenAPI schema format + examples)                         → kept
  - Type mapping from data model (Data-Model Type → OpenAPI Type/Format)                          → kept  [HANDOFF SEAM w/ P7 — shared conceptual-type vocab; dedupe candidate — reconcile]
  - Error-response design (standard format, machine-readable codes, status table) + ERROR-PATTERNS.md → kept
  - List endpoints: pagination / filtering / sorting + PAGINATION-PATTERNS.md                     → kept
  - Brownfield API alignment (base path, auth, error format, pagination; reuse/rename/new collision) → kept
  - OpenAPI structure (minimal + OPENAPI-TEMPLATE.yaml copy-ready template)                        → kept
  - Traceability (endpoint → FR/US mapping)                                                       → kept
  - Deterministic OpenAPI linter (validate-openapi.py)                                            → kept  [skill-local Tier-1 assert; determinism boundary — NOT the substantive validator]
  - Quality checklist (pre-finalize self-check)                                                   → kept
  - Common Mistakes (verbs-in-URLs, GET-state-change, missing errors, naming, generic codes, examples, brownfield) → kept
  - x-integration integration-boundary AUTHORING (failure modes / fallback / auth / per-endpoint  → FLAG-FOR-RECONCILE  [EXPECTED here per run-context; CURRENTLY ABSENT from body — must NOT be silently dropped]
        x-integration OpenAPI extension) for endpoints wrapping external systems
  - model-invocation trigger `description`                                                        → kept-but-rebind  [user-utterance → work-context triggers; agent-consumed]
  - "When NOT to Use" sibling cross-refs (humaninloop:patterns-entity-modeling / patterns-technical-decisions) → kept-but-rebind  [humaninloop: → mochiko:; both targets IN plan-core → won't dangle]
  - "When NOT to Use" scope exclusions (GraphQL / WebSocket-streaming / internal microservice)    → kept  [legitimate scope boundaries]
  - Example/invocation paths (scripts/validate-openapi.py path/to/openapi.yaml; contracts/ dir)   → kept-but-rebind  [→ .mochiko/ contracts location]
  - classification tag (absent in HIL)                                                            → assigned model-invoked (default) via wiring pass
  - router registration (absent in HIL; no plan/design router section yet)                        → added via wiring pass (new Plan-cluster section)
Reconcile flags:
  (1) x-integration responsibility home — run-context "self-contained here" assumption is FALSE (content absent); P5 points here, P2 claims it, P9 grades it Critical → where does the authoring land?
  (2) P7↔P8 handoff/overlap — shared conceptual-type vocabulary at the data-model→api-schema seam + schema-model consistency check (owned by P9/command, not either skill) → dedupe candidate, NOT a merge
  (3) [light] shared validate-*.py scaffold across P7/P8 (+ specify authoring scripts) → keep-distinct-vs-factor; determinism boundary
```

---

## Step 1 — Class / branch

**skill → PLAYS-a-role.** It is one of `technical-analyst` (P2)'s DESIGN-phase authoring procedures (the cluster casts P2 as the producer mounting P5–P8). Weighting per branch: consumed-procedure vs emits-artifact, trigger reliability, **sibling overlap** (the load-bearing check here — P7 entity-modeling and the x-integration boundary), decoupling. Classification/loop-ownership checks are de-weighted (those sit on the command P1, not a skill).

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (kernel / md-supervisor / command / DAG to function)? | **no** — pure self-contained API-design guidance + a skill-local linter. Survives without the brain (ROADMAP test passes). The HIL DAG/catalog sequences the *agent*, not this skill. Body grep is clean: zero `.humaninloop/` / `.workflow` / catalog / DAG / MCP / brain / hil-dag tokens; the only `integration` string in the whole skill is a *noun in the linter's singular-noun list* (`validate-openapi.py:39`). |
| 2 | Multi-responsibility / fans out? | **yes** — endpoint mapping + method selection + naming + schema definition + type mapping + error design + pagination/filtering/sorting + brownfield alignment + OpenAPI structure + traceability + linter. Feeds `contracts/api.yaml`, consumed by the P9 reviewer and the downstream implement cluster. |
| 3 | Emits artifact whose correctness is NOT machine-checkable? | **yes** — *format/conventions* are machine-checkable (the linter: syntax, plural nouns, kebab-case, error-response presence, operationId, security schemes, examples, descriptions); *substance* (the right endpoints for the user actions, schema↔data-model fidelity, realistic error cases, genuine integration failure modes) needs model judgment + the independent reviewer. |

≥1 yes → **full lens**. Gates 2 + 3 tripped (gate 1 no).

## Step 3 — The 7-check lens (weighted for PLAYS-a-role / skill)

1. **Orchestration test.** Content-coupling: **none** (grep clean — no kernel/DAG/catalog/MCP in SKILL.md, either reference, the template, or the script). Orchestration-coupling: the skill is *consumed* by `technical-analyst` via the Skill tool; in HIL the DAG sequences the agent through analysis→design. Nothing orchestration-bound lives *in* this skill, so nothing re-homes **from** it. The 2-phase sequence, dual-reviewer ordering, skip-re-review routing, and incremental Phase-3 review are the **command's** (P1) to rehome — not this skill's.

2. **Role (two altitudes).** Skill-role = **emits-artifact** (the `contracts/api.yaml` OpenAPI spec + the endpoint/error/traceability sections; also run as a consumed design step, but its output is a reviewable artifact). Team-role conferred = **producer** — running it makes the analyst a producer. The embedded Quality Checklist + linter are *producer self-checks*, **not** an independent grade → **not promote material** (the substantive grade is P9 `validation-plan-artifacts`, a different agent's skill).

3. **Independence.** P2 mounts P5–P8 — **all producer/authoring skills, no grading skill**. No self-grade leak inside this skill or its co-mount set. Independence is structural at cluster level: `technical-analyst` (produce) ≠ the plan reviewer(s) (grade, via `validation-plan-artifacts` / feasibility). The *shape* of that reviewer side is RQ1/RQ2 (reconcile/human-gated) — not decided here, and not this skill's leak to fix. ✓

4. **Verdict-sink / loop-driver.** This skill emits **no verdict**. Verdict-sink = the plan reviewer's gate (P9 + feasibility). FAIL-loop (revise-design-on-gaps) is driven by the command supervisor (lead). No loop owned here → nothing to rehome from P8.

5. **Sibling / overlap.** The weighted check → two findings, both `flag-for-reconcile`, detailed below: **(A)** the **x-integration** boundary responsibility (claimed-but-absent), **(B)** the **P7↔P8** conceptual-type handoff seam. Neither is a merge.

6. **Coupling audit.**
   - Hardcoded paths: **none** `.humaninloop/`. Light rebinds only: the linter invocation example (`scripts/validate-openapi.py path/to/openapi.yaml`) and the `contracts/` artifact mention → `.mochiko/` contracts location; `humaninloop:` skill cross-refs → `mochiko:`.
   - Prerequisites/handoffs: **consumes** `data-model.md` (the "Type Mapping from Data Model" table is the explicit upstream seam — it requires P7's entity types) and the requirements (FR/US for traceability); **writes** `contracts/api.yaml`. These are **command-owned handoff edges**, not skill-owned coupling.
   - **Determinism boundary:** `validate-openapi.py` is a **deterministic format/convention linter** (OpenAPI 3.x syntax, info section, plural-noun + kebab-case path checks, error-response presence, request-body presence, operationId presence/uniqueness, security-scheme presence, schema examples, descriptions). It is skill-local (reads a file-path arg; stdlib + optional `yaml`), **not** brain/catalog/DAG/MCP. A Tier-1 self-check, **not** the substantive validator (substance = schema-model consistency, right endpoints, realistic failure modes — model judgment, owned by P9 + P2). Disposition consistent with the specify precedent (`validate-requirements.py` kept): **keep**. (Defensible alternative = drop the linter, lean on model + reviewer; minimalism + the contract's "Tier-1 sub-checks layered where they exist" favor keep. Keep is not a gated disposition → no human gate needed.)

7. **Conventions + loop placement.**
   - Classification: **absent** → assign **model-invoked** (default; agent-consumed) via wiring.
   - Discoverability: not yet in the mochiko router (no plan/design cluster section exists) → register via wiring (new Plan-cluster section).
   - Reliable model-invocation: current `description` is the **"when the user says 'design API' / 'OpenAPI spec' …"** user-utterance pattern. For an **agent-consumed** skill, mochiko convention says describe the **work context** (avoid false auto-trigger expectations) → `port-with-edits` rephrase (keep graded MUST/SHOULD trigger phrases, anchor them to API-design work). **This is the main *wiring* body edit.** Sibling-collision note: triggers overlap with P7 entity-modeling only at the generic "schema" token — P7 owns "data model / entities / relationships / state machine," P8 owns "endpoint / OpenAPI / contract / HTTP." Low collision; the work-context rephrase widens the gap. Wiring-pass detail, not a flag.
   - Agent↔skill composition / **decoupling scan: CLEAN.** Grep + manual read of SKILL.md + both references + the template + the script found **no** sibling-agent names, **no** "dispatch," **no** injected workflow modes/paths/phases, **no** "workflow-agnostic"/independence-by-declaration meta-label, **no** "Generated by <persona>" stamp. (The only `mode/path/phase` grep hits were `file_path` *function parameters* in the linter — false positives.) Pure procedure; persona correctly lives in P2, not here. Independence stated by *role*, not agent name. ✓
   - Producer↔validator pairing: the artifact's independent validator exists at cluster level (P9 `validation-plan-artifacts`, dispatched on a different agent — RQ1 shape pending). This skill is the producer half; the linter is a degenerate deterministic self-check, not that validator.
   - Sound-loop: the enclosing plan loop's done-condition / independent-validation / human gate are the **command's** (P1 + contract) concern, not this skill's. No loop gap owned here.

## Step 4 — Disposition

**`port-with-edits × standalone`** (matches the run-context hypothesis).

- **Body = port-with-edits** (not keep-verbatim, not redesign, not drop): the body is high-quality and mochiko-clean in substance (endpoint mapping, method/idempotency, REST naming, schema + type-mapping, error design, pagination, brownfield, OpenAPI template, traceability, checklists, Common Mistakes — all kept). Edits are localized: **(a)** rephrase the `description` from user-utterance triggers to work-context triggers; **(b)** rebind `humaninloop:` → `mochiko:` cross-refs; **(c)** rebind the linter-invocation / `contracts/` example paths to `.mochiko/`; **(d) [substantive, conditional]** if reconcile lands the **x-integration** authoring responsibility here (Flag 1), `port-with-edits` must **ADD** that content (the integration-boundary procedure + the per-endpoint `x-integration` OpenAPI-extension format: system name, protocol, API version, criticality, failure modes [detection/impact/fallback], auth). It is currently absent — see Flag 1. Adding it is an additive localized edit to a clean body, **not** a `redesign`. Not `keep-verbatim` because (a)–(c) are required; not `redesign` because the approach assumes no kernel/DAG/catalog and is right for mochiko; not `drop`.
- **Structural = standalone**: this is `technical-analyst`'s distinct API-design producer procedure, mounted on P2. Its placement does **not** depend on a sibling. The two relational questions (x-integration home; the P7↔P8 type-vocabulary dedupe) are **content/responsibility** questions, not structural moves *of P8* — they are flags, and `standalone` holds regardless of how they resolve (P8 stays the analyst's API skill either way).

## Step 5 — Responsibility trace

Complete trace in the ASSESSMENT block above. **No `dropped` responsibilities.** One responsibility — **x-integration boundary authoring** — is tagged `FLAG-FOR-RECONCILE` rather than a final landing tag, **precisely because it must not be silently lost**: the run context assumes it lives here, P5 forward-references here, P2's persona promises it, and P9 grades it Critical, yet the skill body does not contain it. Its final tag (`kept-but-rebind`+ADD if landed here, else `moved-to-sibling-skill` → P5, else `split`) is reconcile's to assign with full cluster context. Everything else: three `kept-but-rebind` (description triggers, sibling namespaces, example paths), two new wiring assignments (classification, router), the rest `kept` — one `kept` (type-mapping table) carries a cross-skill dedupe candidate (Flag 2) that only reconcile can assign.

---

## SIBLING-OVERLAP FINDINGS → flag-for-reconcile

### FLAG 1 (load-bearing) — x-integration boundary: the "self-contained here" assumption is FALSE

The run context asked me to **confirm** that, after HIL's ADR-008 techspec→plan merge, the integration boundaries "now live in this skill's x-integration docs (rather than a separate integrations.md)" and are "self-contained here." **They are not.** Evidence (grep-traced across the HIL plugin):

- **`patterns-api-contracts` (P8) contains ZERO x-integration content.** No "integration boundary," no "failure modes," no "fallback," no `x-integration` extension format anywhere in SKILL.md, either reference, the OpenAPI template, or the script. (The lone `integration` token is a noun in the linter's singular-noun list.) The skill the run context names as the home does not hold the responsibility.
- **The authoring procedure actually sits in P5 `authoring-technical-requirements`**, which now *points back here*: "Integrations are now documented as `x-integration` extensions per endpoint in `contracts/api.yaml` during Phase 2 (Design). **See the `humaninloop:patterns-api-contracts` skill for details.**" — a **dangling forward-reference**: P5 says "see P8 for details," P8 has no details. P5 still carries the substance (fields per integration: system name, protocol, API version, criticality, failure modes [detection/impact/fallback], auth — `authoring-technical-requirements/SKILL.md:117`).
- **P2 `technical-analyst` persona promises it** ("Endpoints wrapping external systems include integration boundary documentation (failure modes, fallback strategies, authentication)" — `agents/technical-analyst.md:85`).
- **P9 `validation-plan-artifacts` grades it Critical** ("Do endpoints wrapping external systems have x-integration documentation?" / "Does every integration boundary have documented failure modes and fallbacks?" — `PHASE-CHECKLISTS.md:110-111`), and **P1 `plan` command lists it as a Full Review Check** (`commands/plan.md:570`).

So there is a four-way claim on one responsibility whose *authoring guidance* is currently homeless-in-practice (referenced everywhere, authored in P5, expected in P8). This is a genuine HIL-source gap, not a mochiko artifact.

**Resolution options for reconcile (do NOT decide here):**
- **(i) Add to P8** (most consistent with ADR-008 + this run's scope + P5's own pointer): port-with-edits ADDs the x-integration authoring section + extension format to `patterns-api-contracts`; P5's INT-XXX slice becomes a reference. Makes P8 genuinely self-contained as the run context assumed.
- **(ii) Keep in P5**: x-integration authoring stays with `authoring-technical-requirements`; P8 only references it; fix P5's dangling pointer to stop claiming P8 holds details.
- **(iii) Split**: a shared integration-boundary substrate both P5 and P8 reference.

**Constraint reconcile must honor:** the responsibility **cannot be dropped** — P9 grades it Critical, so dropping it would fail the plan loop's own validation gate. Whichever skill ends up authoring it, P9 must still be able to grade it and P2's persona promise must be backed by a real procedure.

**Flag payload:** `patterns-api-contracts (P8) : x-integration boundary authoring EXPECTED here (run-context) but ABSENT; P5 forward-references here, P2 persona claims it, P9 grades it Critical → decide add-to-P8 vs keep-in-P5 vs split; MUST NOT be dropped.`

### FLAG 2 — P7 `patterns-entity-modeling` ↔ P8 handoff/overlap (schemas must match entities)

**What they genuinely share (the seam the run context flagged):**
- A **conceptual-type vocabulary at the data-model→api-schema handoff.** P7 *defines* the conceptual types (Identifier/UUID, Text, Email, URL, Integer, Decimal, Boolean, Timestamp, Date, Enum, JSON, Reference — `patterns-entity-modeling/SKILL.md:94-112`); P8 *maps* them to OpenAPI (`Data-Model Type → OpenAPI Type/Format` — `patterns-api-contracts/SKILL.md:108-122`: UUID→string/uuid, Email→string/email, Enum[a,b,c]→string/enum, …). The two type tables must stay in sync; that is the "schemas must match entities" coupling.
- **Mutual "When NOT to Use" cross-references** drawing the boundary (P8 → "Entity modeling: use patterns-entity-modeling first"; P7 → "API contract design: use patterns-api-contracts instead").

**What they do NOT share (why this is NOT a merge):**
- **Distinct artifacts:** `data-model.md` (entities, relationships, state machines, PII) vs `contracts/api.yaml` (endpoints, schemas, errors). Distinct procedures, distinct numbering, distinct scripts.
- **The "schema-model consistency" CHECK is owned by neither skill** — it lives in P9 `validation-plan-artifacts` ("Schema-model consistency: schemas match data model entities") and the P1 command's Phase-3 consistency check. So the overlap is a *handoff edge + a downstream cross-artifact check*, not duplicated producer logic.

**Therefore:** **NOT** `merge-into-sibling` (no thin-variant-over-shared-core; they are peer producers of different artifacts). Recommended resolution = **`standalone` for both**, with an **optional** `dedupe` of the shared conceptual-type vocabulary *only if* reconcile judges a single shared type-reference worth it; the handoff edge (data-model → api schema) is **command/lead-owned**, and the consistency check is **P9-owned**. My lean: **keep distinct** (factoring the type table manufactures cross-skill coupling for marginal benefit, against kernel-free minimalism), but reconcile owns the call with P7's assessment in hand.

**Flag payload:** `patterns-entity-modeling (P7) ↔ patterns-api-contracts (P8) : shared conceptual-type vocabulary at the data-model→api-schema handoff seam + a schema-model-consistency check owned by P9/command → dedupe candidate; resolve standalone-for-both vs factor-shared-type-table; NOT a merge.`

### FLAG 3 (light) — shared `validate-*.py` scaffold

`validate-openapi.py` (P8) and `validate-model.py` (P7) — and the specify authoring scripts already flagged — share the same HIL deterministic-linter scaffold (find→check→`checks`/`summary` JSON→exit-code `main()`), checking different artifacts. Same dedupe-candidate-vs-keep-distinct question raised in the specify cluster; determinism boundary noted in check 6. My lean: keep distinct (minimalism). Reconcile may consolidate cross-cluster or leave it.

---

## Decoupling-scan result (run-goal: empirical decoupling-doctrine test)

**Zero deny-list hits.** Grep + manual read of SKILL.md, both references, the OpenAPI template, and the linter found **no** sibling-agent names, **no** "dispatch," **no** injected workflow modes/paths/phases, **no** "workflow-agnostic" meta-label, **no** "Generated by <persona>" stamp. The only `mode/path/phase` grep matches were `file_path` *function parameters* in the script. The skill states its job by *work* (REST/OpenAPI design), not by naming P2 or the plan workflow. The only caller-coupling-adjacent items are the `humaninloop:` skill cross-references in "When NOT to Use" (a namespace `kept-but-rebind`, not a coupling violation) — and unlike the specify ports, both targets (`patterns-entity-modeling` P7, `patterns-technical-decisions` P6) are **inside plan-core**, so these cross-refs **will resolve, not dangle**.

---

**Done-condition (this assessment):** every responsibility tagged ✓ · no untagged responsibility ✓ · the one claimed-but-absent responsibility (x-integration) flagged, not silently dropped ✓ · relational moves flagged not guessed ✓ · ASSESS-only, no edits, no grade ✓.
