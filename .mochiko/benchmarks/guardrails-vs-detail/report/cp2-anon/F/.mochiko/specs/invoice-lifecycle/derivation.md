# Derivation (staged) — invoice-lifecycle

Staged in the spec workspace per the write rules — the live map (`FEATURES.md`,
`.mochiko/features/`) is NOT written until spec acceptance. Map-delta baseline: the map's git
state at run open (empty — greenfield first specify; `FEATURES.md` index shows `_none yet_`).

Derived from the drafted stories (US-1…US-8, plus US-9 void added by ruling at spec review)
against the actual (empty) map. No existing capability to extend or dedup against; every derived
feature is `new (proposed)`.

## Structure

- **FEAT-001 Contractor accounts & authentication** — flat leaf (foundation). Distinct capability
  (accounts), prerequisite for everything.
- **FEAT-002 Invoice lifecycle** — PARENT (navigation + roll-up, never built directly). Children
  FEAT-003…008. Named in one breath; decomposes into deliverable leaves.

## Staged entries

### FEAT-001 — Contractor accounts & authentication  (flat leaf, foundation)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: A contractor registers and authenticates (email+password or Sign in with Google); all data is isolated to their account.
- Extent: sign-up, sign-in (both methods), email verification on the password path, verified-email account merge, per-account tenant isolation. Not: teams/multi-user, client logins.
- Relations: (none) — foundation; every other feature depends-on this.
- Story trace: invoice-lifecycle: US-1
- Obligations: cross-cutting — tenant isolation (GI-011) applies to every feature's data access.

### FEAT-002 — Invoice lifecycle  (parent)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Children: FEAT-003, FEAT-004, FEAT-005, FEAT-006, FEAT-007, FEAT-008
- Capability: The end-to-end life of an invoice — from drafting through delivery, payment, tracking, and follow-up.
- Roll-up: proposed (no child delivered yet).

### FEAT-003 — Client management  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: A contractor maintains the clients they bill (name, email), the payer an invoice is addressed to.
- Extent: create/edit/list clients, email validation. Not: client portal/login, client-side accounts.
- Relations: depends-on FEAT-001.
- Story trace: invoice-lifecycle: US-2
- Obligations: extend — support a "new client" jump from the invoice editor without losing the draft (US-3, homed to FEAT-004).

### FEAT-004 — Invoice authoring  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: A contractor drafts an invoice with line items, a single tax rate, and a due date; totals compute exactly and a gap-free number is assigned at send.
- Extent: line items, single tax rate (rounded half-up), due date, exact Decimal totals, per-account gap-free numbering assigned at send, draft save + unsaved-state indication, draft deletion, new-client jump from editor. Not: multi-currency, recurring, partial payments, PDF export.
- Relations: depends-on FEAT-003.
- Story trace: invoice-lifecycle: US-3

### FEAT-005 — Invoice delivery  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: A contractor sends an invoice to the client by authenticated email including a Stripe hosted payment link; the invoice becomes sent (and viewed if detectable).
- Extent: authenticated email (SPF/DKIM/DMARC), hosted payment link, sent status, bounce handling. Not: SMS delivery, in-app client inbox.
- Relations: depends-on FEAT-004.
- Story trace: invoice-lifecycle: US-4
- Obligations: `viewed` status contingent on reliable view detection (open question US-4) — drop rather than fake if unreliable.

### FEAT-006 — Payment tracking & reconciliation  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: Invoice payment state reflects reality — auto-reconciled from Stripe webhooks and settable manually for checks/cash — with an append-only audit trail.
- Extent: Stripe webhook reconciliation (signature-verified, idempotent, full-amount-only auto-mark, off-amount held/flagged), status lifecycle, manual mark-paid, pay-link deactivation on paid, void (terminal), append-only audit log, Decimal money. Not: partial payments, refunds.
- Relations: depends-on FEAT-005.
- Story trace: invoice-lifecycle: US-5, US-6, US-9

