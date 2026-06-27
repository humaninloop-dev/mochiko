# Backlog

Lightweight capture of open design decisions and workflow scoping work. Not a full issue tracker — just things that need a decision before building can proceed, or ideas that need to be held without cluttering REGISTRY.md or ROADMAP.md.

Items close when a decision is made and recorded in `ROADMAP.md`, or when the work lands in `REGISTRY.md`.

---

## Open design decisions

These carry forward from `agent-skills-research/synthesis/my-framework.md` — unresolved questions that will surface during workflow builds.

- [ ] **Prose vs. gate allocation** — which behaviors earn graded anti-rationalization prose in skills vs. a hard `PreToolUse` hook interceptor? The synthesis has three poles (persuasion, command-hook, kernel) but the kernel is excluded; allocate between prose and hook for mochiko.

- [ ] **Claude-Code portability** — `rfc2119-invocation-trigger` and `invocation-axis-taxonomy`'s `disable-model-invocation` flag are CC-specific. Decide: adopt the principle and bind to CC, or abstract the mechanism? Likely surfaces when the router skill is built.

- [~] **Memory model** — how does the workspace schema (`[[stateful-workspace-as-memory]]`) relate to the spec/plan/task artifact layout? Is reading-the-workspace-first a mandatory framework step or opt-in? **Provisional answer from the `setup` run (2026-06-27):** setup state lives in-session + under `.mochiko/memory/` (`constitution.md`, `codebase-analysis.md`); state-recovery resumes from workspace evidence (not a context-file `phase` field); the ephemeral context-handoff template (`constitution-context-template`) was *absorbed into the lead* rather than carried as a separate artifact. Still provisional pending `specify`; promote to ROADMAP Key Decisions once `specify` confirms (see "Setup-port follow-ups" below).

- [ ] **Intensity modes** — global `lite/full/ultra/off` dial vs. per-rule feature? (`[[intensity-modes]]` still `maybe` in synthesis.) Defer until at least two workflows are built and the pattern is clear.

- [ ] **`implement` orchestration** — parallel TDD slices genuinely need dependency-ordering; do native Claude Code Workflow `pipeline()` calls suffice, or does implement need a lightweight artifact DAG? Resolve before scoping the `implement` workflow.

---

## Workflow scoping

Notes for upcoming workflows, to be fleshed out before building starts.

- [ ] **`plan`** — decide whether to merge `techspec` functionality in (HIL merged them, recorded in ADR-008); check what `technical-analyst` and `task-architect` agents each own vs. overlap.

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

- [ ] **Confirm the two empirical structural calls in `specify`, then promote to ROADMAP.** Setup is one data point for two ROADMAP open questions; `specify` is the second. When `specify` agrees, move these from provisional to ROADMAP Key Decisions and close the items:
  - **Human-gate placement** (ROADMAP OQ#1) — *on gated dispositions + escalations* (the `loop-discipline` default) worked cleanly for setup: reconcile/redesign/drop dispositions and cap-exhaustion escalations gate; routine PASSes run unattended.
  - **Memory model** (ROADMAP OQ#3 / the Memory-model item above) — in-session + `.mochiko/memory/`, workspace-as-state, context-handoff absorbed into the lead.

## Ideas / candidates

Things worth holding but not yet scoped.

- [ ] **Brownfield onboarding path** — `[[brownfield-first-onboarding]]` from the synthesis; HIL has `brownfield-constitution` and `brownfield-integration` skills. Worth keeping as a distinct entry path into `setup`.

- [ ] **Context handoff document** — `[[context-handoff-document]]` for serializing in-flight work across session boundaries; pairs with the router skill. Low priority until other workflows are landed. **Data point from `setup`:** that cluster's HIL context-handoff template (`constitution-context-template`) was *absorbed into the lead* (in-session + `.mochiko/memory/`) rather than kept as a serialized doc — so a cross-session handoff doc, if pursued, is a deliberate add-on, not a carried-over HIL primitive.

- [ ] **Deliberate shortcut ledger** — `[[deliberate-shortcut-ledger]]` for tracking deferral decisions with upgrade triggers; relevant once `implement` is built and shortcuts start accumulating.
