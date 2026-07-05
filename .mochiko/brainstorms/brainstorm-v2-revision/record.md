# Brainstorm Record — revision of `/mochiko:brainstorm` v2: the end-stage review pair

> **Topic:** revise brainstorm v2's agent-team engagement model after its first run. Opened as "compare the three brainstorm executions and modify the rewrite"; the user narrowed it to the v2 run's team dynamics after experiencing the session as *"a continuous loop without my input."*
> **Session:** 2026-07-05 · lead + user, running bare `analysis-iterative` (no live team) · method: **transcript forensics** — the run was judged from its lead + teammate `.jsonl` transcripts, not from its own record/handover (which were lead-authored and scored the routing check ✓ from the lead's seat)
> **Status:** **Accepted and built at v0.5.0.** Decisions user-ruled live, including two deliberate supersessions (the healthy kind — three rulings converging in one direction). This record is itself un-reviewed (the session ran bare); a cold review can be requested.

---

## Context for a cold reviewer

**What existed (v2, plugin v0.4.0):** `/mochiko:brainstorm` as the agent-team pilot — lead runs `analysis-iterative` inline and writes `record.md` as-you-go; a standing **advocate** teammate engaged episodically as decisions crystallize; a conditional **grounder**; a cold **pressure-tester** at convergence auditing the record *and* the advocate. Design record: `.mochiko/brainstorms/brainstorm-command-rewrite/record.md` (D1–D9; D9 = hard-require teams, `Contested`).

**The first v2 run** (2026-07-04, topic "setup workflow rewrite" — deliberately v1's topic, so differences are transport): reached D11 of its record, then died at the **shared account session limit** pre-convergence. Its handover (`.mochiko/brainstorms/setup-workflow-rewrite/dogfood-handover.md`) scored the substance well and left two checks open (routing-from-the-user's-seat; cost). The user's verdict on the first check arrived as this session's opening complaint: **FAIL**.

## Findings (from the transcripts — lead session `d822aa0a…jsonl` + `subagents/agent-aadvocate…jsonl` + `agent-agrounder…jsonl`)

- **F1 · Traffic ratio 3:1.** 8 user inputs vs 14 lead→advocate messages + ~10 advocate→lead replies in a 42-minute run. The advocate was never file-triggered (no such mechanism exists) — every wake was a lead message; the lead messaged it after essentially every record edit batch.
- **F2 · Half the machine traffic was bookkeeping.** Of the 14 lead→advocate messages: 7 engagements, 7 round-closes/holds. Agent teams have no FYI channel — **every message costs a full teammate turn** (context reload, usually a full record re-read, a reply). Ack turns, standby announcements, and an unrequested fold re-verification all trace to bookkeeping wakes.
- **F3 · The advocate ran permanently behind.** Decisions were logged faster than it could process: 3 message crossings, each costing a reconciliation turn plus a "hold" message — the felt "continuous loop."
- **F4 · Consent was theater at the edges.** ~14 autonomous decision-shaping moves vs 6 user rulings; the lead folded an amendment into a user-ratified decision (D6) and created a whole new decision (D9, `Assumed`) with consent handled as "objection windows" the user never actually answered (both still queued at the pause).
- **F5 · Cost (closes the open I-4 check): the standing team was NOT cheaper than v1.** Measured: advocate 118k out / 436k in / 2.70M cache-write / 11.2M cache-read; grounder 25k / 195k / 1.29M / 2.5M; lead 502k / 90k / 1.78M / 23.7M. v1's entire adversarial bill was ≈620k across four dispatches and finished in one window; the v2 run died at the shared limit with its tail unrun. Cost is fully reconstructable from transcripts post-hoc — no in-run instrumentation needed.
- **F6 · The teams-defining capability went unused.** Teammate↔teammate messages across the whole run: **zero**. The standing model exercised nothing that `Task`-subagent dispatch couldn't do.
- **F7 · What worked (do not lose):** 7/7 engagements yielded folds that survived to the final record — including catches on routine-looking ratified decisions (the "best catch of the session," maturity≠stakes → its D9, came from one). The standing grounder steered decisions *before* they formed (closed the I-6 check ✓✓). Record-as-you-go made the mid-run death a cheap pause instead of a catastrophe.

## Decisions

### D1 — Judge the run from its transcripts, not its self-reports
The run's record and handover were lead-authored; the user's-seat verdict and the F1–F6 mechanics were invisible to both. Forensic analysis of the session + teammate `.jsonl` transcripts is the method — and it settled the cost question (F5) without any instrumentation machinery.
- **Mark:** Confident (method demonstrated in this session).

### D2 — Diagnosis: the standing episodic advocate failed the human it serves
The user's complaint decomposes onto F1–F4: volume (bookkeeping wakes, pipelining) and consent (autonomous folds into user territory). The adversarial *content* was good (F7); the *transport and rhythm* were the defect — the same substance/ceremony split that killed v1, one level down.
- **Mark:** Confident (user-experienced + log-verified).

### D3 — Engagement model: end-stage review pair *(final, after two deliberate supersessions)*
**Ruling trail:** (i) *challenge-everything turn-discipline* (one turn per engagement, engagement-only messages, dispatch at hand-to-user, doctrine-skips) — ruled, then superseded; (ii) *on-demand triggering, lead suggests the moment* — ruled on the complexity verdict ("becoming a bit complex to understand"), then superseded; (iii) **end-stage review pair** — ruled final ("okay, go with end stage model"). Three moves in one direction: a revealed preference for end-weighted simplicity, ranked above early catches.
**The shape:** the session is the lead + the user (+ conditional grounder). At convergence, spawn **two cold reviewers** together (same `devils-advocate` type × `validation-brainstorm`, end-stage reviewer role); each's counterpart is **withheld from its spawn prompt** until its independent findings are formed (sequestration generalized); then one **cross-examination** round — each attacks the other's findings, defends its own, **owner-withdrawal only** (persuade, never veto; fact disputes → grounder); unresolved disagreements survive with the objection attached. Survivors return with a **tally** ("N raised, M survived"; fallen retrievable on ask). The old pressure-tester's advocate-coverage/losses audit dissolves — the pair audits each other by construction.
- **Eyes-open cost (recorded as the trade):** before-positions-harden catches die as a class — v2's 7/7 engagement yield (F7) now arrives at review, after decisions have built on each other; end-stage findings can unwind chains. v1 proves end-stage review still catches these things, settled rather than fluid.
- **For it:** the prime directive (a session the human can follow — the verdict that killed v1's ceremony applies to any fix); F1–F4 become structurally impossible (no mid-session machine traffic exists); F6 reverses — the debate is the first design that actually exercises inter-teammate messaging; survivor filtering directly answers the ~20-finding stream the user sat through.
- **Mark:** Confident (user-ruled; mechanism un-exercised until the next run).
- **Rejected:** turn-disciplined challenge-everything (ruling i — replay showed 7/7 yield and formulation-risk arguments, heard and outranked); risk-graded selection (the replay showed the best catches came from decisions a rubric would deprioritize, and it puts the producer in charge of its own review coverage); on-demand triggering (ruling ii — simpler, but keeps the standing seat + suggestion protocol for a capability that would rarely fire); bare cold-end-only with one reviewer (v1's shape — loses the debate filter and the survivors-only interface).

### D4 — Consent rule: the lead's pen covers only its own formulation
Nothing amends a user-ruled decision and no new decision is created without the user's word. (Ruled as territory-gating while the standing advocate still existed; survives the end-stage model structurally — survivors on user territory route to the user, and no mid-session folding surface exists.) The objection-window pattern dies.
- **Mark:** Confident (user-ruled: "i will go with your recommendation" on the territory-gated option).

### D5 — Ride-alongs kept, drops confirmed
**Kept:** pause posture + respawn-from-record (the shared-limit death F5 named in Recovery; trails are the memory); idle-notification silence (teammate housekeeping never narrated or replied to); the grounder exactly as it was (conditional seat, initial map, fact-checks — F7/I-6 closed it ✓✓; it additionally arbitrates the reviewers' fact disputes); ratification-streak check → `analysis-iterative` (third session validating it — this session's lead flagged its own streak once). **Dropped:** in-run cost instrumentation (F5's method supersedes it); record-version stamps + crossing protocol (no mid-session messages exist to cross; the record is frozen at review); any redesign of the disposition/verify/acceptance tail beyond transplanting it (still zero executions — it is now the core surface, not the periphery).
- **Mark:** Confident (user arbitrated the sort in ledger form before the D3 supersessions; re-checked against the final shape).

## Open threads

- **The entire end-stage mechanism is un-exercised.** Named checks live in `BACKLOG.md` (Brainstorm-v2 follow-ups → v2.1 item): filter kills weak-not-true findings; independence-before-contact holds; survivor volume ≪ the v2 run's stream; the tail finally executes; cost re-measured against both baselines.
- **First run planned (user's call, at commit time):** a **fresh re-run** of the setup-workflow-rewrite topic after this branch merges — not a resume of the paused v2 run. Its artifacts stay uncommitted and gitignored; before the fresh run, clear or re-slug `.mochiko/brainstorms/setup-workflow-rewrite/`, or v2.1's Recovery will read the old `record.md` and resume it. The old run's queued rulings (starter-catalog scope; its D9 objection) get re-derived by the fresh session.
- **Flag-off / distribution revisit:** unchanged (`Contested` D9 of the v2 design record); the end-stage model *keeps* the hard teams requirement — the debate needs teammate↔teammate messaging.
- **The middle option stays on file:** if the v2.1 dogfood shows expensive end-stage unwinds (survivors invalidating settled chains), the fallback is one-shot advocate dispatches at user-authorized moments — early catches without a standing seat.
- **This record is un-reviewed** (the session ran bare `analysis-iterative` + transcript forensics; no cold review pass).

## Build plan — executed (v0.5.0, 2026-07-05)

1. ✅ `plugins/mochiko/commands/brainstorm.md` rewritten (~55 lines): lead+user session · grounder unchanged · review pair (cold spawn, counterpart withheld, cross-exam, owner-withdrawal, survivors + tally) · survivor routing + dispositions + verify + plain-text acceptance · consent rule · idle-silence · pause/respawn recovery · authoring-time contract updated.
2. ✅ `skills/validation-brainstorm/SKILL.md` rewritten: one **end-stage reviewer** role ×2 (three phases: independent cold read → cross-examination → survivor report; verify pass kept); v2's live-advocate + pressure-tester roles retired. `references/RECORD-FITNESS.md` reworded for the pair (decision-trail item generalized; "pressure-tested" → "reviewed").
3. ✅ `skills/analysis-iterative/SKILL.md`: ratification-streak paragraph added under Reading Confidence Signals.
4. ✅ Docs: BACKLOG (v2 dogfood item closed with F1–F7 outcomes + measured cost; v2.1 dogfood item opened; substrate item datapointed), REGISTRY (brainstorm / devils-advocate / validation-brainstorm rows), ROADMAP (v2 row annotated; v2.1 Key Decision row + Decision Trail section), router `mochiko` skill (skill / entry-point / agents rows), `agents/devils-advocate.md` skill annotation.
5. ✅ Version bump 0.4.0 → 0.5.0 (`plugin.json` + `marketplace.json`).

## Provenance

Session 2026-07-05, lead + user via `mochiko:analysis-iterative` (bare — no team; the team design was the subject). Inputs: the v2 run's transcripts (`~/.claude/projects/...-mochiko/d822aa0a-…jsonl` + its `subagents/` pair + team config), both dogfood handovers (v1 branch + v2 working tree), the v2 design record, the v2 command/skill sources, `BACKLOG.md`. Decisions ruled live by the user, including one mid-session complexity correction and two deliberate supersessions of fresh rulings. Built and shipped at plugin v0.5.0, 2026-07-05. No cold review run on this record.
