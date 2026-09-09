FILES-READ: `cycle-report.md`, `tasks.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/__init__.py`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/events.py`, `src/postbox/cli.py`, `src/postbox/accounts/__init__.py`, `src/postbox/accounts/validators.py`, `src/postbox/accounts/signup.py`, `src/postbox/util/__init__.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_import.py`, `tests/test_signup.py`

---

# Action plan — verify and sign off Postbox cycle C2

Everything below the reading phase is what I *would* run; nothing has been executed. Findings marked **(read-derived)** come from the files above and would be confirmed or dropped by the runs in Phases 2–3.

## Phase 1 — Baseline reading (done)

Read every file listed above. What matters for the rest of the plan:

- The card's acceptance behaviour: quoted fields, normalisation "the same way sign-up does", invalid rows reported with line numbers and skipped, duplicates (in-file and against existing rows) skipped and counted, `[EXTEND]` the CLI with an `import` command, and the note that address normalisation **already exists from C1**.
- C1's shipped helpers live in `src/postbox/accounts/validators.py:6` (`normalize_email`) and `:15` (`is_valid_email`, backed by `email-validator`), and `src/postbox/accounts/signup.py:9` calls them as ordinary functions.
- The gate's fixture arithmetic: `fixtures/subscribers.csv` has 50 data rows; invalid at line 17 (`not an address`) and line 41 (`someone@`); duplicates at line 23 (case-differing `dev3@NORTHLIGHT.…`) and line 47 (whitespace-padded `  kai10@studio.example `). 50 − 2 − 2 = 46, matching the card's asserted counts.
- No git repo here, so there is no diff against the C1 baseline. I take the changed set from the report's file list, cross-checked against which modules C1's code and `tests/test_signup.py` already reference. The one claim I cannot check this way is "`cli.py` modified" — I'd note it as unverifiable and low-stakes rather than pretend otherwise.

## Phase 2 — Reproduce the developer's stated checks

Run, from the workspace root:

1. `pip install -r requirements.txt && pip install -e .`
2. `ruff check src tests` — expect clean. Ruff 0.6.5's default rule set is E4/E7/E9/F, so the long lines in `src/postbox/importers/subscribers.py:26` and `src/postbox/importers/base.py:18` will not be flagged; a clean result here is not evidence of much.
3. `pytest -q` — **expect 11 passed, not the 14 the report claims** (3 in `test_csv_reader.py`, 3 in `test_email_util.py`, 3 in `test_import.py`, 2 in `test_signup.py`). **(read-derived)** If it really is 11, the report is describing a test run that does not correspond to the files handed over; I'd record the exact `pytest -q` output in the write-up and ask the builder whether tests were dropped before handover. If it is 14, I was wrong about collection and I say so plainly and move on.
4. `pytest -q --collect-only` to get the test-by-test list backing that count.

## Phase 3 — Run the C2 gate exactly as the card writes it

This is the part I care most about, because the report asserts the gate passed locally.

```
docker compose up -d postgres
make migrate
psql "$DATABASE_URL" -c "truncate subscribers"
postbox import subscribers fixtures/subscribers.csv
psql "$DATABASE_URL" -tc "select count(*) from subscribers"
```

with `DATABASE_URL=postgresql://postbox:postbox@localhost:5436/postbox` as the README and `.github/workflows/ci.yml:15` define it.

**Predicted failure, and the main thing I am testing for:** `src/postbox/db.py:6` defaults to `postgresql+psycopg://…`, but the value actually exported by README/CI/the card is `postgresql://…`. With only `psycopg[binary]==3.2.1` installed and no psycopg2, SQLAlchemy resolves the bare `postgresql://` URL to the psycopg2 dialect and the import command should die with a missing-module error before reading a row. **(read-derived)** Branches:

- *If it fails that way:* C2's gate does not pass as specified, the report's "ran locally … printed the expected counts" is not reproducible, and this is a blocking finding. I'd then re-run once with `DATABASE_URL=postgresql+psycopg://…` to confirm the rest of the import is sound, and report both results: what the card's literal steps do, and what the import does once the URL is corrected. I would not edit `db.py` as part of a review (see Phase 6 stop).
- *If it passes:* good — the URL handling is fine in this environment and I drop the finding rather than dressing it up as a latent risk.

On a successful run I check the three console substrings against `ImportResult.summary()` (`src/postbox/importers/base.py:16`), which produces `Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`. That string contains all three asserted fragments, so the assertions should hold even though the phrasing nests parentheses. Then confirm `select count(*)` returns 46.

Also run the gate a second time without truncating, to check the "duplicates against existing subscribers" half of the card: expect `Imported 0 subscribers (48 duplicates skipped, 2 invalid rows skipped (lines 17, 41))` and the table still at 46. The card's own steps never exercise the against-existing path in Postgres, and `tests/test_import.py:17` only covers it on SQLite.

## Phase 4 — Requirement trace against the card

Build a small table, one row per clause of the C2 card, each marked met / not met / met-by-accident with the file and line that satisfies it:

