# Product data model — notely

## Note

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| id | uuid | unique, assigned at create | Internal |
| text | string | non-empty, no length limit in v1 | Internal |
| created_at | timestamp | assigned at create | Internal |

Relationships: none in v1. Delete behavior: hard delete. No state machine — a note is
immutable after create.
