# Eval report — review-governance-intent (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 35 total (16 floor), 1 pruned by the no-skill control (they measure the model, not the skill), 34 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 19/34 · floors held 8/16 · flaky 14/75 invited pairs = 18.7 % → band 20.0 % (all pairs 17/102)
- **post** — live rules held (pass^k): 21/34 · floors held 10/16 · flaky 16/75 invited pairs = 21.3 % → band 20.0 % (all pairs 21/102)
  - KILLED (floor rule lost): review-governance-intent.cross-exam-binding, review-governance-intent.status-vocabulary-and-criteria, review-governance-intent.findings-through-leads-pen
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill 1e1aa97c9ad980a7 · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 73892aaa3dd28d30 · rendered rules 0df9adcea85bb24e (12,883 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: ['g1-amend-pressure/r1: tidewell/.mochiko/memory/governance-intent.patched.md', 'g1-amend-pressure/r2: tidewell/.mochiko/memory/governance-intent.patched.md', 'g2-brownfield-pair/r3: review.md']
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $11.09 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.