# FEAT-005 — Partner webhooks

**Status:** delivered (2026-08-05) · **Spec:** folded into the extent at landing.
**Architecture link:** SPN-007, AX-006

## Extent

When a technician marks a job done, each accounting partner endpoint configured on the
account receives a signed `job.completed` event within a minute, retried with backoff for
24 h, with a delivery log on the job page.

## Work rows

None pending.
