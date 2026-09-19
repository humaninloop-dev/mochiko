FILES-READ: `tasks.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `pyproject.toml`, `requirements.txt`, `src/quill/__init__.py`, `src/quill/db.py`, `src/quill/models.py`, `src/quill/export.py`, `src/quill/app.py`, `src/quill/worker.py`, `src/quill/cli.py`, `migrations/0001_init.sql`, `migrations/0004_shares.sql`, `tests/conftest.py`, `tests/test_export.py`, `tests/test_share.py`, `fixtures/demo-notebook/manifest.json`, `fixtures/demo-notebook/README.md`, `fixtures/demo-notebook/01-welcome.md`, `fixtures/demo-notebook/03-code.md`, `fixtures/demo-notebook/04-images.md`, `docs/cycles-archive.md`

---

# C4 verification plan

My role here is to verify, not to repair. I will not edit `src/quill/`, the fixtures, or the assert strings in `tasks.md` to make the gate pass — if a check fails I record it and hand the failure back. The only file I'd write is the sign-off report, plus the C4 checkbox in `tasks.md` if and only if everything passes.

A note before the phases: reading the code has already surfaced several things that predict specific gate outcomes. I state them as expectations below so the run either confirms or refutes them, rather than discovering them ad hoc.

## Phase 1 — Reproduce "green on the developer's machine", and check CI parity

What I'd run, from the workspace root, exactly what `ci.yml` runs (lines 25–28):

1. `pip install -r requirements.txt` into a clean venv.
2. `ruff check src tests`
3. `mypy src`
4. `pytest -q` — first with no `DATABASE_URL` set (the developer's likely shell), then again with `DATABASE_URL=postgresql://quill:quill@localhost:5434/quill` and `REDIS_URL=redis://localhost:6380/0` exported, which is what CI does (lines 18–19).

Why the second pytest run matters: `src/quill/db.py:6` defaults to `postgresql+psycopg://…`, but CI exports `postgresql://…`. `create_engine` is called at import time (`db.py:9`) and resolves the DBAPI eagerly; bare `postgresql://` selects the psycopg2 dialect, and `requirements.txt` only ships `psycopg[binary]` (v3). My expectation is that `pytest -q` passes with the variable unset and errors at collection with it set — i.e. green locally, red in CI, for a reason unrelated to C4's features. If that's what happens, it is a finding in its own right (the checks the card points at cannot pass as configured).

Expected from the tests themselves: two export tests and one share test, all passing. I'd also record what they *don't* cover — nothing imports `quill.app` or `quill.worker`, and `export_pdf` is never called — so "tests are green" carries almost no weight for the share routes, the worker, or actual PDF generation. That goes in the report as a coverage statement, not as a failure.

## Phase 2 — Bring up the environment for the manual gate

1. `docker compose up -d postgres redis`; wait for the postgres healthcheck.
2. `make migrate`. I run this even though `README.md:22-23` says it can be skipped when the `quill-pg` volume already exists — `migrations/0004_shares.sql` is new in this cycle, so a volume carried over from C3 has no `shares` table and every share test would fail confusingly. I'd also note in the report that the directory jumps `0001` → `0004` with no `0002`/`0003`, so a fresh database cannot be rebuilt to the C3 state from this repo; for C4's tables that's harmless, so I flag it and continue.
3. `make seed-demo`, expecting `seeded notebook 'demo' with 11 notes`.
4. `mkdir -p /tmp/quill-verify`.
5. Export `DATABASE_URL=postgresql+psycopg://quill:quill@localhost:5434/quill` and `REDIS_URL=redis://localhost:6380/0` for all subsequent steps. The `REDIS_URL` export is not optional: C4-c step 4 runs `redis-cli -u "$REDIS_URL"`, and with the variable unset that command talks to a default localhost:6379 that isn't this stack — the `llen` would return `0` from the wrong server and the assert would pass for the wrong reason. I'd say so in the report regardless of the outcome.

