# Trace summary — Ledgerline governance surface set v1.0.0

One row per principle-bearing GI element (floor-asserted / deck-kept / minted / compliance-module
obligation). Non-principle elements (GI-001 fact profile, GI-002 identity, GI-016/017/018 module
selections, GI-019 waiver, GI-020 domain seed, GI-021–024 exclusions/deferrals) carry no principle
row — their content routes to the fact profile, module sections, waiver table, registry block, and
exclusion records respectively.

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | Security by Default | floor-asserted: FLOOR-SEC | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-004 | Testing Discipline | floor-asserted: FLOOR-TEST | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-005 | Error Handling Standards | floor-asserted: FLOOR-ERR | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-006 | Observability Requirements | floor-asserted: FLOOR-OBS | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-007 | Layer Boundaries | deck-kept: BE-HEX | rules/mochiko/layer-boundaries.md | index ✓ · ledger ✓ |
| GI-008 | Complexity Limit | deck-kept: BE-SRP | CLAUDE.md quality-gate line | index ✓ · ledger ✓ |
| GI-009 | Dependency Discipline | deck-kept: BE-DEP | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-010 | CI Is the Gate | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-011 | Accessibility Baseline | module: a11y-wcag-baseline | rules/mochiko/accessibility.md | index ✓ · ledger ✓ |
| GI-012 | Data Durability | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-013 | Financial Audit Trail | minted | rules/mochiko/financial-audit.md | index ✓ · ledger ✓ |
| GI-014 | Tenant Isolation | minted | rules/mochiko/tenant-isolation.md | index ✓ · ledger ✓ |
| GI-015 | Email Authentication & Link Integrity | minted | CLAUDE.md region line | index ✓ · ledger ✓ |

Flagged proposals: none.
Waivers: GI-019 (FLOOR-TEST numeric coverage gate).
Modules attached: knowledge-management (GI-016, + electives CHANGELOG, RUNBOOK) · layer-rules
(GI-017) · release-gates (GI-018) · a11y compliance (GI-001 → obligation GI-011).
