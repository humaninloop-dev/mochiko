---
report: review
feature: invoice-lifecycle-v1
round: 1
incremental: false
verdict: needs-revision
verdict_basis: "Sound spine and strong money-path rigor; gaps in email-bounce handling, invoice correction/void, client-deletion behavior, P1 scenario→flow coverage, and a missing specs-index row draft — all addressable, none foundational."
strengths: "Exact-decimal money path, exactly-once + idempotent payment state, append-only audit, cross-tenant isolation, and manual+hosted double-pay reconciliation all specified; prototype fully walkable; all 8 stories dispositioned."
findings:
  - {id: G1, type: EdgeCase, sev: Important, at: "spec.md FR-009/FR-010, EC-1",
     gap: "Send path covers only provider-accept failure; a later async email bounce is unspecified though governance GI-030 requires bounce handling for transactional invoice/reminder email.",
     fix: "Add an FR: a bounced invoice email MUST surface to the contractor and MUST NOT leave the invoice looking successfully delivered."}
  - {id: G2, type: Missing, sev: Important, at: "spec.md FR-008/FR-009, US-2/US-3",
     gap: "No path to correct or void a SENT invoice with a wrong amount — a money product usually needs one, or an explicit exclusion; today a wrong sent invoice is a dead end.",
     fix: "User ruling: (a) add a void/reissue path to v1, or (b) explicitly out-scope it with the manual workaround stated."}
  - {id: G3, type: Missing, sev: Important, at: "spec.md Key Entities Client, FEAT-001",
     gap: "Deleting a client with open invoices is unspecified — orphaned invoices vs. blocked deletion vs. soft-delete; audit/retention implications.",
     fix: "User ruling on delete behavior; likely block deletion while invoices exist, or soft-delete, given the append-only audit + retention open question."}
  - {id: G4, type: EdgeCase, sev: Important, at: "spec.md Screens & Flows; prototype/",
     gap: "Not every P1 acceptance scenario carries a FLOW (review check 3). Validation/backend-only P1 scenarios — invalid-draft reject (US-2.3), cross-tenant denial (US-1.3), forged/replayed event (US-4.2/4.3), already-paid guard (US-5.2) — are rendered as inline states, not click flows.",
     fix: "Confirm the low-fi scoping is acceptable (backend/validation scenarios have no honest click path), or add lightweight state-transition flows for the UI-visible ones (invalid-draft, already-paid guard)."}
  - {id: G5, type: Assumption, sev: Important, at: "derivation/staged-map-delta.md",
     gap: "Staged acceptance batch omits the .mochiko/specs/index.md row draft; feature-layer check 10 (specs-index agreement) cannot be graded from the staging file.",
     fix: "Add the staged specs-index row (slug, status, FEAT-IDs + outcomes, one-line about) to the staged batch."}
  - {id: G6, type: Ambiguous, sev: Minor, at: "derivation/staged-map-delta.md FEAT-004",
     gap: "FEAT-004 extent spans send + hosted payment + webhook idempotency + manual mark-paid + audit + reconciliation — at/over the one-leaf ~3-line bar.",
     fix: "Either split into Invoice sending vs Payment capture & reconciliation, or record a deliberate keep-whole rationale (the pieces share one payment-state invariant)."}
  - {id: G7, type: Missing, sev: Minor, at: "spec.md Screens & Flows SCR-001",
     gap: "SCR-001 (sign in) carries no FEAT tag; acceptable as the app shell but the no-tag state should be explicit, not an omission.",
     fix: "Keep SCR-001 tagged '— (app shell / auth constraint)'; confirm auth is intentionally not a feature in this spec."}
---

## Clarifications needed

### C1: Correcting or voiding a sent invoice   (G2)

**Question**: A contractor sends an invoice, then notices a wrong amount or wrong client. What can they do?
**Options**: 1. Add a void + reissue path to v1 · 2. Out-scope corrections for v1 (workaround: mark/ignore and send a new invoice), stated explicitly · 3. Allow edit only while unpaid, locking on payment.
**Why it matters**: A money product with no correction path forces contractors to a confusing workaround on their first mistake; but adding void/reissue widens v1 scope and the audit surface.

### C2: Deleting a client with open invoices   (G3)

**Question**: What happens to a client's invoices when the contractor deletes the client?
**Options**: 1. Block deletion while any invoice exists · 2. Soft-delete the client, keep invoices intact · 3. Cascade-delete (loses records — conflicts with append-only audit + retention).
**Why it matters**: Financial records have retention exposure (open question); a hard cascade could destroy auditable payment history.

### C3: P1 scenario flow coverage in the prototype   (G4)

**Question**: Should the prototype add click flows for UI-visible negative P1 scenarios, or is rendering them as inline states enough for a low-fi spec artifact?
**Options**: 1. Accept inline states (backend scenarios like forged webhook have no honest click path anyway) · 2. Add lightweight flows for invalid-draft and already-paid-guard only · 3. Full per-scenario flows.
**Why it matters**: Full coverage catches UX gaps early; over-building the throwaway prototype spends effort the build won't reuse.
