# Strip notes — `agents/validator.md`

Entry formats: `strips/README.md`.

## [v0.113.0] the `validator` persona retired whole — 105 lines, eight sections

- **Disposition:** superseded → the carrier (`author-grader-consolidation` D7): a plain fresh seat
  running the rendered `mochiko:validation-primitive-edit` contract at the primitive-edit gate, and
  `mochiko:validation-constitution` at setup's validate step per the `setup.validate-seat-form`
  rule. Section by section:
  - `# Validator` (12-17) — handed a finished artifact and a bar, say so plainly when either is
    missing → the gate skill's scope section and its `rendered-contract-only` floor; the setup site
    is covered by `setup.validate-seat-form`.
  - `## Skills you lean on` (18-37) — the router-indexed selection order and the
    delivery-not-scope closing line → retired with the mount rather than re-homed: the dispatch
    brief names the skill, and each site names its member, so no runtime routing survives.
  - `## Core Identity` (38-54) — you did not write it, so assume it is not done →
    `validation-primitive-edit.author-grader` and `.default-fail`.
  - `## What You Produce` (55-68) — the verdict block with its `Evidence read:` field → the gate
    skill's output section and `.evidence-floor`.
  - `## Iron Law` (69-79) — no PASS without evidence read from the artifact itself →
    `.tamper-proof-clause` and `.from-file-floor`.
  - `## What You Reject` (80-87) — grading from say-so, "mostly conforms", authoring, grading your
    own work, substituting your own bar → `.binary-verdict`, `.author-grader`, and the skill's
    reserved section.
  - `## Your Judgment` (88-94) — the evidence hierarchy and the deterministic pre-assert →
    `.from-file-floor`, `.pre-pass-first-hand`, `.evidence-floor`. The external-claim source
    re-read sub-clause has **no successor rule** in the gate contract — it retired with the
    persona, and is disclosed here rather than claimed re-homed.
  - `## Delegating Cheap Reads` (95-105) — the disposable `Explore` subagent with an explicit
    `model: haiku` override, one gap per spawn, interpretive reads kept on the seat →
    `mochiko:patterns-model-tiering`, which this section already pointed at by name. It is the
    shared boilerplate carried on every persona file, so nothing unique to this persona leaves
    with it.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/producer-plan-enforcement/record.md` D3 ·
  `.mochiko/brainstorms/author-grader-consolidation/record.md` D7; `DECISIONS.md` 2026-09-03
  and 2026-09-19 rows).
- **Content:** the frontmatter (`name`, the 269-char `description`, `model: opus`, `color: red`,
  `skills: validation-constitution`) plus the eight sections above, each named with its operative
  sentence. The file as it stood is recoverable verbatim at
  `git show 56b44c2:plugins/mochiko/agents/validator.md`.
- **Kept deliberately:** the substance the two skills carry verbatim as floors — default FAIL,
  read-the-file-or-FAIL, a binary verdict with a fix list, never grade work you authored — and the
  explicit-`model:` pin (`validation-primitive-edit.plain-seat-explicit-tier`) that replaces the
  retired frontmatter `model: opus`. The cheap-read discipline of the closing section survives
  unchanged in `mochiko:patterns-model-tiering`, which never belonged to this file.
- **Consumers assessed:** the `plugin.json` agents entry removed · the router's five lines
  reworded (strip `mochiko.md`) · `review-brainstorm/references/EXTERNAL-CLAIMS.md` pointer
  annotated (strip `review-brainstorm.md`) · `CLAUDE.md` item 3, the shipped-primitive paragraph,
  and axis 5 · `.claude/rules/mochiko/primitive-edits.md` parenthetical · `ARCHITECTURE.md` six
  lines · the governance ledger's GI-004 detail and editorial note · the budget ledger's agent row
  struck · migration `0010-validator-retirement.yaml` drops the persona from
  `patterns-model-tiering.seat-default-key` and rewords the two `authoring-constitution` rules ·
  the crate census tests and the frozen eval kit at `evals/agents/validator/`.

## [v0.85.0] Router-registration sentence stripped from the v0.84.0 closing paragraph

- **Disposition:** deleted (the doctrine home already carries it: the router's own "Adding to
  the library" section owns registration-on-authoring; the maintainer rationale — the router
  read, not the persona, keeps the checklist set current — lives in the v0.84.0 ADR, now with
  a 2026-08-26 addendum)
- **Tier failed:** 1 (altitude — restated router doctrine inside a runtime persona; rationale
  aimed at the future maintainer, whose home is the ADR/strip layer, not the prompt). User
  ruling 2026-08-26, in-session, on the maintainer's own "why does this line exist" challenge.
- **Content:** verbatim — "New checklist skills register in the router when they are authored;
  the router read — not this file — is what keeps you current."
- **Kept deliberately:** the paragraph's second sentence, now a standalone closing line — "The
  frontmatter `skills:` mount is delivery for the common case, never your scope." — the
  load-bearing half: it prevents the mount-read-as-scope misread that triggered the v0.84.0
  redesign, at runtime and at maintenance.
- **Consumers assessed:** nothing cites the removed sentence — the v0.84.0 CHANGELOG entry and
  ADR describe the selection order and the delivery-not-scope framing, both intact; the router
  is untouched; no command names the agent.

## [v0.84.0] "Skills you lean on" re-indexed to the router — inline checklist bullet superseded

- **Disposition:** superseded → the rewritten `## Skills you lean on` section: a three-step
  selection order (brief wins · Read the router `skills/mochiko/SKILL.md` and reach for the
  domain-matching checklist, `validation-*` natively the validator's, a `review-*` bar lent
  checklist-only · generic-method fallback), closing with "New checklist skills register in
  the router when they are authored; the router read — not this file — is what keeps you
  current. The frontmatter `skills:` mount is delivery for the common case, never your scope."
