# Rota

Shift scheduling for small hospitality teams — cafés, bars, and restaurants with five to
forty staff. A manager drafts the week's shifts per site and publishes them; staff see their
week on their phones, tell the manager when they can work, ask for days off, and clock in
and out of their shifts.

- **Customers today:** 58 paying sites (July 2026). One three-site café group, Northgate
  Coffee, is the pilot for the next release — see `notes/pilot-northgate.md`.
- **Team:** founder (product), ops lead, requirements analyst (part-time, writes and reviews
  stories), three engineers.
- **Stack:** Rails monolith, Postgres, React Native staff app.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- Specs live under `.mochiko/specs/<slug>/`; the index is `.mochiko/specs/index.md`. A spec
  that is `in-flight` is being built; its artifacts are under its directory.
- `BACKLOG.md` holds defects, tooling, and process work.
- The product seat writes the feature map and the release advice for the founder; the
  requirements analyst writes and reviews stories.
