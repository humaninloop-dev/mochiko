FILES-READ: `tasks.md`, `cycle-report.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/__init__.py`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/cli.py`, `src/postbox/events.py`, `src/postbox/accounts/__init__.py`, `src/postbox/accounts/validators.py`, `src/postbox/accounts/signup.py`, `src/postbox/util/__init__.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_import.py`, `tests/test_signup.py`

---

# Action plan — verify C2 (CSV subscriber import) and audit its code shape

Two jobs on one change: **execute** the C2 gate and the CI checks against real infrastructure and capture evidence, and **audit** the produced code for things that did not need to be written. The audit is advisory input to the sign-off verdict; it is not a gate I fail on my own authority. The whole run ends at a checkpoint.

---

## Phase 0 — Fix the scope of what I am verifying

**What I do:** Pin the surface to exactly what the card and the report define, so I don't grade C1 or the roadmap.

- Card: `tasks.md` §C2 — the checkbox behaviour statement plus the single `**TEST:** C2` case (2 Actions, 4 Asserts, capture=console).
- Change surface, from `cycle-report.md` "Files created / modified": `src/postbox/importers/{__init__,base,csv_reader,pipeline,subscribers}.py`, `src/postbox/util/{__init__,email}.py`, `src/postbox/events.py`, `src/postbox/cli.py`, `tests/test_{import,csv_reader,email_util}.py`.
- Everything else (`accounts/`, `models.py`, `db.py`, `migrations/`, `conftest.py`, `fixtures/subscribers.csv`) is **prior context**, read to judge the change but not itself under audit. `db.py` becomes relevant only because the gate exercises it.
- Quality gates, from `.github/workflows/ci.yml`: `ruff check src tests`, `make migrate`, `pytest -q`. The README repeats these, so this is the agreed check set.

**Classification of the C2 case: CLI.** Every Setup/Action leg is a shell command with machine-readable console output and no GUI or subjective judgment. That makes it eligible for auto-approval on a clean run — but only on a *clean* run, and I already have code-reading reasons (Phase 1) to expect it won't be clean, so I plan for the checkpoint path.

**Refusal noted up front:** the report's phrase "The C2 gate ran locally against the compose database and printed the expected counts" is a claim with no captured console attached. I do not carry it forward as evidence. I re-run the gate myself.

---

## Phase 1 — Static read-through, before touching a terminal

**What I do:** Read every file in the change surface plus the prior-context files above (already done for this plan), and build a list of *predictions* — concrete, falsifiable expectations about what execution will show. Predictions are hypotheses; execution decides.

**Predictions I record:**

| # | Prediction | Basis |
|---|---|---|
| P1 | The gate's fixture arithmetic is sound: 46 / 2 / lines 17, 41 | `fixtures/subscribers.csv` has 50 data rows after the header; line 17 `not an address` and line 41 `someone@` fail the validator; line 23 `dev3@NORTHLIGHT.EXAMPLE.CO.UK` is a domain-case dupe of line 5, line 47 `  kai10@studio.example ` is a whitespace dupe of line 12 — both collapse under `normalize_email`. 50 − 2 − 2 = 46. |
| P2 | The three console Asserts are satisfiable by one line. `ImportResult.summary()` emits `Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`; all three asserted strings are substrings of it. | `importers/base.py:16-19` |
| P3 | **The Action leg may not reach the database at all.** `db.py:6` defaults to `postgresql+psycopg://`, but the Setup leg's `psql "$DATABASE_URL"` and `Makefile:1` both use the bare `postgresql://` form, and `README.md:18` documents that bare form as the default. Under SQLAlchemy 2, bare `postgresql://` resolves to the **psycopg2** driver — and `requirements.txt` ships only `psycopg[binary]` (v3). Exporting the documented `DATABASE_URL` should make `postbox import subscribers` die with a missing-`psycopg2` import error. | `db.py`, `Makefile`, `README.md`, `requirements.txt` |
| P4 | **`pytest -q` will not report 14 passed.** I count 11 test functions: 3 in `test_csv_reader.py`, 3 in `test_email_util.py`, 3 in `test_import.py`, 2 in `test_signup.py`. | file contents |
| P5 | **`test_import_counts_and_persists` is likely red.** It writes `a@x.io` then `A@X.IO` and asserts `duplicates == 1`. But `normalize_email` deliberately keeps the local part as typed (`validators.py:7`, and `test_signup.py:9` locks that in), so `A@X.IO` normalises to `A@x.io` — a different string from `a@x.io`, absent from `ctx.seen` and not matched by the case-sensitive `==` in `DedupeStage`. Expected actual: `(3, 0, [4])` and a three-address set. | `pipeline.py:48-56`, `util/email.py:8-13` |
| P6 | `ruff check src tests` may flag `importers/base.py:18-19` (implicit string concat across lines) and the long call in `subscribers.py:26`; no ruff config exists in `pyproject.toml`, so default rules apply. Low confidence — execution decides. |

