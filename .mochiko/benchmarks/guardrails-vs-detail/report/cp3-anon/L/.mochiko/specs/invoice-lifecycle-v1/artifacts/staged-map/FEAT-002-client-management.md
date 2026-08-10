# FEAT-002 — Client management

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10)

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Maintain the people a contractor bills — name, email, and optional contact details — as reusable
records an invoice is addressed to.

## Extent

- In: create, list, and edit clients; email required and validated; clients are per-tenant (contractor-owned).
- Not: client login or self-service accounts (no client portal in v1); soft-delete/merge of clients.

## Relations

- (foundation — no dependencies; the first buildable leaf)

## Story trace

- invoice-lifecycle-v1: US-1
