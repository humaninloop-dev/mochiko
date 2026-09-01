# Strip notes — `skills/patterns-vertical-tdd/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.22.0–v0.23.0] entries — workflow-token-reduction waves 1–2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6a + D4/wave-2 rulings).

## [v0.102.0] Converted to the `.md` + schema pair form — rule content relocated to `schema.yaml`; the standing +294 HOLDS overage superseded by the conversion re-seed (wave 2B, patterns family)

- **Disposition:** superseded → the pair form: obligation content relocated into
  `plugins/mochiko/skills/patterns-vertical-tdd/schema.yaml` (15 body rules + 3
  reference stubs — 5 floor · 13 must — under the patterns six-section set), the
  `SKILL.md` body keeping the letter/spirit epigraph and the Quality Checklist (the
  v0.27.0 KEPT live residue, census-patterns §A: body-stays-prose), the teaching prose
  (vertical-over-horizontal example, skeleton concept, verified-against-reality,
  heuristics, Case-column definitions, brownfield explanation), and the new "Rules —
  load the schema first" block (floor pin 5 + read-back). The frontmatter
  `description:` is byte-untouched. `references/TEST-GRAMMAR.md` and
  `references/BUNDLE-IDENTIFICATION.md` are untouched — the three TEST-GRAMMAR stubs
  point, never restate (skill-content-schema D3/C2).
- **Tier failed:** n/a — supersession by ruling (`skill-content-schema` D1–D9 as
  amended, `DECISIONS.md` 2026-09-01, D8/C4 supersession-transfer; the wave-2 patterns
  family-door ruling, same date). Protection transfers to the rule IDs via
  `.mochiko/provenance.yaml` (D8/C4), each anchored to its original protecting ruling.
- **Content (superseded body fragments, verbatim, with the relocation map; census §B
  rows in parentheses):**
  1. Overview output sentence with the protected v0.76.0 two-arm pointer — `The output
     is `tasks.md` in the cycle-card shape (the `tasks` schema is the canonical
     skeleton — invoke `mochiko-cli template tasks` when the binary is available;
     otherwise Read `plugins/mochiko/schemas/tasks.yaml` raw): per card — stories +
     rationale, dependencies, the named test-case list (the card's content), cycle-level
     brownfield exposure, and the Simple/Split/Merge case.` →
     `patterns-vertical-tdd.tasks-binding-two-arm` (row 1) — **both arms preserved
     verbatim in the rule text** (census J-P5, GI-020; the RPA two-arm precedent),
     promoted `class: floor` per the lead's plan-approval ruling reconciling census §B's
     table (5 floors) with §D ("5 floors incl. two protected two-arm/authorship rules");
     the §B detail line's `must` marker on this row is queued for the census §K
     build-corrections appendix at landing.
  2. Overview design-time paragraph — `This skill works at **design time inside the
     `/mochiko:implement` run** — after the design phase, or directly on a zero-gap
     sufficiency verdict; never a separate plan run. It decides the slicing and states
     what each cycle must prove. It writes no task lists — the builder decomposes each
     card into concrete tasks, with file paths, at build time with the code in view
     (`mochiko:executing-tdd-cycle`, downstream).` →
     `patterns-vertical-tdd.design-time-inside-implement-run` (row 2) +
     `patterns-vertical-tdd.writes-no-task-lists` (row 3, `class: floor`).
  3. Two-authors paragraph — `**Two authors, one card:** the design seat running this
     skill owns the **slicing judgment** — which bundles exist, Simple/Split/Merge,
     dependencies, the walking-skeleton call; the `qa-engineer` authors the **test-case
     content** (expected behaviour) in the grammar it later executes. The slicing seat
     is a **design seat, never the builder who will execute the card.**` →
     `patterns-vertical-tdd.two-authors-one-card` (row 4) +
     `patterns-vertical-tdd.author-never-executing-builder` (row 5, `class: floor` —
     the v0.91.0 mechanic-(c) rule).
  4. `## When NOT to Use` section whole (bug fixes · documentation-only or
     spike/research · decomposing a card into tasks · deriving or scoping features with
     the feature-map boundary) → `patterns-vertical-tdd.not-for-routes` (row 6). Not a
     KEPT survivor: the v0.64.0/v0.75.0 "kept deliberately" mentions record what those
     edits spared, not a survivor ruling; the census §A live residue is the epigraph +
     checklist only.
  5. Core Principle 1 closing line — `A card whose test cases cannot be demonstrated on
     their own is not a cycle.` → `patterns-vertical-tdd.vertical-over-horizontal`
     (row 7); the Wrong/Right example blocks stay body teaching.
  6. Core Principle 2 normative content — the new-end-to-end-path skeleton rule
     (`the **first cycle is a walking skeleton**: the thinnest end-to-end path through
     all layers with one trivial case green` · `Growth or delta work on an
     already-standing path **skips the skeleton**`) →
     `patterns-vertical-tdd.walking-skeleton-first` (row 8, `when:
     {new_end_to_end_path: present}` — the condition declared as the schema's one
     `conditions:` dimension); `There is **no foundation/feature card type**. …
     **Infra-only cards are never minted.**` (the v0.75.0 re-anchor floor) →
     `patterns-vertical-tdd.no-infra-only-cards` (row 9, `class: floor`);
     `Inter-card dependencies stay explicit; `[P]` parallel eligibility derives from
     dependencies, not from a type column.` →
     `patterns-vertical-tdd.parallel-derives-from-dependencies` (row 10).
  7. Core Principle 3 gate sentences — `Every card closes with a **`**TEST:**` gate**
     — the cycle's named test cases run against real infrastructure, never a re-run of
     the automated tests. This gate is the demonstration the cycle is anchored on; a
     cycle that stops at the mock boundary has proven nothing.` →
     `patterns-vertical-tdd.test-gate` (row 11, `class: floor`); `this skill owns it;
     downstream parsers consume it` →
     `patterns-vertical-tdd.grammar-owned-downstream-consume` (row 12).
  8. Case-column normative tail — `The story→cycle decision and its rationale live
     **on the card** (Stories line) — there is no separate mapping artifact.` (plus the
     two `(record the why, one line)` markers) →
     `patterns-vertical-tdd.case-and-rationale-on-card` (row 13); the Simple/Split/Merge
     definitions stay body teaching.
  9. Brownfield translation sentence — `Design-artifact brownfield markers (e.g. a
     data-model entity flagged as extending existing code) translate into the exposure
     line, so the classification survives design into the build.` and the exposure-line
     duty (`Each card carries a cycle-level exposure line: `none`, or the existing
     surfaces it extends/modifies.`) → `patterns-vertical-tdd.brownfield-exposure-line`
     (row 14).
  10. The per-case citation duty (checklist-carried; row 15) →
      `patterns-vertical-tdd.cases-cite-ids` — the checklist row itself stays (KEPT
      residue), the schema rule is the citable obligation.
  11. Reference stubs (census §B S1–S3; TEST-GRAMMAR.md untouched, its tables and
      examples staying): the four MUST-includes →
      `patterns-vertical-tdd.test-gate-must-includes` · the required-field set →
      `patterns-vertical-tdd.test-gate-field-set` · the producer-vs-runtime routing →
      `patterns-vertical-tdd.test-gate-runtime-classification-routing`.
