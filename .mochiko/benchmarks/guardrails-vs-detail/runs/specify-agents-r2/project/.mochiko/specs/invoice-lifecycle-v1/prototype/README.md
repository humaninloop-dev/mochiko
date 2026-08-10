# Prototype — Invoice lifecycle v1

Clickable low-fidelity rendering of the Screens & Flows manifest in `../spec.md`. Flows and data
are **binding**; layout, styling, and copy are **advisory** (deliberately rough).

## How to view

- **Serve (convenience):** `bunx serve .` from this directory, then open the printed URL.
- **Degrade path (no server):** open `index.html` directly in a browser (`file://`). All
  navigation is plain `<a href>` / `<form>` links, so every flow is walkable with no server, no
  build step, no framework, and no dependency install.

## Design system

Greenfield project — no design tokens or component library exist yet (`ARCHITECTURE.md`: no
components built). Neutral grey-box defaults used. Contrast and semantics kept honest because the
contractor-facing app carries a WCAG 2.1 AA legal mandate
(`.claude/rules/mochiko/accessibility.md`); this remains a throwaway spec artifact, never
production UI.

## Screens

| ID | File | Note |
|----|------|------|
| SCR-001 | `index.html` | Sign in (app entry; auth is a constraint, no story) |
| SCR-002 | `screens/scr-002-invoice-list.html` | Invoice list with statuses + overdue badge |
| SCR-003 | `screens/scr-003-clients.html` | Client list + add-client form |
| SCR-004 | `screens/scr-004-invoice-editor.html` | Draft invoice editor with computed totals |
| SCR-005 | `screens/scr-005-invoice-detail.html` | Invoice detail: actions + payment/audit history |
| SCR-006 | `screens/scr-006-send-confirm.html` | Send / resend confirmation |
| SCR-007 | `screens/scr-007-mark-paid.html` | Mark paid manually (check/cash) |
| SCR-008 | `screens/scr-008-hosted-pay.html` | Stripe hosted checkout — EXTERNAL stub |
| SCR-009 | `screens/scr-009-reminders.html` | Reminders — coming soon (FEAT-006 not selected) |

Full manifest with FEAT tags and flow keys: `../spec.md` § Screens & Flows.
