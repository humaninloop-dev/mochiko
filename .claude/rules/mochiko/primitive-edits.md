---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
  - "plugins/mochiko/schemas/**"
  - ".mochiko/provenance.yaml"
---

# Primitive-edit ceremony (strip / supersede → record → check)

Editing a shipped primitive is a **landing, not an ad-hoc edit**. Any change that REMOVES or
SUPERSEDES content — even one line, even an "obvious" cleanup — obliges both moves before the
change is done. Full contracts: `.mochiko/strips/README.md`.

**Schema data files** (`plugins/mochiko/schemas/*.yaml`) are shipped primitives from v0.76.0
(schema-based-template-guidance D8 — data = source of truth, the binary renders over them). An
edit to one takes the same strip + author≠grader ceremony as any command / skill / agent /
template edit; the path scope above covers them so this reminder injects on a schema Read.

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
  `command-md-scaffold-standardization` D1, C1 fold.) For every other primitive the matching
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
     sections, no per-command extra top-level section.
  2. **Rules-block enumeration.** The section IDs enumerated in the Rules block match the
     schema's section IDs **set-wise** — the six-set `<cmd>.sec.roles` · `reserved` ·
     `tools` · `ways-of-working` · `boundaries` · `fail-conditions`, all six present in
     every schema, a section with no rules carrying its explicit empty marker (D4/D5).
     Every `<cmd>.sec.*` token anywhere in the `.md`, inside the Rules block or outside
     it, resolves to a live node.
  3. **FAIL survival** keys to the **`fail-condition` label set**: every so-labeled rule
     survives (a reword keeps its ID), and the `.md` Not-done line's hard-coded count
     matches the schema's.
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

  Rulings: `.mochiko/brainstorms/command-md-scaffold-standardization/record.md`
  D1–D7 (`DECISIONS.md` 2026-08-27 — the canonical scaffold; supersedes the charter-form /
  goal-form split and this block's former dual-block shape, clause inventory in that
  record's Appendix A) · `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`
  D10 · `.mochiko/decisions/2026-08-13-charter-plan-implement.md` ·
  `.mochiko/brainstorms/charter-ritual-balance/record.md` D3 (`DECISIONS.md` 2026-08-13) ·
  `.mochiko/brainstorms/command-content-schema/record.md` D9 · D11 · D14 · D16
  (`DECISIONS.md` 2026-08-26).

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
