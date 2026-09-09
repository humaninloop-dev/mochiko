FILES-READ: `tasks.md`, `.github/workflows/ci.yml`, `README.md`, `Makefile`, `docker-compose.yml`, `pyproject.toml`, `requirements.txt`, `src/quill/export.py`, `src/quill/app.py`, `src/quill/worker.py`, `src/quill/cli.py`, `src/quill/models.py`, `src/quill/db.py`, `migrations/0001_init.sql`, `migrations/0004_shares.sql`, `tests/conftest.py`, `tests/test_export.py`, `tests/test_share.py`, `fixtures/demo-notebook/manifest.json`, `fixtures/demo-notebook/README.md`, `fixtures/demo-notebook/01-welcome.md`, `fixtures/demo-notebook/03-code.md`, `fixtures/demo-notebook/04-images.md`, `docs/cycles-archive.md`

*(not read: `src/quill/__init__.py` — would open it in Phase 1 for completeness of the code-shape audit)*

---

# Verification plan — Quill C4 (export to PDF + share by link)

## Phase 0 — What "tests are green" is worth here (pre-flight, from reading only)

Before touching infrastructure I record what the developer's green suite actually covers, because it frames everything downstream. The suite is three tests: two in `tests/test_export.py` that call `render_html()` and assert on an HTML string, one in `tests/test_share.py` that round-trips a `Share` row through **in-memory SQLite** (`tests/conftest.py`). Nothing in the suite starts weasyprint, touches PostgreSQL or Redis, calls an HTTP route, or runs the worker. So "green on the developer's machine" carries no evidence about any of C4-a through C4-e. I will say exactly that in the report — not as criticism, as calibration, so nobody reads the green tick as coverage it isn't.

I also build a risk register from the read, each item a **hypothesis to be confirmed or refuted by execution**, never asserted from reading alone:

- **R1 — C4-a's assert string does not match the code.** `export.py:34` prints `exported {pages} page(s) to {out}`. The card asserts console contains `Exported 12 pages`. Case and capital letter and `page(s)` vs `pages`. The *count* should be right (manifest has 11 notes + 1 cover = 12).
- **R2 — C4-b's assert string does not match the wire format.** FastAPI returns compact JSON; `curl` prints `{"token":"…","url":"http://…"}` with no space after the colon. The card asserts `"url": "http://localhost:8300/s/`.
- **R3 — the documented `DATABASE_URL` may break the app.** `db.py:6` defaults to `postgresql+psycopg://…`, but README, Makefile and CI all use bare `postgresql://…`. With psycopg **3** installed and no psycopg2 (`requirements.txt`), exporting a URL in the documented form should send SQLAlchemy looking for psycopg2. The developer likely has no `DATABASE_URL` set at all, which is precisely the shape of a works-on-my-machine failure.
- **R4 — the page CSS may be silently dropped.** `export.py:33` passes a raw CSS *string* to `write_pdf(stylesheets=[PAGE_CSS])`; weasyprint treats list items as CSS objects, filenames, URLs or file objects. If it's being ignored, the A4 size, 18 mm margins and `pre { white-space: pre-wrap }` never apply — which bears directly on C4-e's "code blocks sit where they do on screen".
- **R5 — the share page's two payloads have no routes.** `app.py` serves `<img src='/previews/{token}.png'>` and `<a href='/s/{token}/download'>`, but there is no static mount for `/previews` and no download route anywhere in the file. C4-d's asserts (`Page contains "Download PDF"`) would pass on link *text* while the link 404s.
- **R6 — the worker doesn't render anything.** `worker.py:26` writes an 8-byte PNG signature with the comment *"placeholder until the renderer lands"*. C4-c's assert is satisfied by a log line (`worker.py:46`) that fires regardless. The card's story says the share page "renders a preview produced by the worker".
- **R7 — the demo fixture can't demonstrate what C4-e asks.** `manifest.json` lists 11 notes; only `01-welcome.md`, `03-code.md`, `04-images.md` exist. `cli.py:39` substitutes "Placeholder body for the demo notebook." for the other eight. The one image is `images/quill.png` (`04-images.md:3`) and there is no `images/` directory — and `HTML(string=…)` is constructed with no `base_url`, so a relative image reference has nothing to resolve against. So "images preserved" may be unverifiable with this fixture even in principle.
- **R8 — re-running the gate may fail.** `cli.py:33-35` deletes and re-inserts the demo notebook on re-seed, but `migrations/0004_shares.sql` gives `shares.notebook_id` a foreign key to `notebooks(id)`. Once C4-b/C4-c have created shares, C4-a's `make seed-demo` setup step on a second run should hit an FK violation.
- **R9 — migration numbering has holes.** `migrations/` contains `0001` and `0004` only. `make migrate` globs and sorts, so a fresh database gets whatever 0002/0003 would have added: nothing. C4 needs only `notebooks`, `notes`, `shares`, all of which are covered — but it means a fresh DB is not the same schema as the developer's, and README even advises skipping `make migrate` on an existing volume.
- **R10 — Playwright is not installed.** README says Playwright drives browser checks; it appears in neither `requirements.txt` nor CI. C4-d may be blocked outright.

