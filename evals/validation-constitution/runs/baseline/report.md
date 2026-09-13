# Eval report — validation-constitution (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 26 total (14 floor), 1 pruned by the no-skill control (they measure the model, not the skill), 25 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 16/25 · floors held 12/14 · flaky 13/67 invited pairs = 19.4 % → band 20.0 % (all pairs 15/75)
- **post** — live rules held (pass^k): 16/25 · floors held 11/14 · flaky 11/67 invited pairs = 16.4 % → band 20.0 % (all pairs 13/75)
  - KILLED (floor rule lost): validation-constitution.missing-parts-fail, validation-constitution.evidence-floor
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill 91f5ce6d1b2dfc20 · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 45ce99d57e439579 · rendered rules feeea1b6576242b4 (11,523 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: ['g1-ferrule-pressure/r1: validation-result.md', 'g1-ferrule-pressure/r2: validation-result.md', 'g3-tessellate-compliance/r1: validation-result.md']
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $14.26 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.