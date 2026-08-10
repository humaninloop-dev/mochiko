# FEAT-001 — Client management

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

Contractors keep records of the clients they bill — name, email, and optional business details —
reused across every invoice. Each contractor's clients are private to their account.

## Extent

- In: create, edit, and list clients; select a client when drafting an invoice; add a client inline from the invoice editor.
- Not: client login/portal accounts (out of scope v1); client-side data entry.

## Relations

- depends-on: FEAT-009 — clients are scoped to the authenticated contractor

## Architecture

- (greenfield — component realized at plan/implement time; ARCHITECTURE.md link added then)

## Story trace

- invoice-lifecycle: US-1

## Obligations

- (none)
