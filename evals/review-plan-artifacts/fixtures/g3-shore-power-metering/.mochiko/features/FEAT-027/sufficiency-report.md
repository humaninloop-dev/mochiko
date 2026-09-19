---
report: sufficiency
feature: FEAT-027
round: 1
scope: selection
units:
  - {row: FEAT-027/W1, verdict: gaps, gaps: [G-01, G-02, G-03, G-04, G-05]}
gaps:
  - {id: G-01, clause: contract-exposure, surface: "readings ingest from the gateway; owner consumption view; marina tariffs", grade: named-and-unattachable, note: "no seam in contracts/api.yaml; the design phase authors the delta"}
  - {id: G-02, clause: data-exposure, surface: "Meter, MeterReading, Tariff", grade: named-and-unattachable, note: "new entities"}
  - {id: G-03, clause: structural-trigger, surface: "a new ingest path from an external gateway; AX-013 is open", grade: delta-needed, note: "the design phase authors a store delta; AX-013 takes its first ruling there"}
  - {id: G-04, clause: commodity-exposure, surface: "time-series storage for readings (storage)", grade: unresolved, note: "adopt-first answerable; the design phase weighs and records"}
  - {id: G-05, clause: nfr-targets, surface: "SC-001 ingest-to-visible lag", grade: absent, note: "AX-013 carries no target; the delta states it on the row"}
store_consult:
  read: [ARCHITECTURE.md index, concerns.md AX summary table, spine.md deep view]
  trips: [{row: AX-013, kind: open-row-touched, disposition: "ruled at the design checkpoint via the delta"}]
  no_delta_claim: null
nfr_targets_applicable: [AX-007/NFR-003]
ux_trace: n/a — no Screens & Flows manifest (prototype waived at intent)
delivered_feature_exposure: none
in_flight_exposure: none
quickstart: "required — the Enerlink gateway is an external integration surface (x-integration on the ingest operation)"
---
