# FEAT-006 — Invoice void

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10) — derived and specified, deferred to a fast-follow

## Capability

Void an incorrect sent invoice so it is no longer payable, retaining the record and recording who
voided it and when.

## Extent

- In: void a `sent`/overdue invoice; block further payment; retain the record; audit-trail write; refuse voiding a `paid` invoice.
- Not: deletion (records are retained).

## Relations

- depends-on: FEAT-001.

## Story trace

- invoice-lifecycle-v1: US-5

## Obligations

- Audit-trail write on void (GI-029; principal-ruled parity with payment state).
- Carries the deferred void-audit clause of SC-004 until built.