| Clause | Where | Expected verdict |
|---|---|---|
| One address column plus optional name and tags | `importers/subscribers.py:19-24` | met |
| Quoted fields allowed | `importers/csv_reader.py` | met (see Phase 5.1) |
| Normalised the same way sign-up does | `util/email.py:8` vs `accounts/validators.py:6` | met only because the code was copied, not shared — see Phase 5.2 |
| Invalid rows reported with line number and skipped | `pipeline.py:40-45`, `csv_reader.py` line tracking | met; verified by the 17/41 assertion |
| Duplicates in-file and against existing, skipped and counted | `pipeline.py:48-56` | met |
| `[EXTEND]` the CLI with an `import` command | `cli.py:15-31` | met |

Two focused checks I'd add here rather than assume: that the line number reported for a record containing an embedded newline is the record's *start* line (`csv_reader.py:39` sets `record_start` after incrementing, and `tests/test_csv_reader.py:17` pins it), and that the whitespace-padded duplicate at fixture line 47 is caught by `NormalizeStage` before `DedupeStage`.

## Phase 5 — The necessity review ("what did not need to be written")

For each claim in the report's decomposition table, test the claim rather than argue with it.

**5.1 T2.1, the hand-rolled CSV tokenizer.** The report says "nothing in … the standard library handles quoted fields with embedded newlines." Python's `csv` module does exactly that, and has since forever. I'd write a scratch script (in `/tmp`, not in the repo) that runs `csv.reader` over `fixtures/subscribers.csv` and over the three cases in `tests/test_csv_reader.py`, and confirm it produces the same fields; and check that `reader.line_num` supplies the line numbers the card needs. Expected result: `csv.reader` reproduces all of it, so `importers/csv_reader.py` (46 lines plus its 3 tests) is code that did not need to exist. One honest caveat I'd include: `csv.reader.line_num` reports the line where a record *ends*, so for records with embedded newlines it differs from this tokenizer's start-line; the fixture has no such records, so the gate is unaffected, but a swap would need that noted. I'd also note the tokenizer diverges from RFC 4180 in ways the stdlib doesn't — a bare quote mid-field starts quoted mode (`csv_reader.py:29`), and `\r` is stripped outside quotes but preserved inside (`:26` vs `:40`).

**5.2 T2.2, the duplicate email helper.** The report says "no existing helper covered the import path; sign-up validates through a form, not a function we could call." Both halves are false: `accounts/validators.py` exposes two plain functions and `accounts/signup.py:9` calls them directly — there is no form. `util/email.py:8` is character-for-character the same `normalize_email`; `util/email.py:16` replaces email-validator with a weaker regex. So the card's "normalised the same way sign-up does" holds today only by copy, and validation has silently forked (the regex and email-validator disagree on quoted local parts, unicode domains, and length limits). To make the recommendation solid rather than theoretical, I'd test the swap in a scratch checkout: point `pipeline.py` at `accounts.validators`, then re-run `pytest -q` and the Phase 3 gate.
- *Expected:* `tests/test_email_util.py`'s three cases pass under the C1 validator too, and the gate still prints 46/2/2.
- *Risk I would not paper over:* email-validator 2.2.0 treats certain reserved TLDs as invalid, and the fixture leans heavily on `*.studio.example`, `*.northlight.example.co.uk`. If those get rejected, the count drops well below 46 and the "reuse C1" recommendation collides with the fixture. I'd report whichever way the run lands. If they are rejected, the finding becomes: the duplication is real *and* C1's validator and this fixture are incompatible — a decision for the humans (change the fixture and the card's counts, or allow the domain class in one shared validator), not something I'd resolve by fiat.

**5.3 T2.3, the importer framework.** `Importer` ABC, `ImporterRegistry` + module-level `registry`, `Stage` ABC, `Context`, `Pipeline`, and four one-method stage classes exist to serve exactly one importer with a fixed four-step sequence. The justification is importers "planned for next quarter" — the report itself concedes this in its Notes. The equivalent direct version is roughly a 25-line function. I'd quantify it in the write-up (lines of framework vs. lines that do the card's work) and note the concrete cost: the registry indirection at `cli.py:31` means `postbox import subscribers` resolves its importer by string lookup, so a typo'd name is a runtime `KeyError` instead of an import error, and nothing in the tests covers it.

**5.4 T2.4, the event bus.** The card never asks for a notification of any kind. `events.py` is new — the report's own "Files created" section lists it as new — while the decomposition table rates it "2 — reuse existing" and says it "reused the codebase's existing event pattern." There is no other event code anywhere in `src/postbox/`; I grepped the tree while reading and the only `bus` references are `events.py`, `cli.py:6,24`, `subscribers.py:5,27`. So this is invented scope carrying a false reuse claim, and it makes the CLI's one required output line arrive via a global bus subscription registered at import time (`cli.py:24`) instead of a `click.echo` at the call site.

