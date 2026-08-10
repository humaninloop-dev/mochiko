# FEAT-007 — Client records

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Capability

Maintain a contractor's saved clients — name, email, optional mailing address — that invoices are
issued against, scoped to the contractor's account.

## Extent

- In: add / list / select a client (name + email required, mailing address optional).
- Not: client logins or a client portal (out of scope); phone / notes (deferred); Stripe customer sync.

## Relations

- (foundation — no inbound dependency; FEAT-001 depends-on this)

## Architecture

- _Pending first plan — no components built yet (greenfield)._

## Story trace

- invoice-lifecycle-v1: US-8

## Obligations

- Tenant isolation on client reads/writes (GI-011).
