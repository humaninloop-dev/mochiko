---
report: review
feature: invoice-lifecycle
round: 1
incremental: false
verdict: needs-revision
verdict_basis: "Sound spine, honest derivation and a walkable prototype, but one Critical access-control gap on the public payment link and several financial-correctness/state ambiguities must be resolved before build."
strengths: "Complete story disposition (all homed, none orphaned), honest deferral of FEAT-007/008 with deferred-SC list, exact-money framing, prototype walks end-to-end with no manifest drift, external Stripe boundary correctly modelled."
findings:
  - {id: G1, type: Missing, sev: Critical,
     at: "spec.md FR-013/FR-016; stories/US-3, US-4; prototype SCR-007 link",
     gap: "The client-facing payment link is an unauthenticated bearer URL keyed to the invoice number (prototype shows /i/INV-1005, a sequential, guessable id); no requirement makes the link unguessable or scopes it, so an outsider could enumerate other contractors' invoices — a cross-tenant exposure the tenant-isolation rule (GI-011) would otherwise forbid.",
     fix: "Add an FR: the payment link MUST use an unguessable token, MUST expose only that one invoice, and MUST NOT be enumerable; decide link expiry/revoke-on-paid."}
  - {id: G2, type: Ambiguous, sev: Important,
     at: "spec.md FR-009, SC-004",
     gap: "'Exact to the cent' does not define the rounding rule or whether tax is computed per line item or on the subtotal — for financial software these produce different totals and must be pinned.",
     fix: "State the rounding mode (e.g. round-half-up to the cent) and the tax computation basis (per-line vs on-subtotal) as a requirement."}
  - {id: G3, type: Missing, sev: Important,
     at: "spec.md Intent (lifecycle states), FR-017, Key Entities (Invoice state)",
     gap: "There is no payment-pending state between the client completing Stripe checkout and the confirming webhook arriving (FR-017 marks paid only on the webhook). Meanwhile SCR-008 shows the client 'paid' immediately. The contractor's invoice reads unpaid in that gap with nothing explaining it.",
     fix: "Decide and specify the intermediate state (e.g. 'payment processing') and what contractor and client each see between redirect and webhook confirmation."}
  - {id: G4, type: Contradiction, sev: Important,
     at: "prototype SCR-003 (FEAT-002, selected) vs FEAT-007 (deferred) / FR-024",
     gap: "The selected invoice list (SCR-003, FEAT-002) shows an 'overdue' badge, but overdue derivation is FEAT-007 which is deferred. Either the selected build cannot show overdue, or overdue computation must partly land in FEAT-002.",
     fix: "Rule where the overdue read lives: pull the overdue badge from SCR-003 until FEAT-007 builds, OR move minimal overdue derivation into FEAT-002 scope and adjust the selection."}
  - {id: G5, type: EdgeCase, sev: Important,
     at: "spec.md FR-008..FR-011 (authoring), FR-013 (sent)",
     gap: "Editing/voiding after send is unspecified: can a contractor edit or cancel a sent (or paid) invoice? Nothing says what happens to the client's link or the amount if a sent invoice is changed.",
     fix: "Specify post-send editability (lock on send, allow void/credit, or allow edit-with-resend) and its effect on the payment link and recorded amount."}
  - {id: G6, type: Ambiguous, sev: Minor,
     at: "spec.md FR-008 (invoice number)",
     gap: "Invoice number is contractor-entered but no uniqueness rule is stated; two invoices could share a number, and the payment link keys off it.",
     fix: "Require invoice number unique per contractor (and decide auto-suggest vs free entry)."}
  - {id: G7, type: EdgeCase, sev: Minor,
     at: "spec.md Key Entities (Client↔Invoice), FR-007",
     gap: "Deleting or editing a client that already has invoices is unspecified — orphaned invoices or altered historical client data on a sent invoice.",
     fix: "Specify client-delete behavior with existing invoices (block, soft-delete, or snapshot client details onto the invoice at send)."}
---

## Clarifications needed

### C1: Payment-link access control   (G1)

**Question**: How should the client reach their invoice payment page without logging in, while making sure no one can reach anyone else's?
**Options**: 1. Unguessable one-time-ish token per invoice, link shows only that invoice · 2. Token + expiry, re-issued on resend · 3. Require a lightweight code (e.g. emailed PIN) to open
**Why it matters**: Clients never log in, so the link is the only gate. A guessable link keyed to a sequential invoice number would let outsiders page through other contractors' invoices — a direct breach of the per-contractor isolation promise.

### C2: Money rounding & tax basis   (G2)

**Question**: How should tax and totals round, and is tax figured per line or on the subtotal?
**Options**: 1. Round half-up to the cent, tax on subtotal · 2. Round half-up per line then sum · 3. Banker's rounding on subtotal
**Why it matters**: Different choices change the total a client is charged by a cent or more; for money software the rule must be explicit and testable (SC-004).

### C3: What the contractor sees while a Stripe payment settles   (G3)

**Question**: Between the client paying on Stripe and Ledgerline getting the confirming event, what state should the invoice show?
**Options**: 1. A 'payment processing' state visible to the contractor · 2. Stay 'sent' silently until confirmed · 3. Optimistically show 'paid', reconcile if the event never lands
**Why it matters**: The client already sees "paid"; if the contractor sees "unpaid" with no explanation they may chase a client who already paid.

### C4: Overdue in the first build   (G4)

**Question**: The status dashboard (overdue tracking) is deferred, but the invoice list still needs to flag overdue — where should that come from now?
**Options**: 1. Compute the overdue badge on read in the FEAT-002 list now; full dashboard later · 2. Don't show overdue at all until FEAT-007 builds · 3. Pull minimal overdue derivation into the first build
**Why it matters**: Contractors said knowing who's overdue is core; but the deferral means the selected build has no overdue source unless this is ruled.

### C5: Editing or voiding a sent invoice   (G5)

**Question**: After an invoice is sent (or paid), can the contractor change or cancel it?
**Options**: 1. Lock on send; corrections via a new invoice / credit · 2. Allow void + resend, amount re-locked · 3. Allow free edit with automatic resend
**Why it matters**: Real invoices get corrected; without a rule the amount owed and the live payment link can silently disagree.
