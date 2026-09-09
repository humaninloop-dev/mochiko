# Feature Specification — Credit notes (FEAT-015)

- **Status:** signed off 2026-09-04 · **Owner:** Priya (product) · **Finance:** Tom

## Overview

When a delivery is short or damaged after the invoice has gone out, the supplier and the
retailer agree a reduction by e-mail and Tom fixes the invoice by hand in Xero, which the
platform then no longer matches. A credit note lets the supplier record the agreed reduction
against the invoice inside Pallet; the retailer either has it taken off the next invoice or
refunded.

## User Stories

### US-001 — Issue a credit note (P1)

As a supplier, I want to issue a credit note against an issued invoice for specific lines
(short delivery, damaged goods, price correction) so that the retailer is charged the right
amount.

- **Given** an issued invoice, **when** the supplier selects lines, quantities, and a reason
  and issues the credit note, **then** the retailer sees it against the invoice and the
  invoice shows the credited amount.
- **Given** a draft credit note, **when** the supplier cancels it, **then** nothing is
  recorded against the invoice.

### US-002 — Credit applied or refunded (P1)

As a retailer, I want a credit note taken off my next invoice from that supplier, or refunded
if I paid by card, so that I get the money back.

### US-003 — Books stay right (P1)

As the finance lead, I want every credit note to appear in Xero against the right invoice so
that the books reconcile without manual fixes.

### US-004 — Credit note document (P3)

As a retailer, I want a PDF of the credit note, like the invoice PDF, for my records.

## Functional Requirements

- **FR-001** A supplier MUST be able to create a credit note against one of its issued
  invoices, choosing invoice lines, a quantity or an amount per line, and a reason from a
  fixed list plus free text. *Source: US-001*
- **FR-002** A credit note MUST be issued explicitly; before issue it is a draft the supplier
  can edit or cancel. *Source: US-001*
- **FR-003** An issued credit note MUST show against the invoice for both parties, and the
  invoice MUST show its outstanding amount net of credits. *Source: US-001*
- **FR-004** An issued credit note MUST be applied to the retailer's next invoice from that
  supplier, or refunded to the card the invoice was paid with. *Source: US-002*
- **FR-005** Every issued credit note MUST be exported to the accounting system against its
  invoice. *Source: US-003*
- **FR-006** A retailer MUST be able to download a PDF of an issued credit note.
  *Source: US-004*

## Success Criteria

- **SC-001** Manual invoice corrections in Xero fall to zero within two months of launch,
  measured from Tom's correction log.
- **SC-002** 90% of credit notes are issued within two working days of the delivery date,
  measured from the credit-notes table.

## Notes from the spec author

- The new record is called an **Adjustment** in the stories we ran with suppliers; keep that
  name in the API.
- Amounts should be in pounds with two decimal places (for example `12.50`) so the supplier
  app can show them without conversion.
- The supplier should be able to cancel a credit note after issuing it if it was a mistake;
  the retailer will see it disappear.

## Open questions

- Can a credit note exceed the invoice's outstanding amount (for example, credit for a line
  already paid in full plus a goodwill amount)? Left to design.
