---
report: sufficiency
feature: FEAT-031
round: 1
scope: selection
units:
  - {row: FEAT-031/W1, verdict: gaps, gaps: [G-01, G-02, G-03, G-04]}
gaps:
  - {id: G-01, clause: contract-exposure, surface: "join/leave waitlist, accept/decline offer, marina queue view", grade: named-and-unattachable, note: "no seam in contracts/api.yaml; the design phase authors the delta"}
  - {id: G-02, clause: data-exposure, surface: "WaitlistEntry, Offer; Booking touched on accept", grade: named-and-unattachable, note: "new entities; Booking gains the offer link"}
  - {id: G-03, clause: commodity-exposure, surface: "serialising offer issue when two berths free at once (locking); hold-expiry timing (scheduling)", grade: unresolved, note: "adopt-first answerable: D-001 pg-boss delayed jobs and singleton keys exist; the design phase weighs and records"}
  - {id: G-04, clause: nfr-targets, surface: "SC-002 first offer within 60 s of release", grade: absent, note: "no concern row carries an offer-issue latency target; AX-009 has none. The design phase states the target on its row (in the no-delta amendment) so the build has a number to meet"}
store_consult:
  read: [ARCHITECTURE.md index, concerns.md AX summary table]
  trips: []
  no_delta_claim: "No structural change: the waitlist rides the existing api, worker, and db containers; no new container, boundary, or flow."
nfr_targets_applicable: [AX-007/NFR-003, AX-004/NFR-002]
ux_trace: n/a — no Screens & Flows manifest (prototype waived at intent)
delivered_feature_exposure:
  - {feature: FEAT-012, surface: Booking, amendment: "[MODIFY] Booking gains an optional offer link; the amendment is written as the marked delta on FEAT-012's entry"}
in_flight_exposure: none
quickstart: "not applicable — no external integration surface beyond the existing Stripe and Postmark boundaries"
---
