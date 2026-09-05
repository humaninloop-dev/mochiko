# Strip notes — `skills/patterns-model-tiering/SKILL.md`

Entry formats: `strips/README.md`. First entry (new file, created v0.78.0 — the skill was
born v0.77.0 as a pure addition).

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-model-tiering/SKILL.md`. -->

## [v0.108.0] description, tagline, and Overview widened to the worker rung

- **Disposition:** superseded → the reworded `description:` (bounded execution work joins the
  trigger; the worker tier and its two seats named; 'worker subagent' and 'offload to sonnet'
  trigger phrases added), the reworded tagline ("Every read and every bounded task rides the
  lowest tier where its result can be trusted."), and the reworded Overview sentence naming the
  two build-time seats' Sonnet worker and Sonnet's place in the economics line. The rule changes
  themselves are migration `plugins/mochiko/migrations/0004-sonnet-worker-rung.yaml` (two floor
  rewords under the anchor — `class-key-session-tier`, `override-is-the-pin` — two `must` rewords —
  `dispatch-ladder`, `disposable-per-gap` — three section rewords (`trigger` intent; `discipline`
  and `inputs` title + intent), one note reword, six mints, two of them floors) and take no strip: the prior text is in the log by construction.
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-09-05-sonnet-worker-rung.md`; `DECISIONS.md` 2026-09-05 row; user
  ruling 2026-09-05: the staff engineer and QA engineer may use Sonnet subagents to offload coding
  and verification tasks as they see fit).
- **Content:** verbatim —

  description (parsed value, 655 chars): `This skill MUST be invoked when dispatching exploration
  or fact-finding work in any mochiko run — routing each read by the class key.
  locate/enumerate/targeted-read gaps go to a native `Explore` subagent spawned with an explicit
  `model: haiku` override; interpretive reads, decision-driving absences, and
  completeness-sensitive enumerations stay on the session tier. SHOULD also invoke on 'model
  tiering', 'cheap explorer', 'which model', 'explore the code', 'targeted read', or 'fact-find
  dispatch'. Governs dispatch tier only — rostered seats never change model (model-tiered-seats
  D5); third sibling of patterns-sound-loop and patterns-transport-floor.`

  tagline: `**Every read rides the lowest tier where its answer can be trusted.**`

  Overview: `Rostered mochiko personas run on the strong tier and stay there; this floor governs
  the *reads they and the lead dispatch along the way*. The economics are documented, not
  assumed: Haiku is ~5× cheaper than Opus and ~10× cheaper than Fable per token both directions,
  and on subscription seats cheap-model work preserves Opus-cap headroom (model-tiered-seats D1).`

- **Kept deliberately:** the `## Rules — delivered by mochiko-cli` block — its halt clause, seven
  `!` lines, and read-back sentence — byte for byte; the description's D5 clause ("rostered seats
  never change model") — the ruling is dispatch-level, D5 stands, both personas stay `model: opus`;
  the Haiku economics figures, unchanged.
