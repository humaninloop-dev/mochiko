// The documents pipeline (architecture row AX-007): a `Document` row is created for a
// subject, a BullMQ job `documents.render` picks it up, renders the Handlebars template under
// `src/documents/templates/<kind>.hbs` with the subject loader for that kind, writes the PDF to Tigris under
// `storage_key`, and flips the row to `rendered`. Three failed renders mark it `failed` and
// alert. `GET /<subject>/{id}/document` is generic over kind (202 while queued).
//
// Adding a kind = a `DocumentKind` value + a template + a subject loader registered below.
// Invoices and monthly statements are the two kinds today.

import { DocumentKind } from '@prisma/client';

export interface SubjectLoader {
  load(subjectId: string): Promise<Record<string, unknown>>;
}

export const loaders: Record<DocumentKind, SubjectLoader> = {
  invoice: { load: (id) => loadInvoiceForRender(id) },
  statement: { load: (id) => loadStatementForRender(id) },
};

declare function loadInvoiceForRender(id: string): Promise<Record<string, unknown>>;
declare function loadStatementForRender(id: string): Promise<Record<string, unknown>>;
