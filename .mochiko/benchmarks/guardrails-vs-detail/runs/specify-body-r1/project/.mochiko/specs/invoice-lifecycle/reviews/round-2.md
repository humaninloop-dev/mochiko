---
report: review
feature: invoice-lifecycle
round: 2
incremental: true
scope: "re-review of round-1 findings G1–G7 after the clarification-gate revision"
verdict: ready
verdict_basis: "All seven round-1 findings resolved by pinned requirements (FR-016 token link, FR-010 rounding, FR-022/023 processing state, FR-013 overdue-on-read, FR-019/020 lock+void, FR-009 unique number, plus client-delete assumption); prototype and feature entries updated to match; no new blocking gap."
strengths: "Critical access-control gap closed with an unguessable-token requirement; financial rounding pinned; state model now covers the settlement gap; deferral honesty preserved (SC-006 the only deferred SC)."
findings:
  - {id: G1, type: Missing, sev: Critical, at: "spec.md FR-016; prototype SCR-005 link", gap: "RESOLVED — payment link now MUST use an unguessable per-invoice token, opens only that invoice, non-enumerable, valid until paid/voided.", fix: "Verified in FR-016 and SCR-005 (token URL replaces the invoice-number URL)."}
  - {id: G2, type: Ambiguous, sev: Important, at: "spec.md FR-010, SC-004", gap: "RESOLVED — rounding half-up to the cent, tax on the subtotal, exact non-floating arithmetic.", fix: "Verified in FR-010, SC-004."}
  - {id: G3, type: Missing, sev: Important, at: "spec.md FR-022/FR-023, Key Entities", gap: "RESOLVED — 'payment processing' state added between redirect and webhook; stays processing if unconfirmed, flagged.", fix: "Verified in FR-022/FR-023, Invoice entity state enum, SCR-005 legend."}
  - {id: G4, type: Contradiction, sev: Important, at: "spec.md FR-013; FEAT-002 vs FEAT-007", gap: "RESOLVED — overdue flag computed on read ships in FEAT-002's list now; the filterable dashboard stays deferred in FEAT-007.", fix: "Verified in FR-013, FEAT-002/FEAT-007 extents, SC-005 remapped to FEAT-002."}
  - {id: G5, type: EdgeCase, sev: Important, at: "spec.md FR-019/FR-020", gap: "RESOLVED — sent invoices locked; corrections by new invoice/credit; void cancels + invalidates the link.", fix: "Verified in FR-019/FR-020, FEAT-003 extent, SCR-005 void action."}
  - {id: G6, type: Ambiguous, sev: Minor, at: "spec.md FR-009", gap: "RESOLVED — invoice number MUST be unique per contractor.", fix: "Verified in FR-009, Invoice entity."}
  - {id: G7, type: EdgeCase, sev: Minor, at: "spec.md Assumptions", gap: "PARTIALLY ADDRESSED — client-delete-with-invoices called out; snapshot-vs-block left as a design detail. Acceptable at spec depth.", fix: "Assumption notes client interaction model; full delete behavior deferred to design — non-blocking."}
---
