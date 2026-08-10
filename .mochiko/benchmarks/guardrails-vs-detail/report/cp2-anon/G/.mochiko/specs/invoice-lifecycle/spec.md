# Invoice lifecycle v1

> Spec: invoice-lifecycle
> Created: 2026-08-10
> Status: accepted

---

## Intent

- **Scope boundary:** The end-to-end invoice lifecycle a solo contractor drives to get paid — manage clients; draft an invoice (line items, per-invoice tax rate, due date); send by email with a hosted payment link; track status (draft → sent → viewed → paid → overdue); automatic payment reminders; manual mark-as-paid for check/cash. Contractor never handles a card number; contractor sees payment state without logging into the payment processor.
- **Delivery:** Ship the core spine first and solid, layer the rest after. Spine = create client → draft invoice → send with pay link → take an online payment → status flips to paid. Layer-after = viewed tracking, overdue, automatic reminders + cadence, manual mark-as-paid. The v1 data model must not foreclose the layered items or a later bookkeeper seat. Prefer the spine correct over all six items half-built. ~4 months to first paying customer.
- **Depth / rigor:** Production spine. Money math, payment-state correctness, and per-account data isolation get careful edge-case treatment; no gold-plating of low-traffic paths.
- **UX-bearing:** Yes. Contractor clicks through sign-in → invoice list → draft editor → send → status view. A clickable low-fi prototype was walked before locking — especially reminder cadence and how overdue surfaces.
- **Constraints:** Never touch card numbers (processor-hosted checkout only); money as exact decimal. Per-invoice tax rate the contractor sets — not a tax engine. Strict data isolation — each account sees only its own clients and invoices. Auth = email+password plus Sign in with Google; no enterprise SSO. Outbound email authenticated (SPF/DKIM/DMARC) with bounce handling. WCAG 2.1 AA as a folded-in default. Stack: Python/FastAPI, PostgreSQL, React, Stripe, Render. Runnable by one person, no ops team.
- **Out of scope:** Recurring invoices; multi-currency; estimates/quotes; client portal accounts; partial payments; a teams/multi-user feature (but the data model must leave the door open for a later bookkeeper seat).

---

## Overview

Ledgerline's invoice lifecycle lets a solo contractor go from signing up to a paid invoice without human help: manage clients, draft invoices with correct money math, send them with a hosted payment link, collect payment online, and see payment state without ever opening the payment processor's dashboard. This spec covers the whole lifecycle; the selected delivery round builds the spine (accounts, clients, invoicing, online payment) and defers manual payment recording, status lifecycle, and automated reminders to a later round.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Contractor signs up / signs in to an isolated workspace | P1 | FEAT-001 | homed |
| [US-2](stories/US-2.md) | Contractor adds and edits the clients they bill | P1 | FEAT-002 | homed |
| [US-3](stories/US-3.md) | Contractor drafts an invoice with line items, tax, due date, computed totals | P1 | FEAT-003 | homed |
| [US-4](stories/US-4.md) | Contractor sends an invoice by email with a payment link | P1 | FEAT-003 | homed |
| [US-5](stories/US-5.md) | Client pays online; invoice flips to paid; contractor sees it | P1 | FEAT-005 | homed |
| [US-6](stories/US-6.md) | Contractor manually marks an invoice paid (check/cash) | P2 | FEAT-006 | homed (deferred) |
| [US-7](stories/US-7.md) | Contractor tracks viewed and overdue status | P2 | FEAT-008 | homed (deferred) |
| [US-8](stories/US-8.md) | Ledgerline sends automatic reminders on a cadence | P2 | FEAT-009 | homed (deferred) |

No stories were rejected by the derivation filter — every drafted story earned a place on the map.

---

## Edge Cases

