# Artifact Format — the mochiko deliverable envelope

The single authoritative home of the form every mochiko **deliverable** follows — the
pipeline artifact chain (`spec.md` · `constraints-and-decisions.md` ·
`data-model.md` · `contracts/api.yaml` · `quickstart.md` ·
`tasks.md`), setup's `codebase-analysis.md`, **and every command-minted deliverable a run
writes without a named template** — implement's design-phase plan, the
architecture/store delta, specify's `derivation.md`, and any epic-spine artifact. A
deliverable a command mints is in this envelope by default; escaping it requires a named
format home of its own. Artifact
templates and artifact-authoring skills reference this file for the shared rules; each
carries only its own section schema. (Reports are not deliverables — they follow
`report-format.md`. Brainstorm records and the governance surfaces are governed by their
own doctrine, not here — except the IDs section, which every surface that mints or cites a
durable ID follows.)

## Who reads a deliverable

Unlike a report (one consumer: the lead), a deliverable is read many times: by the human
at its acceptance gate, by every mandated cold-reviewer read, and by every downstream
producer — roughly ten model reads per feature. Every kilobyte is re-paid at each read.
So deliverables are **dense by construction and human-legible**: not machine-first
frontmatter, but no sentence that a field, a table row, or an ID citation could carry.

## Shared rules

1. **Reference by ID — never restate.** Downstream artifacts cite upstream IDs
   (`FR-003-csv-report-export`, `D-012-session-token-storage`,
   `C-001-existing-identity-provider`, `NFR-002-checkout-page-latency`,
   `US-4-task-status-filter`, `SC-005-checkout-completion-rate`,
   `SCR-004-account-settings-screen`, `FLOW-002-cart-checkout-path`) — written per the IDs
   section, behind their owner where cited outside their own file — without re-quoting their
   text. A one-line gloss is allowed only where the ID alone would be unreadable at the point
   of use. Traceability is the ID link, not the quoted text.
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
   Exceeding the guidance obliges **disclosure**: the producing seat carries one line in
   its report naming the delta and its reason (e.g. "overview 9 lines vs ≤ 3 default —
   multi-system migration context"). **Undisclosed or unjustified overage is gradeable**
   — an advisory finding under rule 8, never itself a FAIL.
5. **Table over prose.** Where content is enumerable (fields, mappings, statuses,
   checks), a table carries it. Prose is for judgment and rationale only.
6. **Omit empty.** A section or field with nothing to say is omitted, never written as
   "None" / "N/A" scaffolding.
7. **No doctrine in the artifact.** A deliverable never restates method, legends, or
   discipline its authoring skill single-sources (TDD rules, classification taxonomies,
   evaluation techniques, execution strategy) — cite the owning skill or reference
   instead. The one exception is rule 9's self-containment floor.
8. **Density is not a gap; excess is (the review rule).** Reviewers grade substance —
   coverage against the ID index, measurability, traceability, consistency — never prose
   *style*. **Brevity is never itself a finding**; a gap is missing or unverifiable
   substance. The other direction is now in scope: prose volume past rule 4's defaults
   **with no disclosed justification** is an **advisory finding** the reviewer names
   (section + delta) — advisory means it never alone blocks a verdict; the lead weighs
   it at the gate. An artifact conforming to this envelope is complete when its IDs,
   fields, and criteria are.
9. **Self-containment floor.** Keep in the artifact exactly what its gate-human or a
   downstream producer needs at the point of read — stated **once per document**, never
   once per item (e.g. a handling-defaults matrix appears once; per-item entries record
   only their specifics and deviations).
10. **Conditional deliverables record their null path.** A deliverable that is
    conditionally authored (e.g. `quickstart.md`) records its absence where a consumer
    would look for it (e.g. a one-line "not applicable — no external integration
    surface" in the run's sufficiency report), so absence is a decision, never an
    oversight.
11. **Register.** Deliverables write `full` — dense, articles droppable, fragments fine —
    under the never-compress list and the ambiguity guardrail: IDs, identifiers, commands,
    contract clauses and error strings stay verbatim, and compression stops wherever it
    would make a requirement, criterion or constraint ambiguous. A human reads these at an
    acceptance gate, so plain English wins wherever terse and plain pull apart. Levels, the
    clause manifest and the switch: `templates/output-style.md`.
12. **External claims disclose; they don't verify.** A deliverable asserting a floor-class
    external claim (version/capability, security posture, regulatory content, benchmark
    numbers) carries its disclosure line inline — `verified: <source>` or `memory-asserted`.
    That line is the producer's whole obligation; verification belongs to the reviewing
    skill, per the grammar's single source:
    [`review-brainstorm/references/EXTERNAL-CLAIMS.md`](../skills/review-brainstorm/references/EXTERNAL-CLAIMS.md).
13. **No process self-narration.** A deliverable never narrates its own creation, review
    history, sign-off state, or pass counts — provenance is **one header line** (author
    seat · date · the governing run); lineage, review verdicts, and gate outcomes live in
    the run's record, manifest, or report layer. A multi-line preamble about how the
    artifact came to be is rule-4 overage with no justification available — enforced
    through rule 8's advisory route, like all volume findings.

## IDs

Every durable ID a mochiko surface mints or cites is written by this section — in a deliverable,
a report, a brainstorm record or a governance surface. Rule 1 says when to cite; this section
says how the ID is written.

- **Durable IDs take a slug; local labels don't.** An ID cited outside the file that minted it is
  durable. Project-wide families: `GI`, `FEAT`, `EPIC`, `AX`, `SPN`, `NFR`, `GAP`. Per-artifact
  families and their scopes: `FR`, `SC`, `US`, `SCR`, `FLOW` (a spec); `C-`, `D-`, `IP-`, `INT-`,
  `DS-` (a constraints file); `BR` (a data model); session `D` (a record); cycle `C<n>` (a
  `tasks.md`). Bare: build tasks (`T3.2`), gates (`C3-gate-2`), run folders, report labels (`G1`,
  `Q1`, `A1`, `F1`), review survivors (`S1`), a record's own `F`/`Q`/`S`/`V` labels, and rule IDs
  (dotted slugs). The C4 model's level names (`C4 container`) are not IDs.
- **The joined form** is `<ID>-<w1>-<w2>-<w3>`. The number keeps its family's padding — three
  digits for the `-XXX` families (`FR-012`, `SC-001`, `C-001`), unpadded `US-<n>`, session `D<n>`
  and cycle `C<n>` — and a sub-ID's letter or dotted part attaches to the number (`D2a-…`,
  `D4.1-…`). Then exactly three lowercase ASCII words joined by hyphens, each starting with a
  letter; a hyphen run that is not three such words is not a slug. The three-word count binds
  new mints: a slug a file name already carries is the ID's slug at any length (file names,
  below). `C1-…` is a cycle; `C-001-…` is a constraint.
