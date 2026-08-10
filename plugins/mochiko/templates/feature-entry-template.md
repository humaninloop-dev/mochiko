<!-- Form: templates/artifact-format.md (the deliverable envelope). One file per feature:
     .mochiko/features/FEAT-XXX-<slug>.md. Authored and amended per
     mochiko:authoring-feature-map — the skill carries the derivation method, nesting and
     roll-up rules, the delta grammar, and the write timing; this template carries only
     the shape. Entries nest two levels max: parent (capability, roll-up) and leaf
     (deliverable, the pipeline unit); a flat entry is a leaf. Use Parent on a leaf under
     a parent, Children on a parent — never both.
     Status is owned HERE (and mirrored one-line in FEATURES.md): story files derive
     theirs by following the FEAT-ID. Omit empty sections per artifact-format.md rule 6.
     Register: `full` per artifact-format.md rule 11. -->

# FEAT-{{XXX}} — {{feature_name}}

> Status: {{status}}  <!-- proposed | in-flight | delivered | retired -->
> {{status_line_detail}}
<!-- Status-dependent detail, one line:
     in-flight  → since {{date}} · owning spec: `.mochiko/specs/{{spec-slug}}/`
                  (or owning lane run, for feature-command delta work)
     delivered  → since {{date}}
     retired    → {{date}} · ruling: {{pointer}}
     proposed   → surfaced by {{spec-slug}} ({{date}}) — or, for a capability stub,
                  minted by /mochiko:feature ({{date}}) · marked `unrefined`
     Reconstructed entries add: `reconstructed-from-code ({{date}}) — first touching
     spec re-verifies extent` — cleared at that spec's acceptance.
     `unrefined` stubs carry name + one-breath hook only — no Extent, no Relations;
     only specify's derivation fills them and makes the entry selectable. -->

## Parent

<!-- Leaf under a parent only: one line pointing up. A flat entry omits this section. -->

- [FEAT-{{PPP}}]({{FEAT-PPP-slug}}.md)

## Children

<!-- Parent entries only: one line per leaf child. The parent is navigation + status
     roll-up, never built directly — plan/implement key to leaves. Roll-up: in-flight
     when any child is; delivered when all children were delivered at the time delivered
     was earned. Delivered is sticky: a later in-flight child rides as a Deltas line
     below, never regresses this status. -->

- [FEAT-{{CCC}}]({{FEAT-CCC-slug}}.md) — {{child_status}}

## Capability

{{capability_statement}}
<!-- 1–3 lines, the system's own language — what the system does, standing on its own
     without reference to any story or spec. More than ~3 lines of extent below usually
     means this leaf is two features — or a parent waiting to be minted. -->

## Extent

<!-- What is in, and what is notably NOT in — one line each. This is where stories
     sharpen the feature over time. Leaves and flat entries only — a parent's extent
     is its children. -->

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
     "extent grows by {{X}} — in-flight, {{spec-slug or lane-run}}"; on a delivered
     parent carrying a late child: "new child FEAT-{{YYY}} — in-flight, {{spec-slug or
     lane-run}}". Each delta names its spec or lane run; it folds at that work's
     acceptance landing, then leaves this list. -->

- extent grows by {{X}} — in-flight, {{spec_or_lane_run}}
