# Staged map delta — invoice-lifecycle-v1 (NOT the live map)

Derived by the product-manager seat (mochiko:authoring-feature-map) from the drafted stories.
Baseline: FEATURES.md git state at run open = **empty greenfield scaffold** (no delivered /
proposed / in-flight / stub territory). Everything here is **new**. This stages in the spec
workspace; the live map is written as one atomic batch only at spec acceptance.

## Story disposition (filter) — every story homed or rejected

| Story | Disposition | Home / reason |
|-------|-------------|---------------|
| US-1 Create a client | homed | FEAT-001 Client records |
| US-2 Draft an invoice | homed | FEAT-003 Invoice drafting |
| US-3 Send an invoice | homed | FEAT-004 Invoice delivery & payment |
| US-4 Get paid via hosted page | homed | FEAT-004 Invoice delivery & payment |
| US-5 Mark paid manually | homed | FEAT-004 Invoice delivery & payment |
| US-6 See statuses at a glance | homed | FEAT-005 Invoice status tracking |
| US-7 Send payment reminders | homed | FEAT-006 Payment reminders |
| US-8 See when a client viewed | homed | FEAT-005 Invoice status tracking |

**Filter result:** all 8 stories earn a place on the map; **no rejections**. The filter was
run (each story tested against the derived capabilities); none is scope creep or a story-mirror
duplicate. Recorded so the "filter never fired" red flag is a considered outcome, not an omission.

**Exactly-one-home check:** US-3 (send) and US-4/US-5 (payment) move invoice state that
FEAT-005 (status tracking) reads — FEAT-005 depends-on FEAT-004 by relation; it carries no
second home for those stories. No story has two homes.

## Proposed entries (all `proposed`; selected ones flip `in-flight` at acceptance)

### FEAT-001 — Client records  (flat leaf)
- Capability: Ledgerline keeps the contractor's client records (name, email, notes), scoped to
  their account, so invoices have an addressee.
- Extent: create, list, and soft-delete clients with a required email; per-account isolation;
  soft-delete retains invoices + audit (never cascade). Not: client logins/portal, client-side
  editing.
- Relations: none (foundation).
- Story trace: invoice-lifecycle-v1 — US-1.

### FEAT-002 — Invoice lifecycle  (PARENT — roll-up + navigation, never built directly)
- Capability: the contractor takes an invoice from creation through to a trustworthy paid state.
- Children: FEAT-003, FEAT-004, FEAT-005, FEAT-006.
- Story trace: invoice-lifecycle-v1 — US-2..US-8.

#### FEAT-003 — Invoice drafting  (leaf under FEAT-002)
- Capability: draft invoices with line items, a single invoice-level tax rate, and a due date,
  computing subtotal/tax/total exactly (Decimal).
- Extent: create/edit drafts, edit unpaid invoices after send (lock on paid), exact money math,
  draft-validity rules. Not: sending, payment, void/reissue of paid invoices (deferred),
  multi-jurisdiction/line-item tax, multi-currency.
- Relations: depends-on FEAT-001.
- Story trace: US-2.

#### FEAT-004 — Invoice delivery & payment  (leaf under FEAT-002)  ← highest rigor
- Capability: send an invoice by email carrying a Stripe hosted payment link, collect payment
  via a signature-verified, idempotent webhook, and record manual (check/cash) payments — so
  payment state reflects reality exactly once.
- Extent: send/resend, hosted-link payment, exactly-once webhook handling, manual mark-paid,
  append-only payment audit trail, reconciliation of a manual+hosted double-pay attempt. Not:
  partial payments (deferred), disputes, recurring billing.
- Relations: depends-on FEAT-003.
- Story trace: US-3, US-4, US-5.
- **Keep-whole rationale (review G6):** the extent is near the ~3-line leaf bar, but sending and
  payment capture share one indivisible invariant — an invoice's payment state must be exactly
  once and trustworthy across the send/hosted/manual paths. Splitting "send" from "capture" would
  cut that invariant across two leaves. Kept as one leaf deliberately; if it proves too large at
  plan time it is cut into vertical-slice cycles downstream, not into two features.

#### FEAT-005 — Invoice status tracking  (leaf under FEAT-002)
- Capability: show the contractor every invoice's current status at a glance (list + detail),
  including a computed overdue indicator and a best-effort viewed signal.
- Extent: status list, detail view with payment history, overdue computed from due date, viewed
  from hosted-page visit. Not: analytics/reporting, exports.
- Relations: depends-on FEAT-004 (reads sent/paid state).
- Story trace: US-6, US-8.

#### FEAT-006 — Payment reminders  (leaf under FEAT-002)  ← recommend DEFER
- Capability: send reminder emails for unpaid, sent invoices on contractor-configured intervals,
  stopping when paid.
- Extent: reminder scheduling + send, stop-on-paid. Not: dunning escalation, SMS. Reminder
  interval defaults and bounds are **deferred/undecided** (Assumptions).
- Relations: depends-on FEAT-004.
- Story trace: US-7.

## Recommended selection (PM recommends; the user rules)

- **Build now (dependency order):** FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005. This is exactly
  the principal's first shippable spine: create client → draft → send → get paid → see status.
- **Defer (`proposed`):** FEAT-006 Payment reminders — a deliberate cut. The spine ships and
  earns money without it; the principal asked that this deferral be explicit, not accidental.
  Its deferral means contractors chase unpaid invoices manually until it ships.
- **Foundation (first feature):** FEAT-001 Client records.

### Completeness ledger — parent FEAT-002 Invoice lifecycle
- Delivered leaves: 0 · Selected-to-build: FEAT-003, FEAT-004, FEAT-005 · Undelivered/deferred:
  FEAT-006 · Parked `unrefined` stubs: 0 · Kills: 0.

### Deferred SCs (travel with FEAT-006's obligations line)
- The success criteria verified only by reminders (reminder scheduling/stop-on-paid) wait until
  FEAT-006 builds. Enumerated against SC-IDs in spec.md § Success Criteria / Feature Selection.

## Index-line drafts (for FEATURES.md at acceptance)

| ID | Feature | Status | Capability hook |
|----|---------|--------|-----------------|
| FEAT-002 | Invoice lifecycle | in-flight | Create → send → get paid, trustworthy status |
| FEAT-001 | Client records | in-flight | Client records to address and bill |
| FEAT-003 | ↳ Invoice drafting | in-flight | Draft invoices with exact totals |
| FEAT-004 | ↳ Invoice delivery & payment | in-flight | Send + collect payment exactly once |
| FEAT-005 | ↳ Invoice status tracking | in-flight | Every invoice's status at a glance |
| FEAT-006 | ↳ Payment reminders | proposed | Reminders for unpaid invoices |

(Parent FEAT-002 is `in-flight` by roll-up once any child is selected/in-flight. FEAT-001 is a
flat feature, listed above its dependents. Final ordering in FEATURES.md: parent then leaves.)

## Staged specs-index row (for .mochiko/specs/index.md at acceptance)  (added round 1, review G5)

| Spec | Status | FEAT-IDs touched | About |
|------|--------|------------------|-------|
| [invoice-lifecycle-v1](invoice-lifecycle-v1/spec.md) | in-flight | FEAT-001, FEAT-002, FEAT-003, FEAT-004, FEAT-005 in-flight; FEAT-006 proposed | Contractor invoice create-to-paid spine; reminders derived, deferred |

Spec `status` is derived: it reads closed only when the selected FEAT-IDs (001/003/004/005) reach
`delivered`. FEAT-006 stays `proposed` and does not gate this spec's closure.
