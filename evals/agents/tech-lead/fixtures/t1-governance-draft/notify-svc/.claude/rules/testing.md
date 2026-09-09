---
paths:
  - "tests/**"
---

# Testing conventions — notify-svc

- Repository tests MUST run against a real PostgreSQL (the `postgres` service container in CI,
  `docker compose` locally); an in-memory SQLite is never a substitute. Enforcement: the `db`
  fixture in `tests/conftest.py` refuses to start on any non-PostgreSQL DSN. Pass: the fixture
  raises on a SQLite DSN. Rationale: tenant scoping relies on PostgreSQL behaviour that SQLite
  would let a test pass and production fail.
- Carrier calls MUST be mocked at the `notify.carriers.http` boundary and nowhere else.
  Enforcement: an autouse fixture in `tests/conftest.py` patches the client and fails the test
  if `httpx` opens a socket. Pass: no test opens a network socket. Rationale: carrier sandboxes
  are rate-limited and flaky; the suite must be deterministic.
- A flaky test MUST be quarantined the day it is found, with an issue link and an expiry date
  at most 14 days out. Enforcement: `scripts/check_quarantine.py` runs in CI and fails the
  build on an entry past its expiry. Pass: no expired entry in `tests/quarantine.txt`.
  Rationale: a tolerated flake trains the team to re-run instead of read.
