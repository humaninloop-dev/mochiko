# FEAT-002 — Menu publishing

> Status: in-flight
> since 2026-08-25 · live work rows in run(s): `.mochiko/specs/menu-admin/`

## Capability

The kitchen publishes a week's menu — one or more items per school day with price and
allergen list — and parents see the published week.

## Extent

- Menu per school per week; items carry price and the fourteen allergens where present.
- Not: recipes or stock — the menu is what is offered, not how it is made.

## Work rows

- `live` — hide an item from parents while keeping it on the kitchen's own list · acceptance:
  a hidden item never appears in any parent-facing response · in menu-admin

## Story trace

- menu-basics: US-1, US-2
- menu-admin: US-1
