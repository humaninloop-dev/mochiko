# FEAT-008 — Overdue reminder emails

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10) — derived and specified, deferred to a fast-follow

## Capability

Automatically email a client overdue reminders on a fixed cadence (3, 7, and 14 days overdue), with
a per-invoice off toggle, stopping on settlement.

## Extent

- In: reminder scheduling at 3/7/14 days overdue; per-invoice off toggle; stop-on-paid/void.
- Not: arbitrary custom cadences; dunning / dispute handling (out of scope).

## Relations

- depends-on: FEAT-001.
- depends-on: FEAT-005 — reuses the computed-overdue read.

## Story trace

- invoice-lifecycle-v1: US-7

## Obligations

- Authenticated transactional email + bounce handling (GI-030).
- Carries the deferred SC-006 until built.