- **Budget — the standing overage SUPERSEDED (census J-P5).** The ledgered **+294 HOLDS
  overage** (body 6,781 against the 6,487 budget: +68 through v0.80.0 + the +226
  mechanic-(c) growth ruled HOLDS at the v0.91.0 audit) **is superseded by this
  conversion's re-seed**: the budget re-seeds to the measured delivered-at-invoke
  payload (body + own schema, no +25% headroom) via the ledger's third seeding path, and
  the overage trail survives here and in the `primitive-cost-budgets.md` v0.80.0/v0.91.0
  paragraphs (GI-006 reconstruction). The wave-2B ledger row names this supersession.
- **Kept deliberately:** the letter/spirit epigraph and the Quality Checklist whole
  (v0.27.0 KEPT live residue — census §A body-stays-prose disposition) · all teaching
  prose named in the Disposition line · the `'vertical slice'` SHOULD-trigger in the
  description (the v0.80.0 ruling's explicit keep) · the description byte-identical
  (497 chars) · both `references/` files untouched.
- **Consumers assessed:** `executing-tdd-cycle` (consumes cards + owns build-time
  decomposition — boundary now `patterns-vertical-tdd.writes-no-task-lists` +
  `patterns-vertical-tdd.not-for-routes`, unchanged in substance) · `testing-end-user`
  (consumes the `**TEST:**` grammar — TEST-GRAMMAR.md untouched; the runtime-routing
  stub points at it, no wording change on its side) · `testing-gap-finding` (consumes
  the grammar boundary — untouched) · `review-plan-artifacts` (cycle-card check set —
  the obligations it grades against now carry schema IDs, substance unchanged) ·
  `implement.md` (dispatches this skill at design time — no section-anchor link) ·
  the router `skills/mochiko/SKILL.md` row (names the skill, unaffected) ·
  `brownfield-integration` (consumes the exposure line — unchanged). No shipped surface
  links a removed section anchor of this SKILL.md.

## [v0.91.0] Design-time card authoring re-homed inside the implement run; card author is never the executing builder — plan-stage retirement D1 (c)

- **Disposition:** superseded → the same design-time paragraph, now naming its home: inside the
  `/mochiko:implement` run, after the design phase or directly on a zero-gap sufficiency
  verdict, never a separate plan run.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 mechanic (c): "cards are authored
  by a technical-analyst-class design seat, never the builder who will execute them"; and D1's
  spine: "cycle cards + TEST cases are authored after the design phase (or directly on a
  zero-gap verdict), then build proceeds").