**Stop point (destructive):** if the existing `quill-pg` volume turns out to hold state that makes seeding ambiguous, the clean fix is `docker compose down -v`, which erases the developer's local database. I would not do that unsupervised — I'd stop and ask. Branch: if they approve, recreate and re-migrate and note it in the report; if they decline, proceed on the existing volume and record that the run was not from a clean database. Default while waiting: proceed on the existing volume, since `make migrate` is idempotent (`create table if not exists`) and `seed` deletes and re-inserts the `demo` notebook (`cli.py:32-35`).

## Phase 3 — Run the five gate cases as written, capturing evidence

I run each case literally, capture raw console into `/tmp/quill-verify/`, and judge the assert against the raw output — not against a charitable reading of it.

**C4-a** — `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`.
- Expect the file to exist (passes).
- Expect the console assert `Exported 12 pages` to **fail**: `export.py:34` prints `exported 12 page(s) to /tmp/quill-verify/demo.pdf` — lowercase, and `page(s)` not `pages`. The number 12 is right (11 manifest notes + cover).
- Deeper issue I'd record: `pages` at `export.py:31` is a *section* count, not a PDF page count, and `PAGE_CSS` (`export.py:12`) sets only page size and margins — there is no page break between sections. So the twelve sections will flow together and the actual PDF will have far fewer than twelve pages. The message asserts something the artifact won't show. This matters for the card's "cover page, one note per section" wording and feeds C4-e.

**C4-b** — start `uvicorn quill.app:app --port 8300` in the background, then the `curl -s -w "\n%{http_code}" -X POST …`.
- Expect status **201** (passes; `app.py:22`).
- Expect the console assert `"url": "http://localhost:8300/s/` to **fail** on the raw response: FastAPI serialises compactly, so the body is `{"token":"…","url":"http://localhost:8300/s/…"}` with no space after the colon. The URL itself is correct. I'd re-run the same call piped through `python -m json.tool` and capture that too, so the report can show that the value is right and only the assert's literal spelling is wrong.
- I save the returned `url` for C4-d.

**C4-c** — start `python -m quill.worker` in the background, then `sleep 1 && curl -s -X POST …/share …`.
- Expect `rendered share preview <token> -> /tmp/quill-previews/<token>.png (0.0Xs)` within 15s (passes on the letter).
- Expect `redis-cli -u "$REDIS_URL" llen quill:share-previews` to print `0` (passes) — but I'd record that this assert is nearly vacuous: the worker uses `blpop`, so the queue is drained by construction, and the same `0` appears if the worker never started or if `REDIS_URL` points elsewhere. As a supplementary check I'd read `llen` immediately *before* starting the worker (expect ≥1) so the drain is actually demonstrated.
- The substantive check, which the card does not make: `worker.py:26` writes `b"\x89PNG\r\n\x1a\n"` — an 8-byte PNG signature with the comment "placeholder until the renderer lands". I'd `ls -l` and `file` the artifact. Expect an 8-byte non-image. **No preview is being rendered.** The message that satisfies the assert is printed by a stub.

**C4-d** — Playwright to the `url` from C4-b.
- Expect the URL to be under `/s/` (passes), the page to contain the notebook title, and `Download PDF` (passes; `app.py:46-47`).
- Case-sensitivity watch: the assert is `Page contains "demo"` and the manifest title is `"demo notebook"` (lowercase), so this matches. I'd record the exact rendered text.
- Two things the asserts don't cover that I'd capture in the screenshot and report: (a) `app.py:45` emits `<img src='/previews/{token}.png'>`, but `app.py` mounts no static route for `/previews` — the image is a broken 404 on the page even after the worker has "rendered" it; (b) the `Download PDF` link points at `/s/{token}/download`, and **no such route exists** in `app.py` — clicking it 404s. I'd click it in Playwright and capture the status to make that concrete rather than asserted from reading.

