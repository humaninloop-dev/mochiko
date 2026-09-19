# Plan-only eval report — implement / optionA-grid

Arms: ['pre', 'post'] · replicates 3 · cost $13.0973

Advisory (harness D2): nothing below sets an exit code.

## s1-zero-gap
- post coverage (pass^k): 54/58
- pre coverage (pass^k): 50/58
- **unchanged-bucket regressions:** none
- flaky rules (replicate disagreement — noise-guard input): 8 ['impl.cards-template', 'impl.design-inputs', 'impl.dm-surface-rounds', 'impl.escalation-batching', 'impl.finding-severity-routing', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor']
- stub phases (r3): ['Phase 8 — Repeat Phases 6–7 per remaining card']

## s2-two-gaps
- post coverage (pass^k): 47/58
- pre coverage (pass^k): 51/58
- **unchanged-bucket regressions:** ['impl.acceptance-plain-text', 'impl.design-map-assertion', 'impl.escalation-batching', 'impl.gap-finding-blind-dispatch', 'impl.landing-verifier-folds']
- flaky rules (replicate disagreement — noise-guard input): 12 ['impl.acceptance-plain-text', 'impl.design-map-assertion', 'impl.dm-surface-rounds', 'impl.escalation-batching', 'impl.finding-severity-routing', 'impl.gap-finding-blind-dispatch', 'impl.gates-never-triaged', 'impl.landing-verifier-folds', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor', 'impl.zero-gap-map-assertion']
- stub phases (r1): ['8 — Card review before confirm']
- stub phases (r3): ['8 — Card review (independent)', '9 — Card confirm (USER GATE)']

## s3-empty-args
- post coverage (pass^k): 50/58
- pre coverage (pass^k): 52/58
- **unchanged-bucket regressions:** ['impl.gates-never-triaged', 'impl.minimalism-advisory', 'impl.no-git-mutations']
- flaky rules (replicate disagreement — noise-guard input): 7 ['impl.acceptance-plain-text', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.gates-never-triaged', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations']
- stub phases (r1): ['7 — Card confirm']

- pairwise s3-empty-args/r1: ('2', None) (position_consistent=False)
- pairwise s3-empty-args/r2: ('2', '2') (position_consistent=False)
- pairwise s3-empty-args/r3: (None, None) (position_consistent=False)
- pairwise s1-zero-gap/r1: ('2', '1') (position_consistent=True)
- pairwise s1-zero-gap/r2: (None, None) (position_consistent=False)
- pairwise s1-zero-gap/r3: ('1', '2') (position_consistent=True)
- pairwise s2-two-gaps/r1: (None, None) (position_consistent=False)
- pairwise s2-two-gaps/r2: (None, None) (position_consistent=False)
- pairwise s2-two-gaps/r3: ('2', None) (position_consistent=False)
