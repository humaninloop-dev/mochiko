# Constraints and decisions — FEAT-013 Webhook subscriptions

- **Author:** tech-lead seat (this run — design phase, earlier turn)
- **Date:** 2026-09-08

## Hard constraints

- **C-001** Delivery is at-least-once; a failed delivery is retried with exponential backoff
  for 24 hours, then marked dead. *Source: FR-003.*
- **C-002** At most five in-flight deliveries per endpoint at any moment. *Source: shop
  integrators' receivers are small; ShopLoop partner terms §6.*
- **C-004** No recipient contact data in any payload. *Source: carried over from FEAT-011.*

## Non-functional requirements

- **NFR-006** 95% of deliveries within 60 s of `occurred_at`. *Source: SC-001.*

## Technology decisions

- **D-001** Deliveries run as Celery tasks on the existing worker; retries use Celery's
  `retry(countdown=…)` with the backoff schedule below.
- **D-002** Signing per AX-009: HMAC-SHA256 over the body in `X-Notify-Signature`.
- **D-003** Retry schedule: 5 attempts at 1, 5, 15, 30, and 60 minutes — one hour in total —
  after which the delivery is marked dead.
- **D-004** Per-endpoint concurrency is capped with a Redis semaphore keyed by subscription.
