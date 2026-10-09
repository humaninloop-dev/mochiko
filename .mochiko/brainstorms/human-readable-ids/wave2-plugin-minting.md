# Wave 2 — the plugin: every minting rule writes the joined form

**Status:** planned 2026-10-08; build question B4 ruled; seats not yet dispatched (wave 1's crate
review comes first — W2-tests below writes crate tests only after wave 1's single-writer pen is
released).

Branch `human-readable-ids-build` (off `main` @ `bf410cc`). **Target version:** 0.118.0, MINOR (D20)
— the bump itself is wave 4, the user's. **Record:** `.mochiko/brainstorms/human-readable-ids/record.md`;
the decision cards are the source of every rule text — this plan names carriers, never restates a
decision.

## Build question B4 — ruled 2026-10-08

**Where the joined-ID writing rule lives.** Ruled, user ("as recommded", lead's lean A): **one copy,
in `templates/artifact-format.md`** — a new "IDs" section beside its rule 1 ("Reference by ID"),
the home nine authoring skills, the artifact templates and the review checklists already bind to.
Brainstorm's and setup's rules point at that section for session decisions and governance IDs.
Rejected: a new `templates/id-grammar.md` all three point at (a second pointer hop for every product
seat); three inline copies (one fix, three edits and audits). Accepted risk: the format file declares
brainstorm records and governance surfaces outside its envelope; they now cite one of its sections.
Ratified run: B1–B4, four in a row — the next fork goes in another form (case against first, weak
lean).

## What wave 2 builds — transcribed from the record

1. **The IDs section** (B4; D2, D5, D9, D10, D11 with B1, D12, D13 with direction words, D15's quote
   rule, D16, D19, D22; D4/D7 rename and rekey through `mochiko-cli ids`): the grammar and the
   citation rules, once. Rule 1's examples go joined. The format-version line moves.
2. **Minting rules, templates and example IDs per family** (D1, D3, D9, D19; D21's scope clause —
   minting grammar, placeholders, template definition lines and example IDs are joined; citations of
   decided IDs in rule text stay bare). Carriers, from F20 — the seats re-enumerate before planning:
   - `FR`, `SC` — `skill/authoring-requirements`, `template/spec` (SC settles on three digits, D9);
   - `US` — `skill/authoring-user-stories` and its `SKILL.md` heading form, `template/spec` index row;
     `stories/US-<n>.md` stays a bare path (D12);
   - `FEAT` — `skill/authoring-feature-map`, `command/feature`, `template/feature-entry`,
     `template/features-index`; the entry file's slug is the ID's slug (D12, D19 as changed at review);
   - `EPIC` — `skill/authoring-epic`; `.mochiko/epics/EPIC-XXX/` stays bare;
   - `AX`, `SPN`, `NFR` — `skill/authoring-architecture-store`, `template/architecture-concerns`,
     `template/architecture-spine`, `template/architecture-store`;
   - `C`/`D`/`IP`/`INT`/`DS` — `skill/authoring-technical-requirements` and
     `references/ARTIFACT-TEMPLATES.md`; `command/implement`'s baseline-entry grammar, and its landing
     renumber re-keys through `ids rekey` (D4 as changed at review);
   - `SCR`, `FLOW` — `skill/authoring-prototype`, `template/spec`; screen files keep their slug;
   - cycles `C<n>` — `template/tasks` and every rule that cites a cycle (`**Raised:**`, spine `Raised`);
   - `GI` — `template/governance-intent`, `template/governance-surfaces` (ledger heading joined;
     `<!-- GI-… -->` markers stay bare, D12), `command/setup` pointing at the IDs section;
   - session `D` — `command/brainstorm`'s card and namespace rules, pointing at the IDs section;
   - `GAP` — `templates/constitution-modules/evolution-notes.md`; `BR` —
     `patterns-entity-modeling/references/VALIDATION-RULES.md`.
3. **Graders check it** (D6, D16): every review skill paired with a minting skill checks the joined
   form against the IDs section — bareness, slug fit to the topic (D13), the definition-decides rule
   (D16). Which review skills: the seat enumerates the pairs.
4. **Report clarification label `C<n>` → `Q<n>`** (D10 as changed at review, S6):
   `templates/advocate-report-template.md` and every consumer that names the label.
5. **Scripts accept the joined form** (D12, F21): `validate-requirements.py`, `check-artifacts.py`,
   `validate-user-stories.py`.
6. **Ripple:** lines this wave makes false in the router, `README.md`, `docs/` — found by grep,
   reworded only where false. Citation back-fill of plugin prose is wave 3, not here.
7. **Tests and kits:** crate tests that freeze rule ids or counts (no crate source), `evals/contract/`
   expectations that assert the minted form (D14: they change with the plugin build), eval kits that
   freeze rule ids; wave 1's drift test re-checked against the new minting text.
8. **Ceremony:** strip entries for prose-primitive content superseded (schema content takes none —
   the log holds the prior text); `primitive-cost-budgets.md` re-measured; an unreleased
   `CHANGELOG.md` entry naming the break (D20). No `plugin.json` or `marketplace.json` bump.

## Hand-offs from wave 1 (P1's approved plan, revision 1)

- **H1 — minting wording the drift test reads.** A migration that rewords a minting rule keeps each
  row's `shows` substring and number form, or the family table changes through a crate change and a
  crate review: FR `three-digit padded, no gaps (FR-001` · SC `SC-XXX form` · US
  `` `stories/US-<n>.md` `` · FEAT `FEAT-XXX` · EPIC `` `EPIC-XXX` `` · AX `` `AX-XXX` (unique
  store-wide) `` · SPN `` `SPN-XXX` (unique store-wide) `` · NFR `` `NFR-XXX` targets live on the
  concern row `` · C-/D-/IP-/INT-/DS- `three-digit padded (C-, D-, IP-, INT-, DS-)` · SCR
  `` `SCR-XXX` `` · FLOW `` `FLOW-XXX` `` · GI "`sequential GI-001, GI-002`" · session D `` `D1…` `` ·
  cycle `**Depends on:** C1`.
- **H2 — definition lines keep the ID first.** After its leaders (`#…`, `- `/`* `, `[ ]`/`[x]`,
  `**`, `**Targets**:`, a first table cell `| `), the ID is the line's first token. This wave names
  the joined cycle heading (today `### - [ ] Cycle 1: …`).
- **H3 — the check's reach.** It reports only mentions it can tie to a definition (D18 as changed at
  build); an unresolved bare `C<n>` is a local label. The `C<n>` → `Q<n>` report-label change
  (item 4) still stands on its own.
