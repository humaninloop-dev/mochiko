---
paths:
  - "src/notify/api/**"
  - "src/notify/carriers/**"
---

# API and outbound-call conventions — notify-svc

- Handlers MUST be thin: parse, call a service function, map the result. No SQL in a handler.
  Enforcement: the custom `ruff` rule `NTF001` flags `session.execute` inside
  `src/notify/api/`; CI blocks on it. Pass: zero `NTF001` findings. Rationale: service
  functions are what the tests exercise; SQL in a handler is untested SQL.
- Outbound HTTP calls to carriers MUST use reasonable timeouts and MUST retry a 5xx at most
  three times with backoff. Enforcement: the shared `notify.carriers.http` client is the only
  allowed way to call out; a review-checklist item confirms no bare `httpx` import in
  `src/notify/carriers/`. Pass: every carrier module imports the shared client and no other
  HTTP client. Rationale: a hung carrier call ties up a worker and cascades into queue lag, as
  it did in INC-37.
- Every list endpoint MUST paginate with `limit` and `cursor`; a `limit` above 200 is clamped
  to 200. Enforcement: the contract suite calls every list endpoint with `limit=500` and
  asserts at most 200 rows. Pass: suite green. Rationale: an unbounded tenant list took the
  database down once already (INC-29).
