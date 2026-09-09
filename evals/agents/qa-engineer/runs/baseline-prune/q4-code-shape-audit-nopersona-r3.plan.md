FILES-READ: `cycle-report.md`, `tasks.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/__init__.py`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/cli.py`, `src/postbox/events.py`, `src/postbox/accounts/validators.py`, `src/postbox/accounts/signup.py`, `src/postbox/util/__init__.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_import.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_signup.py`

---

# Action plan — verify and sign off cycle C2 (CSV subscriber import)

Scope note up front: this card asks me to **verify and write up**, not to rework. I would not rewrite the importer as part of this. Every "should be removed" finding lands in the review document as a sized, specific recommendation; actually doing the removals is a separate go-ahead I would ask for at the end (branch described in Phase 8).

## Phase 1 — Pin down the contract, then read the change against it

Already done in this session; in a real run this is the first pass and everything after depends on it.

- Read `tasks.md` C2 to extract the checkable obligations: one address column plus optional name and tags; quoted fields; **addresses normalised the same way sign-up does**; invalid rows reported with line number and skipped; duplicates within-file and against-existing skipped and counted; `[EXTEND]` the CLI with an `import` command; and the explicit note that **address normalisation already exists from C1**. Plus the four gate assertions and their exact strings.
- Read `cycle-report.md` for the five claimed tasks, the rung claimed for each, and the stated justification — these are claims to be checked, not facts to be inherited.
- Read `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql` for how the project is meant to run and what is already a declared dependency (notably `email-validator`, described as "already used at sign-up").
- Read the C1 code the card points at (`accounts/validators.py`, `accounts/signup.py`, `tests/test_signup.py`) **before** the new code, so I judge the new code against what existed rather than the other way round.
- Read every file the report lists, plus `models.py`, `db.py`, `conftest.py`, `fixtures/subscribers.csv`.
- Hand-count the fixture to confirm the gate numbers are arithmetically reachable: 50 data rows (lines 2–51), invalid at line 17 (`not an address`) and line 41 (`someone@`), duplicates at line 23 (`dev3@NORTHLIGHT.EXAMPLE.CO.UK` vs line 5) and line 47 (`  kai10@studio.example ` vs line 12) → 46. This matches the card, and both duplicates only collapse if normalisation strips whitespace and lower-cases the domain, which is exactly the C1 behaviour the card told the builder to reuse.

## Phase 2 — Reproduce the checks the report claims are green

- Set up: `python -m venv .venv && .venv/bin/pip install -r requirements.txt && .venv/bin/pip install -e .` (isolated; I would not install into a system interpreter).
- Run `ruff check src tests`. Expect clean — there is no `[tool.ruff]` block in `pyproject.toml`, so the default rule set (`E4`, `E7`, `E9`, `F`) applies and line length is not enforced; I see no unused imports or undefined names. If it is clean, the report's ruff claim stands.
- Run `pytest -q`. **I expect 11 passed, not the 14 the report states**: 3 in `test_import.py`, 3 in `test_csv_reader.py`, 3 in `test_email_util.py`, 2 in `test_signup.py`. If it reports 11, I record the report's test count as inaccurate and ask (in the write-up) whether three tests were dropped before hand-off. If it reports 14, I collect the names with `pytest -q --collect-only` and correct my own count — this is a cheap thing to be wrong about and the run decides it.
- Note for the record that `tests/conftest.py` builds a **SQLite in-memory** engine. Everything in `pytest` therefore proves nothing about PostgreSQL, the `psycopg` driver, or the CLI. The gate in Phase 3 is the only thing that covers those, so a green `pytest` is not evidence for the gate.

## Phase 3 — Run the C2 gate exactly as the card specifies

**Stop point (destructive step).** The card's setup includes `psql "$DATABASE_URL" -c "truncate subscribers"`, and `docker-compose.yml` gives the Postgres container a **named persistent volume** (`postbox-pg`), so this is not a throwaway database — it may hold whatever the operator has been working with. Before running it I would confirm: "the card's gate truncates `subscribers` in the compose database on port 5436 — OK to proceed, or should I point it at a scratch database?"
- If approved → run the gate verbatim as written.
- If declined → create a scratch database (`createdb postbox_c2gate`), run `make migrate DATABASE_URL=...postbox_c2gate`, and run the gate against it; the assertions are unaffected, and I note in the write-up that the gate ran against a scratch DB.
- Default if no answer is available: the scratch-database route, since it produces identical evidence with nothing destroyed. I would never point the truncate at any host other than `localhost:5436`.

Then:
1. `docker compose up -d postgres`, wait for the healthcheck.
2. `make migrate` — expect `0001_init.sql` applied.
3. Truncate (per the ruling above).
4. `postbox import subscribers fixtures/subscribers.csv`, capturing the console.
5. `psql "$DATABASE_URL" -tc "select count(*) from subscribers"`.

Expected console from `ImportResult.summary()`:
`Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`
I checked the card's three assertions as substrings against that exact string — all three are present, including the parenthesised `(lines 17, 41)` fragment. So the format is compatible with the gate even though it nests the clauses. Row count should be `46`.

**Failure I specifically expect to hit, and would chase before blaming the importer:** `requirements.txt` pins only `psycopg[binary]==3.2.1` (psycopg **3**), but the `DATABASE_URL` documented in `README.md:18`, `Makefile:1`, `.github/workflows/ci.yml:15` and the card's own setup is `postgresql://…`, which SQLAlchemy 2 resolves to the **psycopg2** dialect. Only `db.py:6`'s built-in default uses `postgresql+psycopg://`. If `DATABASE_URL` is exported as documented, `create_engine` should fail with a missing-psycopg2 module error before a single row is read. If that happens I would: re-run once with `DATABASE_URL=postgresql+psycopg://postbox:postbox@localhost:5436/postbox` to get the gate evidence, and record the driver-prefix mismatch separately as a **pre-existing repo/environment defect that C2 is the first cycle to expose** (C2 is the first code path that connects the CLI to Postgres) — not as a C2 build error, and not something I would silently paper over by only ever using the working URL. If the gate passes on the documented URL, I drop this finding.