- **EC-1 (external failure):** A payment succeeds on the hosted page but the processor's confirmation event is delayed or never arrives — the invoice must still reflect paid by reconciling against the processor as the source of truth, not solely by waiting for the event.
- **EC-2 (concurrent / replay):** The processor delivers the same payment event more than once (normal retry behavior) — payment state changes exactly once; no double-recording.
- **EC-3 (invalid input):** A line item with zero/negative quantity or price, or a tax rate outside `0 ≤ rate < 100` (0% tax-exempt is allowed; 100% is rejected as a fat-finger), is rejected with a clear message; totals never compute from invalid input.
- **EC-4 (external failure):** An invoice email hard-bounces — the contractor is notified it was not delivered, and the invoice is never shown as successfully sent-and-awaiting-payment in a way that hides the failure.
- **EC-5 (permission boundary):** A request references an invoice or client belonging to another account — it is refused and nothing about the other account is leaked.
- **EC-6 (concurrent):** A client pays online at nearly the same moment the contractor marks the invoice paid manually — the invoice is recorded paid once, with no double payment and a coherent history. *(manual side deferred with FEAT-006; the constraint is recorded now so the data model honors it.)*
- **EC-7 (external / double-spend):** A client completes two distinct successful payments on a still-live link (second tab, pay-again) — prevented because the payment link deactivates on first successful payment (FR-019); a paid invoice can never be paid online again.

---

## Functional Requirements

- **FR-001**: System MUST let a contractor register and sign in with email+password or Sign in with Google.
- **FR-002**: System MUST scope every data read and write to the authenticated account; no account MUST ever read or modify another account's clients or invoices.
- **FR-003**: Contractors MUST be able to create, edit, and list their clients (name, email, billing address), and the system MUST validate required fields before saving.
- **FR-004**: Contractors MUST be able to draft an invoice for a client with one or more line items (description, quantity, unit price), a per-invoice tax rate, and a due date.
- **FR-005**: System MUST compute invoice subtotal, tax, and total using exact decimal arithmetic, recomputing whenever a line item or the tax rate changes — but only while the invoice is a draft (a sent invoice is immutable per FR-018).
- **FR-006**: System MUST refuse to send an invoice that has no line items.
- **FR-007**: Contractors MUST be able to send an invoice to the client by email, and the email MUST carry the invoice and a hosted online-payment link; on send the invoice status MUST become sent.
- **FR-008**: System MUST authenticate outbound invoice and reminder email (sender-domain authentication) and MUST handle delivery bounces, notifying the contractor when an invoice email is not delivered.
- **FR-009**: System MUST track invoice status through draft → sent → viewed → paid, and MUST automatically mark an unpaid invoice overdue once its due date passes. *(The viewed and overdue transitions are deferred with FEAT-008; the selected round tracks draft → sent → paid.)*
- **FR-010**: System MUST accept online payment through a processor-hosted checkout, and MUST NOT receive, store, or transmit raw card data.
- **FR-011**: On confirmed payment the system MUST mark the invoice paid, record the paid amount, and reconcile payment state against the payment processor as the source of truth.
- **FR-012**: System MUST process inbound payment notifications idempotently and MUST reject unverified or forged notifications, so a replayed or forged event never changes payment state.
- **FR-013**: Contractors MUST be able to manually mark an invoice paid (recording method and date) for offline payment, and the system MUST prevent double-recording an already-paid invoice.
- **FR-014**: System MUST record every invoice and payment state change in an append-only, attributable history (who, what, when); historical entries MUST NOT be altered in place.
- **FR-015**: System MUST send automatic payment reminders for unpaid invoices on a contractor-configurable cadence, and MUST stop reminders immediately once an invoice is paid.
- **FR-016**: The contractor-facing application MUST meet WCAG 2.1 AA — semantic markup, a label for every control, sufficient contrast, and full keyboard navigation.
- **FR-017**: System MUST let a contractor see each invoice's payment state (paid / unpaid / overdue and amount) without logging into the payment processor.
- **FR-018**: Once sent, an invoice MUST be immutable; to change a sent invoice the contractor MUST void it (which deactivates its payment link) and issue a new invoice — the original invoice and its history are retained.
- **FR-019**: On successful payment the system MUST deactivate the invoice's hosted payment link so the invoice cannot be paid online again; refund and over-collection handling is out of scope for v1.
- **FR-020**: System MUST provide email-based password reset for email+password accounts, and SHOULD verify a registration email where verification reuses the same email path.
- **FR-021**: When the same email address is presented via both email+password and Sign in with Google, the system MUST resolve to a single linked account — never two accounts for one email.
- **FR-022**: A client that has invoices MUST NOT be hard-deleted; the system MUST offer archive/hide while retaining the underlying client and invoice records.

---

## Key Entities

### Account
The contractor's isolated workspace and identity. **Attributes:** email · auth method (password / Google) · display name. **Relationships:** owns many Clients and Invoices; every other entity is scoped to exactly one Account.

