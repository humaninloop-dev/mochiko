# Artifact Format — the mochiko deliverable envelope

The single authoritative home of the form every mochiko **deliverable** follows — the
pipeline artifact chain (`spec.md` · `requirements.md` · `constraints-and-decisions.md` ·
`nfrs.md` · `data-model.md` · `contracts/api.yaml` · `quickstart.md` · `plan.md` ·
`task-mapping.md` · `tasks.md` · `slices.md`) and setup's `codebase-analysis.md`. Artifact
templates and artifact-authoring skills reference this file for the shared rules; each
carries only its own section schema. (Reports are not deliverables — they follow
`report-format.md`. Brainstorm records and the governance surfaces are governed by their
own doctrine, not here.)

## Who reads a deliverable

Unlike a report (one consumer: the lead), a deliverable is read many times: by the human
at its acceptance gate, by every mandated cold-reviewer read, and by every downstream
producer — roughly ten model reads per feature. Every kilobyte is re-paid at each read.
So deliverables are **dense by construction and human-legible**: not machine-first
frontmatter, but no sentence that a field, a table row, or an ID citation could carry.

## Shared rules

1. **Reference by ID — never restate.** Downstream artifacts cite upstream IDs (`FR-003`,
   `TR-012`, `C-001`, `NFR-002`, `US-4`, `SC-005`) without re-quoting their text. A
   one-line gloss is allowed only where a bare ID would be unreadable at the point of
   use. Traceability is the ID link, not the quoted text.
2. **The ID index.** Every ID-bearing artifact opens with (or designates) a compact
   summary table — its **ID index** — enumerating the IDs it defines and what each maps
   to (e.g. a Traceability Summary, a Decision Summary, an Entity Summary, a Story→Cycle
   table). The ID index is the coverage surface reviewers verify against.
3. **The statement carries the content.** No `Description` field that re-explains the
   statement: a requirement is its one-to-two-line RFC-2119 sentence plus its structured
   fields (criteria, references); a constraint is the boundary stated as fact plus its
   impact list. Elaboration exists only where the statement genuinely cannot carry it.
4. **Size guidance.** Overview / context / rationale prose defaults to ≤ 3 lines;
   list entries (acceptance criteria, edge cases, scenarios, impacts, consequences) are
   one line each. These are defaults, not caps on substance — a genuinely complex
   decision may carry more, and the extra length should be substance, never padding.
5. **Table over prose.** Where content is enumerable (fields, mappings, statuses,
   checks), a table carries it. Prose is for judgment and rationale only.
6. **Omit empty.** A section or field with nothing to say is omitted, never written as
   "None" / "N/A" scaffolding.
7. **No doctrine in the artifact.** A deliverable never restates method, legends, or
   discipline its authoring skill single-sources (TDD rules, classification taxonomies,
   evaluation techniques, execution strategy) — cite the owning skill or reference
   instead. The one exception is rule 9's self-containment floor.
8. **Density is not a gap (the review rule).** Reviewers grade substance — coverage
   against the ID index, measurability, traceability, consistency — never prose volume.
   Brevity is never itself a finding; a gap is missing or unverifiable substance. An
   artifact conforming to this envelope is complete when its IDs, fields, and criteria
   are.
9. **Self-containment floor.** Keep in the artifact exactly what its gate-human or a
   downstream producer needs at the point of read — stated **once per document**, never
   once per item (e.g. a handling-defaults matrix appears once; per-item entries record
   only their specifics and deviations).
10. **Conditional deliverables record their null path.** A deliverable that is
    conditionally authored (e.g. `quickstart.md`) records its absence where a consumer
    would look for it (e.g. a one-line "not applicable — no external integration
    surface" in `plan.md`'s artifact table), so absence is a decision, never an
    oversight.

---

**Format version:** v1 (2026-07-24 — workflow-token-reduction wave 2) · **Governed by:**
the workflow-token-reduction epic record (D4 + the wave-2 rulings R1–R4/T1–T4) ·
**Consumed by:** the artifact templates in this directory, the artifact-authoring skills
(`authoring-requirements`, `authoring-user-stories`, `authoring-technical-requirements`,
`patterns-entity-modeling`, `patterns-api-contracts`, `patterns-vertical-tdd`,
`authoring-slices`, `analysis-codebase`), and the review-skill checklists that grade the
artifacts.
