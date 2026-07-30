# Brainstorm — Security depth as a Tier-I production-only build

**Status:** accepted (user, 2026-07-30 — "accept") · review complete: pair, lens-split; verify
CLEAN at round 3; clearing verdict ready · opened 2026-07-30, wrapped 2026-07-30.
**Topic as ruled:** PO-D5 Tier I item 1 (record `.mochiko/brainstorms/production-only-focus/record.md`):
"Security depth — threat modeling at plan time, security requirements with teeth, blocking SAST +
dependency-vuln gates, a security lens in the validator set; first-in-line, scoped in its own
follow-on session, not built here."
**Session form:** lead + user in-session (adaptive questioning per `mochiko:analysis-iterative`);
fact-checker seat filled (reality surface: the plugin source's security machinery, post-v0.36.0);
review sizing offered at convergence.

## Baseline shift since the ruling (re-grounded 2026-07-30)

Two waves landed between PO-D5's ruling and this session, both material to its scope:

1. **The architecture primitive (v0.32.0):** `/mochiko:plan` now carries a design-time
   architecture stage — `architecture.md` with a C4-container delta diagram, sequence diagrams
   for qualifying flows, a conditional deployment view keyed to IP-XXX, an early rendered-diagram
   sign-off, and `review-feasibility`'s architecture pass (topology feasibility + governance
   conformance). "Threat modeling at plan time" was ruled when no such stage existed; it now has
   a concrete candidate home.
2. **The PO narrowing build (v0.36.0):** FLOOR-SEC is now a single asserted production row
   (secrets out of repo · CI secret scanning · input validation at boundaries · auth at all
   boundaries · blocking dependency-vuln scanning); the compliance-module registry
   (`COMPLIANCE-MODULES.md`) exists with legal-mandate strata and the audit-evidence seed pool;
   waivers follow D4. The map F21 diagnosis ("one floor card + DS-XXX annotations, no depth
   layer") predates both.

## Problem statement (evolving)

*(seeded from PO-D5; sharpens through questioning)*

The narrowing's rationale was "the narrowing funds the deepening" — security was the first-named
gap ("which we haven't gone deep into"). This session turns PO-D5's four components into a
buildable scope: what each component IS, where it lives (which artifacts, seats, skills, gates),
what is asserted vs arbitrated, and what sequence lands it.

**Q1 ruling (2026-07-30):** of the three candidate framings — (a) secure by construction
(design-time), (b) caught before ship (verification-time), (c) defensible posture (evidence) —
the user ruled **"a and b"** jointly lead; (c) follows rather than leads, consistent with the
thin-seed compliance-module ruling. No incident cited — positioning/design instinct, as with the
PO session's driver. Map alignment: the assertion layer largely exists (F85–F92); the holes are
exactly in (a)'s design surface (F78–F79, F98: zero threat vocabulary, no trust-boundary
convention) and (b)'s enforcement wiring (F93–F94, F97: asserted gates with no modeled carrier,
no endpoint-auth check, input validation consumer-less).

## Fact-checker map — 2026-07-30

*(checker-authored, pasted verbatim)*

# Security Reality Surface — Neutral Map (fact-checker seat, security-depth-scoping)

All paths relative to `plugins/mochiko/`. Verified against working tree at 2026-07-30, post-v0.36.0 (PO narrowing landed; tier ladder retired; `COMPLIANCE-MODULES.md` present). Map, not audit — no recommendations. Facts cut both ways by design.

---

## 1. Security machinery that exists today

### The asserted floor

**F1.** `FLOOR-SEC — Security by Default` (`skills/authoring-constitution/references/catalog/universal-floor.md:23-32`). Type tags: **all**. Layer: **floor-asserted**. Asserted level names five controls: secrets out of the repo (env vars + `.gitignore`) · secret scanning in CI · input validation at boundaries · auth enforced at all boundaries · dependency vulnerability scanning blocking merge. Waiver posture D4, with a stated preference: "prefer narrowing over waiving (e.g. 'no auth — single-user local companion' as a *tightened scope*, not a dropped category)."

**F2.** The floor's level is single and asserted; nothing can lower it; deviation is only ever a recorded waiver, never a loosened card (`universal-floor.md:3-10`, `ESSENTIAL-FLOOR.md:5-11`). Absence is "always deliberate and auditable, never silent."

**F3.** FLOOR-SEC's card carries **no content of its own** — `universal-floor.md:32` points at `ESSENTIAL-FLOOR.md` (Security) for both the category definition and the example principle. The card is one asserted-level line plus pointers.

**F4.** `universal-floor.md:16-19` carries a seed-honesty note: current worked examples are backend/service-flavored (RFC 7807 bodies, `/health`); frontend/mobile/desktop floor examples "ship with their shelves (planned — Tier-I roadmap work)."

**F5.** `ESSENTIAL-FLOOR.md:17-22` — "Security Principle MUST address" is exactly four specifics: **secret management** (env vars OR cloud secret managers — AWS Secrets Manager, Azure Key Vault, HashiCorp Vault), **secret scanning** (CI MUST run — Trivy, Snyk, git-secrets, gitleaks), **config file exclusion** (`*.local.*`, `appsettings.*.json` in `.gitignore`), **input validation** (all external inputs validated before processing). Note the asymmetry with F1: *dependency vulnerability scanning* and *auth at boundaries* are in the card's asserted level but **not** in the Detail Requirements list.

**F6.** The worked example principle `I. Security by Default (NON-NEGOTIABLE)` (`ESSENTIAL-FLOOR.md:49-69`) adds "Authentication MUST be enforced at API boundaries" as a bullet, and names concrete enforcement: `trivy fs --scanners secret .` blocking merge, `snyk test` for dependency vulnerabilities, a code-review checklist item for auth verification, `gitleaks protect` pre-commit. Testability: "Pass: Zero secrets in codebase, zero high/critical vulnerabilities, auth on all endpoints."

**F7.** Security-adjacent obligations live in the other three floor categories, not only FLOOR-SEC: "Stack traces MUST NOT be exposed in production responses" (`ESSENTIAL-FLOOR.md:35`), correlation IDs (`:36`), "Logs MUST NOT contain personally identifiable information" (`:43`), "Logs MUST NOT contain sensitive data (PII, tokens, passwords)" (`:128`).

### Compliance modules (new today)

**F8.** `skills/authoring-constitution/references/COMPLIANCE-MODULES.md` trigger table (`:14-20`) seeds five modules: `hipaa`, `pci-dss`, `gdpr`, `a11y` (WCAG) — all **legal-mandate** — and `attestation` (SOC 2 / ISO 27001 / customer security addenda) — **contractual**. Attachment is "mechanical from the fact profile" (interrogation dimension 2); "No rigor negotiation occurs at attachment — the user rules the *facts*."

**F9.** Strata / waiver posture (`:22-29`): legal-mandate obligations are **unwaivable** (PO-D4.2 — "a recorded permanent waiver of a legal control is documented evidence of a knowing violation"); contractual/non-legal are waivable under D4. Both strata are additive-only over the floor.

**F10.** Seed obligations relocated from the retired `regulated` tier (`:31-48`) — **Security**: audit logging of auth events · documented key-rotation policy · compliance-mapped controls. **Dependencies**: license compliance · documented supply-chain review · vulnerability-blocking severity tightened (high/critical → medium+). **Observability**: log retention policy · access-controlled log storage · audit-grade traceability. **Testing**: coverage ≥90/≥80 + evidence retained for audit. **Error handling**: error-event retention and traceability.

**F11.** Full per-regime obligation sets (HIPAA safeguards, PCI DSS requirements, WCAG level targets) are explicitly **mint-driven** — "authored from real sessions via the catalog's graduation seam, never speculatively" (`:34-37`). The modules are named triggers over a common obligation pool; no regime's control set is authored today.

**F12.** The fact-validation fail-safe (`:50-67`) is four mechanisms: named elicitation (dimension 2 asks industry · data classes · jurisdictions · contractual commitments explicitly), consequence-stated confirmation of each negative ("no health data confirmed — the HIPAA module will not attach"), brownfield cross-check against `codebase-analysis.md` **plus data-model DS-XXX annotations and detected integrations**, and a temporal backstop reopening the fact via an amend run.

**F13.** A fact-profile change is a governance event: full pair review, ledger entry, region semver **MAJOR** bump (`:69-75`); mirrored in `validation-constitution/SKILL.md:148` (MAJOR trigger includes "attaching `hipaa`").

### The DS-XXX taxonomy — how deep, and who consumes it

**F14.** DS-XXX at the analysis layer is a **thin declaration only**: "flag *which* data the feature treats as sensitive … it is **not** the per-attribute classification itself" (`skills/authoring-technical-requirements/SKILL.md:98-100`, boundary restated `:42-46`). There is **no DS-XXX field schema** in `references/ARTIFACT-TEMPLATES.md` (that file templates TR / C / D / IP / NFR only), and the skill's ID-sequence quality check names `TR-001, C-001, D-001, IP-001` — not DS or INT (`SKILL.md:135`).

