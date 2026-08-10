# Invoice lifecycle v1

> Spec: invoice-lifecycle
> Created: 2026-08-10
> Status: draft

---

## Intent

- **Scope boundary:** Contractor creates clients and invoices, sends them with a hosted payment link, tracks payment (auto via Stripe + manual for check/cash), gets automatic reminders, and sees all invoice status in one place — signup to paid invoice without founder help.
- **Delivery:** One v1 feature, built in stages — core (create client → draft → send with link → manual mark-paid) first; Stripe auto-reconciliation + automatic reminders right after. All in v1.
- **Depth / rigor:** Hard production rigor on the money/email core (exact Decimal money, Stripe webhook signature-verify + idempotency, append-only audit trail). WCAG 2.1 AA floor binds but not gold-plated. Non-launch-critical edges documented, not built.
- **UX-bearing:** Yes — clickable low-fi prototype authored; priority screens are the invoice editor and the invoice detail / status view; signup and client creation kept plain.
- **Constraints:** Stripe hosted checkout for all card entry (no raw card data); manual mark-paid always available; marking paid deactivates the invoice's live payment link; auth = email+password + Sign in with Google (email verified on the password path; only verified emails merge); single-tenant-per-account; single tax rate per invoice; invoice number assigned at send/finalize, sequential gap-free per-account; canonical statuses draft · sent · viewed · paid · overdue · void, with `overdue` auto-computed and `void` a terminal contractor action; reminder cadence due/+3/+7, cap 3, per-invoice off-switch.
- **Out of scope:** Recurring/subscription invoices, multi-currency, estimates/quotes, client portal accounts, expense tracking, PDF export, multi-user teams, partial payments.

---

## Overview

Ledgerline's core: a small US contractor goes from signing up to a paid, tracked invoice without
help. It unifies the two chores contractors cobble together today — collecting payment and knowing
who owes what — into one place, with payment state trustworthy enough not to re-check Stripe.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Contractor signs up and signs in to an isolated account | P1 | FEAT-001 | homed |
| [US-2](stories/US-2.md) | Contractor manages the clients they bill | P1 | FEAT-003 | homed |
| [US-3](stories/US-3.md) | Contractor drafts an invoice with line items, tax, due date | P1 | FEAT-004 | homed |
| [US-4](stories/US-4.md) | Contractor sends an invoice with a hosted payment link | P1 | FEAT-005 | homed |
| [US-5](stories/US-5.md) | Client pays online; payment auto-reconciles to paid | P1 | FEAT-006 | homed |
| [US-6](stories/US-6.md) | Contractor marks an invoice paid manually (check/cash) | P1 | FEAT-006 | homed |
| [US-7](stories/US-7.md) | Overdue auto-flagged; reminders sent on a cadence | P2 | FEAT-007 | homed |
| [US-8](stories/US-8.md) | Contractor sees all invoices and status in one dashboard | P1 | FEAT-008 | homed |
| [US-9](stories/US-9.md) | Contractor voids an invoice sent in error (terminal) | P2 | FEAT-006 | homed |

---

## Edge Cases

- **Invalid input:** a malformed client email, or a line item with zero/negative quantity or price, MUST be rejected at entry with a field-level message; totals never compute from invalid lines.
- **Double-collection after paid:** marking an invoice paid (manual or online) MUST deactivate its payment link so a second real payment cannot be taken; a payment event arriving for an already-paid invoice MUST NOT create a second payment and MUST be logged for review.
- **Off-amount online payment:** a Stripe payment settling for less or more than the invoice grand total MUST NOT auto-mark the invoice `paid`; it MUST be held and flagged for the contractor to review (partial payments are out of scope).
- **External failure — payment:** a delayed, missing, unverifiable, or unmatched Stripe event MUST leave payment state unchanged and be logged for review; it never silently alters an invoice.
- **External failure — email:** a bounced invoice or reminder email MUST be surfaced to the contractor; a bounce means the invoice is not shown as successfully delivered, and further reminders to a bounced address MUST be suppressed.
- **Permission boundary:** any attempt to read or modify an invoice or client outside the authenticated account MUST be denied and reveal nothing about the other account's data.
- **Reminder bound:** no more than three automatic reminders are sent per invoice, and none after it becomes `paid`/`void` or when reminders are disabled for that invoice.

