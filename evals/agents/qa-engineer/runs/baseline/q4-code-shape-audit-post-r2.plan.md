FILES-READ: `tasks.md`, `cycle-report.md`, `README.md`, `pyproject.toml`, `requirements.txt`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `fixtures/subscribers.csv`, `src/postbox/__init__.py`, `src/postbox/db.py`, `src/postbox/models.py`, `src/postbox/cli.py`, `src/postbox/events.py`, `src/postbox/accounts/validators.py`, `src/postbox/accounts/signup.py`, `src/postbox/util/__init__.py`, `src/postbox/util/email.py`, `src/postbox/importers/__init__.py`, `src/postbox/importers/base.py`, `src/postbox/importers/csv_reader.py`, `src/postbox/importers/pipeline.py`, `src/postbox/importers/subscribers.py`, `tests/conftest.py`, `tests/test_import.py`, `tests/test_csv_reader.py`, `tests/test_email_util.py`, `tests/test_signup.py`

---

# Action plan — verify C2 (import subscribers from a CSV)

## Phase 1 — Pin the contract and hand-compute the expected numbers

**Do:** Fix what "pass" means before touching a terminal, so no result gets rationalised after the fact. The card's gate is four asserts on console text plus a row count. Every command is a CLI command — no GUI leg, no subjective judgement — so on a clean sweep this is auto-approvable on execution grounds; the shape findings below are what will actually route it to a human.

**Read (done):** `tasks.md` lines 19–27 (the gate), `fixtures/subscribers.csv`.

**Hand-derivation from the fixture, so I can tell a right answer from a lucky one:**
- Line 1 is the header `email,name,tags`; data rows are lines 2–51 = **50 records**.
- Line 17 `not an address` and line 41 `someone@` are the only unparseable addresses → **2 invalid, lines 17 and 41**, matching the card verbatim.
- Line 23 `dev3@NORTHLIGHT.EXAMPLE.CO.UK` collides with line 5 once the domain is lower-cased; line 47 `  kai10@studio.example ` collides with line 12 once stripped → **2 duplicates**.
- 50 − 2 − 2 = **46 imported**, **46 rows in the table**.

So the card's numbers are internally consistent with the fixture. Good — a failure here will be a code failure, not a card failure.

**Cross-check the emitted string against the asserts:** `ImportResult.summary()` (`base.py:16-19`) builds `Imported 46 subscribers (2 duplicates skipped, 2 invalid rows skipped (lines 17, 41))`. All three console asserts are substring matches and all three are satisfied by that one line. I note now that this means a single line carries all three asserts — if the counts are wrong, I expect three simultaneous failures, not one.

## Phase 2 — Environment preflight (I expect this to be the blocker)

**Do:** Establish the gate can even run before running it. I have a concrete, named suspicion from reading the tree:

`db.py:6` defaults to `postgresql+psycopg://...`, but `Makefile:1`, `ci.yml:15` and `README.md:18` all publish `DATABASE_URL` as `postgresql://...` with no driver. The gate's Setup leg *requires* that variable to be exported, because it runs `psql "$DATABASE_URL" -c "truncate subscribers"`. With it exported, SQLAlchemy 2 resolves bare `postgresql://` to the **psycopg2** dialect — and `requirements.txt:4` pins `psycopg[binary]` (psycopg **3**). psycopg2 is not installed anywhere. I expect `postbox import subscribers ...` to die at engine connect with `ModuleNotFoundError: No module named 'psycopg2'`.

**Delegation — Worker A**, a disposable general-purpose subagent, `model: sonnet`.
*Brief:* run exactly, in order, from the repo root: `pip install -r requirements.txt`, `pip install -e .`, `python -c "import psycopg2"`, `python -c "from postbox.db import engine; print(engine.url, engine.dialect.driver)"` first with `DATABASE_URL` unset and then with `DATABASE_URL=postgresql://postbox:postbox@localhost:5436/postbox` exported. Return verbatim stdout, stderr and exit code for each, plus `python -V` and `pip freeze`.
*Fence:* do not fix, patch, pin, install anything not in `requirements.txt`, or edit a file; do not interpret; do not skip a command that fails.
*On return:* I re-run the two `python -c` probes myself — they're one-liners and they're decisive — rather than accept the transcript.

