# Brainstorm — Mochiko for production-quality codebases only

**Status:** accepted (user, 2026-07-30 — "i accept") · review complete: pair, verify pass CLEAN, clearing verdict ready · opened 2026-07-30, wrapped 2026-07-30.
**Topic as given:** "i want to brainstorm turning mochiko only for production quality codebase. I think this will sharpen the focus of the workflow too."
**Session:** `/mochiko:brainstorm` (command shape v3) · lead + user; fact-checker seat filled (reality surface: the mochiko library itself — thesis/positioning docs and every tier/project-type-keyed mechanism in the plugin source). Reviewers at convergence per the sizing gate.

---

## Problem statement (evolving)

Driver, in the user's words: the any-project-type approach has kept the workflow from being *more opinionated for production use cases*. No single incident cited (Q1) — this is a positioning/design instinct, not a reaction to one failed run.

Sharpened at Q2: the hedge lives in the **standards-enforcement layer** — governance, coding standards, and security "always have to hedge for the kind of project." Security is explicitly named as a gap ("which we haven't gone deep into"). Target named: **production projects that ship to customers**. And the move is not purely subtractive — the user names depth the library currently lacks and a production-only mochiko should gain: **security, multi-repo approach, IaC**. Working reframe: the narrowing funds the deepening — cut the tier axis so the standards layer can assert instead of elicit, and reinvest in production-specific depth.

---

## Fact-checker map — 2026-07-30

*(checker-authored, pasted verbatim)*

Neutral map of the tier/flexibility reality surface for the production-only narrowing. Every claim file-grounded (`path:line`); facts cut both ways. All paths under `plugins/mochiko/` unless noted. This is a map, not an audit.

### 1. Current positioning — what mochiko says it is for

**F1.** ROADMAP's thesis positions mochiko as the kernel-free HIL successor: "engineering discipline lives in the quality of the skill library — native agent teams and Workflows orchestrate, and the human is the framework's primary external validator" (`ROADMAP.md:5-9`). There is **no** statement of a target project type or a single quality tier. The opposite: `ROADMAP.md` "Later (non-committed)" explicitly names *breadth* as the direction — "Frontend catalog shelf (stage 2) · mobile/app shelf" (`:29`), "Multi-stack / monorepo domain registries" (`:34`), "Seat-tiering brainstorm" (`:35`).

**F2.** `CLAUDE.md` frames the product as "engineering discipline lives in the quality of the skill library, not in a deterministic kernel" and "Skills and agents are the primary building block." Its only "axes" are the five *skill-library* conventions (classification, discoverability, etc.) — unrelated to project tier. **Neither operating doc asserts a project-type or tier positioning**; the flexibility axis lives entirely inside the plugin source (the governance/setup layer), not in the product's stated identity.

### 2. The tier/type axis as built

**F3.** The tier ladder's actual levels are a fixed, ordered strictness spine: **`poc | internal | production | regulated`** (`skills/authoring-constitution/references/catalog/README.md:39-44`). As written: `poc` = "Throwaway or proof-of-concept; lifespan measured in weeks" (floor at minimum, any category MAY be waived); `internal` = "Internal tool with real users but bounded blast radius" (moderate strictness, waivable with justification); `production` = "Real users, real data, real cost of failure" (full strictness, "Floor categories MUST NOT be waived"); `regulated` = "Compliance obligations on top of production stakes" (full strictness + audit-evidence variants, no waivers). Labels are declared "soft … may be renamed by usage; the structure … is the decision" (`README.md:36-37`).

**F4.** Project **type** is a *separate* axis from tier. Interrogation dimension 3 elicits "frontend / backend / fullstack / CLI / library / service" and feeds "Shelf selection" (`references/INTERROGATION-AGENDA.md:23`). Type selects catalog shelves; tier "filters and parameterizes the cards" (`catalog/README.md:16-30`). Shelf inventory: universal-floor (all types, seeded), backend-service (seeded), **frontend (`frontend.md` — "planned — next authoring pass", not built)**, **CLI and library ("empty by decision — mint-driven")** (`README.md:22-30`). So the ruled boundary's deferred types (library, CLI) map onto shelves that are **already empty**, and the frontend shelf is unbuilt.

**F5.** Tier is elicited at dimension 2 and drives "Floor strictness, waiver defaults, deck filtering — and the pruning license" (`INTERROGATION-AGENDA.md:22`). The "low-tier pruning license" lets whole agenda dimensions be skipped at `poc`/`internal` — "A poc session may legitimately be five questions long" (`INTERROGATION-AGENDA.md:56-61`).

**F6.** The Essential Floor is four categories — Security / Testing / Error-Handling / Observability — that every constitution "MUST account for … with a principle, or, at a low tier, a recorded waiver" (`references/ESSENTIAL-FLOOR.md:5-11`). The **floor concept is invariant** ("no session emits a floor-less constitution"); only **strictness and waiver posture are tier-parameterized**. At `production`/`regulated` the four are NON-NEGOTIABLE and MUST NOT be waived; at `poc`/`internal` a category MAY be waived, recorded (`ESSENTIAL-FLOOR.md:8-11`).