**C4-e** — open `/tmp/quill-verify/demo.pdf` and inspect it against the card's wording.
- Expect **fail** on images: `fixtures/demo-notebook/04-images.md` references `images/quill.png`, that file does not exist in the fixture tree, and `export.py:33` calls `HTML(string=html)` with no `base_url`, so a relative image URL cannot resolve even if the file were added. WeasyPrint will warn and drop it. "Images preserved" is not demonstrable.
- Expect **fail** on layout: no page break between sections, so the cover `<h1>` will not stand alone as a cover page and notes will not start on their own pages.
- Expect code blocks to render (the `fenced_code` extension is on and `03-code.md` is real content).
- Evidence limitation I'd state plainly: only 3 of the 11 notes have real fixture bodies; `02`, `05`–`11` are missing and `cli.py:39-40` substitutes "Placeholder body for the demo notebook." The fixture's own README says this is by design, so I treat it as intended rather than a defect — but it means the "Tables", "Math", "Long note" and other sections in the PDF are one line of filler, and C4-e cannot demonstrate that the export "reads like the notebook" beyond the welcome and code notes.

## Phase 4 — Adjudication and the judgment calls I'd surface

I sort findings into three buckets: (i) the card's assert text is wrong but the behaviour is right, (ii) the behaviour is wrong, (iii) the assert passes but proves nothing.

Bucket (i) as expected: C4-a's `Exported 12 pages`, C4-b's `"url": "…`. My default ruling is that a gate is a literal check — I mark both **failed as written**, and separately state that the underlying behaviour is correct and the fix is one line of card text each (`exported 12 page(s)`; and either drop the space or pipe curl through `json.tool`). **This is the ruling I'd flag for confirmation**: if the sign-off convention is that asserts are read for intent rather than literally, these two flip to pass and C4-a still fails on the page-count point below; if the convention is literal, they stay failed until the card is corrected and re-run. Either way it does not change the overall verdict, because bucket (ii) is non-empty.

Bucket (ii), the blockers: the worker renders no preview (stub PNG), the share page's preview image has no route to be served from, and the `Download PDF` link has no handler. Together these mean the card's own headline — "the share page renders a preview produced by the worker", plus US-008's share-and-download story — is not met, even though C4-c and C4-d pass as written. Also in this bucket: images absent from the PDF and no per-note page breaks, against C4-e. **Second stop point:** whether the placeholder preview is a known deferral to a later cycle. If the developer says it is deliberate and C4 is scoped to the queue plumbing only, then C4-c/C4-d pass on the narrowed scope and I'd ask for the card text to be narrowed to match before signing. Absent that ruling my default is that it blocks sign-off, because the card as written promises a rendered preview.

Bucket (iii): C4-c's `llen == 0`, and the fact that the green test suite touches neither `app.py` nor `worker.py` nor `export_pdf`.

I would **not** sign C4 off as verified under my default rulings, and I would not tick the C4 checkbox in `tasks.md`.

## Phase 5 — Delegation

None. This is a single-threaded verification run on one machine with one stack; splitting it across workers would multiply environment setup and make the captured evidence harder to attribute. I'd do every step myself and keep all artifacts in one place.

## Phase 6 — Write-up

I'd write `docs/verification/C4-report.md` containing:

- Verdict up front: **not verified**, with the blocking items listed in one short list.
- Environment: image versions, the exact `DATABASE_URL`/`REDIS_URL` used, whether the database was fresh or carried over, and the commit state.
- The CI-parity result from Phase 1, including the `postgresql://` vs `psycopg` driver finding if it reproduces, with the exact traceback.
- One section per case C4-a…C4-e: command run, raw captured output, assert-by-assert pass/fail, and for each failure whether it's the card's wording or the code's behaviour.
- Evidence index: `/tmp/quill-verify/demo.pdf`, console captures, worker log, the Playwright screenshot, `ls -l` of the preview PNG.
- A "passes but doesn't prove it" section: the `llen` assert, the stubbed preview behind C4-c's message, the test suite's blind spots.
- Repo-hygiene notes that aren't C4 failures: missing migrations `0002`/`0003`, and the README tip that would skip the migration introducing `shares`.
- Two open questions for the developer: the literal-vs-intent ruling on the assert strings, and whether the placeholder preview renderer is a deliberate deferral.

Then I'd report back in chat: the verdict, the three or four things that must change before a re-run, and the note that the two assert-text mismatches are card bugs rather than code bugs so they don't need a code change.