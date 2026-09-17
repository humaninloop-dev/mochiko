# FEAT-001 — Berth booking

> Status: delivered
> since 2026-03-20 · sticky

## Capability

A visiting boat books a berth for a range of dates; the harbour office confirms it, and the
berth plan shows the boat on its berth for those nights.

## Extent

- A booking is one night at a time; a longer visit is entered as consecutive nights by the office.
- The office confirms or declines; the skipper is told either way.
- The berth plan shows the boat on its berth for each confirmed night.
- Not: a multi-night stay as one booking with its own rate.
- Not: bookings taken from an agency or a third-party portal.

## Relations

- composes-with: FEAT-003 — a confirmed booking opens the stay the invoice bills
- composes-with: FEAT-005 — confirmation and decline go to the skipper

## Architecture

- SPN-001 — Tidewatch web (`.mochiko/product/architecture/`)
- SPN-003 — Office kiosk, the berth plan (`.mochiko/product/architecture/`)

## Story trace

- berth-booking: US-001, US-002, US-004
