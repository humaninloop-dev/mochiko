# Implementation Cycles: FEAT-027 — Shore-power metering

> Generated from the spec folder and the design inputs the cards were authored from: spec.md, features/FEAT-027/sufficiency-report.md, and the design phase's constraints-and-decisions.md, data-model.md, contracts/, and the signed store delta. Re-issued 2026-09-10 (round 2).
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | 5 |
| Stories covered | US-1, US-2, US-3 |

## Cycle Format

Each card is one vertical increment: a coherent bundle of named test cases in the `**TEST:**`
grammar that demonstrate together. The builder implements the card test-first, decomposing at
build time; the cycle is done when its named cases show green against real infrastructure. The
card's checkbox is the progress surface. A new end-to-end path opens (gateway to portal), so
Cycle 1 is a walking skeleton. `[P]` derives from dependencies.

---

### - [ ] Cycle 1: Walking skeleton — a pushed reading round-trips

- **Stories:** US-1 — thinnest path: one reading in from the gateway, one reading out to the owner
- **Depends on:** —
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** A pushed reading is stored and visible
- **Covers**: US-1 / FR-001, FR-002
- **Setup**: Seed a metered berth with a booking; `$SIG` computed with the marina's secret
- **Action**: `curl -s -X POST localhost:3000/api/v1/gateway/readings -H "X-Enerlink-Signature: $SIG" -d '{"marinaId":"'$MARINA'","readings":[{"gatewayMeterId":"KIL-0042","recordedAt":"2026-09-10T10:00:00Z","energyKwh":100.5}]}'`
- **Assert**: Response status: 202
- **Action**: `curl -s localhost:3000/api/v1/berths/$BERTH/consumption`
- **Assert**: Console contains "100.5"
- **Capture**: console

---

### - [ ] Cycle 2: Readings storage layer and repository

- **Stories:** US-1 — the monthly ring-buffer tables (D-005), the union view, and the repository methods the later cycles read through
- **Depends on:** C1
- **Case:** Split — storage is its own concern and lands before the views that use it
- **Brownfield exposure:** none

**TEST:** Repository suite passes against the partitioned tables
- **Covers**: US-1 / FR-001
- **Action**: `npm test -- readings.repository`
- **Assert**: Console contains "passing"
- **Capture**: console

---

### - [ ] Cycle 3: Consumption view for a stay `[P]`

- **Stories:** US-1 — the owner sees kWh per day and the running total for the current stay
- **Depends on:** C1
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** Consumption lists readings newest first with a running total
- **Covers**: US-1 / FR-002 scenario 2
- **Setup**: Seed three readings for the berth's meter across two days
- **Action**: `curl -s "localhost:3000/api/v1/berths/$BERTH/consumption?limit=10"`
- **Assert**: Response status: 200
- **Assert**: Console contains "totalKwh"
- **Assert**: Console contains "energyKwh"
- **Capture**: console

---

### - [ ] Cycle 4: Invoice line at the tariff

- **Stories:** US-2 — the monthly invoice carries a shore-power line priced from the readings
- **Depends on:** C3
- **Case:** Simple
- **Brownfield exposure:** extends `src/invoicing/`

**TEST:** The shore-power line prices the month's kWh at the tariff
- **Covers**: US-2 / FR-003 / SC-002
- **Setup**: Tariff 20p to 15 September, 30p from 16 September; readings: 10 kWh used on 10 September, 10 kWh used on 20 September
- **Action**: `npm run invoice:generate -- --month 2026-09 --marina $MARINA`
- **Assert**: Console contains "shore-power"
- **Assert**: The shore-power line total is 600 (20 kWh at the tariff current at generation, 30p)
- **Capture**: console

---

### - [ ] Cycle 5: Tariff periods `[P]`

- **Stories:** US-3 — staff add a tariff; overlaps are refused
- **Depends on:** C1
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** An overlapping tariff is refused
- **Covers**: US-3 / FR-004 scenario 2
- **Setup**: An existing tariff to 30 September at Kilrush
- **Action**: `curl -s -X PUT localhost:3000/api/v1/marinas/$MARINA/tariffs -d '{"pencePerKwh":30,"validFrom":"2026-09-15"}'`
- **Assert**: Response status: 409
- **Capture**: console

**TEST:** A tariff from 1 October is saved as upcoming
- **Covers**: US-3 / FR-004 scenario 1
- **Setup**: An existing tariff to 30 September at Kilrush
- **Action**: `curl -s -X PUT localhost:3000/api/v1/marinas/$MARINA/tariffs -d '{"pencePerKwh":30,"validFrom":"2026-10-01"}'`
- **Assert**: Response status: 201
- **Capture**: console
