# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by founder (principal)
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (dimension 2), each with its consequence-stated confirmation — negatives
are recorded facts too (the S4 fail-safe):

- **GI-001 — Facts:** industry: invoicing / payment-tracking SaaS for US independent contractors
  (fintech-adjacent, not a card processor and not a payment platform) · data classes: PII (client
  and contractor names, emails, addresses) + financial data (invoice amounts, payment status) ·
  jurisdictions/markets: US-only (users and clients treated as US for v1) · contractual
  commitments: none signed (one prospect raised SOC 2, no active obligation) · **Mark:** Confident
- **Consequence-stated negatives (S4 fail-safe):**
  - `pci-dss` NOT attached — no cardholder/payment-card data stored (Stripe hosted checkout; card
    numbers never touch Ledgerline servers). Confirmed.
  - **No tax identifiers** (SSN/EIN/TIN) stored — no 1099 generation in v1. Confirmed on
    re-elicitation (review C1). Consequence: no Restricted-tier tax-ID handling obligation is
    minted, because none is stored.
  - **No contractor payout/KYC data** — Ledgerline does NOT use Stripe Connect; each contractor
    connects their own Stripe account and is paid directly, so bank details and identity documents
    are held by Stripe, never by Ledgerline. Confirmed on re-elicitation (review C1). Consequence:
    no Restricted-tier bank/identity handling obligation, and no platform/KYC compliance surface.
  - `gdpr` NOT attached now — US-only users and clients for v1. Refined on re-elicitation (review
    I1): the country field is not hard-blocked, so an EU/UK client address could technically be
    typed, but no GDPR handling (DPAs, data-subject rights) is built. Deferred with consequence
    (GI-024). Consequence: `gdpr` does not attach; a real EU push re-opens the fact profile as a
    governance event.
  - `hipaa` NOT attached — no health/medical data. Confirmed.
  - `attestation` NOT attached — nothing signed; SOC 2 raised by one prospect only. Confirmed
    (revisit condition recorded, GI-022).
- **Module attached (mechanical):** `a11y` (WCAG) — legal-mandate — trigger fact: customer-facing
  web UI served to US users (ADA). Confirmed in scope by the user (dimension 2); expression held
  to a baseline per the user's "sensible basics, not a full audit" ruling. Obligation authored as
  GI-011.

## Project identity & type

- **GI-002 — Type:** fullstack (Python/FastAPI API + React frontend) → shelves dealt:
  universal-floor + backend-service (API side). Frontend shelf is planned/unseeded → UI-facing
  floor categories translated by category definition + minting; accessibility routed via the
  `a11y` compliance module, not the frontend shelf. · **Mark:** Confident
- **Identity:** Ledgerline is an invoicing and payment-tracking SaaS for US small independent
  contractors — create clients, send invoices with a Stripe payment link, track paid/unpaid,
  chase, view cash-flow. A real product built to last years, paying customers from ~month four.
  Multi-tenant: many independent contractors share one deployment; each sees only their own data.
- **Risk surface:** Corrupting **or losing** a contractor's financial data (invoice amounts,
  payment status) is the product-killing failure — instant, permanent loss of trust. This drives a
  strict FLOOR-ERR, the financial-path test scope (GI-004), the data-durability principle
  (GI-012), the financial audit trail (GI-013), and the tenant-isolation principle (GI-014).
- **Team reality:** Solo founder + one part-time (~5 hr/wk) UI helper. No code-review culture —
  enforcement MUST lean on CI and automated tooling, never "another human reviews the PR."

## Convergence skips

None — all ten dimensions were worked. Dimension 5 (team) surfaced in dimension 1 (solo founder)
and was confirmed, not skipped (explicitly asked, batch 3 Q1).

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Python lint + format | `ruff check .` · `ruff format --check .` | declared |
| Python complexity | `ruff check --select C901 .` (max-complexity 10) | declared (ruff) |
| Python test | `pytest` | declared |
| Python dep audit | `pip-audit` | declared |
| JS/TS lint + format | `eslint .` · `prettier --check .` | declared |
| Front-end test | `vitest run` | declared (light) |
| JS dep audit | `npm audit` | declared |
| Secret scan (CI) | `gitleaks detect` | declared |
| Accessibility (CI) | `eslint` + `eslint-plugin-jsx-a11y` (+ axe assertions in component tests) | lead-recommended, user-accepted (baseline) |
| CI runner | GitHub Actions on push | declared |
| DB migrations | Alembic (reversible) | lead-recommended, user-accepted |
| DB backups / restore | managed Postgres automated backups + tested restore | user-ruled (review C2) |
| Deploy / rollback | Render — redeploy last good image (≤15 min) | lead-recommended, user-accepted |

## Floor expression & deck rulings

