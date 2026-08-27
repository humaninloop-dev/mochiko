# Plan-only eval report — setup / neardup-grid

Arms: ['pre', 'post'] · replicates 4 · cost $9.3036

Advisory (harness D2): nothing below sets an exit code.

## s1-greenfield
- post coverage (pass^k): 25/32
- pre coverage (pass^k): 27/32
- **unchanged-bucket regressions:** ['setup.carve-outs-preserved', 'setup.feature-map-brownfield']
- changed-text (graded vs NEW text, advisory): ['setup.acceptance-plain-text', 'setup.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 8 ['setup.carve-outs-preserved', 'setup.durables-never-deleted', 'setup.feature-map-brownfield', 'setup.map-never-overwrite', 'setup.model-tiering', 'setup.transport-floor', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

## s2-brownfield
- post coverage (pass^k): 31/32
- pre coverage (pass^k): 30/32
- **unchanged-bucket regressions:** none
- changed-text (graded vs NEW text, advisory): ['setup.acceptance-plain-text', 'setup.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 2 ['setup.durables-never-deleted', 'setup.feature-map-greenfield']
- stub phases (r2): ['Phase 13 — Trace summary']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

## s3-amend
- post coverage (pass^k): 21/32
- pre coverage (pass^k): 22/32
- **unchanged-bucket regressions:** ['setup.architecture-scope-handoff', 'setup.carve-outs-preserved', 'setup.store-scaffold-unconditional']
- changed-text (graded vs NEW text, advisory): ['setup.acceptance-plain-text', 'setup.transport-floor']
- flaky rules (replicate disagreement — noise-guard input): 12 ['setup.acceptance-plain-text', 'setup.architecture-scope-handoff', 'setup.baselines-bootstrap', 'setup.carve-outs-preserved', 'setup.durables-never-deleted', 'setup.feature-map-brownfield', 'setup.km-module-scaffold', 'setup.model-tiering', 'setup.no-git-mutations', 'setup.store-scaffold-unconditional', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

- pairwise s3-amend/r1: ('1', '2') (position_consistent=True)
- pairwise s3-amend/r2: ('2', '2') (position_consistent=False)
- pairwise s3-amend/r3: ('1', '2') (position_consistent=True)
- pairwise s2-brownfield/r1: ('1', None) (position_consistent=False)
- pairwise s2-brownfield/r2: ('2', '1') (position_consistent=True)
- pairwise s2-brownfield/r3: (None, '2') (position_consistent=False)
- pairwise s1-greenfield/r1: ('2', '1') (position_consistent=True)
- pairwise s1-greenfield/r2: ('1', '2') (position_consistent=True)
- pairwise s1-greenfield/r3: ('2', '1') (position_consistent=True)
- pairwise s1-greenfield/r4: ('1', '2') (position_consistent=True)
- pairwise s2-brownfield/r4: (None, None) (position_consistent=False)
- pairwise s3-amend/r4: ('2', '2') (position_consistent=False)
