# FEAT-002 — Delivery scheduling

> Status: delivered
> since 2026-02-16 · sticky — live rows may still be visible below

## Capability

A customer picks a delivery day available on their route and can skip a single upcoming
delivery before the cut-off; a skipped week is not charged.

## Extent

- Delivery day per route; a skip applies to one named week and shows on the account page.
- Not: more than two consecutive skips — the third is refused with a message pointing at
  cancellation.

## Story trace

- delivery-days: US-1, US-2
- skip-a-week: US-1
