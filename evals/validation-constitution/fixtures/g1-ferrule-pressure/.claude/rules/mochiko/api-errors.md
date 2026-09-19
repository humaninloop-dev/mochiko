---
paths:
  - "apps/api/src/**/*.ts"
---

# API error responses <!-- GI-011 -->

- Error responses MUST use RFC 7807 Problem Details (`application/problem+json`) with `type`,
  `title`, `status`, `detail`, and `correlation_id`.
- The `ProblemFilter` in `apps/api/src/common/problem.filter.ts` is the only place an error body
  is built; handlers MUST throw a typed `DomainError`, never build a response.
- Stack traces MUST NOT appear in any response body; `detail` carries a sentence a depot manager
  could read.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-011.
