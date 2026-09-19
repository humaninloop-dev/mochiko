# Strip notes — `agents/product-engineer.md`

Entry formats: `strips/README.md`.

## [v0.110.0] Frontmatter `model: opus` superseded — the seat default key sends this seat to `sonnet`

- **Disposition:** superseded → `model: sonnet` on the same frontmatter key. The seat default key
  (`orchestrator-model-selection` D1) assigns this persona the `down` class on one criterion —
  does a structurally independent seat stand between this seat's output and the run's verdict —
  and D2 pins the class default in the persona file as a tier alias, never `inherit` and never an
  absent `model:`. D3's ground for this row: "prototype producer, graded"; D3's G7 fold states the
  standard that pass rests on — the independent gap-finding pass plus the lead's gate plus the
  user's acceptance.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/orchestrator-model-selection/record.md` D3, the ten-row seat class table,
  with D1/D2 as its mechanism and D8 as its alias rule; `DECISIONS.md` 2026-09-19 row). The same
  ruling supersedes `model-tiered-seats` D5 and its fold F6 — the deferral this table discharges.
- **Content:** verbatim — `model: opus` (frontmatter, line 9).
- **Kept deliberately:** every other byte of the file — the frontmatter `description:` value
  (v0.63.0 protected prose framing), `name:`, `color:`, the `skills: authoring-prototype` mount,
  and the whole body including `## Delegating Cheap Reads`. That section's `model: haiku` override
  sentence is the reads rung, a different key from the seat default; D7 item 2 states the persona
  sections are unchanged by this ruling. D8 also keeps the new value a family alias, never a full
  id such as `claude-sonnet-5`.
- **Consumers assessed:** `plugins/mochiko/.claude-plugin/plugin.json` agents list — path-only
  entries with no model axis, unchanged · the router's agents table
  (`plugins/mochiko/skills/mochiko/SKILL.md`, the `product-engineer` specify-cluster PRODUCER row)
  — no persona row names a model; the row is role text and stays as it is · `ARCHITECTURE.md`
  agents row (line 61 after this wave), which read "Personas (all `model: opus`)" — corrected in
  this same wave ·
  `plugins/mochiko/skills/patterns-model-tiering/` and its floor set — superseded by migration
  `0007-seat-default-key`, which records the schema content by construction and takes no entry
  here · the persona eval kit `evals/agents/product-engineer/` and the runner constant `ARM_MODEL`
  in `evals/commands/agents.py` — both live on the `primitive-evals-v2` branch, not on `main`;
  noted, not edited (D7 item 4, deferred to the branch merge) ·
  `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md` carries a frozen
  `model: opus` copy — a benchmark archive, not a shipped primitive, deliberately untouched.

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
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: a feature's user stories are being drafted and the experience needs to be visible before design starts — commentary claimed the example demonstrated that making a drafted experience clickable at low fidelity, story by story, is the product-engineer's core producer work.
  2. Context: the project has an existing design system and the mock should read as part of the product — commentary claimed it demonstrated that honoring an existing design system at low fidelity, without over-polishing into false precision, is the product-engineer's judgment.
  3. Context: a reviewer found flows in the mock that no story scenario covers — commentary claimed it demonstrated that keeping the prototype an honest rendering of the stories, surfacing gaps instead of inventing scope, is the product-engineer's discipline.

  Description parsed-value char delta: **2,208 → 391** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/product-engineer.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Staff-level Product Engineer who makes intended user experiences tangible before they are built … Produces the prototype and its manifest; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `product-engineer`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
