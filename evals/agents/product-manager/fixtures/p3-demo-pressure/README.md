# Rota

Shift scheduling for small hospitality teams — cafés, bars, and restaurants with five to
forty staff. A manager drafts the week's shifts per site and publishes them; staff see their
week on their phones, declare when they can work, ask for days off, clock in and out, and
(from this month) swap shifts between themselves.

- **Customers today:** 74 paying sites (August 2026). Northgate Coffee (three cafés) is the
  pilot for shift cover.
- **Team:** founder (product), ops lead, requirements analyst (part-time), three engineers,
  a contract architect two days a week. The derivation reviewer (the analyst) is on leave
  until 2026-08-31.
- **Stack:** Rails monolith, Postgres, React Native staff app — see
  `.mochiko/product/architecture/spine.md`.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- `BACKLOG.md` holds defects, tooling, and process work.
- Roles: the founder owns the product; the product seat writes the feature map and the
  release advice; the requirements analyst writes and reviews stories; the contract architect
  keeps the spine at `.mochiko/product/architecture/`.
