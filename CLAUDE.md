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

> The human-in-loop → mochiko transformation tool (the `transform-cluster` command, its
> `transform-producer` agent, and the `assess-primitive` / `reconcile-cluster` /
> `transform-recipes` / `verify-output` skills) was **retired 2026-07-18** once the migration
> landed. The build record is preserved in `REGISTRY.md`, `ROADMAP.md`, and `BACKLOG.md`, and
> the run archive in `.mochiko/transform/`. New primitives are authored directly in mochiko form.

### Starting a new workflow

1. Check `BACKLOG.md` for any scoping notes on that workflow
2. Identify the skills, agents, and templates in `REGISTRY.md` under that workflow's cluster
3. Port the cluster together — don't port agents without their skills or skills without their agents
4. Update `REGISTRY.md` and resolve any related `BACKLOG.md` items

### Making a structural decision

If a decision touches the four skill-library axes (classification, discoverability, model-invocation reliability, agent↔skill composition) or any of the open design questions in `BACKLOG.md` — record the decision in `ROADMAP.md` under Key Decisions and close the backlog item. Don't let structural decisions live only in conversation context.

### Recording brainstorm and design-session outputs

Brainstorm and design-session artifacts (`record.md`, `synthesis.md`) live in `.mochiko/brainstorms/<topic-slug>/` — never at the repo top level. The top level is reserved for the living operating docs (`CLAUDE.md`, `ROADMAP.md`, `REGISTRY.md`, `BACKLOG.md`). A session's *ruling* still lands in `ROADMAP.md` Key Decisions with a pointer to the session record; the record holds the full rationale.

[`.mochiko/brainstorms/index.md`](.mochiko/brainstorms/index.md) is the session index — newest first, one entry per session: when, status (open / accepted / superseded), review state, what it's about, and where the outcome landed. **Read the index before opening any session directory** — it tells you which records are current, which are superseded, and which are un-reviewed. Enforced both ways: opening a session adds an entry at the top; concluding one updates its status. A session directory without an index entry, or an entry whose status contradicts its record, is a defect — fix it on sight.

## Skill-library conventions (evolving)

These will be extracted from real workflows as they're built. Do not pre-define conventions that no workflow has yet needed. Current adopted axes from the synthesis:

- **Classification**: every skill declares whether it is `user-invoked` or `model-invoked` — the organizing principle for who may call what
- **Discoverability**: one user-invoked router skill indexes the others with when-to-reach-each guidance
- **Model-invocation reliability**: model-invoked skills encode graded trigger phrases in their description
- **Agent↔skill composition**: agents declare a `skills:` list; persona (what the agent cares about) bakes into the agent, procedure (step-by-step how) factors into a skill

Document conventions here as they crystallize from `setup` and `specify`.