- **Slug words name the topic** — what the ID is about, never what it demands. No strength or
  verdict words (`must`, `should`, `never`, `may`): `FR-007-invoice-pdf-download`, not
  `FR-007-must-download-invoice`. A ruling that removes, retires, moves out or declines something
  carries its direction word, which counts toward the three (`D-014-no-message-queue`,
  `D2-billing-leaves-checkout`). The seat that mints the ID coins the slug: from the definition's
  existing name, or from the line's text where there is none (`FR`, `SC`, `EPIC`, `GAP`, `BR`).
- **Every single mention is joined, the definition included.** A definition line leads with its
  joined ID, after its leaders: `### D<n>-<slug> — <name>`, `- **FR-001-<slug>**:`,
  `| GI-0XX-<slug> |`, `# FEAT-XXX-<slug> — <name>`, `### - [ ] C<n>-<slug> — <title>`.
- **File names carry the ID's slug**: `FEAT-XXX-<slug>.md`, `concerns/AX-XXX-<slug>.md`,
  `scr-001-<slug>.html`. An existing file-name slug is the ID's slug at any length, and prefix
  case does not matter (`scr-001` is `SCR-001`). An epic's slug is its manifest heading's
  (`# EPIC-XXX-<slug> — <name>`). Renaming a slug moves the file.
- **Compounds.** A range stays bare (`D1–D7`, `FR-001–FR-009`). A slash pair or a list of up to
  three is written joined (`FR-003-csv-report-export / FR-004-report-email-delivery`); a list of
  four or more is a range-like summary and stays bare. An owner in front of the first member
  covers the compound.
