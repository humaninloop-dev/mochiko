# Ledgerlite — Backlog

Open items only. Closed items live in `.mochiko/archive/backlog-trail.md`.

## Payments

- **Clients pay online** · 2026-08-30 · provenance: studio interviews, July batch
  (`docs/interviews-2026-07.md`) · Eleven of fourteen studios asked for a "pay now"
  button on the invoice email. Card and bank transfer both came up; nobody asked for
  wallets. Resume-cold: start from the interview notes; FEAT-005 was stubbed as a
  placeholder on the map and has not been refined.

## Invoices

- **PDF totals rendered in minor units** · 2026-08-22 · provenance: support ticket #412 ·
  Some invoice PDFs show "Total: 125000" instead of "1,250.00". Reproduces when a line item
  carries a discount. Resume-cold: `src/billing/pdf.js`, the discount branch; the
  money-handling rule says rounding happens once, at presentation.

## Reminders

- **Reminder cadence per client** · 2026-08-18 · provenance: the FEAT-004 pending row ·
  Studios want a gentler schedule for long-standing clients. Resume-cold: the pending row on
  `.mochiko/features/FEAT-004.md` carries the acceptance criteria.
