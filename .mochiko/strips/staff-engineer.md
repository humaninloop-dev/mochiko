# Strip notes — `agents/staff-engineer`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (v0.17.0). Implement-only
agent (the TDD producer, mounted on `implement`) — strips ruled in-wave (single consumer).

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

## [v0.64.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (Wave 2 editorial extension of the agents-arm ruling); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept verbatim.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail agents-arm user ruling (b) 2026-08-10, extended to the untested agents by the Wave 2 user ruling; `DECISIONS.md` 2026-08-11 build row Wave 2 residual; `report/final-verdict.md` — 0 route misses over 20+ staffings).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Task list built test-first — red/green/refactor producer work.
  2. Reported failures resolved — reproduce-with-a-failing-test, narrowest scoped fix.
  3. Extend an existing module without breaking callers — read-before-write, interface preservation (brownfield).

  Description parsed-value char delta: **1,852 → 273** (chars of the parsed block-scalar value; block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in git history of `plugins/mochiko/agents/staff-engineer.md` (pre-v0.64.0).
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Staff Software Engineer who implements code through strict TDD discipline — executing task lists with red/green/refactor rigor, integrating with existing codebases, and producing honest reports of what was built. Produces the implementation; does not grade its own output.") — and the entire agent body, byte-for-byte untouched (verified against git HEAD).
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `staff-engineer`: referenced only by the router `plugins/mochiko/skills/mochiko/SKILL.md`; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed.
- **Standing watch:** an F-X1-class route miss on the untested agents re-opens ruling (b).
- **KEPT reconciliation:** the prior [v0.17.0] KEPT entry protects the "Quality Standards" persona *body* enumeration; the [v0.49.0] and [v0.17.0] strips touched persona *body* wording and a "Skills Available" *body* bullet. None touch the frontmatter `description:` value or any `<example>` block. No overlap with this edit.

## [v0.49.0] Card-form wording re-key
- **Disposition:** superseded → cycle-card vocabulary (card checkbox flip; exposure-driven read-before-write)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D2.1)
- **Content:** "Updated `tasks.md` with completed task checkboxes (`[x]`)" · "when a task says EXTEND or MODIFY, you read the full file first".
- **Kept deliberately:** persona otherwise untouched (keystone: no workflow trace added).
- **Consumers assessed:** implement · executing-tdd-cycle.

## [v0.17.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the two skills themselves
  (`mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration`), which already single-source it —
  each skill's `description:` carries its scope (the red/green/refactor execution sequence + task parsing
  + targeted rework + `cycle-report.md` format live in `executing-tdd-cycle`; the EXTEND/MODIFY
  consumption + read-before-write + interface-preservation + conflict-detection craft lives in
  `brownfield-integration`); the skill **names + a one-line reach-for-it hint are kept** (they carry the
  team-form function — teammates ignore `skills:` frontmatter, so the in-body names are how a spawned
  teammate learns its skills)
- **Tier failed:** 1 (a second home for each skill's scope — the bullets restated what each skill's
  `description:` already single-sources, then pointed at the same skill as "the single source of truth")
- **Content:** the two full scope descriptions in the "Skills Available" bullets — "TDD red/green/refactor
  discipline, the task-execution sequence, task parsing, targeted rework when specific tasks fail, and the
  `cycle-report.md` format. This is the single source of truth for the execution procedure and the report
  schema behind everything you produce." and "EXTEND/MODIFY semantics, the read-before-write checklist,
  interface preservation, and conflict detection for existing codebases."
- **Consumers assessed:** implement only (staff-engineer is implement-only). One instance of the 7-agent
  library-wide "Skills Available" pattern; ruling in-wave is D9-authorized (single consumer), consistent
  with the `technical-analyst` single-consumer ruling (v0.15.0) and noted so the escalation lands
  consistently across all seven agent personas — the `devils-advocate` instance is the ≥3-consumer
  escalation; `task-architect` (2 consumers), `principal-architect` (2 consumers), `technical-analyst`
  (single), and now `staff-engineer` (single) were ruled in-wave.

## [v0.17.0] KEPT: the "Quality Standards" persona enumeration (TDD rigor / Scope discipline / Brownfield respect / Honest reporting)
- **Tier-2 evidence:** the agent↔skill composition axis blesses persona (what the agent cares about) in
  the agent; "Quality Standards" is the engineer's *taste*, explicitly disclaimed — "this is the *taste*
  you bring, not the format spec. The concrete procedure lives in your skills, which are the single source
  of truth" — so it names the bar without restating the red/green/refactor procedure or the report schema
  (those live in `executing-tdd-cycle`). Persona altitude, not a behavior-driving procedure restatement —
  distinct from the "Skills Available" strip above, which was a scope catalog pointing at the very homes it
  copied. Matches the `technical-analyst` "Quality Standards" keep and the `task-architect` "What You
  Produce" keep. (The "What You Produce" section's `cycle-report.md` bullet likewise references the skill
  as the single source — "Its format lives in `mochiko:executing-tdd-cycle`; consult it there rather than a
  copy here" — persona altitude by the same test.)
