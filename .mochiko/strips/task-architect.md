# Strip notes — `agents/task-architect`

Entry formats: `strips/README.md`. Wave context: the slice cluster wave (v0.14.0). Shared agent,
**2 consumers** (`tasks` + `slice`) — under the D9 3-consumer threshold, ruled in-wave; each entry
names both consumers assessed.

## [v0.49.0] Agent retired — seat dies, crafts become lead-dispatched skills
- **Disposition:** superseded → no successor seat; `patterns-vertical-tdd` (slimmed) and `authoring-slices` (re-scoped) are lead-dispatched to whichever producer seat fits the run, per v8 lead-owned seating; file deleted, plugin.json agents 9→8
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D4)
- **Content:** the full persona — Core Identity (four scars), What You Produce (task-mapping.md · tasks.md · slices.md), Quality Standards (vertical/test-first/verified-against-reality/traceable/file-anchored/independently-completable/minimally-coupled), What You Reject/Embrace, Brownfield Awareness. Full text: git history at v0.48.0.
- **Kept deliberately:** the quality bar itself — it lives on in `patterns-vertical-tdd`'s principles/checklist and `review-plan-artifacts`' cycle-card checks; nothing of the persona's judgment was lost, its enforcement surface moved from a seat to the skill + grader pair.
- **Consumers assessed:** plugin.json (array edited) · router (agent row removed) · ARCHITECTURE.md (shared-seats paragraph re-counted) · both former dispatching commands (plan, the retired slice).

## [v0.14.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the skills themselves
  (`mochiko:patterns-vertical-tdd`, `mochiko:authoring-slices`), which already single-source it —
  the `description:` fields carry the scope summaries; the `**TEST:**` grammar and task-quality
  checklist live in the skill bodies (audit precision note, 2026-07-19); the
  skill **names + a one-line reach-for-it hint are kept** (they carry the team-form function —
  teammates ignore `skills:` frontmatter, so the in-body names are how a spawned teammate learns its
  skills)
- **Tier failed:** 1 (a second home for each skill's scope — the bullets pointed at "the single
  source of truth" while restating it)
- **Content:** the two full scope descriptions in the "Skills Available" bullets — "Vertical-slicing
  discipline and TDD cycle structure — … the `**TEST:**` verification-task grammar, the brownfield
  marker set, and the task-quality checklist. The single source of truth for the cycle/task
  artifacts." and the parallel `authoring-slices` scope paragraph (invariants, extend obligations,
  Feature-Done declaration, spec stamp, null exit).
- **Consumers assessed:** tasks + slice (both skills' scopes are single-sourced in the skills
  themselves; the strip holds for both). This is one instance of a 7-agent library-wide "Skills
  Available" pattern; the `devils-advocate` instance is raised for the ≥3-consumer escalation and
  ruled there — ruling `task-architect` in-wave is D9-authorized (2 consumers) and noted so the
  escalation lands consistently across all seven agent personas.

## [v0.14.0] KEPT: the "What You Produce" persona enumeration (incl. the slices.md self-description)
- **Tier-2 evidence:** the agent↔skill composition axis blesses persona (what the agent cares about)
  in the agent; "What You Produce" is the agent's self-description of its outputs, with the two
  "consult the skill, not a copy here" lines already referencing the skills as the single source of
  the concrete formats. Persona altitude, not a behavior-driving procedure restatement — distinct
  from the "Skills Available" strip above, which was a scope catalog pointing at the very homes it
  copied.
