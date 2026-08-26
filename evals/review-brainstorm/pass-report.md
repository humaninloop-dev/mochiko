# Compression pass report — `review-brainstorm`

Pass opened and ratified 2026-08-26. Compressor: session lead. Mode: **user-directed cut** on
the `review-feasibility` v0.82.0 precedent ("cut now, eval validates later" — the pre-cut eval
is superseded as an instrument and re-purposed as a post-cut regression check; `rules.json` +
`evals.json` to be authored by a non-compressor seat when that check runs).

## Pass narrative

1. User directive: reduce the skill by 90%.
2. First draft: a breakup (floors-and-dispatch body at −87.0% + new
   `references/REVIEW-PROTOCOL.md` carrying the relocated protocol) on the review-feasibility
   pattern. **Rejected by the user mid-pass** — "all the verbosity is shifted to review
   protocol": here the reference did not pre-exist (review-feasibility's did, twinning its
   body), so relocation grew the corpus and a real review run would read the protocol file
   anyway. The relocation file was deleted; nothing of it shipped.
3. Second draft: **true deletion** — single file, no new reference, every behavioral rule
   compressed to a clause, all rationale/worked prose deleted. Rule-retention floor as first
   measured: ~1,965 chars (−83.3%) — proved low by six rules at audit, the true rule-complete
   floor being the landed 2,497; reaching −90.0% (≤1,175) required deleting six
   ruled/load-bearing rules outright (reopen-born verify grading, synthesis fidelity sample, class-6 calibration
   clause, coverage materiality gate, blind-map grounding fence, cross-exam substrate
   binding).
4. Ratification gate: user ruled **ship the rule-complete cut** over forcing −90%.
5. Post-gate repairs: the integrity-lens sample-audit clause (present in the old body,
   missing from the ratified draft) restored first (+71 chars, lead-caught). Then the
   author≠grader audit (FAIL round 1, 7 blocking) forced six further restorations the
   ratified draft had silently dropped — the lens taxonomy definitions (deleted while both
   gate uses remained), the coverage-severity test clause, the full cross-exam substrate
   binding (fact authority), the decision(s)-touched contract field, the fidelity-sample
   criteria, and the v0.46.0 its-command-states-them clause — plus strip/ADR contradiction
   repairs and the DECISIONS row. Landed body: **2,497 chars, −78.8%**.

## Measurements (chars, Python `len`, never `wc -c`)

| Surface | Baseline | Landed | Δ |
|---------|----------|--------|---|
| `SKILL.md` body | 11,754 | 2,497 | **−78.8%** |
| `SKILL.md` `description:` | 490 | 490 (out of scope, D6) | 0 |
| `references/CROSS-EXAM.md` | 3,159 | untouched (shared with `review-governance-intent`) | 0 |
| `references/EXTERNAL-CLAIMS.md` | 5,034 | untouched (`agents/validator.md` points here) | 0 |
| `references/RECORD-FITNESS.md` | 1,606 | untouched | 0 |

No new files. Body budget re-seeded 11,508 → 2,497 (cap 3,122, +25% rounded up) at landing
(R11), executed in `.mochiko/memory/primitive-cost-budgets.md`.

## Protected-content reconciliation (R2)

`.mochiko/strips/review-brainstorm.md` read end to end before drafting. Protected set and its
fate — full detail in the [v0.83.0] strip entry:

1. **[v0.26.0] KEPT: entire remaining body** — superseded by this ruling (the ratification is
   the recorded ruling; review-feasibility v0.82.0 is the worked precedent).
2. **[v0.60.0] ruled machinery** (blind angle map · coverage diff/materiality/dismissed-angle
   · reopen-born verify) — survives compressed, no rule deleted.
3. **[v0.67.0] class-6 row** — survives compressed with its calibration clause.
4. **[v0.64.0] floor line** — substance intact, verbatim wording superseded by its compressed
   form under the same ruling.
5. **[v0.52.0] carve-out** — untouched (lives in the untouched references).

## Consumers assessed

`agents/devils-advocate.md` (description untouched, routing intact) · `agents/validator.md`
(→ EXTERNAL-CLAIMS.md, untouched) · `review-governance-intent` (→ CROSS-EXAM.md, untouched) ·
`commands/brainstorm.md` (blind-map dispatch mechanics live in the command; "blind angle
map" / coverage / reopen-born vocabulary survives in the body). Grep confirmed no external
consumer quotes a deleted body line.

## Eval status

Deferred at the cut, then **run 2026-08-26** before commit at the user's direction: a
non-compressor seat authored `rules.json` (67 rules from the pre-cut baseline) + 3 goldens +
the preregistration (user-ratified: 6-rule floor set, ≤5/61 non-floor, 0 vocab, ≤1 must);
probe clean; results in the Verify round section below. Losses re-added via the strips re-add
path, never by silent edit.

## Verify round (2026-08-26, grid `runs/20260826-110222`)

27 runs (noskill · armA pre-cut · armB cut), case-major order, ~$10.7 est. Sonnet under test,
pass^k over 9 runs per arm-rule.

**Round 1 (cut as shipped at v0.83.0 + audit fixes):** BAR BREACHED — floor R-001 lost: 5 of 9
cut runs produced no blind-map artifact (pre-cut 9/9 wrote `phase0-blind-map.md`); the map
cluster (R-010/R-011/R-012, vocab R-059/R-060) fell with it, plus R-035. Root cause: "Return
the map to the lead as a message" was deleted as prose but was behavioral. The pre-cut arm's
apparent R-004 floor loss was a judge false positive (silence about `Contested` scored as
unverifiable; the rule is a prohibition — silence complies).

**Re-adds (user-ruled "re-add + verify"):** map-as-own-deliverable + "Phase 0" vocab; the
commentary clause.

**Round 2 (cut arm re-run, 9 fresh sessions):** floor CLEAN — 9/9 map deliverables. Residual:
R-060 tally form (6/9 — survivor counts without "N raised, M survived"), R-049 critical-gaps
criteria, R-035 1-of-9 replicate miss. R-060/R-049 re-added (+148 chars, ship unverified);
final body 2,747 chars (−76.6% vs pre-cut 11,754). Bar after round 2, before the micro
re-adds: non-floor 3/≤5 (in), vocab 1/0 (breach — the re-added tally form addresses it),
must 2/≤1 (breach — R-049 re-added; R-035 flake-adjacent, already present in the body).

Judge health: round-1 parse failures 15 (one armA/G3/r1 chunk), round-2 total 22 (adds one
armB/G3/r2 chunk); parse failures score as not-held and were hand-checked not to drive the
named losses. The runner's own "rules lost" line is against the absent `baseline` arm and is
ignored per the preregistration; the figures above are the hand-computed diff over `held`.