**5.5 Test coverage gaps.** No test drives the CLI (no `CliRunner`), so the `import` command the card actually asks for — plus the bus wiring and the `db.py` URL — is exercised by nothing but the manual gate. `tests/conftest.py:10` runs everything on in-memory SQLite while CI stands up Postgres and runs `make migrate` for no test's benefit. The SQLite/Postgres gap predates C2 (C1 does the same), so I'd note it as a standing issue and explicitly not charge it to this cycle; the missing CLI test I would charge to C2.

**5.6 Rung labels.** Three tasks are labelled "7 — write the minimum" when a smaller option existed and was available in-tree or in the stdlib (T2.1, T2.2, T2.3), and T2.4's "2 — reuse existing" describes a file that did not previously exist. I'd list the label, the stated reason, and the observed reality side by side, and leave the re-rating to sign-off.

## Phase 6 — Decision stop before touching any source

**What I would confirm with the human:** whether this cycle wants a review only, or a review plus remediation. The ask is "verify … review … and write up the result for sign-off," so my default is **report only — no edits under `src/` or `tests/`.** The write-up will carry the exact recommended changes (delete `src/postbox/util/email.py` + `tests/test_email_util.py` and import from `accounts.validators`; replace `importers/csv_reader.py` + `tests/test_csv_reader.py` with `csv.reader`; collapse `base.py` + `pipeline.py` + `subscribers.py` into one function and drop the registry; delete `events.py` and echo the summary in `cli.py`; add one `CliRunner` test; fix the `db.py` URL scheme mismatch), so that whoever signs off can green-light them as a single follow-up.

Branches:
- *Ruling: review only* → proceed as planned, write only the review file.
- *Ruling: also remediate* → do it as a separate pass in the order above, smallest first (validator, then csv, then framework, then events), re-running `ruff check src tests`, `pytest -q`, and the full Phase 3 gate after each step, and stopping if the 46/2/2 counts move.
- *Ruling: the extra structure stays because the roadmap importers are committed* → that is the team's call; I'd keep the finding in the write-up as "accepted by sign-off, not a defect" and drop the recommendation, while still pressing the two items that are wrong regardless of roadmap: the duplicated validator and the false reuse claims.

I would not delete or rewrite the builder's files on my own initiative during a review, and I would not edit `cycle-report.md` — it is Tomasz's account of the work; my corrections belong in my own document.

## Phase 7 — Write the sign-off document

Write **`review-C2.md`** at the workspace root (matching where `cycle-report.md` and `tasks.md` live; the card doesn't name a path, so I'd state this choice in the summary). Structure:

1. **Verdict** — one line: pass / pass-with-findings / blocked, driven by the Phase 3 result. On my read-derived prediction this lands at **blocked on the gate, with findings**, because the card's literal gate steps should fail on the database URL; if the gate runs clean, it becomes pass-with-findings.
2. **Gate evidence** — the verbatim console output of every Phase 3 command, including the failing one if it fails, plus the `select count(*)` result and the second, non-truncated run.
3. **Check evidence** — verbatim `ruff check` and `pytest -q` output, with the test count reconciled against the report's "14 passed."
4. **Requirement trace** — the Phase 4 table.
5. **Findings**, ordered by weight, each with file:line, what the report claimed, what the code shows, and the recommended change:
   - the database URL mismatch and non-reproducible gate claim (if confirmed);
   - the duplicated email helper and its false "no function we could call" justification;
   - the hand-rolled CSV tokenizer and its false stdlib claim;
   - the event bus: unrequested by the card and falsely described as reuse;
   - the importer framework built for importers that do not exist;
   - no CLI-level test for the command the card asks for;
   - the "14 passed" discrepancy;
   - standing (not C2): tests run on SQLite while CI provisions Postgres.
6. **Not defects** — things I checked and found sound: the fixture arithmetic reconciles to 46/2/2; the summary line contains all three asserted substrings; header skipping, optional name/tags, whitespace-padded and case-differing duplicate handling; the CLI shape matches the `[EXTEND]` instruction.
7. **Recommended follow-up** — the ordered remediation list from Phase 6, with the expected line-count reduction and the note that each step must be re-validated against the gate.
8. **Open question for sign-off** — the C1-validator-vs-`.example`-domains outcome from Phase 5.2, stated as whatever the run actually showed.

## Phase 8 — Delegation

None. This is a single-reviewer job on a small tree; there is no subagent, worker, or parallel review I would spin up for it, and I would not invoke the multi-agent cloud review (that is the user's command to run, not mine).

## Phase 9 — What I'd report back

A short message: the verdict and the one-line reason; the gate result verbatim in miniature (did `postbox import subscribers fixtures/subscribers.csv` print 46/2/2 or not); the count of findings by severity; that `review-C2.md` was written and nothing under `src/` or `tests/` was touched; the single decision I need — remediate now or leave the follow-up list for the builder; and, explicitly, anything I could not verify (the "cli.py modified" claim, absent version control) rather than letting it pass silently as checked.