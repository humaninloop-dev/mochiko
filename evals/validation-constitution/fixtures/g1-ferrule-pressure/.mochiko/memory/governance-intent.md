# Governance Intent — Ferrule

**Session date:** 2026-09-08 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-09-08 by Priya Nandakumar (CTO)
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: B2B equipment hire (tool-hire depots), UK · data classes: trade-customer contact details (names, phone numbers, e-mail, site addresses), driver names and shift times, booking and invoice records; no payment-card data (customers are invoiced monthly and pay by bank transfer); no health data · jurisdictions/markets: UK only · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** `gdpr` — personal data of UK residents (UK GDPR). Negatives confirmed with consequence: no cardholder data → `pci-dss` does not attach ("we never see a card number" — Priya); no health data → `hipaa` does not attach; no attestation commitments → `attestation` does not attach; customer-facing UI in the UK only → `a11y` does not attach under the seed table's listed statutes (ADA / EAA / EN 301 549) — revisit the moment an EU customer onboards, recorded as a standing amend trigger.

## Project identity & type

- **GI-002 — Type:** fullstack (backend service + web portal + mobile driver app) → shelves dealt: universal floor · backend-service · **Mark:** Confident
- **Identity:** Ferrule replaces the spreadsheets and phone calls tool-hire depots run their fleet on. Depots list assets, trade customers book online, drivers deliver and collect against dockets, and the office invoices at month end. Expected lifespan: years; first paying depot live since July.
- **Risk surface:** a lost or duplicated booking is a van sent to the wrong site and a customer without a digger on a Monday morning. Customer contact data and driver rotas are personal data. No card data, no health data.
- **Team reality:** four engineers, PR review on every change, GitHub Actions CI, one on-call engineer per week. Enforcement can lean on CI, pre-commit hooks, and a reviewer who is not the author.

## Depth level declaration

- **GI-003 — Declared level:** low · **Declared:** 2026-09-08 by Priya Nandakumar · **Mark:** Confident
- **Rationale:** greenfield with three months of code; the team wants coverage measured and ratcheted from reality before a blocking threshold is asserted. Setup recommended `low`; the user ruled `low`.
- **Ratchet:** one-way — a later `low`→`high` move happens only through a flip ceremony (a conscious `/mochiko:setup` rerun in high mode), never silently, never reversed.

## Convergence skips

- Dimension 5 (team) — settled by dimension 1: four engineers, PR review, CI already running.

## Real commands

(dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `pnpm lint` | detected (`package.json`) |
| Test | `pnpm test` | detected |
| Coverage | `pnpm test:cov` | detected (`vitest --coverage`) |
| Smoke | `pnpm smoke` | detected (Playwright, the booking path) |
| Secret scan | `gitleaks detect --source .` | declared (pre-commit hook + CI job `secrets`) |
| Dependency advisories | `pnpm audit --audit-level=high` | declared (CI job `audit`) |
| Build | `pnpm build` | detected |
| Deploy | GitHub Actions → Render (`staging` on merge, `production` by manual promotion) | declared |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | Low row: secrets out of the repo (Render environment groups; `.env*` gitignored) · all external inputs validated at the API and portal boundaries (NestJS `ValidationPipe` with `whitelist: true`; Zod schemas in the portal's route handlers) · auth enforced at every HTTP boundary (the `AuthGuard` is global) · `gitleaks` runs pre-commit and in CI · `pnpm audit --audit-level=high` runs on every PR. At `low` neither scan blocks merge yet. | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | Low row: `pnpm smoke` (Playwright) covers the booking critical path from day one · coverage reported on every PR by `pnpm test:cov` · ratchet: the baseline MUST NOT decrease. No blocking threshold at `low`. | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | Low row: a failure never silently corrupts a booking or a docket (every write is transactional; the sync worker reconciles) · no leaked stack traces · errors surfaced, never swallowed. | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | Low row: logs exist on the booking and docket paths (`pino`, one line per state change) · no personal data in logs — the shared logger's redaction list covers names, phone numbers, e-mail, and site addresses. | Confident |
| — | BE-HEX | arbitrated | **dropped** — "a modular monolith, no layer ceremony yet; revisit when the second team forms" (Priya). No `layer-rules` module. | Confident |
| GI-011 | BE-API-ERR | arbitrated | **kept** — RFC 7807 Problem Details on every error response, built in exactly one place. Scope-bound: the API (`apps/api/src/`) and the portal's route handlers (`apps/portal/src/app/api/`), which shape their own error bodies for the browser. | Confident |

## Compliance module obligations (from GI-001)

- **GI-008 — gdpr obligations (legal-mandate, unwaivable):** a lawful-basis record per data class · a retention schedule (trade-customer contacts: life of the account plus 12 months; driver rotas: 6 months; invoices: 7 years) · subject access and erasure requests fulfilled within 30 days through the `privacy` runbook · personal-data breaches reported to the ICO within 72 hours of discovery · **Mark:** Confident

## Minted principle intents

- **GI-009 — Small units:** "no function over 50 lines, no file over 400 lines, and ESLint fails the build when either is crossed" — the team's own words for the readability standard they already argue about in review. · **Mark:** Confident
  *Elicited from:* dimension 9 — "what do you already argue about in review?" — ESLint `max-lines` at 400 and `max-lines-per-function` at 50 named directly.
- **GI-010 — Snappy API:** list endpoints answer in under 300 ms at p95 at ten times today's load, measured by the nightly k6 run in CI against `staging`; a regression past 300 ms is a bug, not a tuning ticket. · **Mark:** Confident
  *Elicited from:* dimension 9 — the depot managers' complaint about the previous product ("every page took three seconds"); the 300 ms p95 figure is Priya's.

## Waivers

None.

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-012 | release-gates | adopted | Deployment dimension: Render `staging` → `production`, `staging` on every merge, `production` promoted manually by the on-call engineer on weekdays; a two-hour Sentry soak, a reversible-migration check, and the smoke run block promotion; rollback is a redeploy of the previous Render release within 15 minutes. Region carries one summary line; the detail lives in the ledger. | Confident |
| GI-013 | knowledge-management | adopted — core, whole | Offered default-on at dimension 7; the team has no operating docs today and wants the layer from the start. Electives: `CHANGELOG.md` declined (no user-facing release history yet), `RUNBOOK.md` declined (Render runbooks live in Notion; revisit at the first incident). | Confident |
| — | layer-rules | declined | BE-HEX dropped; no layered intent minted. | Confident |
| — | evolution-notes | n/a | Greenfield. | — |

## Domain-dependency seeds

Not applicable — `layer-rules` not adopted.

## Deliberate exclusions

None recorded.

## Review

**2026-09-08 — first ratification**

- **Sizing:** 13 elements · marks all Confident · reality-surface load low (greenfield, four engineers); default on first ratification is pair; **lead sized:** single — one cold reviewer, departure-trail line: "thirteen elements, no contested marks, one compliance module; a pair would be reviewing each other."
- **Review:** solo reviewer; **tally** 4 raised → 2 merged survivors; recommended status: needs-revision (resolved before ratification)
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-001 | The `a11y` negative rested on "B2B" alone; the seed table keys on jurisdiction and UI, not on the customer type. | resolved — negative re-confirmed on the listed-statute basis with an EU-customer amend trigger recorded |
  | S2 | Minor | GI-010 | "Fast" had no number when first drafted. | resolved — 300 ms p95 at 10× load, k6 nightly, recorded in the element |
