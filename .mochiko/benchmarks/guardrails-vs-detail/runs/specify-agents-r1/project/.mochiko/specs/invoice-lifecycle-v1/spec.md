# Invoice lifecycle v1

> Spec: invoice-lifecycle-v1
> Created: 2026-08-10
> Status: in-flight

---

## Intent

<!-- Confirmed one-screen synthesis from the intent stage, with the amendments folded in during
     lockstep prototyping (tax, saved clients, manual mark-as-paid and void into the slice,
     reminders back in scope) and re-confirmed by the principal at selection. Governs the run. -->

- **Scope boundary:** An invoice moves draft → sent → viewed → paid, with overdue shown as a computed indicator over a sent, past-due, unpaid invoice (due-date driven, never a stored status). A contractor authors an invoice against a saved client (line items, tax, due date), sends it, and the client pays via Stripe-hosted checkout — reconciling to `paid` against Stripe as the source of truth. Also in the whole feature: manual mark-as-paid (cash/check), void of a wrong sent invoice, and automated overdue reminders. Paid in full only.
- **Delivery:** Sliced. The v1 selection is the core get-paid loop — client records, authoring/sending, tracking, Stripe pay/reconcile, and manual mark-as-paid. Void and reminders are specified but deferred to a fast-follow.
- **Depth / rigor:** Maximum on money and payment-state correctness — Stripe reconciliation, webhook signature + idempotency (exactly-once), "paid means paid", and an append-only audit trail of every state change (payment and void alike). Lighter on cosmetic surfaces.
- **UX-bearing:** Yes — contractor-facing invoice list, create/edit invoice form, invoice detail, client management, plus the client payment landing. Low-fi clickable prototype under `prototype/`.
- **Constraints:** Stripe-hosted checkout only — Ledgerline never touches raw card data (GI-014). All money as `Decimal`, stored in cents, no floating point (GI-013). Strict tenant isolation — a contractor sees only their own data (GI-011). Append-only, immutable audit trail for state changes (GI-029). Single account per contractor in v1; no design that forecloses a later "bookkeeper seat."
- **Out of scope (v1):** recurring invoices, multi-currency, estimates/quotes, client portal logins, partial payments & deposits, multi-user roles/permissions, dunning/dispute handling.

---

## Overview

The invoice lifecycle is Ledgerline's spine: a contractor creates an invoice against a saved
client, sends it, the client pays via Stripe without an account, and the invoice reconciles to
`paid` — with the contractor trusting that state without re-checking Stripe. This spec covers the
whole lifecycle and delivers the core get-paid loop first.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Create and send an invoice (client, line items, tax, due date) | P1 | FEAT-001 | homed |
| [US-2](stories/US-2.md) | Client pays via Stripe-hosted checkout without an account | P1 | FEAT-003 | homed |
| [US-3](stories/US-3.md) | Monitor invoices and their status (list + detail + history) | P1 | FEAT-005 | homed |
| [US-4](stories/US-4.md) | Record an off-platform payment (manual mark-as-paid) | P2 | FEAT-004 | homed |
| [US-5](stories/US-5.md) | Void an incorrect sent invoice | P2 | FEAT-006 | homed (feature deferred) |
| [US-6](stories/US-6.md) | See overdue invoices at a glance (computed) | P2 | FEAT-005 | homed |
| [US-7](stories/US-7.md) | Automated overdue reminder emails (3/7/14 days) | P2 | FEAT-008 | homed (feature deferred) |
| [US-8](stories/US-8.md) | Manage clients (add and select) | P1 | FEAT-007 | homed |

No stories rejected this run. US-7 was drafted as the filter's rejection candidate (reminder
cadence undecided at intent) and homed by the principal's ruling on the prototype walk; the record
is in `stories/US-7.md`.

---

## Edge Cases

- **EC-1 (external failure):** A Stripe payment event is delayed, never arrives, or is delivered twice — the invoice stays in its pre-payment status until a signature-verified event arrives, and a replayed/duplicate event never changes state more than once (exactly-once).
- **EC-2 (invalid input):** An invoice with no line items or a non-positive total cannot be sent.
- **EC-3 (concurrent access):** A client completes Stripe payment at the same moment the contractor marks it manually paid — exactly one settlement takes effect; the second is refused and the audit trail records the settlement that won.
- **EC-4 (permission boundary):** A payment link opened for an already-paid invoice shows `paid` and does not allow a second payment.
- **EC-5 (system limit / boundary):** Overdue is computed at a day boundary in the account's timezone — an invoice due today is not overdue until the day has passed.