- **Tier failed:** n/a — supersession by ruling (user ruling 2026-08-26, in-session; ADR
  `.mochiko/decisions/2026-08-26-validator-router-indexed-checklists.md`; `DECISIONS.md`
  2026-08-26 row). Drivers: the one-item list read as scope (confused the maintainer this
  ruling came from); the bullet was stale ("drafted constitution" — no constitution.md
  exists) and restated `validation-constitution`'s single-sourced checklist internals
  (altitude); an inline list obliges a persona landing per new checklist skill, where the
  router already owns discoverability.
- **Content:** verbatim superseded spans — the intro sentence "You carry graded checklists.
  Reach for the one whose domain matches the artifact in front of you:"; the bullet
  "**`mochiko:validation-constitution`**: for grading a drafted constitution — Three-Part Rule
  (enforcement / testability / rationale per principle) plus trace stamps, the deterministic
  trace-ID cross-check against the governing intent record, tier / waiver / floor-accounting
  checks, module-parameterized section checks, anti-pattern scan, placeholder scan,
  quantification, semantic version-bump."; the framing sentence "When the artifact fits one
  cleanly, that checklist is your strongest asset — use it."
- **Kept deliberately:** the frontmatter `skills: validation-constitution` mount and the
  generic-grader method — the v0.45.0 `Kept deliberately` set: the Iron Law untouched, the
  fallback clause "fall back on your own method and grade the artifact against the bar you
  were given, check by deliberate check. The rigor is the same either way." surviving
  near-verbatim as selection-order step 3. The `description:` value (v0.63.0 protected prose
  framing) and every other section byte-for-byte.
- **Consumers assessed:** router agents-table row + two-family paragraph remain true (no
  router edit) · `.claude/rules/mochiko/primitive-edits.md` audit-dispatch clause aligned,
  not contradicted · setup grading seat unaffected (mount survives) · no command names the
  agent. Full assessment: the ADR.

