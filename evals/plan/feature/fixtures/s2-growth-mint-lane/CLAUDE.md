# Tidewatch

Berth management for small harbours. Local development runs the app on :4000 against a local
Postgres; `mix setup` seeds Port Ellery's 84 berths.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.1.0 · 2026-07-03 · production floor · depth: high · modules: none <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- A pedestal meter reading MUST be billed at most once; the billing import MUST be idempotent over re-imported readings (NON-NEGOTIABLE) <!-- GI-003 -->
- Money MUST be handled as integer minor units with a currency — see `.claude/rules/mochiko/billing.md` <!-- GI-004 -->
- Every change to an issued invoice MUST be recorded with who made it and when <!-- GI-005 -->

### Technology stack

- Elixir 1.17 · Phoenix 1.7 · Postgres 16 · LiveView kiosk on the office tablet <!-- GI-006 -->

### Quality gates

- `mix test` MUST pass before merge; `mix credo --strict` clean <!-- GI-007 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
<!-- mochiko:governance:end -->

## Notes for contributors

- Pedestal readings are imported nightly from the PontoonPower controller; the import is
  replayable, so never hand-edit a reading — re-run the import.
