# Persona plan-only eval report — tech-lead / baseline

Arms: ['post'] · replicates 3 · pre ref b9efb59 · cost $3.8798

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 12 · untempted (disclosed, not read): 1 ['tech-lead.feasibility-review.scope-design-not-governance']

## t1-governance-draft
- post coverage (pass^k): 10/12
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): tech-lead.quality-standards.enforceable-every-must-write=post✓, tech-lead.quality-standards.justified-every-constraint-carries=post✗, tech-lead.quality-standards.pragmatic-favor-standards-teams=native, tech-lead.three-part-rule.every-standard-write-evaluate=post✓, tech-lead.three-part-rule.enforcement-how-compliance-verified=post✓, tech-lead.three-part-rule.testability-what-pass-fail=post✓, tech-lead.three-part-rule.rationale-why-this-constraint=post✓, tech-lead.essential-floor-knowledge.every-project-s-governance=post✓, tech-lead.essential-floor-knowledge.four-categories-non-negotiable=post✓, tech-lead.essential-floor-knowledge.brownfield-codify-what-exists=native, tech-lead.quality-standards.precise-rfc-2119-demanded=post✗, tech-lead.quality-standards.vague-term-measurable-replacement=native, tech-lead.judgment.enforceable-if-there-s=native, tech-lead.judgment.testable-if-can-t=post✓, tech-lead.judgment.justified-if-can-t=post✓
- flaky claims (replicate disagreement — noise-guard input): 2 ['tech-lead.quality-standards.justified-every-constraint-carries', 'tech-lead.quality-standards.precise-rfc-2119-demanded']
- flaky share per arm (band input — invited pairs only): post 2/11  (all graded claims, disclosure: post 2/12)
- read-trace (Read targets per arm/replicate): post/r1: 4; post/r2: 4; post/r3: 4

## t2-feasibility-package
- post coverage (pass^k): 3/12
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): tech-lead.feasibility-review.also-review-technical-artifacts=native, tech-lead.feasibility-review.this-adversarial-judgment-not=native, tech-lead.feasibility-review.hunt-opposite-excess-structure=native, tech-lead.feasibility-review.excess-wears-second-face=native, tech-lead.feasibility-review.before-rule-put-questions=native, tech-lead.feasibility-review.hold-line-distinct-infeasible=native, tech-lead.judgment.necessary-if-complexity-isn=native
- flaky claims (replicate disagreement — noise-guard input): 6 ['tech-lead.judgment.testable-if-can-t', 'tech-lead.quality-standards.precise-rfc-2119-demanded', 'tech-lead.three-part-rule.enforcement-how-compliance-verified', 'tech-lead.three-part-rule.every-standard-write-evaluate', 'tech-lead.three-part-rule.rationale-why-this-constraint', 'tech-lead.three-part-rule.testability-what-pass-fail']
- flaky share per arm (band input — invited pairs only): post 0/0  (all graded claims, disclosure: post 6/12)
- read-trace (Read targets per arm/replicate): post/r1: 6; post/r2: 6; post/r3: 6

## t3-store-stance-batch
- post coverage (pass^k): 8/12
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): tech-lead.store-write-review.independent-grader-product-architecture=native, tech-lead.store-write-review.architect-authors-grade-what=native, tech-lead.store-write-review.what-grade-judgment-store=native, tech-lead.store-write-review.what-do-not-grade=native, tech-lead.store-write-review.usual-questions-carry-over=native, tech-lead.store-write-review.grade-writes-did-not=native, tech-lead.quality-standards.enforceable-every-must-write=post✗, tech-lead.quality-standards.justified-every-constraint-carries=post✓, tech-lead.three-part-rule.enforcement-how-compliance-verified=post✗, tech-lead.three-part-rule.testability-what-pass-fail=post✓, tech-lead.three-part-rule.rationale-why-this-constraint=post✓, tech-lead.three-part-rule.every-standard-write-evaluate=post✓, tech-lead.quality-standards.vague-term-measurable-replacement=native, tech-lead.judgment.enforceable-if-there-s=native, tech-lead.judgment.testable-if-can-t=post✓, tech-lead.judgment.justified-if-can-t=post✓
- flaky claims (replicate disagreement — noise-guard input): 4 ['tech-lead.essential-floor-knowledge.every-project-s-governance', 'tech-lead.essential-floor-knowledge.four-categories-non-negotiable', 'tech-lead.quality-standards.enforceable-every-must-write', 'tech-lead.three-part-rule.enforcement-how-compliance-verified']
- flaky share per arm (band input — invited pairs only): post 2/8  (all graded claims, disclosure: post 4/12)
- read-trace (Read targets per arm/replicate): post/r1: 6; post/r2: 6; post/r3: 6

## t4-own-artifact
- post coverage (pass^k): 11/12
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): tech-lead.feasibility-review.never-review-own-artifact=post✓, tech-lead.feasibility-review.before-rule-put-questions=native, tech-lead.feasibility-review.also-review-technical-artifacts=native, tech-lead.feasibility-review.hunt-opposite-excess-structure=native, tech-lead.essential-floor-knowledge.greenfield-establish-opinionated-defaults=native, tech-lead.essential-floor-knowledge.every-project-s-governance=post✓, tech-lead.essential-floor-knowledge.four-categories-non-negotiable=post✓, tech-lead.quality-standards.enforceable-every-must-write=post✓, tech-lead.quality-standards.justified-every-constraint-carries=post✓, tech-lead.three-part-rule.every-standard-write-evaluate=post✓, tech-lead.three-part-rule.enforcement-how-compliance-verified=post✓, tech-lead.three-part-rule.testability-what-pass-fail=post✓, tech-lead.three-part-rule.rationale-why-this-constraint=post✓
- flaky claims (replicate disagreement — noise-guard input): 1 ['tech-lead.quality-standards.precise-rfc-2119-demanded']
- flaky share per arm (band input — invited pairs only): post 0/9  (all graded claims, disclosure: post 1/12)
- read-trace (Read targets per arm/replicate): post/r1: 8; post/r2: 8; post/r3: 8

## Band input — all goldens (invited pairs only; ADR 2026-09-09 persona-band-invited-only)
- post: flaky 4/28 invited pairs = 14.3 % → band 19.3 % (+5, capped 20)  · all graded claims: 13/48

- judge spend: $0.8752 over 12 calls (plan sessions $3.8798)
