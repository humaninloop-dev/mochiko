# Runbook

Operational procedures for running Ledgerline in production (Render). Fill in as the service
grows; kept lean at launch.

## Deploy & rollback

- Deploy: push to main → GitHub Actions gates → Render deploy.
- Rollback (bad deploy): Render dashboard → redeploy the previous good deploy.
- Rollback (bad migration): restore the pre-migration Postgres backup, then redeploy prior code.

## Backups

- Render managed Postgres scheduled backups enabled; a test-restore is run periodically to prove recoverability (GI-028).
- An automated backup is taken immediately before every migration.

## Incidents

_TBD — no formal incident process at launch (GI-021, deferred). Sentry captures errors._
