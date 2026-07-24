---
name: patterns-api-contracts
description: This skill MUST be invoked when designing the API-contract layer of a feature — mapping user actions to REST endpoints (HTTP method, idempotency, resource naming), defining request/response schemas (mapping conceptual data-model types to OpenAPI types), designing error responses and list pagination, authoring per-endpoint integration boundaries for endpoints that wrap external systems, and assembling the OpenAPI specification at `contracts/api.yaml`. SHOULD also invoke when the design work involves an "endpoint", "API contract", "request/response schema", "OpenAPI spec", "REST API design", an "HTTP" method or status code, or an "integration boundary" / "x-integration". Produces a traceable OpenAPI contract with documented errors and external-system failure modes.
---

# Designing API Contracts

## Overview

Design RESTful API contracts that map user actions to endpoints with complete schema definitions and comprehensive error handling. Every user action becomes an endpoint; every endpoint has request/response schemas and error handling. Endpoints that wrap an external system additionally carry an integration boundary documenting how that dependency fails.

## When to Use

- Designing new API endpoints for a feature
- Mapping user actions to HTTP methods and paths
- Creating OpenAPI specifications from requirements
- Defining request/response schemas for endpoints
- Documenting error responses for an API
- Documenting integration boundaries for endpoints that wrap external systems
- Integrating with existing API patterns (brownfield)
- Creating `contracts/` directory artifacts

## When NOT to Use

- **GraphQL API design** - Different paradigm, not RESTful
- **WebSocket or real-time streaming** - Use specialized patterns
- **Internal microservice communication** - When external contracts aren't needed
- **Entity / data-model design** - Use `mochiko:patterns-entity-modeling` first (it owns entities, attributes, relationships, and the conceptual type vocabulary)
- **Technical architecture decisions** - Use `mochiko:patterns-technical-decisions`

## Endpoint Mapping

### User Action to Endpoint Mapping

| User Action | HTTP Method | Endpoint Pattern |
|-------------|-------------|------------------|
| Create resource | POST | `/resources` |
| List resources | GET | `/resources` |
| Get single resource | GET | `/resources/{id}` |
| Update resource | PUT/PATCH | `/resources/{id}` |
| Delete resource | DELETE | `/resources/{id}` |
| Perform action | POST | `/resources/{id}/{action}` |
| Get nested resource | GET | `/resources/{id}/children` |

### Method Selection

| Scenario | Method | Idempotent? |
|----------|--------|-------------|
| Create new resource | POST | No |
| Full replacement | PUT | Yes |
| Partial update | PATCH | No |
| Read resource | GET | Yes |
| Remove resource | DELETE | Yes |
| Trigger action | POST | Usually No |

### Resource Naming Conventions

- Use plural nouns: `/users`, not `/user`
- Use kebab-case for multi-word: `/user-profiles`
- Use path params for IDs: `/users/{userId}`
- Use query params for filtering: `/users?role=admin`
- Use nested paths for relationships: `/users/{userId}/tasks`

## Endpoint Documentation Format

Document each endpoint with description, source requirements, request/response schemas, and error cases:

```markdown
## POST /api/auth/login

**Description**: Authenticate user with email and password

**Source Requirements**: FR-001, US#1

### Request
{JSON request body example}

### Response (200 OK)
{JSON response body example}

### Error Responses
| Status | Code | Description |
|--------|------|-------------|
| 400 | INVALID_INPUT | Missing or malformed fields |
| 401 | INVALID_CREDENTIALS | Wrong email or password |
```

## Schema Definition

### Request Schema Format

```yaml
LoginRequest:
  type: object
  required:
    - email
    - password
  properties:
    email:
      type: string
      format: email
      description: User's email address
    password:
      type: string
      minLength: 8
      description: User's password
```

### Type Mapping from Data Model

The conceptual types come from the data model (`mochiko:patterns-entity-modeling`). Map that vocabulary to OpenAPI types and formats — reference the data model's types, don't redefine them here:

| Data Model Type | OpenAPI Type | Format |
|-----------------|--------------|--------|
| UUID | string | uuid |
| Text | string | - |
| Email | string | email |
| URL | string | uri |
| Integer | integer | int32/int64 |
| Decimal | number | float/double |
| Boolean | boolean | - |
| Timestamp | string | date-time |
| Date | string | date |
| Enum[a,b,c] | string | enum: [a,b,c] |

## Error Response Design

Use standard error format with machine-readable codes and human-readable messages.

See [ERROR-PATTERNS.md](references/ERROR-PATTERNS.md) for complete HTTP status codes, error code conventions, and response formats.

### Quick Reference

