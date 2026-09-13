Review complete — `review.md` written to the working directory.

Summary of the stress-test: 3 Critical and 4 Important findings, grouped and evidenced against `kestrel/`'s decisions, venue-facts docs, and the committed feature map/spec-index baseline.

- **F1 (Critical)** — FR-004 and FR-006 directly contradict each other on *when* an accepted swap changes the published rota (mirrored by a US-1/US-3 conflict).
- **F2 (Critical)** — the spec *assumes* any colleague is eligible for any swapped-in shift, contradicting decision 0003's eligibility rules (age, till training, alcohol sign-off) with no requirement to check it.
- **F3 (Critical)** — the derivation's new FEAT-006 entry duplicates the existing FEAT-005 "Open shifts" capability and contradicts the spec's own stated out-of-scope line, despite derivation.md claiming "nothing to dedup against."
- **F4 (Important)** — the Overview's "everyone has push notifications on" premise is contradicted by `docs/staff-devices.md`, and decision 0007's SMS-fallback/manager-flag path is missing from the FRs, threatening SC-001's target.
- **F5 (Important)** — FR-008 (dark-mode toggle) is unsourced excess scope; FR-009 smuggles a specific implementation mechanism into a MUST requirement.
- **F6 (Important)** — SC-002 isn't homed to any feature or deferred list; SC-003 has no measurable threshold.
- **F7 (Important)** — the staged specs-index row omits the FEAT-001 delta, and that delta doesn't disclose an in-flight status mark the way FEAT-004's entry does.

Each finding includes a concrete failure scenario, and the four blocking/rework-risk items carry decision-style clarifying questions (2–3 options with trade-offs) for the lead to rule on. This is gap-finding input, not a pass/fail verdict — recommendation is to hold before design/prototype work until F1–F3 are resolved.