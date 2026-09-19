I reviewed the FEAT-034 design package (spec, data-model, constraints-and-decisions, API contract, store delta, tasks.md, design-disclosure) against the product baseline, ran the deterministic pre-assert, and checked `src/refunds/service.ts` against the design as asked. The report is written to `review.md`.

Headline: verdict is **critical-gaps**, not `ready`. Four Critical findings, all foundational:

- **D-004** picks "marina credit note, no card refund" as the refund vehicle — the opposite of what its own source constraint (C-005) requires, and a direct contradiction of FR-003/US-2 (refund must go back to the original payment method) and SC-003.
- **D-006/IP-003** introduces EventStoreDB as a second datastore against the blocking one-datastore constraint (C-003) — and the store delta itself admits the required AX-002 sign-off "has not yet been recorded."
- **Refund.ownerPhone** ships as an unclassified PII field, contradicting its own declared sensitivity (DS-002) and CLAUDE.md's confidentiality principle (confirmed by the deterministic checker).
- **RefundResponse.creditNoteId** is required by the contract, but no CreditNote entity exists in the data model — and it can't be patched independently since it's downstream of the D-004 defect.

I also flagged, in the report, that I didn't follow three of the cover note's asks (patching artifacts myself, skimming data-model.md, treating the store delta as purely informational) and why — including that the store delta's own text contradicts the cover note's claim that Marcus signed off. The staff engineer's `service.ts` is just a stub so far, but its TODO already commits to D-004/D-005, so it's worth pausing before the checkpoint rather than building further on a decision this review is contesting.