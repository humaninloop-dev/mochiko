<!--
AGENT DISPATCH BRIEFING — a caller-side guide
=============================================
A checklist for the CALLER (a `commands/*.md` supervisor) when it dispatches a mochiko
agent. It is a briefing guide, NOT a correctness gate.

WHY THIS EXISTS:
A mochiko agent is a self-contained professional: it carries its craft and the skills it
can lean on, and it DEGRADES GRACEFULLY. Hand it a rich brief and it does its best work
immediately; hand it a thin one and it still works — it falls back on its own method and
asks for what it genuinely needs rather than inventing it. So a sparse brief produces a
WORSE result, not a broken agent. This checklist exists to raise loop quality, not to
make the agent functional.

The agent owns NONE of this knowledge. It does not know which workflow it is in, which
other agents exist, or what "done" means this run — and it should not. All of that lives
HERE, on the caller side, where workflow knowledge belongs. Do not push any of it into the
persona (that is the coupling this template is the antidote to).

This is the call-time companion to `workflow-contract.md`:
- `workflow-contract` proves the LOOP is sound (done-condition, independence, bounds, gate).
- `agent-dispatch` is how the caller briefs each CALL inside that loop well.

HOW TO USE:
- This is a checklist, not a file you commit. Fold the fields below into the prompt string
  of each Task() call in a `commands/*.md` supervisor.
- Independence (loop-discipline req. 2) is STRUCTURAL, and it is the one thing the caller
  MUST get right: guarantee it by dispatching a DIFFERENT agent (running a DIFFERENT skill)
  to grade than to produce, and by never asking one agent to grade its own output. The
  persona asserts nothing about this — the structure carries it.
-->

# Agent Dispatch Briefing — [AGENT] for [WORKFLOW], [PHASE]

A good brief carries the context below. None of it is a precondition for the agent to
*function* — it is how the caller gets the agent's *best* work and keeps the loop sound.
Name the skill as a hint, not a command: the agent decides whether it fits.

| # | Field | What to provide | Example |
|---|-------|----------------|---------|
| 1 | **Skill(s) to lean on** | The model-invoked skill(s) likely to fit this work, by name — a hint the agent can take or set aside | "This is in `mochiko:validation-constitution`'s domain." |
| 2 | **Role this run** | What the agent is doing in plain terms — author / grade / reconcile — so it frames its output | "You are grading this artifact independently." |
| 3 | **Input(s) to read** | Every artifact the agent should Read before acting (and any in-session inputs) | "Read `.mochiko/memory/constitution.md` and `…/codebase-analysis.md`." |
| 4 | **Where the output goes** | The path to write to, or "return only, write nothing" | "Write to `.mochiko/memory/constitution.md`." |
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

**Briefing version:** v2 · **Governed by:** `loop-discipline` · **Pairs with:** `workflow-contract.md`
