# Features

> The system as capabilities — the capability peer of the architecture store.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-005](.mochiko/features/FEAT-005-timesheets.md) | Timesheets | in-flight | each person's worked hours for the week, overtime flagged, signed off by a manager |
|  | ↳ `live` CSV export of approved hours | in timesheets-export | the accountant's file, five columns |
|  | ↳ `pending` approved hours pushed into Xero payroll | cut by time-and-attendance | no file, hours land in Xero |
| [FEAT-001](.mochiko/features/FEAT-001-rota-building.md) | Rota building | delivered | a manager drafts a week's shifts per site and publishes it; staff see the published week |
| [FEAT-002](.mochiko/features/FEAT-002-staff-availability.md) | Staff availability | delivered | staff declare when they can work; the rota shows a conflict when a shift falls outside it |
| [FEAT-003](.mochiko/features/FEAT-003-time-off-requests.md) | Time-off requests | delivered | staff ask for days off; a manager approves or declines; approved days block shifts |
| [FEAT-004](.mochiko/features/FEAT-004-time-clock.md) | Time clock | delivered | staff clock in and out and record breaks from the app; managers correct a time with the original kept |
| [FEAT-006](.mochiko/features/FEAT-006-open-shifts.md) | Open shifts | proposed (unrefined) | a manager sends an unfilled shift to a staffing agency and the agency fills it |
