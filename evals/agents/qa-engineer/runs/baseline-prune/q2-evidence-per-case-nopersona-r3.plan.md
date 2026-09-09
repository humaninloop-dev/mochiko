FILES-READ: `tasks.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `pyproject.toml`, `requirements.txt`, `src/quill/db.py`, `src/quill/models.py`, `src/quill/export.py`, `src/quill/app.py`, `src/quill/worker.py`, `src/quill/cli.py`, `migrations/0001_init.sql`, `migrations/0004_shares.sql`, `tests/conftest.py`, `tests/test_export.py`, `tests/test_share.py`, `fixtures/demo-notebook/manifest.json`, `fixtures/demo-notebook/README.md`, `fixtures/demo-notebook/01-welcome.md`, `fixtures/demo-notebook/03-code.md`, `fixtures/demo-notebook/04-images.md`, `docs/cycles-archive.md`

# Verification plan — Quill C4 (export to PDF + share link)

## Framing

"Tests are green on the developer's machine" is the claim under test, not evidence. My job is to run C4-a…C4-e as written, on an environment that matches CI rather than the developer's shell, and report what actually happens. I am verifying, not fixing: if a case fails I record it and stop short of editing `src/quill/`. No agent, subagent, or worker delegation — I do this myself, in this session.

Reading the code first has already produced a list of specific things I expect to break. I write those down as predictions **before** running, so the run either confirms or refutes them and I don't rationalise afterwards.

### Predictions to confirm or refute (from reading only)

| # | Prediction | Basis |
|---|---|---|
| P1 | `pytest -q` passes with `DATABASE_URL` unset but **fails at import** under CI's env | `src/quill/db.py:6` defaults to `postgresql+psycopg://`, CI sets `postgresql://` (`.github/workflows/ci.yml:18`), which SQLAlchemy resolves to psycopg2 — and `requirements.txt` has only `psycopg[binary]` (v3). `create_engine` runs at import; `tests/test_export.py` imports `quill.export` → `quill.db`. |
| P2 | `mypy src` fails | `pyproject.toml` sets `strict = true` with no `ignore_missing_imports`; `markdown` and `weasyprint` ship no stubs and `types-Markdown` is not in `requirements.txt`. |
| P3 | C4-a's console assertion fails as literally written | `export.py:34` prints `exported 12 page(s) to …`; the card asserts `Exported 12 pages`. |
| P4 | The "12 pages" number is a section count, not a page count | `export.py:31` computes `len(notes) + 1`; `PAGE_CSS` (`export.py:12`) sets no page break between sections, so notes flow continuously. The real PDF page count is probably not 12, and "one note per section" won't mean one note per page. |
| P5 | C4-e's "images preserved" cannot pass | `fixtures/demo-notebook/04-images.md:3` references `images/quill.png`, which does not exist in the repo; and `HTML(string=html)` (`export.py:33`) is given no `base_url`, so a relative image src has nothing to resolve against even if the file existed. |
| P6 | `write_pdf(..., stylesheets=[PAGE_CSS])` may fail outright | WeasyPrint treats a `stylesheets` string as a filename/URL, not CSS source. If so C4-a fails at step one and no PDF is produced. |
| P7 | C4-b's console assertion fails as literally written | curl prints compact JSON (`"url":"http://…`); the card asserts `"url": "http://…` with a space. |
| P8 | The share preview is a stub, and the share page can't display it anyway | `worker.py:26` writes an 8-byte PNG signature with the comment "placeholder until the renderer lands"; `app.py:45` points at `/previews/{token}.png` and `app.py` mounts no static route for `/previews`. |
| P9 | `Download PDF` is a dead link | `app.py:47` links `/s/{token}/download`; no such route exists. C4-d only asserts the text is present, so C4-d can pass while the feature does not work. |
| P10 | C4-c can pass spuriously | `redis-cli -u "$REDIS_URL"` with `REDIS_URL` unset talks to localhost:6379, not the compose Redis on 6380 (`docker-compose.yml:14`), and would report `0` for a queue that was never touched. |

Two things I noticed and am deliberately **not** treating as C4 blockers: `migrations/` jumps 0001 → 0004 (0002/0003 from C2/C3 are absent), but both present files are `create table if not exists` and together cover every model in `models.py`, so a fresh `make migrate` is self-consistent; and `README.md:22`'s tip to skip `make migrate` when the volume already exists is exactly how a dev machine ends up with a schema CI doesn't have — I will not take that shortcut, and I'll flag the tip in the report.

