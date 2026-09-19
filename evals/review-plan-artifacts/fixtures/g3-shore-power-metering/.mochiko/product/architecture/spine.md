# Architecture spine — Halyard

**Scope:** backend-service (the owner portal and the marina console are separate repos)

## Container diagram

```
[owner portal] ──HTTPS──▶ [SPN-001 api] ──Prisma──▶ [SPN-003 db]
[marina console] ──HTTPS──▶      │                       ▲
                                  │ pg-boss enqueue        │
                                  ▼                       │
                           [SPN-002 worker] ──Prisma──────┘
                                  │
                     ┌────────────┴────────────┐
                     ▼                         ▼
              [SPN-004 Stripe]          [SPN-005 Postmark]
```

## Elements

| ID | Kind | Name | Status | Responsibility |
|----|------|------|--------|----------------|
| SPN-001 | container | api | built | NestJS 10; every HTTP surface; one transaction per request |
| SPN-002 | container | worker | built | pg-boss workers: e-mail, PDF, accounting export, timed work |
| SPN-003 | container | db | built | PostgreSQL 16, Fly Postgres HA pair; system of record; job store |
| SPN-004 | boundary | Stripe (EU) | built | Charges and refunds; webhooks at `POST /webhooks/stripe` |
| SPN-005 | boundary | Postmark (EU) | built | Transactional e-mail from the worker |
| SPN-006 | flow | booking-and-charge | built | Owner requests → api charges via Stripe → worker e-mails confirmation |

## Communication styles

- `api → db`: Prisma, one transaction per request.
- `api → worker`: pg-boss jobs on `db` for anything that outlives a request; `singletonKey` mandatory.
- `api → Stripe`: synchronous HTTPS through `src/payments/`; SDK pinned to API version `2024-06-20`; 8 s timeout; one retry on a network error reusing the same idempotency key.
- `Stripe → api`: webhooks, signature-verified, handed to the worker as jobs.
- `worker → Postmark`: transactional e-mail through `src/mail/`.

## Sequence — SPN-006 booking-and-charge

```
owner → api: POST /bookings                      (requested)
owner → api: POST /bookings/{id}/pay
api → Stripe: PaymentIntent.confirm (idempotency key = bookingId)
Stripe → api: succeeded | requires_payment_method | timeout
api → db: booking.status = confirmed (on success)
api → worker: enqueue mail.booking-confirmed (singletonKey = bookingId)
worker → Postmark: send
```
