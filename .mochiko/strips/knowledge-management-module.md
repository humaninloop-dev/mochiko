# Strip notes — `templates/constitution-modules/knowledge-management.md`

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

## [v0.44.0] Redesign record citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
`.mochiko/brainstorms/operating-docs-maintenance/record.md`
D1–D12 + R1–R7;
```
- **Kept deliberately:** the ruling date (2026-07-25) and the substantive clause it introduced — the redesign deliberately supersedes the prior four-part "no inner menu" bundle rule.

## [v0.29.0] Module rewritten whole per the operating-docs-maintenance redesign
- **Disposition:** superseded-by-redesign (ruled, not a minimalism strip) — old content
  retrievable at git history `7920ccb` and `.mochiko/archive/`
- **Content that left:** the four-part bundle (incl. the `DECISIONS.md` artifact + its
  evolution-roadmap disambiguation form) · the "no inner menu" whole-bundle rule (deliberately
  superseded by core-whole + electives, R6) · the three-carrier chain incl. the stub-backed
  CLAUDE.md-sync rows carrier (replaced by scaffolded pointers, D7) · the content-quality
  exemption (replaced by the enforced floor: boundary invariants + subtractive landing, D6/R1)
- **Provenance:** DECISIONS.md OD-D1–D12;
  `.mochiko/brainstorms/operating-docs-maintenance/record.md`
- **Consumers assessed:** `setup.md` (G5 — updated) · `brainstorm.md` (invariant source —
  retargeted to the project-pinned copy) · `specify.md` (Finalize KM landing added —
  GLOSSARY minting; format home fixed at audit: Document contracts pinned with the copy) ·
  `plan.md` (Finalize KM landing + ARCHITECTURE dispatch) · `implement.md` (Finalize KM
  landing + ARCHITECTURE dispatch) · `INTERROGATION-AGENDA.md` dimension 7 (updated) ·
  `validation-constitution` module fragment (the template's own embedded fragment rewritten
  with it) · `governance-intent-template.md` module-ruling rows (unchanged — module name and
  recorded-decline semantics survive)
