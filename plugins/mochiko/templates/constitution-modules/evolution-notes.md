<!--
MODULE: evolution-notes
=======================
Attach when: mode is brownfield (always) — the constitution documents its brownfield context and
the Essential-Floor gap status so future maintainers can tell codified-existing-capability from
aspirational MUST-implement targets. Trace: the GI module-selection element that names
`evolution-notes`.
-->

## Evolution Notes

This constitution was created from brownfield analysis (`.mochiko/memory/codebase-analysis.md`,
[DATE]).

**Essential Floor Status** (assessed against the codebase; waived categories carry their waiver
record in the Governance Tier section, not a gap):

| Category | Status | Response |
|----------|--------|----------|
| Security | [present / partial / absent] | [codified existing pattern / MUST-implement → GAP-XXX / waived (see Waivers)] |
| Testing | [status] | [response] |
| Error Handling | [status] | [response] |
| Observability | [status] | [response] |

**Confrontations resolved in session:** [detected-reality-vs-declared-intent conflicts and their
rulings, from the synthesis — e.g. "declared production tier with no tests: ruled MUST-implement,
GAP-002." Or "none."]

See `.mochiko/memory/evolution-roadmap.md` for the improvement plan.
<!-- Roadmap stub (moved-to-other-cluster): producing evolution-roadmap.md is the roadmap
cluster's job, not ported yet. Write the gap-status table now; the linked roadmap is filled in
when that cluster lands. -->

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Essential Floor status table present, all four categories assessed
- [ ] Every "absent"/"partial" category has a response: codified pattern, GAP reference, or a waiver record in Governance Tier
- [ ] Statuses match codebase-analysis.md (cross-check)
- [ ] Technology stack matches codebase analysis
- [ ] Quality gates reflect current + target state
- [ ] Session confrontations recorded (or "none")
-->
