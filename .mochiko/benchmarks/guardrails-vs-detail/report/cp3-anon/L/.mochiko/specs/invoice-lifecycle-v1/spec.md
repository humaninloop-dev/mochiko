# Invoice lifecycle v1

> Spec: invoice-lifecycle-v1
> Created: 2026-08-10
> Status: accepted

---

## Intent

- **Scope boundary:** End-to-end invoice lifecycle for solo US contractors — client management; invoice drafting (line items, tax rate, due date); send by email carrying a hosted Stripe payment link; online payment plus manual mark-paid for cash/check; status tracking (draft → sent → viewed → paid, with overdue); automatic reminder emails. The whole lifecycle is v1.
- **Delivery:** Whole lifecycle built as one v1 (no narrower slice). Build order: client + draft → send with Stripe link → payment + manual mark-paid → status tracking → reminders. Reminders are the one piece to cut last if runway runs short.
- **Depth / rigor:** Full production rigor under the governance floor — tests, append-only audit trail on payment state, tenant isolation, Stripe-hosted card entry only (no raw card data), money as `Decimal`.
- **UX-bearing:** Yes — contractor-facing React app (dashboard, client list, invoice editor, invoice detail with status). Clients get no screens and no accounts — email plus the Stripe payment page only.
- **Constraints:** Stack fixed (FastAPI, PostgreSQL, React/TypeScript, Stripe); must be runnable by the founder alone; part-time UI help; production-quality from day one.
- **Out of scope:** Recurring invoices, multi-currency, estimates/quotes, client portal accounts, partial payments (all firm for v1).

---

## Overview

Ledgerline's core product: a solo contractor signs up, adds a client, drafts an invoice, sends it
with a hosted payment link, and gets paid — online or by cash/check — while seeing every invoice's
payment state without opening Stripe. This spec covers the whole lifecycle as one v1, at full
production rigor because it handles contractors' real financial data.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Contractor adds and maintains the clients they bill | P1 | FEAT-002 | homed |
| [US-2](stories/US-2.md) | Contractor drafts an invoice with line items, tax, due date | P1 | FEAT-003 | homed |
| [US-3](stories/US-3.md) | Contractor sends an invoice by email with a payment link | P1 | FEAT-004 | homed |
| [US-4](stories/US-4.md) | Client pays online through Stripe's hosted page | P1 | FEAT-005 | homed |
| [US-5](stories/US-5.md) | Contractor records a manual (cash/check) payment | P1 | FEAT-005 | homed |
| [US-6](stories/US-6.md) | Contractor tracks invoice status at a glance | P1 | FEAT-006 | homed |
| [US-7](stories/US-7.md) | Unpaid invoices trigger automatic reminder emails | P2 | FEAT-007 | homed |
| [US-8](stories/US-8.md) | Contractor voids a mistaken or disputed invoice | P2 | FEAT-003 | homed |

---

## Edge Cases

- **Concurrent payment + manual mark:** an online payment confirmation arrives while the contractor is marking the same invoice paid manually — the invoice settles to a single `paid` state, recorded once, no double count.
- **Lost or delayed Stripe event:** if a payment webhook is delayed or never arrives, the invoice is not stranded — the contractor can still mark it paid manually, and a late event reconciles idempotently.
- **Email bounce on send:** the client's email hard-bounces or the provider rejects the send — the invoice does not show `sent`; the failure is surfaced for retry.
- **Invalid invoice input:** zero line items, negative quantity/price, or a due date in the past — the save is rejected with a message naming the problem.
- **Cross-tenant access attempt:** a contractor requests another contractor's invoice or client — the request is denied; no data leaks across tenants.

---

## Functional Requirements

### Client management (FEAT-002)
- **FR-001**: Contractors MUST be able to create, list, and edit clients, each with a name and a valid email address.
- **FR-002**: System MUST reject a client with a blank name or a malformed email, returning a field-level error.
- **FR-003**: System MUST scope every client to the owning contractor; a contractor MUST NOT view or address another contractor's clients.

