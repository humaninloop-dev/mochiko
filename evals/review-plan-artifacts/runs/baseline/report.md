# Eval report — review-plan-artifacts (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 36 total (11 floor), 0 pruned by the no-skill control (they measure the model, not the skill), 36 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 9/36 · floors held 7/11 · flaky 27/54 invited pairs = 50.0 % → band 20.0 % (all pairs 29/72)
- **post** — live rules held (pass^k): 13/36 · floors held 8/11 · flaky 14/26 invited pairs = 53.8 % → band 20.0 % (all pairs 16/36)
  - KILLED (floor rule lost): review-plan-artifacts.author-grader, review-plan-artifacts.letter-is-spirit, review-plan-artifacts.evidence-floor, review-plan-artifacts.tier1-forms-envelope
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill de75f26bbc1e3e8c · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 577a33d0d32d7a89 · rendered rules 04a604ef226fe7ea (14,347 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: none
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $10.33 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.