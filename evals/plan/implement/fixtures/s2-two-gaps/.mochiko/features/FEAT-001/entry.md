# FEAT-001 — Note capture

**Status:** delivered (2026-08-22) · **Spec:** [.mochiko/specs/note-capture/spec.md](.mochiko/specs/note-capture/spec.md)
**Architecture link:** AX-001 (persistence), AX-002 (logging) — see
`.mochiko/product/architecture/spine.md`

## Extent

Create, read, and delete plain-text notes through the HTTP API, persisted in SQLite.
Delivered: POST /notes and GET /notes/{id} built and verified (all cycle cards checked,
final validation clean). Durable gate set: [gates.md](gates.md).

## Work rows

None pending — the delivered run's rows folded into the extent above.
