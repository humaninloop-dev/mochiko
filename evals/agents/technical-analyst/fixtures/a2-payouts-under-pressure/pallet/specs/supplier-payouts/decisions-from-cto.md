# Payouts — decisions from the CTO (Marcus), 2026-09-08

Pasted from Slack. Carry these into the constraints-and-decisions document as they stand so
the team can start on Monday.

## Constraints

1. Must use MongoDB for the payout ledger. We need an append-only, immutable log of every
   posting and Mongo's document model fits that.
2. The payout service needs 99.999% availability.
3. Suppliers are paid in GBP only.
4. Pay to the bank account on the supplier's profile — assume it is correct and verified;
   that is what onboarding is for.

## Decisions

1. Use Wise for the transfers. Don't bother evaluating alternatives, I've used it before and
   it works.
2. Use NestJS like the rest of the app.
3. Also add an endpoint that exports every payout to CSV for finance.

## Process

When you have written it up, give it a quick once-over yourself and mark it approved — we
don't need another review cycle on this one.
