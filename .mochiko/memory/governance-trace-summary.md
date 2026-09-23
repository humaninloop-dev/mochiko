# Trace summary — governance surface set v3.1.2 (2026-09-23, AM-4)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–022). Grading
surface for `validation-constitution`. Superseded rounds: v2.0.0 and v3.0.0 carried their
manifests inside their producer reports
(`.mochiko/brainstorms/cli-schema-delivery/reports/wave2-gov-producer.md` for v3.0.0), and the
v1.0.0 manifest this file held — including its ruled FP-1 — is in git history. The v3.1.0
manifest (AM-3, with its FP-1 to FP-5) is in git history at `794cea8`. v3.1.1 was a lead PATCH
(the `validator` retirement) that carried no manifest.

## Trace summary — GI element to homes

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile — no compliance modules, negatives incl. no-UI; unchanged at AM-4 | session fact profile | region ratified stamp | ledger header ✓ · **stamp re-versioned v3.1.2 · 2026-09-23 (AM-4)** at region `:109` and ledger `:4` |
| GI-002 | Identity · risk surface · team reality | session dimensions 1–5 | region Technology stack line | ledger amendment-policy trigger ✓; no Three-Part entry (FP-3 at AM-2, accepted) — unchanged at AM-4 |
| GI-003 | Secrets out of the repo | floor-asserted: FLOOR-SEC | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — unchanged at AM-4 |
| GI-004 | Primitive audit ratchet | floor-asserted: FLOOR-TEST | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` quality-gate bullet ✓ — unchanged at AM-4 |
| GI-005 | Record-layer integrity | floor-asserted: FLOOR-ERR | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` log-is-truth bullet ✓ — unchanged at AM-4 |
| GI-006 | Traceability as observability | floor-asserted: FLOOR-OBS | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — unchanged at AM-4; the rule the KM file's preserved trigger text and the ledger's kept GI-009 bullet serve |
| GI-007 | Deliberate exclusions, narrowed to markdown primitives | session dimension 10 | ledger GI-004/005/006 trace lines | synthesis ✓ — unchanged at AM-4 |
| GI-008 | Waiver — FLOOR-TEST for the skill-shipped helpers | waiver | ledger waiver table | synthesis ✓ — unchanged at AM-4 |
| GI-009 | Knowledge-management core — **carried `GLOSSARY.md` deviation discharged at AM-4** | module: knowledge-management-core | `.mochiko/memory/knowledge-management.md` — **`GLOSSARY.md` joins the core set at `:11`; deferral clause and revisit trigger retired at `:14-20`, prior trigger text preserved in the discharge parenthetical** | region Governance-operations line `:140` gains `GLOSSARY.md` (FP-1) ✓ · ledger GI-009 **AM-4 Enforcement bullet `:227-230` + trace `:241`**, v1.0.0 bullet kept verbatim ✓ · ledger amendment policy `:42-46` trigger discharged and struck ✓ · region amend-triggers line `:135` trigger struck ✓ · `operating-docs.md` ✓ (already globs `GLOSSARY.md`; not edited) |
| GI-010 | CHANGELOG elective | module: km-elective-changelog | `CHANGELOG.md` | ledger ✓ · region release-gates line, gate 4 ✓ · `rust-cli.md` `paths` ✓ — unchanged at AM-4 |
| GI-011 | RUNBOOK elective — declined durable | module decline | ledger declined-durable line | synthesis ✓ — unchanged at AM-4 |
| GI-012 | Release gates | module: release-gates | ledger GI-012 (detail) | region gates lines verbatim ✓ · ledger ✓ · `rust-cli.md` ✓ — **the hook wave's owed pre-authorized PATCH re-keyed v3.1.2 → v3.1.3 at ledger `:407`** (the PATCH that strikes the wave-4 precondition) |
| GI-013 | layer-rules — declined durable | module decline | ledger amendment-policy line | ✓ — unchanged at AM-4 |
| GI-014 | evolution-notes — declined durable | module decline | ledger amendment-policy line | ✓ — unchanged at AM-4 |
| GI-015 | Live-token confrontation | brownfield confrontation | ledger confrontation table | folded into GI-003 enforcement ✓ — unchanged at AM-4 |
| GI-016 | Marketplace-lag confrontation | brownfield confrontation | ledger confrontation table | folded into GI-012 gate 5 ✓ — unchanged at AM-4 |
| GI-017 | Pointer-only region | minted | region line | index = line ✓ · ledger ✓ — **binds this round: the `:140` addition (FP-1) is a pointer to the doc, the term format stays in the KM file; the `:135` strike adds nothing** |
| GI-018 | ARCHITECTURE.md lag accepted | brownfield confrontation | ledger confrontation table | ✓ — unchanged at AM-4 |
| GI-019 | Kernel-class tooling admission (bright line) | minted AM-1 · widened AM-2 · conformance-gate admission AM-3 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` ✓ — **clause (iv)'s C1 note re-keyed at ledger `:407`: the owed PATCH now v3.1.3**; no other change |
| GI-020 | Clone-only install, required `mochiko-cli` | minted AM-1, superseded-by-ruling AM-2 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` dependency bullet ✓ — unchanged at AM-4 |
| GI-021 | Depth level `high` | minted AM-1 | region ratified stamp | ledger header ✓ — unchanged at AM-4 |
| GI-022 | No repo-level feature map for this repo | minted AM-2 | region index line | ledger ✓ — unchanged at AM-4 |

