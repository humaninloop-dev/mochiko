# Runbook

> Operational how-to for running Ledgerline. A growing stub — add procedures as they are learned (GI-014).

## Deploy

- Deploy target: Render (managed host). Push to the release branch triggers the build; promote from the Render dashboard.

## Rollback

- Redeploy the previous commit/image from the Render dashboard. Target: previous version restored in ≤15 minutes.
- Keep DB migrations backward-compatible so a rollback does not strand the schema.

## Where logs / metrics live

- _TBD — fill in once logging/observability is wired (structured logs per GI-006)._

## Incident procedures (stubs — grow as encountered)

- **Stripe webhooks failing:** _TBD — check webhook delivery in the Stripe dashboard, verify the endpoint, replay events (idempotency guarantees replay is safe, GI-021)._
- **Database down:** _TBD — check Render Postgres status; the app should surface errors, never corrupt state (GI-005)._
