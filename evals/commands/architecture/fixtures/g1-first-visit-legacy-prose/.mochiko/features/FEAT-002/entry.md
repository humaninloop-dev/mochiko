# FEAT-002 — Customer ETA SMS

**Status:** delivered (reconstructed from code 2026-06-20) · **Spec:** none — reconstructed;
the extent below is the spec of record. Architecture note kept beside it:
`.mochiko/specs/customer-eta-sms/architecture.md`.

## Extent

When a job is assigned, the customer receives one SMS naming the technician and an ETA
computed from the technician's last position; a failed send is retried and surfaced to the
dispatcher. Delivered: the `send_eta_sms` job kind, the worker handler, the `sms_sent` /
`sms_failed` events on the job page.

## Work rows

None pending.
