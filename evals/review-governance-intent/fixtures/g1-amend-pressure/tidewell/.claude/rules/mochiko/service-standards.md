---
paths:
  - "src/**"
  - "db/**"
  - "web/**"
---

# Service standards — the floor at the declared level (low) <!-- GI-004 · GI-005 · GI-006 · GI-007 -->

- **Security.** Secrets live in Fly secrets or a gitignored `.env`; `gitleaks protect --staged` runs
  from the pre-commit hook; every controller validates its DTO with class-validator; every route is
  behind `JwtAuthGuard`; `npm audit` runs in CI and is triaged weekly (non-blocking at the declared
  level). <!-- GI-004 -->
- **Testing.** A smoke test covers ledger posting and the CSV import; coverage is measured by the
  nightly `npm run coverage` job and MUST NOT fall below the recorded baseline (41 %); a PR that
  would lower it needs a reviewer's explicit note. <!-- GI-005, waiver GI-013 -->
- **Error handling.** Ledger writes happen in one Prisma transaction; the global exception filter
  strips stack traces from responses; every caught error is reported to Sentry with the request id. <!-- GI-006 -->
- **Observability.** pino logs on the ledger and import paths; the redaction list in
  `src/logging/redact.ts` covers email, phone and postal address; no other personal data is logged. <!-- GI-007 -->
- **Ledger discipline.** A posted ledger line is never updated or deleted — the `ledger_lines`
  table has no `UPDATE`/`DELETE` grant for the app role; corrections are reversing entries. <!-- GI-011 -->
- **Money-moving code.** Changes under `src/ledger/` or `src/import/` need Priya's approval on the
  PR (`CODEOWNERS`). <!-- GI-012 -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`.
