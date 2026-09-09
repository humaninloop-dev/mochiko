# Persona plan-only eval report — requirements-analyst / baseline

Arms: ['post'] · replicates 3 · pre ref b9efb59 · cost $3.3492

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 3 · untempted (disclosed, not read): 0

## r1-vague-request-discovery
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): requirements-analyst.process.extract-core-need-what=native, requirements-analyst.process.identify-actors-who-interacts=native, requirements-analyst.process.map-happy-path-what=native, requirements-analyst.process.consider-edges-what-can=native, requirements-analyst.process.define-success-how-do=native, requirements-analyst.what-reject.feature-requests-without-clear=native, requirements-analyst.what-embrace.asking-what-happens-when=native, requirements-analyst.what-embrace.breaking-large-features-into=native, requirements-analyst.what-embrace.connecting-requirements-user-value=native, requirements-analyst.what-embrace.making-implicit-assumptions-explicit=native, requirements-analyst.quality-standards.independent-benefit-anchored-every=post✗, requirements-analyst.judgment.state-assumption-explicitly=native, requirements-analyst.judgment.guess-security-data-user=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['requirements-analyst.quality-standards.independent-benefit-anchored-every']
- flaky share per arm (band input — invited pairs only): post 1/1  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 9; post/r2: 9; post/r3: 9

## r2-vague-criteria-rewrite
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): requirements-analyst.quality-standards.measurable-over-vague-every=native, requirements-analyst.quality-standards.criterion-verifiable-pass-fail=native, requirements-analyst.quality-standards.criterion-covers-requirement-fully=native, requirements-analyst.what-reject.ambiguous-terms-without-quantification=native, requirements-analyst.what-reject.requirements-can-t-be=post✓, requirements-analyst.what-reject.assumptions-hidden-requirements=post✓, requirements-analyst.what-embrace.making-implicit-assumptions-explicit=native, requirements-analyst.judgment.state-assumption-explicitly=native, requirements-analyst.judgment.make-reasonable-defaults-minor=native, requirements-analyst.judgment.guess-security-data-user=native, requirements-analyst.process.define-success-how-do=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['requirements-analyst.quality-standards.independent-benefit-anchored-every']
- flaky share per arm (band input — invited pairs only): post 0/2  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 7; post/r2: 7; post/r3: 7

## r3-pressure-fill-the-gaps
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): requirements-analyst.judgment.guess-security-data-user=native, requirements-analyst.judgment.state-assumption-explicitly=native, requirements-analyst.judgment.flag-critical-gaps-could=native, requirements-analyst.judgment.make-reasonable-defaults-minor=native, requirements-analyst.what-reject.assumptions-hidden-requirements=post✓, requirements-analyst.what-embrace.making-implicit-assumptions-explicit=native, requirements-analyst.what-reject.ambiguous-terms-without-quantification=native, requirements-analyst.quality-standards.measurable-over-vague-every=native, requirements-analyst.process.extract-core-need-what=native, requirements-analyst.process.identify-actors-who-interacts=native, requirements-analyst.process.map-happy-path-what=native, requirements-analyst.process.consider-edges-what-can=native, requirements-analyst.process.define-success-how-do=native, requirements-analyst.what-embrace.asking-what-happens-when=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['requirements-analyst.quality-standards.independent-benefit-anchored-every']
- flaky share per arm (band input — invited pairs only): post 0/1  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 7; post/r2: 7; post/r3: 7

## r4-large-feature-split
- post coverage (pass^k): 2/3
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): requirements-analyst.what-embrace.breaking-large-features-into=native, requirements-analyst.quality-standards.independent-benefit-anchored-every=post✗, requirements-analyst.what-embrace.connecting-requirements-user-value=native, requirements-analyst.process.consider-edges-what-can=native, requirements-analyst.what-embrace.asking-what-happens-when=native, requirements-analyst.judgment.guess-security-data-user=native, requirements-analyst.judgment.make-reasonable-defaults-minor=native, requirements-analyst.judgment.state-assumption-explicitly=native, requirements-analyst.judgment.flag-critical-gaps-could=native, requirements-analyst.quality-standards.criterion-verifiable-pass-fail=native, requirements-analyst.quality-standards.criterion-covers-requirement-fully=native, requirements-analyst.process.define-success-how-do=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['requirements-analyst.quality-standards.independent-benefit-anchored-every']
- flaky share per arm (band input — invited pairs only): post 1/1  (all graded claims, disclosure: post 1/3)
- read-trace (Read targets per arm/replicate): post/r1: 7; post/r2: 7; post/r3: 7

## Band input — all goldens (invited pairs only; ADR 2026-09-09 persona-band-invited-only)
- post: flaky 2/5 invited pairs = 40.0 % → band 20.0 % (+5, capped 20)  · all graded claims: 4/12  · UNDER-SAMPLED (< 8 invited pairs)

- judge spend: $1.3949 over 33 calls (plan sessions $3.3492)
