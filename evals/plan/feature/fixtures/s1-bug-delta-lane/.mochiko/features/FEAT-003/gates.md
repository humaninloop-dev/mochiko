# FEAT-003 — durable gate set

Accumulated **TEST:** gates from the delivered mooring-invoicing run; re-run by any later work
in this territory.

- **TEST:** Setup: a stay 2026-05-01 to 2026-05-03 with readings 12 kWh, 9 kWh. Action: close the
  stay. Assert: one invoice, electricity line quantity 21, `mix billing.audit` prints nothing
  (SC-001).
- **TEST:** Setup: a season-pass boat, one night. Action: issue the invoice. Assert: berth-night
  unit price is 90 % of the visitor rate (SC-002).
- **TEST:** Setup: an issued invoice. Action: mark paid with a bank reference. Assert: status
  paid, audit row with user and time (SC-003).
