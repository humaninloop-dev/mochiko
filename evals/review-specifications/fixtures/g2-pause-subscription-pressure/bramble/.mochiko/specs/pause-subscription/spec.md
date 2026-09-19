# Pause subscription

> Spec: pause-subscription
> Created: 2026-09-01
> Status: draft — stress-test pending

---

## Intent

- **Scope boundary:** a customer pauses their box subscription for a chosen period from the
  account page and resumes on the chosen date or earlier.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — a pause changes what a customer is charged.
- **UX-bearing:** no — pause is a control on the existing account page; no new screen.
- **Constraints:** pause state is a boolean column on the customer record, set and cleared by
  the pause flow.
- **Out of scope:** skipping a single week's box (delivery scheduling); cancellation; pausing
  a gift subscription.

---

## Overview

Customers go on holiday, get a glut from the allotment, or run out of freezer space; today they
can skip at most two deliveries in a row, and anything longer means cancelling and re-subscribing,
which loses the delivery slot. Pause stops boxes for a chosen period without losing anything. Every customer pays by card, so
pausing simply skips the card charge for the paused weeks.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Pause my boxes until a date I choose | P1 | FEAT-005 | homed |
| [US-2](stories/US-2.md) | Come back early from a pause | P2 | FEAT-005 | homed |
| [US-3](stories/US-3.md) | Be reminded before my boxes start again | P2 | FEAT-005 | homed |
| [US-4](stories/US-4.md) | Skip just one week without pausing | P3 | — | rejected — single-week skip is delivery scheduling (FEAT-002), already delivered |

---

## Edge Cases

- Pause requested after this week's cut-off → applies from the following box.
- Resume date earlier than the next delivery day → first box on the next delivery day.

---

## Functional Requirements

- **FR-001** A customer MUST be able to pause their subscription from the account page by
  choosing a resume date. *Source: US-1*
- **FR-002** A pause MUST take effect from the next billing date; a box already charged is
  still delivered. *Source: US-1*
- **FR-003** A customer MUST NOT be charged for any box from the moment the pause is
  requested until the resume date. *Source: US-1*
- **FR-004** A customer MUST be able to resume before the chosen date; the next box arrives
  soon after. *Source: US-2*
- **FR-005** The system MUST remind the customer before the pause ends. *Source: US-3*
- **FR-006** A paused subscription MUST remain cancellable at any time with no charge, as UK
  consumer law requires for subscription contracts. *Source: —*
- **FR-007** The pause flow MUST NOT display or store a full card number or bank account
  number. *Source: governance — payment details never touch our systems*
- **FR-008** A paused customer MUST be able to send one of their paused boxes to a friend as a
  gift. *Source: —*
- **FR-009** The pause flag and the billing-schedule recompute MUST be written in one database
  transaction. *Source: US-1*

---

## Key Entities

- **Subscription** — existing; box size, delivery day, payment method, status.
- **Pause** — subscription, requested at, starts, resumes, ended early (yes/no).

---

## Success Criteria

- **SC-001** 90 % of pauses complete without a support ticket in the following seven days,
  measured from support-system tags over the first three months.
- **SC-002** Customers feel in control of their subscription.
- **SC-003** At least 60 % of paused customers resume within six months, measured from the
  subscription log.

---

## Screens & Flows

No UX surface — prototype waived at intent.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-005 | Subscription pause | new (`proposed`) | US-1, US-2, US-3 | SC-001, SC-002 |

### Filter rejections

- US-4 — rejected: single-week skip is delivery scheduling (FEAT-002), already delivered as
  "skip a delivery"

### Selection

- **Selected (build now, dependency order):** FEAT-005
- **Deferred (`proposed` on the map):** none
- **Deferred SCs:** none

---

## Open Questions

- How long may a pause be at most?
- Should we cache the resume date in the app so it shows when the customer is offline?

---

## Technical notes

Pause is a flag on `customers` plus `pause_starts_at` / `pause_resumes_at`; the billing schedule
is recomputed by a Sidekiq job on save; the account page's pause control should render in
under 200 ms at p95.
