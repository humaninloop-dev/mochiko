# Wave 3 — the budget table (for the user's ratification)

**Author:** staff-engineer seat, wave 3 · **Date:** 2026-09-13 · **Status: ratified 2026-09-15 with
the C1 amendment** (four rows moved; the rest as tabled) · **Ruling home:** record D6, D4a/D4f, OQ1 ·
**Keyed to:** `plugins/mochiko/templates/artifact-format.md` rule 4 and `report-format.md`,
as amended by the ADR `.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md` · **Built
into:** `plugins/mochiko/migrations/0005-artifact-homes.yaml`, re-keyed to the amendment.

Ratifying this table settles the per-section budgets a templated artifact carries, the whole-file
bound a template-less one carries, and where the answer is deliberately **no bound at all**. Nothing
here grades content: every row is a count. The observed column is the gap, never the source.

## The three keys, and where each one's text lives

- **A4p** — `plugins/mochiko/templates/artifact-format.md` rule 4 ("Size guidance"), as amended by
  the 2026-08-22 ADR, which carries no rule 4 of its own: "Overview / context / rationale prose
  defaults to ≤ 3 lines". Budget: 3 content lines + the `##` line + 2 blanks = **6**.
- **A4l** — the same rule: "list entries … are one line each". A list section's budget is a stated
  entry ceiling × 1 line + 3; the ceiling is the ADR's compact intent, never the corpus's count.
- **RE** — two sources: the ADR's watch line, "reviews should land at report-envelope scale (KBs,
  not tens of KBs)", and `report-format.md` rules 2–3, which close the prose set to three sections
  and make a clean report frontmatter-only. A report's prose section carries 15.
- **S** — the seat's call, no class behind it; the whole-file bounds are the only rows keyed so.

## Per-section budgets — the templated kinds

`observed` is the largest span the same rule finds in the corpus today (both repos, `##` to the next
`##`, nested content and fences in); **tight** marks a row today's worst file exceeds.

| kind | section | budget | key | observed | today |
|---|---|---|---|---|---|
| `spec` | `## Intent` | 10 | A4l | 14 | **tight** |
| `spec` | `## Overview` | 6 | A4p | 9 | **tight** |
| `spec` | `## Key Entities` | 15 | A4l | 15 | ok |
| `spec` | `## Screens & Flows` | 30 | A4l | 65 | **tight** |
| `spec` | `## Feature Selection` | 20 | A4l | 36 | **tight** |
| `spec` | `## Assumptions` | 10 | A4l | 14 | **tight** |
| `spec` | `## Open Questions` | 10 | A4l | 10 | ok |
| `tasks` | `## Overview` | 8 | A4l | 20 | **tight** |
| `tasks` | `## Cycle Format` | 6 | A4p | 1143 | **tight** |
| `feature-entry` | `## Capability` | 6 | A4p | 7 | **tight** |
| `feature-entry` | `## Extent` | 6 | A4p | 16 | **tight** |
| `feature-entry` | `## Work rows` | 20 | A4l | 19 | ok |
| `feature-entry` | `## Relations` | 20 | A4l | 26 | **tight** |
| `feature-entry` | `## Architecture` | 20 | A4l | 26 | **tight** |
| `feature-entry` | `## Story trace` | 20 | A4l | 9 | ok |
| `feature-entry` | `## Obligations` | 20 | A4l | 58 | **tight** |
| `governance-intent` | `## Fact profile` | 15 | A4l | 145 | **tight** |
| `governance-intent` | `## Project identity & type` | 10 | A4l | 48 | **tight** |
| `governance-intent` | `## Depth level declaration` | 6 | A4p | 16 | **tight** |
| `governance-intent` | `## Convergence skips` | 10 | A4l | 17 | **tight** |
| `governance-intent` | `## Real commands` | 12 | A4l | — | — |
| `governance-intent` | `## Floor expression & deck rulings` | 25 | A4l | 152 | **tight** |
| `governance-intent` | `## Minted principle intents` | 25 | A4l | 158 | **tight** |
| `governance-intent` | `## Waivers` | 25 | A4l | 17 | ok |
| `governance-intent` | `## Module selections` | 25 | A4l | 25 | ok |
| `governance-intent` | `## Domain-dependency seeds` | 10 | A4l | 18 | **tight** |
| `governance-intent` | `## Deliberate exclusions` | 10 | A4l | — | — |
| `governance-intent` | `## Review` | 25 | A4l | 167 | **tight** |
| `governance-intent` | `## Amendment Log` | 25 | A4l | 123 | **tight** |
| `codebase-analysis` | `## Part 1: Inventory (Factual)` | 40 | A4l | 117 | **tight** |
| `codebase-analysis` | `## Part 2: Assessment (Judgment)` | 40 | A4l | 62 | **tight** |
| `codebase-analysis` | `## Appendix: Detection Method` | 15 | A4l | 9 | ok |
| `report-envelope` | `## Failure narrative` | 15 | RE | 117 | **tight** |
| `report-envelope` | `## Notes of note` | 15 | RE | 314 | **tight** |
| `report-envelope` | `## Null-exit reasoning` | 15 | RE | — | — |
| `architecture-spine` | `## Container diagram` | 30 | A4l | 127 | **tight** |
| `architecture-spine` | `## Elements` | 40 | A4l | 92 | **tight** |
| `architecture-spine` | `## Key flows` | 30 | A4l | 221 | **tight** |
| `architecture-spine` | `## As-built notes` | 15 | A4l | 152 | **tight** |