---

## Functional Requirements

- **FR-001**: The system MUST let a contractor register and sign in via email+password and via Sign in with Google; the password path MUST verify the email address, and only a verified email MUST resolve both methods to a single account.
- **FR-002**: The system MUST isolate every data read and write to the authenticated contractor's account; no account may access another account's data.
- **FR-003**: A contractor MUST be able to create, edit, and list clients, each with a name and a validated email address.
- **FR-004**: A contractor MUST be able to draft an invoice for a client with one or more line items (description, quantity, unit price), a single tax rate, and a due date; a draft MAY be deleted while unsent.
- **FR-005**: The system MUST compute line totals, tax, and grand total exactly to the cent using decimal arithmetic (never floating point), rounding tax half-up to the cent, and MUST NOT lose precision on monetary values.
- **FR-006**: The system MUST assign each invoice a per-account invoice number at send/finalize time — not at draft creation — and the assigned numbers MUST be sequential and gap-free per account (unsent drafts carry no number).
- **FR-007**: The invoice editor MUST let a contractor add a new client without losing the in-progress draft, and MUST make unsaved-draft state visible.
- **FR-008**: A contractor MUST be able to send an invoice to its client by email; the email MUST be authenticated (SPF/DKIM/DMARC), MUST include a summary of the invoice (client, line items, total, due date) and a hosted payment link, and MUST NOT include a PDF attachment (PDF export is out of scope).
- **FR-009**: The system MUST advance an invoice's status through the canonical set draft → sent → (viewed) → paid, MUST auto-compute `overdue` for any sent/viewed invoice that is unpaid past its due date, and MUST support the transitions sent/viewed → overdue and overdue → paid.
- **FR-010**: The system SHOULD record a `viewed` status when a client opens the hosted invoice link, provided a view can be reliably detected; if it cannot, `viewed` MAY be omitted rather than shown unreliably.
- **FR-011**: The system MUST surface a delivery bounce to the contractor and MUST NOT represent a bounced invoice as delivered.
- **FR-012**: The system MUST accept online payment for an invoice through Stripe's hosted checkout only; no raw card data may enter the system.
- **FR-013**: On a signature-verified Stripe payment event whose amount equals the invoice grand total, the system MUST mark the invoice `paid` and record the paid amount and date, processing each event exactly once even if replayed, and rejecting forged or unverifiable events; a verified payment whose amount does not equal the total MUST NOT auto-mark `paid` and MUST be held and flagged for the contractor.
- **FR-014**: A contractor MUST be able to mark any unpaid invoice paid manually (check/cash) with a payment date and method; the system MUST prevent a duplicate payment on an already-paid invoice.
- **FR-015**: Marking an invoice paid (online or manual) MUST deactivate its hosted payment link so no further real payment can be taken for it.
- **FR-016**: A contractor MUST be able to void a sent, viewed, or overdue invoice as a terminal action (no in-place editing after send); a voided invoice accepts no payment and is excluded from outstanding totals.
- **FR-017**: The system MUST record every invoice and payment-state change (including void) in an append-only, immutable audit log capturing who changed what and when.
- **FR-018**: The system MUST automatically send reminder emails for unpaid invoices on the cadence due date / +3 / +7 days, capped at three reminders, stopping when the invoice is paid, voided, reminders are disabled for it, or delivery to the client has bounced.
- **FR-019**: A contractor MUST be able to disable automatic reminders per invoice and MUST be able to see each invoice's reminder state (next reminder or off) on its detail view.
- **FR-020**: The system MUST present a dashboard listing all of the account's invoices with client, amount, due date, and current status, filterable by status; when an invoice is both viewed and past-due-unpaid, the dashboard MUST show `overdue` (overdue takes precedence), while the invoice detail timeline retains the `viewed` history.
- **FR-021**: The dashboard MUST show correct outstanding and paid totals (voided invoices excluded from outstanding), and MUST reflect payment state without the contractor logging into Stripe.
- **FR-022**: Reminder and invoice emails MUST handle bounces (per FR-011) and MUST be authenticated (per FR-008).

---

## Key Entities

### Contractor Account
The authenticated business owner and tenant boundary. **Attributes:** email · auth methods (password, Google) · account identity. **Relationships:** owns many Clients and Invoices; every other entity is scoped to it.