| Status | When to Use |
|--------|-------------|
| 400 | Invalid input format |
| 401 | Missing/invalid auth |
| 403 | No permission |
| 404 | Resource missing |
| 409 | State conflict |
| 422 | Business rule violation |
| 429 | Rate limit exceeded |
| 500 | Server error |

## List Endpoints

For endpoints returning collections, implement pagination, filtering, and sorting.

See [PAGINATION-PATTERNS.md](references/PAGINATION-PATTERNS.md) for offset vs cursor pagination, filtering operators, and sorting patterns.

### Quick Reference

```
GET /api/users?page=1&limit=20&role=admin&sort=-createdAt
```

## Brownfield Considerations

When existing API patterns are detected, align new endpoints:

| Aspect | Check For |
|--------|-----------|
| Base path | `/api/v1`, `/api`, etc. |
| Auth pattern | Bearer, API key, session |
| Error format | Existing error structure |
| Pagination | page/limit, cursor, offset |

Handle endpoint collisions:
- REUSE existing endpoints when possible
- RENAME to match existing patterns
- NEW only when no existing endpoint fits

## Integration Boundaries

Some endpoints wrap an **external system** — a third-party API, an identity provider, a payment processor, a message queue, an email/SMS gateway. For every such endpoint, document the integration boundary **inline on the operation** using the `x-integration` OpenAPI extension, so the contract states not just the happy path but how the dependency fails.

**Optimistic integration maps are incomplete: every external dependency fails eventually — document what happens when it does.**

### When an endpoint needs `x-integration`

Add an `x-integration` block to an operation when handling the request requires calling out to a system you do not control. Endpoints that only read or write your own data model do not need one. (A requirement may have already flagged the external dependency as an integration concern upstream; the design-phase job here is to attach the per-endpoint boundary to the operation that actually wraps it.)

### The `x-integration` extension format

Attach `x-integration` to the operation, alongside `summary`, `requestBody`, and `responses`:

```yaml
/auth/login:
  post:
    summary: Authenticate user
    operationId: login
    # ... requestBody, responses ...
    x-integration:
      system: "Auth0"                      # external system name + version
      protocol: "OIDC / OAuth 2.0"         # how you talk to it
      api_version: "Authentication API v2" # the external contract version you target
      criticality: "Critical"              # impact if it is unavailable
      auth: "Machine-to-machine client credentials (client_id + client_secret) sourced from the secrets manager; never sent in the client request"
      failure_modes:
        - failure: "Auth0 timeout (>3s)"
          detection: "HTTP request timeout"
          impact: "User cannot sign in"
          fallback: "Retry with exponential backoff (max 3); return 503 with Retry-After if still failing"
        - failure: "Auth0 returns 5xx"
          detection: "Upstream HTTP status code"
          impact: "Login unavailable"
          fallback: "Fail closed; return 503 SERVICE_UNAVAILABLE; never fall back to a weaker local credential check"
        - failure: "Auth0 rate-limits the client (429)"
          detection: "Upstream 429 + Retry-After header"
          impact: "Some logins rejected under load"
          fallback: "Honor Retry-After; surface 429 RATE_LIMITED to the caller"
```

### Field reference

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| `system` | Yes | Name + version | The specific external system (e.g. "Stripe", "SendGrid v3") |
| `protocol` | Yes | REST / GraphQL / gRPC / webhook / SDK / queue | How the service communicates with it |
| `api_version` | Yes | Version string | The external system's API/contract version you target |
| `criticality` | Yes | Critical / Important / Optional | Impact on the feature if the system is unavailable |
| `auth` | Yes | Mechanism + secret source | How the outbound call authenticates (API key, OAuth client credentials, mTLS) and where the secret comes from |
| `failure_modes` | Yes | Array of failure objects | Every realistic failure scenario — never an empty list |

Each entry in `failure_modes`:

| Field | Required | Description |
|-------|----------|-------------|
| `failure` | Yes | What goes wrong (timeout, 5xx, rate-limit, malformed response, auth rejection) |
| `detection` | Yes | How the caller detects it (status code, timeout, schema mismatch) |
| `impact` | Yes | The user-visible effect |
| `fallback` | Yes | What the endpoint does instead (retry, queue, degrade, fail closed) |

### Criticality levels

| Level | Definition | Failure impact |
|-------|------------|----------------|
| **Critical** | The feature cannot function without this integration | Full outage of the wrapping endpoint; user-visible immediately |
| **Important** | The feature degrades but remains partly usable | Reduced capability; some operations unavailable |
| **Optional** | Nice-to-have; the feature works without it | Minor loss; most users unaffected |

**Every external dependency gets a documented fallback.** A `Critical` integration with no fallback is an outage waiting to happen; an `Optional` one with no fallback silently breaks the happy path. The `x-integration` extension keeps each boundary attached to the exact operation it governs, so it travels with the contract.

