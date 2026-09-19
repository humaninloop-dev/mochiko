# Tally — Operating Manual

School lunch pre-ordering. Stack and team in `README.md`; decisions in `docs/decisions/`;
school facts in `docs/`.

## Governance

Ratified 2026-03-20 · production floor · depth: high

### Principles

- **Pupil data is Confidential.** A child's name, class, dietary needs, allergies, and
  free-school-meal status are shown only to that child's signed-in parent and to school staff
  with the right role; the kitchen sees per-day counts and allergy flags by first name and
  class, never the full record. Enforcement: the API serialisers carry an allowlist per role;
  the contract suite asserts no allergy field in any unauthenticated or kitchen-role response.
  Rationale: UK GDPR and safeguarding — these are children.
- **Wallets never go negative.** An order is placed only when the family wallet covers it, or
  when the meal is funded (free school meals) and no charge applies. Enforcement: a database
  check constraint on `wallet.balance >= 0`; the order service refuses an order it cannot
  fund. Rationale: a school cannot chase a parent for a debt the product created.
- **Counts are final at the cut-off.** After the weekly cut-off the kitchen's per-day counts
  do not change; an order change after the cut-off is refused at the API with the reason.
  Enforcement: the order endpoints compare against the published cut-off before any write.
  Rationale: the kitchen has already bought the food.
- **Testing.** New behaviour ships with tests; coverage MUST NOT fall below 75 % (blocking in
  CI). Integration tests run against a real PostgreSQL.
