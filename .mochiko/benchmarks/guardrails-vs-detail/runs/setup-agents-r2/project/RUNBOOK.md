# Runbook

Operational procedures for the one-person deployment. Adopted as a KM elective (deployed service).
Fill in as procedures are learned.

## Rollback

- Redeploy the last good image on Render. Target: back online in ≤15 minutes.
- Destructive/irreversible migrations are flagged in the PR and approved explicitly before running.

## Backup & restore

- Managed Postgres automated backups (Render). A restore drill MUST be executed and verified
  before launch, and periodically after. Record each drill here with its date and outcome.

## Email deliverability

- Sending domain configured with SPF, DKIM, DMARC. Verify records at launch; re-check on any DNS
  or email-provider change.

## Incident response

_(Deferred — SLO/on-call standards excluded until real traffic, GI-021. Capture ad-hoc steps here
as incidents occur.)_
