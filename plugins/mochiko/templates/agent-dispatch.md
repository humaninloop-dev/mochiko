# Agent Dispatch Briefing — [AGENT] for [WORKFLOW], [STAGE]

A caller-side checklist, not a file you commit: fold the fields below into the prompt of
each dispatch a `commands/*.md` supervisor makes. None of it is a precondition for the agent
to *function* — the agent degrades gracefully on a thin brief — it is how the caller gets the
agent's *best* work and keeps the loop sound. The agent owns none of this knowledge
(workflow, siblings, "done") — all of that lives on the caller side; never push it into a
persona. Name the skill as a hint, not a command: the agent decides whether it fits.

| # | Field | What to provide | Example |
|---|-------|----------------|---------|
| 1 | **Skill(s) to lean on** | The model-invoked skill(s) likely to fit this work, by name — a hint the agent can take or set aside | "This is in `mochiko:validation-constitution`'s domain." |
| 2 | **Role this run** | What the agent is doing in plain terms — author / grade / reconcile — so it frames its output | "You are grading this artifact independently." |
| 3 | **Input(s) to read** | Every artifact the agent should Read before acting (and any in-session inputs) — incl. the governance obligated-read line naming relevant `.claude/rules/mochiko/` files | "Read `.mochiko/memory/governance-intent.md` and `…/codebase-analysis.md`; read `.claude/rules/mochiko/api.md` before authoring." |
| 4 | **Where the output goes** | The path to write to, or "return only, write nothing" | "Write the governance region into `CLAUDE.md` (between the mochiko:governance markers only)." |
| 5 | **What good looks like** | The bar the output must clear this run | "Every principle has Statement/Enforcement/Testability/Rationale; no placeholders." |
| 6 | **Prior feedback (retries)** | On round > 1, by the route the feedback actually took: where the verifying seat's gap list already reached this producer **peer-routed**, point at it and open the round — the dispatch is what releases field 9's hold; where no peer edge carried it — a one-shot regrade, or a successor that was not in the room for the hand-off — paste the prior issues verbatim, as before. Else omit | Peer-routed: "Open the round on `verifier`'s gap list — items 2 and 4." · Relayed: "Address: <validator's issues-requiring-fix>." |
| 7 | **Independence framing** | The plain-language reminder that matches the structural guarantee | Author: "Don't grade your own output." · Grader: "Read the artifact itself; default FAIL; quote your evidence." |
| 8 | **Return vs. write** | What to return in the reply vs. what to persist to a file | "WRITE the artifact; RETURN a short report + any clarifications you need." |
| 9 | **Peer edges + hand-off holds** | Team-form seat briefs only — omitted for a one-shot: this seat's peer edges from the command's roster, and the hold that rides a peer-routed hand-off, both per `command-shape.md` **Layer 2** (in-loop mesh); named here, never restated | "Your peer edge is `verifier` — hand each finished cycle straight to it; what comes back is a hand-off, so hold until I open the round." |

Fill what raises quality; trust the professional with the rest.

---

## What the caller MUST get right (the one hard line)

Everything above is quality. **Independence is structure, and it is not optional:**

- [ ] The agent dispatched to **grade** is a *different* agent, running a *different* skill,
      than the one dispatched to **produce**.
- [ ] No agent is ever asked to grade its own output.

This is the caller's job, carried by *who it dispatches* — never by a line in the persona.
A loop that violates it is unsound (see `loop-discipline` req. 2), no matter how well the
other fields are filled. In a command, the same guarantee is visible in the artifact rather
than trusted at call time: the shape's **Seats & checks** table is where no row grades its
own output. This checklist is the per-call restatement of that structure, at the moment of
the call.

> A thin brief is a quality cost, recoverable by the agent. A collapsed
> producer↔validator boundary is an unsound loop, recoverable by no one downstream.

---

**Seat transport and per-seat context lifecycle** (spawning a named teammate, the `name:`
discriminator, the first-spawn probe, and the recycle cadence with its respawn-as-reset
briefing) live in `templates/command-shape.md` **Layer 2** — command-layer-only mechanics,
homed with the team transport they belong to. This file is form-agnostic: it briefs a call,
whether that call fills a seat, **refills one with a versioned-name successor**, or fires a
one-shot subagent — a refill is an ordinary dispatch to brief, never the transport anti-pattern.

**Briefing version:** v7 (2026-07-31) · **Governed by:** `loop-discipline` · **Pairs with:**
`command-shape.md` (the command pattern, seat transport + per-seat context lifecycle) ·
`workflow-contract.md`
