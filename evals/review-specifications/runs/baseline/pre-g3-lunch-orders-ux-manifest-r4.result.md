Review written to `review.md`. Summary of what I found walking `tally/.mochiko/specs/lunch-orders/`:

**Verdict: critical-gaps** — six critical findings, none of them cosmetic:

1. **Funded (free-school-meal) children can't be ordered for** — the spec assumes every family pays by wallet, contradicting `docs/free-school-meals.md` and the already-delivered Wallet feature's own zero-cost funded-meal support.
2. **Direct contradiction with the in-flight menu-admin spec** — this spec requires showing hidden menu items to parents; menu-admin requires hidden items never reach a parent-facing response. The derivation's own baseline table misrecorded FEAT-002 as "delivered" instead of "in-flight," which is likely how the conflict was missed.
3. **US-2 (P1, change/cancel an order) has zero screens or flows** anywhere in the manifest or prototype.
4. **Clicking the prototype surfaces a real dead end**: the mock's own numbers put the wallet below the order total (exactly the scenario US-1 requires a top-up prompt for), but the "Pay" button leads to a confirmation screen that doesn't exist and no shortfall warning is shown.
5. **SCR-004 (Order history)** is linked from every single page's nav bar and the spec's own flow, but the file doesn't exist — broken sitewide.
6. **The Wallet feature's map delta illegally regresses `delivered` → `proposed`**, contradicting the map's own established pattern (FEAT-002 went to `in-flight`, staying sticky, when it picked up new work).

Also flagged: a mistraced flow citing a nonexistent story scenario, an unsourced "rate your meal" requirement smuggled into scope, a cut-off date that contradicts the ratified decision doc, two unmeasurable success criteria, missing FEAT-tags on the screens/flows table, an unverifiable regulatory citation (no web access this session to check it), and a placeholder-text screen. Five clarifying questions with concrete options are included for the lead to put to the stakeholder.