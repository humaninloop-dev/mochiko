# Backlog

Lightweight capture of open design decisions and workflow scoping work. Not a full issue tracker — just things that need a decision before building can proceed, or ideas that need to be held without cluttering REGISTRY.md or ROADMAP.md.

Items close when a decision is made and recorded in `ROADMAP.md`, or when the work lands in `REGISTRY.md`.

---

## Open design decisions

These carry forward from `agent-skills-research/synthesis/my-framework.md` — unresolved questions that will surface during workflow builds.

- [ ] **Prose vs. gate allocation** — which behaviors earn graded anti-rationalization prose in skills vs. a hard `PreToolUse` hook interceptor? The synthesis has three poles (persuasion, command-hook, kernel) but the kernel is excluded; allocate between prose and hook for mochiko.

- [ ] **Claude-Code portability** — `rfc2119-invocation-trigger` and `invocation-axis-taxonomy`'s `disable-model-invocation` flag are CC-specific. Decide: adopt the principle and bind to CC, or abstract the mechanism? Likely surfaces when the router skill is built.

- [x] **Memory model** — **RESOLVED 2026-06-27 (confirmed by `specify`; promoted to ROADMAP Key Decisions).** Originally: how does the workspace schema (`[[stateful-workspace-as-memory]]`) relate to the spec/plan/task artifact layout? Is reading-the-workspace-first a mandatory framework step or opt-in? **Provisional answer from the `setup` run (2026-06-27):** setup state lives in-session + under `.mochiko/memory/` (`constitution.md`, `codebase-analysis.md`); state-recovery resumes from workspace evidence (not a context-file `phase` field); the ephemeral context-handoff template (`constitution-context-template`) was *absorbed into the lead* rather than carried as a separate artifact. Confirmed by the `specify` run — its `context-template` absorb-into-lead is a *stronger* data point (no drift; more state, still dissolves cleanly). Promoted to ROADMAP Key Decisions.

- [ ] **Intensity modes** — global `lite/full/ultra/off` dial vs. per-rule feature? (`[[intensity-modes]]` still `maybe` in synthesis.) Defer until at least two workflows are built and the pattern is clear.

- [ ] **`implement` orchestration** — parallel TDD slices genuinely need dependency-ordering; do native Claude Code Workflow `pipeline()` calls suffice, or does implement need a lightweight artifact DAG? Resolve before scoping the `implement` workflow.

