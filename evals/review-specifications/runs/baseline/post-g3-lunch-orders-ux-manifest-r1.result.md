I've completed the stress-test review and written it to `review.md` in the working directory. No other files were modified.

Summary of what the review found:

**7 Critical/Important findings** (within the 5–7 gap bound), the headline ones being:
- **F1**: `lunch-orders` FR-002 ("parents see hidden items too") directly contradicts the in-flight, accepted `menu-admin` FR-002 ("hidden item MUST NOT appear in any parent-facing response") for the same capability (FEAT-002) — traced back to the derivation reading FEAT-002's baseline status as `delivered` when the committed map says `in-flight`.
- **F2**: The cut-off time contradicts itself — US-1's own acceptance scenario says "before Sunday midnight," while FR-005 and the ratified decision 0003 say "17:00 Thursday the week before."
- **F3**: The prototype doesn't actually walk — `scr-004-order-history.html` and `scr-006-confirmation.html` are linked everywhere but don't exist on disk, and `scr-005-allergens.html` exists but is an unreferenced orphan with no manifest row.
- **F4**: FLOW-003 cites a nonexistent "US-3 / scenario 3" for the allergy flow (should be US-4).
- **F5**: Three of five P1 acceptance scenarios (both of US-2, plus US-1's insufficient-wallet scenario) have zero screen/flow coverage.
- **F6**: The FEAT-003 wallet delta regresses a `delivered` feature wholesale to `proposed` instead of an in-flight mark, per feature-map delta legality.
- **F7**: FEAT-005's extent asserts capabilities (meal rating, multi-child single-checkout) that no story or FR in the spec actually backs.

Plus 4 minor/logged items (undisclosed external claim on FR-007, an unmeasurable SC-002, missing FEAT-tags on Screens & Flows rows, a possibly-already-answered open question). The report closes with clarifying questions framed as concrete stakeholder decisions, a recommended (non-binding) `needs-revision` verdict, and one strength (the allergy-safety mechanic is well-modelled end-to-end once F4 is fixed).