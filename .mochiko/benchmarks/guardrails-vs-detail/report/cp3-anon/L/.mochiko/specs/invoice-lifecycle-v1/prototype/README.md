# Invoice lifecycle v1 — clickable low-fi prototype

Static HTML, no build step, no framework. This renders the spec's Screens & Flows manifest so
the contractor experience is walkable while the stories are still wet. **Flows and data shown
are binding; layout, styling, and copy are deliberately rough and advisory.**

## Serve it

```
bunx serve .mochiko/specs/invoice-lifecycle-v1/prototype/
```

Then open the printed URL (defaults to `index.html`, the dashboard = SCR-001).

## Degrade path (no server)

Open `index.html` directly in a browser (`file://…/prototype/index.html`). All navigation is
plain `<a href>` / `<form action>` relative links, so every flow walks with no server at all.

## Design system

The project has no design-system tokens or component library yet (React/TS/Vite is chosen but
no UI kit is committed). Screens therefore use neutral grey-box defaults — when a design system
lands, these low-fi screens are replaced, not migrated.

## Screen manifest

| SCR | Screen | Story |
|-----|--------|-------|
| SCR-001 | Dashboard / invoice list (`index.html`) | US-6 |
| SCR-002 | Client list | US-1 |
| SCR-003 | Client editor | US-1 |
| SCR-004 | Invoice editor (draft) | US-2 |
| SCR-005 | Invoice detail (sent + paid variants) | US-3, US-5, US-6 |
| SCR-006 | Sent confirmation | US-3 |
| SCR-007 | Reminder settings (P2, greyed) | US-7 |
| SCR-008 | Client payment page (Stripe-hosted, external stub) | US-4 |

Full manifest with FLOW rows, story-scenario keys, and FEAT tags: the **Screens & Flows**
section of `../spec.md`. This prototype is a specification artifact — throwaway, never migrated
into production UI.
