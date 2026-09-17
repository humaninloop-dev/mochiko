# Payment recording and reminders

> Spec: payment-recording
> Created: 2026-08-04
> Status: closed — accepted 2026-08-11, both capability-batch runs delivered 2026-08-18

---

## Intent

- **Scope boundary:** a studio records what a client paid and Ledgerlite chases what they
  have not; nothing the client does themselves.
- **Delivery:** whole feature specified; payment recording ships first, reminders second.
- **Depth / rigor:** production; money paths under the money-handling rule.
- **UX-bearing:** yes — a record-payment form on the studio's invoice page and a balance
  block on the public link.
- **Constraints:** email only, through Postmark; no client accounts.
- **Out of scope:** client-initiated payment; refunds; SMS reminders.
- **Capability frame:** two new capabilities — payment recording, payment reminders — both a
  new kind of thing the product does; invoice sending's public page extends to show a balance.

---

## Overview

A studio marks an invoice as paid, in full or in part, and the invoice keeps its own
balance. Unpaid invoices are chased by email on a schedule the studio sets once.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Record a payment against an invoice | P1 | FEAT-003 | homed |
| [US-2](stories/US-2.md) | Chase an unpaid invoice on a schedule | P1 | FEAT-004 | homed |
| [US-3](stories/US-3.md) | Export payments to my accounting software | P3 | — | rejected — an integration wish, not something the product does yet; parked in `BACKLOG.md` |

---

## Edge Cases

- A payment larger than the balance is rejected, never credited.
- A reminder due on a paid invoice is skipped, not queued.
- A voided invoice drops out of the reminder schedule.

---

## Functional Requirements

- FR-010: The system MUST accept a payment with amount, date, method, and optional note.
- FR-011: The system MUST reject a payment that would take the balance below zero.
- FR-012: The system MUST flip an invoice to `paid` when its balance reaches zero.
- FR-013: The system MUST send up to three reminder emails at studio-set offsets after due.
- FR-014: The system MUST stop reminders the moment an invoice is paid or voided.

---

## Key Entities

- **Payment** — amount (minor units + currency), date, method, note, recorded-by.
- **Reminder schedule** — up to three day offsets, one per studio.
- **Invoice balance** — derived: total minus the sum of payments.

---

## Success Criteria

- SC-010: A 1,000.00 + 250.00 pair of payments on a 1,250.00 invoice reads `paid` with balance 0.
- SC-011: An overpayment attempt is rejected with the balance unchanged.
- SC-012: A reminder scheduled for day 7 goes out on day 7 and not on a paid invoice.

---

## Screens & Flows

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Record payment | Studio enters a payment | amount, date, method, note, running balance | FEAT-003 |
| SCR-002 | Invoice status block | Balance and payments on the studio page and the public link | balance, payments list, status | FEAT-003 |
| SCR-003 | Reminder schedule | Studio sets the offsets once | three offsets, preview of the next send | FEAT-004 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Record a payment | SCR-002 -> record -> SCR-001 -> save -> SCR-002 | US-1 / full payment | FEAT-003 |
| FLOW-002 | Set the schedule | SCR-003 -> save -> SCR-003 | US-2 / offsets saved | FEAT-004 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with bun
or open `prototype/index.html` directly. Flows and data are binding; layout and styling
advisory.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-003 | Payment recording | new (`proposed`) | US-1 | SC-010, SC-011 |
| FEAT-004 | Payment reminders | new (`proposed`) | US-2 | SC-012 |

### Filter rejections

- US-3 — rejected: an integration wish, not something the product does yet; parked in `BACKLOG.md`.

### Selection

- **Selected (build now, dependency order):** FEAT-003, FEAT-004
- **Deferred (`pending` row on FEAT-004):** per-client reminder cadence — carries no SC; ruled
  2026-08-11 as a later increment.
- **Deferred SCs:** none

---

## Assumptions

- Studios reconcile bank transfers by hand; nothing here reads a bank feed.
