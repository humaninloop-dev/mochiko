---
report: disclosure
feature: FEAT-031
round: 1
seats: [technical-analyst, qa-engineer]
ladder:
  - {element: WaitlistEntry, rung: "1 required", claim: "FR-001; the queue is the feature"}
  - {element: Offer, rung: "1 required", claim: "FR-003, FR-004"}
  - {element: Notification, rung: "1 required", claim: "FR-007 asks for a record of every notice sent"}
  - {element: Booking.offerId, rung: "3 already exists", claim: "extends the delivered Booking; no new entity"}
  - {element: "GET /berths/{berthId}/availability-forecast", rung: "4 minimum now", claim: "marina ops asked for it at kickoff; 90-day window is the minimum useful"}
  - {element: D-004 offer_locks table, rung: "2 simpler shape", claim: "a table and a poll is simpler than a queue"}
  - {element: D-005 Redis TTL expiry, rung: "3 already exists", claim: "Redis is already in the stack for sessions; nothing new to run"}
  - {element: "tasks.md Cycle 2 task list", rung: "5 builder's room", claim: "file paths listed to save the builder the lookup"}
adopt_first:
  - {decision: D-004, category: locking, shelf_named: no, note: "small enough to own"}
  - {decision: D-005, category: scheduling, shelf_named: "Redis TTL", custom_beats_shelf: "n/a — shelf chosen"}
---
