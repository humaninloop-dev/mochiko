# Ops & observability hardening (Tier I) — scoping session record

- **Status:** accepted (user, 2026-07-31 — "accept"; opened 2026-07-30)
- **Driver:** PO-D5's second Tier-I depth item. BACKLOG mandate: "SLOs as first-class NFRs, RUNBOOK elective→asserted, incident-response basics, release-health expectations per product kind; mostly promoting existing electives to asserted (map F32)." Scoped against the post-v0.36.0 production-only surface, with the security-depth build (SD-D1–D6) pending but not started.
- **Form:** bare session — fact-checker map → adaptive one-question-at-a-time questioning with recommendations + steelmans → provenance notes → sizing gate → cold review per sizing → landing.
- **Canonical:** this record. Fact map embedded verbatim below once delivered.

## Fact-checker map

*Delivered 2026-07-30 in four sequential messages, concatenated here verbatim (delivery part-headers and continuation markers stripped; nothing else altered). F1–F133, Areas A–N + coverage notes.*

**Session-open erratum (fact-checker correction to the dispatch briefing):** the briefing stated the security-scoping landing was uncommitted; the checker verified HEAD `b32dd82` with a clean tree (only `?? .claude/settings.local.json` untracked) — **the landing is committed.**

Repo `/Users/deepeshadmin/Documents/GitHub/mochiko`, branch `main`, HEAD `b32dd82`. Plugin version `0.36.0` (`plugins/mochiko/.claude-plugin/plugin.json:3`).

## Area A — The floor as it stands (ops-relevant rows)

**F1.** `ESSENTIAL-FLOOR.md` declares itself the canonical home of the four floor **category definitions** only — cards carry the level.

> **Canonical home.** This file is the single source of truth for the four Essential Floor category **definitions**. Both modes of `authoring-constitution` (greenfield writes them at the asserted floor level; brownfield assesses the codebase against them) reference this definition, and the cross-cluster `analysis-codebase` skill references it for its present/partial/absent status assessment rather than re-defining the categories. Edit the four categories here, nowhere else.

`plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md:3`

**F2.** What "asserted" means at the floor, verbatim.

> Every constitution MUST **account for** all four floor categories — with a principle or a
> **recorded waiver** (D4: justification in the governance ledger; permanent pending the D4.1
> revisit). The floor concept is invariant: no session emits a floor-less constitution, and absence
> is never silent. The floor's **level is single and asserted** — the production level on each
> floor card in [catalog/universal-floor.md](catalog/universal-floor.md); nothing can lower it, and
> a deviation is only ever a recorded waiver, never a loosened card. Audit-evidence variants live
> in [COMPLIANCE-MODULES.md](COMPLIANCE-MODULES.md) and attach via the fact profile.

`ESSENTIAL-FLOOR.md:5-11`

**F3.** The Observability category's four required specifics — the entire observability mandate at definition level.

> ### Observability Principle MUST address:
>
> - **Logging format**: Structured JSON logging with standard fields
> - **APM tools**: Name specific tools if detected (e.g., Application Insights, Datadog, New Relic)
> - **Health checks**: Endpoint path and what it validates
> - **PII prohibition**: Logs MUST NOT contain personally identifiable information

`ESSENTIAL-FLOOR.md:38-43`

**F4.** The Error Handling category's specifics (the correlation-ID home shared with observability).

> ### Error Handling Principle MUST address:
>
> - **Response format**: RFC 7807 Problem Details (preferred) or consistent JSON schema
> - **Error codes**: Naming convention (e.g., `ERR_DOMAIN_ACTION`)
> - **Stack traces**: MUST NOT be exposed in production responses
> - **Correlation**: Error responses MUST include correlation/trace IDs

`ESSENTIAL-FLOOR.md:31-36`

**F5.** The worked observability example principle — the only place the floor states operative observability MUSTs.

> ### IV. Observability Requirements (NON-NEGOTIABLE)
>
> The app MUST be observable. When something goes wrong in production, there MUST be enough information to diagnose and fix it.
>
> - Logs MUST use structured JSON format with standard fields
> - Logs MUST have appropriate levels (debug, info, warning, error)
> - Logs MUST NOT contain sensitive data (PII, tokens, passwords)
> - Errors MUST be logged with context (user action, app state, correlation ID)
> - Health check endpoint MUST exist at `/health` or `/healthz`

`ESSENTIAL-FLOOR.md:122-130`

**F6.** The log-levels table shipped with that example.

> | Level | Use For | Example |
> |-------|---------|---------|
> | `error` | Failures requiring attention | API call failed, database write error |
> | `warning` | Recoverable issues | Retry succeeded, fallback used |
> | `info` | Significant state changes | User logged in, sync completed |
> | `debug` | Development diagnostics | Request/response bodies, state dumps |

`ESSENTIAL-FLOOR.md:134-139`

**F7.** Its enforcement / testability / rationale triad.

> **Enforcement**:
> - Structured logging with required fields enforced by wrapper
> - Code review MUST verify no PII in log statements
> - Health check endpoint verified in integration tests
>
> **Testability**:
> - Pass: All errors logged with context, no PII in logs, health check responds
> - Fail: Silent failures OR PII in logs OR missing correlation IDs
>
> **Rationale**: You cannot fix what you cannot see. Production issues without observability become guessing games. Good observability reduces mean time to resolution.

`ESSENTIAL-FLOOR.md:141-150`

**F8.** ABSENCE — the file's four category definitions are exactly Security / Testing / Error Handling / Observability. There is no deployment, release, monitoring-tooling, backup, runbook, or incident category at definition level. Verified by reading the whole 150-line file; the only `###` category headers are at lines 17, 24, 31, 38 and the four example headers at 49, 73, 99, 122.

## Area B — Catalog cards

**F9.** Card format and the two layers, verbatim.

> ```markdown
> ### CARD-ID — Card Name
> **Type tags:** [which project types this card fits]
> **Layer:** [floor-asserted | arbitrated]
> **Asserted level / Recommended form:** [the single production-level content — thresholds,
>              enforcement strength; floor-asserted cards state the asserted level, arbitrated
>              cards the recommended form]
> **Content:** [the principle material — statement skeleton, enforcement/testability/rationale
>              source, or a pointer to the canonical definition]
> ```
>
> - **floor-asserted** — enters every session at the asserted level; not arbitrated; expression
>   shaped by type; loosening only via recorded waiver.
> - **arbitrated** — dealt recommend-then-arbitrate (the S7 carve-out layer: architecture-opinion
>   and other per-project-judgment cards); the user keeps / tightens / drops / re-ranks.

`plugins/mochiko/skills/authoring-constitution/references/catalog/README.md:59-74`

**F10.** The three provenance sources a card's content can carry (the trace vocabulary ops rows would enter under).

> 1. **Floor-asserted** — an Essential Floor card at the asserted production level, its expression
>    shaped by type facts during the session; deviations only ever through recorded waivers
>    (never a loosened card).
> 2. **Deck-kept** — an arbitrated catalog card the user kept (possibly tightened) during
>    arbitration.
> 3. **Minted** — a principle written fresh from the user's elicited intent (values dimension).
>    Minted content MUST trace to elicited intent, never to shallow prompting.

`catalog/README.md:8-14`

**F11.** What varies per project under the single asserted level — the "expression" axis names a per-kind ops translation directly.

> - **Expression** — type facts translate each floor category into its correct form (an API error
>   schema vs UI error states; a web health check vs a desktop crash reporter).
> - **Modules** — compliance obligations attach additively from the fact profile per
>   [../COMPLIANCE-MODULES.md](../COMPLIANCE-MODULES.md) (the retired `regulated` rows live there
>   as seed content).
> - **Waivers** — any asserted standard can be waived with a recorded, auditable justification in
>   the governance ledger (D4; permanent pending the D4.1 revisit) — except legal-mandate module
>   obligations (D4.2). A waiver is never silent: recorded in the synthesis and the ledger, it is
>   the honest staged-adoption on-ramp for early-stage teams (PO-D7). Accumulated waivers are the
>   governance re-entry checklist as the team matures.

`catalog/README.md:44-53`

**F12.** The shelf table — only two shelves are seeded.

> | Shelf | File | Dealt to | Status |
> |-------|------|----------|--------|
> | Universal floor | [universal-floor.md](universal-floor.md) | every project, every type | seeded |
> | Backend / service | [backend-service.md](backend-service.md) | backend, service, fullstack (API side) | seeded |
> | Frontend | `frontend.md` | frontend, fullstack (UI side) | **planned — Tier-I roadmap work** |
> | Mobile | `mobile.md` | mobile | **planned — Tier-I roadmap work** |
> | Desktop | `desktop.md` | desktop | **planned — Tier-I roadmap work** |

`catalog/README.md:22-28`

**F13.** FLOOR-OBS in full — the single ops-touching floor card.

> ### FLOOR-OBS — Observability Requirements
>
> **Type tags:** all
> **Layer:** floor-asserted
> **Asserted level:** structured logs · correlation IDs · health checks (in the form that fits the
> type) · no PII in logs.
> **Waiver posture:** D4 — recorded justification in the ledger (historically the most-waived
> category on immature stacks — the recorded waiver, not a silent gap, is the honest state).
> **Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Observability).

`plugins/mochiko/skills/authoring-constitution/references/catalog/universal-floor.md:62-70`

**F14.** FLOOR-ERR (adjacent; carries correlation IDs and the type-fitting error surface).

> ### FLOOR-ERR — Error Handling Standards
>
> **Type tags:** all
> **Layer:** floor-asserted
> **Asserted level:** failures never silently corrupt data · consistent error surface in the form
> that fits the type (API error schema, UI error states, mobile/desktop failure surfaces) ·
> correlation IDs · no leaked stack traces.
> **Waiver posture:** D4 — recorded justification in the ledger.

`universal-floor.md:50-57`

**F15.** FLOOR-TEST's asserted level (the only floor card with a numeric threshold and a ratchet).

> **Asserted level:** coverage pre-seed (session-overridable): ≥80% warning, ≥60% blocking ·
> ratchet rule (baseline MUST NOT decrease) · a smoke test on the critical path exists from day
> one.

`universal-floor.md:40-42`

**F16.** FLOOR-SEC's asserted level (the row the pending security build edits).

> **Asserted level:** secrets out of the repo (env vars + `.gitignore`) · secret scanning in CI ·
> input validation at boundaries · auth enforced at all boundaries · dependency vulnerability
> scanning blocking merge.

`universal-floor.md:27-29`

**F17.** The universal-floor shelf's seed-honesty note — the backend flavor of the shipped ops examples is stated as a known gap.

> > **Seed honesty note:** the current worked examples are backend/service-flavored (RFC 7807 error
> > bodies, `/health` endpoints). Frontend-, mobile-, and desktop-appropriate floor examples ship
> > with their shelves (planned — Tier-I roadmap work). Until then, translate the *category
> > requirements* to the declared type during the session rather than copying misfitting examples.

`universal-floor.md:16-19`

**F18.** ABSENCE — `universal-floor.md` contains exactly four cards (FLOOR-SEC, FLOOR-TEST, FLOOR-ERR, FLOOR-OBS; `###` headers at lines 23, 36, 50, 62 of a 70-line file). There is no FLOOR-OPS, FLOOR-REL, FLOOR-DEPLOY, or runbook/SLO/incident card at floor level.

**F19.** The backend-service shelf's three cards are all arbitrated-layer, none observability/ops-shaped.

