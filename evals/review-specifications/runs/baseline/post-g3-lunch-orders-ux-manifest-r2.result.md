I've completed the stress-test review and written it to `review.md` in the working directory (not inside `tally/`), touching no other files.

Read-back confirmed at the start: 8 floor rules (`author-grader`, `no-scope-creep`, `map-git-baseline`, `check-existing-first`, `density-never-gap`, `evidence-floor`, `gap-bound`, `input-not-verdict`).

**7 findings** (within the 5–7 gap-bound), 4 Critical / 3 Important:

1. **F1 (Critical)** — Free-school-meal orders are never addressed; the spec assumes every family pays by card, contradicting `docs/free-school-meals.md` (up to ~41%+18% of pupils funded) and FEAT-003's own zero-charge behavior.
2. **F2 (Critical)** — FR-002 requires showing hidden menu items to parents, directly contradicting the committed FEAT-002 capability and the in-flight menu-admin spec's own FR-002.
3. **F3 (Critical)** — US-2 (P1, change/cancel order) has zero screen/flow coverage; the prototype's own primary flow dead-ends (Pay button links to a nonexistent confirmation page; Order History is linked from every page but was never built).
4. **F4 (Critical)** — No handling for insufficient wallet balance at checkout (US-1 scenario 3), and the prototype's own sample data (£9.10 wallet vs £9.70 order) reproduces this unhandled, violating the "wallets never go negative" governance principle.
5. **F5 (Important)** — FEAT-003 illegally regressed `delivered → proposed`; lunch-orders itself is missing from `specs/index.md`.
6. **F6 (Important)** — No failure-path requirement for the Sheets write-back that SC-003 depends on.
7. **F7 (Important)** — Unmeasurable SC-002/FR-010, untraced FR-009 (meal rating), and FLOW-003 mis-cited to a nonexistent story scenario.

The report also includes stakeholder-facing clarifying questions, the feature-layer/Screens-and-Flows check summary, minor logged items, strengths, and a recommended next step (not a clearing verdict, per this seat's scope).