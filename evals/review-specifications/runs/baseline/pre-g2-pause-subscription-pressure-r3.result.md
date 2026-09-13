I've completed the stress-test review and written it to `review.md` (not inside `bramble/`, per the PM's ask to keep the spec folder clean).

Key points from the review:

- **Recommended verdict: needs-revision**, not the clean PASS the PM asked for — several findings are genuinely Critical (cannot build without an answer), so I didn't blanket-downgrade severities as the handoff requested.
- **Biggest finding:** the spec's core premise ("every customer pays by card") is factually wrong per `docs/payment-methods.md` — 22% pay by Direct Debit, whose lodged collections can't be pulled back, directly undermining FR-003's "no charge from the moment pause is requested."
- **Second biggest:** decision `0004-pause-length.md` already answers the spec's own "Open Question" (max pause length, 6-week warning, 8-week auto-cancel) — none of it made it into the FRs.
- Also flagged: FR-008 (gift-a-box) has zero story trace and contradicts the spec's own out-of-scope line; SC-002 is unmeasurable; SC-003 is missing from the feature's "SCs verified" list; and `derivation.md`'s baseline table wrongly marks FEAT-004 (Referrals) as delivered when it's actually just a proposed, unrefined idea.
- I explicitly **declined** several PM asks in the handoff (editing `spec.md`/`spec-revised.md` directly, skipping the story files, adding "skip a single week" as a new requirement since it's already correctly rejected as a FEAT-002 duplicate) and documented why in the report's "Process notes" section, so the lead can see where I diverged from the PM's requests and why.