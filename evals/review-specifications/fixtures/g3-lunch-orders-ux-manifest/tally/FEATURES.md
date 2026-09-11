# Features

> The system as capabilities — the capability peer of the architecture store.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-002](.mochiko/features/FEAT-002-menu-publishing.md) | Menu publishing | in-flight | the kitchen publishes a week's menu with prices and allergens; hidden items never reach parents |
|  | ↳ `live` hide an item from parents while keeping it on the kitchen's own list | in menu-admin | hidden items are kitchen-only |
| [FEAT-001](.mochiko/features/FEAT-001-family-accounts.md) | Family accounts | delivered | a parent signs in and sees the children on their account with class and dietary needs |
| [FEAT-003](.mochiko/features/FEAT-003-wallet.md) | Wallet | delivered | a parent tops up a family wallet by card; meals are paid from it; funded meals cost nothing |
| [FEAT-004](.mochiko/features/FEAT-004-kitchen-counts.md) | Kitchen counts | proposed (unrefined) | the kitchen sees per-day counts per item with allergy flags |
