# Trace summary — governance surface set v1.0.0 (2026-08-06)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–018).
Grading surface for `validation-constitution`.

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile (no compliance modules; negatives incl. no-UI) | fact profile | region ratified stamp | ledger header ✓ |
| GI-002 | Identity/type (no shelf; procedural quality; solo approver) | identity | region Technology stack | ledger amendment-policy approvers + standing triggers ✓ |
| GI-003 | Secrets Out of the Repo (narrowed scanning) | floor-asserted: FLOOR-SEC | region principle line | index=line ✓ · ledger ✓ · `.gitignore` fix landed ✓ |
| GI-004 | Primitive Audit Ratchet | floor-asserted: FLOOR-TEST | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-005 | Record-Layer Integrity | floor-asserted: FLOOR-ERR | region principle line | index=line ✓ · ledger ✓ · quality-gates line ✓ |
| GI-006 | Traceability as Observability | floor-asserted: FLOOR-OBS | region principle line | index=line ✓ · ledger ✓ |
| GI-007 | Exclusion: application-shaped machinery inapplicable in kind | exclusion | ledger (subsumed-clause notes in GI-004/5/6 entries + floor-status rollup) | synthesis ✓ |
| GI-008 | Helper-script FLOOR-TEST waiver | waiver | ledger waiver table | synthesis ✓ |
| GI-009 | KM core — pin ratified | module: knowledge-management-core | `.mochiko/memory/knowledge-management.md` (pin header updated) | region operating-docs line ✓ · ledger ✓ |
| GI-010 | CHANGELOG elective | module: km-elective-changelog | `CHANGELOG.md` (scaffolded) | ledger ✓ · release gate 4 ✓ |
| GI-011 | RUNBOOK elective declined (durable) | module ruling | synthesis (recorded decline — nothing scaffolded, by design) | ledger not required for a decline; synthesis row ✓ |
| GI-012 | Release gates (5 gates; marketplace sync = gate 5) | module: release-gates | region quality-gates line + ledger detail entry | index ✓ · ledger ✓ |
| GI-013 | layer-rules — not offered, no ruling, offerable on amend | bookkeeping | synthesis only (correct — no surface content) | — |
| GI-014 | evolution-notes module — not offered, no ruling | bookkeeping | synthesis only; **note:** brownfield evolution content (floor status + confrontation rulings) carried as plain ledger sections without attaching the module — see flagged proposal FP-1 | — |
| GI-015 | Token exposure confrontation | confrontation | `.gitignore` (fixed this run) + ledger confrontation table | GI-003 enforcement ✓ |
| GI-016 | Marketplace lag confrontation | confrontation | GI-012 gate 5 + ledger confrontation table | ✓ |
| GI-017 | Pointer-Only Region | minted | region principle line | index=line ✓ · ledger ✓ |
| GI-018 | ARCHITECTURE.md version-lag accepted | confrontation | ledger confrontation table | ✓ |

Every surface element traces back: region lines → GI-001/002/003/004/005/006/009/012/017 ·
ledger entries → GI-003–012, 015–018 · `CHANGELOG.md` → GI-010 · `.gitignore` entry → GI-003/015 ·
KM pin header edit → GI-009/010/011 · output-style rules file → template-mandated (Shape 5,
routed from no principle, by design).

**Flagged proposals:**
- **FP-1** — the skill mandates `evolution-notes` module attachment in brownfield ("always"),
  but the synthesis records GI-014 as *not offered, no ruling* — attaching it would be
  unsanctioned selection, which binds harder. Authored middle path: the brownfield content the
  module would carry (floor-status rollup + confrontation rulings) lives as plain ledger
  sections, module unattached, GI-014 stays offerable on amend. User rules at acceptance:
  accept as authored, or attach the module formally (amend-scale edit).

**Waivers:** GI-008 (sole).
**Narrowings:** GI-003 secret-scanning (recorded in the ledger entry, not the waiver table).
