# Feature Specification — Volunteer Invitation

- **Feature:** Volunteer Invitation (single)
- **Status:** Shipped (revision 2)
- **Author:** analyst seat
- **Date:** 11 June 2026

## Overview

A coordinator adds a volunteer by name and email; Rota sends the invitation, and the volunteer
joins the organisation by accepting it. Until they accept, they exist only as an invited record
and receive nothing but the invitation itself.

## User Stories

### US-001 — Invite a volunteer (P1)

As a coordinator, I want to add a volunteer and have them invited in one step so that they can pick
up shifts without me chasing them by phone.

- **Given** a coordinator enters a name and an email not already in the organisation, **when** they
  save, **then** a volunteer record is created in the invited state and the invitation email is
  sent within 1 minute.
- **Given** the email already belongs to a volunteer in the organisation, **when** they save,
  **then** the form refuses and shows the existing volunteer.
- **Given** the email belongs to a volunteer deleted within the last 90 days, **when** they save,
  **then** the deleted record is restored with its history rather than a new one created.

**Independent test:** invite a fresh address; confirm the invited record and the email; invite it
again and confirm the refusal.

### US-002 — Accept an invitation (P1)

As a volunteer, I want to accept my invitation in one step so that I start receiving shift offers.

- **Given** a volunteer opens a valid invitation link, **when** they set a password (or sign in),
  **then** they become active, their consent is recorded with a timestamp, and the coordinator
  sees them as active.
- **Given** the link is older than 14 days, **when** it is opened, **then** the page says it has
  expired and offers to ask the coordinator for a new one.

**Independent test:** accept an invitation and confirm the active status and consent timestamp;
open a 15-day-old link and confirm the expiry message.

## Functional Requirements

- **FR-001** Saving a new volunteer MUST create the record in the invited state and MUST send the
  invitation within 1 minute. *Source: US-001*
- **FR-002** The system MUST refuse a second volunteer with the same email in one organisation,
  comparing case-insensitively. *Source: US-001*
- **FR-003** An invitation link MUST expire 14 days after it was sent. *Source: US-002*
- **FR-004** Consent MUST be recorded at acceptance and MUST be present before any scheduling
  message is sent to the volunteer. *Source: US-002*

## Success Criteria

- **SC-001** At least 70% of invitations are accepted within 7 days, measured monthly from
  invited and accepted timestamps.

## Assumptions

- **A-001** The invitation is sent from the organisation's name with the coordinator's name in the
  body; replies go to the coordinator.

## Open Questions

- None open.

## Out of Scope

- Adding volunteers in bulk from a file — its own enrichment and spec.