**Branch:**
- If psycopg2 is genuinely absent and the exported-URL probe reports the psycopg2 driver, the gate's Action leg **cannot execute as specified**. That is not a pass and not a fail — it is a **blocked gate**, and I say so in exactly those words.
- To still produce evidence about the code under test, I would run the gate a second time with `DATABASE_URL=postgresql+psycopg://postbox:postbox@localhost:5436/postbox`, clearly labelled as **a deviation from the card's Setup**, and report both runs separately. I will not quietly substitute the working URL and call the card green.

**Refusal:** I will not edit `db.py`, the `Makefile`, `ci.yml` or `requirements.txt` to make the gate run. Fixing it is builder work; my job is to prove it's broken and hand that to the lead.

**Stop / human decision:** if preflight confirms the mismatch I flag it immediately rather than at the end, because the lead may want C2 held and a driver fix folded in before sign-off. What I'd ask: *does the driver mismatch get fixed inside C2 (it blocks C2's own gate and the first real CI run), or is it a separate card?* Onward branches — **fix-now**: C2 stays open, I re-verify after the fix; **separate card**: I record C2's gate as run-under-deviation and author the mismatch as a durable case. **My stated default if no answer:** report it as a blocking environment defect, present the deviation run as supplementary evidence, and recommend *not verified* pending the fix.

## Phase 3 — Quality gates

**Do:** Run the two commands CI runs (`ci.yml:21,23`).

**Delegation — Worker A (same spawn, sonnet):** `ruff check src tests` and `pytest -q -rA`. Return verbatim output and exit codes; do not fix a single lint or test failure.

**What I expect and what I'm specifically checking:**
- `ruff check` clean is plausible — there's no `[tool.ruff]` block in `pyproject.toml`, so the default rule set (E4/E7/E9/F) applies and line-length is not enforced. I'll note that the report's "ruff clean" is a weaker signal than it sounds.
- **The report claims `pytest -q` shows 14 passed. I count 11 test functions in the tree** — 3 in `test_import.py`, 3 in `test_csv_reader.py`, 3 in `test_email_util.py`, 2 in `test_signup.py`. I expect `11 passed`. If the run says 11, the write-up's headline evidence is wrong and every other number in it becomes a claim I have to check rather than read. I'd report the discrepancy plainly and ask the builder where 14 came from.
- **I will not accept a green pytest as evidence for this card.** `conftest.py:10` binds every test to `sqlite+pysqlite:///:memory:`. The suite never touches PostgreSQL. CI starts a Postgres service and runs `make migrate` and then runs a suite that ignores both. Only the C2 gate speaks to the card's actual requirement.

## Phase 4 — Execute the C2 gate against real PostgreSQL

**Delegation — Worker B**, disposable general-purpose subagent, `model: sonnet`.
*Brief:* export the DATABASE_URL decided in Phase 2; run `docker compose up -d postgres`, wait for the healthcheck, `make migrate`, `psql "$DATABASE_URL" -c "truncate subscribers"`; then `postbox import subscribers fixtures/subscribers.csv`; then `psql "$DATABASE_URL" -tc "select count(*) from subscribers"`. Capture verbatim console output, exit codes and wall-clock timing for every command.
*Fence:* no mocks, no SQLite, no fixture edits, no skipped setup step, no retry on failure, no tidying or summarising of output, and **no evaluation of the asserts** — raw evidence only.
*On return:* I read each captured line against each assert myself, and I personally re-run the decisive pair — the `import` command against a freshly truncated table, and the `select count(*)` — because those two carry all five asserts. Anything whose output doesn't read cleanly gets re-captured, never inferred.

**Assert evaluation (mine, one at a time):**

| Assert | Expected | How I judge it |
|---|---|---|
| Console contains `Imported 46 subscribers` | present | substring, verbatim console |
| Console contains `2 duplicates skipped` | present | substring |
| Console contains `2 invalid rows skipped (lines 17, 41)` | present | substring, **line numbers exact** |
| `select count(*)` console contains `46` | present | I re-run this myself |

**Supplementary evidence I'd also capture (labelled as beyond the card):** `psql -tc "select name, tags, source from subscribers where email='ann0@example.com'"`. The fixture's line 2 has a quoted name containing a comma (`"Ann, A."`), and the entire hand-rolled tokenizer of T2.1 exists to handle exactly that — yet **the card's gate never checks a quoted field survived to the database**, nor that `name`/`tags`/`source='import'` were written at all. That's a coverage hole in the card I authored the shape of, and I'd say so.

I would *not* silently add that assert to the card mid-verification — changing the contract during the run is exactly the move I distrust. I'd draft it as a proposed durable case and put it in front of the lead:

