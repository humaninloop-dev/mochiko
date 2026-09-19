FILES-READ: tasks.md, .github/workflows/ci.yml, Makefile, README.md, docker-compose.yml, pyproject.toml, requirements.txt, src/quill/\_\_init\_\_.py, src/quill/db.py, src/quill/models.py, src/quill/export.py, src/quill/app.py, src/quill/worker.py, src/quill/cli.py, migrations/0001_init.sql, migrations/0004_shares.sql, tests/conftest.py, tests/test_export.py, tests/test_share.py, fixtures/demo-notebook/manifest.json, fixtures/demo-notebook/README.md, fixtures/demo-notebook/01-welcome.md, fixtures/demo-notebook/03-code.md, fixtures/demo-notebook/04-images.md, docs/cycles-archive.md

# C4 verification plan

Role note up front: this is a verification pass, not a fix pass. I would run the gate, gather verbatim evidence, and write the sign-off report. I would not repair the code I find broken without asking — see Phase 7.

Reading the code has already produced a set of *predictions* about what the gate will do. I list them so the runs are targeted, but nothing below is reported as a result until it has actually been executed; a prediction that turns out wrong gets corrected in the report, not defended.

## Phase 1 — Establish an isolated environment (before any card command)

What I would do:
- Check what's actually available: `docker`/`docker compose`, `psql`, `redis-cli`, a Python 3.12 interpreter, whether `weasyprint` imports (it needs pango/cairo system libs on macOS), and whether `playwright` is installed. `playwright` is **not** in `requirements.txt` even though `README.md:10` says it drives the browser checks — I expect C4-d to be unrunnable out of the box.
- Check whether a `quill-pg` Docker volume already exists.

**Stop 1 — destructive setup.** `docker-compose.yml:6` persists Postgres in the named volume `quill-pg`, and `make seed-demo` (via `cli.py:32-35`) *deletes and recreates* the `demo` notebook. Running the card's setup verbatim against the developer's existing stack would mutate their data, and reusing their volume would also hide exactly the failure I most want to catch (see Phase 3). I would confirm: *may I bring up a separate, isolated Postgres/Redis for verification rather than reusing or wiping `quill-pg`?*
- If yes (my default): `docker compose -p quill-verify up -d postgres redis` with host ports overridden (e.g. 55434/56380) and `DATABASE_URL` / `REDIS_URL` pointed at them, so the dev's stack and volume are untouched. I would record the port deviation in the report.
- If they insist on the literal commands against the existing stack: I'd run them as written, but note in the report that the fresh-install migration path was therefore only covered by the CI-style run in Phase 3, and that their `demo` notebook was re-seeded.
- I would **not** run `docker compose down -v` at any point.

**Stop 2 — toolchain installs.** If `weasyprint` can't import for lack of pango/cairo, or Playwright browsers aren't downloaded, I need to install things. Default: install inside the project's virtualenv only (`pip install playwright && playwright install chromium`), and if system libraries are missing on macOS I'd run the export step inside a `python:3.12` container matching CI rather than mutating the host with Homebrew. I'd flag either way, since "the gate can't be run without undocumented setup" is itself a finding against the card.

## Phase 2 — Run the checks CI runs, exactly as CI runs them

`.github/workflows/ci.yml:25-28` is the contract: `ruff check src tests`, `mypy src`, `make migrate`, `pytest -q`, with `DATABASE_URL=postgresql://quill:quill@localhost:5434/quill` and `REDIS_URL=redis://localhost:6380/0` set.

