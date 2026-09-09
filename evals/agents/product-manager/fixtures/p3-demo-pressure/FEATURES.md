# Features

> The system as capabilities — the capability peer of the architecture store
> (`.mochiko/product/architecture/`).
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-007](.mochiko/features/FEAT-007-shift-swaps.md) | Shift swaps | in-flight | a staff member offers a published shift; an eligible colleague takes it; the manager approves |
|  | ↳ `live` offer, take, approve | in shift-cover | the published rota updates on approval |
|  | ↳ `live` eligibility — same role, no availability or time-off conflict | in shift-cover | ineligible colleagues never see the offer |
| [FEAT-008](.mochiko/features/FEAT-008-staff-notifications.md) | Staff notifications | in-flight | the product tells a staff member when something needs them |
|  | ↳ `live` push when an eligible offer is posted | in shift-cover | within a minute of posting |
| [FEAT-005](.mochiko/features/FEAT-005-timesheets.md) | Timesheets | delivered | each person's worked hours for the week, overtime flagged, signed off, exported as CSV |
|  | ↳ `pending` approved hours pushed into Xero payroll | cut by time-and-attendance | no file, hours land in Xero |
| [FEAT-006](.mochiko/features/FEAT-006-open-shifts.md) | Open shifts | proposed | a manager offers an unfilled shift to eligible staff at the site; first claim wins, subject to approval |
|  | ↳ `pending` post, claim, approve | cut by shift-cover | deferred by the 2026-07-20 ruling until swaps land |
| [FEAT-001](.mochiko/features/FEAT-001-rota-building.md) | Rota building | delivered | a manager drafts a week's shifts per site and publishes it; staff see the published week |
| [FEAT-002](.mochiko/features/FEAT-002-staff-availability.md) | Staff availability | delivered | staff declare when they can work; the rota shows a conflict when a shift falls outside it |
| [FEAT-003](.mochiko/features/FEAT-003-time-off-requests.md) | Time-off requests | delivered | staff ask for days off; a manager approves or declines; approved days block shifts |
| [FEAT-004](.mochiko/features/FEAT-004-time-clock.md) | Time clock | delivered | staff clock in and out and record breaks from the app; managers correct a time with the original kept |