P3 and P5 together directly contradict the report's headline "all tasks green … 14 passed … printed the expected counts". I do **not** report that as a finding yet. I go execute.

**Nothing written in this phase.**

---

## Phase 2 — Stand up real infrastructure and run the quality gates

**What I do:** Bring up the actual Postgres 16 container from `docker-compose.yml` — no SQLite substitute, no stub — and run the three CI commands in CI's order, capturing verbatim stdout/stderr, exit code and wall time for each.

Commands, in order:
1. `docker compose up -d postgres`, then poll `docker compose ps` / `pg_isready -U postbox` until the healthcheck reports healthy (up to the 20×5s the compose file allows).
2. `pip install -r requirements.txt && pip install -e .`
3. `ruff check src tests`
4. `make migrate` (expect `applying migrations/0001_init.sql`, exit 0; the table is `create table if not exists`, so it is safe to re-apply)
5. `pytest -q`

**What I expect to see:** migrate clean; `pytest -q` reporting **11 collected** with at least `test_import_counts_and_persists` failing on the duplicate count (P4, P5). If it is instead 14 passed, my read is wrong and I say so plainly and investigate `test_import.py` again rather than defend the prediction.

**Delegation:** I would hand the mechanical execution of steps 1–5 to one disposable general-purpose worker at **sonnet**. Brief: run exactly these five commands in this order in this directory; do not modify any file; do not add, skip, deselect or `-k`-filter any test; do not substitute SQLite for the container; do not interpret or judge results. Return verbatim stdout+stderr, exit code and elapsed time for each command, plus the output of `docker compose ps`. On its return I read the raw output against my predictions, and I personally re-run the two decisive commands — `pytest -q` and `ruff check src tests` — myself, because a pass/fail claim on those is the load-bearing part of the sign-off. Any output that looks summarised, tidied or reconstructed rather than captured gets thrown away and re-captured by me. The delegation and the re-run are both disclosed in the report.

**What I'd stop on:** if the container will not come up healthy, or `pip install` fails, I do not fall back to running the suite against SQLite to get a green number. I stop, capture the failure, and take it to the checkpoint as *unable to verify* — an environment I cannot stand up is a reportable outcome, not a licence to approximate.

**Written:** nothing yet; raw evidence held for the report.

---

## Phase 3 — Execute the C2 gate case, leg by leg

**What I do:** Run the `**TEST:** C2` case exactly as written, in a shell where `DATABASE_URL` is exported to the value the README and Makefile document — `postgresql://postbox:postbox@localhost:5436/postbox` — because that is the environment an operator following the project's own docs will be in, and the Setup leg's `psql` invocation requires it.

- **Setup:** `docker compose up -d postgres` · `make migrate` · `psql "$DATABASE_URL" -c "truncate subscribers"` — all three run, none skipped. Capture each.
- **Action 1:** `postbox import subscribers fixtures/subscribers.csv` — capture full console and exit code.
- **Asserts 1–3:** evaluate each asserted substring against the captured console independently, and record the *actual* line verbatim next to each verdict rather than a bare pass/fail.
- **Action 2:** `psql "$DATABASE_URL" -tc "select count(*) from subscribers"` — capture.
- **Assert 4:** console contains `46`.

**If P3 lands** (Action 1 dies on the driver): the case fails at Action 1 and Asserts 1–3 are **not evaluated** — I record them as `not reached`, never as inferred passes, because a downstream assert whose action never ran has no evidence behind it. I then run one *diagnostic-only* extra pass with `DATABASE_URL=postgresql+psycopg://…` to establish whether the fault is the driver URL alone or the import logic underneath it. That second pass is labelled as diagnosis, clearly separated from the gate result, and it does **not** convert the gate to a pass — the case as written, in the documented environment, failed.