---

## Functional Requirements

- **FR-001**: System MUST let a contractor create and edit an invoice draft with a selected client, one or more line items, a tax rate, and a due date. `[FEAT-001]`
- **FR-002**: System MUST compute an invoice total as line items plus tax using exact decimal arithmetic, with no floating-point rounding drift. `[FEAT-001]`
- **FR-003**: System MUST let a contractor send an invoice, transitioning it from `draft` to `sent`, generating a client-payable hosted payment link, and delivering it to the client (FR-018). `[FEAT-001]`
- **FR-004**: System MUST let a contractor maintain saved clients — name and email required, mailing address optional — and select one when creating an invoice. `[FEAT-007]`
- **FR-005**: System MUST allow a client to pay a `sent` invoice through hosted checkout without creating an account or authenticating. `[FEAT-003]`
- **FR-006**: System MUST mark an invoice `viewed` and record a viewed event when its payment link is first opened. `[FEAT-003, FEAT-005]`
- **FR-007**: System MUST reconcile an invoice to `paid` only on a verified settlement — a signature-verified payment-provider event, or a contractor-recorded manual settlement. `[FEAT-003, FEAT-004]`
- **FR-008**: System MUST process each payment-provider event idempotently, so a replayed or duplicate event never changes payment state more than once. `[FEAT-003]`
- **FR-009**: System MUST let a contractor mark a `sent` or overdue invoice paid manually, recording the method and date, and MUST distinguish a manual settlement from a provider settlement. `[FEAT-004]`
- **FR-010**: System MUST refuse to settle an already-`paid` invoice again (no double settlement). `[FEAT-003, FEAT-004]`
- **FR-011**: System MUST present a contractor a list of their invoices with each invoice's current status, and a detail view showing client, line items, amounts, tax, total, due date, and status history. `[FEAT-005]`
- **FR-012**: System MUST record every invoice state change (created, sent, viewed, paid, void) in an append-only history — not mutated in place — capturing the actor and timestamp, and MUST surface it in the invoice detail. `[FEAT-005; cross-cutting audit, GI-029]`
- **FR-013**: System MUST show an overdue indication for any unpaid, past-due invoice in `sent` or `viewed` status, computed on read without changing stored status, and MUST NOT show it for paid, void, or not-yet-due invoices. `[FEAT-005]`
- **FR-014**: System MUST scope every invoice and client read and write to the authenticated contractor, so no contractor can access another's data. `[FEAT-005, FEAT-007; GI-011]`
- **FR-015**: System MUST let a contractor void a `sent` or overdue invoice, making it non-payable while retaining the record and recording who voided it and when. `[FEAT-006 — deferred fast-follow]`
- **FR-016**: System MUST automatically email a client overdue reminders at 3, 7, and 14 days past due, stopping on settlement, with a per-invoice off toggle. `[FEAT-008 — deferred fast-follow]`
- **FR-017**: System MUST assign each invoice a number that is sequential per contractor, unique within that contractor's account, and not editable after creation. `[FEAT-001]`
- **FR-018**: On send, System MUST deliver the payment link to the client's email as an authenticated transactional message, and MUST also expose a copyable link to the contractor as a fallback. `[FEAT-001; GI-030]`
- **FR-019**: System MUST let a contractor edit and resend a `sent` invoice while it is unpaid (correcting a mistake before money moves); once an invoice is `paid` it MUST NOT be edited (correction is then void territory, deferred to FEAT-006). `[FEAT-001]`

---

## Key Entities

### Invoice
An amount a contractor bills a client, moving through its lifecycle. **Attributes:** number ·
client reference · line items · tax rate · subtotal · tax · total (Decimal, cents) · currency (USD
v1) · status (draft/sent/viewed/paid/void) · due date · payment link · reminders-on flag.
**Relationships:** belongs to one Contractor (tenant); references one Client; has many LineItems;
has many StatusEvents; has zero-or-one PaymentSettlement.

