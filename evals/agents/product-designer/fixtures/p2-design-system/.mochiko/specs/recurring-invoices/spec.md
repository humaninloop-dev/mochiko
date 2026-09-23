# Feature Specification — Recurring invoices

- **Status:** stories drafted · design review Monday · **Author:** Tom (product) · **Date:** 2026-09-04

## Summary

A freelancer who bills the same client the same amount every month re-creates the invoice by
hand today. Let them turn an invoice into a schedule that sends itself, see what is scheduled,
pause or end it, and find out when a send could not happen.

## User Stories

### US-001 — Make an invoice recurring (P1)

As a freelancer, I want to turn an existing invoice into a recurring schedule, so that I stop
re-creating it every month.

- **Scenario 1 — Given** a sent or paid invoice, **when** I choose to make it recurring and
  set a cadence (monthly or quarterly), a first send date, and either an end date or a number
  of sends, **then** a schedule is created and I see its next send date.
- **Scenario 2 — Given** the schedule form, **when** I set a first send date earlier than
  today, **then** I am told the date must be today or later and nothing is created.

Independent test: from a paid invoice, create a monthly schedule ending after 6 sends and see
it listed with next month's date.

### US-002 — See my schedules (P1)

As a freelancer, I want to see all my recurring schedules in one place, so that I know what
will go out and when.

- **Scenario 1 — Given** I have schedules, **when** I open Recurring, **then** I see each
  schedule's client, amount, cadence, next send date, and status.
- **Scenario 2 — Given** I have no schedules, **when** I open Recurring, **then** I am shown
  how to create one from an invoice.

Independent test: open Recurring with three schedules and see all three with their next dates;
open it on a fresh account and see the guidance.

### US-003 — Pause or end a schedule (P2)

As a freelancer, I want to pause or end a schedule, so that a project that stops does not
keep getting billed.

- **Scenario 1 — Given** an active schedule, **when** I pause it, **then** no invoice is sent
  until I resume it and the schedule shows as paused.
- **Scenario 2 — Given** an active or paused schedule, **when** I end it and confirm,
  **then** it is marked ended and cannot be resumed.

Independent test: pause a schedule and see it paused; end one and see it cannot be resumed.

### US-004 — Know when a send could not happen (P2)

As a freelancer, I want to be told when a scheduled invoice could not be generated, so that
I can fix it before the client notices.

- **Scenario 1 — Given** a schedule whose client has since been archived, **when** the send
  date arrives, **then** the schedule shows as failed with the reason and I am given a way to
  fix it.

Independent test: archive a client with an active schedule, pass the send date, and see the
failed schedule with its reason.

## Functional Requirements

- **FR-001** A schedule MUST be created from an existing invoice; it copies the invoice's line
  items, client, and currency. *Source: US-001*
- **FR-002** Cadence MUST be one of monthly or quarterly. *Source: US-001*
- **FR-003** A paused schedule MUST NOT generate invoices. *Source: US-003*
- **FR-004** An ended schedule MUST NOT be resumable. *Source: US-003*

## Screens & Flows

_To be filled by the prototype._
