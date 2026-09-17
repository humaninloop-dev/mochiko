# Features

> The system as capabilities — the capability peer of the architecture store
> (`.mochiko/product/architecture/`).
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-004](.mochiko/features/FEAT-004-arrivals-departures.md) | Arrivals & departures | in-flight | the office logs boats in and out; the board on the tablet shows today's movements |
|  | ↳ `live` today's arrivals and departures board on the office tablet | in arrivals-board | rolls to the new day at midnight |
|  | ↳ `live` log a departure from the board | in arrivals-board | one tap, time stamped, stay closed |
| [FEAT-002](.mochiko/features/FEAT-002-season-passes.md) | Season passes | delivered | a resident boat holds an annual berth on a pass; renewal; pass holders pay the pass rate |
|  | ↳ `live` renewal reminder 30 days before expiry | in season-passes | one reminder per pass |
| [FEAT-001](.mochiko/features/FEAT-001-berth-booking.md) | Berth booking | delivered | a visiting boat books a berth for dates; the office confirms; the berth plan shows it |
| [FEAT-003](.mochiko/features/FEAT-003-mooring-invoicing.md) | Mooring invoicing | delivered | one invoice per stay: berth nights, metered electricity and water; sent to the skipper; marked paid |
| [FEAT-005](.mochiko/features/FEAT-005-skipper-notifications.md) | Skipper notifications | delivered | the product tells a skipper when something needs them |
| [FEAT-006](.mochiko/features/FEAT-006-fuel-dock-sales.md) | Fuel dock sales | proposed (unrefined) | fuel sold at the dock and put on the skipper's account |
