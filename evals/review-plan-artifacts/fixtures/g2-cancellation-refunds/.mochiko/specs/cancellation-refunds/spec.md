# Feature Specification — Cancellation refunds (FEAT-034)

- **Status:** accepted 2026-09-04 · **Author:** requirements analyst · **Owner:** Ines (product)

## Intent

- Scope boundary: an owner cancels a confirmed booking before it starts; refund by notice tier; the marina's view of refunds.
- Delivery: whole feature in one batch; first shippable value is cancel-and-refund.
- Depth / rigor: high (money moves).
- UX-bearing: no — the portal and console teams build against the contract; prototype waived at intent.
- Constraints: Stripe only (C-001); one datastore (C-003); money moves on authority.
- Out of scope: marina-initiated cancellations; partial-stay refunds after the first night; disputes.

## Overview

An owner who can no longer make a stay cancels it from the portal and is refunded according to
how much notice they gave. The berth goes back on sale, the money goes back where it came from,
and the marina can see what was refunded and why.

## User Stories

| ID | Story | Priority | Feature | Disposition |
|----|-------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Cancel a booking and see the refund amount by notice tier | P1 | FEAT-034 | homed |
| [US-2](stories/US-2.md) | The refund reaches the payment method I paid with | P1 | FEAT-034 | homed |
| [US-3](stories/US-3.md) | Marina admin sees a refund ledger | P2 | FEAT-034 | homed |

## Edge Cases

- Cancellation on the first night or later: refused; the stay has started.
- The original card has expired or been removed: Stripe still refunds to the original card via the scheme; the owner is told it may take longer.
- The booking was paid by Bacs Direct Debit: refund goes back to the same bank account; timing differs from card.
- Two cancel requests for the same booking (double click): one cancellation, one refund.
- Notice computed in the marina's timezone at the moment of cancellation, not the owner's.

## Functional Requirements

- **FR-001** An owner MUST be able to cancel a `confirmed` booking at any time before its first night. *Source: US-1*
- **FR-002** The refund amount MUST be 100 % of the paid amount with 14 or more days' notice, 50 % with 7 to 13 days, and 0 % with fewer than 7 days, notice measured to the first night in the marina's timezone. *Source: US-1*
- **FR-003** A refund MUST be returned to the payment method the booking was paid with. *Source: US-2*
- **FR-004** On cancellation the owner MUST see the refund amount, the tier applied, and when to expect the money. *Source: US-1, US-2*
- **FR-005** Cancelling MUST release the berth for the booking's dates. *Source: US-1*
- **FR-006** Marina staff MUST be able to see every refund for their marina: booking, owner, amount, tier, status, and date. *Source: US-3*

## Key Entities

- **Refund** — the money returned for one cancelled booking: amount, tier, status against the processor.

## Success Criteria

- **SC-001** A refund is initiated within 60 seconds of the cancellation, p95, measured from the booking's `cancelledAt` to the processor's refund `created` timestamp.
- **SC-002** Fewer than 2 % of refunds lead to a support ticket, measured monthly from the helpdesk tag.
- **SC-003** 95 % of card refunds settle to the cardholder within 5 business days, measured from the processor's refund events.

## Screens & Flows

No UX surface — prototype waived at intent.