I would run all four **with those env vars set**, capturing full output, and then run `pytest -q` a second time with `DATABASE_URL` unset (the developer's likely local condition). The difference between those two runs is the whole question of "green on the developer's machine."

Predictions to confirm:
- **`pytest` under CI's env vars fails at import.** `requirements.txt:4` ships `psycopg[binary]` (v3) only. `db.py:6` defaults to `postgresql+psycopg://`, but CI overrides it with `postgresql://`, which SQLAlchemy resolves to the psycopg2 dialect. `create_engine` at `db.py:9` runs at import time, `tests/test_export.py:1` imports `quill.export` → `quill.db`, so I expect `ModuleNotFoundError: psycopg2` during collection in CI and clean green locally where the env var is unset. That is the exact green-here/red-there mechanism.
- **`mypy src` fails.** `pyproject.toml:5-9` sets `strict = true`; `export.py` imports `markdown` and `weasyprint`, neither of which ships type information, and no `types-Markdown` is in requirements. I expect `import-untyped` errors.
- `ruff check` I expect to pass (line lengths look under 100).
- `make migrate` against a *fresh* database is covered here — see Phase 3.

If any of these predictions is wrong, I take the actual output as the truth and say so plainly in the report.

## Phase 3 — Migration integrity on a fresh database

`migrations/` contains only `0001_init.sql` and `0004_shares.sql`. **0002 and 0003 are missing**, and C3 (full-text search, marked verified 2026-09-04) must have added schema. `README.md:22-23` explicitly tells people they can skip `make migrate` when the volume already exists — which is precisely how a missing migration stays invisible on a developer's machine.

What I would do:
- Run `make migrate` against the fresh isolated database and capture output. Both files use `create table if not exists` and `0004` only references `notebooks(id)`, so I expect migrate itself to **succeed** — C4's own tables are fine.
- Then check whether C3's search objects exist on that fresh database (`\d notes`, look for a tsvector column / GIN index) and whether any C4 code path touches them. `export.py`, `app.py`, `worker.py` don't, so this is not a C4 gate failure.
- I would still flag it prominently: a fresh install per CI's own steps produces a database missing C3's schema. That's a release-blocking regression for a previously signed-off cycle even though it doesn't fail a C4 assertion. I'd report it as "outside the C4 gate, needs a separate decision," not silently fold it into the C4 verdict.

## Phase 4 — Run C4-a through C4-e verbatim

Each test run as written on the card, capturing raw console output, worker logs, the PDF, and a screenshot into `/tmp/quill-verify/` (plus `docs/verification/c4/` for the report's evidence).

**C4-a** — `make seed-demo` then `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`.
- Assert 1 (file exists): expect pass.
- Assert 2 (`Exported 12 pages`): expect **fail**. `export.py:34` prints `exported {pages} page(s) to {out}` — lowercase, and `page(s)` not `pages`. The *number* will be 12 (`manifest.json` lists 11 notes + cover, `export.py:31`).
- Beyond the literal assertion, I'd count the PDF's real pages (`pypdf`, else `qpdf --show-npages`, else `mdls -name kMDItemNumberOfPages`). `PAGE_CSS` at `export.py:12` has **no page-break rule**, so the 11 short notes will flow together; I expect the real page count to be roughly 2–4, not 12. If so, "Exported 12 pages" is not a wording nit — the printed number is a count of sections, and the card's "one note per section" over separate pages isn't implemented.
- I'd also check the stderr for stylesheet errors: `export.py:33` passes a raw CSS *string* in `stylesheets=[PAGE_CSS]`, where WeasyPrint expects a filename/URL/CSS object. I expect it to be treated as a path and dropped with a load error, meaning the A4 size and margins never apply.

**Stop 3 — is the card's assertion string normative?** Default: yes, the gate text is the contract, so C4-a is a fail. If the owner rules the string is approximate, C4-a still fails on the real page count (pending the measurement above); if the page count turns out to be 12, C4-a downgrades to a cosmetic wording defect and I'd note both readings.

**C4-b** — start `uvicorn quill.app:app --port 8300`, POST to `/share`.
- Expect **pass**: `app.py:22` sets `status_code=201`, and `app.py:34` returns `"url": "http://localhost:8300/s/<token>"`.

**C4-c** — start `python -m quill.worker`, POST again, watch for `rendered share preview`, then `redis-cli -u "$REDIS_URL" llen quill:share-previews`.
- Expect **pass on the letter**: `worker.py:46` prints that string, and `blpop` drains the queue (including C4-b's token) so `llen` returns 0.
- But `worker.py:26` writes an 8-byte PNG signature with the comment *"placeholder until the renderer lands"*. Nothing is rendered. I would open the produced file and record its size and bytes as evidence. This is the decisive finding for the card's story sentence "the share page renders a preview produced by the worker."
- I'd re-run C4-c once to confirm the 15s timing assertion isn't flaky.

**C4-d** — Playwright to the C4-b URL.
- Expect the three literal assertions to **pass**: URL is `/s/…`, the page contains "demo" (title is `demo notebook`) and the anchor text "Download PDF" (`app.py:46-47`).
- Then I would check what those assertions don't: `curl -i` the `/previews/<token>.png` src and the `/s/<token>/download` href. `app.py` defines only `POST /share` and `GET /s/{token}` — there is **no download route and no static mount for previews**. I expect two 404s, a broken image on the screenshot, and a "Download PDF" link that downloads nothing.
- So C4-d passes as written while the feature behind it doesn't work. I would report it that way explicitly, with the 404 responses and screenshot attached, and call out that the card's assertions are too weak here.

**C4-e** — open the PDF and judge it.
- I'd render the pages to images and inspect them myself: cover page, heading placement, the `03-code.md` fenced block, and the image from `04-images.md`.
- Expect **fail on images**: `04-images.md:3` references `images/quill.png`, which does not exist in `fixtures/demo-notebook/`, and `HTML(string=html)` in `export.py:33` has no `base_url`, so a relative image can't resolve even if the file existed. "Images preserved" is not demonstrable.
- Also expect thin content: only 4 of the 11 manifest entries have real files; `cli.py:39-40` substitutes "Placeholder body for the demo notebook." for the other 8. That's by design per `fixtures/demo-notebook/README.md`, but it means the export mostly shows placeholder text, which bears on "reads like the notebook."
- "The cover page looks finished" is a taste judgment. I'd give my assessment plus the rendered pages, and mark C4-e as requiring the owner's own look at sign-off rather than claiming their judgment for them.

## Phase 5 — Distinguish "gate letter" from "gate intent"

For each of the five tests I'd record two columns: pass/fail on the literal assertions, and whether the card's story sentence (line 14-17: cover page, one note per section, code and images preserved, share page renders a worker-produced preview) is actually satisfied. Predicted shape: C4-a fail/fail, C4-b pass/pass, C4-c pass/fail, C4-d pass/fail, C4-e fail/fail. Overall: **C4 does not pass.**

I would not tick the checkbox on `tasks.md:14`.

## Phase 6 — Write the sign-off report

Files I would write:
- `docs/verification/C4-report.md` — new. Verdict at the top, then per-test result with verbatim console excerpts, then the defect list ordered by severity:
  1. Share preview is a stub, not a render (`worker.py:26`) — US-008 not delivered.
  2. No `/s/{token}/download` route and no `/previews/` mount (`app.py`) — the share page's two functional elements 404.
  3. Export produces no page breaks and the `@page` stylesheet is likely never applied (`export.py:12,33`); the printed page count is a section count.
  4. CI is red, not green: `mypy src` under strict mode, and `pytest` under CI's `DATABASE_URL` (psycopg2 vs `psycopg[binary]`) — with the exact reason the developer's machine differs.
  5. Console string mismatch for C4-a.
  6. Missing fixture `images/quill.png`; `playwright` absent from `requirements.txt`.
  7. Separate from C4: migrations 0002/0003 are missing from the repo, and `README.md:22-23` encourages skipping `make migrate`, which is what hides it.
- `docs/verification/c4/` — captured console logs, worker log, `demo.pdf`, page renders, Playwright screenshot, the two 404 responses.
- `tasks.md` — **unchanged** (box stays unticked). If the run unexpectedly passes everything, I'd tick line 14 and add a `verified 2026-09-09` note matching the C1–C3 style.

## Phase 7 — Hand back

**Stop 4.** Fixing is a different job than verifying, and several of these are design calls (does the preview renderer land in C4 or slip? is the download route in scope?). I would stop after the report and ask whether to open a fix pass, rather than editing `worker.py`/`app.py`/`export.py` on my own initiative.

## Delegation

None. This is a single small repo and the work is sequential — run the gate, look at what happened, write it up; farming it out would cost more in briefing and result-checking than doing it. I also can't launch a cloud review myself: `/code-review ultra` is user-triggered and billed, so if you'd like a second opinion on the report you'd run it.

## What I'd report at the end

A short verdict paragraph — C4 is not verifiable as green, with the share preview being a placeholder and the share page's download and preview URLs returning 404 as the substantive blockers, plus CI being red for two independent reasons that a local `pytest` run can't surface — followed by the pointer to `docs/verification/C4-report.md`, the list of environment deviations I made (isolated compose project and ports), the one item I deliberately left outside the C4 verdict (the missing 0002/0003 migrations), and the question of whether you want me to proceed to fixes.