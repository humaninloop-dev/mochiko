# Invoice Lifecycle — low-fi clickable prototype

Throwaway specification artifact for the `invoice-lifecycle` spec. It renders the spec's user
stories as clickable screens so the experience is visible before design. **Flows and data shape
are binding; layout, styling, and copy are advisory** (deliberately low-fi).

## How to view

- **Serve (convenience):** from this `prototype/` directory run `bunx serve .` and open the
  printed URL, or `bun x serve .`.
- **Degrade path (no server):** open `index.html` directly in a browser (`file://`). All
  navigation is plain `<a href>`; no build step, framework, or install is required to view.

## Manifest

The binding contract is the **Screens & Flows** section of `../spec.md` (SCR-XXX / FLOW-XXX).
This app is its clickable rendering — every SCR-XXX has a page under `screens/`, every FLOW-XXX
is walkable by clicking.

## Design system

None yet — the greenfield Ledgerline project has no design system or component library, so these
are neutral grey-box defaults (per authoring-prototype invariant 6). When one exists, later specs
adopt its tokens.

## Selection & greying

First-build selection: FEAT-001, FEAT-002, FEAT-003, FEAT-005, FEAT-006 (rendered normally).
Deferred (greyed coming-soon, still reachable): FEAT-007 (SCR-009 status dashboard) and FEAT-008
(reminder indicator on SCR-005). No stories were filter-rejected, so no rejected screens.
SCR-010 (login) is an auth prerequisite, not a derived feature of this spec.
