# Architecture — notely (derived index)

Derived from the architecture store at `.mochiko/product/architecture/spine.md` — do
not hand-edit; edit the store and re-render.

- api-service (container, ruled) · notes-db (SQLite datastore, ruled)
- AX-001 persistence (ruled) · AX-002 logging (ruled) · AX-003 auth (n-a, trigger:
  network exposure)