**F15.** The taxonomy proper lives in `skills/patterns-entity-modeling/SKILL.md:101-143`: four levels (Public / Internal / Confidential / Restricted), a decision tree (`:118-133`), and "PII maps onto these levels — it is not a separate axis" with a classify-up-not-down default (`:116`, `:132`).

**F16.** Depth of handling: `references/DATA-SENSITIVITY.md:23-31` is a 7×4 handling-by-level matrix — encryption at rest, encryption in transit, access control (None / Basic auth / Role-based / Role-based + MFA), audit logging, retention limits, masking in logs, breach notification. Stated **once per document**; per-attribute rows record only specifics + deviations (`:5`, `:33-46`).

**F17.** Per-attribute depth: every Confidential+ attribute carries retention (duration + expiry action), access control (role-based description), named deviations, and a compliance mapping. Compliance examples cover GDPR (Art. 6 / Art. 17), HIPAA, PCI-DSS, NIST 800-63, SOC 2 (`DATA-SENSITIVITY.md:64-72`).

**F18.** **Downstream consumers of the annotations, exhaustively** — (a) `skills/patterns-entity-modeling/scripts/validate-model.py:414-458`, a presence heuristic that explicitly "does NOT judge whether the classification is correct"; (b) `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:143-147` (PII identification **Critical**, Sensitivity details **Critical**, Compliance coverage / Retention / Encryption standards **Important**) and `:214` (Sensitivity-contract alignment, **Critical**); (c) `templates/plan-template.md:45-49` — the Entities table's `Sensitivity` column = "highest classification"; (d) `COMPLIANCE-MODULES.md:62` and `templates/governance-intent-template.md:57` — the brownfield fact cross-check. **Nothing in the implement cluster consumes them**: no mention of sensitivity or classification in `commands/implement.md`, `skills/patterns-vertical-tdd/`, `skills/executing-tdd-cycle/`, or `skills/testing-end-user/`.

**F19.** The Tier-1 pre-assert contradicts the taxonomy. `skills/review-plan-artifacts/scripts/check-artifacts.py:182-227` (`check_pii_markers`) requires a literal `[PII]` marker on the field line, matched against a hardcoded 11-pattern field-name list (`:60-72`) — while the taxonomy forbids exactly that ("PII expressed through classification … not a parallel marker", `DATA-SENSITIVITY.md:81`; `patterns-entity-modeling/SKILL.md:116`). Its `REQUIRED_SECTIONS` for `data-model.md` (`:52-56`) are `## Entities` / `## Relationships` / `## Validation Rules`, but the canonical template's headings are `## Data Sensitivity Summary` / `## Entity Summary` / `## Entity: X` / `## Relationships` / `## State Machines` / `## Validation Rules` — `## Entities` never appears. `review-plan-artifacts/SKILL.md:85-88` calls this script's failed count "ground truth."

### The per-endpoint `x-integration` boundary

**F20.** Security semantics carried: exactly one required field — `auth`: "Mechanism + secret source | How the outbound call authenticates (API key, OAuth client credentials, mTLS) and where the secret comes from" (`skills/patterns-api-contracts/SKILL.md:128`). The other five required fields (`system`, `protocol`, `api_version`, `criticality`, `failure_modes`) are availability/dependency semantics. "auth rejection" appears as one example failure among timeout/5xx/rate-limit/malformed (`:135`). Every failure mode requires a fallback (`:129-148`).

**F21.** `scripts/validate-openapi.py:387-440` checks x-integration well-formedness only — required keys present, criticality in enum, `failure_modes` a non-empty list. "It does **not** judge whether the endpoints, schemas, or failure modes are the *right* ones" (`SKILL.md:194`).

**F22.** Separate from x-integration, the contract layer carries conventional API security surface: `securitySchemes` presence check (`validate-openapi.py:292-314`), a 401-response check that fires only on operations already declaring `security` (`:211-214`), `bearerAuth` + a document-level default `security:` in `references/OPENAPI-TEMPLATE.yaml:500-503, 875-878`, and a 401/403/429 error catalogue including `MFA_REQUIRED` (`references/ERROR-PATTERNS.md:52-59, 81-93, 127-131`). `ERROR-PATTERNS.md:300`: "Stack traces in prod | Security risk | Log internally, return safe message."

### BE-DEP and dependency discipline

**F23.** `BE-DEP — Dependency Discipline` (`catalog/backend-service.md:106-133`) is **arbitrated**, not floor: "the user keeps / tightens / drops." When kept: dependency scanning blocks merge at high/critical. Content: deps justified in PR description, versions pinned in lock files, external calls through ports, transitive deps audited for known vulnerabilities. Red flags include "unpatched known vulnerabilities" and "incompatible license."

**F24.** Direct tension worth naming: FLOOR-SEC asserts dependency-vulnerability scanning blocking merge for **every project, every type** (`universal-floor.md:25-29`), while the same control on the backend shelf is arbitrated and droppable (`backend-service.md:109`). `backend-service.md:110` routes license compliance / supply-chain review / medium+ severity to `COMPLIANCE-MODULES.md`.

**F25.** Stack tooling map (`backend-service.md:142-148`) — Dependency Audit column: `pip-audit` · `npm audit` · `govulncheck` · `cargo audit` · **OWASP Dependency-Check** (Java). This is the plugin's only OWASP mention.

**F26.** Supply-chain-adjacent machinery that already has teeth: BE-HEX's port-interface requirement (`backend-service.md:48-52`), the approved-domain-dependency registry (`:37-46`, `references/DOMAIN-DEPENDENCIES.md`), and — in the implement loop — a non-empty `domain_deps_added` **always** forcing the escalated human checkpoint, never auto-approved (`commands/implement.md:68, 128-130`).

### Everything else the grep sweep surfaced

**F27.** `skills/analysis-codebase/SKILL.md:83-89` — the brownfield Security status assessment is three checks: auth at boundaries (middleware patterns `authenticate`/`authorize`/`requireAuth`), secrets from env (`.env.example` exists, no hardcoded credentials), input validation (schema validation libraries). Assessment is "intent-blind and waiver-blind by design" (`:75-81`). Mirrored as three rows in `templates/codebase-analysis-template.md:105-107` + a category rollup at `:118`.

**F28.** `skills/analysis-codebase/scripts/detect-stack.sh` contains **zero** security/secret/audit/vuln detection (grep returns nothing). It is the deterministic baseline at setup G1 (`commands/setup.md:48`) and says nothing about security tooling.

**F29.** `skills/analysis-codebase/references/CONTEXT-GATHERING.md:157` — "**Security scanning**: [Yes/No]" under Team Signals, a free-text field with no detection recipe, in contrast to `:62-70` which supplies concrete greps for test commands and coverage thresholds.

**F30.** `references/EMERGENT-CEILING-PATTERNS.md:20` lists **Authorization** as a codifiable pattern category ("All endpoints MUST validate user permissions") but ships **no worked example principle** for it (its Example Principles are code quality, error format, layer discipline, dependency discipline, product analytics, naming). `:149` "Security scanner blocks merge on known vulnerabilities" appears inside the dependency-discipline example's Enforcement.

**F31.** `skills/validation-constitution/references/ANTI-PATTERNS.md:15` — one security anti-pattern row: "Missing secret management | 'Secrets from env' only | Specify secret managers, scanning tools, .gitignore rules." The "Missing Enforcement" worked example (`:39-56`) uses a Security principle.

**F32.** `references/QUALITY-CHECKLIST.md:75` — "Security tools are named (e.g., 'Trivy + Snyk', NOT '[SECURITY_COMMAND]')." The only named-security-tool bar in the validator. `:35` carries the floor-accounting row.

**F33.** NFR category enum includes `security` (`authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:299`), with one worked row (`:313`): bad "Must be secure" → good "Zero plaintext PII in logs; all data classified confidential+ encrypted AES-256-equivalent at rest."

**F34.** `ARTIFACT-TEMPLATES.md:237` — IP-XXX infrastructure type `security` = "IAM, certificates, secrets management." `:246` — constraint type `regulatory` = "Laws, regulations, or compliance mandates," example "GDPR requires right-to-erasure within 30 days."

**F35.** `skills/patterns-technical-decisions/SKILL.md:40` — Security is one of eight weighted evaluation criteria; `references/EVALUATION-MATRIX.md:14` "Security posture and track record? Known vulnerabilities? Security features?"; `:76-82` an Authentication technology-comparison table.

**F36.** `skills/authoring-requirements/references/EDGE-CASES.md` category 2 (`:40`) lists "Injection attempts (SQL, XSS, command)" with examples `<script>` tags and `'; DROP TABLE` (`:46-47`); category 5 (`:106-129`) is "Permission and Access Boundaries" — expired tokens, role change mid-session, cross-tenant access attempts, API key rotated mid-process. `:154` names "security breach" as a Critical-impact class.