- [ ] **Command orchestration substrate — agent teams vs. `Task`-subagents** — DEFERRED from the 2026-06-30 altitude work (`COMMAND-ALTITUDE-SYNTHESIS.md`). The thin commands are substrate-agnostic and currently use one-shot `Task`-subagent dispatch under a prose lead, though the ROADMAP thesis says "agent teams handle orchestration" — a thesis↔build gap. Inputs already gathered: agent teams give producer↔validator independence for free (isolated context) **but** are experimental/flag-gated (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`), ephemeral, **ignore the `skills:` frontmatter for teammates** (hits convention axis 4), and their direct teammate↔teammate messaging can *dilute* the artifact-grounded independence an adversarial pair needs. A thin command now exists to evaluate against. Relates to the Claude-Code portability item.

---

## Workflow scoping

Notes for upcoming workflows, to be fleshed out before building starts.

- [x] **`plan`** — **DONE (ported 2026-07-01).** techspec merge → **excluded** (HIL already merged it via ADR-008; the ported `plan` IS the merged form). technical-analyst vs task-architect overlap → no conflict: `task-architect` is the **tasks** cluster (not pulled into plan-core); `technical-analyst` owns plan's analysis+design. See ROADMAP Decision Trail + the plan-port follow-ups below.

- [ ] **`tasks`** — `task-architect` + `validation-task-artifacts` + `validation-plan-artifacts` are the obvious cluster; check if `strategy-core` is consumed directly or through `state-analyst`.

- [ ] **`implement`** — depends on the orchestration decision above; hold until that backlog item is resolved.

- [ ] **`audit`** — lowest priority; scope after `implement` is landed.

---

## Setup-port follow-ups (from the `/mochiko:transform-cluster setup` run, 2026-06-27)

Concrete deferred work left by the setup-cluster port (**core-only** scope). Full run record + rationale in `.mochiko/transform/setup/` (`report.md`, `reconcile.md`, per-primitive assessments). Each item has enough context to resume cold. Shipped in PR #4 (`port-setup-cluster`).

- [ ] **Wire the cross-cutting stubs when their clusters port.** The setup port left `syncing-claude-md` (CLAUDE.md governance sync) and `authoring-roadmap` (evolution-roadmap) as *documented reference stubs*, not live skill mounts — mounting an unported skill would dangle. They were removed from `principal-architect`'s `skills:` list this run. Locations to update later: `plugins/mochiko/skills/authoring-constitution/SKILL.md` (Related + the CLAUDE.md-sync section spec), `plugins/mochiko/commands/setup.md` (Phase 5 stub), `plugins/mochiko/agents/principal-architect.md` (re-add to `skills:`). When those clusters land, re-mount and replace the stub prose.

- [ ] **Port `codebase-inventory-schema.json` with the spec/plan cluster.** Orphan in setup (no consumer after mode-scoping `analysis-codebase` to its setup-brownfield slice). Its `collision_risks`/`spec_item` shape is the contract for `analysis-codebase`'s *collision / spec-plan mode*, which was tagged moved-to-other-cluster. Port it alongside that mode when spec/plan is built and wire its consumer then. HIL source: `human-in-loop/plugins/humaninloop/templates/codebase-inventory-schema.json`.

- [ ] **Confirm `brownfield-integration`'s home (REGISTRY mis-file).** REGISTRY filed it in the setup cluster, but the setup agent/command never invoke it and it reads as implement-time (`[EXTEND]`/`[MODIFY]` markers, integrating against existing interfaces). Deferred from this run. Decide its real cluster (likely `implement`) when that workflow is scoped, and fix the REGISTRY row. HIL source: `human-in-loop/plugins/humaninloop/skills/brownfield-integration/`.

- [ ] **Decide `approved-domain-deps.md`'s fate.** Referenced by `constitution-template.md` and `authoring-constitution/references/{EMERGENT-CEILING-PATTERNS,RECOMMENDED-PATTERNS}.md`. The live `${CLAUDE_PLUGIN_ROOT}` path was dropped and softened to prose (project-maintained registry) this run, accepted at the human gate. Later: ship a registry template in mochiko, or keep as project-maintained prose. Low priority.

- [ ] **Dogfood `/mochiko:setup` for real (behavioral validation).** The port passed *structural* verification (conformance + trace audit), but the workflow has not run end-to-end. Run it on a real project — greenfield: the mochiko repo itself; brownfield: human-in-loop — to confirm the produce→validate loop converges, the validator's fix-list flows back to the producer, the brownfield `detect-stack.sh` path works, and the `AskUserQuestion` gates fire sensibly. Cheapest behavioral check before relying on the port. **Recommended next action after PR #4 merges.**

- [x] **Confirm the two empirical structural calls in `specify` → DONE (2026-06-27).** Both confirmed by the `specify` run and promoted to ROADMAP Key Decisions:
  - **Human-gate placement** (ROADMAP OQ#1) — *on gated dispositions + escalations* (the `loop-discipline` default) worked cleanly for setup: reconcile/redesign/drop dispositions and cap-exhaustion escalations gate; routine PASSes run unattended.
  - **Memory model** (ROADMAP OQ#3 / the Memory-model item above) — in-session + `.mochiko/memory/`, workspace-as-state, context-handoff absorbed into the lead.

- [x] **Validate the agent↔workflow decoupling doctrine by porting `specify` → DONE (2026-06-27): doctrine HELD.** No new persona/skill acquired a deny-list token; the only genuine hit was the canonical grep-catchable `analysis-specifications` "Devil's Advocate" case (decoupled to role); `verify-output`'s scan + keystone test caught every case; no deny-list refinement was needed. The decoupling rules — personas/skills carry no workflow vocabulary, independence is structural, one generic `validator` — landed in the agent personas, the transformer skills, and conventions 4 & 5 (full record in `AGENT-DECOUPLING-SYNTHESIS.md`; ROADMAP Key Decisions + Decision Trail). The empirical test is the next port: run `/mochiko:transform-cluster specify` and confirm (a) no new persona or skill acquires a deny-list token (sibling-agent name, "dispatch," injected workflow modes/paths/phases, "workflow-agnostic" meta-label), and (b) `verify-output`'s decoupling scan + the keystone test catch any that slip in. If the doctrine holds it is proven; if a primitive genuinely needs a token, refine the deny-list/keystone test rather than carry the coupling. Pairs with the dogfood-`/mochiko:setup` behavioral check above.

## Specify-port follow-ups (from the `/mochiko:transform-cluster specify` run, 2026-06-27)

Deferred work from the specify-cluster port (**core-only** scope). Full run record in `.mochiko/transform/specify/` (`report.md`, `reconcile.md`, per-primitive assessments). All 14 primitives PASSed independent verification in a single round.

- [~] **Re-mount `devils-advocate`'s deferred validation skills.** `validation-plan-artifacts` → **DONE (plan port, 2026-07-01)** — now a live mount (plan completeness reviewer). `validation-task-artifacts` → **still stubbed** (comment-only; re-mount when `tasks` ports — a live mount of the unported skill would dangle). Location: `plugins/mochiko/agents/devils-advocate.md`.
- [ ] **`strategy-implementation` will likely dissolve into `loop-discipline`** when `implement` ports — the same dedupe `strategy-core` + `strategy-specification` took this run. REGISTRY notes this on its row.
- [ ] **`ui-designer` + design track remain deferred** (out of specify-core; the specify catalog never invokes them). Scope with a dedicated design cluster or `plan`. HIL sources: `agents/ui-designer.md` + `skills/{analysis-screenshot, authoring-design-system, patterns-flow-mapping, patterns-interface-design}`.
- [ ] **Dogfood `/mochiko:specify` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature to confirm the produce→critique loop converges, the advocate's gap list flows back as a targeted revision, the gap-classification routing (Explore / clarification / halt) fires sensibly, and the G3 acceptance gate works. Pairs with the still-open dogfood-`/mochiko:setup` check.

## Plan-port follow-ups (from the `/mochiko:transform-cluster plan` run, 2026-07-01)

Deferred work from the plan-cluster port (**core-only** scope). Full run record in `.mochiko/transform/plan/` (`report.md`, `reconcile.md`, per-primitive assessments + traces). All 15 primitives + the wiring PASSed independent verification in a single round, zero required fixes.

- [ ] **Re-mount `validation-task-artifacts` on `devils-advocate` when `tasks` ports.** The only remaining stubbed mount on the advocate (the plan re-mount of `validation-plan-artifacts` is done). Location: `plugins/mochiko/agents/devils-advocate.md`.
- [ ] **Reclaim `validation-plan-artifacts`'s Phase-A0 codebase-discovery review when the brownfield/discovery track ports.** Tagged `moved-to-other-cluster` (out of plan-core); the review capability is *parked* (documented in the skill's `references/ARTIFACT-CHECKLISTS.md`), not dropped — the receiving cluster must reclaim it.
- [ ] **`evolution-roadmap-template` + `authoring-roadmap` remain deferred** (roadmap / setup-brownfield track — REGISTRY-filed under plan but never invoked by `/plan`). Port with the roadmap track.
- [ ] **Design track (`patterns-flow-mapping`, `patterns-interface-design`, `ui-designer`) + `patterns-vertical-tdd` remain deferred** — they belong to the design track and the tasks cluster, not plan-core. (The specify-port design-track deferral already tracks the first three.)
- [ ] **Dogfood `/mochiko:plan` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature (after `/mochiko:specify` produces a spec) to confirm: the producer→two-reviewer loop converges, the feasibility-once-then-completeness ordering + skip-architect routing fire sensibly, the `infeasible`→business-escalation gate works, the Phase-2 incremental review works, and the G5 `plan.md` acceptance gate fires. Pairs with the still-open dogfood-`/mochiko:setup` + `/mochiko:specify` checks.
- [x] **Quickstart / Integration Guide label aligned.** The `technical-analyst` persona's deliverable #6 was "Integration Guide" while the router/command/template name the file `quickstart.md`; aligned the persona label to "Quickstart / Integration Guide" (coupling-free; the file stays `quickstart.md`, canonical everywhere that gates). Finalize housekeeping surfaced by the P10 + cluster verifies.

## Command-altitude pass (2026-06-30)

The first two command ports were re-transformed to the thin "stitch a team to a goal under a contract" altitude, and the transform recipe was fixed so future commands come out thin by construction. Full record: `COMMAND-ALTITUDE-SYNTHESIS.md`; decision in ROADMAP (Key Decisions + Decision Trail).

- [x] **Fix the transform recipe for altitude → DONE.** `assess-primitive` (generic discipline → `dedupe`, not `moved-to-lead`), `transform-recipes` (thin-command `redesign` target + wiring step 6 single-source), `verify-output` (altitude / single-source conformance item 8 = grep floor + keystone ceiling), `PLAYBOOK` (altitude in the mochiko-form definition).
- [x] **Retrofit `specify` + `setup` → DONE.** specify 329→66, setup 385→78; both independently verified PASS (8 conformance items + altitude scan + trace audit; every workflow-specific responsibility preserved, generic discipline deduped to references).
- [~] **Apply the altitude shape to `plan`/`tasks`/`implement` when they port.** `plan` → **DONE (2026-07-01)**: came out at 82 lines thin-by-construction; `verify-output`'s altitude scan PASSed first try (no retrofit needed). `tasks`/`implement` remain. The fixed recipe produces it by default; watch the "what you own" footer if a command grows.

## Ideas / candidates

Things worth holding but not yet scoped.

- [ ] **Brownfield onboarding path** — `[[brownfield-first-onboarding]]` from the synthesis; HIL has `brownfield-constitution` and `brownfield-integration` skills. Worth keeping as a distinct entry path into `setup`.

- [ ] **Context handoff document** — `[[context-handoff-document]]` for serializing in-flight work across session boundaries; pairs with the router skill. Low priority until other workflows are landed. **Data point from `setup`:** that cluster's HIL context-handoff template (`constitution-context-template`) was *absorbed into the lead* (in-session + `.mochiko/memory/`) rather than kept as a serialized doc — so a cross-session handoff doc, if pursued, is a deliberate add-on, not a carried-over HIL primitive.

- [ ] **Deliberate shortcut ledger** — `[[deliberate-shortcut-ledger]]` for tracking deferral decisions with upgrade triggers; relevant once `implement` is built and shortcuts start accumulating.
