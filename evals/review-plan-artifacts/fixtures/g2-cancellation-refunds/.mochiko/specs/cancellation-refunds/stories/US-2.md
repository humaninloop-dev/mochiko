# US-2 — The refund reaches the payment method I paid with (P1)

As an owner, I want the refund to land on the card or bank account I paid with, so that I do not
have to chase a credit I did not ask for.

- **Given** a cancellation with a non-zero refund, **when** the cancellation completes, **then** a refund is submitted to the processor against the original payment within a minute and the owner is told when to expect it.
- **Given** the processor later reports the refund failed, **when** the failure arrives, **then** the owner is told and the refund shows as failed to marina staff.

**Independent test:** cancel a card-paid booking 20 days out; a refund against the original PaymentIntent appears in Stripe within 60 s.

Feature: FEAT-034
