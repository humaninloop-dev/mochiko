# Strip notes — `skills/patterns-plan-minimalism/`

Entry formats: `strips/README.md`. First entry for this skill.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-plan-minimalism/SKILL.md`. -->

<!-- Wave context: wave 6 of the CLI schema-delivery build (v0.107.0) — the end state. No schema
file ships in the plugin: the 20 files under `plugins/mochiko/schemas/` and the 30
`skills/*/schema.yaml` were deleted, and every delivery they served now has a CLI form. Ruling for
the [v0.107.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6, with
the `DECISIONS.md` 2026-09-05 row and that session's `wave6-plan.md`. Pre-edit verbatim text:
`git show 62aa99d:plugins/mochiko/skills/patterns-plan-minimalism/SKILL.md`. -->

## [v0.107.0] the Ladder heading's "live in the schema"

- **Disposition:** superseded → "are delivered by `mochiko-cli`"
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md`
  D9 wave 6; `DECISIONS.md` 2026-09-05)
- **Content:** "Rung by rung — the stop rule, the rung scopes, and the read duty live in the schema:"
- **Kept deliberately:** the rung-by-rung framing and all three named subjects — the stop rule, the
  rung scopes, and the read duty — as rules rather than prose in this body. The five rungs beneath
  it are untouched.

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any rung is claimed: **Read `schema.yaml` (this skill's own
  directory) raw, in full, as one declared first action.** The schema is the source of truth
  for this skill's binding rules, nested in six sections, each addressable by its section
  ID: `patterns-plan-minimalism.sec.trigger` · `patterns-plan-minimalism.sec.scope` ·
  `patterns-plan-minimalism.sec.discipline` · `patterns-plan-minimalism.sec.inputs` ·
  `patterns-plan-minimalism.sec.disclosure` · `patterns-plan-minimalism.sec.reserved`.
  Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
  `constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
  rule binds you to that file's or skill's procedure, referenced never restated; labels come
  from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 2 rules of
  `class: floor` are non-waivable. Before the first rung is claimed, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire.
- **Consumers assessed:** none shared — the block was this skill's own text, and this family
  ships no common file.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the 2 rules of
  `class: floor` are non-waivable. Before the first rung is claimed, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.102.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2B, patterns family)

Ruling for every entry below: skill-content-schema D3 (obligations move, procedure stays
prose) / D8/C4 (protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema
ruled · the patterns-family door); census:
`.mochiko/brainstorms/skill-content-schema/census-patterns.md` §A (PM) + §B (PM rows 1–10).
Schema home: `plugins/mochiko/skills/patterns-plan-minimalism/schema.yaml`. Minted IDs
carry the `patterns-plan-minimalism.` prefix (omitted below). Map — census §B row →
minted ID: 1 `three-firing-sites` · 2 `not-for-routes` · 3 `rung-1-never-deletes` · 4
`stop-at-first-failing-rung` · 5 `rung-scope` · 6 `read-before-claim` · 7
`floor-both-ways` · 8 `disclosure-grammar` · 9 `grading-routing` · 10
`epic-joint-plan-one-plan`.
**Section distribution (patterns six-set, disclosed):** trigger {1, 10} · scope {2, 9} ·
discipline {3, 4, 5, 7} · inputs {6} · disclosure {8} · reserved {} (`rules: []` + note —
no user-reserved decision at census grain). No `conditions:` block — census §B's
live-`when:` dimension list omits PM; the load-first block legally omits the `when:`
grammar sentence (wave-1 RCM-4 wave-wide ruling).
**The rung-3 widening double-statement (lead-ruled OQ3):** the [v0.73.0] widening clause
rides `read-before-claim`'s schema text as the binding obligation, while the ladder's
rung-3 body prose keeps its procedural wording verbatim — the sanctioned D3
procedure-vs-obligation split (the ladder prose teaches, the schema rule binds);
protection anchors on the schema rule in the provenance sidecar.
**Deleted as dedup / moved, no content loss:** `## When NOT to Use` (row 2; its rung-1
carve bullet mirrors row 3), the Ladder's stop-rule lead-in (row 4), `**Scope of each
rung:**` (row 5), `**Read before you claim:**` (row 6), `## The floor — lazy, not
negligent` (rows 3/7), `## Disclosure grammar` (row 8), Overview's firing-sites paragraph
(rows 1, 9, 10), and `## Quality Checklist` (5 boxes, all mapped: box 1 = row 8 · box 2 =
row 4 · box 3 = rows 3/7 · box 4 = row 6 · box 5 = row 5). The five rungs stay body prose
as the sequenced procedure (D3); the `## Sibling` section stays as identity voice.
Accounting: body 4,276 → 2,812 (obligations out + the load-first Rules block in) + schema
5,782 = **payload 8,594** (census §F estimate ~7,660); structural overhead only, no
content growth claimed. PM was unbudgeted (hard-cap-only); the conversion re-seed is its
first budget row (ledger's third seeding path, no headroom — the wave closer executes the
ledger row).

## [v0.102.0] Stop-at-first-failing-rung + rung-scope rule — protection transfers (census §A PM row 1)
- **Disposition:** superseded — protection transfers to `patterns-plan-minimalism.stop-at-first-failing-rung` (must) and `patterns-plan-minimalism.rung-scope` (must), per D8/C4; the five-rung ladder itself stays body prose (D3).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows; birth lineage `DECISIONS.md` 2026-08-12 plan-structure-yagni + `DECISIONS.md` 2026-08-13 architect-role restructure).
- **Content:** "Per design element (component, entity, contract, mechanism, flow, constraint), stop at the **first rung that fails**, before it enters the package. Descend only when the rung above genuinely holds, and say why in one line when it doesn't." · "**Scope of each rung:** rungs 1, 4, 5 apply to every element without exception; rungs 2, 3 apply to design elements — shape and reuse are design judgments."
- **Consumers assessed:** `mochiko:review-plan-artifacts` grades rung claims against this skill by name; nothing quotes the moved lead-in.

## [v0.102.0] Rung-3 widening (adoptable proven component) — protection transfer into the read-before-claim rule (census §A PM row 2)
- **Disposition:** superseded — protection transfers to `patterns-plan-minimalism.read-before-claim` (must, duty), whose text carries "An adoptable proven component (per `mochiko:patterns-adopt-first`) satisfies rung 3's 'already exists'", per D8/C4 and the census §A disposition. The ladder's rung-3 body prose keeps the widened wording verbatim as procedure (the map entry's OQ3 note).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; the widening's protecting ruling `DECISIONS.md` 2026-08-15 build-vs-off-the-shelf D5 as amended, Build Surface item 3).
- **Content:** "3. **Already exists?** — a baseline, the current system, an installed dependency, or an adoptable proven component (per `mochiko:patterns-adopt-first`) carries it: extend, reference, or adopt — never re-design." (stays in the body ladder verbatim; the obligation limb now binds from the schema rule)
- **Kept deliberately:** the rung's number and position (the [v0.73.0] entry below chose widening over renumbering); the "never re-design" prohibition, in both homes' wording.
- **Consumers assessed:** `mochiko:patterns-adopt-first` ("its rung 3 reads adoption as a way something already exists" — still true on both surfaces); the description's rung-list summary unchanged (byte-identical description).

## [v0.102.0] Three firing sites + epic joint plan — protection transfers (census §A PM row 3; [v0.91.0] re-scope)
- **Disposition:** superseded — protection transfers to `patterns-plan-minimalism.three-firing-sites` (must) and `patterns-plan-minimalism.epic-joint-plan-one-plan` (must), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; protecting ruling `DECISIONS.md` 2026-08-26 plan-stage-utility D1/D4, wording ruled at the v0.91.0 wave).
- **Content:** "It fires at three generation-time sites inside the implement run's **design phase**: **what the design phase authors** — scoped to the sufficiency gap list, signed by the user at the design checkpoint — **each producing seat's plan**, and the **epic joint design-phase plan** (one plan over all members)."
- **Consumers assessed:** the router's row names the three surviving sites (re-keyed at v0.91.0, unchanged now); `mochiko:authoring-epic` carries the joint design-phase plan the third site names.

## [v0.102.0] The floor both ways + disclosure grammar — protection transfers (census §A PM row 4)
- **Disposition:** superseded — protection transfers to `patterns-plan-minimalism.rung-1-never-deletes` (floor), `patterns-plan-minimalism.floor-both-ways` (floor), and `patterns-plan-minimalism.disclosure-grammar` (must, duty), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; birth lineage `DECISIONS.md` 2026-08-12/13).
- **Content:** "Rung 1's 'required' reads **ratified requirements AND asserted floor obligations** … the floor never enters the package through imagination, and never leaves it to reach a cheaper rung." · "`<element> — rung N (one-line why)` … An undisclosed element reads as rung-skipped at review."
- **Consumers assessed:** `mochiko:review-plan-artifacts`'s rung-honesty lane grades the same disclosure grammar it always graded — wording verbatim-in-substance in the schema.

## [v0.102.0] Remaining body obligations relocated (census §B rows 2 · 9)
- **Disposition:** relocated → `plugins/mochiko/skills/patterns-plan-minimalism/schema.yaml`, per the map entry above (D3).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 2 the two When-NOT routes (build-time code — the code-shaping ladder is `mochiko:patterns-code-minimalism`, downstream; a delta-scope run — no design phase fires by default, its deliverable the desk-confirmed delta card, already minimal) · 9 "`mochiko:review-plan-artifacts` grades them at review — rung-honesty advisory, gap-list conformance blocking".
- **Consumers assessed:** none restates these rules; the router's row stays true.

## [v0.91.0] Fix round 3 — the last surviving "proposed" whose object died (advisory)

- **Disposition:** superseded → "Every **authored** artifact and major element carries a
  disclosed rung stop".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4**, the dead-gates ruling that
  retired the proposal). Raised as an **advisory** by the wave audit: "proposed" was the last
  word in this skill still presupposing a proposal gate — with the artifact gone, an element is
  *authored* by the design phase, never *proposed* for approval.
- **Content (superseded fragment, verbatim):**

  ```
  - [ ] Every proposed artifact and major element carries a disclosed rung stop
  ```

- **Kept deliberately:** the checklist item's whole force — **every** artifact and major element
  carries a disclosed rung stop, and the four checklist items beside it (no element past a rung
  it fails; rung-1 "required" honored both ways; rung-2/3 claims backed by a real read; rungs
  1/4/5 to all elements, 2/3 to design elements). One word changed; the coverage obligation is
  identical.
- **Budget:** unbudgeted (hard-cap-only). Body 4,276 → **4,276** (one-word swap of equal length);
  description unchanged at 600, inside the 1,536 cap.
- **Consumers assessed:** `mochiko:review-plan-artifacts` grades these disclosed rung stops
  (re-scoped to the gap list in the main pass) and its own rung-honesty lane never used
  "proposed"; the router's row for this skill was re-keyed to the three surviving firing sites in
  fix round 1. Nothing in the library now presupposes the proposal gate.

## [v0.91.0] Fix round — the authoring-proposal artifact and the architect's contest site DIE as firing sites (V1)

- **Disposition:** superseded → the three generation-time sites become **what the design phase
  authors** (scoped to the sufficiency gap list, signed at the design checkpoint), **each
  producing seat's plan**, and the **epic joint design-phase plan**.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4**, which lists plan's
  plan-the-plan proposal approval among the **dead gates**, and D1 mechanic (b) for the epic
  site). Raised by the v0.91.0 wave audit as **V1**; wording ruled by the wave lead 2026-08-26.
- **This entry corrects this seat's own main-pass work, stated plainly:** the main pass re-keyed
  "the lead's plan-the-plan proposal" to "the lead's **authoring proposal** (what the user signs
  at the design checkpoint)" and kept "the principal-architect's adversarial contest **of that
  proposal**". That preserved a gate D4 had killed. The proposal artifact does not survive the
  retirement in any form: the **sufficiency gap list** is the scope contract and the **design
  checkpoint** is the user gate. With no proposal, the contest of it has no object, so that
  firing site dies with it.
- **Content (superseded fragments, verbatim — four sites, all main-pass v0.91.0 text):**

  1. `description:` clause: `the design phase's authoring proposal inside `/mochiko:implement`, each producing seat's plan, the principal-architect's contest, and any design-artifact decision`
  2. Overview:

     ```
     It fires at three generation-time sites inside the implement run's **design phase**: the
     lead's **authoring proposal** (what the user signs at the design checkpoint), **each producing
     seat's plan**, and the **principal-architect's adversarial contest** of that proposal, before
     the user signs.
     ```
  3. When NOT to Use: `- **A delta-scope run** — no authoring proposal fires; its deliverable is the desk-confirmed`
  4. Disclosure grammar: `The lead discloses at the authoring proposal, each seat in its plan, the contest in its cuts.`

- **Kept deliberately:** the count of **three** firing sites (the third is now the epic joint
  design-phase plan, not the contest), all five rungs, the floor's both-directions carve-out, the
  rung-2/3 read obligation, the one-line-per-element disclosure grammar and its
  undisclosed-reads-as-rung-skipped rule, the `review-plan-artifacts` grading split
  (rung-honesty advisory / gap-list conformance blocking), and the whole Quality Checklist.
  The delta-scope carve-out survives with its reason restated in surviving terms — no design
  phase fires by default, and the desk-confirmed delta card is already minimal.
- **Budget:** unbudgeted (hard-cap-only). Body 4,274 → **4,276**; description 572 → **600**,
  well inside the 1,536 delivery cap.
- **Consumers assessed:** the router's `patterns-plan-minimalism` row and `principal-architect`
  row both carried the dead proposal/contest and were fixed in the same round (strip
  `.mochiko/strips/mochiko.md` [v0.91.0] fix-round entry); `mochiko:authoring-epic` carries the
  joint design-phase plan this skill's third site now names; `mochiko:patterns-entity-modeling`
  and `mochiko:patterns-api-contracts` cited "the design-phase proposal" in their ladder
  blockquotes — an artifact this entry retires — so both were **fixed in this same round** to
  "disclosed by the design phase as it authors", matching the surviving disclosure grammar. Their
  own strips carry the entry; nothing in the library now names the dead proposal.

## [v0.91.0] `description:` re-keyed from plan time to the design phase — plan-stage retirement D1

- **Disposition:** superseded → the same description firing at a design decision: the design
  phase's authoring proposal inside `/mochiko:implement`, each producing seat's plan, the
  principal-architect's contest, and any design-artifact decision.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 carry-overs: "plan-minimalism
  ladder governs what the design phase authors").
- **Content (superseded text, verbatim):**

  ```
  This skill MUST be invoked at plan time — the lead's plan-the-plan proposal, each producing seat's plan, and the principal-architect's contest — running the simplest-execution ladder over every package element (stop at the first failing rung: required · simpler shape · already exists · minimum now · builder's room), disclosed rung-wise. SHOULD also invoke on 'plan minimalism' or 'is this artifact needed'. Single source of the plan ladder; plan-time sibling of `mochiko:patterns-code-minimalism`.
  ```

- **Budget:** description-class edit, measured with the canonical snippet
  (`.mochiko/memory/primitive-cost-budgets.md`): **499 → 572 chars**. The skill is deliberately
  unbudgeted (hard-cap-only, ≤1,536 delivery cap) — well inside it. Body 4,159 → 4,274 (+115),
  likewise hard-cap-only.
- **Kept deliberately:** the five rung names in order, the stop-at-first-failing-rung rule, the
  rung-wise disclosure obligation, both SHOULD trigger phrases (unchanged so existing routing
  keeps firing), and the single-source claim plus the `patterns-code-minimalism` sibling
  pointer. Only the firing site and the "plan"/"package" vocabulary moved.
- **Consumers assessed:** the router's design-phase-cluster row (re-keyed same wave),
  `mochiko:review-plan-artifacts`'s rung-honesty lane (re-scoped same wave),
  `mochiko:patterns-adopt-first`'s Siblings pointer (unchanged — it cites rung 3 by number,
  which did not move).

## [v0.91.0] Body re-scoped to the design phase — plan-run and proposal-gate wording superseded — plan-stage retirement D1

- **Disposition:** superseded → the same ladder governing the design phase's authoring proposal
  and any design-artifact decision.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1).
- **Content (superseded fragments, verbatim):**

  1. Overview, both paragraphs:

     ```
     Before an artifact or design element enters the plan, run the ladder: a ranked check that
     stops at the **first rung that fails**. It grades the *solution the plan commits the build
     to*, not the weight of the documents — thin documents are a consequence, not the test.

     It fires at three generation-time sites: the lead's **plan-the-plan proposal** (what the user
     approves), **each producing seat's plan**, and the **principal-architect's adversarial
     contest** of the proposal, before the user approves. Each discloses a rung stop per element,
     and `mochiko:review-plan-artifacts` grades them at review — rung-honesty advisory, proposal
     conformance blocking.
     ```
  2. When NOT to Use:

     ```
     - **A delta-scope plan run** — no proposal gate fires; its deliverable is the confirmed delta
       card, already minimal.
     ```
  3. The Ladder lead-in: `Per package element (component, entity, contract, mechanism, flow, TR), stop at the **first`
     `rung that fails**, before it enters the plan.`
  4. Rung 1: `not enter the plan. Strict: no glue exception`
  5. Rung 5: `**Builder's room** — the plan states WHAT plus its binding constraints`
  6. Scope-of-each-rung: `rungs 1, 4, 5 apply to every element, TRs included`
  7. The floor: `the floor never enters the plan through imagination`
  8. Disclosure grammar: `The lead discloses at the proposal, each seat in its`

- **Kept deliberately:** all five rungs verbatim, the floor section's both-directions carve-out,
  the rung-2/3 read-before-you-claim obligation, the disclosure grammar's one-line-per-element
  form and its undisclosed-reads-as-rung-skipped rule, the sibling section, and the whole
  Quality Checklist. Three sites additionally lost the dying `TR` vocabulary (fragments 3 and 6)
  per D3, which retires the TR-XXX layer: "TR" in the element list became "constraint", and
  "TRs included" became "without exception" — the rung-scope rule itself is unchanged.
- **Consumers assessed:** `mochiko:review-plan-artifacts` (grades the rung claims; re-scoped to
  the gap list in the same wave), `mochiko:review-feasibility` hunt class 7 (unchanged),
  `mochiko:authoring-technical-requirements`'s ladder blockquote (re-keyed same wave).

## [v0.73.0] Rung 3 widened beyond the repo — an adoptable proven component now satisfies "already exists"

- **Disposition:** superseded → the same rung, rewritten in place; the adoption discipline itself
  is single-sourced at `mochiko:patterns-adopt-first` (new sibling skill, this wave).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-15 build-vs-off-the-shelf
  row; record `.mochiko/brainstorms/build-vs-off-the-shelf/record.md`, D5 as amended at Q5a/Q5b,
  Build Surface item 3 — "rung 3 text widened … no renumbering").
- **Content (superseded text, verbatim):**

  ```
  3. **Already exists?** — a baseline, the current system, or an installed dependency carries
     it: extend or reference, never re-design.
  ```

  Replaced by:

  ```
  3. **Already exists?** — a baseline, the current system, an installed dependency, or an
     adoptable proven component (per `mochiko:patterns-adopt-first`) carries it: extend,
     reference, or adopt — never re-design.
  ```

  The ruling's ground (record F7): the old rung was **repo-bounded** — "a baseline, the current
  system, or an installed dependency" — so a hand-built store, lock, and atomic-replace layer
  cleared every rung honestly while a proven off-the-shelf component sat outside the ladder's
  vocabulary entirely. The ladder could express *fewer* boxes, never *cheaper* ones. The tail
  gained "or adopt" for coherence: adoption is neither extending nor referencing, and a rung
  whose remedy list excludes its own new limb reads as an editing slip to the next auditor.
- **Kept deliberately:** the rung's number and position (rung 3, between *Simpler shape?* and
  *Minimum now*) — D5 chose widening over inserting a new rung expressly to avoid renumbering
  every rung citation across the library; the "never re-design" prohibition; the epigraph, the
  other four rungs, the scope-of-each-rung line, the *Read before you claim* line, the floor
  section, the disclosure grammar, and the Quality Checklist — all untouched by this edit.
- **Consumers assessed:** `commands/plan.md`, `skills/patterns-code-minimalism/`,
  `skills/patterns-map-minimalism/`, `skills/patterns-sound-loop/`, `skills/patterns-adopt-first/`,
  `skills/patterns-api-contracts/`, `skills/patterns-system-design/`, `skills/patterns-entity-modeling/`,
  `skills/authoring-technical-requirements/`, `skills/review-plan-artifacts/`,
  `skills/review-feasibility/`, `skills/review-code-minimalism/`, router `skills/mochiko/` — all
  reference the skill by name (`mochiko:patterns-plan-minimalism`) or by the ladder's rung-list
  summary; none quotes rung 3's text. The one semantic citation,
  `patterns-adopt-first/SKILL.md:106` ("its rung 3 reads adoption as a way something already
  exists"), was authored this wave against the widened text and agrees with it. The description's
  rung-list summary ("required · simpler shape · already exists · minimum now · builder's room")
  is unaffected — the rung's *name* did not change.