- **Consumers assessed:** the router row (`skills/mochiko/SKILL.md`, reworded in the same landing —
  entry in `strips/mochiko.md`); the six command floor lines and the two command-common blocks
  (`common.model-tiering`, `arch.model-tiering`) — unchanged, they point at the skill and never
  restate it; the ten persona `## Delegating Cheap Reads` sections — unchanged; `staff-engineer`
  and `qa-engineer` gain a sibling `## Delegating Bounded Work` section as a pure addition (rides
  the decision row, no strip); the contract suite's frozen floor set for this skill replaced by
  ruling (`evals/contract/expected-skills.json`, `floor_ids` 4 → 6 and `floor_pin` 4 → 6, every
  byte column untouched).

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any dispatch: **Read `schema.yaml` (this skill's own directory)
  raw, in full, as one declared first action.** The schema is the source of truth for this
  skill's binding rules, nested in six sections, each addressable by its section ID:
  `patterns-model-tiering.sec.trigger` · `patterns-model-tiering.sec.scope` ·
  `patterns-model-tiering.sec.discipline` · `patterns-model-tiering.sec.inputs` ·
  `patterns-model-tiering.sec.disclosure` · `patterns-model-tiering.sec.reserved`.
  Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
  `constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
  rule binds you to that file's or skill's procedure, referenced never restated; labels come
  from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 4 rules of
  `class: floor` are non-waivable. Before the first dispatch, state the floor count back — a
  skipped or partial read leaves that count blank: halt and surface it, and halt likewise if
  the schema's `class: floor` count disagrees with the pin.
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
  The floor pin: the 4 rules of
  `class: floor` are non-waivable. Before the first dispatch, state the floor count back — a
  skipped or partial read leaves that count blank: halt and surface it, and halt likewise if
  the schema's `class: floor` count disagrees with the pin.
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
`.mochiko/brainstorms/skill-content-schema/census-patterns.md` §A (MT) + §B (MT rows
1–11). Schema home: `plugins/mochiko/skills/patterns-model-tiering/schema.yaml`. Minted
IDs carry the `patterns-model-tiering.` prefix (omitted below). Map — census §B row →
minted ID: 1 `rostered-seats-never-retier` · 2 `cheap-rung-explore-haiku` · 3
`class-key-cheap-tier` · 4 `class-key-session-tier` · 5 `dispatch-ladder` · 6
`disposable-per-gap` · 7 `override-is-the-pin` · 8 `fact-finder-brief` · 9
`terse-return` · 10 `weak-negative-watch` · 11 `brief-obligation`.
**Section distribution (patterns six-set, disclosed; lead-ruled OQ4):** trigger {3, 4 —
the class key read as the lane key: which dispatch lane fires for each read class} ·
scope {1} · discipline {2, 5, 6, 7} · inputs {10} · disclosure {8, 9, 11} · reserved {}
(`rules: []` + note — the retier-deferral reservation rides row 1's text in the scope
section). No `conditions:` block — census §B's live-`when:` dimension list omits MT; the
load-first block legally omits the `when:` grammar sentence (wave-1 RCM-4 wave-wide
ruling).
**Both-rows citation (the census §A MT ceremony):** every move below cites BOTH
protecting rows — birth `DECISIONS.md` 2026-08-16 model-tiered-seats D1–D5 AS AMENDED by
the recorded [v0.78.0] supersession, `DECISIONS.md` 2026-08-19 explorer-retarget
(`.mochiko/decisions/2026-08-19-explorer-retarget-native.md`). The retarget wording — "a
spawn without the override inherits the session tier and has failed this floor" —
survives verbatim in `override-is-the-pin`.
**Deleted as dedup / moved, no content loss:** `## The class key` (rows 3–4), `## The
dispatch ladder` (row 5), `## Dispatch shape` (rows 6–10), `## The brief obligation`
(row 11), `## When NOT to Use` (row 1 + row 5's direct-tool-call rung + row 4's
never-tiered-down carve incl. its sound-loop cross-floor pointer — "the sound-loop floor
governs those seats" rides `class-key-session-tier`'s text with the `floor-pointer`
label, restored in the W2 fix round), `## Quality Checklist` (6 boxes, all mapped: box 1 = rows 3/4 ·
box 2 = rows 2/6/7 · box 3 = row 4 · box 4 = row 11 · box 5 = row 6 · box 6 = row 10),
and Overview's retarget-mechanics sentence (row 2). The D1 economics paragraph stays body
prose (teaching — documented, not assumed).
Accounting: body 4,785 → 1,614 (obligations out + the load-first Rules block in) + schema
7,179 = **payload 8,793** (census §F estimate ~8,460; re-measured after the W2 fix round
restored the sound-loop cross-floor pointer); structural overhead only, no
content growth claimed. MT was unbudgeted (hard-cap-only); the conversion re-seed is its
first budget row (ledger's third seeding path, no headroom — the wave closer executes the
ledger row).

## [v0.102.0] Class key, D5 roster fence, override pin, dispatch shape, brief obligation — protection transfers (census §A MT)
- **Disposition:** superseded — protection transfers per D8/C4 to `patterns-model-tiering.rostered-seats-never-retier` (floor), `patterns-model-tiering.class-key-cheap-tier` (must), `patterns-model-tiering.class-key-session-tier` (floor), `patterns-model-tiering.cheap-rung-explore-haiku` (must), `patterns-model-tiering.disposable-per-gap` (must), `patterns-model-tiering.override-is-the-pin` (floor, duty), `patterns-model-tiering.fact-finder-brief` (must, binding), `patterns-model-tiering.weak-negative-watch` (must), and `patterns-model-tiering.brief-obligation` (floor, duty); the provenance sidecar carries the protected status per rule.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows; protecting rows `DECISIONS.md` 2026-08-16 model-tiered-seats D1–D5 AND `DECISIONS.md` 2026-08-19 explorer-retarget — both cited on every move, per the census §A MT ceremony).
- **Content (decisive lines):** "rostered seats never change model … retiering a rostered seat is deferred to its own reliability-gated brainstorm (D5), never done ad hoc" · the cheap-tier and session-tier class-key lists, verbatim-in-substance · "the cheap rung is native `Explore` **with an explicit `model: haiku` override on the Agent tool call**" · "**The override is the pin** … a spawn without the override inherits the session tier and has failed this floor" (verbatim) · "Never a standing 'librarian' seat: a standing seat re-pays its transcript across gate pauses (D4/F5)" · the fact-finder brief constraints (terse spot-checkable facts with `file:line` provenance, verbatim quotes never paraphrase, absence reported method-scoped, no interpretation) · the weak-negative watch + re-route rule · "one line in every seat brief … a run whose seat briefs omit it has not applied this floor".
- **Kept deliberately:** the D1 economics paragraph (body prose, teaching); the bare-spawn-inherits-session-model fact, restated inside `cheap-rung-explore-haiku` as the reason the override is mandatory (the [v0.78.0] entry below kept it the same way).
- **Consumers assessed:** six command floor lines + ten persona sections + the router row point here by name (reworded at v0.78.0); none quotes a section anchor — all unaffected by the relocation.