### Client
A payer a contractor bills. **Attributes:** name · email. **Relationships:** belongs to one Contractor Account; addressed by many Invoices.

### Invoice
The central object. **Attributes:** per-account sequential number (assigned at send) · status (draft/sent/viewed/paid/overdue/void) · due date · tax rate · monetary totals (Decimal) · reminder settings · payment-link active flag. **Relationships:** belongs to one Account and one Client; has many Line Items, many Payments, and many Audit Entries.

### Line Item
A billable line. **Attributes:** description · quantity · unit price · line total (Decimal). **Relationships:** belongs to one Invoice.

### Payment
A recorded settlement of an invoice. **Attributes:** amount (Decimal) · date · method (online/manual) · source reference. **Relationships:** belongs to one Invoice.

### Audit Entry
An append-only record of a state change. **Attributes:** actor · timestamp · from-state → to-state · action. **Relationships:** belongs to one Invoice; never mutated.

---

## Success Criteria

- **SC-001**: A contractor can go from sign-up to a sent invoice with a working payment link without any assistance.
- **SC-002**: Every invoice's line totals, tax (rounded half-up to the cent), and grand total are correct to the cent for any valid inputs.
- **SC-003**: Invoice numbers within an account are sequential with no gaps, even when drafts are abandoned or deleted.
- **SC-004**: A completed full-amount online payment is reflected as `paid` in the product without contractor action; a repeated payment notification never double-records, and an off-amount payment is never silently shown as `paid`.
- **SC-005**: A contractor can mark an invoice paid for a check or cash payment, and every such change is recoverable from an append-only history of who did what and when.
- **SC-006**: The dashboard shows each invoice's correct current status, and filtering by overdue returns exactly the invoices past due and unpaid.
- **SC-007**: Unpaid invoices past due are flagged overdue automatically, and clients receive at most three reminders on the due/+3/+7 cadence, none after payment or when disabled.
- **SC-008**: A contractor can determine the payment state of every invoice without logging into Stripe, and that state matches Stripe.
- **SC-009**: Invoice and reminder emails reliably reach clients, and any undelivered email is surfaced to the contractor.
- **SC-010**: No contractor can read or modify another contractor's invoices or clients.

---

