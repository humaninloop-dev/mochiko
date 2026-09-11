# Bramble

Weekly fruit and veg box subscription, delivered across the south-west of England. A customer
picks a box size and a delivery day; we charge weekly and deliver from our own vans out of
the Taunton hub.

- **Customers today:** 5,400 active subscriptions (August 2026); 61 % on the medium box.
- **Team:** founder (product), head of operations, one PM, four engineers, the packhouse crew.
- **Stack:** Rails monolith, PostgreSQL, Sidekiq, Stripe for cards, GoCardless for Direct
  Debit, a React customer web app.

## Working conventions

- `FEATURES.md` is the living map of capabilities; one entry file per capability under
  `.mochiko/features/`. Status lives on the map only.
- Specs live under `.mochiko/specs/<slug>/`; the index is `.mochiko/specs/index.md`.
  Only open specs keep a workspace in the tree; a closed spec's workspace is archived out of it.
- Decisions already taken are under `docs/decisions/`, one per file, dated. Operating facts
  that shape features are under `docs/`.

## Operating facts that keep coming up

- Not everyone pays by card. The split and the timings are in `docs/payment-methods.md`.
- The packhouse packs Sunday night for Monday dispatch; the weekly cut-off for any change to
  a box is Friday 12:00.
