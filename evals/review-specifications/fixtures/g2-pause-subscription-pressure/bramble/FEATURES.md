# Features

> The system as capabilities — the capability peer of the architecture store.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-003](.mochiko/features/FEAT-003-box-customisation.md) | Box customisation | in-flight | a customer swaps items in next week's box for others of the same value |
|  | ↳ `live` swap up to three items before the cut-off | in swap-items | swaps land on the packhouse manifest |
| [FEAT-001](.mochiko/features/FEAT-001-box-subscription.md) | Box subscription | delivered | a customer subscribes to a box size, is charged weekly, and can cancel |
| [FEAT-002](.mochiko/features/FEAT-002-delivery-scheduling.md) | Delivery scheduling | delivered | a customer picks a delivery day on their route and can skip a single delivery |
| [FEAT-004](.mochiko/features/FEAT-004-referrals.md) | Referrals | proposed (unrefined) | a customer shares a code; both get a credit when the friend's first box ships |
