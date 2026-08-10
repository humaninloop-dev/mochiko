# US-1 — Create and send an invoice (Priority: P1)

A contractor drafts an invoice for a saved client — line items, a tax rate, and a due date —
reviews the tax-inclusive total, and sends it, which delivers the client a hosted payment link.

**Why this priority**: The spine begins here — nothing else in the lifecycle exists until a
contractor can author an invoice and get it in front of a client. Blocks every other story.

**Independent Test**: Seed one contractor account with one saved client. Create a draft selecting
that client, adding a line item, a tax rate, and a due date; verify the total = line items + tax
and that it persists as `draft` and is editable. Send it; verify status flips to `sent` and a
client-payable link is produced. Passing = a sent invoice with a correct tax-inclusive total and a
resolvable payment link; failing = the draft cannot be created/edited/sent, or the total ignores
tax.

**Acceptance Scenarios**:
1. **Given** a logged-in contractor with at least one saved client, **When** they create a draft selecting a client, adding a line item, a tax rate, and a due date, **Then** the invoice is saved as `draft`, its total equals line items plus tax, and it remains editable.
2. **Given** a `draft` invoice, **When** the contractor edits a line item, the tax rate, or the due date, **Then** the total recomputes, the changes persist, and the invoice stays `draft`.
3. **Given** a complete `draft`, **When** the contractor sends it, **Then** its status becomes `sent`, a hosted payment link is generated, and the link is emailed to the selected client (with a copyable fallback for the contractor).
4. **Given** a `sent` invoice that is still unpaid, **When** the contractor edits a mistake and resends, **Then** the corrected invoice is re-delivered; once an invoice is `paid` it can no longer be edited.

**Feature**: FEAT-001 (Invoice authoring & sending). Depends on FEAT-007 (Client records) for the
client selection. Sending delivers the link by authenticated email (FR-018, GI-030); editing stays
open until the invoice is paid (FR-019).
