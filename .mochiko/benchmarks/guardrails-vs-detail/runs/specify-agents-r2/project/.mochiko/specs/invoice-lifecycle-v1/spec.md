# Invoice lifecycle v1

> Spec: invoice-lifecycle-v1
> Created: 2026-08-10
> Status: accepted

---

## Intent

- **Scope boundary:** A contractor takes an invoice from creation to paid unaided — create a client; draft an invoice (line items, single invoice-level tax rate, due date); send it by email carrying a Stripe hosted payment link; track status; and mark paid manually for check/cash. Invoice states: draft, sent, viewed, paid; "overdue" is computed from the due date, not a stored status.
- **Delivery:** Build a subset now. First shippable slice = the spine create → draft → send → get paid (contractor issues invoice, client pays via link, contractor sees sent→paid). Reminders and richer status handling follow. Reminder interval defaults/bounds are deferred.
- **Depth / rigor:** Day-one paying-contractor quality. HIGH rigor on the money/payment path — exact totals + tax, "paid" means paid, no double-charge, no lost payment, trustworthy payment state. Pragmatic (move-fast, iterate) on reminder timing and status polish.
- **UX-bearing:** Yes. The only on-screen user is the contractor; clients receive only email + the Stripe hosted page (no client login/portal). Clickable low-fi prototype delivered under `prototype/`.
- **Constraints:** Stripe hosted checkout only, no raw card data, with manual mark-paid alongside; email provider undecided (managed/cheap); strict per-contractor data isolation, single account per contractor with the future "bookkeeper seat" door left open but NOT designed now; auth = email+password plus Sign in with Google (no enterprise SSO); stack = FastAPI + Postgres + React (hosting leaning Render, not locked vs Railway). Legal/financial retention obligations unknown and deferred (Open Questions) — not a v1 blocker.
- **Out of scope (firm):** recurring invoices, multi-currency, estimates/quotes, client portal accounts. (Also excluded from v1 but revisitable, not hard lines: partial payments, dispute handling, team/multi-seat.)

---

## Overview

Ledgerline's first feature: the contractor-facing invoice lifecycle. A solo contractor creates a
client, drafts an invoice with exact money math, sends it with a hosted payment link, and watches
it move to paid — via Stripe or a manual check/cash mark — all without leaving the app or trusting
a second system. This spec covers the create-to-paid spine; payment reminders are derived but
deliberately deferred.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Create a client to address and bill | P1 | FEAT-001 | homed |
| [US-2](stories/US-2.md) | Draft an invoice with exact totals | P1 | FEAT-003 | homed |
| [US-3](stories/US-3.md) | Send an invoice with a payment link | P1 | FEAT-004 | homed |
| [US-4](stories/US-4.md) | Get paid through the hosted page | P1 | FEAT-004 | homed |
| [US-5](stories/US-5.md) | Mark an invoice paid manually | P1 | FEAT-004 | homed |
| [US-6](stories/US-6.md) | See invoice statuses at a glance | P1 | FEAT-005 | homed |
| [US-7](stories/US-7.md) | Send payment reminders | P2 | FEAT-006 | homed (deferred) |
| [US-8](stories/US-8.md) | See when a client viewed the invoice | P2 | FEAT-005 | homed |

No stories rejected: the filter was run and every drafted story earned a place on the map.

---

## Edge Cases

- **EC-1 — External failure (send):** The email provider rejects or times out on send → the invoice MUST NOT show `sent`; the failure is surfaced and the invoice stays `draft`/send-failed (FR-010). A later hard bounce after acceptance is surfaced too, not left looking delivered (FR-024).
- **EC-2 — Duplicate/replayed event:** The same payment event is delivered more than once → processed idempotently; no second payment recorded, paid invoice unchanged (FR-014).
- **EC-3 — Forged/unsigned event:** A payment event fails signature verification → rejected; payment state does not change (FR-013).
- **EC-4 — Manual + hosted double-pay:** An invoice is marked paid manually and a hosted payment also arrives (or the reverse order) → reconciled to a single paid state; the second event records no duplicate payment and the conflict is visible in the audit trail, never silently overwritten (FR-017).
- **EC-5 — Cross-tenant access:** Contractor A requests contractor B's invoice or client → denied; no cross-account read or write (FR-002).
- **EC-6 — Invalid draft:** A draft submitted with no line items or a past due date → rejected with a clear message, left unsent (FR-007).
- **EC-7 — Edit after send:** A sent but unpaid invoice is edited (amount/client/due date) → allowed; the payable amount and link update to match; once paid, edits are rejected (FR-026).

