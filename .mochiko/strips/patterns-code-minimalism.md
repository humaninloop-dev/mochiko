# Strip notes — `skills/patterns-code-minimalism/`

Entry formats: `strips/README.md`. First strip of this skill. Wave context: [v0.64.0] entry —
guardrails-vs-detail **Wave 2** (editorial extension of the D4 cut line to the untested
primitives; design: `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`; build plan:
`.mochiko/benchmarks/guardrails-vs-detail/report/build-plan.md`, Wave 2 sketch).

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-code-minimalism/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any rung is ranked: **Read `schema.yaml` (this skill's own
  directory) raw, in full, as one declared first action.** The schema is the source of truth
  for this skill's binding rules, nested in six sections, each addressable by its section
  ID: `patterns-code-minimalism.sec.trigger` · `patterns-code-minimalism.sec.scope` ·
  `patterns-code-minimalism.sec.discipline` · `patterns-code-minimalism.sec.inputs` ·
  `patterns-code-minimalism.sec.disclosure` · `patterns-code-minimalism.sec.reserved`.
  Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
  `constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
  rule binds you to that file's or skill's procedure, referenced never restated; labels come
  from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 3 rules of
  `class: floor` are non-waivable. Before the first rung is ranked, state the floor count
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
  The floor pin: the 3 rules of
  `class: floor` are non-waivable. Before the first rung is ranked, state the floor count
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
ruled · Skill-schema wave-2 family doors ruled — the patterns-family door); census:
`.mochiko/brainstorms/skill-content-schema/census-patterns.md` §A (CM) + §B (CM rows 1–10).
Schema home: `plugins/mochiko/skills/patterns-code-minimalism/schema.yaml`. Minted IDs carry
the `patterns-code-minimalism.` prefix (omitted below). Map — census §B row → minted ID:
1 `fires-at-decomposition` · 2 `rung-disclosed-in-cycle-report` · 3
`card-criteria-are-the-floor` · 4 `not-for-routes` · 5 `rung-zero-read-before-rank` · 6
`stop-at-first-rung` · 7 `new-dep-not-a-rung` · 8 `adopt-first-binding-constraint` · 9
`lazy-not-negligent` · 10 `one-intensity`.
**Section distribution (patterns six-set, disclosed):** trigger {1} · scope {4} ·
discipline {3, 6, 8, 9, 10} · inputs {5} · disclosure {2} · reserved {7 — the
domain-registry ruling is a reservation: never auto-approved}. No `conditions:` block —
census §B's live-`when:` dimension list omits CM; the load-first block legally omits the
`when:` grammar sentence (wave-1 RCM-4 wave-wide ruling).
**Floor count 3 (rows 7, 9, 10), per the wave lead's OQ1 ruling:** the census summary
table's "4 floor" cell disagrees with its own row-grain enumeration (three `(floor)`
markers); the row grain wins, and the summary-cell correction is queued for the census §K
landing appendix. Row 3 stays must/constraint.
**Protected content homed in the `description:` (census J-P3):** the [v0.64.0] RETURNED
existing-code-slimming trigger lives in the frontmatter description, which the conversion
pins byte-identical (skill-pair criterion 7) — protected by construction; the audit should
read the pin as protected content, not as unprotected text.
**Disclosed one-line ruled repair (lead-approved OQ5):** the body's
`references/DOMAIN-DEPENDENCIES.md` citation dangled — this skill ships no `references/`
directory; the catalog lives at
`../authoring-constitution/references/DOMAIN-DEPENDENCIES.md`, and `new-dep-not-a-rung`'s
`pointer:` now carries the resolvable cross-directory path (census J-P8 class; the
checker's climb-out resolution covers it). Never a silent fix.
**Deleted as dedup / moved, no content loss:** `## When NOT to Use` (rows 3–4),
`## Rung zero — read before you rank` (row 5), `## The floor — lazy, not negligent`
(row 9), `## One intensity` (row 10), and `## Quality Checklist` (5 boxes, all mapped:
box 1 = row 2 · box 2 = row 6 · box 3 = row 5 · box 4 = row 9 · box 5 = rows 1/3) leave
the body as moves or mapped mirrors. The Ladder's seven rungs stay body prose as the
sequenced procedure (D3); its stop-rule lead-in moved (row 6), and rung 5 lost its moved
parenthetical + carve (rows 7–8, entries below). Overview slims to identity + sibling
voice; its firing-site and disclosure sentences moved (rows 1–2).
Accounting: body 3,795 → 2,235 (obligations out + the load-first Rules block in) + schema
5,789 = **payload 8,024** (census §F estimate ~7,180); the delta over the pre-conversion
body is structural overhead (IDs, keys, section scaffolding, reading grammar) — no content
growth claimed. The old 4,319 body budget is superseded by the conversion re-seed
(ledger's third seeding path, no headroom — the wave closer executes the ledger row).

## [v0.102.0] PT-D1–D10 core — stop rule, disclosure duty, grading routing: protection transfers (census §A CM row 1)
- **Disposition:** superseded — protection transfers to `patterns-code-minimalism.stop-at-first-rung` (must), `patterns-code-minimalism.rung-disclosed-in-cycle-report` (must, duty), and `patterns-code-minimalism.fires-at-decomposition` (must, gate), per D8/C4; the provenance sidecar carries the protected status. The seven-rung ladder itself stays body prose as sequenced procedure (D3).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows; lineage `DECISIONS.md` 2026-08-05 ponytail-concepts-integration PT-D1–D10 ×2).
- **Content:** "Stop at the **first rung that applies**. Descend only when the rung above genuinely does not apply — and be able to say why, in one line, when it doesn't." · "The chosen rung per task is disclosed in the cycle report's decomposition, where the verification seat grades it against this file (`mochiko:review-code-minimalism`)." · "The ladder fires at card decomposition (`mochiko:executing-tdd-cycle`, step 2), before the red phase — it governs whether and how much code exists, not how the code that exists is written."
- **Consumers assessed:** `mochiko:executing-tdd-cycle` (points here at the decompose step) and `mochiko:review-code-minimalism` (grades against this skill) reference the skill by name, never a section anchor — the ladder-as-standard contract is intact across the pair.

