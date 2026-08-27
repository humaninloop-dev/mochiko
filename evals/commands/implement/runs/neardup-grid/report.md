# Plan-only eval report — implement / neardup-grid

Arms: ['pre', 'post'] · replicates 4 · cost $11.644

Advisory (harness D2): nothing below sets an exit code.

## s1-zero-gap
- post coverage (pass^k): 52/58
- pre coverage (pass^k): 52/58
- **unchanged-bucket regressions:** ['impl.gates-never-triaged', 'impl.minimalism-advisory']
- changed-text (graded vs NEW text, advisory): ['impl.author-grader-default-fail', 'impl.model-tiering', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 8 ['impl.design-inputs', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.gates-never-triaged', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor']
- stub phases (r3): ['Phase 10 — Close with verdict']

## s2-two-gaps
- post coverage (pass^k): 50/58
- pre coverage (pass^k): 48/58
- **unchanged-bucket regressions:** ['impl.cards-template', 'impl.minimalism-advisory']
- changed-text (graded vs NEW text, advisory): ['impl.author-grader-default-fail', 'impl.model-tiering', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 12 ['impl.acceptance-plain-text', 'impl.cards-template', 'impl.dm-landing-whole', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.gates-full-suite', 'impl.gates-never-triaged', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor', 'impl.zero-gap-map-assertion']

## s3-empty-args
- post coverage (pass^k): 50/58
- pre coverage (pass^k): 50/58
- **unchanged-bucket regressions:** ['impl.graded-fold', 'impl.no-git-mutations']
- changed-text (graded vs NEW text, advisory): ['impl.author-grader-default-fail', 'impl.model-tiering', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 9 ['impl.cards-template', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.gates-never-triaged', 'impl.graded-fold', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor']
- stub phases (r3): ['Phase 7 — Gate: card confirm']

- pairwise s2-two-gaps/r1: ('2', '2') (position_consistent=False)
- pairwise s2-two-gaps/r2: ('1', '2') (position_consistent=True)
- pairwise s2-two-gaps/r3: ('1', '2') (position_consistent=True)
- pairwise s3-empty-args/r1: ('2', '2') (position_consistent=False)
- pairwise s3-empty-args/r2: ('1', '2') (position_consistent=True)
- pairwise s3-empty-args/r3: ('1', '2') (position_consistent=True)
- pairwise s1-zero-gap/r1: ('2', '1') (position_consistent=True)
- pairwise s1-zero-gap/r2: ('1', '2') (position_consistent=True)
- pairwise s1-zero-gap/r3: ('2', '1') (position_consistent=True)
- pairwise s1-zero-gap/r4: ('2', '2') (position_consistent=False)
- pairwise s2-two-gaps/r4: ('2', '1') (position_consistent=True)
- pairwise s3-empty-args/r4: ('2', '2') (position_consistent=False)
