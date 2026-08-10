# FEAT-001 — Invoice authoring & sending

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Capability

Create, edit, number, and send invoices against a saved client — line items, tax, and a due date —
delivering the hosted payment link to the client by authenticated email.

## Extent

- In: draft create/edit; tax-inclusive totals (Decimal, cents); per-contractor sequential, non-editable invoice numbers; send (`draft`→`sent`); payment-link generation; authenticated email delivery + copyable fallback; edit/resend of a `sent` unpaid invoice (locked once `paid`); transitions recorded to the audit trail.
- Not: payment collection (FEAT-003); status display (FEAT-005); void of a paid invoice (FEAT-006).

## Relations

- depends-on: FEAT-007 — an invoice is issued against a saved client.

## Architecture

- _Pending first plan — no components built yet (greenfield)._

## Story trace

- invoice-lifecycle-v1: US-1

## Obligations

- Audit-trail write on send/edit (GI-029).
- Authenticated invoice email (GI-030) — pulled into v1 core by principal ruling; reminder email (FEAT-008) stays deferred.
- Tenant isolation on invoice writes (GI-011).
- Money as Decimal / no floating point (GI-013).
