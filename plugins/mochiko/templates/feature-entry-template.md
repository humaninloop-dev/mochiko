<!-- Form: templates/artifact-format.md (the deliverable envelope). One file per feature:
     .mochiko/features/FEAT-XXX-<slug>.md. Authored and amended per
     mochiko:authoring-feature-map — the skill carries the derivation method, the delta
     grammar, and the write timing; this template carries only the shape.
     Status is owned HERE (and mirrored one-line in FEATURES.md): story files derive
     theirs by following the FEAT-ID. Omit empty sections per artifact-format.md rule 6.
     Register: `full` per artifact-format.md rule 11. -->

# FEAT-{{XXX}} — {{feature_name}}

> Status: {{status}}  <!-- proposed | in-flight | delivered | retired -->
> {{status_line_detail}}
<!-- Status-dependent detail, one line:
     in-flight  → since {{date}} · owning spec: `.mochiko/specs/{{spec-slug}}/`
     delivered  → since {{date}}
     retired    → {{date}} · ruling: {{pointer}}
     proposed   → surfaced by {{spec-slug}} ({{date}})
     Reconstructed entries add: `reconstructed-from-code ({{date}}) — first touching
     spec re-verifies extent` — cleared at that spec's acceptance. -->

## Capability

{{capability_statement}}
<!-- 1–3 lines, the system's own language — what the system does, standing on its own
     without reference to any story or spec. More than ~3 lines of extent below usually
     means this is two features. -->

## Extent

<!-- What is in, and what is notably NOT in — one line each. This is where stories
     sharpen the feature over time. -->

- {{in_scope_line}}
- Not: {{out_of_scope_line}}

## Relations

<!-- One line per relation to another FEAT-ID: depends-on / extends / composes-with. -->

- depends-on: {{FEAT-ID}} — {{one_line_why}}

## Architecture

<!-- Which ARCHITECTURE.md components realize this capability — pointers only, navigable
     both directions; never restate the component view. -->

- {{component_name}} (`ARCHITECTURE.md` § {{section}})

## Story trace

<!-- Accumulating provenance: which specs/stories informed this feature — IDs and
     pointers only, newest first. -->

- {{spec-slug}}: US-{{n}}, US-{{n}}

## Obligations

<!-- Deferred work that rides this entry until it builds — one line each:
     deferred SC-XXX (travels here when its covering feature went unselected) ·
     deferred seams ("when built, verify seam against FEAT-XXX") ·
     cross-cutting extend obligations from stories homed elsewhere. -->

- {{obligation_line}}

## Deltas

<!-- Marked changes riding a delivered entry — status never regresses. Grammar:
     "extent grows by {{X}} — in-flight, {{spec-slug}}". Each delta names its spec;
     it folds into Extent at that work's acceptance landing, then leaves this list. -->

- extent grows by {{X}} — in-flight, {{spec-slug}}
