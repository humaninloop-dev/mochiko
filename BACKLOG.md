# Backlog

Lightweight capture of open design decisions and workflow scoping work. Not a full issue tracker — just things that need a decision before building can proceed, or ideas that need to be held without cluttering REGISTRY.md or ROADMAP.md.

Items close when a decision is made and recorded in `ROADMAP.md`, or when the work lands in `REGISTRY.md`.

---

## Open design decisions

These carry forward from `agent-skills-research/synthesis/my-framework.md` — unresolved questions that will surface during workflow builds.

- [ ] **Prose vs. gate allocation** — which behaviors earn graded anti-rationalization prose in skills vs. a hard `PreToolUse` hook interceptor? The synthesis has three poles (persuasion, command-hook, kernel) but the kernel is excluded; allocate between prose and hook for mochiko.

- [ ] **Claude-Code portability** — `rfc2119-invocation-trigger` and `invocation-axis-taxonomy`'s `disable-model-invocation` flag are CC-specific. Decide: adopt the principle and bind to CC, or abstract the mechanism? Likely surfaces when the router skill is built.

- [ ] **Memory model** — how does the workspace schema (`[[stateful-workspace-as-memory]]`) relate to the spec/plan/task artifact layout? Is reading-the-workspace-first a mandatory framework step or opt-in? Resolve when `setup` is built.

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

## Ideas / candidates

Things worth holding but not yet scoped.

- [ ] **Brownfield onboarding path** — `[[brownfield-first-onboarding]]` from the synthesis; HIL has `brownfield-constitution` and `brownfield-integration` skills. Worth keeping as a distinct entry path into `setup`.

- [ ] **Context handoff document** — `[[context-handoff-document]]` for serializing in-flight work across session boundaries; pairs with the router skill. Low priority until other workflows are landed.

- [ ] **Deliberate shortcut ledger** — `[[deliberate-shortcut-ledger]]` for tracking deferral decisions with upgrade triggers; relevant once `implement` is built and shortcuts start accumulating.