- **Content (superseded fragments, verbatim):**

  1. Overview, first line: `Transform a plan's stories into **cycle cards** — vertical increments`
  2. Overview, design-time paragraph opening:
     `This skill works at **design time**: it decides the slicing and states what each cycle must prove.`
  3. Brownfield exposure: `Plan-artifact brownfield markers (e.g. a data-model entity flagged as extending existing code)`
  4. Quality Checklist: `- [ ] Each named test case cites the spec/plan ID(s) it covers — never re-quoted content`

- **Budget — OVERAGE DECLARED.** Body measured with the canonical snippet: **6,555 → 6,781
  chars** against the recorded budget of **6,487** — a **+294 total overage**, of which +68 was
  already on the ledger from v0.80.0 and **+226 is this wave's**. Description unchanged at 497
  (budget 620). The growth is two ruled obligations from D1 mechanic (c) and nothing else:
  naming the card-authoring home (~+128) and the card-author-is-never-the-executing-builder rule
  (~+82), plus ~+7 across three vocabulary re-keys. No playbook prose was restored; a rationale
  clause drafted alongside the mechanic-(c) rule ("an executing builder authoring its own cards
  collapses the two altitudes this skill separates") was cut before landing precisely because it
  was prose rather than obligation. Justification offered to the audit: mechanic (c) is a new
  ruled obligation with no other home — this skill single-sources card authorship, so the rule
  either lives here or nowhere.
- **Kept deliberately:** the whole slicing craft — vertical-over-horizontal, walking-skeleton-
  first with its growth-work skip, the no-infra-only-cards rule, the `**TEST:**` real-
  infrastructure gate and its grammar ownership, Simple/Split/Merge with on-card rationale, the
  brownfield exposure line, and every Quality Checklist item. The two-authors split (design seat
  owns slicing, `qa-engineer` owns test-case content) survives intact and gained the mechanic-(c)
  qualifier rather than being rewritten.
- **Consumers assessed:** `mochiko:review-plan-artifacts`'s cycle-card check set (unchanged — it
  already forbids pre-written decomposition), `mochiko:testing-gap-finding` (consumes the
  `**TEST:**` grammar, untouched), `implement.md` (P1's rewrite carries the card-confirm
  checkpoint), the router's row (re-keyed same wave).

## [v0.80.0] Slice-as-unit vocabulary purged; `SLICE-IDENTIFICATION.md` renamed `BUNDLE-IDENTIFICATION.md`

- **Disposition:** superseded → "vertical increment" / "bundle" / "cycle" as the unit nouns, per the replacement language the ruling fixes. The reference file is renamed (`git mv`, history preserved), not deleted; every heuristic in it survives.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`, "Slice vocabulary purged from shipped primitives", ruled 2026-08-19; `DECISIONS.md` row of the same date). Ground: the slice died as a pipeline unit at v0.57.0 (`feature-map-layer` D4/D22) and as a TDD unit at v0.75.0 (D1's test-case-bundle anchor), but the noun outlived both units and a post-v0.75.0 dogfood run reified it — a fresh lead read the residue and minted slices alongside cycles.
- **Content (superseded, verbatim — old → new):**
  - `SKILL.md` frontmatter `description:` — `structuring a feature's implementation into vertical-slice cycle cards` → `structuring a feature's implementation into cycle cards`
  - `SKILL.md` Overview — `Transform a plan's stories into **cycle cards** — vertical slices that each deliver observable, testable value.` → `… — vertical increments that each deliver observable, testable value.`
  - `SKILL.md` Core Principle 1 close — `A card whose test cases cannot be demonstrated on their own is not a slice.` → `… is not a cycle.`
  - `SKILL.md` Quality Checklist — `- [ ] Cards are vertical slices (not horizontal layers)` → `- [ ] Cards are vertical increments (not horizontal layers)`
  - `SKILL.md` Identifying Cycles + Case column — both `[SLICE-IDENTIFICATION.md](references/SLICE-IDENTIFICATION.md)` links re-pointed to `[BUNDLE-IDENTIFICATION.md](references/BUNDLE-IDENTIFICATION.md)`
  - `references/SLICE-IDENTIFICATION.md` → `references/BUNDLE-IDENTIFICATION.md`, with its seven unit-noun uses purged: `# Slice Identification Heuristics` → `# Bundle Identification Heuristics` · `identifying good vertical slices — coherent bundles of named test cases` → `identifying good bundles — coherent bundles of named test cases` · `For each potential slice, ask` → `For each potential bundle, ask` · `| Yes, but needs other slices first |` → `| Yes, but needs other bundles first |` · `| No, it's internal refactoring | Not a slice; attach to a bundle |` → `| No, it's internal refactoring | Not a bundle on its own; attach to one |` · `line carries its story set and slice rationale` → `line carries its story set and bundle rationale` · `When unsure how to slice, use this matrix` → `When unsure how to bundle, use this matrix`
  - `references/TEST-GRAMMAR.md` "Why This Matters" — `- The "vertical slice" isn't actually vertical—it stops at the mock boundary` → `- The increment isn't actually vertical—it stops at the mock boundary`. **Vocabulary only.** `TEST-GRAMMAR.md` was deliberately untouched at v0.75.0 (D4, grammar ownership unchanged); this edit does not reopen that — no field, modifier, assert pattern, classification rule, or legacy marker is altered, and `patterns-vertical-tdd` still owns the grammar.
