# Strip notes — `templates/constitution-modules/release-gates.md`

Entry formats: `strips/README.md`.

<!-- Wave context: wave 1 of the setup-product-agnostic build (v0.115.0). Ruling for the
[v0.115.0] entry below: `.mochiko/brainstorms/setup-product-agnostic/record.md` D1–D6 as
review-amended (`DECISIONS.md` 2026-09-24 "Setup goes product-agnostic ruled" row). Pre-edit
verbatim text: `git show bbe303f:plugins/mochiko/templates/constitution-modules/release-gates.md`. -->

## [v0.115.0] the fragment's compliance-module consistency check

- **Disposition:** superseded → deleted. The check's only subject leaves with the modules, and the
  lead ruled out a retargeted check as a new obligation.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/setup-product-agnostic/record.md`
  D1, D6, and the review fold S9; `DECISIONS.md` 2026-09-24)
- **Content:** verbatim — "- [ ] Gates consistent with the attached compliance modules (an attached
  module names its audit-evidence gate)"
- **Kept deliberately:** the fragment's other three checks — environments and cadence with real
  names, a release-gate table with concrete verification, and a rollback procedure with a time
  expectation — and the module's header comment.
- **Consumers assessed:** `validation-constitution/references/QUALITY-CHECKLIST.md` runs this
  fragment when the module is selected. Its "`release-gates` fragment" line is unchanged.
