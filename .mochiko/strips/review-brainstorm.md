# Strip notes — `skills/review-brainstorm/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave — R1 live-defect
repair batch (design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified
2026-07-25). Description ledger — separate accounting from SKILL.md body lines per R2. Repair
context: R1 measurement (2026-07-25) proved delivery truncates descriptions at exactly 1,536
chars; this description was shipping with its TAIL silently cut — the negative boundary +
FAIL-posture clauses never reached any session. The rewrite preserves all MUST/SHOULD triggers,
restores the boundary clauses under the cap, and is strict-YAML-safe. Delivery verification
deferred: the in-session skill listing is a session-start snapshot (probe received pre-edit
text), so the probe re-runs in a fresh session.

## [v0.67.0] Sixth hunt class (excess machinery / unpaid decision) added — three "five hunt classes" counts re-keyed
- **Disposition:** superseded → the excess posture from the architect-role ruling: a sixth, remove-shaped hunt class is added, so the three "five hunt classes" counts become "six".
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md`, D3 as amended by its F3 calibration clause; DECISIONS.md combined-wave build row).
- **Content (superseded, verbatim — count reconciliations):**
  - Phase 1 item 2 — OLD: "2. **The five hunt classes**, per decision:" → "2. **The six hunt classes**, per decision:".
  - Coverage-findings paragraph — OLD: "a first-class finding beside the five hunt classes above." → "...beside the six hunt classes above.".
  - Description — OLD: "independent cold read, the five hunt classes, then cross-examination" → "...the six hunt classes...".
- **Kept deliberately:** hunt classes 1–5 (Unchallenged assumption, Missing dimension, Passive acceptance, Rejected-road steelman, Inconsistency) untouched; the coverage-findings class and the Phase 0 blind-map machinery untouched. Pure addition riding the decision row (no strip): the class-6 table row carrying the calibration clause in one breath.
- **Consumers assessed:** no reference file or command carries the "five hunt classes" count (grep confirmed the three sites are all inside this SKILL.md). Description stays under the 1,536 delivery cap (490 chars) and within budget (614).

## [v0.64.0] Guardrails Wave 2 — slim description + review-evidence floor line (no body deletions)
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line). Description slimmed; body carries no deletion — only the sanctioned floor-line pure addition below.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,506 → 491 chars (−67%). Body 11,326 → 11,508 chars (+182, +2% — the floor-line addition; **zero body deletions**). Description cut: the full protocol enumeration (blind-map-diff mechanics, the five named hunt classes spelled out, reality-grounding-via-fact-checker-map, the CROSS-EXAM owner-withdrawal detail, the severity buckets, the three verdict-state names, the SHOULD-trigger production detail) compressed. Kept in the slim description: the MUST cold-END-STAGE-REVIEWER trigger, `record.md`, the blind-angle-map-yielding-coverage-findings phrase (routing to the v0.60.0 protected machinery), the five-hunt-classes phrase, cross-examination, the paired/solo + never-in-the-room independence triggers, the SHOULD triggers (verify pass · synthesis fidelity sample · one-shot cold review), and the never-a-co-author / defaults-to-FAIL posture.
  - **Old description (verbatim):** "This skill MUST be invoked when serving as a cold END-STAGE REVIEWER of a collaborative thinking session's decision record (`record.md`) — spawned at convergence (one of a lens-briefed pair by default, or solo when the user sized the review down), never in the room during the session. Protocol — a blind angle map (topic alone) whose diff yields coverage findings for never-visited angles; then independent cold read; scenario stress per decision; the five hunt classes (unchallenged assumptions, missing dimensions, passive acceptances, steelman-able rejected alternatives, inconsistencies); reality-grounding of load-bearing claims via the record's fact-checker map (no map → the files directly); the standalone-record fitness checklist. Then CROSS-EXAMINE the counterpart per the one-shot protocol (`references/CROSS-EXAM.md` — owner-withdrawal only; fact disputes route to the fact-checker, never to argument) and return survivors severity-classified (Critical/Important/Minor) with a tally and a RECOMMENDED status (ready / needs-revision / critical-gaps) — the cross-set merge and clearing verdict are lead-owned. SHOULD also invoke for the verify pass over a record's folded resolutions, the fidelity sample of a requested synthesis, or a one-shot cold review of a decision record outside a live team. Run by an independent reviewer, never a session co-author; defaults to a FAIL posture — zero findings means hunt harder, and every finding needs a concrete failure scenario or cited contradiction."
  - Verbatim homes for the removed description text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line added (pure addition, cross-cutting finding 1 / F-X1 mitigation):** "The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation." Placed in `## Independence`, as a new bullet after the "Findings enter the record through the lead's pen, with dispositions" bullet — mirroring the Wave-1 `review-governance-intent` placement (which carries the same line alongside its lead's-pen bullet). Rides the same decision row.
