# Persona eval noise band — counted on invited pairs only (instrument re-key across kits)

- **Status:** ruled
- **Date:** 2026-09-09
- **Evidence:** the run directories cited below are archived at the tag `eval-evidence-2026-09-19`,
  not carried in the working tree (`git fetch origin tag eval-evidence-2026-09-19`, then
  `git show eval-evidence-2026-09-19:<path>`).
- **Context:** The pre-registered noise band for the persona target (`primitive-eval-harness-v2`
  I7) is the replicate-disagreement share over every (golden, graded claim) pair, plus five points,
  capped at 20 %. Every graded claim is judged on every golden, so the count includes pairs where
  the golden never invites the claim — a governance-standard claim read against a feasibility
  review, a delegation claim read against a bug-fix card. At the `tech-lead` baseline (wave A,
  `evals/agents/tech-lead/runs/baseline`) those uninvited pairs carried most of the noise: 13/48
  flaky on all pairs (27.1 %, six of them on the feasibility golden t2) against 4/28 on the pairs
  the goldens tempt (14.3 %), with pass^k coverage 24/28 on the invited set. Recounted on the three
  earlier kits the effect runs both ways — `staff-engineer` pilot 1 `post` 35.0 % all-pairs vs
  61.5 % invited (the latitude claims pilot 1 ruled out of instrument, no longer diluted by trivial
  absents), `validator` pilot 2 `post` 17.9 % all-pairs vs 10.0 % invited (the guard no longer trips; the
  pilot's reading rests on the control-not-exercised evidence, unchanged), `devils-advocate` baseline 33.3 % vs 50 %
  on two invited pairs. The user ruled (2026-09-09): adopt the invited-only band as recommended.
- **Decision:**
  1. **The band counts invited pairs only.** For each arm, the flaky share is the number of
     (golden, claim) pairs with replicate disagreement over the pairs where the golden's `tempts`
     list names the claim; plus five points, capped at 20 %. Uninvited pairs are still judged and
     still feed coverage and the regression read; they are disclosed beside the band, never in it.
  2. **Ship bar (b) reads the invited-only figure.** A kit whose invited flaky share is ≤ 15 % has a
     band that binds uncapped.
  3. **Under-sampling is disclosed, not hidden.** A kit with fewer than eight invited pairs on an
     arm carries an `UNDER-SAMPLED` mark; its band is the cap (20 %) and its next edit takes the
     one extra replicate per arm the guard already allows before any difference is read.
  4. **Applies to every kit from this ruling, retroactively recounted and disclosed.** The four
     existing fill logs carry the recount as an addendum; the pilot rulings stand as read
     (pilot 1 trips harder under the new count; pilot 2's guard no longer trips, its ruling unchanged). The `tech-lead` kit is READY:
     invited band 19.3 % uncapped, ship bar (b) met. The `devils-advocate` kit is READY with the
     under-sampled mark: two invited pairs, one flaky (the d3 persona finding), band at the cap.
  5. **The report carries both figures** (`agent-report`: the per-golden band line names the
     invited count with the all-pairs count in parentheses, and a closing *Band input — all
     goldens* section prints the aggregate per arm with the band it fixes).
- **Rationale:** A claim read against a task that never invites it is a control read: the honest
  answer is `absent`, and disagreement there measures the judge's tolerance for near-misses, not
  the persona's stability. Counting it either inflates the band with near-misses or dilutes it
  with easy absents, depending on the kit; neither is the quantity the guard exists to bound. The
  invited-only figure is the one that stays comparable across kits with different golden shapes.
  Rejected: **re-cut t2** to invite the governance claims (≈ $2; the persona's remit rule says it
  declines governance under the feasibility lens, so the noise would move rather than drop);
  **accept the capped band with a disclosure** (cheapest; leaves every kit's next-edit read
  binding only through the cap).
- **Disclosure:** post-hoc for `tech-lead` — the recount was made after the baseline was read;
  recorded in its fill log as a measurement first and adopted here by ruling. The cross-kit recount
  is the check against cherry-picking: the rule was adopted knowing it worsens two of the four
  figures.
- **Alternatives considered:** weight uninvited pairs at half (rejected — an invented constant);
  drop uninvited pairs from the judge call entirely (rejected — they are the coverage read for
  claims a golden happens to reach, and the removed-claim ghost read depends on them).
