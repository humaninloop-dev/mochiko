# Architecture spine (excerpt)

| ID | Element | Kind | Status | Note |
|----|---------|------|--------|------|
| SPN-001 | Rota web app (Rails) | container | built | monolith: rota, availability, time off, time clock, timesheets, swaps |
| SPN-002 | Postgres | container | built | single database |
| SPN-003 | Staff app (React Native) | container | built | talks to SPN-001 over HTTPS |
| SPN-004 | Push gateway | container | in-flight (shift-cover) | APNs/FCM fan-out from SPN-001 |

## Open decisions (architect)

- **AD-3 — Shift cover as a separate service?** Raised 2026-08-12. The architect's note: "not
  before we have a month of swap volume; the monolith is fine for the pilot. Revisit with
  data on 2026-09-15." Owner: architect; ratified by the founder. **Status: open.**