> Dealt when the declared project type is **backend, service, or fullstack (API side)**. These were
> mochiko's former universal greenfield defaults; they are now type-selected cards — good
> architecture for services, misfitting baggage for an SPA or a mobile app. All three are
> **arbitrated-layer** cards (architecture-opinion — PO-D3's S7 carve-out): architecture choice is
> per-project judgment, not a rigor dial, so they are dealt recommend-then-arbitrate, never
> asserted.

`plugins/mochiko/skills/authoring-constitution/references/catalog/backend-service.md:3-8` — the three cards are BE-HEX (`:16`), BE-SRP (`:69`), BE-DEP (`:106`).

**F20.** BE-DEP is the shelf's only card touching a CI-blocking runtime-risk gate.

> **When kept:** production-strength enforcement — dependency scanning blocks merge at high/critical. License compliance, documented supply-chain review, and tightened blocking severity (medium+) are compliance-module content ([../COMPLIANCE-MODULES.md](../COMPLIANCE-MODULES.md)), attached via the fact profile.

`backend-service.md:110`

**F21.** The compliance-module seed relocates the retired `regulated` observability rows — the only place log retention appears in the plugin.

> - **Observability** *(ex FLOOR-OBS `regulated`)*: log retention policy · access-controlled log
>   storage · audit-grade traceability.

`plugins/mochiko/skills/authoring-constitution/references/COMPLIANCE-MODULES.md:45-46`

## Area C — The release-gates module

**F22.** The module lives at `plugins/mochiko/templates/constitution-modules/release-gates.md` (40 lines; the constitution-modules directory is under `templates/`, not under the skill's `references/`). Its full attach condition:

> MODULE: release-gates
> =====================
> Attach when: always offered for the target class — customer-facing software the team deploys
> and operates has a real release process by definition (PO-D1), and the deployment-and-release
> dimension (always interrogated — no pruning license) supplies its content: environments,
> cadence, release-blocking criteria, rollback expectations. Trace: the GI module-selection
> element that names `release-gates`.

`plugins/mochiko/templates/constitution-modules/release-gates.md:2-8`

**F23.** Content headline and instruction.

> ## Release Gates
>
> <!--
> INSTRUCTION: What blocks a release — beyond the per-merge Quality Gates. Source from the
> synthesis's deployment-reality element. Use the project's actual environment names and real
> verification commands.
> -->
>
> **Environments:** [e.g. dev → staging → production, with promotion rules]
> **Cadence:** [e.g. on-merge continuous / weekly cut / manual]
>
> | Gate | Requirement | Verified by | Blocks |
> |------|-------------|-------------|--------|
> | [e.g. Staging soak] | [e.g. 24h error rate < baseline] | [dashboard/command] | promotion to production |
> | [e.g. Migration check] | [e.g. reversible migration verified] | [command] | deploy |
> | [e.g. Changelog] | [entry present for user-facing change] | PR check | release cut |

`release-gates.md:11-26`

**F24.** The rollback subsection — the plugin's entire rollback surface.

> ### Rollback
>
> - Rollback procedure MUST be documented and executable by [role]: [pointer or inline steps]
> - [Rollback time expectation, e.g. "restore previous version in ≤15 minutes"]
> - Releases that cannot be rolled back (e.g. destructive migrations) MUST be flagged in the PR and
>   approved explicitly

`release-gates.md:28-33`

**F25.** Its validator checklist fragment.

> <!-- ── Validator checklist fragment (checked only when this module is attached) ──
> - [ ] Environments and cadence stated with the project's real environment names
> - [ ] Release-gate table present; every gate has a concrete verification (command/dashboard), no placeholders
> - [ ] Rollback procedure documented with a time expectation
> - [ ] Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)
> -->

`release-gates.md:35-40`

**F26.** VERIFIED — the offering dimension is **dimension 8**, and the always-offer language is in the agenda row, not the module.

> | 8 | **Deployment & release reality** | Target, environments, cadence, what blocks a release, rollback expectations | Quality Gates; Observability/Error-Handling expression; the `release-gates` module offer — default-on for a deployed product, recorded either way — **always interrogated**: the target is software the team deploys and operates |

`plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md:35`

**F27.** Dimension 7 is the other ops-adjacent dimension (it owns the RUNBOOK elective).

> | 7 | **Knowledge management** | Adopt or decline the `knowledge-management` module — the operating-docs layer (brainstorms + `index.md`, open-only `BACKLOG.md` + trail, `ROADMAP.md`, the decisions layer, `ARCHITECTURE.md`, `GLOSSARY.md`, plus the enforcement surfaces), offered default-on — **core taken whole, electives (`CHANGELOG.md` / `RUNBOOK.md`) per-doc** | Module ruling (recorded either way); elective rulings; G5 scaffolding; collision rulings |

`INTERROGATION-AGENDA.md:34`

**F28.** Dimension 4 (risk surface) and dimension 6 (existing practices) are the remaining ops-adjacent rows; there is **no scale, availability, traffic, or operating-load dimension**.

> | 4 | **Risk surface** | What failure costs: data loss, money, reputation, compliance, user trust | Honest context for the floor's expression and the arbitrated card layer |
> | 6 | **Existing practices & tools** | Detected stack, CI, linters, tests; brownfield analysis feeds in here | The real commands the validator requires |

`INTERROGATION-AGENDA.md:31,33` — the full dimension table is lines 26-37 and contains ten rows; none names scale, availability, uptime, traffic, SLO, or incident.

**F29.** The no-pruning-license clause that makes dimension 8 unconditional.

> ## No pruning license
>
> The retired tier ladder's low-tier pruning license is gone (PO-D2): every project here is a
> deployed, operated, customer-facing product, so no dimension is foreclosed by declaration —
> deployment reality in particular is always interrogated. Adaptive convergence still applies —
> skip what an answer has already settled, and say so — but a convergence skip is bookkeeping,
> never a scope ruling.

`INTERROGATION-AGENDA.md:62-68`

**F30.** The module's routing row in the authoring skill's assembly table — release-gates content goes to the region as one line, detail to the ledger.

> | `release-gates` | Always offered (a deployed/operated target class — PO-D1); content from the always-interrogated deployment dimension | Region: one summary line + pointer; detail in the ledger |

`plugins/mochiko/skills/authoring-constitution/SKILL.md:232`

**F31.** The corresponding (bracketed = conditional) region line in the surfaces template.

> [- Release gates: [one-line summary] — detail in the ledger <!-- GI-XXX -->]

`plugins/mochiko/templates/governance-surfaces-template.md:59`

## Area D — RUNBOOK

**F32.** `grep -rni "runbook" plugins/ .mochiko/` returns **4 hits under `plugins/`** and 7 under `.mochiko/`. The complete plugin set is: `templates/constitution-modules/knowledge-management.md:8`, `:60`, and `INTERROGATION-AGENDA.md:34`, `:44`. It appears in **zero** commands, **zero** agents, **zero** skills, and **zero** templates other than the KM module.

**F33.** Elective declaration #1 (module header).

> Adopted as CORE + ELECTIVES: the core bundle is adopted or declined WHOLE (a project for
> which the core feels heavy declines the module, not a fragment); the electives
> (`CHANGELOG.md`, `RUNBOOK.md`) are per-doc opt-in, elicited by project type. The ruling is
> recorded in the synthesis either way; a recorded decline is durable — amend runs never
> re-offer a ruled module.

`plugins/mochiko/templates/constitution-modules/knowledge-management.md:6-10`

**F34.** Elective declaration #2 (module body) — the entire body treatment of RUNBOOK, two lines.

> **Electives** (per-doc opt-in at setup, elicited by project type): `CHANGELOG.md`
> (release-shaped projects), `RUNBOOK.md` (deployed services). A recorded decline is durable.

`knowledge-management.md:59-60`

**F35.** ABSENCE — RUNBOOK has **no document contract, no read-job, no writer moment, no carrier**. The module's core table (`knowledge-management.md:39-47`) has one row per core doc with `| Artifact | Read-job | Writer moment · carrier |`; RUNBOOK is not in it. The `### Document contracts` section (`:62`) covers ROADMAP, BACKLOG, the decisions layer, ARCHITECTURE, GLOSSARY — grep for `RUNBOOK` inside the file returns only lines 8 and 60 (F32). The admission rule that governs core entry:

> **Admission rule:** a doc enters this module only with a named
> **read-job**, a **writer moment**, and a **carrier** — no carrier, no scaffold.

`knowledge-management.md:34-35`

**F36.** Elective declaration #3 (agenda) — names the eliciting condition and the re-ruling date.

> **electives** (`CHANGELOG.md` for release-shaped projects, `RUNBOOK.md` for deployed services)
> are per-doc opt-ins elicited by project type; each elective ruling is recorded and durable
> (re-ruled 2026-07-25, deliberately superseding the prior no-inner-menu clause).

`INTERROGATION-AGENDA.md:44-46`

**F37.** ABSENCE — the project-pinned KM copy in this repo contains zero occurrences of `RUNBOOK` or `CHANGELOG`: `grep -n "RUNBOOK\|CHANGELOG" .mochiko/memory/knowledge-management.md` → no output. (The pinned copy carries Document-contracts + Landing-ritual + Invariants; electives are not pinned.)

**F38.** RUNBOOK's origin ruling and its recorded confidence mark.

> **CHANGELOG.md and RUNBOOK.md stay elective**, elicited at setup by project type (releases / deployed service).

`.mochiko/brainstorms/operating-docs-maintenance/record.md:258`

> RUNBOOK-elective, risk-register-drop, README-exclusion each `Confident` (ruled with the table).

`.mochiko/brainstorms/operating-docs-maintenance/record.md:262`

**F39.** ABSENCE — no grader anywhere checks a RUNBOOK. `validation-constitution` checks "core + exactly the adopted modules" (F97); the KM module ships no validator checklist fragment naming RUNBOOK (F32 shows the only two body mentions).

## Area E — SLOs and NFRs

**F40.** ABSENCE — **"SLO" appears zero times in `plugins/`.** `grep -rnE "\bSLOs?\b" plugins/` → count 0. `grep -rniE "service.level|service-level" plugins/` → no output. The term exists only in `.mochiko/` planning prose: `production-only-focus/record.md:142`, `BACKLOG.md:225`, `brainstorms/index.md:14`, `ops-observability-hardening/record.md:4`.

**F41.** "SLA" appears three times in `plugins/`, all as an NFR *source* label, never as an authored artifact class.

> | NFR-002 | availability | 99.9% monthly | Business SLA commitment |

`plugins/mochiko/skills/authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:278`

> | Source | Yes | Free text | On the statement line; business requirement, SLA, or stakeholder justifying the target |

`ARTIFACT-TEMPLATES.md:300`

> 3. **Non-Functional Requirements** — Measurable quality attributes: performance targets, availability SLAs, scalability thresholds, security requirements—each with a specific numeric target, measurement method, and source justification.

`plugins/mochiko/agents/technical-analyst.md:88`

**F42.** The NFR quality bar (skill level).

> ### 3. Non-Functional Requirements (nfrs.md) -- NFR-XXX
>
> Define measurable quality attributes. Every NFR has a numeric target. Field schema in ARTIFACT-TEMPLATES.md.
>
> **"Fast" is not a requirement.** "p95 response time < 200ms under 1000 concurrent users, measured by APM" is.
>
> **No exceptions:** Not for "standard" performance expectations. Not for "obvious" availability targets. Every NFR gets a number, a measurement method, and a source — no deferrals to "later during design."

`plugins/mochiko/skills/authoring-technical-requirements/SKILL.md:84-90`

**F43.** The full NFR field schema — the plumbing an SLO row would have to enter.

> | Field | Required | Format | Rules |
> |-------|----------|--------|-------|
> | ID | Yes | NFR-XXX | Sequential, three-digit padded, no gaps |
> | Title | Yes | Free text | Descriptive, concise |
> | Category | Yes | performance / availability / scalability / security / usability / maintainability | On the statement line; exactly one category |
> | Source | Yes | Free text | On the statement line; business requirement, SLA, or stakeholder justifying the target |
> | Requirement | Yes | One line | The quality attribute — the statement IS the description |
> | Target | Yes | Numeric | Specific, measurable threshold |
> | Measured | Yes | Compact line or short list | Tool, conditions, frequency of measurement |
> | Applies to | No | TR-XXX references | Which technical requirements this NFR constrains |

`ARTIFACT-TEMPLATES.md:295-304`

**F44.** The six NFR categories with their vague/measurable pairs — `availability` and `maintainability` are the ops-shaped ones.

> | Category | Bad (Vague) | Good (Measurable) |
> |----------|-------------|-------------------|
> | **performance** | "System must be fast" | "p95 response time < 200ms under 1000 concurrent users, measured by APM" |
> | **availability** | "System must be reliable" | "99.9% uptime measured monthly, excluding scheduled maintenance" |
> | **scalability** | "Must handle growth" | "Must support 10,000 concurrent users with linear resource scaling to 50,000" |
> | **security** | "Must be secure" | "Zero plaintext PII in logs; all data classified confidential+ encrypted AES-256-equivalent at rest" |
> | **usability** | "Must be easy to use" | "New users complete primary workflow within 3 minutes without documentation" |
> | **maintainability** | "Must be maintainable" | "Mean time to deploy hotfix < 2 hours from commit to production" |

`ARTIFACT-TEMPLATES.md:308-315`

**F45.** The `Measured:` line requirement — the closest existing analogue to an SLI.

> Every target's `Measured:` line names **what tool**, **under what conditions**, and **how frequently**. Compact example:
>
> ```markdown
> ## NFR-001: API Response Latency
>
> **performance · source:** FR-001 (real-time interaction expectation) — API responses feel instantaneous under production load.
>
> **Target:** p95 < 200ms, p99 < 500ms
> **Measured:** APM, rolling 24h windows, continuous — at 1,000 concurrent users (70% read / 20% write / 10% search); excludes maintenance windows and bulk imports
> **Applies to:** TR-001 · TR-005
> ```

`ARTIFACT-TEMPLATES.md:319-329`

**F46.** NFR traceability rules — NFRs trace up to a source and down to IP-XXX.

> **Mandatory links:**
> - TR -> FR (every technical requirement traces to business source)
> - NFR -> source (every quality attribute has a justification)
> - C -> D (constraints reference the decisions they shape; decisions reference constraints that shaped them)
> - C -> impact (every constraint identifies what it restricts)
> - C/NFR -> IP (constraints and NFRs with infrastructure implications reference IP-XXX items)

`plugins/mochiko/skills/authoring-technical-requirements/SKILL.md:108-113`

**F47.** The IP-XXX (infrastructure provisioning) type set — `monitoring` is a first-class provisioning type today.

> | Type | Scope |
> |------|-------|
> | compute | Containers, serverless, VMs, orchestration |
> | networking | DNS, load balancers, VPN, firewall rules |
> | storage | Databases, object storage, caches (provisioning, not schema) |
> | ci-cd | Build pipelines, deployment automation, environments |
> | monitoring | APM, logging, alerting, health checks |
> | security | IAM, certificates, secrets management |
> | environment-config | Environment variables, feature flags, config management |

`ARTIFACT-TEMPLATES.md:230-238` (section header `### Infrastructure Types` at `:228`)

**F48.** The IP-XXX field schema.

> | ID | Yes | IP-XXX | Sequential, three-digit padded |
> | Title | Yes | Free text | Descriptive, concise |
> | Type | Yes | compute / networking / storage / ci-cd / monitoring / security / environment-config | On the statement line; exactly one |
> | Source | Yes | C-XXX / NFR-XXX refs | On the statement line; constraints/NFRs that necessitate this |
> | Priority | Yes | MUST / SHOULD / MAY | RFC 2119, on the statement line |
> | Statement | Yes | One-to-two lines | WHAT to provision, not HOW — no separate Description paragraph |
> | Criteria | Yes | Bullet list, one line each | Independently verifiable |
> | Deps | No | IP-XXX refs | Other infra items this depends on |

`ARTIFACT-TEMPLATES.md:218-226`

**F49.** The rule that forces IP rows off constraints.

> **Every constraint that implies platform work gets an IP-XXX item.** Constraints document boundaries; IP-XXX items document what those boundaries require operationally.

`authoring-technical-requirements/SKILL.md:82`

**F50.** Downstream verification of NFRs, side 1 — the completeness reviewer (`review-plan-artifacts`) grades measurability and IP coverage, not achievement.

> | NFR measurability | Does every NFR have a specific, measurable target? | Critical |
> | NFR measurement method | Is the measurement approach defined? | Critical |
> | NFR source tracing | Do NFR sources trace to valid TRs or business requirements? | Important |

`plugins/mochiko/skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:70-72`

> | IP-NFR coverage | Do NFRs requiring platform infrastructure (availability, scalability) have corresponding IP-XXX items? | Critical |

`ARTIFACT-CHECKLISTS.md:58`

**F51.** Downstream verification of NFRs, side 2 — the feasibility reviewer grades whether targets can be met.

> | 6 | **Constraint ↔ design buildability** | Are the design artifacts buildable/deployable given the constraints and captured infrastructure? | constraints / infrastructure ↔ data-model / contracts |

`plugins/mochiko/skills/review-feasibility/SKILL.md:43`

> | **Constraint ↔ topology** | Is the topology buildable/deployable under the constraints and captured `IP-XXX`? (a shape needing a managed queue the constraints forbid and no `IP-XXX` provisions) | constraints / IP ↔ architecture |

`review-feasibility/SKILL.md:57`

**F52.** The reviewer's own boundary line — measurement is graded, achievement is routed to feasibility.

> - Can each NFR target actually be measured with available tooling?

`ARTIFACT-CHECKLISTS.md:90`

> | Measurability | testable TR criteria; NFR measurable target present; measurement method defined | — |
> | **Contradiction** (do artifacts conflict?) | — | **TR ↔ constraint contradictions; NFR ↔ constraint conflicts; NFR ↔ NFR impossibilities** |
> | **Buildability** (can it be built / met?) | — | **NFR-design feasibility (can the design meet the NFR targets?); constraint-design buildability (can the design satisfy the constraints?); integration failure modes realistic vs aspirational** |

`ARTIFACT-CHECKLISTS.md:332,335-336`

**F53.** ABSENCE — **no NFR reaches a `**TEST:**` task or the qa seat.** `review-task-artifacts`' checklist (`PHASE-CHECKLISTS.md:62-76`) contains rows for cycle coverage, TDD structure, file paths, verification-task presence, task IDs, story labels, parallel markers, checkpoints, dependencies, IP-XXX task coverage, deployment cycle, brownfield markers — **no NFR row**. Grep for `NFR` in `plugins/mochiko/skills/review-task-artifacts/` and `plugins/mochiko/skills/patterns-vertical-tdd/` returns no checklist row binding an NFR target to a verification task.

**F54.** The requirements-layer script actively flags uptime language in success criteria as a *defect* (an outcome-focus check), i.e. availability language is pushed out of SC-XXX.

> ```python
>     # Patterns that suggest technical metrics instead of outcomes
>     technical_patterns = [
>         r'\d+\s*ms\b',  # milliseconds
>         r'\d+%\s*(cpu|memory|coverage)',  # technical percentages
>         r'uptime',
>         r'error rate.*\d',  # numeric error rates (vs "decreased errors")
>         r'requests?\s*per\s*second',
>         r'concurrent\s*(users?|connections?)\s*>\s*\d',
>     ]
> ```

`plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py:173-181`

## Area F — Deployment view

**F55.** The deployment view's trigger, verbatim, and its IP-XXX keying (skill side).

> ### 4. Deployment view — conditional
>
> A deployment view (runtime/infra placement) **only when the feature changes deployment reality**.
> **Trigger:** the feature carries `IP-XXX` infrastructure-provisioning rows. No `IP-XXX` → omit it,
> and record the omission in one line rather than shipping an empty section.

`plugins/mochiko/skills/patterns-system-design/SKILL.md:69-73`

**F56.** The same trigger, conventions side, with what the view contains.

> ## Deployment view — conditional
>
> Author **only when the feature carries `IP-XXX` provisioning rows** (it changes deployment reality).
> A flowchart with runtime/infra boundaries as subgraphs:
>
> ```mermaid
> flowchart TB
>   subgraph aws[AWS eu-west-1]
>     subgraph ecs[ECS cluster]
>       api["Profile API task"]
>       worker["Avatar Worker task"]:::new
>     end
>     sqs[("SQS queue")]:::new
>     s3[("S3 bucket")]
>   end
>   api --> sqs --> worker --> s3
>   classDef new stroke:#2e7d32,stroke-width:3px;
> ```
>
> No `IP-XXX` rows → omit the section and record the omission in one line
> (`no deployment change — no IP-XXX rows`).

`plugins/mochiko/skills/patterns-system-design/references/DIAGRAM-CONVENTIONS.md:94-114`

**F57.** Its slot in the `architecture.md` template — one conditional section, no sub-structure.

> ## Deployment  *(conditional — only when IP-XXX rows exist)*
>
> {runtime/infra placement, or one line: "no deployment change — no IP-XXX rows"}

`patterns-system-design/SKILL.md:135-137`

**F58.** The producer's own quality-checklist row.

> - [ ] The deployment view is present iff IP-XXX rows exist (else its absence is recorded)

`patterns-system-design/SKILL.md:151`

**F59.** The grader's mirror row — Minor severity.

> | Deployment-view conditionality | If `IP-XXX` rows exist, is the deployment view present? If none, is its absence recorded (not a stub)? | Minor |

`plugins/mochiko/skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:110`

**F60.** `commands/plan.md` names the artifact set and the IP-XXX class but carries no deployment-view language of its own.

> - **Artifacts**, under `.mochiko/specs/<feature>/` (slice-scoped: `plan.md`, `architecture.md`,
> …
>   `requirements.md` (FR→TR) · `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX, with a
>   designated **structural-decisions section** the architect owns) · `nfrs.md` (NFR-XXX) ·

`plugins/mochiko/commands/plan.md:116,119-120` — `grep -n "deployment" plugins/mochiko/commands/plan.md` returns no hits.

**F61.** The task-layer deployment rows exist but are explicitly **dormant/parked**.

> | IP-XXX task coverage — *dormant/parked* | Does every IP-XXX item have at least one corresponding task? | Critical |
> | Deployment cycle — *dormant/parked* | Is there a deployment/CI/CD cycle if IP-XXX items require it? | Important |

`plugins/mochiko/skills/review-task-artifacts/references/PHASE-CHECKLISTS.md:74-75`

**F62.** The dormancy declaration, verbatim.

> > The IP-XXX / platform-app-ordering / deployment-cycle / `[EXTEND]`·`[MODIFY]` brownfield /
> > constraint-task-traceability checks are **present but dormant** — they belong to the
> > roadmap/brownfield track, deferred for the core tasks loop, and activate in lock-step with the
> > producer's parked side. Sequencing of which artifact is reviewed when is the lead's call, not this
> > skill's.

`plugins/mochiko/skills/review-task-artifacts/SKILL.md:75-79`

## Area G — Incident response

**F63.** ABSENCE — `grep -rni "incident" plugins/` returns exactly **2 hits, both rhetorical prose**, neither a mechanism:

> Verification testing exists to catch failures before they reach production. Every shortcut in this process is a potential production incident waiting to happen.

`plugins/mochiko/skills/testing-end-user/SKILL.md:14`

> If you're skipping a step because you "know" the result, you're not verifying — you're assuming. Assumptions are the source of production incidents.

`plugins/mochiko/skills/executing-tdd-cycle/references/TDD-ANTI-RATIONALIZATION.md:19`

**F64.** ABSENCE — related incident vocabulary, all zero under `plugins/`: `grep -rniF` counts for `"incident response"` = 0, `"incident-response"` = 0, `"postmortem"` = 0, `"post-mortem"` = 0, `"MTTR"` = 0, `"on-call"` = 0, `"oncall"` = 0, `"paging"` = 0. `"mean time to"` = 2 (both are the F5 rationale line "reduces mean time to resolution" and the F44 maintainability example "Mean time to deploy hotfix < 2 hours"). `"pager"` = 2, both inside a worked questioning example (`analysis-iterative/references/ADAPTIVE-EXAMPLES.md:57,61` — "critical alerts go through a dedicated channel (PagerDuty, SMS)"), i.e. sample dialogue for a hypothetical product, not mochiko machinery.

## Area H — Release health

**F65.** ABSENCE — `"canary"` = **0 hits under `plugins/`**. Its 3 repo hits are all in the pending security-depth record (`.mochiko/brainstorms/security-depth-scoping/record.md:494`, `:556`, `:626`) — see F84.

**F66.** ABSENCE — `"error budget"` = **0 hits** anywhere in the repo (`grep -rni "error budget" plugins/ .mochiko/` → no output).

**F67.** ABSENCE — `"blue-green"` / `"blue/green"` = 0 hits under `plugins/`.

**F68.** `"rollout"` = 1 hit under `plugins/`, inside a worked ADR example.

> - Migration path: Gradual rollout with feature flag

`plugins/mochiko/skills/patterns-technical-decisions/references/DECISION-RECORD.md:168`

**F69.** `"rollback"` = 6 hits under `plugins/`, **all inside release-gates.md (5) plus the agenda row (1)** — F24, F25, F26. No rollback surface exists outside the module.

**F70.** `"feature flag"` = 4 hits under `plugins/`, none an obligation: the F68 ADR example; `ARTIFACT-TEMPLATES.md:238` (the `environment-config` IP type, F47); `EMERGENT-CEILING-PATTERNS.md:22` (a brownfield-pattern example — "Configuration | Strongly-typed options, feature flags"); `ADAPTIVE-EXAMPLES.md:242` (sample dialogue).

**F71.** `"health check"` / `"healthcheck"` / `"/health"` = 12 hits under `plugins/`. The load-bearing ones are F3, F5, F7, F13, F47 plus the per-type translation note (F11) and two TDD examples:

> - "Server starts and responds to health check"

`plugins/mochiko/skills/patterns-vertical-tdd/references/CYCLE-STRUCTURE.md:211`

> - [ ] **T4.8**: **TEST:** - API server responds to health check
> …
>   - **Action**: `sleep 2 && curl -s localhost:3000/health`

`plugins/mochiko/skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md:80,83`

**F72.** `"uptime"` = 2 hits under `plugins/`: the F44 availability example and the F54 anti-pattern regex. `"availability"` = 9 hits, enumerated at F41/F42/F44/F50/F51 plus:

> - **Constraint-to-infrastructure tracing** — When a constraint says "must deploy on AWS ECS" or an NFR requires "99.9% availability," you identify the infrastructure provisioning that makes it achievable: container orchestration, load balancers, health checks, deployment pipelines, monitoring.

`plugins/mochiko/agents/technical-analyst.md:135`

**F73.** `"alerting"` appears exactly once as a mechanism — inside the `monitoring` IP type (F47). `"alert"`'s other 11 hits are: a sequence-diagram note (`DIAGRAM-CONVENTIONS.md:82` — "after N retries → dead-letter, alert"), 8 lines of sample dialogue in `ADAPTIVE-EXAMPLES.md`, and one data-sensitivity row (`patterns-entity-modeling/SKILL.md:189` — "All access logged + real-time anomaly alerts").

**F74.** `"staging"` = 4 hits: 2 in release-gates (F23), 2 in an OpenAPI server-block template (`patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml:41-42`).

## Area I — The PO mandate

**F75.** F32 of the production-only fact map, verbatim (the electives map).

> **F32 ("the team deploys/operates" is exactly what is conditional today).** The ruled boundary assumes an operated, deployed product; in the library as built that assumption is elicited/gated, not assumed: (a) deployment-and-release is interrogation dimension 8, prunable under the low-tier license (example skip "dimension 8 (deployment) skipped: poc tier", `templates/governance-intent-template.md:62`); (b) FLOOR-OBS is "out" at poc, "offer" at internal, only "default-in" at production/regulated (`catalog/universal-floor.md:76-89`); (c) the `release-gates` constitution module attaches only "when the deployment dimension elicited a real release process" (`templates/constitution-modules/release-gates.md:4-6`); (d) RUNBOOK is a per-doc elective offered only "for deployed services" and CHANGELOG only "for release-shaped projects" (`INTERROGATION-AGENDA.md:37`). A production-only stance taking deploy/operate as given would make deployment-reality, observability, release gates, and the RUNBOOK elective unconditional rather than tier/type-gated.

`.mochiko/brainstorms/production-only-focus/record.md:105`

> **NOTE ON F75's currency:** all four of F32's cited line numbers are pre-v0.36.0 and three of its four gating claims have since been closed by the narrowing build — (a) closed by F29's no-pruning-license; (b) closed by F13 (FLOOR-OBS is now unconditionally floor-asserted, no tier rows); (c) closed by F22 ("always offered"). Only **(d) RUNBOOK remains an elective**, per F32–F37.

**F76.** The PO-D5 ruling text on Tier-I ops, verbatim.

> - **Tier I — rides with the narrowing (identity-critical):** (1) **Security depth** — threat modeling at plan time, security requirements with teeth, blocking SAST + dependency-vuln gates, a security lens in the validator set; first-in-line, scoped in its own follow-on session, not built here. (2) **Operations & observability hardening** — SLOs as first-class NFRs, RUNBOOK promoted elective→asserted, incident-response basics, release-health expectations per product kind. (3) **Shelf translation tables for the in-scope kinds** — frontend first, then mobile, then desktop; the narrowing's own load-bearing prerequisite (map F30).

`.mochiko/brainstorms/production-only-focus/record.md:142`

**F77.** PO-D5's Tier-II row — the adjacent items the ops session must not absorb.

> - **Tier II — queued immediately behind, own scoping sessions, production-only frame attached:** (4) **IaC/deployment engineering, staged** — release gates + environment discipline asserted first; infrastructure-code authoring second (new artifact class, map F23). (5) **Data lifecycle** — schema-migration discipline, backup/restore verification, retention riding DS-XXX. (6) **Reliability & resilience** — timeout/retry/circuit-breaker cards, perf/load verification keyed to existing NFR targets.

`production-only-focus/record.md:143` — note **"release gates + environment discipline asserted"** is explicitly ruled Tier-II, not Tier-I.

**F78.** The BACKLOG item, verbatim and complete.

> - [ ] **Ops & observability hardening (Tier I)** (2026-07-30, PO-D5) — SLOs as first-class
>   NFRs, RUNBOOK elective→asserted, incident-response basics, release-health expectations per
>   product kind; mostly promoting existing electives to asserted (map F32).

`BACKLOG.md:225-227`

**F79.** The neighbouring Tier-II BACKLOG item, for the same fence.

> - [ ] **IaC / deployment engineering — staged (Tier II)** (2026-07-30, PO-D5) — stage 1:
>   release gates + environment discipline asserted; stage 2: infrastructure-code authoring (new
>   artifact class, map F23). Own scoping session; data lifecycle + reliability/resilience ride
>   Tier II behind it.

`BACKLOG.md:231-234`

**F80.** ROADMAP's Next row naming the sequencing.

> - Security-depth build (Tier I) — scoped 2026-07-30 (SD-D1–D6, verify-CLEAN record; trail); then ops-hardening + shelf-build scoping → [BACKLOG](BACKLOG.md#production-only-narrowing)

`ROADMAP.md:29`

## Area J — Security-build collision surface

**F81.** The security session's scope fence, verbatim — runtime ops is explicitly assigned to this session.

> ### Scope fences (out of this build, explicitly)
>
> DAST / pentest workflows (zero footprint today, F79; later candidate once SAST is live) ·
> per-regime compliance content (mint-driven per the PO ruling; the area registry doesn't change
> that) · type-specific area expressions (CSP/XSS, keychain, code-signing — ride the Tier-I shelf
> builds, F99; the registry marks them as shelf content) · runtime security ops (WAF, incident
> response, key-rotation execution — the ops-hardening item's territory).

`.mochiko/brainstorms/security-depth-scoping/record.md:529-535`

**F82.** The one seam the security session pulled *back* across that fence.

> **S15 fold (review):** the config/crypto areas carry a **rotatability obligation** — secrets
> rotatable and tokens revocable without redeploy is a design-time SEC prompt, not ops-fenced;
> key-rotation *execution* stays with ops-hardening.

`security-depth-scoping/record.md:490-492`

**F83.** The coverage ledger's home — the security build claims a new section inside the governance ledger and a new `validation-constitution` check.

> **S9 fold (review; the N/A half user-ruled — U6):** coverage's home and scope: the
> **app-level coverage ledger lives in the governance ledger** (setup-owned;
> `validation-constitution` gains the check), graded **once per app** and updated at amend;
> per-feature `review-plan-artifacts` rows check only the areas the feature touches. **A
> shelf-pending area is a recorded known-gap with a Tier-I pointer — never "N/A", never
> silently "addressed"** — the ledger stays honest for the three unseeded product types.

`security-depth-scoping/record.md:479-484`

**F84.** The gate-canary item — every blocking gate demonstrated failing once.

> **S20 fold (review):** every blocking gate is **demonstrated failing once** — a known-bad
> canary at setup or first implement (a gate that cannot fail is not a gate); lands as a build
> item.

`security-depth-scoping/record.md:493-495`

**F85.** F94 of the security fact map — the quality-gate list has a consumer and no producer.

> **F94.** The quality-gate list qa executes has a named consumer and **no authored producer**: `testing-end-user/SKILL.md:139` reads `## Quality Gates` from `tasks.md` / build config from `plan.md`, and neither template contains either (F58).

`security-depth-scoping/record.md:282`

**F86.** F94's fix is claimed by the security build in two places.

> `testing-end-user` gains the gate-list wiring (F94) with the security gates in it; qa executes —
> exit-code determinism untouched.

`security-depth-scoping/record.md:509-510`

> `tasks-template` gains the `## Quality Gates` producer (F94) · checklist/lens extensions per D6

`security-depth-scoping/record.md:549`

**F87.** The security build's full surface list (the collision inventory), trimmed to the ops-adjacent lines.

> One new skill (`patterns-threat-modeling`-shaped: STRIDE sweep + trust-boundary conventions +
> the **security area registry** reference, consumed by `system-architect`) ·
> `patterns-system-design`'s DIAGRAM-CONVENTIONS gains trust-boundary marks ·
> `authoring-technical-requirements` gains SEC-XXX with full plumbing (+ the DS-XXX plumbing
> close, F95) · `patterns-api-contracts` gains response-classification + endpoint-auth-declared
> checks (F96/F97) · FLOOR-SEC's asserted row gains the SAST + no-hand-rolled-security lines,
> with the stack tooling map **and the enforcement clause + threshold** relocating to a
> universal home (closing both F100 clauses — the "closes F100" shorthand corrected per S7) ·
> dimension 6 elicits
> scanners (F66) · the governance quality-gates template models the security gates (F93) ·
> `tasks-template` gains the `## Quality Gates` producer (F94) · checklist/lens extensions per D6
> · the F19 defect fix (the `[PII]`-marker script contradiction + heading mismatch) folds in as
> the build's prerequisite stage.

`security-depth-scoping/record.md:539-551`

**F88.** The security build's BACKLOG item — its own fence line repeats "runtime ops".

> - [ ] **Security-depth build (Tier I)** (2026-07-30; rulings SD-D1–D6, record
>   `.mochiko/brainstorms/security-depth-scoping/record.md` — pair-reviewed, verify CLEAN round
>   3) — build surface (record: "Build surface" + its V4 extension): `patterns-threat-modeling`
>   skill + `SECURITY-AREAS.md` registry · trust-boundary diagram convention · SEC-XXX full
>   plumbing + DS-XXX close (F95) · FLOOR-SEC row edits (SAST high/critical · no-hand-rolled
>   line · tooling map + enforcement clause relocated universal) · semgrep floor rulesets
>   (governance-protected) · contracts checks (F96/F97) · quality-gates producer (F94) ·
>   coverage ledger + `validation-constitution` check · persona edits (keystone-checked) · spec
>   producer prompt · gate canaries · F19 fix as prerequisite · one command edit (`plan.md`
>   Bindings, shape-v5 audited). Fences: DAST/pentest · per-regime content · type-specific
>   expressions (shelves) · runtime ops.

`BACKLOG.md:214-224`

**F89.** VERIFIED — the security build is **not started**. `.mochiko/brainstorms/index.md:20` reads "**Landed:** DECISIONS.md rows SD-D1–D6 (2026-07-30) · BACKLOG: scoping item → trail, "Security-depth build" opened with the reviewed build surface · ROADMAP Next touch. Build not started." Corroborated: `plugins/mochiko/skills/patterns-threat-modeling/` does not exist (`ls plugins/mochiko/skills/` shows 30 skills, none named `patterns-threat-modeling`); `grep -n "Quality Gates" plugins/mochiko/templates/tasks-template.md plugins/mochiko/templates/plan-template.md` returns no output (F94's defect is live).

## Area K — Where asserted rows land

**F90.** The trace-key line — the four provenance tokens an asserted ops row would carry.

> **Trace**: GI-XXX (floor-asserted: CARD-ID | deck-kept: CARD-ID | minted | module: <module>-<obligation>)

`plugins/mochiko/templates/governance-surfaces-template.md:142`

**F91.** The D4 waivers table header and mechanics (ledger side).

> ## Waivers
>
> Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
> **Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
> validator FAIL.
>
> | Standard | Justification | Revisit trigger (optional) | Trace |
> |----------|---------------|----------------------------|-------|
> | [floor category / card / non-legal module obligation, or "None."] | [recorded reason] | [or "permanent (D4.1 pending)"] | GI-XXX |

`governance-surfaces-template.md:99-107`

**F92.** The same table, synthesis side.

> | GI-ID | Standard (floor category / card / non-legal module obligation) | Justification | Revisit trigger (optional) | Mark |
> |-------|---------------------------------------------------------------|---------------|---------------------------|------|
> | GI-0XX | [e.g. FLOOR-TEST coverage gate] | [recorded reason] | [or "permanent (D4.1 pending)"] | [Confident] |

`plugins/mochiko/templates/governance-intent-template.md:111-113`

**F93.** The governance ledger's location and section set.

> # Shape 3 — the governance ledger (`.mochiko/memory/governance-ledger.md`)
>
> Read by setup/amend runs and the validator only — never force-loaded into working sessions.

`governance-surfaces-template.md:89-91` — its sections, by `##` header: `## Waivers` (`:99`), `## Amendment policy` (`:109`), `## Exception registry` (`:117`), `[## Domain-dependency policy …]` (`:123`), `## Principles (Three-Part metadata, keyed by GI-ID)` (`:130`), `[## Evolution notes …]` (`:144`), `## Amendment log` (`:147`).

**F94.** The ledger's header stamp — where the floor level and modules are recorded.

> **Governance Floor:** production (asserted) · **Modules:** [attached compliance modules with strata, or "none"] · **Trace:** GI-001 (fact profile)
> **Version:** [X.Y.Z] (must match the region stamp)

`governance-surfaces-template.md:96-97`

**F95.** The CLAUDE.md region's quality-gates block — the only always-on gate surface.

> ### Quality gates
>
> - `[actual command]` MUST pass before merge <!-- GI-XXX -->
> - Coverage ≥ [asserted floor level, session-overridable]% on new code (`[actual command]`) <!-- GI-XXX -->

`governance-surfaces-template.md:48-51`

**F96.** The five surfaces of the governance set (the routing menu an ops row must land on).

>   1. A marked governance region in CLAUDE.md          (always-on; short-form only)
>   2. `paths`-scoped rules files under .claude/rules/mochiko/   (scope-on-touch)
>   3. Skill pointers                                    (procedure lives in skills)
>   4. The governance ledger at .mochiko/memory/governance-ledger.md  (setup/amend + validator only)
>   5. The trace summary manifest                        (the validator's grading surface)

`governance-surfaces-template.md:8-12`

**F97.** The module-selection table in the synthesis — where a `release-gates` or new ops module ruling is recorded, and the "declines are rulings too" rule.

> **Template modules** (the operating-docs layer — knowledge-management, layer-rules,
> release-gates), ruled in session; *compliance modules attach mechanically in the Fact profile
> above, never here.* **Declines are rulings too** (recorded, durable: amend runs offer only
> modules with *no* recorded ruling here; a recorded decline is never re-asked until the user
> reopens it). The validator checks core + exactly the adopted modules:

`governance-intent-template.md:117-121`

**F98.** The floor-expression table — where an ops row's per-type translation is recorded.

> Floor cards enter asserted — their rows record *expression* (type translation), never a level
> ruling; arbitrated cards (architecture-opinion and kin) record the user's ruling. **Dropped
> arbitrated cards are rulings too:**
>
> | GI-ID | Card | Layer | Ruling / Expression | Mark |
> |-------|------|-------|---------------------|------|
> | GI-0XX | FLOOR-SEC | floor-asserted | [expression shaped how, or "at floor level"] | [Confident] |

`governance-intent-template.md:84-90`

**F99.** The "real commands" table — dimension 6/8's output, the validator's placeholder bar.

> ## Real commands (dimension 6/8 → the validator's placeholder bar)
>
> | Purpose | Command | Source |
> |---------|---------|--------|
> | Lint | `[actual command]` | [detected / declared] |
> | Test | `[actual command]` | [detected / declared] |
> | [etc.] | | |

`governance-intent-template.md:74-80`

**F100.** The never-route-unselected rule.

> **Never route module content the synthesis didn't select** — an unselected module attached "to be
> safe" is unsanctioned selection. Each module's validator checklist fragment still applies to its
> routed content.

`plugins/mochiko/skills/authoring-constitution/SKILL.md:237-239`

## Area L — Waiver model

**F101.** D4, verbatim statement — governs whether a promoted-to-asserted ops row can be waived.

> **Statement:** Any asserted standard — the four floor categories included — can be waived with a recorded, auditable justification in the governance ledger. There is no unwaivable stratum **except the legally-mandated compliance obligations carved at D4.2** (S1 fold — verify-pass repair #1). This deliberately *loosens* today's `production` posture, which forbids floor waivers outright (map F6).

`.mochiko/brainstorms/production-only-focus/record.md:134`

**F102.** D4's confidence mark and its provenance — the ruling is `Contested`.

> **Provenance note:** the lead had stated the fact-translation model materially strengthens the opposite option (floor-unwaivable); the user chose B anyway — deliberate preference, held after the lead's one pressure-test — marked `Contested`.

`production-only-focus/record.md:136`

**F103.** D4.2, verbatim — the only unwaivable stratum.

> **D4.2 — Legal-mandate exception (S1 fold, review, user-ruled) — `Confident`.** Obligations that enter via a legally-mandated, fact-triggered compliance module (PCI, HIPAA, …) are **unwaivable**. A recorded permanent waiver of a legal control is not an honest escape valve — it is documented evidence of a knowing violation. The floor itself stays waivable per D4; only the legal-mandate stratum is exempt. (Both reviewers converged on this seam independently: DQ-3 + RI-1.)

`production-only-focus/record.md:138`

**F104.** D4.1 — the deferred expiry question.

> **D4.1 — Waiver expiry: `Deferred`.** The pressure-test offered mandatory expiry/review-by dates on floor waivers (temporary relief matching the shelf-immaturity rationale). User ruling: "keep the permanent waiver for now. This decision I will come to revisit later." Explicit revisit marker — permanent waivers stand until then.

`production-only-focus/record.md:137`

**F105.** The DECISIONS.md index row for D4.

> | 2026-07-30 | PO-D4 — waivers reach everything, recorded + auditable; legal-mandate module obligations unwaivable (D4.2); expiry deferred (D4.1) | ruled (`Contested`; expiry `Deferred`) | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |

`DECISIONS.md:25`

**F106.** PO-D7 — the waiver-as-on-ramp ruling that a "promote RUNBOOK to asserted" move would rely on.

> **Statement:** Early-stage customer-facing teams (an MVP, a 2-person team) are in scope and enter at the full asserted floor; recorded waivers (D4) are the honest, visible staged-adoption mechanism. Nothing replaces the retired `poc`/`internal` rungs.

`production-only-focus/record.md:157`

**F107.** The waiver-rulings step in the interrogation agenda (the session-time carrier).

> 5. **Waiver rulings** — for any asserted standard the user chooses to deviate from: record the
>    standard, the justification, and any revisit trigger (waivers are permanent pending the D4.1
>    revisit — an expiry is the user's option, never a default). Legal-mandate module obligations
>    are unwaivable (D4.2); contractual/non-legal module obligations are waivable like any other
>    standard (ruled 2026-07-30, `po-narrowing-build-scope`).

`INTERROGATION-AGENDA.md:114-118`

## Area M — Producers and verifiers

**F108.** ABSENCE — there is **no quality-gates skill**. The 30 skills under `plugins/mochiko/skills/` contain no gate-authoring skill; the gate *list* has a consumer (`testing-end-user`) and no producer (F85, F89). The consumer line:

> 1. **Identify quality gate commands** from the `## Quality Gates` section of `tasks.md` and/or the build configuration in `plan.md`.

`plugins/mochiko/skills/testing-end-user/SKILL.md:139`

**F109.** The gate execution semantics — deterministic, never a judgment call.

> When a verification run includes quality gates, execute them alongside `**TEST:**` task verification. Quality gates are command-based checks that **always auto-resolve** — they are deterministic ground truth, not a matter of judgment.

`testing-end-user/SKILL.md:135`

> Quality gate failures are surfaced through the verification report to the gate that consumes it, which evaluates them deterministically. (This exit-code determinism is ground truth; it MUST NOT be softened into an LLM judgment call.)

`testing-end-user/SKILL.md:159`

**F110.** The gates are explicitly the verifier's, never the producer's.

> - **Running the quality gates** (lint, build, test suite) or the final real-infrastructure verification that gates a cycle — that is the verifier's work (`testing-end-user`), never this skill's. This skill executes the failing-test / implementation / refactor tasks and runs their tests; the final verification gate belongs to the verifier.

`plugins/mochiko/skills/executing-tdd-cycle/SKILL.md:27`

**F111.** The qa-engineer's four outputs.

> 1. **Verification Reports** — Structured evidence of what passed, what failed, and why
> 2. **Quality Gate Results** — Deterministic pass/fail for lint, build, and test suites
> 3. **Checkpoint Presentations** — Evidence summaries with actionable recommendations for human approval
> 4. **Evidence Artifacts** — Console output, timing data, file state captures—the raw proof

`plugins/mochiko/agents/qa-engineer.md:60-63`

**F112.** The qa persona's evidence standard.

> - **Evidence-first** — No assertion is "passed" without captured proof. Console output, file checks, HTTP responses—record everything.
> - **Reproducible** — Every verification can be re-run. Capture the exact commands, environment state, and timing.
> - **Honest** — Report what you observe, not what you expect. A test that "should" pass but didn't is a failure, full stop.

`qa-engineer.md:67-69`

**F113.** qa's single mounted skill — the seat carries no ops-specific procedure.

> - **`mochiko:testing-end-user`** — executing and reporting verification.

`qa-engineer.md:46`

**F114.** implement.md's verifier seat row — the whole ops-verification surface at command level.

> | verifier | `qa-engineer` × `testing-end-user` | verifies each cycle, then the whole implementation, against real infrastructure — executes the cycle's `**TEST:**` tasks, runs the quality gates, captures evidence → verification report + a checkpoint recommendation; never implements | cold at the first cycle verification, standing after | peer-edged with the producer for cycle hand-offs; the endgame is lead-routed |

`plugins/mochiko/commands/implement.md:40`

**F115.** implement's done / not-done conditions naming gates.

> **Not done:** an unchecked task, or a cycle with no report · a failing quality gate · a cycle or the

`implement.md:30`

> - **Fact route:** real infrastructure — executed `**TEST:**` tasks and quality-gate exit codes;

`implement.md:126`

**F116.** The `**TEST:**` task-class definition — the grammar an ops verification would have to fit.

> The final task of each cycle (typically TN.4, or the last task in longer cycles) is the **Verification** task. This is NOT just another automated test—it is the gate that ensures the cycle delivers real, working functionality.
>
> ## What Verification MUST Include
>
> 1. **Real Infrastructure**: Use real file systems, real databases, real APIs—NOT mocks
> 2. **Tangible Output**: Something observable (console output, file, response, UI state)
> 3. **Explicit Steps**: Concrete commands or actions to perform
> 4. **Observable Outcome**: What should be observed when it works

`plugins/mochiko/skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md:9-16`

**F117.** The `**TEST:**` field skeleton and its ownership split.

> ```markdown
> - [ ] **TN.X**: **TEST:** - {Description}
>   - **Setup**: {Prerequisites} (optional)
>   - **Action**: {Command or instruction}
>   - **Assert**: {Expected outcome}
>   - **Capture**: {console, screenshot, logs} (optional)
> ```

`TEST-GRAMMAR.md:22-28`

> The canonical `**TEST:**` verification-task grammar — marker set, field skeleton, action
> modifiers, assert patterns, worked examples, and legacy support. Owned by
> `patterns-vertical-tdd` (authored at design time into `tasks.md`); consumed at runtime by
> `testing-end-user`, which owns the execution/evaluation semantics.

`TEST-GRAMMAR.md:3-6`

**F118.** The runtime classification that decides auto-approve vs human checkpoint.

> | Classification | Criteria | Execution |
> |----------------|----------|-----------|
> | **CLI** | Backtick commands + measurable asserts | May auto-approve if 100% pass |
> | **GUI** | UI actions, screenshot captures | Human checkpoint |
> | **SUBJECTIVE** | Qualitative terms (looks, feels) | Human checkpoint |

`TEST-GRAMMAR.md:32-36`

**F119.** The brownfield analysis observability probes — the three checks a codebase is assessed on today.

> #### Observability — status indicators
>
> | Check | How to Detect | Status Values |
> |-------|---------------|---------------|
> | Structured logging | Logger config (winston, pino, structlog, logrus) | present/partial/absent |
> | Correlation IDs | Request ID middleware, trace ID patterns | present/partial/absent |
> | No PII in logs | Log sanitization, no email/password in log statements | present/partial/absent |

`plugins/mochiko/skills/analysis-codebase/SKILL.md:107-113`

**F120.** Their mirror in the analysis template — note there is **no health-check row** despite F5 asserting one.

> | Observability | Structured logging | {{status}} | {{evidence}} |
> | Observability | Correlation IDs | {{status}} | {{evidence}} |
> | Observability | No PII in logs | {{status}} | {{evidence}} |

`plugins/mochiko/templates/codebase-analysis-template.md:114-116`

**F121.** The brownfield gap-status carrier (evolution-notes module) — where an absent observability category lands.

> | Category | Status | Response |
> |----------|--------|----------|
> | Security | [present / partial / absent] | [codified existing pattern / MUST-implement → GAP-XXX / waived (see Waivers)] |
> | Testing | [status] | [response] |
> | Error Handling | [status] | [response] |
> | Observability | [status] | [response] |

`plugins/mochiko/templates/constitution-modules/evolution-notes.md:18-23`

**F122.** The constitution validator's floor check — waiver-or-principle, neither fails.

> - [ ] Every Essential Floor category (Security, Testing, Error Handling, Observability) has a principle **or a recorded waiver** — neither is a FAIL

`plugins/mochiko/skills/validation-constitution/references/QUALITY-CHECKLIST.md:35`

**F123.** The technical-analyst persona's ops-tracing line — the only persona sentence connecting an NFR to operational provisioning.

> - **Infrastructure-aware** — Every constraint implying platform provisioning, deployment, or environment configuration carries a corresponding infrastructure-provisioning requirement. Constraints have operational consequences.

`plugins/mochiko/agents/technical-analyst.md:105`

**F124.** The system-architect persona's operability line.

> - **Buildability** — a shape is only a design if it can be built, deployed, and operated under the

`plugins/mochiko/agents/system-architect.md:86`

**F125.** ABSENCE — `review-specifications`' gap categories contain no operational/availability class.

> | Category | Example Questions |
> …
> | **User expectations** | "What should users see when...?" |
> | **Business rules** | "Is X allowed? Under what conditions?" |
> | **Scope boundaries** | "Is Y in scope for this feature?" |
> | **Success/failure states** | "What happens if the user...?" |
> | **Permissions** | "Who can do X? Who cannot?" |

`plugins/mochiko/skills/review-specifications/SKILL.md:52-58` — and the skill actively re-frames ops questions as product questions: "| "What's the retry policy for failed API calls?" | "How long should users wait before seeing an error?" |" (`:36`).

## Area N — OTel disambiguation

**F126.** `grep -rni "otel\|opentelemetry"` across the repo returns **zero hits under `plugins/`** — every hit is in `.mochiko/` or the operating docs, and every one is about **mochiko's own run telemetry** (token/cost measurement of workflow runs), not product observability.

**F127.** The originating fact-check heading — explicitly about Claude Code's own usage monitoring.

> ### F-d. Claude Code OpenTelemetry (user-commissioned at the S1 ruling)
>
> OTel fact-check, from the official doc **https://code.claude.com/docs/en/monitoring-usage** (quoted verbatim). Bottom line up front: **this materially upgrades my earlier hooks/statusline settlement** — Claude Code natively emits per-run, per-type, AND per-subagent token+cost data by config alone.

`.mochiko/brainstorms/workflow-token-reduction/record.md:216-218`

**F128.** The probe's payload — token and cost fields, not application telemetry.

> - **`claude_code.api_request`** — *"Logged for each API request to Claude"* — carries `input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_creation_tokens`, `cost_usd`, `cost_usd_micros`, `model`, `duration_ms`, `request_id`, `query_source`, `agent.name`, `skill.name`.

`workflow-token-reduction/record.md:241`

**F129.** The open BACKLOG item — a one-shot probe of mochiko's own runs.

> - [ ] **D2 upgrade — the one-shot OTel probe** — enable documented config in a dogfood run;

`BACKLOG.md:191`

**F130.** The ROADMAP Next row, same sense.

> - Token epic: D5 sizing-gate generalization + the one-shot OTel probe (2026-07-23) → [BACKLOG](BACKLOG.md#token-reduction-epic)

`ROADMAP.md:26`

**F131.** The DECISIONS row and the strip note, same sense (run-cost measurement path).

> the shape's run-cost element dropped by supersession (retires shape v3's manual-baseline carrier; OTel probe remains the cost path)

`DECISIONS.md:31`

> - **The token epic's OTel probe remains the future cost-measurement path** — the capability is

`.mochiko/strips/command-shape.md:163`

**F132.** The per-seat extension, same sense (seat-level token attribution).

> **D4** per-seat measurement rides the epic's OTel probe (per-seat attribution, the teammate-compaction/idle/cache unknowns as probe questions, a manual per-seat `/usage` reading at gates)

`.mochiko/archive/ROADMAP.md:91`

**F133.** NEUTRAL — the ESSENTIAL-FLOOR APM line is the product-observability sense, and names no OTel-family tooling:

> - **APM tools**: Name specific tools if detected (e.g., Application Insights, Datadog, New Relic)

`ESSENTIAL-FLOOR.md:41` — i.e. the two senses never collide in any single file.

---

## Coverage notes

**Files read in full:** `ESSENTIAL-FLOOR.md` · `catalog/README.md` · `catalog/universal-floor.md` · `catalog/backend-service.md` · `INTERROGATION-AGENDA.md` · `constitution-modules/release-gates.md` · `constitution-modules/evolution-notes.md` · `governance-surfaces-template.md` · `authoring-technical-requirements/SKILL.md`.

**Files read in part (line ranges cited):** `COMPLIANCE-MODULES.md:1-70` · `constitution-modules/knowledge-management.md:1-80` · `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:160-340` · `patterns-system-design/SKILL.md:60-160` · `patterns-system-design/references/DIAGRAM-CONVENTIONS.md:88-120` · `governance-intent-template.md:40-145` · `qa-engineer.md:40-100` · `patterns-vertical-tdd/references/TEST-GRAMMAR.md:1-60` · `analysis-codebase/SKILL.md:95-130` · `codebase-analysis-template.md:100-125` · `review-task-artifacts/references/PHASE-CHECKLISTS.md:60-85` · `review-task-artifacts/SKILL.md:68-82` · `authoring-constitution/SKILL.md:220-245` · `check-artifacts.py:75-100` · `validate-requirements.py:165-190` · `production-only-focus/record.md:100-160` · `security-depth-scoping/record.md:475-560` · `BACKLOG.md:205-245` · `ROADMAP.md:1-45`.

**Greps run** (all from repo root; `plugins/` and `.mochiko/` unless noted):
`runbook` · `SLO\b|SLOs|service level` · `\bSLOs?\b` (case-sensitive) · `service.level|service-level` · `\bSLAs?\b` · `canary` · `rollout` · `rollback` · `feature flag|feature-flag` · `error budget` · `on-call|oncall` · `health check|healthcheck|/health` · `uptime` · `availability` · `incident` · `incident response` · `incident-response` · `postmortem` · `post-mortem` · `MTTR` · `mean time to` · `dashboard` · `alert` · `paging` · `pager` · `blue-green` · `blue/green` · `staging` · `smoke test` · `log retention` · `trace` · `span` · `metric` · `observab|monitoring|alerting|telemetry|structured log|APM\b` · `quality gate` · `deploy` · `backup|restore\b` · `disaster` · `otel|opentelemetry` · `deployment` (scoped to patterns-system-design + plan.md) · `PO-D4|PO-D5|PO-D2|PO-D7|D4.2` (DECISIONS.md) · `NFR|IP-XXX|Infrastructure|availability|latency|uptime|throughput` (ARTIFACT-TEMPLATES.md) · `RUNBOOK|CHANGELOG` (knowledge-management.md + `.mochiko/memory/knowledge-management.md`).

**Caveat on term counts:** `grep -rniF "SLO"` returns 24 under `plugins/` purely from substring matches inside *slot / slow / slog / slope*; the word-boundary count is **0** (F40). Any per-term count above was re-verified with `-E "\b…\b"` or by reading the hit lines.

**Could not read / does not exist:** `plugins/mochiko/skills/patterns-threat-modeling/` (not created — the security build has not started, F89) · `plugins/mochiko/skills/authoring-constitution/references/catalog/frontend.md`, `mobile.md`, `desktop.md` (planned shelves, F12) · `human-in-loop/` and `agent-skills-research/` submodules (removed 2026-07-21 per CLAUDE.md) · `.mochiko/memory/governance-ledger.md` (this repo has never run `/mochiko:setup`; the ledger is a template shape only, F93).

*(end of map — F1–F133, Areas A–N)*

## Decisions

### D1 — Session frame: governance-primary with narrow per-feature hooks — `Confident`

**Statement:** Ops depth lands primarily at the **governance layer** (app-level, setup-elicited: the ops-floor asserted-level extension *(amended at D5: the new FLOOR-OPS card, not FLOOR-OBS)*, release-gates strengthening, a RUNBOOK document contract, incident basics), with **per-feature hooks only where plumbing already exists** — the NFR `availability`/`maintainability` categories (F43–F44), the IP-XXX `monitoring` type (F47), the deployment view (F55–F57) — plus a **narrow close of the F53 gap** (no NFR ever reaches a `**TEST:**` task or the qa seat today; without that wiring, "SLOs as first-class NFRs" is paper). No security-parallel full weave: no new ops requirements class through plan artifacts, no standing ops lens rollout as a session premise (individual lens edits may still be ruled where a specific gap demands one). *(Fence restated at review fold M17: the fence D1 actually holds is **no new artifact class, no lens, no seat** — "only where plumbing already exists" was breached in-session by D4.2's landing fold, a new command-level mechanism ruled as an explicit, shape-audited exception.)*

**Options considered:** (A) governance layer only — leaves F53 open, SLO-NFRs unverifiable; (B) full weave, security-parallel — new design-time ops flow + lens at review sites; (C) governance-primary + narrow hooks — adopted.

**Rationale:** Security earned a full weave because security defects are born per-feature at design time (each feature adds attack surface). Ops obligations are mostly *standing* app-level infrastructure — one SLO set, one runbook, one alert route per app, not per feature — which is governance-shaped. The per-feature half already has homes (F43–F52: NFR categories, IP-XXX monitoring, deployment view, feasibility review); the defect is the named gaps in those homes, not a missing parallel structure. Steelman for B (recorded): F53 + F125 together mean ops has *no voice* anywhere in the per-feature pipeline — spec review actively re-frames ops questions into product questions — so narrow hooks bolted on without a lens may simply never fire. Held against it: close the named gaps at their existing homes first; a lens rollout is re-openable if dogfood shows the hooks silent.

**Headline map finding folded into the frame:** the BACKLOG premise "mostly promoting existing electives to asserted" is **false** — only RUNBOOK is a genuine elective→asserted promotion (F32–F39: two lines, no contract, no producer, no grader); SLO, incident-response, and release-health have zero plugin footprint (F40, F63–F68). The session is mostly minting, not promoting.

**Provenance:** lead-recommended, user-adopted ("go with recommendation") — adoption #1, streak watch open.

### D2 — Tier-II fence: split by nature — signals asserted now, process deferred — `Confident`

**Statement:** Release-**health signals** (post-deploy error rate vs baseline, crash-free sessions, and kin — *per product kind*) are observability content and get asserted via this session's ops-floor extension *(amended at D5: the FLOOR-OPS card)*. Release-**process** discipline (environments, cadence, the gate table, rollback execution) stays the always-offered, declinable `release-gates` module (F22) until Tier-II (IaC stage 1) asserts it, per the ruled PO-D5 boundary (F77). The overlap (the module's "staging soak / error rate" gate example, F23) resolves by the module **referencing** the asserted signals, never defining its own.

**Options considered:** (A) strict fence — expectations content only, all assertion waits for Tier-II; (B) pull release-gates offered→asserted forward now; (C) split by nature — adopted.

**Rationale:** The Tier boundary is a ruled decision — B re-litigates PO-D5 without new evidence, and Tier-II is "queued immediately behind" anyway. But the Tier-I mandate's own words place release-*health* here, and health signals are genuinely what-you-watch-after-a-deploy observability, not deployment engineering. Steelman for B (recorded): durable module declines (F97) recorded between now and Tier-II create a migration surface when assertion lands — accepted as Tier-II's problem, noted for its scoping session. Steelman for A (recorded): asserting signals partially overlaps the module — resolved by the reference-not-redefine rule in the statement.

**Provenance:** lead-recommended, user-adopted — adoption #2, streak watch running.

### D3 — SLO anatomy: app-level set, region-visible, feasibility-coupled — `Confident`

**Statement:** SLOs are **app-level standing commitments** — one set per app, never per-feature artifacts. (1) **Elicited at dimension 8** (deployment & release reality, which already owns Observability expression — F26); no new interrogation dimension. (2) **Landing** *(amended at review fold M5 — the region's short-form contract forbids a multi-line block: `governance-surfaces-template.md:26`, `authoring-constitution/SKILL.md:65`, and the release-gates one-line precedent `:232`)*: the CLAUDE.md governance region carries **one summary line + pointer**; the full SLO block (critical paths, targets, measurement sources) lives in a **`paths`-scoped rules file scoped to `.mochiko/specs/**`** (surface 2 of F96 — loads exactly when plan/spec work happens, part of the validator-checked surface set) + Three-Part metadata in the ledger *(M5's harness-fact rider — whether producer seats see the region — is moot under this landing)*; **the ops floor's asserted level gains** *(amended at D5: on the FLOOR-OPS card)* "SLOs defined for the app's critical paths · alerting on SLO breach" (waivable per D4, recorded). (3) **Per-feature relation:** feature NFR `availability`/`performance` rows cite the app SLO as `Source` (the field already accepts "SLA" — F41/F43); **`review-feasibility` gains one row** — feature NFR targets must not contradict the app SLO block (a feature p95 500ms under an app SLO of 200ms is a cross-artifact contradiction nobody catches today, F51–F52). *(Extended at review fold M10: `review-plan-artifacts` additionally gains an **SLO→NFR coverage row** — a feature touching an SLO-covered critical path authors the availability/performance NFR or records why not; contradiction detection alone never fires on the silent path, F54/F125.)* (4) **Numbers are the user's:** elicited and recorded, never invented — the FLOOR-TEST session-overridable-pre-seed posture (F15).

**Options considered:** (A) per-feature-only NFR strengthening — wrong altitude, SLOs aren't feature-scoped; (B) the adopted package; (C) B + SLO-breach wiring into implement verification — rejected: an SLO is measured in production over time, not at a cycle gate; build-time verifiability is instrumentation *existence*, which rides the F53 close (separate decision).

**Drift-risk note (adopted option)** *(relabeled at review fold M14 — this argued the adopted option's risk, not A's case)*: the app-level SLO block is a second NFR-like surface with block↔feature drift risk *(wording aligned to M5's landing at verify round 1, defect C)* — mitigated: the M10 coverage row + the feasibility row are the drift detectors. **Steelman recorded (for A — added at M14):** A's real case: the NFR layer already carries plumbing, grading, and traceability (F42–F52) while an app-level block is a second home; why it lost: SLOs are app-scoped standing commitments no per-feature artifact can carry, and the M5/M10 folds give the block a validator-graded home and a coverage detector.

**Provenance:** lead-recommended package, user-ratified — adoption #3, streak active (flagged to user at Q4).

### D4 — RUNBOOK promotion: floor obligation + KM scaffold; incident basics as runbook sections — `Confident`

**Statement:** (1) **Obligation/scaffold split:** the asserted obligation — *an operational runbook exists and stays current* — enters the **floor** (waivable per D4/PO-D4). It cannot live only inside the declinable adopt-whole KM module (F33): a team declining KM satisfies the floor obligation with its own runbook or a recorded waiver; the KM module's `RUNBOOK.md` is the scaffolded carrier when adopted. (2) **Document contract** (closes the F35 admission-rule violation): read-job = incident-time diagnosis by the responder; writer moments = setup scaffolds the skeleton + an **implement-landing fold with its own dual-key trigger** *(amended at review fold M2 — the original "same landing boundary as `ARCHITECTURE.md`'s fold" equivalence was false: that fold fires on **built structural change**, `authoring-architecture/SKILL.md:19,42-44` / `implement.md:42,132`, a condition disjoint from operational-surface change in both directions)*: the runbook fold fires when **the feature carried IP-XXX rows OR the landing's built-vs-approved diff shows new/changed deployment-relevant components** — co-located at the same landing *step* as the architecture fold but stated as its own condition; carrier = that landing step + a KM validator checklist fragment. (3) **Incident-response basics are runbook sections, not machinery:** health surface (where logs/dashboards live) · alert routing + severity · known failure modes with diagnosis steps · rollback *pointer* (referencing release-gates content per D2's reference-not-redefine rule; *fallback added at review fold M6*: when the module is declined — permitted and durable per F97 — **FLOOR-OPS carries the rollback-documented obligation directly**, inheriting F24's line, so the asserted document never depends on optional content) · escalation contacts (*sensitivity ruling added at review fold M20*: a **pointer to the team's rota/contact source, never inline personal data** by default; per-project override recordable). (4) **Migration:** past durable declines of the elective don't survive promotion — an assertion is not an offer; at next amend a recorded decline converts to a prompted waiver ruling.

**Held back:** standalone incident machinery (postmortem templates, severity taxonomy as floor content) — Tier-II reliability territory. **Steelman recorded:** a runbook nobody exercises is shelf-ware; the honest fix (an incident-drill analog to the security build's gate canaries, F84) is out of Tier-I scope — **watch item** for the reliability/resilience scoping session.

**Options considered** *(added at review fold M14 — not recorded in-session)*: (a) keep RUNBOOK elective but add the missing contract — closes F35 without promotion; rejected: PO-D5's mandate explicitly promotes, and production-only makes the elective's "deployed services" condition always true. (b) Promote as a module obligation rather than floor content; rejected: modules are declinable — an assertion needs the floor anchor (the statement's own argument). (c) Carry it as a skill-pointer surface (F96 surface 3) rather than a doc; rejected: the read-job is incident-time human reading — doc-shaped. Adopted: floor obligation + KM scaffold.

**Provenance:** lead-recommended package, user-ratified — adoption #4, streak active (flagged at Q4, user continued).

### D5 — Fifth floor category: Operations, carried by a new FLOOR-OPS card — `Confident`

**Statement:** (1) A fifth Essential Floor category, **Operations**, is minted in `ESSENTIAL-FLOOR.md` in the file's existing form (F1): category definition (what an Operations principle MUST address), worked example principle, enforcement/testability/rationale triad. (2) A new **FLOOR-OPS** card in `universal-floor.md` — floor-asserted, type tags all — asserted level: **SLOs defined and measured for the app's critical paths · alerting on SLO breach · operational runbook exists and stays current (D4's obligation) · release-health signals watched post-deploy in the form that fits the kind** *(+ at review fold M6: · rollback procedure documented — asserted directly when the `release-gates` module is declined, otherwise carried by the module and referenced)*. Waiver posture D4. (3) **FLOOR-OBS stays instrumentation only** — structured logs · correlation IDs · health checks · no PII — unchanged in scope *(amended at review fold M3: it additionally gains D6's one-line established-tooling carrier — a line addition within its instrumentation scope, built at stage 1)*. (4) **Enumeration ripple** *(restated at review fold M1 — the original "five files" list was recall-assembled and false; the set below is grep-verified by both reviewers and re-verified by the lead at fold)*: the four-category enumeration lives in **nine files** *(eight at the fold; the ninth caught at verify round 1, defect A — hidden from every grep by the `{{status}}` separators)* — `ESSENTIAL-FLOOR.md:3,5` · `catalog/universal-floor.md:3` · `authoring-constitution/SKILL.md:98,145,215` · **`agents/principal-architect.md:120,125` — a persona edit, keystone-checked class** · `validation-constitution/references/QUALITY-CHECKLIST.md:35 AND :88` (two floor-accounting lines; patching only `:35` ships a grader enforcing five categories greenfield / four brownfield) · `analysis-codebase/SKILL.md:12,42,66,123` · `constitution-modules/evolution-notes.md:35` + status-table rows · `INTERROGATION-AGENDA.md:94` · **`codebase-analysis-template.md:118` — the Category-rollup line** (the same file also gains the Operations probe rows: runbook present, alerting config, SLO/monitoring config detected). `catalog/README.md` carries no enumeration (verified — the original hedge resolves no-op). **Riders:** the health-check probe gap exists in BOTH the detection table (`analysis-codebase/SKILL.md:107-113`) and the template (F120) — both named, both fixed in the same edit (M18) · `validation-constitution` also gains the **known-gap check** (M8): floor-expression rows for unseeded kinds are known-gap-with-Tier-I-pointer or content — never "N/A", never absent (the S9 precedent's operative half, F83). (5) **Per-kind release-health seed:** backend/service seeded now (post-deploy error rate vs baseline · latency vs SLO); frontend (JS error rate + core-web-vitals), mobile (crash-free sessions), desktop (crash reports) are **recorded known-gaps with Tier-I shelf pointers** in the floor-expression rows (F98) — never silent, never "N/A" (the security session's S9 precedent, F83). (6) **Waiver key ruled clause-level** *(review fold M9, user-ruled)*: waivers cut at **clause** level (e.g. FLOOR-OPS's SLO clause alone), matching F92's worked example; F91's waivers-table `Standard` wording is amended in the build (category / card / **clause** / non-legal module obligation).

**Options considered:** (A) extend FLOOR-OBS with everything — cheapest, but the card carries two natures, the category definition muddies, and waiver granularity breaks: PO-D7's on-ramp story depends on waivers cutting at honest joints (an early team waiving SLO+runbook while meeting logging basics needs the ops/observability joint to exist) *(demoted at review fold M9: with the waiver key ruled clause-level — D5.6 — fine-grained waivers exist under either home, so the granularity argument is secondary; the category-definition/identity argument carries D5)*; (B) fifth category + FLOOR-OPS — adopted.

**Rationale:** PO-D5 called ops "identity-critical"; the four-category floor predates the production-only turn. Operating a service (commitments, response) is distinct from observing it (instrumentation). **Steelman recorded (for A):** the four-category floor is stable and enumerated across files and heads; a fifth category is a structural mint — held against: this is content, not kernel plumbing, and the ripple is mechanical (five files, verbatim-mapped) *(corrected at review fold M1: **eight files including a keystone-checked persona edit and both grader lines** — the "five files, verbatim-mapped" claim understated the cost in the sentence that justified the decision; **D5 re-ratified by the user with the corrected size in view — M16, 2026-07-31**. Verify round 1 found a ninth — the template's Category-rollup line, defect A; per the verify recommendation M16 is **not re-opened**: the delta is one summary line in an already-touched file, and the ratified fact — substantially larger than five, including the persona and both graders — is unchanged.)*.

**Fold note:** D1, D2, and D3's "FLOOR-OBS extension" landing phrases amended inline in this same edit (marked *amended at D5*).

**Provenance:** lead-recommended, user-adopted, ruled 2026-07-31 — adoption #5, streak active.

### D6 — Tooling stance: established-never-hand-rolled, open-source defaults, one universal map — `Confident`

**Statement:** (1) FLOOR-OPS/FLOOR-OBS carry the SD-D5-analog line: instrumentation, metrics, and error tracking ride **established tooling, never hand-rolled** — no bespoke metrics pipelines or homegrown log shippers; platform facilities legitimate (CloudWatch, App Insights — the iOS-Keychain analog) *(build home fixed at review fold M3: stage 1 edits BOTH cards — the FLOOR-OBS one-liner was previously in no stage)*. (2) **Open-source defaults, named:** OpenTelemetry as the default instrumentation carrier where the stack supports it (vendor-neutral); Sentry-class error tracking; Prometheus/Grafana-class metrics/dashboards. Defaults, not mandates — dimension 6/8 elicits what exists (F99); detected tooling wins over defaults. (3) **One map, not two:** the ops tooling rows join the **same universal home** as FLOOR-SEC's relocated stack tooling map *(amended at review fold M4 — F87 names no file, the security build is unstarted (F89), and the sole existing map sits on the arbitrated backend/service shelf, structurally unable to host type-tags-all floor rows: the home is now **named and defaulted** — `authoring-constitution/references/STACK-TOOLING.md`; whichever build lands first creates it, the other joins it; a 2-line coordination note is added to the security build's BACKLOG item at landing)*. (4) **Enforcement shape:** not a blocking CI gate — the validator's real-names/no-placeholder bar (F99) + build-time existence verification (D7). (5) **Disambiguation rider:** one sentence at the tooling rows — product-observability OTel ≠ the token-epic's Claude Code run-telemetry OTel (F126–F133).

**Steelman recorded:** named tools age; the library carries maintenance — held against: SD-D5 already crossed this bridge (version-pinned, library-maintained), and unnamed "use good tools" is Three-Part-Rule vagueness.

**Options considered** *(added at review fold M14 — not recorded in-session)*: (a) tool-agnostic floor line ("use established tooling", no names) — rejected: Three-Part-Rule vagueness; (b) mandated specific tools — rejected: detected-wins, defaults not mandates; (c) adopted — named open-source defaults, platform facilities legitimate.

**Provenance:** lead-recommended, user-adopted ("makes sense the recommendation") — adoption #6, streak active.

### D7 — The F53 close: instrumentation-existence verification, narrow — `Confident`

**Statement:** (1) **`patterns-vertical-tdd` gains one rule:** NFRs and IP-`monitoring` rows with a **build-time-verifiable surface** get `**TEST:**` tasks in the existing grammar (F116–F117) against real infrastructure — health endpoint responds · a real request emits a structured log line carrying a correlation ID · metrics exporter up · a test event reaches the error tracker. **Existence, never achievement** — SLO attainment is production-time (D3) and stays out. *(Criterion added at review fold M12: build-time-verifiable = executable against the built artifact or a local run with a deterministic assert — no production traffic or time-window measurement required. The four exemplars are backend-shaped (the F17 bias): frontend/mobile/desktop instrumentation checks ship with their shelves and are recorded as known-gaps mirroring D5.5 — the qualifier never silently exempts a kind.)* (2) **`review-task-artifacts` gains one active instrumentation-coverage row keyed on BOTH NFR and IP-`monitoring` rows** *(amended at review fold M7 — an NFR-only key cannot reach constraint-sourced monitoring rows: F48's `Source` is C-XXX or NFR-XXX, either not both, and F49 makes the constraint path canonical)* (no NFR row exists today, F53); the dormant IP/deployment rows (F61–F62) **stay dormant** (the deliberately-parked Tier-II deployment-engineering track) — the asymmetry is deliberate and gets a one-line note where the row lands. (3) **qa executes unchanged** — CLI-class TEST tasks under existing exit-code determinism (F109); no new seat, no new skill. (4) **Alert-route canary, SHOULD-level:** the gate-canary logic (F84) applied to alerting — each alert route demonstrated firing once via test event; needs a live monitoring stack, so it lands as a runbook-section prompt ("alert route last verified: date") *(bounded at review fold M13: re-verified at each amend run and each ops-touching implement landing — the KM validator fragment grades the field against the most recent such boundary, so the canary can actually fail)*, never a blocking build gate.

**Steelman recorded:** this is the thin end of the full-weave wedge D1 declined — held against: two checklist edits + the existing TEST grammar; no new artifact class, lens, or seat.

**Options considered** *(added at review fold M14 — not recorded in-session)*: (a) full NFR-verification activation including the dormant IP/deployment rows — rejected: that track is deliberately parked for Tier-II (F62); (b) defer all verification wiring to the shelf builds — rejected: F53 stays open for the seeded backend kind too; (c) adopted — the narrow close.

**Provenance:** lead-recommended, user-adopted — adoption #7, streak active.

## Build surface (drafted at convergence, pre-review)

Ordered as build stages; author ≠ grader per repo constraint; one command edit, shape-v5 audited.

**Tooling-home coordination (D6.3 as amended at M4)** *(this preamble rewritten at verify round 1, defect B — it previously restated the hard ordering dependency M4 dissolved)*: the universal tooling home is named and defaulted — `authoring-constitution/references/STACK-TOOLING.md`; whichever build (security/ops) lands first creates it, the other joins it. No hard ordering dependency remains; a 2-line coordination note lands on the security build's BACKLOG item at landing.

*(Stages restated at the review fold — M1/M2/M3/M4/M5/M7/M8/M9/M10/M12/M13/M18 repairs incorporated; the original stage list stands superseded.)*

1. **Floor cluster (D5):** `ESSENTIAL-FLOOR.md` fifth category **Operations** (definition + worked example + triad, file's existing form) · `catalog/universal-floor.md` new **FLOOR-OPS** card (asserted level per D5.2 incl. the M6 rollback fallback, D4's runbook obligation, D6's established-never-hand-rolled line) **+ the FLOOR-OBS one-line established-tooling edit (M3)** · enumeration ripple across the **M1-verified nine-file set** *(ninth added at verify round 1, defect A)*: `ESSENTIAL-FLOOR.md:3,5` · `universal-floor.md:3` · `authoring-constitution/SKILL.md:98,145,215` · **`principal-architect.md:120,125` (persona edit — keystone-checked)** · `QUALITY-CHECKLIST.md:35` **and** `:88` · `analysis-codebase/SKILL.md:12,42,66,123` · `evolution-notes.md:35` + table rows · `INTERROGATION-AGENDA.md:94` · **`codebase-analysis-template.md:118` (Category-rollup line)** · `analysis-codebase` Operations probes + `codebase-analysis-template` rows, with the health-check probe fix **in both files** (M18) · `validation-constitution` **known-gap check** (M8) · floor-expression known-gap rows for the three unseeded kinds (D5.5, S9-precedent language) · waivers-table **clause-level** `Standard` wording fix in `governance-surfaces-template.md` + `governance-intent-template.md` (M9/D5.6).
2. **SLO cluster (D3):** `INTERROGATION-AGENDA.md` dimension 8 gains SLO elicitation beats (critical paths, targets, measurement source) · **region one-summary-line + pointer; full SLO block as a `paths`-scoped rules file scoped to `.mochiko/specs/**`, scaffolded at setup G5 (M5)** — `governance-surfaces-template.md` + `authoring-constitution` routing edits · `governance-intent-template.md` SLO synthesis elements · `review-feasibility` one row (feature NFR ↔ app SLO contradiction) · **`review-plan-artifacts` SLO→NFR coverage row (M10)** · `ARTIFACT-TEMPLATES.md` Source-field guidance names the app SLO (tiny edit).
3. **RUNBOOK cluster (D4):** `knowledge-management.md` module gains the RUNBOOK document contract (read-job / writer moments / carrier) + skeleton sections incl. incident basics, the M6 rollback fallback, the M20 contacts-as-pointer rule, and the M13-bounded alert-canary field + validator fragment grading it · **`implement.md` gains the landing fold with the M2 dual-key trigger** (IP-XXX rows present OR built deployment-relevant component change; co-located with the architecture fold, its own condition; **the one command edit, shape-v5 audited**) · amend-time decline→waiver conversion at the agenda's waiver-rulings step (F107).
4. **Release-gates touch (D2):** `release-gates.md` gains the reference-not-redefine line pointing at FLOOR-OPS's asserted signals (small edit).
5. **Tooling rows (D6):** the universal tooling home **named and defaulted — `authoring-constitution/references/STACK-TOOLING.md`; whichever build (security/ops) lands first creates it, the other joins (M4)** · ops rows + the OTel-disambiguation sentence · 2-line coordination note on the security build's BACKLOG item (at landing).
6. **Verification cluster (D7):** `patterns-vertical-tdd` TEST-task rule with the **M12 criterion** (built-artifact/local-run deterministic assert) + per-kind known-gap language · `review-task-artifacts` active instrumentation-coverage row **keyed on NFR and IP-`monitoring` both (M7)** + asymmetry note · runbook alert-canary prompt line (rides stage 3's contract).

### Scope fences (out of this build, explicitly)

Release-**process** assertion — environments, cadence, gate table, rollback execution (Tier-II IaC stage 1, D2/F77) · the dormant IP/deployment-cycle rows stay dormant (D7/F61–F62) · SLO-**achievement** verification (production-time, D3) · standalone incident machinery — postmortems, severity taxonomy, drills (Tier-II reliability; drill watch below) · per-kind release-health **expressions** for frontend/mobile/desktop (shelf builds; recorded known-gaps, D5.5) · ops **lens** rollout at review sites (D1 held back; re-open condition below) · backup/restore + data lifecycle (Tier-II) · mochiko's own run telemetry (the token-epic OTel probe — different sense, F126–F133) · **error-budget / breach-consequence policy** (M11, user-ruled: recorded omission — the floor ships the alarm; what changes after it fires is the Tier-II reliability session's, see watch items).

### Watch items

- **Incident-drill analog** of gate canaries (D4 steelman) — offer at the reliability/resilience scoping session.
- **Tier-II migration surface** (D2 steelman): durable `release-gates` declines accumulating before Tier-II asserts — hand to the IaC scoping session.
- **D1 lens re-open probe** *(made positive at review fold M15 — hook silence is indistinguishable from a feature with genuinely no ops surface)*: at the first dogfood feature with a **known operational surface**, check that an availability/performance NFR and an instrumentation TEST task were actually authored; their absence re-opens the ops-lens question.
- **Error-budget / breach-consequence policy** (M11, user-ruled): deliberately omitted from Tier-I; handed to the Tier-II reliability/resilience scoping session (extends F77's row — timeout/retry/circuit-breaker + perf/load verification).
- **M2 trigger residual** (verify round 1, flagged-not-raised): the dual-key runbook-fold trigger doesn't cover a monitoring/alerting change inside an existing component with no IP row and no architecture delta — narrower than the pre-fold gap (the fold is a net improvement); hand to the build's own review.

## Review

**Sizing gate (user-ruled, 2026-07-31): pair** — lens-split cold review (decision-quality + record-integrity), cross-exam per the one-shot protocol, verify pass on the integrity lens. Named hunt target briefed to both: the 7-for-7 unelaborated adoption streak (every ruling lead-recommended, user-adopted without pushback) — passive-acceptance hunting is this review's backstop.

**Findings formed (sequestered cold reads, pre-cross-exam):** decision-quality 15 · record-integrity 10 — 25 total. Cross-exam opened (one-shot four-message protocol, dq initiates).

**Survivor reports (post-cross-exam):** decision-quality **15 → 15** (0 withdrawn; 6 materially restated — 2 downgraded, 1 upgraded, 3 re-anchored; final 2C/11I/2Min) · record-integrity **10 → 10** (8 as filed, 2 reduced; 2C/5I/3Min). No fact disputes required arbitration. **Both lenses recommend `critical-gaps`** — keyed on two broken load-bearing claims, each independently verified by both reviewers.

**Affirmative results recorded:** (1) the F1–F133 map audited clean — 21 entries sample-verified against files incl. re-run absence greps, every one held; all findings target the record's *use* of the map, no F-number needs correcting. (2) The D5 inline amendments all landed; no stale "FLOOR-OBS extension" phrasing survives. (3) The 7-for-7 adoption streak is honestly disclosed; recorded steelmans are real arguments (one exception → M14).

### Merged survivor set (lead merge: 25 → 20 — dup collapses DQ-1≡RI-1, DQ-3≡RI-2, DQ-7≡RI-4; clusters DQ-10+RI-9, DQ-14+RI-5)

**CRITICAL**

- **M1** (RI-1≡DQ-1) — **D5's ripple claim is false and it carried the decision.** The four-category enumeration lives in **8 files**, not "five, verbatim-mapped": adds `principal-architect.md:120,125` (persona — keystone-checked class), `authoring-constitution/SKILL.md:98,145,215`, `QUALITY-CHECKLIST.md:88` (a **second** floor-accounting line beyond F122's `:35` — patching only `:35` ships a grader enforcing 5 categories greenfield / 4 brownfield), `analysis-codebase/SKILL.md:12,42,66,123`, `universal-floor.md:3`, `INTERROGATION-AGENDA.md:94`, `evolution-notes.md:35`; `catalog/README.md` hedge resolves no-op (no enumeration). List was recall-assembled, not grepped.
- **M2** (RI-2≡DQ-3) — **D4.2's fold-boundary equivalence is false against the files.** `ARCHITECTURE.md`'s fold fires on *built structural change* (`authoring-architecture/SKILL.md:19,42-44`; `implement.md:42,132`), not on "IP-XXX rows / deployment view present" — triggers disjoint both directions; D4.2's own sentence names two different conditions; governs the session's single command edit. Failure: analyst omits an IP row → fold never fires → the 3am responder's runbook lacks the component that paged them; "stays current" satisfied vacuously.

**IMPORTANT**

- **M3** (RI-3) — D6 rules FLOOR-OBS carries the established-tooling line; **no build stage edits FLOOR-OBS** (compounded by D5.3's "unchanged in scope"). A ruled decision ships unbuilt.
- **M4** (RI-4≡DQ-7) — D6.3's "same universal home" **doesn't exist and can't be defaulted**: F87 names no file; sole existing map is `backend-service.md:137-148` on an arbitrated, type-selected shelf that structurally can't host rows for a type-tags-all floor card; security build unstarted (F89).
- **M5** (RI-5+DQ-14) — **D3.2's region SLO block breaks the region's own short-form contract** (`governance-surfaces-template.md:26` one-line-per-entry; `authoring-constitution/SKILL.md:65` always-on budget scarcest; `:232` release-gates precedent = one line + pointer); compressing to one line + ledger pointer kills D3's visibility purpose (F93: ledger never force-loaded). Only 2 of 5 surfaces (F96) were compared; the exit satisfying both constraints is surface 2 — a `paths`-scoped rules file. Rider: whether plan-producer seats see the CLAUDE.md region is a harness fact to confirm.
- **M6** (RI-6) — **an asserted runbook section depends on a declinable module:** D4.3's rollback *pointer* targets release-gates content; declines are durable (F97); the floor asserts a document whose contents depend on optional content.
- **M7** (RI-7) — **D7's producer rule reaches IP-`monitoring` rows; its NFR-keyed grader row cannot** — F48's `Source` is "C-XXX / NFR-XXX" *either*, F49 makes the constraint path canonical; constraint-sourced monitoring rows are unreachable; F61 stays dormant per D7.2. The F53 gap re-opens on the IP half.
- **M8** (DQ-2, restated) — **D5.5's known-gap expression rows ship ungraded:** no validator row reaches floor-expression rows at all; the invoked S9 precedent's operative half was "`validation-constitution` gains the check" (F83) — this session adds none.
- **M9** (DQ-4, restated) — **the waiver key is contradictory in the substrate** (F91 keys card/category-level; F92's worked example is clause-level "FLOOR-TEST coverage gate") and D5's granularity rationale depends on which reading holds: under clause-level, option A never had the granularity break; under card-level, FLOOR-OPS reproduces it one level down (a PO-D7 MVP waiving SLOs would drop the runbook obligation it meets).
- **M10** (DQ-5) — **the SLO↔feature link detects contradiction, never absence:** nothing obliges a feature touching an SLO-covered path to author an NFR, and upstream layers push ops language out (F54 flags uptime as a defect; F125 re-frames ops questions); D3's drift detector never fires on the silent path.
- **M11** (DQ-6) — **error-budget / breach-consequence policy is unassigned, not deferred:** in no fence, no watch item, and not in PO-D5's Tier-II row (F77). The floor promises the alarm and nothing after it.
- **M12** (DQ-8, restated) — **D7.1's "build-time-verifiable surface" qualifier has no criterion and four backend-shaped exemplars** (the F17 bias): predictable reading closes F53 for backend only, silently, for three of five kinds — the same kinds D5.5 handles honestly.
- **M13** (DQ-9, restated) — **the alert canary has a date field and no bound:** D4.2's KM validator fragment could grade staleness but D7.4 gives it nothing to grade against, while borrowing F84's "a gate that cannot fail is not a gate."
- **M14** (DQ-10+RI-9) — **D4, D6, D7 record no alternatives at all** (real cheap-to-state roads exist for D4: contract-without-promotion, module obligation, skill-pointer surface) **and D3's "Steelman recorded (for A)" argues the adopted option's risk, not A's case** (whose live form is M10). Against a 7-for-7 streak, was-it-raised is the whole question.
- **M15** (DQ-11) — **D1's re-open condition cannot fire:** hook silence is indistinguishable from no-ops-surface; the layers M10 names prevent the NFR that would make reviews non-silent.
- **M16** (DQ-12, restated) — **D5 enlarged D1–D3 without re-ratification:** adopted against "extend the ops floor," amended to "mint a fifth category + card + ripple," with the ripple understated (M1) in the sentence justifying it. Marking was honest (verified); marking is not re-ratification. Both reviewers rate this the streak hunt's strongest payout.
- **M17** (DQ-13, restated+upgraded) — **D4.2's fold breaches D1's own "only where plumbing already exists" fence:** the implement-landing fold is a new command-level mechanism (shape-audited), not existing plumbing; the fence future ops scope will be measured against is breached by the session that set it.

**MINOR**

- **M18** (RI-8) — D5.4's health-check rider names the template only; the detection table (`analysis-codebase/SKILL.md:107-113`) has the same gap — fix one and the template gains an unfillable `{{status}}` slot.
- **M19** (RI-10, reduced) — the Form line sequences "provenance notes → sizing gate"; the gate was ruled with the provenance-notes step unwritten (streak was in view via the Review section; bookkeeping fidelity).
- **M20** (DQ-15) — runbook **escalation contacts get no sensitivity ruling**: a shared/open repo ships on-call names and phone numbers in a committed file.

**Dispositions (2026-07-31): 20/20.** 17 folded per lead recommendation (M1–M8, M10, M12–M15, M17–M20 — each repaired **at the site of the superseded text**, marked inline with its M-number); 3 user-ruled against an explicit three-way ask, each adopting the recommendation ("go with recommendation"): **M9** — waiver key ruled **clause-level** (D5.6; F91 wording fixed in the build); **M11** — error-budget/breach-consequence policy recorded as a deliberate omission and handed to the Tier-II reliability session (fence + watch item); **M16** — **D5 re-ratified with the corrected eight-file ripple in view** (D1–D3 stand as amended). M1's eight-file set was independently grep-verified by both reviewers and re-verified by the lead before folding. **Verify pass (record-integrity lens): round 1 NOT CLEAN — 3 defects, all repaired same round:** (A, Important) M1's own repair falsely excluded `codebase-analysis-template.md:118`'s Category-rollup line — a ninth enumeration file, missed by every grep behind the `{{status}}` separators; lead re-verified the line before repairing; the M16 re-ratification **not re-opened** per the verify recommendation (delta = one summary line in an already-touched file), footnoted on D5. (B, Important) The build-surface preamble still carried the hard sequencing dependency M4 dissolved — rewritten to the M4 resolution. (C, Minor) M14's relabel had preserved M5-superseded "region SLO block" wording — aligned. Plus the M2 trigger residual recorded as a flagged-not-raised watch item. **Round 2: CLEAN** — all three repairs verified at site, both nine-file lists independently counted and matching, stale sweeps zero (historical retentions per the record's correction convention), repairs broke nothing. Round-2 rider (reviewer's beyond-scope sweep, recorded not repaired): `COMPLIANCE-MODULES.md:39-45` is category-keyed seed content — **build-time judgment line:** a five-category floor leaves it without an Operations bullet; either seed one or record "no ops-shaped compliance seed yet" (harmless-if-true, unlike the assessment tables). **Clearing verdict (lead): READY** — 25 raised → 25 survived cross-exam → 20 lead-merged → 20/20 dispositioned (17 folded, 3 user-ruled) → verify round 1 NOT CLEAN (3 defects, repaired same round) → round 2 CLEAN.

## Provenance notes

*(Written at the review fold — M19: the Form line sequenced this section before the sizing gate; it was unwritten when the gate was ruled. The streak was in view via per-decision provenance lines and the Review section's hunt-target brief; recorded here for fidelity.)*

All seven decisions were **lead-recommended and user-adopted without elaboration** — a 7-for-7 streak ("go with recommendation" ×4, "ratify" ×2, "makes sense the recommendation" ×1). The streak was flagged to the user once at Q4 ("say the word if you want me to argue harder"); the user continued. The pair review was briefed with the streak as its named hunt target; its payout: M14 (three decisions carried no alternatives), M16 (D5's enlargement of D1–D3 rode in on adoption #5 without re-ratification), M1 (the enlargement's cost understated in the sentence justifying it). All three repaired at the fold; D5 re-ratified by the user with the corrected size in view (M16, 2026-07-31). The three review-time user rulings (M9 clause-level waiver key · M11 defer error-budget policy · M16 confirm D5) were presented as an explicit three-way ask with recommendations and adopted as a batch ("go with recommendation").
