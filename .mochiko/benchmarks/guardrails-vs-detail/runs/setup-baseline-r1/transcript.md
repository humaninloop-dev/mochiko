# Transcript — setup-baseline-r1

Simulated `/mochiko:setup` run for the greenfield **Ledgerline** project. Variant: **baseline**
(skill bodies from `plugins/mochiko/skills/`, skill + agent descriptions original). Run-lead
plays every seat; the principal (Priya) is answered only via fresh subagent spawns reading
`fixture/persona-card.md` (D3 contamination fence — the lead never reads the card).

Mode determination: empty workspace, no code → **greenfield**. Production floor asserted.

---

## Stage 1 — Interrogation (skill: `mochiko:analysis-iterative`, body loaded; agenda:
`authoring-constitution/references/INTERROGATION-AGENDA.md`)

Conducted inline by the lead, one question-beat per principal spawn. Ten agenda dimensions
worked adaptively, then the asserted floor + arbitrated deck.

### Beat 1 — dimensions 1 (identity/intent) + 2 (fact profile)

**Lead:** What are you building with Ledgerline, for whom, expected lifespan? Then the
compliance-relevant facts, each with its consequence stated (industry · data classes — including
explicit cardholder / health / EU-UK checks · jurisdictions · contractual commitments).

**Priya:** Ledgerline is a US-only SaaS invoicing + payment-tracking tool for small independent US
contractors (plumbers, electricians, freelance designers). They create clients, issue invoices,
track payment status, send reminders, see a cash-flow dashboard. Built to last — her company, not
a side project; production quality from day one; pre-launch, no code yet, first paying customer
~4 months out. Industry: financial-adjacent (invoicing/payment tracking), not a bank/lender;
payment collection via Stripe. Data classes, precisely: **no** cardholder data (Stripe hosts
checkout — card data never hits her servers; confirmed), **no** health data (confirmed), **no**
EU/UK-resident personal data (US only; confirmed). Does store: names, emails, addresses, invoice
amounts, payment status = PII of US persons. Jurisdictions: US only. Contractual attestation
commitments: none active (SOC 2 only on a prospect's procurement form — no signed obligation).
Flagged her own gap: knows backend basics, never run production SaaS solo, never set up real
observability, security past basics is fuzzy — wants to be told what she's missing. Asked whether
all compliance is needed before launch.

**Lead:** The quality floor is a single production-level standard — not dialed down for
early-stage — but a floor item you can't meet yet is a **recorded waiver** with justification +
revisit trigger, honest and auditable rather than silently ignored. We'll do that per-item.

*(Consequence-stated confirmations recorded: pci-dss will NOT attach — no cardholder data;
hipaa will NOT attach — no health data; gdpr will NOT attach — US only; attestation will NOT
attach — no contractual commitment. Stripe integration present but hosted-checkout-only, so
cardholder data is genuinely not handled/stored — consistent, not a conflict.)*

### Beat 2 — dimensions 3 (type) + 4 (risk) + 5 (team) + 6 (practices), plus the a11y fact

**Lead:** Project type? One more fact with consequence: Ledgerline has a customer-facing web UI
used in the US → accessibility statutes (ADA) apply → an a11y (WCAG) obligation attaches as a
**legal-mandate** (non-waivable) module. Confirm the customer-facing UI is in scope? Risk surface
— what does failure cost? Team reality? Existing practices/tools + actual lint/test commands?

**Priya:** Fullstack web app — FastAPI backend + React frontend, one product, not headless. A
contractor friend helps ~5h/week on the React side. Accessibility: yes with a correction — the
customer-facing UI is the **contractor-facing React app** she owns; WCAG applies to it and she
accepts the obligation as real. But a **client portal is explicitly out of scope for v1** —
clients only touch the invoice email and Stripe's hosted page, neither her UI. Risk is high (the
reason she wants it done right): data loss stalls a contractor's income; a wrong balance makes
them chase a paid client or miss an unpaid one; a **cross-tenant leak of financial data ends the
company**. Team: her full-time doing everything + friend ~5h/week; no ops, no dedicated reviewer;
self-review today (acknowledged not real review culture); must be operable by one person at 2am.
Stack decided: Python/FastAPI, PostgreSQL, React, deploy on Render (leaning) with managed
Postgres. Tools she'd reach for: `pytest`, `ruff`, `black` (backend); ESLint + `vitest`
(frontend). No CI pipeline yet — would be GitHub Actions. Asked whether CI is needed before first
customer.