- **Kept deliberately:**
  - The `'vertical slice'` SHOULD-trigger phrase in the description — **explicitly kept by the ruling**: it routes a user who still reaches for the legacy term. Removing it would cost routing recall and buy nothing, since a trigger phrase names what the user says, not what the doctrine calls the unit.
  - Every *gerund* use, which names the activity and not a unit: the `# Vertical Slicing — Cycle Cards` title, "it decides the slicing", "the **slicing judgment**", "**Wrong** (horizontal slicing)" / "**Right** (vertical slicing)", and the reference's `### Horizontal Slicing` anti-pattern heading. The ruling purges the unit noun, not the word; a total ban was considered and rejected.
  - The reference's whole substance — value-stream test, walking skeleton, bundle grain, dependency analysis, worked example, anti-patterns, decision matrix — unchanged; this is a rename plus seven word swaps.
- **Measurements (D7 pre-assert, canonical snippet per `.mochiko/memory/primitive-cost-budgets.md`):** `SKILL.md` description **512 → 497** chars (budget 620) — shrank, well inside. `SKILL.md` body **6,543 → 6,555** chars against a 6,487 budget — **+12 from this edit, 68 over budget in total, and the overage is declared for the grader's ruling.** The +12 is arithmetically forced by the ruling's own replacement language: "increments" is four chars longer than "slices" at two sites (+8) and "BUNDLE-" one char longer than "SLICE-" at four link sites (+4). No prose was added and none restored. **Pre-existing drift, not this wave's:** the body was 6,457 (inside budget) at v0.75.0 and reached 6,543 at the v0.76.0 `mochiko-cli` merge's two-arm schema re-point, which shipped without a ledger re-measure — so 56 of the 68 predate this edit. The `authoring-feature-map` +450 from the same v0.76.0 re-point *was* declared and ruled HOLDS; this file's +86 was missed. Recommend the ledger record the v0.76.0 drift and re-derive or re-affirm the budget; this entry does not invent one.
- **Consumers assessed:** the renamed reference had exactly one live inbound link — `SKILL.md`, re-pointed here (both sites). No other shipped primitive, command, agent, template, or schema references `SLICE-IDENTIFICATION.md`; remaining repo hits are `.mochiko/brainstorms/` records and this strip log, which are historical and correctly keep the old name. `references/TEST-GRAMMAR.md` is consumed by `testing-end-user` (executes the grammar) and `executing-tdd-cycle` (parses cards): the anti-pattern line is prose in a "Why This Matters" section, parsed by nothing, so both are unaffected. `schemas/tasks.yaml` carries the same "vertical slice" sentence and is re-worded in the same wave (its own strip entry).

## [v0.76.0] `tasks-template.md` read-pointer → `tasks` schema (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template tasks`, or Read `plugins/mochiko/schemas/tasks.yaml` raw (D8-first-class). One site: the Overview cycle-card-shape pointer.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `in the cycle-card shape ([\`tasks-template.md\`](../../templates/tasks-template.md) is the canonical skeleton):`
- **Kept deliberately:** the `**TEST:**`-grammar ownership and all descriptive text; the in-skill reference pointers to `TEST-GRAMMAR.md` / `SLICE-IDENTIFICATION.md` (untouched — not in-scope templates).
- **Consumers assessed:** n/a (single-writer skill).

## [v0.75.0] Cycle anchor re-keyed to test-case bundles; foundation/feature classification + time-based sizing superseded (SKILL.md + SLICE-IDENTIFICATION.md)