**F37.** `skills/patterns-vertical-tdd/references/SLICE-IDENTIFICATION.md:35` lists "Authentication/authorization" and `:44` "Environment configuration and secrets management" as foundation-cycle infrastructure; `SKILL.md:106` names auth among foundation infra ("Could ANY user story work **in production** without this?").

**F38.** RFC-2119 guidance names security twice: `authoring-constitution/references/RFC-2119-KEYWORDS.md:17` (MUST is for "Security requirements") and `authoring-requirements/references/RFC-2119-KEYWORDS.md:155-163` — a named anti-pattern, "Under-specifying Security: Security requirements marked as SHOULD when they need MUST."

**F39.** `authoring-constitution/SKILL.md:155` — the enforcement-strength table's Code Review row reads "Architecture compliance, **security review** | Strong—explicit checklist item." One of only two places security review is positioned as a process (the other is F40).

**F40.** `authoring-technical-requirements/SKILL.md:148` — the rationalization row: "'Data classification is a security team concern' | Every technical requirement that touches data needs classification before design. **Security reviews supplement, not replace.**"

**F41.** Persona-level security prose (motivation only, no procedure): `agents/requirements-analyst.md:80, 106`; `agents/technical-analyst.md:74, 88, 92, 113, 133, 134, 138`; `agents/devils-advocate.md:51`; `agents/principal-architect.md:120` ("Every project constitution should address four essential categories — **Security, Testing, Error Handling, Observability** … NON-NEGOTIABLE baseline requirements").

---

## 2. Plan-time design surfaces a threat-modeling stage could ride

**F42.** `architecture.md`'s four pieces (`skills/patterns-system-design/SKILL.md:38-73`, template `:97-138`): (1) container delta diagram — **the sign-off surface**; (2) sequence diagrams for qualifying flows; (3) container-level component table + delta summary; (4) conditional deployment view.

**F43.** The qualifying-flow trigger rule (`:53-59`): "any flow that **crosses ≥2 components and has non-trivial ordering or failure semantics** — a user journey *or* a system flow (async settlement, retry, webhook re-entry, saga). P1 user journeys are the **floor, never the cap**." The trigger is ordering/failure-keyed; it contains no security, trust, or abuse criterion.

**F44.** Component-table fields (`:61-67`): `name — responsibility — boundary — status (new/modified/existing)`. "Boundary" is used in the ownership sense throughout — the worked rows read "owns Profile store", "reads queue, writes blob store" (`:120-121`). Not a trust boundary (see F98).

**F45.** Diagram conventions (`:43-51`): `subgraph` blocks for boundaries, technology named in every node label, arrows labelled **protocol + purpose** (`HTTPS / fetch profile`, never a bare line), delta styled via `classDef`. Protocol labelling is the closest existing thing to a channel-security annotation; nothing requires an arrow to state authn/authz or the classification of data crossing it.

**F46.** Deployment view is conditional on `IP-XXX` rows existing (`:69-73`); no IP-XXX → omitted with a one-line record. `IP-XXX` type `security` exists (F34), so an IAM/certificate/secrets provisioning row is the existing trigger for a deployment view.

**F47.** Scope bound: the delta neighborhood — changed components + direct collaborators, default ~12 rendered nodes, wider system linked past that (`:75-81`). The no-delta form still presents a reseeded diagram + a one-line "changes nothing structurally" claim (`:83-89`).

**F48.** `review-feasibility`'s **architecture pass** (`skills/review-feasibility/SKILL.md:45-75`): group A **topology feasibility** (NFR↔topology, constraint↔topology); group B **governance conformance** (layer rules honored, dependency allowlist respected, GI-linked principles actually satisfied — "verified, not asserted"). Routing (`:69-75`): a topology that must break a governance surface is "**never awarded `feasible` silently**" — exactly two exits, redesign to conform or a user-ruled amendment/waiver through `governance-ledger.md`; "the feature-level review never overrules the constitution."

**F49.** The six base feasibility classes (`:36-43`) are constraint / NFR / decision / requirement-shaped. None is security-shaped. A security concern reaches this reviewer only if it was first authored as an NFR-XXX or C-XXX (then it rides class 2, 5, or 6).

**F50.** `review-feasibility`'s G1 guardrail (`:28`, `:135`): it "operates over plan analysis/design artifacts, **never the constitution**." The governance-conformance lens reads the governance surface strictly **as an input** to grade the topology's conformance.

**F51.** `authoring-technical-requirements` requirement classes (`SKILL.md:50-100`): TR-XXX, C-XXX, D-XXX (incl. the architecture-owned Structural Decisions subsection, `:70-78`), IP-XXX, NFR-XXX, plus thin INT-XXX and DS-XXX declarations. The **only security-shaped affordances** are the NFR `security` category (F33), constraint type `regulatory` (F34), IP type `security` (F34), and DS-XXX (F14). There is no SEC-XXX / threat / abuse-case / misuse-case class.

**F52.** `review-plan-artifacts` security-shaped checks, complete list (`references/ARTIFACT-CHECKLISTS.md`): data-model — PII identification **Critical** (`:143`), Sensitivity details **Critical** (`:144`), Compliance coverage / Retention policies / Encryption standards **Important** (`:145-147`); contracts — Integration boundary presence **Critical** (`:161`), Failure-mode presence **Critical** (`:162`), "Authentication | Are auth requirements clear? | **Important**" (`:163`); quickstart — Auth documentation **Important** (`:181`); cross-artifact — "Sensitivity-contract alignment | Do API responses respect data classification (no Restricted data in responses)? | **Critical**" (`:214`); key questions `:189-190`.

**F53.** `commands/plan.md` gates post-merge are **G1–G7** (`:50-89`): G1 entry · G2 baseline (bootstrap only) · **G3 architecture sign-off (always-on, rendered diagram, user-ruled)** · G4 feasibility/governance rejection · G5 clarification · G6 exit-early/escalation · G7 package acceptance. Bounds: cap 3 produce↔review rounds **per stage** across five stages (`:90-93`). Ordering invariant: "the architecture is the **first** artifact of the design work" (`:94`). No security stage, seat, or gate exists.

**F54.** `plan.md` Bindings artifact set (`:116-127`): requirements · constraints-and-decisions · nfrs · architecture · data-model (entities + sensitivity) · contracts/api.yaml (OpenAPI + x-integration) · quickstart (conditional) · task-mapping · tasks · plan.md. Five report files (`:128-130`). No security artifact.

---

## 3. Gate machinery that exists for "blocking gates"

**F55.** `commands/implement.md`'s gate set (`:51-100`): G1 entry · package gate · governance surface · **cycle checkpoint** (carries the devolved branch) · **architecture deviation** · G3 clarification · G4 exit-early/escalation · G5 final acceptance.

**F56.** What actually blocks: qa's per-cycle verification plus the whole-implementation final validation, on "real-infrastructure evidence and quality-gate exit codes" (`:22-28`); the Not-done list includes "a failing quality gate" (`:30-33`).

**F57.** Where the gate list comes from: `skills/testing-end-user/SKILL.md:139` — "Identify quality gate commands from the `## Quality Gates` section of `tasks.md` and/or the build configuration in `plan.md`." Execution semantics (`:133-159`): gates **always auto-resolve**, exit 0 = pass, non-zero = fail, **no human checkpoint** — "This exit-code determinism is ground truth; it MUST NOT be softened into an LLM judgment call."

**F58.** Neither template supplies that section. `templates/tasks-template.md` headings: Overview · Cycle Format · Markers · Foundation Cycles · Feature Cycles · Story→Cycle Mapping — **no `## Quality Gates`**. `templates/plan-template.md` headings: Summary · Architecture · Key Decisions · Infrastructure Requirements · Entities · Endpoints · Artifacts · Next Steps — **no build configuration and no gates section**. The pattern appears only as a parse target in `skills/executing-tdd-cycle/references/TASK-PARSING.md:84-95` (worked example: `pnpm lint` / `pnpm build` / `pnpm test`).

**F59.** FLOOR-SEC's named enforcement is **not wired into implement's loop**. Grep for secret-scanning / dependency-audit vocabulary returns nothing in `commands/implement.md`, `skills/testing-end-user/`, `skills/executing-tdd-cycle/`, or `agents/qa-engineer.md`. Its only transmission path is the governance region's quality-gates summary — whose template (`templates/governance-surfaces-template.md:48-51`) models exactly two lines: "`[actual command]` MUST pass before merge" and a coverage threshold. No secret-scan or dependency-audit line is modelled.

**F60.** `implement.md:58-62` governance surface: region present → each code-touching brief carries a one-line **obligated read** naming the `.claude/rules/mochiko/` files relevant to that cycle's file paths. Region absent is surfaced, **never a blocking gate**.

