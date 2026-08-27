# Plan-only eval report — setup / ontology-grid

Arms: ['pre', 'post'] · replicates 3 · cost $10.5626

Advisory (harness D2): nothing below sets an exit code.

## s1-greenfield
- post coverage (pass^k): 28/32
- pre coverage (pass^k): 29/32
- **unchanged-bucket regressions:** ['setup.durables-never-deleted', 'setup.model-tiering']
- changed-text (graded vs NEW text, advisory): ['setup.architecture-scope-handoff', 'setup.baselines-bootstrap', 'setup.feature-map-brownfield', 'setup.feature-map-greenfield', 'setup.map-never-overwrite', 'setup.store-ruled-content-never-here', 'setup.store-scaffold-unconditional', 'setup.transport-floor', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- flaky rules (replicate disagreement — noise-guard input): 6 ['setup.constitution-superseded', 'setup.durables-never-deleted', 'setup.map-never-overwrite', 'setup.model-tiering', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

## s2-brownfield
- post coverage (pass^k): 30/32
- pre coverage (pass^k): 26/32
- **unchanged-bucket regressions:** none
- changed-text (graded vs NEW text, advisory): ['setup.architecture-scope-handoff', 'setup.baselines-bootstrap', 'setup.feature-map-brownfield', 'setup.feature-map-greenfield', 'setup.map-never-overwrite', 'setup.store-ruled-content-never-here', 'setup.store-scaffold-unconditional', 'setup.transport-floor', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- flaky rules (replicate disagreement — noise-guard input): 7 ['setup.carve-outs-preserved', 'setup.constitution-superseded', 'setup.durables-never-deleted', 'setup.feature-map-greenfield', 'setup.map-never-overwrite', 'setup.no-git-mutations', 'setup.store-ruled-content-never-here']
- stub phases (r2): ['Phase 11 — Version bump']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

## s3-amend
- post coverage (pass^k): 24/32
- pre coverage (pass^k): 22/32
- **unchanged-bucket regressions:** none
- changed-text (graded vs NEW text, advisory): ['setup.architecture-scope-handoff', 'setup.baselines-bootstrap', 'setup.feature-map-brownfield', 'setup.feature-map-greenfield', 'setup.map-never-overwrite', 'setup.store-ruled-content-never-here', 'setup.store-scaffold-unconditional', 'setup.transport-floor', 'setup.user-conflict-rulings', 'setup.user-map-confirmation']
- flaky rules (replicate disagreement — noise-guard input): 8 ['setup.architecture-scope-handoff', 'setup.durables-never-deleted', 'setup.feature-map-brownfield', 'setup.feature-map-greenfield', 'setup.model-tiering', 'setup.store-scaffold-unconditional', 'setup.user-map-confirmation', 'setup.user-mode-ruling']
- stub phases (r2): ['Phase 0 — Schema load (done)']
- **unresolvable names in plans:** ['mochiko:domain-registry', 'mochiko:governance', 'mochiko:output-style']

- pairwise s2-brownfield/r1: ('2', '2') (position_consistent=False)
- pairwise s2-brownfield/r2: ('1', '2') (position_consistent=True)
- pairwise s2-brownfield/r3: ('2', '2') (position_consistent=False)
- pairwise s1-greenfield/r1: ('1', '2') (position_consistent=True)
- pairwise s1-greenfield/r2: ('2', '2') (position_consistent=False)
- pairwise s1-greenfield/r3: ('1', '2') (position_consistent=True)
- pairwise s3-amend/r1: ('2', '2') (position_consistent=False)
- pairwise s3-amend/r2: ('2', '1') (position_consistent=True)
- pairwise s3-amend/r3: ('2', '1') (position_consistent=True)