- **Disposition:** superseded → the test-case-bundle anchor + walking-skeleton first-cycle rule in SKILL.md (Overview + Core Principles 1–3, Quick heuristics, Case column, Quality Checklist) and the bundle-identification re-key of `references/SLICE-IDENTIFICATION.md`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Vertical-TDD cycle anchor + QA test-case authorship (D1–D4)"; record `.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md`, D1 test-case-bundle anchor + walking skeleton, D2 two-author split, D3 foundation/feature type dies, D4 grammar ownership unchanged).
- **Content (faithfully compressed; body 5,189 → 6,457 chars, description 496 → 512):**
  - **SKILL.md — superseded whole (protected, D3):** Core Principle **2. Foundation + Parallel**, verbatim: "Foundation cycles run sequentially and establish what every feature depends on — platform infrastructure (IP-XXX items from constraints-and-decisions.md), data models, auth, API framework, error handling. **Identification:** ask \"Could ANY user story work **in production** without this?\" If no, it's foundation. Feature cycles deliver user value incrementally — mapping directly to user stories, independently completable, parallel-eligible `[P]` once foundation is complete unless dependent on another feature cycle. **Identification:** ask \"Does this deliver value a user could observe?\" If yes, it's a feature." → replaced by **2. Walking Skeleton First, Infrastructure Homed by Need** (skeleton first-cycle under the greenfield/new-path carve; no foundation/feature type; infra homed inside the first bundle that needs it, skeleton-path infra in the skeleton; infra-only cards never minted; `[P]` derives from dependencies).
  - **SKILL.md — Overview card-field list:** "per card — stories + feature rationale, foundation/feature type, dependencies, acceptance criteria (by ID), the closing `**TEST:**` gate, and cycle-level brownfield exposure" → "per card — stories + rationale, dependencies, the named test-case list (the card's content), cycle-level brownfield exposure, and the Simple/Split/Merge case"; anchor sentence added (cycle = coherent bundle of named test cases demonstrating together; done = green on real infra). The **Two authors, one card** line added (D2: design seat keeps slicing judgment; qa-engineer authors test-case content).
  - **SKILL.md — Quick heuristics item 4 (time-based sizing, D1):** "**Is sized appropriately** — completable in 1–3 implementation sessions" → "**Is worth demonstrating** — a bundle the user would want to watch pass; merge until it is". Item 3 "its gate can pass without later cycles" → "its cases can pass without later cycles".
  - **SKILL.md — Case column** re-read against bundles: "story = cycle" / "story > cycle" / "stories < cycle" → "story's cases = one bundle" / "story > bundle" / "stories < bundle"; "Size calibration is in SLICE-IDENTIFICATION.md" → "Bundle identification is in SLICE-IDENTIFICATION.md".
  - **SKILL.md — Identifying Cycles pointer:** "the value-stream test, extraction from user stories, size calibration, dependency analysis, worked domain examples, and anti-patterns" → "the value-stream test, the walking skeleton, bundle identification, dependency analysis, and anti-patterns".
  - **SKILL.md — Principle 1 closing** "A card that cannot be demonstrated on its own is not a slice." → "A card whose test cases cannot be demonstrated on their own is not a slice." **Principle 3** re-keyed: "a real-infrastructure verification of the cycle's acceptance criteria" → "the cycle's named test cases run against real infrastructure"; TEST-gate ownership + TEST-GRAMMAR.md pointer kept intact (D4).
  - **SKILL.md — Quality Checklist:** rows "Foundation cycles identified and sequenced; feature cycles marked `[P]` where independent" and "Each card's acceptance criteria cite spec/plan IDs — never re-quoted content" superseded → replaced by walking-skeleton-first check, no-infra-only-cards check, named-test-case-list check, `[P]`-from-dependencies check, and the re-keyed case-level citation row "Each named test case cites the spec/plan ID(s) it covers — never re-quoted content" (D2 amendment: acceptance-ID trace relocated into the test cases, not dropped).
  - **SKILL.md — description:** "classifying foundation vs feature cycles" (false under D3) → "mapping user stories to cycles as demonstrable test-case bundles (Simple/Split/Merge, walking skeleton first)". MUST/SHOULD structure + `**TEST:**` grammar ownership + design-time/build-time boundary kept.
  - **SLICE-IDENTIFICATION.md — superseded whole:** "**Size Calibration**" section (Too Small "< 30 minutes" / Too Large "> 1 day" / Just Right "1-3 hours" — the time-based bars, D1) → replaced by "**Bundle Grain**" (worth demonstrating; merge/split on demonstrability, no clock-time bar). "**Examples by Domain**" (CRUD = 4 cycles, Search = 6, Authentication = 7 with foundation stacks C1–C3 — the F7 proliferation exemplars) → replaced by one "Worked Example — Skeleton First, Then Bundles". Step 2 "Identify Foundation Needs" (Application + Platform foundation lists) → "Identify the Walking Skeleton". Step 3 map-table `(Foundation)`/`(Feature)` labels → C1 = walking skeleton, later cycles bundles. Step 4 "After foundation" → "After the skeleton". Value-Stream-Test row "No, it's infrastructure → Foundation cycle" → "Home it inside the first bundle that needs it (skeleton-path infra → the skeleton cycle)". Dependency-types "A is foundation for B" rows → "B depends on A"; the Infrastructure→"Foundation cycle (platform)" row → home-in-first-needing-bundle. "Minimizing Dependencies" items "Extract shared infrastructure to foundation" + "Separate platform from application foundation" → "Home shared infrastructure where it is first needed". Decision-Matrix rows "Is this user-facing? → Feature cycle / May be foundation", "Is it > 1 day of work? → Split it", "Is it < 30 min of work? → Merge it" → bundle questions (demonstrates-on-its-own / needs-other-cycles / cases-span-more-than-one-demo / runs-on-real-infra). Anti-Patterns (Horizontal / Big Bang / Premature Generalization) kept, re-keyed to skeleton/bundle framing.
  - Verbatim removed text survives in: git history of both files (pre-v0.75.0).
