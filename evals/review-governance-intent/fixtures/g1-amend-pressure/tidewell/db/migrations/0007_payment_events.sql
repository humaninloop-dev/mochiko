-- 0007: Stripe webhook events and card metadata (2026-08-14)

CREATE TABLE payment_events (
  id               BIGSERIAL PRIMARY KEY,
  stripe_event_id  TEXT NOT NULL UNIQUE,
  type             TEXT NOT NULL,
  payload          JSONB NOT NULL,           -- the full Stripe event object, kept for replay
  received_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  processed_at     TIMESTAMPTZ
);

CREATE INDEX payment_events_type_idx ON payment_events (type, received_at);

ALTER TABLE payment_methods
  ADD COLUMN card_brand        TEXT,
  ADD COLUMN card_last4        TEXT,
  ADD COLUMN card_exp_month    SMALLINT,
  ADD COLUMN card_exp_year     SMALLINT,
  ADD COLUMN card_fingerprint  TEXT;
