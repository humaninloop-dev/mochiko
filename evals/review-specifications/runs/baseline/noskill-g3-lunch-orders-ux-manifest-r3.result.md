I've written the stress-test review to `review.md` in the workspace root (the `mochiko:review-specifications` skill wasn't actually available, so I did the review directly per your brief).

Headline findings, most severe first:

- **C1** — The one hard safety rule (blocking allergen-containing items from being ordered for a child with that allergy) exists in the story/prototype/derivation but has **no corresponding Functional Requirement** in `spec.md`. FR-007 only requires showing allergen info, not refusing unsafe selections.
- **C2** — FR-002 requires showing parents *hidden* menu items, directly contradicting the already-accepted, in-flight `menu-admin` spec's FR-002 ("a hidden item MUST NOT appear in any parent-facing response").
- **C3** — The spec assumes every family has a card on file and charges unconditionally at order time, contradicted by `docs/free-school-meals.md` (~59% of pupils are free-meal funded, many with no card/balance) — breaking both FR-001 and the "wallets never go negative" governance rule.
- **C4** — US-1's own acceptance scenario says changes are allowed "before Sunday midnight," contradicting the spec's own FR-005 and decision 0003 (cut-off is Thursday 17:00 the week before).
- Several major issues: a flow mistraced to a nonexistent story scenario, a menu architecture that's asserted two incompatible ways (Google Sheet vs. app-native), broken links in the prototype (missing order-history and confirmation screens), and a prototype that stages an insufficient-funds case without showing the required top-up behavior.

Full detail, plus minor nits and what's working well, is in `review.md`.