# Governance Intent — Tidewell

**Session date:** 2026-03-02 · **Mode:** brownfield — amend session 2026-09-10 (amend)
**Confirmed at synthesis checkpoint:** 2026-03-02 by Priya · amend re-confirmed at synthesis checkpoint 2026-09-10 by Priya
**Governs:** the governance surface set v1.1.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`) — v2.0.0 proposed by this amend

## Fact profile

- **GI-001 — Facts:** industry: proptech (residential lettings, UK) · data classes: landlord and tenant personal data (names, emails, phone numbers, postal addresses, tenancy documents); rent ledger (amounts, bank-transfer references, arrears) · jurisdictions/markets: UK (England and Wales) · contractual commitments: 99.9 % monthly availability commitment to Brightwater Homes and Kelso Lettings (housing-association contracts signed January 2026) · **Mark:** Confident
- **Modules triggered (mechanical):** `gdpr` (UK GDPR — personal data of UK residents) · `a11y` (customer-facing UI in the UK — Equality Act 2010, WCAG 2.2 AA) · negatives confirmed: no health data → `hipaa` does not attach; no cardholder data (rent arrives by bank transfer; the app reconciles a bank CSV export) → `pci-dss` does not attach; no attestation commitments (neither contract carries a SOC 2 or ISO clause) → `attestation` does not attach
- **Brownfield cross-check:** consistent — `codebase-analysis.md` (2026-02-27) detects the tenant and landlord tables, the bank-statement CSV importer, and no payment processor
- *Amend 2026-09-10 (GI-001 edited):* card payments through Stripe Checkout live since 2026-08-19 (hosted payment page). **No new data classes** — cardholder data never touches Tidewell: Stripe hosts the card form, so under PCI DSS the integration qualifies as SAQ A and Tidewell is out of scope; the `pci-dss` module does not attach. · **Mark:** Confident
  *Brownfield cross-check (amend):* consistent — the refreshed analysis (2026-09-09) lists `stripe` as the one new dependency.

## Project identity & type

- **GI-002 — Type:** fullstack (NestJS API + React SPA) → shelves dealt: backend-service (no frontend shelf yet — the floor categories were translated for the SPA in session) · **Mark:** Confident
- **Identity:** Tidewell collects rent and routes maintenance requests for small UK landlords and letting agents (1–40 units). Core product, indefinite lifespan; 1,900 paying landlords as of September 2026.
- **Risk surface:** money (a wrong ledger line is a wrong arrears letter to a tenant); personal data of tenants and landlords; reputational exposure with the two housing associations. *Amend 2026-09-10 (Priya):* "a payment outage is tolerable for up to a day — tenants can still pay by transfer and the ledger catches up."
- **Team reality:** four engineers; every PR reviewed; Priya reviews anything touching the ledger.

## Depth level declaration

- **GI-003 — Declared level:** low · **Declared:** 2026-03-02 by Priya · **Mark:** Confident
- **Rationale:** setup recommended `high` (brownfield, live tenants, money on the ledger); Priya ruled `low`: "coverage ratchet first — we flip once CI is something we trust."
- **Ratchet:** one-way — `high` is terminal; a later `low`→`high` move happens only through a flip ceremony.
- *Amend 2026-09-10 — GI-003 superseded by* **GI-018 — Declared level:** high · **Declared:** 2026-09-10 by Priya · **Mark:** Confident · **Rationale:** "we have 1,900 paying landlords now; time to be strict" — confirmed in session; effective on ratification.

## Convergence skips

none (first ratification) · *Amend 2026-09-10:* dimensions 1, 3, 5, 7 and 9 untouched by the event — not re-asked (bookkeeping); dimensions 2, 4 and 10 re-asked on the event's slice.

## Real commands

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `npm run lint` | detected (eslint) |
| Typecheck | `npm run typecheck` | detected (`tsc --noEmit`) |
| Test | `npm test` | detected (jest) |
| Coverage | `npm run coverage` | detected (jest `--coverage`; nightly job) |
| Build | `npm run build` | detected |
| Secret scan | `gitleaks protect --staged` | detected (pre-commit hook) |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | low row — secrets in Fly secrets + `.gitignore`; class-validator DTOs at every controller; JWT guard on every route; gitleaks pre-commit; `npm audit` in CI, non-blocking | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | low row — smoke test on ledger posting and the CSV import; coverage measured (jest) and reported; ratchet from the 41 % baseline | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | low row — ledger writes in one transaction; no stack traces past the exception filter; every caught error to Sentry | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | low row — pino on the ledger and import paths; redaction list for email, phone and postal address | Confident |
| GI-008 | BE-HEX | arbitrated | kept — ports and adapters around the ledger core and the bank-import adapter; Priya: "the import format will change; the ledger must not" | Confident |
| GI-009 | BE-SRP | arbitrated | kept — one module per bounded area (tenancies, ledger, import, maintenance) | Confident |
| GI-010 | BE-DEP | arbitrated | tightened — lockfile committed, Renovate weekly, no `latest` tags; Priya: "a surprise upgrade cost us a weekend in January" | Confident |

*Amend 2026-09-10 — re-dealt on the event's slice (payments in; level raised):*

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-019 | FLOOR-SEC (supersedes GI-004) | floor-asserted | unchanged — low row as above; the Stripe secret key and webhook signing secret live in Fly secrets | Confident |
| GI-020 | BE-HEX (supersedes GI-008) | arbitrated | dropped — the user overruled the lead's challenge | Contested |
| GI-021 | BE-SRP (supersedes GI-009) | arbitrated | kept — as recommended | Confident |
| GI-022 | BE-DEP (supersedes GI-010) | arbitrated | kept — as recommended | Confident |

## Minted principle intents

- **GI-011 — Append-only ledger:** a posted rent line is never edited or deleted; a correction is a reversing entry; enforced in the ledger service and by the database grants · **Mark:** Confident
  *Elicited from:* Priya, dimension 9: "we never edit a posted rent line — ever. You reverse it. That's the one rule I'd fire someone over."
- **GI-012 — Founder review on money-moving code:** any change under `src/ledger/` or `src/import/` carries Priya's approval on the PR · **Mark:** Confident
  *Elicited from:* Priya, dimension 9: "anything that moves money, I look at."
- *Amend 2026-09-10:*
- **GI-023 — Two-engineer review on payments code (supersedes GI-012):** any change under `src/payments/`, `src/ledger/` or `src/import/` needs two approving reviews, one of them Priya's · **Mark:** Confident
  *Elicited from:* Priya, amend card 2: "anything touching money gets two reviewers now, not just me — I'm the bottleneck."
- **GI-024 — Idempotent webhook handling:** every Stripe webhook handler is idempotent on the Stripe event id · **Mark:** Confident
  *Elicited from:* "idempotency matters for webhooks" (amend card 3).
- **GI-025 — OpenAPI in CI:** every service regenerates its OpenAPI document in CI and fails the build on an undocumented diff against `main` · **Mark:** Confident
  *Elicited from:* lead proposal at amend card 5; Priya: "ok, sure."

## Waivers

| GI-ID | Standard | Justification | Revisit trigger (optional) | Mark |
|-------|----------|---------------|---------------------------|------|
| GI-013 | FLOOR-TEST — coverage measured and reported on every PR | the per-PR coverage job doubled CI wall time; coverage runs nightly and the ratchet reads the nightly number | CI wall time under 8 minutes | Confident |

*Amend 2026-09-10:* no waiver changes.

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-014 | knowledge-management | adopted (core; CHANGELOG elective added 2026-05-14) | offered default-on at dimension 7; Priya: "we lose decisions in Slack" | Confident |
| GI-015 | release-gates | adopted | deployed product — Fly.io, on-merge to staging, manual promotion to production, roughly weekly | Confident |
| GI-016 | layer-rules | declined | BE-HEX kept, but Priya declined the registry: "we are not going to police imports" | Confident |

*Amend 2026-09-10:* GI-015 re-confirmed adopted — as recommended · GI-016 re-confirmed declined — as recommended.

## Deliberate exclusions

- **GI-017:** front-end performance budgets — not covered until the frontend shelf exists; revisit when it ships · **Mark:** Confident

*Amend 2026-09-10:* dimension 10 re-asked — no exclusion added or withdrawn.

## Review

**2026-03-02 — first ratification**

- **Sizing:** lead stated weight [17 elements · 17 Confident · reality-surface load: NestJS API, CSV importer, Fly.io]; default pair on first ratification; **lead sized:** pair (coverage · coherence)
- **Review:** reviewers coverage / coherence lenses; **tally** 9 raised → 4 merged survivors; recommended status needs-revision
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-001 | a11y trigger unexamined (customer-facing UI, UK) | resolved — module attached, Priya confirmed |
  | S2 | Important | GI-005 | coverage baseline unstated | resolved — 41 % recorded from the nightly job |
  | S3 | Minor | GI-010 | tightening reason absent | resolved — reason recorded |
  | S4 | Minor | GI-007 | redaction list unnamed | resolved — email, phone, address named |

- **Verify pass:** PASS — folds confirmed by the coherence-lens reviewer 2026-03-02

**2026-09-10 — amend**

- **Sizing:** lead stated weight [10 elements touched — GI-001 and GI-002 edited, GI-018 to GI-025 new · marks 9 Confident / 1 Contested · reality-surface load: `src/payments/`, migration 0007, Stripe webhooks]; default pair (fact-profile question plus a level change — a governance event); **lead sized:** single — departure trail: "one bounded delta; a single cold seat is proportionate, and Priya wants to author today"
- **Review:** pending — one cold seat, solo, no lens
- **Survivor dispositions:** pending — the seat reports; the lead folds
- **Verify pass:** pending

## Amendment Log

- 2026-05-14 — GI-014 edited (CHANGELOG elective adopted) — a release-shaped product wanted a user-facing change log — re-confirmed at synthesis checkpoint 2026-05-14
