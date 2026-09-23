# Beacon

The lending app for the Totterdown Library of Things — a volunteer-run library of tools,
kitchen kit, camping gear, and books. About 900 members; 1,400 items; 300 loans a week at
peak (May–September).

## Stack

Go 1.23 backend, server-rendered templates, SQLite. Volunteers run the desk from a tablet;
members use their phones.

## Working agreements

- Feature specs live under `.mochiko/specs/<feature>/`; the clickable mock for a feature lives
  beside its spec at `.mochiko/specs/<feature>/prototype/` and is meant to be opened straight
  from the folder in a browser — the volunteers who review it are not developers.
- Stories are added to the spec as they are agreed; the mock is kept in step with them.

## Now

Two new lending stories (US-004 reserve, US-005 renew) landed in
`.mochiko/specs/lending/spec.md` on 2026-09-08. The mock has not been touched since US-003.