## [v0.102.0] Remaining body obligations relocated (census §B rows 5 · 9)
- **Disposition:** relocated → `plugins/mochiko/skills/patterns-model-tiering/schema.yaml`, per the map entry above (D3).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 5 "Direct tool call → cheap `Explore` (haiku) → session-tier read … A spawn that costs more than the read it saves has failed the ladder." · 9 "The explorer returns the smallest decisive facts with `file:line` provenance; the bulk read stays inside the disposable context (the context-health test, D1)."
- **Consumers assessed:** none restates these rules; the router's row stays true.

## [v0.78.0] Cheap rung retargeted — `mochiko:explorer` superseded by native `Explore` + explicit `model: haiku` override

- **Disposition:** superseded → the reworked skill (same file): cheap rung = native
  `Explore` spawned via the Agent tool with an explicit `model: haiku` override; "The
  override is the pin" replaces "The frontmatter is the pin"; a new "Fact-finder brief"
  dispatch-shape clause carries the constraints the deleted explorer persona used to pin
  (terse `file:line`-provenanced facts, verbatim quotes, method-scoped absence, no
  interpretation).
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` rung failed on the transport doing most of the exploration.
- **Content:** superseded spans, faithfully compressed —
  - `description:` — "go to the cheap explorer seat (`mochiko:explorer`, model-pinned
    haiku)";
  - Overview — "Since Claude Code v2.1.198 the native `Explore` agent inherits the session
    model (Opus-capped), so 'just use Explore' is no longer cheap — the cheap rung is the
    plugin's own scoped seat: **`mochiko:explorer`**, its `model: haiku` pinned in
    frontmatter (D4)";
  - class-key heading — "dispatch `mochiko:explorer`, disposable per gap";
  - ladder — "Direct tool call → cheap explorer → session-tier read";
  - dispatch shape — "spawn `mochiko:explorer` via the Agent tool" and "**The frontmatter
    is the pin** — dispatching the scoped seat by name is what makes the read cheap; no
    per-spawn model parameter is needed or relied on";
  - brief obligation + checklist — "route ... to `mochiko:explorer`".
- **Kept deliberately:** the class key itself, the session-tier carve-outs (interpretive ·
  absence-driven · completeness-sensitive · producing/reviewing/grading never tiered down,
  D5), the dispatch ladder's three-rung shape, disposable-per-gap / no-librarian (D4/F5),
  the weak-negative watch, the brief obligation, the D1 economics paragraph, and the
  bare-spawn-inherits-session-model fact (restated as the reason the override is
  mandatory).
- **Consumers assessed:** six command floor lines + ten persona sections + the router row
  all point here and were reworded in the same v0.78.0 wave (their strip files carry the
  mirrored entries).
