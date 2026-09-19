# Feature Specification — Member Sign-up

- **Feature:** Member Sign-up (portal)
- **Status:** Shipped (revision 3)
- **Author:** analyst seat
- **Date:** 22 May 2026

## Overview

A prospective member joins a studio from the studio's public page without visiting the desk:
they pick a plan, enter their card, and can check in the same day. Before this, every sign-up was
keyed by front-desk staff from a paper form.

## User Stories

### US-001 — Join a studio online (P1)

As a prospective member, I want to pick a plan and pay from the studio's page so that I can start
training today without waiting for the desk.

- **Given** a visitor chooses a plan and enters a valid card, **when** they confirm, **then** the
  membership is active, the first charge is taken, and a welcome email with the billing day and
  the plan's terms arrives within 2 minutes.
- **Given** the card is declined, **when** they confirm, **then** the form says the card was
  declined, keeps everything they typed, and no membership is created.
- **Given** the studio has a notice period configured, **when** the visitor views the plan, **then**
  the notice period is shown before they confirm.

**Independent test:** join a UK test studio on a monthly plan with a test card; confirm the
welcome email names the billing day and the 30-day notice; check in the same day.

### US-002 — Add a linked member to a Family plan (P2)

As a payer on a Family plan, I want to add a family member from my portal so that they get their
own login without visiting the desk.

- **Given** a payer has fewer than four linked members, **when** they add one by email, **then** the
  person receives a login invitation and appears as a linked member on the payer's plan.
- **Given** a payer has four linked members, **when** they try to add a fifth, **then** the form
  says the plan is full and offers the desk's contact.

**Independent test:** add a linked member by email to a test Family plan; confirm the invitation
arrives and the member shows as linked.

## Functional Requirements

- **FR-001** The system MUST show the plan price, the billing day the member will get, and any
  configured notice period before the visitor confirms. *Source: US-001*
- **FR-002** The system MUST NOT create a membership unless the first charge succeeds. *Source:
  US-001*
- **FR-003** A welcome email MUST be sent within 2 minutes of a successful sign-up and MUST name
  the plan, the billing day, and the notice period if any. *Source: US-001*
- **FR-004** A Family plan MUST hold at most four linked members in addition to the payer.
  *Source: US-002*
- **FR-005** A linked member MUST have their own login and MUST NOT see the payer's billing.
  *Source: US-002*

## Success Criteria

- **SC-001** At least 60% of new memberships at studios with the portal enabled are created
  online rather than at the desk, measured monthly per studio.
- **SC-002** Fewer than 2% of online sign-ups raise a support ticket within 14 days, measured from
  ticket tags.

## Assumptions

- **A-001** The billing day is the join day; a join on the 29th, 30th, or 31st bills on the last
  day of shorter months (matches the desk behaviour).
- **A-002** A visitor may hold only one membership per studio; a second sign-up with the same
  email is refused with a pointer to the portal login.

## Open Questions

- None open. The notice-period display question from revision 1 was settled with Dan (18 May).

## Out of Scope

- Sign-up for annual prepaid plans online (card limits at some studios; staff-only for now).
- Corporate memberships paid by an employer.