### Invoice drafting (FEAT-003)
- **FR-004**: Contractors MUST be able to create and edit a draft invoice for one of their clients, with one or more line items (description, quantity, unit price), a tax rate, and a due date.
- **FR-005**: System MUST compute subtotal, tax, and total exactly, with no rounding error on monetary amounts.
- **FR-006**: System MUST reject saving an invoice that has no line items or a due date in the past.
- **FR-007**: A draft invoice MUST remain editable until it is sent.
- **FR-028**: Each invoice MUST carry a human-facing invoice number that is unique per contractor.
- **FR-030**: A sent invoice's amounts and line items MUST be immutable; a correction requires voiding and reissuing, not in-place editing.
- **FR-031**: Contractors MUST be able to void an unpaid invoice; a `voided` invoice MUST be excluded from owed and overdue views and MUST NOT be re-sent.
- **FR-032**: Voiding an invoice MUST stop its reminders and MUST NOT reverse or refund any payment Stripe has already collected (a `paid` invoice cannot be voided from the app).

### Invoice sending (FEAT-004)
- **FR-008**: Contractors MUST be able to send and resend an invoice to its client's email; the email MUST carry the invoice details and a hosted payment link.
- **FR-009**: Outbound invoice and reminder email MUST be authenticated (SPF/DKIM/DMARC) and the system MUST handle bounces.
- **FR-010**: On a successful send the system MUST record the invoice as `sent` with a timestamp; on a failed send it MUST NOT mark the invoice `sent` and MUST surface the failure for retry.
- **FR-011**: Resending MUST reuse the existing invoice and its payment link, never creating a duplicate invoice.

### Payment capture & recording (FEAT-005)
- **FR-012**: Clients MUST be able to pay an invoice online through a hosted card-entry page (Stripe hosted checkout); the system MUST NOT receive or store raw card data.
- **FR-013**: System MUST mark an invoice `paid` only from a confirmed, trusted payment event (webhook), never from the browser redirect alone.
- **FR-014**: Payment processing MUST be idempotent — a duplicate or replayed payment event MUST NOT double-record or double-count payment.
- **FR-015**: Contractors MUST be able to mark an invoice paid manually with a payment date, and MUST be able to reverse a manual mark within the same session.
- **FR-016**: A `paid` invoice MUST NOT offer the manual mark-paid action.
- **FR-017**: Every payment-state change (paid online, marked manually, reversed) MUST be written to an append-only audit trail.
- **FR-018**: Monetary values MUST be represented as a decimal type end-to-end; no binary floating point for money.

### Invoice status & dashboard (FEAT-006)
- **FR-019**: System MUST present a dashboard listing every invoice with its client, amount, due date, and current status (draft, sent, viewed, paid, voided).
- **FR-020**: An unpaid invoice past its due date MUST show an overdue indicator derived from the due date; the system MUST NOT require a separately stored overdue state or a background job to compute it.
- **FR-021**: System SHOULD mark an invoice `viewed` when the client opens the hosted payment page, where that signal is available; it MUST NOT use email-open pixel tracking.

### Payment reminders (FEAT-007)
- **FR-022**: System MUST support automatic reminder emails for unpaid invoices on a contractor-controlled global schedule; the default schedule is on the due date, then every 7 days until paid, capped at 3 reminders.
- **FR-023**: Reminders MUST stop for an invoice once it is paid, whether online or manually.
- **FR-024**: Contractors MUST be able to turn automatic reminders off.
- **FR-025**: Each reminder send MUST be logged.

### Cross-cutting
- **FR-026**: Every contractor-facing screen MUST meet the accessibility floor (WCAG 2.1 AA) per `.claude/rules/mochiko/accessibility.md`.
- **FR-027**: User-facing errors MUST NOT expose stack traces and MUST carry a correlation ID for support, per the error-handling governance principle.
- **FR-029**: All monetary amounts in v1 MUST be in USD; multi-currency is out of scope.

---

## Key Entities

- **Contractor** — the account/tenant that owns everything below; the isolation boundary. Attributes: identity, reminder settings.
- **Client** — a person or business a contractor bills. Attributes: name, email, optional contact. Belongs to one contractor.
- **Invoice** — the billed document. Attributes: status (draft/sent/viewed/paid), issue and due dates, tax rate, computed totals, sent/paid timestamps. Belongs to one client; has many line items.
- **Line item** — one billable row. Attributes: description, quantity, unit price. Belongs to one invoice.
- **Payment record** — how an invoice was paid. Attributes: method (online/manual), amount, paid date, source event reference. Belongs to one invoice.
- **Reminder log entry** — a record of a reminder sent for an invoice. Attributes: sent timestamp, schedule step.
- **Audit entry** — append-only record of a payment-state change. Attributes: what changed, when, actor, prior/new state.

