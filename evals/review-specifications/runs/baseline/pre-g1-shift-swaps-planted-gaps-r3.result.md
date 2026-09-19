I've written the stress-test review to `review.md`. Summary of what it flags:

**Critical (5):**
1. FR-004 vs FR-006 directly contradict each other on when an accepted swap changes the published rota — three of four artifacts (US-1, US-3, derivation.md) agree with FR-006, FR-004 is the outlier.
2. No eligibility check on swaps at all — contradicts the existing role-flag decision (0003) that the rota builder refuses ineligible placements; the spec just assumes it away.
3. derivation.md's baseline table omits FEAT-005 (Open shifts) entirely, then the proposed FEAT-006 entry bakes in an offer-to-everyone/claim mechanic that duplicates FEAT-005 and contradicts this same spec's own "out of scope" line.
4. SC-002 is dropped — not verified by any feature row, not listed as deferred either.
5. Screens & Flows section is empty despite Intent declaring "UX-bearing: yes" for three named screens — no SCR/FLOW rows, no prototype directory, and the placeholder text isn't a valid waiver.

**Important (8)** including: the Overview's "everyone has push on" claim contradicting the staff-devices survey (no SMS fallback per decision 0007), an untraceable dark-mode FR, vague "reasonable time" expiry, unmeasurable success criteria, an implementation-posture constraint (CSV on shared drive) disconnected from US-4, a rejected story missing its rejection note, and a specs-index/feature-selection mismatch.

**Minor (2):** an uncited legal claim (11-hour rest rule) and a few missing symmetric edge cases.

Each finding includes a clarifying question with concrete options and impact for the lead to rule on.