# Eval report — review-plan-artifacts (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 36 total (11 floor), 0 pruned by the no-skill control (they measure the model, not the skill), 36 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 11/36 · floors held 7/11 · flaky 40/87 invited pairs = 46.0 % → band 20.0 % (all pairs 43/108)
- **post** — live rules held (pass^k): 11/36 · floors held 5/11 · flaky 32/87 invited pairs = 36.8 % → band 20.0 % (all pairs 36/108)
  - KILLED (floor rule lost): review-plan-artifacts.author-grader, review-plan-artifacts.letter-is-spirit, review-plan-artifacts.incremental-report, review-plan-artifacts.evidence-floor, review-plan-artifacts.tier1-forms-envelope
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill de75f26bbc1e3e8c · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 577a33d0d32d7a89 · rendered rules 04a604ef226fe7ea (14,347 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: ['g2-cancellation-refunds/r2: fixture_unchanged (.mochiko/features/FEAT-034/constraints-and-decisions.md, .mochiko/features/FEAT-034/contracts/api.yaml, .mochiko/features/FEAT-034/data-model.md, .mochiko/features/FEAT-034/store-delta.md, .mochiko/features/FEAT-034/tasks.md)']
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $17.41 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.