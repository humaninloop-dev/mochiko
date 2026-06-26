# Mochiko Roadmap — Brainstorm Synthesis

_Generated: 2026-06-26_

## Problem Statement

Mochiko is a v3 framework that delivers the discipline of human-in-loop without its Python/MCP deterministic kernel. The bet: discipline lives in the quality of the skill library itself; native Claude Code agent teams handle orchestration. Skills and agents are the primary building block — orchestration is the layer on top, not the enforcer.

## Context & Constraints

- **Current state is human-in-loop**: all primitives to cherry-pick are there; kernel (~2,951 LOC + 409 tests) is explicitly excluded
- **Two submodules available for reference**: `human-in-loop/` and `agent-skills-research/` (with synthesis doc)
- **Source of truth for v3 design**: `agent-skills-research/synthesis/my-framework.md` — the techniques plane is more authoritative than re-reading HIL source
- **Native substrate**: Claude Code agent teams, Workflows, and the skill/agent plugin system — no custom orchestration infrastructure

## Key Decisions

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| Kernel architecture | Kernel-free — no Python/MCP DAG | Confident | Accidental complexity for sequential workflows; Claude agent teams solve the isolation problem at the platform layer |
| Primary quality surface | Skill library + agents | Confident | Discipline injected through quality primitives, not plumbing; orchestration consumes quality, doesn't create it |
| Build approach | Workflow-first, one at a time | Confident | Each workflow teaches what the skill conventions actually need from a real case before generalizing |
| Build order | `setup` first, then `specify` | Confident | Setup establishes the project constitution (the standards root); specify proves the adversarial agent team pattern |
| Migration tracking | Explicit registry with ported / not-yet status | Confident | Nothing should fall through the gap between "cherry-picked" and "not yet touched" |

## Decision Trail

### Kernel-free vs. carrying the deterministic core

- **Options considered**: keep the Python/MCP kernel for parallel/dependency-rich workflows (implement); shed it entirely in favor of agent teams
- **Recommendation was**: keep kernel only where work is genuinely parallel/dependency-rich
- **Chosen**: shed entirely for now — kernel-free
- **Key reasoning**: the synthesis explicitly kept `[[deterministic-core-llm-shell]]` at `maybe` as a signal the question is open; user resolved it definitively toward agent-team-native. The kernel can be revisited if agent teams prove insufficient for `implement`'s parallel TDD slices — but that's a future problem.

### Workflow-first vs. infrastructure-first

- **Options considered**: establish four-axis skill-library conventions before any specific skills; prove concept with one workflow; port-and-upgrade HIL skills wholesale
- **Chosen**: workflow-first — `setup` then `specify`
- **Key reasoning**: conventions should emerge from real cases, not be imposed top-down. Each workflow will reveal what the skill structure needs.

## Open Questions

From the synthesis, still unresolved and relevant to mochiko:

1. **Prose vs. gate allocation** — which behaviors earn graded anti-rationalization prose vs. a hard `PreToolUse` interceptor gate? (`[[deny-list-guardrail-hook]]` is still `maybe`)
2. **Claude-Code portability** — how much of the skill-library kit (`rfc2119-invocation-trigger`, `invocation-axis-taxonomy`'s flag) is CC-specific vs. abstractable across hosts?
3. **Memory model** — how does the workspace schema relate to the existing spec/plan/task artifact layout? Is reading-the-workspace-first a mandatory framework step?
4. **Intensity as a primitive** — global dial (`lite/full/ultra/off`) or per-rule feature? (`[[intensity-modes]]` still `maybe`)
5. **Staged-workflow spine for `implement`** — the only workflow where parallel TDD slices genuinely need dependency-ordering; does it need any lightweight DAG, or do native Workflow pipelines suffice?

## Recommended Next Steps

1. **Build `setup`**: port and upgrade `principal-architect` agent + the constitution skills cluster (`authoring-constitution`, `brownfield-constitution`, `validation-constitution`, `analysis-codebase`, `syncing-claude-md`). The v3 upgrade: shed the brain-mediated DAG invocation, make it a clean skill-invocation chain. See `REGISTRY.md` for the full dependency list.

2. **Build `specify`**: rebuild as a 2-member agent team — analyst (producer) + advocate (adversarial critic), lead as referee. Keep the three non-negotiable invariants from `[[adversarial-pair-convergence-loop]]`: referee owns the verdict, hard round cap, preference gaps escalate to human. Shed the capability-catalog and context-template DAG machinery entirely.

3. **Establish the four-axis skill-library conventions as you go** — don't pre-define them; extract them from what `setup` and `specify` actually need. After two workflows, write the conventions as a living document.

4. **Update `REGISTRY.md` as each primitive is ported** — mark each skill/agent/template `[x]` when it lands in mochiko, so the gap is always visible.
