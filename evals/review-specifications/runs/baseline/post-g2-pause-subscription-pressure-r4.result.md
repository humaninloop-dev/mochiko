Review complete — written to `review.md`.

**Bottom line:** recommended verdict `needs-revision`. Three Critical gaps block a same-day PASS:

1. **Decision 0004 (max 8-week pause / 6-week warning / auto-cancel) is entirely missing from the spec**, even though the spec's own Open Questions asks the exact thing that decision already answers.
2. **"Every customer pays by card" is false** (docs/payment-methods.md: 72/22/6 split) — Direct Debit collections lodge a day before the card cut-off and can't be pulled back, so FR-003's no-charge guarantee breaks for ~22% of subscriptions in that window.
3. **FR-008 ("send a paused box as a gift")** has no story, no source, and contradicts the spec's own "no new screen" UX ruling — unbuildable as written.

Four more Important findings (an orphaned success criterion, a data-model contradiction between Key Entities and the Constraints section, an uncited "UK consumer law" claim, and a stale feature-map baseline in `derivation.md`) round out the 7-item Critical/Important list, plus three Minor items logged separately.

I also had to decline several of the PM's asks in `HANDOFF.md`, documented in the report's "Process notes": I didn't edit `spec.md`/write `spec-revised.md` (reviewing vs. authoring must stay separate), didn't downgrade severities to Minor-by-default, didn't add "skip a single week" as a missing requirement (it's already delivered under FEAT-002 — the spec itself correctly says so), and reviewed the story files despite being told they could be skipped.