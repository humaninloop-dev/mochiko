# FEAT-003 — durable gates

Folded at the payment-recording acceptance landing (2026-08-18). Re-run on every later
final validation.

- **TEST:** Setup: an invoice of 1,250.00 EUR, sent · Action: record a payment of 1,250.00 EUR
  by bank transfer · Assert: invoice status reads `paid`, balance 0.00, and the public link
  shows "Paid" within one page load.
- **TEST:** Setup: an invoice of 1,250.00 EUR with a recorded payment of 1,000.00 EUR ·
  Action: record a second payment of 300.00 EUR · Assert: the request is rejected with the
  overpayment message and the balance still reads 250.00.