## Phase 1 — Prerequisite and toolchain check

**Delegation.** One disposable `Explore` subagent, **model haiku**, one gap: *"Report presence and version for each of these on this machine, one line each, verbatim command output: `docker`, `psql`, `redis-cli`, `pdfinfo`/`pdftotext`, `python -c 'import weasyprint; print(weasyprint.__version__)'`, `python -c 'import playwright'`, `pip show psycopg2-binary`. Do not install anything, do not modify anything."* On return I check that every one of the seven has real output attached rather than a summary judgement, and I re-run by hand any that came back ambiguous. I do not let it draw conclusions.

Why these: `make migrate` shells out to `psql`; C4-c's second action shells out to `redis-cli`; weasyprint on macOS needs Homebrew pango/cairo present or the import itself fails; `pdfinfo` is how I will get objective page-geometry evidence for R4; the psycopg2 probe settles R3's precondition.

**Refusal/stop.** If weasyprint won't import or `psql`/`redis-cli` are missing, I stop and report the gate as **blocked on environment**, with the exact missing pieces. I will not substitute a mock PDF renderer or a Python Redis call in place of `redis-cli` to make a case "run" — a case executed against a substitute has not been executed.

## Phase 2 — Clean, non-destructive infrastructure

`docker compose up -d postgres redis` as the card says. I will **not** run `docker compose down -v`: that volume is the developer's local data and destroying it is not my call. **Stop point:** I'd confirm with the lead whether wiping is acceptable. If yes, wipe and migrate from scratch — cleanest. If no (my default, and what I proceed with here), I take the non-destructive path: create a scratch database `quill_verify` on the same server via `psql`, and run `make migrate DATABASE_URL=postgresql://quill:quill@localhost:5434/quill_verify`. That gives me a virgin schema without touching their data, and it also exercises R9 — I capture the `applying migrations/0001_init.sql` / `applying migrations/0004_shares.sql` lines verbatim, which is the evidence that 0002 and 0003 do not exist.

Then I probe R3 deliberately, because a documentation bug that bricks the app is worth catching once rather than by every future operator:
1. Export `DATABASE_URL=postgresql://quill:quill@localhost:5434/quill_verify` exactly as README/Makefile/CI write it, and run `python -m quill.cli seed --notebook demo --from fixtures/demo-notebook`. I expect either a psycopg2 import error or a clean seed; whichever happens, the verbatim output is the evidence.
2. If it fails, re-export as `postgresql+psycopg://…` and re-run. The gate proceeds on the working form, and the failure of the documented form is filed as a finding against README/Makefile/CI — not quietly worked around.

Then `make seed-demo` (pointed at the scratch DB) and I record its output: expect `seeded notebook 'demo' with 11 notes`. I also capture, from `psql`, the count of notes whose body starts with "Placeholder body" — that is the hard number behind R7 and I want it in the report rather than an impression.

Evidence lands in `/tmp/quill-verify/evidence/` (`00-prereqs.txt`, `01-migrate.txt`, `02-seed.txt`, and so on). Everything is `tee`'d; nothing is retyped from memory.

## Phase 3 — The checks CI runs

