---
description: Think a problem through with a live adversarial agent team — a devil's advocate challenges decisions as they form, an optional grounder verifies claims against the codebase, and a cold pressure-tester audits the finished record (and the advocate) before acceptance. The human tie-breaks deadlocks and accepts the record. Deliverable is one decision record; pipeline entry is an offer, never a default. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Brainstorm — Live Adversarial Thinking

**Goal:** think `$ARGUMENTS` through with the user and leave one artifact behind — `.mochiko/brainstorms/<slug>/record.md`, a decision record that survived adversarial pressure while the thinking was still fluid and cold review after it settled. Empty topic (the known `@`-reference drop) → ask what we are thinking through.

**You are the lead.** You run the questioning inline via `mochiko:analysis-iterative` (one question per turn, format adapted to the user's state), write the record as you go, route every challenge, own every verdict, and bring the user in as tie-breaker. Teammates pressure-test; they never author. This is a `mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill — **no per-run contract is written**.

## Hard requirement — agent teams

Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the environment before anything else; unset → stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178). The env check is a proxy — the **first advocate spawn is the authoritative probe**: if it fails, stop with the same instructions. Never proceed teamless — **no fallback transport**; the live team *is* this command (a deliberate dogfood-pilot choice, marked `Contested` in the design record; revisit when mochiko distributes beyond the author's machines).

## Session constraints

- The conversation is the production surface. **Never narrate machinery at the user** — no "phase", "round", or "gate" talk; adversarial beats surface as conversation, not process.
- Write the record **as you go**: every decision with its rationale, its challenge(s) and resolution, and a confidence mark — `Confident / Assumed / Contested / Unsure / Deferred`. One decision namespace (D1…); no other ID schemes. The record is the grading surface, the audit trail, and the deliverable — it must read standalone, and it must stay legible: decision entries carry statement + rationale + mark with the challenge trail as compact sub-lines; pressure-test findings and dispositions go in a closing Review section, never interleaved with the decisions.
- Scope at the start: derive the kebab-case `<slug>`; decide whether the topic has a **reality surface** (existing code / docs / a system under redesign) — that decides the grounder seat. Read `.mochiko/memory/constitution.md` if present and carry it as governing context — never a blocking gate.
- Tell the user at the start that they can watch or message any teammate directly.

## The team

Teammates do **not** load `skills:` frontmatter — every spawn prompt must name the skill and role itself, plus the topic and what to Read (see `templates/agent-dispatch.md` for the briefing fields).

- **advocate** — spawn at start from the `mochiko:devils-advocate` agent type; brief: invoke `mochiko:validation-brainstorm` in the **live-advocate role**. Engaged episodically: when a decision crystallizes, log it in the record first, then point the advocate at the entry — **it attacks the recorded entry, not your relay** (your summary is a notification, not the artifact) — and may Read the codebase to sharpen an attack. Confirm it is alive before each engagement; a dead teammate is an escalation (respawn from the record), never a silent skip. Never re-raises a `Contested` decision.
- **grounder** — conditional seat, filled only when the topic has a reality surface: a neutral empiricist following the relayed decisions with the code open. Verifies load-bearing claims, volunteers what nobody asked, may undercut the advocate as readily as the room. Role constraint in the spawn prompt: **reports what is, never argues what should be.** One-off fact-fetches with no standing-perspective value go to Explore subagents instead.
- **pressure-tester** — spawned **only at convergence** (never in the room before that) from the `mochiko:devils-advocate` agent type; brief: invoke `mochiko:validation-brainstorm` in the **pressure-test role**. Reads the record **cold first**, forms its own attack, *then* may cross-examine the advocate directly — **withhold the advocate's name from its spawn prompt and hand it over only after the cold findings land** (sequestration made structural, not just instructed). Three targets: the record's content (scenario stress + reality-grounding against actual files), the advocate's coverage (what sailed through unchallenged, soft challenges, dodged resolutions, agreement drift), and the advocate's losses (an overruled challenge steelmanned once — the user's ruling stays final).

## Routing — who answers what

- Challenge to a **user decision** → the user, directly. That is not a tie-break; it is theirs to answer.
- Challenge to **your reasoning** → argue it out with the advocate, **max two exchanges (you count)**. Unresolved after the cap is a deadlock → **tie-break**: present both positions and your recommendation; the user rules. Each episodic engagement is one bounded challenge — there is no open-ended sub-loop.
- **Fact dispute** → grounder (or an Explore subagent). Facts are checked, never argued.
- Overruled challenge → mark the decision `Contested`; nobody re-raises it.

## Convergence and the cold review

On convergence signals (answers turning confirmatory, no new dimensions), spawn the pressure-tester. From that spawn until its findings land, the record is **frozen** except for logging dispositions — a cold reviewer cannot grade a mutating file. Route each of its findings by kind — fact → grounder/Explore · preference → user · reasoning → argue or fold — and record a disposition per finding: **resolved / user-ruled / recorded-open**. The pressure-tester then verifies the folds in one incremental pass. **Review + verify is the bound**; a finding still blocking after the verify pass escalates to the user with both positions — the user is the top validator, and out of bounds is never silently done.

## Done-condition (default FAIL) and acceptance

Done only when: the pressure-tester has reviewed the converged record, **every finding carries a disposition**, the verify pass confirms the folds landed, and the user **accepts the record** — plain blocking text, never a timed prompt. On acceptance the record stays in place as the deliverable. Then offer, don't push: if the record is honestly the shape of a next stage (e.g. a feature description for `/mochiko:specify`), name that as an option and stop.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** as above; initial state `FAIL` — absence of a pressure-test review, an undispositioned finding, or no user acceptance all read as not-done.
- **Producer ↔ validator:** producer = you + the user (the session; `analysis-iterative`). Validator = the **pressure-tester** (`mochiko:devils-advocate` × `mochiko:validation-brainstorm`) — different agent, different skill, and structurally never in the room until convergence. The advocate is *additional pressure during production*, not the validator.
- **Bounds:** pressure-test = one review + one verify pass; advocate arguments = **max two exchanges per challenge, lead-counted**, then tie-break; each episodic engagement is a single bounded challenge, so no open-ended sub-loop exists. The human-attended session is the escalation surface, not a substitute for the caps.
- **Human gate:** tie-breaks (the preference-gap placement) + final acceptance of the record.

## Recovery

Teams do not survive `/resume`. To resume a session: re-read `record.md`, re-spawn the team, continue from the last decision.
