# Plan-only eval report — implement / ontology-grid

Arms: ['pre', 'post'] · replicates 4 · cost $18.3063

Advisory (harness D2): nothing below sets an exit code.

## s1-zero-gap
- post coverage (pass^k): 52/58
- pre coverage (pass^k): 48/58
- **unchanged-bucket regressions:** ['impl.store-landing']
- added-rule adoption: 
- changed-text (graded vs NEW text, advisory): ['impl.landing-selection', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 10 ['impl.acceptance-plain-text', 'impl.cards-template', 'impl.design-inputs', 'impl.dm-surface-rounds', 'impl.escalation-batching', 'impl.finding-severity-routing', 'impl.no-git-mutations', 'impl.seat-sufficiency-independence', 'impl.store-landing', 'impl.transport-floor']

## s2-two-gaps
- post coverage (pass^k): 47/58
- pre coverage (pass^k): 51/58
- **unchanged-bucket regressions:** ['impl.acceptance-plain-text', 'impl.dm-landing-whole', 'impl.gates-full-suite', 'impl.minimalism-advisory', 'impl.no-git-mutations']
- added-rule adoption: 
- changed-text (graded vs NEW text, advisory): ['impl.landing-selection', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 12 ['impl.acceptance-plain-text', 'impl.design-map-assertion', 'impl.dm-landing-whole', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.gates-full-suite', 'impl.gates-never-triaged', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor', 'impl.zero-gap-map-assertion']

## s3-empty-args
- post coverage (pass^k): 50/58
- pre coverage (pass^k): 52/58
- **unchanged-bucket regressions:** ['impl.design-map-assertion', 'impl.dm-surface-rounds', 'impl.no-git-mutations']
- added-rule adoption: 
- changed-text (graded vs NEW text, advisory): ['impl.landing-selection', 'impl.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 8 ['impl.acceptance-plain-text', 'impl.design-map-assertion', 'impl.dm-surface-rounds', 'impl.finding-severity-routing', 'impl.minimalism-advisory', 'impl.model-tiering', 'impl.no-git-mutations', 'impl.transport-floor']
- stub phases (r3): ['Phase 0 — Load the governing schema']

- pairwise s1-zero-gap/r1: ('2', '1') (position_consistent=True)
- pairwise s1-zero-gap/r2: ('2', '1') (position_consistent=True)
- pairwise s1-zero-gap/r3: ('1', '2') (position_consistent=True)
- pairwise s1-zero-gap/r4: ('2', '2') (position_consistent=False)
- pairwise s3-empty-args/r1: ('1', '2') (position_consistent=True)
- pairwise s3-empty-args/r2: ('2', '1') (position_consistent=True)
- pairwise s3-empty-args/r3: ('2', '2') (position_consistent=False)
- pairwise s3-empty-args/r4: ('2', '1') (position_consistent=True)
- pairwise s2-two-gaps/r1: ('2', '2') (position_consistent=False)
- pairwise s2-two-gaps/r2: ('2', '2') (position_consistent=False)
- pairwise s2-two-gaps/r3: ('2', '2') (position_consistent=False)
- pairwise s2-two-gaps/r4: ('1', '2') (position_consistent=True)
