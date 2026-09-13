Review complete — written to `review.md` in the current directory, no inputs modified.

Top-line: this spec is **needs-revision**, not because it lacks ambition but because it contains real self-contradictions and a compliance gap:

- **FR-004 directly contradicts FR-006** on when a swap actually changes the published rota — two MUSTs disagreeing on the core state machine.
- **Screens & Flows is a placeholder** ("Figma link to follow") despite Intent declaring the feature UX-bearing, with three named screens.
- **No eligibility check on swap acceptance** — the Assumptions section says any colleague can take any shift, which conflicts with the existing role-flag rules (bar needs 18+/sign-off, box office needs till training) that the rota builder already enforces elsewhere.
- **FR-005's "eleven hours' rest... as UK law requires"** looks wrong for under-18 staff (UK young workers get 12 hours), and this product explicitly serves teenage venue staff.
- **The staged FEAT-006 derivation duplicates FEAT-005 (Open Shifts')** first-claim mechanic in a clause no story supports, and the derivation's own "map at baseline" omits FEAT-005 entirely even though it predates the run.
- Several other contradictions and gaps: swap history is specified as both a CSV-on-shared-drive (Intent) and an audit-table-backed in-app view (FR-007/US-4); SC-002 is orphaned from the feature map; the Overview's "everyone has push notifications on" claim contradicts the venue's own device survey.

Full detail, severity ranking, and concrete stakeholder-facing options are in `review.md`.