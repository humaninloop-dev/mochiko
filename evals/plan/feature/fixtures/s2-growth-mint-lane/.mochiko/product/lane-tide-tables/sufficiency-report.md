# Lane run — tide-tables

- **Status**: live (opened 2026-09-11)
- **Request**: tide times on the berth plan, the arrivals board, and the skipper messages are
  shown in UTC, not harbour local time; skippers plan departures against the wrong hour.
- **Touches**: FEAT-001 (the plan), FEAT-004 (the board), FEAT-005 (the messages)

## Sufficiency (three-clause form)

- Spec coverage: the harbour's local time zone is on the deployment (D-002); no story states
  the display zone — gap, closed in this run's design note.
- Baseline coverage: `.mochiko/product/contracts/api.yaml` returns ISO 8601 with offset; the kiosk formats — no
  contract change.
- Store coverage: no element change.

Verdict: sufficient with one design note. Card confirmed 2026-09-11; building.
