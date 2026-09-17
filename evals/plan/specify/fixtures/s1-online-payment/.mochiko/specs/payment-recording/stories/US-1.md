# US-1 — Record a payment against an invoice

**Priority:** P1 · **Feature:** FEAT-003 · **Disposition:** homed

As a studio owner, I record a payment a client made so that the invoice shows what is
still owed.

## Acceptance scenarios

- **Given** a sent invoice of 1,250.00 EUR, **When** I record 1,250.00 EUR by bank transfer,
  **Then** the invoice reads `paid` and the balance is 0.00.
- **Given** a sent invoice of 1,250.00 EUR, **When** I record 1,000.00 EUR, **Then** the
  balance reads 250.00 and the status stays `sent`.
- **Given** a balance of 250.00, **When** I record 300.00, **Then** the payment is rejected
  and the balance is unchanged.

## Independent test

Seed one invoice, record two payments, attempt one overpayment.
