I've completed the stress-test review and written it to `review.md`. Summary of what I found:

**5 Critical findings:**
1. **FR-004 vs FR-006 contradict** on whether an accepted swap changes the rota immediately or only after manager approval — and the story acceptance criteria (US-1 vs US-3) split the same way.
2. The spec **assumes any colleague is eligible for any swapped shift**, bypassing the existing role-eligibility (decision 0003), availability, and time-off checks the rota builder already enforces — plus an uncited "11-hour rest" legal claim.
3. The proposed **FEAT-006 entry invents an "offer to everyone, first-claim" mechanic** that duplicates the existing FEAT-005 (Open shifts) — and derivation.md's baseline map table omits FEAT-005 entirely, so the dedup step never caught it.
4. **SC-002 is orphaned** (verified by no feature) and **SC-003 ("intuitive") is unmeasurable**.
5. Intent declares the spec **UX-bearing: yes**, but Screens & Flows has neither a manifest+prototype nor the legal waiver line — just "Figma link to follow."

**2 Important findings:** the Overview's "everyone has push notifications on" claim contradicts the team's own staff-devices survey and decision 0007's SMS-fallback rule; and a cluster of unsourced/smuggled items (unsourced dark-mode FR, a CSV-on-shared-drive history constraint that bypasses the existing audit table, and a specs-index draft that drops the FEAT-001 delta).

Recommended verdict: **needs-revision**. The report includes six clarifying questions with concrete options for the lead to rule on.