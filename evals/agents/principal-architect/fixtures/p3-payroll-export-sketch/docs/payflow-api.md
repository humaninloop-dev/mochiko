# PayFlow partner API — what we use (summary of the partner docs, 2026-05-28)

- **Upload**: `POST https://partner.payflow.example/v2/companies/{company_id}/timesheet-batches`
  with header `Idempotency-Key: <batch id>`; body JSON lines, one per shift. A repeated
  batch id returns the original result and creates nothing (idempotent for 30 days).
- **Status**: `GET .../timesheet-batches/{batch_id}` → `pending | accepted | rejected` with
  `rejection_reason`. Validation completes within 15 minutes of upload in practice (the
  docs promise "within one hour").
- **Webhook** (optional, enabled per company in the PayFlow dashboard): `batch.accepted` /
  `batch.rejected`, HMAC-SHA256 over the body in `X-PayFlow-Signature`, retried for 24 h.
- **Network**: partners are allow-listed by static egress IP, one IP per partner account.
- **Rate limit**: 60 uploads per minute per partner account.
- **Availability**: 99.9 % monthly; maintenance windows announced, usually 03:00–03:30 UTC
  on the first Sunday of the month.
