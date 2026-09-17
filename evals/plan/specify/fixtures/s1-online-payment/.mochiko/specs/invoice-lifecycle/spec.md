# Invoice lifecycle

> Spec: invoice-lifecycle
> Created: 2026-06-20
> Status: closed — written in the pre-map form (inline stories, a Delivery Slices table)
> before the feature map existed; kept as written since the 2026-07-08 migration. Both
> slices delivered 2026-07-30.

---

## Intent

- **Scope boundary:** a studio drafts an invoice and sends it; the client reads it on a link.
- **Delivery:** two slices — drafting first, sending second.
- **Depth / rigor:** production; money paths under the money-handling rule.
- **Constraints:** Postmark for email; Postgres; no client accounts.
- **Out of scope:** payments, reminders, recurring invoices.

---

## User Stories

### US-1 — Draft an invoice (P1)

As a studio owner, I draft an invoice with line items so that I can bill a client.

- **Given** a client on file, **When** I add two line items and a due date, **Then** the
  draft shows the correct total in the invoice currency.
- **Given** a draft, **When** I open the preview, **Then** the PDF matches the draft.

### US-2 — Send the invoice (P1)

As a studio owner, I send the invoice by email so that the client can read it.

- **Given** a complete draft, **When** I send it, **Then** the client receives an email with
  the PDF and a link that opens the invoice without a login.
- **Given** a sent invoice, **When** I resend it, **Then** the link is unchanged.

---

## Functional Requirements

- FR-001: The system MUST compute invoice totals in integer minor units.
- FR-002: The system MUST render a PDF preview identical to the sent PDF.
- FR-003: The system MUST send the invoice email through Postmark with the PDF attached.
- FR-004: The public link MUST carry an unguessable token and MUST NOT embed the invoice id.

## Success Criteria

- SC-001: A two-line draft with a 10% discount totals correctly in a unit test and in the PDF.
- SC-002: A sent invoice is readable on its public link without cookies.

## Screens & Flows

| ID | Screen | Purpose | Data shown |
|----|--------|---------|------------|
| SCR-001 | Invoice editor | Draft and edit | client, line items, totals, due date |
| SCR-002 | Public invoice | Client reads the invoice | invoice, status, bank details |

Prototype archived at migration; see git history before 2026-07-08.

## Delivery Slices

| Slice | Stories | Status |
|-------|---------|--------|
| S1 — drafting | US-1 | delivered 2026-07-14 |
| S2 — sending | US-2 | delivered 2026-07-30 |
