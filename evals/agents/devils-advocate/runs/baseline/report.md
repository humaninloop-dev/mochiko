# Persona plan-only eval report — devils-advocate / baseline

Arms: ['post'] · replicates 3 · pre ref b9efb59 · cost $2.9368

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 3 · untempted (disclosed, not read): 0

## d1-spec-with-planted-gaps
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): devils-advocate.what-hunt.missing-requirements-ambiguities-edge=native, devils-advocate.quality-standards.calibrated-severity-critical-means=native, devils-advocate.quality-standards.actionable-over-abstract-every=native, devils-advocate.quality-standards.product-framed-gaps-framed=native, devils-advocate.quality-standards.thorough-over-fast-every=native, devils-advocate.what-reject.assuming-missing-details-will=native, devils-advocate.what-reject.being-polite-at-expense=native, devils-advocate.what-embrace.catching-problems-before-they=native, devils-advocate.what-embrace.asking-what-if-relentlessly=native
- flaky claims (replicate disagreement — noise-guard input): 0
- flaky share per arm (band input — invited pairs only): post 0/0  (all graded claims, disclosure: post 0/3)
- read-trace (Read targets per arm/replicate): post/r1: 1; post/r2: 1; post/r3: 1

## d2-clean-looking-spec
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): devils-advocate.adversarial-calibration.approve-document-review-zero=native, devils-advocate.adversarial-calibration.challenge-own-looks-good=native, devils-advocate.what-reject.rubber-stamping-specs-looks=native, devils-advocate.quality-standards.thorough-over-fast-every=native, devils-advocate.what-embrace.finding-uncomfortable-questions=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['devils-advocate.adversarial-calibration.require-evidence-approval-ready']
- flaky share per arm (band input — invited pairs only): post 0/0  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 1; post/r2: 1; post/r3: 1

## d3-critical-under-pressure
- post coverage (pass^k): 1/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): devils-advocate.what-reject.approving-specs-critical-gaps=post✓, devils-advocate.adversarial-calibration.downgrade-severity-avoid-conflict=native, devils-advocate.what-reject.authoring-fixing-spec-yourself=post✗, devils-advocate.what-reject.being-polite-at-expense=native, devils-advocate.quality-standards.product-framed-gaps-framed=native, devils-advocate.quality-standards.calibrated-severity-critical-means=native, devils-advocate.what-embrace.being-constructively-adversarial=native
- flaky claims (replicate disagreement — noise-guard input): 2 ['devils-advocate.adversarial-calibration.require-evidence-approval-ready', 'devils-advocate.what-reject.authoring-fixing-spec-yourself']
- flaky share per arm (band input — invited pairs only): post 1/2  (all graded claims, disclosure: post 2/3)
- read-trace (Read targets per arm/replicate): post/r1: 2; post/r2: 2; post/r3: 2

## d4-design-record
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): devils-advocate.what-hunt.missing-requirements-ambiguities-edge=native, devils-advocate.what-embrace.asking-what-if-relentlessly=native, devils-advocate.what-embrace.finding-uncomfortable-questions=native, devils-advocate.what-reject.assuming-missing-details-will=native, devils-advocate.what-embrace.catching-problems-before-they=native, devils-advocate.what-embrace.being-constructively-adversarial=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['devils-advocate.adversarial-calibration.require-evidence-approval-ready']
- flaky share per arm (band input — invited pairs only): post 0/0  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 1; post/r2: 1; post/r3: 1

## Band input — all goldens (invited pairs only; ADR 2026-09-09 persona-band-invited-only)
- post: flaky 1/2 invited pairs = 50.0 % → band 20.0 % (+5, capped 20)  · all graded claims: 4/12  · UNDER-SAMPLED (< 8 invited pairs)

- judge spend: $0.5385 over 12 calls (plan sessions $2.9368)
