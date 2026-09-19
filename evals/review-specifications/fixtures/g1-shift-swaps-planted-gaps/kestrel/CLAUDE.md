# Kestrel — Operating Manual

Rota and shift management for independent venues. Stack and team in `README.md`; decisions
in `docs/decisions/`; venue facts in `docs/`.

## Governance

Ratified 2026-05-14 · production floor · depth: high

### Principles

- **Every change to a published rota is attributable.** Once a week's rota is published, any
  change to who works which shift records who made it, when, and through what path (manager
  edit, swap, open-shift claim, time-off approval). Enforcement: the `rota_change` audit table
  is append-only; the rota model refuses a change to a published shift without an actor.
  Rationale: pay disputes and a licensing inspection both reconstruct who was on shift from
  this record — a rota change with no author cannot be defended.
- **Staff personal data is Confidential.** Phone numbers, home addresses, dates of birth, and
  right-to-work details are visible to managers only; a staff member sees colleagues' names
  and shifts, never their contact details. Enforcement: the staff-app API serialisers carry an
  allowlist; the contract suite asserts no phone number in any staff-app response.
  Rationale: UK GDPR, and a venue's staff are often teenagers.
- **The venue tablet works when the venue's wifi does not.** Clock-in and the day's rota must
  be usable on the tablet without a network, reconciling when it returns. Enforcement: the
  tablet app's contract suite runs its clock-in cases with the network stubbed out.
  Rationale: half our venues are Victorian buildings with one router.
- **Testing.** New behaviour ships with tests; coverage MUST NOT fall below 70 % (blocking in
  CI). Integration tests run against a real PostgreSQL.
