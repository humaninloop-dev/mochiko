# Agent Dispatch Briefing — [AGENT] for [WORKFLOW], [PHASE]

A caller-side checklist, not a file you commit: fold the fields below into the prompt of
each dispatch a `commands/*.md` supervisor makes. A good brief carries the context below.
None of it is a precondition for the agent to *function* — the agent degrades gracefully
on a thin brief — it is how the caller gets the agent's *best* work and keeps the loop
sound. The agent owns none of this knowledge (workflow, siblings, "done") — all of that
lives on the caller side; never push it into a persona. Name the skill as a hint, not a
command: the agent decides whether it fits. (Rationale + provenance:
`.mochiko/strips/agent-dispatch.md`.)

| # | Field | What to provide | Example |
|---|-------|----------------|---------|
| 1 | **Skill(s) to lean on** | The model-invoked skill(s) likely to fit this work, by name — a hint the agent can take or set aside | "This is in `mochiko:validation-constitution`'s domain." |
| 2 | **Role this run** | What the agent is doing in plain terms — author / grade / reconcile — so it frames its output | "You are grading this artifact independently." |
| 3 | **Input(s) to read** | Every artifact the agent should Read before acting (and any in-session inputs) — incl. the governance obligated-read line naming relevant `.claude/rules/mochiko/` files | "Read `.mochiko/memory/governance-intent.md` and `…/codebase-analysis.md`; read `.claude/rules/mochiko/api.md` before authoring." |
| 4 | **Where the output goes** | The path to write to, or "return only, write nothing" | "Write the governance region into `CLAUDE.md` (between the mochiko:governance markers only)." |
| 5 | **What good looks like** | The bar the output must clear this run | "Every principle has Statement/Enforcement/Testability/Rationale; no placeholders." |
| 6 | **Prior feedback (retries)** | On round > 1, paste the validator's prior issues verbatim; else omit | "Address: <validator's issues-requiring-fix>." |
| 7 | **Independence framing** | The plain-language reminder that matches the structural guarantee | Author: "Don't grade your own output." · Grader: "Read the artifact itself; default FAIL; quote your evidence." |
| 8 | **Return vs. write** | What to return in the reply vs. what to persist to a file | "WRITE the artifact; RETURN a short report + any clarifications you need." |

A field you leave out isn't a failure — it's context the agent will ask for or supply from
its own judgment. Fill what raises quality; trust the professional with the rest.

---

## What the caller MUST get right (the one hard line)

Everything above is quality. **Independence is structure, and it is not optional:**

- [ ] The agent dispatched to **grade** is a *different* agent, running a *different* skill,
      than the one dispatched to **produce**.
- [ ] No agent is ever asked to grade its own output.

This is the caller's job, carried by *who it dispatches* — never by a line in the persona.
A loop that violates it is unsound (see `loop-discipline` req. 2), no matter how well the
other fields are filled.

> A thin brief is a quality cost, recoverable by the agent. A collapsed
> producer↔validator boundary is an unsound loop, recoverable by no one downstream.

---

## Seat transport (team-form commands only)

A team-form command's seats ride the **same Agent tool** as one-shot subagents — since
Claude Code v2.1.178 there is no separate team-creation step, the fork is one parameter,
and the substrate documentedly picks wrong sometimes (*"Claude may sometimes use subagents
instead of creating a team"* — agent-teams docs). So the caller carries the mechanics, not
just the vocabulary:

- **Spawning a seat** = one Agent call carrying **`name:`** (e.g. `name: producer`), phrased
  in the docs' own idiom — "create an agent team", "spawn a teammate named `<seat>`" — not
  only mochiko's "seat". **A spawn without a `name:` is a one-shot subagent — in a team-form
  command, the forbidden transport.**
- **Every later round** is a `SendMessage` to that same name. A fresh spawn per round is the
  subagent anti-pattern wearing a team's clothes.
- **Verify before proceeding:** the first spawn is the authoritative probe — confirm it
  yielded an **addressable teammate** (a named agent you can `SendMessage`; the agent panel
  alone doesn't distinguish teammates from subagents, per the docs). Not addressable → kill
  it and respawn, explicitly requesting an agent team.

All seven commands are currently team-form, each per its recorded conversion assessment
with a first-dogfood confirm-or-revert checkpoint (`.mochiko/strips/<command>.md`;
assessment doctrine: `.mochiko/brainstorms/pattern-codification-and-minimalism/record.md`,
D2). One-shot dispatch remains the rebuttable Layer-1 default for any future command
designed on it; this section binds only commands that hard-require teams. Defect history +
ruling: `.mochiko/brainstorms/setup-v3-team-defect/record.md` (D1).

**Briefing version:** v4 (2026-07-23 — header relocated to the strip note; roster
staleness fixed) · **Governed by:** `loop-discipline` · **Pairs with:** `workflow-contract.md`
