# Architecture — Tidewatch (derived index)

Derived from the architecture store at `.mochiko/product/architecture/spine.md` and
`.mochiko/product/architecture/concerns.md` — do not hand-edit; edit the store and re-render.

## Elements

- SPN-001 Tidewatch web (container, built) · SPN-002 Postgres (container, built) · SPN-003 Office
  kiosk (container, built) · SPN-004 Nightly pedestal import (flow, built)

## Concerns

- AX-001 tenancy (decided, built) · AX-002 card payment (n-a, accountant) · AX-003 offline kiosk
  (not-now)

## Health view

- open rows: 0
- stale `not-now` triggers: 0
- fired triggers awaiting routing: 1 — AX-003 (fired 2026-09-10)
- orphan elements: 1 — SPN-005 keys `removing (FEAT-012)`; no FEAT-012 exists
- drift register: 0
