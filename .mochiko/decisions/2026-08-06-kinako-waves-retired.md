# Kinako waves 1–2 investigation retired

**Status:** ruled (retired, user)
**Date:** 2026-08-06

## Context

The BACKLOG item "Waves 1–2 didn't land — investigate, then dogfood (residual B)"
(2026-07-24, Kinako follow-up run section) tracked a forensic obligation against the kinako
evidence repo: S4-era artifacts showed the v0.22/v0.23 dense forms absent, the design layer
~28% above the 555k baseline, and run-costs at 2/~15 rows. A candidate mechanism was found
2026-07-31 (stale plugin cache: 0.7.0/0.28.0/0.36.0 cached vs the labelled v0.38.0), with a
confirm + re-run + re-measure step still owed. It also sat as a ROADMAP *Now* row.

## Decision

Retired by user ruling 2026-08-06: the plugin has moved too far from the versions under
investigation. The waves 1–2 target surfaces were rebuilt or deleted across v0.44.0–v0.53.0
(doctrine purge waves, the v8 goal+harness rebuild, task-granularity dissolution); the
token-reduction wave-1/wave-2 forms the item would have confirmed no longer describe the
shipped library, and any re-run today would measure a different system. Item → trail as
CLOSED-RETIRED; the ROADMAP *Now* row (which pointed at exactly this work) drops.

Not retired with it: the other Kinako follow-up items (Cluster-2 ratification wave ·
artifact-filename collisions · domain-allowlist amend confirm) and the stale-plugin-cache
finding itself, which stays useful as recorded forensics (`plan-run-transport-forensics`
record) should cache staleness recur.

## Rationale

An investigation is worth carrying only while its target still exists. Confirm-the-forms +
re-measure against a library whose report/artifact layers have since been rebuilt twice
produces no actionable evidence; the acceptance checks it named (archive ROADMAP wave-1/2
trail entries) are provenance-only now.

## Alternatives considered

- **Keep open, re-scope to current version:** rejected — a re-measure of the current library
  is the OTel probe's job (token-epic D2 item), which is already open and better shaped.
- **Fold into the OTel probe item:** rejected — nothing left to fold; the cache-mechanism
  forensics are already recorded in `plan-run-transport-forensics`.
