# Implementation Cycles: FEAT-034 — Cancellation refunds

> Generated from the spec folder and the design inputs the cards were authored from: spec.md, features/FEAT-034/sufficiency-report.md, and the design phase's constraints-and-decisions.md, data-model.md, contracts/, store-delta.md
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | 3 |
| Stories covered | US-1, US-2, US-3 |

## Cycle Format

Each card is one vertical increment: a coherent bundle of named test cases in the `**TEST:**`
grammar that demonstrate together. The builder implements the card test-first, decomposing at
build time; the cycle is done when its named cases show green against real infrastructure. The
card's checkbox is the progress surface. Growth on the standing booking path: no walking
skeleton. `[P]` derives from dependencies.

---

### - [ ] Cycle 1: Cancel with the notice tier

- **Stories:** US-1 — an owner cancels before the first night, the tier is computed, the berth is released
- **Depends on:** —
- **Case:** Simple
- **Brownfield exposure:** modifies `src/bookings/`

**TEST:** Cancelling 20 days out returns tier full and releases the berth
- **Covers**: US-1 / FR-001, FR-002, FR-005
- **Setup**: Seed a confirmed, card-paid booking starting 20 days from today at Kilrush
- **Action**: `curl -s -X POST localhost:3000/api/v1/bookings/$BOOKING/cancel`
- **Assert**: Response status: 200
- **Assert**: Console contains "\"tier\":\"full\""
- **Action**: `curl -s "localhost:3000/api/v1/marinas/$MARINA/berths?from=$FROM&to=$TO"`
- **Assert**: Console contains "\"available\":true"
- **Capture**: console

**TEST:** Cancelling 3 days out returns tier none
- **Covers**: US-1 / FR-002 scenario 2
- **Setup**: Seed a confirmed booking starting 3 days from today
- **Action**: `curl -s -X POST localhost:3000/api/v1/bookings/$BOOKING/cancel`
- **Assert**: Response status: 200
- **Assert**: Console contains "\"tier\":\"none\""
- **Capture**: console

**TEST:** Cancelling after the first night is refused
- **Covers**: US-1 / FR-001 scenario 3
- **Setup**: Seed an active booking that started yesterday
- **Action**: `curl -s -X POST localhost:3000/api/v1/bookings/$BOOKING/cancel`
- **Assert**: Response status: 409
- **Capture**: console

---

### - [ ] Cycle 2: Refund reaches the processor

- **Stories:** US-2 — a non-zero refund is submitted against the original payment and its state follows the processor
- **Depends on:** C1
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** Cancelling a card-paid booking submits a refund
- **Covers**: US-2 / FR-003 / SC-001
- **Setup**: Seed a confirmed booking paid with a Stripe test card, starting 20 days out
- **Action**: `curl -s -X POST localhost:3000/api/v1/bookings/$BOOKING/cancel`
- **Action**: `sleep 60 && stripe refunds list --payment-intent $PI`
- **Capture**: console

**TEST:** A failed refund is shown as failed
- **Covers**: US-2 / FR-003 scenario 2
- **Setup**: Seed a refund in `submitted`; send a `refund.updated` webhook with status failed
- **Action**: `curl -s localhost:3000/api/v1/bookings/$BOOKING/refund`
- **Assert**: Console contains "\"status\":\"failed\""
- **Capture**: console

---

### - [ ] Cycle 3: Marina refund ledger `[P]`

- **Stories:** US-3 — staff see every refund for their marina
- **Depends on:** C2
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** The ledger lists the credit notes issued
- **Covers**: US-3 / FR-006, FR-011
- **Setup**: Two cancellations at Kilrush, tiers full and half
- **Action**: `curl -s localhost:3000/api/v1/marinas/$MARINA/refunds`
- **Assert**: Response status: 200
- **Assert**: Console contains "creditNoteId"
- **Assert**: Two credit notes listed, newest first
- **Capture**: console
