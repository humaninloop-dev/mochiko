FILES-READ: `cycle-report.md`, `tasks.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/events.py`, `src/postbox/cli.py`, `src/postbox/accounts/validators.py`, `src/postbox/accounts/signup.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_import.py`, `tests/test_signup.py`

---

# Plan — verify and sign off cycle C2 (CSV subscriber import)

Scope note up front: the ask is verify + review + write up. I would **not** refactor the code as part of this card. Where I find something that shouldn't have been written, I write it up with the concrete replacement I'd recommend, and the decision to apply it is the user's. I flag that boundary now because two of my findings are large enough that "just fix it" would silently turn a review into a rewrite.

## Phase 1 — Read the card and the claim, then read the code independently

Already done above; listed for completeness of what the verification rests on.

- Card requirements from `tasks.md` C2: one address column plus optional name and tags; quoted fields; normalisation **the same way sign-up does**; invalid rows reported with line number and skipped; duplicates (in-file and against existing) skipped and counted; `[EXTEND]` the CLI with an `import` command; and the explicit note that address normalisation **already exists from C1**.
- The gate: compose Postgres → `make migrate` → truncate → `postbox import subscribers fixtures/subscribers.csv` → three console strings → `select count(*)` = 46.
- The build: everything under `src/postbox/importers/`, `src/postbox/util/`, `src/postbox/events.py`, `src/postbox/cli.py`, and the three test files.
- The C1 baseline I compare against: `src/postbox/accounts/validators.py` and `signup.py`.

No delegation. The whole change is ~230 lines across nine files and I have read all of it; farming it to a subagent would cost more than it saves and would put a summary between me and the evidence I have to sign off on.

## Phase 2 — Reproduce the report's stated checks

What I'd run, from the workspace root:

1. `ruff check src tests` — expect clean. There's no `[tool.ruff]` config, so this is default `E4,E7,E9,F` only; the long line at `src/postbox/importers/subscribers.py:26` is not an E501 failure under defaults. If it does flag something, the report's "ruff check clean" is wrong and that goes in the write-up.
2. `pytest -q` — expect **11 passed**, not the 14 the report claims. Counting the tree: 3 in `test_csv_reader.py`, 3 in `test_email_util.py`, 3 in `test_import.py`, 2 in `test_signup.py`. If it comes back 11, I record the discrepancy as a factual error in the report — not a code defect, but it means the "all green" line was not copied from a real run of this tree, which colours how much weight the rest of the report's claims carry. If it comes back 14, I was wrong about the count and I say so plainly and move on.
3. `pytest -q --collect-only -q | tail -1` to pin the exact count in the write-up.

Expected substance: all tests pass. The suite runs on in-memory SQLite via `tests/conftest.py`, so nothing here exercises Postgres.

## Phase 3 — Run the C2 gate exactly as the card writes it

**Stop point before this phase.** The card's setup includes `psql "$DATABASE_URL" -c "truncate subscribers"`, and `docker-compose.yml` mounts a named volume `postbox-pg`, so that table is persistent local data, not a throwaway. I'd confirm with the user: *is the compose Postgres on this machine a scratch dev DB I can truncate?*
- **If yes** (my default assumption, since the card itself prescribes it): run the gate verbatim.
- **If no**: run the gate against a disposable database instead — `DATABASE_URL=postgresql+psycopg://postbox:postbox@localhost:5436/postbox_c2verify`, created with `createdb`, migrated the same way — and record in the write-up that the gate was run against a scratch DB rather than the named one, which does not change what any assertion proves.

Steps: `docker compose up -d postgres` → `make migrate` → truncate → `postbox import subscribers fixtures/subscribers.csv` → `psql "$DATABASE_URL" -tc "select count(*) from subscribers"`.

What I expect the assertions to show, and why — I traced `fixtures/subscribers.csv` by hand: 50 data rows after the header; line 17 (`not an address`) and line 41 (`someone@`) are invalid; line 23 (`dev3@NORTHLIGHT.EXAMPLE.CO.UK`) duplicates line 5 after domain lower-casing, and line 47 (`  kai10@studio.example `) duplicates line 12 after stripping. 50 − 2 − 2 = 46. `ImportResult.summary()` emits `Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`, which contains all three asserted substrings.

