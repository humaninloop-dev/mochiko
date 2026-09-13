# Codebase analysis — Tidewell

**Run:** 2026-09-09 (refresh for the amend session; previous run 2026-02-27) · **Mode:** brownfield

## Detected stack (detect-stack baseline)

| Area | Detected | Evidence |
|------|----------|----------|
| Runtime | Node 22.6 | `.nvmrc`, `package.json` engines |
| Framework | NestJS 10.4, TypeScript 5.5 | `package.json` |
| Data | Postgres 16 through Prisma 5.19; SQL migrations under `db/migrations/` (0001–0007) | `prisma/schema.prisma`, `db/migrations/` |
| Front end | React 18.3 with Vite under `web/` | `web/package.json` |
| Logging / errors | pino 9; Sentry SDK 8 | `src/logging/`, `src/main.ts` |
| CI | GitHub Actions: `lint`, `typecheck`, `test` on every PR; `coverage` nightly; `npm audit --audit-level=high` non-blocking | `.github/workflows/ci.yml` |
| Deploy | Fly.io (`fly.toml`); on-merge to staging; manual promote to production | `fly.toml`, `.github/workflows/deploy.yml` |
| Secret hygiene | gitleaks pre-commit hook (`npm run hooks`) | `.husky/pre-commit` |
| Payments | **new since 2026-02-27:** `stripe` 16.8.0; `src/payments/` (Checkout session creation, webhook handler at `POST /webhooks/stripe`) | `package.json`, `src/payments/` |

## Changes since the previous run

- `src/payments/` added (2026-08-12 to 2026-08-19): `checkout.service.ts` creates hosted Checkout sessions; `webhook.controller.ts` verifies the Stripe signature and hands the event to `webhook.service.ts`.
- Migration `0007_payment_events.sql` adds `payment_events (id, stripe_event_id UNIQUE, type, payload JSONB NOT NULL, received_at)` and adds `card_brand`, `card_last4`, `card_exp_month`, `card_exp_year`, `card_fingerprint` to `payment_methods`.
- `webhook.service.ts` upserts on `stripe_event_id` and **persists the full Stripe event object** into `payment_events.payload` before dispatching by type ("kept for replay and support debugging" — `src/payments/README.md`).

## Data classes detected

- Landlord and tenant personal data (names, emails, phone numbers, postal addresses) — `tenancies`, `landlords`, `tenants` tables (unchanged).
- Rent ledger — `ledger_lines` (unchanged; append-only grants in `db/grants.sql`).
- **Payment-instrument metadata (new):** `payment_methods.card_brand`, `card_last4`, `card_exp_month`, `card_exp_year`, `card_fingerprint`.
- **Raw Stripe event payloads (new):** the stored `payment_events.payload` documents sampled from `test/fixtures/stripe/*.json` carry `payment_method_details.card` (brand, last4, exp_month, exp_year, fingerprint, country) and `billing_details` (cardholder name, email, postal address). No full card number appears in the sampled fixtures.

## Integrations detected

- Stripe (Checkout + webhooks) — new.
- Bank CSV import (Barclays and Starling export formats) — unchanged.
- Sentry, Fly.io — unchanged.

## Observations for the session

- The webhook handler's idempotency (upsert on `stripe_event_id`) is already implemented and tested (`test/payments/webhook.spec.ts`).
- Coverage from the nightly job on 2026-09-08: 44 % (baseline 41 %).
- `.github/CODEOWNERS` routes `src/ledger/**` and `src/import/**` to Priya; `src/payments/**` has no entry.
- Six distinct commit authors in the last 90 days: the four engineers in `CODEOWNERS` plus two contractor accounts active in July and August.
- `npm audit` on 2026-09-09: 2 high findings, both in transitive dev dependencies; triaged in the weekly issue.
