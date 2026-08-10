# Invoice lifecycle v1 — low-fi prototype

Clickable low-fidelity specification artifact for the `invoice-lifecycle` spec. It renders the
user stories as screens and walkable flows so the experience is visible before build.

## How to view

- **Serve (convenience):** `bunx serve .mochiko/specs/invoice-lifecycle/prototype/` then open the printed URL.
- **Degrade path (no server):** open `index.html` directly in a browser (`file://`). All navigation is
  plain `<a href>` / `<form action>` — no build step, no framework, no dependency install required.

## What is binding vs advisory

- **Binding:** which screens exist, what data each shows, and what the user can do (the flows).
  Downstream design must serve every screen's data and route every action.
- **Advisory:** layout, styling, copy, exact pixels — deliberately rough grey boxes. There is no
  design system in the project yet (greenfield), so neutral defaults are used, not an invented
  visual language.

## Manifest

The binding manifest (SCR-XXX screens, FLOW-XXX flows, FEAT tags) lives in the spec's **Screens &
Flows** section: `../spec.md`. This app is its clickable rendering.
