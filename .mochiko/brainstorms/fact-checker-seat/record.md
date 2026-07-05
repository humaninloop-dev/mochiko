# Brainstorm Record — the fact-checker seat: role, value, and the rename

> **Topic:** opened as "why was the grounder triggered at the start of my fresh v2.1 run — I want to deeply understand the flow"; converged to a keep-or-remove ruling on the seat.
> **Session:** 2026-07-05 · lead + user, running bare `mochiko:analysis-iterative` (no live team) · method: file-grounded walkthrough of the v2.1 flow + **transcript forensics on both grounder runs** (the audited v2 run `d822aa0a…/subagents/agent-agrounder…jsonl` and the interrupted fresh run `2977fde4…/subagents/agent-agrounder…jsonl`)
> **Status:** **Ruled and built at v0.5.1 (2026-07-05).** This record is un-reviewed (bare session, no cold pair).

---

## Findings

- **F1 · The surprise was a role misread, not a misexecution.** The user expected "grounder" to be the agent that pressure-tests the devils-advocate's findings — the retired v2 **pressure-tester**'s job, which v2.1 dissolved into the review pair's mutual cross-examination. The actual seat is a neutral fact-checker: initial factual map at spawn, on-demand claim checks, fact-dispute arbitration at review; constitutionally "reports what is, never argues what should be."
- **F2 · The command primed the misread.** `brainstorm.md:10` said the machinery "enters exactly twice: grounder fact-checks **when a claim needs verifying**…" and the description called the seat "**optional**" — both suggest lazy, on-demand entry; the at-start spawn lived only in the team section. "Conditional" governs whether the seat exists (topic has a reality surface), never when it spawns.
- **F3 · The fresh run delivered zero value — and never could have.** Transcript: spawn brief received, ~80% through the factual map (setup files, HIL lineage, synthesis), then "[Request interrupted by user]". The unexplained, unannounced spawn read as a malfunction; the run died before the map landed.
- **F4 · The seat's v2-run evidence is real and itemized.** Two engagements, both load-bearing: (1) the initial map surfaced the `setup.md:45` command↔skill contradiction, the no-upfront-elicitation gap, and the discovery that `analysis-iterative` already implements the brainstorm engine the setup rewrite needed (unwired, and absent from principal-architect's `skills:` list); (2) a fact-check of the user's own claim (backend-shaped principles misfit a React app) confirmed it with teeth — `project_type` a false friend, the Essential Floor "stack-agnostic only in its labels," zero frontend floor content. Cost 25k out — the cheapest seat by 5–20×; the only teammate the v2 forensics filed under "What worked (do not lose)" (F7/I-6 ✓✓).
- **F5 · Removal has a designed fallback, with named losses.** The function can't vanish (review fact disputes must be checked, not argued); "remove" means demoting the standing seat to one-shot Explore dispatches — losing volunteered facts, warm context, the early teams probe, and (post-advocate) the only live defense against false premises entering the decision chain.

## Decisions

### D1 — Keep the seat; fix its legibility; judge it on the planned re-run
Keep the conditional at-start seat. Rename **grounder → fact-checker** (the old name affords "tests whether findings are grounded" — it confused its own designer one day after shipping). Rewrite the two misleading phrasings (`brainstorm.md` description + line 10; router entry). Add a spawn-announcement rule: filling the seat is announced in one line — what reality surface warrants it, and that a factual map is coming.
- **Mark:** Confident (recommendation-led; adopted with the removal steelman in full view — the user's revealed preference trail toward end-weighted simplicity, and the precedent that experienced-confusion killed the higher-yielding advocate, were both argued before the ruling).
- **Rejected:** demote-to-on-demand now (Explore one-shots preserve the function but discard the volunteering + warm context that produced F4's catches, and the seat has never been watched to completion by the user); remove entirely (incoherent — review-stage fact disputes need a checker; the Explore fallback already exists at `brainstorm.md` survivor routing).

### D2 — Pre-register the kill check instead of arguing n=1 further
The seat's evidence is one audited run. Rather than relitigate, the doubt becomes a named check on the already-planned v2.1 dogfood (BACKLOG): **in a run watched to completion, the fact-checker's map or checks produce something the user visibly values — else demote to on-demand Explore dispatches** (end-only grounding, the fallback the v2 design named at D5). Same idiom that resolved I-6 the first time.
- **Mark:** Confident (user-ruled via adoption of the packaged recommendation).

## Built (v0.5.1, 2026-07-05)

1. ✅ `plugins/mochiko/commands/brainstorm.md` — rename across all 8 mentions; description + line-10 rewritten ("holds exactly two seats," at-start semantics explicit); spawn-announcement rule added; rename breadcrumb in the team section.
2. ✅ `plugins/mochiko/skills/validation-brainstorm/SKILL.md` — fact-dispute routing renamed (description + cross-examination phase).
3. ✅ `plugins/mochiko/skills/mochiko/SKILL.md` (router) — brainstorm entry re-worded (seat semantics), reviewer row renamed.
4. ✅ `REGISTRY.md` (brainstorm + validation-brainstorm rows, with né-pointer) · `ROADMAP.md` (v2.1 row pointer + trail **Addendum** recording this ruling) · `BACKLOG.md` (v2.1 dogfood item: rename, the new seat-value kill check, first-fresh-attempt outcome note).
5. ✅ Version bump 0.5.0 → 0.5.1 (`plugin.json` + `marketplace.json`).

Untouched by design: both historical design records, the closed v2 BACKLOG item, and ROADMAP's v2 row/trail — they describe the past, where the seat was named *grounder*.

## Provenance

Session 2026-07-05, lead + user via `mochiko:analysis-iterative`. Inputs: `plugins/mochiko/commands/brainstorm.md` (v0.5.0), both design records, `BACKLOG.md`, and the two grounder transcripts (v2 run + interrupted fresh run). Changes uncommitted at record time. Next step: re-run the fresh v2.1 dogfood on the setup-workflow-rewrite topic — the slug dir is empty, so Recovery has nothing stale to resume.