- **Ordering.** The crate's drift test replays the working tree's log, so no wave-2 migration lands
  while wave 1 is building or under review; wave-2 seats may plan in parallel, read-only.

## Sound-loop wiring (floor tripped: plugin primitives are a governing surface)

- **Leg 1.** Three persona-less producers (`general-purpose`, `model: opus`, as the 2026-10-06
  precedent), each spawned plan-only and read-only; the lead snapshots the tree before and diffs
  after (a change is `dirty`); a fresh generic peer (`general-purpose`, `model: opus`) grades each
  plan default-FAIL on `mochiko:review-seat-plan`, the render pasted; only a PASS is approved and the
  same seat resumed. One re-plan round shared across the three; a second goes to the user.
- **Leg 2.** One gate grader for the wave (`general-purpose`, `model: opus`, explicit), the
  `mochiko:validation-primitive-edit` render pasted verbatim; it runs the pre-pass itself.
- **Leg 3.** The user gates commit, merge and the wave-4 bump. No git mutation by the lead.
- **Transport.** Subagents, disjoint ownership, every hand-off lead-relayed; every brief carries the
  routing line (locate and enumerate reads to a native `Explore` subagent spawned `model: haiku`).

## Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **W2-schema** | new migrations from `0046` · `.mochiko/schema-views/**` (re-emit only) · similar-rules allowlist rows the sweep reports | wave 1 closed |
| **W2-prose** | `templates/artifact-format.md` · prose templates and references named above · `SKILL.md` bodies the IDs section touches · the three scripts · strips · budgets · the `CHANGELOG.md` unreleased entry · ripple lines | W2-schema landed (budgets measure renders) |
| **W2-tests** | crate tests that freeze rule ids or counts · `evals/contract/` expectations · eval kits | W2-schema landed; wave 1's pen released |

Order: W2-schema plan, grade, build · W2-prose and W2-tests plan, grade, build in parallel · the gate
audit · wave 3.
