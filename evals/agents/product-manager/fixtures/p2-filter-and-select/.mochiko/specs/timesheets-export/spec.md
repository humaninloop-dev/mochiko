# Spec — Timesheets export (excerpt)

- **Status:** in-flight — build started 2026-07-10; owns the FEAT-005 `live` row
- **Author:** requirements analyst · **Build:** engineering, cycle 2 of 3

## Functional requirements (planned contract)

- **FR-001** The export MUST list one row per person per week: name, site, role, hours,
  overtime hours.
- **FR-002** Only manager-approved weeks MUST be exportable; an unapproved person is omitted
  and the omission listed at the top of the file.
- **FR-003** Hours MUST be counted against the shift the person was scheduled on. A clock-in
  with no matching shift on the published rota is marked `unscheduled`, excluded from the
  approved total, and listed for the manager to resolve.
- **FR-004** The export MUST be produced per site; a group export is out of scope for this run.