**F7.** Every floor card carries an explicit per-tier strictness ladder. FLOOR-SEC: `poc` = secrets out of repo only; `internal` = + secret scanning + input validation; `production` = + auth enforced at all boundaries + blocking dependency-vuln scanning; `regulated` = + audit logging, key-rotation, compliance mapping (`catalog/universal-floor.md:21-33`). FLOOR-TEST coverage: `poc` no threshold → `internal` ≥60% warn → `production` ≥80% warn/≥60% block + ratchet → `regulated` ≥90%/≥80% + audit retention (`universal-floor.md:39-53`). FLOOR-ERR and FLOOR-OBS ladder identically (`universal-floor.md:57-89`); FLOOR-OBS is "not dealt by default" at `poc`.

**F8.** Backend-service coding-standards cards are tier-keyed the same way: BE-HEX (hexagonal architecture), BE-SRP (single-responsibility + complexity metrics), BE-DEP (dependency discipline) each default `poc: out · internal: offer · production: default-in · regulated: default-in`, with enforcement strength keyed to tier — e.g. BE-HEX "at `internal`, enforcement MAY be code-review-only; at `production`+, import-linter rules in CI" (`catalog/backend-service.md:17-18, 70-71, 107-108`).

**F9.** The Emergent Ceiling is the brownfield companion: "beyond the essential floor, identify existing good patterns worth codifying from the codebase" (`references/EMERGENT-CEILING-PATTERNS.md:1-3`) — a discovery mechanism, not tier-keyed.

**F10.** Templates encode the tier axis structurally: `governance-intent-template.md` has a mandatory "Tier declaration" (GI-001 `poc|internal|production|regulated` + graduation path + rationale, `:45-50`), a deck-ruling table with a "tightened/loosened … or 'at tier preset'" column (`:78`), and a waivers table gated "Only at tiers whose waiver posture permits it" (`:95-99`). `governance-surfaces-template.md` stamps tier into the ratified region (`:33`) and the Governance Tier section (`:96-108`). Constitution modules `evolution-notes`, `layer-rules`, `release-gates` all reference tier gates.

### 3. What is already unconditional (production-only would NOT change)

**F11.** TDD red/green/refactor and **real-infrastructure verification are tier-blind**. `patterns-vertical-tdd`, `executing-tdd-cycle`, and `testing-end-user` carry no tier keying; every cycle ends in a mandatory real-infrastructure `**TEST:**` gate: "Use real file systems, real databases, real APIs—NOT mocks" (`skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md:13`); "The final task of every cycle is a real-infrastructure `**TEST:**` verification task … gates cycle completion" (`templates/tasks-template.md:28`); `testing-end-user` "verifies against real infrastructure, never mocks" (SKILL description).

**F12.** Producer↔validator pairing is a structural universal, not tier-keyed — skill-library axis 5 in `CLAUDE.md`: "every reviewable artifact is graded by a structurally independent validator (different agent, different skill)." Applies to every workflow regardless of type/tier.

**F13.** Quality gates (lint/build/test as exit-code checks) run on every implement cycle and the final-validation run, tier-blind (`commands/implement.md:185`).

**F14.** The **entire design→build→verify pipeline is already tier-blind**. Among the seven commands, only `setup.md` and `implement.md` reference governance tier at all; `specify.md`, `slice.md`, `tasks.md`, `brainstorm.md` contain zero governance-tier mentions (grep-confirmed). `plan.md`'s single "project type" read is a brownfield lookup from `governance-intent.md` (`commands/plan.md:103`), not a tier branch. So a production-only narrowing would leave the pipeline's rigor essentially untouched.

### 4. Where flexibility softens opinion (elicits/defers rather than asserts)

**F15.** Governance is elicitation-first *by charter*: "The interrogation leads, the deck follows. No catalog card is dealt until the dimensions that select and filter the deck (tier, type) are elicited" (`INTERROGATION-AGENDA.md:10-13`). Cards are dealt "recommend-then-arbitrate" — the user "keeps / tightens / loosens / drops / re-ranks each" (`:81-86`; `setup.md:126-131`). This is the central place the workflow *elicits* a standard rather than *asserting* it.

**F16.** Security is softened specifically by tier: FLOOR-SEC is "waivable at `poc`/`internal`", and auth-at-all-boundaries + blocking dependency-vuln scanning only enter at the `production` row (`universal-floor.md:24-33`). A production-only stance could assert the `production` security row as the fixed floor.

**F17.** Coding-standards depth is hedged below production: BE-HEX/BE-SRP/BE-DEP are `offer` at `internal` and only `default-in` at `production`+, with enforcement downgraded to code-review-only at `internal` (`backend-service.md:17-18, 70-71`).

**F18.** Setup's synthesis-review **sizing gate keys on tier**: "`poc`/`internal` → single, `production`/`regulated` → pair" (`commands/setup.md:138-139`). Under production-only this fork collapses to a fixed pair.

