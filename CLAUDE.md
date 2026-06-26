# Mochiko — Operating Manual

## What this is

Mochiko is the v3 successor to [human-in-loop](human-in-loop/). The core bet: engineering discipline lives in the quality of the skill library, not in a deterministic kernel. Native Claude Code agent teams and Workflows handle orchestration. Skills and agents are the primary building block — orchestration is the layer on top, not the enforcer.

Read [`ROADMAP.md`](ROADMAP.md) for the full v3 thesis and architectural decisions.
Read [`REGISTRY.md`](REGISTRY.md) for the migration inventory — what's been ported, what hasn't.
Read [`BACKLOG.md`](BACKLOG.md) for open design decisions and upcoming workflow scoping.

## Reference sources

Both repos are submodules — use them as read-only reference:

- `human-in-loop/plugins/humaninloop/` — all primitives to cherry-pick (skills, agents, commands, templates)
- `agent-skills-research/synthesis/my-framework.md` — the authoritative v3 design doc; read this before making any structural decisions

**The techniques plane is more authoritative than re-reading HIL source.** When the synthesis and HIL source conflict on design intent, the synthesis wins.

## Non-negotiable constraints

**No kernel infrastructure.** Never introduce Python/MCP brain code, capability catalogs, or DAG-mediated orchestration. If a workflow needs structure, use native Claude Code Workflows and agent teams.

**Skills and agents are the quality surface.** Discipline is injected through how skills are written and how agents are composed — not through plumbing. When porting a primitive, ask: does this skill survive without the brain? If not, redesign it, don't carry the brain dependency forward.

**Workflow-first build order.** Port primitives in the context of a workflow, not in isolation. The workflow reveals what the skill actually needs. See `REGISTRY.md` for the build order.

## How to work in this repo

### Porting a primitive from human-in-loop

1. Read the HIL source in `human-in-loop/plugins/humaninloop/`
2. Check `agent-skills-research/synthesis/my-framework.md` for any technique notes on it
3. Ask: what does this primitive do that the brain was doing for it? Replace brain dependencies with skill-invocation or agent-team patterns
4. Mark it `[x]` in `REGISTRY.md` when landed; `[~]` if ported but needs a follow-up upgrade pass

### Starting a new workflow

1. Check `BACKLOG.md` for any scoping notes on that workflow
2. Identify the skills, agents, and templates in `REGISTRY.md` under that workflow's cluster
3. Port the cluster together — don't port agents without their skills or skills without their agents
4. Update `REGISTRY.md` and resolve any related `BACKLOG.md` items

### Making a structural decision

If a decision touches the four skill-library axes (classification, discoverability, model-invocation reliability, agent↔skill composition) or any of the open design questions in `BACKLOG.md` — record the decision in `ROADMAP.md` under Key Decisions and close the backlog item. Don't let structural decisions live only in conversation context.

## Skill-library conventions (evolving)

These will be extracted from real workflows as they're built. Do not pre-define conventions that no workflow has yet needed. Current adopted axes from the synthesis:

- **Classification**: every skill declares whether it is `user-invoked` or `model-invoked` — the organizing principle for who may call what
- **Discoverability**: one user-invoked router skill indexes the others with when-to-reach-each guidance
- **Model-invocation reliability**: model-invoked skills encode graded trigger phrases in their description
- **Agent↔skill composition**: agents declare a `skills:` list; persona (what the agent cares about) bakes into the agent, procedure (step-by-step how) factors into a skill

Document conventions here as they crystallize from `setup` and `specify`.