- **Kept deliberately:** the letter/spirit epigraph (v0.27.0-KEPT), `## Overview`, `## When NOT to Use` (incl. bug-fixes exclusion — D1's skeleton carve confirms bug fixes stay out of scope — and the feature-map boundary), Core Principle **1. Vertical Over Horizontal** (D1 retains the vertical keystone), the `**TEST:**` gate ownership + `references/TEST-GRAMMAR.md` pointer (D4: grammar unchanged — TEST-GRAMMAR.md untouched this wave), `## Brownfield exposure`, the value-stream test / dependency analysis / anti-patterns in the reference (re-keyed, not cut). The design-time vs build-time boundary with `executing-tdd-cycle` intact.
- **Protected-content reconciliation (MANDATORY):** the foundation/feature classification and its identification question ("Could ANY user story work in production without this?") were a prior KEPT-protected survivor (v0.27.0/v0.49.0/v0.58.0 keep-sets listed "foundation/feature test" among Core Principles kept). It leaves **only** by the D3 ruling recorded here — not a silent deletion. All other prior KEPT elements survive intact: letter/spirit epigraph, vertical-over-horizontal, TEST-gate ownership, Simple/Split/Merge cases (re-read against bundles, still on-card fields), the Markers/Rationalizations lineage (unaffected), every implementation-level "vertical slice" naming. The acceptance-ID trace (a `review-plan-artifacts` cross-artifact dependency) is **relocated, not removed** — D2's oracle-semantics clause grades each Assert against the scenario it cites, so citation moves into the per-case `Covers` line. Nothing protected silently dropped.
- **Consumers assessed:** `commands/plan.md` (dispatches this skill at design time — anchor change is internal to card authoring, no section-anchor link), `skills/authoring-feature-map/SKILL.md` (names this skill as downstream cycle owner — boundary intact), `skills/brownfield-integration/SKILL.md` (consumes brownfield exposure — unchanged), `skills/executing-tdd-cycle/SKILL.md` + `references/TASK-PARSING.md` (consumes cards + owns build-time decomposition — re-keyed same wave: type field dropped, test-case-list/`Covers` extraction added, execution discipline unchanged), `skills/testing-end-user/SKILL.md` (consumes the `**TEST:**` grammar — unchanged, D4; ignores the card-level `Covers` trace line), router `skills/mochiko/SKILL.md`, `templates/tasks-template.md` (re-keyed same wave), `templates/artifact-format.md`, `references/TEST-GRAMMAR.md` (untouched — grammar owner, D4). None links a removed section anchor. `qa-engineer` gains the D2 design-time authoring seat — role side owned by the P2 seat, not this file.

## [v0.64.0] Guardrails cut — When-to-Use removed, principles/TEST-gate/checklist kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 5,492 → 5,189 chars, −303, −6%; description 814 → 496 chars):**
  - **Removed whole:** `## When to Use` — the four-bullet list ("Structuring an accepted plan's stories into cycle cards (`tasks.md`)" · "Deciding Simple / Split / Merge story→cycle cases and recording the rationale" · "Classifying foundation vs feature cycles and their dependencies" · "Authoring a cycle's `**TEST:**` gate (this skill owns the grammar)"). Restates the description; the Simple/Split/Merge firing survives in `### Case column`, the TEST-gate-ownership in `### 3. Verified against reality`, foundation/feature in `### 2. Foundation + Parallel`.
  - Old description verbatim: "This skill MUST be invoked when structuring a feature's implementation into vertical-slice cycle cards — mapping user stories to cycles (Simple / Split / Merge cases), classifying foundation versus feature cycles, and authoring `tasks.md` as cycle cards (stories + rationale, dependencies, acceptance criteria, the closing `**TEST:**` real-infrastructure gate, cycle-level brownfield exposure). SHOULD also invoke when the work involves \"structure implementation\", \"define cycles\", \"cycle cards\", \"vertical slice\", \"story→cycle mapping\", \"testable increment\", or \"implementation cycles\". Owns the `**TEST:**` grammar. Structures the cycles at design time — it does NOT write task lists: decomposing a card into concrete tasks with file paths happens at build time, owned by mochiko:executing-tdd-cycle, downstream."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the letter/spirit epigraph (v0.27.0-KEPT under R4b), `## Overview`, `## When NOT to Use` (incl. the feature-map boundary), `## Core Principles` (vertical-over-horizontal + Foundation+Parallel + Verified-against-reality — all v0.49.0/v0.27.0-KEPT; TEST-gate ownership + TEST-GRAMMAR.md pointer), `## Identifying Cycles` (SLICE-IDENTIFICATION.md pointer + quick heuristics + the Simple/Split/Merge `### Case column`), `## Brownfield exposure`, `## Quality Checklist`. The description keeps the MUST trigger, the cycle-card gist, the `**TEST:**` gate + grammar ownership, the top trigger phrases, and the design-time vs build-time boundary with `executing-tdd-cycle`.
- **Protected-content reconciliation (MANDATORY):** the v0.27.0 KEPT set (Markers table, Common Rationalizations, Quality Checklist, task-mapping shape, letter/spirit epigraph) and the v0.49.0/v0.58.0 supersession-kept elements (vertical-over-horizontal, foundation/feature test, TEST-gate ownership, Simple/Split/Merge cases, every implementation-level "vertical slice" naming) all survive intact. The removed `## When to Use` is NOT among any prior KEPT enumeration (a description-restatement, densified 6→4 at v0.27.0 but never named a survivor). Nothing protected silently dropped.
- **Consumers assessed:** `commands/plan.md`, `skills/authoring-feature-map/SKILL.md`, `skills/brownfield-integration/SKILL.md`, `skills/executing-tdd-cycle/SKILL.md` (consumes cards + owns build-time decomposition — boundary intact), `skills/testing-end-user/SKILL.md` (+ `references/TASK-PARSING.md`; TEST-grammar pointer intact), router `skills/mochiko/SKILL.md`, `templates/artifact-format.md`, `templates/tasks-template.md`, `references/TEST-GRAMMAR.md` — all reference the skill by name; none links a removed section anchor. The `**TEST:**` grammar ownership and the design-time/build-time boundary are intact.

## [v0.58.0] Graduation-slice vocabulary re-keyed to feature
- **Disposition:** superseded → "stories + feature rationale" in the Overview card-field list · the When-NOT-to-Use feature boundary line ("Deriving or scoping features — the feature is the pipeline unit, owned by `mochiko:authoring-feature-map`, upstream (its vocabulary table disambiguates feature vs cycle); a cycle is a within-one-feature increment") · "its rationale" in the Case-column paragraph
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D4 — graduation slices retire, `authoring-slices` superseded by `authoring-feature-map`)
- **Content:** Overview "per card — stories + slice rationale, …" · When NOT to Use "**Grouping stories into graduation slices** — spec-level decomposition is `mochiko:authoring-slices`, upstream (that skill's vocabulary table disambiguates the two 'slices')" · Case paragraph "The story→cycle decision and its slice rationale live **on the card** (Stories line)"
- **Kept deliberately:** every use of "vertical slice"/"slice" naming the implementation-level cycle unit (title, Core Principles, SLICE-IDENTIFICATION.md pointer, quick heuristics) — that vocabulary is this skill's own and survives D4; only graduation-slice (spec-level) references left. TEST-grammar ownership and the build-time-decomposition boundary untouched.
- **Consumers assessed:** tasks-template (re-keyed same wave) · executing-tdd-cycle (consumes cards, wording unaffected — grep-verified no graduation-slice reference) · plan command (re-keyed same wave) · authoring-feature-map (wave 1, already points here as downstream cycle owner).