## [v0.78.0] Delegating Cheap Reads retargeted — `mochiko:explorer` dispatch superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded `## Delegating Cheap Reads` sentence: "spawn a
  disposable native `Explore` subagent with an explicit `model: haiku` override (the
  override makes the read cheap; a bare spawn inherits the session tier)".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` dispatch this section prescribed failed on exactly the transport the
  section was built for.
- **Content:** verbatim superseded span (identical across all ten personas): "spawn a
  disposable `mochiko:explorer` subagent (its `model: haiku` frontmatter makes the read
  cheap)".
- **Kept deliberately:** the rest of the `## Delegating Cheap Reads` section byte-for-byte —
  the class-key summary (locate/enumerate/targeted-read cheap; interpretive, absence-driven,
  completeness-sensitive kept), one-gap-per-spawn, the bulk-read-stays-out rule, and the
  closing pointer to `mochiko:patterns-model-tiering`.
- **Consumers assessed:** the section wording is shared across the ten personas; all ten
  edited in the same v0.78.0 wave (this entry mirrored in each persona's strip file). No
  command or skill names the section.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/validator.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **2 `<example>` blocks removed** from the `description:` value:
  1. Context: a finished constitution needs an independent quality grade before acceptance — commentary claimed the example demonstrated that independent grading of a finished artifact against an explicit checklist is the validator's core work.
  2. Context: an artifact is handed over with a checklist outside the validator's built-in domains — commentary claimed it demonstrated that the same skeptical, evidence-first grading applies; the artifact and checklist differ, the craft does not.

  Description parsed-value char delta: **1,448 → 268** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/validator.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/validator.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Skeptical, independent reviewer who grades a finished artifact against an explicit checklist … Defaults to FAIL. Never grades work it authored.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `validator`: `skills/*/SKILL.md` and `references/` (`ARTIFACT-CHECKLISTS.md`, `EXTERNAL-CLAIMS.md`, `INTERROGATION-AGENDA.md`, `QUALITY-CHECKLIST.md`, `backend-service.md`, `check-artifacts.py`); no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** the prior [v0.45.0] entry touches the `skills:` frontmatter list and a body "Skills you lean on" bullet (the `validation-command-shape` mount drop). It does not touch the frontmatter `description:` value or any `<example>` block. Its `Kept deliberately` set (the `validation-constitution` mount and the generic-grader method in the body) is untouched by this edit. No overlap.

## [v0.45.0] `validation-command-shape` mount dropped (fourth-consumer edit)
- **Disposition:** superseded → the generic-checklist fallback already in the persona body
  ("When it does not, do not force it: fall back on your own method and grade the artifact
  against the bar you were given")
- **Tier failed:** n/a — supersession by ruling (user ruling 2026-08-02, framework-trio
  deletion; ADR `.mochiko/decisions/2026-08-02-framework-trio-deleted.md`). This is the
  fourth-consumer edit pre-flagged in `strips/authoring-commands.md` [v0.44.0] DEFERRED —
  owed at the move, landed at the delete instead.
- **Content:** frontmatter `skills: validation-constitution, validation-command-shape` →
  drops the second name; the "Skills you lean on" bullet — *"**`mochiko:validation-command-shape`**:
  for grading an orchestration command's conformance to its codified shape — a deterministic
  grep floor (references present, no restated single-sourced prose, exceptions marked,
  frontmatter correct) run first and recorded as evidence, then the prose judgment ceiling
  (altitude, parameter completeness, contract-fill soundness), plus the strip-note audit when
  a minimalism wave is closing."* — deleted whole.
- **Kept deliberately:** the `validation-constitution` mount and the entire generic-grader
  method (Iron Law, checklist-fallback paragraph) — command audits now ride that fallback
  with `templates/command-shape.md` as the handed bar.
- **Consumers assessed:** router `validator` row (skills list edited in the same wave) ·
  setup command's validator seat unaffected (uses `validation-constitution`).
