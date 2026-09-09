FILES-READ: `tasks.md`, `cycle-report.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/cli.py`, `src/postbox/events.py`, `src/postbox/accounts/signup.py`, `src/postbox/accounts/validators.py`, `src/postbox/util/__init__.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_import.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_signup.py`

---

# Action plan — verify C2 (import subscribers from a CSV)

## Phase 0 — Intake and baseline (done above, no execution)

Read the card, the builder's write-up, and every file the write-up lists, plus the surrounding infrastructure so I know what "real infrastructure" means here. Established facts I carry forward:

- The C2 gate is a CLI case end to end — no GUI, no subjective judgement. Classification: **CLI, executable, evidence-capturable**. It is auto-approvable on evidence alone *if* everything is green; the code-shape audit and any red leg force a human checkpoint instead.
- The gate's expected numbers are checkable against `fixtures/subscribers.csv` before I run anything: 51 lines = 1 header + 50 data rows; line 17 (`not an address`) and line 41 (`someone@`) are the invalid rows; line 23 (`dev3@NORTHLIGHT.EXAMPLE.CO.UK`) duplicates line 5 after domain-lowercasing and line 47 (`  kai10@studio.example `) duplicates line 12 after stripping. 50 − 2 − 2 = 46. The card's asserted 46/2/2 and "(lines 17, 41)" are internally consistent with the fixture. That is a paper check, not evidence — it only tells me what a correct run must print.
- The summary string built in `importers/base.py` is `Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`. All three asserted substrings occur inside it, so substring assertions can pass on that shape.

**Two contradictions in the write-up I flag now and resolve by execution, not argument:**
1. It claims `pytest -q` → **14 passed**. I count **11** test functions across the four test files (3 + 3 + 3 + 2). Either tests exist that I cannot see, or the number is not from a real run.
2. It claims T2.4 was rung 2, "reused the codebase's *existing* event pattern" — while its own file list marks `src/postbox/events.py` **(new)**. There is no prior event bus anywhere in the tree.

I do not treat either as proven until the commands run. But they set my prior: the report's green claims are unverified until I reproduce them.

## Phase 1 — Stand up real infrastructure and reconcile the environment

What I'd do:
- `docker compose up -d postgres`, then poll `docker compose ps` / `pg_isready` until the healthcheck is green (the container binds host port **5436**).
- `pip install -r requirements.txt && pip install -e .` in a clean venv, capture the resolved versions.
- `make migrate` (applies `migrations/0001_init.sql`).
- `psql "$DATABASE_URL" -c "truncate subscribers"`.

**A blocker I expect here, and how I'd handle it.** The card's Setup uses `$DATABASE_URL` but never sets it. README and CI both document it as `postgresql://postbox:postbox@localhost:5436/postbox`. `psql` needs exactly that form. But SQLAlchemy resolves a bare `postgresql://` URL to the **psycopg2** dialect, and `requirements.txt` pins **psycopg 3** only (`psycopg[binary]==3.2.1`); `db.py`'s own default is `postgresql+psycopg://`. So with the documented environment exported, `postbox import` should die with `ModuleNotFoundError: No module named 'psycopg2'` before touching a row.

I would run it that way first anyway — the documented, card-faithful environment — and capture the failure verbatim if it occurs. Then I'd re-run the Action leg with `DATABASE_URL=postgresql+psycopg://...` for the app while keeping the psql-compatible form for `psql`/`make`, and record that as an **explicit deviation from the card's Setup**, with both transcripts in the report. That isolates "environment plumbing" from "product defect" instead of quietly papering over it.

I would report this as a **real gap regardless of which way it lands**: CI spins up Postgres, runs `make migrate`, then runs a `pytest` suite whose `conftest.py` uses `sqlite+pysqlite:///:memory:`. **Nothing in CI ever connects the application to Postgres.** The one path that would have caught the driver mismatch does not exist. That is worth a durable regression case (Phase 6).

I would **refuse** to substitute SQLite for Postgres in the gate run, or to hand-write the summary line from unit-test output. The gate says PostgreSQL; if Postgres cannot be reached, the gate is BLOCKED, not passed.

## Phase 2 — Quality gates, reproducing the builder's claims

Run exactly what CI runs, in CI's order, capturing full output and exit codes:
- `ruff check src tests` — expect clean. Default rule set is E4/E7/E9/F with no `[tool.ruff]` config, so the long call in `subscribers.py` won't trip a line-length rule; the unused `ctx` parameter in `NormalizeStage.process` won't either.
- `pytest -q` — I expect **11 tests collected, not 14**, and I expect **`tests/test_import.py::test_import_counts_and_persists` to FAIL**.

