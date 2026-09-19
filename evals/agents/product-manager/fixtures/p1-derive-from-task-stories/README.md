# Rota

Shift scheduling for small hospitality teams — cafés, bars, and restaurants with five to
forty staff. A manager drafts the week's shifts per site and publishes them; staff see their
week on their phones, tell the manager when they can work, and ask for days off.

- **Customers today:** 41 paying sites (June 2026), mostly single-location cafés; two small
  groups with three sites each.
- **Team:** one founder (product), one ops lead (runs onboarding and support, writes most
  stories), three engineers.
- **Stack:** Rails monolith, Postgres, a React Native staff app. The `shifts` table still
  lives in the original SQLite-backed reporting sidecar (see `BACKLOG.md`).

## What the product does today

See `FEATURES.md` — the living map of capabilities. Three are delivered: rota building,
staff availability, and time-off requests. Everything else is a story, not a capability,
until the map says otherwise.

## Working conventions

- Specs live under `.mochiko/specs/<slug>/`; the index is `.mochiko/specs/index.md`.
- `BACKLOG.md` holds defects, tooling, and process work.
- The product seat writes the feature map; the ops lead and the requirements analyst write
  and review stories.
