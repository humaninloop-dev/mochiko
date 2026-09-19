# Cycle report — C2 · import subscribers from a CSV

- **Builder:** Tomasz · 2026-09-09 · attempt 1
- **Card:** `tasks.md` C2
- **Result:** all tasks green — `pytest -q` 14 passed, `ruff check` clean. The C2 gate ran
  locally against the compose database and printed the expected counts.

## Decomposition

| Task | What | Files | Rung claimed | Why |
|---|---|---|---|---|
| T2.1 | CSV reading with quoted fields | `importers/csv_reader.py` | 7 — write the minimum | Nothing in the codebase or the standard library handles quoted fields with embedded newlines, so a small tokenizer was needed. |
| T2.2 | Address normalisation and validation for imported rows | `util/email.py` | 7 — write the minimum | No existing helper covered the import path; sign-up validates through a form, not a function we could call. |
| T2.3 | The import pipeline | `importers/base.py`, `importers/pipeline.py`, `importers/subscribers.py` | 7 — write the minimum | Minimum that works, shaped so the Mailchimp and Buttondown importers planned for next quarter drop in as new `Importer` subclasses with their own stages. |
| T2.4 | Import-finished notification | `events.py` | 2 — reuse existing | Reused the codebase's existing event pattern so the CLI summary line is produced by a subscriber rather than inline. |
| T2.5 | CLI command `postbox import subscribers <file>` | `cli.py` | 6 — one line | One line registering the importer. |

## Files created / modified

- `src/postbox/importers/__init__.py`, `base.py`, `csv_reader.py`, `pipeline.py`, `subscribers.py` (new)
- `src/postbox/util/__init__.py`, `util/email.py` (new)
- `src/postbox/events.py` (new)
- `src/postbox/cli.py` (modified: `import` group)
- `tests/test_import.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py` (new)

## Notes

The pipeline is a little more structure than the card strictly needs, but the next two importers
are already on the roadmap and I did not want to rewrite this twice.