Why I expect that failure, stated as a prediction to be confirmed by the run: the test writes `a@x.io` then `A@X.IO` and asserts one duplicate. `normalize_email` lowercases only the domain and preserves the local part as typed, so `A@X.IO` becomes `A@x.io` — not equal to `a@x.io`. `DedupeStage` compares exact strings in `ctx.seen` and in a case-sensitive `email == ...` query, so no duplicate is detected. Expected actual: `imported=3, duplicates=0`, and the email-set assertion also fails.

If it fails, that is a direct contradiction of "all tasks green — 14 passed" and the single most important thing in my report. If it passes, I capture the transcript and investigate why my reading was wrong — I do not silently drop the prediction.

**Delegation:** I would run these two commands myself. They are cheap, decisive, and the whole point of the phase is that the claim about them is in doubt.

## Phase 3 — Execute the C2 gate against Postgres

Run the card's legs literally, in order, capturing console verbatim with timestamps:

| Leg | Command | What must be true |
|---|---|---|
| Setup | `docker compose up -d postgres` · `make migrate` · `psql "$DATABASE_URL" -c "truncate subscribers"` | migrations applied, table empty (I add a confirming `select count(*)` → `0` as a precondition capture) |
| Action | `postbox import subscribers fixtures/subscribers.csv` | exits 0 |
| Assert | console contains `Imported 46 subscribers` | |
| Assert | console contains `2 duplicates skipped` | |
| Assert | console contains `2 invalid rows skipped (lines 17, 41)` | |
| Action | `psql "$DATABASE_URL" -tc "select count(*) from subscribers"` | |
| Assert | console contains `46` | |

I would evaluate each assert individually against the captured text — no "output looked right, all four pass".

**Delegation:** this is the one block I would hand to a disposable general-purpose worker on **sonnet**. Brief: run these exact commands in this exact order in this directory with this exported environment; do not use mocks, do not substitute SQLite, do not skip the truncate, do not evaluate any assertion, do not fix anything you find broken; return verbatim stdout/stderr, exit codes, and wall-clock timings for each command. On its return I read every assert against the raw text myself, and I personally re-run the two decisive commands — the `postbox import` invocation and the row-count query — because they are cheap and they are the whole gate. Any evidence that does not read back cleanly gets re-captured, never inferred. The delegation is named in the report.

**Extra captures beyond the card**, cheap and worth having for sign-off:
- `select source, count(*) from subscribers group by source` → expect `import | 46`.
- `select email from subscribers where email in ('dev3@northlight.example.co.uk','kai10@studio.example')` → confirms the two duplicate targets exist exactly once each, i.e. the duplicate counting is not masking a silent drop.
- Spot-check that a quoted field with an embedded comma landed intact: `select name from subscribers where email='ann0@example.com'` → expect `Ann, A.` This is the card's "quoted fields allowed" clause, which the gate's console asserts do not actually cover.
- A **re-run** of the same import against the now-populated table → expect `Imported 0 subscribers (48 duplicates skipped...)`. This is the card's "duplicates against existing subscribers" clause; the first run only proves within-file deduplication plus whatever the empty table gives. I mark this as my own added evidence, not a card assert.

## Phase 4 — Card fidelity beyond the gate

Walk each clause of the C2 sentence and record met / not met / not covered, with a file-and-line citation or a captured transcript for each:

- *one column of addresses plus optional name and tags* — covered by `CsvSubscriberImporter.run`'s tolerant field indexing; the `email`-only case is exercised by `test_import_skips_existing`.
- *quoted fields allowed* — covered by my Phase 3 spot-check.
- *"Addresses are normalised the same way sign-up does"* — **this is the clause I expect to report as not met.** `util/email.py` is a second, independent copy. Normalisation happens to be character-identical today, but validation is not: sign-up uses the `email-validator` library (which the README names as the project's address check), the importer uses a hand-written regex. Two code paths that may disagree are not "the same way"; today's fixture simply does not contain a case where they diverge. I would demonstrate the divergence concretely rather than assert it — a short throwaway comparison over a handful of addresses (e.g. quoted local parts, an over-length local part, a unicode domain) run through both `postbox.accounts.validators.is_valid_email` and `postbox.util.email.is_valid_email`, transcript captured. If they agree on everything I try, I say so and downgrade the finding to duplication-only.
- *invalid rows reported with their line number and skipped* — covered by the gate assert (`lines 17, 41`).
- *duplicates within the file and against existing subscribers, skipped and counted* — within-file by the gate, against-existing by my re-run capture.
- *`[EXTEND]` the CLI with an `import` command* — met; `cli.py` gained an `import` group with a `subscribers` subcommand. I'd also capture `postbox import --help` as evidence the command is discoverable.

## Phase 5 — Code-shape audit of what was built

Advisory only. I state findings with evidence and hand them to the lead's verdict; I do not fail the cycle on shape myself. Each rung claim in the write-up gets checked against the actual codebase rather than taken at its word.

1. **`util/email.py` duplicates `accounts/validators.py` (T2.2, rung claim "7 — write the minimum, no existing helper covered the import path").** The claim is false on inspection. `normalize_email` is character-for-character identical to `accounts/validators.py:6-12`. And "sign-up validates through a form, not a function we could call" is contradicted by `accounts/signup.py:4`, which imports both functions directly from `validators.py` — they are plain module-level functions. The card itself said "the address normalisation already exists from C1". This is the headline finding: dead duplication *and* a live behavioural divergence against a card requirement. Recommendation: delete `util/email.py` and `tests/test_email_util.py`, import from `accounts.validators` (or relocate `validators.py` to a shared home if the accounts package is the wrong owner). Correct rung: reuse.
2. **`csv_reader.py` reimplements the standard library (T2.1, rung claim "nothing in the codebase or the standard library handles quoted fields with embedded newlines").** The claim is false. `csv.reader` handles commas in quoted fields, doubled quotes, and embedded newlines. The one genuine wrinkle — the card wants the *starting* line of a record, and `csv.reader.line_num` advances past embedded newlines — is a few lines of counter, not a 45-line hand-rolled tokenizer. The hand-rolled version also carries holes the stdlib does not: bare `\r` outside quotes is silently dropped, a stray `"` mid-field opens a quote, no dialect handling. Recommendation: replace with `csv.reader` plus a line-tracking wrapper.
3. **`pipeline.py`'s Stage/Context/Pipeline machinery (T2.3).** An ABC plus four single-method classes plus a driver loop to express what is a straight-line loop of roughly fifteen lines over the rows. Its stated justification is the Mailchimp and Buttondown importers "planned for next quarter" — structure built for requirements that do not exist yet. The write-up's own Notes admit it is more than the card needs.
4. **`base.py`'s `Importer` ABC and `ImporterRegistry`.** One implementation, one registration, one lookup — and the CLI hard-codes the key it looks up (`registry.get("subscribers")`). `ImporterRegistry.names()` is never called anywhere: dead code. Only `ImportResult` earns its place. Also, T2.5's "one line registering the importer" undersells the change: `cli.py` gained a group, a command, an event subscription, and a print handler.
5. **`events.py` (T2.4, rung claim "2 — reuse existing").** The claim is self-refuting — the same report lists the file as new, and nothing else in the tree publishes or subscribes. A publish/subscribe bus with one event type, one publisher and one subscriber exists solely to move `click.echo(result.summary())` out of the command body, and it makes the CLI's output depend on an import side effect in `cli.py:24`. Recommendation: print the summary in the command; delete the bus.

Net shape read: of five tasks, three are speculative structure for a single-importer feature, one is a stdlib reimplementation, and one is a duplicate of existing code. The two rung claims that were checkable against the codebase (T2.2's "no existing helper", T2.4's "reuse existing") both fail the check.

## Phase 6 — Report, checkpoint, and branches

**Write:** `verification-report.md` at the repo root, beside `cycle-report.md`. Contents:
- Verdict per gate leg, each assert marked pass/fail with the verbatim console line that proves it.
- The quality-gate transcripts (`ruff`, `pytest`) with collected/passed counts, set against the write-up's "14 passed" claim.
- Card-fidelity table from Phase 4, including clauses the gate does not cover and how I covered them.
- The environment deviation (driver mismatch) with both transcripts, and the CI-never-touches-Postgres observation.
- The five code-shape findings, marked **advisory**, each with a file-and-line citation.
- Delegation disclosure: which legs a sonnet worker executed, which commands I re-ran myself.
- A reproduction section: exact commands, environment variables, image tags, package versions, timings.

**I would not tick the C2 checkbox in `tasks.md`.** That mark belongs to the sign-off decision, not to me; I'd note in the report that it is pending.

**Stop for a human here.** What I'd put in front of them: the gate result; the pytest discrepancy; the "normalised the same way sign-up does" clause and whether the duplicated validator is fix-now; the driver/CI gap; and the shape findings as advisory input. Branches:

- **Ruled fix-now** → cycle returns to the builder with findings 1 (delete `util/email.py`, import from `accounts.validators`) and the `pytest` failure as required work; I re-verify the full gate plus suite from Phase 2 on the fix, not just the changed test.
- **Ruled backlog** → I author the missed coverage as durable executable cases in the feature's `gates.md` (which does not exist in this workspace yet, so I would create it): a case asserting the importer and sign-up return identical verdicts across a shared address table; a case asserting the CLI connects to Postgres using the URL form the README documents; a case asserting a second import of the same fixture imports 0 and counts 48 duplicates. A one-time finding becomes permanent armor.
- **Ruled accept-as-is** → I record the acceptance and the accepting party in the report and leave the findings on file, unresolved but visible.

**My default if no ruling comes back: do not sign off.** My expected end state is a gate that passes on its four console asserts while the unit suite is red and the report's headline claim ("all tasks green — 14 passed") does not reproduce. A passing gate over an unverifiable write-up is exactly the situation that gets a checkpoint, not an approval.