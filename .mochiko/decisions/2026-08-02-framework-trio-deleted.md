# Framework-maintenance trio deleted outright (supersedes the D6 repo-side move)

- **Status:** ruled
- **Date:** 2026-08-02

## Context

The user asked for a purge of every command and skill not essential to mochiko *running* —
the product pipeline (setup → specify → slice → plan → implement, plus brainstorm). A full
cross-reference sweep of `plugins/mochiko/` found exactly one cluster with zero pipeline
consumers: the framework-maintenance trio — `agents/command-architect.md`,
`skills/authoring-commands/`, `skills/validation-command-shape/` — the tooling mochiko uses
to author and audit its own commands. Its only references were the router's Framework
maintenance section, the shipped `validator` agent's declared mount (the fourth-consumer
finding recorded in the v0.44.0 DEFERRED entry), and the trio's own files.
`templates/command-shape.md` is NOT part of the purge: all six pipeline commands
obligate-read it as the shape's single-sourced home.

The standing plan was the D6 trio **move** to `.claude/` (verbosity-caveman-ops-separation
D6, 2026-08-01), DEFERRED behind the S14 probe, which had failed in-session (repo-side
agents load only at session start; the spawn instantiated generic).

## Decision

1. **Delete the trio outright** — no repo-side copy. User-ruled 2026-08-02, presented
   against the move option and a keep-the-grader-only option. This supersedes the D6 move
   remainder (the DEFERRED entry's move set, router edits, and CLAUDE.md re-homing are
   discharged by deletion, not executed).
2. **Probe run first, then the pair deleted.** The S14 probe was re-run in this fresh
   session before any deletion, per its delete-when-recorded contract:
   - **Agent half: PASS.** `mochiko-probe` spawned via the Agent tool instantiated as
     itself (reported its seat name and its context), not as a generic agent — the
     fresh-session hypothesis from the v0.44.0 failure confirmed.
   - **Skill half: PASS.** The repo-side `mochiko-probe` skill was discoverable (listed in
     the session's available skills, named in the probe subagent's context) and fired when
     invoked, its body delivered in full.
   - The S14 gate is therefore closed empirically — repo-side agents and skills both work
     from a fresh session — even though the delete ruling makes the move itself moot.
   - Probe pair deleted after recording: `.claude/agents/mochiko-probe.md`,
     `.claude/skills/mochiko-probe/`.
3. **Consequences landed with the deletion (v0.45.0):** `plugin.json` agents 10 → 9;
   router loses the Framework maintenance section, the `command-architect` agent row, the
   `validation-command-shape` family example, and the `validator` row's second skill;
   `agents/validator.md` drops the `validation-command-shape` mount (frontmatter + bullet);
   `ARCHITECTURE.md` loses the Framework maintenance cluster and updates counts;
   `CLAUDE.md` + `.claude/rules/mochiko/primitive-edits.md` re-key the command audit to
   `mochiko:validator` grading against `templates/command-shape.md` as the explicit
   checklist (the validator persona's generic-checklist mode, already designed in).

## Rationale

- The trio's consumers were exclusively mochiko's own command authoring (plus the generic
  validator's declared mount, corrected in the v0.44.0 finding). No pipeline command,
  persona, or skill depends on it.
- The plugin is what adopters install; self-authoring tooling is dead weight there. The
  user ruled the repo does not need a live copy either — command edits are rare, and the
  generic `validator` persona grading against `command-shape.md` (which stays shipped)
  preserves an author ≠ grader audit path without a dedicated skill.
- The deleted skill bodies survive verbatim in git history and their strip notes; nothing
  is unrecoverable if the framework-maintenance cluster is ever rebuilt.

## Alternatives considered

- **Execute the D6 move to `.claude/`** (the standing ruled plan; probe now passing) —
  rejected by user ruling in favor of full deletion.
- **Delete authoring tooling, keep `validation-command-shape` repo-side** so the dedicated
  grader survives — rejected; the generic validator + explicit checklist covers the audit
  need.