**F19.** implement.md's escalated-branch checkpoint **keys on tier**: "at `production`/`regulated` tier … a domain-registry addition forces the human checkpoint; at lower tiers surface the additions … non-blocking" (`commands/implement.md:133-134`; mirrored at `skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md:48` and the tier-keyed add-gate at `references/DOMAIN-DEPENDENCIES.md:64`). Under production-only this becomes always-blocking.

**F20.** The low-tier pruning license defers deployment and other dimensions at `poc`/`internal` (`INTERROGATION-AGENDA.md:56-61`; example skip "dimension 8 (deployment) skipped: poc tier" at `templates/governance-intent-template.md:62`). Production-only removes the license — deployment reality would always be interrogated.

### 5. The named gaps as built today

**F21 (security — present but shallow).** Security exists as: the FLOOR-SEC card + the ESSENTIAL-FLOOR security detail (secret management, CI secret scanning, config exclusion, input validation — `ESSENTIAL-FLOOR.md:17-23,49-69`), plus a per-attribute 4-level data-sensitivity taxonomy (Public/Internal/Confidential/Restricted, DS-XXX) owned by `patterns-entity-modeling` and applied to every data model unconditionally. There is **no** dedicated security skill or agent (no `skills/security-*` dir), **no** threat modeling, and **no** SAST/DAST/pentest workflow — grep for threat-model/SAST/DAST/OWASP/pentest returns only "OWASP Dependency-Check" named as one Java dep-audit tool (`backend-service.md:146`) and two passing mentions of "security review" as a code-review checklist item. Security is one tier-parameterized floor card + data annotations, not a depth layer.

**F22 (multi-repo — effectively silent).** No multi-repo or cross-repo orchestration/governance exists. The only traces: `detect-stack.sh` emits a `"monorepo"` pattern label (`:325-327`), and `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md:66` has one "Monorepo" path-convention row. Multi-repo is named only as a non-committed future item — "Multi-stack / monorepo domain registries" (`ROADMAP.md:34`).

**F23 (IaC / deployment — requirements-level only).** Deployment is present as *requirements*, not as authored infrastructure-as-code. IP-XXX infrastructure-provisioning requirements (compute, networking, CI/CD, monitoring) are authored by `authoring-technical-requirements` (`SKILL.md:26,121`; `references/ARTIFACT-TEMPLATES.md:208-221`); the `release-gates` constitution module fires "when the deployment dimension elicited a real release process" (`templates/constitution-modules/release-gates.md:4-6`); a "deployment cycle" exists in task artifacts but is marked **"dormant/parked"** (`skills/review-task-artifacts/references/PHASE-CHECKLISTS.md:75`, `ISSUE-TEMPLATES.md:55,146`). There is **no** IaC artifact generation — no terraform/pulumi/k8s/helm authoring anywhere (grep finds only Dockerfile *detection* in `detect-stack.sh` and a stopword list in `validate-requirements.py`). Deployment = provisioning requirements, not deployed-infrastructure code.

### 6. Scale of the cut

**F24.** The governance-tier axis (`poc/internal/production/regulated`) is **concentrated, not diffuse** — roughly 18–20 files, almost all in the setup/constitution cluster: `commands/setup.md`; `skills/authoring-constitution/SKILL.md` + its six references (INTERROGATION-AGENDA, ESSENTIAL-FLOOR, catalog/README, catalog/universal-floor, catalog/backend-service, DOMAIN-DEPENDENCIES); `skills/review-governance-intent/SKILL.md`; `skills/validation-constitution/SKILL.md` + QUALITY-CHECKLIST; `skills/analysis-codebase/SKILL.md` (tier-*blind by design* but references it — `:75-78`); `skills/mochiko/SKILL.md` (router); and five templates (governance-intent, governance-surfaces, evolution-notes, layer-rules, release-gates). **Outside that cluster, only two tier-keyed runtime sites:** `implement.md`'s domain-registry checkpoint (F19) and its `CYCLE-REPORT-FORMAT`/`DOMAIN-DEPENDENCIES` pair. Caveat on counting: 34 files match the string "tier", but ~14 are *non-governance* senses that would not move — validator-trustworthiness "Tier-2" (`agents/validator.md`, `templates/workflow-contract.md`, `patterns-entity-modeling`, `patterns-api-contracts` script), strip-altitude "Tier-1/Tier-2" (`command-architect`, `authoring-commands`, `validation-command-shape`, `templates/command-shape.md`), a "model-tiered-seats" pointer (`grooming-operating-docs`), and a two-tiers logging *example* (`analysis-iterative/references/ADAPTIVE-EXAMPLES.md`).

**F25.** The type/shelf axis is ~10 files, same cluster (catalog + interrogation agenda + `detect-stack.sh`). Two of the three deferred types (CLI, library) are **already empty-by-decision shelves** and the frontend shelf is planned-but-unbuilt (F4) — so "defer libraries/SDKs/CLIs" removes mostly-empty scaffolding, not built content.

### 7. Both-ways facts