## Deliberately unbounded — the `none` rows (OQ1, ruled R11)

A count the feature owns is not a verbosity lever, so these carry no budget and the log says so by
declaring the section with no `max_lines`:

| kind | section | why |
|---|---|---|
| `spec` | `## User Stories` · `## Edge Cases` · `## Functional Requirements` · `## Success Criteria` | the story, edge-case, FR and SC counts are the feature's |
| `tasks` | `## Cycle Cards` | the card count is the work's |
| `architecture-concerns` | every `AX-XXX` row | the concern count is the product's; the file carries no bound either |

`tasks`'s observed 1,143-line `## Cycle Format` is F10's drift, not a budget gap: cards written
under the wrong heading. R7 fixes the skeleton, and the 6 lines bind the prose it really governs.

## Whole-file bounds — the template-less deliverables

All keyed **S**: no document-level line figure exists in artifact-format.md or the ADR, so every
figure below is the seat's call rather than a derivation, and not read off the corpus either. **The
C1 amendment moved four rows**; the classes, not the individual files, are what it ruled on.

| bound | files |
|---|---|
| none | `record.md` · `synthesis.md` — a decision record is as long as the session it records, and the cold review grades it. Declared, never silent: the deliverable carries `bound_reason` and `mochiko-cli home` prints it |
| 300 | the template-less pipeline deliverables — `data-model.md` · `constraints-and-decisions.md` · `requirements.md` · `architecture.md` · `derivation.md` · `plan.md` · `design-closure.md` · `sufficiency-report.md` · `proposal.md` · `contest-brief.md` · `quickstart.md` · `build-order.md` · `screens-and-flows.md` · `manifest.md` — plus `wave<n>-<slug>.md`, and `baseline-delta.md` as before |
| 150 | the entry class — `US-<n>.md` · `<date-slug>.md` · `<AX-ID>.md` · `gates.md` · `README.md` · `selection-card.md` · `specs-index.md` · `FEATURES.md`. One row, one entry, one screen |
| per entry, 60 | `implement-log.md` and `build-log.md` — a `form: log` deliverable is bounded per `##` entry, never per file (D4d). One entry is one `##` block, which is how the entry span is measured |

The amendment does not close the corpus gaps, it halves the distance: `baseline-delta.md` 4,745
against 300 · `architecture.md` 2,373 · `requirements.md` 1,648 ·
`constraints-and-decisions.md` 1,437 · `data-model.md` 1,080. Every other file is under 840, and the
two now-unbounded records are out of the count entirely. Closing the rest is the violator pass's.

## Where size is not checked at all (`bounds: elsewhere`, D4f/V2)

Location and the closed file set still bind, and a bound template still shapes them; only the size
check is off, because the bound lives somewhere this schema does not own.

| home | bounds cite | why |
|---|---|---|
| `memory` | `.mochiko/memory/knowledge-management.md` | three of its six files are registries that must grow — the ledger, the trace manifest, the KM invariants |
| `brainstorms-index` · `specs-index` | `.mochiko/memory/knowledge-management.md` | an index's length is its entry count; the KM invariants hold its caps |
| `spec-contracts` · `feature-contracts` · `epic-contracts` · `product-contracts` | the contract's own interface, per `mochiko:patterns-api-contracts` | an interface document's length is the interface's; observed 76–2,151 lines |

