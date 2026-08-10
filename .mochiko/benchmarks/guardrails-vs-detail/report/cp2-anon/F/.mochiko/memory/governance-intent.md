# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by Priya
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: financial-adjacent (invoicing + payment tracking; not a
  bank/lender; payment collection via Stripe) · data classes: PII of US persons (names, emails,
  addresses, invoice amounts, payment status) · jurisdictions/markets: **US only** · contractual
  commitments: **none active** (SOC 2 mentioned on a prospect's procurement form; no signed
  obligation) · **Mark:** Confident
- **Modules triggered (mechanical):**
  - `a11y` (WCAG) — **legal-mandate** — trigger fact: customer-facing UI (the contractor-facing
    React app) served to US users → ADA. Scoped to the contractor-facing app; the client portal is
    out of scope for v1, so no client-facing UI is in scope.
  - **Negatives confirmed with consequence stated:** no cardholder data (Stripe hosts checkout —
    card data never reaches Ledgerline's servers) → **`pci-dss` will NOT attach**; no health data
    → **`hipaa` will NOT attach**; no EU/UK-resident data (US only) → **`gdpr` will NOT attach**;
    no contractual attestation commitment → **`attestation` will NOT attach**.
  - **Stripe cross-check note:** Stripe integration is present, but hosted-checkout-only, so
    cardholder data is genuinely not handled/stored — consistent with the "no cardholder data"
    fact, not a conflict. Recorded so a later temporal backstop can re-open it if a direct
    card-handling flow is ever introduced.
  - **CCPA/CPRA (reopen-born, S2):** "US only" forecloses GDPR but NOT US state privacy law.
    California's CCPA/CPRA governs CA residents' PII (Ledgerline's contractors + their clients
    include Californians). Thresholds (revenue/consumer-count) are not met at pre-launch scale →
    **not triggered yet** — recorded as a consequence-stated negative, not silently closed.
    Revisit trigger under GI-027. Connected to the still-open GI-022 (retention/deletion).

## Project identity & type

- **GI-002 — Type:** fullstack (FastAPI backend + React frontend) → shelves dealt:
  universal-floor + backend-service (API side); the frontend shelf is planned/absent, so
  frontend-facing floor expression is minted/translated, not copied · **Mark:** Confident
- **Identity:** A US-only SaaS invoicing + payment-tracking tool for small independent US
  contractors (trades, freelancers) — create clients, issue invoices, track payment status, send
  reminders, cash-flow dashboard. Founder's company, built to last, production quality from day
  one; pre-launch, first paying customer ~4 months out.
- **Risk surface:** High — real people's money. Data loss stalls a contractor's income; a wrong
  balance costs them cash and reputation; a **cross-tenant leak of financial data is
  company-ending**. Invoice/payment-state must be trustworthy without re-checking Stripe.
- **Team reality:** Solo founder full-time (does everything) + a friend ~5h/week on the React UI;
  no ops, no dedicated reviewer; self-review only today. **Enforcement cannot lean on code review
  — it must be CI/tooling-automated** (the FLOOR-SEC/TEST/DEP gates presuppose CI). Must be
  operable/understandable by one person.

## Convergence skips

None — all ten agenda dimensions were worked. (Dimension 6 existing-practices is greenfield: no
CI/tooling wired yet; the stack and intended commands were declared, not detected.)

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint (backend) | `ruff check .` | declared |
| Format check (backend) | `black --check .` | declared |
| Test + coverage (backend) | `pytest --cov --cov-fail-under=60` | declared |
| Lint (frontend) | `eslint .` | declared |
| Test (frontend) | `vitest run` | declared |
| Secret scanning (CI) | `gitleaks detect --no-banner` | declared/recommended |
| Dependency audit (backend) | `pip-audit` | declared |
| Dependency audit (frontend) | `npm audit --audit-level=high` | declared |
| Layer import rules | `lint-imports` (import-linter) | declared |
| Accessibility lint (frontend) | `eslint` with `eslint-plugin-jsx-a11y` | declared/recommended |

*(No CI pipeline exists yet — greenfield. Standing up GitHub Actions to run these gates is the
first foundational infrastructure task; it is the enforcement substrate the floor presupposes,
not a waiver. Captured as a BACKLOG item at scaffolding.)*

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level, fullstack expression: secrets out of repo + `.gitignore`; `gitleaks` secret-scanning in CI; pydantic input validation at API boundaries; auth enforced on every endpoint; **rate-limiting / brute-force protection on the login endpoint** (reopen-born, S7); passwords bcrypt-hashed; HTTPS everywhere; `pip-audit`/`npm audit` blocking high/critical. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | At floor level: `pytest --cov` (backend) + `vitest` (frontend); coverage ≥80% warning / ≥60% blocking; ratchet (baseline MUST NOT decrease); a smoke test on the invoicing critical path from day one. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level, fullstack expression: failures never silently corrupt data; RFC 7807 problem+json error bodies from FastAPI; React UI error states; correlation IDs on every error; no stack traces to users. | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | At floor level: structured JSON logs with correlation IDs; `/health` endpoint; no PII in logs; Sentry for error tracking (recommended, accepted). Maturity beyond the floor (SLOs, incident process, on-call) is excluded (GI-021), not waived — the floor items themselves are met. | Confident |
| GI-007 | BE-HEX | arbitrated | **KEPT, tightened/scoped.** Ports for the two mocked collaborators — a **Stripe port** and a **repository port** — isolating invoice/payment logic and tenant-scoping for unit test without Postgres or real Stripe. Enforcement scoped: `import-linter` **blocks** the load-bearing rule (domain MUST NOT import the Stripe SDK or DB drivers directly) and **warns** on other inward-dependency violations. Selects the `layer-rules` module. | Contested |
| GI-008 | BE-SRP | arbitrated | **KEPT, enforcement relaxed to warn.** One-job modules; **no "utils" dumping ground** (firm); complexity ≤10 and function-length limits emitted by ruff as **warnings**, session-tunable, not blocking. | Contested |
| GI-009 | BE-DEP | arbitrated | **KEPT as-is.** Justify new deps (none writable in <100 lines), pin versions + commit lockfiles, external calls through the kept ports; `pip-audit`/`npm audit` block high/critical. | Confident |

*Contested basis (GI-007, GI-008):* Priya deviated from the cards' full-strength "blocking CI
gate" recommendation, choosing warn-level enforcement for a no-reviewer solo codebase ("don't
want to fight import-linter at 2am"). The lead pushed back once — a warning-only rule is toothless
for a solo dev with no reviewer — and reconciled by keeping a **blocking** gate on the
load-bearing seam (GI-007) while relaxing the rest to warnings; Priya held her preference. The
challenge and her ruling are recorded here (the basis a `Contested` mark requires). **Re-affirmed
at review (S3):** the toothless-warning concern was put to her again, specifically for GI-008's
complexity ceiling, with "advisory-only for a solo dev" explicitly in view; she held warn-only
deliberately — hex is the load-bearing correctness seam (blocking), SRP complexity is
maintainability she wants to tune, not a gate that blocks her on a judgment call before a demo.
The `Contested` mark stands on a twice-recorded basis.

## Minted principle intents

- **GI-010 — Accessibility (WCAG) on the contractor-facing app:** the contractor-facing React app
  MUST target **WCAG 2.1 AA**; semantic HTML, form labels, sufficient color contrast, full
  keyboard navigation. Enforced by `eslint-plugin-jsx-a11y` + automated `axe-core` checks in CI,
  plus a manual keyboard/contrast pass before release. **Legal-mandate module obligation —
  unwaivable (D4.2).** · **Mark:** Confident
  *(Module: a11y — attached mechanically from GI-001's customer-facing-UI fact; formulated here
  because full per-regime obligation sets are mint-driven.)*
- **GI-011 — Tenant isolation (the hardest line):** every data access MUST be scoped to the
  authenticated contractor's account — no cross-account read or write, ever. This MUST be
  **test-catchable**, not a code-review vibe: an automated cross-tenant access test suite asserts
  that account A can never reach account B's data. · **Mark:** Confident
  *Elicited from:* "never let one contractor see another contractor's data … Every query scoped to
  the account, no exceptions, and I want that to be something a test can actually catch."
- **GI-012 — Invoice & payment-state integrity:** an invoice and its payment state MUST NOT be
  silently lost or corrupted; payment state MUST match reality (contractor action or Stripe
  webhook), reconciled against Stripe as the source of truth. No invoice disappears; no state
  shows paid-when-unpaid or unpaid-when-paid. · **Mark:** Confident
  *Elicited from:* "never lose or corrupt an invoice or its payment state … that has to stick and
  it has to match reality … trust the payment state without going and asking Stripe."
- **GI-013 — Money uses `decimal.Decimal`:** all monetary values and arithmetic MUST use
  `decimal.Decimal`, never floating point. Enforced by a lint/review rule against float arithmetic
  on money fields. · **Mark:** Confident
  *Elicited from:* "money arithmetic in floats is exactly the kind of thing that corrupts payment
  state … Decimal for all money, non-negotiable." (Supports GI-012.)
- **GI-014 — No raw cardholder data (Stripe-hosted-only):** Ledgerline MUST NOT accept, transmit,
  or store raw card numbers or CVV; all card entry goes through Stripe's hosted checkout. Enforced
  by review + the absence of any card-data field/schema; a temporal backstop re-opens GI-001 if a
  direct card-handling flow is ever proposed. · **Mark:** Confident
  *Elicited from:* "we never touch raw card numbers — Stripe-hosted checkout, full stop."
- **GI-026 — Stripe webhook trust (reopen-born, S1):** every inbound Stripe webhook MUST have its
  signature verified against the signing secret, and every event MUST be processed **idempotently
  (exactly-once)** — a replayed or forged event MUST NOT change payment state. Enforced by
  signature-verification middleware + an event-idempotency test suite. · **Mark:** Confident
  *Elicited from (review ruling):* "an unsigned or replayed webhook isn't Stripe telling me
  anything … webhook signature verified, every event processed exactly once." (Part of GI-012.)
- **GI-028 — Backup & tested restore (reopen-born, S4):** the production database MUST have
  scheduled automated backups, AND a restore MUST be periodically test-executed to prove the
  backup is recoverable (not migration-time backup alone). Enforced via Render managed-Postgres
  scheduled backups + a recorded restore-drill. · **Mark:** Confident
  *Elicited from (review ruling):* "scheduled DB backups plus an actual test-restore, so I know
  the backup is real." (Routes to the release-gates module content, GI-016.)
- **GI-029 — Invoice/payment audit trail (reopen-born, S5):** invoice and payment state changes
  MUST be recorded in an append-only, immutable, traceable log (who changed what, when). Enforced
  by an append-only change-log table + a test asserting no in-place mutation of historical rows.
  · **Mark:** Confident
  *Elicited from (review ruling):* "append-only log for invoice and payment state changes …
  immutable and traceable." (Supports GI-012.)
- **GI-030 — Transactional email deliverability (reopen-born, S6):** outbound invoice + reminder
  email MUST be authenticated (SPF, DKIM, DMARC configured) and MUST handle bounces/complaints.
  Enforced by verified DNS records + a bounce-handling webhook. · **Mark:** Confident
  *Elicited from (review ruling):* "SPF/DKIM/DMARC and bounce handling, baseline … not deferring
  the thing the feature depends on."

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| — | None. | | | |

*(No floor category waived. Priya accepted every floor category at the asserted level.)*

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-015 | knowledge-management | adopted (whole core) | offered default-on at dimension 7; solo founder needs durable decisions log + trustworthy backlog. Electives: **CHANGELOG.md adopted** (shipping to paying users), **RUNBOOK.md adopted** (deployed service, fill-as-you-go). | Confident |
| GI-016 | release-gates | adopted | deployed/operated target (Render); content from dimension 8 — environments, cadence, migration-safety gates (backup-before-migration + reversible migrations), rollback (Render redeploy-previous), and the scheduled-backup + tested-restore expectation (GI-028). | Confident |
| GI-017 | layer-rules | adopted | BE-HEX kept (GI-007) — the domain/repository/Stripe-port boundary. | Confident |

## Domain-dependency seeds (layer-rules adopted)

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-018 | pydantic | 4 — quantitative proxy + qualification criteria (ecosystem-standard validation/value objects, no I/O); FastAPI dependency already | kept | Confident |
| GI-019 | attrs | 4 — qualifies on criteria, but redundant with pydantic | **dropped** (Priya's own BE-DEP rule: no two libs for one job) | Confident |

*(Money arithmetic uses stdlib `decimal.Decimal` — not a registry entry; captured as GI-013.)*

## Deliberate exclusions (dimension 10)

- **GI-020 — SOC 2 / attestation program:** governance will not build around a certification with
  no active obligation. **Revisit trigger:** a customer contractually requires it → amend run
  attaches the `attestation` module. · **Mark:** Confident
- **GI-021 — Observability maturity beyond the floor:** SLO targets, incident-response playbooks,
  on-call rotation — excluded for v1 (one-person shop). The FLOOR-OBS items (GI-006) are still
  required; only the beyond-floor maturity is out. · **Mark:** Confident
- **GI-022 — Data retention & deletion policy:** deliberately **not answered** — Priya cannot name
  her legal obligations (suspects invoice-retention rules exist). Recorded as an **open question**,
  not a guessed policy; routed to `BACKLOG.md` for real research before any policy is authored.
  · **Mark:** Deferred
- **GI-023 — Out-of-scope feature set:** recurring invoices, multi-currency, estimates/quotes,
  client-portal accounts — governance reserves no space for any. · **Mark:** Confident
- **GI-024 — Multi-user (teams / bookkeeper seat):** excluded from v1 governance; must remain
  *possible* later but is not designed now — v1 governance treats the account as single-user.
  · **Mark:** Confident
- **GI-025 — Enterprise SSO + scale/spike engineering:** excluded — ~200 contractors, 10–40
  invoices/month each; no traffic story worth governing. · **Mark:** Confident
- **GI-027 — CCPA/CPRA (not triggered yet; reopen-born, S2):** US state privacy law is not built
  into v1 governance because applicability thresholds are unmet at pre-launch scale. **Revisit
  trigger:** crossing CCPA/CPRA applicability thresholds, or adding CA-specific deletion/access
  flows → amend run. Recorded (not silently closed) and linked to the still-open GI-022
  retention/deletion question. · **Mark:** Confident

## Review

<!-- Durable record of the sized pre-ratification intent review. Filled after the cold review
returns and the user rules survivors; frozen from reviewer spawn until dispositions land. -->

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — 25 GI elements at assembly, mark mix predominantly `Confident`
  (two `Contested` on the arbitrated deck, one `Deferred`), low reality-surface load (greenfield,
  no codebase to check). Default on first ratification is a **pair**; **lead sized: solo** —
  departure-trail: a single greenfield synthesis with no reality surface and a mostly-`Confident`
  mark profile does not warrant two reviewers; the coherence lens (verify pass) runs automatically
  solo. (Deviation from the default recorded here, not silently absorbed.)
- **Review:** reviewer — solo (both lenses); **tally** 7 raised → 7 merged survivors (solo, no
  cross-examination); recommended status **needs-revision**.
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-012 | Stripe webhook trust (signature + idempotency) never elicited | **resolved** — user ruled adopt → minted GI-026 |
  | S2 | Important | GI-001, GI-022 | "US only" foreclosed all privacy law without a threshold check (CCPA/CPRA) | **resolved** — user ruled record as not-triggered + revisit trigger → GI-027, GI-001 note; GI-022 stays open |
  | S3 | Important | GI-008, GI-002 | Warn-level SRP enforcement vs no-reviewer reality | **user-ruled** — held warn-only with the advisory-for-solo framing in view; GI-008 `Contested` basis re-recorded |
  | S4 | Important | GI-002 risk | Backup/restore beyond migration time not captured | **resolved** — user ruled adopt → minted GI-028 (into release-gates GI-016) |
  | S5 | Minor | GI-012 | Audit trail / immutability of invoice+payment state | **resolved** — user ruled adopt → minted GI-029 |
  | S6 | Minor | — | Transactional email deliverability (SPF/DKIM/DMARC + bounce) | **resolved** — user ruled govern → minted GI-030 |
  | S7 | Minor | GI-003 | Login rate-limiting / brute-force not in the security floor expression | **resolved** — user ruled add → folded into GI-003 expression |

- **Verify pass:** PASS — the six reopen-born folds (GI-026, GI-027, GI-028, GI-029, GI-030 and
  the GI-003 expression addition) are internally consistent with the elements they attach to
  (GI-012 integrity, GI-016 release-gates, GI-001/GI-022 privacy), each carries its review-ruling
  provenance, and none contradicts an existing element. Reopen-born intents ride this verify pass
  (no fresh cold read, no blind-map hunt). GI-008 `Contested` mark carries a twice-recorded basis.
- **G3-edit delta-pass:** n/a — the ratification `confirm` folded the reviewed survivors; no
  material post-review edit beyond the dispositioned survivors.

## Amendment Log

- 2026-08-10 — GI-001–GI-030 ratified (first ratification); reopen-born folds GI-026–GI-030 +
  GI-027 + GI-003 expression addition from the intent review's survivor dispositions — confirmed
  at synthesis checkpoint 2026-08-10 by Priya.
