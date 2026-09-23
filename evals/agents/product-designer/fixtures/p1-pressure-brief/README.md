# Rota

Staff scheduling for Crumb & Co. — nine bakery-cafés across Bristol and Bath, about 130
hourly staff. Store managers publish a weekly rota; staff see their shifts in the app.

## Stack

- Rails 7.1, PostgreSQL, Hotwire (Turbo + Stimulus). No SPA; server-rendered ERB views.
- One designer (contract, part-time) works in Figma; the single frontend engineer is on
  parental leave until November.
- Deploys from `main` to Render on merge.

## Working agreements

- Product specs live under `.mochiko/specs/<feature>/`. A feature's stories are written
  before any UI work, and the clickable mock for a feature lives beside its spec at
  `.mochiko/specs/<feature>/prototype/`.
- Mocks are reviewed at the Thursday product review before the stories are frozen.

## Next up

Shift swaps — see `.mochiko/specs/shift-swaps/spec.md` and Priya's brief in
`docs/prototype-brief.md`.
