FILES-READ: `tasks.md`, `.github/workflows/ci.yml`, `README.md`, `Makefile`, `docker-compose.yml`, `pyproject.toml`, `requirements.txt`, `src/quill/db.py`, `src/quill/models.py`, `src/quill/export.py`, `src/quill/app.py`, `src/quill/worker.py`, `src/quill/cli.py`, `migrations/0001_init.sql`, `migrations/0004_shares.sql`, `tests/conftest.py`, `tests/test_export.py`, `tests/test_share.py`, `fixtures/demo-notebook/manifest.json`, `fixtures/demo-notebook/README.md`, `fixtures/demo-notebook/03-code.md`, `fixtures/demo-notebook/04-images.md`, `docs/cycles-archive.md`

# C4 Verification Plan — Quill (export to PDF + share links)

## Phase 0 — What the read already tells me (pre-execution hypotheses)

I have read the card, the CI file, all of `src/quill/`, both migrations, all three test files, and the demo fixture. Everything below is a **hypothesis to be confirmed by execution**, not a result. I record them up front so that the run is a test of them rather than a hunt, and so nobody can later claim I retro-fitted the story.

- **"Tests green" says almost nothing about C4.** `tests/conftest.py` builds a SQLite in-memory engine; `tests/test_export.py` only calls `render_html` (a pure string function, never `export_pdf`, never WeasyPrint); `tests/test_share.py` only round-trips a `shares` row through SQLite. Nothing in `tests/` exercises the export CLI, `POST /share`, `GET /s/{token}`, or `quill/worker.py`. A green `pytest` on the developer's machine is not evidence for any of C4's five cases.
- **C4-a's string assert cannot match as written.** The card asserts console contains `Exported 12 pages`; `export.py:34` prints `exported {pages} page(s) to {out}` — lowercase, and `page(s)`. The *count* should be 12 (manifest lists 11 notes + 1 cover), but the literal substring will be absent.
- **`pages` is a section count, not a PDF page count.** `export.py:31` is `len(notes) + 1`. There is no page-break rule anywhere: `PAGE_CSS` (`export.py:12`) sets only `@page` size/margin and `pre` wrapping. So "one note per section" as a *page* promise is unbacked, and the printed 12 is arithmetic, not measurement.
- **The stylesheet is likely never applied.** `HTML(string=html).write_pdf(str(out), stylesheets=[PAGE_CSS])` passes a raw CSS *string* where WeasyPrint expects a filename/URL/CSS object. Expect a "failed to load stylesheet" style warning on stderr and default page geometry. Must capture stderr, not just stdout.
- **Images cannot resolve.** `04-images.md` references `images/quill.png`; no `images/` directory exists in the fixture, and `HTML(string=...)` is constructed with no `base_url`, so even a present relative image would not resolve. The card's headline promise "images preserved" and C4-e's assert are both aimed straight at this.
- **The worker's preview is an admitted stub.** `worker.py:26` writes an 8-byte PNG signature with the comment "placeholder until the renderer lands". C4-c's assert only looks for the log line `rendered share preview`, so the assert can pass over a stub. This is the single most important thing to surface: the card's headline says "the share page renders a preview produced by the worker."
- **Two dead links on the share page.** `app.py:45` emits `<img src='/previews/{token}.png'>` and `app.py:47` emits `<a href='/s/{token}/download'>`. There is no static mount and no download route in `app.py`, which is the only router. C4-d asserts only that the text `Download PDF` is present — it passes while the link 404s.
- **CI's own env may break the app.** `db.py:6` defaults to `postgresql+psycopg://`, but `ci.yml:18` exports `DATABASE_URL=postgresql://…`, which SQLAlchemy 2 resolves to the psycopg2 dialect; `requirements.txt` ships only `psycopg[binary]` (v3). `create_engine` at import time would then fail, and `tests/test_export.py` imports `quill.export` → `quill.db`. Hypothesis: pytest is green locally because the developer's shell has no `DATABASE_URL` (or the `+psycopg` form), and red under CI's env.
- **`mypy --strict` on `src` has untyped imports.** `weasyprint` and `markdown` ship no type information and `pyproject.toml` has no per-module overrides, so strict mode should error on `src/quill/export.py`. (Ruff's default rule set does not include line-length, so the long lines in `app.py:45` and `test_export.py:7` are *not* violations — I will not report them.)
- **Migration numbering has a hole.** `migrations/` contains `0001` and `0004` only. I cannot know what `0002`/`0003` held; `0001` already creates `notebooks` and `notes`, and `models.py` needs no column that is missing. So this is probably not a C4 blocker, but it means a fresh database is not provably the schema C1–C3 were verified against. Flag, do not claim.
- **The README's shortcut is the "works on my machine" mechanism.** README lines 22–23 invite skipping `make migrate` when the named `quill-pg` volume already exists. A warm volume is exactly how a stale/incomplete schema stays invisible.

**I refuse to take that shortcut.** The card's Setup says `make migrate`, and I will run C4 against a database whose provenance I can state.

## Phase 1 — Pin the environment and record it as evidence

Actions (all recorded verbatim into the evidence directory):

1. Create `/tmp/quill-verify/c4-evidence/` for every capture; every command gets its own `NN-<name>.log` with the exact command line, exit code, and wall time as the first lines.
2. Record the starting world: `docker compose ps`, `docker volume ls | grep quill-pg`, `docker volume inspect quill-pg` (creation date), `python -V`, `pip freeze`, `env | grep -E 'DATABASE_URL|REDIS_URL'`, `which redis-cli psql`, `weasyprint --version`, `playwright --version`.
3. Decide the volume question. If `quill-pg` already exists, I will **not** verify on it silently. I run the card twice-over on the schema question: first `make migrate` against the existing volume (that is literally what the card says), then, for the record, `docker compose down`, `docker volume rm quill-pg`, `docker compose up -d postgres redis`, `make migrate` — and the C4 cases run against the **fresh** volume, because that is what CI and any new install get. Both migrate logs go in evidence.
4. Export the environment the app actually needs and log that I did: `DATABASE_URL=postgresql+psycopg://quill:quill@localhost:5434/quill`, `REDIS_URL=redis://localhost:6380/0`. The card's C4-c uses `redis-cli -u "$REDIS_URL"`; unset, that would silently talk to a default `localhost:6379` and could return `0` for the wrong reason. I will not let that assert pass on an unpinned variable.

**Stop #1 (environment divergence).** Before running the cases I put one question to the lead: the card and CI disagree with `db.py` on the `DATABASE_URL` form. What do I verify against?
- Ruling "verify as CI runs it" → I run C4 with `postgresql://…` and expect app/CLI startup failure; C4-a through C4-d fail at Setup, and C4 does not pass.
- Ruling "verify as README/`db.py` intend" → `postgresql+psycopg://`, and the CI/`db.py` mismatch is filed as a defect found during verification.
- **My default while waiting: I do both** — the `+psycopg` form for the C4 cases (so the cycle gets a fair hearing) and a separate CI-env reproduction in Phase 2. Both results go in the report; the disagreement is reported either way, because a cycle whose CI cannot boot the app is not signed off on my say-so.

## Phase 2 — Reproduce CI's gates myself, under CI's env

I do not accept "green on the developer's machine." I run, with `DATABASE_URL`/`REDIS_URL` set exactly as `ci.yml:18–19`, against the compose services on their real ports:

- `pip install -r requirements.txt` (into a clean venv, log resolved versions)
- `ruff check src tests` — expect clean (default rule set; no line-length rule active)
- `mypy src` — **expect failure** on missing type information for `weasyprint` and `markdown` in `src/quill/export.py`
- `make migrate` — expect success (0001, 0004 apply; `create table if not exists` is idempotent), and I log which files were applied so the 0002/0003 hole is on the record
- `pytest -q` — **expect collection-time failure** (`psycopg2` module missing) under CI's `DATABASE_URL`; then re-run with `postgresql+psycopg://` and with the variable unset, to isolate the cause and prove which shell state produces "green"

I additionally record what the suite covers, by reading it, not by trusting the count: three tests, zero of which touch Postgres, Redis, WeasyPrint, the CLI, the routes, or the worker. That sentence goes in the report next to whatever pass count `pytest` prints.

**Delegation here.** Running these five commands and returning raw output is fully specified work: I would hand it to one disposable general-purpose worker at the `sonnet` tier. Brief: create the venv, export exactly these two variables, run exactly these five commands in this order, do not stop on first failure, do not fix anything, do not modify any file, do not install packages beyond `requirements.txt`, return for each command the verbatim stdout+stderr, exit code, and duration. On return I read every exit code against the command, and I personally re-run `mypy src` and `pytest -q` myself, because those two carry the verdict and a worker's transcript is a claim until I have seen the terminal. Any output that looks summarized rather than verbatim gets re-captured by me, not accepted.

## Phase 3 — C4-a: does the demo notebook export to a PDF

Setup, exactly as the card: compose up, `make migrate`, `make seed-demo`, `mkdir -p /tmp/quill-verify`.
Action: `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`, capturing stdout **and** stderr.

Evidence I capture beyond the card's minimum, because the card's asserts are weaker than its own promise:

- `psql "$DATABASE_URL" -c "select count(*) from notes n join notebooks b on b.id=n.notebook_id where b.slug='demo'"` → expect 11, and the seed line `seeded notebook 'demo' with 11 notes`
- `ls -l /tmp/quill-verify/demo.pdf`, `file demo.pdf`, sha256
- **Actual PDF page count** (`pdfinfo`, or a two-line `pypdf` read) — this is the measurement the card's "12 pages" only pretends to be
- Extracted text (`pdftotext -layout`) grepped for each of the 11 note titles and for `Placeholder body for the demo notebook`
- stderr grepped for stylesheet/image loading warnings

Assert evaluation:
- *File exists* — expect **PASS**.
- *Console contains "Exported 12 pages"* — expect **FAIL on the literal string**, with the observed line quoted byte-for-byte. I will not normalize case or wording to make it pass, and I will not edit `export.py` or the card to close it. I report it as a wording mismatch with the count apparently correct, and hand the lead the two possible corrections (align the print, or align the assert) as *their* call. Separately I report the substantive version: even if the string matched, 12 is a section count, and my measured page count is the honest number.
- Expected secondary findings: 8 of the 12 sections carry the fixture's placeholder body (this is by fixture design per `fixtures/demo-notebook/README.md`, so context for C4-e, not a defect), and the one real image is absent/unresolvable (a defect against "images preserved").

**Delegation.** Setup + action legs can go to a `sonnet` worker with an explicit fence: run these four setup commands and this one action, capture stdout and stderr separately with exit codes, do not evaluate any assert, do not retry a failure, do not create or edit files other than the output PDF. I evaluate the asserts and re-run the export myself once so the PDF I later put in front of a human is one I produced.

## Phase 4 — C4-b: share link creation

Setup: `uvicorn quill.app:app --port 8300` in the background, log captured to file; poll for readiness rather than sleeping blind; hard 30s cap, and if it does not come up I stop and report Setup failure rather than proceeding.
Action: the card's `curl -s -w "\n%{http_code}"` POST.

- *Response status 201* — expect **PASS** (`app.py:22`).
- *Console contains `"url": "http://localhost:8300/s/`* — this depends on exact JSON spacing. FastAPI's default `JSONResponse` emits compact separators (`{"token":"…","url":"…"}`), so the asserted substring with a space after the colon will very likely **FAIL on the literal string** while the value is correct. I report the raw response body verbatim and grade the assert as written, flagging it as a brittle formatting assert that should be rewritten against the parsed `url` field.
- Extra evidence: `psql` row for the new token, and `redis-cli -u "$REDIS_URL" llen quill:share-previews` → expect `1` (no worker running yet). I record the token and URL for C4-d.

## Phase 5 — C4-c: does the worker render the preview

Setup: `python -m quill.worker` in the background, output to a log; confirm `worker listening on quill:share-previews` before acting.

Sequencing note I will handle explicitly: the token from C4-b is still queued, so the worker drains **two** items. I record queue depth immediately before starting the worker (expect `1`), then after (expect `0`), so that `0` demonstrably means *drained* rather than *wrong Redis instance* — and I also `redis-cli -u "$REDIS_URL" ping` and `info server` to prove which instance I queried. The card's `Assert: Console contains "0"` is too weak to carry a verdict on its own; I say so and back it with the before/after pair.

- *Console contains "rendered share preview" within 15s* — expect **PASS** on the literal log line.
- *Queue length 0* — expect **PASS**, on pinned evidence.
- **But I will not report C4-c as satisfying the card.** `worker.py:26` writes an 8-byte PNG signature and nothing else, self-described as a placeholder. I capture `ls -l /tmp/quill-previews/<token>.png` (expect 8 bytes), `file` on it (expect "PNG image data" with no dimensions / truncated), and the `shares.preview_path` value. Conclusion I will state plainly: the asserts pass and the deliverable is a stub — the card's promise that "the share page renders a preview produced by the worker" is **not met**. This is the finding I would refuse to bury even if every assert on the card were green.

## Phase 6 — C4-d: the share page in a browser (GUI → human decision)

Action: Playwright navigate to the C4-b `url`; capture full-page screenshot, final URL, page HTML, HTTP status, console errors, and the network log.

- *Screen reached `/s/`*, *page contains "demo"* (notebook title is literally `demo notebook`), *page contains "Download PDF"* — all three expected **PASS**.
- Additional captures that the asserts do not ask for and that I consider mandatory: the network entries for `/previews/<token>.png` and a click on the `Download PDF` link. Expect **404** on both, since neither a static mount nor `GET /s/{token}/download` exists anywhere in `app.py`. Before asserting that absence I run one targeted search across `src/` for `StaticFiles`, `previews`, `mount`, and `download` so the "no such route" claim is a checked fact — a bounded locate I would hand to a cheap `haiku` explore worker (one question: which files under `src/` reference any of these four tokens, with file and line), then confirm the decisive hit or miss myself.

**Stop #2 (browser evidence is human-judged).** I do not self-approve a GUI case. I present the screenshot plus the two 404s and ask: does a share page with a broken preview image and a dead "Download PDF" link satisfy C4-d?
- Ruling "assert text only, C4-d passes" → recorded as pass-as-written, with the 404s carried forward as defects against the cycle's headline.
- Ruling "C4-d fails" → C4 does not pass; the download route and preview serving are fix-now.
- **My default: pass-as-written, cycle-not-signed-off**, with both 404s escalated as blocking findings against the card's stated behaviour.

## Phase 7 — C4-e: does the PDF read like the notebook (subjective → human decision)

This one is not mine to grade, and I will not simulate it. I prepare the human's decision instead:

- Attach `/tmp/quill-verify/demo.pdf` and a rendered page-thumbnail sheet
- State the measured page count against the claimed 12
- Point at the specific things a reader will hit: whether note sections start on fresh pages (no page-break CSS exists, and the stylesheet is likely not even loaded), whether the cover page is anything more than an unstyled `<h1>`, whether the `images` section shows the quill logo or a void, and that 8 of 12 sections carry fixture placeholder text by design
- Include the code-block section as the one positive: `fenced_code` is enabled, so `03-code.md` should render as `<code>`

**Stop #3.** The human opens the PDF and rules.
- "Looks finished" → C4-e passes, recorded with the reviewer's name and date.
- "Doesn't" → C4-e fails; my captured evidence already localizes it to the unloaded stylesheet, the missing page-break rule, the bare cover section, and the unresolvable image path.
- **My default while unresolved: C4-e is UNVERIFIED, not passed.** An assert I did not execute never appears in my report as green.

## Phase 8 — Code-shape audit of what C4 produced (advisory only)

Read-and-cite pass over the new/extended surfaces (`export.py`, `app.py` share routes, `worker.py`, `cli.py export`). Findings I expect to file, each as advisory input to the lead's verdict and never as a gate I close myself:

- **Unrequested optionality:** the `--no-cover` flag (`cli.py:21`), the `cover` parameter threaded through `export.py:15`/`:25`/`:31`, and the test that exists only to exercise it (`test_export.py:14–17`). The card asks for a cover page; nothing asks for the ability to suppress one. Roughly a dozen lines and one test that no requirement pulls.
- **Unused return values:** `export_pdf` returns `int` (`export.py:35`) and `render_preview` returns `Path` (`worker.py:29`); neither caller uses them.
- **Stub shipped behind a passing assert:** `worker.py:26`. I classify this as a defect rather than a shape finding, and it is the headline of my report.
- **Test suite shape:** the SQLite fixture in `conftest.py` means CI's Postgres service and its `make migrate` step do nothing for `pytest`; `test_share.py` asserts only that SQLAlchemy can round-trip a row, which exercises the ORM rather than C4. Advisory: the cycle has no automated coverage of its own behaviour.
- Minor: notebook and note titles are interpolated into HTML unescaped (`export.py:18`, `:21`, `app.py:46`).

I check the codebase before grading any reuse claim, and I state these as opinions with line citations — the lead decides what, if anything, is acted on.

## Phase 9 — Write up for sign-off

Artifacts I would write:

- `docs/verification/C4-report.md` — the verification report: environment fingerprint (including which database volume, fresh or warm), the CI-gate reproduction, then per case C4-a…C4-e a table of each assert with the command, exit code, verbatim observed output, and a verdict of PASS / FAIL / UNVERIFIED; every delegation disclosed with what the worker ran and which commands I re-ran myself; then the defect list (stub preview, two 404s on the share page, unresolvable image, likely-unloaded stylesheet, page-count claim vs. measurement, CI `DATABASE_URL` mismatch, `mypy` strict failures, the 0002/0003 migration hole flagged as unknown rather than broken); then the advisory shape findings; then an explicit recommendation.
- `/tmp/quill-verify/c4-evidence/` — every log, the PDF, the screenshot, the preview PNG, the Redis before/after captures, the migrate transcripts.

I would **not** tick C4's checkbox in `tasks.md` and would not touch its `**TEST:**` blocks. The verdict is the lead's; editing the card to match observed output is exactly the failure mode I exist to prevent.

**Expected recommendation, subject to what execution actually shows:** do not sign off C4. Three literal string asserts appear mis-worded against real output (fixable on either side), and — the part that matters — the share preview the card headlines is a placeholder, the share page's two links both 404, the exported PDF cannot resolve its one image, and CI's own environment appears unable to import the app. The developer's green suite is consistent with all of this, because it tests none of it.

**Follow-on I own only if asked:** if the lead rules any of these fix-now or backlog, I author the durable regression case in the same Setup/Action/Assert grammar — e.g. a case asserting the preview PNG decodes to real dimensions and that `GET /s/{token}/download` returns 200 with `application/pdf`. This workspace has no separate gates file, so I would propose the case text for the card owner to place rather than inventing a new home for it. Finding further gaps beyond the card is not my seat; an independent adversarial pass should hunt past the cases I would have written.