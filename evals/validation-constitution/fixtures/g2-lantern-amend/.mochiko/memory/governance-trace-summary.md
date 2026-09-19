# Trace summary — governance surface set v1.0.0 (2026-04-14)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–016).
The validator's grading surface.

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile (no compliance modules; negatives confirmed) | fact profile | region ratified stamp | ledger header ✓ |
| GI-002 | Identity/type (backend + dashboard; two people) | identity | region Technology stack | ledger amendment-policy approvers ✓ |
| GI-003 | Depth level: low | depth declaration | region ratified stamp | ledger header ✓ |
| GI-004 | Security by Default | floor-asserted: FLOOR-SEC | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-005 | Testing Discipline | floor-asserted: FLOOR-TEST | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-006 | Error Handling | floor-asserted: FLOOR-ERR | region principle line | index=line ✓ · ledger ✓ |
| GI-007 | Observability (waived in part — GI-009) | floor-asserted: FLOOR-OBS | region principle line | index=line ✓ · ledger ✓ · waiver row ✓ |
| GI-008 | Tenant-Scoped Data Access | minted | rules/mochiko/data-access.md | index ✓ · ledger ✓ |
| GI-009 | FLOOR-OBS structured-logging waiver | waiver | ledger waiver table | synthesis ✓ |
| GI-010 | Repositories per Aggregate | minted | region principle line | index=line ✓ · ledger ✓ |
| GI-011 | BE-HEX dropped; layer-rules declined | bookkeeping | synthesis only | — |
| GI-012 | Release gates | module: release-gates | region summary line + ledger detail | index ✓ · ledger ✓ |
| GI-013 | evolution-notes module | module: evolution-notes | ledger Evolution notes section | region pointer ✓ |
| GI-014 | knowledge-management declined (durable) | module ruling | synthesis only | — |
| GI-015 | Short-Lived Feature Flags | minted | region principle line | index=line ✓ · ledger ✓ |
| GI-016 | `fly.toml` password confrontation | confrontation | GI-004 enforcement + ledger confrontation line | ✓ |

Every surface element traces back: region lines → GI-001/002/003/004/005/006/007/010/012/013/015 ·
ledger entries → GI-004–010, 012, 015, 016 · `rules/mochiko/data-access.md` → GI-008 ·
output-style rules file → template-mandated (Shape 5, routed from no principle).

**Flagged proposals:** none

**Waivers:** GI-009