**F26 (supports the premise).** Much of the multi-type/multi-tier surface is scaffolding, not battle-tested. The frontend shelf is unbuilt and CLI/library shelves are empty by decision (`catalog/README.md:22-30`); the floor cards' seed-honesty note admits worked examples "are backend/service-flavored … Frontend-appropriate floor examples ship with the frontend shelf (planned next authoring pass)" (`universal-floor.md:14-17`). I find no recorded dogfood exercising a `poc` or `internal` constitution — the domain-dependencies brainstorm and the setup work were driven by real projects; the non-production rungs of the ladder appear un-exercised. (Bounded check of `brainstorms/index.md` + `BACKLOG.md` + `DECISIONS.md`; not exhaustive.)

**F27 (cuts against the premise).** The review-**sizing** machinery is load-bearing and mostly does **not** key on tier — it keys on **artifact weight**. `command-shape.md:47` makes the sizing default a per-command `[PARAM]` — "e.g. heavyweight→pair, **or** tier-keyed" — and only setup chose tier-keying. `brainstorm.md:30` sizes on "decision count, confidence-mark mix, reality-surface load" with "a heavyweight record defaults to the full pair." So collapsing the tier axis would **not** simplify or remove the sizing spine; that spine survives intact and would still need its non-tier keys.

**F28 (cuts against the premise).** Removing the tier *choice* does not remove the elicitation. Even at a fixed production tier, the interrogation still elicits 8 of its 10 dimensions — identity, type, risk surface, team reality, existing practices, knowledge-management, deployment reality, values, exclusions (`INTERROGATION-AGENDA.md:19-30`) — and the deck is still dealt recommend-then-arbitrate. The stated driver ("hedge per project kind") is split across two axes: **tier** drives the waiver/strictness hedging (which production-only genuinely removes), but **type** drives shelf selection and **risk/values** still tune the preset per project. Production-only asserts a fixed *floor*; it does not by itself convert the elicited, arbitrated governance model into an asserted one.

**F29 (neutral / clarifying).** The Essential Floor is *already* unconditional in concept — "no session emits a floor-less constitution … Absence is always deliberate and auditable, never silent" (`universal-floor.md:6-8`). Production-only would harden the four categories to their `production` rows and make the waiver machinery (the governance-intent waivers table, the `validation-constitution` waiver-posture checks at `QUALITY-CHECKLIST.md:35-36`) **dead code** — nothing left that can legally waive. The floor *structure* is unchanged either way.

## Fact-checker map — 2026-07-30 · addendum (refined boundary)

*(checker-authored, pasted verbatim)*

Folds the ruled target — customer-facing product software the team deploys/operates (SaaS, web, mobile, desktop); libraries/SDKs/CLIs deferred — onto the mapped facts. Same grounding rules.

**F30 (the in-scope product kinds are mostly unbuilt at the shelf level).** The dimension-3 project-type list is exactly "frontend / backend / fullstack / CLI / library / service" (`INTERROGATION-AGENDA.md:23`) — no "mobile", no "desktop", no "SaaS" in it. Mapping the four ruled in-scope kinds onto the shelf machinery: **web/SaaS** → frontend + backend + fullstack + service types (backend-service shelf **seeded**; frontend shelf **planned-but-unbuilt**, `catalog/universal-floor.md:14-17`, `catalog/README.md:23`); **mobile** → named only as the non-committed "mobile/app shelf" (`ROADMAP.md:29`), absent from the type list, appearing elsewhere only in worked examples; **desktop** → **absent entirely** (zero occurrences of "desktop" anywhere in the plugin). So of the four ruled in-scope kinds, only backend/service has seeded catalog content today; frontend is unbuilt scaffolding, mobile is a deferred future item, desktop has no representation at all.

**F31 (the deferred kinds are already-empty scaffolding — confirms F4/F25).** libraries/SDKs/CLIs map onto the `library` and `cli` types in dimension 3 and their catalog shelves, both "empty by decision — mint-driven" (`catalog/README.md:24-30`). Deferring them removes two type options and two already-empty shelves; no built content is lost. ("SDK" as a term appears nowhere in the plugin; it falls under the `library` type.)

**F32 ("the team deploys/operates" is exactly what is conditional today).** The ruled boundary assumes an operated, deployed product; in the library as built that assumption is elicited/gated, not assumed: (a) deployment-and-release is interrogation dimension 8, prunable under the low-tier license (example skip "dimension 8 (deployment) skipped: poc tier", `templates/governance-intent-template.md:62`); (b) FLOOR-OBS is "out" at poc, "offer" at internal, only "default-in" at production/regulated (`catalog/universal-floor.md:76-89`); (c) the `release-gates` constitution module attaches only "when the deployment dimension elicited a real release process" (`templates/constitution-modules/release-gates.md:4-6`); (d) RUNBOOK is a per-doc elective offered only "for deployed services" and CHANGELOG only "for release-shaped projects" (`INTERROGATION-AGENDA.md:37`). A production-only stance taking deploy/operate as given would make deployment-reality, observability, release gates, and the RUNBOOK elective unconditional rather than tier/type-gated.