Floor cards enter asserted — rows record *expression* (type translation), not a level ruling;
arbitrated cards record the user's ruling. Dropped/tightened arbitrated cards are rulings too:

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level. Secrets via env + `.gitignore`; `gitleaks` secret scanning in CI; input validation at API boundaries (Pydantic); authentication enforced at every API endpoint; dependency vulnerability scanning (`pip-audit` + `npm audit`) blocking merge on high/critical. (Object-level / cross-tenant authorization is minted separately, GI-014.) | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | Expression: tests MUST pass in CI (no merging red) + a smoke test on the critical financial path from day one. Scope **widened on review (I2)**: the financial-path test covers invoice create, amount/tax/rounding correctness, AND asserts the Stripe payment-status webhook actually flips paid/unpaid (the webhook is the source of truth, not the UI). Numeric coverage gate **waived** (GI-019). | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level, kept in full (the user's #1 priority). Failures MUST NOT silently corrupt financial data; API returns a consistent error schema (RFC 7807 Problem Details); no leaked stack traces; correlation/trace IDs on every error. | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | Expression: minimal baseline — structured JSON logs; no PII/secrets in logs; `/health` endpoint; error tracking via Sentry; correlation IDs. Heavier SLO/on-call/incident-response depth excluded (GI-021), user-confirmed at the floor-assertion turn. | Confident |
| GI-007 | BE-HEX | arbitrated | **Kept, tightened to a pragmatic form**: domain/business logic (invoices, payments, money math) MUST be isolated from FastAPI routes and the DB behind a service + repository boundary, enforced by an import-linter rule in CI. Full four-layer ports/adapters ceremony dropped as over-build. Enforcement is CI (import-linter), never review. | Confident |
| GI-008 | BE-SRP | arbitrated | **Kept partial**: the CI complexity limit only — cyclomatic complexity ≤10 per function (`ruff` C901, CI-blocking). Review-enforced metrics (parameter count, file length, nesting depth) **dropped** — no reviewer to run them. | Confident |
| GI-009 | BE-DEP | arbitrated | **Kept**: new dependencies justified in the commit/PR description; versions pinned in lock files; vulnerability scanning blocks merge on high/critical (`pip-audit`, `npm audit`). | Confident |

## Minted principle intents

Traced to elicited answers, never to prompting. GI-012–GI-015 are **reopen-born** — re-elicited in
the post-review interrogation follow-up (review survivors C2/C3/C4/M1), landing in the GI namespace:

- **GI-010 — CI is the gate (no red to production):** merge to the deploy branch MUST be blocked
  while any configured CI check (lint, tests, secret scan, dependency audit) is failing; nothing
  reaches production over a red check. Enforced via GitHub Actions + branch protection (no
  human-review gate). · **Mark:** Confident
  *Elicited from:* dimension 9 — "CI has to block on the linters and tests before anything hits
  prod … No merging red."
- **GI-011 — Accessibility baseline on the customer-facing UI** *(the `a11y` module obligation):*
  the React customer-facing UI MUST meet a WCAG 2.1 AA **baseline** — semantic markup, labelled
  form controls, keyboard operability, sufficient contrast — enforced by automated checks in CI
  (`eslint-plugin-jsx-a11y` + axe assertions in component tests). A full manual WCAG audit is
  deferred. · **Mark:** Confident
  *Elicited from:* dimension 2 — "a web app for US customers, so I'd want accessibility in scope …
  sensible basics for v1, not a full audit."
- **GI-012 — Data durability (backups + tested restore):** invoice/payment data MUST be protected
  by automated managed-Postgres backups, and a restore MUST be executed at least once before
  launch and periodically after (an untested backup is not a backup). · **Mark:** Confident
  *Elicited from:* review follow-up — "automated DB backups plus a restore I've actually run at
  least once before launch and periodically after. Untested backup isn't a backup." (Resolves the
  "losing financial data" half of the stated top risk, previously ungoverned.)
- **GI-013 — Financial audit trail:** every change to an invoice **amount** or **payment status**
  MUST be recorded as an immutable log entry (who, what, when). Scope is deliberately limited to
  those two fields — not every field in the app. · **Mark:** Confident
  *Elicited from:* review follow-up — "Immutable log of who changed an invoice amount or payment
  status and when … Keep it to amount and payment-status changes — I don't need an audit trail on
  every field, that's gold-plating."
- **GI-014 — Tenant isolation (object-level authorization):** every data-access query MUST be
  scoped to the authenticated contractor; a contractor MUST NOT be able to read or mutate another
  contractor's data by object ID. Enforced by a test proving cross-tenant access returns 403/404.
  · **Mark:** Confident
  *Elicited from:* review follow-up — "every query scoped to the logged-in contractor, and a test
  that proves contractor A gets a 404/403 fetching contractor B's invoice by ID. Cross-tenant leak
  is the one bug I can't ship."
