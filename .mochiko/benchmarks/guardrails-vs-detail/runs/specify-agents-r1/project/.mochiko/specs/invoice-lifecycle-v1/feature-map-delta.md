# Staged feature-map delta — invoice-lifecycle-v1

> Derivation output, staged in the spec workspace per the write rules (the live `FEATURES.md`
> and `.mochiko/features/` stay untouched until spec acceptance). Baseline = git state of the map
> at run open: `FEATURES.md` empty (greenfield, no entries), no `.mochiko/features/` files.
> Graded with the spec by mochiko:review-specifications. All entries are NEW (`proposed`);
> selected ones flip to `in-flight` at acceptance.

## Derivation summary — stories first, features derived

| Story | Feature home | Note |
|-------|--------------|------|
| US-1 Create & send invoice | FEAT-001 | authoring core; depends on FEAT-007 for client |
| US-2 Client pays via Stripe (no account) | FEAT-003 | online settlement + reconciliation |
| US-3 Monitor invoices & status | FEAT-005 | list + detail + history view |
| US-4 Record off-platform payment | FEAT-004 | manual settlement |
| US-5 Void an incorrect sent invoice | FEAT-006 | kill path + audit |
| US-6 See overdue at a glance | FEAT-005 | overdue is a computed read — extent of the tracking view, not its own feature |
| US-7 Automated overdue reminders | FEAT-008 | filter candidate → homed by principal ruling (reminders core v1) |
| US-8 Manage clients | FEAT-007 | saved client records invoices are issued against |

**Disposition completeness:** all 8 drafted stories homed to exactly one feature. Zero rejections
this run — US-7 was drafted as the filter's rejection candidate (cadence undecided at intent), then
homed by the principal's ruling on the prototype walk (recorded in `stories/US-7.md`). The filter
fired (US-7 was genuinely weighed and escalated); it did not need to reject.

**Dedup:** greenfield map — nothing to dedup against; no `unrefined` stubs to confirm.

## Proposed entries (staged; land at acceptance)

### FEAT-001 — Invoice authoring & sending  (flat leaf)
- Status: proposed → in-flight (selected)
- Capability: Create, edit, number, and send invoices against a saved client — line items, tax, due date — delivering the hosted payment link to the client by email.
- Extent: in — draft create/edit, tax-inclusive totals (Decimal), per-contractor sequential non-editable invoice numbering, send + `draft`→`sent`, payment-link generation, authenticated email delivery + copyable fallback, edit/resend of a `sent` unpaid invoice (locked once paid), transitions recorded to the audit trail. Not — payment collection (FEAT-003), status display (FEAT-005), void of a paid invoice (FEAT-006).
- Relations: depends-on FEAT-007 (issues against a saved client).
- Story trace: invoice-lifecycle-v1 — US-1.
- Obligations: audit-trail write on send/edit (GI-029); authenticated invoice email (GI-030 — pulled into v1 core by principal ruling; reminders FEAT-008 stay deferred); tenant isolation on invoice writes (GI-011).

### FEAT-002 — Payment settlement & reconciliation  (PARENT)
- Status: proposed → in-flight (a child is selected)
- Capability: Settle an invoice to `paid` and keep payment state reconciled with the truth — online via Stripe and manually for off-platform payments.
- Children: FEAT-003 (Stripe-hosted payment & reconciliation), FEAT-004 (Manual payment recording).
- Note: capability-first parent — settlement decomposes into online and manual paths; both selected.

### FEAT-003 — Stripe-hosted payment & reconciliation  (leaf under FEAT-002)
- Status: proposed → in-flight (selected)
- Capability: Accept a client's payment through Stripe-hosted checkout with no client account, and reconcile the invoice to `paid` against Stripe as the source of truth, exactly once.
- Extent: in — pay-without-account happy path, `viewed` on link open, signature-verified webhook, idempotent (exactly-once) reconciliation to `paid`, audit-trail write. Not — manual settlement (FEAT-004), card data handling (Stripe-hosted only, GI-014).
- Relations: depends-on FEAT-001.
- Story trace: invoice-lifecycle-v1 — US-2.
- Obligations: audit-trail write on payment state change (GI-029); webhook trust + idempotency (GI-026); tenant isolation on payment-state writes (GI-011).

### FEAT-004 — Manual payment recording  (leaf under FEAT-002)
- Status: proposed → in-flight (selected)
- Capability: Record an off-platform (cash/check) settlement so an invoice reconciles to `paid` outside Stripe, distinguishably from a Stripe payment.
- Extent: in — mark-as-paid with method + date, `paid` transition, manual-vs-Stripe provenance, audit-trail write, double-settle refusal. Not — Stripe collection (FEAT-003).
- Relations: depends-on FEAT-001.
- Story trace: invoice-lifecycle-v1 — US-4.
- Obligations: audit-trail write on payment state change (GI-029); tenant isolation on payment-state writes (GI-011).

### FEAT-005 — Invoice tracking & lifecycle view  (flat leaf)
- Status: proposed → in-flight (selected)
- Capability: Present a contractor's invoices and their lifecycle state — a tenant-scoped list, an invoice detail with status history, and a computed overdue indicator over unpaid, past-due invoices in sent or viewed status.
- Extent: in — invoice list, detail view (sent and paid states), status history (incl. `viewed` timestamp and settlement row), computed overdue badge over `sent`/`viewed` unpaid past-due (no stored status). Not — editing/sending (FEAT-001), state transitions themselves.
- Relations: depends-on FEAT-001.
- Story trace: invoice-lifecycle-v1 — US-3, US-6.
- Obligations: tenant isolation on list/detail (GI-011).