Capture the raw console output verbatim; it goes into the write-up as the evidence for the gate, replacing the report's unwitnessed "ran locally and printed the expected counts".

## Phase 4 — Requirement-by-requirement verification against the card

For each card clause, name the file and line that satisfies it and, where cheap, the test or gate line that proves it:

| Card clause | Where | Status I expect |
|---|---|---|
| Address column + optional name/tags | `importers/subscribers.py:19-24` | Met; header row skipped by name, extra/short rows tolerated |
| Quoted fields allowed | `importers/csv_reader.py` + `tests/test_csv_reader.py` | Met behaviourally (see Phase 5 on *how* it is met) |
| Invalid rows reported with line number and skipped | `pipeline.py:40-45`, `base.py:16-19`, gate line `(lines 17, 41)` | Met |
| Duplicates within-file skipped and counted | `pipeline.py:48-56` via the `seen` set | Met — and the `seen` set is genuinely required, since `PersistStage` only `session.add`s without flushing, so the DB query alone would miss same-run duplicates |
| Duplicates against existing subscribers | `pipeline.py:50-52`, `tests/test_import.py:17-23` | Met |
| **Normalised the same way sign-up does** | `pipeline.py:11` imports from `util/email.py`, **not** `accounts/validators.py` | **Not met** — see Phase 5, finding 1 |
| `[EXTEND]` CLI with an `import` command | `cli.py:15-31`, `pyproject.toml:7` entry point | Met |

I would also confirm the registry is actually wired — `importers/__init__.py:1-4` registers `CsvSubscriberImporter` and `cli.py:7` imports the package, so `registry.get("subscribers")` resolves at runtime. Worth checking explicitly because a registry populated only by an import side-effect is a classic way for a CLI to die with a `KeyError` in a packaged install; the gate run in Phase 3 settles it.

Gaps in coverage I would record: nothing exercises the CLI command itself, nothing exercises the event-bus wiring in `cli.py:20-24`, and nothing asserts that import and sign-up agree on which addresses are valid — which is the one property the card called out by name.

## Phase 5 — Necessity review: what did not need to be written

Five findings, ordered by how much they matter. For each I would cite the report's stated justification and whether it holds.

**1. `src/postbox/util/email.py` is a re-implementation of code the card said to reuse — blocking.**
`accounts/validators.py:6-12` and `util/email.py:8-13` contain a byte-identical `normalize_email`. The report justifies the copy with "sign-up validates through a form, not a function we could call" — that is factually wrong: `accounts/validators.py` exposes two plain module-level functions and `accounts/signup.py:9-11` calls them directly. The card itself says "the address normalisation already exists from C1."

