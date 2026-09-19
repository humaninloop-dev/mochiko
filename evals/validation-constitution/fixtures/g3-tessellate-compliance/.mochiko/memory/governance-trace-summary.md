# Trace summary — governance surface set v1.0.0 (2026-09-09)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–016).
The validator's grading surface.

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile (gdpr · pci-dss · a11y attached) | fact profile | region ratified stamp | ledger header ✓ |
| GI-002 | Identity/type (fullstack web; seven people) | identity | region Technology stack | ledger amendment-policy approvers ✓ |
| GI-003 | Depth level: high | depth declaration | region ratified stamp | ledger header ✓ |
| GI-004 | Security by Default | floor-asserted: FLOOR-SEC | region principle line | index=line ✓ · ledger ✓ · rules/mochiko/security.md ✓ · quality-gates line ✓ |
| GI-005 | Testing Discipline | floor-asserted: FLOOR-TEST | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-006 | Error Handling | floor-asserted: FLOOR-ERR | region principle line | index=line ✓ · ledger ✓ |
| GI-007 | Observability | floor-asserted: FLOOR-OBS | region principle line | index=line ✓ · ledger ✓ |
| GI-008 | Personal-Data Obligations | module: gdpr | region principle line | index=line ✓ · ledger ✓ |
| GI-009 | Cardholder Data | module: pci-dss | rules/mochiko/cardholder-data.md | index ✓ · ledger ✓ |
| GI-010 | Accessibility | module: a11y | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-011 | Fast Pages | minted | region principle line | index=line ✓ · ledger ✓ |
| GI-012 | pci-dss ASV waiver | waiver | ledger waiver table | synthesis ✓ |
| GI-013 | Release gates | module: release-gates | region summary line + ledger detail | index ✓ · ledger ✓ |
| GI-014 | Knowledge management (core + CHANGELOG) | module: knowledge-management | `.mochiko/memory/knowledge-management.md` | region operating-docs line ✓ · rules/mochiko/operating-docs.md ✓ · `CHANGELOG.md` ✓ · ledger ✓ |
| GI-015 | layer-rules declined | module ruling | synthesis only | — |
| GI-016 | British English | minted | region principle line | index=line ✓ · ledger ✓ |

Every surface element traces back: region lines → GI-001/002/003/004/005/006/007/008/009/010/011/013/014/016 ·
ledger entries → GI-004–011, 013, 014, 016 · `rules/mochiko/security.md` → GI-004 ·
`rules/mochiko/cardholder-data.md` → GI-009 · `rules/mochiko/operating-docs.md`, the KM pin, and
`CHANGELOG.md` → GI-014 · output-style rules file → template-mandated (Shape 5, routed from no principle).

**Flagged proposals:** none

**Waivers:** none
