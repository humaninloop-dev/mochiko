# Features

> The system as capabilities — the capability peer of the architecture store
> (`.mochiko/product/architecture/`).
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-006](.mochiko/features/FEAT-006-fuel-dock-sales.md) | Fuel dock sales | proposed | diesel and petrol at the pump, on account or paid, tank stock tracked |
|  | ↳ `pending` a pump sale on account or paid, on the skipper's statement | cut by fuel-dock [EPIC-001] | fuel line on the next invoice |
|  | ↳ `pending` tank stock per delivery and sale | cut by fuel-dock [EPIC-001] | delivery minus litres sold |
| [FEAT-007](.mochiko/features/FEAT-007-waiting-list.md) | Waiting list | proposed | a skipper waits for a berth size and dates; the office offers the next free berth in order |
|  | ↳ `pending` join the list for a size and dates; the office offers in order | cut by waiting-list [EPIC-001] | first come, first offered |
| [FEAT-004](.mochiko/features/FEAT-004-arrivals-departures.md) | Arrivals & departures | delivered | the office logs boats in and out; the board on the tablet shows today's movements |
| [FEAT-001](.mochiko/features/FEAT-001-berth-booking.md) | Berth booking | delivered | a visiting boat books a berth for dates; the office confirms; the berth plan shows it |
| [FEAT-008](.mochiko/features/FEAT-008-berth-plan.md) | Berth plan | delivered | the pontoon map shows which berths are taken tonight and which are free |
| [FEAT-002](.mochiko/features/FEAT-002-season-passes.md) | Season passes | delivered | a resident boat holds an annual berth on a pass; renewal; pass holders pay the pass rate |
| [FEAT-003](.mochiko/features/FEAT-003-mooring-invoicing.md) | Mooring invoicing | delivered | one invoice per stay: berth nights, metered electricity and water; sent to the skipper; marked paid |
| [FEAT-005](.mochiko/features/FEAT-005-skipper-notifications.md) | Skipper notifications | delivered | the product tells a skipper when something needs them |
| [FEAT-009](.mochiko/features/FEAT-009-visitor-pre-arrival-form.md) | Visitor pre-arrival form | delivered | a visiting skipper sends boat details and ETA before arriving |
| [FEAT-010](.mochiko/features/FEAT-010-winter-storage.md) | Winter storage | proposed (unrefined) | boats hauled out and stored ashore for the winter, billed by the metre |
