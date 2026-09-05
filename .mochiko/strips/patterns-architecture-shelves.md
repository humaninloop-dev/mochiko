# Strip notes — `skills/patterns-architecture-shelves`

Entry formats: `strips/README.md`. Skill born at v0.81.0 (the product-architecture-schema
wave, Stage 1); this file opens with the first edit that superseded any of its shipped text —
the wave-2B schema conversion (census-patterns J-P2: no post-birth edit had removed content
before this, so the conversion entry is the file's first).

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-architecture-shelves/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any shelf walk: **Read `schema.yaml` (this skill's own directory)
  raw, in full, as a declared first action.** The schema is the source of truth for this skill's
  binding rules, nested in six sections, each addressable by its section ID:
  `patterns-architecture-shelves.sec.trigger` · `patterns-architecture-shelves.sec.scope` ·
  `patterns-architecture-shelves.sec.discipline` · `patterns-architecture-shelves.sec.inputs` ·
  `patterns-architecture-shelves.sec.disclosure` ·
  `patterns-architecture-shelves.sec.reserved`. Interpret it live: a rule's `kind:` names what
  it is, and an absent `kind:` reads `constraint`; a rule of `class: floor` is always read and
  always delivered; a `pointer:` rule binds you to that file's or skill's procedure, referenced
  never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin:
  the 5 rules of `class: floor` are non-waivable. Before the first walk step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and halt
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
  The floor pin:
  the 5 rules of `class: floor` are non-waivable. Before the first walk step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.102.0] Schema conversion — ruled shelf machinery relocated, census-row → minted-ID map (skill-content-schema wave 2B, patterns family)

- **Disposition:** relocated → `plugins/mochiko/skills/patterns-architecture-shelves/schema.yaml` (skill-content-schema D3, the three-home boundary). Per the census R-c idiom the body is birth-by-ruling, **not wholesale-protected** (no KEPT line exists); each move below is recorded here citing the birth row, and the DECISIONS-traceable core rules inherit protected status through the provenance sidecar (D8/C4).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows: Skill-content schema ruled · Skill-schema wave-2 family doors ruled — the patterns-family door; birth/protecting ruling `DECISIONS.md` 2026-08-19 product-architecture-schema — shelves dealt recommend-then-arbitrate under three-strata floor precedence. Census: `.mochiko/brainstorms/skill-content-schema/census-patterns.md` §A (AS) + §B (AS row inventory)).

**Census-row → minted-ID map.** Minted IDs carry the `patterns-architecture-shelves.` prefix
(omitted below). Census §B row → minted ID: 1 (opinions-in-data Read-raw) `opinions-in-data`
· 2 (AX-XXX grammar routed) `ax-grammar-routing` · 3 (never a rigor dial)
`never-a-rigor-dial` · 4 (not per-feature design; the not-the-store's-grammar half rides
row 2's rule) `not-per-feature-design` · 5 (recommend-then-arbitrate)
`recommend-then-arbitrate` · 6 (memory-asserted provenance) `memory-asserted-provenance` ·
7 (scope from `spine.md`) `scope-from-spine` · 8 (honest gaps) `honest-gaps` ·
9 (retrofit-cost walk order) `retrofit-cost-walk-order` · 10 (breadth invariant)
`breadth-invariant` · 11 (stance vocabulary) `stance-vocabulary` · 12 (`n-a` reason axis +
owner-pointer) `na-reason-axis` · 13 (`not-now` upgrade trigger) `not-now-carries-trigger` ·
14 (floor-asserted rows) `floor-asserted-rows` · 15 (card/shelf conflict)
`card-shelf-conflict-users` · 16 (one-dial-one-system) `one-dial-one-system` · 17
(event-keyed freshness) `event-keyed-freshness`. Class mix 5 floor (rows 3 · 5 · 8 · 10 ·
14) / 11 must / 1 advisory (row 17), census-exact.

**Section distribution (build call, disclosed row-by-row):** trigger {17 — the
revisit-firing rule, lead-confirmed over an empty marker} · scope {2, 3, 4, 16} · discipline
{8, 9, 10, 14} · inputs {1, 7} · disclosure {6, 11, 12, 13} · reserved {5, 15}. No
`conditions:` block — census §B: none obvious, none forced; the load-first block legally
omits the `when:`-grammar sentence (wave-1 RCM-4 wave-wide ruling).

**Pointer disclosures.** `opinions-in-data` carries
`pointer: ../../schemas/architecture-shelf-backend.yaml` — the census J-P8 schemas-home data
class, resolving base-dir-relative via the `../../schemas/` climb. `scope-from-spine`
deliberately carries **no `pointer:` field**: `spine.md` is a runtime artifact in the USER'S
repo (`.mochiko/product/architecture/`), not a plugin file, so pointer resolution would
rightly fail — the path stays in the rule's text (lead-endorsed; an audit must not read the
absent pointer as an omission).

