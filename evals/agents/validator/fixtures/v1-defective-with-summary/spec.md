# Feature Specification — Saved Searches

- **Feature:** Saved Searches (FEAT-014)
- **Status:** Draft for validation
- **Author:** requirements-analyst seat
- **Date:** 2026-09-08

## Overview

Support agents at mid-size helpdesk tenants re-type the same ticket filters many times a day —
by queue, priority, SLA breach state, and assignee — because the console has no way to keep a
filter. Saved Searches lets an agent name a filter once, recall it in one click, and share it
with their team, so the daily triage starts from a known view instead of a rebuilt one.

## User Stories

### US-001 — Save the current filter (P1)

As a support agent, I want to save the filter I have built so that I can return to the same
view tomorrow without rebuilding it.

- **Given** an agent has applied one or more filters on the ticket list, **when** they choose
  *Save search* and enter a name, **then** the search appears in their *Saved* list with that name
  and reopens with the same filters applied.
- **Given** the agent enters a name already used by one of their saved searches, **when** they
  confirm, **then** the console asks whether to overwrite or rename, and nothing is saved until
  they choose.

**Independent test:** save a two-clause filter, reload the console, recall it, and compare the
result set with the original.

### US-002 — Share a saved search with the team (P2)

As a team lead, I want to share a saved search with my team so that everyone triages from the
same queue view.

- **Given** a saved search owned by the lead, **when** they mark it *Shared with team*, **then**
  every member of that team sees it under *Team searches* in read-only form.
- **Given** a shared search, **when** a team member edits its filters, **then** the edit is saved
  as that member's own copy and the shared original is unchanged.

**Independent test:** share one search from a lead account, sign in as a member, confirm it is
listed, edit it, and confirm the original is unchanged from the lead account.

### US-003 — Pin a saved search to the sidebar (P3)

As a support agent, I want to pin up to three saved searches to the sidebar so that my most
used views are one click away.

- **Given** an agent has at least one saved search, **when** they pin it, **then** it appears in
  the sidebar above the queue list, and a fourth pin is refused with a message naming the limit.

**Independent test:** pin three searches, attempt a fourth, confirm the refusal message and that
the first three remain pinned after a reload.

## Functional Requirements

- **FR-001** The system MUST persist a saved search as a name plus the exact filter clauses in
  effect at save time, scoped to the owning agent. *Source: US-001*
- **FR-002** The system MUST reject a saved-search name that is empty, longer than 60
  characters, or a duplicate of another search owned by the same agent, unless the agent
  explicitly chooses to overwrite. *Source: US-001*
- **FR-003** The system MUST let a team lead mark a search as shared with exactly one team, and
  MUST present shared searches to members as read-only. *Source: US-002*
- **FR-004** When a member edits a shared search, the system MUST create a private copy for that
  member and MUST NOT modify the shared original. *Source: US-002*
- **FR-005** The system MUST retain an agent's saved searches for TBD days after the agent's
  account is deactivated, then delete them. *Source: US-001*
- **FR-006** The system SHOULD allow at most three pinned searches per agent and MUST refuse a
  fourth pin with a message that names the limit. *Source: US-003*

## Success Criteria

- **SC-001** An agent recalls a saved search and sees the filtered list within 1.5 seconds at the
  95th percentile, measured by the console's existing page-timing beacon over one week of
  production traffic.
- **SC-002** At least 40% of active agents on a tenant have saved one or more searches within
  30 days of the feature's release to that tenant, measured from the saved-search table.
- **SC-003** Agents perceive saved-search recall as responsive and the sharing model as fair,
  measured by team consensus at the sprint demo.
- **SC-004** Zero shared-search edits by members modify the original, measured by a nightly
  integrity job comparing shared-search rows against their last lead-authored revision.

## Out of Scope

- Cross-tenant sharing of saved searches — tenant isolation is a platform constraint and
  sharing across it needs a separate design.
- Scheduled email digests of a saved search's results — a notifications capability, not a
  search capability.
- Saved searches for the customer-facing portal — the portal has its own filter model and no
  agent workflow.

## Open Questions

- Whether a team lead can un-share a search that members have already copied is undecided; the
  default in this draft is that copies survive un-sharing.
