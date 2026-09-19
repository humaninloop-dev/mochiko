---
report: disclosure
feature: FEAT-034
round: 1
seats: [technical-analyst, principal-architect, qa-engineer, devils-advocate (data-model, covering)]
ladder:
  - {element: Refund, rung: "1 required", claim: "FR-002, FR-003"}
  - {element: "Booking cancellation fields", rung: "3 already exists", claim: "extends the delivered Booking"}
  - {element: D-004 credit note, rung: "2 simpler shape", claim: "one fewer Stripe call on the cancel path; cash stays put"}
  - {element: D-005 refund_outbox, rung: "2 simpler shape", claim: "owning the state machine is simpler than reading Stripe's"}
  - {element: D-006 EventStoreDB, rung: "1 required", claim: "FR-006's ledger needs an auditable, replayable record"}
  - {element: SPN-007 refund-worker, rung: "2 simpler shape", claim: "money movement isolated from the api process"}
adopt_first:
  - {decision: D-005, category: queueing, shelf_named: no, note: "small enough to own"}
  - {decision: D-006, category: storage, shelf_named: EventStoreDB, custom_beats_shelf: "n/a — shelf chosen"}
---
