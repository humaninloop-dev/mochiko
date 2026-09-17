# Constraints and decisions — Tidewatch

## Hard constraints

- C-001 — Money is integer minor units with a currency everywhere (GI-004).
- C-002 — Pedestal readings are the PontoonPower controller's; the product never edits a reading,
  it re-imports (GI-003).
- C-003 — One harbour per deployment; no cross-harbour data (AX-001).

## Decisions

- D-001 — One Phoenix monolith with LiveView for the office kiosk; no separate services until a
  harbour needs a second office (ADR 2026-03-02).
- D-002 — Readings imported nightly at 02:00 harbour local time, replayable (ADR 2026-04-14).
- D-003 — Skipper messages go out by email or SMS per the skipper's preference; no push app
  (ADR 2026-05-28).

## Infrastructure provisioning

- IP-001 — PontoonPower controller API, one per harbour, polled by the nightly import.
- IP-002 — SMS via the harbour's own gateway account; email via the deployment's SMTP relay.

## Integrations

- INT-001 — Accountant export: a CSV of issued invoices, one row per invoice line, monthly.