## Screens & Flows

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with
`bunx serve prototype/` or open `prototype/index.html` directly (no server needed). Flows and the
data each screen shows are binding; layout and styling are advisory. No design system exists yet
(greenfield) — neutral grey-box defaults are used.

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Sign in / Sign up | Create account or sign in | email, password, Google button | FEAT-001 |
| SCR-002 | Invoice dashboard | See all invoices and status | invoice list (#, client, amount, due, status), outstanding/paid totals, status filter | FEAT-008 |
| SCR-003 | Clients list | See and reach clients | client rows (name, email), add/edit | FEAT-003 |
| SCR-004 | Client form | Add or edit a client | name, email, validation message | FEAT-003 |
| SCR-005 | Invoice editor | Draft an invoice | client select + new-client jump, line items, tax rate, due date, live subtotal/tax/total, gap-free number, unsaved-state | FEAT-004 |
| SCR-006 | Invoice detail & status timeline | Live where the contractor manages one invoice | status, amount, due, reminder state, status timeline, actions (send, pay-as-client, mark-paid) | FEAT-005/006/007 |
| SCR-007 | Send confirmation | Confirm sending with a payment link | recipient, payment-link + auth note, bounce note | FEAT-005 |
| SCR-008 | Hosted payment page (client-facing) | Client pays online | invoice amount, Stripe hosted pay (no card data in-app) | FEAT-006 |
| SCR-009 | Reminder settings | Control reminders for an invoice | on/off, fixed cadence, overdue note | FEAT-007 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Sign up → dashboard | SCR-001 → submit → SCR-002 | US-1 / sc1 | FEAT-001 |
| FLOW-002 | Google sign-in → dashboard | SCR-001 → Google → SCR-002 | US-1 / sc2 | FEAT-001 |
| FLOW-003 | Create a client | SCR-003 → SCR-004 → save → SCR-003 | US-2 / sc1 | FEAT-003 |
| FLOW-004 | Draft an invoice with a total | SCR-005 → add lines + tax + due → save → SCR-006 | US-3 / sc1 | FEAT-004 |
| FLOW-005 | Send invoice with payment link | SCR-006 → SCR-007 → confirm → SCR-006 (sent) | US-4 / sc1 | FEAT-005 |
| FLOW-006 | Client pays online → paid | SCR-008 → pay → SCR-006 (paid) | US-5 / sc1 | FEAT-006 |
| FLOW-007 | Mark paid manually | SCR-006 → mark-paid → SCR-006 (paid) | US-6 / sc1 | FEAT-006 |
| FLOW-008 | Dashboard filter overdue | SCR-002 → filter overdue | US-8 / sc2 | FEAT-008 |
| FLOW-009 | Overdue + reminder | SCR-006 → SCR-009 (cadence, off-switch) | US-7 / sc2 | FEAT-007 |

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-001 | Contractor accounts & authentication (foundation) | new (`proposed`) | US-1 | SC-001, SC-010 |
| FEAT-002 | Invoice lifecycle (parent) | new (`proposed`) | — (roll-up) | — |
| FEAT-003 | ↳ Client management | new (`proposed`) | US-2 | SC-001 |
| FEAT-004 | ↳ Invoice authoring | new (`proposed`) | US-3 | SC-001, SC-002, SC-003 |
| FEAT-005 | ↳ Invoice delivery | new (`proposed`) | US-4 | SC-001, SC-009 |
| FEAT-006 | ↳ Payment tracking & reconciliation | new (`proposed`) | US-5, US-6 | SC-004, SC-005, SC-008 |
| FEAT-007 | ↳ Payment reminders | new (`proposed`) | US-7 | SC-007, SC-009 |
| FEAT-008 | ↳ Invoice dashboard & status view | new (`proposed`) | US-8 | SC-006, SC-008 |

Full staged entries: [`derivation.md`](derivation.md). Map-delta baseline: empty map at run open.

### Filter rejections

None — all 8 stories home to exactly one feature (see `derivation.md`). The request is a coherent
in-scope v1; no story failed to earn a feature, so none was rejected.

### Selection

<!-- The user's ruling is recorded here at acceptance. -->

- **PM recommendation (build now, dependency order):** FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005 → FEAT-006 → FEAT-008, then FEAT-007.
- **Foundation (first by dependency):** FEAT-001.
- **Completeness ledger — parent FEAT-002:** 0 delivered / 6 undelivered leaves / 0 stubs / 0 kills.
- **User selection (ruled at acceptance):** all seven deliverable features selected for v1 — the foundation leaf FEAT-001 plus FEAT-002's six leaves (FEAT-003, FEAT-004, FEAT-005, FEAT-006, FEAT-008, FEAT-007) — in the recommended dependency order (reminders kept in v1 rather than deferred; staging is a build-order concern for `/mochiko:plan`).
- **Deferred SCs:** none — every SC is covered by a selected feature.

---

## Assumptions

- `overdue` is an auto-computed status applied when an unpaid invoice passes its due date, never set by hand (confirmed at intent).
- Invoice numbering is sequential, gap-free, per-account, with a configurable start; no US federal gap-free mandate, gap-free chosen for auditability (confirmed at intent).
- Single tax rate per invoice is sufficient for v1 (confirmed at intent).
- Overdue timing and reminder scheduling are evaluated in the contractor account's timezone (assumed; confirm at plan if multi-timezone accounts appear).
- Marking paid deactivates the payment link; a real second payment is prevented at source rather than reconciled after the fact (ruled at review, disposition #1).
- Data retention: working assumption of 7-year invoice retention (common US tax-record norm) with soft-delete on account deletion — parked, not solved (see Open Questions).

---

## Open Questions

- **Data retention & deletion (unresolved):** the real legal retention requirement and account-deletion behavior are undecided; the 7-year assumption is a placeholder to revisit before launch. Tracked on ROADMAP Later; must stay visible so it does not silently become permanent (principal's explicit request).
- **`viewed` detection:** whether a client's view of the hosted invoice link can be reliably detected. If not, drop `viewed` rather than show a status that may be wrong (FR-010).
- **Partial payments:** held out of v1; revisit post-launch (most competitor tools support it).