### FEAT-007 — Payment reminders  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: Unpaid invoices past due are auto-flagged overdue and the client is reminded on a fixed cadence until paid.
- Extent: auto overdue detection, reminders at due/+3/+7 (cap 3, stop on paid), per-invoice off-switch, reminder state visible on invoice detail. Not: custom cadences, per-account reminder templates.
- Relations: depends-on FEAT-006 (needs status + email path).
- Story trace: invoice-lifecycle: US-7

### FEAT-008 — Invoice dashboard & status view  (leaf, parent FEAT-002)
> Status: proposed — surfaced by invoice-lifecycle (2026-08-10)
- Capability: A contractor sees all invoices and their live payment status in one place, with outstanding/paid totals and status filtering.
- Extent: invoice list with status/client/amount/due, outstanding & paid totals (void excluded from outstanding), filter by status, overdue-precedence display. Not: reporting/analytics, exports.
- Relations: depends-on FEAT-006 (renders reconciled status).
- Story trace: invoice-lifecycle: US-8
- Obligations: extend — surface each invoice's reminder state (next reminder / off) on the invoice detail view (US-7, homed to FEAT-007).

## Filter — story disposition (complete)

All 9 stories earn a place on the map; none rejected. Each homes to exactly ONE feature:

| Story | Home | Also touches (extend obligation) |
|-------|------|----------------------------------|
| US-1 | FEAT-001 | (tenant isolation obligation rides all features) |
| US-2 | FEAT-003 | — |
| US-3 | FEAT-004 | FEAT-003 (new-client jump) |
| US-4 | FEAT-005 | — |
| US-5 | FEAT-006 | — |
| US-6 | FEAT-006 | — |
| US-7 | FEAT-007 | FEAT-008 (reminder state shown on detail) |
| US-8 | FEAT-008 | — |
| US-9 | FEAT-006 | — |

No silent drops; no orphans. The filter fired and found no story that fails to earn a feature —
the request is a coherent in-scope v1, so a manufactured rejection would be dishonest. (US-9 void
was added by principal ruling at spec review; it homes to FEAT-006, the payment-state feature.)

## SC → verifying feature (completed during spec authoring; SC IDs from spec.md)

- SC-001 (signup→sent-invoice unassisted): FEAT-001, 003, 004, 005
- SC-002 (exact Decimal totals): FEAT-004
- SC-003 (gap-free numbering): FEAT-004
- SC-004 (online payment auto-reconciles, exactly once): FEAT-006
- SC-005 (manual mark-paid + audit log): FEAT-006
- SC-006 (dashboard status + filter correctness): FEAT-008
- SC-007 (overdue + reminders cadence): FEAT-007
- SC-008 (in-product payment state matches Stripe, no Stripe login): FEAT-006, FEAT-008
- SC-009 (authenticated email + bounce handling): FEAT-005, FEAT-007
- SC-010 (cross-account isolation): FEAT-001 (cross-cutting)

## Selection card (PM recommended; the user ruled)

- **Recommended selection (build now), dependency order:** FEAT-001 → FEAT-003 → FEAT-004 →
  FEAT-005 → FEAT-006 → FEAT-008, then FEAT-007.
- **Foundation (first by dependency):** FEAT-001 Contractor accounts & authentication.
- **Deferral candidate (flagged):** FEAT-007 Payment reminders (P2) — the one leaf that could ship
  after a first cut without breaking the core loop (draft→send→get-paid→see-status).
- **Completeness ledger — parent FEAT-002 Invoice lifecycle:** 0 delivered / 6 undelivered leaves
  / 0 parked stubs / 0 kills.
- No dependency-blocked leaf or stub requiring forced escalation (all leaves selected in order).

**USER RULING (spec acceptance, 2026-08-10):** all seven deliverable features selected for v1 —
FEAT-001 plus FEAT-002's six leaves (FEAT-003, FEAT-004, FEAT-005, FEAT-006, FEAT-008, FEAT-007),
in the recommended dependency order. Reminders kept in v1; no feature deferred; no deferred SCs.
