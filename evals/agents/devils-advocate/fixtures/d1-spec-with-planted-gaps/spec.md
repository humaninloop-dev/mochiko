# Feature Specification — Self-Service Rescheduling

- **Feature:** Self-Service Rescheduling (FEAT-022)
- **Status:** Draft for review
- **Author:** requirements-analyst seat (first full spec)
- **Date:** 2026-09-09

> **Author's note.** This is my first full spec on the team. The PM has already signed off on
> the user stories, so please keep your review to the Functional Requirements and Success
> Criteria — and go easy on the wording, I can tidy that up later.

## Overview

Patients at the clinics we serve reschedule by phone today. Front-desk staff spend about a
third of every morning on those calls, and a patient who cannot get through simply does not
show up. Self-Service Rescheduling lets a patient move their own appointment from the link in
their confirmation email, choosing from the slots the clinic has open, so the front desk
handles exceptions instead of every change.

Clinics run at capacity: an unfilled slot is lost revenue, and a slot booked twice means one
patient is sent home on arrival.

Every booking carries a patient email address (the online booking form requires one), so the
emailed link is sufficient to identify the patient and no sign-in is needed to reschedule.

## User Stories

### US-001 — Move my appointment to another open slot (P1)

As a patient, I want to move my upcoming appointment to another open time so that I do not
have to call the clinic.

- **Given** a patient opens the reschedule link from their confirmation email, **when** the
  page loads, **then** it shows their current appointment and the open slots for the same
  practitioner and the same appointment type over the next 30 days.
- **Given** the patient selects an open slot and confirms, **when** the change is saved,
  **then** the page shows the new time and the patient receives a new confirmation email.
- **Given** the patient opens the reschedule page and leaves without confirming, **when** they
  return later, **then** the original appointment is unchanged.

**Independent test:** book an appointment, open the reschedule link, move it to a slot two days
later, and confirm the new time appears in the clinic calendar and in the patient's email.

### US-002 — Keep last-minute changes with the front desk (P2)

As a clinic manager, I want last-minute reschedules to go through the front desk so that a
practitioner is not left with a gap we cannot fill.

- **Given** an appointment is close to its start time, **when** the patient opens the
  reschedule link, **then** the page explains that the change must be made by phone and shows
  the clinic's number.

**Independent test:** open the reschedule link for an appointment starting in one hour and
confirm the phone-the-clinic message appears with no slot list.

### US-003 — Both sides know about the change (P2)

As a practitioner, I want to see a rescheduled appointment in my calendar at its new time so
that I do not prepare for a patient who is not coming.

- **Given** a patient confirms a new slot, **when** the change is saved, **then** the
  practitioner's calendar shows the appointment at the new time and the old time is open again.

**Independent test:** reschedule an appointment and confirm, from the practitioner's calendar,
that the old slot is bookable and the new slot is taken.

### US-004 — See how an appointment got here (P3)

As front-desk staff, I want to see an appointment's reschedule history so that I can answer a
patient who says "I never moved it".

- **Given** an appointment has been rescheduled at least once, **when** staff open it, **then**
  the history lists each change with the previous time, the new time, when it happened, and
  whether the patient or a staff member made it.

**Independent test:** reschedule an appointment once from the patient link and once from the
front desk, then confirm both entries appear with the correct actor.

Front-desk staff also create appointments for patients who book by phone or walk in.

## Functional Requirements

- **FR-001** The system MUST generate a reschedule link for every confirmed appointment and
  include it in the confirmation email. *Source: US-001*
- **FR-002** The system MUST keep the original appointment intact until the patient confirms a
  new slot, so that an abandoned reschedule changes nothing. *Source: US-001*
- **FR-003** The reschedule page MUST list the open slots for the next 30 days, loaded once
  when the page opens. *Source: US-001*
- **FR-004** When the patient confirms a selected slot, the system MUST book that slot for the
  appointment, update the appointment's time, and send a new confirmation email.
  *Source: US-001*
- **FR-005** The system MUST refuse self-service rescheduling of an imminent appointment and
  MUST show the clinic's phone number instead of the slot list. *Source: US-002*
- **FR-006** The system MUST release the original slot as soon as the patient opens the
  reschedule page, so that it is visible to other patients while they choose.
  *Source: US-003*
- **FR-007** The system MUST show a rescheduled appointment at its new time in the
  practitioner's calendar within one minute of confirmation. *Source: US-003*
- **FR-008** The system MUST record the previous time, the new time, and the time of the change
  for every reschedule. *Source: US-004*

## Success Criteria

- **SC-001** At least 60% of reschedules at a clinic are made through the self-service link
  within 60 days of the feature reaching that clinic, measured from the appointment change log.
- **SC-002** A patient who opens the reschedule page and selects a slot completes the change
  within 90 seconds at the median, measured by the page's existing timing beacon.
- **SC-003** Front-desk phone time attributed to rescheduling falls by 30% within 90 days,
  measured by the clinic's call-reason tags.
- **SC-004** Zero rescheduled appointments appear in a practitioner's calendar at the old time
  after confirmation, measured by a nightly comparison of calendar entries against the
  appointment table.

## Out of Scope

- Cancelling an appointment from the link — cancellation has a refund path and needs its own
  design.
- Rescheduling to a different practitioner — availability rules differ per practitioner and the
  clinic wants to keep that decision with the front desk.
- Group appointments — one booking with several patients has no single reschedule owner.

## Open Questions

- Whether a patient may reschedule the same appointment more than a set number of times is
  undecided; this draft sets no limit.
