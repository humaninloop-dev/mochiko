# FEAT-009 — Automated reminders

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10) — deferred (not selected this round)

## Parent

- [FEAT-007](FEAT-007-payment-follow-up.md)

## Capability

Automatically emailing a client reminders on unpaid invoices on a cadence the contractor can see and adjust, and stopping immediately once the invoice is paid — so the contractor never chases payment by hand.

## Extent

- In: configurable reminder cadence, automatic reminder sends on unpaid invoices, stop-on-payment (online or manual), reminder sends recorded on the invoice.
- Not: SMS/other channels, per-client custom messaging templates.

## Relations

- depends-on: FEAT-008 — reminders key off overdue/unpaid status.
- depends-on: FEAT-003 — reminders reuse the authenticated-email send path.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-8

## Obligations

- deferred SC-007 (shared with FEAT-008) — waits until this feature builds.
- Open question: reminder cadence defaults/bounds unsettled — resolve when specified for build.
