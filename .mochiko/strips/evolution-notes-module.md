# Strip notes — `templates/constitution-modules/evolution-notes.md`

Entry formats: `strips/README.md`. Filename follows the sibling constitution-module convention
(`knowledge-management-module.md`): `<module>-module.md`, one file per primitive.

**Wave context (v0.77.0 — the template-schema ratchet, D3 later-ratchet).** The
`schema-based-template-guidance` D3 later-ratchet is exercised over the remaining `.md` templates
per the **user ruling 2026-08-16** (recorded at the v0.76.0 landing, against the DM's
scope-breadth recommendation). Mechanism verbatim: the D8 raw-Read data files stay the
source of truth, the binary renders over them, and each converted `.md` takes this strip ceremony.
Standing honesty flag (not voided by contest): the mechanism is still **n=0** — this ratchet
extends it over 15 files before the first-live-run watch resolves, so the M7 rollback surface
widens to every converted file. This module is one of the three **Class C** constitution-module
conversions (verbatim standard, validation-constitution audit). Record:
`.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` will carry the
ratchet's own landing row.

## [v0.77.0] Module retired — superseded by schema-based template guidance (D3 later-ratchet)
- **Disposition:** superseded → plugins/mochiko/schemas/evolution-notes.yaml + mochiko-cli template evolution-notes
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema ratchet (D3 later-ratchet exercised)")
- **Content (superseded module, full verbatim below):**

````markdown
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
record in the ledger's Waivers section, not a gap):

| Category | Status | Response |
|----------|--------|----------|
| Security | [present / partial / absent] | [codified existing pattern / MUST-implement → GAP-XXX / waived (see Waivers)] |
| Testing | [status] | [response] |
| Error Handling | [status] | [response] |
| Observability | [status] | [response] |

**Confrontations resolved in session:** [detected-reality-vs-floor conflicts and their
rulings, from the synthesis — e.g. "the floor requires tests and the codebase has none: ruled
MUST-implement, GAP-002." Or "none."]

See `.mochiko/memory/evolution-roadmap.md` for the improvement plan.
<!-- Roadmap stub (moved-to-other-cluster): producing evolution-roadmap.md is the roadmap
cluster's job, not ported yet. Write the gap-status table now; the linked roadmap is filled in
when that cluster lands. -->

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Essential Floor status table present, all four categories assessed
- [ ] Every "absent"/"partial" category has a response: codified pattern, GAP reference, or a waiver record in the ledger's Waivers section
- [ ] Statuses match codebase-analysis.md (cross-check)
- [ ] Technology stack matches codebase analysis
- [ ] Quality gates reflect current + target state
- [ ] Session confrontations recorded (or "none")
-->
````

- **Section map (I2 — fragment-line-driven, 1:1).** The module's 6 Validator-checklist-fragment lines each get exactly one producer section (section count = fragment-line count); each section's `check` reproduces its fragment line **verbatim** (confirmed against the source via `mochiko-cli template evolution-notes --check`). Map: (1) *Essential Floor Status* → "table present, all four categories assessed"; (2) *Gap responses* → response required for absent/partial; (3) *Status cross-check with codebase analysis* → "Statuses match codebase-analysis.md"; (4) *Technology stack alignment* → "Technology stack matches codebase analysis"; (5) *Quality gates current and target* → "Quality gates reflect current + target state"; (6) *Confrontations resolved in session* → "Session confrontations recorded (or 'none')".
- **Section fabrication (disclosed).** The source carries one heading (`## Evolution Notes`) and organizes the rest by bold labels. Two section names reuse the source's bold labels verbatim — *Essential Floor Status*, *Confrontations resolved in session* (not new headings). **Four section headings are fabricated** to give their fragment line a 1:1 home: *Gap responses* (the Response-column facet of the floor table), *Status cross-check with codebase analysis*, *Technology stack alignment*, *Quality gates current and target*. Fragments 4 and 5 are cross-checks into the **core constitution's** own Technology-stack and Quality-gates content (which lives in the CLAUDE.md region, not this module body); their fabricated sections carry the check obligation as producer framing and point at that core content — no module-body content is invented, only a section home for the check.
- **Kept deliberately:** every operative doctrine line carried **verbatim** into the schema — the MODULE header (in `overview`), the `## Evolution Notes` heading + brownfield-context line + `**Essential Floor Status**` note + the 4-row floor table (in the *Essential Floor Status* contract), the `**Confrontations resolved in session:**` block + roadmap pointer + roadmap-stub comment (in the *Confrontations resolved in session* contract); the 6 fragment lines became the 6 `check` lines verbatim. The `<!--`/`-->` MODULE-comment delimiters were dropped so the Attach-when guidance renders as visible `overview` prose (words unchanged) — the same treatment the first wave gave `governance-surfaces.yaml`. **Shape metadata not in the source:** `form: artifact-format.md` and `register: full` are schema-shape fields the source `.md` never declared; set to match the sibling governance schema `governance-surfaces.yaml` (same `validation-constitution` grader). **Modeling:** the fillable markdown lives in the two content sections' contract fenced blocks (each block intact, not fragmented); the `skeleton` is an assembly note (the `governance-surfaces.yaml` model). Nothing removed.
- **Consumers assessed:** the constitution-module set is referenced by `validation-constitution/SKILL.md:45` (glob), `validation-constitution/references/QUALITY-CHECKLIST.md:61`, and `authoring-constitution/SKILL.md:221` (dir link) — each re-pointed by the re-point seat (P5) to the `--check`/schema for the converted modules (mixed-source: `knowledge-management` stays raw `.md`). `authoring-constitution/references/INTERROGATION-AGENDA.md:46` (KM pointer) and `commands/setup.md:83` (KM pinning) are untouched. The `templates/constitution-modules/` directory survives (knowledge-management stays). Each re-pointed consumer appends its own supersession strip.
- **Source deletion:** at phase 2 only, after the `validation-constitution` (V2) doctrine-preservation audit PASSES.
