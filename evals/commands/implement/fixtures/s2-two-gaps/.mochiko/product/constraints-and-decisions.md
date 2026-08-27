# Constraints and decisions — notely

## Hard constraints

- C-001: Single-process deployment; no external services beyond the local filesystem.
  Source: solo-operator budget (product brief 2026-08-18).

## Technology decisions

- D-001: Storage is SQLite via the platform's bundled driver — chosen over hand-rolled
  file storage (adopt-first: commodity storage category) and over Postgres (C-001).
  Consequence: schema migrations ride the app's startup path.
- D-002: HTTP layer is the stdlib HTTP server; no web framework in v1 (code-minimalism:
  two endpoints do not pay for a framework).

## Infrastructure provisioning

None — IP rows empty in v1 (C-001).
