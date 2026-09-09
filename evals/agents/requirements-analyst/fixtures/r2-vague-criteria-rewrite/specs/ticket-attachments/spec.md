# Feature Specification — Ticket Attachments

- **Feature:** Ticket Attachments
- **Status:** Draft, awaiting analyst revision
- **Author:** Maya (PM)
- **Date:** 2 September 2026

## Overview

Requesters and agents need to attach files to tickets: screenshots of an error, a log file, a
PDF invoice. Today the only way a file reaches a ticket is as an email attachment on an inbound
message; anything opened or answered in the web portal or the agent workspace has no way to carry
a file, so people fall back to email or to pasting text. Attachments lets a requester attach files
when opening a ticket or replying in the portal, lets an agent attach files to a reply, and lets
everyone on the ticket open them in place.

## User Stories

### US-001 — Attach a file when opening or replying to a ticket (P1)

As a requester, I want to attach files when I open or reply to a ticket in the portal so that the
agent can see exactly what I see without me describing it.

- **Given** a requester is composing a new ticket in the portal, **when** they add a file and
  submit, **then** the file appears on the ticket for the agent.
- **Given** a requester is replying, **when** they attach a file, **then** the reply carries the
  file.
- **Given** the upload fails, **when** the requester looks at the form, **then** they see a
  user-friendly error message.

**Independent test:** open a ticket with a screenshot attached and confirm the agent sees it.

### US-002 — Attach a file to an agent reply (P1)

As an agent, I want to attach files to my reply so that I can send the requester a document or an
annotated screenshot instead of describing it.

- **Given** an agent is composing a reply, **when** they attach a file and send, **then** the
  requester receives the reply with the file.
- **Given** an agent attaches a file, **when** the upload is in progress, **then** the experience
  is seamless.

**Independent test:** reply with a PDF attached and confirm the requester can open it.

### US-003 — Open an attachment in place (P2)

As an agent, I want to preview an attachment inside the ticket view so that I do not have to
download every screenshot to see what the requester is talking about.

- **Given** a ticket carries an image attachment, **when** the agent opens the ticket, **then**
  the image is shown inline in the conversation.

**Independent test:** open a ticket with a PNG attached and confirm it renders inline.

## Functional Requirements

- **FR-001** The system MUST accept file uploads on ticket creation and on replies from both the
  portal and the agent workspace. *Source: US-001, US-002*
- **FR-002** Uploads MUST be fast. *Source: US-001, US-002*
- **FR-003** The system MUST support large files. *Source: US-001*
- **FR-004** Attachments MUST be secure. *Source: US-001, US-002*
- **FR-005** The system MUST show a user-friendly error message when an upload fails. *Source:
  US-001*
- **FR-006** The system MUST preview image and PDF attachments inline in the ticket view.
  *Source: US-003*
- **FR-007** The upload experience MUST feel seamless. *Source: US-002*
- **FR-008** Agents always work from a desktop browser, so uploading from a mobile device is not
  required for the agent workspace. *Source: US-002*
- **FR-009** Attachments MUST work on all browsers. *Source: US-001, US-002, US-003*
- **FR-010** The system MUST handle many concurrent uploads. *Source: US-001*
- **FR-011** An attachment MUST be visible only to the ticket's requester, the agents assigned
  to the ticket, and admins. *Source: US-001, US-002*

## Success Criteria

- **SC-001** Users are happy with attachments.
- **SC-002** Fewer support tickets about attachments after launch.
- **SC-003** Attachments are used on a meaningful share of tickets.

## Assumptions

- None recorded.

## Open Questions

- Maximum file size — engineering to decide.
- Retention — how long attachments are kept after a ticket closes. Legal was asked in August and
  has not answered.
- Should the attachment list on a ticket show newest first or oldest first? Nobody has an opinion.
- How long a file name is shown before it is cut off in the list.

## Out of Scope

- Attachments on internal notes between agents (a later cut).
- Editing or annotating images inside Deskline.
