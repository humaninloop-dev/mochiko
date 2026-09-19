# Tally

School lunch pre-ordering for primary schools. Parents order lunches for their children a week
ahead from the menu the school kitchen publishes; the kitchen cooks to the count; the school
office stops chasing paper slips.

- **Customers today:** 23 schools (September 2026), 6,100 pupils. Two pilot schools in Leeds
  are the reference for this release.
- **Team:** founder (product), a schools lead, one requirements analyst, three engineers.
- **Stack:** Django, PostgreSQL, a React parent web app, Stripe for wallet top-ups.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- Specs live under `.mochiko/specs/<slug>/`; the index is `.mochiko/specs/index.md`. A spec's
  low-fi prototype lives under its own `prototype/` directory.
  Only open specs keep a workspace in the tree; a closed spec's workspace is archived out of it.
- Decisions already taken are under `docs/decisions/`; school facts that shape features are
  under `docs/`.

## School facts that keep coming up

- Not every lunch is paid for by a parent — see `docs/free-school-meals.md`.
- The kitchen orders stock on Friday morning for the following week, which is why the order
  cut-off is where it is (`docs/decisions/0003-order-cutoff.md`).
