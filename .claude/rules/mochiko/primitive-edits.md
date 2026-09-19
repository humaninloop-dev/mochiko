---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
  - "plugins/mochiko/migrations/**"
  - "plugins/mochiko/hooks/**"
---

# Primitive-edit ceremony (strip / supersede → record → check)

Editing a shipped primitive is a **landing, not an ad-hoc edit**. Any change that REMOVES or
SUPERSEDES content — even one line, even an "obvious" cleanup — obliges both moves before the
change is done. Full contracts: `.mochiko/strips/README.md`.

**Schema content** — every command's and skill's rules, the family common blocks, the label
registries, the artifact templates, the shelf data — lives in the migration log at
`plugins/mochiko/migrations/` and is delivered at fire by `mochiko-cli` (`cli-schema-delivery`
D1–D3, D9 wave 6). No schema file ships. Editing schema content is a new migration file under the
log (grammar in the log's README), validated by `mochiko-cli migrate validate`; the migration
carries its ruling anchor where the hard set demands one, and the verbatim prior content is in the
log by construction, so schema-content edits take no strip entry. The human-readable projection is
`.mochiko/schema-views/`, regenerated never hand-edited.

- **Record** — a version-stamped entry in `.mochiko/strips/<primitive>.md` (one file per primitive,
  newest-first; stamp = the `plugin.json` version that made it):
  - a **strip entry** for an altitude / duplication cut — `Disposition: relocated → <home> | deleted`,
    `Tier failed: 1 (altitude) | 2 (no behavior named)`, verbatim content;
  - a **supersession-by-ruling entry** for a decision — `Disposition: superseded`,
    `Tier failed: n/a — supersession by ruling`, citing the ruling: a `DECISIONS.md` row + a
    `.mochiko/decisions/` ADR when no session record exists, verbatim content, `Kept deliberately`,
    `Consumers assessed`.

  An edit whose only trace is the changed file, with nothing in `.mochiko/strips/`, is **incomplete**.

- **Check** — one binary gate audit over one unit, run by **the gate grader**: a plain fresh seat
  (`general-purpose`, no persona) that authored nothing in the unit, spawned with an **explicit
  `model:` alias** equal to the tier the edit was produced at and never below — `opus` when the
  lead made the edit. An omitted alias is a floor miss
  (`patterns-model-tiering.persona-less-grader-pin`). The seat's contract is
  `mochiko:validation-primitive-edit`'s render pasted verbatim into the brief; the dispatcher
  writes only the unit, its file paths, and the pre-pass commands, and a hand-written contract
  section is a floor miss on the same terms as an omitted alias. **One seat takes every unit of a
  wave** (`author-grader-consolidation` D11): each unit keeps its own verdict block and its own
  outcome line tagged with the seat, and the seat splits into two only when the units' files
  would not fit its context, saying so in the lines. The editor never grades their own edit.

  **The deterministic pre-pass runs first, and the grader runs it.** `mochiko-cli migrate
  validate --report --plugin-root plugins/mochiko` (0 rejecting to proceed; advisory findings
  noted, never gating) and the char-budget measurement are executed by the grader itself and
  quoted in the verdict block. A pre-pass result quoted from the brief is not evidence, and
  nothing that output asserts is re-derived by judgment.

  **Char-budget pre-assert (D7).** The grader counts the edited primitive's budgeted classes —
  skill body, skill `description:` value, agent `description:` value — as **characters of the
  parsed value, never `wc -c` bytes**, against `.mochiko/memory/primitive-cost-budgets.md`
  (canonical measurement snippet lives there). Over budget = FAIL, unless the editor named the
  overage in the audit brief with a justification the grader rules holds (a genuine new
  obligation — never restored playbook prose). `references/` files are exempt. Primitives
  without a measured budget fall back to hard caps only (skill `description:` ≤ 1,536 delivery
  cap); budgets are never invented.

  **The graded unit is keyed by kind, and the criteria follow the unit.** For a **command**, the
  graded unit is the command's own **pair** — `plugins/mochiko/commands/<cmd>.md` + the command's
  rules as `mochiko-cli` renders them from the log — held against the canonical-scaffold criteria
  below. (This supersedes the "the command's own text" bar of ADR
  `2026-08-02-doctrine-purge-wave-1` decision 4; ruling:
  `command-md-scaffold-standardization` D1, C1 fold.) For a **schema-bearing skill** the graded
  unit is likewise the pair — `SKILL.md` + that skill's rendered rules — held
  against the skill-pair criteria block below
  (skill-content-schema D8/I6; the matching-skill routing never applies to the pair). For
  **schema content** — a migration file plus its regenerated view diff — the unit is that pair
  and the items are the AM-2 five: intent stated · anchor present where required · ID lifecycle
  right · floor and fail survival · register. For every other primitive — the seven prose skills
  and the router included — the unit is the file, graded on internal coherence plus preserved
  responsibilities, with the matching `validation-*` / `review-*` skill reached as the domain
  lens where one exists (`templates/command-shape.md` was deleted at v0.46.0; the dedicated
  `validation-command-shape` skill at v0.45.0).

  **The loop is bounded, and the bound has one home.** A FAIL sends the edit back for a fix and a
  re-audit — by the **same grader seat resumed**, reading only what the fix touched and what it
  could have broken, never the whole cluster again. How many such rounds a landing gets is
  `common.gate-loop-bound`'s to say and nowhere else's; this file and
  `validation-primitive-edit.gate-loop-bound` cite that id rather than restate what it holds, so a
  rename or tombstone of it sweeps them both. A second FAIL halts the landing and goes to the user
  with both fix lists; the user rules — fix again, or drop the edit. No run raises this bound.
  Overruling a grader the user judges wrong rides the governance ledger's existing waiver path.

  **The landing carries the audit's outcome line.** Each gate audit writes one line — `audit:
  <unit> · <seat> · <tier> · <n> files · <n> rounds · <n> blocking[ · cost: $<x>]` — into the
  record of the landing it belongs to: the wave's `build-log.md` entry when the edit ships from a
  session, the `.mochiko/decisions/` ADR when it is an ad-hoc defect close. The `cost:` field is
  present only for an audit run as a launched session, read from the SDK's per-session
  `total_cost_usd`; an in-session spawn carries none. A landing whose audits left no lines is
  incomplete.

  Each criterion below is tagged for where its answer comes from: **[CLI]** — asserted by
  `mochiko-cli migrate validate`, read from the pre-pass output and never re-derived by
  judgment · **[suite]** — asserted by the plugin contract suite's `converted-shape` host case ·
  **[judgment]** — the grader's own, confirmed once with one line of evidence. A criterion with
  two tags has a mechanical limb and a judgment limb, named in the item.

  **Canonical-scaffold criteria — every pair-form command, all six commands.** A command ships as
  a `.md` whose rules `mochiko-cli` renders from the log, and is graded across **both surfaces** on
  one criteria set. There is no second block and no per-form exception: the library has one
  scaffold (`command-md-scaffold-standardization` D1/D2), and the only branch is the
  done-condition class at the end of this list.

  1. **Scaffold conformance** — **[judgment]**, except the `allowed-tools` grant (**[suite]**).
     The `.md` carries the canonical headings in the canonical
     order — frontmatter (`description` · `argument-hint` ·
     `disable-model-invocation: true` · `allowed-tools: Bash(mochiko-cli *)` — a required
     key set; YAML key order is not graded, though all six ship in this order) ·
     `# <Name> — <epithet>` · `## Identity & Mission` (one tight
     section, never materially delaying the Rules block) ·
     `## Rules — delivered by mochiko-cli` · `## Adaptive Goal Protocol` with its three
     steps **Entry** → **Goal** → **Not done — default FAIL** (last). `$ARGUMENTS` is
     handled in Entry; the Not-done line cites the CLI-printed pin rather than carrying a
     count (criterion 3). No `**Goal:**` opener line, no `Harness` / `Bindings`
     sections, no per-command extra top-level section. Nothing mechanical reads these headings:
     the suite asserts only the literal `allowed-tools: Bash(mochiko-cli *)` string.
  2. **Rules-block enumeration** — **[suite]** for the enumeration, **[CLI]** for the schema's own
     set, **[judgment]** for tokens in prose. The section IDs enumerated in the Rules block match the
     schema's section IDs **set-wise** — the six-set `<cmd>.sec.roles` · `reserved` ·
     `tools` · `ways-of-working` · `boundaries` · `fail-conditions`, all six present in
     every schema, a section with no rules carrying its explicit empty marker (D4/D5).
     Every `<cmd>.sec.*` token anywhere in the `.md`, inside the Rules block or outside
     it, resolves to a live node.
  3. **FAIL survival** — **[CLI]** (`fail-segment`, `tombstone-integrity`, `mint-once`),
     **[judgment]** for the `.md`'s citation and halt clause, and for the lifecycle a tombstone's
     free-text `disposition` claims — "a reword keeps its ID" is a disposition the grader reads,
     not a check the binary makes (criterion 4). It keys to **`kind: fail`**
     (ontology D1, build item 4): every
     `kind: fail` rule survives (a reword keeps its ID), and the correspondence
     between the `<cmd>.fail.*` ID segment and `kind: fail` holds in both directions —
     `kind:` is never defaulted on a `.fail.*` ID. The
     hand-pinned count is gone by ruling (`cli-schema-delivery` D3: the counts are
     computed and printed by the CLI, never hand-pinned): the pin is the
     `- kind: fail · N rules` line the render prints under `pins` in the preamble block,
     and the `.md`'s Not-done line cites that pin and obliges a halt-and-surface when a
     delivered section's end-line count disagrees with it. Grade the citation and the
     halt clause; a hard-coded number there is the defect, not its absence.
  4. **ID continuity (D11/D14)** — **[CLI]** for the mechanics (`tombstone-integrity`,
     `mint-once`, `id-format`, `id-prefix`, `id-duplicate`, `cite-unresolved`), **[judgment]** for
     a reference carried in prose rather than in a structured field, and for the lifecycle each
     disposition claims. The binary checks only that an id is tombstoned once, is never both live
     and tombstoned, and carries a non-empty `disposition`; the disposition is free text, so the
     grader reads each tombstone's and each supersession's disposition and confirms the lifecycle
     it claims actually happened — a reword that kept its id, a split whose children record the
     parent, a merge whose losers are the ones tombstoned.
     No `<cmd>.*` ID — rule **or** `<cmd>.sec.*` section —
     vanishes without a tombstone. A reword keeps its ID, a split mints children recording
     the parent, a merge tombstones the losers; no surviving rule text references a
     tombstoned or re-homed node.
  5. **`class: floor` = must-survive (M3)** — **[CLI]** for the exit and its anchor
     (`protected-exit`, `anchor-format`), **[judgment]** for whether the cited ruling covers it. A
     `floor`-class rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  6. **Substance across the pair** — **[judgment]**. Plan approval before any producing seat works ·
     author ≠ grader independence (no self-grading seat row) · decisions reserved to the
     user, carried in `<cmd>.sec.reserved` · bindings complete — paths, templates, entry
     condition — in `<cmd>.sec.tools` and the Entry step · the non-waivable floor in
     `<cmd>.sec.boundaries`. The floor includes the sound-loop pointer line
     `mochiko:patterns-sound-loop` on the three DM-chartered commands — `architecture` ·
     `feature` · `implement` (`charter-ritual-balance` D3) — counted on whichever surface
     carries it; the scaffold does not extend that pointer to `brainstorm` · `setup` ·
     `specify`, and an audit must not demand it there. Where the command is DM-chartered,
     the DM's bare-minimum responsibilities are present as owned responsibilities.
  7. **Done-condition class — grade the branch that matches the command, and only that
     branch** — **[judgment]**.
     - **Desk commands — `architecture` · `feature` — per-visit contract.** The protocol
       converges *each visit*, with the user, to a one-line goal **and its explicit done
       condition**, then runs to it and closes with a verdict against it. A visit ending
       with no stated done-condition verdict is a defect. Do **not** demand a fixed done
       condition here.
     - **Run commands — `brainstorm` · `implement` · `setup` · `specify` — fixed
       contract.** The **Entry** step carries the entry gating and, where the command
       routes, the neither-source routing; the **Goal** step states a **fixed** done
       condition; **Not done** defaults it to FAIL and is count-pinned. Do **not** demand
       a negotiated per-run goal — that is the desk's form, not the run's.
     - **`implement` additionally** (ADR `2026-08-13-charter-plan-implement` ruling 3):
       convergence at a named EXISTING user gate — run-open confirmation naming batch,
       scope type, attempt bounds (redeclarable there and only there), and the fixed done
       condition stated — closing at the existing acceptance gate. No new ceremony: the
       gate must already exist in the run.
  8. **Preserved responsibilities** — **[judgment]**, as for any primitive: protected content
     leaves only by recorded supersession-by-ruling, and strips + budgets apply unchanged.
  9. **Deterministic pre-pass** — **[CLI]**. `mochiko-cli migrate validate --report --plugin-root
     plugins/mochiko` is **run by the grader** and its output quoted, beside the char-budget
     measurement. (The Python checkers it replaces retired at v0.107.0.)
  10. **Provenance anchors (D16 as re-keyed)** — **[CLI]** for the anchor's presence and format,
      **[judgment]** for whether the ruling covers the exit. Decision anchors live on the log's own
      rules, carried by the migration that writes them — a supersession or tombstone of
      protected content carries its anchor in the migration, and the binary enforces that
      at apply. An anchored rule still leaves only by recorded supersession-by-ruling. The
      former sidecar is frozen at `.mochiko/archive/provenance-frozen-2026-09-05.yaml`,
      for provenance queries only.
  11. **Ontology-grammar conformance (D1–D8)** — **[CLI]** for the grammar (`rule-kind-unknown`,
      `class-unknown`, `condition-declaration`, `when-undeclared`, `when-value`,
      `moment-undeclared`, `moment-declaration`, `enforces-required`, `enforces-unresolved`,
      `enforces-misplaced`, `extends-unresolved`, `extends-class-local`, `extends-cross-family`),
      **[judgment]** for the extraction bar and the MOVE/DECLARE call. Across the pair: every
      `kind:` value comes
      from the nine-kind closed set — `constraint` · `duty` · `gate` · `reservation` ·
      `binding` · `bound` · `routing` · `fail` · `latitude` — with `constraint` the omitted
      default (an absent `kind:` reads `constraint` and is never written) · the schema
      declares the `conditions:` and `moments:` blocks its own rules use, and every `when:`
      term and every moment-resolved resolution point resolves against them · a rule-level
      activation guard is single-homed in `when:` and has left the `text` (MOVE), except
      where the condition rides the rule's subject noun and extraction would falsify the
      text or strand a referent, where `when:` is added and the text stands unchanged
      (DECLARE) · a `class: floor` rule is always read and always delivered whatever its
      `when:` — `when:` gates when the obligation applies, never whether it is delivered ·
      every `kind: fail` node carries `enforces:`, each listed ID resolving to a live local
      rule, an empty list legal only with its one-line reason · an `extends: common.<slug>`
      stub inherits `text` / `labels` / `pointer` only, declares `class:` locally (C3), and
      binds only where the block carries the command's responsibility under the extraction
      bar — exact duplicate across 3+ commands (ontology D8), or a 3+-command near-identical
      family converged under strongest-wording-wins (near-dup convergence ruling R1/R2,
      `.mochiko/decisions/2026-08-28-near-dup-convergence.md`; a member whose extra content
      is command-specific keeps local text, the edge recorded in
      `scripts/similar-rules-allowlist.yaml`). No co-Read of a common file is demanded: the
      render resolves every stub before the model sees it.

  Rulings: `.mochiko/brainstorms/command-md-scaffold-standardization/record.md`
  D1–D7 (`DECISIONS.md` 2026-08-27 — the canonical scaffold; supersedes the charter-form /
  goal-form split and this block's former dual-block shape, clause inventory in that
  record's Appendix A) · `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`
  D10 · `.mochiko/decisions/2026-08-13-charter-plan-implement.md` ·
  `.mochiko/brainstorms/charter-ritual-balance/record.md` D3 (`DECISIONS.md` 2026-08-13) ·
  `.mochiko/brainstorms/command-content-schema/record.md` D9 · D11 · D14 · D16
  (`DECISIONS.md` 2026-08-26) ·
  `.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 (`DECISIONS.md`
  2026-08-27 — the run-shape grammar; amends command-content-schema D6, and D3 narrowly) ·
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 (`DECISIONS.md` 2026-08-28 —
  widens the D8 extraction bar to 3+-command near-identical families,
  strongest-wording-wins).

  **Skill-pair criteria — every schema-bearing skill (the thirty); the review family from
  v0.100.0.** The seven prose skills and the router carry no rule set, never had a schema,
  and take the plain primitive ceremony above, not this block. A schema-bearing skill
  ships as `SKILL.md` whose rules `mochiko-cli` renders from the log
  (the skill directory stays the self-contained shipping unit) and is graded
  across **both surfaces** on this criteria set. The grader is the gate grader defined above,
  exactly as for a command pair; the matching `validation-*` / `review-*` skill routing never
  applies to the pair (no validator-for-skills exists, and a pilot member never
  grades itself). This block is a **sibling** of the command block above, never a fork of
  it: skills are their own grammar family — no `moments:`, no `$ARGUMENTS` protocol, no
  Not-done count-pin — under the same governance envelope.

  1. **Load-first section** — **[suite]** for the `!`-line enumeration and the `allowed-tools`
     grant, **[judgment]** for the `## Rules — delivered by mochiko-cli` heading itself, the
     no-raw-Read clause, and the reference-read sequencing. The `SKILL.md` body carries a
     `## Rules — delivered by mochiko-cli` section whose seven `!` lines are the
     enumeration. No raw Read of a schema or of a family common file is demanded: the
     render resolves every `extends:` stub and every `${var}` before the model sees it, and
     prints the reading grammar — `when:` interpretation, floors always delivered, stub
     inheritance limits — as the preamble's `legend`. A member's own obligated reference
     read (e.g. `review-feasibility`'s lens) sequences in that section.
  2. **Section enumeration** — **[suite]** for the `.md` to render match, **[CLI]**
     (`section-set`) for the schema's own set, **[judgment]** for tokens in prose. The section IDs
     enumerated in the load-first block match the
     schema's section IDs **set-wise** — the skill's **family section set**, minted once
     by that family's census-backed rollout ruling, uniform within the family, every
     section present in every member schema, a section with no rules carrying its
     explicit empty marker. Sets minted so far: the review family's
     `<skill>.sec.independence` · `scope` · `inputs` · `verdict` · `output` · `reserved`
     (census §H, v0.100.0); the authoring family's, swapping `verdict` for `artifact` —
     `independence` · `scope` · `inputs` · `artifact` · `output` · `reserved`
     (census-authoring J-1, v0.101.0); the patterns family's — a full swap-out, not one
     slot — `trigger` · `scope` · `discipline` · `inputs` · `disclosure` · `reserved`
     (census-patterns §B/J-P7, v0.102.0). The small families' dense five —
     `testing-end-user` · `testing-gap-finding` · `executing-tdd-cycle` ·
     `brownfield-integration` · `analysis-codebase` — mint no set of their own: they
     REUSE the review six-set with explicit empty markers, by the 2026-09-01
     small-families door ruling (census-small-families §B fit table, v0.103.0).
     Every `<skill>.sec.*` token
     anywhere in the `.md`, inside the load-first block or outside it, resolves to a live
     node. "The load-first block" reads as the delivered section's
     `--section` arguments — the six family ids in the preamble's printed order, behind the
     `preamble` line — and the set-wise match is graded against those.
  3. **Floor-count pin + read-back** — **[CLI]** for the printed pin, **[judgment]** for the
     sentence that cites it, **[suite]** for the frozen floor set in `expected-skills.json`.
     The load-first block obligates stating the floor
     count back before the first procedural step (the delivery read-back,
     skill-content-schema D6 as amended). The hand-pinned count is
     gone by ruling: the pin is the `- class: floor · N rules` line the render prints under
     `pins` in the preamble block together with the `floors:` index line beneath it, and the
     read-back sentence cites both — a hard-coded number there is the defect, not its
     absence.
  4. **Floor survival** — **[CLI]** (`protected-exit`, `anchor-format`), **[judgment]** for
     whether the cited ruling covers it. A `class: floor` rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  5. **ID continuity** — **[CLI]** for the mechanics (`tombstone-integrity`, `mint-once`,
     `id-duplicate`, `cite-unresolved`), **[judgment]** for the lifecycle each disposition
     claims. The binary checks only that an id is tombstoned once, is never both live and
     tombstoned, and carries a non-empty `disposition`; the disposition is free text, so the
     grader reads each tombstone's and each supersession's disposition and confirms the
     lifecycle it claims actually happened — a reword that kept its id, a split whose children
     record the parent, a merge whose losers are the ones tombstoned.
     No `<skill>.*` ID — rule or `<skill>.sec.*` section — vanishes
     without a tombstone. A reword keeps its ID, a split mints children recording the
     parent, a merge tombstones the losers; no surviving rule text references a tombstoned
     or re-homed node.
  6. **`extends:` conformance** — **[CLI]** (`extends-unresolved`, `extends-cross-family`,
     `extends-class-local`), **[judgment]** for the near-dup bar. An
     `extends: <family>-common.<slug>` stub binds only the
     skill's own family library (cross-family sharing forbidden, D5), inherits
     `text` / `labels` / `pointer` only, declares `class:` locally, and binds only under the
     near-dup bar — near-identical across 3+ members, strongest-wording-wins (R1/R2,
     `.mochiko/decisions/2026-08-28-near-dup-convergence.md`); a member whose extra content
     is skill-specific keeps local text, the edge recorded in
     `scripts/similar-rules-allowlist.yaml`. The stub's `<skill>.*` ID stays the citable
     ID. No co-Read of a family common file is demanded: the render resolves every stub
     before the model sees it.
  7. **`description:` untouched** — **[judgment]** (a diff the grader reads; the cap is measured
     with the canonical snippet). The frontmatter `description:` value is byte-identical
     across the conversion and ≤ 1,536 chars (the delivery cap); it never moves to schema. For a
     skill born in its wave there is no prior value to be identical to: the item reads
     "≤ 1,536 chars" only.
  8. **Budget = delivered-at-invoke payload** — **[CLI]** for the measurement (the render plus the
     ledger's canonical snippet), **[judgment]** for any overage argument. The budgeted quantity is the `SKILL.md` body
     plus the seven rendered blocks the `!` lines deliver, one number, characters of the
     parsed value (`cli-schema-delivery` D10 clause 6 — no schema file is read at invoke, so
     none is part of the payload). The budget re-seeded to that measured figure at
     conversion with **no +25% headroom** (the ledger's third seeding path — the conversion
     is a relocation, never a measured winner); content growth takes the normal
     argued-overage path, named in the brief. The hook lines are excluded as the harness's,
     not the primitive's; `references/` and `scripts/` stay exempt. `migrate validate`'s
     `budget` finding is advisory and counts rule text only — it is never the ledger comparison.
  9. **Pointer resolution** — **[CLI]** (`pointer-unresolved`). Every `pointer:` resolves
     base-dir-relative from the skill
     directory, cross-directory climbs included (`../<other-skill>/references/...` is
     legal; the Single-source convention governs the pointed-at files).
  10. **Deterministic pre-pass** — **[CLI]**. `mochiko-cli migrate validate --report
      --plugin-root plugins/mochiko` is **run by the grader** and its output quoted, beside the
      char-budget measurement. (The
      Python checkers it replaces retired at v0.107.0.)
  11. **Skill-grammar conformance** — **[CLI]** (`skill-grammar`, `rule-kind-unknown`,
      `when-undeclared`, `moment-declaration`). Every `kind:` value comes from the **eight-kind**
      skill set — `constraint` · `duty` · `gate` · `reservation` · `binding` · `bound` ·
      `routing` · `latitude` — with `constraint` the omitted default; **`kind: fail` and
      `enforces:` are illegal in a skill schema** (census-evidence retirement,
      skill-content-schema D9/M2). Every `when:` term resolves against the schema's
      declared `conditions:`; a `class: floor` rule is always read and always delivered
      whatever its `when:`; no `moments:` block exists (procedure stays prose, D3).
  12. **Provenance anchors** — **[CLI]** for the anchor mechanics, **[judgment]** for whether the
      ruling covers the exit. Decision anchors live on the log's own rules, carried by the
      migration that writes them and enforced by the binary at apply; a rule carrying a
      supersession-transfer (a `KEPT:`-protected or `DECISIONS.md`-traceable line relocated
      into schema content, skill-content-schema D8/C4) inherits protected status through
      that anchor and leaves only by recorded supersession-by-ruling. The former sidecar is
      frozen at `.mochiko/archive/provenance-frozen-2026-09-05.yaml`.

  Rulings: `.mochiko/brainstorms/skill-content-schema/record.md` D1–D9 as amended
  (`DECISIONS.md` 2026-09-01) · the census inventory
  `.mochiko/brainstorms/skill-content-schema/research/census.md` (§E kind retirement · §H section
  set · J-7 cross-directory pointers) · the authoring-family census
  `.mochiko/brainstorms/skill-content-schema/research/census-authoring.md` (§I labels · J-1
  section set · J-6 budget · J-7 first-seeds) · the patterns-family census
  `.mochiko/brainstorms/skill-content-schema/research/census-patterns.md` (§B section proposal ·
  §ROAD rejection · §I labels · J-P2 first-strips · J-P5 two-arm/overage) · the
  small-families census
  `.mochiko/brainstorms/skill-content-schema/research/census-small-families.md` (§B six-set-reuse
  fit table · §C zero common blocks · §D abort-tripped — the dense five convert on the
  B/C drivers · J2-8 dual-homing twins · J2-9 ruled repair) ·
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 ·
  `.mochiko/brainstorms/author-grader-consolidation/record.md` D2–D7, D9, D11 (`DECISIONS.md`
  2026-09-19 — the gate form: the unit-keyed judgment items, the first-hand pre-pass, the plain
  grader seat, the bounded loop, the outcome line).

**Persona edits with an eval kit carry an advisory grid read.** When the edited primitive is
`plugins/mochiko/agents/<persona>.md` and `evals/agents/<persona>/` exists, the audit brief cites a
`pre`/`post` plan-only grid — `uv run evals/run.py agent grid <persona> --arms pre,post`
(`pre` at the kit's pinned `refs.pre`, `post` the working tree), judged and reported — and the report's
regression, adoption, and band lines are quoted in the strip entry (or the decision row for a pure
addition). Advisory only, never a gate (ADR `2026-09-09-persona-edit-advisory-grid`, harness D2,
GI-019); the landing re-pins `refs.pre` to the landed ref.

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
