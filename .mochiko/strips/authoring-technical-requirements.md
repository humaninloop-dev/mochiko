# Strip notes — `skills/authoring-technical-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T2;
ratified 2026-07-24).

## [v0.77.0] `references/ARTIFACT-TEMPLATES.md` retired — the 3 analysis-artifact templates move to schemas (D3 later-ratchet)
- **Disposition:** superseded → `plugins/mochiko/schemas/requirements.yaml` + `constraints-and-decisions.yaml` + `nfrs.yaml` + `mochiko-cli`
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, full verbatim below):**

````markdown
# Artifact Templates

Templates and field definitions for the **three analysis artifacts this skill authors**: `requirements.md` (TR-XXX), `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX), and `nfrs.md` (NFR-XXX). All three follow the deliverable envelope in [`artifact-format.md`](../../../templates/artifact-format.md): the statement carries the content (no separate Description paragraph), entries are one line each, upstream text is cited by ID and never re-quoted, and each artifact's summary table is its **ID index** — the coverage surface reviewers verify against. Register: `full` per that envelope's rule 11 — with every ID, numeric target and constraint clause a never-compress item, and compression stopping wherever it would make a requirement ambiguous.

> **Design-artifact templates live with their canonical owners.** The `data-model.md` template (entities + per-attribute data-sensitivity taxonomy) is owned by `mochiko:patterns-entity-modeling`; the `contracts/api.yaml` template (endpoints, schemas, and the `x-integration` boundary extension) and the integration `quickstart.md` are owned by `mochiko:patterns-api-contracts`. This bundle declares the analysis requirements those design artifacts build on — it does not restate their templates.

## 1. Technical Requirements (requirements.md)

### Document Template

```markdown
# Technical Requirements: {feature_id}

> Technical decomposition of business functional requirements.

## Traceability Summary  *(the ID index)*

| Source FR | Technical Requirements | Coverage |
|-----------|----------------------|----------|
| FR-001 | TR-001, TR-002, TR-003 | Full |
| FR-002 | TR-004, TR-005 | Full |

---

## TR-001: [Descriptive Title]

**FR-001 · MUST** — System MUST [technical capability in technology-agnostic terms; the statement is the description].

**Criteria:**
- [Testable condition, one line]
- [Testable condition, one line]

**Deps:** C-001 · NFR-002  *(omit if none)*

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | TR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Source | Yes | FR-XXX reference(s) | On the statement line; must reference existing FR(s) |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119 keyword, on the statement line |
| Statement | Yes | One-to-two lines | Technology-agnostic; WHAT, not HOW — no separate Description paragraph |
| Criteria | Yes | Bullet list, one line each | Each item independently testable |
| Deps | No | ID references | C-XXX, NFR-XXX, other TR-XXX — cited by ID, never re-quoted |

### Decomposition Examples

**Business FR:** "Users must be able to sign in to their account" (FR-001)

| TR | Title | Aspect |
|----|-------|--------|
| TR-001 | Authentication Flow | Credential validation, error handling |
| TR-002 | Session Management | Token issuance, expiration, refresh |
| TR-003 | Account Lockout | Brute-force protection, lockout thresholds |
| TR-004 | Authentication Audit | Login attempt logging, anomaly flags |

Each TR addresses a distinct technical concern the single business FR implies but does not state.

### Writing Criteria

Good criteria are independently testable (pass/fail), technology-agnostic, and cover success, failure, and edge cases — one line each:

```markdown
## TR-001: Authentication Flow

**FR-001 · MUST** — System MUST validate credentials and establish an authenticated session.

**Criteria:**
- Valid credentials result in an authenticated session
- Invalid credentials return a generic error (no credential-type leakage)
- Expired accounts cannot authenticate
- Authentication completes within the NFR-001 latency target
- All attempts logged per TR-004
```

---

## 2. Constraints and Decisions (constraints-and-decisions.md)