Run the three commands from `ci.yml` steps 25-28 locally against the same source: `ruff check src tests`, `mypy src`, `pytest -q`.

**Delegation.** One general-purpose subagent, **model sonnet**, briefed: *"From the repository root with this virtualenv active, run these three commands in order, each independently, and return for each the exact command, complete stdout+stderr verbatim, and the exit code. Continue past failures — do not stop at the first non-zero. Do not fix, edit, reformat, install, or re-run anything. Do not interpret the output. Do not touch the database or Docker."* On return I verify each block carries an exit code and looks like real tool output, and I personally re-run any command it reported as failing before I write that failure down. Judgement on what the results mean stays with me.

What I expect and will state precisely: three tests collected and passing; ruff clean under its **default** rule selection. Note for the report — `pyproject.toml:2` sets `line-length = 100` but ruff's default rules don't include the line-length check, so the several lines over 100 (`app.py:45`, `worker.py:46`, `tests/test_export.py:7`) are not enforced. That setting is decorative today. Mypy runs `strict` over the `quill` package only (`pyproject.toml:9`), so `tests/` is unchecked. Both are advisory notes, not gate failures.

## Phase 4 — C4-a, export to PDF (CLI, machine-verifiable)

Setup already done in Phase 2, plus `mkdir -p /tmp/quill-verify`. Run the action exactly as written:

`python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`

- **Assert 1 — file exists.** `ls -l` plus `file` on the output. I expect this to pass.
- **Assert 2 — console contains "Exported 12 pages".** Evaluated **literally against captured stdout**. Per R1 I expect the console to read `exported 12 page(s) to /tmp/quill-verify/demo.pdf` and this assert to **FAIL** on the string, while the count 12 is correct. I record it as a fail with both strings quoted side by side, and carry the remedy to the checkpoint as two branches — amend `export.py:34` to the card's wording, or amend the card's wording to the code. My recommendation would be to change the code, since the card is the behaviour that was agreed at design time and this is user-facing copy; but choosing is the lead's call, and I will not quietly rewrite the assert to match the build. A test edited to fit the code it is testing is not a test.

**Extra evidence beyond the card, for R4 and R7:** `pdfinfo /tmp/quill-verify/demo.pdf` for page count and page geometry, and `pdftotext` to a text file. If the page size is not 595×842 pt with real margins, the stylesheet is being dropped and I have objective proof rather than a hunch. I also compare the `pdfinfo` page count against the console's "12" — the code counts *notes + cover* (`export.py:31`), which is not the same thing as PDF pages once a note runs long, so the two numbers may legitimately diverge. That is a naming honesty issue worth surfacing. And I grep the extracted text for "Placeholder body" to quantify R7.

## Phase 5 — C4-b, create a share link (CLI, machine-verifiable)

Start `uvicorn quill.app:app --port 8300` in the background with stdout/stderr to `evidence/uvicorn.log`, using the `DATABASE_URL` form that Phase 2 established actually works, and wait for readiness rather than sleeping blind.

Run the `curl` exactly as the card writes it, capturing body and status.

- **Assert — status 201.** Expect pass (`app.py:22`).
- **Assert — console contains `"url": "http://localhost:8300/s/`.** Per R2, expect **FAIL** on the literal, because compact JSON has no space after the colon. I then re-run the same request piping through `python -m json.tool` to demonstrate on the record that the substantive behaviour — a well-formed `url` on the right host with the `/s/` prefix — is correct. The report will say plainly: literal assert failed, underlying behaviour verified, the case text needs amending. Two separate facts, both stated.

I save the returned `token` and `url` to `evidence/share-b.json`; C4-d depends on that exact URL.

**Extra evidence for R5:** `curl -i` the `/s/{token}` page, then `curl -i` the two things that page points at — `/s/{token}/download` and `/previews/{token}.png`. Neither has a route in `app.py`, so I expect 404s. This is outside the card's asserts and I say so; it is reported as a gap finding with the response codes attached, because C4-d as written will show a page that *looks* right while both of its payloads are dead.

## Phase 6 — C4-c, the worker renders the preview (CLI, machine-verifiable)