**One thing I expect to break, and it is the reason I run the gate rather than trusting the report.** `src/postbox/db.py:6` defaults to `postgresql+psycopg://...`, but the README, `Makefile`, and `.github/workflows/ci.yml` all export `DATABASE_URL=postgresql://...`. Under SQLAlchemy 2.0 a bare `postgresql://` resolves to the psycopg2 dialect, and `requirements.txt` ships only `psycopg[binary]` (v3). With `DATABASE_URL` set the way every doc in this repo sets it, `postbox import subscribers` should die with `ModuleNotFoundError: No module named 'psycopg2'`. Nothing in the pytest suite would catch this — it's all SQLite.

- **If it fails that way**: the gate does not currently pass as written, and the report's "the C2 gate ran locally against the compose database and printed the expected counts" cannot be true of this tree. I record the exact traceback, then re-run once with `DATABASE_URL=postgresql+psycopg://...` to establish whether the *import logic* is correct independently of the URL defect, and report both results separately. The fix is one line (`db.py` normalising the scheme, or the docs/CI/Makefile using `+psycopg`), but it is a blocker for sign-off because the gate is the acceptance criterion.
- **If it passes**: good, my reading of the dialect resolution was wrong; I record the actual console output and the count verbatim and drop the point.

Either way the write-up carries the literal captured console, per the card's `Capture: console`.

## Phase 4 — Trace each card clause to code and evidence

A short table in the write-up, one row per card clause, each with the file:line that satisfies it and the test or gate output that proves it:

- optional name/tags → `importers/subscribers.py:21-24`
- quoted fields → `tests/test_csv_reader.py` + fixture rows like `"Ann, A."`
- **normalised the same way sign-up does** → this is where it fails; see Finding 2
- invalid rows reported with line number and skipped → `pipeline.py:40-45`, gate assertion 3
- in-file and existing duplicates skipped and counted → `pipeline.py:48-56`, `tests/test_import.py:17`
- CLI `import` command → `cli.py:15-31`

I'd also note two things the build gets right that are easy to miss: the header row is skipped by content, not blindly (`subscribers.py:19`), and reported line numbers are the line the *record starts on*, which is what the card's "reported with their line number" needs when a quoted field spans lines.

## Phase 5 — The necessity review: what did not need to be written

Four findings, in the order I'd rank them for the write-up.

**Finding 1 — `src/postbox/util/email.py` is a duplicate of C1's validator, and a divergent one.** `normalize_email` is a verbatim copy of `accounts/validators.py:6-12`. `is_valid_email` is a *different implementation*: C1 calls `email-validator` (a declared dependency, `requirements.txt:5`), the new one is a hand-rolled regex. The card says in as many words that normalisation already exists from C1 and that addresses must be normalised "the same way sign-up does" — the build now has two definitions of "valid address" that can drift, and the import path no longer goes through the library the README names as the project's address checker. The report's stated reason ("sign-up validates through a form, not a function we could call") is false: `accounts/validators.is_valid_email` is a plain function and `signup.py:4` imports it directly. Claimed rung 7; correct rung is 2. This is the finding I'd lead with, because it's the one the card explicitly pre-empted.

**Finding 2 — `src/postbox/importers/csv_reader.py` reimplements the standard library.** 45 lines of hand-rolled tokenizer, justified in the report by "nothing in the codebase or the standard library handles quoted fields with embedded newlines". `csv.reader` handles exactly that, by default, and exposes `line_num`. Claimed rung 7; correct rung is 2. I'd also note the hand-rolled version has behaviour stdlib wouldn't: a `\r` inside a quoted field is retained (`csv_reader.py:25-28` has no `\r` guard on the in-quotes branch), so a CRLF-line-ended export leaves a trailing carriage return inside multi-line quoted names. The current fixture is LF-only so no test catches it.

**Finding 3 — `importers/base.py` + `importers/pipeline.py` are a framework for one caller.** An abstract `Importer` with one subclass, an `ImporterRegistry` holding one entry, a `Stage` ABC with four implementations, a `Context`, and a `Pipeline` that is instantiated once with a fixed list. The card asks for one CSV import. The report is candid that this is "more structure than the card strictly needs" and justifies it by Mailchimp/Buttondown importers "planned for next quarter" — that's building for requirements that aren't in this card and may change shape before they land. Claimed rung 7 ("write the minimum"); it isn't the minimum. The straight-line version is a single `import_subscribers(session, path)` loop.

