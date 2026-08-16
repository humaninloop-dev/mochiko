# Strip notes — `templates/constitution-modules/layer-rules.md`

Entry formats: `strips/README.md`. Filename follows the sibling constitution-module convention
(`knowledge-management-module.md`): `<module>-module.md`, one file per primitive.

**Wave context (v0.77.0 — the template-schema ratchet, D3 later-ratchet).** The
`schema-based-template-guidance` D3 later-ratchet is exercised over the remaining `.md` templates
per the **user ruling 2026-08-16** (recorded at the v0.76.0 landing, against the DM's
scope-breadth recommendation). Mechanism verbatim: the D8 raw-Read data files stay the source of
truth, the binary renders over them, and each converted `.md` takes this strip ceremony. Standing
honesty flag (not voided by contest): the mechanism is still **n=0** — this ratchet extends it
over 15 files before the first-live-run watch resolves, so the M7 rollback surface widens to every
converted file. This module is one of the three **Class C** constitution-module conversions
(verbatim standard, validation-constitution audit). Record:
`.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` will carry the
ratchet's own landing row.

## [v0.77.0] Module retired — superseded by schema-based template guidance (D3 later-ratchet)
- **Disposition:** superseded → plugins/mochiko/schemas/layer-rules.yaml + mochiko-cli template layer-rules
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema ratchet (D3 later-ratchet exercised)")
- **Content (superseded module, full verbatim below):**

