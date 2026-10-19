# Pagination, Filtering, and Sorting Patterns

The project's conventions for list endpoints. Every list endpoint in a contract follows the same
parameter names and response envelope.

```
GET /api/users?page=1&limit=20&role=admin&sort=-createdAt
```

## Pagination

**Choose offset** for page-numbered UIs and small datasets. **Choose cursor** for large datasets,
real-time feeds and infinite scroll; deep offset pagination is the failure to avoid.

| Parameter | Rule |
|-----------|------|
| `page` | Offset style; 1-indexed; default 1 |
| `cursor` | Cursor style; an opaque string taken from the previous response |
| `limit` | Default 20, maximum 100; never unlimited |

Responses wrap results in `data` and put paging state in `pagination`:

```json
{
  "data": [...],
  "pagination": {"page": 2, "limit": 20, "total": 150, "totalPages": 8, "hasNext": true, "hasPrev": true}
}
```

Offset `pagination` requires `page`, `limit`, `total` and `totalPages`. Cursor `pagination` carries
`nextCursor` and `prevCursor` (each null at its end) and requires `hasNext` and `hasPrev`. Define
each meta object once under `components/schemas` (`PaginationMeta`, `CursorPaginationMeta`).

## Filtering

- **Equality** uses the field name: `?role=admin&status=active`. Enumerated fields declare their
  `enum` in the parameter schema.
- **Several values** are comma-separated: `?role=admin,user` (`style: form`, `explode: false`).
- **Operators** go in brackets after the field:

  | Operator | Meaning | Example |
  |----------|---------|---------|
  | none or `[eq]` | Equals | `status=active` |
  | `[ne]` | Not equals | `status[ne]=deleted` |
  | `[gt]` / `[gte]` | Greater than / or equal | `createdAt[gte]=2024-01-01` |
  | `[lt]` / `[lte]` | Less than / or equal | `amount[lt]=500` |
  | `[in]` / `[nin]` | In / not in list | `status[in]=active,pending` |

- **Free-text search** uses `search`, with `minLength: 2`.
- **An invalid filter value is rejected, never silently ignored**: 400 with code `INVALID_FILTER`
  and one entry per bad parameter in `details.errors` (`field`, `value`, `error`).

## Sorting

`sort=field` sorts ascending; a `-` prefix sorts descending (`sort=-createdAt`); several fields are
comma-separated in priority order (`sort=role,-createdAt`). Declare the default sort on every list
endpoint, and expose only fields that are indexed.
