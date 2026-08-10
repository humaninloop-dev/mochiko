# Invoice Lifecycle v1

> Spec: invoice-lifecycle
> Created: 2026-08-10
> Status: accepted (2026-08-10)

---

## Intent

<!-- Confirmed one-screen synthesis from the intent stage; confirmed by the user before authoring. -->

- **Scope boundary:** From "create a client" through "invoice is paid and the contractor can see it's paid." Covers client records, invoice drafting (line items, tax rate, due date, invoice number, memo), preview + send by email with a Stripe-hosted payment link, status tracking (draft/sent/viewed/paid/overdue), automatic overdue reminders, and manual mark-as-paid for check/cash.
- **Delivery:** Subset now — the create → send → pay → mark-paid "paying spine" ships first (authentication folded in at the selection gate as its foundation); automatic reminders and the fuller status/overdue dashboard follow in a later build.
- **Depth / rigor:** Production-grade. Financial correctness (exact money handling, accurate payment state) is held to the build and is non-negotiable.
- **UX-bearing:** Yes — contractor-facing screens (sign-in, client list, invoice editor/list, invoice detail/status, email preview, payment confirmation). A clickable low-fi prototype accompanies this spec.
- **Constraints:** Stripe-hosted checkout only, never handle raw card data; money as exact decimals; every contractor sees only their own data (tenant isolation); authenticated transactional email carries the payment link; auth = email/password + Google; solo-operable.
- **Out of scope:** Recurring invoices, multi-currency, estimates/quotes, client portal accounts.

---

## Overview

Ledgerline's first feature: the end-to-end get-paid loop for a solo US contractor. A signed-in
contractor creates a client, drafts an invoice with exact totals, previews and sends it by email
with a Stripe payment link, and reaches "paid" either from the client's online Stripe payment or
by recording a check/cash payment manually. The whole lifecycle is specified and prototyped; the
first build delivers the paying spine (auth, clients, authoring, delivery, both payment channels),
deferring the status dashboard and automatic reminders.

---

## User Stories

<!-- Index only; story text + acceptance scenarios + FEAT mapping live in stories/US-*.md. -->

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-8](stories/US-8.md) | Sign in (email/password or Google), scoped to own data | P1 | FEAT-009 | homed |
| [US-1](stories/US-1.md) | Create and manage client records | P1 | FEAT-001 | homed |
| [US-2](stories/US-2.md) | Draft an invoice with exact totals | P1 | FEAT-002 | homed |
| [US-3](stories/US-3.md) | Preview + send an invoice with a payment link | P1 | FEAT-003 | homed |
| [US-4](stories/US-4.md) | Client pays online via Stripe | P1 | FEAT-005 | homed |
| [US-5](stories/US-5.md) | Manually mark an invoice paid (check/cash) | P1 | FEAT-006 | homed |
| [US-6](stories/US-6.md) | Track invoice status at a glance | P2 | FEAT-007 | homed (deferred) |
| [US-7](stories/US-7.md) | Automatic overdue reminders | P2 | FEAT-008 | homed (deferred) |

No stories were filter-rejected.

---

## Edge Cases

- Invalid client input — missing name or malformed email is rejected with a field-level message; nothing is saved.
- Invalid line input — negative/zero quantity or non-numeric unit price is rejected; totals are not recomputed from rejected input.
- Tax rounding across multiple line items — subtotal, tax, and total are computed with exact decimals and are correct to the cent.
- Email undeliverable — a bounce is reported to the contractor; the invoice is not shown as delivered.
- Duplicate/forged payment event — a replayed or unverified Stripe event never changes payment state or double-records a payment.
- Double payment recording — marking an already-paid invoice paid again is prevented with an explanation.

---

## Functional Requirements

<!-- FRs marked [Cn] were pinned by user rulings at the clarification gate (review round 1). -->

