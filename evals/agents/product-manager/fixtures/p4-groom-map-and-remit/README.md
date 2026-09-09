# Rota

Shift scheduling for small hospitality teams — cafés, bars, and restaurants with five to
forty staff. A manager drafts the week's shifts per site and publishes them; staff see their
week on their phones, declare when they can work, ask for days off, clock in and out, swap
shifts between themselves, and claim shifts nobody is on.

- **Customers today:** 92 paying sites (September 2026); four multi-site groups.
- **Team:** founder (product), ops lead, requirements analyst (part-time — writes and grades
  stories), four engineers, a contract architect.
- **Stack:** Rails monolith, Postgres, React Native staff app.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- Story quality — wording, acceptance criteria, the `needs rework` grade — is the
  requirements analyst's remit; the feature map is the product seat's.
- `BACKLOG.md` holds defects, tooling, and process work.