---

## Success Criteria

- **SC-001**: A signed-in contractor can go from an empty account to sending their first invoice in under 10 minutes, unassisted. *(verifies FEAT-002, FEAT-003, FEAT-004; account creation/auth is a platform prerequisite — see Assumptions)*
- **SC-002**: A contractor can determine the payment state of every invoice from the dashboard with zero logins to Stripe. *(verifies FEAT-006)*
- **SC-003**: ≥95% of online payments completed on the hosted page are reflected as `paid` in Ledgerline within 1 minute of completion. *(verifies FEAT-005)*
- **SC-004**: 100% of invoice monetary totals match independent recomputation to the cent across a representative test set — no rounding errors. *(verifies FEAT-003, FEAT-005)*
- **SC-005**: With reminders on, ≥90% of overdue unpaid invoices receive at least one reminder within the configured schedule. *(verifies FEAT-007)*
- **SC-006**: Zero invoices are double-marked paid or double-counted under duplicate/replayed payment events. *(verifies FEAT-005)*

---

## Screens & Flows

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Dashboard | See every invoice and its state at a glance | Invoice list: client, amount, due date, status, overdue badge | FEAT-006 |
| SCR-002 | Client list | Browse and reach clients | Client name, email, edit link | FEAT-002 |
| SCR-003 | Client editor | Create / edit a client | Name, email, optional contact fields; validation | FEAT-002 |
| SCR-004 | Invoice editor | Draft / edit an invoice | Client, line items, tax rate, due date, live totals | FEAT-003 |
| SCR-005 | Invoice detail | View an invoice, act on it (send, mark paid) | Line items, totals, status, action buttons, status history | FEAT-005 (send action → FEAT-004) |
| SCR-006 | Sent confirmation | Confirm send / surface a delivery failure | Recipient, payment-link note, sent status or failure | FEAT-004 |
| SCR-007 | Reminder settings | Configure the global reminder schedule | On/off, schedule options, stop-on-paid note | FEAT-007 |
| SCR-008 | Client payment page | The client-facing pay surface (Stripe hosted, external) | Amount due, hosted card entry, pay/cancel | FEAT-005 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Create client | SCR-002 → SCR-003 → save → SCR-002 | US-1 / scenario 1 | FEAT-002 |
| FLOW-008 | Edit client | SCR-002 → SCR-003 (edit) → save → SCR-002 | US-1 / scenario 2 | FEAT-002 |
| FLOW-002 | Draft invoice | SCR-001 → SCR-004 → save → SCR-005 | US-2 / scenario 1 | FEAT-003 |
| FLOW-003 | Send invoice | SCR-005 → send → SCR-006 (status `sent`) | US-3 / scenario 1 | FEAT-004 |
| FLOW-012 | Resend invoice | SCR-005 → resend → SCR-006 (same link, no duplicate) | US-3 / scenario 2 | FEAT-004 |
| FLOW-004 | Client pays online | SCR-008 → pay → SCR-005 (`paid`) | US-4 / scenario 1 | FEAT-005 |
| FLOW-014 | Abandon payment → viewed | SCR-008 → cancel → SCR-001 (invoice `viewed`, unpaid) | US-4 / scenario 2 | FEAT-005 |
| FLOW-005 | Mark paid manually | SCR-005 → mark paid → SCR-005 (`paid`) | US-5 / scenario 1 | FEAT-005 |
| FLOW-016 | Undo manual mark | SCR-005 (paid) → undo → SCR-005 (prior status; audit records both) | US-5 / scenario 3 | FEAT-005 |
| FLOW-006 | Track status & overdue | SCR-001 renders mixed states + overdue badge | US-6 / scenarios 1–2 | FEAT-006 |
| FLOW-017 | Paid drops overdue | SCR-005 (paid) → SCR-001 (badge gone, `paid`) | US-6 / scenario 3 | FEAT-006 |
| FLOW-007 | Configure reminders | SCR-001 → SCR-007 → save | US-7 / scenario 1 | FEAT-007 |
| FLOW-018 | Void invoice | SCR-005 → void → SCR-001 (invoice `voided`, reminders stopped) | US-8 / scenario 1 | FEAT-003 |