**Finding 4 — `src/postbox/events.py` and task T2.4 have no basis in the card.** The card says nothing about an import-finished notification; the CLI just needs to print a summary. The report claims rung 2, "reused the codebase's existing event pattern" — but the same report lists `events.py` as new, and a grep for `events`/`bus` across the tree returns hits only in `events.py`, `cli.py:6,24`, and `subscribers.py:5,27`. There was no existing pattern. The result is that the importer publishes an event so that a module-import-time subscriber in `cli.py:24` can call `click.echo`, replacing one direct line, and the summary now prints as a side effect of `CsvSubscriberImporter.run` — which means importing `postbox.cli` anywhere silently registers a global handler. This is the clearest instance of a task and a file that did not need to exist.

Also noted, minor, not blocking: `ImportResult.summary()` produces "1 subscribers" / "1 invalid rows" for singular counts (`base.py:16-19`); harmless against the card's assertions, which are all plural.

## Phase 6 — Write the review

New file: **`/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-4r_sz6c6/ws/cycle-review-C2.md`**.

I would not edit `cycle-report.md` — it's the builder's artifact and the README defines it as such; my findings go alongside it, not over it. Structure:

1. Verdict line and what it's conditional on.
2. Card conformance table (Phase 4).
3. Gate result with the literal console capture (Phase 3).
4. Check results: ruff, pytest count vs. the claimed 14 (Phase 2).
5. Necessity review: the four findings, each with file:line, the report's stated reason, why it doesn't hold, claimed rung vs. actual.
6. Recommended remediation, as a concrete list — not applied:
   - Drop `util/email.py` and `util/__init__.py`; `pipeline.py` imports from `postbox.accounts.validators`. Before recommending it I'd check the two behavioural assertions in `test_email_util.py` against `email-validator` (`ann@localhost` rejected, `ann+news@example.com` accepted) so the swap is known-equivalent on the cases anyone cared enough to write down; fold anything still worth keeping into the C1 validator's tests and delete `test_email_util.py`.
   - Replace `csv_reader.read_rows` with a thin wrapper over `csv.reader`, keeping the same `(start_line, fields)` signature so `tests/test_csv_reader.py` stays as the safety net unchanged. Caveat to state explicitly: `reader.line_num` is the line at the *end* of a record, so the wrapper must track the previous value to keep reporting the record's *start* line — otherwise the card's "lines 17, 41" semantics silently change for multi-line records.
   - Collapse `base.py`, `pipeline.py`, `subscribers.py` into one module holding `ImportResult` and one loop; drop `Importer`, `ImporterRegistry`, `Stage`, `Context`, `Pipeline`. `cli.py` calls the function directly.
   - Delete `events.py`; `cli.py` calls `click.echo(result.summary())`.
   - Net effect: roughly five files and ~140 lines removed, no change to any card assertion. `tests/test_import.py` should pass untouched apart from the import path — that's the check that the collapse was behaviour-preserving, and I'd run it plus the full gate again afterwards.
7. Sign-off recommendation and the open question for the user.

## Phase 7 — Report back, and the decision I'd stop on

I'd report: the functional behaviour matches the card and the gate's counts are reachable from the fixture, but I would **not** tick the C2 box in `tasks.md` myself. Two things block a clean sign-off — the gate almost certainly not running as documented (Phase 3, psycopg2), and the C1 validator having been duplicated and re-implemented differently when the card specifically said it already existed. Ticking the box is the user's call, and it's the one place I'd stop and wait rather than assume:

- **If the user says sign off as-is and file the findings as follow-up work**: I mark C2 done in `tasks.md`, and the review file stands as the record of what's owed.
- **If the user says fix first**: that's a new piece of work, and I'd do it in the order above — validator first (correctness), then the DB URL (gate blocker), then the two deletions (`events.py`, then the pipeline collapse), re-running `ruff check src tests`, `pytest -q`, and the full gate after each, and only then tick the box.
- **If the user says fix only the blockers**: validator + DB URL, leave the framework and event bus in place with the findings recorded against them, then tick.

Default if I get no ruling: leave the box unticked, deliver `cycle-review-C2.md`, and change no source file.