### Client
A party the contractor bills. **Attributes:** name · email · billing address. **Relationships:** belongs to one Account; addressed by many Invoices.

### Invoice
The central billable document and its lifecycle state. **Attributes:** number · status (draft/sent/viewed/paid/overdue) · issue date · due date · tax rate · subtotal · tax · total (all money as exact decimal). **Relationships:** belongs to one Account and one Client; has many LineItems; has many Payments; has one append-only history.

### LineItem
One billed line. **Attributes:** description · quantity · unit price · line total. **Relationships:** belongs to one Invoice.

### Payment
A recorded payment against an invoice. **Attributes:** amount · method (online / manual: check, cash) · timestamp · processor reference (online only). **Relationships:** belongs to one Invoice.

### InvoiceEvent (audit history)
An immutable record of a state change. **Attributes:** timestamp · actor (contractor / system) · event · before/after summary. **Relationships:** belongs to one Invoice; append-only.

---

## Success Criteria

- **SC-001**: A new contractor goes from sign-up to a sent invoice with a working payment link in under 10 minutes with no external help.
- **SC-002**: Online payments appear as paid to the contractor within one minute of the client completing payment, without the contractor opening the processor dashboard.
- **SC-003**: Displayed invoice totals match a hand calculation to the cent for every invoice (no rounding errors).
- **SC-004**: No contractor can ever see or change another contractor's clients or invoices — zero cross-account incidents.
- **SC-005**: A duplicated or forged payment notification never changes an invoice's paid state — zero double-payments or spoofed paid states.
- **SC-006**: Contractors can record an offline payment and see the invoice as paid with the method captured. *(deferred — FEAT-006)*
- **SC-007**: Unpaid invoices past due show as overdue automatically, and configured reminders send until the invoice is paid. *(deferred — FEAT-008, FEAT-009)*
- **SC-008**: The contractor-facing app passes automated accessibility checks and a keyboard-only walkthrough of the core flow. *(Cross-cutting: FR-016 binds every contractor-facing feature — FEAT-001, FEAT-002, FEAT-003 — not one screen.)*

---

## Screens & Flows

Authored per the intent's UX-bearing ruling. Manifest binding (screens, data, actions); prototype pixels advisory. FEAT tags applied as a re-tag pass after derivation; screens of deferred features are kept, reachable, and greyed **coming-soon**.

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Sign in / Register | Authenticate into an isolated workspace | email, password, Google sign-in | FEAT-001 |
| SCR-002 | Invoice list (dashboard) | See all invoices and their state | number, client, due date, total, status | FEAT-003 (status column FEAT-008, deferred) |
| SCR-003 | Clients list | Browse the clients billed | name, email, city | FEAT-002 |
| SCR-004 | Client editor | Add / edit a client | name, email, billing address | FEAT-002 |
| SCR-005 | Invoice draft editor | Draft an invoice and see totals | client, line items, tax rate, due date, subtotal/tax/total | FEAT-003 |
| SCR-006 | Invoice detail & status | Act on and track one invoice | totals, status, history, send / pay / mark-paid actions | FEAT-003 send · FEAT-005 online pay · FEAT-006 manual (deferred) · FEAT-008 status (deferred) |
| SCR-007 | Send confirmation | Confirm the send and recipient | recipient, subject, resulting status | FEAT-003 |
| SCR-008 | Reminder cadence settings | Configure automatic reminders | first reminder, repeat interval, stop condition | FEAT-009 (deferred) |
| SCR-009 | Hosted payment page (client-facing) | Client pays online off Ledgerline | amount owed, processor-hosted card entry | FEAT-005 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Sign in to workspace | SCR-001 → sign in → SCR-002 | US-1 / returning contractor reaches own workspace | FEAT-001 |
| FLOW-002 | Add a client | SCR-002 → Clients → SCR-003 → New client → SCR-004 → save → SCR-003 | US-2 / add a client with details | FEAT-002 |
| FLOW-003 | Draft an invoice | SCR-002 → New invoice → SCR-005 (add line items + tax) → save | US-3 / two line items + tax persisted with exact totals | FEAT-003 |
| FLOW-004 | Send an invoice | SCR-005 → Save & send → SCR-007 → Send now → SCR-006 (status sent) | US-4 / client receives email with pay link, status sent | FEAT-003 |
| FLOW-005 | Client pays online | SCR-006 → open payment page → SCR-009 → Pay → SCR-006 (status paid) | US-5 / payment completes, invoice becomes paid | FEAT-005 |
| FLOW-006 | Manual mark-paid | SCR-006 → Mark paid manually *(coming-soon)* | US-6 / offline payment recorded | FEAT-006 (deferred) |
| FLOW-007 | Configure reminder cadence | SCR-008 *(coming-soon)* → set cadence → save | US-8 / new cadence governs reminders | FEAT-009 (deferred) |

