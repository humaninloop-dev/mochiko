# FEAT-001 — Client records

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Capability

Ledgerline keeps the contractor's client records — the people they bill — scoped to their own
account, so every invoice has an addressee.

## Extent

- Create, list, and soft-delete clients; email required; records isolated per contractor account.
- Soft-delete retains the client's invoices and audit history — never a cascade delete.
- Not: client logins or a client portal; client-side editing of their own record.

## Relations

- (none — foundation feature)

## Architecture

- None yet — greenfield; realizing components filled at the first `/mochiko:plan`.

## Story trace

- invoice-lifecycle-v1: US-1

## Obligations

- (none)