- **GI-015 — Email authentication & payment-link integrity:** invoice email MUST be sent from a
  domain configured with SPF, DKIM, and DMARC; the payment link MUST be integrity-protected
  (non-tamperable/non-guessable). · **Mark:** Confident
  *Elicited from:* review follow-up — "SPF/DKIM/DMARC is just DNS records … if my invoices land in
  spam contractors don't get paid. Note link integrity too. Zero cost, so no defer."

## Waivers

Any asserted standard may be waived with a recorded justification (D4) — except legal-mandate
module obligations (D4.2). Permanent pending the D4.1 revisit:

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| GI-019 | FLOOR-TEST numeric coverage gate (≥80% warning / ≥60% blocking) | Solo team + ~5 hr/wk UI helper; a coverage percentage would be gamed rather than earn real coverage. Financial correctness is protected instead by the widened critical-path test (GI-004) + all-tests-green CI gate (GI-010). PO-D7 young-team on-ramp. | When a second full-time engineer joins, or after first paying customers (~month four), re-evaluate a real coverage ratchet | Confident |

*(FLOOR-ERR, FLOOR-SEC, FLOOR-OBS retained — no waiver. The a11y obligation is legal-mandate, never
waivable; its baseline expression is not a waiver.)*

## Module selections

Template modules ruled in session; compliance modules attach mechanically in the Fact profile.
Declines are rulings too (durable):

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-016 | knowledge-management | adopted (core whole) | offered default-on at dimension 7; solo founder needs cross-month memory. Electives: `CHANGELOG.md` adopted (paid product, real releases), `RUNBOOK.md` adopted (one-person deployed service). | Confident |
| GI-017 | layer-rules | adopted | BE-HEX kept in pragmatic form (GI-007) — the layered-architecture beat fires. | Confident |
| GI-018 | release-gates | adopted | always offered for a deployed/operated product (PO-D1); dimension 8 supplied content (Render staging→prod, ship-when-ready, rollback ≤15 min, Alembic reversible migrations, destructive-migration self-approval). | Confident |

*(evolution-notes: not attached — greenfield. Compliance modules: `a11y` attached (GI-001, obligation
GI-011); `pci-dss` / `gdpr` / `hipaa` / `attestation` not attached — negatives confirmed above.)*

## Domain-dependency seeds (only when `layer-rules` is adopted)

Per `authoring-constitution/references/DOMAIN-DEPENDENCIES.md` — domain-relevance filtered first,
then trust-ranked, each level-cited:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-020 | `pydantic` | 4 — quantitative proxy (ecosystem-standard Python validation/modeling; PyPA is packaging-only, so level 4 + criteria) | kept — domain-relevant (value objects, validation, no I/O); already the declared stack | Assumed |

*Note (transparency):* the seed was **lead-recommended from the confirmed stack** (Pydantic already
declared) rather than card-by-card arbitrated, hence `Assumed`. Money math uses Python's stdlib
`decimal` (not a third-party dependency → not a registry row). The seed is deliberately minimal — a
non-empty day-one registry so the domain layer is not silently reduced to "standard library only."
Review M2 confirmed no additional money/validation helper belongs alongside `pydantic` for v1.

## Deliberate exclusions (dimension 10)

- **GI-021:** Observability/incident-response *depth* — SLO targets, on-call rotation, formal
  incident-response standards — out of governance scope for now. The one-person company cannot
  responsibly define these pre-launch. The cheap FLOOR-OBS baseline (GI-006) is retained; only the
  heavy layer is excluded. Revisit at real production traffic. · **Mark:** Confident
- **GI-022:** Formal-compliance / attestation governance (SOC 2, ISO 27001) — no active obligation,
  so no governance built around it yet. **Revisit condition:** a signed customer security addendum
  or attestation commitment re-opens the fact profile via an amend run (governance event). ·
  **Mark:** Confident
- **GI-023:** Product-scope exclusions — recurring invoices, multi-currency, estimates/quotes,
  client portal are out of the *product* scope for v1; governance covers none of them. (Product
  scope, not a floor category.) · **Mark:** Confident
- **GI-024:** GDPR / EU data-subject handling — **deferred with consequence** (review I1). Clients
  are treated as US for v1; the country field is not hard-blocked, but no GDPR handling, DPAs, or
  data-subject-rights machinery is built. **Consequence/revisit:** targeting EU/UK users or clients
  re-opens the fact profile as a governance event (module attach = MAJOR) — "a real project, not a
  patch." · **Mark:** Confident

## Review

