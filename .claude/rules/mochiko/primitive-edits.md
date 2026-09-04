---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
  - "plugins/mochiko/schemas/**"
  - "plugins/mochiko/migrations/**"
  - "plugins/mochiko/hooks/**"
  - ".mochiko/provenance.yaml"
---

# Primitive-edit ceremony (strip / supersede → record → check)

Editing a shipped primitive is a **landing, not an ad-hoc edit**. Any change that REMOVES or
SUPERSEDES content — even one line, even an "obvious" cleanup — obliges both moves before the
change is done. Full contracts: `.mochiko/strips/README.md`.

**Schema data files** (`plugins/mochiko/schemas/*.yaml`, and from v0.100.0 the in-directory
skill schemas `plugins/mochiko/skills/*/schema.yaml` — skill-content-schema D2/D8) are shipped
primitives from v0.76.0 (schema-based-template-guidance D8 — data = source of truth, the binary
renders over them). An edit to one takes the same strip + author≠grader ceremony as any
command / skill / agent / template edit; the path scope above covers them so this reminder
injects on a schema Read. From v0.104.0 a **converted** command's rules are no longer served
from its schema file at all: they are rendered at fire by `mochiko-cli` from the migration log
the plugin carries at `plugins/mochiko/migrations/` (`cli-schema-delivery` D3/D4), so editing
that content means adding a **new migration file** under the log — the grammar is in the log's
own README — never an in-place edit of a migration already applied.

- **Record** — a version-stamped entry in `.mochiko/strips/<primitive>.md` (one file per primitive,
  newest-first; stamp = the `plugin.json` version that made it):
  - a **strip entry** for an altitude / duplication cut — `Disposition: relocated → <home> | deleted`,
    `Tier failed: 1 (altitude) | 2 (no behavior named)`, verbatim content;
  - a **supersession-by-ruling entry** for a decision — `Disposition: superseded`,
    `Tier failed: n/a — supersession by ruling`, citing the ruling: a `DECISIONS.md` row + a
    `.mochiko/decisions/` ADR when no session record exists, verbatim content, `Kept deliberately`,
    `Consumers assessed`.

  An edit whose only trace is the changed file, with nothing in `.mochiko/strips/`, is **incomplete**.

