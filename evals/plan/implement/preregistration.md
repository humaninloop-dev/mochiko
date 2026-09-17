# Preregistration — implement plan-only eval

Committed BEFORE the first grid (record build 2; skill-harness R6 carried over). The
runner refuses a grid without this file. Amending it after results exist is a recorded,
deliberate act — never a quiet retro-fit.

## Read rule (D6 as amended — tolerance band, verify V4)

- Comparison substrate: the **in-grid pre-edit arm** (never the committed baseline
  file, which is the pinned historical record).
- **Unchanged bucket:** a rule regresses when it is `reflected` under pass^k in the
  pre arm and not in the post arm. Tolerance: **0 regressed rules** is the pass reading;
  1–2 regressed rules = "investigate — read the evidence quotes before any verdict";
  ≥3 = "regression reading, present to the maintainer". Coverage-count drift without a
  named regressed rule is noise, not signal.
- **Removed bucket:** any removed rule still surfacing (pass^k in post) = "edit did not
  take" finding.
- **Added bucket:** an added observable rule not reflected in any post replicate =
  DEAD-TEXT finding.
- **Changed bucket:** graded against the NEW text; pre-vs-post comparison advisory
  only.

## Noise guard (F2's guard, verbatim discipline)

Same-variant replicate spread exceeding the variant gap = noise; run one more
replicate pair before any verdict. Operationally here: if the count of flaky rules
(replicate disagreement within one arm) exceeds the count of pass^k differences
between arms, the grid is noise-dominated — add one replicate per arm and re-judge
before reading anything.

## Grid shape

3 goldens (s1-zero-gap · s2-two-gaps · s3-empty-args) × 3 replicates × 2 arms
(pre + post), plus a one-time no-command control arm (3 × 3) at pilot. Judges: Haiku
coverage checklist over the 58-rule observable subset + stub axis; Sonnet pairwise,
position-swapped. All judges advisory (harness D2) — the runner exits 0 on judged
degradation.

## Ship bar (advisory instrument — informs, never gates)

The instrument is useful if, on its first real edit, it (a) localizes at least one
true behavioral difference to named rule IDs, and (b) its flaky-rule set stays under
20% of the observable subset. Failing (b) triggers the record's noise falsifier
(open question 3): the substrate bet weakens and the session premise is revisited.