---

## Functional Requirements

**Client records (FEAT-001)**
- **FR-001**: Users MUST be able to create a client with a name and an email; the system MUST reject a client saved without a valid email.
- **FR-002**: System MUST scope every client record to the authenticated contractor's account; a contractor MUST NOT read or modify another account's clients.
- **FR-003**: Users MUST be able to select an existing client when starting an invoice.
- **FR-025**: Users MUST be able to remove a client by soft-delete; the client's invoices and their audit history MUST be retained and MUST NOT be cascade-deleted. *(Added round 1, C2 ruling.)*

**Invoice drafting (FEAT-003)**
- **FR-004**: Users MUST be able to draft an invoice for a client with one or more line items (description, quantity, unit price), a single invoice-level tax rate, and a due date.
- **FR-005**: System MUST compute subtotal, tax, and total using exact decimal arithmetic, accurate to the cent with no floating-point error.
- **FR-006**: System MUST recompute subtotal, tax, and total whenever a line item or the tax rate changes.
- **FR-007**: System MUST reject a draft that has no line items or a due date in the past, with a clear message, leaving the invoice unsent.
- **FR-008**: System MUST hold an invoice in `draft` status until it is sent.
- **FR-026**: Users MUST be able to edit an invoice while it is unpaid, including after it has been sent (correcting amount, client, or due date); once an invoice is paid, System MUST lock it against edits. Editing a sent, unpaid invoice MUST keep its payable amount and payment link consistent with the corrected total. *(Added round 1, C1 ruling; full void/reissue deferred with the retention question.)*

**Invoice delivery & payment (FEAT-004)**
- **FR-009**: Users MUST be able to send a draft invoice to the client's email carrying a hosted payment link for the invoice's exact amount; on successful send the status MUST move `draft` → `sent`.
- **FR-010**: System MUST NOT mark an invoice `sent` when the email provider fails to accept the message; the failure MUST be surfaced and the invoice left `draft`/send-failed.
- **FR-011**: Resending a sent invoice MUST re-notify the client without creating a second payable link or an additional charge.
- **FR-012**: System MUST collect card payment only through the hosted payment page; it MUST NOT accept, process, or store raw card data.
- **FR-013**: On a signature-verified payment event, System MUST mark the invoice `paid` exactly once, recording amount and timestamp; an event that fails signature verification MUST NOT change payment state.
- **FR-014**: System MUST process payment events idempotently; a replayed or duplicate event MUST NOT record a second payment or alter an already-paid invoice.
- **FR-015**: Users MUST be able to mark a sent invoice paid manually (check/cash/bank transfer) with method and date; manual mark-paid MUST be unavailable on an already-paid invoice.
- **FR-016**: System MUST record every invoice and payment state change in an append-only audit trail (actor, action, timestamp); historical entries MUST NOT be mutated in place.
- **FR-017**: When a hosted payment arrives for an invoice already paid manually (or a manual mark is attempted after a hosted payment), System MUST reconcile to a single paid state without recording a duplicate payment, and MUST make the conflict visible in the audit trail.
- **FR-024**: When an invoice or reminder email hard-bounces after being accepted for delivery, System MUST surface the bounce to the contractor and MUST NOT leave the invoice presented as successfully delivered. *(Added round 1 — governance GI-030: transactional email must handle bounces.)*

**Invoice status tracking (FEAT-005)**
- **FR-018**: Users MUST be able to view a list of their invoices showing each invoice's current status (draft / sent / viewed / paid).
- **FR-019**: System MUST indicate an invoice as overdue when it is sent, unpaid, and past its due date, computed from the due date rather than stored as a separate status.
- **FR-020**: Users MUST be able to open an invoice detail view showing line items, totals, current status, and its payment/audit history.
- **FR-021**: System SHOULD mark a sent invoice `viewed` when the client opens the hosted payment page; the viewed signal is best-effort and MUST NOT be inferred from email opens.

**Payment reminders (FEAT-006 — derived, deferred)**
- **FR-022**: System MUST send reminder emails for unpaid, sent invoices on contractor-configured intervals and MUST stop sending once the invoice is paid. *(Deferred with FEAT-006; not built this delivery.)*

**Cross-cutting**
- **FR-023**: System MUST require the contractor to be authenticated (email+password or Google sign-in) for every client and invoice operation.

---

## Key Entities

