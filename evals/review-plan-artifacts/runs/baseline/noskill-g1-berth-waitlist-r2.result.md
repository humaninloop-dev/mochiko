I flagged upfront that the named skill (`mochiko:review-plan-artifacts`) isn't installed here, so this review ran on the brief directly rather than that skill's process.

Review written to `review.md`. Headline findings:

- **Blocking:** D-005 (Redis TTL for offer expiry) justifies itself by claiming Redis is "already in the stack for sessions" — false; the baseline puts sessions in a Postgres table. This dodges the blocking one-datastore constraint (C-003) and its required AX-002 sign-off, which is currently recorded as none.
- **Blocking:** US-4/FR-006 (marina staff queue view, a real MUST requirement) has no contract endpoint and no task anywhere in the package, while `GET /berths/{berthId}/availability-forecast` — citing a nonexistent "US-5"/"FR-009" — got a full contract, schema, disclosure entry, and test cycle. Scope substitution, not just an omission.
- **Blocking:** `Offer.status` has no `lapsed` state, so FR-005's lapse behavior and SC-001's accept-vs-decline metric can't actually be represented in the schema as designed.
- Several significant gaps also noted: undocumented sensitivity handling for `WaitlistEntry.contactEmail`, no stated trigger for `issueOffer`, the SC-002 latency target still unresolved from the sufficiency report, and an unclear money/Payment-row path on offer accept.
- Per the requirements analyst's ask, spec-level gaps are called out separately at the bottom (no umbrella FR for notifications, "berth frees" left operationally undefined, the open `[NEEDS CLARIFICATION]` on marketing consent).