# Feature Specification — Workspace Invitations

- **Feature:** Workspace Invitations (FEAT-031)
- **Status:** Ready for review (revision 3)
- **Author:** requirements-analyst seat
- **Date:** 2026-09-09
- **Read by:** product manager (revision 2) · tech lead (revision 2)

## Overview

Teams on Ledgerline add a colleague today by sharing the sign-up link and then asking a
workspace admin to move the new account into the right workspace — a two-step dance that loses
about one in five new members before they see a project. Workspace Invitations lets an
authorised member invite a colleague by email with a role already chosen, so the invitee lands
in the workspace with the right access on first sign-in.

## User Stories

### US-001 — Invite a colleague by email (P1)

As a workspace admin, I want to invite a colleague by email with a role already chosen so that
they land in the workspace with the right access on first sign-in.

- **Given** an admin enters a valid email address and chooses a role, **when** they send the
  invitation, **then** the invitee receives an email with an accept link and the invitation
  appears in the workspace's Pending list with its role and expiry.
- **Given** an admin enters an address that already belongs to a workspace member, **when**
  they send, **then** the form says the person is already a member and sends nothing.
- **Given** an admin enters an address with a pending invitation, **when** they send, **then**
  the form offers to resend the existing invitation instead of creating a second one.

**Independent test:** invite a fresh address and confirm the email arrives and the Pending list
shows the invitation with role and expiry; invite the same address again and confirm only the
resend offer appears.

### US-002 — Accept an invitation (P1)

As an invitee, I want to accept an invitation in one step so that I do not have to ask anyone
to add me after signing up.

- **Given** an invitee opens a valid accept link, **when** they sign in or create an account
  with the invited address, **then** they become a member of the workspace with the role named
  in the invitation and the invitation leaves the Pending list.
- **Given** an invitee opens an accept link after its expiry, **when** the page loads, **then**
  it explains that the invitation has expired and names who to ask for a new one.
- **Given** an invitee signs in with an address other than the invited one, **when** they try
  to accept, **then** the page refuses and shows which address the invitation was sent to.

**Independent test:** accept one invitation with the invited address and confirm membership and
role; open an expired link and confirm the expiry message; accept with a different address and
confirm the refusal.

### US-003 — Manage pending invitations (P2)

As a workspace admin, I want to see, resend, and revoke pending invitations so that a wrong
address or a changed decision does not leave a door open.

- **Given** the Pending list shows an invitation, **when** an admin revokes it, **then** the
  accept link stops working immediately and the invitation leaves the list.
- **Given** the Pending list shows an invitation, **when** an admin resends it, **then** a
  fresh email goes out and the expiry restarts.

**Independent test:** revoke an invitation and confirm its link refuses; resend another and
confirm a second email arrives with a later expiry.

## Functional Requirements

- **FR-001** The system MUST let a workspace admin, or a member holding the *Invite people*
  permission, send an invitation to an email address for exactly one workspace.
  *Source: US-001*
- **FR-002** The inviter MUST choose the invitee's role from the workspace's role list (Viewer,
  Member, Admin) at send time, and the accepted member MUST receive exactly that role.
  *Source: US-001, US-002*
- **FR-003** The system MUST reject an invitation to an address that already belongs to a
  member of that workspace and MUST offer a resend when a pending invitation for the address
  exists. *Source: US-001*
- **FR-004** An invitation MUST expire 14 days after it was last sent; an expired link MUST
  show the expiry message and MUST NOT grant membership. *Source: US-002*
- **FR-005** The system MUST grant membership only when the accepting account's verified
  address matches the invited address. *Source: US-002*
- **FR-006** Revoking an invitation MUST invalidate its link within one second; resending MUST
  issue a new link and restart the expiry. *Source: US-003*
- **FR-007** The system MUST delete an invitation row on acceptance, expiry, or revocation,
  retaining no record of the invitee's address beyond that point. *Source: US-002, US-003*

## Success Criteria

- **SC-001** At least 70% of invitations sent are accepted within 7 days, measured monthly
  from the invitations table.
- **SC-002** The median time from an invitation being sent to the new member's first sign-in
  is under 48 hours, measured from the `invited_at` and `first_sign_in_at` fields the
  membership record carries.
- **SC-003** Support requests tagged "add me to a workspace" fall by 80% within 60 days of
  release, measured from the support desk's request tags.
- **SC-004** Zero memberships are granted to an address other than the invited one, measured by
  a nightly check of membership rows against the acceptance audit entries.

## Out of Scope

- Bulk invitations from a CSV — a separate import flow with its own error handling.
- Invitations to a single project rather than the workspace — project access is a permissions
  change, not an invitation.
- Single sign-on provisioning — SSO workspaces provision members from the identity provider and
  do not use invitations.

## Open Questions

- None outstanding. The two questions from revision 1 (expiry length, resend limit) were
  resolved with the PM and folded into FR-004 and FR-006.