Start `python -m quill.worker` in the background, logging to `evidence/worker.log`. The card's setup does not export `REDIS_URL`, but its second action interpolates `"$REDIS_URL"` into `redis-cli` — so I export `REDIS_URL=redis://localhost:6380/0`, matching `db.py:7` and `ci.yml:19`, and I note in the report that the case is under-specified on this point.

Run the POST action, then evaluate:

- **Assert — console contains "rendered share preview" within 15 s.** Expect pass from `worker.py:46`. But I corroborate rather than accept the substring: I match the token in the worker's log line against the token returned by the POST, so I know *this* request was serviced and I'm not reading a stale line from the C4-b share that is also sitting in the queue.
- **Assert — `llen quill:share-previews` is 0.** Expect pass. I flag it as a weak assert: an empty queue is equally consistent with the worker draining it and with the app never having pushed. To make it mean something I capture `llen` *before* starting the worker in a controlled repeat (post a share with the worker stopped, observe the queue at 1, start the worker, observe it drop to 0). Same assert, real evidence behind it.

**Gap finding for R6, reported regardless of the asserts passing.** I `ls -l` and `file` the produced `/tmp/quill-previews/{token}.png`. Per `worker.py:26` I expect an 8-byte file that is a PNG signature and nothing else — not a decodable image. Both of C4-c's asserts pass on that. The card's story promises "the share page renders a preview produced by the worker", and a stub with a comment saying the renderer hasn't landed does not meet it. This is the single most important thing in my report: the case as authored cannot distinguish a working renderer from a placeholder, and the placeholder is what shipped. I state it as an observation with the file size, the `file` output and the source line, and I let the lead rule on it.

## Phase 7 — Reproducibility probe (R8)

Not in the card; I run it because a gate that can only pass once isn't a gate. With shares now in the database, re-run `make seed-demo` — C4-a's own setup step. I expect a foreign-key violation from `cli.py:35` deleting a notebook that `shares.notebook_id` still references. If it fails, I capture the error verbatim and report that the C4 gate is not re-runnable from a used database, which also means CI could not run this sequence twice. If it succeeds, I record that too and drop R8.

## Phase 8 — C4-d, the share page in a browser (GUI → checkpoint, never auto-approved)

This case involves a browser and a screenshot; it does not get an automated pass from me under any outcome. If Phase 1 found Playwright missing (R10), the case is **BLOCKED**, not failed — and I would **stop** rather than `pip install playwright && playwright install`, because installing browser binaries changes machine state beyond the verification's remit. Branches I'd put to the lead: (a) authorize the install and I run the navigation and capture the screenshot for human review; (b) I do a manual browser walkthrough against the C4-b URL and attach the screenshot; (c) defer C4-d and record it unverified. **Default if no answer: (b)** — I open the URL from `evidence/share-b.json`, capture a full-page screenshot to `evidence/c4d-share-page.png`, and note in the report exactly how the evidence was produced.

Either way I present the screenshot alongside the Phase 5 finding: the human is being asked to look at a page whose preview image and whose "Download PDF" link both 404. The assert `Page contains "Download PDF"` is likely to pass on text alone. I will not let a passing string assert be reported as a working download.

## Phase 9 — C4-e, does the PDF read like the notebook (subjective → checkpoint)

"Headings, code blocks, and images sit where they do on screen and the cover page looks finished" is a human judgement. I do not render a verdict on it. What I do is make the human's judgement cheap and well-founded: attach `/tmp/quill-verify/demo.pdf`, the `pdfinfo` geometry, the `pdftotext` extraction, and three specific things for their eye:

1. **Eight of eleven sections are placeholder text** (R7, with the count measured in Phase 2/4). The person is being asked whether the PDF "reads like the notebook" when most of the notebook isn't in the fixture.
2. **The only image is a broken reference** — `images/quill.png` doesn't exist in `fixtures/demo-notebook/`, and `HTML(string=…)` has no `base_url` to resolve a relative path against even if it did. So "images preserved", which the card's story names explicitly, is not demonstrable by this case with this fixture. That's a hole in the *case*, and it needs closing whatever the verdict on the code.
3. **Whether the page CSS applied** (R4), from the Phase 4 geometry evidence — because if it didn't, the code-block wrapping and margins they're being asked to bless are weasyprint defaults, not Quill's intent.