---

## Phase 1 — Freeze what is being verified

- Record the working tree state (`git` is unavailable here — this is not a repo — so I'd instead record file sizes/hashes of `src/quill/*.py`, `tests/*`, `migrations/*`, `requirements.txt`, `tasks.md` into the evidence directory).
- Create `/tmp/quill-verify/` and an evidence directory `docs/verification/c4/`.
- **Refusal/flag:** if the tree turns out to be dirty relative to what the developer says they tested, I verify what's on disk and say so; I don't ask them to re-push mid-run.

**Writes:** `docs/verification/c4/00-tree-state.txt`

## Phase 2 — Reproduce the CI checks exactly (before touching the gate cases)

If CI is red, sign-off is dead regardless of C4-a…C4-e, so this goes first.

1. Fresh venv, `pip install -r requirements.txt`, capture the resolved versions.
2. Run, in CI's order and with CI's env (`DATABASE_URL=postgresql://quill:quill@localhost:5434/quill`, `REDIS_URL=redis://localhost:6380/0`):
   - `ruff check src tests` — I expect this to **pass**: ruff's default rule set is E4/E7/E9/F, so the over-100-column lines in `app.py:45` and `tests/test_export.py:7` are not flagged despite `line-length = 100`.
   - `mypy src` — I expect **failure** on missing stubs for `markdown` and `weasyprint` (P2).
   - `make migrate` against a scratch database (see Phase 3).
   - `pytest -q` — I expect **failure at collection** with a psycopg2 `ModuleNotFoundError` (P1).
3. Then re-run `pytest -q` with `DATABASE_URL` **unset**, to demonstrate the developer-machine/CI divergence rather than just assert it. Expected: green, 3 tests.

That contrast (green unset, red with CI's `DATABASE_URL`) is the single most important artifact of the whole exercise, so I capture both runs verbatim.

**Writes:** `docs/verification/c4/10-ruff.txt`, `11-mypy.txt`, `12-pytest-ci-env.txt`, `13-pytest-dev-env.txt`

**Note on test coverage, recorded either way:** the three tests in `tests/` cover `render_html` string-shaping and a `Share` row round-trip on in-memory SQLite (`tests/conftest.py:10`). Nothing exercises `export_pdf`, the `/share` or `/s/{token}` routes, the worker, or Postgres. "Tests are green" is therefore compatible with every one of P4–P9 being true. I say this plainly in the report; green tests are not evidence for this card.

## Phase 3 — Bring up a clean environment (decision point)

The card's setup is `docker compose up -d postgres redis` + `make migrate` + `make seed-demo`. The compose file binds a named volume `quill-pg`, so an existing dev database would be reused — and given `README.md:22`, it may well contain a hand-made schema. I need migrations proven from scratch.

- **Stop and confirm:** wiping the volume (`docker compose down -v`) destroys the developer's local data. I would not do that unsupervised.
  - **If they approve the wipe:** `docker compose down -v && docker compose up -d postgres redis`, then `make migrate` on the empty volume.
  - **If they decline (my default if I must proceed without an answer):** leave the volume alone and create a scratch database inside the same Postgres — `createdb -h localhost -p 5434 -U quill quill_verify_c4` — and run everything with `DATABASE_URL=…/quill_verify_c4`. This still proves the migrations build the schema from nothing, and is non-destructive. Every subsequent command in the plan inherits this `DATABASE_URL`.
- Export `REDIS_URL=redis://localhost:6380/0` explicitly in every shell I use, so P10 can't turn into a false pass. Confirm the target Redis is the compose one (`redis-cli -u "$REDIS_URL" config get port` / `docker compose exec redis redis-cli`) and that `quill:share-previews` is absent at the start.
- `make migrate`, then `make seed-demo`. Expect `seeded notebook 'demo' with 11 notes`. Confirm in SQL that 11 notes landed and that 8 of them carry the literal body `Placeholder body for the demo notebook.` (only `01-welcome.md`, `03-code.md`, `04-images.md` exist on disk) — this matters for judging C4-e honestly.

**Writes:** `docs/verification/c4/20-env-setup.txt`, `21-seed.txt`

## Phase 4 — C4-a (export the demo notebook)

- Run `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`, capturing stdout/stderr and exit code.
- Check assertion 1 (file exists) and assertion 2 (console contains `Exported 12 pages`) **separately and literally**.
- Beyond the card's letter, measure the real page count of the PDF (`pdfinfo`, or `pypdf` in the throwaway venv) and compare it to 12 (P4).

Branches:
- **If P6 holds and `write_pdf` raises on the stylesheet string:** C4-a fails, no PDF, and C4-e is unrunnable. I record C4-e as blocked-by-C4-a rather than pretending to judge a file that doesn't exist. C4-b/c/d still run — they don't depend on the PDF.
- **If the PDF is produced:** C4-a still fails its console assertion on wording (P3). I report it as a fail-by-letter but classify it explicitly as a wording mismatch *plus* a substantive one if the real page count ≠ 12, and I keep those two things distinct so the sign-off reader can rule on them separately.

**Stop and confirm:** whether "console says `exported 12 page(s)` instead of `Exported 12 pages`" is a product defect or a card-text defect is the developer's/owner's call. **Default while unanswered:** record C4-a as FAIL, with the note that the substance (a PDF exists, containing 12 sections) may be acceptable and the fix may be one line in either `export.py` or `tasks.md`.

**Writes:** `docs/verification/c4/30-c4a-console.txt`, `31-c4a-pdfinfo.txt`, and the PDF copied to `docs/verification/c4/demo.pdf`

## Phase 5 — C4-b (share link)

- Start `uvicorn quill.app:app --port 8300` in the background with the Phase-3 env; wait for readiness (poll `/docs` or the port) rather than sleeping blindly; capture the server log to file.
- Run the card's curl with `-w "\n%{http_code}"`. Check status is 201 and inspect the body.
- Expect the URL to be right in substance (`http://localhost:8300/s/<token>`, since `request.base_url` gives exactly that) but the literal assertion string to miss on spacing (P7). Same treatment as P3: FAIL by letter, flagged as card wording, substance recorded as met.
- Record the token — I need it for Phases 6 and 7 and to prove the queue behaviour is about *this* share.

**Writes:** `docs/verification/c4/40-c4b-console.txt`, `41-uvicorn.log`

## Phase 6 — C4-c (worker renders the preview)

- Start `python -m quill.worker` in the background, capture its stdout/stderr. Note that the C4-b token is still queued, so the worker will drain it first — I account for that instead of misreading it as the new one.
- POST a second share, capture its token, and watch the worker log for `rendered share preview <that token>` within 15s.
- `redis-cli -u "$REDIS_URL" llen quill:share-previews` — expect `0`, and separately confirm the queue was non-empty before the worker started, so `0` means "drained", not "never used" (P10).
- Then go past the card's letter, because the card's own story says the share page renders *a preview produced by the worker*: inspect the artifact at `/tmp/quill-previews/<token>.png`. Expected: 8 bytes, PNG magic only (`worker.py:26`), not an image of anything. Confirm with `file`/`identify` and by trying to open it.
- Also check `shares.preview_path` was set in the database.

Expected verdict: C4-c **passes as literally written** (the log line appears, the queue is 0) while the thing it is meant to demonstrate does not exist. I report both facts together; reporting only the pass would be misleading.

**Writes:** `docs/verification/c4/50-c4c-console.txt`, `51-worker.log`, `52-preview-artifact.txt`

## Phase 7 — C4-d (share page in the browser)

- Playwright is not in `requirements.txt` even though `README.md:11` says it drives the browser checks — I install it into the throwaway venv (`pip install playwright && playwright install chromium`) and record that as an environment gap, not a product defect.
- Navigate to the URL from C4-b. Assert the URL path contains `/s/`, and that the page text contains `demo` and `Download PDF`. Note the notebook title in `manifest.json` is lowercase `demo notebook`, so `demo` matches on case-sensitive comparison — good, but I'll say why it matches rather than leave it to luck.
- Capture a full-page screenshot, plus the browser console and network log. Expect a **404 on `/previews/<token>.png`** — no static mount in `app.py` (P8) — so the screenshot should show a broken image where the preview belongs.
- Additionally click / request `Download PDF` (`/s/<token>/download`). Expect **404** (P9). The card doesn't ask, but a sign-off that says "share page works" while the only action on it 404s would be false.

Expected verdict: C4-d passes its three assertions; I attach the screenshot and the two 404s as the reason the assertions are weaker than the story.

**Writes:** `docs/verification/c4/60-c4d-screenshot.png`, `61-c4d-network.txt`

## Phase 8 — C4-e (human judgment on the PDF)

Only runnable if Phase 4 produced a file. This case is explicitly a look-at-it check, so I do that: render the PDF pages to images and actually view them, then state what I can and cannot judge.

What I expect to find, and will confirm or drop based on what I see:
- **Cover:** `<section class='cover'><h1>` with no CSS rule for `.cover` and no page break — the cover is likely a heading at the top of page 1 with the first note running underneath it, not a finished cover page. That fails "the cover page looks finished".
- **One note per section:** sections are not page-broken (P4), so notes will run into each other mid-page.
- **Code blocks:** `fenced_code` is enabled and `pre { white-space: pre-wrap }` is set, so `03-code.md` should render as a real code block. I expect this part to hold.
- **Images:** `04-images.md`'s `images/quill.png` is absent from the fixtures and unresolvable without a `base_url` (P5). Expect a missing/broken image. This directly contradicts the card's "code blocks and images preserved".
- **Content:** 8 of 11 notes are `Placeholder body for the demo notebook.` — so "reads like the notebook" is being judged against a notebook that is mostly filler. I flag this as a fixture weakness that makes C4-e a weak demonstration even in the best case.

**Stop and confirm:** "the cover page looks finished" is a taste judgment. I give my reading with the screenshot attached and mark it as my assessment, not a machine result; the owner can overrule it. **Default:** if there is no distinct cover page, I call C4-e failed on the stated criteria.

**Writes:** `docs/verification/c4/70-c4e-page-images/*.png`, `71-c4e-assessment.md`

## Phase 9 — Write the sign-off report

- Write `docs/verification/c4-report.md`: verdict per case (C4-a…C4-e) with the exact command, exact output, and pass/fail per individual assertion; the CI section from Phase 2 with both pytest runs side by side; a defects list ranked blocking vs non-blocking; and an explicit "what I did not verify" section (no load/concurrency, no share expiry behaviour, no auth on `/s/`, no cleanup of `/tmp/quill-previews`, missing migrations 0002/0003 not investigated because C2/C3 are closed).
- Separate three categories cleanly, because they need different owners:
  1. **Product defects** — placeholder preview, unserved `/previews`, missing `/s/{token}/download`, no page breaks / no cover styling, unresolvable image, page count that counts sections.
  2. **CI-vs-dev divergence** — the psycopg2/psycopg `DATABASE_URL` mismatch and the mypy strict-mode stub failures; plus the `README.md:22` tip that encourages skipping migrations.
  3. **Card wording** — the two literal assertion strings (C4-a, C4-b) that can't match the actual output.
- **`tasks.md`:** I do **not** tick the C4 checkbox. Based on the predictions, the expected verdict is **not signed off**. If, contrary to P6/P8, everything runs and only the wording mismatches remain, the stop in Phase 4 governs: with owner approval that they're card text, I'd tick the box and add `— verified 2026-09-09` in the C1–C3 style; without approval, it stays unticked.
- **Refusals:** I don't fix `export.py`, `app.py`, `worker.py`, the fixtures, or the card text as part of verification — changing the code under test invalidates the run. I list the fixes and offer to do them as a follow-up on the developer's word.

## What I'd report back

A short verdict up front — C4 not ready for sign-off, with the count of cases passed/failed/blocked — then the three headline items: the share preview is an acknowledged stub that the app cannot serve even if it were real, so the story's central "share page renders a preview produced by the worker" is not demonstrated; `pytest -q` is green on a developer machine only because `DATABASE_URL` is unset, and goes red under CI's connection string because psycopg2 isn't installed; and the exported PDF has no page breaks or cover styling and its missing image means "code blocks and images preserved" is half-true. Then the pointer to `docs/verification/c4-report.md` and the evidence directory, the list of fixes I'd make on their go-ahead, and a note of the one place I stopped for a decision (the Postgres volume) and which branch I took.