**Deleted as dedup, no content loss:** `## When NOT to Use` (3 bullets — rows 3 · 2 · 4),
`## Shelves are dealt, never asserted` (rows 5 · 6), `## Scope — from setup, overridable at
the desk` (rows 7 · 8; the compose-shelves line kept as Overview teaching), the walk-order
section's obligation sentence (row 9; the early/late poles kept as body teaching), `## The
breadth invariant` (row 10; the considered-and-declined note kept as body teaching), the
stance-vocabulary obligation sentences (rows 11 · 12 · 13; the table and the time-bombs line
kept as body teaching), `## Three strata — what binds what` (rows 14 · 15 · 16), `## Shelf
freshness` (row 17), and `## Quality checks` (10 boxes — 9 mapped above at build; the first
box's **display-for-override obligation**, "shown to the user for override, never silently
assumed", was initially dropped and restored to `scope-from-spine`'s text in the W1 fix
round, so all 10 now map).

**W1 fix round (audit-driven, both in-flight amendments to this unshipped entry):** (a) the
display clause above; (b) the three-strata precedence the frozen `description:` advertises is
named as a set in `floor-asserted-rows`'s text — the first stratum citing
`card-shelf-conflict-users` and `one-dial-one-system` as the second and third — so the set
survives its split across three sections.

**Accounting (canonical snippet, post-fix-round):** body 6,927 → **2,923** (obligations out,
the Rules block in) + schema 9,438 = **payload 12,361** (census §F estimate ~12,370; re-measured
by the closer seat after the W1/W2 fix rounds landed — the ledger row agrees); description untouched at 473. The
delta over the pre-conversion body is structural overhead (IDs, keys, section scaffolding,
reading grammar) — no content growth claimed. AS was unbudgeted at birth (hard-cap-only; the
post-birth +343 fix-round growth is ledgered, no strip owed for additions); the conversion
re-seed is its first budget row, via the ledger's third seeding path, no headroom (the
ceremony seat executes the ledger row).

- **Content (decisive line per moved unit, verbatim):** breadth invariant — "**Every row on the scoped shelf is walked.** … it is never silently skipped. … There is **no magnitude scaling**." → `breadth-invariant` (floor) · recommend-then-arbitrate — "name the suggested default, say what it costs and what would argue against it, then let the user rule. … an asserted default here would be an opinion wearing a rule's clothes" → `recommend-then-arbitrate` (floor) · floor-asserted stratum — "`n-a — genuinely never` is **unavailable** at the desk. … A genuine drop is a governance waiver, recorded in the ledger — never a shelf stance." → `floor-asserted-rows` (floor) · honest gaps — "**honest gaps, not silent ones** … Never deal a filtered backend list to a non-backend surface" → `honest-gaps` (floor) · rigor dial — "the shelf never reads the governance low/high depth level" → `never-a-rigor-dial` (floor) · stance vocabulary + `n-a` reason axis + `not-now` trigger — "`n-a` is always written with its reason suffixed … The **pointer to that owner is required**, not optional." → `stance-vocabulary` / `na-reason-axis` / `not-now-carries-trigger` · scope binding — "**Read the scope from `spine.md`'s `Scope:` line** … do not re-ask the user at each walk." → `scope-from-spine` · opinions-in-data — "The backend shelf ships at `plugins/mochiko/schemas/architecture-shelf-backend.yaml` — Read it raw." → `opinions-in-data` · walk order — "Walk the shelf in order of **what costs most to retrofit**, not in file order" → `retrofit-cost-walk-order` · strata 2/3 — "A genuine conflict is the user's to rule — do not resolve it at the desk." / "One dial must not silently drive two systems" → `card-shelf-conflict-users` / `one-dial-one-system` · freshness — "Revisit is **event-keyed, never calendrical**" → `event-keyed-freshness` · AX routing — "Stances land as `AX-XXX` rows in the store (`mochiko:authoring-architecture-store` owns that grammar)" → `ax-grammar-routing` · per-feature carve — "a feature's structural delta is `mochiko:patterns-system-design`" → `not-per-feature-design`.
- **Kept deliberately:** the Overview's shelf definition and opinions-in-data identity line, the compose-shelves teaching line, the walk-order early/late poles, the stance table with the time-bombs line, and the considered-and-declined note guarding the breadth invariant's rationale — teaching stays body-side (D3), the obligations now schema-side.
- **Consumers assessed:** `/mochiko:architecture` (the desk) and `mochiko:authoring-architecture-store` reference the skill, never restate the walk (untouched); `architecture-shelf-backend.yaml` is untouched (its `floor_bound` values are the data `floor-asserted-rows` binds).