**Scenario coverage (P1 scenarios without a distinct click path — covered on-screen or in backend behavior, recorded per authoring-prototype invariant 3):**

- US-1 / scenario 3 (client validation error) — same-screen: SCR-003 rejects a blank name / malformed email with a field error (no navigation).
- US-2 / scenario 2 (edit recomputes totals) — same-screen: SCR-004 recomputes subtotal/tax/total live on edit.
- US-2 / scenario 3 (invoice validation error) — same-screen: SCR-004 rejects an empty invoice / past due date with a message.
- US-3 / scenario 3 (send failure) — same-screen state: SCR-006 renders the delivery failure and keeps the invoice unsent (success/failure variants of one screen).
- US-4 / scenario 3 (duplicate payment event) — backend behavior, no UI path: idempotency per FR-014, verified by test, not a click path.
- US-5 / scenario 2 (mark-paid guard on a paid invoice) — same-screen: SCR-005 (paid variant) disables the mark-paid action.

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with `bunx serve prototype/` or open `prototype/index.html` directly. Flows and data are binding; layout and styling advisory. SCR-008 is an external-surface stub (Stripe's hosted page, not a Ledgerline screen), included so the pay flow walks.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-001 | Invoice lifecycle (parent) | new (`proposed`) | US-1…US-7 | — (roll-up) |
| FEAT-002 | ↳ Client management | new (`proposed`) | US-1 | SC-001 |
| FEAT-003 | ↳ Invoice drafting & void | new (`proposed`) | US-2, US-8 | SC-001, SC-004 |
| FEAT-004 | ↳ Invoice sending & delivery | new (`proposed`) | US-3 | SC-001 |
| FEAT-005 | ↳ Payment capture & recording | new (`proposed`) | US-4, US-5 | SC-003, SC-004, SC-006 |
| FEAT-006 | ↳ Invoice status & dashboard | new (`proposed`) | US-6 | SC-002 |
| FEAT-007 | ↳ Payment reminders | new (`proposed`) | US-7 | SC-005 |

### Filter rejections

- None — every drafted story earned a home on the map.

### Selection

- **Selected (build now, dependency order):** FEAT-002, FEAT-003, FEAT-004, FEAT-005, FEAT-006, FEAT-007 (reminders built last).
- **Deferred (`proposed` on the map):** none.
- **Deferred SCs:** none — every SC is covered by a selected feature.

---

## Assumptions

- "Viewed" is detected from the client opening the hosted Stripe payment page, not from email-open tracking; the founder noted this signal may be low-value (a client on the payment page is already near paying) so no effort beyond the cheap signal is warranted.
- Overdue is a computed badge from the due date, not a stored status, and requires no background job to flip statuses.
- Reminders use a single global schedule in v1 (default: on due date, then every 7 days until paid, max 3); per-invoice override is deferred post-v1.
- Manual mark-paid is all-or-nothing; partial payments are out of scope for v1 (re-confirmed by the founder at the prototype walk).
- One contractor = one tenant; there is no multi-user-per-account or team concept in v1.
- Contractor account creation and authentication are a platform prerequisite, not part of this feature's scope; every story assumes an authenticated, tenant-isolated contractor. Auth is homed as a separate platform feature (subject to the OQ-2/C2 ruling below). Every endpoint still enforces auth per the security governance floor.

---

## Open Questions

- **OQ-1 (RESOLVED at acceptance): Disputed invoices.** Ruled option (c) — an explicit invoice void state (US-8, FR-031/FR-032) that stops reminders and excludes the invoice from owed/overdue views, without reversing any Stripe-collected payment. Homed to FEAT-003.
- **OQ-2 (Minor): "Viewed" signal feasibility.** Whether Stripe's hosted checkout reliably signals a page open before payment. If the signal is not cheaply available, `viewed` degrades to being skipped (draft → sent → paid), which the founder has accepted as low-value.
- **OQ-3 (Minor): Lost-webhook reconciliation.** How long the system waits on a Stripe payment event before treating it as not-yet-paid, and whether any automated reconciliation runs. v1 leans on manual mark-paid as the fallback (Edge Cases); a reconciliation policy may be needed at scale.
