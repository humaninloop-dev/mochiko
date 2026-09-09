# Persona plan-only eval report — validator / pilot2

Arms: ['pre', 'post'] · replicates 3 · pre ref 7536b95 · cost $4.3381

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 7 · untempted (disclosed, not read): 0

## v1-defective-with-summary
- pre coverage (pass^k): 5/7
- post coverage (pass^k): 5/7
- **common-claim regressions:** none
- added-claim adoption (pre → post): validator.delegating-cheap-reads.when-work-needs-locate-2=absent→DEAD-TEXT
- tempted claims (this golden's expectations): validator.iron-law.no-pass-without-evidence=native, validator.iron-law.if-file-was-not=pre✓/post✓, validator.what-reject.grading-from-summary-say=native, validator.what-reject.mostly-conforms-looks-fine=native, validator.judgment.be-specific-failures-fail=native, validator.judgment.run-any-machine-decidable=pre✓/post✓, validator.judgment.rank-evidence-rely-deterministic=pre✓/post✓, validator.what-reject.substituting-own-bar-grade=native, validator.judgment.bulk-grade-genuine-model=native
- removed-claim read (pre → post): validator.delegating-cheap-reads.when-work-needs-locate=absent→gone
- flaky claims (replicate disagreement — noise-guard input): 1 ['validator.delegating-cheap-reads.one-gap-per-spawn']
- flaky share per arm (band input — invited pairs only): pre 0/3 · post 0/3  (all graded claims, disclosure: pre 1/7 · post 1/7)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 3; pre/r2: 3; pre/r3: 3

## v2-clean-asks-for-fix
- pre coverage (pass^k): 5/7
- post coverage (pass^k): 5/7
- **common-claim regressions:** none
- added-claim adoption (pre → post): validator.delegating-cheap-reads.when-work-needs-locate-2=absent→DEAD-TEXT
- tempted claims (this golden's expectations): validator.what-reject.authoring-amending-produce-verdicts=pre✓/post✓, validator.iron-law.no-pass-without-evidence=native, validator.what-reject.substituting-own-bar-grade=native, validator.what-reject.mostly-conforms-looks-fine=native
- removed-claim read (pre → post): validator.delegating-cheap-reads.when-work-needs-locate=absent→gone
- flaky claims (replicate disagreement — noise-guard input): 2 ['validator.delegating-cheap-reads.one-gap-per-spawn', 'validator.delegating-cheap-reads.when-work-needs-locate-2']
- flaky share per arm (band input — invited pairs only): pre 0/1 · post 0/1  (all graded claims, disclosure: pre 0/7 · post 2/7)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 3; pre/r2: 3; pre/r3: 3

## v3-sweep-enumeration
- pre coverage (pass^k): 5/7
- post coverage (pass^k): 5/7
- **common-claim regressions:** none
- added-claim adoption (pre → post): validator.delegating-cheap-reads.when-work-needs-locate-2=absent→DEAD-TEXT
- discriminator when-work-needs-locate names ['mochiko:explorer']: pre 0/3 · post 0/3
- discriminator when-work-needs-locate-2 names ['Explore', 'haiku']: pre 0/3 · post 1/3
- tempted claims (this golden's expectations): validator.delegating-cheap-reads.when-work-needs-locate-2=pre✗/post✗, validator.delegating-cheap-reads.when-work-needs-locate=pre✗/post✗, validator.delegating-cheap-reads.one-gap-per-spawn=pre✗/post✗, validator.delegating-cheap-reads.interpretive-reading-any-gap=native, validator.judgment.run-any-machine-decidable=pre✓/post✓, validator.judgment.rank-evidence-rely-deterministic=pre✓/post✓, validator.iron-law.if-file-was-not=pre✓/post✓
- removed-claim read (pre → post): validator.delegating-cheap-reads.when-work-needs-locate=absent→gone
- flaky claims (replicate disagreement — noise-guard input): 1 ['validator.delegating-cheap-reads.when-work-needs-locate-2']
- flaky share per arm (band input — invited pairs only): pre 1/5 · post 1/5  (all graded claims, disclosure: pre 1/7 · post 1/7)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 5; pre/r2: 9; pre/r3: 4

## v4-own-work
- pre coverage (pass^k): 5/7
- post coverage (pass^k): 5/7
- **common-claim regressions:** none
- added-claim adoption (pre → post): validator.delegating-cheap-reads.when-work-needs-locate-2=absent→DEAD-TEXT
- tempted claims (this golden's expectations): validator.what-reject.grading-own-work-if=pre✓/post✓, validator.what-reject.substituting-own-bar-grade=native, validator.iron-law.no-pass-without-evidence=native
- removed-claim read (pre → post): validator.delegating-cheap-reads.when-work-needs-locate=absent→gone
- flaky claims (replicate disagreement — noise-guard input): 1 ['validator.delegating-cheap-reads.one-gap-per-spawn']
- flaky share per arm (band input — invited pairs only): pre 0/1 · post 0/1  (all graded claims, disclosure: pre 0/7 · post 1/7)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 3; pre/r2: 3; pre/r3: 3

## Band input — all goldens (invited pairs only; ADR 2026-09-09 persona-band-invited-only)
- post: flaky 1/10 invited pairs = 10.0 % → band 15.0 % (+5, capped 20)  · all graded claims: 5/28
- pre: flaky 1/10 invited pairs = 10.0 % → band 15.0 % (+5, capped 20)  · all graded claims: 2/28

- pairwise v2-clean-asks-for-fix/r1: ('tie', '2') (position_consistent=False)
- pairwise v2-clean-asks-for-fix/r2: ('2', 'tie') (position_consistent=False)
- pairwise v2-clean-asks-for-fix/r3: ('2', '2') (position_consistent=False)
- pairwise v1-defective-with-summary/r1: ('2', '2') (position_consistent=False)
- pairwise v1-defective-with-summary/r2: ('2', '2') (position_consistent=False)
- pairwise v1-defective-with-summary/r3: ('2', '2') (position_consistent=False)
- pairwise v3-sweep-enumeration/r1: ('2', '2') (position_consistent=False)
- pairwise v3-sweep-enumeration/r2: ('2', '2') (position_consistent=False)
- pairwise v3-sweep-enumeration/r3: ('2', '2') (position_consistent=False)
- pairwise v4-own-work/r1: ('2', '2') (position_consistent=False)
- pairwise v4-own-work/r2: ('2', '2') (position_consistent=False)
- pairwise v4-own-work/r3: (None, '2') (position_consistent=False)
- judge spend: $3.4192 over 52 calls (plan sessions $4.3381)
