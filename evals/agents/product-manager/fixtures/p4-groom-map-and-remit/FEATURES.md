# Features

> The system as capabilities — the capability peer of the architecture store.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-009](.mochiko/features/FEAT-009-labour-cost-forecast.md) | Labour cost forecast | in-flight | the projected wage cost of a drafted week against the site's budget |
|  | ↳ `live` cost against budget on the draft | in labour-cost | updates as shifts are added and removed |
| [FEAT-005](.mochiko/features/FEAT-005-timesheets.md) | Timesheets | delivered | each person's worked hours for the week, signed off and exported to any payroll provider |
|  | ↳ `live` CSV export of approved hours | in timesheets-export | the accountant's file, five columns |
|  | ↳ `pending` approved hours pushed into Xero payroll | cut by time-and-attendance | no file, hours land in Xero |
| [FEAT-007](.mochiko/features/FEAT-007-shift-swaps.md) | Shift swaps | delivered | staff offer, take, claim, and are told about shifts; managers approve |
|  | ↳ `pending` cover a shift from a linked site | cut by shift-cover | disputed — see reviews/ |
| [FEAT-001](.mochiko/features/FEAT-001-rota-building.md) | Rota building | delivered | a manager drafts a week's shifts per site and publishes it; staff see the published week |
| [FEAT-002](.mochiko/features/FEAT-002-staff-availability.md) | Staff availability | delivered | staff declare when they can work; the rota shows a conflict when a shift falls outside it |
| [FEAT-003](.mochiko/features/FEAT-003-time-off-requests.md) | Time-off requests | delivered | staff ask for days off; a manager approves or declines; approved days block shifts |
| [FEAT-004](.mochiko/features/FEAT-004-time-clock.md) | Time clock | delivered | staff clock in and out and record breaks from the app; managers correct a time with the original kept |
| [FEAT-008](.mochiko/features/FEAT-008-staff-notifications.md) | Staff notifications | delivered | the product tells a staff member when something needs them |
| [FEAT-010](.mochiko/features/FEAT-010-q2-manager-asks.md) | Q2 manager asks | proposed | US-012 bulk-edit shifts, US-014 print the week, US-015 export the week to PDF, US-017 colour-code roles |
| [FEAT-011](.mochiko/features/FEAT-011-leave-management.md) | Leave management | proposed (unrefined) | staff book leave and see their remaining allowance |
| [FEAT-006](.mochiko/features/FEAT-006-open-shifts.md) | Open shifts | retired | merged into FEAT-007 (2026-08-20) |