### Document Template

```markdown
# Constraints and Decisions: {feature_id}

> Hard boundaries and technology choices that shape the implementation.

## Part 1: Hard Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-001 | infrastructure | Existing production environment | blocking |
| C-002 | regulatory | GDPR Article 17 | blocking |

---

### C-001: [Descriptive Title]

**infrastructure · blocking · source:** [where it originates] — [the boundary stated as a one-to-two-line fact, not a preference].

**Impact:** eliminates [design choice A] · requires [consideration B] · affects TR-001, TR-003 · shapes D-001
**Verify:** [how to confirm it still applies — one line]  *(omit if self-evident)*

---

## Part 2: Technology Decisions

> **Decision *technique* is owned by `mochiko:patterns-technical-decisions`.** The field schema below is the artifact slot a decision lands in; how to evaluate alternatives, score trade-offs, and set ADR depth lives in that skill. Fill these fields with the result; do not restate the evaluation method here.

### Decision Summary  *(the ID index — all D-XXX, both origins)*

| ID | Decision | Choice | Shaped By | Origin |
|----|----------|--------|-----------|--------|
| D-001 | Primary database | PostgreSQL 15 | C-001 | analysis |
| D-002 | Auth mechanism | JWT with refresh tokens | C-003 | analysis |
| D-004 | Avatar processing placement | async worker off a queue | NFR-002 | structural |

---

### D-001: [Decision Title]

**Context** ([one-to-two lines: the problem needing a decision]) · **Shaped by:** C-001 · NFR-001

| Option | Pros | Cons |
|--------|------|------|
| [Option A] | [one line] | [one line] |
| [Option B] | [one line] | [one line] |

**Choice:** [selected option] — **Rationale** (≤ 3 lines): [WHY this choice — the reasoning, not a restatement of the choice].
**Consequences:** [trade-off 1 accepted] · [trade-off 2] · [future consideration]
**Governance alignment:** [one line — only when a project principle applies; omit otherwise]

---

### Structural Decisions  *(architecture-time — authored by the architecture seat, not the analysis author)*

> Topology D-XXX rows (component boundaries, interaction style, responsibility placement) decided at
> the architecture stage and authored by `mochiko:patterns-system-design` — the analysis-time author
> **preserves** this subsection, never fills it. Same D-XXX record format as above, same ADR
> discipline (`mochiko:patterns-technical-decisions`), continuing the shared D-XXX sequence. The
> architecture delta summary links each structural change to its row here. Omit the subsection when
> the feature makes no structural decisions.

### D-004: [Structural Decision Title]

**Context** ([the topology choice needing a decision]) · **Shaped by:** NFR-002 · C-003

| Option | Pros | Cons |
|--------|------|------|
| [Option A] | [one line] | [one line] |
| [Option B] | [one line] | [one line] |

**Choice:** [selected shape] — **Rationale** (≤ 3 lines): [WHY this topology].
**Consequences:** [trade-off accepted] · [future consideration]
**Governance alignment:** [one line — the layer/dependency principle the shape respects; omit if none]

---

## Part 3: Infrastructure Requirements

### Infrastructure Summary  *(the ID index)*

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-001 | compute | C-001 | MUST |
| IP-002 | ci-cd | C-004, NFR-002 | MUST |

---

### IP-001: [Descriptive Title]

**compute · MUST · source:** C-001, NFR-003 — [what must be provisioned, one-to-two lines; WHAT, not HOW].

**Criteria:**
- [Verifiable condition, one line]

**Deps:** IP-002  *(omit if none)*

---
```

### Field Definitions — Constraints

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | C-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | infrastructure / compatibility / regulatory / migration / organizational | On the statement line; exactly one type |
| Source | Yes | Free text | On the statement line; traceable origin — system, regulation, contract, team |
| Severity | Yes | blocking / significant / minor | On the statement line |
| Statement | Yes | One-to-two lines | States the boundary as fact — no separate Description paragraph |
| Impact | Yes | One line, `·`-separated (or a short bullet list) | What this eliminates or forces; references to affected TR-XXX and shaped D-XXX |
| Verify | No | One line | How to confirm the constraint still applies |

