# FEAT-003 — Client management

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

A contractor maintains the clients they bill (name, email) — the payer an invoice is addressed to.

## Extent

- In: create / edit / list clients, email validation.
- Not: client portal / login, client-side accounts.

## Relations

- depends-on: FEAT-001 — clients are account-scoped.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-2

## Obligations

- Extend: support a "new client" jump from the invoice editor without losing the in-progress draft (US-3, homed to FEAT-004).