## Trace summary, reverse — changed surface line to GI element

| Surface | Changed line | GI |
|---------|--------------|-----|
| region `:109` | ratified stamp v3.1.1 → v3.1.2 · 2026-09-23 (AM-4) | GI-001, GI-021 |
| region `:135` | amend-triggers line: "· GLOSSARY.md content" struck | GI-009 |
| region `:140` | operating-docs line gains "terms in `GLOSSARY.md`" (FP-1) | GI-009, GI-017 |
| ledger `:4` | version 3.1.1 → 3.1.2 | GI-001 |
| ledger `:42-46` | amendment policy: GLOSSARY.md-content trigger fired and discharged at AM-4, struck from the standing set; synthesis-recorded trigger list at `:36-41` kept verbatim | GI-009 |
| ledger `:227-230` | GI-009 Enforcement gains the AM-4 discharge bullet; v1.0.0 bullet kept verbatim | GI-009, GI-006 |
| ledger `:241` | GI-009 trace gains "AM-4 2026-09-23: GLOSSARY.md deviation discharged" | GI-009 |
| ledger `:407` | GI-019 clause (iv) C1 note: owed hook PATCH re-keyed v3.1.2 → v3.1.3 | GI-019, GI-012 |
| ledger `:632` | amendment-log row 3.1.2 (incl. the FP-1 clause) | all of the above |
| KM `:11` | adopted core set gains `GLOSSARY.md` (AM-4, 2026-09-23; term format below) | GI-009 |
| KM `:13-15` | "Deviation carried" loses the `GLOSSARY.md` clause; command-boundary clause kept, re-wrapped only | GI-009 |
| KM `:18-20` | revisit-trigger line removed; its text preserved in a discharge parenthetical | GI-009, GI-006 |
| this file | regenerated whole at v3.1.2; superseded-rounds pointer extended to the v3.1.0 manifest and the manifest-less v3.1.1 PATCH | GI-006 |

## Floor coverage

All four Essential Floor categories accounted for, none dropped and none re-expressed at AM-4:
GI-003 (FLOOR-SEC) · GI-004 (FLOOR-TEST) · GI-005 (FLOOR-ERR) · GI-006 (FLOOR-OBS). Module set
unchanged: compliance none · knowledge-management (core + CHANGELOG) · release-gates.

**Flagged proposals** — the user's to rule at acceptance, reversible by a single edit:

- **FP-1** — the region's operating-docs line (`CLAUDE.md:140`) gains "terms in `GLOSSARY.md`".
  Claimed "minimum now, defensible", not required: without it the only always-on text naming
  `GLOSSARY.md` is `CLAUDE.md:94`, outside the region and out of AM-4's scope, which still reads
  "when they gain content". A pointer only (GI-017); the term format stays in the KM file. The
  ledger's 3.1.2 row names it. *Alternative:* revert `:140` and the row's FP-1 clause.
  **Accepted by the user 2026-09-23 ("accept").**

## Validator record (evidence floor)

- **Round 1:** FAIL, 59/61, one blocking finding — **B1**: no AM-4 trace summary manifest (this
  file still headed v3.1.0, its GI-009 row silent on the discharge). Fixed this round: this file
  regenerated at v3.1.2.
- **A1** — the 3.1.2 amendment-log row did not name the region operating-docs addition. Fixed
  this round: ledger `:632` gains "region operating-docs line gains `GLOSSARY.md` (FP-1,
  user-ruled at acceptance)"; if the user declines FP-1, the lead reverts that clause and `:140`.
- **A2** — no writer moment for `GLOSSARY.md` (no command or skill names when terms are minted).
  Out of AM-4 scope; routed to the user as a candidate follow-up.
- **A3** — the region's amend-triggers line omits the helper-script waiver trigger; GI-009
  Testability names "in-flight agreement". Pre-existing, not AM-4; routed to the user.
- **A4** — evidence floor: discharged by this validator record.

**Waivers:** GI-008 (sole) — row and notes unchanged at AM-4.
**Narrowings:** GI-003 secret-scanning, recorded in its ledger entry and un-narrowed at v0.76.0.
**Owed at the next PATCH:** the ledger's semver MAJOR limb lacks the template's `depth-level flip
(low→high)` clause — pre-existing, outside AM-4's scope, not authored here.