## [v0.102.0] Adopt-first binding-constraint carve + new-dependency reservation — protection transfers (census §A CM rows 2–3; [v0.91.0] design-time re-key)
- **Disposition:** superseded — protection transfers to `patterns-code-minimalism.adopt-first-binding-constraint` (must) and `patterns-code-minimalism.new-dep-not-a-rung` (floor, reservation), per D8/C4; rung 5's body prose loses the carve sentence and the new-dependency parenthetical.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; the carve's protecting ruling `DECISIONS.md` 2026-08-26 plan-stage-utility D1, wording ruled at the v0.91.0 wave).
- **Content:** "(Adding a NEW dependency is not a rung — it rides the domain-registry ruling where `references/DOMAIN-DEPENDENCIES.md` applies, and is never auto-approved.) A design-committed adopt-first choice reaches these cards as a binding constraint, not a rung to re-open — the design-time discipline is `mochiko:patterns-adopt-first`."
- **Kept deliberately:** rung 5's core line ("A dependency already in the manifest does this — use it.") stays body prose; the carve's wording survives verbatim-in-substance in the schema rule; the dangling reference path is repaired in the pointer per the map entry's OQ5 note.
- **Consumers assessed:** `mochiko:patterns-adopt-first` (its Siblings pointer cites this ladder by name, unchanged); `mochiko:patterns-plan-minimalism` rung 3 cites adopt-first on its own side.

## [v0.102.0] The floor (lazy-not-negligent) + one-intensity — protection transfers (census §A CM row 4)
- **Disposition:** superseded — protection transfers to `patterns-code-minimalism.lazy-not-negligent` (floor) and `patterns-code-minimalism.one-intensity` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; PT-lineage `DECISIONS.md` 2026-08-05).
- **Content:** "No rung ever sacrifices a floor obligation (security, testing, error/data-loss handling, observability — the project's asserted Essential Floor) **or accessibility** … the ladder deletes speculation, not safety." · "The ladder ships at one strength — there is no off/lite/full/ultra dial. Per-project variance rides the recorded-waiver machinery, never a mode line."
- **Consumers assessed:** `mochiko:review-code-minimalism` grades rung claims against this skill by name; the floor text moved whole, wording verbatim-in-substance.

## [v0.102.0] Remaining body obligations relocated (census §B rows 3 · 4 · 5)
- **Disposition:** relocated → `plugins/mochiko/skills/patterns-code-minimalism/schema.yaml`, per the map entry above (D3).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 3 "the card's acceptance criteria are the floor of what to build; the ladder decides how cheaply to meet them, never whether to meet them" · 4 the two When-NOT routes (green-phase minimum-code rule is `mochiko:executing-tdd-cycle`'s, later; architecture topology sizing is `mochiko:patterns-system-design`'s, upstream) · 5 "Trace the real flow of the code being touched before rung 1 … Brownfield touches ride `mochiko:brownfield-integration`".
- **Consumers assessed:** none restates these rules; the router's `patterns-code-minimalism` row describes the skill generically and stays true.

## [v0.91.0] Two sibling references re-keyed from plan-time to design-time — plan-stage retirement D1

- **Disposition:** superseded → "design-time sibling" and "the design-time discipline"; the
  adopt-first constraint arrives design-committed rather than plan-committed.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1; wording ruled by the wave lead
  2026-08-26).
