I've written the review to `review.md`. Summary for you:

**Verdict: critical-gaps** (4 Critical, 3 Important findings, 7 total — within the 5–7 gap bound).

The core problems:
1. **FR-004 vs FR-006 directly contradict** each other on when an accepted swap actually hits the published rota (immediately on acceptance vs. only after manager approval).
2. **No eligibility check on swaps** — the spec assumes "any colleague is eligible for any shift," which conflicts with decision 0003 and the role-flag rules the rota builder already enforces (till training, 18+/alcohol sign-off, duty-manager flag).
3. **Feature-layer derivation failure** — `derivation.md`'s baseline read skips FEAT-005 (Open shifts) entirely, then FEAT-006's proposed extent bakes in the exact broadcast-claim mechanic the spec's own Intent rules out of scope. SC-002 is also orphaned (homed nowhere, not deferred), and the specs-index draft line omits the FEAT-001 delta.
4. **UX-bearing: yes declared, but no manifest and no valid waiver** — Screens & Flows just says "Figma link to follow."
5. Notification-channel gaps against decision 0007 and the staff-devices survey (Important).
6. An orphaned CSV swap-history constraint with a possible confidentiality risk (Important).
7. A cluster of smaller ambiguities/gaps: unquantified expiry, undisclosed external legal claim, unmeasurable SC-003, unsourced dark-mode FR, a missing "manager-refused" state (Important).

Seven clarifying questions (C1–C7) are included with concrete options and impact framing for the lead to take to the stakeholder. I noted one limitation transparently in the report: WebSearch wasn't available in this environment, so I couldn't complete a live disconfirming-source check on the UK rest-period claim (FR-005) — the undisclosed-claim finding stands regardless, per the external-claims rule.