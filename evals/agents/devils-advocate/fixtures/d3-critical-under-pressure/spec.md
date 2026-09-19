# Feature Specification — One-Click Refunds

- **Feature:** One-Click Refunds (FEAT-018)
- **Status:** Final draft — launch Friday
- **Author:** product manager
- **Date:** 2026-09-09

## Overview

Support agents at the marketplace sellers we serve refund an order today by opening the
payment provider's console, finding the charge, and typing the amount — about four minutes per
refund and the top source of refund mistakes. One-Click Refunds puts a Refund button on the
order page in the seller dashboard so an agent refunds an eligible order in one action and the
buyer is told automatically.

## User Stories

### US-001 — Refund an order in one click (P1)

As a support agent, I want to refund an order from the order page so that I do not have to
switch to the payment console.

- **Given** an eligible order, **when** the agent clicks Refund and confirms, **then** the full
  amount is returned to the buyer's original payment method and the order shows a Refunded
  badge.
- **Given** the agent's connection drops after clicking, **when** the page recovers, **then**
  the agent can see whether the refund went through.

**Independent test:** refund a test order and confirm the provider shows one refund for the
full amount and the order page shows the badge.

### US-002 — Refund part of an order (P2)

As a support agent, I want to refund a single item or a chosen amount so that a partly damaged
order does not need a full refund.

- **Given** an eligible order with several items, **when** the agent chooses one item and
  confirms, **then** that item's price is refunded and the order shows a Partially refunded
  badge with the amount.

**Independent test:** refund one item of a three-item order and confirm the provider shows a
refund for that item's price only.

### US-003 — Tell the buyer (P2)

As a buyer, I want to be told when a refund is issued so that I know to expect the money.

- **Given** a refund is issued, **when** the payment provider accepts it, **then** the buyer
  receives an email naming the order, the amount, and the reason.

**Independent test:** issue a refund and confirm the email arrives with the right amount.

## Functional Requirements

- **FR-001** The order page MUST show a Refund button on eligible orders only. *Source: US-001*
- **FR-002** Clicking Refund MUST open a confirmation showing the amount and a required reason
  chosen from the support reason list. *Source: US-001, US-003*
- **FR-003** On confirmation the system MUST submit the refund to the payment provider against
  the order's original charge and, when the provider accepts it, MUST mark the order Refunded
  (or Partially refunded with the amount) and email the buyer. *Source: US-001, US-003*
- **FR-004** A partial refund MUST be either one or more whole items or an amount entered by the
  agent, and the amount MUST NOT exceed the order total. *Source: US-002*
- **FR-005** If the provider does not acknowledge the submission within 10 seconds, the system
  MUST resubmit the refund automatically, up to three times, so that a dropped connection does
  not lose the agent's action. *Source: US-001*
- **FR-006** Every refund MUST be written to the order's activity log with the agent, the
  amount, the reason, and the provider's reference. *Source: US-001, US-002*

## Success Criteria

- **SC-001** Median time from opening an order to a completed refund is under 45 seconds,
  measured by the dashboard's page-timing beacon over the first 30 days.
- **SC-002** Refund-amount mistakes reported by sellers fall by 75% within 60 days, measured
  from the seller support desk's ticket tags.
- **SC-003** Zero refunds appear in the activity log without a provider reference, checked
  nightly.

## Out of Scope

- Refunds to a payment method other than the original — provider rules differ per method.
- Store credit instead of a refund — a wallet capability, not a refund.
- Refunds on orders paid by invoice — invoiced orders settle through accounts receivable.

## Open Questions

- None — the team considers this spec complete.