**Authentication (FEAT-009)**
- **FR-001**: System MUST allow a contractor to sign up and sign in with email and password.
- **FR-002**: System MUST allow a contractor to sign in with Google ("Sign in with Google").
- **FR-003**: System MUST scope every signed-in contractor's access to only their own account's data.
- **FR-004**: System MUST deny access on invalid credentials with a non-specific error and expose no account data.

**Client management (FEAT-001)**
- **FR-005**: Contractors MUST be able to create a client with a name and a valid email, and optionally a business name.
- **FR-006**: System MUST reject a client with a missing name or malformed email with a field-level message, saving nothing.
- **FR-007**: Contractors MUST be able to view and edit their clients, and MAY add a client inline while drafting an invoice.

**Invoice authoring (FEAT-002)**
- **FR-008**: Contractors MUST be able to draft an invoice for one of their clients with line items (description, quantity, unit price), a tax rate, a due date, an invoice number, and an optional memo.
- **FR-009**: An invoice number MUST be unique per contractor. [C5-adjacent]
- **FR-010**: System MUST compute subtotal, tax, and total with exact (non-floating) arithmetic, rounding half-up to the cent, with tax computed on the subtotal (not per line). [C2]
- **FR-011**: System MUST reject invalid line input (non-positive quantity, non-numeric price) with a field-level message and MUST NOT recompute totals from rejected input.
- **FR-012**: System MUST persist a draft invoice and reload every field and total unchanged.
- **FR-013**: The invoice list MUST flag an invoice as overdue when it is unpaid past its due date, computed on read (the dedicated dashboard is deferred to FEAT-007). [C4]

**Invoice delivery (FEAT-003)**
- **FR-014**: Contractors MUST be able to preview the client-facing email before sending.
- **FR-015**: On send, system MUST deliver an authenticated email to the client carrying the payment link and MUST transition the invoice from draft to sent.
- **FR-016**: The payment link MUST use an unguessable per-invoice token (not the invoice number), MUST open only that one invoice, and MUST NOT be enumerable; the token stays valid until the invoice is paid or voided (no expiry or PIN in v1). [C1]
- **FR-017**: System MUST surface a delivery failure (bounce/undeliverable) and MUST NOT mark the invoice delivered when delivery failed.
- **FR-018**: Re-sending an invoice MUST resend it to the client without creating a duplicate invoice record.
- **FR-019**: A sent invoice MUST be locked from edits; corrections are made by issuing a new invoice or a credit, never by rewriting the sent invoice. [C5]
- **FR-020**: A void action MUST cancel a sent invoice and invalidate its payment link. [C5]

**Online payment (FEAT-005)**
- **FR-021**: System MUST let a client pay a sent invoice through Stripe's hosted checkout reached from the emailed link, and MUST NOT handle raw card data.
- **FR-022**: After the client returns from Stripe checkout, the invoice MUST show a "payment processing" state until confirmation. [C3]
- **FR-023**: System MUST mark an invoice paid only from Stripe's signature-verified confirmation event, never from the browser redirect; if no confirmation arrives, the invoice MUST remain "payment processing" and be flagged for attention. [C3]
- **FR-024**: System MUST process payment confirmation events idempotently — a duplicated or replayed event MUST NOT record payment more than once.
- **FR-025**: An abandoned or failed checkout MUST leave the invoice unpaid with no partial payment recorded.

**Manual payment (FEAT-006)**
- **FR-026**: Contractors MUST be able to mark a sent, unpaid invoice paid, recording the method (check/cash) and date.
- **FR-027**: System MUST prevent recording a second payment on an already-paid invoice and explain that it is already paid.
- **FR-028**: System MUST record every payment-state change (online or manual) to an append-only history noting who and when.

**Status tracking (FEAT-007 — deferred)**
- **FR-029**: System SHOULD present invoices in a filterable dashboard by state (draft/sent/processing/paid/overdue). (The on-read overdue flag itself ships in the first build per FR-013.)

