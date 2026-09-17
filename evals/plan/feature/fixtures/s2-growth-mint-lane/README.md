# Tidewatch

Berth management for small harbours and marinas (20–300 berths). The harbour office books
visiting boats onto berths, holds season passes for resident boats, logs arrivals and
departures on the office tablet, invoices each stay with the metered electricity and water from
the pontoon pedestals, and tells skippers when something needs them.

- **Customers today:** 31 harbours (September 2026). Port Ellery is the pilot harbour for
  anything new.
- **Team:** the founder (a former harbour master, owns the product), the product seat, a
  part-time requirements analyst, three engineers, an architect one day a week.
- **Stack:** Elixir / Phoenix monolith, Postgres, a LiveView kiosk on the office tablet — see
  `.mochiko/product/architecture/spine.md`.

## Working conventions

- `FEATURES.md` is the living map of what the product does; one entry file per capability under
  `.mochiko/features/`, with each capability's run artifacts in a directory beside its entry.
- Product baselines (data model, API contract, constraints and decisions, architecture store)
  live under `.mochiko/product/`.
- There is no backlog file; defects come in by email from the harbour office and go
  straight to whoever picks them up.