**F61.** The cycle checkpoint's **devolved branch** (`implement.md:63-70`) skips the lead's read *exactly* when every verification in the cycle is a deterministic CLI check at 100% pass **and** no architecture deviation is reported **and** `domain_deps_added` is empty. Any failure, any GUI/subjective verification, any deviation, any registry addition fires the human checkpoint. A non-empty `domain_deps_added` "**always** forces the escalated human checkpoint — never auto-approved, no stamp read" (`:128-130`).

**F62.** The architecture-deviation gate (`implement.md:71-75`): a diagram-anchored self-check run at cycle **open and close** — "does this cycle add or remove a box, add, remove or redirect an arrow, or move a responsibility across a boundary on the approved diagram?" A yes stops the cycle and is presented to the user; exits are build-as-approved or a consented amendment.

**F63.** The real-infrastructure rule (`skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md:9-16`): verification MUST use real file systems, real databases, real APIs — **not mocks** — with tangible output, explicit steps, observable outcome. Assert vocabulary is exactly three patterns — `Console contains "…"` (and its `(within Ns)` form), `File exists: {path}`, `Response status: {code}` (`:57-64`); anything else is "a **custom assertion for human evaluation**" (`testing-end-user/SKILL.md:78`).

**F64.** Runtime classification CLI / GUI / SUBJECTIVE (`testing-end-user/SKILL.md:96-104`) — default SUBJECTIVE when uncertain; any failure on any classification forces a checkpoint. "No default to PASS — an unevaluated assert is a failure" (`:80`).

**F65.** Neither `agents/qa-engineer.md` nor `skills/testing-end-user/SKILL.md` carries any security verification. qa's declared products are verification reports, quality-gate results, checkpoint presentations, evidence artifacts (`qa-engineer.md:60-63`); `skills: testing-end-user` only (`:35`). No security vocabulary appears in either file.

**F66.** The real-commands elicitation: interrogation **dimension 6** "Existing practices & tools — Detected stack, CI, linters, tests; brownfield analysis feeds in here → **The real commands the validator requires**" (`references/INTERROGATION-AGENDA.md:33`) and **dimension 8** "Deployment & release reality → Quality Gates; Observability/Error-Handling expression; the `release-gates` module offer — **always interrogated**" (`:35`). Dimension 6's named elicitation list is "stack, CI, linters, tests" — scanners are not named.

**F67.** The validator's placeholder bar (`skills/validation-constitution/SKILL.md:130-142`, Step 7): FAIL on `[PLACEHOLDER]`, `[COMMAND]`, `[THRESHOLD]`, `[TOOL]`, `GI-XXX`, and **any** `[BRACKETED_TEXT]`, across every member of the surface set. "**No exceptions.**" Reinforced by the named-security-tools row at `QUALITY-CHECKLIST.md:75` (F32).

**F68.** The validator's floor/module/waiver checks (`validation-constitution/SKILL.md:112-124`): every Essential Floor category has a principle **or a recorded waiver**, in any mode; every waiver carries standard + justification + revisit-trigger-or-"permanent (D4.1 pending)" + trace; "**A waiver naming a legal-mandate module obligation is a FAIL**"; attached modules match the fact profile one-for-one; "attached content that loosens a floor principle is a FAIL."

**F69.** The `release-gates` module (`templates/constitution-modules/release-gates.md`) supplies a release-blocking gate table (gate · requirement · verified-by · blocks) with a rollback section, and a validator fragment requiring "Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)" (`:39`). Attached only on module selection; its example gates are staging soak / migration check / changelog — none security.

---

## 4. Review/validator seats a security lens could brief onto

**F70.** `agents/devils-advocate.md:25` carries six review skills: `review-specifications`, `review-plan-artifacts`, `review-task-artifacts`, `review-brainstorm`, `review-slices`, `review-governance-intent`. Its persona names security once, as motivation only ("Found security holes that 'obvious' requirements missed", `:51`). Its hunt taxonomy is deliberately not in the persona — it single-sources to `review-specifications/SKILL.md:64-69`, five requirement-defect classes: missing requirements / ambiguities / edge cases / assumption gaps / contradictions. No security class.

**F71.** `agents/principal-architect.md` is the seat for `review-feasibility` (`plan.md:39`), the setup producer (`setup.md:37`), the arch-diff seat and the arch-scribe seat (`implement.md:41-42`). `:120` states the four essential categories including Security are NON-NEGOTIABLE baseline requirements — the persona's only security content.

**F72.** `templates/sized-end-stage-review.md` is a **conditional read**, loaded only where P6 binds it (`templates/command-shape.md:142-144`). Today only two commands bind it: `commands/brainstorm.md:38` (decision-quality / record-integrity pair) and `commands/setup.md:41-44, 76-82` (coverage / coherence pair, event-scaled on amend). `specify` (`:36-38`), `slice` (`:41-43`), `plan` (`:44-46`) and `implement` (`:44-48`) all run **in-loop critique** instead — specify and slice say "unsized by design," implement says the same of qa's verification.

**F73.** How lenses are briefed: `sized-end-stage-review.md:14-16` — "Each spawns at convergence (**P5's rows carry agent × `review-*` skill × lens brief**), reads the frozen artifact cold, forms findings independently, and reports findings-formed — count only — before its counterpart is introduced." The lens is a **spawn-brief string**, not a file. Its semantics are stated inside each review skill: `review-brainstorm/SKILL.md:12` and `review-governance-intent/SKILL.md:20-26` — "**The lens sets your depth, not your jurisdiction**: work your lens hard, and still report anything real you trip over outside it — the lead owns the cross-set merge. Solo, the whole surface is yours." `skills/mochiko/SKILL.md:134` notes spawn prompts must name "skill + role + lens, since teammates ignore `skills:` frontmatter."

**F74.** Structural cost of a lens differs by site: at the two sized-review sites a lens is a brief string plus a hunt-class list in the review skill; at the four in-loop sites there is **no lens slot** — the review skill's checklist *is* the lens, and the seat is selected by artifact.

**F75.** Security-shaped checks in review checklists today, complete: `review-plan-artifacts` only (F52). `review-specifications` carries none — its gap categories are user expectations / business rules / scope boundaries / success-failure states / **permissions** ("Who can do X? Who cannot?", `SKILL.md:52-58`), and it explicitly rules technical concerns out of scope (`:39-40`). `review-task-artifacts` carries none — its lens is task quality: vertical-slice integrity, TDD ordering, `**TEST:**` presence, traceability (`SKILL.md:28-31`). `review-slices` carries none. `review-brainstorm` carries none. `review-governance-intent` carries none security-specific (its five hunt classes are agenda-coverage, tier/fact calls, passive card acceptances, too-easily-resolved reality conflicts, thin-rationale echoes).

**F76.** `templates/agent-dispatch.md:11-20` — eight brief fields; field 5 "**What good looks like** — the bar the output must clear this run" is the existing carrier for a per-run lens. Its one hard line (`:28-34`) is independence: the grading agent must be a different agent running a different skill; "No agent is ever asked to grade its own output."

---

## 5. Absences (grep-confirmed, stated precisely)

**F77.** **No `skills/security-*` directory.** `skills/` holds 30 directories; none is security-named. **No security agent** — `agents/` holds exactly 10 files: command-architect, devils-advocate, principal-architect, qa-engineer, requirements-analyst, staff-engineer, system-architect, task-architect, technical-analyst, validator.

**F78.** **"threat" occurs zero times** anywhere in `plugins/mochiko/` (case-insensitive, all file types). No threat model, threat modeling, threat actor, threat surface.

**F79.** Also **zero occurrences**: `STRIDE`, `SAST`, `DAST`, `pentest` / "penetration test", `CSRF`, `CVE`, "trust boundary", "attack surface", "least privilege", "abuse case", "hardening", `CSP`, "secure storage", "code signing", `SBOM`.

**F80.** **`OWASP` occurs exactly once**: `catalog/backend-service.md:148`, the Java row's "OWASP Dependency-Check" in the **Dependency Audit** column. No OWASP Top 10, ASVS, or Cheat Sheet reference exists.

**F81.** **`XSS` occurs exactly once**: `skills/authoring-requirements/references/EDGE-CASES.md:40`, inside the "Invalid or Malformed Input" pattern list ("Injection attempts (SQL, XSS, command)"), with two illustrative examples at `:46-47`. It appears in a reference file's discovery prompts — not in that file's documentation template (`:166-180`), not in `spec-template.md`, and not in any review checklist.

**F82.** **No SAST/DAST tool is named anywhere.** The only scanner names in the plugin are secret scanners — Trivy, Snyk, git-secrets, gitleaks (`ESSENTIAL-FLOOR.md:20, 60-63`) — and dependency auditors — pip-audit, npm audit, cargo audit, govulncheck, OWASP Dependency-Check, Snyk (`backend-service.md:124-125, 148`; `ESSENTIAL-FLOOR.md:61`).