- **Content (superseded fragments, verbatim — two sites):**

  1. Overview, the sibling pointer:

     ```
     this file (`mochiko:review-code-minimalism`). Its plan-time sibling
     `mochiko:patterns-plan-minimalism` runs the same discipline over the plan's design elements
     ```
  2. Rung 5, the adopt-first carve-out:

     ```
        plan-committed adopt-first choice reaches these cards as a binding constraint, not a rung
        to re-open — the plan-time discipline is `mochiko:patterns-adopt-first`.
     ```

- **Kept deliberately:** the two-ladders-one-discipline framing and the build-time/design-time
  altitude split it states; the rung-5 carve-out in full — a committed adopt-first choice reaches
  the cards as a **binding constraint, not a rung to re-open**. That carve-out remains exactly
  true under D1 mechanic (d): mechanic (d) opens a build-time firing site only for a commodity
  need the design phase **never ruled**, which is not a re-opening, and it routes to the user
  rather than to this ladder.
- **Budget:** body 3,794 → **3,795** against the 4,319 budget; description unchanged at 564
  against 705. Both inside.
- **Consumers assessed:** `mochiko:patterns-plan-minimalism` (re-scoped to the design phase this
  wave — the sibling framing now matches on both sides); `mochiko:patterns-adopt-first` (gained
  the build-time gate this wave, which this carve-out is consistent with, as noted above);
  `mochiko:review-code-minimalism` carried the parallel sibling line, re-keyed in the same wave.

## [v0.64.0] RETURNED: existing-code-slimming trigger added to the description
- **Evidence:** the Wave 2 M1 fire-rate probe (14-scenario blind routing spot-check, lead
  dispatch 2026-08-11) found S12 ("slim this function down — grew three abstraction layers")
  a real scope gap PRE-DATING the wave: the MUST clause is scoped to pre-code card
  decomposition, and no description gave an active refactor request a clean home (ambiguous
  against `review-code-minimalism`, which grades a diff). Minimal fix, one clause appended to
  the SHOULD list: "or when slimming existing code that grew unneeded abstraction layers."
  Description 497 → 564 chars (cap 1,536). Not a re-add of cut content — a new trigger the
  probe demanded. User-ruled 2026-08-11.

## [v0.64.0] Guardrails cut — When-to-Use removed, ladder/floor/checklist kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 3,710 → 3,455 chars, −255, −7%; description 1,023 → 497 chars):**
  - **Removed whole:** `## When to Use` — the three-bullet list ("Decomposing a cycle card into tasks — every prospective task gets a rung" · "Weighing write-new against reuse, stdlib, platform, or an installed dependency" · "Grading a diff's rung claims (the review skill cites this file as its standard)"). Each bullet restates the description's MUST/SHOULD triggers; no obligation lost (the decompose-time firing is stated in Overview + the description; the review-skill grading pointer survives in Overview).
  - Old description verbatim: "This skill MUST be invoked at build-time card decomposition, BEFORE any red-phase test is written — running the pre-code ladder over each prospective task: stop at the first rung that applies (does it need to exist at all · already in the codebase · standard library · native platform feature · installed dependency · one line · only then the minimum that works), with rung choices disclosed in the cycle report. SHOULD also invoke when the work involves \"should this code exist\", \"reuse before build\", \"stdlib first\", \"over-engineering\", \"YAGNI\", \"smallest change\", or when weighing writing new code against reusing, deleting, or skipping it. This is the single source of the ladder: producers apply it (mochiko:executing-tdd-cycle points here at the decompose step), and mochiko:review-code-minimalism grades against it. It governs the DECISION to write code — distinct from the green-phase \"minimum code to pass\" rule, which fires after that decision is made. No rung ever sacrifices a floor obligation or accessibility."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the epigraph ("The cheapest code is the code never written."), `## Overview` (the decompose-step firing + the review-skill grading pointer), `## When NOT to Use` (the green-phase / topology-sizing / card-scope boundaries), `## Rung zero — read before you rank`, `## The Ladder` (the seven rungs — the core decision content), `## The floor — lazy, not negligent` (the non-waivable floor + accessibility), `## One intensity` (the no-mode-dial rule), `## Quality Checklist`. The description keeps the MUST trigger, the seven-rung gist, the top trigger phrases, and the review-skill / green-phase sibling distinctions.
- **Protected-content reconciliation (MANDATORY):** no prior strip file existed; grep of git history found no `KEPT:` / protected / `DECISIONS.md`-traceable line for this skill. The `## When to Use` list was never a protected survivor. Nothing silently dropped.
- **Consumers assessed:** `commands/implement.md`, `agents/staff-engineer.md`, `skills/executing-tdd-cycle/SKILL.md` (+ `references/CYCLE-REPORT-FORMAT.md`), `skills/review-code-minimalism/SKILL.md`, router `skills/mochiko/SKILL.md` — all reference the skill by name (`mochiko:patterns-code-minimalism`); none links a removed section anchor. The ladder-as-standard contract that `review-code-minimalism` grades against and `executing-tdd-cycle` points to is intact (`## The Ladder`, `## The floor` untouched).
