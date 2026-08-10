# Invoice lifecycle v1 — low-fi prototype

Clickable low-fidelity rendering of the spec's Screens & Flows manifest (`../spec.md`).

## How to view

- **Serve (convenience):** `bunx serve .` from this directory, then open the printed URL.
- **Degrade path (no server):** open `index.html` directly in a browser (`file://`). All
  navigation is plain `<a href>` / `<form>` links, so every flow is walkable with no server,
  no build step, and no dependency install.

## Design system

The seed project has no design system or component library (governance surfaces name none).
Screens therefore use neutral grey-box defaults in `assets/lofi.css` — no new visual language
is invented. Fidelity is deliberately low: realistic field shapes and cardinality, rough pixels.

## Authority split

- **Binding:** which screens exist, what each shows, and what the user can do (the SCR/FLOW
  manifest in `spec.md`).
- **Advisory:** layout, styling, copy — all may improve at build time without a spec amendment.

## FEAT tagging

FEAT tags were applied as a re-tag pass after feature derivation. Screens whose feature was
not selected for this delivery round are present but greyed **coming-soon** so navigation never
dead-ends. No stories were filter-rejected, so no screens carry a rejected mark.