### Contractor (account)
The authenticated user and tenant boundary. **Attributes:** name · email · auth identity.
**Relationships:** owns many Clients and Invoices; every other entity is scoped to one Contractor.

### Client
Someone the contractor bills. **Attributes:** name · email (required) · notes. **Relationships:**
belongs to one Contractor; addressee of many Invoices.

### Invoice
The core billable artifact. **Attributes:** status (draft/sent/viewed/paid) · due date · tax rate
· subtotal · tax · total (exact decimal) · sent/paid timestamps. **Relationships:** belongs to one
Client; has many LineItems, Payments, and AuditEntries. Overdue is derived from due date + status,
not stored.

### LineItem
One billed line. **Attributes:** description · quantity · unit price · line total.
**Relationships:** belongs to one Invoice.

### Payment
A recorded payment against an invoice. **Attributes:** source (hosted / check / cash / bank
transfer) · amount · timestamp · external reference (for hosted). **Relationships:** belongs to one
Invoice.

### AuditEntry
An append-only record of a state change. **Attributes:** actor · action · timestamp · detail.
**Relationships:** belongs to one Invoice; never mutated after write.

---

## Success Criteria

- **SC-001**: A contractor can go from signup to a paid invoice — create client, draft, send, receive payment — without contacting support. *(FEAT-001/003/004/005)*
- **SC-002**: Every invoice's subtotal, tax, and total are exact to the cent, with zero rounding discrepancies across a representative set of tax rates and line-item counts. *(FEAT-003)*
- **SC-003**: A contractor can determine which invoices are paid, unpaid, and overdue from the app alone, without logging into the payment processor. *(FEAT-005)*
- **SC-004**: 100% of payments — hosted or manual — result in exactly one paid transition; no double-charges and no lost payments across duplicate, replayed, or cross-path (manual + hosted) events. *(FEAT-004)*
- **SC-005**: No invoice ever shows `sent` without a dispatched email; every send failure is visibly flagged to the contractor. *(FEAT-004)*
- **SC-006**: Reminders are sent only for unpaid, sent invoices and cease upon payment. *(FEAT-006 — **deferred**; waits until reminders build.)*

---

## Screens & Flows

Clickable low-fi prototype: `prototype/` — serve with `bunx serve prototype/` or open
`prototype/index.html` directly (no server, no build). Flows and data are binding; layout and
styling are advisory. FEAT tags applied in the post-derivation re-tag pass; FEAT-006 screens are
greyed coming-soon (not selected this delivery).

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Sign in | App entry (auth constraint; no story) | email/password + Google | — (app shell) |
| SCR-002 | Invoice list | Statuses at a glance | invoice, client, amount, due, status/overdue badge | FEAT-005 |
| SCR-003 | Clients | List + add a client | client name, email, notes; client table | FEAT-001 |
| SCR-004 | Invoice editor | Draft an invoice | line items, tax rate, due date, computed subtotal/tax/total | FEAT-003 |
| SCR-005 | Invoice detail | Detail + actions | line items, totals, status, payment/audit history; send/mark-paid actions | FEAT-005 (hosts FEAT-004 actions) |
| SCR-006 | Send confirmation | Confirm recipient + amount before send | recipient, hosted-link amount, message, send-failure note | FEAT-004 |
| SCR-007 | Mark paid manually | Record a check/cash payment | method, date, note | FEAT-004 |
| SCR-008 | Hosted checkout | External Stripe page (stub) | amount due; card entry handled by Stripe | FEAT-004 (external) |
| SCR-009 | Reminders | Configure reminder intervals | interval (defaults undecided) | FEAT-006 (coming-soon) |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Add a client | SCR-001 → SCR-002 → SCR-003 → save → SCR-003 | US-1 / save client with name+email | FEAT-001 |
| FLOW-002 | Draft an invoice | SCR-002 → SCR-004 → add items/tax/due → Save draft → SCR-005 | US-2 / draft saves with exact total | FEAT-003 |
| FLOW-003 | Send an invoice | SCR-005 (draft) → SCR-006 → Send now → SCR-005 (sent) | US-3 / send moves draft→sent with link | FEAT-004 |
| FLOW-004 | Client pays (external) | SCR-005 → SCR-008 → Pay → SCR-005 (paid) | US-4 / verified event marks paid | FEAT-004 |
| FLOW-005 | Mark paid manually | SCR-005 (sent) → SCR-007 → record → SCR-005 (paid) | US-5 / manual mark records method+audit | FEAT-004 |
| FLOW-006 | Statuses at a glance | SCR-002 (mixed statuses + overdue badge) | US-6 / list shows status; overdue flagged | FEAT-005 |
| FLOW-007 | View invoice detail | SCR-002 → SCR-005 | US-6 / detail shows items, total, history | FEAT-005 |
| FLOW-008 | Configure reminders | SCR-009 (coming-soon) | US-7 / reminder on configured interval | FEAT-006 |
| FLOW-009 | Viewed status | SCR-008 (client opens) → SCR-005 shows `viewed` | US-8 / hosted-page visit sets viewed | FEAT-005 |
| FLOW-010 | Send failure surfaced | SCR-006 → provider fails → SCR-005 stays draft/send-failed | US-3 / send-fail not silent | FEAT-004 |
| FLOW-011 | Double-pay reconciled | SCR-007 (marked paid) → SCR-008 (hosted pay) → SCR-005 audit shows conflict, single paid | US-5 / EC-4 reconcile | FEAT-004 |
| FLOW-012 | Invalid draft rejected | SCR-004 → submit with no line items / past due date → SCR-004 shows validation error, stays draft | US-2 / EC-6 | FEAT-003 |
| FLOW-013 | Already-paid guard | SCR-005 (paid) → Send-as-new and Mark-paid disabled | US-5 / already-paid scenario | FEAT-004 |

