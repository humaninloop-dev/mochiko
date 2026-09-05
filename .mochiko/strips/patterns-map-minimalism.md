# Strip notes — `skills/patterns-map-minimalism`

Entry formats: `strips/README.md`. Skill born at v0.68.0 (the pm-role-and-feature-derivation
wave); this file opens with the first edit that superseded any of its shipped text.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-map-minimalism/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any capability judgment: **Read `schema.yaml` (this skill's own
  directory) raw, in full, as a declared first action.** The schema is the source of truth for
  this skill's binding rules, nested in six sections, each addressable by its section ID:
  `patterns-map-minimalism.sec.trigger` · `patterns-map-minimalism.sec.scope` ·
  `patterns-map-minimalism.sec.discipline` · `patterns-map-minimalism.sec.inputs` ·
  `patterns-map-minimalism.sec.disclosure` · `patterns-map-minimalism.sec.reserved`. Interpret
  it live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
  carrying `when:` binds only where its terms hold against the schema's declared `conditions:`,
  except that a `class: floor` rule is always read and always delivered — `when:` gates when its
  obligation applies, never whether it reaches you; a `pointer:` rule binds you to that file's
  or skill's procedure, referenced never restated; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 3 rules of `class: floor` are
  non-waivable. Before the first capability-judgment step, state the floor count back — a
  skipped or partial read leaves that count blank: halt and surface it, and halt likewise if the
  schema's `class: floor` count disagrees with the pin.
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
  The floor pin: the 3 rules of `class: floor` are
  non-waivable. Before the first capability-judgment step, state the floor count back — a
  skipped or partial read leaves that count blank: halt and surface it, and halt likewise if the
  schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.102.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2B, patterns family)