**F83.** **No artifact template has a security section.** Headings verified for: `spec-template.md` (Overview · User Stories · Edge Cases · Functional Requirements · Key Entities · Success Criteria · Assumptions · Open Questions), `plan-template.md` (F54), `tasks-template.md` (F58), `slices-template.md`, `artifact-format.md`, `governance-surfaces-template.md` (region: Governance · Principles · Technology stack · Quality gates · Governance operations; ledger: Waivers · Amendment policy · Exception registry · Principles · Amendment log), and the four `constitution-modules/`. The one exception is not a section: `codebase-analysis-template.md:105-107` carries three **Security rows** inside the Essential-Floor status table.

**F84.** **The specify layer carries no security prompt.** `templates/spec-template.md` has no security field; `skills/authoring-requirements/SKILL.md` has none; `commands/specify.md` has no security-shaped gate or evidence and its G1 governance check is presence-only (`:42-48`). `review-specifications/SKILL.md:39-40` actively pushes technical concerns downstream: "Implementation details (databases, APIs, protocols), technical edge cases, architecture decisions, and performance targets are valid concerns — they belong in later design work, not spec review." The only security-shaped material at this layer is `EDGE-CASES.md`'s two categories (F36) — reference material the producer may reach, graded by nothing.

---

## 6. Both-ways facts

### Already exists unconditionally — a security build would not add it

**F85.** Dependency-vulnerability scanning **blocking merge**, secret scanning in CI, secrets out of the repo, input validation at boundaries, and auth at all boundaries are already asserted on **every project, every type**, with no tier and no negotiation (`universal-floor.md:25-29`). Concrete enforcement commands and pass/fail criteria are already written (`ESSENTIAL-FLOOR.md:59-67`).

**F86.** Every attribute of every data model already carries a sensitivity classification (`patterns-entity-modeling/SKILL.md:107`, checklist `:289`); every Confidential+ attribute already carries retention, access control, named deviations and a compliance mapping (`:290`); this is graded **Critical twice** by an independent reviewer (`ARTIFACT-CHECKLISTS.md:143-144`) and pre-asserted by a script (`validate-model.py:414-458`).

**F87.** Every endpoint wrapping an external system already documents its outbound auth mechanism **and secret source** (`patterns-api-contracts/SKILL.md:128`), plus every realistic failure mode with a mandatory fallback (`:129-148`), Critical-graded twice (`ARTIFACT-CHECKLISTS.md:161-162`) and format-checked by a script (`validate-openapi.py:387-440`).

**F88.** The floor is already unwaivable-by-silence: absence is a ledger-recorded waiver with justification, revisit trigger and trace, FAIL-checked (`validation-constitution/SKILL.md:116-118`), and a legal-mandate module obligation cannot be waived at all (`:118`; `COMPLIANCE-MODULES.md:23-25`).

**F89.** The compliance-attachment **mechanism** is fully built: mechanical trigger table, two strata, additive-only rule, a four-part fail-safe against a wrong module-driving fact, and a MAJOR-bump amend path (`COMPLIANCE-MODULES.md:12-75`). What is missing is regime **content**, and the file says so (`:34-37`).

**F90.** A governance-violating topology already has a named, non-silent escalation with exactly two exits (`review-feasibility/SKILL.md:69-75`; `plan.md` G4 `:73-77`), and `infeasible` is preserved as a distinct business-level state that must not be flattened (`review-feasibility/SKILL.md:114-124`).

**F91.** Real-infrastructure verification with captured evidence and deterministic exit-code gating already runs on **every cycle** and again on the whole implementation (`implement.md:40, 84-89`; `testing-end-user/SKILL.md:133-159`). Any security check expressible as a CLI command with an exit code already has a runner and a blocking path.

**F92.** Independent grading is already structural, not aspirational: no seat row grades its own output (`agent-dispatch.md:28-34`; `command-shape.md:188`), and `validation-command-shape` greps for it. A security reviewer would inherit that guarantee rather than establish it.

### Thinner than the vocabulary suggests

**F93.** FLOOR-SEC names secret scanning and dependency-vuln blocking as asserted, merge-blocking controls — but the governance region's Quality-gates template models neither (`governance-surfaces-template.md:48-51`), and nothing downstream names either (F59). The asserted control's entire enforcement carrier is whatever a setup session happens to write into a two-line summary.

**F94.** The quality-gate list qa executes has a named consumer and **no authored producer**: `testing-end-user/SKILL.md:139` reads `## Quality Gates` from `tasks.md` / build config from `plan.md`, and neither template contains either (F58).

**F95.** DS-XXX has a name, trigger phrases in two skill `description`s, and a brownfield cross-check consumer — but **no field schema**, no ID-sequence check (`authoring-technical-requirements/SKILL.md:135` covers TR/C/D/IP only), and no reviewer check that a DS-XXX exists or that a Confidential+ attribute closes back onto one. `review-plan-artifacts` grades the data-model **annotations**, never DS↔attribute closure. The traceability rule is opt-in by construction: "cite DS-XXX **where realized**" (`DATA-SENSITIVITY.md:19, 76`).

**F96.** `ARTIFACT-CHECKLISTS.md:214` — "Sensitivity-contract alignment: no Restricted data in responses, **Critical**" — is the plugin's only cross-artifact security-shaped check, and it has **no producer-side counterpart**: `patterns-api-contracts`' schema section maps data-model *types* to OpenAPI types and never mentions classification (`SKILL.md:51-70`), and its quality checklist (`:200-210`) has no classification row.

**F97.** "Auth enforced at all boundaries" is asserted (`universal-floor.md:28`) but nothing checks that a given endpoint *is* authenticated: `ARTIFACT-CHECKLISTS.md:163` asks "Are auth requirements clear?" at **Important**, and `validate-openapi.py:211-214` fires its 401 check only on operations that already declare `security` — it never asks whether an operation should. Symmetrically, "input validation at boundaries" (`universal-floor.md:28`; `ESSENTIAL-FLOOR.md:22`) has **no downstream consumer at all** — no plan checklist row, no task check, no verification pattern; `patterns-entity-modeling/references/VALIDATION-RULES.md` is data-model constraint documentation, not boundary validation.

**F98.** The architecture surface uses "boundary" throughout — a component-table column, `subgraph` blocks, the deviation gate's "move a responsibility across a boundary" — always in the ownership/deployment sense. "Trust boundary" appears nowhere (F79), no convention marks where authentication happens or where untrusted input enters, and the qualifying-flow trigger is ordering/failure-keyed (F43).

**F99.** One of four type shelves is seeded. `catalog/README.md:22-28`: universal-floor and backend-service **seeded**; frontend, mobile, desktop **"planned — Tier-I roadmap work."** Every browser-, mobile-, or desktop-specific security expression (CSP/XSS handling, keychain/secure storage, code signing, update integrity) has no shelf to live on today, and `universal-floor.md:16-19` says so in the open.

**F100.** BE-DEP's arbitrated status (F23) means a session can legitimately **drop** the card carrying `pip-audit`/`npm audit`/`govulncheck` enforcement while FLOOR-SEC still asserts "dependency vulnerability scanning blocking merge" — and BE-DEP is dealt only to backend/service/fullstack-api types (`backend-service.md:108`), so a declared frontend, mobile or desktop project is never dealt the card that carries the tooling for a floor control asserted on it.

## Decisions

