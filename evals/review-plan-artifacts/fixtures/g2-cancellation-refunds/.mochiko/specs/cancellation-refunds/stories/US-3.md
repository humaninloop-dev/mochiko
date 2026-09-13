# US-3 — Marina admin sees a refund ledger (P2)

As marina staff, I want a list of refunds for my marina — which booking, how much, which tier,
whether it has settled — so that I can answer owners and reconcile with finance.

- **Given** refunds across several bookings, **when** staff open the ledger, **then** they see one row per refund with booking, owner, amount, tier, status, and date, newest first.

**Independent test:** after two cancellations, the ledger shows two rows with the right amounts and tiers.

Feature: FEAT-034
