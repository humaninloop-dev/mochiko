# Handoff Brief Template

This template defines the structure for a Handoff Brief — the destination-shaped deliverable of a
brainstorm session whose conclusion skips ahead of specification/design: either into task slicing
(destination `task-derivability`) or into a direct implement-and-verify pass (destination
`direct-execution`). The provenance header is what downstream consumers trust: it records the two
adversarial reviews this brief survived and the destination checklist it cleared.

---

```markdown
# Handoff Brief — {{topic}}

> Source session: {{brainstorm_workspace_path}}
> Destination: {{task-derivability | direct-execution}}
> Produced: {{date}}
> Provenance: digest challenged ({{challenge_rounds}} round(s), verdict {{digest_verdict}}) ·
> pressure-tested ({{pressure_rounds}} round(s), verdict {{pressure_verdict}}) ·
> destination checklist: {{checklist_name}} — all items evidenced

---

## Goal

{{one_paragraph_goal}}

## Key Decisions (from the session digest)

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| {{decision}} | {{choice}} | {{Confident | Contested | Deferred}} | {{why}} |

## Constraints — what must not break

- {{preserved_interface_or_compatibility_bound}}

## Affected Areas

| File / module | Change kind | Notes |
|---------------|-------------|-------|
| {{path}} | {{extend | modify | move | delete}} | {{note}} |

## Work Items

<!-- task-derivability: sliceable increments, each independently observable.
     direct-execution: typically ONE focused item. -->

### {{item_number}}. {{item_title}}

{{what_to_do}}

**Done when**: {{observable_completion_check}}

## Out of Scope

- {{tempting_but_excluded}}

## Verification

{{how_the_overall_result_is_proven_against_real_infrastructure — commands, checks, exit criteria}}

## Open Notes

{{contested_or_deferred_items_a_downstream_reader_must_know — "none" if empty}}
```

---

## Usage Notes

1. **Placeholders**: `{{mustache}}` style — replace each with actual content; this is a reference
   shape, not literal output.
2. **Provenance is load-bearing**: consumers accept this brief *because of* the recorded reviews
   and checklist. A brief without a completed provenance header carries no stamp and earns no skip.
3. **Destination decides the Work Items shape**: `task-derivability` briefs carry multiple
   sliceable increments (each with its own done-check); `direct-execution` briefs carry one focused
   change-set.
4. **Checklist alignment**: every section maps to an item in the corresponding
   `validation-brainstorm` readiness checklist — if a section is hard to fill, the pressure test
   would have failed it; go back rather than pad it.