**Scenario coverage note:** every P1 story (US-1..US-6) has at least one happy-path flow, plus
flows for the UI-visible negative P1 scenarios: send failure (FLOW-010), double-pay reconcile
(FLOW-011), invalid-draft rejection (FLOW-012), already-paid guard (FLOW-013) — the two latter
added at C3 ruling. The remaining negative scenarios that have no honest screen path
(cross-tenant denial, forged/replayed payment event) are rendered as inline notes on
SCR-003/SCR-005 rather than standalone flows — a deliberate low-fi scoping, user-ruled (C3).

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-001 | Client records | new (`proposed` → selected) | US-1 | SC-001 |
| FEAT-002 | Invoice lifecycle (parent) | new (roll-up) | US-2..US-8 | — (roll-up) |
| FEAT-003 | ↳ Invoice drafting | new (`proposed` → selected) | US-2 | SC-001, SC-002 |
| FEAT-004 | ↳ Invoice delivery & payment | new (`proposed` → selected) | US-3, US-4, US-5 | SC-001, SC-004, SC-005 |
| FEAT-005 | ↳ Invoice status tracking | new (`proposed` → selected) | US-6, US-8 | SC-001, SC-003 |
| FEAT-006 | ↳ Payment reminders | new (`proposed`, deferred) | US-7 | SC-006 |

### Filter rejections

None — all eight stories homed.

### Selection

- **Selected (build now, dependency order):** FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005. Foundation: FEAT-001.
- **Deferred (`proposed` on the map):** FEAT-006 Payment reminders — a deliberate cut (user-ruled), carries SC-006 and its reminder obligations; first in line after the spine.
- **Deferred SCs:** SC-006 — waits until FEAT-006 builds.
- **Completeness ledger (parent FEAT-002):** delivered 0 · selected FEAT-003/004/005 · deferred FEAT-006 · parked stubs 0 · kills 0.

---

## Assumptions

- Tax is a single invoice-level rate for v1; no per-line-item or multi-jurisdiction tax (confirmed at prototype walk).
- The `viewed` signal derives from a hosted-page visit, not email-open tracking (unreliable, privacy-fraught); it is best-effort and non-blocking.
- Overdue is a computed indicator, not a stored state (user-confirmed as the simpler choice).
- Manual mark-paid supports check, cash, and bank transfer; partial payments are excluded from v1 but revisitable.
- Hosting leans Render but is not locked (Railway is a live alternative); this spec is hosting-agnostic.
- Reminder interval defaults, bounds, and per-invoice opt-out are undecided and deferred with FEAT-006.

---

## Open Questions

- **Legal/financial retention & deletion of invoice records** — the founder cannot yet name the retention obligation for financial records or CA deletion-rights exposure. Deferred by ruling; not a v1 blocker. If the eventual policy changes how invoice data is stored or deleted, revisit before it bites. (Tracks BACKLOG open question; research the actual obligation before authoring policy — do not guess.)
- **Email provider** — undecided (managed/cheap, e.g. SendGrid); a plan-time decision. Send behavior (FR-009/FR-010) is provider-agnostic.
