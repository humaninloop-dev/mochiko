---
report: disclosure
feature: FEAT-027
round: 2
seats: [technical-analyst, principal-architect, qa-engineer]
ladder:
  - {element: Meter, rung: "1 required", claim: "FR-001"}
  - {element: MeterReading, rung: "1 required", claim: "FR-001, FR-002, FR-003"}
  - {element: Tariff, rung: "1 required", claim: "FR-003, FR-004"}
  - {element: SPN-007 meter-ingest, rung: "2 simpler shape", claim: "one listener beats retrofitting burst handling into api"}
  - {element: D-004 hourly pull, rung: "3 already exists", claim: "the worker already owns scheduled work"}
  - {element: D-005 ring-buffer tables, rung: "2 simpler shape", claim: "twelve plain tables beat a partitioning extension nobody here has run"}
  - {element: "GET /marinas/{marinaId}/consumption-export", rung: "4 minimum now", claim: "finance asked for a CSV; one month at a time is the minimum useful"}
  - {element: "tasks.md Cycle 2 storage layer", rung: "5 builder's room", claim: "gives the builder the tables before the views"}
adopt_first:
  - {decision: D-005, category: storage, shelf_named: no, note: "plain tables; nothing to adopt"}
round_2_changes: "contracts: pagination on consumption, Reading example, consumption export added; constraints: D-004 rationale rewritten, D-005 added; tasks: Cycle 5 added for US-3"
---