### Client
A contractor's customer, invoiced and paid via Stripe; no login. **Attributes:** name · email ·
mailing address (optional). **Relationships:** belongs to one Contractor; referenced by many
Invoices.

### LineItem
One billable line. **Attributes:** description · quantity · unit price (Decimal) · amount.
**Relationships:** belongs to one Invoice.

### StatusEvent (audit trail)
An append-only record of one lifecycle change. **Attributes:** event type (created/sent/viewed/
paid/void) · actor (contractor / system / client-view) · timestamp · note. **Relationships:**
belongs to one Invoice; never mutated in place.

### PaymentSettlement
How an invoice became paid. **Attributes:** source (stripe/manual) · method (for manual) ·
provider event id (for stripe idempotency) · amount · settled-at. **Relationships:** belongs to
one Invoice.

> Raw card data is never modeled or stored — payment is Stripe-hosted (GI-014). Per-attribute
> data-sensitivity classification is `data-model.md`, downstream.

---

## Success Criteria

- **SC-001**: A contractor can take an invoice from creation through sending to Stripe-paid end-to-end without leaving Ledgerline. `[FEAT-007, FEAT-001, FEAT-003]`
- **SC-002**: An invoice shows `paid` only when a real settlement occurred (a verified Stripe event or a recorded manual settlement) — never otherwise. `[FEAT-003, FEAT-004]`
- **SC-003**: A replayed or duplicate Stripe event never changes an invoice's payment state a second time. `[FEAT-003]`
- **SC-004**: Every payment-state change (and, once built, every void) is recorded in the append-only audit trail and visible in the invoice's history. `[FEAT-003, FEAT-004, FEAT-005; void clause → FEAT-006, deferred]`
- **SC-005**: A contractor can see at a glance which invoices are overdue, with no paid, void, or not-yet-due invoice mis-flagged. `[FEAT-005]`
- **SC-006**: Overdue invoices with reminders on trigger client reminder emails at 3, 7, and 14 days overdue and stop on settlement. `[FEAT-008 — deferred]`
- **SC-007**: A contractor only ever sees and affects their own invoices and clients. `[FEAT-005, FEAT-007]`
- **SC-008**: A client can pay an invoice without creating an account or logging in. `[FEAT-003]`

> Deferred SCs: **SC-006** (waits for FEAT-008); the **void clause of SC-004** (waits for
> FEAT-006). All other SCs are this delivery's done-condition.

---

## Screens & Flows

Clickable low-fi prototype rendering this manifest. Flows and data shown are binding; layout and
styling advisory.

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Invoice list | See all invoices and their status at a glance | invoices (client, amount, due, status, overdue) | FEAT-005 |
| SCR-002 | Create / edit invoice form | Author a draft against a saved client | client picker, line items, tax rate, due date, tax-inclusive total | FEAT-001 |
| SCR-003 | Invoice detail | Inspect one invoice, its history, and act on it | client, line items, amounts, tax, total, status, status history, actions | FEAT-001 / FEAT-004 / FEAT-005 / FEAT-006 |
| SCR-004 | Client payment landing | Let a client pay without an account | amount due, due date, pay-with-card handoff | FEAT-003 |
| SCR-005 | Payment confirmation | Confirm payment received and reconciled | paid amount, confirmation | FEAT-003 |
| SCR-006 | Overdue reminders | Show reminder cadence, activity, per-invoice toggle | cadence, per-invoice toggle, reminder log | FEAT-008 (deferred, coming-soon) |
| SCR-007 | Clients | Add and select saved clients | client name, email, optional address; saved client list | FEAT-007 |
| SCR-008 | Invoice detail — paid state | Show the paid terminal a contractor trusts without re-checking Stripe | paid badge, settlement row (source), full status history | FEAT-005 / FEAT-003 / FEAT-004 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Create & send | SCR-007 (pick client) → SCR-002 → Save & send → SCR-003 (`sent`) | US-1 / scenarios 1,3 | FEAT-001 |
| FLOW-002 | Client pays via Stripe | SCR-004 (open → `viewed`) → Stripe checkout → SCR-005 (`paid`) → SCR-008 (`paid` detail) | US-2 / scenarios 1,2 | FEAT-003 |
| FLOW-003 | Monitor status | SCR-001 → SCR-003 (detail + history) | US-3 / scenarios 1,2,3 | FEAT-005 |
| FLOW-004 | Manual mark-as-paid | SCR-003 → Mark as paid → SCR-008 (`paid` detail) | US-4 / scenario 1 | FEAT-004 |
| FLOW-005 | Void invoice | SCR-003 → Void → SCR-003 (`void`) | US-5 / scenario 1 | FEAT-006 (deferred, coming-soon) |
| FLOW-006 | Overdue at a glance | SCR-001 (overdue badge on past-due sent/viewed) | US-6 / scenario 1 | FEAT-005 |
| FLOW-007 | Reminder cadence & toggle | SCR-006 (3/7/14-day activity; per-invoice off) | US-7 / scenarios 1,3 | FEAT-008 (deferred, coming-soon) |
| FLOW-008 | Add & select a client | SCR-007 (add) → SCR-002 (select) | US-8 / scenarios 1,2 | FEAT-007 |
| FLOW-009 | Edit & resend before payment | SCR-003 (`sent`, unpaid) → Edit → SCR-002 (recompute total) → resend → SCR-003 | US-1 / scenario 2 | FEAT-001 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with `bunx serve
prototype/` or open `prototype/index.html` directly (no server required). Flows and data are
binding; layout and styling advisory. Deferred features' screens (SCR-006 reminders, the void
action on SCR-003) are present but greyed **coming-soon**.

