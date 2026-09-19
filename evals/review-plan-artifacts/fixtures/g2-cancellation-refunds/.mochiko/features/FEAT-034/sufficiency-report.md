---
report: sufficiency
feature: FEAT-034
round: 1
scope: selection
units:
  - {row: FEAT-034/W1, verdict: gaps, gaps: [G-01, G-02]}
gaps:
  - {id: G-01, clause: contract-exposure, surface: "cancel a booking; owner's refund view; marina refund ledger", grade: named-and-unattachable, note: "no cancel or refund seam in contracts/api.yaml; the design phase authors the delta"}
  - {id: G-02, clause: data-exposure, surface: "Refund (new); Booking gains a cancelled state", grade: named-and-unattachable, note: "new entity; Booking's state machine gains a transition"}
store_consult:
  read: [ARCHITECTURE.md index, concerns.md AX summary table]
  trips: []
  no_delta_claim: "No structural change: cancellation and refunds ride the existing api, worker, db and the standing Stripe boundary; no new container, boundary, or flow."
commodity_exposure: "no gap — refund state rides Stripe's Refund object plus PostgreSQL rows; queueing via D-001 pg-boss; no new storage, locking, or serialization need"
nfr_targets_applicable: [AX-004/NFR-002, AX-007/NFR-003]
ux_trace: n/a — no Screens & Flows manifest (prototype waived at intent)
delivered_feature_exposure:
  - {feature: FEAT-012, surface: Booking, amendment: "[MODIFY] Booking gains status `cancelled`, `cancelledAt`, and `cancellationReason`; written as the marked delta on FEAT-012's entry"}
in_flight_exposure: none
quickstart: "not applicable — no external integration surface beyond the standing Stripe boundary"
---
