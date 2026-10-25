# Error Patterns Reference

The project's error conventions for `contracts/api.yaml`. Standard HTTP semantics are assumed; this
file fixes the choices a contract must make the same way every time.

## Error response format

Every error response uses one schema, defined once under `components/schemas` and referenced
everywhere:

```yaml
ErrorResponse:
  type: object
  required: [code, message]
  properties:
    code:
      type: string
      description: Machine-readable error code in SCREAMING_SNAKE_CASE
      example: VALIDATION_ERROR
    message:
      type: string
      description: Human-readable message
      example: One or more fields are invalid
    details:
      type: object
      additionalProperties: true
      description: Additional error context
```

Field-level validation failures list each field under `details.fields`:

```json
{
  "code": "VALIDATION_ERROR",
  "message": "One or more fields are invalid",
  "details": {
    "fields": [
      {"field": "email", "error": "Invalid email format"},
      {"field": "password", "error": "Must be at least 8 characters"}
    ]
  }
}
```

## Status codes

| Status | Use for |
|--------|---------|
| 400 | Malformed input or a field that fails validation |
| 401 | Missing, invalid or expired authentication |
| 403 | Authenticated, but not permitted |
| 404 | The resource does not exist |
| 405 | The method is not supported on this path |
| 409 | A conflict with current state: duplicates, version conflicts |
| 410 | The resource was permanently deleted |
| 422 | Well-formed input that breaks a business rule |
| 429 | Rate limit or quota exceeded; the response carries `Retry-After` |
| 500 | Unexpected server error; never return the stack trace |
| 502 / 504 | An upstream system failed or timed out |
| 503 | The service is temporarily unavailable |

400 versus 422 is the split contracts most often blur: 400 for the shape of the input, 422 for a
rule the well-formed input breaks. Never return an error inside a 200.

## Error code naming

1. **SCREAMING_SNAKE_CASE**: `INVALID_EMAIL`, not `invalidEmail`.
2. **Be specific**: `EMAIL_ALREADY_EXISTS`, not just `CONFLICT`.
3. **Name the resource**: `USER_NOT_FOUND`, not just `NOT_FOUND`.
4. **Describe the problem**: `PASSWORD_TOO_WEAK`, not `INVALID_PASSWORD`.

## Per-endpoint error table

Document each endpoint's errors in this form:

```markdown
## Error Responses: POST /api/users

| Status | Code | Condition | Response Details |
|--------|------|-----------|------------------|
| 400 | INVALID_EMAIL | Email format invalid | `details.field: "email"` |
| 409 | EMAIL_ALREADY_EXISTS | Email already registered | `details.email: "user@..."` |
| 422 | TERMS_NOT_ACCEPTED | Terms acceptance flag not set | None |
| 429 | RATE_LIMITED | More than 5 attempts per minute | `details.retryAfter: 60` |
```
