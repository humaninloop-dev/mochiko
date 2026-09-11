# Features

> The system as capabilities — the capability peer of the architecture store.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-004](.mochiko/features/FEAT-004-clock-in.md) | Clock-in | in-flight | staff clock in and out on the venue tablet; a manager corrects a time with the original kept |
|  | ↳ `live` offline clock-in with later reconciliation | in tablet-offline | tablet keeps clock-ins without wifi |
| [FEAT-001](.mochiko/features/FEAT-001-rota-publishing.md) | Rota publishing | delivered | a manager drafts a week's shifts by role and publishes; staff see the published week |
| [FEAT-002](.mochiko/features/FEAT-002-availability.md) | Availability | delivered | staff declare when they can work; the draft flags a shift outside it |
| [FEAT-003](.mochiko/features/FEAT-003-time-off.md) | Time off | delivered | staff ask for days off; a manager approves or declines; approved days block shifts |
| [FEAT-005](.mochiko/features/FEAT-005-open-shifts.md) | Open shifts | proposed (unrefined) | a manager offers an unfilled shift to eligible staff; the first to claim it takes it |
