# FEAT-002 — Client management

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

A contractor maintains the clients they bill — creating, editing, and listing client records so invoices can be addressed to a saved client.

## Extent

- In: create / edit / list clients (name, email, billing address); required-field validation; archive/hide a client (no hard-delete when invoices exist, FR-022).
- Not: client portal accounts or any client-facing login (out of scope for v1).

## Relations

- depends-on: FEAT-001 — clients are owned by an account.
- composes-with: FEAT-003 — invoices are addressed to a client.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-2

## Obligations

- Cross-cutting: SC-008 / FR-016 (WCAG 2.1 AA) binds this feature's contractor-facing screens.
- Client archive default ties to the unresolved data-retention open question — revisit when retention is settled.
