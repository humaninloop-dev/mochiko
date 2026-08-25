# Strip notes — `agents/requirements-analyst.md`

Entry formats: `strips/README.md`.

## [v0.86.0] Skills Available section rewritten to the ruled precedent form

- **Disposition:** relocated → each mounted skill's own `description:`/body (the declared
  single source); the section now carries the precedent form (single-source framing + one
  routing line per mount) established at the devils-advocate [v0.25.0] Tier-1 strip and used
  by every other persona.
- **Tier failed:** 1 (altitude — the bullets restated the mounted skills' internals; the same
  file's Quality Standards section already pointed at the same templates with a correct
  consult-there line, so the content existed twice, once at the wrong altitude). User ruling
  2026-08-26, persona hygiene pass (ADR
  `.mochiko/decisions/2026-08-26-persona-hygiene-pass.md`; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim superseded spans — the intro "You have access to specialized skills
  that provide detailed guidance:"; the bullets "**`mochiko:authoring-requirements`**: Write
  FR-XXX format requirements with RFC 2119 keywords (MUST, SHOULD, MAY), measurable success
  criteria (SC-XXX), and edge-case identification." and "**`mochiko:authoring-user-stories`**:
  Write user stories with P1/P2/P3 priorities, Given/When/Then acceptance scenarios, and
  independent tests."; the closing "Use the Skill tool to invoke these when you need detailed
  formatting guidance for your output artifacts."
- **Kept deliberately:** both mounts (frontmatter and bullets — only their annotations
  thinned); the Quality Standards consult-there paragraph, byte-for-byte; the `description:`
  (v0.63.0 protected prose, 303 chars against the 379 budget) and every other section
  untouched.
- **Consumers assessed:** no command names the agent; the router's agents-table row is
  body-agnostic; the mounted skills' own descriptions carry the relocated detail (verified
  present in both `SKILL.md` descriptions).

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
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/requirements-analyst.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: a rough feature idea ("share reports with their team") needs turning into an implementable spec — commentary claimed the example demonstrated that turning a vague feature request into precise, testable requirements is the analyst's core authoring work.
  2. Context: a spec says the system should be "fast and easy to use" with no measurable targets — commentary claimed it demonstrated that quantifying vague expectations into measurable, testable requirements is producer work the analyst owns.
  3. Context: a feature needs prioritized user stories with acceptance scenarios before work begins — commentary claimed it demonstrated that authoring prioritized, independently testable user stories is the analyst's spec-authoring responsibility.

  Description parsed-value char delta: **1,952 → 302** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/requirements-analyst.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/requirements-analyst.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior analyst who transforms vague feature requests into precise, implementable specifications … Authors the specification; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `requirements-analyst`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
