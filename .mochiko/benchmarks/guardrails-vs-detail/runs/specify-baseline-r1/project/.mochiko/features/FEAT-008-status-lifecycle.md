# FEAT-008 — Invoice status lifecycle

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10) — deferred (not selected this round)

## Parent

- [FEAT-007](FEAT-007-payment-follow-up.md)

## Capability

Tracking an invoice beyond sent — registering when a client has viewed it and automatically marking it overdue once its due date passes — so the contractor sees what needs attention without asking.

## Extent

- In: viewed detection, automatic overdue transition on due-date passing, overdue→paid resolution.
- Not: dunning workflows, collections, escalations.

## Relations

- depends-on: FEAT-003 — status lifecycle rides sent invoices.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-7

## Obligations

- deferred SC-007 (shared with FEAT-009) — waits until this feature builds.
