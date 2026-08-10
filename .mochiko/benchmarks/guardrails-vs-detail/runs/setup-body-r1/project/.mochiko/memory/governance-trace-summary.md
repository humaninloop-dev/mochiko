# Trace summary — Ledgerline governance surface set v1.0.0

One row per principle-bearing GI element; presented at the acceptance gate, graded by the
independent validator for two-way trace closure.

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-002 | Technology stack | identity/type | CLAUDE.md region (Technology stack) | index n/a (stack line, not a principle) · ledger n/a (context) |
| GI-003 | Security by Default | floor-asserted: FLOOR-SEC | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-004 | Testing Discipline (critical-path) | floor-asserted: FLOOR-TEST | CLAUDE.md region line | index ✓ · ledger ✓ · waiver GI-014 |
| GI-005 | Error Handling Standards | floor-asserted: FLOOR-ERR | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-006 | Observability Requirements | floor-asserted: FLOOR-OBS | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-007 | Layered (Hexagonal) Architecture | deck-kept: BE-HEX | rules/mochiko/architecture-layers.md | index ✓ · ledger ✓ |
| GI-008 | Single Responsibility & Complexity | deck-kept: BE-SRP | rules/mochiko/architecture-layers.md | index ✓ · ledger ✓ |
| GI-009 | Dependency Discipline | deck-kept: BE-DEP | CLAUDE.md region (quality gate) | index ✓ · ledger ✓ |
| GI-010 | Payment-State Integrity | minted | rules/mochiko/payments.md | index ✓ · ledger ✓ |
| GI-011 | Currency as Integer Cents | minted | rules/mochiko/payments.md | index ✓ · ledger ✓ |
| GI-012 | Tenant Isolation | minted | rules/mochiko/data-access.md | index ✓ · ledger ✓ |
| GI-013 | No Customer Data in Logs/Telemetry | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-016 | Knowledge-Management (operating docs) | module: knowledge-management | CLAUDE.md region (pointer) | index ✓ · ledger ✓ |
| GI-017 | Release Gates | module: release-gates | CLAUDE.md region line + quality gate | index ✓ · ledger ✓ |
| GI-018 | Approved Domain Dependencies | module: layer-rules (registry) | rules/mochiko/domain-dependencies.md | index ✓ · ledger ✓ |
| GI-019 | `zod` domain-dep seed | module: layer-rules (registry) | rules/mochiko/domain-dependencies.md (registry block) | ledger ✓ (folded under GI-018) |
| GI-024 | Data Durability & Recoverability | minted (S5 fold) | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-025 | Accessibility (WCAG 2.1 AA, core flows) | deck-kept: DECK-1 (reclassified) | rules/mochiko/accessibility.md | index ✓ · ledger ✓ |
| GI-026 | Card Data Never Touches Our Servers | minted (S2 fold) | CLAUDE.md region line | index ✓ · ledger ✓ |

## Non-principle GI elements (recorded, not surface principles)

| GI-ID | Element | Where recorded |
|-------|---------|----------------|
| GI-001 | Fact profile (+ negatives, a11y reclass, PCI/SOC2/state-privacy watches) | ledger stamp + Watches; region stamp |
| GI-014 | FLOOR-TEST coverage waiver | ledger Waivers |
| GI-015 | layer-rules module adopted | realized via GI-007/GI-018 |
| GI-016a | CHANGELOG elective adopted | region operating-docs line |
| GI-016b / GI-023 | RUNBOOK elective declined/deferred | ledger Watches (re-offer after first deploy) |
| GI-020 | Exclusion: SLOs / beyond-floor observability | synthesis exclusions (not governed) |
| GI-021 | Exclusion: formal incident response | synthesis exclusions (not governed) |
| GI-022 | Open question: data retention & deletion policy | ledger Watches (flagged, not gated) |

Flagged proposals: none — all authoring stayed within the ratified synthesis.
Waivers: GI-014 (FLOOR-TEST numeric coverage gate).

## Floor coverage check (all four accounted for)

- Security → GI-003 (principle)
- Testing → GI-004 (principle) + GI-014 (waiver of the numeric gate only; category kept)
- Error handling → GI-005 (principle)
- Observability → GI-006 (principle)
