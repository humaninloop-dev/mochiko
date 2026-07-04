# Brainstorm Record — rewrite of `/mochiko:brainstorm` as an agent-team framework

> **Topic:** comprehensive rewrite of `/mochiko:brainstorm` — from a phase/gate dispatch pipeline into a minimal framework (goal · session constraints · team spec · routing · one gate · one artifact) built on native Claude Code agent teams. Pilot for a team-based pattern other mochiko workflows adopt if dogfooding succeeds.
> **Session:** 2026-07-04 · lead + user, running bare `analysis-iterative` (no live team — the team design is this session's subject) · 6 structured questions, 1 requested contrast (cast), 1 fate-ledger slow-down check
> **Status:** **Accepted for build; built at v0.4.0.** Cold review (one `mochiko:devils-advocate` pass): 1 Critical, 8 Important, 3 Minor — all dispositioned (see Review trail); verify pass run on the folds. Facts verified clean (every load-bearing claim checked against sources; one off-by-one).

---

## Context for a cold reviewer

**What exists (v1):** `plugins/mochiko/commands/brainstorm.md` (88 lines) — 7 phases, 5 named gates (G1–G5), 2 adversarial dispatch loops with round counters, a per-run `contract.md` filled before questioning starts, kill-switch file, 11-row state-recovery table, 4 destination ramps with promotion mechanics and a staff↔qa mini-loop. Reviewer machinery lives in `plugins/mochiko/skills/validation-brainstorm/` (two branches: digest challenge, pressure test; per-destination readiness checklists in `references/`).

**Dogfood evidence** (branch `brainstorm-dogfood-setup-rewrite`, `.mochiko/brainstorms/setup-workflow-rewrite/dogfood-handover.md`) cuts both ways:

- *Adversarial content earned its keep:* round-1 challenge caught two structural defects the room couldn't see; pressure round 2 caught a contradiction introduced by a round-1 fix; reality-grounding verified 13 factual claims against real files (0 refuted, 2 sharpened decisions).
- *Ceremony strained everywhere:* the done-condition was unreachable as written (a competent advocate always finds something, so clearing happened twice via the "escalation" failure path); the run generated five ID namespaces (D/G/R in the handover; C/F in the session's `digest.md`); four cold dispatches cost ≈620k subagent tokens (each re-read everything); the first-ever destination answer escaped the enum ("just save the .md"); a mid-loop `AskUserQuestion` gate timed out; the handover recommends 10 patches after one run.
- *The human who sat through it could not follow the flow* — phase narration made a thinking session read like a process being administered. The rewrite's prime directive: strip everything not absolutely necessary.

**Agent teams** (per https://code.claude.com/docs/en/agent-teams, v2.1.178+): experimental, disabled by default (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); lead + teammates with shared task list and direct teammate↔teammate messaging; the user can interact with any teammate directly; teammates are full sessions that do **not** inherit the lead's conversation history; teammates can spawn from subagent definitions but `skills:`/`mcpServers:` frontmatter is **not** applied (spawn prompt must name skills); `/resume` does not restore in-process teammates; one team per session; no nested teams; teammates may stop on errors; token usage scales with active teammates. Doc-blessed patterns: "one playing devil's advocate"; "have them talk to each other to try to disprove each other's theories."

**Doctrine:** `mochiko:loop-discipline` binds every workflow to four requirements (default-FAIL done-condition · independent validator · bounded iteration · named human gate). Its human-gate menu already includes "preference-gap only — the loop converges autonomously and only escalates genuine judgment calls," and its scope line already names "agent-team workflow[s]."

**User's standing preferences** (confirmed by dogfood): close with a standalone .md record; pipeline entry as offer, not default. Depth over speed on design sessions; recommendations with steelmans over open questions.

> **Marks note** *(added post-review, M-2):* confidence marks below reflect the **session's reasoning confidence**, not empirical validation — the D2/D3/D5 mechanisms are un-exercised until the first flagged run. The dogfood checks in `BACKLOG.md` name what the first run must demonstrate.

---

## Decisions

### D1 — Diagnosis: keep the adversarial substance, kill the ceremony *(amended post-review, I-1)*
The value shown in dogfood came from adversarial *perspective* (challenges, steelmans, reality-grounding); the illegibility came from the *ceremony* around it (freeze → dispatch → report → fold → ratify, phase narration, ID namespaces, per-run contract). The rewrite preserves the former and deletes the latter.
- **Honest framing (amendment):** the dogfooded pains are **transport-independent** — the handover's own R1–R10 patch the subagent transport and require no teams. The agent-team migration is the **user's deliberate strategic bet** (stated in the session's opening brief, before the dogfood evidence was read): pilot the team pattern on brainstorm and, if it dogfoods well, generalize it. The dogfood motivates the *strip*; the *teams* half is a bet whose unique payoff (live challenge before positions harden) is un-validated until the first flagged run.
- **Confidence:** Confident (user-experienced; handover evidence supports both halves of the diagnosis).
- **Rejected:** presentation-only fix (better narration over the same machinery); R1–R10 applied to the dispatch transport (fixes the measured pains but tests nothing the user set out to test — kept as the honest fallback if the pilot fails).

### D2 — Engagement model: hybrid — episodic DA during, one cold pressure-test at the end *(amended post-review, I-3/I-4/I-8)*
The devil's advocate is a standing teammate engaged **episodically at decision moments**. The lead logs the crystallized decision in the record first, then points the advocate at the entry — **the advocate attacks the recorded entry, not the lead's relay** (the relay is a notification; the record is the artifact — keeping the file-grounding doctrine intact and containing the lead-as-relay-gatekeeper filter, which the pressure-tester's coverage audit backstops). At the end, **one pressure-tester with cold context** (spawned late; never in the room) attacks the finished record.
- Challenge trail: *lead-as-filter* (I-3) → resolved: DA grades the record entry, PT audits coverage. · *DA silent death between pings* (I-8) → resolved: lead confirms liveness before each engagement; a dead DA is an escalation (respawn from the record), never a silent skip.
- **Cost claim (amended):** "cheaper than v1's ≈620k" is **`Assumed`** — marginal pings are incremental, but 2–3 standing teammate contexts are not free; the first flagged run measures total cost against the 620k baseline (BACKLOG check).
- **Confidence:** Confident in reasoning (chosen from a four-option contrast: live-shadow / episodic / cold-end-only / hybrid); mechanism un-exercised.
- **Rejected:** live shadow debate (cost + crosstalk + adversary marinates); cold review only (challenges arrive after positions harden); v1's freeze-and-dispatch at both points.

### D3 — The pressure test audits the devil's advocate too (user-initiated: "i want that") *(amended post-review, I-5)*
Three target classes: **(1)** the record's content — scenario stress + reality-grounding of load-bearing factual claims; **(2)** adversarial coverage — decisions that sailed through unchallenged, soft/generic challenges, resolutions that dodged rather than answered, agreement drift; **(3)** the DA's losses — overruled challenges re-examined once; the user's ruling remains final (no-nagging preserved). Protocol: the PT reads the record **cold first**, forms its own attack, **then** may cross-examine the advocate directly.
- **Sequestration hardened (amendment):** cold-read-first is prompt-level on an LLM teammate — so the lead **withholds the advocate's name from the PT's spawn prompt** and hands it over only after the cold findings land. Structural where the platform allows; the residual (a PT reading team config to find the advocate) is accepted and disclosed — the same trust level v1's file-only briefs carried.
- **Confidence:** Confident in reasoning (user-initiated); mechanism un-exercised.
- **Rejected:** PT limited to record content (leaves the watchdog unaudited); PT free to interrogate the DA before forming its own view.

### D4 — One running record with an inline challenge trail; freeze ceremony and report artifacts die *(amended post-review, I-7/M-1)*
The session writes **one artifact as it goes** — `record.md`: each decision with rationale, its challenge(s), resolution, and a confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`). No digest freeze, no report files — DA and PT findings arrive as messages and their dispositions are logged in the record. One decision-ID namespace only.
- **Legibility structure (amendment, I-7):** the three jobs (grading surface, audit trail, deliverable) share one file but not one texture — decision entries stay legible (statement + rationale + mark, challenge trail as compact sub-lines); PT findings + dispositions live in a closing **Review trail** section, never interleaved. This file demonstrates the shape.
- **Freeze-at-PT-time (amendment, M-1):** "no freeze ceremony" ≠ "the PT grades a mutating file" — from PT spawn until its findings land, the record is frozen except for logging dispositions.
- **Confidence:** Confident (entailment of D3; the convention itself is dogfood-validated).

### D5 — Cast: two standing roles + one conditional seat *(amended post-review, I-6)*
**DA** (standing, episodic) · **cold PT** (standing, spawned late) · **grounder** (conditional seat): filled when the topic has a **reality surface** (existing code/docs/system being redesigned) — decided by the lead when scoping the session. The grounder is a *neutral* standing perspective — verifies claims, volunteers what nobody asked, may undercut the DA as readily as the room. Role constraint: reports what **is**, never argues what **should be**. Pure fact-fetches route to one-shot Explore subagents.
- **Evidence honesty (amendment):** the two dogfood grounding nuances were volunteered by the **end-stage cold reviewer**, not by any live grounder — so they prove grounding's value, not the standing seat's. The standing-seat case is *theoretical* (catch false premises at decision time; give tie-breaks a neutral fact base instead of competing assertions) and the seat is deliberately **conditional** — the first flagged run tests whether it catches anything the PT's reality-grounding lens wouldn't (BACKLOG check). End-only grounding remains the fallback if it doesn't.
- **Confidence:** Confident in reasoning (recommendation-led; adopted after a requested A-vs-B contrast with the trade-offs in view); seat value un-exercised.
- **Rejected:** grounder always present (dead weight on unanchored topics); DA grounds everything itself (motivated empiricism); grounding only at the end (un-falsified — now the explicit fallback rather than a rejected road).

### D6 — Human role: answer-owner routing, tie-break on deadlock, one acceptance gate *(amended post-review, C-1)*
Challenges to the **user's stated decisions** go straight to the user (only they can answer; not tie-breaks). Challenges to the **lead's reasoning** are argued lead↔DA — **max two exchanges, lead-counted** (a deterministic cap, not a felt "positions stopped moving"); unresolved at the cap is a deadlock → tie-break: both positions + the lead's recommendation, user rules. **Fact disputes** go to the grounder, not to argument. The user may watch or summon any teammate directly at any time. The session ends at **one** named human gate: the user accepts the record.
- **Rationale:** `loop-discipline`'s wire-crossing warning — the lead defending guesses about user preferences is a wasted round; the doctrine's "preference-gap only" placement blesses tie-break-shaped escalation.
- **Confidence:** Confident (recommendation-led, adopted; the exchange cap added at review).
- **Rejected:** everything-surfaces (gate fatigue in conversational clothing); strict tie-break-only (lead argues about preferences it can't know); LLM-judged "no progress" as the only argument exit (the review's Critical — an LLM-controlled exit is what the cap must backstop).

### D7 — Doctrine stance: claim sound-loop status via native mapping; contract authored once, never per-run *(amended post-review, C-1/I-2)*
The rewrite **claims sound-loop status** — the four requirements map natively:
- **Done-condition** (default FAIL, *reachable*): every cold-PT finding has a disposition (resolved / user-ruled / recorded-open) **and** the user accepts the record — no "advocate says ready" clause (v1's was unreachable by construction). Vacuously clean on zero findings; clears through dispositions when findings exist.
- **Validator:** the cold PT, never in the room — stronger independence than v1's file-freeze.
- **Bounds (amended — the review's Critical):** every iteration surface carries a **deterministic cap**: PT = one review + one verify pass; lead↔DA argument = max two exchanges per challenge, lead-counted; each episodic DA engagement is a single bounded challenge (no open-ended sub-loop exists by construction). The human-attended session is the **escalation surface**, not a substitute for any cap — requirement 3 is carried by the counts, requirement 4 by the gates; neither borrows from the other.
- **Human gate:** tie-breaks + final acceptance.
- **Contract:** filled **once at authoring time** — embedded as the command's Contract section (the altitude doctrine already requires commands to state their contract parameters; v1's sin was *also* filling a per-run copy whose every value was constant — ritual, not proof).
- **Availability probe (amendment, I-2):** the hard-require check is two-stage — the env var as proxy, then **the first advocate spawn as the authoritative probe**; spawn failure → stop with enable instructions (settings/env; Claude Code ≥ v2.1.178). Never proceed teamless. Residual risk (no platform-blessed "is-teams-enabled" API) disclosed as an open thread.
- **Confidence:** Confident (recommendation-led, adopted; bounds repaired at review).
- **Rejected:** carve-out ("a conversation is not a production loop" — the pilot must generalize to specify/plan/tasks, which are production loops); per-run contract with lighter fill; human-presence-as-bound (the review's Critical: substitutes requirement 4 for requirement 3, which the doctrine's own text forecloses).

### D8 — Strip ledger confirmed in full (user-checked after a flagged acceptance streak)
Dies: 7 phases + narration · 5 gates · per-run contract · freeze ceremony · report artifacts · round-counter narration · kill-switch file · state-recovery table (teams don't survive `/resume`; recovery = re-read record, re-spawn) · 4 destination ramps + promotion + mini-loop (record is the default deliverable; **pipeline entry becomes an offer at acceptance**, no machinery) · 4 destination checklists (replaced by one **standalone-record fitness** check) · batched-`AskUserQuestion` gate pattern (acceptance is plain blocking text). Survives: challenge trail + confidence vocabulary · running record in `.mochiko/brainstorms/<slug>/` · cold independent review · verify pass after folds · Contested/no-re-raise (softened per D3) · reachable done-condition + acceptance gate · constitution as non-blocking context. Retired with the ramps: the `/mochiko:tasks` stamped design-light entry variant + `handoff-brief-template.md` (v1's cross-command edit — removed rather than left as dead code). Transforms: `validation-brainstorm` → two live teammate roles (spawn prompts name skill + role, since teammates ignore `skills:` frontmatter).
- **Process note:** three consecutive "go with your recommendation" turns triggered the slow-down check (the pattern the setup-rewrite session's own D7 named); the ledger was the check vehicle; user confirmed it holds.
- **Confidence:** Confident (explicitly checked line-by-line).

### D9 — Flag-off posture: hard require; no fallback transport — `Contested`
The command checks for agent-team availability at start (per D7's two-stage probe) and **refuses with enable instructions** when unavailable. No subagent-transport fallback, no degraded mode.
- **Rationale (user's, overriding the recommendation):** this is a dogfood pilot — a fallback means some runs silently test the wrong thing; hard-require guarantees every run exercises the actual hypothesis; a fallback is speculative machinery of exactly the kind being stripped.
- **Confidence:** **Contested** — the lead recommended announced-degrade-to-subagent-transport (distribution readiness, experimental-API churn across five recent point releases); the user overrode with the steelman in full view. Not re-raised (the review honored this; its I-2 touched only the check's mechanism, a new angle).
- **Accepted costs, eyes open:** churn exposure on an experimental surface; a wall for any unflagged machine.
- **Revisit trigger (open thread):** when mochiko ships beyond the author's machines (marketplace path), the flag-off question reopens — decided then with real dogfood data.

---

## Open threads

- **Flag-off revisit** — per D9's trigger; not before distribution.
- **Un-dogfooded surface** — this session designed the team but ran bare; the first flagged run is the real test. Named checks live in `BACKLOG.md` (Brainstorm-v2 follow-ups), including the two the review added: grounder-vs-end-only value (I-6) and total-cost vs the 620k baseline (I-4).
- **Availability probe residual** — no platform-blessed "is-teams-enabled" check exists; env-var-then-spawn-probe is the best available (D7). If the platform ships a real check, adopt it.
- **Generalization caveat (from the review's design probes)** — the done-condition lets a *fresh Critical surfaced on the verify pass* be dispositioned `recorded-open` under an accepting human. Acceptable for a thinking record; **not** acceptable when this mapping generalizes to production loops (specify/plan/tasks) — there, a fresh Critical on verify must block. Carry this into the generalization work.
- **Prompt-level guarantees** — grounder neutrality and DA episodic discipline remain prompt-declared (I-5's residue beyond the PT-name hardening); watch them at dogfood.
- **`analysis-iterative` companion edit** — landed: the 4th question format (Recommend-then-Arbitrate), per dogfood R10; this session ran the pattern live (contrast-then-choose, ledger-then-confirm).

## Build plan — executed (v0.4.0)

1. ✅ `plugins/mochiko/commands/brainstorm.md` rewritten (88 → ~57 lines): goal · hard-require (two-stage probe) · session constraints (as-you-go record, legibility structure, no machinery narration) · team spec (three roles, spawn prompts name skill + role, PT-name withholding, liveness check) · answer-owner routing with the two-exchange cap · convergence + record freeze + disposition protocol · reachable done-condition + plain-text acceptance · embedded authoring-time contract · one-line recovery.
2. ✅ `skills/validation-brainstorm/SKILL.md` rewritten: two live roles (advocate grades the record entry, not the relay; PT cold-read-first with coverage/losses audit + verify pass); `references/RECORD-FITNESS.md` replaces the four destination checklists (deleted).
3. ✅ Ramp retirement: `/mochiko:tasks` stamped entry variant removed (4 edits); `templates/handoff-brief-template.md` deleted.
4. ✅ `analysis-iterative`: Recommend-then-Arbitrate format added.
5. ✅ Docs: ROADMAP (v1 row superseded; v2 Key Decision + Decision Trail entry), REGISTRY (command/skill/template/agent/analysis-iterative rows), BACKLOG (v1 items closed with outcomes; v2 follow-ups opened; substrate item datapointed), router (`mochiko` skill: cluster + entry-point + agents rows), `agents/devils-advocate.md` skill annotation.
6. ✅ Version bump 0.3.0 → 0.4.0 (`plugin.json` + `marketplace.json`).

## Review trail

**Cold review** (independent `mochiko:devils-advocate`, `validation-brainstorm`, digest-challenge shape; grounded — every load-bearing claim verified against sources, all clean except one off-by-one). Recommended status: `critical-gaps` (rubric-driven by the single Critical). **Verify pass** (same reviewer, folds only): all 12 dispositions **verified landed** with quoted evidence; one fold-residue found (the skill's frontmatter description still carried the pre-fold relay model — fixed) and one guard-against-miscorrection honored (the two-exchange cap stays lead-owned in the command, never coupled into the advocate's skill, per the decoupling doctrine). Final recommended status: **ready**. Lead verdict: **cleared**.

| # | Sev | Finding (compressed) | Disposition |
|---|-----|----------------------|-------------|
| C-1 | Critical | "Sound-loop" certification substituted human presence for requirement 3's deterministic cap on the lead↔DA argument loop — the doctrine's own text forecloses this | **Resolved** — two-exchange lead-counted cap per challenge; episodic engagements are single bounded challenges; human presence reframed as escalation surface (D6, D7, command) |
| I-1 | Important | Record framed teams as the dogfood's conclusion; the measured pains are transport-independent (R1–R10 fix them without teams) | **Resolved** — honest framing folded into D1: strip = dogfood-motivated; teams = the user's deliberate, un-validated bet; R1–R10-on-dispatch recorded as the fallback road |
| I-2 | Important | Availability check unspecified; env var is an unreliable proxy (false positives < v2.1.178) | **Resolved** — two-stage probe: env check, then first-spawn as authoritative; residual disclosed (D7, command, open thread) |
| I-3 | Important | Episodic DA graded the lead's relay (a summary), contradicting grades-from-files doctrine; lead = relay-gatekeeper | **Resolved** — DA Reads and attacks the running record entry; relay demoted to notification; PT coverage audit named as the filter backstop (D2, command, skill) |
| I-4 | Important | Cost argument counted marginal pings, ignored standing context cost of 2–3 teammates | **Resolved** — claim downgraded to `Assumed`; cost measurement vs the 620k baseline added to the dogfood checks (D2, BACKLOG) |
| I-5 | Important | Sequestration/neutrality are prompt-declared, not structural | **Resolved (partially structural)** — PT spawn prompt withholds the advocate's name until cold findings land; remaining prompt-level guarantees disclosed as an open thread (D3, command) |
| I-6 | Important | Grounder justified by evidence that actually supports end-only grounding (the dogfood nuances came from the end-stage cold reviewer) | **Resolved** — D5 rationale corrected (evidence proves grounding, not the standing seat); seat stays conditional with the value question added to dogfood checks; end-only grounding named the fallback |
| I-7 | Important | One-record-three-jobs sets audit density against the legibility prime directive | **Resolved** — record structure specified: legible decision entries + compact challenge sub-lines; findings/dispositions in this closing section (D4, command; this file demonstrates it) |
| I-8 | Important | DA can die silently between episodic pings — forfeiting the pilot's one unique benefit unnoticed | **Resolved** — liveness check before each engagement; dead DA = escalation + respawn from the record (D2, command) |
| M-1 | Minor | "Freeze ceremony dies" hid a real freeze-at-PT-time invariant | **Resolved** — invariant stated (D4, command) |
| M-2 | Minor | Confidence marks conflated reasoning-confidence with validation status | **Resolved** — global marks note added; D2/D3/D5 marked "Confident in reasoning; mechanism un-exercised" |
| M-3 | Minor | "Five ID namespaces" only partially cited; "89 lines" is 88 | **Resolved** — C/F cited to `digest.md`; count corrected |

**Review strengths (cited):** all six agent-teams platform claims verified verbatim; the cold-PT design confirmed platform-supported; the done-condition redesign confirmed as a real fix for the dogfood's unreachable-done defect (vacuously clean on zero findings, clears through dispositions otherwise); the record confirmed self-contained with honest `Contested` marking.

## Provenance

Session 2026-07-04, lead + user via `mochiko:analysis-iterative` (bare). Inputs: v1 command + `validation-brainstorm` (main), dogfood artifacts incl. handover (branch `brainstorm-dogfood-setup-rewrite`), `loop-discipline`, agent-teams official docs (fetched 2026-07-04). Cold review: one independent `mochiko:devils-advocate` pass over the draft record (1C/8I/3M, all dispositioned above) + one verify pass over the folds — this session practiced the design it produced (episodic challenge absent by necessity, cold review + verify present). Built and shipped at plugin v0.4.0, 2026-07-04.