## [v0.49.0] Slimmed to cycle-card structuring — task-level content superseded
- **Disposition:** superseded → build-time decomposition in `executing-tdd-cycle` (step 2 of its execution sequence); `references/CYCLE-STRUCTURE.md` deleted
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D2.1)
- **Content:** the task-mapping.md canonical compact shape section (story→cycle + cycles tables + slicing notes) · the TDD Task Sequence section (red/green/refactor as pre-ordered task rows) · the task-level Markers table · Layered Testability · the Common Rationalizations table and Red Flags list (plan-time task-ordering framing) · checklist rows "each cycle has TDD structure (failing test first)" / "every task has a specific file path" · CYCLE-STRUCTURE.md whole (cycle anatomy, task-ID format, file-path conventions, worked task examples). Full text: git history at v0.48.0.
- **Kept deliberately:** vertical-over-horizontal · foundation/feature test · TEST-gate ownership (TEST-GRAMMAR.md untouched as grammar owner) · SLICE-IDENTIFICATION.md (one stale task-mapping paragraph re-keyed to on-card rationale) · Simple/Split/Merge cases (now card fields). Red/green/refactor discipline was double-encoded design+runtime; the runtime copy (`executing-tdd-cycle`) is now the sole carrier — deliberate, not a loss.
- **Consumers assessed:** router · tasks-template · executing-tdd-cycle (boundary flipped: builder now decomposes) · testing-end-user (grammar pointer intact) · brownfield-integration (marker-source wording re-keyed).

