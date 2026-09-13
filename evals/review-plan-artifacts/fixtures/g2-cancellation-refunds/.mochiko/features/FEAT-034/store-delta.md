# Architecture store delta — FEAT-034 (draft, unsigned)

> principal-architect · 2026-09-10 · /mochiko:implement FEAT-034 design phase. Drawn against the
> standing store at `.mochiko/product/architecture/`.

## Container diagram (delta)

```
[owner portal] ──HTTPS──▶ [SPN-001 api] ──pg-boss──▶ [SPN-007 refund-worker] ──HTTPS──▶ [SPN-004 Stripe]
                                │                              │
                                ▼                              ▼
                          [SPN-003 db]                  [SPN-008 EventStoreDB]
```

## Elements

| ID | Kind | Name | Status | Change |
|----|------|------|--------|--------|
| SPN-007 | container | refund-worker | in-flight (FEAT-034) | new — a dedicated worker process for refund submission, the outbox poller (D-005), and the EventStoreDB projection (D-006) |
| SPN-008 | boundary | EventStoreDB | in-flight (FEAT-034) | new — the refund event store |
| SPN-009 | flow | cancel-and-refund | in-flight (FEAT-034) | new (FR-003) — owner cancels via api; refund-worker submits to Stripe; webhook back to api; projection to EventStoreDB |

**Ruling (SPN-007):** money movement isolated from the request path so a Stripe slowdown never
holds an api machine. Sequence diagram for SPN-009 to follow.

## Concern rows

| Row | Change |
|-----|--------|
| AX-002 Datastore | stance stays `decided`; sign-off for EventStoreDB requested from Marcus, not yet recorded |

## Consult

Root `ARCHITECTURE.md` index read.
