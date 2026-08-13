<!-- Form: templates/artifact-format.md (the deliverable envelope). One file per capability:
     .mochiko/features/FEAT-XXX-<slug>.md. Authored and amended per
     mochiko:authoring-feature-map — the skill carries the derivation method, the work-row
     and fold rules, and the write timing; this template carries only the shape. An entry is
     a durable capability plus a block of transient work rows (its delivery increments);
     there is NO parent/leaf nesting — capabilities are flat, grouped by domains only at the
     soft cap (mochiko:patterns-map-minimalism). Status is owned HERE (and mirrored one-line
     in FEATURES.md): story files derive theirs by following the FEAT-ID. Omit empty sections
     per artifact-format.md rule 6. Register: `full` per artifact-format.md rule 11. -->

# FEAT-{{XXX}} — {{capability_name}}

> Status: {{status}}  <!-- proposed | in-flight | delivered | retired -->
> {{status_line_detail}}
<!-- Status-dependent detail, one line:
     in-flight  → since {{date}} · live work rows in run(s): `.mochiko/specs/{{spec-slug}}/`
                  (or the owning lane run, for feature-command growth-door work)
     delivered  → since {{date}} · sticky — live rows may still be visible below
     retired    → {{date}} · ruling: {{pointer}}  (or, on a merge: merged-into FEAT-{{YYY}})
     proposed   → surfaced by {{spec-slug}} ({{date}}) — or, for a capability stub,
                  minted by /mochiko:feature ({{date}}) · marked `unrefined`
     Reconstructed entries add: `reconstructed-from-code ({{date}}) — first touching
     spec re-verifies extent` — cleared at that spec's acceptance.
     `unrefined` stubs carry name + one-breath hook only — no Extent, no work rows;
     only specify's derivation fills them and makes the entry selectable. -->

## Capability

{{capability_statement}}
<!-- 1–3 lines, the system's own language — what the product does, standing on its own
     without reference to any story or spec. More than ~3 lines of extent below usually
     means this is two capabilities, or its bulk is undelivered work rows. -->

## Extent

<!-- What is in, and what is notably NOT in — one line each. This is where stories sharpen
     the capability over time, and where a delivered work row folds in at its landing. -->

- {{in_scope_line}}
- Not: {{out_of_scope_line}}

## Work rows

<!-- Transient delivery increments under this capability — one line each, newest first.
     Each row carries a state and its run pointer; a `live` row folds into Extent above at
     its acceptance landing and leaves this list, a `pending` row persists as open
     obligation (deferred work never silently disappears). Grammar:
       - `pending` — {{increment}} · acceptance: {{criteria}} · cut by {{spec-slug or lane-run}}
       - `live`    — {{increment}} · acceptance: {{criteria}} · in {{spec-slug or lane-run}}
     Story-shaped increments are fine here; a story-shaped *capability* is not. -->

- `live` — {{increment}} · acceptance: {{criteria}} · in {{spec_or_lane_run}}

## Relations

<!-- One line per relation to another FEAT-ID: depends-on / extends / composes-with.
     A work row may depend on another capability's row — note it here; row-level
     dependencies order the two capability-batch runs. -->

- depends-on: {{FEAT-ID}} — {{one_line_why}}

## Architecture

<!-- Which ARCHITECTURE.md components realize this capability — pointers only, navigable
     both directions; never restate the component view. (Domain → components mapping is
     dormant: it lands only at the soft cap, per mochiko:patterns-map-minimalism.) -->

- {{component_name}} (`ARCHITECTURE.md` § {{section}})

## Story trace

<!-- Accumulating provenance: which specs/stories informed this capability — IDs and
     pointers only, newest first. -->

- {{spec-slug}}: US-{{n}}, US-{{n}}

## Obligations

<!-- Deferred completeness pointers riding this entry until the work builds — one line each,
     distinct from work rows (which are cut delivery increments): deferred SC-XXX (travels
     here when its covering capability went unselected) · deferred seams ("when built,
     verify seam against FEAT-XXX") · cross-cutting extend obligations from stories homed
     elsewhere. -->

- {{obligation_line}}
