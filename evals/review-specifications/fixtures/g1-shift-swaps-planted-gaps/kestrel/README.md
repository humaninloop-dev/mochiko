# Kestrel

Rota and shift management for independent cinemas, theatres, and arts venues — front-of-house
teams of eight to sixty: ushers, box office, bar, and duty managers. A venue manager publishes
the week's rota; staff see their shifts on their phones, tell the manager when they can work,
ask for days off, and clock in and out on the venue tablet.

- **Customers today:** 41 venues (August 2026), from a single-screen cinema to a three-stage
  theatre. The Regal group (four cinemas) is the pilot for the next release.
- **Team:** founder (product), one venue-operations lead, a part-time requirements analyst,
  three engineers.
- **Stack:** Django monolith, PostgreSQL, React Native staff app, a venue tablet web app for
  clock-in.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- Specs live under `.mochiko/specs/<slug>/`; the index is `.mochiko/specs/index.md`.
  Only open specs keep a workspace in the tree; a closed spec's workspace is archived out of it.
- Decisions the team has already taken are under `docs/decisions/` — numbered, dated, one per
  file. Facts about venues and staff that shape features are under `docs/`.
- `BACKLOG.md` holds defects, tooling, and process work.

## Venue facts that keep coming up

- Bar shifts need staff aged 18 or over who hold the venue's alcohol-service sign-off; box
  office shifts need till training. Ushering has no prerequisite. Details in
  `docs/roles-and-eligibility.md`.
- Not every staff member uses the app on shift. The 2026 survey is in `docs/staff-devices.md`.
