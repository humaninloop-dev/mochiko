# Strip notes — `skills/authoring-constitution/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.22.0] entries — workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6b; ratified 2026-07-23).

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

**Wave context (v0.65.0 — the production-floor two-row conversion).** The `catalog`/floor doctrine
across the `references/` tree converts from a single asserted production level to a **two-row
`low`/`high` depth dial** (one project-wide level, user-declared, one-way `low`→`high`). Ruling:
`production-floor-adaptive-depth`, ratified 2026-08-11, D1–D8 — `.mochiko/brainstorms/production-floor-adaptive-depth/record.md`;
`DECISIONS.md` 2026-08-11 adaptive-depth row. **PO-D2 amended** (one floor, now at two depth levels;
the four-rung tier ladder stays retired), **PO-D7 superseded** (staged adoption is the `low` level,
not a recorded waiver). D5 draws the `low` row on the retrofit-cost cut line: obligations expensive
to retrofit hold identically at both levels; addable rigor (merge-blocking gates, coverage
thresholds) may relax at `low`. All seven entries below are supersessions **by ruling**, not tier
failures. `catalog/backend-service.md` was deliberately left untouched (its cards are arbitrated
arch-opinion, outside the depth dial per PO-D3 S7 / review-fold #9). `COMPLIANCE-MODULES.md` and the
`ESSENTIAL-FLOOR.md` worked-example framing gained **pure additions** (D7 level-blind clause; a
depth note + coverage annotation) — additions ride the decision row and take no strip entry.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the authoring family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/authoring-constitution/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any authoring step: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same
  declared first action** — schema, then common. The schema is the source of truth for this
  skill's binding rules, nested in six sections, each addressable by its section ID:
  `authoring-constitution.sec.independence` · `authoring-constitution.sec.scope` ·
  `authoring-constitution.sec.inputs` · `authoring-constitution.sec.artifact` ·
  `authoring-constitution.sec.output` · `authoring-constitution.sec.reserved`. Interpret it
  live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
  carrying `when:` binds only where its terms hold against the schema's declared
  `conditions:`, except that a `class: floor` rule is always read and always delivered —
  `when:` gates when its obligation applies, never whether it reaches you; a `pointer:` rule
  binds you to that file's or skill's procedure, referenced never restated; `${var}`
  substitutes from this schema's `vars:` at read time; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
  `extends: authoring-common.<slug>` inherits text/labels/pointer from
  `skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `authoring-constitution.*` ID stays the citable ID. The floor pin: the 12
  rules of `class: floor` are non-waivable. Before the first authoring step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The `extends:` stub resolution
  and the family common-file co-Read are discharged by the render, which resolves every stub
  before the model sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-authoring-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the 12
  rules of `class: floor` are non-waivable. Before the first authoring step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A, authoring family)

Ruling for every entry below: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers + the whole-body re-home clause), `DECISIONS.md` 2026-09-01 rows
(Skill-content schema ruled · Skill-schema wave-2 family doors ruled — the authoring-family
door); census: `.mochiko/brainstorms/skill-content-schema/census-authoring.md` §A (AC) +
§B (AC). Schema home: `plugins/mochiko/skills/authoring-constitution/schema.yaml`. Minted
IDs carry the `authoring-constitution.` prefix (omitted below). Map — census §B row →
minted ID:
1 `no-constitution-file` · 2 `surface-set-binding` · 3 `trace-summary-in-output` ·
4 `ratified-synthesis-only` · 5 `independent-grade` (C-A2 stub,
`extends: authoring-common.independent-grade`; `${artifact}` = "governance surface set",
`${grader}` = `` `validation-constitution` ``) **+ `never-co-mounted` (residue mint,
wave-lead ruled: the stub carries only the block core; the protected "never co-mounted" and
"loop + gates lead-owned" clauses mint an adjacent LOCAL rule in `sec.independence` — AC
builds 48 rules, census 47, deviation recorded here)** · 6 `every-principle-traces` ·
7 `realized-or-flagged` · 8 `no-unsanctioned-selection` · 9 `waivers-authored-not-skipped` ·
10a `universal-routing` · 10b `scope-bound-routing` · 10c `rules-inject-on-read-caveat`
(`when: {rules_file_in_set: present}`) · 10d `procedure-shaped-routing` ·
10e `index-and-ledger-closure` · 11 `two-modes-one-core` · 12 `content-source-bindings` ·
13 `two-arm-schema-binding` (C-A3 stub, `extends: authoring-common.two-arm-template`,
`${template}` = `governance-surfaces` — BOTH body sites, the "Artifact shapes" line and the
content-inventory preamble, single-home into this one rule, wave-lead confirmed; the
governance-intent arm lives only in `references/INTERROGATION-AGENDA.md` and rides the file
per census J-2) · 14 `missing-synthesis-stop` (`when: {synthesis: absent}`) ·
15a `markers-only-regeneration` · 15b `preserved-blocks` · 15c `amend-preserves-verbatim`
(`when: {mode: amend}`) · 16a `grading-routing` · 16b `eliciting-routing` ·
16c `analysis-routing` · 17 `three-part-rule` · 18 `enforcement-fits-team-reality` ·
19 `operative-constraint-only` · 20a `region-content-set` ·
20b `quality-gates-actual-commands` · 20c `output-style-file-every-run` ·
20d `ledger-riders-and-trace-manifest` · 21 `floor-categories-accounted` ·
22 `no-sync-section` · 23 `never-route-unselected-modules` · 24a `brownfield-ceiling-test`
(`when: {mode: brownfield}`) · 24b `evolution-notes-always-brownfield`
(`when: {mode: brownfield}`) · 25 `roadmap-stub` (advisory) · **reference stubs (files
untouched, stub points):** 26 `essential-floor-canonical-home` ·
27 `depth-dial-breadth-invariant` · 28 `floor-doctrine` · 29 `floor-cards-content-sets` ·
30 `three-source-rule` · 31 `arbitrated-shelf-frame` · 32 `module-mechanical-attachment` ·
33 `s4-fail-safe` · 34 `domain-dependency-admissibility` ·
35 `domain-dependency-growth-gate`. `references/INTERROGATION-AGENDA.md` carries NO stub —
census J-2, by ruling, named here so the omission reads as a ruling, not a miss;
RFC-2119-KEYWORDS.md and EMERGENT-CEILING-PATTERNS.md stay teaching, no stubs.
**Section distribution (build call, disclosed):** the census gave body totals
(independence 1 · scope 5 · inputs 3 · output 6 · reserved 4 · artifact 18) with stubs
distributing "the same way"; built: independence 2 (stub + residue mint) · scope 5 (16a ·
16b · 16c · 22 · 25) · inputs 5 (4 · 12 · 14 + stubs 30 · 31) · artifact 27 (**20 body**:
1 · 2 · 6 · 7 · 9 · 10a · 10b · 10c · 10d · 11 · 13 · 15a · 15b · 15c · 17 · 18 · 19 ·
21 · 24a · 24b + stubs 26 · 27 · 28 · 29 · 32 · 33 · 34) · output 6 (3 · 10e · 20a–20d) ·
reserved 3 (8 · 23 + stub 35). Named placement calls: row 9
`waivers-authored-not-skipped` sits in artifact (waiver records are ledger content), and
row 21 `floor-categories-accounted` sits in artifact — a move from the census's
reserved-leaning read of it (the accounting duty binds the set's content, not a
user-reserved decision). The deltas vs the census tallies are placement calls within the
lead-granted latitude, all rows accounted.
**Deleted as dedup, no content loss:** the synthesis-contract bullet list (rows 6–9), the
surface-routing bullet list (10a–e), the "Artifact shapes" line + the mandatory-content
inventory's 8-item list (13 · 20a–d · 21's blockquote), the mode-prerequisites and
ownership-boundary blockquotes (14 · 15a–c), `## When NOT to Use` (16a–c), the
principle-writing-format line (19), the no-sync line (22), and the roadmap-stub blockquote
(25) — every line's obligation lives in the mapped rule. The body keeps: the overview +
surface table (disclosure levels, teaching), the traceable-contract framing, the
two-modes table, the content-source pointers, the Three-Part + RFC 2119 teaching, the
module-assembly table (the KEPT survivor's narrative face), the brownfield floor/ceiling
narrative, and Related.
Accounting: body 17,886 → 7,255 (−10,631, obligations out + the load-first Rules block in)
+ schema 23,132 = **payload 30,387** (census §F estimate was ~36,350; figure includes the
V1 fix-round Shape-token restorations); the delta over
the pre-conversion body is structural overhead (IDs, keys, section scaffolding, reading
grammar) — no content growth claimed beyond the residue mint recorded above. Budget
re-seeds from 21,550 to the measured payload, third seeding path, no headroom (P5 executes
the ledger row).

## [v0.101.0] Whole-body survivor protection re-homes onto the pair (D8/C4 explicit clause)
- **Disposition:** superseded — the [v0.28.0] "KEPT: the remaining body" under-band survivor ruling (already narrowed by the two recorded v0.63.0 supersessions — the enforcement-strength table and the brownfield action-mapping paragraph) re-homes onto the PAIR: `SKILL.md` body + `schema.yaml` jointly, recorded once here per skill-content-schema D8/C4 — the review-feasibility precedent replayed; no per-line ambiguity survives. Every named survivor's home after conversion: the synthesis contract in `ratified-synthesis-only` + `sec.reserved` + `every-principle-traces`/`realized-or-flagged`; surface routing in `universal-routing` · `scope-bound-routing` · `rules-inject-on-read-caveat` · `procedure-shaped-routing` · `index-and-ledger-closure`; the module-assembly table's obligations in `never-route-unselected-modules` (narrative table stays body prose); both mode contracts in `two-modes-one-core` + the `mode:` dimension; the ceiling test in `brownfield-ceiling-test`; the floor-accounting and mode-prerequisites blockquotes in `floor-categories-accounted` + `missing-synthesis-stop`; the Three-Part Rule in `three-part-rule` (+ `enforcement-fits-team-reality`); RFC 2119 in `operative-constraint-only` + the body's teaching pointer; the inventory behavioral riders in `region-content-set` · `quality-gates-actual-commands` · `output-style-file-every-run` · `ledger-riders-and-trace-manifest`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows).
- **Content:** the [v0.28.0] "KEPT: the remaining body (under-band survivor ruling)" claim — see that entry below for its lineage.
- **Kept deliberately:** `description:` byte-untouched; all 9 `references/` files untouched (6 root + 3 catalog); `templates/constitution-modules/` untouched.
- **Consumers assessed:** GI-005/GI-006 hold — every protected line traceable through this ledger to its new home, reconstructible in both directions.

## [v0.101.0] Adaptive-depth machinery — protection transfers (census §A row 2; production-floor-adaptive-depth D1–D8, v0.65.0)
- **Disposition:** superseded — the body-borne depth rules transfer to `two-modes-one-core` (the declared-level row clause), `region-content-set` (floor-first, `(NON-NEGOTIABLE)`, depth-row-selected, `high`-only-absent-at-`low`), and `floor-categories-accounted` (the not-a-missing-category clause); the reference-borne machinery (two-row dial + breadth invariant, floor doctrine, floor cards' content sets, level-blind module attachment) transfers to reference stubs `depth-dial-breadth-invariant` · `floor-doctrine` · `floor-cards-content-sets` · `module-mechanical-attachment` — the reference files stay the procedure homes, referenced never restated (D3/C2 stub-default). Per D8/C4 the protections re-home onto these rule IDs.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** body: "floor cards authored at the synthesis's declared depth level (the two-row `low`/`high` card form)" · "Author each floor principle at the row the synthesis's declared depth level selects; a `high`-only check is absent at `low`." · "(A `high`-only check absent at `low` is not a missing category — its category is still present through its other rows.)"
- **Consumers assessed:** all v0.65.0 reference-file products untouched; `/mochiko:setup` reads the same doctrine through the catalog (untouched).

## [v0.101.0] Two-arm governance-surfaces pointers — protection transfers to C-A3 stub (census §A row 3; schema-based-template-guidance D1/D8, v0.76.0; GI-020)
- **Disposition:** superseded — both `SKILL.md` sites (the "Artifact shapes" line and the content-inventory preamble) single-home into `two-arm-schema-binding` (`extends: authoring-common.two-arm-template`, must, `kind: binding`, `${template}` = `governance-surfaces`), per D8/C4; both arms preserved verbatim through the inherited block text (GI-020 — raw Read the first-class degraded path). The third site — the governance-intent pointer in `references/INTERROGATION-AGENDA.md` — rides the untouched file (census J-2).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "invoke `mochiko-cli template governance-surfaces` when the binary is available; otherwise Read `plugins/mochiko/schemas/governance-surfaces.yaml` raw" (both body sites carried this two-arm form).
- **Consumers assessed:** `plugins/mochiko/schemas/skill-authoring-common.yaml` (P1's, this wave) carries the block; `schemas/governance-surfaces.yaml` untouched.

## [v0.101.0] D8 ownership boundary — protection transfers (census §A row 4; constitution-native-surfaces D8, 2026-07-18)
- **Disposition:** superseded — protection transfers to `markers-only-regeneration` (floor), `preserved-blocks` (floor), and `amend-preserves-verbatim` (must, `when: {mode: amend}`), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "Regenerate only what sits between the markers — user content outside them is untouchable." · "the two preserved blocks: `mochiko:domain-registry` … and the `mochiko:output-style` switch line … on every later run **read the existing values and re-emit them unchanged**" · "In amend, preserve untouched principles verbatim (their GI-IDs are stable) and bump the region's semver. A `.mochiko/memory/constitution.md` on disk is superseded — the lead deletes it; never author into it."
- **Consumers assessed:** `/mochiko:setup`'s amend mode consumes the boundary by reference (untouched).

## [v0.101.0] D4/D4.1/D4.2 waiver discipline — protection transfers (census §A row 5)
- **Disposition:** superseded — the body rule transfers to `waivers-authored-not-skipped` (floor); the reference copies in COMPLIANCE-MODULES.md and catalog/universal-floor.md ride stubs `module-mechanical-attachment` / `s4-fail-safe` / `floor-cards-content-sets` with this dedup note — the files stay the procedure homes, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "**Waivers are authored, not skipped.** A waived standard gets a waiver record in the ledger (standard, justification, optional revisit trigger, trace — D4: permanent pending the D4.1 revisit; legal-mandate module obligations are never waivable, D4.2) — absence is always deliberate and auditable."
- **Consumers assessed:** `validation-constitution` grades waiver records against the same D4 model (P3-family member wave 1, untouched here).

## [v0.101.0] Remaining body obligations relocated (census §B rows 1 · 2 · 3 · 6 · 7 · 8 · 10a–10e · 16a–16c · 17 · 18 · 19 · 20a–20d · 21 · 22 · 23 · 25)
- **Disposition:** relocated → `plugins/mochiko/skills/authoring-constitution/schema.yaml`, per the map entry above (D3). Protected standing rides the whole-body re-home entry above; per-line lineage stays reconstructible through the map.
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 1 "**There is no `constitution.md`.** Governance lands on the surfaces Claude Code natively loads" · 2 the four-surface set (region markers · rules files · skill pointers · ledger) · 3 "write it as part of the output, not as an afterthought" · 6 "**Every principle traces.** … always in the **ledger** … the canonical trace record" · 7 "**Every element is realized or flagged.** … never silently dropped" · 8 "**No unsanctioned selection.** … emit a **flagged proposal** … ruled on by the user at the acceptance gate" · 10a "a **short imperative line in the CLAUDE.md region** … Universal principles do NOT go to unconditional rules files" · 10b "`paths` globs covering **every path whose code can violate the concern**" · 10c "rules files inject on **Read**, not Write — … emit the region's standing new-file read line" · 10d "Mint a new skill only for a session-minted procedure" · 10e "Index → home → ledger must close; the validator checks it" · 16a–c the three When-NOT routings (validator · interrogation · `analysis-codebase`) · 17 "Every principle MUST have three components … A principle without all three is incomplete" · 18 "Enforcement MUST fit the team reality recorded in the synthesis" · 19 "The surface … carries the **operative constraint only** … do not restate them" · 20a–d the mandatory content inventory's eight items as four set-rules (stamp/index/universal-lines/stack · gates with **actual commands, never placeholder tokens** · the unconditional output-style rules file every run · ledger riders + trace summary manifest) · 21 "MUST **account for** all four Essential Floor categories … A floor category with neither is a defect in either mode" · 22 "There is **no CLAUDE.md-synchronization section**" · 23 "**Never route module content the synthesis didn't select**" · 25 "reference `.mochiko/memory/evolution-roadmap.md` as a documented stub"
- **Consumers assessed:** `validation-constitution` grades the produced set, never this skill's text (untouched); the router row stays generically true.

## [v0.76.0] `governance-surfaces-template.md` + `governance-intent-template.md` read-pointers → schemas (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template governance-surfaces` / `template governance-intent`, or Read `plugins/mochiko/schemas/governance-surfaces.yaml` / `plugins/mochiko/schemas/governance-intent.yaml` raw (D8-first-class). Three sites: `SKILL.md` "Artifact shapes" pointer + "Every governance set MUST include, per" pointer (both → governance-surfaces); `references/INTERROGATION-AGENDA.md` synthesis-shape mention (→ governance-intent).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `Artifact shapes (region block, rules file, ledger):` / `[\`governance-surfaces-template.md\`](../../templates/governance-surfaces-template.md).` — `SKILL.md`
  - `Every governance set MUST include, per` / `[\`governance-surfaces-template.md\`](../../templates/governance-surfaces-template.md):` — `SKILL.md`
  - `session synthesis at \`.mochiko/memory/governance-intent.md\` (see` / `\`templates/governance-intent-template.md\`).` — `references/INTERROGATION-AGENDA.md`
- **Kept deliberately:** all in-body "the template's Shape N" contextual references (no file path — they resolve through the re-pointed shapes, which the governance-surfaces schema carries as shape-blocks per the record's §3 verbatim-preservation ruling); the `templates/constitution-modules/` pointer (`SKILL.md` line 219) and the `knowledge-management.md` module pointer (`references/INTERROGATION-AGENDA.md` line 45) — not in-scope templates, stay `.md`.
- **Consumers assessed:** n/a (single-writer skill + its reference). Doctrine-dense governance-surfaces / governance-intent schema fidelity (verbatim preservation) is V2's audit, not this re-point.

## [v0.65.0] `catalog/universal-floor.md` — single-level shelf doctrine superseded (two-row dial)
- **Disposition:** superseded → the shelf header in `references/catalog/universal-floor.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2/D5, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row; PO-D2 amended).
- **Content (verbatim):**
```
The floor *level* is the asserted
production level below — single, non-negotiable in level (nothing can lower it); a deviation is
only ever a **recorded waiver** (D4: justification in the governance ledger, permanent pending
the D4.1 revisit), never a loosened card.
```
  Also superseded, the same paragraph's category-definition pointer clause: `cards here carry the asserted level, never a second definition` → now `cards here carry the two-row `low`/`high` depth`.
- **Kept deliberately:** the floor-*concept* invariant (no session emits a floor-less constitution), "absence is always deliberate and auditable, never silent", the D4 recorded-waiver discipline (now framed as a per-check fit exception at both levels), and the audit-evidence-variants pointer to `COMPLIANCE-MODULES.md`. The breadth invariant (every category present at both rows) is the new companion to the surviving concept invariant.
- **Consumers assessed:** `ESSENTIAL-FLOOR.md` (category definitions — its own single-level line superseded in a sibling entry below) and `catalog/README.md` (doctrine — superseded below); no consumer links a removed anchor.

## [v0.65.0] `catalog/universal-floor.md` — four FLOOR cards' single `**Asserted level:**` lines superseded (two-row tables)
- **Disposition:** superseded → the FLOOR-SEC / FLOOR-TEST / FLOOR-ERR / FLOOR-OBS cards in `references/catalog/universal-floor.md`; each single asserted-level line becomes a `low`/`high` two-row table. Preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D5 retrofit-cost cut line, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim, the four superseded lines):**
```
FLOOR-SEC
**Asserted level:** secrets out of the repo (env vars + `.gitignore`) · secret scanning in CI ·
input validation at boundaries · auth enforced at all boundaries · dependency vulnerability
scanning blocking merge.

FLOOR-TEST
**Asserted level:** coverage pre-seed (session-overridable): ≥80% warning, ≥60% blocking ·
ratchet rule (baseline MUST NOT decrease) · a smoke test on the critical path exists from day
one.

FLOOR-ERR
**Asserted level:** failures never silently corrupt data · consistent error surface in the form
that fits the type (API error schema, UI error states, mobile/desktop failure surfaces) ·
correlation IDs · no leaked stack traces.

FLOOR-OBS
**Asserted level:** structured logs · correlation IDs · health checks (in the form that fits the
type) · no PII in logs.
```
- **Kept deliberately (D5's never-relax set holds at both levels — nothing lost, only redistributed):** the full obligation set survives, split across the two rows. Expensive-to-retrofit obligations are pinned to `low` (and therefore both rows): secrets-out-of-repo + input-validation + auth-at-boundaries (FLOOR-SEC), no-silent-data-corruption + no-leaked-stack-traces (FLOOR-ERR), no-PII-in-logs (FLOOR-OBS), ratchet + day-one smoke test (FLOOR-TEST). Addable rigor moves to `high`: merge-blocking scan gates (FLOOR-SEC — the scanners still *run* at `low`), coverage ≥80%/≥60% thresholds (FLOOR-TEST), error-schema + correlation IDs (FLOOR-ERR), structured logs + correlation IDs + health checks (FLOOR-OBS). Type-tags, Layer, and Content pointers untouched.
- **Consumers assessed:** `ESSENTIAL-FLOOR.md` worked examples (now annotated as the `high` row); `catalog/README.md` card-format field (superseded below); graders in Cluster C re-key to the two-row table form. No removed obligation.

## [v0.65.0] `catalog/universal-floor.md` — FLOOR waiver postures: staged-adoption-as-waiver superseded (PO-D7)
- **Disposition:** superseded/amended → the `**Waiver posture:**` lines of FLOOR-TEST (superseded), FLOOR-SEC and FLOOR-OBS (extended with the both-levels clause); preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D4/PO-D7 superseded, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim — the PO-D7 staged-adoption-as-waiver framing, now superseded because staged adoption is the `low` level):**
```
FLOOR-TEST
**Waiver posture:** D4 — recorded justification in the ledger (the young-team on-ramp, PO-D7: a
recorded waiver with the ratchet's starting point set from reality beats a silently ignored
threshold).
```
  FLOOR-SEC and FLOOR-OBS waiver postures were **extended, not removed** — the "available at either level as a per-check *fit* exception" clause and a "staged adoption is the **low** level, not a waiver (PO-D7 superseded)" line were added; their original bodies (prefer-narrowing example on SEC; most-waived-on-immature-stacks parenthetical on OBS) survive verbatim in place.
- **Kept deliberately:** the D4 recorded-justification-in-the-ledger discipline (both cards), FLOOR-SEC's prefer-narrowing example, FLOOR-OBS's honest-state parenthetical.
- **Consumers assessed:** `catalog/README.md` Waivers bullet (its own PO-D7 sentence superseded below); the governance ledger's waiver home is unchanged.

## [v0.65.0] `catalog/README.md` — "The asserted production floor" one-level doctrine section superseded
- **Disposition:** superseded → the `## The asserted production floor` section body of `references/catalog/README.md` (renamed `## The two-row production floor`); preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2/D7, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row; PO-D2 amended, PO-D7 superseded).
- **Content (verbatim):**
```
There is exactly one standard level — the production floor (PO-D2). The retired
`poc → internal → production → regulated` ladder is gone: no card carries per-tier defaults or
strictness ladders, and no session negotiates the floor's level.
```
  And the Waivers bullet's PO-D7 sentence:
```
A waiver is never silent: recorded in the synthesis and the ledger, it is
the honest staged-adoption on-ramp for early-stage teams (PO-D7). Accumulated waivers are the
governance re-entry checklist as the team matures.
```
- **Kept deliberately:** the ladder-stays-retired fact (recast as "two rows, never a four-rung tier ladder"), the Expression bullet in full, the Modules bullet (extended with the D7 level-blind clause + legal-mandate-stratum note), the D4 waiver discipline and D4.2 legal-mandate carve-out, and "accumulated waivers remain the governance re-entry checklist as the team matures".
- **Consumers assessed:** this file's own card-format section (superseded below); `COMPLIANCE-MODULES.md` (the level-blind clause now cross-stated there as a pure addition).

## [v0.65.0] `catalog/README.md` — card-format field + two "asserted level" bullets superseded
- **Disposition:** superseded → the card-format field and two doctrine bullets in `references/catalog/README.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D5, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**Asserted level / Recommended form:** [the single production-level content — thresholds,
             enforcement strength; floor-asserted cards state the asserted level, arbitrated
             cards the recommended form]
```
```
1. **Floor-asserted** — an Essential Floor card at the asserted production level, its expression
   shaped by type facts during the session; deviations only ever through recorded waivers
   (never a loosened card).
```
```
- **floor-asserted** — enters every session at the asserted level; not arbitrated; expression
  shaped by type; loosening only via recorded waiver.
```
- **Kept deliberately:** the field name (`**Asserted level / Recommended form:**` — retained as the field key; floor-asserted cards now populate it with a two-row `low`/`high` table, arbitrated cards still with the single recommended form), the arbitrated-card definition and arbitration-is-a-session-act paragraph, the Deck-kept and Minted source definitions, the graduation-seam section.
- **Consumers assessed:** Cluster C graders re-key to the two-row table in the field; no removed anchor.

## [v0.65.0] `ESSENTIAL-FLOOR.md` — single-and-asserted level doctrine line superseded
- **Disposition:** superseded → the floor-accounting paragraph of `references/ESSENTIAL-FLOOR.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
The floor's **level is single and asserted** — the production level on each
floor card in [catalog/universal-floor.md](catalog/universal-floor.md); nothing can lower it, and
a deviation is only ever a recorded waiver, never a loosened card.
```
- **Kept deliberately:** the file's canonical-home ownership header (single source for the four category *definitions*), the "account for all four floor categories with a principle or a recorded waiver" rule, the floor-concept invariant and never-silent absence, the four Detail-Requirements category checklists, and the four worked Example Principles (now carrying a depth note that frames them as the `high` row + a `low`-row coverage annotation — both pure additions). The NON-NEGOTIABLE labels are kept with a one-line clarifier (label = category presence / breadth invariant; depth = the dial).
- **Consumers assessed:** `analysis-codebase` (present/partial/absent read against these definitions — definitions unchanged); `catalog/universal-floor.md` and `authoring-constitution` SKILL both point here for definitions — pointers intact.
- **Landing reword (audit advisory F1, same ruling):** canonical-home blockquote phrase "greenfield writes them at the asserted floor level" → "greenfield writes them at the declared depth level's row" — disambiguation only ("level" reserved for the depth declaration per the Cluster-B terminology ruling), no responsibility moved.

## [v0.65.0] `DOMAIN-DEPENDENCIES.md` — one-universal-gate carrier line superseded (level-acknowledging)
- **Disposition:** superseded → the Growth-section carrier line of `references/DOMAIN-DEPENDENCIES.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D8 — pipeline process rigor uniform at both levels, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
One universal gate at the asserted floor (the retired tier fork is gone — PO-D2):
```
- **Kept deliberately:** the one-universal-gate model itself (uniform at both depth levels per D8; the retired tier fork stays gone), the explicit-ruling-before-registry-entry rule, the `domain_deps_added` non-empty auto-approve block, the visibility-floor disclosure, and the no-setup-re-run-for-additions rule — all untouched.
- **Consumers assessed:** the domain-layer rules file the registry block lives in; no removed anchor. The line now names the depth level (level-blind per D8) rather than implying a single asserted floor.

**Cluster B sub-wave (v0.65.0 — the setup/authoring-language half of the same ruling).** The seven
entries above (producer-a) convert the `catalog`/floor doctrine; the five entries below convert the
authoring skill body (`SKILL.md`) and the interrogation agenda (`references/INTERROGATION-AGENDA.md`)
that drive it. Same ruling: `production-floor-adaptive-depth`, ratified 2026-08-11, D1–D8;
`DECISIONS.md` 2026-08-11 adaptive-depth row. All five are supersessions **by ruling**. The pure
additions on these two surfaces (the flip-ceremony section, the no-pruning-license guard paragraph,
step 0 "Declare the depth level", and the depth annotations in the mandatory content inventory) ride
the decision row and take no strip entry. `governance-intent-template.md` and
`governance-surfaces-template.md` are separate primitives — their strips live in their own files.

## [v0.65.0] `SKILL.md` — greenfield-mode "floor cards at the asserted production level" superseded
- **Disposition:** superseded → the greenfield row of the "Two modes, one shared core" table in `skills/authoring-constitution/SKILL.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2/D5, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
floor cards at the asserted production level.
```
- **Kept deliberately:** the greenfield-mode definition and its "principles formulated from the synthesis's deck rulings + minted intents" clause, verbatim; the brownfield row untouched. The rewrite names the two-row `low`/`high` card form producer-a's `catalog/universal-floor.md` conversion authors.
- **Consumers assessed:** `SKILL.md` body only (this table is cited nowhere else); no removed anchor.

## [v0.65.0] `SKILL.md` — "the asserted level and waiver posture per category" ESSENTIAL-FLOOR bullet superseded
- **Disposition:** superseded → the ESSENTIAL-FLOOR "both →" bullet in the shared-core content-sources list of `SKILL.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
The asserted level and waiver posture per category
live on the floor cards in the catalog;
```
- **Also reworded for disambiguation (lead ruling #2, same citation):** the quality-gates inventory item 5 phrase `coverage pre-seeds from the FLOOR-TEST card's asserted level` → `coverage pre-seeds from the FLOOR-TEST card's coverage threshold`. "Asserted level" there named the coverage-threshold number, not the new depth level; reworded so "level" is reserved for the depth declaration.
- **Kept deliberately:** the ESSENTIAL-FLOOR canonical-definition pointer, the audit-evidence-variants clause pointing at `COMPLIANCE-MODULES.md`, and the fact-profile attachment clause — all verbatim.
- **Consumers assessed:** `SKILL.md` body only; `ESSENTIAL-FLOOR.md` itself has its single-level line superseded in producer-a's sibling entry above.

## [v0.65.0] `INTERROGATION-AGENDA.md` — agenda-test "the floor's level is the library's, asserted" block superseded
- **Disposition:** superseded → the **agenda test (PO-D3)** paragraph of `references/INTERROGATION-AGENDA.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2/D3, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**The agenda test (PO-D3):** every question elicits a **fact** — no question negotiates a
standard. The floor's level is the library's, asserted; the session sets its *shape* (product-kind
facts translate each floor category into its correct expression), its *triggers* (fact-profile
facts attach compliance modules mechanically), and its *path* (brownfield facts set the ratchet's
starting point, never its target). The one deliberately arbitrated layer is the
architecture-opinion card set (PO-D3's S7 carve-out).
```
- **Kept deliberately:** the agenda-test core (every question elicits a fact), the shape/triggers/path framing verbatim, and the S7 architecture-opinion carve-out. The rewrite ADDS the single depth-level declaration as the ONE arbitrated exception (D3's single dial), explicitly guarded against reviving a per-check tier ladder.
- **Consumers assessed:** `INTERROGATION-AGENDA.md` body (the "No pruning license" and step-1 sections re-keyed elsewhere this wave); the setup command binds this agenda — its harness elicits facts, now plus the one level declaration.

## [v0.65.0] `INTERROGATION-AGENDA.md` — dimension-2 "the floor's level is fixed" Feeds-cell tail superseded
- **Disposition:** superseded → the Feeds cell of the dimension-2 (Fact profile) row in the ten-dimensions table; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
The floor's level is fixed; facts shape its expression, never its strictness
```
- **Kept deliberately:** the dimension-2 module-trigger machinery and the consequence-stated-confirmation S4 fail-safe, verbatim. The tail is superseded because a depth level now exists — but it stays user-declared, not fact-derived (D1), so the "facts never set the level" spirit survives in the rewrite.
- **Consumers assessed:** the table's other rows untouched; no consumer cites this cell.

## [v0.65.0] `INTERROGATION-AGENDA.md` — step-1 "Assert the floor" opening (single production level) superseded
- **Disposition:** superseded → the opening clause of step 1 in "After the dimensions"; preserved verbatim here. A new **step 0 (Declare the depth level)** is added ahead of it (pure addition, rides the decision row).
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2/D3/D7, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim, the superseded opening — before "Type facts shape..."):**
```
1. **Assert the floor** — the four floor cards enter at the production level
   ([catalog/universal-floor.md](catalog/universal-floor.md)): presented, not negotiated.
```
- **Kept deliberately:** the rest of step 1 verbatim — presented-not-negotiated, type-facts-shape-expression, tightening-always-open, loosening-only-by-recorded-waiver, and the mechanical compliance-module attachment; steps 2–5 untouched. The rewrite ADDS the two-row `low`/`high` card selection (the declared level picks the low row or the high row of the card, per cluster A), the breadth invariant, and the D7 level-blind modules clause.
- **Consumers assessed:** the section heading gained "the depth level," and step 0 references the synthesis GI element + ledger; the setup command drives these steps in order — the added step 0 precedes the floor assertion.
- **Landing reword (audit advisory, same ruling):** the rewritten step-1 opening "the four floor cards enter at the production level" → "the four floor cards enter the session" ("level" reserved for the depth declaration), and the build-internal "(per cluster A)" tag dropped — the two-row form is stated in place, no external referent. Disambiguation/hygiene only, no responsibility moved.

## [v0.63.0] Guardrails cut — trigger + procedure prose removed, behavioral core kept; slim description

- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`variants/body/authoring-constitution/`, `variants/descriptions/authoring-constitution/`), one merged edit.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory, body 19,408 → 17,240 chars, −11%, the smallest proportional cut in the wave because most of the body is the v0.28.0 KEPT behavioral core):**
  - **Removed whole:** `## When to Use` (the greenfield/brownfield/amend/enforcement trigger list — carried by the description).
  - **Shortened:** `### 1. Enforcement` (794 → 386 — the enforcement-strength table removed, see reconciliation) · `# Greenfield branch (formulate the synthesis)` (1,007 → 189 — the three-step Floor/Type/Minted formulation walkthrough removed; the "deck rulings + minted intents ARE the selection; the job is formulation + routing quality" contract kept) · `# Brownfield branch (codify existing patterns)` (1,831 → 1,406 — the "Floor, assessed against the code, waiver-aware" action-mapping paragraph removed, see reconciliation; the Emergent-Ceiling paragraph, the evolution-notes attach, and the roadmap-stub blockquote kept).
  - **Kept intact:** the Overview and the native-surfaces routing table, `## The synthesis contract (selection vs. formulation)`, `## Surface routing (which content lands where)`, `## Two modes, one shared core` with the mode-prerequisites and D8 ownership-boundary blockquotes, `## When NOT to Use`, `## The Three-Part Principle Rule` with the Testability and Rationale subsections, `## Principle Writing Format`, `## RFC 2119 Keywords`, `## Mandatory content inventory` with the four-floor-accounting blockquote, `## Module assembly` (the module routing table including KM and layer-rules), `## Related`.
  - Old description (new slim form is 481 chars; **old verbatim, 1,517 chars**): "This skill MUST be invoked when authoring or amending a project's governance surface set — formulating enforceable principles from a ratified session synthesis (`.mochiko/memory/governance-intent.md`) and landing them on native Claude Code surfaces; there is NO constitution.md. The set: a marked, setup-owned governance region in `CLAUDE.md` (ratified stamp, principle index, universal principles as short imperative lines, tech stack, quality-gates summary, module pointers), `paths`-scoped `.claude/rules/` files for scope-bound principles, skill pointers for procedure-shaped standards, and a governance ledger at `.mochiko/memory/governance-ledger.md` (Three-Part metadata keyed by GI-ID, waivers, amendment/version policy, exception registry) — plus the trace summary manifest. Handles BOTH modes in one place: greenfield (authoring from the synthesis's deck rulings and minted intents) and brownfield (the same, additionally codifying an existing codebase's patterns — Essential Floor assessed against the code plus an Emergent Ceiling, informed by `.mochiko/memory/codebase-analysis.md`). SHOULD also invoke when the authoring work concerns principle enforcement, testability, rationale, the Three-Part Rule, RFC 2119 keywords, trace stamps, the fact profile, compliance modules, floor waivers, surface routing, the governance region, module assembly, an Essential Floor, or an Emergent Ceiling. The single governance-authoring skill for both new and existing projects — there is no separate brownfield skill."
  - Verbatim removed text survives in: git history; the before/after pair in `variants/`; archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (guardrails keep-set):** the entire v0.28.0-KEPT behavioral core except the two elements reconciled below — the synthesis contract, the surface-routing craft, the module-assembly routing table, both mode branches' contracts (deck-rulings-are-selection; floor assessed-against-the-code as a fact + Emergent Ceiling + the "Would I recommend this for a new project?" ceiling test — survives), the mandatory content inventory + floor-accounting blockquote, the mode-prerequisites and D8 blockquotes, the Three-Part Rule, and RFC 2119.
- **Protected-content reconciliation (MANDATORY — the wave's top protected case):** the `[v0.28.0] KEPT: the remaining body (under-band survivor ruling)` entry ruled the whole post-doctrine body a behavioral-core survivor. Two of its explicitly named survivors are touched by this cut and are recorded here as superseded-by-this-ruling; the rest survive intact.
  - **The enforcement-strength table (CI Automated / Code Review / Tooling / Audit, with the Strength column) — superseded by this ruling.** The v0.28.0 KEPT entry named it "the enforcement-strength table (CI/Review/Tooling/Audit — no other home)." The guardrails cut removes the table from `### 1. Enforcement`; the surrounding rule ("enforcement MUST fit the team reality … the teeth are CI, hooks, review, audit") survives. With no other in-tree home, the CI/Review/Tooling/Audit strength ranking now lives only in the three verbatim homes above.
  - **The brownfield "Floor, assessed against the code, waiver-aware" action-mapping paragraph — superseded by this ruling.** Named in the v0.28.0 KEPT as "floor-assessed-against-code waiver-aware authoring." The removed paragraph mapped present → codify-with-enforcement, absent → "MUST implement" + roadmap-gap reference, waived → ledger-waiver-record, plus the "the session already confronted … never re-litigate it" rider. The *fact* that the floor is assessed against the code survives (the Two-modes table and the brownfield lead-in's present/partial/absent read); the general waiver discipline survives (the synthesis contract's "Waivers are authored, not skipped" plus the mandatory-inventory floor-accounting blockquote). The specific present/absent/waived → action mapping and the no-re-litigate rider are superseded; verbatim in the three homes.
  - All other v0.28.0-KEPT survivors — the synthesis contract, surface routing, the module-assembly routing table, the ceiling test, the floor-accounting and mode-prerequisites blockquotes, and the inventory behavioral riders — **survive intact** (see the kept-section list above).
- **Consumers assessed (1 consuming command):** `commands/setup.md` — dispatches this skill to author/amend the governance surface set; relies on the produced surfaces + trace summary and the produce→validate→revise loop, not on any removed section anchor. Contract intact. (The wider citing set — `validation-constitution`, `principal-architect`, `analysis-codebase` — references the skill; none links a removed anchor.)


## [v0.44.0] Design-record citation, surface-routing preamble
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(design record: `.mochiko/brainstorms/constitution-native-surfaces/record.md`, D1–D8)
```
- **Kept deliberately:** the routing statement and its surface/disclosure table, untouched.

## [v0.44.0] Design-record citation in `references/DOMAIN-DEPENDENCIES.md`
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
Design record:
`.mochiko/brainstorms/domain-dependency-allowlist/record.md` (D1–D5 + review folds).
```
- **Kept deliberately:** the file's own purpose statement — single source for how the allowlist is seeded at interrogation and grows at implement time.

## [v0.28.0] Template restatements and homed mistake rows stripped (body 350 → 278, −21%, under-band)
- **Disposition:** deduped against pre-existing homes, each Read before landing (nothing written
  to `templates/` — D4's destination ban not engaged): the Principle Writing Format blocks
  (region-line example + full GI-XXX ledger record — verbatim-richer in
  `templates/governance-surfaces-template.md` Shapes 1/3), the Three-Part fenced
  Enforcement/Testability examples (four complete worked principles in
  `references/ESSENTIAL-FLOOR.md`; a worked-examples pointer added to the section intro), the
  RFC 2119 table → one-line keyword enumeration + reference pointer, the per-layer
  violation-test prose + delivery caveat (template Shape 2 preamble + Shape 1 standing line
  carry both, incl. the kinako example and observed-behavior wording), the D8 ownership detail
  and trace-mechanics parentheticals (template comment block), the EMERGENT-CEILING pattern-list
  parenthetical (the reference's Discovery step 2) · **Common Mistakes deleted whole** (wave-3
  precedent — all 15 rows homed: rows 1–4 the Three-Part sections, 5–7 the synthesis-contract
  bullets, 8–13 the routing bullets + D8 + template preambles, 14 tier parameterization
  (greenfield step 1 + `catalog/universal-floor.md` + validator tier checks), 15 greenfield
  step 1's backend-flavored warning + the catalog's seed-honesty note) · deleted (Tier 2): the
  Related section's "(Retired 2026-07-18: `syncing-claude-md` stub)" line — historical
  bookkeeping, names no behavior; recorded here: governance lives ON CLAUDE.md, no copy left to
  synchronize · densified: inventory items 1/6/7 (Shape-3 field restatements → one line each;
  every behavioral rider kept inline — actual-commands, FLOOR-TEST pre-seed, gates-omitted-for-
  waived, waiver-posture, floor-first, NON-NEGOTIABLE marking), Related 12 → 4 (validator bullet
  folded into the Overview with the "never co-mounted" clause preserved; roadmap-stub bullet cut
  — the brownfield blockquote is the surviving home), Overview table cells slimmed (width-only,
  no line delta)
- **Tier failed:** 1 throughout (every cut had a verified home, most in the mandated-read
  template) · 2 for the retired-stub line only
- **Content:** two format blocks, two fenced examples, one 3-row table, the 15-row mistakes
  table, routing/caveat/ownership prose, two Related bullets. Forecast disclosure: proposal
  forecast −26%; landed −21% (inventory item 8 was already tight — untouched; the table-cell
  slim is width-only)
- **Consumers assessed:** wave-open enumeration — 10 citing files (incl. `validation-constitution`
  + its QUALITY-CHECKLIST, `principal-architect`, setup, `analysis-codebase`); none links a
  section anchor of this SKILL. Shared-home audit: `references/ESSENTIAL-FLOOR.md` (3 external
  consumers, ownership header intact) untouched

## [v0.28.0] KEPT: the remaining body (under-band survivor ruling, 21% vs 30–70)
- **Tier-2 evidence:** this body was rewritten at the constitution-native-surfaces dissolution
  (2026-07-18) — a post-doctrine body whose remaining mass is behavioral core with no other home:
  the synthesis contract (selection-vs-formulation — the skill's constitutional discipline), the
  surface-routing craft, the module-assembly routing table (unique rulings incl. the KM and
  layer-rules routes), both mode branches (floor-assessed-against-code waiver-aware authoring;
  the "Would I recommend this for a new project?" ceiling test — checked: not in
  EMERGENT-CEILING-PATTERNS), the enforcement-strength table (CI/Review/Tooling/Audit — no other
  home), the floor-accounting blockquote, the mode-prerequisites blockquote, and the inventory's
  behavioral riders. D1: the band is a calibration bar, not a quota. Feeds the wave-4-close
  watch-item (post-doctrine skills run under-band). Session ruling: wave-4 batch-1 ratified
  2026-07-25.

## [v0.22.0] `references/SYNC-IMPACT-FORMAT.md` deleted (dead)
- **Disposition:** deleted (D6b) — full content in git history (`plugins/mochiko/skills/authoring-constitution/references/SYNC-IMPACT-FORMAT.md`, removed at v0.22.0)
- **Tier failed:** dead reference: it documented the SYNC IMPACT REPORT — an HTML-comment changelog "embedded at the top of the constitution file" — but no `constitution.md` exists anywhere since the v3 dissolution (constitution-native-surfaces, 2026-07-18, no-backward-compatibility ruling); zero inbound references (unmentioned in the SKILL.md body, epic F-a)
- **Content (compressed):** version-change header (X.Y.Z → A.B.C with MAJOR/MINOR/PATCH rationale), modified/added/removed section lists, and a per-template alignment status table (✅/⚠️/❌) — the governance ledger's amendment/version policy now carries this concern on the native surfaces.
