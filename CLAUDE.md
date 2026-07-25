# Mochiko — Operating Manual

## What this is

Mochiko is the v3 successor to [human-in-loop](human-in-loop/). The core bet: engineering discipline lives in the quality of the skill library, not in a deterministic kernel. Native Claude Code agent teams and Workflows handle orchestration. Skills and agents are the primary building block — orchestration is the layer on top, not the enforcer.

Read [`ROADMAP.md`](ROADMAP.md) for the thesis, current work, and standing bets.
Read [`DECISIONS.md`](DECISIONS.md) for the ruled-decision index (rationale lives in session records and `.mochiko/decisions/`).
Read [`BACKLOG.md`](BACKLOG.md) for the complete open-item set.

History: the pre-migration fat `ROADMAP.md` and the retired `REGISTRY.md` are frozen at `.mochiko/archive/` (provenance queries only); closed backlog items live in `.mochiko/archive/backlog-trail.md`.

## Reference sources

> **The `human-in-loop` and `agent-skills-research` submodules were removed on 2026-07-21** so
> the plugin installs cleanly for other users (git otherwise tries to fetch them on install). The
> migration has already landed, so they are not needed day-to-day; the paths below will not exist
> until the submodules are re-added. Restore them only when reference access is genuinely needed:
> ```
> git submodule add https://github.com/deepeshBodh/human-in-loop.git human-in-loop
> git submodule add https://github.com/humaninloop-dev/agent-skills-research.git agent-skills-research
> ```

These were read-only reference sources:

- `human-in-loop/plugins/humaninloop/` — all primitives to cherry-pick (skills, agents, commands, templates)
- `agent-skills-research/synthesis/my-framework.md` — the authoritative v3 design doc

**The techniques plane is more authoritative than re-reading HIL source.** When the synthesis and HIL source conflict on design intent, the synthesis wins.

## Non-negotiable constraints

**No kernel infrastructure.** Never introduce Python/MCP brain code, capability catalogs, or DAG-mediated orchestration. If a workflow needs structure, use native Claude Code Workflows and agent teams.

**Skills and agents are the quality surface.** Discipline is injected through how skills are written and how agents are composed — not through plumbing. A primitive that only works with a brain behind it gets redesigned, not carried forward.

## How to work in this repo

> The HIL→mochiko transformer cluster was **retired 2026-07-18** (`.mochiko/decisions/2026-07-18-transformer-cluster-retired.md`); the run archive is `.mochiko/transform/`. New primitives are authored directly in mochiko form.

### Starting new work

1. Check `BACKLOG.md` for the item and its scoping notes; `ROADMAP.md` for where it sits.
2. Port or author a cluster together — never agents without their skills or skills without their agents.
3. Author ≠ grader: built or converted primitives get an independent audit (`validation-command-shape` for commands; the matching `validation-*`/`review-*` skill otherwise).

### Landing work (the subtractive ritual)

Closing **or superseding** anything is one move, per the project-pinned KM invariants at [`.mochiko/memory/knowledge-management.md`](.mochiko/memory/knowledge-management.md): append the `DECISIONS.md` row (a `.mochiko/decisions/` record when no session record exists) · move the closed `BACKLOG.md` item to the trail · touch `ROADMAP.md` Now/Next — statuses agreeing across the brainstorms index, the record, and the decisions index. A landing that only adds is incomplete. A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight. Don't let structural decisions live only in conversation context. (Compliance here is manual until more commands run in-repo.)

### Recording brainstorm and design-session outputs

Session artifacts (`record.md`, `synthesis.md`) live in `.mochiko/brainstorms/<topic-slug>/` — never at the repo top level. **The top level is reserved for the living operating docs: `CLAUDE.md`, `ROADMAP.md`, `DECISIONS.md`, `BACKLOG.md`** (plus `ARCHITECTURE.md` / `GLOSSARY.md` when they gain content). A session's ruling lands as `DECISIONS.md` row(s) pointing at the record; the record holds the rationale.

[`.mochiko/brainstorms/index.md`](.mochiko/brainstorms/index.md) is the session index — newest first. **Read the index before opening any session directory.** Opening a session adds an entry; concluding one updates it. A directory without an entry, or an entry whose status contradicts its record, is a defect — fix on sight.

## Skill-library conventions (five axes)

1. **Classification** — every skill declares `user-invoked` or `model-invoked`; user-invoked may call model-invoked, never each other.
2. **Discoverability** — one user-invoked router indexes the rest with when-to-reach-each guidance.
3. **Reliable model-invocation** — model-invoked skills encode graded MUST/SHOULD + exact trigger phrases in their `description` (delivery truncates at 1,536 chars — measure first).
4. **Agent↔skill composition** — agents declare `skills:`; persona carries judgment, skill carries procedure; a persona contains no trace of any workflow (decoupling by absence, the keystone test). Caller-side context lives in `agent-dispatch.md`. Details: [`.mochiko/brainstorms/agent-decoupling/synthesis.md`](.mochiko/brainstorms/agent-decoupling/synthesis.md).
5. **Producer↔validator pairing** — every reviewable artifact is graded by a structurally independent validator (different agent, different skill); mirror-checklist form for objective criteria, adversarial-critique form for judgment artifacts.
