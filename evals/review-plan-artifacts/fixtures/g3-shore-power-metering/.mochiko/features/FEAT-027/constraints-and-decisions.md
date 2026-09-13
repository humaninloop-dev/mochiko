# Constraints and Decisions: FEAT-027 (delta over the product baseline)

> technical-analyst · 2026-09-05, re-issued 2026-09-10 (round 2) · /mochiko:implement FEAT-027
> design phase. Rows added beside `.mochiko/product/constraints-and-decisions.md`, numbering
> continued. Baseline rows C-001–C-004, D-001–D-003, IP-001–IP-002 apply unchanged.

## Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-005 | infrastructure | Enerlink gateway API v3 | blocking |
| C-006 | organizational | Marina operations | significant |

### C-005: Enerlink exposes readings hourly, by pull

**infrastructure · blocking · source:** Enerlink gateway API v3 (memory-asserted) — the gateway stores a cumulative reading per meter once an hour and exposes them through a pull endpoint; there is no per-reading push and no sub-hourly reading.
**Impact:** bounds how fresh consumption can be · shapes D-004

### C-006: Only marina staff set tariffs

**organizational · significant · source:** marina operations — Halyard staff never set or edit a marina's tariff; the marina is the authority on its own price.
**Impact:** eliminates a Halyard-side tariff console · shapes the tariff endpoint's authorisation [TODO] confirm the marina-admin role name

## Decisions

### Decision Summary  *(the ID index)*

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-004 | Ingest mode | Worker pulls readings hourly from the Enerlink API | C-005 |
| D-005 | Readings storage | Hand-rolled monthly ring-buffer tables rotated by a job | — |

### D-004: Ingest mode

**Context** (how readings get from the gateway into Halyard) · **Shaped by:** C-005

| Option | Pros | Cons |
|--------|------|------|
| Gateway pushes to a Halyard endpoint | Fresh; no polling | Enerlink v3 has no push (C-005) |
| Worker pulls hourly from the Enerlink API | Matches what the gateway offers; retries are ours | Freshness bounded by the hour |

**Choice:** hourly pull by the worker — **Rationale:** C-005 rules push out: the gateway cannot push, so pulling on the hour is the only ingest the vendor supports, and the worker already owns scheduled work (D-001).
**Consequences:** consumption freshness is up to 60 min · backfill is the same pull with a wider window (FR-005)

### D-005: Readings storage

**Context** (readings grow by ~1.4 M rows a year per 100 marinas; queries are by meter over a stay) · **Shaped by:** —

| Option | Pros | Cons |
|--------|------|------|
| Twelve monthly `meter_readings_MM` tables, rotated and pruned by a worker job; a view unions them | Predictable size; prune is a `TRUNCATE TABLE` | Rotation job to write; the view is ours to maintain |

**Choice:** monthly ring-buffer tables — **Rationale:** the rotation is simple and we keep the schema fully in our hands.
**Consequences:** a rotation job (D-001) · queries go through the union view · a MeterReading maps to whichever monthly table holds it

## Declarations

### Declaration Summary  *(the ID index)*

| ID | Kind | Source | Downstream home |
|----|------|--------|-----------------|
| INT-003 | integration | FR-001 | POST /gateway/readings (`x-integration`) |

### INT-003: Enerlink gateway

**integration · criticality: degraded · source:** FR-001 — the feature MUST integrate with the Enerlink gateway; its unavailability delays readings, never bookings.
**Authored downstream:** POST /gateway/readings (`mochiko:patterns-api-contracts`)