- **Kept deliberately (body — the entire body survives):** no body prose was deleted. The [v0.26.0] KEPT whole-body survivor ruling is honored intact — every phase, table, and pointer stands. The v0.60.0 DECISIONS-traceable machinery (Phase 0 blind angle map, the coverage-findings map-vs-record diff, the reopen-born-decision verify grading) is protected, ruled content and survives whole. The five-hunt-class table, the two verdict tables, the CROSS-EXAM/EXTERNAL-CLAIMS/RECORD-FITNESS pointers, and the Common Mistakes table are untouched.
- **MANDATORY KEPT reconciliation:** grep of this strip for `KEPT`/protected/DECISIONS-traceable entries — [v0.26.0] KEPT (entire remaining body) and [v0.60.0] supersession (hunt-class-2 re-key) and [v0.52.0] (CROSS-EXAM carve-out). This cut removes **no** body line, so no prior KEPT or protected content is touched. The v0.60.0 blind-map/coverage/reopen content (RULED, protected) fully survives — verified present after the edit.
- **Consumers assessed:** commands `plugins/mochiko/commands/` — grep clean (the brainstorm command orchestrates it by dispatch, not by name in body). Agents — `plugins/mochiko/agents/devils-advocate.md` declares it in `skills:` and carries a when-to-reach bullet ("cold end-stage review of a thinking session's `record.md`"); the slim description preserves that cold-review role and every routing trigger, so the composition is intact. `plugins/mochiko/agents/validator.md` points at `skills/review-brainstorm/references/EXTERNAL-CLAIMS.md` — a `references/` file, untouched by this cut. `review-governance-intent` shares `references/CROSS-EXAM.md` — substrate-agnostic, untouched. Contract intact.