## [v0.27.0] Third-copy formats, TDD-sequence snippets, case subsections, and Common Mistakes stripped (body 368 → 204, −45%, in-band)
- **Disposition:** relocated/deduped → verified homes, each Read before landing: the Standard
  Cycle Format block (verbatim in `references/CYCLE-STRUCTURE.md` Cycle Anatomy; conforms-to
  `templates/tasks-template.md`, the declared single source — the in-file third copy was the
  drift risk its own intro warned about) · Task Numbering (CYCLE-STRUCTURE's Task ID Format,
  richer — adds the >4-tasks rule) · Common Mistakes, all six rows homed: tests-after +
  missing-TEST-task (cycle-format/template ordering), horizontal-disguised (in-file Core
  Principle 1 + SLICE-IDENTIFICATION Anti-Patterns), foundation-without-TDD (the identical
  in-file rationalization row), cycles-too-large (SLICE-IDENTIFICATION Size Calibration),
  missing-file-paths (CYCLE-STRUCTURE File Path Conventions + the in-file Quality Checklist row)
  · densified (form + in-file dedup): TDD Task Sequence 50 → 9 lines (four re-shown task-line
  snippets cut; behavioral glosses kept — red must FAIL and express acceptance criteria, green
  minimal across all layers, refactor no behavior change, TEST = real-infra gate checking
  **spec acceptance criteria**), Principle-2 tree + Principle-3 diagram → one-line principles
  pointing at the sections that carry them, Foundation-vs-Feature 24 → 6 lines (both
  identification questions + IP-XXX pointer kept), Simple/Split/Merge 30 → 8 (Case-column form,
  micro-examples one line each, split/merge calibration pointed at SLICE-IDENTIFICATION),
  When-to-Use 6 → 4, No-exceptions 5 bullets → 1 line (refusal clause kept), duplicate
  TEST-GRAMMAR pointer collapsed to one
- **Tier failed:** 1 (every stripped block had ≥1 verified home, most in the skill's own tree) ·
  n/a for the form-only densifications
- **Content:** the format block, numbering list, six mistake subsections, four snippet blocks,
  two ASCII diagrams, three case subsections; nothing was written to `templates/` this wave —
  dedups run against pre-existing template content, so D4's destination ban is not engaged
  (R4a credit)
- **Consumers assessed:** wave-open enumeration — 13 citing files; none links a section anchor
  of this SKILL; `task-architect`'s "cycle structure lives in patterns-vertical-tdd" claim still
  holds via CYCLE-STRUCTURE.md (skill tree). Shared-home audit: `references/TEST-GRAMMAR.md`
  (4 external consumers) is a clean single source — ownership header, CYCLE-STRUCTURE holds
  pointer-only, no dead pointers, untouched

## [v0.27.0] KEPT: Markers table (ownership ruling), rationalizations, checklist, mapping shape, epigraph
- **Tier-2 evidence:** the Markers table was contested (identical table in `tasks-template.md` —
  a legal R4a dedup) and **kept on an ownership ruling**: `task-architect` and
  `brownfield-integration` both name this skill as where the marker vocabulary is *defined*, and
  the skill tree's own CYCLE-STRUCTURE covers only `[EXTEND]`/`[MODIFY]` — stripping would leave
  the claimed owner defining 2 of 4 markers, inverting ownership toward a fill-target. Common
  Rationalizations (8 rows, excuse + reality each) kept as the discipline core — distinct from
  `executing-tdd-cycle`'s runtime red flags (checked: no cross-skill duplicate). Quality
  Checklist kept (aggregation function). The task-mapping.md shape block untouched — it IS the
  v0.23.0 canonical home (below). The letter/spirit epigraph kept as-is under R4b: its
  consequence anchor is the Overview's discipline paragraph eight lines down. Session ruling:
  wave-3 batch-1 ratified 2026-07-25.

## [v0.23.0] task-mapping.md gains its canonical compact shape (wave 2)
- **Disposition:** addition, not a strip — recorded here because it materially changes the authored artifact: `task-mapping.md` had **no prescribed structure anywhere** (the commands called it "the freehand story→cycle mapping"), and kinako's freehand form ran to 45.9k B for one slice
- **Content:** SKILL.md's Mapping-Stories-to-Cycles section now carries the canonical shape — the Story→Cycle table (Case column for Simple/Split/Merge with one-line why) + a per-cycle table (≤ 2-line rationale cells) + an optional Slicing-notes section (≤ 3 lines each, omit when empty), under the `artifact-format.md` envelope. `commands/tasks.md`'s two "freehand" mentions updated to "compact".
- **Consumers assessed:** tasks producer (task-architect — authors it) + review-task-artifacts (mapping checklist grades coverage/slice-quality, shape-agnostic; density note added this wave) + implement's producer (reads it as a design input — a table serves that read better than prose).

## [v0.22.0] TEST: grammar split out of CYCLE-STRUCTURE.md
- **Disposition:** relocated → `references/TEST-GRAMMAR.md` (new; ownership stays with this skill) — the runtime verifier (`testing-end-user`) now loads only the grammar (~5k B) instead of the full CYCLE-STRUCTURE.md (18.6k B)
- **Tier failed:** pure waste (D6a): the verifier's mandated cross-skill read pulled 13k B of cycle-structuring content it never uses
- **Content:** the `## Verification Task Requirements` block (what verification MUST include · Unified TEST: format + runtime-classification table · field definitions · action modifiers · assert patterns · four worked examples · bad-verification examples · why-this-matters · legacy format support) moved verbatim; CYCLE-STRUCTURE.md keeps a pointer section. Retargeted refs: testing-end-user SKILL.md (5) + TASK-PARSING.md (3), patterns-vertical-tdd SKILL.md (2), tasks-template.md (1).