```
**TEST:** C2 — quoted fields and row detail survive the import
- **Setup**: docker compose up -d postgres · make migrate · psql "$DATABASE_URL" -c "truncate subscribers"
- **Action**: postbox import subscribers fixtures/subscribers.csv
- **Action**: psql "$DATABASE_URL" -tc "select name, tags, source from subscribers where email='ann0@example.com'"
- **Assert**: Console contains "Ann, A."
- **Assert**: Console contains "vip"
- **Assert**: Console contains "import"
- **Capture**: console
```

**Branch on gate outcome:** all four asserts pass → gate PASS, and the cycle's fate then turns entirely on Phase 5. Any assert fails → gate FAIL with the verbatim diff between expected and observed, no attempt by me to diagnose-and-fix. Cannot execute → BLOCKED, per Phase 2.

## Phase 5 — Code-shape audit of what was built

**Do:** Read every file the report lists against the codebase that already existed, and grade each of the builder's five rung claims by going and looking rather than believing the justification column. Three of the five justifications are checkable factual claims, and I've already checked them.

**F1 — `util/email.py` duplicates `accounts/validators.py`, and the justification is false. (major)**
`util/email.py:8-13` `normalize_email` is line-for-line identical to `accounts/validators.py:6-12`. The report's T2.2 reason is *"sign-up validates through a form, not a function we could call."* That is not true: `validators.py` exposes two plain module-level functions, and `signup.py:4` imports them exactly that way — `from postbox.accounts.validators import is_valid_email, normalize_email`. It was a function, it was callable, it was one import.
Worse, the copy **already diverged**: `validators.py:3` validates with the `email_validator` library (which `README.md:8` names as the project's address checker, "already used at sign-up"); `util/email.py:5` validates with a hand-rolled regex. The card requires addresses to be *"normalised the same way sign-up does"* — that holds today only by copy-paste, and the validation half is a different engine already. The two will drift on consecutive dots, quoted local parts, unicode domains, and length limits.
Rung claimed 7 (write the minimum); actual is 2 (reuse existing). Recommended: delete `util/email.py`, import from `accounts.validators`, retire or repoint `tests/test_email_util.py`.

**F2 — `csv_reader.py` reinvents the standard library, and that justification is false too. (major)**
45 hand-written lines of tokenizer, justified as *"Nothing in the codebase or the standard library handles quoted fields with embedded newlines."* The `csv` module does precisely that — commas, double-quoted fields, doubled quotes, embedded newlines — and `csv.reader.line_num` gives the line tracking the card's line-number reporting needs (record start = previous `line_num` + 1). The module and its three tests collapse to a handful of lines, and those three tests are currently asserting that Python's CSV semantics work.
The hand-rolled version is also **weaker than the thing it replaced**: `csv_reader.py:9` reads with `encoding="utf-8"`, so a BOM-prefixed export — and the card's whole premise is *"a CSV export of their previous tool"*, which is BOM-prefixed more often than not — leaves `\ufeffemail` in field 0, the header check at `subscribers.py:19` fails to match, and the header row becomes a 51st record and an invalid row. All three of the card's console counts would shift. Also, a bare `"` appearing mid-field toggles quoting (`csv_reader.py:29`) rather than being treated as literal. Minimum remedy `encoding="utf-8-sig"`; correct remedy, stdlib `csv`.

**F3 — T2.3 is a framework for importers nobody asked for. (major)**
`base.py` and `pipeline.py` supply an abstract `Importer`, an `ImporterRegistry` singleton, an abstract `Stage`, a `Context`, and a `Pipeline` runner — for **one** importer and **four** stages that are constructed in a fixed order at exactly one call site (`subscribers.py:26`) and never varied. The report's own reason names the driver: *"the Mailchimp and Buttondown importers planned for next quarter."* Those are not in this card or any card in `tasks.md`. Dead surface confirmed by search: `ImporterRegistry.names()` (`base.py:42-43`) has no callers, and `Importer.name` (`base.py:23`) is set on the subclass but never read — the registry keys off the string passed to `register()`. The `registry.get("subscribers")` hop at `cli.py:31` buys nothing over calling the class.
Rung claimed 7 ("write the minimum"); the minimum here is one function, `import_subscribers(session, path) -> ImportResult`, with the four steps inline — roughly 120 lines down to 35. `ImportResult` is worth keeping; it carries the counts the card asserts on.

**F4 — T2.4's rung claim is contradicted by the report's own file list. (major)**
T2.4 is graded *"2 — reuse existing … Reused the codebase's existing event pattern."* There is no existing event pattern. I searched the whole tree for bus/publish/subscribe/event: every hit outside the report itself is in a file **this cycle created** — `events.py`, `cli.py:6,24`, `subscribers.py:5,27`. And the report's own file list (line 22) marks `src/postbox/events.py` **(new)**. A newly invented abstraction was graded as reuse; the two statements cannot both stand.
On the merits it is pure indirection: one publisher, one subscriber, wired by a module-level side effect at `cli.py:24`, to move one `click.echo` out of the command body. Side effects: the handler list grows on re-import, and any programmatic caller of `.run()` now writes to the console. The card asks for a summary line; that is `click.echo(result.summary())` in the command. Recommended: delete `events.py`.

**F5 — the write-up's headline number doesn't match the tree. (accuracy)** "14 passed" vs 11 collectible tests. Confirmed by execution in Phase 3.

**F6 — the driver mismatch (Phase 2).** Not introduced by C2, but it blocks C2's own gate and would fail the first real CI run.

**F7 — the suite is SQLite-only.** `conftest.py:10`. "All tests green" is not evidence about PostgreSQL, the unique constraint on `subscribers.email`, or `server_default now()`. Advisory, and it explains how F6 survived to now.

**F8 — cosmetic, operator-visible.** `ImportResult.summary()` emits `Imported 1 subscribers` and, with no invalid rows, the dangling `0 invalid rows skipped (lines )` — `tests/test_import.py:30` pins the singular/plural form as correct. Neither the card's asserts nor the tests catch the empty-list case.

**Framing I hold to:** every one of these is **advisory input to the lead's verdict**, not a gate I fail on my own authority. F1–F4 are the "did not need to be written" cluster and together account for the large majority of the cycle's new code. F5's significance is not the number — it's that three of five rung justifications (T2.1, T2.2, T2.4) are factually contradicted by the codebase, so the write-up's self-assessment cannot be relied on and needs a builder pass.

**Delegation:** one disposable `Explore` subagent, `model: haiku`, for the bounded enumeration backing F1/F3 — *"list every call site, with file:line, of `ImporterRegistry.names`, `Importer.name`, and any import of `postbox.util.email`, across `src/` and `tests/`; facts and paths only."* One gap, one spawn. I've done the interpretive reading myself; I'd use its return only to corroborate the dead-surface claims, and I'd spot-check any file:line it names before it goes in the report.

## Phase 6 — Checkpoint and write-up

**Stop for human sign-off.** Even on a clean four-assert pass this does not get an auto-approval, because the shape findings need a verdict I don't own.

**What I'd put in front of the lead, in one screen:**
1. Gate result — PASS / FAIL / BLOCKED, with the four asserts and their verbatim evidence.
2. Quality gates — `ruff` and `pytest` exit codes and counts, with the 14-vs-11 discrepancy called out.
3. The environment blocker (F6) and the SQLite-only caveat (F7).
4. F1–F4 with the three contradicted rung justifications quoted against the code that contradicts them, plus an estimate of the code that would disappear.
5. The proposed extra acceptance case from Phase 4 and its coverage rationale.

**The decision I'd ask for:** *given the gate outcome, do F1–F4 go back to the builder before C2 is signed off, or are they backlogged with C2 verified as-is?*
- **Rework now** → C2 stays unticked; I re-run the full gate and re-audit after the changes, and F1/F2 will change behaviour (validator engine, BOM handling), so a re-run is mandatory, not a formality.
- **Backlog** → I author F1–F4 and F6 as durable `**TEST:**` cases in the feature's `gates.md` so the duplication, the BOM hole and the driver mismatch become permanent regression armour rather than one-time notes.
- **My stated default with no answer:** report the gate result honestly on its own evidence, recommend **rework before sign-off** on F1, F2 and F4 (each is a deletion, not a redesign, and F2 carries live BOM risk), leave F3 to the lead as a genuine judgement call about roadmap timing, and hold C2 unticked.

**Files I would write:** `verification-report.md` at the repo root — gate result with per-assert evidence, quality-gate output, the code-shape findings, the delegations made and what I re-ran myself, and the recommendation.

**Files I would not touch:** `tasks.md` (I do not tick C2's checkbox — that's the lead's verdict, and I'd propose the new acceptance case rather than insert it), `cycle-report.md` (the builder's write-up is his record; my corrections go in mine), and any source file. `gates.md` gets written only after a fix-now or backlog ruling.