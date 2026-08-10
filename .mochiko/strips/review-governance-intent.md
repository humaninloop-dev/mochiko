# Strip notes — `skills/review-governance-intent/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave — R1 live-defect
repair batch (design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified
2026-07-25). Description ledger — separate accounting from SKILL.md body lines per R2. Repair
context: R1 measurement (2026-07-25) proved delivery truncates at exactly 1,536 chars; the
shipped description was losing its TAIL — the bounded delta-pass SHOULD trigger and the
never-session-lead / FAIL-posture boundary clauses. The rewrite preserves all MUST/SHOULD
triggers, restores the tail under the cap, and is strict-YAML-safe. Delivery verification
deferred: the in-session skill listing is a session-start snapshot (probe received pre-edit
text), so the probe re-runs in a fresh session.

## [v0.63.0] Guardrails cut — body deletions + slim description (benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/`)
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict — `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed).** Body 13,374 → 7,273 chars (−6,101, −46%; this figure is net of the +~185-char review-evidence floor line added as a pure addition, so the deletion alone is larger). Description 1,499 → 483 chars. Sections removed or shortened:
  - **## Phase 0 — Blind angle map** (removed whole) — the topic-only blind angle-map pre-read, its free-repo-grounding rules, and the augments-hunt-class-1 governance (v0.60.0 cold-review gap-challenge addition).
  - **Phase 1 five-hunt-classes table** (removed) — the missed-dimension / unchallenged-fact-profile / passive-card-acceptance / too-easily-resolved-reality-conflict / thin-rationale-echo-hunt table, the sweep-remaining-elements paragraph (waivers · modules · minted intents · exclusions), and the "Use the marks to prioritize, never to skip" doctrine paragraph. Heading kept, renamed **## Independent cold read**; its sequestration and finding-shape prose kept.
  - **Coverage-findings paragraph** (removed) — the beyond-agenda blind-map-diff finding class and its materiality bar (v0.60.0 addition).
  - **## The verify pass — and the post-review-edit delta-pass** (removed whole) — the verify-pass fold-check, the bounded post-review-edit delta-pass, and reopen-born-intents handling.
  - **Phase framing dropped** — Phase 1/2/3 headings renamed to plain **## Independent cold read** / **## Cross-examination** / **## Survivor report**; Phase 3's cross-set-merge / survivor-routing detail trimmed to the survivor-report essentials.
  - Old description verbatim: "This skill MUST be invoked when serving as a cold INTENT REVIEWER in a `/mochiko:setup` run — stress-testing the frozen, confidence-marked interrogation synthesis (`.mochiko/memory/governance-intent.md`) BEFORE the user ratifies it at setup's synthesis-ratification checkpoint — spawned at the sizing gate (one of a coverage/coherence lens-briefed pair by default, or solo when sized down), never a participant in the interrogation session. Protocol — independent cold read FIRST; the five setup hunt classes (missed dimensions against the ten-dimension agenda, unchallenged fact-profile calls, passive card acceptances, too-easily-resolved reality conflicts, thin-rationale echo hunts); reality-grounding against `codebase-analysis.md` in brownfield. Then CROSS-EXAMINE the counterpart per the single-sourced pair protocol (`review-brainstorm`'s `references/CROSS-EXAM.md`) and return survivors severity-classified (Critical/Important/Minor) with a tally and a RECOMMENDED status (ready / needs-revision / critical-gaps) — survivor routing, the clearing verdict, and ratification are the lead's and the user's. SHOULD also invoke for the verify pass over the synthesis's folded dispositions (the coherence lens in a pair, automatic when solo) or the bounded delta-pass on a material post-review edit. Run by an independent reviewer, never the session lead; defaults to a FAIL posture — zero findings means hunt harder, and every finding needs a concrete failure scenario or cited contradiction."
  - Verbatim removed text survives in three places: (a) git history of the original `plugins/mochiko/skills/review-governance-intent/SKILL.md`; (b) the before/after pair in this tree — `.mochiko/benchmarks/guardrails-vs-detail/variants/body/review-governance-intent/SKILL.md` (after) and the pre-edit original (before, in git); (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** goal/output contract (Overview + the frozen-synthesis contract); the permanent out-of-jurisdiction floor (authored surface set + the D1 formulation-quality exclusion); the pair-protocol-by-reference with skill-specific substrate bindings (the reality-surface vs user-declared vs external-sourced fact-authority split; the `references/CROSS-EXAM.md` and `EXTERNAL-CLAIMS.md` pointers); the verdict table (ready / needs-revision / critical-gaps) with "never default to `ready`"; the Common Mistakes table; the FAIL-posture floor. **Added (pure addition):** the review-evidence floor line in ## Independence.
- **Protected-content reconciliation.** The `[v0.26.0] KEPT: the entire remaining body` survivor ruling named five elements as kept; this guardrails cut REMOVES three of them, recorded here as superseded-by-this-ruling — never silently dropped:
  - "the five setup hunt classes" (the Phase-1 hunt-class table) — REMOVED. Superseded.
  - "the marks-prioritize-never-skip doctrine" — REMOVED. Superseded.
  - "the G3-edit delta-pass" (now the post-review-edit delta-pass) — REMOVED. Superseded.
  The other two v0.26.0-KEPT elements survive: the pair-protocol-by-reference with substrate bindings, and the D1 jurisdiction exclusion; the verdict-table criteria also survive. The v0.60.0 cold-review gap-challenge additions removed here (blind angle map, coverage findings, verify/delta/reopen passes) are DECISIONS-traceable (`4e2f1b3`, `baf67d7`) and are likewise superseded by this same benchmark ruling. The prior `[v0.24.0]`/`[v0.46.0]` DESCRIPTION entries concern the earlier description; the slim description supersedes that earlier description in full.
- **Consumers assessed:** no command references this skill (grep `plugins/mochiko/commands/` clean). `agents/devils-advocate.md` declares it in `skills:`; the kept goal/contract/floor/`references` pointers leave that composition intact.

## [v0.46.0] loop-discipline pointer reworded
- **Disposition:** superseded → "its command states them"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "— see `loop-discipline`; this skill does not restate them." → "— its command states them; this skill does not restate them."
- **Consumers assessed:** setup command briefs unchanged.

## [v0.26.0] Three pair-protocol / in-file-restatement Common-Mistakes rows deleted (body 138 → 135, −2.2%)
- **Disposition:** deleted as pure restatements of verified single-sourced homes — the
  counterpart-contact row restates this file's own Phase-1 sequestration line + the shared
  `review-brainstorm/references/CROSS-EXAM.md` withholding rule; the tally-merging row restates
  Phase 3's "the cross-set merge and the combined count are the lead's" + CROSS-EXAM's
  flagged-not-merged standard; the lens-dropping row restates the Overview lens paragraph. All
  homes Read and confirmed before landing (mirror of the same-day `review-brainstorm` strip).
  Wave context: skill-succinctness wave 2 (review-\* cluster), batch-3 proposal ratified
  2026-07-25 — user directed continuation on the recommended dispositions
- **Tier failed:** 1
- **Content:** the three table rows; the nine setup-specific rows (mark-audit, `Contested`
  rationale audit, tier-consistency vs tier-choice, user-declared facts, D1 jurisdiction,
  governance-taste, resolution paths, session-confirmation softening, own-session grading) kept
- **Consumers assessed:** wave-open enumeration — 5 citing files, none reference the rows

## [v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling, 2.2% vs the 30–70 band)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — authored
  post-doctrine at altitude: the pair protocol runs by reference with skill-specific substrate
  bindings (the reality-surface vs user-declared fact-authority split is unique load-bearing
  content); the five setup hunt classes, the marks-prioritize-never-skip doctrine, the G3-edit
  delta-pass, and the D1 jurisdiction exclusion each name their failure mode; the verdict-table
  criteria are setup-specific. The description is the fresh v0.24.0 repair (1,500 chars, ≤1,536) —
  untouched, boundary clauses intact. Sixth whole-skill survivor of the pass. Session ruling:
  batch-3, 2026-07-25.

## [v0.24.0] DESCRIPTION: cut 1,778 → 1,500 chars (delivery cap measured at exactly 1,536)
- **Disposition:** deleted (description ledger)
- **Tier failed:** 2 — every dropped clause is doctrine restated from the SKILL.md body's hunt-class protocol; no trigger or boundary behavior lost
- **Content:** dropped clauses — the confidence-marks-are-lead-self-reported / echo-rationales-stay-independent rationale clause; the `Contested`-mark-is-no-unaudited-shield guard (body-owned); the "interrogation" qualifier on the ten-dimension agenda; the long solo-sizing phrasing ("or solo when the user sized the review down" → "or solo when sized down"); "before any contact with the counterpart reviewer" (substance preserved by "independent cold read FIRST")
- **Consumers assessed:** delivery-side only — no file consumes description text
