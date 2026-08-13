---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
---

# Primitive-edit ceremony (strip / supersede → record → check)

Editing a shipped primitive is a **landing, not an ad-hoc edit**. Any change that REMOVES or
SUPERSEDES content — even one line, even an "obvious" cleanup — obliges both moves before the
change is done. Full contracts: `.mochiko/strips/README.md`.

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

  **Exception — charter-form commands (`feature.md` D10 v0.68.0; `plan.md` / `implement.md`
  ADR 2026-08-13-charter-plan-implement v0.69.0):** a charter command is graded against
  *floor present + goal contract present* — the Boundaries floor (including the sound-loop
  floor pointer `mochiko:patterns-sound-loop`, present in all three charters) + the DM's
  bare-minimum responsibilities, and the Adaptive Goal Protocol's contract — in place of the
  default-FAIL-goal / Harness / Bindings checks. For `feature.md` (the standing desk) the
  contract is **per-visit**: converge-to-done-condition. For `plan.md` / `implement.md`
  (pipeline runs) the contract is **per-run**: (i) a protocol step literally labeled
  **Entry** carrying the neither-source routing; (ii) convergence at a named EXISTING user
  gate — plan: proposal approval, the approved list = the run's done condition and
  default-FAIL floor, delta scope collapsing; implement: run-open confirmation naming batch,
  scope type, attempt bound (redeclarable there and only there), and the fixed done
  condition; (iii) a fixed done condition closing at the existing acceptance gate; (iv) the
  `**Not done — default FAIL**` list present with every prior FAIL clause surviving. The
  check must NOT demand a per-run negotiated goal (that is the desk's per-visit form, not
  the pipeline's) and must not demand Goal / Harness / Bindings sections. Everything else
  (independence, decisions reserved to the user, preserved responsibilities, strips,
  budgets) applies unchanged. Rulings:
  `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` D10 ·
  `.mochiko/decisions/2026-08-13-charter-plan-implement.md` ·
  `.mochiko/brainstorms/charter-ritual-balance/record.md` D3 (`DECISIONS.md` 2026-08-13).

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
