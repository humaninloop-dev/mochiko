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

  Then the independent **author ≠ grader** audit: `mochiko:validator` grading a command
  against **the command's own text** — internal coherence (default-FAIL goal · harness present:
  plan approval for producing seats, author ≠ grader independence, decisions reserved to the
  user · bindings complete: paths, templates, entry condition) plus preserved responsibilities
  (`templates/command-shape.md` was deleted at v0.46.0; the dedicated `validation-command-shape`
  skill at v0.45.0) — the matching `validation-*` / `review-*` skill otherwise. The editor never
  grades their own edit — dispatch a separate validator.

  **Pair-form commands (command-content-schema D9; implement from v0.92.0, the D10
  five-command rollout — `architecture` · `brainstorm` · `feature` · `setup` · `specify` —
  from v0.95.0):** a command shipped as `.md` + `plugins/mochiko/schemas/<cmd>.yaml` is graded
  across **both surfaces**, whichever criteria block applies — the charter-form exception for
  a charter command, this default block otherwise. On any pair: FAIL survival keys to the
  **`fail-condition` label set** (every so-labeled rule surviving, the `.md` Not-done line's
  hard-coded count matching the schema's); **ID continuity (D11/D14):** no `<cmd>.*` ID —
  rule or `<cmd>.sec.*` section — vanishes without a tombstone; **`class: floor` =
  must-survive (M3)**, `advisory` may change without supersession ceremony; the D13 checker's
  output (explicit `--schema`/`--md` flags for the pair) is cited in the audit brief as the
  deterministic pre-pass. Decision anchors live in **`.mochiko/provenance.yaml`** (D16),
  keyed by rule ID — repo-side, never in the schemas, never shipped; an anchored rule still
  leaves only by recorded supersession-by-ruling, and the checker resolves every anchor.

  **Exception — charter-form commands (`feature.md` D10 v0.68.0; `plan.md` / `implement.md`
  ADR 2026-08-13-charter-plan-implement v0.69.0):** a charter command is graded against
  *floor present + goal contract present* — the Boundaries floor (including the sound-loop
  floor pointer `mochiko:patterns-sound-loop`, present in all three charters) + the DM's
  bare-minimum responsibilities, and the Adaptive Goal Protocol's contract — in place of the
  default-FAIL-goal / Harness / Bindings checks. For `implement.md` from v0.92.0 these criteria
  grade the **`.md` + `plugins/mochiko/schemas/implement.yaml` pair** (command-content-schema
  D9): the narrative `.md` carries Identity & Mission, the protocol, and the obligated schema
  read; the schema carries the R&R / Tools / Ways-of-Working / Boundaries rule blocks — floor
  present + goal contract present are judged across both surfaces, the sound-loop floor pointer
  counted on whichever surface carries it. For `feature.md` (the standing desk) the
  contract is **per-visit**: converge-to-done-condition. For `plan.md` / `implement.md`
  (pipeline runs) the contract is **per-run**: (i) a protocol step literally labeled
  **Entry** carrying the neither-source routing; (ii) convergence at a named EXISTING user
  gate — plan: proposal approval, the approved list = the run's done condition and
  default-FAIL floor, delta scope collapsing; implement: run-open confirmation naming batch,
  scope type, attempt bounds (redeclarable there and only there), and the fixed done
  condition; (iii) a fixed done condition closing at the existing acceptance gate; (iv) FAIL
  survival — for implement, keyed to the **`fail-condition` label set**: every rule labeled
  `fail-condition` in `implement.yaml` surviving (a reword keeps its ID), and the `.md`
  Not-done line's hard-coded count matching the schema's; for plan (retired v0.91.0,
  historical audits) the `**Not done — default FAIL**` list present with every prior FAIL
  clause surviving; (v) for implement, **ID continuity (D11):** no `impl.*` ID — rule or
  `impl.sec.*` section (D14, v0.93.0) — vanished without a tombstone — a reword keeps its ID,
  a split mints children recording the parent, a merge tombstones the losers; (vi) for implement, **`class: floor` = must-survive (M3):** a `floor`-class rule
  leaves only by recorded supersession-by-ruling; an `advisory`-class rule may change without
  supersession ceremony. The D13 advisory checker's output is cited in the audit brief as a
  deterministic pre-pass, beside the char-budget pre-assert.
  The check must NOT demand a per-run negotiated goal (that is the desk's per-visit form, not
  the pipeline's) and must not demand Goal / Harness / Bindings sections. Everything else
  (independence, decisions reserved to the user, preserved responsibilities, strips,
  budgets) applies unchanged. Rulings:
  `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` D10 ·
  `.mochiko/decisions/2026-08-13-charter-plan-implement.md` ·
  `.mochiko/brainstorms/charter-ritual-balance/record.md` D3 (`DECISIONS.md` 2026-08-13) ·
  `.mochiko/brainstorms/command-content-schema/record.md` D9 · D11 · D14 (`DECISIONS.md` 2026-08-26).

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
