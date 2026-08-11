# Strip notes — `templates/governance-surfaces-template.md`

Entry formats: `strips/README.md`.

**Wave context (v0.65.0 — the production-floor adaptive-depth landing).** The asserted production
floor gains a **two-row `low`/`high` depth level** (one project-wide dial, user-declared, one-way
`low`→`high`). Ruling: `production-floor-adaptive-depth`, ratified 2026-08-11, D1–D8 —
`.mochiko/brainstorms/production-floor-adaptive-depth/record.md`; `DECISIONS.md` 2026-08-11
adaptive-depth row. On this template the two entries below carry **level state** into the region
stamp (Shape 1) and the ledger's Governance-Floor line (Shape 3) — both are identity-carrying,
`DECISIONS.md`-traceable stamps, so they are supersessions **by ruling**. The rest of this
template's depth work is **pure addition** and takes no strip entry: the semver MAJOR list gains
the `low`→`high` flip event, and the amendment-policy Route line gains the flip as a governance
event recorded via amendment-log rows (no new ledger structure).

## [v0.65.0] Shape 1 region stamp — single-floor Ratified line superseded (carries depth level)
- **Disposition:** superseded → the `**Ratified:**` region stamp in Shape 1 of `templates/governance-surfaces-template.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**Ratified:** v[X.Y.Z] · [YYYY-MM-DD] · production floor · modules: [attached compliance modules, or "none"] <!-- GI-001 (fact profile) -->
```
- **Also reworded for disambiguation (lead ruling #2, same citation):** the Shape 1 Quality-gates line `Coverage ≥ [asserted floor level, session-overridable]%` → `Coverage ≥ [floor card's coverage threshold, session-overridable]%`. "Asserted floor level" there named the coverage-threshold number, not the new depth level; reworded so "level" is reserved for the depth declaration.
- **Kept deliberately:** the stamp's every other field verbatim — version, ratified date, `production floor`, the modules field, and the `GI-001 (fact profile)` trace comment; the depth field and its `GI-0XX (depth level)` trace are additions within the same line.
- **Consumers assessed:** grep across `plugins/` — `authoring-constitution/SKILL.md` mandatory-content-inventory item 1 (Ratified stamp) re-keyed to name the declared depth level this same wave (cluster B); `validation-constitution` grades the stamp fields (Cluster C re-keys it to the two-row form). No removed anchor.

## [v0.65.0] Shape 3 ledger Governance-Floor line — single-floor line superseded (carries depth level)
- **Disposition:** superseded → the `**Governance Floor:**` line in Shape 3 (the ledger) of `templates/governance-surfaces-template.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**Governance Floor:** production (asserted) · **Modules:** [attached compliance modules with strata, or "none"] · **Trace:** GI-001 (fact profile)
```
- **Kept deliberately:** `production (asserted)`, the Modules field with its strata note, and the `GI-001 (fact profile)` trace, all verbatim; the **Depth level** field (user-declared, one-way, `high` terminal) and its `GI-0XX (depth level)` trace are additions within the same line.
- **Consumers assessed:** the ledger is read by setup/amend runs and `validation-constitution` only; the amendment-policy semver + Route lines below it gained the flip event as pure additions this wave. No removed anchor.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Design-record citation in the ownership header
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(design: .mochiko/brainstorms/constitution-native-surfaces/record.md,
D1–D8)
```
- **Kept deliberately:** the operative assertion — there is NO constitution.md, and governance is a SET authored by `authoring-constitution` and graded by `validation-constitution`.
