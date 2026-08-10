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
