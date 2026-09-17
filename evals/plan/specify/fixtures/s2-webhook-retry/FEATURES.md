# Features

> The system as capabilities — the capability peer of the architecture store
> (`.mochiko/product/architecture/`).
> Entries: one file per capability under `.mochiko/features/` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-003](.mochiko/features/FEAT-003.md) | Alert delivery | delivered | Tell the team about an incident on the channels they chose |
|  | ↳ `pending` per-channel quiet hours | cut by alert-channels | A channel that stays silent overnight |
| [FEAT-002](.mochiko/features/FEAT-002.md) | Incident timeline | delivered | Every incident keeps an ordered record of what happened |
| [FEAT-001](.mochiko/features/FEAT-001.md) | Monitor checks | delivered | Probe an endpoint on a schedule and open an incident when it fails |
