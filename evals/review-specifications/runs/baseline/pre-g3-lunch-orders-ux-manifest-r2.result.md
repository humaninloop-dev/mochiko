I reviewed the `lunch-orders` spec workspace (spec, stories, derivation, and the prototype walked directly by opening each HTML file) against the operating manual, school-facts docs, the committed feature map, and the specs index baseline, and wrote the full findings to `review.md`.

Headline issues (10 critical, most-severe first):

1. **Free-school-meal charging gap** — the spec assumes every family has a card on file, directly contradicting `docs/free-school-meals.md` (59% of pilot pupils get some funded meal); as written, funded families can never complete an order, and it risks the "wallets never go negative" governance floor either way it's forced through.
2. **FR-002 contradicts the in-flight menu-admin feature** — it requires showing parents items the kitchen has hidden, reversing the safety rule menu-admin just shipped for recalls.
3. **Illegal map delta** — `derivation.md` regresses FEAT-003 (Wallet) from `delivered` to `proposed`, which the map grammar forbids; the existing precedent (FEAT-002) uses a work row instead.
4. **Untraced scope** — FR-009 (meal rating) has `Source: —`, no story supports it, and it's already leaked into the FEAT-005 extent.
5. **Broken prototype** — `scr-004-order-history.html` and a "confirmation" screen referenced by the Pay button don't exist; every nav bar and two of three flows dead-end.
6–7. **Zero flow coverage** for US-2 (P1 change/cancel) and for the insufficient-wallet scenario (US-1 scenario 3), which SCR-003 actively contradicts (shows a live Pay button when wallet < total).
8. **FLOW-003 cites a nonexistent scenario** ("US-3 / scenario 3" — should be US-4).
9. **Cut-off contradiction** — US-1 scenario 2 says "before Sunday midnight," but the ratified decision and FR-005 both fix it at Thursday 17:00.
10. **Manifest drift** — SCR-005 (Allergen key) exists as a built page with no row in the Screens table.

Plus 8 important/minor findings (unverified Google Sheet constraint baked into FR-006, an unverified regulatory claim in FR-007, oversized FEAT-005 extent, missing FEAT tags on all screen/flow rows, an unmeasurable SC-002, a self-contradicting caching note, leftover placeholder text in the prototype, and no stated behavior for two parents on a shared account).

Recommended verdict: **not ready to build** — revision needed before this leaves stress-test.