Worse than the duplication: the copy replaces `email-validator` (a declared dependency, `requirements.txt:5`) with a hand-rolled regex (`util/email.py:5`). The two disagree. I would write a throwaway falsifying test to make this concrete rather than assert it:

```python
# scratch, not committed unless the user wants it in tests/
from postbox.util.email import is_valid_email as importer_valid
from postbox.accounts.validators import is_valid_email as signup_valid

def test_import_and_signup_disagree():
    assert importer_valid("a..b@example.com") is True    # regex allows consecutive dots
    assert signup_valid("a..b@example.com") is False     # email-validator rejects them
```
Expected: both assertions hold, proving the import path can create subscribers that sign-up would have refused. If the test *fails* — i.e. they agree on that input — I try a leading-dot local part and a quoted local part before softening the finding; if they turn out to agree everywhere I try, I downgrade this to "duplication with no observed behavioural divergence" and say so plainly.
Removable: `util/email.py` (17 lines), `util/__init__.py`, `tests/test_email_util.py` (13 lines); `pipeline.py:11` re-points at `postbox.accounts.validators`.

**2. `importers/csv_reader.py` re-implements the standard library — removable.**
The report's justification is "Nothing in the codebase or the standard library handles quoted fields with embedded newlines." Python's `csv` module handles commas in quoted fields, doubled quotes, and embedded newlines — all three of the behaviours `tests/test_csv_reader.py` checks — and exposes `reader.line_num` for line numbers. 45 lines of hand-written tokenizer plus 23 lines of tests exist to re-obtain that.
The hand-rolled version is also slightly worse: `csv_reader.py:26-28` keeps a `\r` inside a quoted field while `:40` strips it outside, so a CRLF export leaves stray carriage returns in name/tag values.

**Caveat I would verify before recommending the swap, not after.** There is one genuine behavioural difference: `read_rows` reports the *first* line of a record (`record_start`), whereas `csv.reader.line_num` after a record equals its *last* physical line. For the fixture this is identical (no embedded newlines, so lines 17 and 41 are unchanged), but `tests/test_csv_reader.py:16` depends on start-line semantics. The recommendation must therefore say: track `prev = reader.line_num` before each `next()` and report `prev + 1` to preserve start-line semantics. I would state this in the write-up rather than hand-waving "just use `csv`".

**3. `importers/base.py`'s `Importer` ABC and `ImporterRegistry` — speculative, removable.**
An abstract base with one implementation and a registry with one entry, justified by Mailchimp/Buttondown importers "planned for next quarter". `cli.py:31` could call `CsvSubscriberImporter().run(...)` directly. `ImportResult` (lines 10-19) is genuinely needed and stays. Removable: roughly lines 22-46, ~23 lines, plus the registration in `importers/__init__.py`.

**4. `importers/pipeline.py` — speculative, removable.**
78 lines defining a `Stage` ABC, a `Context` dataclass, four one-method stage classes, and a `Pipeline` runner, to express what is a single loop with four steps over one list of rows. The card describes exactly one import shape. Folding normalise/validate/dedupe/persist into a ~20-line loop in `subscribers.py` would preserve every behaviour, including the `seen` set. `Row` is worth keeping.

**5. `src/postbox/events.py` — not needed, and the report describes it incorrectly.**
The report puts T2.4 at "rung 2 — reuse existing" and says it "reused the codebase's existing event pattern". The same report lists `events.py` as **new** two sections later, and there is no other publisher or subscriber anywhere in the codebase — `grep` for `bus.` returns only `subscribers.py:27` and `cli.py:24`. So this is a new 22-line in-process event bus, plus subscription wiring, whose entire job is to move one `click.echo` out of the CLI. It also makes the command's only output depend on an import-time side effect: if `cli.py:24` is ever missed, the import runs and prints nothing, and the C2 gate fails with no error. `click.echo(result.summary())` inside `import_subscribers` is the whole feature.

**Rough size of the finding:** about 200 lines of source (`csv_reader.py` 45, `pipeline.py` 78, `events.py` 22, registry/ABC ~23, `util/email.py` 17, `__init__` files) and 36 lines of tests exist for a card that needs on the order of 60–70 lines. I would state that as an estimate derived from line counts, not a precise measurement.

## Phase 6 — Check that my own recommendations are safe before writing them down

I would not ship a recommendation I have not sanity-checked, because both of the big ones can change the gate result:

