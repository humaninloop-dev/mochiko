---
description: Think a problem through with the user and harden the record at the end — the session is just the lead and the user (plus an optional grounder verifying facts against the codebase); at convergence a cold review pair reads the record independently, cross-examines each other to kill weak findings, and returns only the survivors for the user's rulings. Deliverable is one decision record; pipeline entry is an offer, never a default. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one artifact behind — `.mochiko/brainstorms/<slug>/record.md`, a decision record hardened by an end-stage adversarial review. Empty topic (the known `@`-reference drop) → ask what we are thinking through.

**You are the lead.** You run the questioning inline via `mochiko:analysis-iterative` (one question per turn, format adapted to the user's state) and write the record as you go. The machinery enters exactly twice: grounder fact-checks when a claim needs verifying, and the review pair at convergence. Between those, the conversation is you and the user — no standing challenger. (A first-run ruling: the v2 standing episodic advocate generated 3:1 machine-to-user traffic and folded amendments into user-ruled decisions without consent — log analysis in `.mochiko/brainstorms/brainstorm-v2-revision/record.md`.) This is a `mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill — **no per-run contract is written**.

## Hard requirement — agent teams

Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the environment before anything else; unset → stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178). The env check is a proxy — the **first teammate spawn is the authoritative probe** (the grounder at start when its seat is filled, otherwise the review pair at convergence): if it fails, stop with the same instructions. Never proceed teamless — **no fallback transport** (a deliberate dogfood-pilot choice, marked `Contested` in the v2 design record; revisit when mochiko distributes beyond the author's machines).

## Session constraints

- The conversation is the production surface, and it belongs to you and the user. **Never narrate machinery** — no "phase", "round", or "gate" talk; teammate housekeeping (idle notifications, acks) is never narrated and never replied to.
- Write the record **as you go**: every decision with statement + rationale + confidence mark — `Confident / Assumed / Contested / Unsure / Deferred` — one decision namespace (D1…), user corrections and reversals logged where they happen. The record is the review surface, the audit trail, and the deliverable: it must read standalone, and review findings with their dispositions live in a closing Review section, never interleaved with the decisions.
- **Your pen covers only your own formulation.** Nothing amends a user-ruled decision and no new decision is created without the user's word.
- Scope at the start: derive the kebab-case `<slug>`; decide whether the topic has a **reality surface** (existing code / docs / a system under redesign) — that decides the grounder seat. Read `.mochiko/memory/constitution.md` if present and carry it as governing context — never a blocking gate.
- Tell the user at the start that they can watch or message any teammate directly.

## The team

Teammates do **not** load `skills:` frontmatter — every spawn prompt must name the skill and role itself, plus the topic and what to Read (see `templates/agent-dispatch.md` for the briefing fields).

- **grounder** — conditional seat, filled only when the topic has a reality surface: a neutral empiricist spawned at start with an initial factual-map task; thereafter it speaks only when you send it a fact to check. **Reports what is, never argues what should be**, and volunteers file-grounded facts that cut either way. One-off fact-fetches with no standing-perspective value go to Explore subagents instead. During the review it settles the reviewers' fact disputes — facts are checked, never argued.
- **review pair** — spawned together **only at convergence** (never in the room before that): two reviewers from the `mochiko:devils-advocate` agent type, each briefed to invoke `mochiko:validation-brainstorm` in the **end-stage reviewer role**. **Withhold each reviewer's counterpart from its spawn prompt**: each reads the frozen record cold, forms its own findings independently, and reports findings-formed (count only); only then do you introduce them to each other and open the cross-examination. In the debate each attacks the other's findings and defends its own — **a finding is withdrawn by its owner or it survives**; the counterpart persuades, never vetoes. Each returns its survivors (severity, concrete failure scenario, resolution path, unresolved counterpart objections attached), a tally ("N raised, M survived"), and a recommended status; fallen findings stay retrievable on ask.

## Convergence, the review, and survivor routing

On convergence signals (answers turning confirmatory, no new dimensions — confirm the wrap with the user per the questioning skill), spawn the pair. From spawn until every disposition is logged, the record is **frozen** except for its Review section. Route each survivor by answer-owner:

- Challenge to a **user decision** → the user, directly. That is not a tie-break; it is theirs to answer.
- Challenge to **your reasoning** → argue it with the finding's owner, **max two exchanges (you count)**; unresolved at the cap is a deadlock → **tie-break**: present both positions and your recommendation; the user rules.
- **Fact dispute** → grounder (or an Explore subagent). Facts are checked, never argued.
- Overruled survivor → mark the decision `Contested`; nobody re-raises it.

Record a disposition per survivor — **resolved / user-ruled / recorded-open** — then the pair verifies the folds in one incremental pass, quoting the evidence they landed. **Review + verify is the bound**; a survivor still blocking after the verify pass escalates to the user with both positions — the user is the top validator, and out of bounds is never silently done.

## Done-condition (default FAIL) and acceptance

Done only when: the pair has reviewed the converged record, **every survivor carries a disposition**, the verify pass confirms the folds landed, and the user **accepts the record** — plain blocking text, never a timed prompt. Zero survivors is vacuously clean — the tally is still reported. On acceptance the record stays in place as the deliverable. Then offer, don't push: if the record is honestly the shape of a next stage (e.g. a feature description for `/mochiko:specify`), name that as an option and stop.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** as above; initial state `FAIL` — absence of the pair's review, an undispositioned survivor, or no user acceptance all read as not-done.
- **Producer ↔ validator:** producer = you + the user (the session; `analysis-iterative`). Validator = the **review pair** (`mochiko:devils-advocate` × `mochiko:validation-brainstorm`) — different agents, different skill, structurally never in the room until convergence, and independent of each other until their cold findings are formed.
- **Bounds:** per reviewer — one independent cold read + one cross-examination round + one verify pass; lead↔reviewer argument = **max two exchanges per survivor, lead-counted**, then tie-break; grounder checks are facts, never arguments. The human-attended session is the escalation surface, not a substitute for the caps.
- **Human gate:** survivor rulings on user territory + tie-breaks + final acceptance of the record.

## Recovery

Teams do not survive `/resume`, and a shared account limit can throttle the team and the main session together — escalation then has nowhere to go but pause. Pause posture: update the record's Status line with resume state. To resume: re-read `record.md`, respawn what the stage needs (grounder mid-session; the pair at review — the frozen record makes a cold re-read cheap), and continue from the last decision or the survivor queue.
