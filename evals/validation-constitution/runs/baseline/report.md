# Eval report — validation-constitution (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 26 total (14 floor), 1 pruned by the no-skill control (they measure the model, not the skill), 25 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 16/25 · floors held 11/14 · flaky 12/67 invited pairs = 17.9 % → band 20.0 % (all pairs 14/75)
- **post** — live rules held (pass^k): 18/25 · floors held 13/14 · flaky 9/44 invited pairs = 20.5 % → band 20.0 % (all pairs 10/50)
  - 1 rules lost vs pre: validation-constitution.not-for
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill 91f5ce6d1b2dfc20 · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 45ce99d57e439579 · rendered rules feeea1b6576242b4 (11,523 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: none
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $13.98 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.