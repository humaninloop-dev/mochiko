# Strip notes — `agents/staff-engineer`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (v0.17.0). Implement-only
agent (the TDD producer, mounted on `implement`) — strips ruled in-wave (single consumer).

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
