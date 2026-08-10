# Runbook — Ledgerline

Lightweight operational runbook for a one-person shop: restart, roll back, restore, and what to
check when something breaks. Kept minimal by design (governance GI-016).

## Restart the service

- Managed host (Render/Railway): trigger a restart of the web service from the host dashboard, or redeploy the current commit.

## Roll back a bad deploy

- Roll back to the previous deploy from the host dashboard (target: previous version live in ≤15 minutes).
- Safe because migrations are forward-only + backward-compatible (expand-then-contract) — the previous code runs against the current schema. If a deploy included a destructive migration, it was flagged in the PR; do not roll back code across it without checking the schema.

## Restore the database from backup (GI-023)

- Managed Postgres backups (daily + point-in-time restore) are enabled on the production database.
- To restore: from the host database dashboard, choose the backup or point-in-time to restore to, and follow the host's restore flow. Confirm the app reconnects and `/health` is green afterward.
- Use this for a bad data event: accidental mass-delete, a data-corrupting bug, or disk failure.

## What to check when it breaks

- `/health` endpoint — is the service up?
- Recent deploys — did something ship just before the breakage? If so, roll back first, diagnose after.
- Stripe webhooks — are payment statuses reconciling? A backlog of failed webhooks means payment status may be stale (the safety poll should catch it).
- Logs — recent errors (no PII in them by policy).
