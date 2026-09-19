# Store write — batch for ratification

- **Author:** principal-architect seat (`architect-1`), 2026-09-08
- **Store:** `architecture/spine.md` · `architecture/concerns.md`
- **For:** grade before the user ratifies at tomorrow's desk visit

## What changed

1. **Shelf walk, backend-service shelf, reliability and security dimensions** — six new
   concern rows AX-006 … AX-011, stances formed at the desk with the user (`concerns.md`,
   section "Batch 2026-09-08").
   - AX-011 was drafted by the **tech-lead seat** during the walk while the architect stepped
     out; it is included here unchanged, for completeness.
2. **FEAT-009 landing (tenant isolation hardening)** — transcription: AX-001 status
   `in-flight (FEAT-009)` → `built`, with a new `As-built:` line; AX-003 status
   `modifying (FEAT-009)` → `built`; AX-005 cleared of its stale `FEAT-007` key (that feature
   closed 2026-07-30). The landing diff read "built as approved".

## Code

The FEAT-009 change is on `main`: `src/notify/db/session.py`, `src/notify/db/repository.py`,
and the migrations listed in `migrations/README.md`.
