# Eval report — review-specifications (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 4 · old ref 475c955 · reference arm pre

Rules: 30 total (8 floor), 0 pruned by the no-skill control (they measure the model, not the skill), 30 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 17/30 · floors held 4/8 · flaky 14/79 invited pairs = 17.7 % → band 20.0 % (all pairs 17/90)
- **post** — live rules held (pass^k): 18/30 · floors held 8/8 · flaky 18/79 invited pairs = 22.8 % → band 20.0 % (all pairs 27/90)
  - 5 rules lost vs pre: review-specifications.complete-coverage, review-specifications.sf-legal-shapes, review-specifications.feature-critical-checks, review-specifications.sf-important-checks, review-specifications.clarifications-shape
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill bd74d7f82ea9ad7b · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 34ace842ec3ab3e6 · rendered rules 43dbb61cb4ee9e8c (12,502 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: ['g1-shift-swaps-planted-gaps/r3: review.md', 'g2-pause-subscription-pressure/r2: review.md', 'g2-pause-subscription-pressure/r2: review.md', 'g2-pause-subscription-pressure/r4: review.md']
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $18.76 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.