**Automated reminders (FEAT-008 — deferred)**
- **FR-030**: System SHOULD automatically email a payment-link reminder for overdue-unpaid invoices, and MUST NOT remind a paid invoice or remind twice within the same reminder window.

---

## Key Entities

- **Contractor** — an authenticated account; owns all clients and invoices; the tenant-isolation boundary.
- **Client** — a person/business the contractor bills (name, email, optional business); belongs to one contractor.
- **Invoice** — a bill to a client (invoice number unique per contractor, line items, tax rate, due date, memo, computed totals, an unguessable payment token, state ∈ draft/sent/payment-processing/paid/overdue/void); belongs to one contractor; locked from edits once sent.
- **Line item** — one billed row on an invoice (description, quantity, unit price, amount).
- **Payment** — a recording that an invoice was paid (channel: Stripe/manual; method; amount; date); one or more per invoice, guarded to a single paid recording.
- **Reminder** *(deferred, FEAT-008)* — a sent overdue notice tied to an invoice (sent date/window).

---

## Success Criteria

- **SC-001**: A signed-in contractor can go from no invoice to a sent invoice with a working payment link in under 5 minutes.
- **SC-002**: 100% of successful Stripe payments are reflected as paid in the app with no manual step.
- **SC-003**: Contractors can record check/cash payments so the paid/unpaid view matches reality for off-platform payments.
- **SC-004**: Invoice subtotal, tax, and total are correct to the cent for every invoice (half-up, tax on subtotal).
- **SC-005**: A contractor can see at a glance which invoices are unpaid or overdue (overdue flag on the invoice list; the filterable dashboard follows in FEAT-007).
- **SC-006** *(deferred → FEAT-008)*: Overdue-unpaid invoices trigger a client reminder with no contractor action.
- **SC-007**: A contractor can sign in with email/password or Google and reach only their own account's data.

---

## Screens & Flows