- **Owners.** A per-artifact ID cited outside its owning file carries its owner's name in front,
  in a code span; project-wide families carry none. `FR`, `SC`, `US`, `SCR`, `FLOW`: the spec's
  slug (`` `lunch-orders` FR-012-order-cutoff-time ``). `C-`, `D-`, `IP-`, `INT-`, `DS-`:
  `` `product` `` for the product baseline, or the spec's slug for a spec's own file
  (`` `product` C-003-<slug> ``); `BR`: its data model's owner (`` `product` ``). A cycle: the
  run's key — the joined FEAT or EPIC ID, or `` `lane-<slug>` ``
  (`` `FEAT-012-tenant-workspace-admin` C3-<slug> ``); an artifact homed under a feature's or an
  epic's directory is owned by that joined ID. A session decision: the session's slug, an
  existing session's record directory name (`` `checkout-redesign` D4-<slug> ``).
- **A record link names a line's owner.** A bare session `D<n>` belongs to the one session that a
  link to its record on the same line names (`…/checkout-redesign/record.md`), when the line
  names no other session. A leading `` `slug` `` qualifies only the mention it stands before,
  never the line's other bare `D<n>`; a link or qualifier naming any other session makes the line
  name several, and every `D<n>` on it then carries its own qualifier. A mention's own qualifier
  always qualifies that mention.
- **Sub-decisions and clauses.** A dotted or lettered sub-decision that is its own decision takes
  its own slug after its full ID. A bracketed clause pointer such as `(ii)` is not an ID: cite the
  parent joined, then the clause (`` `checkout-redesign` D5-<slug> (ii) ``).
- **Spots a program reads stay bare:** migration `anchor:` lines, `<!-- GI-… -->` markers, and
  number-only paths (`.mochiko/features/FEAT-001/`, `.mochiko/epics/EPIC-XXX/`,
  `stories/US-12.md`). A link's target stays the path while its text is joined
  (`[US-1-<slug>](stories/US-1.md)`). Rule text `mochiko-cli` renders cites decided IDs bare, by
  ruling; its templates and minting rules show the joined form.
- **Quotes keep their form.** `mochiko-cli ids` never rewrites text inside a `"…"` quote
  (except in HTML files, where a quoted attribute is a link), a `>` blockquote or a code fence,
  including a quote that wraps across lines. A seat never re-slugs a
  verbatim quote of a source (the migration log, an archive, git output, a user's words): it
  keeps the source's form, bare where the source is bare.
- **The definition decides, and the number is the key.** A citation is joined with its
  definition's slug where the definition is joined, and bare where the definition is bare (a
  project's IDs from before the upgrade). Never coin a slug for an ID you cite: its slug comes
  from its definition — minted there, or renamed there through `ids rename`. Searches and checks
  match on the number within its scope (`GI-031`; `FR-012` within its spec; `D7` within its
  record), never on the joined string.
- **A slug follows its topic.** When the topic of the thing an ID names changes, rename it with
  `mochiko-cli ids rename <owning-file> <ID> <new-slug>` (the ID bare): read the preview, then run
  it with `--write`. Every mention outside the quote spans above follows (the tool skips those),
  and a slugged file name moves. If `--write` refuses because an old file name is still
  unrewritten somewhere — matched ignoring case, such as another spec's own link to a same-named
  screen — leave those foreign files out with `--exclude <path>`, re-run, then run
  `mochiko-cli ids --check <path>…` on the files you excluded; a spot that does mean the moved
  file is fixed by hand. A change of strength or wording within one topic renames nothing. A
  landing renumber goes through `mochiko-cli ids rekey <owning-file> <old-ID> <new-ID>` the same
  way; the slug travels with the entry. Templates spell the slug `<slug>`, or `{{slug}}` on a
  `{{…}}` line, and the seat fills it with three topic words.

---

**Format version:** v4 (2026-10-08 — the IDs section added, rule 1's examples joined. Record:
`.mochiko/brainstorms/human-readable-ids/record.md`, build question B4 with D1–D22) ·
**Consumed by:** the artifact templates in this
directory, the artifact-authoring skills (`authoring-requirements`, `authoring-user-stories`,
`authoring-technical-requirements`, `patterns-entity-modeling`, `patterns-api-contracts`,
`patterns-vertical-tdd`, `authoring-feature-map`, `authoring-prototype`, `analysis-codebase`), and
the review-skill checklists that grade the artifacts. The IDs section is also read by the
`brainstorm`, `setup` and `implement` rules, the `feature` and `architecture` desk rules, the
`authoring-epic` and `authoring-technical-requirements` minting rules, the review family's
`review-common.joined-ids`, and the `governance-intent` and `tasks` templates.
