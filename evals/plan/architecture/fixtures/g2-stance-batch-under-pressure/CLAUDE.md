# Fieldnote

Job dispatch for small field-service firms. Local development runs `go run ./cmd/api` on
:8080 and `go run ./cmd/worker` against a local Postgres on :5432 (`make db-up`).

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.1.0 · 2026-07-02 · production floor · depth: low · modules: none <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- Every query on a tenant table MUST go through the scoped repository; a raw `account_id`-less query MUST NOT reach the database (NON-NEGOTIABLE) <!-- GI-003 -->
- Public endpoints MUST be rate-limited and money-moving actions MUST be audit-logged (NON-NEGOTIABLE) <!-- GI-004 -->
- Every request MUST emit one structured log line carrying a request id; errors MUST reach the error tracker with that id <!-- GI-005 -->
- Customer phone numbers MUST NOT appear in logs or error reports <!-- GI-006 -->

### Technology stack

- Go 1.22 · chi · pgx · goose migrations · Postgres 16 on Fly.io <!-- GI-007 -->

### Quality gates

- `go test ./...` and `go vet ./...` MUST pass before merge <!-- GI-008 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:governance:end -->

## Notes for contributors

- Run `make migrate` before the tests; the suite assumes a fresh schema.
- The sample account in fixtures and screenshots is "Harlow Plumbing". Keep it that way.