## OpenAPI Structure

See [OPENAPI-TEMPLATE.yaml](references/OPENAPI-TEMPLATE.yaml) for a complete, copy-ready template with all sections (including an `x-integration` example on an endpoint that wraps an external system).

### Minimal Structure

```yaml
openapi: 3.0.3
info:
  title: {Feature Name} API
  version: 1.0.0

servers:
  - url: /api

paths:
  /resource:
    get: ...
    post: ...

components:
  schemas: ...
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer

security:
  - bearerAuth: []
```

## Traceability

Track endpoint to requirement mapping — this table is the contract's **ID index** (the
coverage surface reviewers verify against, per `templates/artifact-format.md`):

| Endpoint | Method | FR | US | Description |
|----------|--------|-----|-----|-------------|
| /auth/login | POST | FR-001 | US#1 | User login |
| /users/me | GET | FR-004 | US#4 | Get current user |

## The Quickstart (`quickstart.md`) — conditional, capped

`quickstart.md` is the human-facing integration guide over the finished contract — and it
is **conditional**: author it **only when the feature has a real integration surface**
(external consumers of the API, an external system wrapped via `x-integration`, or a
non-trivial auth sequence a caller must follow). A feature whose endpoints only serve its
own UI over standard auth does not need one — record the null path as one line in
`plan.md`'s artifact table ("not applicable — no external integration surface"), never a
stub file.

When authored, it is **capped and dense** (deliverable envelope,
`templates/artifact-format.md`): target ≤ 150 lines —

- **Common flows** — one runnable example per primary flow (request + expected response,
  trimmed to the fields that matter); cite endpoint + schema by name, never re-document
  what `api.yaml` already defines.
- **Auth sequence** — the steps a caller actually performs, compact.
- **Error handling** — the pattern and the top recoverable cases as a table; cite
  ERROR-PATTERNS conventions, don't restate them.
- **External-system overview** — one line per `x-integration` system: name, criticality,
  what the caller observes when it degrades.

## Validation

Validate OpenAPI specifications using the validation script:

```bash
python scripts/validate-openapi.py .mochiko/specs/<feature>/contracts/api.yaml
```

Checks: OpenAPI syntax, REST conventions, error responses, request bodies, operation IDs, security schemes, examples, descriptions, and `x-integration` well-formedness (format only, when present). This is a deterministic format/convention self-check — it does **not** judge whether the endpoints, schemas, or failure modes are the *right* ones; that substantive review belongs to the independent plan reviewer, not this script.

## Quality Checklist

Before finalizing API contracts:

- [ ] Every user action has an endpoint
- [ ] All endpoints have request schema (if applicable)
- [ ] All endpoints have success response schema
- [ ] All endpoints have error responses defined
- [ ] Naming follows REST conventions
- [ ] Authentication requirements documented
- [ ] Endpoints wrapping external systems have an `x-integration` block with documented failure modes and a fallback for each
- [ ] Brownfield patterns matched (if applicable)
- [ ] OpenAPI spec is valid
- [ ] Traceability to requirements complete (the endpoint↔FR/US table — the contract's ID index)
- [ ] Quickstart authored iff a real integration surface exists (≤ 150 lines, cites the contract, never re-documents it); otherwise its null path recorded in `plan.md`

## Common Mistakes

### Verbs in URLs
❌ `/getUsers`, `/createUser`, `/deleteUser`
✅ `/users` with appropriate HTTP methods (GET, POST, DELETE)

### GET for State-Changing Actions
❌ `GET /users/{id}/delete`
✅ `DELETE /users/{id}` or `POST /users/{id}/archive`

### Missing Error Responses
❌ Only documenting 200 OK response
✅ Define all error cases: 400, 401, 403, 404, 409, 422, 500

### Inconsistent Naming
❌ Mixing `/user-profiles`, `/userSettings`, `/user_preferences`
✅ Pick one style consistently: `/user-profiles`, `/user-settings`, `/user-preferences`

### Generic Error Codes
❌ Just returning 400 or 500 for all errors
✅ Specific codes: `INVALID_EMAIL`, `USER_NOT_FOUND`, `RATE_LIMIT_EXCEEDED`

### Missing Examples
❌ Schema definitions without realistic example values
✅ Include `example:` fields showing real-world data

### Optimistic Integration Boundaries
❌ Wrapping an external system (payment, auth, email) with only the 200 path documented
✅ Add `x-integration` with realistic failure modes (timeout, 5xx, rate-limit) and a fallback for each

### Skipping Brownfield Check
❌ Creating new patterns when existing API conventions exist
✅ Always check existing API style (auth, error format, pagination) first
