# FEAT-010 — Payroll export to PayFlow

**Status**: specified · **Owner**: product · **CTO sketch**: `target-architecture.md`

## Overview

Managers approve worked hours in crewboard (FEAT-004) and then, every pay period, download
a CSV and upload it to PayFlow by hand. Uploads are late, venues are missed, and a
mis-typed batch pays people twice. This feature sends each venue's approved hours to
PayFlow automatically every night and shows the manager whether it went through.

## User stories

- **US-001 (P1)** As a manager, approved hours reach PayFlow without me doing anything.
- **US-002 (P1)** As a manager, I can see whether last night's export for each of my venues
  succeeded, and why not if it didn't, and I can re-run it.
- **US-003 (P2)** As an owner, I connect a venue to its PayFlow company once.

## Functional requirements

- **FR-001** Nightly, per venue, all shifts approved since the venue's last successful
  export are sent to PayFlow as one batch.
- **FR-002** The batch id is `crewboard-{venue_id}-{night}`; PayFlow deduplicates by batch
  id (see `docs/payflow-api.md`).
- **FR-003** Re-running a night's export for a venue never duplicates lines at PayFlow.
- **FR-004** The manager sees per venue: pending / uploaded / accepted / rejected, with
  PayFlow's rejection reason.
- **FR-005** A manager can trigger a re-run for one venue and one night.
- **FR-006** The approval flow (manager approves a shift's hours, FEAT-004) is unchanged;
  an exported shift shows as exported and cannot be re-approved without a manager override.
- **FR-007** An owner stores a venue's PayFlow company id and API credential once.

## Non-functional requirements

- **NFR-001** Every venue's export is uploaded between 02:00 and 05:00 local time.
- **NFR-002** A PayFlow outage of up to three hours inside that window loses no export: the
  batch is uploaded when PayFlow returns, or reported failed to the manager by 08:00 local.
- **NFR-003** PayFlow's accepted/rejected result is visible to the manager by 08:00 local.

## Constraints

- **C-001** Render deployment as today: `api`, `worker`, managed PostgreSQL and Redis. No
  new services this quarter. Managed Kafka or Redpanda is not on the approved vendor list
  until the SOC 2 audit closes (2027-Q1).
- **C-002** PayFlow allow-lists one static egress IP per partner account. Render static
  outbound IPs are per service; `worker` has them (enabled for Xero); `api` does not.
- **C-003** Up to 200 venues this year; at most 3,000 approved shifts per venue per month.
- **C-004** Team of three; one release.

## Out of scope

- Pay-rate calculation (PayFlow computes pay from hours and its own rate tables).
- Any payroll provider other than PayFlow.