Negative and error states — email hard-bounce (US-4 sc.2), payment replay/forgery rejected (US-5 sc.2–3), cross-account refusal (US-1 sc.3), blank-required-field refusal (US-2 sc.3), send-with-no-line-items refusal (US-3 sc.3) — are specified as requirements and acceptance scenarios but are **not** rendered as clickable prototype branches: a static low-fi mock cannot exercise server-side branches. They are verified at build (ruled at disposition). Password reset (FR-020) is surfaced as a "Forgot password?" link on SCR-001; the reset itself is an email round-trip verified at build, not prototyped.

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with `bunx serve .` or open `prototype/index.html` directly (no server required). Flows and data are binding; layout and styling advisory.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-001 | Contractor accounts & access | new (`proposed`) | US-1 | SC-001, SC-004, SC-008 (cross-cutting) |
| FEAT-002 | Client management | new (`proposed`) | US-2 | SC-001, SC-008 (cross-cutting) |
| FEAT-003 | Invoicing (draft & send) | new (`proposed`) | US-3, US-4 | SC-001, SC-003, SC-008 |
| FEAT-004 | Payments (parent) | new (`proposed`) | US-5, US-6 | — (roll-up) |
| FEAT-005 | ↳ Online payment collection | new (`proposed`) | US-5 | SC-002, SC-005 |
| FEAT-006 | ↳ Manual payment recording | new (`proposed`) | US-6 | SC-006 |
| FEAT-007 | Payment follow-up (parent) | new (`proposed`) | US-7, US-8 | — (roll-up) |
| FEAT-008 | ↳ Invoice status lifecycle | new (`proposed`) | US-7 | SC-007 (part) |
| FEAT-009 | ↳ Automated reminders | new (`proposed`) | US-8 | SC-007 (part) |

### Filter rejections

- None — every drafted story homed to exactly one feature.

### Selection

- **Selected (build now, dependency order):** FEAT-001 → FEAT-002 → FEAT-003 → FEAT-005. Foundation = FEAT-001. This is the founder's smallest valuable slice: sign up, add a client, draft, send, take an online payment, see it flip to paid. Selecting the leaf FEAT-005 flips its parent FEAT-004 (Payments) to `in-flight`.
- **Deferred (`proposed` on the map):** FEAT-006 — carries SC-006 and the EC-6 concurrent-payment seam obligation. FEAT-008 and FEAT-009 (under parent FEAT-007) — carry SC-007.
- **Deferred SCs:** SC-006 (waits for FEAT-006); SC-007 (waits for FEAT-008 + FEAT-009).
- **Completeness ledgers:** Payments (FEAT-004) — 0 delivered / 2 undelivered leaves (FEAT-005 selected, FEAT-006 deferred) / 0 stubs / 0 kills. Payment follow-up (FEAT-007) — 0 delivered / 2 undelivered leaves / 0 stubs / 0 kills.
- No deferral blocks the selected spine — no selected feature depends on a deferred one.

---

## Assumptions

- Stripe is the hosted-checkout and webhook provider referenced by the constraints; the spec stays processor-agnostic in requirements but the governance stack names Stripe. *(Confidence: Assumed — consistent with the intent constraints and governance.)*
- "Viewed" is detected via the client opening the invoice/payment link; exact mechanism is a design-track decision (deferred with FEAT-008).
- One tax rate per invoice is sufficient for v1 (no per-line-item tax, no tax engine). *(Confident — stated by the founder.)*

## Open Questions

- **Data retention & account deletion** *(deferred, do NOT guess)* — the legal retention obligation for financial records and the behavior on account deletion are unknown to the founder. Research the real obligation before any retention/deletion behavior is written; nothing in v1 ships hinging on it. Tracked in `BACKLOG.md` (Open questions).
- Reminder cadence defaults and bounds (first-reminder timing, repeat interval, stop condition) — the founder wants to react to concrete options in the prototype before locking; settle when FEAT-009 is specified for build.