## [v0.60.0] Hunt class 2 narrowed to intra-decision scope; topic-level coverage moves to the new coverage class
- **Disposition:** superseded → the new "Coverage findings — the map-vs-record diff" class (topic-level never-visited dimensions) + a re-keyed class 2 (intra-decision missing factors)
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/cold-review-gap-challenge/record.md` D10; DECISIONS.md 2026-08-10 row)
- **Content:** `| 2 | **Missing dimension** | What angle (cost, failure mode, actor, timescale) was never visited? |`
- **Kept deliberately:** the intra-decision reading — a decision that never weighed cost/failure-mode/actor/timescale is a real miss the map-diff will not catch, so class 2 survives re-keyed rather than deleted (keep-both-unbounded and full-supersession both rejected in D10).
- **Consumers assessed:** `review-governance-intent` keeps its own hunt class 1 (agenda-diff) — a different class, unaffected; the shared `CROSS-EXAM.md` is substrate-agnostic and carries no class text. No other consumer references class 2 text.

## [v0.52.0] CROSS-EXAM.md fact-dispute rule gains an external-claim carve-out
- **Disposition:** superseded → `references/EXTERNAL-CLAIMS.md` (external-claim disputes only;
  every other fact dispute keeps routing to the session's fact authority exactly as shipped)
- **Tier failed:** n/a — supersession by ruling (DECISIONS.md 2026-08-04 external-research row,
  ER-D4 as amended at review fold F1; record
  `.mochiko/brainstorms/external-research-in-review/record.md`)
- **Content:** the fact-dispute bullet's totalizing reading — "route it to the session's fact
  authority… one route per fact" as the *only* route, with no path for a fact that authority
  holds no jurisdiction over (the fact-checker never fetches; an external-claim dispute
  dead-ended).
- **Kept deliberately:** the fact-dispute bullet's own text verbatim; the four-message exchange
  and all other attack/defense standards untouched. The carve-out lands as an added bullet, not
  a rewrite — CROSS-EXAM.md remains the single pair-protocol home, delegating exactly one
  dispute class by pointer.
- **Consumers assessed:** `mochiko:review-brainstorm` and `mochiko:review-governance-intent`
  (the file's charter: "An edit here changes both skills") — neither restates the fact-dispute
  rule locally; both pick up the carve-out through the shared file. Both also gained their own
  EXTERNAL-CLAIMS.md binders in the same v0.52.0 build, so the routes agree.

## [v0.46.0] loop-discipline pointer reworded
- **Disposition:** superseded → "its command states them"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "— see `loop-discipline`; this skill does not restate them." → "— its command states them; this skill does not restate them."
- **Consumers assessed:** brainstorm command briefs unchanged.

## [v0.26.0] Six pair-protocol / in-file-restatement Common-Mistakes rows deleted (body 78 → 72, −7.7%)
- **Disposition:** deleted as pure restatements of verified single-sourced homes: three rows
  (consensus-seeking, withdrawal-under-pressure, fact re-routing) restate `references/CROSS-EXAM.md`'s
  Attack-and-defense standards near-verbatim — the file Phase 2 mandates loading at exactly the
  temptation moment; three rows (counterpart-contact, tally-merging, lens-dropping) restate this
  file's own Phase-1 sequestration line, Phase-3 tally line, and Overview lens paragraph
  (26–60 lines up). All homes Read and confirmed before landing. Wave context: skill-succinctness
  wave 2 (review-\* cluster), batch-3 proposal ratified 2026-07-25 — user directed continuation
  on the recommended dispositions
- **Tier failed:** 1 (distinct from the R4b aphorism ruling, which protected copies with **no**
  home — these had two verified homes each)
- **Content:** the six table rows; the five skill-specific rows (resolution paths, map-trusting,
  `Contested`-raising, softening, own-session grading) kept
- **Consumers assessed:** wave-open enumeration — 6 citing files, none reference the rows.
  Shared-home audit this wave: `references/CROSS-EXAM.md` (4 consumers) is a clean single source —
  no dead pointers, no duplication-only content, untouched; `references/RECORD-FITNESS.md` is
  single-consumer but at correct altitude (checklist detail — inlining would add body lines), no action

## [v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling, 7.7% vs the 30–70 band)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — authored
  post-doctrine at altitude: the pair protocol is run by reference (never restated), every
  Overview and Phase paragraph names its failure mode (framing-inheritance, captured-by-the-room,
  unverifiable-claim-is-a-finding), the hunt-class and verdict tables are the skill's core unique
  content. The sentences shared with `review-governance-intent` ("lens sets depth, not
  jurisdiction"; "every finding carries…"; "findings-formed — count only") are KEPT by design:
  CROSS-EXAM's header assigns per-skill substrate bindings, no canonical home exists for them,
  and pointer economics are negative at 1–3 lines each. Fifth whole-skill survivor of the pass.
  Session ruling: batch-3, 2026-07-25.

## [v0.24.0] DESCRIPTION: cut 1,795 → 1,413 chars (delivery cap measured at exactly 1,536)
- **Disposition:** deleted (description ledger)
- **Tier failed:** 2 — every dropped clause is protocol detail restated from the SKILL.md body / `references/CROSS-EXAM.md`; no trigger or boundary behavior lost
- **Content:** dropped clauses — the sample-audit-the-map record-integrity-lens instruction; the attack/defend, persuades-never-vetoes, cited-never-re-routed exchange detail; the tally example string ("N raised, M survived"); the `RECORD-FITNESS.md` path (checklist still named); "before any contact with the counterpart reviewer" (substance preserved by "independent cold read FIRST")
- **Consumers assessed:** delivery-side only — no file consumes description text