### FEAT-006 — Invoice void  (flat leaf)
- Status: proposed (DEFERRED — fast-follow, not selected this run)
- Capability: Void an incorrect sent invoice so it is no longer payable, retaining the record and recording who voided it and when.
- Extent: in — void a sent/overdue invoice, block further payment, retain record, audit-trail write, refuse voiding a paid invoice. Not — deletion (records are retained).
- Relations: depends-on FEAT-001.
- Story trace: invoice-lifecycle-v1 — US-5.
- Obligations: audit-trail write on void (GI-029, principal-ruled parity with payment state); carries the deferred void-audit clause of SC-004 until built.

### FEAT-007 — Client records  (flat leaf)
- Status: proposed → in-flight (selected) — FOUNDATION (first by dependency order)
- Capability: Maintain a contractor's saved clients — name, email, optional mailing address — that invoices are issued against, tenant-scoped.
- Extent: in — add/list/select client (name + email required, address optional). Not — client logins/portal (out of scope), phone/notes (deferred), editing history.
- Relations: (none inbound); FEAT-001 depends on it.
- Story trace: invoice-lifecycle-v1 — US-8.
- Obligations: tenant isolation (GI-011).

### FEAT-008 — Overdue reminder emails  (flat leaf)
- Status: proposed (DEFERRED — fast-follow, not selected this run)
- Capability: Automatically email a client overdue reminders on a fixed cadence (3, 7, 14 days overdue), with a per-invoice off toggle, stopping on settlement.
- Extent: in — reminder scheduling at 3/7/14 days overdue, per-invoice off toggle, stop-on-paid/void. Not — arbitrary custom cadences, dunning/disputes (out of scope).
- Relations: depends-on FEAT-001; depends-on FEAT-005 (computed-overdue read).
- Story trace: invoice-lifecycle-v1 — US-7.
- Obligations: authenticated transactional email (GI-030); carries deferred SC-006 until built.

## SC re-homing

| SC | Verified by |
|----|-------------|
| SC-001 create→send→Stripe-paid end-to-end | FEAT-007, FEAT-001, FEAT-003 |
| SC-002 `paid` always reflects a real settlement | FEAT-003, FEAT-004 |
| SC-003 replayed Stripe event never double-applies | FEAT-003 |
| SC-004 every payment/void change in the append-only audit trail, visible in history | FEAT-003, FEAT-004, FEAT-006, FEAT-005 (view) |
| SC-005 overdue shown accurately, nothing mis-flagged | FEAT-005 |
| SC-006 reminders fire at 3/7/14 days overdue, stop on settlement, respect per-invoice off | FEAT-008 |
| SC-007 a contractor sees only their own invoices and clients | FEAT-005, FEAT-007 |
| SC-008 clients pay without an account or login | FEAT-003 |

**Deferred SCs** (covered only by unselected features — travel with their `proposed` entry):
- SC-006 → FEAT-008 (reminders deferred).
- SC-004 void-audit clause → FEAT-006 (void deferred); SC-004 is otherwise met now for payment-state changes by FEAT-003/004/005.
All other SCs (SC-001/002/003/005/007/008) are verified by selected features → this delivery's done-condition.

## Selection outcome (user ruling)

- **Selected (build now, dependency order):** FEAT-007 → FEAT-001 → FEAT-005 → FEAT-003 → FEAT-004. (FEAT-002 parent → in-flight, both selected children.) Foundation: FEAT-007.
- **Deferred (`proposed`, fast-follow):** FEAT-006 (void), FEAT-008 (reminders). Reason (recorded): neither blocks a contractor getting paid; reminder cadence settled but held to keep launch lean; void is a fast-follow.
- **Deferred SCs:** SC-006 (waits for FEAT-008); SC-004 void-audit clause (waits for FEAT-006).
- **Completeness ledger — FEAT-002 (parent):** 2 leaves (FEAT-003, FEAT-004), both selected/in-flight · 0 undelivered · 0 parked stubs · 0 kills. No dependency-blocked leaf/stub (the deferred FEAT-006/008 block no incoming selected work).

## Staged index lines (FEATURES.md) — land at acceptance

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| FEAT-007 | Client records | in-flight | Saved clients invoices are issued against |
| FEAT-001 | Invoice authoring & sending | in-flight | Create, tax-total, and send invoices with a hosted payment link |
| FEAT-005 | Invoice tracking & lifecycle view | in-flight | List, detail, status history, computed overdue |
| FEAT-002 | Payment settlement & reconciliation | in-flight | Settle invoices to paid and keep state reconciled with the truth |
| FEAT-003 | ↳ Stripe-hosted payment & reconciliation | in-flight | Client pays via Stripe-hosted checkout; exactly-once reconcile to paid |
| FEAT-004 | ↳ Manual payment recording | in-flight | Record cash/check settlement outside Stripe |
| FEAT-006 | Invoice void | proposed | Void a wrong sent invoice, retained + audited |
| FEAT-008 | Overdue reminder emails | proposed | Auto-remind clients at 3/7/14 days overdue |

(Ordering: in-flight first, then proposed; leaf rows sit under their parent — FEAT-003/004 under FEAT-002.)

## Staged specs-index row (.mochiko/specs/index.md) — lands at acceptance

`invoice-lifecycle-v1 · in-flight · FEAT-007/001/005/002/003/004 (→in-flight), FEAT-006/008 (proposed, deferred) · Invoice lifecycle v1 — clients, author/send, tracking, Stripe + manual settlement; void + reminders deferred`