**If the diagnostic pass then prints the expected counts**, that is the most useful thing I can hand the lead: the import logic is right, the wiring between `db.py`'s URL scheme and the project's documented `DATABASE_URL` is wrong, and it is a one-line fix in `db.py` (or a requirements/doc correction) — but it is a real failure of the card's own gate today, and the report's "ran locally and printed the expected counts" only holds under a `DATABASE_URL` nobody's docs tell you to use.

**Delegation:** same shape as Phase 2 — a sonnet worker runs the Setup and both Action legs and returns raw console; **evaluating the four Asserts is mine and stays mine**, as does the decision about the diagnostic pass. Its brief forbids re-running the import to "get a better result", editing the fixture, or reordering the legs.

**Refusal:** I will not edit `db.py`, `requirements.txt`, `test_import.py` or anything else to make the gate go green. Fixing the build is the builder's job; my job is to show exactly what happens and hand it over.

---

## Phase 4 — Code-shape audit of what was built

**What I do:** Grade the five tasks in the report's decomposition table against the code actually on disk, and against what the codebase already offered before C2. Each claimed rung is a claim, so I go check it. Findings are advisory, evidence-cited, and carry a concrete cheaper alternative — I state them and leave them for the lead's verdict.

**Findings I already have from Phase 1, to be written up with file/line citations:**

- **T2.2 — `util/email.py` is a duplicate, and its justification is factually wrong.** The report says "No existing helper covered the import path; sign-up validates through a form, not a function we could call." `src/postbox/accounts/validators.py` exposes exactly two module-level functions, `normalize_email` and `is_valid_email`, and `accounts/signup.py:4` imports them as plain functions — there is no form. `util/email.py:8-13` is a line-for-line copy of `validators.py:6-12`. There is no form anywhere in the tree. This is my heaviest finding.
- **T2.2, second edge — the copy also *diverges*.** The original validates through `email-validator`, which `requirements.txt` pins and `README.md:8` names as the project's address checker "already used at sign-up". The copy hand-rolls a regex. The card explicitly requires that imported addresses are "normalised the same way sign-up does"; the codebase now has two definitions of a valid address that will drift apart. Cheaper shape: delete `util/email.py` and `util/__init__.py`, import from `accounts.validators`, and delete `tests/test_email_util.py`, which exists only to test the duplicate.
- **T2.1 — the hand-rolled tokenizer's justification is also wrong.** The report says "Nothing in the codebase or the standard library handles quoted fields with embedded newlines." Python's standard `csv` module does exactly that — quoted fields, embedded newlines, and doubled quotes — which is precisely the three behaviours `tests/test_csv_reader.py` exercises. The only thing `csv.reader` doesn't hand back directly is the record's *start* line, which is a few lines of wrapper around `reader.line_num`. That is ~40 lines of bespoke state machine plus 3 tests replaceable by a short wrapper over a battle-tested parser. I'll note the concrete risk of keeping it: the tokenizer silently treats a quote appearing mid-field as an opening quote (`csv_reader.py:29`), which no test covers.
- **T2.3 — structure built for a roadmap, not for this card.** `base.py` gives an abstract `Importer`, an `ImporterRegistry` and a global `registry` singleton; `pipeline.py` gives an abstract `Stage`, a `Context`, four one-method Stage classes and a `Pipeline` runner. All of it serves exactly one importer running exactly one fixed four-step sequence that no caller varies. The report's own note admits it — "a little more structure than the card strictly needs … the next two importers are already on the roadmap". The card asked to extend the CLI with one `import` command. Cheaper shape: the four stages are ~20 lines of straight-line loop body inside `CsvSubscriberImporter.run`; the abstraction can be extracted when the second importer actually exists and shows what varies.
- **T2.3, dead code:** `ImporterRegistry.names()` (`base.py:42-43`) has no caller anywhere.
- **T2.4 — the rung claim contradicts the report's own file list.** It is booked as "2 — reuse existing … Reused the codebase's existing event pattern", but `events.py` appears in "Files created / modified" as **(new)**, and there is no event bus, publisher, subscriber or observer anywhere in the pre-C2 tree (`accounts/`, `models.py`, `db.py`, `signup.py` contain none). So a new global event bus was written, and its sole purpose is to route one summary string from `subscribers.py:27` to the `click.echo` in `cli.py:20-24` that could have been called directly on the returned `ImportResult`. Cheaper shape: delete `events.py`, have `import_subscribers` echo `result.summary()`. It also introduces an import-time global side effect (`cli.py:24`) that will double-register handlers if `cli` is imported more than once in a test process.
- **Test-shape gap (advisory, not a rung finding):** `tests/conftest.py` binds every test to in-memory SQLite while the product targets Postgres 16 — so CI faithfully starts a Postgres service and runs `make migrate`, then runs a suite that never touches it. The migration SQL is unexercised by tests, and case/collation behaviour differs between the two engines, which is adjacent to the P5 failure. I flag this; I do not rewrite the fixture.

