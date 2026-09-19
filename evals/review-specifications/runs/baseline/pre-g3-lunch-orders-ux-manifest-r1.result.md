I've written the stress-test report to `review.md` at the workspace root (sibling to `tally/`), without modifying any input files.

Headline findings, most severe first:

1. **FR-002 directly contradicts the in-flight menu-admin feature** — it requires showing hidden menu items to parents, the exact opposite of the accepted "hidden item MUST NOT appear in any parent-facing screen" contract. Root cause: `derivation.md`'s baseline table misread FEAT-002 as `delivered` when the committed map already had it `in-flight`.
2. **Free-school-meal pupils (59% at the pilot schools per `docs/free-school-meals.md`) are entirely unhandled** — the spec assumes every family has a card on file, contradicting product docs and the constitution's own funded-meal carve-out.
3. **The prototype's flagship flow dead-ends** — no confirmation screen exists (linked but never built), SCR-004 (order history) is linked everywhere but the file is missing, and an orphan allergen-key page exists with no manifest row.
4. **US-2 (P1, change/cancel)** has zero screens or flows, and **US-1's insufficient-wallet scenario** is demonstrated by the mock's own numbers (£9.70 owed, £9.10 balance) but not handled.
5. Several smaller issues: a wrong story citation on FLOW-003, an unsourced "rate a meal" requirement, a wallet status regression that breaks the map's own precedent, and a contradictory "Sunday midnight" edit deadline in US-1 vs. the ratified Thursday 17:00 cut-off.

Full detail, severity tags, clarifying questions with options, and the feature/screens-flows check tables are in `review.md`. Recommended verdict: **not ready to build**.