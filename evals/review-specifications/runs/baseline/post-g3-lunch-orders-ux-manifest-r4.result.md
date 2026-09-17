Report written to `review.md`. Summary of the stress test on `tally/.mochiko/specs/lunch-orders/`:

**Verdict: critical-gaps** — 7 findings (4 Critical, 3 Important), each paired with a stakeholder clarification.

The four Critical issues are foundational, not polish:
1. **Free-school-meal charging is entirely unaddressed** — the spec's own "every family has a card on file" assumption is contradicted by `docs/free-school-meals.md`, and the governance floor's funded/no-charge path has no implementation. This blocks ~59% of pilot-school pupils.
2. **FR-002 contradicts the in-flight FEAT-002/menu-admin work** — it requires showing parents hidden items, while menu-admin is being built right now to guarantee the opposite. The derivation also misread FEAT-002's baseline status (listed `delivered`, actually `in-flight`).
3. **The FEAT-003 Wallet delta illegally regresses a `delivered` capability to `proposed`** for a change that only touches charge timing.
4. **The Screens & Flows manifest and prototype have real breaks**: dead links to a nonexistent order-history screen and a nonexistent payment-confirmation screen (whose own demo data — wallet £9.10 vs. a £9.70 order — exercises the exact insufficient-funds case it doesn't handle), an orphan allergen-key screen, and a flow mis-cited to the wrong story.

Three Important gaps cover an untraceable "rate a meal" requirement, several unmeasurable/mis-homed success criteria plus an unverified regulatory claim, and a missing order-status field for the "held" order state.