<!-- Durable record of the sized pre-G3 intent review. Recovery keys off this section's state. -->

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — element set ~20 at review time (7 floor/deck + 2 minted + 3
  modules + 1 waiver + 1 domain seed + 3 exclusions + 2 profile), mark mix predominantly
  `Confident` with one `Assumed` (GI-020), reality-surface load moderate (greenfield — declared
  facts only, no codebase to cross-check). Default on first ratification is a **pair**; **lead
  sized: single**. Departure-trail line: *departure — first ratification defaults to a pair; sized
  to single because there is no detected-reality surface to cross-check (greenfield) and the element
  set is small and predominantly Confident; the single reviewer carries both coverage and coherence
  lenses and the verify pass.*
- **Review:** reviewer — devils-advocate (solo, `review-governance-intent`), both lenses,
  blind-map dispatch. **Tally: 10 raised → 8 survived** (2 fell: US-state-privacy at the blind-map
  diff; availability/SLO as saw-and-ruled). Surviving severities: 4 Critical, 2 Important, 2 Minor.
  Recommended status: **needs-revision** (all survivors session-resolvable; escalation to
  critical-gaps only if C1 had confirmed stored tax-ID/bank data — it did not).
- **Survivor dispositions** (each routed to the user as a coverage/fact ruling; re-elicited answers
  landed as GI elements):

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | C1 | Critical | GI-001 | Tax IDs (SSN/EIN/TIN) and contractor payout/KYC data (Stripe Connect) never named or consequence-confirmed — silent Restricted-data under-scoping. | resolved — re-elicited: **no tax IDs stored** (no 1099 in v1) and **not Stripe Connect** (contractor connects own Stripe, paid directly; Ledgerline never touches bank/identity data). Both recorded as consequence-stated negatives in GI-001. No critical-gaps escalation — no Restricted data stored. |
  | C2 | Critical | GI-012 (new) | Data-loss half of the stated top risk ("corrupting OR losing") had no governed control — no backup/restore obligation. | resolved — user ruled a baseline: automated managed-Postgres backups + a restore tested once before launch and periodically after. Minted GI-012. |
  | C3 | Critical | GI-013 (new) | No immutable financial audit trail — the standard control for the user's #1 corruption risk and for disputes. | resolved — user minted a scoped audit trail: immutable log of invoice-amount and payment-status changes (who/when), not every field. Minted GI-013. |
  | C4 | Critical | GI-014 (new) | "Auth at every endpoint" is authentication, not tenant-scoped authorization; cross-tenant object-level access (BOLA/IDOR) ungoverned in an inherently multi-tenant SaaS. | resolved — user confirmed contractors only ever see their own data; minted a tenant-isolation principle: every query scoped to the authenticated contractor + a cross-tenant 403/404 test. Minted GI-014. |
  | I1 | Important | GI-001, GI-024 (new) | GDPR negative generalized from the US *user* population to the *client* population it never examined (a US contractor can enter an EU client's PII). | resolved — user-declared fact confirmed: clients treated as US for v1, no country hard-block but no GDPR handling; `gdpr` stays unattached. Deferred with consequence recorded as GI-024 (EU push = governance event). |
  | I2 | Important | GI-004 | Day-one financial-path test covered only create + status-flip existence, not amount/rounding correctness or webhook truth — under-serving FLOOR-ERR. | resolved — user widened the test scope: amount/tax/rounding correctness + assert the Stripe webhook flips paid/unpaid. Folded into GI-004's expression. |
  | M1 | Minor | GI-015 (new) | Email deliverability / anti-spoofing (SPF/DKIM/DMARC) and payment-link integrity unaddressed, though emailing invoices+links is the core action. | resolved — user minted a cheap principle (zero-cost DNS records + link integrity). Minted GI-015. |
  | M2 | Minor | GI-020 | Domain-dependency registry thin (one Assumed row) for a money-heavy domain. | resolved — user confirmed no additional money/validation helper belongs alongside `pydantic` for v1 (`decimal` is stdlib). Transparency note retained; mark stays `Assumed`. |

  Fell at the diff (retrievable): US-state privacy CCPA/CPRA (no module trigger; below thresholds
  pre-launch — immaterial now); availability/SLO (saw-and-ruled, excluded GI-021); PCI/card path
  (ruled — hosted checkout; refined by C1).
- **Verify pass:** PASS — every fold confirmed against this updated synthesis: GI-001 carries the
  two new consequence-stated negatives (C1) and the refined GDPR negative (I1); GI-012/013/014/015
  present as minted reopen-born intents with quoted provenance (C2/C3/C4/M1); GI-004 expression
  widened (I2); GI-020 note updated (M2). No contradictions introduced by the folds. Reopen-born
  intents rode this verify pass (internal consistency + provenance), no fresh cold read.

## Amendment Log

Empty on first ratification.
