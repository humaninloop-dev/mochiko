I reviewed the `shift-swaps` spec workspace against the product's governance, decisions, venue facts, and the feature map as committed at run-open, and wrote the full report to `review.md`.

Headline findings (20 total, most-severe first):

- **FR-004 directly contradicts FR-006** on whether an accepted swap changes the published rota immediately or only after manager approval — the spec's own acceptance scenarios disagree with each other on this.
- **The staged FEAT-006 map entry duplicates FEAT-005 (Open shifts)** and contradicts the spec's own out-of-scope line — traced to derivation.md's baseline table silently omitting FEAT-005 from the dedup check.
- **SC-002 is unhomed** to any feature despite "Deferred SCs: none," and the **specs-index draft row omits the FEAT-001 delta** entirely.
- **No prototype exists** despite Intent ruling the spec UX-bearing — the Screens & Flows section is a placeholder, not a manifest or a valid waiver.
- **The eligibility assumption** ("any colleague can take any shift") overrides the ratified role-eligibility rules (decision 0003) with no enforcing requirement — a swap could legally place an unqualified/underage person on a bar shift.
- **The Overview's "everyone has push notifications" claim is contradicted by the staff-devices survey** (~36% actual push reach), and no requirement wires in decision 0007's SMS fallback for swap notifications.
- Plus missing withdrawal requirement, an untraceable dark-mode FR, incomplete swap-request states, and several Important/Minor gaps (manager-notification timing vs. decision 0007, unbounded manager-approval wait, unexplained CSV-on-shared-drive constraint, unmeasurable success criteria).

The report includes the full feature-layer and Screens & Flows check tables plus 7 stakeholder-facing clarification questions with concrete options. No input files were modified.