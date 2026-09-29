# Evidence from uncommitted work cites the base commit plus a diff fingerprint — joint-build seam R10

**Date:** 2026-09-29
**Status:** ruled (user) 2026-09-29, "as recommended", at the wave-3 plan approval of the joint build
(branch `joint-hook-delta`); **built 2026-09-29 at plugin v0.116.0** (wave 3, migrations 0036–0039)
**Driver:** the seams record's "smaller two"
([2026-09-29-joint-hook-delta-build-seams.md](2026-09-29-joint-hook-delta-build-seams.md), routed items),
put to the user at the wave-3 plan approval as that record said.

## Context

The hook field review's D4 has every report cite the command and commit that reproduce its evidence,
never a "full log" path. During a run the work being tested is often uncommitted, so there is no
commit to cite, and D4 gives no fallback. The delta record's V1 has one for a different case (the
checkpoint table stands in for a missing sign-off commit), which does not identify a working state.

## Decision

**R10 —** A report whose evidence comes from uncommitted work cites the run's pinned base commit
(delta D3c, the run-open pin) plus a fingerprint of the uncommitted change against it — for example
`base 5558fd7 + working diff sha256:3f9a1c…` from `git diff 5558fd7 | shasum -a 256` — with the
command that produced the evidence. No seat commits to make a citation possible. How the fingerprint
also covers files the run added but git does not yet track is the rule's wording to settle (wave 3).

## Rationale

It identifies the exact working state without a git mutation (seats make none), without a lead
checkpoint commit in the middle of a run, and with the base the run already pins.

## Alternatives considered

- The lead makes a checkpoint commit before any report cites evidence (every citation a real commit;
  an extra step mid-run, and seats wait on the lead).
- The base commit only, plus the command (simplest; the tested state is not identified).
