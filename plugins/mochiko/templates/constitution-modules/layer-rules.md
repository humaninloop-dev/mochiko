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
(session-arbitrated), grown at implement time under the tier-keyed add-process. Craft and
policy single source: `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`.

The authored domain-layer rules file carries **two parts with different ownership**:

- **Policy preamble** (setup-owned, regenerated): the qualification criteria, the add-process,
  and the tier gate — regenerated from the governance ledger's Domain-dependency policy section.
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
- [ ] Enforcement for layer rules names a real tool (import linter / CI rule), not "code review" alone at production+ tier
- [ ] Every rules file whose concern a layer can violate (per the Import Rules table — including orchestration through ports) includes that layer in its `paths`
- [ ] Domain-layer rules file carries exactly one `mochiko:domain-registry` begin/end marker pair, with the policy preamble above it
- [ ] Every registry row carries justification, signal level, provenance, and gate fields (no blank metadata)
- [ ] At ratification, registry rows match the synthesis's Domain-dependency seed rulings (implement-time rows added later carry their add-gate provenance instead)
- [ ] CLAUDE.md sync table carries a row for this module's content
-->
