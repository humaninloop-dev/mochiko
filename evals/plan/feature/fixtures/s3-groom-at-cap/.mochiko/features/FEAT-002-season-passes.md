# FEAT-002 — Season passes

> Status: delivered
> since 2026-06-12 · sticky — live rows may still be visible below

## Capability

A resident boat holds an annual berth on a season pass; the pass renews each year, and pass
holders pay the pass rate for berth nights and services.

## Extent

- A pass covers one named berth for the season, at the pass rate for berth nights and metered services (10 % off).
- A renewal reminder goes to the skipper 30 days before the pass expires.
- Not: monthly passes.
- Not: transferring a pass between boats.

## Relations

- composes-with: FEAT-003 — the pass rate is applied on the invoice
- composes-with: FEAT-005 — renewal reminders go to the skipper

## Architecture

- SPN-001 — Tidewatch web (`.mochiko/product/architecture/`)

## Story trace

- season-passes: US-001, US-002, US-003, US-005
