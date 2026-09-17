I've written the stress-test review to `review.md`. Summary of what it found:

**Verdict: critical-gaps** — six Critical findings, not just polish:

1. **FR-004 vs FR-006 contradict each other** on the feature's core mechanic — does the rota change the moment both staff accept (matching US-1's own acceptance scenario), or only after manager approval (matching US-3)? Two different products are described.
2. **Eligibility silently dropped** — the spec assumes any colleague can take any swapped shift, but the product already enforces role eligibility (till training, alcohol sign-off, age, duty-manager) elsewhere and refuses ineligible placements with a reason. A swap is left as an unguarded bypass of that licensing control.
3. **Derivation dishonesty / dedup failure** — the staged FEAT-006 entry includes an "offer to everyone, first to claim" mechanic that's verbatim FEAT-005 (Open Shifts, already `proposed` on the map), unsupported by any of the four stories, and directly contradicted by the spec's own out-of-scope line. The derivation's baseline table also omits FEAT-005 entirely.
4. **Screens & Flows section is a placeholder** ("Figma link to follow") despite Intent ruling the spec UX-bearing with three named screens — no prototype exists to walk.
5. **SC-002 is orphaned** — not mapped to any verifying feature, not listed as deferred either.
6. **Specs-index draft omits FEAT-001** even though the spec stages a delta on it.

Plus 12 Important/Minor findings (a missing state for manager refusal, an unrealized CSV-history constraint that conflicts with the Confidential-data principle, an unfounded dark-mode FR, an undisclosed external legal claim, a notification-reach contradiction against the venue-facts survey, and more), each with a proposed fix. Seven clarifying questions with concrete options are included for the lead to render at the gate.