### D1 — Structural form: woven into existing seats; the craft lands in new model-invoked skills — `Confident`
**Statement:** No security-engineer agent. Threat modeling rides `system-architect` × a new
model-invoked skill on the architecture stage; verification wiring rides existing machinery (the
gate-list producer fix F94, the exit-code runner F91); review pressure enters via lens briefs and
checklist extensions on existing reviewers (F73–F75), never a new seat.
**Rationale:** the repo's thesis — discipline lives in skill quality; the existing seats already
own every carrier surface (architecture.md, requirements, verification, review); a security seat
would have no producer artifact of its own.
**Rejected road (recorded with steelman):** a dedicated security seat. *Steelman:* the
architecture session minted `system-architect` on exactly the distinct-craft-deserves-a-persona
argument, and a cold specialist catches what generalists-with-a-lens miss — a lens is
jurisdictionally weaker than a seat. *Rejected because:* that precedent earned its seat by owning
a producer artifact (`architecture.md`); a security seat would be a pure reviewer, which is what
lens briefs exist for. **Hybrid revisit stands:** if dogfoods surface a genuine dedicated
security artifact, the seat question reopens.
**Provenance note:** lead-recommended, user-adopted without elaboration ("go with your
recommendations") — streak watch opened. **S16 fold (review, user-ruled):** D1 explicitly
confirmed at disposition — the user answered a plain-terms U5 restatement of both structural
calls with "confirmed"; mark retained `Confident` citing that direct affirmation. The streak
closed at D3's reversal (engagement evidenced by a user ruling against recommendation).
**S10 fold (review):** D1's "lens briefs" language corrected — F74 puts no lens slot at the
in-loop sites where all three D6 sites land; the available mechanisms are **checklist
extensions + persona-judgment edits**. The build surface gains the persona edits (a
security-judgment line in `system-architect` for the sweep, and in `devils-advocate` for the
abuse hunts), each re-checked against the decoupling keystone test. Checklists are procedure;
the judgment must live in the persona per axis 4.

### D2 — Threat modeling = trust-boundary annotation + a security-keyed flow sweep inside `architecture.md` — `Confident`
**Statement:** No new artifact. Three mechanisms: (1) a **trust-boundary convention** on the C4
delta diagram — where untrusted input enters, which arrows cross trust lines, authn/authz points
annotated on arrows (closes F98's ownership-only "boundary" sense); (2) the **qualifying-flow
trigger gains a security key** — "crosses a trust boundary / carries Confidential+ data"
alongside ordering/failure (F43) — so security-relevant flows get sequence diagrams and an abuse
sweep; (3) threats found land as **requirements rows** (form ruled at Q4), never a separate
document. DS-XXX gains its missing consumer (F95): the sweep reads the data-model
classifications to judge which crossings matter. Surfaces at G3 — the user rules the topology
and its trust picture in the same act.
**Rejected roads (with steelmans):** dedicated `threat-model.md` (*steelman:* a named artifact
cannot be skipped silently, auditor-pointable; *rejected:* restates the annotated diagram, adds
a stage to a five-stage loop, and (c)-posture is not the lead driver) · requirements-only
(*rejected:* leaves F98 standing — threats without a topology view is the blindness the
architecture primitive exists to fix).
**Provenance note:** lead-recommended, user-adopted without elaboration — second consecutive;
streak at 2. **S16 fold (review, user-ruled):** D2 explicitly confirmed at disposition — the user's direct
"confirmed" on the plain-terms U5 restatement; mark retained `Confident` citing it.
**S1 fold, edge 2 (review, user-ruled — U1):** the sweep runs **before `data-model.md` exists**
(architecture precedes detailed design), so "carries Confidential+ data" leaves the trigger —
the security key becomes **"crosses a trust boundary / carries declared-sensitive data (the
DS-XXX thin declarations + spec-level sensitivity)"**, coarse by design. When the downstream
`data-model.md` contradicts the ruled trust picture (e.g. Restricted data crossing an
unannotated boundary), **plan's existing detailed-design-contradiction backward path to G3
fires** — no new machinery, no second sign-off act. D2's DS-consumer claim is re-grounded:
the sweep consumes the *declarations*; the classification detail feeds the backward check.
**S17 fold (review):** the question trail, reconstructing every "ruled at QN" reference:
Q1 driver → Q2 = D1 → Q3 = D2 → Q4 = D3 (`Contested`, the streak-breaking reversal) →
Q5 = D4 → Q6 = D5 → Q7 = D6.

### D3 — SEC-XXX minted: a dedicated security-requirement class, full plumbing at birth — `Contested` (user-ruled over the lead's reuse recommendation)
**Statement:** Threats and security obligations land as **SEC-XXX** rows — threat → control →
verification method — a first-class requirement class in the analysis layer. **Full plumbing is
mandatory at birth** (the F95 lesson: a half-minted class rots): field schema in
`ARTIFACT-TEMPLATES.md` · ID-sequence check in `authoring-technical-requirements` ·
`review-plan-artifacts` coverage + **closure** rows (every trust-crossing boundary/flow on the
diagram has ≥1 SEC-XXX or a recorded none-needed — the teeth) · feasibility routing (SEC rows
ride `review-feasibility`'s constraint-shaped classes) · traceability (each SEC row cites the
boundary/flow that motivated it, and DS-XXX where data-keyed).
**Basis for `Contested`:** the lead recommended reusing C-XXX/NFR-XXX with a security-owned
subsection (namespace growth; the Structural-Decisions precedent; the closure check works either
way); the user chose the mint with that against-case in view. The steelman that carried:
greppable coverage, "requirements with teeth" as the ruled phrase, and a fully-minted class
cannot rot the way F95's half-minted DS-XXX did.
**Riders (independent of the mint choice, kept in scope):** DS-XXX's plumbing holes close in the
same wave (F95: field schema, ID-sequence check, DS↔attribute closure) · F96's producer-side
counterpart lands (`patterns-api-contracts` states response classification; checklist row gains
its producer).
**S1 fold, edge 1 (review, user-ruled — U1):** SEC-XXX authoring is **two-phase**: rows seed
coarse at the analysis stage (from the spec's abuse cases + DS declarations), and the sweep
**completes them at the architecture stage** — an architect-owned SEC subsection of the
designated home (S6 below), mirroring the structural-decisions precedent. The closure check
runs at the architecture stage's review, where the diagram exists.
**S6 fold (review):** the named artifact home — the plumbing organ D3's own list omitted —
is **`constraints-and-decisions.md`**, which gains a designated SEC-XXX section beside its
structural-decisions section; `plan.md`'s Bindings class enumeration updates with it (one
command edit — the build surface's zero-command-edits claim corrects accordingly).
**S11 fold (review):** the SEC field schema **prefers CLI-expressible verification methods**
(the three TEST-grammar patterns); the residue rule is stated, never silent: a SEC row whose
verification is genuinely GUI/subjective de-devolves its cycle to the human checkpoint —
accepted checkpoint inflation, on the record.

### D4 — SAST joins FLOOR-SEC's asserted row — `Confident`
**Statement:** Every project runs a **blocking SAST scan**, same standing as secret scanning and
dependency-vuln blocking. The stack tooling map gains a SAST column; interrogation dimension 6's
elicitation names scanners (closes F66); the governance quality-gates template models the line
(closes F93 for it). The D4 recorded waiver is the valve for a genuinely noisy stack — visible
and justified, never a silent drop.
**Rejected road (with steelman):** arbitrated default-in. *Steelman:* SAST false-positive rates
vary by stack and codebase age; an asserted-but-noisy gate teaches gate-bypassing. *Rejected
because:* that is the F100 shape (a floor control whose tooling lives on a droppable card), and
the waiver route handles the noisy case with a record.
**Provenance note:** lead-recommended, user-adopted — with a user-added direction ruled at D5.
**S3 fold (review, user-ruled — U3):** the bar and the brownfield policy: SAST blocks at
**high/critical** (parity with dependency scanning) · brownfield onboarding runs
**new-findings-only diff scanning**, pre-existing findings recorded as a known-debt baseline
register *(home marked at the verify tidy: the register is carried in the **synthesis** as
V2's gate-strictness override — the validator's legal home; the original "governance ledger"
is superseded)* · the ruleset/config lives in-repo and changes via review
like code — the in-loop tuning path a setup-time waiver cannot provide. **V2 repair (verify
pass, user-ratified):** the brownfield baseline **splits by what a finding hits** — generic
SAST findings enter the debt register, carried as a **recorded session override in the
synthesis** (the validator's existing gate-strictness clause: its legal home); findings
matching **floor-line rulesets** (the hand-rolled auth/crypto patterns carrying D5.3) never
enter the baseline — they are **brownfield floor confrontations**, resolved in the open as
MUST-implement gaps or recorded waivers. A floor-asserted line admits no third deviation
category. **R2 repair (verify round 2, extending V2's ratified logic one level down):** the
**floor-line ruleset subset is governance-protected** — a change to it is a governance event
(ledger entry, the same standing as a waiver), because it is the enforcement carrier of a
floor-asserted line; generic-ruleset tuning stays in-repo and review-light, exactly as S3
intends. A narrowed floor ruleset must leave a governance record, or the floor line stops
being enforced with no event at all.
**S7 fold (review):** the "closes F100" claim corrects — relocating tool *names* closes only
F100's type-gate clause. The **enforcement clause and threshold** ("blocks merge at
high/critical", "scanner in CI") relocate to FLOOR-SEC's universal home with the tooling map;
BE-DEP's version becomes a tightening over the floor, never the carrier.

### D5 — OSS-leveraged security, organized by area coverage — `Confident`
**Statement (user-shaped):** three confirmed expressions plus the user's organizing principle:
1. **Enforcement rides OSS scanners** — gates carried by named open-source defaults per stack
   (`semgrep` SAST · `trivy`/`gitleaks` secrets · `osv-scanner`/ecosystem auditors for deps);
   **paid products are legitimate substitutes, but the named default is open source.** Mochiko
   names and wires tools, never reimplements scanning.
2. **Methodology grounded in OSS frameworks** — STRIDE is the sweep's per-boundary question
   vocabulary; **OWASP ASVS / Top 10** seed the SEC-XXX vocabulary and the review hunt list —
   established frameworks, never a homegrown taxonomy (honest to the graduation-seam
   philosophy: seed from established sources).
3. **No hand-rolled security** — a new FLOOR-SEC asserted line: security-critical functionality
   (auth, crypto, session management, input sanitization) MUST use **established libraries or
   platform facilities, never hand-rolled** *(statement amended at verify R1, per S2/U2:
   "open-source" widened — platform-native and vendor facilities are legitimate; the carrier is
   the semgrep rulesets riding the SAST gate, with the domain-dependency registry gating
   domain-layer security libraries only, as a tightening where it exists — the original
   "(F26)" registry-gate routing is withdrawn)*.
**The organizing principle (user's words):** "recognise what areas of security needs to be
addressed in a production app … It is important to have areas covered." → the build authors a
single-sourced **security area registry** (ASVS-chapter-derived: authn · session · access
control · input validation · crypto · data protection · communications · config · logging/error
· business logic · API · files), each area mapping to its pipeline expression (floor line ·
SEC-XXX prompt · gate/tool · verification · lens hunt). **Coverage is the check:** every area is
addressed or recorded as a **known-gap** for this app *(statement amended at verify V3 — the
original "explicitly recorded N/A" vocabulary superseded by S9/U6's known-gap ruling)* —
absence deliberate and auditable, mirroring the floor's own rule. Consumed by the sweep, the
SEC-XXX author, the review lens, and setup's floor expression.
**Provenance note:** lead-interpreted from the user's direction, confirmed "all three as you
recommended" with the coverage principle added in the user's own words.
**S2 fold (review, user-ruled — U2):** D5.3's carrier corrected — the domain-dependency
registry cannot reach adapter/infrastructure code (layer-scoped, I/O-excluding filter,
arbitrated, type-gated, conditionally existent; the F26 citation had dropped the word
"domain"). The carrier becomes **semgrep rulesets riding the asserted SAST gate**
(hand-rolled-crypto/auth patterns — reaches all code, all layers, all types) plus the per-area
SEC prompt; the registry still gates domain-layer security libraries where it exists, as a
tightening. **"Open-source" reads as *established, never hand-rolled*:** platform-native and
vendor security facilities (iOS Keychain, DPAPI, AWS KMS) are legitimate — the line bans
hand-rolling, not platforms.
**S4 fold (review, user-ruled — U4):** the access-control area carries a **data-model-keyed
obligation**: every entity carrying an owner/tenant attribute yields an ownership-check
SEC-XXX (object-level authorization) — closing the intra-zone blindness of a purely
topology-keyed sweep; the cross-tenant-read failure class gains a checkable producer
obligation and closure. **V1 repair (verify pass, user-ratified):** the obligation **fires at
the detailed-design review** — where entity attributes exist and `review-plan-artifacts`
already grades the data-model set — as a second, narrow closure check (every
owner/tenant-attributed entity → an ownership-check SEC row or a recorded none-needed); the
architecture-stage closure (S1) covers boundary/flow rows only. Ownership rows land in the
same SEC section via the later-stage backflow the structural-decisions precedent already uses;
no new machinery. **R3 (verify round 2):** the seat is named — the **technical-analyst**
authors ownership rows in the SEC section (it owns the entity attributes that trigger them);
the architect's subsection stays scoped to boundary/flow rows. The two closure checks are
seat-aligned: the architecture-stage closure grades the architect's rows, the detailed-design
closure the analyst's.
**S8 fold (review):** the area registry gains the **supply-chain / malicious-code area**
(ASVS V10 — mandatory given D5.3 widens exactly that surface); the ASVS version is **pinned
at build time to the current stable release**; the registry's maintainer is the library
itself (graduation-seam owner), re-derivation triggered by an ASVS major release.
**S9 fold (review; the N/A half user-ruled — U6):** coverage's home and scope: the
**app-level coverage ledger lives in the governance ledger** (setup-owned;
`validation-constitution` gains the check), graded **once per app** and updated at amend;
per-feature `review-plan-artifacts` rows check only the areas the feature touches. **A
shelf-pending area is a recorded known-gap with a Tier-I pointer — never "N/A", never
silently "addressed"** — the ledger stays honest for the three unseeded product types.
**S13 fold (review):** D5's rejected roads, recorded: registry-vs-SEC-closure-alone (rejected
— closure without a completeness denominator cannot say what is missing) ·
ASVS-vs-Top-10/CWE seed (rejected — the Top 10 is a risk ranking, not a coverage taxonomy) ·
commercial-default tooling (rejected — adoption friction; paid substitutes stay legitimate
per D5.1).
**S15 fold (review):** the config/crypto areas carry a **rotatability obligation** — secrets
rotatable and tokens revocable without redeploy is a design-time SEC prompt, not ops-fenced;
key-rotation *execution* stays with ops-hardening.
**S20 fold (review):** every blocking gate is **demonstrated failing once** — a known-bad
canary at setup or first implement (a gate that cannot fail is not a gate); lands as a build
item.

### D6 — The security lens lands at three sites, never a new seat — `Confident`
**Statement:** (1) **Plan reviewers, primary:** `review-plan-artifacts` gains SEC coverage +
closure rows (D3) and the D5 area-coverage row *(statement amended at verify V3: scoped to the
areas the feature touches — the app-level grade lives in the governance ledger per S9; the
original every-area/"recorded N/A" text superseded by S9/U6)*;
`review-feasibility`'s architecture pass gains the trust picture — an unauthenticated arrow
crossing a trust line is a conformance finding routed like any governance conflict (F48
machinery unchanged — withdrawn at S5's fold below). (2) **Spec layer, minimal and business-language:** `review-specifications`'
existing *permissions* gap category sharpens to abuse-shaped cases (cross-tenant, role-boundary,
"who must NOT") — its jurisdiction already, cheapest catch-point; the F84 downstream-push
otherwise stands. (3) **Implement:** `patterns-vertical-tdd` maps each SEC-XXX verification
method into `**TEST:**` tasks (F63's grammar already carries `Response status:`);
`testing-end-user` gains the gate-list wiring (F94) with the security gates in it; qa executes —
exit-code determinism untouched.
**Provenance note:** lead-recommended (with the site-2-drop alternative stated); user ruled
"all three".
**S5 fold (review):** "F48 machinery unchanged" is **withdrawn** — that pass is citation-keyed
and cannot catch an uncited floor violation. `review-feasibility`'s group B gains a
**floor-keyed bullet**: every trust-crossing arrow on the diagram is graded against FLOOR-SEC
(authn/authz stated) whether or not the architecture cites it.
**S14 fold (review):** the spec-layer asymmetry closes on the producer side —
`authoring-requirements` / `spec-template` gain a business-language abuse-case prompt for
stories touching sensitive data (the F96 lesson applied at this layer);
`review-specifications`' sharpened permissions category grades what a producer was actually
asked to author.
**S12 fold (review):** review economics, stated: the sweep rides the existing architecture
rounds · app-level area coverage is once-per-app (S9), never per-run · the V1 ownership
closure adds a detailed-design review row — and a second, later path to the feasibility
re-fire (R4) · the per-stage cap-3 stands, with the feasibility re-fire on SEC-expanded scope
accepted as the existing `plan.md` path; if dogfoods show routine G6 escalations, the cap
ruling reopens.

### Scope fences (out of this build, explicitly)

DAST / pentest workflows (zero footprint today, F79; later candidate once SAST is live) ·
per-regime compliance content (mint-driven per the PO ruling; the area registry doesn't change
that) · type-specific area expressions (CSP/XSS, keychain, code-signing — ride the Tier-I shelf
builds, F99; the registry marks them as shelf content) · runtime security ops (WAF, incident
response, key-rotation execution — the ops-hardening item's territory).

### Build surface implied by D1–D6 (consequences, not new decisions)

One new skill (`patterns-threat-modeling`-shaped: STRIDE sweep + trust-boundary conventions +
the **security area registry** reference, consumed by `system-architect`) ·
`patterns-system-design`'s DIAGRAM-CONVENTIONS gains trust-boundary marks ·
`authoring-technical-requirements` gains SEC-XXX with full plumbing (+ the DS-XXX plumbing
close, F95) · `patterns-api-contracts` gains response-classification + endpoint-auth-declared
checks (F96/F97) · FLOOR-SEC's asserted row gains the SAST + no-hand-rolled-security lines,
with the stack tooling map **and the enforcement clause + threshold** relocating to a
universal home (closing both F100 clauses — the "closes F100" shorthand corrected per S7) ·
dimension 6 elicits
scanners (F66) · the governance quality-gates template models the security gates (F93) ·
`tasks-template` gains the `## Quality Gates` producer (F94) · checklist/lens extensions per D6
· the F19 defect fix (the `[PII]`-marker script contradiction + heading mismatch) folds in as
the build's prerequisite stage. **Review corrections to this surface (S6, S10, S18, S20):**
the skill is named **`patterns-threat-modeling`**, its registry at
`skills/patterns-threat-modeling/references/SECURITY-AREAS.md` · **persona edits are in
scope** — a security-judgment line in `system-architect` (the sweep's adversarial instinct)
and in `devils-advocate` (the abuse hunts), each re-checked against the decoupling keystone
test · the gate-canary build item (every blocking gate demonstrated failing once) · and the
zero-command-edits claim corrects to **one command edit**: `plan.md`'s Bindings class
enumeration gains SEC-XXX (under the goal-shape v5 audit regime). **V4 extension — every fold
with a build consequence, named:** S2 the semgrep hand-rolled-auth/crypto rulesets authored as
D5.3's carrier (riding the SAST gate) · S3 the SAST bar (high/critical) + brownfield
new-findings-only diff scan with the V2 split (generic findings → synthesis-recorded
gate-strictness override; floor-line hits → brownfield confrontations) + in-repo ruleset
config · S4 the ownership-check obligation with its detailed-design closure row (V1) · S5
`review-feasibility` group B's floor-keyed bullet · S8 the supply-chain area + ASVS version
pin + maintainer and re-derivation trigger · S9 the governance-ledger coverage section +
`validation-constitution`'s coverage check · S14 the spec-layer producer abuse-case prompt ·
S15 the rotatability prompt in the config/crypto areas. Everything else rides skills,
references, and templates (D1's payoff); the build plan confirms or corrects.

---

## Review

**Sizing gate (2026-07-30):** weight stated — 6 decisions (D1–D6); mix 5 `Confident` /
1 `Contested` (D3, user-ruled over the lead's recommendation); a 100-fact embedded map; one
user-added organizing principle (D5); scope fences + consequence list presented. Default keying:
heavyweight → pair. **User ruling: pair.** Lens split: decision-quality vs record-integrity;
both briefs name the embedded map as the fact substrate; verify pass owned by the
record-integrity reviewer. The record is frozen from reviewer spawn until dispositions land.
*(S16 correction: the "5 Confident" tally overstated D1/D2 at sizing time — both were then
unconfirmed recommendation-adoptions; both were explicitly confirmed at disposition.)*

**Cold reads + cross-exam (2026-07-30):** dq-reviewer 16 raised (4C/8I/4M) · ri-reviewer 13
raised (2C/7I/4M) · one-shot four-message cross-exam; **29 raised → 25 survived** (1 dq
withdrawal absorbed, 3 ri withdrawals absorbed; zero unresolved cross-objections) → **20
lead-merged survivors** (5 Critical / 12 Important / 3 Minor). ri-reviewer's sample audit of
the embedded map: **CLEAN** — 23 of 100 entries verified against the cited files; sole
immaterial imprecision F97 (errata below). Both reviewers recommended **critical-gaps** on the
broken-load-bearing-claims bar: D2's sweep input (absent at its own gate), D5.3's carrier
(cannot reach its subject), D6's "F48 machinery unchanged", and the "closes F100" claim.

**Map errata (S19):** F97's wording — `validate-openapi.py`'s 401 check falls back to the
document-level `security` block (per F22), not only per-operation declarations. F97's
conclusion (nothing asks whether an operation *should* be authenticated) is unchanged.

**Dispositions (2026-07-30) — 20/20 landed.** User ruling batch: U1–U4 + U6 adopted as
recommended ("go with your recommendation"); U5 answered with an explicit **"confirmed"**
after a plain-terms restatement of both structural calls (D1 + D2 stand, marks retained).
Lead-formulated folds landed in the decision bodies and build surface above. *(V5 honesty
note: U1–U4 + U6 — the four Critical repairs among them — were batch adoptions of the lead's
recommended folds, the same form S16 was raised about; the record claims no more deliberation
for them than D1/D2's original adoptions carried. The V1/V2 verify repairs were likewise
batch-ratified after individual presentation.)*

| # | Sev | Disposition |
|---|-----|-------------|
| S1 | Crit | user-ruled (U1) — two-phase SEC authoring + coarse sweep key + the existing backward path; both edges closed by one ruling (D2 + D3 folds) |
| S2 | Crit | user-ruled (U2) — semgrep-on-SAST carrier · registry scope corrected · "established, never hand-rolled" (D5 fold) |
| S3 | Crit | user-ruled (U3) — high/critical bar · brownfield diff-scan + debt register · in-repo config (D4 fold) |
| S4 | Crit | user-ruled (U4) — data-model-keyed ownership-check obligation (D5 fold) |
| S5 | Crit | resolved — floor-keyed conformance bullet; "unchanged" withdrawn (D6 fold) |
| S6 | Imp | resolved — home = `constraints-and-decisions.md` SEC section; +1 command edit (D3 fold, build surface) |
| S7 | Imp | resolved — enforcement clause + threshold relocate with the tooling map (D4 fold) |
| S8 | Imp | resolved — V10 area added · version pinned at build · maintainer + re-derivation trigger (D5 fold) |
| S9 | Imp | half user-ruled (U6: known-gap, never N/A) · half resolved (ledger home, once-per-app) (D5 fold) |
| S10 | Imp | resolved — persona edits in scope; D1's "lens briefs" language corrected (D1 fold, build surface) |
| S11 | Imp | resolved — CLI-preferred verification + stated de-devolve residue (D3 fold) |
| S12 | Imp | resolved — economics stated; cap stands with a reopen condition (D6 fold) |
| S13 | Imp | resolved — D5's rejected roads recorded (D5 fold) |
| S14 | Imp | resolved — producer-side abuse prompt added (D6 fold) |
| S15 | Imp | resolved — rotatability obligation in config/crypto areas (D5 fold) |
| S16 | Imp | user-ruled (U5) — D1/D2 explicitly confirmed; streak closed at D3; sizing tally corrected |
| S17 | Imp | resolved — the question trail recorded (D2 fold) |
| S18 | Min | resolved — `patterns-threat-modeling` + `SECURITY-AREAS.md` named (build surface) |
| S19 | Min | resolved — map errata above |
| S20 | Min | resolved — gate-canary build item (D5 fold, build surface) |

**Verify pass (ri-reviewer, owner per sizing gate) — round 1: NOT CLEAN.** 20/20 folds
confirmed landed with quoted evidence; four blocking + one recommended repairs, all in the
fold-introduced-contradiction class: **V1** S4's ownership obligation keyed to entity
attributes that exist only at detailed design — after S1's architecture-stage closure
(re-opened the ordering defect on a third edge) · **V2** S3's brownfield debt register
composed with S2's carrier exempted hand-rolled crypto in old code — a floor-lowering third
deviation category the floor rule forecloses · **V3** D5/D6 statement text still carried the
superseded "recorded N/A" / every-area scope · **V4** the build surface still claimed "closes
F100" against S7 and omitted eight folds' build consequences · **V5 (recommended)** the
disposition batch applied a stricter engagement standard to the challenged decisions than to
the rulings that closed the challenges.

**Repairs penned (V1/V2 user-ratified — batch, per V5's own standard):** V1 — the ownership
check fires at the detailed-design review as a second narrow closure (D5 fold amended) · V2 —
the baseline splits: generic findings → synthesis-recorded gate-strictness override,
floor-line hits → brownfield confrontations (D4 fold amended) · V3 — both statements amended
in the S1/S5/S7 marking pattern · V4 — the F100 line corrected + the correction block extended
to all eight folds · V5 — the honesty note added to the dispositions paragraph.
**Re-verify round 2: NOT CLEAN — 2 blocking + 2 recommended, all narrow.** All five round-1
repairs confirmed landed, none regressed; the seam checks cleared (V1×S11 fits natively —
ownership verification is `Response status: 403`; V1×S12 incomplete not contradictory).
**R1** D5.3's statement still carried the withdrawn registry carrier + "open-source" wording +
the erroneous F26 cite (the one V3-class instance round 1 missed — verifier self-flagged) ·
**R2** the in-repo ruleset config was a fourth deviation door: narrowing a floor-line ruleset
would silence the floor with no governance event · **R3** ownership rows had no named author ·
**R4** S12's economics didn't count V1's added review row.
**Repairs penned:** R1 — D5.3 statement amended in the marking pattern · R2 — the floor-line
ruleset subset is governance-protected (a change is a ledger-recorded governance event;
generic tuning stays light — V2's ratified logic one level down) · R3 — technical-analyst
authors ownership rows, seat-aligned closures · R4 — the economics enumeration extended.
**Re-verify round 3: CLEAN.** All four round-2 repairs landed, no earlier repair regressed, no
new contradictions; R3 checked against the independence rule (analyst + architect produce, the
review seat grades — no self-grading) and the native ownership pattern. One recommended tidy
applied (the debt register's home marked — synthesis, per V2; the "governance ledger" wording
superseded); one cosmetic instance (D2's statement "Confidential+") left self-resolving per
the verifier. Verifier's status recommendation: **ready** — all three broken load-bearing
claims repaired at the source. Carried forward (verifier's closing note): the pattern behind
every round-2/3 item was *folds landing correctly in their own block and colliding with
unamended text elsewhere* — apply the superseded-text marking at fold time, not review time.

**Lead's clearing verdict: ready.** Combined tally: 29 raised → 25 survived cross-exam → 20
lead-merged survivors → 20/20 dispositioned (6 user-ruled, 14 lead-formulated) + 5 verify
repairs (V1–V5, two user-ratified) + 4 round-2 repairs (R1–R4) + 1 tidy — verify CLEAN at
round 3. Awaiting the user's acceptance.
