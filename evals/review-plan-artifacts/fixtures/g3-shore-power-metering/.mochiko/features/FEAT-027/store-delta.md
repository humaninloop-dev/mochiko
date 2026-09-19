# Architecture store delta — FEAT-027 (signed 2026-09-08 at the round-1 design checkpoint)

> principal-architect · 2026-09-05 · /mochiko:implement FEAT-027 design phase. Drawn against the
> standing store at `.mochiko/product/architecture/`; signed by the user at the round-1
> checkpoint; folds into the store at landing.

## Container diagram (delta)

```
[SPN-008 Enerlink gateway] ──HTTPS push──▶ [SPN-007 meter-ingest] ──Prisma──▶ [SPN-003 db]
                                                                                   ▲
[owner portal] ──HTTPS──▶ [SPN-001 api] ───────────────────────────────────────────┘
                                                                     [SPN-002 worker] ─ invoice lines
```

## Elements

| ID | Kind | Name | Status | Change |
|----|------|------|--------|--------|
| SPN-007 | container | meter-ingest | in-flight (FEAT-027) | new — receives gateway pushes on its own listener, validates and de-duplicates, writes readings; isolated from api so a burst never holds an api machine |
| SPN-008 | boundary | Enerlink gateway | in-flight (FEAT-027) | new — external; pushes readings, exposes a store for backfill |
| SPN-009 | flow | gateway-push | in-flight (FEAT-027) | new — gateway → meter-ingest → db; owner reads via api |

**Ruling (SPN-007):** a separate ingest container: push volume is bursty (every meter reports on
the hour) and a slow write must not hold the request path. **Ruling (SPN-009):** push over pull:
the 5-minute visibility target (FR-002) needs the gateway to tell us, not us to ask.

## Sequence — SPN-009 gateway-push

```
gateway → meter-ingest: POST batch (signed)
meter-ingest → meter-ingest: validate; drop duplicates (meterId, readAt)
meter-ingest → db: insert readings (commit)
owner → api: GET /berths/{id}/consumption
api → db: readings since stay start
```

## Concern rows

| Row | Change |
|-----|--------|
| AX-013 Telemetry ingest | `open` → `decided` · Ruling: push-based ingest through `meter-ingest`; readings are the system of record for consumption · **NFR-005 — Ingest lag** · performance · source: SC-001 · **Target:** p95 ≤ 5 min from `readAt` to the consumption endpoint reflecting the reading · **Measured:** `readAt` vs `MeterReading.createdAt` plus endpoint span, rolling 24 h, continuous |
| AX-002 Datastore | no change; readings in PostgreSQL |

## Consult

Root `ARCHITECTURE.md` index, `concerns.md` AX summary table, `spine.md` deep view (structural trigger). Trip: AX-013 open row touched — ruled here.

## Deployment view

IP rows: none added; `meter-ingest` runs on the existing worker machine class. Absence of a separate deployment view recorded.
