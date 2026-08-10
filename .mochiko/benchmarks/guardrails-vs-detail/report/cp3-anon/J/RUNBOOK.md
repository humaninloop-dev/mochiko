# Runbook

> Operations playbook for the solo operator. Skeleton at setup; filled as the system is built and
> operated. Formal incident-response process is deferred (governance GI-020) — this skeleton
> carries the minimal data-at-risk stub so a day-one financial product is never left with zero
> playbook (cold-review F7).

## Deploy

- _(fill at first deploy: how to promote staging → production on Render)_

## Rollback

- Redeploy the previous Render image; run the contract-phase down-migration where applicable.
- Target: previous version restored in ≤ 15 minutes.
- Destructive migrations are flagged + explicitly approved in the PR (they cannot be rolled back).

## Data at risk (minimal stub)

- **Suspected data loss:** verify the latest PostgreSQL backup exists and restore-check status;
  restore from the verified backup path (GI-012).
- **Suspected data leak / cross-account access:** revoke affected sessions/keys; check the
  cross-account authz tests and access logs; assess scope before any disclosure.
- **Error spike:** triage in Sentry (payloads are PII/financial-scrubbed); correlate by
  correlation ID in structured logs.

## Monitoring

- Health endpoint: `/healthz` (Render).
- Error tracking: Sentry.
