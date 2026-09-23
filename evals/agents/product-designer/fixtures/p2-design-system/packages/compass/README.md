# Compass

The Northstar design system. Three parts:

- `tokens.css` — the design tokens as CSS custom properties. Every colour, spacing step,
  radius, and type size the product uses is here; nothing else is used.
- `src/` — the React components. Props and states are catalogued in `COMPONENTS.md`.
- `COMPONENTS.md` — the catalogue: what exists, its variants, its states, and what it is for.

Rules of the road: use the tokens by name, never a raw value; use the catalogue's component
names in designs and mocks so engineers and designers talk about the same thing; a new
pattern is a design-guild decision, not a local one.
