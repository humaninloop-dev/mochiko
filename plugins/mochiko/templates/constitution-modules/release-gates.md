<!--
MODULE: release-gates
=====================
Attach when: always offered for the target class — customer-facing software the team deploys
and operates has a real release process by definition (PO-D1), and the deployment-and-release
dimension (always interrogated — no pruning license) supplies its content: environments,
cadence, release-blocking criteria, rollback expectations. Trace: the GI module-selection
element that names `release-gates`.
-->

## Release Gates

<!--
INSTRUCTION: What blocks a release — beyond the per-merge Quality Gates. Source from the
synthesis's deployment-reality element. Use the project's actual environment names and real
verification commands.
-->

**Environments:** [e.g. dev → staging → production, with promotion rules]
**Cadence:** [e.g. on-merge continuous / weekly cut / manual]

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| [e.g. Staging soak] | [e.g. 24h error rate < baseline] | [dashboard/command] | promotion to production |
| [e.g. Migration check] | [e.g. reversible migration verified] | [command] | deploy |
| [e.g. Changelog] | [entry present for user-facing change] | PR check | release cut |

### Rollback

- Rollback procedure MUST be documented and executable by [role]: [pointer or inline steps]
- [Rollback time expectation, e.g. "restore previous version in ≤15 minutes"]
- Releases that cannot be rolled back (e.g. destructive migrations) MUST be flagged in the PR and
  approved explicitly

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Environments and cadence stated with the project's real environment names
- [ ] Release-gate table present; every gate has a concrete verification (command/dashboard), no placeholders
- [ ] Rollback procedure documented with a time expectation
- [ ] Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)
-->
