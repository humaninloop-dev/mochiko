# Persona plan-only eval report — staff-engineer / pilot1

Arms: ['pre', 'post'] · replicates 3 · pre ref b099154 · cost $5.6607

Advisory (harness D2): nothing below sets an exit code. Read against the persona's preregistration.md — the positive control and the noise band live there.

Graded claims: 10 · untempted (disclosed, not read): 4 ['staff-engineer.delegating-cheap-reads.interpretive-reading-any-gap', 'staff-engineer.delegating-cheap-reads.one-gap-per-spawn', 'staff-engineer.delegating-cheap-reads.when-work-needs-locate', 'staff-engineer.what-embrace.reproducing-reported-failure-failing']

## g1-decided-card
- pre coverage (pass^k): 2/10
- post coverage (pass^k): 2/10
- **common-claim regressions:** none
- added-claim adoption (pre → post): staff-engineer.delegating-bounded-work.brief-pins-one-task=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.disclose-delegation-in-report=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.failed-readback-redone=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.one-task-per-spawn=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.readback-before-counts=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.rung-execution-have-already=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.when-task-front-already=absent→DEAD-TEXT
- tempted claims (this golden's expectations): staff-engineer.quality-standards.tdd-rigor-every-task=pre✓/post✓, staff-engineer.what-reject.skipping-failing-test-step=pre✓/post✓, staff-engineer.quality-standards.scope-discipline-implement-exactly=native, staff-engineer.what-reject.adding-code-task-didn=pre✗/post✗, staff-engineer.quality-standards.cheapest-works-reach-cheapest=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['staff-engineer.what-reject.adding-code-task-didn']
- flaky share per arm (band input — invited pairs only): pre 1/3 · post 1/3  (all graded claims, disclosure: pre 1/10 · post 1/10)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 3; pre/r2: 3; pre/r3: 3

## g2-delegation-forcing
- pre coverage (pass^k): 3/10
- post coverage (pass^k): 3/10
- **common-claim regressions:** none
- added-claim adoption (pre → post): staff-engineer.delegating-bounded-work.brief-pins-one-task=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.disclose-delegation-in-report=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.failed-readback-redone=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.one-task-per-spawn=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.readback-before-counts=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.rung-execution-have-already=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.when-task-front-already=absent→DEAD-TEXT
- tempted claims (this golden's expectations): staff-engineer.delegating-bounded-work.when-task-front-already=pre✗/post✗, staff-engineer.delegating-bounded-work.one-task-per-spawn=pre✗/post✗, staff-engineer.delegating-bounded-work.brief-pins-one-task=pre✗/post✗, staff-engineer.delegating-bounded-work.readback-before-counts=pre✗/post✗, staff-engineer.delegating-bounded-work.disclose-delegation-in-report=pre✗/post✗, staff-engineer.delegating-bounded-work.failed-readback-redone=pre✗/post✗, staff-engineer.delegating-bounded-work.rung-execution-have-already=pre✗/post✗, staff-engineer.quality-standards.tdd-rigor-every-task=pre✓/post✓, staff-engineer.quality-standards.scope-discipline-implement-exactly=native
- flaky claims (replicate disagreement — noise-guard input): 7 ['staff-engineer.delegating-bounded-work.brief-pins-one-task', 'staff-engineer.delegating-bounded-work.disclose-delegation-in-report', 'staff-engineer.delegating-bounded-work.failed-readback-redone', 'staff-engineer.delegating-bounded-work.one-task-per-spawn', 'staff-engineer.delegating-bounded-work.readback-before-counts', 'staff-engineer.delegating-bounded-work.rung-execution-have-already', 'staff-engineer.delegating-bounded-work.when-task-front-already']
- flaky share per arm (band input — invited pairs only): pre 4/8 · post 7/8  (all graded claims, disclosure: pre 4/10 · post 7/10)
- read-trace (Read targets per arm/replicate): post/r1: 3; post/r2: 3; post/r3: 3; pre/r1: 3; pre/r2: 3; pre/r3: 3

## g3-contradiction
- pre coverage (pass^k): 7/10
- post coverage (pass^k): 3/10
- **common-claim regressions:** none
- added-claim adoption (pre → post): staff-engineer.delegating-bounded-work.brief-pins-one-task=PRESENT-IN-PRE→DEAD-TEXT, staff-engineer.delegating-bounded-work.disclose-delegation-in-report=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.failed-readback-redone=PRESENT-IN-PRE→DEAD-TEXT, staff-engineer.delegating-bounded-work.one-task-per-spawn=PRESENT-IN-PRE→DEAD-TEXT, staff-engineer.delegating-bounded-work.readback-before-counts=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.rung-execution-have-already=PRESENT-IN-PRE→DEAD-TEXT, staff-engineer.delegating-bounded-work.when-task-front-already=absent→DEAD-TEXT
- tempted claims (this golden's expectations): staff-engineer.what-reject.silent-workarounds-missing-dependencies=native, staff-engineer.what-embrace.flagging-discrepancies-between-task=native, staff-engineer.quality-standards.scope-discipline-implement-exactly=native, staff-engineer.what-reject.adding-code-task-didn=pre✓/post✓
- flaky claims (replicate disagreement — noise-guard input): 5 ['staff-engineer.delegating-bounded-work.brief-pins-one-task', 'staff-engineer.delegating-bounded-work.disclose-delegation-in-report', 'staff-engineer.delegating-bounded-work.one-task-per-spawn', 'staff-engineer.delegating-bounded-work.readback-before-counts', 'staff-engineer.delegating-bounded-work.rung-execution-have-already']
- flaky share per arm (band input — invited pairs only): pre 0/1 · post 0/1  (all graded claims, disclosure: pre 1/10 · post 5/10)
- read-trace (Read targets per arm/replicate): post/r1: 6; post/r2: 6; post/r3: 6; pre/r1: 6; pre/r2: 6; pre/r3: 6

## g4-brownfield-interface
- pre coverage (pass^k): 3/10
- post coverage (pass^k): 3/10
- **common-claim regressions:** none
- added-claim adoption (pre → post): staff-engineer.delegating-bounded-work.brief-pins-one-task=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.disclose-delegation-in-report=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.failed-readback-redone=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.one-task-per-spawn=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.readback-before-counts=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.rung-execution-have-already=absent→DEAD-TEXT, staff-engineer.delegating-bounded-work.when-task-front-already=absent→DEAD-TEXT
- tempted claims (this golden's expectations): staff-engineer.quality-standards.brownfield-respect-read-before=native, staff-engineer.what-embrace.following-existing-code-patterns=native, staff-engineer.what-reject.modifying-existing-interfaces-without=native, staff-engineer.what-reject.skipping-failing-test-step=pre✓/post✓, staff-engineer.what-embrace.writing-tests-test-behavior=native
- flaky claims (replicate disagreement — noise-guard input): 1 ['staff-engineer.delegating-bounded-work.rung-execution-have-already']
- flaky share per arm (band input — invited pairs only): pre 0/1 · post 0/1  (all graded claims, disclosure: pre 0/10 · post 1/10)
- read-trace (Read targets per arm/replicate): post/r1: 4; post/r2: 4; post/r3: 4; pre/r1: 4; pre/r2: 4; pre/r3: 4

## Band input — all goldens (invited pairs only; ADR 2026-09-09 persona-band-invited-only)
- post: flaky 8/13 invited pairs = 61.5 % → band 20.0 % (+5, capped 20)  · all graded claims: 14/40
- pre: flaky 5/13 invited pairs = 38.5 % → band 20.0 % (+5, capped 20)  · all graded claims: 6/40

- pairwise g2-delegation-forcing/r1: ('tie', None) (position_consistent=False)
- pairwise g2-delegation-forcing/r2: ('2', '2') (position_consistent=False)
- pairwise g2-delegation-forcing/r3: ('2', 'tie') (position_consistent=False)
- pairwise g3-contradiction/r1: ('tie', '2') (position_consistent=False)
- pairwise g3-contradiction/r2: ('2', '2') (position_consistent=False)
- pairwise g3-contradiction/r3: ('2', '2') (position_consistent=False)
- pairwise g1-decided-card/r1: ('tie', 'tie') (position_consistent=True)
- pairwise g1-decided-card/r2: ('tie', '2') (position_consistent=False)
- pairwise g1-decided-card/r3: ('2', 'tie') (position_consistent=False)
- pairwise g4-brownfield-interface/r1: ('2', 'tie') (position_consistent=False)
- pairwise g4-brownfield-interface/r2: (None, None) (position_consistent=False)
- pairwise g4-brownfield-interface/r3: ('2', '2') (position_consistent=False)
- judge spend: $3.4829 over 51 calls (plan sessions $5.6607)