## Heading grammar — what an undeclared `##` costs

| kind | undeclared `##` | note |
|---|---|---|
| `spec` · `tasks` · `feature-entry` · `codebase-analysis` · `architecture-spine` | **denied** | the declared set is the artifact's shape (D4a) |
| `governance-intent` | allowed | its section set is dimension-driven: the corpus carries `## Deployment & release reality` and `## Confrontation rulings (brownfield)`, neither enumerated by the template |
| `report-envelope` | allowed | one envelope serves six types; a type's payload sections ride its `by_type` template (D2/M8) |
| `features-index` · `governance-surfaces` · `architecture-store` · `architecture-concerns` | no shape rule | R9a/R9b: zero `##`, five surfaces, and a product-owned row set — never a shape invented from prose (D4f) |

`codebase-analysis` declares three heading overrides, its section names carrying an em dash where
its skeleton writes a colon. Four kinds carry a heading-less meta section, which governs none.

**The report type enum, a ratification item of its own.** `report:` is required in every report's frontmatter, and must be one of **cycle · verification ·
final-validation · review · feasibility · disclosure** — the six `templates/report-format.md`
already names, verbatim rather than widened. The enum is also the D9 sniff's vocabulary: a write
outside every home carrying one of these types is denied. `feature:` and `round:` stay conventional
and unchecked, since a brainstorm-session report has no feature.

## What ratification cost — ratified 2026-09-15 with the C1 amendment

**A change in kind, first.** Rule 4 calls these numbers *defaults, not caps on substance*, and the
ADR made an undisclosed overage an advisory finding a reviewer names. Under the gate they become
**caps**: a write past a budget is denied, and the hook has no channel for the disclosure that used
to ride with the artifact — past the budget that justification travels as a reviewer finding or as a
migration moving the number. The escapes are structural, not editorial: first-touch amnesty, which
needs the overage already on disk; `extra_headings: allow` where a kind declares it; and splitting
material into declared sections. A new artifact with an honestly long section has none of them.

Twenty-nine of the forty section rows are tighter than today's worst artifact, which is the ADR's
intent: kinako's EPIC-001 run wrote ~906KB of design-phase artifacts, the evidence it was ruled on.
Two mechanisms keep the bite survivable:

- **First-touch amnesty.** An `Edit` whose overage is already on disk and not made worse is allowed,
  with the standing overage named as context. Existing artifacts stay editable.
- **The gate is live from wave 4.** `0005` declares and the hook now enforces, so loosening a row
  costs a migration rather than an edit. The C1 amendment is exactly that move, taken before the
  first gated run rather than after it.

Four section rows a fresh run would still meet first, none of them touched by C1:
`governance-intent`'s `## Fact profile` (15 against 145 observed) and `## Review` (25 against 167),
both long by nature; `report-envelope`'s `## Notes of note` (15 against 314);
`architecture-spine`'s `## Key flows` (30 against 221). They stay as tabled, and each still moves
for one migration if a run proves the number wrong.

## The C1 amendment, as built

| row | was | now | why |
|---|---|---|---|
| `record.md` · `synthesis.md` | 150 | **no bound**, with `bound_reason` | the length is the session's; the cold review is the grader a line count cannot be |
| the fourteen template-less pipeline deliverables, and `wave<n>-<slug>.md` | 150 | **300** | the `baseline-delta.md` figure, applied to the class that writes at the same scale |
| the entry class, eight files | 150 | **150** | unchanged — one row, one entry, one screen |
| `build-log.md` | 150 whole-file | **`form: log`, 60 per entry** | a log grows by entries forever; bounding the file would wedge it at the first full one |

Declaring the absence of a bound took one crate change: a template-less deliverable may now carry
`bound_reason` in place of `max_lines`, `migrate validate` accepts it and rejects carrying both, and
`mochiko-cli home` prints the reason where a seat reads it. Verified end to end: a 602-line
`record.md` is allowed; a 301-line `wave<n>-<slug>.md` and a 301-line `data-model.md` are denied at
300; a 62-line `build-log.md` entry is denied at 60; `gates.md` still denies at 151.