- **Check** — first the deterministic **char-budget pre-assert (D7)**, then the model-judgment
  audit. The grader counts the edited primitive's budgeted classes — skill body, skill
  `description:` value, agent `description:` value — as **characters of the parsed value, never
  `wc -c` bytes**, against `.mochiko/memory/primitive-cost-budgets.md` (canonical measurement
  snippet lives there). Over budget = FAIL, unless the editor named the overage in the audit
  brief with a justification the grader rules holds (a genuine new obligation — never restored
  playbook prose). `references/` files are exempt. Primitives without a measured budget fall
  back to hard caps only (skill `description:` ≤ 1,536 delivery cap); budgets are never invented.

  Then the independent **author ≠ grader** audit. For a **command**, the graded unit is the
  command's own **pair** — `plugins/mochiko/commands/<cmd>.md` +
  `plugins/mochiko/schemas/<cmd>.yaml` — held against the canonical-scaffold criteria below.
  (This supersedes the "the command's own text" bar of ADR
  `2026-08-02-doctrine-purge-wave-1` decision 4; ruling:
  `command-md-scaffold-standardization` D1, C1 fold.) For a **converted skill** the graded
  unit is likewise the pair — `SKILL.md` + the skill's in-directory `schema.yaml` — held
  against the skill-pair criteria block below and graded by `mochiko:validator`
  (skill-content-schema D8/I6; the matching-skill routing never applies to a converted pair).
  For every other primitive the matching
  `validation-*` / `review-*` skill applies, graded on internal coherence plus preserved
  responsibilities (`templates/command-shape.md` was deleted at v0.46.0; the dedicated
  `validation-command-shape` skill at v0.45.0). The editor never grades their own edit —
  dispatch a separate validator.

  **Canonical-scaffold criteria — every pair-form command, all six commands.** A command ships as
  `.md` + `plugins/mochiko/schemas/<cmd>.yaml` and is graded across **both surfaces** on one
  criteria set. There is no second block and no per-form exception: the library has one
  scaffold (`command-md-scaffold-standardization` D1/D2), and the only branch is the
  done-condition class at the end of this list.

  1. **Scaffold conformance.** The `.md` carries the canonical headings in the canonical
     order — frontmatter (`description` · `argument-hint` ·
     `disable-model-invocation: true` — a required key set; YAML key order is not graded,
     though all six ship in this order) · `# <Name> — <epithet>` · `## Identity & Mission` (one tight
     section, never materially delaying the Rules block) · `## Rules — load the schema
     first` · `## Adaptive Goal Protocol` with its three steps **Entry** → **Goal** →
     **Not done — default FAIL** (last). `$ARGUMENTS` is handled in Entry; the Not-done
     line is the count-pin. No `**Goal:**` opener line, no `Harness` / `Bindings`
     sections, no per-command extra top-level section. **On a converted command** — one
     whose rules `mochiko-cli` renders at fire (`cli-schema-delivery` D3) — the same
     scaffold holds with three substitutions: the frontmatter carries the additional
     required key `allowed-tools: Bash(mochiko-cli *)`, the Rules heading reads
     `## Rules — delivered by mochiko-cli`, and the Not-done line cites the CLI-printed
     pin instead of carrying a count (criterion 3).
  2. **Rules-block enumeration.** The section IDs enumerated in the Rules block match the
     schema's section IDs **set-wise** — the six-set `<cmd>.sec.roles` · `reserved` ·
     `tools` · `ways-of-working` · `boundaries` · `fail-conditions`, all six present in
     every schema, a section with no rules carrying its explicit empty marker (D4/D5).
     Every `<cmd>.sec.*` token anywhere in the `.md`, inside the Rules block or outside
     it, resolves to a live node.
  3. **FAIL survival** keys to **`kind: fail`** (ontology D1, build item 4): every
     `kind: fail` rule survives (a reword keeps its ID), the `.md` Not-done line's
     hard-coded count matches the schema's `kind: fail` count, and the correspondence
     between the `<cmd>.fail.*` ID segment and `kind: fail` holds in both directions —
     `kind:` is never defaulted on a `.fail.*` ID. **On a converted command** the
     hand-pinned count is gone by ruling (`cli-schema-delivery` D3: the counts are
     computed and printed by the CLI, never hand-pinned): the pin is the
     `- kind: fail · N rules` line the render prints under `pins` in the preamble block,
     and the `.md`'s Not-done line cites that pin and obliges a halt-and-surface when a
     delivered section's end-line count disagrees with it. Grade the citation and the
     halt clause; a hard-coded number there is the defect, not its absence. Everything
     else in this criterion is unchanged.
  4. **ID continuity (D11/D14).** No `<cmd>.*` ID — rule **or** `<cmd>.sec.*` section —
     vanishes without a tombstone. A reword keeps its ID, a split mints children recording
     the parent, a merge tombstones the losers; no surviving rule text references a
     tombstoned or re-homed node.
  5. **`class: floor` = must-survive (M3).** A `floor`-class rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  6. **Substance across the pair.** Plan approval before any producing seat works ·
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
     branch.**
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
  8. **Preserved responsibilities**, as for any primitive: protected content leaves only
     by recorded supersession-by-ruling, and strips + budgets apply unchanged.
  9. **Deterministic pre-pass.** The D13 advisory checker's output (explicit
     `--schema` / `--md` flags for the pair) is cited in the audit brief, beside the
     char-budget pre-assert.
  10. **Provenance sidecar unchanged (D16).** Decision anchors live in
      **`.mochiko/provenance.yaml`**, keyed by rule ID — repo-side, never in the schemas,
      never shipped; an anchored rule still leaves only by recorded
      supersession-by-ruling, and the checker resolves every anchor.
  11. **Ontology-grammar conformance (D1–D8).** Across the pair: every `kind:` value comes
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
      `scripts/similar-rules-allowlist.yaml`) — and where any stub binds, the `.md`'s first
      action Reads `plugins/mochiko/schemas/common.yaml` raw beside the schema. **On a
      converted command** that co-Read is discharged by the render, which resolves every
      stub before the model sees it, so no raw common-file Read is demanded there.

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

  **Skill-pair criteria — every converted skill; the review family from v0.100.0.** A
  converted skill ships as `SKILL.md` + `plugins/mochiko/skills/<name>/schema.yaml`
  (in-directory — the skill directory stays the self-contained shipping unit) and is graded
  across **both surfaces** on this criteria set. The grader is **`mochiko:validator`**,
  exactly as for a command pair; the matching `validation-*` / `review-*` skill routing never
  applies to a converted pair (no validator-for-skills exists, and a pilot member never
  grades itself). This block is a **sibling** of the command block above, never a fork of
  it: skills are their own grammar family — no `moments:`, no `$ARGUMENTS` protocol, no
  Not-done count-pin — under the same governance envelope.

  1. **Load-first section.** The `SKILL.md` body carries a "Rules — load the schema first"
     section whose obligated first action is a raw, whole Read of the skill's own
     `schema.yaml` (base-dir-relative) and, where any stub binds, the skill's family
     common file — `plugins/mochiko/schemas/skill-review-common.yaml` for the review
     family, `plugins/mochiko/schemas/skill-authoring-common.yaml` from v0.101.0 — in the
     same first action; the patterns family ships no common file (census-patterns §ROAD),
     so its load-first block reads the pair's own schema only; a member's
     own obligated reference read (e.g. `review-feasibility`'s lens) sequences there too.
     The reading grammar — `when:` interpretation, floors always delivered, stub
     inheritance limits — is carried in the block; the `when:`-interpretation clause is
     omitted where the schema declares no `conditions:` (the RCM-4 wave-wide ruling).
  2. **Section enumeration.** The section IDs enumerated in the load-first block match the
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
     node.
  3. **Floor-count pin + read-back.** The `.md`'s pinned line — "the N rules of
     `class: floor`" — matches the schema's `class: floor` count (a stale pin is an
     out-of-sync halt-and-surface), and the load-first block obligates stating the floor
     count back before the first procedural step (the delivery read-back,
     skill-content-schema D6 as amended).
  4. **Floor survival.** A `class: floor` rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  5. **ID continuity.** No `<skill>.*` ID — rule or `<skill>.sec.*` section — vanishes
     without a tombstone. A reword keeps its ID, a split mints children recording the
     parent, a merge tombstones the losers; no surviving rule text references a tombstoned
     or re-homed node.
  6. **`extends:` conformance.** An `extends: <family>-common.<slug>` stub binds only the
     skill's own family library (cross-family sharing forbidden, D5), inherits
     `text` / `labels` / `pointer` only, declares `class:` locally, and binds only under the
     near-dup bar — near-identical across 3+ members, strongest-wording-wins (R1/R2,
     `.mochiko/decisions/2026-08-28-near-dup-convergence.md`); a member whose extra content
     is skill-specific keeps local text, the edge recorded in
     `scripts/similar-rules-allowlist.yaml`. The stub's `<skill>.*` ID stays the citable
     ID, and where any stub binds, the load-first action Reads the skill's family common
     file raw beside the schema.
  7. **`description:` untouched.** The frontmatter `description:` value is byte-identical
     across the conversion and ≤ 1,536 chars (the delivery cap); it never moves to schema.
  8. **Budget = delivered-at-invoke payload.** The budgeted quantity is the `SKILL.md` body
     plus the skill's own `schema.yaml`, one number, characters of the parsed value. At
     conversion the budget re-seeds to the measured post-conversion payload with **no +25%
     headroom** (the ledger's third seeding path — the conversion is a relocation, never a
     measured winner), the audit grading the delta against the pre-conversion body figure
     as structural overhead only (IDs, keys, grammar); content growth takes the normal
     argued-overage path, named in the brief. The family common file is budgeted once as
     its own primitive, never per binding skill; `references/` and `scripts/` stay exempt.
  9. **Pointer resolution.** Every `pointer:` resolves base-dir-relative from the skill
     directory, cross-directory climbs included (`../<other-skill>/references/...` is
     legal; the Single-source convention governs the pointed-at files).
  10. **Deterministic pre-pass.** The advisory skill-schema checker's output for the pair
      is cited in the audit brief, beside the char-budget pre-assert.
  11. **Skill-grammar conformance.** Every `kind:` value comes from the **eight-kind**
      skill set — `constraint` · `duty` · `gate` · `reservation` · `binding` · `bound` ·
      `routing` · `latitude` — with `constraint` the omitted default; **`kind: fail` and
      `enforces:` are illegal in a skill schema** (census-evidence retirement,
      skill-content-schema D9/M2). Every `when:` term resolves against the schema's
      declared `conditions:`; a `class: floor` rule is always read and always delivered
      whatever its `when:`; no `moments:` block exists (procedure stays prose, D3).
  12. **Provenance sidecar.** Decision anchors live in **`.mochiko/provenance.yaml`**,
      keyed by rule ID — repo-side, never shipped; a rule carrying a supersession-transfer
      (a `KEPT:`-protected or `DECISIONS.md`-traceable line relocated into the schema,
      skill-content-schema D8/C4) inherits protected status through its sidecar entry and
      leaves only by recorded supersession-by-ruling.

  Rulings: `.mochiko/brainstorms/skill-content-schema/record.md` D1–D9 as amended
  (`DECISIONS.md` 2026-09-01) · the census inventory
  `.mochiko/brainstorms/skill-content-schema/census.md` (§E kind retirement · §H section
  set · J-7 cross-directory pointers) · the authoring-family census
  `.mochiko/brainstorms/skill-content-schema/census-authoring.md` (§I labels · J-1
  section set · J-6 budget · J-7 first-seeds) · the patterns-family census
  `.mochiko/brainstorms/skill-content-schema/census-patterns.md` (§B section proposal ·
  §ROAD rejection · §I labels · J-P2 first-strips · J-P5 two-arm/overage) · the
  small-families census
  `.mochiko/brainstorms/skill-content-schema/census-small-families.md` (§B six-set-reuse
  fit table · §C zero common blocks · §D abort-tripped — the dense five convert on the
  B/C drivers · J2-8 dual-homing twins · J2-9 ruled repair) ·
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6.

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
