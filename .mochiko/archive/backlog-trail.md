# Backlog Trail — append-only

Closed BACKLOG items, one line each (DONE date + pointer). Entries append at the bottom;
never edited, never deleted. Read-job: resume-cold on a reopened item + provenance lookup.
Seeded 2026-07-25 by the operating-docs migration from the pre-migration `BACKLOG.md`
(full original entries: git history at `7920ccb` and earlier).

## Migration snapshot (2026-07-25)

- DONE 2026-06-27 — Memory model resolved (in-session + workspace-as-state) → `.mochiko/decisions/2026-06-27-roadmap-v2-rebaseline.md`
- DONE 2026-06-27 — Setup/specify empirical calls confirmed (human-gate placement · memory model) → same ADR
- DONE 2026-06-27 — Decoupling doctrine validated on the specify port → `.mochiko/brainstorms/agent-decoupling/synthesis.md`
- DONE 2026-06-30 — Command-altitude pass: recipe fix + specify/setup retrofit → `.mochiko/brainstorms/command-altitude/synthesis.md`
- DONE 2026-07-01 — Altitude shape applied to plan/tasks/implement (thin by construction) → archive ROADMAP Decision Trail
- DONE 2026-07-01 — `implement` orchestration resolved (sequential-first) → `.mochiko/decisions/2026-06-27-roadmap-v2-rebaseline.md`
- DONE 2026-07-01 — `plan` · `tasks` · `implement` ported (workflow-scoping rows) → `.mochiko/transform/*/report.md`
- DONE 2026-07-01 — devils-advocate deferred mounts re-mounted; `strategy-*` family fully dissolved → archive ROADMAP Decision Trail
- DONE 2026-07-01 — `validation-task-artifacts` re-mounted · TEST-classification reclaimed · vertical-tdd↔executing boundary confirmed · task-architect decoupling proven → `.mochiko/transform/tasks/`, `/implement/`
- DONE 2026-07-01 — `brownfield-integration` home confirmed implement (REGISTRY mis-file corrected) → archive REGISTRY
- DONE 2026-07-02 — `brainstorm` v1 built + dogfooded (drove v2); plan-ramp/tasks-variant watches retired with v2 → `.mochiko/brainstorms/brainstorm-command/synthesis.md`
- DONE 2026-07-02 — `slice` / vertical-graduation substrate scoped + built → `.mochiko/brainstorms/vertical-graduation/synthesis.md`
- DONE 2026-07-04/05 — brainstorm v2 + v2.1 dogfoods (drove v2.1/v2.2) → `.mochiko/brainstorms/brainstorm-v2-revision/record.md`
- DONE 2026-07-16 — Setup v2 dogfood superseded by v3; amend-backfill moot under v3 → archive ROADMAP Decision Trail
- DONE 2026-07-18 — D7 substrate defect resolved (team-transport fix, agent-dispatch v3) → `.mochiko/brainstorms/setup-v3-team-defect/record.md`
- DONE 2026-07-18 — Setup-adversarial-review build (review-governance-intent · template deltas · setup propagation · seven-skill rename set, v0.10.0) → `.mochiko/brainstorms/setup-adversarial-review/record.md`
- DONE 2026-07-19 — Pattern-codification build (command-shape.md · command-architect + authoring-commands · validation-command-shape · strips convention · one-shot-line demotion · specify frontmatter fix, v0.11.0); five command conversion waves v0.13.0–v0.17.0 → `.mochiko/brainstorms/pattern-codification-and-minimalism/record.md` + `.mochiko/strips/`
- DONE 2026-07-19 — Governance-injection probe suite productized (v0.12.0) → `.mochiko/decisions/2026-07-19-governance-injection-probe.md`
- DONE 2026-07-21 — `approved-domain-deps.md` fate resolved (dissolved: registry block + ledger + DOMAIN-DEPENDENCIES.md, v0.18.0) → `.mochiko/brainstorms/domain-dependency-allowlist/record.md`
- DONE 2026-07-24 — Token D4 reference-by-ID (wave 2, v0.23.0, `templates/artifact-format.md`) → `.mochiko/decisions/2026-07-24-token-reduction-wave-2.md`
- DONE 2026-07-25 — Succinctness pilot + R1 measurement + D1–D4 checkpoint + ≥3-consumer queue + waves 1–4 (v0.24.0–v0.28.0; 27 skills −23.7%) → `.mochiko/brainstorms/skill-succinctness-strip/record.md` + `.mochiko/strips/`
- CLOSED-ABANDONED 2026-07-25 — Wire cross-cutting stubs (`syncing-claude-md` died with the v3 dissolution + OD-D7; `authoring-roadmap` abandoned per R3) → archive REGISTRY disposition table
- CLOSED-ABANDONED 2026-07-25 — Roadmap-track deferrals (plan-port + tasks-port; `evolution-roadmap-template` with `authoring-roadmap`; the task-architect `[GAP:XXX]` read stays a documented stub) → archive REGISTRY disposition table
- CLOSED-ABANDONED 2026-07-25 — Wire the KM sync rows when `syncing-claude-md` ports (carrier replaced by scaffolded CLAUDE.md pointers, OD-D7) → `.mochiko/brainstorms/operating-docs-maintenance/record.md`
- CLOSED 2026-07-25 — Implement-port deferral row (approved-domain-deps resolved v0.18.0; cross-cutting utilities abandoned) → archive REGISTRY disposition table
- DONE 2026-07-25 — Operating-docs build items 1–7 (KM template rewrite · setup G5 · command landing steps · grep floor · two new skills + boundary edits · this migration · D12 watch landed in the groom skill), v0.29.0 → `.mochiko/brainstorms/operating-docs-maintenance/record.md`
- DONE 2026-07-30 — Layer-2 mesh rewrite (team-method D1–D3 → shape v4; six commands conformed at v0.31.0, all audit-PASSed; keeper-skill Job-4 mode encoded same day at v0.30.0) → `.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md` + `.mochiko/strips/`
- DONE 2026-07-30 — Architecture-primitive cluster built (merged `/mochiko:plan` with architecture stage + early sign-off · implement AD-D6 edits · `system-architect` + `patterns-system-design` · four skill extensions · router · `/mochiko:tasks` retirement), v0.32.0; both independent audits PASS → `.mochiko/brainstorms/architecture-design-primitive/record.md` + `.mochiko/strips/`
- DONE 2026-07-30 — Plan absorbs tasks (team-method D4+D5): `/mochiko:tasks` merged into `/mochiko:plan` (7 → 6 commands, one final package acceptance, validators unchanged), built with the architecture cluster at v0.32.0 → `.mochiko/brainstorms/team-method-vs-command-shape/record.md` + `.mochiko/strips/tasks.md`
- DONE 2026-07-30 — Goal-shape rebuild step 1: shape v5 + revised grader (v0.33.0; five-block anatomy · Seat transport absorbed · sized-review conditional split; floor checkpoint A+B + KM/run-cost adjudications user-ruled; audit FAIL→fix→PASS) → ADRs `2026-07-30-goal-shape-step1-checkpoint` / `2026-07-30-goal-shape-step1-adjudications` + `.mochiko/strips/command-shape.md`