````markdown
<!--
MODULE: layer-rules
===================
Attach when: the synthesis kept a layered-architecture card (e.g. BE-HEX) OR minted a
layered-architecture intent — the module ruling is recorded in the synthesis either way (the
interrogation's layered-architecture beat). Never attach for projects that ruled it out: these
sections are exactly the layer flavor the core template no longer carries.
Trace: the GI module-selection element that names `layer-rules`.
-->

## Project Structure

<!--
INSTRUCTION: The expected folder organization, matching the kept architecture card's layer
structure and the project's actual conventions (brownfield: from codebase-analysis.md).
-->

```
[PROJECT_ROOT]/
├── [SOURCE_DIR]/
│   ├── [LAYER_1]/        # [Purpose]
│   ├── [LAYER_2]/        # [Purpose]
│   └── [LAYER_3]/        # [Purpose]
├── [TEST_DIR]/
│   ├── unit/             # Unit tests
│   └── integration/      # Integration tests
└── [CONFIG_FILES]
```

### Layer Import Rules

| Layer | MAY Import | MUST NOT Import |
|-------|------------|-----------------|
| [LAYER_1] | [Allowed layers] | [Prohibited layers] |
| [LAYER_2] | [Allowed layers] | [Prohibited layers] |
| [LAYER_3] | [Allowed layers] | [Prohibited layers] |

### Domain-Dependency Registry

The domain layer MAY import libraries listed in the registry block below — seeded at setup
(session-arbitrated), grown at implement time under the human-gated add-process. Craft and
policy single source: `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`.

The authored domain-layer rules file carries **two parts with different ownership**:

- **Policy preamble** (setup-owned, regenerated): the qualification criteria, the add-process,
  and the add gate — regenerated from the governance ledger's Domain-dependency policy section.
- **Registry block** (living, preserved): the list itself, between
  `<!-- mochiko:domain-registry:begin -->` and `<!-- mochiko:domain-registry:end -->` markers —
  **preserved verbatim across setup/amend regenerations** (implement-time additions live here;
  the one carve-out from rules-files-regenerated-whole). Row schema:

  | Dependency | Justification | Signal level | Added (by/when) | Gate |
  |------------|---------------|--------------|-----------------|------|

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Project Structure tree present with real directory names (no [LAYER_N] placeholders)
- [ ] Layer Import Rules table present; every layer has MAY and MUST NOT columns filled
- [ ] Import rules consistent with the kept (or minted) architecture ruling's layer structure
- [ ] Enforcement for layer rules names a real tool (import linter / CI rule), not "code review" alone — production-strength enforcement
- [ ] Every rules file whose concern a layer can violate (per the Import Rules table — including orchestration through ports) includes that layer in its `paths`
- [ ] Domain-layer rules file carries exactly one `mochiko:domain-registry` begin/end marker pair, with the policy preamble above it
- [ ] Every registry row carries justification, signal level, provenance, and gate fields (no blank metadata)
- [ ] At ratification, registry rows match the synthesis's Domain-dependency seed rulings (implement-time rows added later carry their add-gate provenance instead)
- [ ] CLAUDE.md sync table carries a row for this module's content
-->
````

- **Section map (I2 — fragment-line-driven, 1:1).** The module's 9 Validator-checklist-fragment lines each get exactly one producer section (section count = fragment-line count); each section's `check` reproduces its fragment line **verbatim** (confirmed via `mochiko-cli template layer-rules --check`): (1) *Project Structure*; (2) *Layer Import Rules*; (3) *Import-rule architecture consistency*; (4) *Layer-rule enforcement tooling*; (5) *Rules-file path coverage*; (6) *Domain-Dependency Registry*; (7) *Registry row metadata*; (8) *Registry seed matches synthesis rulings*; (9) *CLAUDE.md sync-table row*.
- **Section fabrication (disclosed).** The source carries three headings: `## Project Structure`, `### Layer Import Rules`, `### Domain-Dependency Registry`. Three section names reuse those headings verbatim (not fabricated): *Project Structure*, *Layer Import Rules*, *Domain-Dependency Registry*. **Six section headings are fabricated** to give their fragment line a 1:1 home: *Import-rule architecture consistency* (cross-check against the architecture ruling), *Layer-rule enforcement tooling* (real-tool enforcement), *Rules-file path coverage* (`paths`-glob coverage), *Registry row metadata* (row-field completeness facet of the registry), *Registry seed matches synthesis rulings* (ratification-time seed check), *CLAUDE.md sync-table row* (sync-table cross-check). These carry the check obligation as producer framing and point at the content home; no module-body content is invented, only a section home for each check. **Named producer-framing addition (V2 F1):** the *Layer-rule enforcement tooling* contract adds one locational claim not present in the source text — "The mechanism is recorded in the governance ledger's Three-Part metadata for the layer-rules principle" — accurate to the governance-ledger structure (`governance-surfaces.yaml` Shape 3) and added as producer guidance, not a verbatim source line.
- **Kept deliberately:** every operative doctrine line carried **verbatim** into the schema — the MODULE header (in `overview`); the `## Project Structure` heading + `INSTRUCTION` comment + the folder tree (in the *Project Structure* contract); the `### Layer Import Rules` heading + import table (in *Layer Import Rules*); the `### Domain-Dependency Registry` heading + the full registry prose (two-part-ownership, policy-preamble/registry-block doctrine, `mochiko:domain-registry` marker names, the preserved-verbatim rule) + the row-schema table (in *Domain-Dependency Registry*). The 9 fragment lines became the 9 `check` lines verbatim. The `<!--`/`-->` MODULE-comment delimiters were dropped so the Attach-when guidance renders as visible `overview` prose (words unchanged) — matching `governance-surfaces.yaml`. **Shape metadata not in the source:** `form: artifact-format.md` and `register: full` are schema-shape fields the source never declared; set to match the sibling `governance-surfaces.yaml`. **Modeling:** the fillable markdown lives in the three content sections' contract fenced blocks (the tree wrapped in a 4-backtick fence so its inner 3-backtick code fence survives; each block intact, not fragmented); the `skeleton` is an assembly note (the `governance-surfaces.yaml` model). Nothing removed.
- **Consumers assessed:** referenced by `validation-constitution/SKILL.md:45` (glob), `validation-constitution/references/QUALITY-CHECKLIST.md:61`, and `authoring-constitution/SKILL.md:221` (dir link) — each re-pointed by the re-point seat (P5) to the `--check`/schema for the converted modules (mixed-source: `knowledge-management` stays raw `.md`). `INTERROGATION-AGENDA.md:46` and `commands/setup.md:83` (KM) untouched; the `templates/constitution-modules/` directory survives. The registry doctrine's other home, `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`, is the craft/policy single source and is untouched (this module points at it, does not restate it). Each re-pointed consumer appends its own supersession strip.
- **Source deletion:** at phase 2 only, after the `validation-constitution` (V2) doctrine-preservation audit PASSES.
