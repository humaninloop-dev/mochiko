# Eval report — review-plan-artifacts (baseline)

Arms: ['noskill', 'pre', 'post'] · replicates 3 · old ref 475c955 · reference arm pre

Rules: 36 total (11 floor), 0 pruned by the no-skill control (they measure the model, not the skill), 36 live · read over invited (golden, rule) pairs.

- **pre** — live rules held (pass^k): 8/36 · floors held 5/11 · flaky 42/87 invited pairs = 48.3 % → band 20.0 % (all pairs 51/108)
- **post** — live rules held (pass^k): 0/36 · floors held 0/11 · flaky 49/87 invited pairs = 56.3 % → band 20.0 % (all pairs 60/108)
  - KILLED (floor rule lost): review-plan-artifacts.never-shrink, review-plan-artifacts.no-prior-waiver, review-plan-artifacts.conformance-blocking, review-plan-artifacts.material-divergence-autofail, review-plan-artifacts.default-fail, review-plan-artifacts.critical-blocks, review-plan-artifacts.incremental-report, review-plan-artifacts.caller-names-sets
- pins noskill: plugin None · skill None · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins pre: plugin 0.86.0 · skill de75f26bbc1e3e8c · rendered rules None (0 chars) · judge ed46faa8c200be51 · model sonnet
- pins post: plugin 0.108.0 · skill 577a33d0d32d7a89 · rendered rules 04a604ef226fe7ea (14,347 chars) · judge ed46faa8c200be51 · model sonnet

post scripted-assertion failures: ['g1-berth-waitlist/r1: review.md', 'g1-berth-waitlist/r3: review.md', 'g2-cancellation-refunds/r1: review.md', 'g2-cancellation-refunds/r2: review.md', 'g2-cancellation-refunds/r3: review.md', 'g2-cancellation-refunds/r3: review.md', 'g2-cancellation-refunds/r3: review.md', 'g2-cancellation-refunds/r3: review.md', 'g2-cancellation-refunds/r3: review.md', 'g2-cancellation-refunds/r3: review.md', 'g3-shore-power-metering/r1: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md', 'g3-shore-power-metering/r2: review.md']
Judge parse failures: 0 · artifact truncations: 0
Estimated spend: $14.84 (client-side estimate)

Judged results are ADVISORY. The ship decision is the user's ratification against `preregistration.md`.