# FEAT-007 — Invoice status tracking

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10)

## Capability

A dedicated, filterable dashboard over all invoices by state (draft/sent/processing/paid/overdue),
so a contractor can slice their receivables. The at-a-glance overdue flag itself ships earlier on
the invoice list (FEAT-002, FR-013); this feature is the richer filterable view on top.

## Extent

- In: filterable dashboard by state; richer receivables slicing over the FEAT-002 overdue flag.
- Not: "viewed" (client-open) tracking — deferred pending an open question on whether it is worth the fuss.

## Relations

- depends-on: FEAT-002 — reads invoices and their states

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-6

## Obligations

- open question rides here: is "viewed" (client-open) tracking in scope for this feature (see spec Open Questions)