<!-- Manifest is binding (screens, data, actions); prototype pixels advisory. Rendered by
     prototype/ (serve with bun or open prototype/index.html directly). -->

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-010 | Login | Sign in to the account | Email/password fields, Google button | FEAT-009 |
| SCR-001 | Client list | See and open clients | Client name, email, business, invoice count | FEAT-001 |
| SCR-002 | Client form | Add/edit a client | Name, email, business; field errors | FEAT-001 |
| SCR-003 | Invoice list | Find and open invoices | Invoice #, client, amount, due, status badge | FEAT-002 |
| SCR-004 | Invoice editor | Draft/edit an invoice | Client, line items, tax rate, due date, number, memo, live totals | FEAT-002 |
| SCR-005 | Invoice detail | View one invoice + act on it | Line items, totals, state, actions, payment link, history | FEAT-002 |
| SCR-011 | Email preview | Preview the client email before send | To, subject, body, payment link | FEAT-003 |
| SCR-006 | Send confirmation | Confirm the invoice was sent | Recipient, sent state, bounce note | FEAT-003 |
| SCR-007 | Stripe checkout (external) | Client pays online | Amount, Stripe-hosted card entry | FEAT-005 |
| SCR-008 | Payment confirmation | Confirm payment received | Paid state, amount | FEAT-005 |
| SCR-009 | Status dashboard *(deferred, coming-soon)* | Track status at a glance | Filter, per-invoice state, overdue | FEAT-007 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Sign in | SCR-010 → sign in → SCR-001 | US-8 / scenario 1 | FEAT-009 |
| FLOW-002 | Create client | SCR-001 → Add → SCR-002 → Save → SCR-001 | US-1 / scenario 1 | FEAT-001 |
| FLOW-003 | Draft invoice | SCR-003 → New → SCR-004 → Save draft → SCR-005 | US-2 / scenario 1 | FEAT-002 |
| FLOW-004 | Preview + send | SCR-005 → Send → SCR-011 → send → SCR-006 → SCR-005 (sent) | US-3 / scenario 1 | FEAT-003 |
| FLOW-005 | Client pays online | email link → SCR-007 → Pay → SCR-008 → SCR-005 (paid) | US-4 / scenario 1 | FEAT-005 |
| FLOW-006 | Mark paid manually | SCR-005 (sent) → Mark paid → SCR-005 (paid) | US-5 / scenario 1 | FEAT-006 |
| FLOW-007 | Track status *(deferred)* | SCR-009 (filter → state view) | US-6 / scenario 1 | FEAT-007 |
| FLOW-008 | Overdue reminder *(deferred, system flow)* | reminder recorded on SCR-005 indicator | US-7 / scenario 1 | FEAT-008 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with `bunx serve .`
from the prototype directory or open `prototype/index.html` directly. Flows and data are binding;
layout and styling advisory. Deferred features' screens (SCR-009, the reminder indicator on
SCR-005) are greyed coming-soon but reachable.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-009 | Authentication | new (`proposed`) | US-8 | SC-007 |
| FEAT-001 | Client management | new (`proposed`) | US-1 | SC-001 (partial) |
| FEAT-002 | Invoice authoring | new (`proposed`) | US-2 | SC-001, SC-004, SC-005 |
| FEAT-003 | Invoice delivery | new (`proposed`) | US-3 | SC-001 |
| FEAT-004 | Payments (parent) | new (`proposed`) | — | (roll-up) |
| FEAT-005 | ↳ Online payment via Stripe | new (`proposed`) | US-4 | SC-002 |
| FEAT-006 | ↳ Manual payment recording | new (`proposed`) | US-5 | SC-003 |
| FEAT-007 | Invoice status tracking | new (`proposed`) | US-6 | — (filterable dashboard, FR-029; SC-005's glance ships via FEAT-002) |
| FEAT-008 | Automated overdue reminders | new (`proposed`) | US-7 | SC-006 |

### Filter rejections

- None — every drafted story homed to exactly one feature.

### Selection

- **Selected (build now, dependency order):** FEAT-009 → FEAT-001 → FEAT-002 → FEAT-003 → FEAT-005, FEAT-006 (parent FEAT-004 rolls up `in-flight`). Authentication was folded into this build by user ruling at the selection gate.
- **Deferred (`proposed` on the map):** FEAT-007 (filterable status dashboard + "viewed" question), FEAT-008 (carries SC-006; depends on FEAT-007).
- **Deferred SCs:** SC-006 — waits until FEAT-008 builds. (SC-005's at-a-glance overdue now ships via the FEAT-002 overdue flag, FR-013.)

---

## Assumptions

- Authentication is in scope for this build (folded in at the selection gate): email/password + Google sign-in only, no enterprise SSO.
- A transactional email provider capable of authenticated delivery (SPF/DKIM/DMARC) and bounce signals is available (governance GI-030).
- Stripe is the payment provider; card entry is always on Stripe-hosted checkout (governance GI-014).
- The client never logs in; clients interact only via the emailed link and Stripe's hosted page.
- Money is held and computed with exact decimals per governance (GI-013); the founder's "integer cents" preference is a compatible storage detail deferred to design.

---

## Open Questions

- **"Viewed" status** (FEAT-007, deferred): the founder listed "viewed" as a lifecycle state; tracking client opens (pixel/link) may be fussy and privacy-touching. In scope for FEAT-007 or dropped? Decide at FEAT-007.
- **Partial payments**: a client pays part of an invoice by check, or Stripe captures a partial amount. Not in v1 (ruled out at the clarification gate); flagged so downstream isn't surprised. Revisit after the spine ships.

_Resolved at the clarification gate (review round 1): overdue is computed on read and flagged in the invoice list now (FR-013), the dashboard deferred; payment link uses an unguessable token (FR-016); a "payment processing" state covers the redirect→webhook gap (FR-022/FR-023); sent invoices lock and are corrected by void-and-reissue (FR-019/FR-020); money rounds half-up with tax on the subtotal (FR-010)._