---

## Feature Selection

Derivation and the staged map delta (proposed entries, index-line drafts, specs-index row) live in
`feature-map-delta.md` in this workspace; the live map is written only at spec acceptance.

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-007 | Client records | new (`proposed`→`in-flight`) | US-8 | SC-001, SC-007 |
| FEAT-001 | Invoice authoring & sending | new (`proposed`→`in-flight`) | US-1 | SC-001 |
| FEAT-005 | Invoice tracking & lifecycle view | new (`proposed`→`in-flight`) | US-3, US-6 | SC-004, SC-005, SC-007 |
| FEAT-002 | Payment settlement & reconciliation (parent) | new (`proposed`→`in-flight`) | — (roll-up) | — |
| FEAT-003 | ↳ Stripe-hosted payment & reconciliation | new (`proposed`→`in-flight`) | US-2 | SC-002, SC-003, SC-008 |
| FEAT-004 | ↳ Manual payment recording | new (`proposed`→`in-flight`) | US-4 | SC-002, SC-004 |
| FEAT-006 | Invoice void | new (`proposed`, deferred) | US-5 | SC-004 (void clause) |
| FEAT-008 | Overdue reminder emails | new (`proposed`, deferred) | US-7 | SC-006 |

### Filter rejections

None this run.

### Selection

- **Selected (build now, dependency order):** FEAT-007 → FEAT-001 → FEAT-005 → FEAT-003 → FEAT-004 (FEAT-002 parent in-flight). Foundation: FEAT-007.
- **Deferred (`proposed` on the map, fast-follow):** FEAT-006 (void) — carries the void clause of SC-004; FEAT-008 (reminders) — carries SC-006. Neither blocks a contractor getting paid.
- **Deferred SCs:** SC-006 and the void clause of SC-004 — wait until FEAT-008 / FEAT-006 build.

---

## Assumptions

- **A1**: v1 is single-currency USD (multi-currency is explicitly out of scope).
- **A2**: A contractor authenticates before any invoice/client action; the auth mechanism itself is out of this feature's scope (assumed provided by the platform).
- **A3**: The overdue day boundary uses the contractor account's timezone, defaulted from the platform account profile at signup and contractor-settable later (the auth/profile mechanism itself is out of this feature's scope, per A2).

---

## Open Questions

- **Q1**: When void ships (FEAT-006), should voiding an already-`viewed` invoice notify the client, or silently disable the link? (Does not block the selected slice.)
- **Q2**: Reminder emails (FEAT-008) assume the client email on the invoice's client record is deliverable; bounce handling is governed by GI-030 but the per-reminder retry/bounce behavior is unspecified — settle before FEAT-008 builds.