**Stop point.** C4-e ends in a human ruling: accept the PDF as-is, accept-with-followup, or reject. Onward branch for each: accept → C4-e passes and I record who signed and on what evidence; accept-with-followup or reject → I author the follow-up as a durable executable case (Setup/Action/Assert) rather than a prose note, so it re-runs forever. My default while planning is that C4-e cannot be signed clean, because point 2 means the case does not cover a behaviour the card explicitly promises.

## Phase 10 — Code-shape audit of what C4 produced

Against the card's declared exposure (`[EXTEND] export.py` for the cover page, `[EXTEND] app.py` for share routes, `worker.py` new), reading the code myself rather than taking the shape on trust. Candidate findings, each evidence-cited, each **advisory input to the lead's verdict and never a gate I fail on my own authority**:

- **Unrequested optionality in export.** `cover: bool` threaded through `render_html` and `export_pdf`, plus a `--no-cover` CLI flag (`cli.py:21`). No story, no case, and no caller asks for a cover-less export; one of the two existing unit tests exists only to cover the branch that optionality created. The card asked for a cover page, not a cover toggle.
- **A parameter that is accepted and never read.** `cli.py:19,22` takes `--format` as `Choice(["pdf"])` and the body never uses `fmt`. A one-value choice that nothing consumes.
- **A return value nobody consumes.** `export_pdf` returns `pages`; `cli.py:23` discards it.
- **A column with no reader.** `Share.created_at` (`models.py:39`, `migrations/0004_shares.sql:7`) is written by the default and read nowhere.
- **Code that likely executes to no effect.** `PAGE_CSS` and its `stylesheets=` argument, pending the Phase 4 geometry evidence (R4). If confirmed, that's three lines that look like layout control and aren't.
- **Test fidelity.** The suite's only DB fixture is in-memory SQLite (`conftest.py:10`) against a product that is PostgreSQL-only with raw-SQL migrations; the migrations themselves are never exercised by any test.
- **A decorative lint setting.** `line-length = 100` with no rule selected to enforce it.

## Phase 11 — Write-up and sign-off checkpoint

I would write `docs/verification/C4-report.md` (new directory), with evidence in `/tmp/quill-verify/evidence/` referenced by filename. Structure: per-case verdict table (C4-a … C4-e, each PASS / FAIL / BLOCKED / AWAITING-HUMAN with the exact assert text and the exact captured output beside it); then the CI-check results with exit codes; then gap findings ordered by weight; then the advisory shape findings; then every delegation I made, what I asked for, and which of its results I re-ran myself before trusting.

I would **not** tick the C4 checkbox in `tasks.md`. That tick is the lead's verdict, not mine; I propose the edit and leave it.

The headline I expect to be presenting, stated as findings and not as a verdict:

1. **The build cannot be signed off on the strength of "tests are green."** Three tests, none of which touch the PDF renderer, the routes, the worker, Postgres or Redis.
2. **Two cases fail on their literal assert text while the underlying behaviour is right** (C4-a, C4-b). Case-authoring defects — mine to own and mine to fix, in whichever direction the lead rules.
3. **The worker ships a placeholder, and C4-c passes anyway.** The gap the case was supposed to catch is the gap it can't see.
4. **The share page's preview image and download link have no routes**, so C4-d's asserts pass on text over a broken page.
5. **C4-e cannot demonstrate "images preserved"** with the current fixture — a hole in the case, independent of the code.
6. **The documented `DATABASE_URL` may not work with the shipped driver** (pending Phase 2's evidence), and the gate may not survive a second run (pending Phase 7's).

Given items 3-5, my recommendation at the checkpoint would be that C4 is **not ready to close**, with the branches spelled out: if the lead rules the worker placeholder is fix-now, C4 goes back and I re-run the full gate on return; if it's ruled backlog, I author it as a durable executable case so no future cycle can pass over it silently. Either way, items 2 and 5 are case repairs I do before this gate is trusted again, and item 3's blind assert gets rewritten to check the produced PNG is a decodable image rather than checking that a log line was printed.