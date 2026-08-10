# Invoice lifecycle v1 — low-fi prototype

Clickable low-fidelity rendering of the Screens & Flows manifest in
`../spec.md`. **Flows and data shape are binding; layout and styling are advisory.**

## How to view

- **Serve (convenience):** `bunx serve .` from this directory, then open the printed URL.
- **Degrade path (no server):** open `index.html` directly in a browser (`file://`). All
  navigation is plain `<a href>` — no build step, no framework, no dependency install required.

## Design system

No design system or component library exists in the project yet (greenfield — no `frontend/`
built, no tokens/theme in the repo). Screens use neutral grey-box defaults. When the React app's
design system lands, production UI adopts it; these throwaway screens are not migrated.

## Manifest pointer

Screens (SCR-XXX) and flows (FLOW-XXX) are enumerated in the **Screens & Flows** section of
`../spec.md`. Every SCR here is a reachable page; every FLOW is clickable end-to-end.

FEAT tags reflect the derivation re-tag pass. Screens outside the selected first slice
(mark-as-paid, void, overdue badge) are present but greyed **coming-soon**; the rejected story
(US-7) keeps a screen greyed and marked **rejected** with a pointer to `stories/US-7.md`.