- **Validator swap:** every one of the 46 valid fixture addresses must still pass under `email-validator`. The fixture uses `.example`, `.example.org`, `.example.co.uk` and `.example.com` domains; `email-validator` has special-use domain handling and I do not want to assume from memory how it treats these. I would run a one-off loop over all 50 fixture addresses through `accounts.validators.is_valid_email` and confirm exactly lines 17 and 41 are rejected. If any of the 46 are rejected, the recommendation changes from "delete the copy" to "delete the copy **and** reconcile the fixture/validator", and I say which addresses are affected.
- **CSV swap:** run the three assertions in `tests/test_csv_reader.py` against a `csv.reader`-based implementation using the `prev + 1` line-number scheme, and confirm the fixture still yields invalid lines 17 and 41.
- Both together: re-run the Phase 3 gate would be the real proof, but since I am not making the change in this cycle, I record these as *verified-on-paper preconditions* attached to the recommendation, clearly labelled as such.

## Phase 7 — Write the sign-off document

Write **`cycle-review.md`** at the repo root (new file). `README.md:27` names `cycle-report.md` as the builder's write-up and defines no convention for a review, so I am choosing a sibling file rather than editing Tomasz's report — I would not overwrite or edit someone else's write-up. If the user prefers it appended as a section inside `cycle-report.md`, that is a one-line change I would make on request.

Structure:
1. **Verdict** — one line, up top.
2. **Gate evidence** — the verbatim console from Phase 3, each of the four card assertions marked met/not met, and which `DATABASE_URL` form it took to get there.
3. **Checks** — actual `ruff` and `pytest` output, with the 11-vs-14 discrepancy stated as a fact with both numbers.
4. **Card clause table** from Phase 4, with file:line citations.
5. **Findings** — the five from Phase 5, each with: what was built, what the report claimed, what is actually true, the file:line evidence, and the concrete removal with its caveat. Findings 1 is marked blocking; 2–5 are marked non-blocking-but-recommended.
6. **Report accuracy** — a short, non-editorial list of the three claims that do not hold: the stdlib justification for T2.1, the "sign-up validates through a form" justification for T2.2, and the "reuse existing" rung for T2.4 against a new file. This matters because the rungs are what a reviewer would otherwise trust instead of reading the code.
7. **Not covered by any test** — CLI command, event wiring, import/sign-up validator agreement, anything PostgreSQL-specific.
8. **Recommended follow-up**, sized.

Expected verdict, stated so the user can overrule it: **not signed off as-is.** One blocking item — the card explicitly directed reuse of C1's normalisation and the build shipped a diverging copy, which is a missed card requirement rather than a style preference. Everything else in the card is satisfied and the gate numbers are right.

## Phase 8 — Report, and the ruling I would stop for

I would report: the gate result and whether it needed a different `DATABASE_URL`; the real test count; that the card's functional requirements are met except for the normalisation reuse clause; the ~200 lines that did not need to be written and what each would collapse to; the three inaccurate claims in the report; and the path of the file I wrote.

**Stop point — the sign-off ruling is the user's, not mine.** Three branches:
- **Reject and rework** → I would then, in order: re-point `pipeline.py` at `accounts/validators.py` and delete `util/email.py` + `tests/test_email_util.py`; replace `csv_reader.py` with `csv.reader` using the `prev + 1` line scheme, keeping `tests/test_csv_reader.py`'s three cases pointed at the new function; collapse `pipeline.py` and the registry/ABC into `subscribers.py`; replace the event bus with a direct `click.echo` and add a `CliRunner` test for the command; then re-run `ruff`, `pytest`, and the full Phase 3 gate and confirm the counts are still 46/2/2 with lines 17 and 41.
- **Accept with follow-up** → I would fix only finding 1 (the card requirement) and record findings 2–5 as a new card in `tasks.md` rather than leaving them in a review file nobody re-reads.
- **Accept as-is** → I record the decision and the findings in `cycle-review.md` unchanged, and say plainly in the write-up that the "normalised the same way sign-up does" clause is knowingly unmet, so the next person does not discover it as a surprise.

Default while waiting: leave the code untouched and the review written.

**No delegation.** This is a single-repo review of about 300 lines of new code; splitting it across workers would cost more in briefing and cross-checking than doing it directly, and the findings depend on holding the C1 code and the new code side by side. I would not spawn any subagent for it.