I've completed the stress-test review and written it to `review.md`. Summary of what I found:

**7 findings** (4 Critical, 3 Important), grouped by theme:

1. **Lifecycle contradiction** — FR-004 says a swap takes effect the instant both staff accept; FR-006 (and US-3) say it only takes effect after manager approval. The stories contradict each other on this too, and the state model is missing a "withdrawn" state that US-1 requires.
2. **Eligibility gap** — the spec assumes "any colleague is eligible for any shift offered in a swap," directly contradicting the ratified role-flags decision (0003) and roles-and-eligibility.md that the rota builder already enforces (age/alcohol sign-off, till training, duty-manager flag).
3. **Scope/dedup miss** — the staged FEAT-006 entry includes an "offer to everyone, first claim" capability that duplicates the existing FEAT-005 (Open shifts) almost verbatim, was left out of the derivation's own baseline table, and contradicts the spec's own "out of scope: that is open shifts" line.
4. **Screens & Flows** — the spec is marked UX-bearing but the section is neither a real manifest nor the proper waiver (compared directly against tablet-offline's correct usage of the waiver line).
5. **Notification premise** — "every staff member has push on" contradicts the 2026 device survey and ignores decision 0007's mandated SMS fallback.
6. **Unpaid/smuggled requirements** — FR-008 (dark-mode toggle) has no source at all; FR-009 bakes in implementation detail (retry count, queue mechanism) beyond what US-2 asks for.
7. **Success criteria** — SC-002 isn't verified by any feature nor marked deferred; SC-002/SC-003 aren't measurable as worded.

Each finding has a matching clarification question with concrete options and impact, plus a recommended (advisory, non-binding) verdict of `needs-revision`.