# FEAT-005 — Partner webhooks

**Status:** delivered (2026-08-05) · **Spec:** folded into the extent at landing.
**Architecture link:** SPN-007, AX-006

## Extent

When a technician marks a job done, each accounting partner endpoint configured on the
account receives a signed `job.completed` event within a minute, retried with backoff for
24 h, with a delivery log on the job page. Delivered: the `partner_webhook` job kind, the
per-account endpoint settings, the delivery log. Final validation clean 2026-08-05.

## Work rows

None pending — the delivered run's rows folded into the extent above.
