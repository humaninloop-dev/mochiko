# FEAT-002 — Invoice authoring

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10)

## Capability

Contractors draft and edit invoices for a client — line items (description, quantity, unit
price), tax rate, due date, invoice number, and an optional memo — with subtotal, tax, and total
computed exactly, saved and reopenable as a draft.

## Extent

- In: create/edit/save draft invoices; line items with exact totals (half-up, tax on subtotal); tax rate; due date; invoice number (unique per contractor); memo; a basic list that flags overdue on read (unpaid past due date).
- Not: recurring invoices, multi-currency, estimates/quotes (out of scope v1); the dedicated filterable status dashboard (FEAT-007).

## Relations

- depends-on: FEAT-001 — an invoice is drafted for a client record

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-2

## Obligations

- (none)
