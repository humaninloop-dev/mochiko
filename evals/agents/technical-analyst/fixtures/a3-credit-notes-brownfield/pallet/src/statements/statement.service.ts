// Monthly retailer statement: sums each supplier's invoices for the retailer by status.
// Carries the same exhaustiveness guard as `src/accounting/xero-export.ts` — a new
// InvoiceStatus value that is not handled here is a compile error.

import { InvoiceStatus } from '@prisma/client';

export function countsTowardsOutstanding(status: InvoiceStatus): boolean {
  switch (status) {
    case 'issued':
      return true;
    case 'draft':
    case 'paid':
    case 'void':
      return false;
    default: {
      const never: never = status;
      throw new Error(`unhandled invoice status ${never}`);
    }
  }
}