**Lead:** WCAG is mostly hygiene if built accessibly from the start (semantic HTML, labels,
contrast, keyboard nav) — cheap early, expensive to retrofit. On CI: because you're solo with no
reviewer, **CI is essentially your only real enforcement mechanism** — standing up GitHub Actions
early is strongly recommended; without it most quality rules have no teeth. Not negotiating the
standard, naming how it gets enforced for a one-person team.

### Beat 3 — dimensions 7 (knowledge-management) + 8 (deployment/release)

**Lead:** KM module (default-on, whole core bundle): decisions log, backlog, thin roadmap, living
architecture doc, session records + the anti-rot rules. Adopt? Electives CHANGELOG.md and
RUNBOOK.md — each your call. Deployment/release reality — where it runs, environments, cadence,
release blockers, rollback expectation?

**Priya:** Adopt KM, whole bundle — solo founder, no team memory but hers, a decisions log +
trustworthy backlog is exactly the 2am problem. CHANGELOG.md — yes, shipping to paying people,
cheap. RUNBOOK.md — turn it on as a fill-as-you-go doc (won't have much at launch); punt only if
it's big up-front ceremony. Runs on Render + managed Postgres. Environments: local dev + prod; no
staging planned (asked if needed pre-customer). Cadence: ship-when-ready; post-launch a few times
a week, small changes. Release blocker: the invoicing core must be right — she'd hold a release
rather than ship something that could drop/corrupt invoice data or leak across tenants; once CI is
up, failing tests block too. Rollback: redeploy last good version (Render keeps previous deploys);
**DB-migration rollback is her known gap** — redeploying old code doesn't un-migrate data; asked
the normal one-person move.

**Lead:** Staging is nice-to-have pre-launch, not required if you have backup-before-migration +
Render's redeploy-previous. The normal one-person move is exactly automated Postgres backup
immediately before every migration + keep migrations reversible; we'll write that as a release
gate.

### Beat 4 — dimensions 9 (values/non-negotiables) + 10 (deliberate exclusions)

**Lead:** Beyond the floor, what must Ledgerline always/never do — as behavior a check could
block? What is deliberately out of scope for v1 governance?

**Priya:** Hard lines — (1) **never let one contractor see another's data**; single-tenant-per-
account, every query scoped to the account, and it must be **test-catchable, not a code-review
vibe** — the hardest line, a cross-tenant financial-data leak is fatal. (2) **Never lose or
corrupt an invoice or its payment state**; if a contractor or Stripe marks something paid it
sticks and matches reality — no invoice silently disappearing, no wrong paid/unpaid state; the
product's whole value is trusting payment state without re-checking Stripe. (3) Baseline she still
counts non-negotiable: passwords always hashed, everything over HTTPS, never touch raw card
numbers (Stripe-hosted, full stop). Won't inflate the list — tenant isolation + invoice/payment
integrity are the two she loses sleep over.

Deliberate exclusions for v1: **SOC 2** (no active obligation — revisit when a customer requires
it); **formal observability/ops maturity** — SLOs, incident-response playbooks, on-call (one
person; Sentry if it matters, but no uptime-target regime); **data retention & deletion policy** —
she genuinely doesn't know her obligations (suspects invoices have a legal retention rule but
can't name it) → **flag as an open question, don't guess a policy into governance**; the
out-of-scope feature set (recurring invoices, multi-currency, estimates/quotes, client portal —
governance reserves no space); **teams / bookkeeper seat** (multi-user must stay *possible* later
but not designed now); **enterprise SSO** and any scale/spike engineering (200 contractors,
10–40 invoices/month each — no traffic story worth governing).

### Beat 5 — asserted floor (expression confirm) + arbitrated deck (recommend-then-arbitrate)

**Lead (A — floor expression, asserted not negotiated):** Security (secrets out of repo +
gitignore, gitleaks in CI, pydantic input validation at boundaries, auth on every endpoint,
pip-audit/npm-audit blocking high/critical); Testing (`pytest --cov`, ≥80% warn / ≥60% block,
ratchet, smoke test on the invoicing critical path); Error handling (RFC 7807 problem+json from
FastAPI, React error states, correlation IDs, no stack traces to users); Observability (structured
JSON logs, correlation IDs, `/health`, no PII in logs; Sentry recommended). Object to / can't do
any of it yet?

**Priya:** No objections — most she'd want anyway. Only honesty: observability she doesn't know
well, but the concrete floor items (structured JSON logs, `/health`, no PII in logs, correlation
IDs) she'll just do; Sentry she'll pay for. If later asked for SLO targets or an incident process
she won't have answers — **not a waiver, just doesn't know what's normal yet**. Nothing to waive.

**Lead (B — arbitrated architecture cards, you sort):** BE-HEX (hexagonal — recommend KEEP, it's
what lets you unit-test invoice/payment logic + tenant-scoping without real DB/Stripe; enforcement
import-linter in CI; counter: up-front structure overhead for a solo dev). BE-SRP (single
responsibility + complexity ≤10 etc — recommend KEEP, metrics tunable, ruff-enforced). BE-DEP
(dependency discipline — recommend KEEP).

**Priya:**
- **BE-HEX — TIGHTEN (keep the seam, drop the purity ceremony):** keep ports for the two things
  she actually mocks — a **Stripe port** and a **repository port** — so invoice/payment logic and
  tenant-scoping unit-test without Postgres or real Stripe. Does **not** want full ports-and-
  adapters policed by a hard blocking import-linter over every inward dependency at 2am. Make
  import-linter block **only** the load-bearing rule (domain MUST NOT import Stripe SDK / DB
  drivers directly), warn on the rest. *(Deviates from the card's full-strength "import-linter
  blocking in CI" recommendation — lead noted a warning-only rule is toothless for a no-reviewer
  solo dev; reconciled to blocking on the load-bearing seam, warning elsewhere. Priya held her
  preference → marked Contested with recorded basis.)*
- **BE-SRP — KEEP, limits tunable and warn-not-block:** ruff-enforced one-job modules; complexity
  ≤10 + function-length as **warnings** she can tune, not hard blockers; **no "utils" dumping
  ground** firm. *(Deviates from the card's "complexity gate is a CI block" → Contested, recorded
  basis: solo dev, tune-not-fight.)*
- **BE-DEP — KEEP as-is:** pin versions, lockfiles, external calls through the kept ports; don't
  add a dep writable in <100 lines. Confident.

**Lead (C — domain allowlist, since BE-HEX kept):** seed with pydantic + attrs; money uses stdlib
`decimal.Decimal` (no dep). Keep/drop each?

**Priya:** **pydantic KEEP** (on FastAPI already, covers value objects, ecosystem standard).
**attrs DROP** — by her own BE-DEP rule, pydantic already covers value objects; won't run two
libraries for one job. **`decimal.Decimal` KEEP the note** — stdlib, and money in floats is
exactly what corrupts payment state: **all money uses Decimal, non-negotiable**. Seeded allowlist:
pydantic + the standing Decimal-for-money rule; attrs off.

**Convergence:** decisions settled, answers confirmatory, no new dimensions emerging, active
arbitration throughout (no passive-acceptance streak). Session concluded → synthesis assembled.

---

*(Stages 2–7 — synthesis, cold intent review, ratification, authoring, validation, KM/feature-map
scaffolding — continue below.)*

## Stage 3 — Cold intent review (skill: `mochiko:review-governance-intent`, body loaded)

Sized **solo** (a single greenfield synthesis, mostly `Confident` marks, low reality-surface load
— the lead sized down from the default pair; departure-trail line recorded in the Review section).
Verify pass is automatic when solo. The lead played the reviewer against the **frozen** synthesis
text; blind-map fence honored — the angle map was built from the project identity + goal only,
before reading the synthesis.

### Phase 0 — Blind angle map (topic/identity/goal only)

Angles a governance setup for a US SaaS invoicing + payment-tracking tool for contractors SHOULD
cover: (1) multi-tenant data isolation; (2) Stripe payment-integration correctness — webhook
authenticity, idempotency, reconciliation; (3) money representation (decimal/rounding); (4)
financial-record integrity + audit trail/immutability; (5) PII handling for US persons incl. US
state privacy law (CCPA/CPRA); (6) accessibility of the customer-facing UI; (7) auth/session
security incl. brute-force/rate-limiting; (8) legal data retention for financial records; (9)
backup / disaster recovery / tested restore; (10) observability without leaking PII; (11)
deploy/rollback incl. DB migrations; (12) transactional email deliverability for invoices +
reminders; (13) secrets management for Stripe keys.

### Phase 1 — cold read + five hunt classes (diffed against the frozen synthesis)

Marks used to prioritize, not skip. `Deferred`/`Contested` elements scrutinized hardest. FAIL
posture — findings below, none of the thirteen angles waved through unexamined.

**Survivors (7 raised, 7 survived — solo, no cross-examination):**

| # | Sev | GI element(s) | Finding | Failure scenario / resolution |
|---|-----|---------------|---------|-------------------------------|
| S1 | Important | GI-012 (coverage) | **Stripe webhook trust never elicited.** GI-012 names reconciliation "against Stripe as the source of truth" but the session never surfaced the *authenticity/exactly-once* sub-angle: webhook signature verification + idempotency/replay protection. | A forged or replayed `payment_intent.succeeded` marks an invoice paid when no money moved — directly violating the hardest integrity line. Resolution: one interrogation follow-up eliciting webhook-trust intent (a distinct principle, not just "reconcile"). |
| S2 | Important | GI-001, GI-022 (hunt class 2) | **"US only" foreclosed all privacy-law obligation without a consequence-stated threshold check.** GI-001 correctly foreclosed GDPR, but "US only" ≠ "no privacy law": CCPA/CPRA governs California residents' PII, and Ledgerline stores CA contractors' + clients' PII. It was recorded as if privacy is fully closed. | A CA user exercises a deletion/access right, or Ledgerline crosses CCPA thresholds, and nothing flagged it — compounding the already-`Deferred` retention/deletion open question. Resolution: record CCPA/CPRA as a consequence-stated negative with a revisit trigger (threshold crossing), the same treatment SOC 2 got (GI-020). |
| S3 | Important | GI-008, GI-002 (coherence: fact↔ruling) | **Warn-level SRP enforcement coheres poorly with "enforcement cannot lean on review."** The toothless-warning concern was put to Priya for BE-HEX (GI-007) and reconciled by keeping a blocking gate on the load-bearing seam — but the *same* concern was never put to her for GI-008: a ruff **warning** nobody reads is zero enforcement for a solo dev with no reviewer. A genuinely new framing the Contested ruling did not see. | Complexity creeps unblocked; 2am unmaintainable code — the exact failure SRP was kept to prevent. Resolution: confirm she accepts warn-level knowing it is effectively unenforced solo, or make the complexity gate blocking (as GI-007's seam is). |
| S4 | Important | GI-002 risk surface (coverage) | **Backup/restore beyond migration time not captured.** Release-gates covers backup-*before-migration*, but data loss is the founder's #1 fear and there is no scheduled-backup + tested-restore expectation for non-migration corruption. | An application bug corrupts invoice data (not a migration); the most recent backup is stale or never restore-tested. Resolution: elicit a backup cadence + a restore-drill expectation (Render managed Postgres supports it). |
| S5 | Minor | GI-012 (coverage) | **Audit trail / immutability of paid invoices not surfaced.** Supports GI-012 but was never raised as its own angle. | A contractor disputes a balance and there is no record of who changed the invoice state and when. Resolution: ask whether invoice state changes should be append-only/audit-logged. |
| S6 | Minor | (coverage) | **Transactional email deliverability** for the invoice + reminder send path (SPF/DKIM/DMARC, bounce handling) — a core capability's reliability angle never surfaced. | Reminders land in spam; contractors think clients were notified when they were not. Resolution: confirm whether send-path deliverability is governed or explicitly deferred. |
| S7 | Minor | GI-003 (hunt class 3: floor expression) | **Login rate-limiting / brute-force protection** not explicit in the FLOOR-SEC expression (auth-enforced is present; abuse-resistance is not). | Credential-stuffing against the login endpoint. Resolution: add rate-limiting to the FLOOR-SEC expression or note it deferred. |

**Recommended status: needs-revision.** Every survivor is resolvable by a session act — an
interrogation follow-up (S1, S3, S4, S5, S6, S7) or an added consequence-stated fact + revisit
trigger (S2). None is a fact-profile self-contradiction or an unrecorded ruling, so not
critical-gaps; and the hunt was actively worked with no blocking gap unaddressed, so not ready.

### Verify pass (solo, automatic)

No folds to verify yet (findings precede dispositions). Will re-run after the lead folds
dispositions and the user rules survivors.

## Stage 5 — Authoring the surface set (skill: `mochiko:authoring-constitution`, body loaded, greenfield branch)

Plan approved (producing seat plans first): 4 floor principles + no-cardholder-data + email-auth →
universal region lines; tenant-isolation, payment-integrity (invoice integrity + Decimal + webhook
trust + audit trail), hexagonal layers (+ domain registry), code-quality, dependencies,
accessibility → scope-bound rules files; KM + release-gates → module pointers; layer-rules →
layers.md + ledger domain policy. Authored: `CLAUDE.md` governance region, 8 rules files
(6 principle-bearing + operating-docs + output-style Shape 5), the ledger, and the trace-summary
manifest.

## Stage 7 (finalize scaffolding, run before validation so the grade sees the whole set)

KM bundle scaffolded (project-pinned `knowledge-management.md`, operating-docs rules file, core docs
ROADMAP/BACKLOG/DECISIONS/ARCHITECTURE/GLOSSARY + brainstorms & specs indexes + backlog-trail,
electives CHANGELOG + RUNBOOK). Greenfield feature map scaffolded (`FEATURES.md` empty index +
`.mochiko/features/`). Two BACKLOG items opened: stand up CI (GI-003/GI-004 enforcement substrate),
research data-retention policy (GI-022/GI-027 open question).

## Stage 6 — Independent validation (skill: `mochiko:validation-constitution`, body loaded; default FAIL)

Graded strictly from the files (author≠grader — the run-lead read the authored surfaces, not its
own authoring report).

**Deterministic scans:** placeholder scan clean (no `[PLACEHOLDER]`/`GI-XXX`/bracketed tokens);
exactly one governance region (begin+end) + one output-style marker pair; all 6 principle rules-file
pointers resolve; KM operating-docs pointer resolves; all 8 rules files carry `paths` frontmatter.
Floor categories all principled (GI-003/4/5/6); waiver table "None."; a11y (legal-mandate) not
waived (D4.2 respected). Trace: 15/16 manifest principle GI-IDs closed in synthesis + ledger.

**VALIDATION RESULT: FAIL (round 1)**

Issues requiring fix:
- **GI-028 (Backup & tested restore)** is listed in the trace-summary manifest as a principle-bearing
  element with `ledger ✓`, but the ledger has **no three-part entry keyed `### GI-028`** — it appears
  only as a row in the Release gates table. Step 3 (every manifest principle needs a three-part
  ledger entry) and Step 4.3 (companion entries must actually be present) both fail. Fix: add a
  `### GI-028` three-part entry (enforcement/testability/rationale/home/trace) in the Principles
  section.

Advisory (non-blocking): ledger `Modules:` header lists template modules (KM, release-gates,
layer-rules) alongside the compliance module; the region stamp correctly lists only the compliance
module (a11y). Compliance-module sets match — no fix required.

### Validation round 2 (after fix)

`### GI-028` three-part entry added (enforcement/testability/rationale/trace). All 16 manifest
principles now carry a three-part ledger entry; trace closes both ways.

**VALIDATION RESULT: PASS**

- Checklist items: core + 4 module fragments (a11y · knowledge-management · release-gates · layer-rules) verified.
- Surface integrity: region markers OK · index→home resolution 6/6 rules files + KM pointer · rules files paths-scoped 8/8 · new-file read line present · universal-in-rules violations: none.
- Trace closure: manifest principle rows closed 16/16 (primary home + index + ledger) · synthesis principle-bearing elements realized 16/16, non-principle elements (modules, exclusions) recorded · waivers matched (none) · modules matched to the fact profile (a11y compliance; KM/release-gates/layer-rules template modules).
- Floor/module accounting: production floor asserted (region stamp = ledger) · a11y compliance module matched to GI-001's customer-facing-UI fact · floor categories 4 principled + 0 waived.
- Anti-patterns: none (thresholds quantified — coverage 60/80, complexity ≤10, rollback ≤30min, WCAG 2.1 AA; enforcement named per principle).
- Version bump: 1.0.0 (initial ratification).

## Stage 8 — Final acceptance gate

Trace summary presented to Priya; no flagged proposals to rule. She accepted the surface set.
Commits suggested (not executed — the lead never runs git mutations). Setup complete.
