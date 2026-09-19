// Hourly BullMQ job `accounting.export` — pushes issued invoices and settled payments to Xero.
// A failed push lands in `accounting_export_failures` and retries with backoff (five
// attempts, then an alert). Xero allows 60 calls/min and 5,000/day per tenant; the OAuth
// token is refreshed by `xero-auth.ts`, and a refresh failure pauses the queue and pages
// on-call. Xero's API has a first-class CreditNotes resource (`POST /CreditNotes`, then
// `PUT /CreditNotes/{id}/Allocations` to allocate it to an invoice); nothing uses it today.

import { InvoiceStatus } from '@prisma/client';

type XeroInvoiceStatus = 'DRAFT' | 'AUTHORISED' | 'PAID' | 'VOIDED';

export function xeroStatusFor(status: InvoiceStatus): XeroInvoiceStatus {
  switch (status) {
    case 'draft':
      return 'DRAFT';
    case 'issued':
      return 'AUTHORISED';
    case 'paid':
      return 'PAID';
    case 'void':
      return 'VOIDED';
    default: {
      // Exhaustiveness guard: a new InvoiceStatus value that is not handled here is a compile
      // error. The same guard exists in the retailer app's invoice badge component and in
      // `src/statements/statement.service.ts`, which sums outstanding invoices by status.
      const never: never = status;
      throw new Error(`unhandled invoice status ${never}`);
    }
  }
}
