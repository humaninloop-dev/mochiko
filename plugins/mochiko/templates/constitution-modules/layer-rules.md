<!--
MODULE: layer-rules
===================
Attach when: the synthesis kept a layered-architecture card (e.g. BE-HEX) — typically backend /
service / fullstack-api types at internal tier and above. Never attach for types that didn't
select it: these sections are exactly the backend flavor the core template no longer carries.
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

**Domain Layer Note**: The domain layer MAY import libraries listed in the project's approved domain dependencies registry. Maintain this registry as a project-level document that records each approved domain dependency together with the justification for permitting it in the domain layer.

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Project Structure tree present with real directory names (no [LAYER_N] placeholders)
- [ ] Layer Import Rules table present; every layer has MAY and MUST NOT columns filled
- [ ] Import rules consistent with the kept architecture card's layer structure
- [ ] Enforcement for layer rules names a real tool (import linter / CI rule), not "code review" alone at production+ tier
- [ ] Every rules file whose concern a layer can violate (per the Import Rules table — including orchestration through ports) includes that layer in its `paths`
- [ ] CLAUDE.md sync table carries a row for this module's content
-->