**F33 (neutral — the refined boundary narrows the *type* axis more than the *tier* axis).** The ruled target is a statement about project **type/shape** (which product kinds are in scope), not directly about the poc→regulated **tier** ladder. The two are orthogonal in the source: a customer-facing SaaS can still be declared at any tier today (nothing binds product-kind to tier). So the refined boundary bears most directly on dimension 3 + the shelves (F4, F30, F31); its effect on the tier ladder is the separate, second question the session's stated driver raises (assert production standards vs. hedge per kind). The two cuts are independent and can be ruled independently.

---

## Decisions

### D1 — Target boundary: deployed, operated, customer-facing product software — `Confident`
**Statement:** Mochiko's sole target is customer-facing product software that the team deploys and operates: SaaS, web, mobile, and desktop apps. Internal tools, prototypes/experiments, and distributed artifacts (libraries, SDKs, CLIs) are out of scope.
**Rationale (user's):** keep the focus sharp — every standard must be assertable unconditionally; the wider readings each reintroduce a "depends on the project" fork.
**Logged nuance:** the user was tempted by the wider "anything shipped beyond the team" reading (libraries/SDKs/CLIs included) and deferred it deliberately — "for now, let's keep A." Revisiting the distributed-artifact shelf later is legitimate; it is deferred, not rejected.
**S5 fold (review):** the operative criterion is **application vs building-block** — mochiko targets end-user-facing *applications* (SaaS, web, mobile, desktop); *building-blocks* (libraries, SDKs, CLIs) and internal/prototype software are out. The "team deploys and operates" phrasing is subordinated to descriptive color: it fits SaaS/web but does not separate in-scope desktop/mobile (shipped artifacts) from excluded CLIs — application-vs-building-block is the axis that does.

### D2 — The tier axis dies: one production floor + fact-triggered modules — `Confident`
**Statement:** Within the production-only world there is exactly one production standard — a non-negotiable floor. The tier ladder (prototype → … → regulated) is removed as an axis. Regulated/compliance needs (HIPAA, PCI, SOC 2, …) stop being a tier and become **additive elective modules triggered by project facts** (industry, data classes) — they only ever add obligations on top of the floor, never subtract.
**Rationale:** a tier asks the user *how much rigor they want* — that question is itself the hedge being eliminated; a module asks *what constraints the project factually has*, which has no rigor negotiation in it. Cleanest to enforce and document.
**Provenance note:** lead-recommended, user-adopted without elaboration ("yes A").
**S1 fold (review, user-ruled):** "non-negotiable" means the floor's *level* is not negotiable — nothing can lower it. It does not mean unwaivable: the D4 recorded-waiver escape applies to floor categories. The scoped exception is carved at D4.2: legally-mandated compliance-module obligations are unwaivable. Module obligations remain additive-only in both readings.

### D3 — Mochiko owns the production standard; setup elicits facts, not standards — `Confident`
**Statement:** The library ships a canonical, versioned production standard — governance, coding, and security baseline. Setup stops eliciting *the asserted layers — the safety floor and fact-triggered modules* (architecture-opinion standards stay arbitrated per the S7 fold below; verify-pass repair #2): it elicits project **facts** (stack, industry, data classes, integrations) and applies the asserted floor plus fact-triggered modules. Deviations happen only through recorded waivers in the governance ledger — auditable, visible, never silent.
**Rationale:** kills the deepest form of the hedge — every project re-deriving its own standards through elicitation (map F15); directly answers map F28's finding that removing the tier axis alone would leave the elicited, recommend-then-arbitrate governance model intact. Waiver machinery already exists in the ledger, so the escape valve costs nothing new.
**Provenance note:** lead-recommended, user-adopted without elaboration ("i will go with your recommendation") — second consecutive unelaborated adoption; streak watch active per the questioning discipline.
**S7 fold (review, user-ruled):** assertion-scope carve-out — the safety floor (Security / Testing / Error-Handling / Observability) is asserted; **architecture-opinion standards** (BE-HEX, BE-SRP, BE-DEP and kin) remain recommend-then-arbitrate, the one deliberately elicited layer: architecture choice is per-project judgment, not a rigor dial, and asserting it at floor strength would make a day-one waiver the default entry state for every competent non-hexagonal team.
**S4 fold (review):** D3's "a fact has no negotiation in it" rationale is recorded as incomplete — a *wrong* elicited fact is a new, silent under-scoping failure surface (user answers "no regulated data," the app stores health metrics, the HIPAA module never fires, nothing is recorded). The D3 follow-on carries a fact-validation fail-safe for module-driving facts.
**Open sub-question (→ Q6):** waiver scope. Today's `production` tier already forbids floor waivers (map F6, F29); D3 as adopted allows recorded waivers generally. Where waivers stop — floor included or floor exempt — is unruled. → Ruled at D4.

### D4 — Waivers reach everything, floor included — recorded, auditable, never silent — `Contested`
**Statement:** Any asserted standard — the four floor categories included — can be waived with a recorded, auditable justification in the governance ledger. There is no unwaivable stratum **except the legally-mandated compliance obligations carved at D4.2** (S1 fold — verify-pass repair #1). This deliberately *loosens* today's `production` posture, which forbids floor waivers outright (map F6).
**Rationale (as steelmanned in session; user picked B without elaborating):** translation tables for the in-scope product kinds have real gaps today — desktop has zero built content, mobile and frontend shelves are unbuilt (F30) — and an absolute rule that doesn't fit reality gets bypassed silently; a visible recorded waiver is safer than an invisible workaround.
**Provenance note:** the lead had stated the fact-translation model materially strengthens the opposite option (floor-unwaivable); the user chose B anyway — deliberate preference, held after the lead's one pressure-test — marked `Contested`.
**D4.1 — Waiver expiry: `Deferred`.** The pressure-test offered mandatory expiry/review-by dates on floor waivers (temporary relief matching the shelf-immaturity rationale). User ruling: "keep the permanent waiver for now. This decision I will come to revisit later." Explicit revisit marker — permanent waivers stand until then.
**D4.2 — Legal-mandate exception (S1 fold, review, user-ruled) — `Confident`.** Obligations that enter via a legally-mandated, fact-triggered compliance module (PCI, HIPAA, …) are **unwaivable**. A recorded permanent waiver of a legal control is not an honest escape valve — it is documented evidence of a knowing violation. The floor itself stays waivable per D4; only the legal-mandate stratum is exempt. (Both reviewers converged on this seam independently: DQ-3 + RI-1.)

### D5 — The production-depth agenda: three ratified tiers — `Confident`
**Statement:** the deepening that the narrowing funds, in three tiers.
- **Tier I — rides with the narrowing (identity-critical):** (1) **Security depth** — threat modeling at plan time, security requirements with teeth, blocking SAST + dependency-vuln gates, a security lens in the validator set; first-in-line, scoped in its own follow-on session, not built here. (2) **Operations & observability hardening** — SLOs as first-class NFRs, RUNBOOK promoted elective→asserted, incident-response basics, release-health expectations per product kind. (3) **Shelf translation tables for the in-scope kinds** — frontend first, then mobile, then desktop; the narrowing's own load-bearing prerequisite (map F30).
- **Tier II — queued immediately behind, own scoping sessions, production-only frame attached:** (4) **IaC/deployment engineering, staged** — release gates + environment discipline asserted first; infrastructure-code authoring second (new artifact class, map F23). (5) **Data lifecycle** — schema-migration discipline, backup/restore verification, retention riding DS-XXX. (6) **Reliability & resilience** — timeout/retry/circuit-breaker cards, perf/load verification keyed to existing NFR targets.
- **Tier III — later, real but separable:** (7) **Multi-repo topology** (dedicated brainstorm; map F22). ~~(8) Accessibility (WCAG)~~ → re-routed at the S8 fold below.
**S8 fold (review, user-ruled):** accessibility leaves Tier III — WCAG for customer-facing software is jurisdiction-mandated in most target markets, making jurisdiction/industry a *trigger fact*: a11y routes through D2's fact-triggered compliance modules (its shelf *content* may still ride the Tier-I frontend build). Tier III retains multi-repo alone.
**Rationale:** Tier I items are what keep the asserted floor from being hollow on arrival; Tier II are staged identity extensions; Tier III are separable capabilities.
**Provenance note:** recommend-then-arbitrate — lead-supplied set (user: "security and IaC is what came to my mind, it is not exhaustive… I like you to suggest"); user ratified the tiers with both salient demotions of their own named items (IaC → Tier II staged; multi-repo → Tier III) explicitly confirmed: "keep the tiers as you suggested. keep tier 3 for later."

### D6 — Identity landing: the boundary gets written into the operating docs — `Confident`
**Statement:** D1's boundary sentence (customer-facing product applications — SaaS, web, mobile, desktop) is written into **ROADMAP.md's thesis** and **CLAUDE.md's "What this is"**; the session's rulings land as **DECISIONS.md rows**; ROADMAP's Later items re-frame per D5's tiers (frontend/mobile shelves → Tier-I work; monorepo registries → under Tier III; CLI/library items retire under D1's deferral).
**Rationale:** map F1/F2 — no operating doc states any target today; the identity was never written down. A positioning that lives only in plugin machinery doesn't exist. Standard landing per the operating-docs contract.
**S3 fold (review):** the landed identity text carries the honest qualification — *backend/service standards are seeded today; frontend, mobile, and desktop shelves are Tier-I roadmap work* — so the public docs never advertise coverage ahead of content.
**S9 fold (review):** ROADMAP's "Seat-tiering brainstorm" Later item is explicitly **untouched** by the reframe — its "tier" is the model-seat sense (see `model-tiered-seats`), not the governance tier D2 kills.
**Provenance note:** posed as a confirmation (Q10); initially confirmed only implicitly at the wrap ruling. **S2 fold (review, user-ruled):** explicitly confirmed at review disposition — "i will go with your recommendations" adopting S2's confirm-D6-with-qualification — mark retained `Confident`, now citing an active affirmation.

### D7 — The immature-but-in-scope team is served: full floor + recorded waivers as the staged on-ramp — `Confident`
**Statement:** Early-stage customer-facing teams (an MVP, a 2-person team) are in scope and enter at the full asserted floor; recorded waivers (D4) are the honest, visible staged-adoption mechanism. Nothing replaces the retired `poc`/`internal` rungs.
**Rejected road (recorded with steelman):** a maturity axis orthogonal to product-kind. *Steelman:* binary floor items (auth-at-all-boundaries, blocking SAST) don't ratchet the way coverage percentages do, so a maturity ladder would give young teams a legitimate intermediate rung instead of day-one waivers. *Rejected because:* it reintroduces the rigor dial D2 killed — the waiver ledger achieves staged adoption without making the standard itself negotiable, and D4.1's deferred expiry question is the natural place to add time-boxing if waiver-as-normal-state proves real in dogfoods.
**Provenance note:** review survivor S6 (DQ-2); lead-recommended, user-adopted in the disposition batch.

### Mechanical edit surface implied by D1–D4 (consequences, not new decisions)
From the map; listed so the landing is honest about scope. All in the setup/constitution cluster plus two implement sites (F24):
- Interrogation agenda: dimension 2 (tier) removed; low-tier pruning license removed (F5, F20) — deployment reality always interrogated (F32); dimension 3's type list narrows to the in-scope kinds — **mobile and desktop ADDED as type options** (S3 fold; F30 shows neither exists today); `cli`/`library` options + their empty shelves retire under D1's deferral (F4, F31).
- `governance-intent-template`: tier declaration GI-001 + graduation path removed; deck-ruling "at tier preset" column reframed to the fact profile; waivers table re-scoped to D4 (any standard, recorded justification, permanent pending D4.1) (F10, F29).
- Floor cards: per-tier strictness ladders collapse to the `production` row as the single asserted level; `regulated` rows become fact-triggered module content (F7, F16, D2).
- Backend-service architecture-opinion cards (BE-HEX/SRP/DEP): stay **recommend-then-arbitrate** per D3's S7 carve-out; single-level production-strength enforcement *when kept* (F8, F17).
- Fact→module trigger mechanism: the attachment logic (which fact fires which module — industry, data classes, jurisdiction) is authored as part of the narrowing's build — previously unscheduled (S4 fold, RI-4); carries the fact-validation fail-safe for module-driving facts (S4 fold, DQ-6).
- Setup's synthesis-review sizing gate: tier key collapses → fixed pair default (F18).
- Implement's domain-registry checkpoint: always blocking (F19; + `CYCLE-REPORT-FORMAT`/`DOMAIN-DEPENDENCIES` mirrors).
- `validation-constitution` waiver-posture checks rewritten to the D4 model (F29).
- `analysis-codebase` + router + `review-governance-intent` tier references cleaned (F24).

### The interview's new job (D3 mechanics, presented and not yet contradicted)
The interview never sets the floor's *level*; it sets its **shape** (product-kind facts translate each floor category into its correct expression — *translation, not waiver*: the D4 recorded-waiver path is a separate, ledger-visible mechanism, and D4.2's legal-mandate stratum is exempt from it), its **triggers** (industry/data-class facts attach additive modules mechanically), and its **path** (brownfield facts + codebase analysis set the ratchet's starting point, never its target; Emergent Ceiling survives untouched). Agenda test for every question: elicits a fact → keep; negotiates a standard → cut. Scope per D3's S7 carve-out: this asserted model covers the safety floor and modules; architecture-opinion cards remain the deliberately arbitrated layer.

---

## Review

**Sizing gate (2026-07-30):** weight stated — 6 decisions (D1–D6) + 1 deferred sub-ruling (D4.1); confidence mix 5 `Confident` / 1 `Contested` (D4) / 1 `Deferred`; two presented-model sections (interview mechanics, mechanical edit surface); reality-surface load heavy — a 33-finding fact-checker map (F1–F33) embedded verbatim. Default keying: heavyweight → full pair. **User ruling: pair** ("wrap and pair review"). Lens split per the command: decision-quality vs record-integrity; both briefs name the embedded map as the fact substrate; verify pass owned by the record-integrity reviewer.

**Cold reads + cross-exam (2026-07-30):** dq-reviewer 9 raised (1C/6I/2M) · ri-reviewer 5 raised (0C/2I/3M) · one-shot four-message cross-exam run; DQ-9 withdrawn (fallen, retrievable). ri-reviewer's sample audit of the embedded map: **CLEAN** (9 load-bearing F-findings verified against cited files; sole immaterial drift: F24's "34 files" is 35 case-insensitive/27 case-sensitive, self-hedged, no decision turns on it). No fact disputes arose. Both reviewers recommend **needs-revision**.

**Lead's cross-set merge:** 14 raised → 13 survived cross-exam → **9 merged survivors** (1 Critical, 6 Important, 2 Minor):

| # | Sev | Root | Merged from | Answer owner |
|---|-----|------|-------------|--------------|
| S1 | Critical | D2×D4 seam: "non-negotiable floor / modules only add" vs "no unwaivable stratum, permanent waivers" — incl. legally-mandated compliance obligations being waivable | DQ-3 + RI-1 | **user** (amends D4/D2) |
| S2 | Important | D6 confidence mark overclaims — `Confident` on an implicit, never-affirmed confirmation; D6 authorizes the operating-docs rewrite | RI-2 ≡ DQ-8 (independent cold convergence) | **user** (confirm or re-mark) |
| S3 | Important | F30 cluster: identity landing advertises SaaS/web/mobile/desktop while only backend/service has content; dimension-3 type list gains no mobile/desktop | DQ-5 + RI-3 | lead formulation |
| S4 | Important | D3's fact-trigger dependency has no build home (consequence list) and no fail-safe for wrong facts (silent under-scope — HIPAA example); D3 rationale gap | RI-4 + DQ-6 | lead formulation |
| S5 | Important (at the Minor line) | D1's operative criterion misfit: "team deploys and operates" doesn't separate in-scope mobile/desktop from excluded CLIs/SDKs; application-vs-building-block is the real axis | DQ-1 | lead formulation |
| S6 | Important | In-scope-but-immature (customer-facing MVP, small team) meets binary floor items with only permanent waivers; rejected maturity-axis road unrecorded | DQ-2 | **user** (D1/D2 scope) |
| S7 | Important | Assert-don't-elicit overreaches into architecture taste: BE-HEX/SRP/DEP flattened to asserted floor strength; day-one waiver becomes the default entry state for competent non-hexagonal teams | DQ-4 | **user** (amends D3 scope) |
| S8 | Minor | A11y mis-categorized: legally-mandated customer-facing dimension routed to D5 Tier III instead of D2's fact-triggered modules | DQ-7 | **user** (amends D5) |
| S9 | Minor | D6's ROADMAP reframe silent on the "Seat-tiering brainstorm" Later item (model-seat sense of "tier", untouched — needs saying) | RI-5 | lead formulation |

**Lead's clearing read:** needs-revision, session-resolvable — the Critical resolves by scoping clause (S1), not by structurally reopening D4; escalation to critical-gaps not triggered unless the user's S1 ruling reopens D4. Dispositions below as they land; verify pass (ri-reviewer) after folds.

**Dispositions (2026-07-30) — 9/9 landed.** User ruling batch: "i will go with your recommendations" (all five user-territory recommendations adopted after full per-survivor presentation).
- **S1 — user-ruled (adopted):** D4.2 legal-mandate exception + D2 reconciling clause; interview-mechanics phrasing aligned.
- **S2 — user-ruled (adopted):** D6 explicitly confirmed at disposition; mark retained `Confident` with the affirmation cited.
- **S3 — resolved:** D6 qualification (backend/service today, other shelves Tier-I) + mobile/desktop added to the type-list consequence.
- **S4 — resolved:** trigger-mechanism authoring scheduled in the consequence list + D3 fail-safe note and rationale-gap record.
- **S5 — resolved:** D1 operative-axis cleanup (application vs building-block; deploy/operate subordinated).
- **S6 — user-ruled (adopted):** D7 (immature-in-scope served; maturity-axis rejected road recorded with steelman).
- **S7 — user-ruled (adopted):** D3 carve-out (safety floor asserted; architecture-opinion cards stay arbitrated) + consequence-list edit.
- **S8 — user-ruled (adopted):** a11y re-routed from D5 Tier III to D2's fact-triggered modules.
- **S9 — resolved:** D6 seat-tiering untouched-disposition made explicit.

**Verify pass (ri-reviewer, owner per sizing gate):** 9/9 folds CONFIRMED landed with quoted evidence; disposition list agrees with the body. Caught one fold-introduced contradiction — **#1 (required):** D4's statement still read "no unwaivable stratum" against the new D4.2 — plus **#2 (recommended):** D3's "stops eliciting standards" headline vs its own S7 carve-out. Both repairs penned (D4 statement exception clause; D3 statement scope qualifier) and **re-verified CLEAN**; D2/D4.2/D7 triangle coherent after repair. #3 (D1 "deploys/operates" phrasing — fold reconciles in place) recorded acceptable-no-repair.
**Recorded-open (pre-existing, not a verify finding):** whether *non-legal* fact-triggered module obligations are waivable under D4 or additive-only under D2 — surfaced at acceptance; natural home: the D4.1 waiver-expiry revisit or the trigger-mechanism build (S4).
**Lead's clearing verdict: ready.** Both reviewers' `needs-revision` recommendations were pre-disposition; with 9/9 dispositioned, both verify repairs landed, and the map audited clean, the review contract is discharged. **Combined tally: 14 raised → 13 survived cross-exam → 9 merged survivors → 9/9 dispositioned (5 user-ruled, 4 resolved) + 2 verify-pass repairs, re-verified CLEAN.**