Ruling for every entry below: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema ruled ·
Skill-schema wave-2 family doors ruled — the patterns-family door); census:
`.mochiko/brainstorms/skill-content-schema/census-patterns.md` §A (MM) + §B (MM row
inventory). Schema home: `plugins/mochiko/skills/patterns-map-minimalism/schema.yaml`.
Minted IDs carry the `patterns-map-minimalism.` prefix (omitted below). Map — census §B row
→ minted ID: 1 (fires at derivation/review/grooming) `firing-sites` · 2 (fewness never
hides) `fewness-never-hides` · 3 (discipline-vs-vocabulary boundary) `vocabulary-boundary` ·
4 (tests grade the durable layer) `durable-layer-only` · 5 (mint/merge/retire the user's)
`capability-layer-user-ruled` · 6 (three governing tests all hold) `three-governing-tests` ·
7 (read the current map first) `read-map-first` · 8 (noun+verbs aid never a gate)
`noun-verbs-aid-only` · 9 (extend beats mint) `extend-beats-mint` · 10 (soft cap ~9)
`soft-cap-trigger` · 11 (merge preserves the four mechanics — built as ONE 4-limb rule, the
census-sanctioned option keeping the count at 13) `merge-preserves-mechanics` ·
12 (re-parenting navigation-only) `reparenting-navigation-only` · 13 (domains at cap-trip
with the deferred co-sign machinery) `domains-dormant`. Class mix 3 floor (rows 2 · 5 · 9) /
10 must / 0 advisory, census-exact.
**Section distribution (build call, disclosed row-by-row):** trigger {1, 10 — the soft cap
IS the grooming trigger} · scope {3, 4} · discipline {2, 6, 8, 9, 13} · inputs {7} ·
disclosure {11, 12} · reserved {5}.
**`conditions:`/`when:` (census §B dims):** one dimension `cap_trip` (presence,
entry-derived). `when: {cap_trip: present}` on row 13 only — **DECLARE form**
(lead-confirmed the idiom transfers): "minted only at cap-trip" rides the rule's subject and
extraction would falsify the text, so the text stands and `when:` is added beside it. The
`absent` pole is named by no rule's `when:` (deliberate absence); the checker's coverage
warning is expected and accepted.
**Deleted as dedup, no content loss:** the Overview's fires-at sentence (row 1), the "Few is
not sparse" paragraph (row 2 — a floor, never duplicated inline per D6), the Boundary line
(row 3), `## When NOT to Use` (3 bullets — rows 4 · 5 and the sibling-altitudes half of row
4), the capability-tests lead-in obligation sentence (rows 6 · 7; the three tests' teaching
kept body-side per the wave brief), the noun+verbs paragraph (row 8), `## Extend beats mint`
(row 9), `## Soft cap and grooming` (rows 10 · 5 · the roll-up line into row 3's rule),
`## Merge mechanics` (rows 11 · 12), the `## Domains (dormant)` obligation sentences (row
13; the Sessions/Knowledge example and the disagreement-is-a-conversation line kept as body
teaching), `## Sibling` (row 4's rule), and `## Quality Checklist` (6 boxes, all mapped
above).
Accounting (canonical snippet): body 4,647 → **2,470** (obligations out, the Rules block in)
+ schema 6,892 = **payload 9,362** (census §F estimate ~8,910); description untouched at
499. The delta over the pre-conversion body is structural overhead (IDs, keys, section
scaffolding, reading grammar) — no content growth claimed. MM was unbudgeted at birth
(hard-cap-only); the conversion re-seed is its first budget row, via the ledger's third
seeding path, no headroom (the ceremony seat executes the ledger row).

## [v0.102.0] Ruled derivation machinery relocated — moves recorded per the R-c ceremony class (census §A; pm-role-and-feature-derivation D1–D12)

- **Disposition:** relocated → `plugins/mochiko/skills/patterns-map-minimalism/schema.yaml`, per the map entry above (D3). The body is birth-by-ruling, not wholesale-protected (no KEPT line exists); each move below is recorded here citing the birth row, and the DECISIONS-traceable core rules inherit protected status through the provenance sidecar (D8/C4).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows; birth/protecting ruling `DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation — D3 extend-beats-mint + soft cap, D4 ranked capability tests + domains kept-but-shrunk + this carrier skill, D5 firing sites, D12 capability-merge mechanics).
- **Content (decisive line per moved unit, verbatim):** three governing tests — "A candidate earns a capability only when the **three governing tests all hold** — read the current map first, since \"new kind\" is claimed against what is already there." → `three-governing-tests` + `read-map-first` · noun+verbs — "**noun + verbs is a heuristic aid, never a gate** … it never blocks a cross-cutting capability (an onboarding journey) that passes the governing tests." → `noun-verbs-aid-only` · extend beats mint — "A new story **grows an existing capability first** (the new-kind test decides); a mint must argue against extending. An unargued mint is the story-mirroring defect the map exists to stop." → `extend-beats-mint` (floor) · soft cap — "Past roughly **nine top-level capabilities**, a grooming pass runs … The cap is a **trigger, never a hard block**" → `soft-cap-trigger` · user rules — "Merge and retire are always **PM recommendations; the user rules**." / "the capability layer is the user's to rule, never self-executed" → `capability-layer-user-ruled` (floor) · fewness — "**Few is not sparse** — fewness never hides a capability that passes the tests" → `fewness-never-hides` (floor) · merge mechanics — "the **absorbing ID survives**; the merged entry flips **`retired`**, dated, with a **merged-into pointer** — never deleted; **extents union under an honesty pass** — no flattering over-claim; **story traces and SC references consolidate** onto the absorbing entry; **pending rows and unfolded deltas transfer** — no obligation dropped." → `merge-preserves-mechanics` (one 4-limb rule) · re-parenting — "Re-parenting under a domain header is navigation-only — no status semantics." → `reparenting-navigation-only` · domains — "Every capability lives in **exactly one** domain, minted **only at cap-trip** — a small map stays flat; the **PM proposes** names and the **principal-architect co-signs**" → `domains-dormant` · boundary — "this skill owns the discipline; the capability/work-row vocabulary, file shapes, and fold mechanism live in `mochiko:authoring-feature-map`" → `vocabulary-boundary` · work rows — "the tests grade the durable layer, not the rows" → `durable-layer-only` · firing — "firing at the **PM's derivation** (specify), the **spec reviewer's** grade, and **`/mochiko:feature` grooming** at cap-trip" → `firing-sites`.
- **Kept deliberately:** the v0.81.0 dormant-domains re-keyed wording survives **unchanged in substance** inside `domains-dormant`'s text — "the store's domain-to-spine mapping line and the co-sign duty are deferred to the first real cap-trip: neither is built until a live map actually approaches the cap" (the entry below records that re-key; the deferral's force and trigger are untouched by this conversion). Also kept body-side as teaching: the three tests' explanations with the kinako examples, and the Sessions/Knowledge domains line.
- **Consumers assessed:** `mochiko:authoring-feature-map` owns the vocabulary this skill routes to (untouched); `/mochiko:feature` grooming reaches this skill at cap-trip and restates nothing; `mochiko:review-specifications`' map-layer checks reference the tests by pointer (untouched).

## [v0.81.0] Dormant-domains pointer re-keyed: `ARCHITECTURE.md` mapping → the store's domain-to-spine mapping — product-architecture-schema D3/D4

- **Disposition:** superseded → the architecture store's domain-to-spine mapping line. The
  deferral is unchanged in force and in trigger; only the surface the deferred line would one day
  be written on has moved, since `ARCHITECTURE.md` is now a derived index rather than a
  hand-maintained doc.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3/D4; `DECISIONS.md`
  2026-08-19).
- **Content (superseded, verbatim — two fragments):**

  ```
  **The `ARCHITECTURE.md` domain-to-components mapping line and the principal-architect's
  co-sign duty are deferred to the first real cap-trip — neither is built until a live map actually
  approaches the cap.**
  ```

  ```
  - [ ] Domains flat below cap-trip; no ARCHITECTURE.md mapping line or co-sign machinery before first cap-trip
  ```
- **Kept deliberately:** the deferral itself and both its halves — the mapping line **and** the
  principal-architect co-sign duty stay dormant until a live map actually approaches the cap.
  Nothing here is activated by the store's arrival; the architect's recharter as store steward
  (D7) does not fire the cap-trip machinery early.
- **Consumers assessed:** `mochiko:authoring-feature-map` owns the domain vocabulary and is
  re-keyed in the same edit set by this seat. `/mochiko:feature` grooming (P2) reaches this skill
  at cap-trip and states no architecture surface of its own (grep clean).
