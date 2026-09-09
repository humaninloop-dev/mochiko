# Tallyhouse — cycle cards

Each card is a vertical increment; the block at its foot is the cycle's verification gate. The
commands CI runs for every cycle are in `.github/workflows/ci.yml`.

## C1 · create an invoice — verified 2026-09-02

- [x] A studio user creates an invoice for a client with line items; the invoice number is
  issued in sequence and the total is computed server-side. Story US-001. Case: Simple.
  Brownfield exposure: none.

**TEST:** C1 — an invoice is created and numbered through the API
- **Setup**: `docker compose up -d postgres` · `make migrate` · `make seed-demo`
- **Setup**: `uvicorn tally.app:app --port 8100` (background) (timeout 30s)
- **Action**: `curl -s -w "\n%{http_code}" -X POST localhost:8100/invoices -H 'content-type: application/json' -d @fixtures/invoice-two-lines.json`
- **Assert**: Response status: 201
- **Assert**: Console contains "\"number\": \"INV-"
- **Capture**: console

## C2 · dashboard — verified 2026-09-08 (see `verify/C2-gate.md`)

- [x] The dashboard lists open invoices with their balance due and totals outstanding per
  client. Story US-002. Case: Simple. Brownfield exposure: `[EXTEND]` the invoice list query.

**TEST:** C2 — the dashboard shows open invoices with balances
- **Setup**: `docker compose up -d postgres` · `make migrate` · `make seed-demo`
- **Setup**: `uvicorn tally.app:app --port 8100` (background) (timeout 30s)
- **Action**: open http://localhost:8100/dashboard in the browser
- **Assert**: Page contains "INV-1007"
- **Assert**: Page contains "240.00"
- **Capture**: screenshot

## C3 · record a payment

- [ ] A payment recorded against an invoice — by hand (bank transfer) or from a Paylane
  `payment.succeeded` webhook — reduces the balance due, and an invoice whose balance reaches
  zero becomes `paid`. Card payments carry the 2.5 % Paylane fee, recorded on the payment.
  Story US-003 (record a payment), US-004 (card payments settle automatically). Case: Merge —
  US-004 is too thin to demonstrate alone. Brownfield exposure: `[EXTEND]` `services/reconcile.py`
  (balance and fee), `[MODIFY]` none.

**TEST:** C3-a — a payment recorded through the API settles the invoice balance
- **Setup**: `docker compose up -d postgres` · `make migrate` · `make seed-demo` (INV-1007, total 240.00, unpaid)
- **Setup**: `uvicorn tally.app:app --port 8100` (background) (timeout 30s)
- **Action**: `curl -s -w "\n%{http_code}" -X POST localhost:8100/invoices/INV-1007/payments -H 'content-type: application/json' -d '{"amount":"240.00","method":"bank_transfer","reference":"BT-55120"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "\"balance_due\": \"0.00\""
- **Action**: `psql "$DATABASE_URL" -c "select status from invoices where number = 'INV-1007'"`
- **Assert**: Console contains "paid"
- **Capture**: console

**TEST:** C3-b — a Paylane sandbox payment reaches the webhook and settles the invoice
- **Setup**: `source .envrc.local` (PAYLANE_SANDBOX_KEY and PAYLANE_WEBHOOK_SECRET from the team vault)
- **Setup**: `make seed-demo` (INV-1008, total 89.50, unpaid)
- **Setup**: `paylane-cli listen --forward-to localhost:8100/webhooks/paylane` (background) (timeout 30s)
- **Action**: `python -m tally.cli paylane simulate-payment --invoice INV-1008 --amount 89.50`
- **Assert**: Console contains "webhook received: payment.succeeded" (within 20s)
- **Action**: `curl -s localhost:8100/invoices/INV-1008`
- **Assert**: Console contains "\"status\": \"paid\""
- **Assert**: Console contains "\"fee\": \"2.24\""
- **Capture**: console, logs

**TEST:** C3-c — the invoice page shows the payment
- **Action**: open http://localhost:8100/invoices/INV-1007/page in the browser
- **Assert**: Page contains "Paid"
- **Assert**: Page contains "BT-55120"
- **Assert**: The payment row sits under the totals block and the Paid badge reads clearly against the header
- **Capture**: screenshot