### Field Definitions — Decisions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | D-XXX | Sequential, three-digit padded, no gaps — one shared sequence across both origins |
| Origin | Yes | analysis / structural | Which stage authored the row: `analysis` (this skill's author — technology decisions) or `structural` (the architecture seat via `mochiko:patterns-system-design` — topology decisions, grouped in the Structural Decisions subsection) |
| Title | Yes | Free text | Descriptive, concise |
| Context | Yes | One-to-two lines | The problem that needed solving |
| Shaped By | Yes | C-XXX / NFR-XXX references | On the context line; constraints and NFRs that narrowed options |
| Options | Yes | Table, one line per option | Minimum 2 alternatives with pros/cons |
| Choice | Yes | Free text | Selected option |
| Rationale | Yes | ≤ 3 lines | WHY, not just WHAT |
| Consequences | Yes | One line, `·`-separated (or a short bullet list) | Trade-offs accepted, future considerations |
| Governance alignment | No | One line | Only when a project principle applies — omit otherwise |

### Field Definitions — Infrastructure Requirements

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | IP-XXX | Sequential, three-digit padded |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | compute / networking / storage / ci-cd / monitoring / security / environment-config | On the statement line; exactly one |
| Source | Yes | C-XXX / NFR-XXX refs | On the statement line; constraints/NFRs that necessitate this |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119, on the statement line |
| Statement | Yes | One-to-two lines | WHAT to provision, not HOW — no separate Description paragraph |
| Criteria | Yes | Bullet list, one line each | Independently verifiable |
| Deps | No | IP-XXX refs | Other infra items this depends on |

### Infrastructure Types

| Type | Scope |
|------|-------|
| compute | Containers, serverless, VMs, orchestration |
| networking | DNS, load balancers, VPN, firewall rules |
| storage | Databases, object storage, caches (provisioning, not schema) |
| ci-cd | Build pipelines, deployment automation, environments |
| monitoring | APM, logging, alerting, health checks |
| security | IAM, certificates, secrets management |
| environment-config | Environment variables, feature flags, config management |

### Constraint Types

| Type | Definition | Examples |
|------|------------|---------|
| **infrastructure** | Existing systems, platforms, or environments that cannot change | "Production database is PostgreSQL 15 on AWS RDS" |
| **compatibility** | Existing consumers, APIs, or interfaces that must continue working | "Mobile app v2.x expects REST API v1 responses" |
| **regulatory** | Laws, regulations, or compliance mandates | "GDPR requires right-to-erasure within 30 days" |
| **migration** | Deployment, transition, or coexistence requirements | "Zero-downtime migration required; both old and new schemas must coexist" |
| **organizational** | Team, process, or business constraints | "Maximum 3-person team for initial implementation" |

### Distinguishing Constraints from Preferences

| Statement | Constraint? | Why |
|-----------|-------------|-----|
| "Must use PostgreSQL" | Maybe | Only if existing production infrastructure mandates it |
| "Must use React" | Probably not | Unless existing codebase and team skills make alternatives infeasible |
| "Must support IE11" | Yes | If contractual obligation exists |
| "Must deploy to AWS" | Yes | If organization has AWS-only policy |
| "Should use TypeScript" | No | Preference, not hard boundary |

**Test:** "If we violated this, what concrete thing would break or what rule would we violate?" If the answer is vague, it is a preference, not a constraint.

---

## 3. Non-Functional Requirements (nfrs.md)

### Document Template

```markdown
# Non-Functional Requirements: {feature_id}

> Measurable quality attributes with specific targets.

## NFR Summary  *(the ID index)*

| ID | Category | Target | Source |
|----|----------|--------|--------|
| NFR-001 | performance | p95 < 200ms | FR-001 (user expects instant feedback) |
| NFR-002 | availability | 99.9% monthly | Business SLA commitment |

---

## NFR-001: [Descriptive Title]

**performance · source:** [business requirement or stakeholder justifying the target] — [the quality attribute, one line].

**Target:** [specific, measurable numeric threshold]
**Measured:** [tool + conditions + frequency — compact; a multi-condition method uses a short bullet list]
**Applies to:** TR-001 · TR-005  *(omit if it applies globally)*

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | NFR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Category | Yes | performance / availability / scalability / security / usability / maintainability | On the statement line; exactly one category |
| Source | Yes | Free text | On the statement line; business requirement, SLA, or stakeholder justifying the target |
| Requirement | Yes | One line | The quality attribute — the statement IS the description |
| Target | Yes | Numeric | Specific, measurable threshold |
| Measured | Yes | Compact line or short list | Tool, conditions, frequency of measurement |
| Applies to | No | TR-XXX references | Which technical requirements this NFR constrains |

### NFR Categories with Examples

| Category | Bad (Vague) | Good (Measurable) |
|----------|-------------|-------------------|
| **performance** | "System must be fast" | "p95 response time < 200ms under 1000 concurrent users, measured by APM" |
| **availability** | "System must be reliable" | "99.9% uptime measured monthly, excluding scheduled maintenance" |
| **scalability** | "Must handle growth" | "Must support 10,000 concurrent users with linear resource scaling to 50,000" |
| **security** | "Must be secure" | "Zero plaintext PII in logs; all data classified confidential+ encrypted AES-256-equivalent at rest" |
| **usability** | "Must be easy to use" | "New users complete primary workflow within 3 minutes without documentation" |
| **maintainability** | "Must be maintainable" | "Mean time to deploy hotfix < 2 hours from commit to production" |

### Writing Measurement Methods

Every target's `Measured:` line names **what tool**, **under what conditions**, and **how frequently**. Compact example:

```markdown
## NFR-001: API Response Latency

**performance · source:** FR-001 (real-time interaction expectation) — API responses feel instantaneous under production load.

**Target:** p95 < 200ms, p99 < 500ms
**Measured:** APM, rolling 24h windows, continuous — at 1,000 concurrent users (70% read / 20% write / 10% search); excludes maintenance windows and bulk imports
**Applies to:** TR-001 · TR-005
```

---

## ID Numbering Rules (All Artifacts)

All artifact types follow the same numbering conventions:

1. **Three-digit padding:** TR-001, not TR-1
2. **Sequential, no gaps:** TR-001, TR-002, TR-003 (never TR-001, TR-003)
3. **Prefix identifies type:** TR- / C- / D- / NFR- / IP-
4. **Cross-references use full ID:** "See TR-001" not "See requirement 1"
5. **Grouping by concern:** Related items should be sequential where possible
````
- **Kept deliberately:** every line of guidance preserved — the three Document Templates → each schema's `skeleton`; the three Field Definitions tables + Constraint Types + Distinguishing-Constraints-from-Preferences + Infrastructure Types + NFR Categories (bad/good) + Decomposition/Writing-Criteria/Measurement examples + the ID Numbering rules → the schemas' `contract` / `good` / `bad` / `overview`; the artifact-format.md envelope note + `full` register → each schema's `form` / `register` / `overview`. The design-artifact ownership note routes on unchanged: `data-model` → `patterns-entity-modeling` and `quickstart` → `patterns-api-contracts` convert this same wave (own strips); `contracts-api` stays `.md` (I1 — its source `patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml` is already raw-readable YAML). Net-new per-section `check` lines were **authored under D7** (disclosed, not lifted). Nothing dropped.
- **Phase note:** these three schemas are authored at PHASE 1. **P2 phase-2 (gated on the V1 fidelity audit PASSing these schemas — deletion-after-audit, plan §6):** the `ARTIFACT-TEMPLATES.md` file deletion + the `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` additive structure-presence re-key (rich tables kept VERBATIM). **P5 re-point seat (plan §6), not P2:** the six `authoring-technical-requirements/SKILL.md` pointer lines (41/45/55/71/73/79) + `references/TRACEABILITY-PATTERNS.md:35` swap to the two-arm form. Coordination: `.mochiko/strips/authoring-technical-requirements.md` is a shared P2/P5 write surface — this ARTIFACT-TEMPLATES entry is P2's; P5's re-point entry is distinct and the DM sequences the two writers.
- **Consumers assessed:** `authoring-technical-requirements/SKILL.md` (6 pointer lines — re-point phase 2) · `references/TRACEABILITY-PATTERNS.md` (1 line — phase 2) · `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` (Analysis checklists — additive `--check` structure-presence citation phase 2, rich tables kept VERBATIM) · router `skills/mochiko/SKILL.md` (no ARTIFACT-TEMPLATES row — in-skill reference).

## [v0.77.0] Consumer re-points: SKILL.md + `references/TRACEABILITY-PATTERNS.md` `ARTIFACT-TEMPLATES.md` pointers → the 3 schemas (D3 later-ratchet; re-point companion to the file retirement above)
- **Disposition:** superseded → the `requirements` / `constraints-and-decisions` / `nfrs` schemas (`mochiko-cli template <name>`, or Read `plugins/mochiko/schemas/<name>.yaml` raw when the binary is absent). SKILL.md's six `ARTIFACT-TEMPLATES.md` pointers now name the specific schema; the primary "complete field definitions and examples" pointer (§ The Three Analysis Artifacts) carries the full two-arm delivery form for all three, and the five inline asides name the schema (delivery single-sourced at the primary pointer, per the reference-by-ID density doctrine). `references/TRACEABILITY-PATTERNS.md`'s Pattern pointer carries the full two-arm form (standalone reference file).
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim):**

```text
SKILL.md:41  See [ARTIFACT-TEMPLATES.md](references/ARTIFACT-TEMPLATES.md) for complete field definitions and examples.
SKILL.md:45  ... (worked decomposition + field definitions: ARTIFACT-TEMPLATES.md).
SKILL.md:55  **Section 1: Hard Constraints (C-XXX)** and **Section 2: Technology Decisions (D-XXX)** — field schemas in ARTIFACT-TEMPLATES.md.
SKILL.md:71  here. (Template + the `Origin` marker: ARTIFACT-TEMPLATES.md.)
SKILL.md:73  **Section 3: Infrastructure Requirements (IP-XXX)** — field schema in ARTIFACT-TEMPLATES.md.
SKILL.md:79  Define measurable quality attributes. Every NFR has a numeric target. Field schema in ARTIFACT-TEMPLATES.md.
references/TRACEABILITY-PATTERNS.md:35  **Pattern** (the source rides the statement line — see ARTIFACT-TEMPLATES.md):
```
- **Kept deliberately:** every surrounding sentence — only the trailing `ARTIFACT-TEMPLATES.md` pointer token was superseded; the TR / C / D / IP / NFR doctrine, the decomposition rules, and the `Origin`-marker section are unchanged.
- **Consumers assessed:** no external consumers of these SKILL.md / references pointers; the retired `ARTIFACT-TEMPLATES.md` file itself is superseded in the companion entry immediately above (P2 scope — this strip file is a shared write surface, flagged in the P5 report). Full cold re-grep confirms zero remaining `ARTIFACT-TEMPLATES` references in `plugins/`.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 11,255 → 10,628 chars (−6%); description 1,001 → 496
  chars (−50%). Body cut: the **When to Use** section deleted whole (six bullets restating the
  description's invocation conditions — TR-XXX / C-XXX / NFR-XXX / IP-XXX authoring plus the
  INT-XXX and DS-XXX analysis-declaration bullets; each obligation survives in §4 System
  Integrations and §5 Data Sensitivity respectively, and in the three-artifact spine). Description
  cut: the trigger-phrase enumeration trimmed and the trailing "Produces requirements.md,
  constraints-and-decisions.md, and nfrs.md from a business specification" sentence dropped; the
  MUST clause, core triggers, and the constraints-and-decisions.md-ownership +
  `patterns-technical-decisions` sibling distinction kept. Verbatim homes: git history of this
  file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when authoring the technical-requirements layer of a feature specification — decomposing business functional requirements into technical requirements (TR-XXX), documenting hard constraints (C-XXX), defining measurable non-functional requirements (NFR-XXX) with numeric targets, and specifying infrastructure-provisioning requirements (IP-XXX), each traced to a business source. SHOULD also invoke when the work involves "TR-", "C-", "NFR-", "IP-", "technical requirements", "hard constraints", "non-functional requirements", "infrastructure provisioning", or authoring the constraints-and-decisions.md artifact and its C↔D / IP traceability. This skill owns the constraints-and-decisions.md artifact structure (the D-XXX field schema) and traceability — NOT the decision-evaluation technique (use mochiko:patterns-technical-decisions to evaluate alternatives and write ADRs). Produces requirements.md, constraints-and-decisions.md, and nfrs.md from a business specification.
- **Kept deliberately:** the guardrails keep-set — the three-artifact spine (§§1–5 incl. the
  Structural Decisions subsection), the analysis-vs-downstream router blockquote, Traceability
  Rules mandatory links, Technology-Agnostic Writing, the "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, the three no-exceptions
  lines, the Common Rationalizations table, the Quality Checklist, the Red Flags STOP paragraph,
  the letter/spirit epigraph, and all `references/` pointers (ARTIFACT-TEMPLATES,
  TRACEABILITY-PATTERNS).
- **KEPT reconciliation:** the [v0.28.0] and [v0.23.0] kept-sets below survive this cut in full —
  the When-to-Use bullets are not in any prior KEPT/protected set, and no `DECISIONS.md`-traceable
  line was removed. No prior KEPT or protected line is touched.
- **Consumers assessed:** technical-analyst (mounts it) · patterns-entity-modeling,
  patterns-technical-decisions (cross-reference the artifacts / D-XXX schema) · mochiko router.
  None links the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.28.0] Reference-copied field tables, homed mistake rows, and excuse-column red flags stripped (body 229 → 135, −41%, in-band)
- **Disposition:** deduped → `references/ARTIFACT-TEMPLATES.md` (Read: every field appears in
  its Field Definitions with extra Format/Rules columns, plus document templates — and the SKILL
  already declared it the home): all five in-body field tables (TR / C / D / IP / NFR) and the
  sign-in decomposition example (richer 4-TR worked table there) · deleted (Tier 1, in-file
  homes): the §4/§5 canonical-home blockquotes (restated the top analysis-vs-downstream router
  blockquote, which stays; the x-integration field list and four-level taxonomy they enumerated
  live only in `patterns-api-contracts` / `patterns-entity-modeling` — one-line canonical-home
  clauses folded into the §4/§5 declaration paragraphs), the Completeness-check line (its six
  checks restate the kept Quality Checklist), the Red Flags trigger bullets + no-exceptions list
  (the six bullets map ~1:1 onto the Common Rationalizations table's Excuse column — five are
  semantic parallels, none verbatim; bullet 4's excuse shifted, "sensitivity is obvious"
  (authoring shortcut) vs the kept row's "classification is a security team concern"
  (delegation excuse) — its substance is homed at §5 + rationalizations row 4. STOP framing
  kept as one paragraph, the table kept whole as the discipline core, vertical-tdd precedent) · **Common Mistakes deleted whole** (all 6 rows homed: transcribing → decomposition
  rule + rationalizations row 1; unmeasurable NFRs → "'Fast' is not a requirement" + the
  reference's NFR-categories table; never-bounded → §4's kept optimistic-maps paragraph;
  preferences-disguised → the reference's Distinguishing table + violation test; unclassified →
  §5 + rationalizations row 4; orphans → Traceability Rules + checklist) · densified: the
  technology-agnostic Wrong/Right table → rule + one pair (constraints-MAY-name-tech exception
  kept), the D-technique blockquote → 2 lines (the boundary is also in the description,
  When-NOT-to-Use, ARTIFACT-TEMPLATES' Part-2 blockquote, and TRACEABILITY-PATTERNS' note —
  width-only, no line delta)
- **Tier failed:** 1 throughout (every cut had a verified richer home, most in the already-
  declared reference) · n/a for the densifications
- **Content:** five field tables, one example sentence, two blockquotes, one 4-row table, six
  mistake subsections, eleven red-flag/no-exception bullets; nothing written to `templates/` —
  dedups run against pre-existing reference content, D4's destination ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files (technical-analyst, plan,
  patterns-entity-modeling ×2, patterns-technical-decisions ×2, artifact-format, mochiko
  router); none links a section anchor. Kept: the three-artifact spine, Traceability Rules
  mandatory links, INT/DS declaration paragraphs, "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, three no-exceptions
  lines, Common Rationalizations table, Quality Checklist, the letter/spirit epigraph (R4b:
  anchored to the envelope density rules directly above it). Session ruling: wave-4 batch-2
  ratified 2026-07-25.

## [v0.23.0] Description fields collapsed into the statement line across TR/C/IP/NFR blocks (T2, user-ruled)
- **Disposition:** revised per the wave-2 T2 ruling — the separate `**Description:**` paragraph field is deleted from all block templates (`references/ARTIFACT-TEMPLATES.md`) and field-definition tables (SKILL.md + reference); the ID line's statement IS the description
- **Tier failed:** artifact density (epic D4 extension): kinako's requirements.md 61k B / constraints-and-decisions.md 67k B were dominated by per-item field ceremony (label lines + a Description paragraph restating the statement), re-paid ~10× per feature
- **Content:** per-block forms compressed — TR: `**FR-XXX · MUST** — statement` + Criteria bullets + `**Deps:**` line (was Title/Source/Priority/Description/AC-checkboxes/Dependencies-list); C: type·severity·source on the statement line + one-line Impact (was 6 labeled fields); D: one-to-two-line Context + compact options table + ≤3-line Rationale + one-line Consequences (options/choice/ADR substance kept — `patterns-technical-decisions` owns the technique); IP: same collapse; NFR: statement line + Target/Measured/Applies-to lines (was 6 labeled fields + paragraph Requirement + paragraph Measurement Method). Summary tables kept and designated the **ID index** per `templates/artifact-format.md`. `references/TRACEABILITY-PATTERNS.md` pattern examples aligned to the statement-line form.
- **Consumers assessed:** plan producer (technical-analyst) + review-plan-artifacts (ARTIFACT-CHECKLISTS retargeted this wave) + review-feasibility (reads the artifacts; field-agnostic, checked — no edit needed) + downstream tasks/implement readers (consume IDs + statements, unaffected).

## [v0.23.0] Corrections landed in-wave (not strips)
- **Content:** (1) ARTIFACT-TEMPLATES' constraint Severity value set said `Hard` ("all constraints are hard boundaries by definition") while SKILL.md's field table says `blocking / significant / minor` — aligned to the SKILL (blocking/significant/minor), the pre-existing drift resolved toward the skill body; (2) `Constitution Alignment` field renamed `Governance alignment` (post-dissolution vocabulary, the plan-wave Constitution→Governance precedent), now optional-and-omitted when no principle applies.
