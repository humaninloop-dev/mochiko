---
report: review
feature: invoice-lifecycle-v1
round: 1
incremental: false
verdict: needs-revision
verdict_basis: "Strong lifecycle spec with solid payment-integrity coverage, but the flow manifest under-covers P1 scenarios (Critical) and two capability prerequisites (invoice numbering, auth boundary) are unspecified."
strengths: "Payment integrity thorough (webhook-trust FR-013, idempotency FR-014, append-only audit FR-017, Decimal FR-018); tenant isolation explicit FR-003; out-of-scope honest and re-confirmed; prototype serves and every screen reachable."
findings:
  - {id: G1, type: Missing, sev: Critical,
     at: "spec.md Screens & Flows FLOW table",
     gap: "Only one FLOW per story (the happy path). P1 acceptance scenarios with no click path: US-1/2 (edit client), US-1/3 & US-2/3 (validation errors), US-3/2 (resend), US-3/3 (send failure), US-4/2 (abandoned → viewed), US-4/3 & US-5/2/3 (duplicate event, mark-paid guard, undo). review-specifications check 3 / authoring-prototype invariant 3: every P1 scenario needs a flow.",
     fix: "Extend the FLOW manifest (and prototype paths) to key a flow to each P1 acceptance scenario, or record the ones deliberately left unpathed with a reason."}
  - {id: G2, type: Missing, sev: Important,
     at: "spec.md Functional Requirements (FEAT-003)",
     gap: "Prototype shows invoices as '#1042' but no FR defines a human-facing invoice number/identifier or its uniqueness per contractor.",
     fix: "Add an FR: each invoice MUST carry a per-contractor unique, human-facing number."}
  - {id: G3, type: Assumption, sev: Important,
     at: "spec.md SC-001 / Overview",
     gap: "SC-001 measures 'signup to first invoice' and every story assumes a signed-in contractor, but no story or FR covers account creation / authentication. Governance requires auth on every endpoint; the boundary between this feature and platform auth is unstated.",
     fix: "State auth/account as a platform prerequisite (separate feature) in Assumptions, or add auth FRs to scope; reconcile SC-001 which currently spans signup."}
  - {id: G4, type: EdgeCase, sev: Important,
     at: "spec.md OQ-1 / FEAT-007 obligations",
     gap: "A disputed invoice keeps receiving scheduled reminders — no dispute state and no per-invoice reminder pause in v1. Currently only an open question; needs a product ruling before build.",
     fix: "Route OQ-1 to the principal at the disposition gate: accept-for-v1 (global off / void) vs. per-invoice pause vs. cancel/void state."}
  - {id: G5, type: Ambiguous, sev: Minor,
     at: "spec.md FR-007",
     gap: "FR-007 says a draft is editable until sent, but immutability of a sent invoice's amounts/line items is not stated — can a sent invoice be edited?",
     fix: "State whether a sent invoice is immutable (and if a correction requires void + reissue)."}
  - {id: G6, type: Ambiguous, sev: Minor,
     at: "spec.md Out of scope / FRs",
     gap: "Multi-currency is out of scope but no FR pins the positive constraint (all amounts are USD).",
     fix: "Add a one-line FR or assumption: all monetary amounts are USD in v1."}
  - {id: G7, type: Ambiguous, sev: Minor,
     at: "staged-map FEAT-005",
     gap: "FEAT-005 bundles online payment and manual mark-paid — two distinct mechanisms — under one leaf. Defensible (both record payment) but worth confirming it is one pipeline unit, not two.",
     fix: "Confirm the single-leaf decision, or split online vs. manual into two leaves."}
---

## Clarifications needed

### C1: Disputed-invoice reminders (G4)

**Question**: When a client disputes an invoice's amount, what should happen to the automatic reminders?
**Options**: 1. Accept for v1 — contractor turns reminders off globally or voids the invoice · 2. Add a per-invoice "pause reminders" toggle · 3. Add an explicit cancel/void invoice state that also halts reminders
**Why it matters**: Reminders nagging a client mid-dispute every 7 days is a real reputational cost for a solo contractor; the founder raised it unprompted at the prototype walk.

### C2: Auth / account scope boundary (G3)

**Question**: Is contractor signup/authentication part of this feature's scope, or a separate platform feature this spec depends on?
**Options**: 1. Separate platform feature — this spec assumes a signed-in contractor (adjust SC-001) · 2. In scope — add auth stories and FRs here
**Why it matters**: Every story silently assumes a signed-in, isolated contractor; leaving auth unhomed risks it falling through the cracks between features.

## Prototype walk notes

Served structure verified: all 8 SCR pages reachable from `index.html`; FLOW-001…FLOW-007 each
walk end-to-end with no dead ends; SCR-008 correctly marked external (Stripe-hosted). Data shape
honest (5-row invoice list, realistic amounts/dates). Low-fi discipline held — no cosmetic
findings raised (advisory half). The gap is coverage (G1), not walkability.
