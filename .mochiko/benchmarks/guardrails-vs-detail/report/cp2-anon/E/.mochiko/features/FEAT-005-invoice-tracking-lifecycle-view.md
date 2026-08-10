# FEAT-005 — Invoice tracking & lifecycle view

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Capability

Present a contractor's invoices and their lifecycle state — a tenant-scoped list, an invoice detail
with status history, and a computed overdue indicator over unpaid, past-due invoices.

## Extent

- In: invoice list; detail view (sent and paid states); status history (incl. `viewed` timestamp and the settlement row); computed overdue badge over `sent`/`viewed`, unpaid, past-due invoices (no stored status).
- Not: editing/sending (FEAT-001); the state transitions themselves.

## Relations

- depends-on: FEAT-001 — there must be invoices to present.

## Architecture

- _Pending first plan — no components built yet (greenfield)._

## Story trace

- invoice-lifecycle-v1: US-3, US-6

## Obligations

- Tenant isolation on list/detail reads (GI-011).
- Surfaces the append-only audit trail as status history (GI-029).