**Delegation:** none here beyond, at most, one haiku-tier lookup to confirm there is no other definition of `normalize_email`/`is_valid_email` or any pre-existing event/bus/publish symbol in the tree — a bounded enumeration whose answer I've already sighted by reading, used only to make the finding's evidence airtight. The judgment of every finding is mine.

---

## Phase 5 — Write the verification report

**What I write:** `verification-report.md` at the workspace root (or append to `cycle-report.md` under a `## Verification` heading if the lead prefers the record kept in one place — I'd default to a separate file so the builder's write-up and my findings stay attributable).

Contents:
1. **Verdict line** — my expectation, subject to Phase 2–3 evidence: **not signed off; back to the builder.** Two blocking items (the C2 gate failing at Action 1 under the documented `DATABASE_URL`; a red test in `pytest -q`) plus a report whose headline numbers do not reconcile with the tree.
2. **Gate result table** — each Setup/Action/Assert leg with verbatim captured console, exit code, timing, and per-assert verdict including explicit `not reached` where an upstream action failed.
3. **Quality gates** — ruff / migrate / pytest, verbatim output and exit codes, with the actual collected-test count set against the report's "14 passed".
4. **Discrepancies against `cycle-report.md`** — the test count, the "all green" claim, the unattached "ran locally and printed the expected counts" claim, and the three rung justifications that the code contradicts (T2.1 stdlib claim, T2.2 form claim, T2.4 reuse claim). Stated as observations with citations, not as accusations.
5. **Code-shape findings** — the Phase 4 list, each with file:line, the cheaper alternative, and a rough line-count delta; marked **advisory, for the lead's verdict**.
6. **What I confirmed works** — the fixture arithmetic and the summary-line format do satisfy the card's four asserts (P1, P2); normalisation collapses both planted duplicate forms correctly; the CLI surface matches the card's `postbox import subscribers <file>` exactly. Credit where the evidence supports it.
7. **Reproduction** — exact commands, env vars, container state, so anyone can re-run this.
8. **Delegation disclosure** — which legs a sonnet worker executed, which commands I re-ran myself.

---

## Phase 6 — Checkpoint

**The stop.** I do not auto-approve. Even though the case classifies CLI, the run is not clean, and the report contains claims the tree contradicts — that is squarely a human call. I present the evidence and ask the lead to rule on:

- **(a) The blocking failures** — my recommendation: return to the builder. Branch if the lead instead rules them environmental: I re-run the gate under whatever `DATABASE_URL` they specify and record the result, but the report keeps the original failure under the documented URL, because the operator-facing defect is real either way.
- **(b) Each code-shape finding — fix now, backlog, or accept.** My recommendation: fix now for `util/email.py` (a duplicated-and-diverging validator against an explicit "same way sign-up does" requirement is a correctness risk, and the fix is a delete plus an import); fix now for `events.py` (a delete); backlog the `csv` module swap and the pipeline/registry flattening as follow-up cards, since both are contained and the second importer will force the question anyway; accept-with-note the SQLite test fixture, raised as its own card.
- **(c) Whether the duplicate-detection semantics the card wants are actually what `test_import_counts_and_persists` assumes.** This is the one genuinely ambiguous point and it needs a product answer, not my guess: C1 deliberately preserves local-part case, but that test expects `A@X.IO` and `a@x.io` to be the same subscriber. Either the card means case-insensitive dedupe — in which case C1's normalisation is also implicated and this is bigger than C2 — or the test is simply wrong and should be corrected to match C1. I present both readings and let the lead decide. Branch: if the ruling is "case-insensitive dedupe is intended", I author it as a durable `**TEST:**` case into the feature's `gates.md` so it becomes permanent regression armour rather than a one-off correction, and flag that C1 needs re-verification.

**Default if no ruling arrives:** the cycle stays unsigned. C2's checkbox in `tasks.md` remains unticked, and no "verified" date is added. I don't tick a box on evidence that says otherwise.