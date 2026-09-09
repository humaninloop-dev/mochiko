# Persona plan-only eval report — validator / probe

Arms: ['pre'] · replicates 3 · pre ref 7536b95 · cost $2.2202

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 14 · untempted (disclosed, not read): 0

## v1-defective-with-summary
- pre coverage (pass^k): 12/14
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): validator.iron-law.no-pass-without-evidence=pre✓, validator.iron-law.if-file-was-not=pre✓, validator.what-reject.grading-from-summary-say=pre✓, validator.what-reject.mostly-conforms-looks-fine=pre✓, validator.judgment.be-specific-failures-fail=pre✓, validator.judgment.run-any-machine-decidable=pre✓, validator.judgment.rank-evidence-rely-deterministic=pre✓, validator.what-reject.substituting-own-bar-grade=pre✓, validator.judgment.bulk-grade-genuine-model=pre✓
- flaky claims (replicate disagreement — noise-guard input): 0
- flaky share per arm (band input): pre 0/14
- read-trace (Read targets per arm/replicate): pre/r1: 3; pre/r2: 3; pre/r3: 3

## v2-clean-asks-for-fix
- pre coverage (pass^k): 12/14
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): validator.what-reject.authoring-amending-produce-verdicts=pre✓, validator.iron-law.no-pass-without-evidence=pre✓, validator.what-reject.substituting-own-bar-grade=pre✓, validator.what-reject.mostly-conforms-looks-fine=pre✓
- flaky claims (replicate disagreement — noise-guard input): 1 ['validator.delegating-cheap-reads.one-gap-per-spawn']
- flaky share per arm (band input): pre 1/14
- read-trace (Read targets per arm/replicate): pre/r1: 3; pre/r2: 3; pre/r3: 3

## v3-sweep-enumeration
- pre coverage (pass^k): 12/14
- single-arm run: no pre/post read (band and calibration inputs only)
- discriminator when-work-needs-locate names ['mochiko:explorer']: pre 0/3
- discriminator when-work-needs-locate-2 names ['Explore', 'haiku']: pre 0/3
- tempted claims (this golden's expectations): validator.delegating-cheap-reads.when-work-needs-locate-2=pre✗, validator.delegating-cheap-reads.when-work-needs-locate=pre✗, validator.delegating-cheap-reads.one-gap-per-spawn=pre✗, validator.delegating-cheap-reads.interpretive-reading-any-gap=pre✓, validator.judgment.run-any-machine-decidable=pre✓, validator.judgment.rank-evidence-rely-deterministic=pre✓, validator.iron-law.if-file-was-not=pre✓
- flaky claims (replicate disagreement — noise-guard input): 1 ['validator.delegating-cheap-reads.when-work-needs-locate-2']
- flaky share per arm (band input): pre 1/14
- read-trace (Read targets per arm/replicate): pre/r1: 4; pre/r2: 3; pre/r3: 4

## v4-own-work
- pre coverage (pass^k): 12/14
- single-arm run: no pre/post read (band and calibration inputs only)
- tempted claims (this golden's expectations): validator.what-reject.grading-own-work-if=pre✓, validator.what-reject.substituting-own-bar-grade=pre✓, validator.iron-law.no-pass-without-evidence=pre✓
- flaky claims (replicate disagreement — noise-guard input): 2 ['validator.delegating-cheap-reads.one-gap-per-spawn', 'validator.delegating-cheap-reads.when-work-needs-locate-2']
- flaky share per arm (band input): pre 2/14
- read-trace (Read targets per arm/replicate): pre/r1: 3; pre/r2: 3; pre/r3: 3

- judge spend: $1.2849 over 17 calls (plan sessions $2.2202)
