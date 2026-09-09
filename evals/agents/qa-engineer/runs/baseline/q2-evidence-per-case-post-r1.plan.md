FILES-READ: `tasks.md`, `.github/workflows/ci.yml`, `README.md`, `docker-compose.yml`, `Makefile`, `requirements.txt`, `pyproject.toml`, `src/quill/__init__.py`, `src/quill/db.py`, `src/quill/models.py`, `src/quill/export.py`, `src/quill/app.py`, `src/quill/worker.py`, `src/quill/cli.py`, `migrations/0001_init.sql`, `migrations/0004_shares.sql`, `tests/conftest.py`, `tests/test_export.py`, `tests/test_share.py`, `fixtures/demo-notebook/manifest.json`, `fixtures/demo-notebook/README.md`, `fixtures/demo-notebook/03-code.md`, `fixtures/demo-notebook/04-images.md`, `docs/cycles-archive.md`

---

# Verification plan — Quill C4 (export to PDF + share link)

## What I am verifying against

Five cases on the C4 card (C4-a…C4-e) plus the four checks CI runs (`ruff check src tests`, `mypy src`, `make migrate`, `pytest -q`). "Green on the developer's machine" is a claim, not evidence — it gets re-established here on real Postgres and real Redis, on a clean volume, or it doesn't get claimed.

Reading the code has already produced a set of **predictions**. I state each one so that when I execute, the run either confirms or refutes it — I will report what the run actually printed, not what I predicted.

---

## Phase 1 — Pre-flight: is this host able to run the gate at all?

**Do:** Probe and record, with versions, before touching anything: `docker`/`docker compose`, `psql`, `redis-cli`, `python3.12`, `pip`, and whether `playwright` and its browsers are installed. Check ports 5434, 6380 and 8300 are free. Confirm `weasyprint` imports on this host (macOS; it needs pango/cairo/gobject system libraries that pip does not install).

**Read:** already done — `README.md`, `requirements.txt`, `docker-compose.yml`, `Makefile`, `pyproject.toml`.

**Known gaps I expect to hit, and how I handle each:**

| Gap | Predicted effect | My action |
|---|---|---|
| `requirements.txt` has no `playwright`, but `README.md` says Playwright drives the browser checks | C4-d has no runner | Do **not** silently substitute a `curl` of the share page for the browser case. Install Playwright + Chromium if that is possible non-destructively and record it as an environment adjustment; if not, C4-d is **BLOCKED**, not passed. |
| `requirements.txt` pins `psycopg[binary]` (v3), but `Makefile`, `README.md` and `ci.yml` all set `DATABASE_URL=postgresql://…` — a bare `postgresql://` URL makes SQLAlchemy load **psycopg2**, which is not installed. `src/quill/db.py`'s own default is `postgresql+psycopg://` | Every command that touches the DB with the documented env var fails with `ModuleNotFoundError: No module named 'psycopg2'` | Run it **as documented first** and capture the failure as evidence. Then re-run with `DATABASE_URL=postgresql+psycopg://…` to get past it, and record the substitution explicitly in the report as a defect finding, not as a fixed environment. |
| There is no `[build-system]` / install step and no `pip install -e .`; `pyproject.toml` only puts `src` on the path *for pytest* | `python -m quill.cli …`, `uvicorn quill.app:app` and `python -m quill.worker` — all three exactly as written on the card and in the README — fail with `No module named quill` | Capture the failure, then proceed with `PYTHONPATH=src`, disclosed in the report as an adjustment to the card's literal commands. |
| `weasyprint` native deps on darwin | C4-a cannot produce a PDF | Hard stop, reported as BLOCKED. No mock PDF, ever. |

**Clean-state decision:** `README.md` says you can skip `make migrate` when the `quill-pg` volume already exists. I will do the opposite — `docker compose down -v` first, so the run starts on an empty volume. Rationale: `migrations/` contains only `0001_init.sql` and `0004_shares.sql`; **0002 and 0003 are absent from the repo**. If the developer's machine has a volume carrying schema from C2/C3, their green run is standing on a schema that does not exist in this repository. That has to surface here, not in production.

**Stop point:** if pre-flight shows Docker unavailable or weasyprint unloadable, I stop and report BLOCKED with the exact probe output. *Branch:* if the human says "verify on the CI runner instead," the same plan runs there unchanged (CI already has the service containers); if they say "proceed with what works," I run C4-b/c/d and mark C4-a/e blocked — I do not sign off a partial gate as a pass.

**Delegate:** one cheap read-only `Explore` subagent, model `haiku`, one question: *"List every route decorator, `StaticFiles` mount, and `app.mount(...)` call anywhere under `src/`, with file and line."* I need certainty that nothing outside `app.py` serves `/previews/…` or `/s/{token}/download`. On return I check it names files and line numbers I can re-open, and I re-open the ones that matter myself.

---

## Phase 2 — The four CI checks, exactly as `ci.yml` runs them

**Do, in order, capturing full output and exit code for each:**
1. `docker compose up -d postgres redis`, wait for the Postgres healthcheck to report healthy.
2. `ruff check src tests`
3. `mypy src`
4. `make migrate`
5. `pytest -q`

**What I expect, and what I will call it:**
- `ruff`: likely clean (the config sets `line-length = 100` but ruff's default rule set does not include line-length, so `app.py:45` at ~106 chars will not trip it). Report actual.
- `mypy src` under `strict = true`: I expect errors — `markdown` and `weasyprint` ship no type information, so strict mode should raise missing-stub errors, and `redis`'s `blpop`/`rpush` returning `Any` is a strict-mode hazard in `worker.py`/`app.py`. If mypy is green, I will want to know why and will look for an ignore mechanism I have not seen. Either way: report the verbatim output.
- `make migrate`: it globs `migrations/*.sql` and will apply exactly two files. It will "succeed" — every statement is `create table if not exists` — while producing a schema missing whatever 0002/0003 contained. **Green here is not evidence the schema is complete.** I will follow it with `\dt` and `\d notes` against the DB and record the actual tables and indexes, and flag the missing migration files as a finding regardless of exit code.
- `pytest -q`: I expect green — and I will say plainly in the report that this green is close to meaningless for C4. `tests/conftest.py` builds a **SQLite in-memory** engine, so no test in the suite touches Postgres. `tests/test_export.py` tests `render_html` string assembly only — it never renders a PDF. `tests/test_share.py` inserts one `Share` row and reads it back — it never calls `POST /share`, never calls `GET /s/{token}`, and never exercises `worker.py` at all. The two behaviours C4 exists to deliver have **zero** automated coverage. The developer's "tests are green" is true and does not bear on C4.

**Delegate:** the four checks are fully specified, so I hand them to one disposable general-purpose worker at model `sonnet`, briefed: run these exact commands in this order from the repo root with these env vars, return verbatim stdout+stderr and exit code for each, do not fix anything, do not skip a step, do not judge pass/fail. On return I read each output against the check myself and personally re-run `mypy src` and `pytest -q` — the two whose outcome drives findings. Worker evidence that doesn't read cleanly gets re-captured by me, not inferred. The delegation is named in the report.

---

## Phase 3 — C4-a: does the demo notebook export to a PDF?

**Setup:** `make seed-demo` (i.e. `python -m quill.cli seed --notebook demo --from fixtures/demo-notebook`), `mkdir -p /tmp/quill-verify`.
**Action:** `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`

**Assert 1 — file exists:** expected to PASS once the path/driver adjustments from Phase 1 are in place. Capture `ls -l` and the byte size.

**Assert 2 — console contains `"Exported 12 pages"`:** I expect this to **FAIL as written**. `src/quill/export.py:34` prints `exported {pages} page(s) to {out}` — lowercase "exported", and the form is `12 page(s)`, not `12 pages`. The literal string on the card cannot appear. I will record the exact line the command printed and mark the assert failed. I will **not** quietly rewrite the assert to match the code; the mismatch itself is the finding, and which side is wrong is the lead's call.

**Evidence I will capture beyond the asserts** (because these asserts under-specify what the card promises):

- **Actual PDF page count.** `export.py:31` computes `pages = len(notebook.notes) + 1` — that is a count of *sections*, not pages, and it is printed as "page(s)". Meanwhile `PAGE_CSS` (`export.py:12`) is `@page { size: A4; margin: 18mm } pre { white-space: pre-wrap }` — **there is no page break between sections and none after the cover**. So the sections flow continuously and I predict the real PDF is roughly 2–4 pages, not 12, with the cover heading sitting on the same page as the first note. If that holds, the card's "cover page, one note per section" is not met and the "12 pages" message is inaccurate. I will count the pages in the produced file and report the number.
- **Content fidelity.** `fixtures/demo-notebook/manifest.json` lists **eleven** notes, but only `01-welcome.md`, `03-code.md` and `04-images.md` exist on disk. `cli.py:39-40` silently substitutes `"Placeholder body for the demo notebook."` for every missing file. So **eight of the twelve sections in this PDF are placeholder filler**, and the "12" in the assert is reached only by counting them. `fixtures/demo-notebook/README.md` documents this as intended, which makes it a deliberate choice — but it means C4-a's page count demonstrates the seeder, not the export. Recorded as a finding with the fixture listing as evidence.
- **Images.** `04-images.md` references `images/quill.png`; there is **no `images/` directory in `fixtures/demo-notebook/`**. The card claims "images preserved". I will capture weasyprint's missing-resource warning from the run and check the rendered PDF for the image. I expect it to be absent — which makes the "images preserved" half of the card's story **unverified by any case**, since no assert on any case checks for an image.

**Capture:** console (verbatim), `ls -l`, PDF page count, extracted text, the fixtures directory listing.

**Delegate:** the setup+action legs go to a `sonnet` worker with the exact commands and env, returning verbatim console, exit code and `ls -l`. Assert evaluation stays mine, and I re-run the export command myself once to see the print line with my own eyes.

---

## Phase 4 — C4-b: is a share link created?

**Setup:** `uvicorn quill.app:app --port 8300` in the background (with `PYTHONPATH=src`), 30s timeout; poll until it answers before acting. I keep this server up through Phases 5 and 6 — C4-c and C4-d both depend on it, and on the token minted here.
**Action:** the `curl -s -w "\n%{http_code}" -X POST localhost:8300/share …` from the card.

- **Assert — status 201:** expected PASS; `app.py:22` sets `status_code=201`.
- **Assert — console contains `"url": "http://localhost:8300/s/`:** expected PASS. `app.py:34` builds the URL from `request.base_url`, which for a request to `localhost:8300` yields exactly that prefix. One caveat I will verify rather than assume: FastAPI's JSON response has no spaces after the colon by default, so the raw body is `{"token":"…","url":"http://localhost:8300/s/…"}`. The card's assert string contains `": "` with a space. **If the raw bytes lack that space, this assert fails as literally written** — same class of defect as C4-a's. I will record the exact response body and grade against it honestly; if it fails only on whitespace I will say so plainly so the lead can rule on the wording.

**Extra evidence:** confirm the row actually landed — `select token, notebook_id, expires_at, preview_path from shares order by id desc limit 1` — and confirm the token was pushed to Redis (`app.py:33`). Save the returned `url` and token for Phases 5–6.

**Stop point:** if `uvicorn` cannot start (import path, driver), that is a blocker for b, c and d together; I stop, report, and do not proceed to fabricate a token.

---

## Phase 5 — C4-c: does the worker render the share preview?

**Setup:** `python -m quill.worker` in the background, 30s timeout; wait for its `worker listening on quill:share-previews` line before acting.
**Action:** `sleep 1 && curl -s -X POST localhost:8300/share …` (a second share).

- **Assert — console contains `"rendered share preview"` within 15s:** expected PASS. `worker.py:46` prints exactly that. But see the finding below — passing this assert does not mean a preview was rendered.
- **Action/Assert — `redis-cli -u "$REDIS_URL" llen quill:share-previews` contains `"0"`:** requires `redis-cli` on the host and `REDIS_URL` exported (the card's Setup never exports it; `db.py:7` only defaults it *inside Python*). I will export `REDIS_URL=redis://localhost:6380/0` and disclose that. I also note the assert is weak — `redis-cli` prints `(integer) 0`, and a substring test for `"0"` would also pass on `10`, `20`, `100`. I will record the exact integer, not just the substring match, and will additionally drain-check by re-running `llen` twice a second apart to rule out a timing coincidence.

**The finding that matters here.** `worker.py:24-28` does not render anything. It writes **eight bytes** — the bare PNG magic number `\x89PNG\r\n\x1a\n` — with the comment *"placeholder until the renderer lands"*, then sets `share.preview_path` and commits. The card's story says "the share page renders a preview **produced by the worker**." The queue drains, the log line prints, the assert passes, and no preview exists. I will capture `ls -l /tmp/quill-previews/<token>.png` showing 8 bytes and try to open it as an image to show it is not a valid PNG. **C4-c's asserts pass while the behaviour the card describes is not implemented.** This is the single most important item in the report and it goes to the lead as a stop, not as a note.

---

## Phase 6 — C4-d: does the share page open in the browser?

Classified as a browser case — it needs a real browser, so it runs under Playwright and I do not substitute `curl` for it. Navigate to the `url` C4-b returned.

- **Assert — screen reached `/s/`:** expected PASS.
- **Assert — page contains `"demo"`:** expected PASS — `app.py:46` emits the notebook title, which `manifest.json` sets to `"demo notebook"`.
- **Assert — page contains `"Download PDF"`:** expected PASS **as a string**, and this is the second place the case is weaker than the story. `app.py:47` emits `<a href='/s/{token}/download'>Download PDF</a>` — and **no route in the codebase serves `/s/{token}/download`** (this is what the haiku sweep in Phase 1 confirms). The link is dead. Likewise `app.py:45` emits `<img src='/previews/{token}.png'>` and **there is no static mount serving `/previews/`**, so the preview image cannot load even when `preview_path` is set.

**Extra evidence I capture beyond the asserts:** `curl -o /dev/null -w '%{http_code}' localhost:8300/s/<token>/download` and the same for `/previews/<token>.png`. I predict **404 for both**. Screenshot the page (expected: a title, a broken-image icon, and a link that goes nowhere), and capture the browser console/network log showing the failed image request.

**Stop point:** if Playwright is unavailable, C4-d is BLOCKED and reported as such; I will still capture the raw HTML and the two status codes as partial evidence, labelled partial. *Branch:* if the lead accepts an HTML-only check in place of the browser run, I record that as an explicit downgrade of the case in the report; if not, C4 cannot be signed off until a machine with a browser runs it.

---

## Phase 7 — C4-e: does the PDF read like the notebook?

This case has no mechanical assert — *"headings, code blocks, and images sit where they do on screen and the cover page looks finished"* is a human judgment. **I will not auto-approve it.** It goes to a checkpoint with the artifact and the objective measurements attached.

**What I prepare for the human:**
- The PDF itself at `/tmp/quill-verify/demo.pdf`.
- Its real page count (predicted 2–4, versus the "12 page(s)" the tool announced).
- Rendered images of the first two pages, so the cover can be judged.
- Text extraction showing where the eight `"Placeholder body for the demo notebook."` sections fall.
- Whether the code block from `03-code.md` survived as a code block, and whether any image rendered at all.

**My stated expectation going in:** this fails. With no page-break rule in `PAGE_CSS`, the "cover page" is a heading at the top of page one with the first note running underneath it; that is not a finished cover, and it is not "one note per section" in any page sense. The images are missing because the fixture image file does not exist. **Branch:** if the human judges it acceptable anyway, I record their ruling verbatim with their name against it and C4-e closes as an approved-with-known-gaps pass; if they judge it a fail, C4-e fails and C4 does not close.

---

## Phase 8 — Code-shape audit of what C4 produced

Advisory only — evidence-cited findings that ride the report to the lead's verdict. I do not gate on them. Candidates I already have in hand:

- **`export.py`'s `cover` parameter and `cli.py`'s `--no-cover` flag.** The card asks for a cover page. Nothing in the card, and nothing in C1–C3 as archived, asks for the ability to *suppress* it. Its only consumer is `tests/test_export.py:14`, a unit test written for the flag itself. Candidate code that never needed to exist — I will confirm no other caller exists before I say so.
- **`Share.created_at`** (`models.py:39`, `migrations/0004_shares.sql`) is written and never read anywhere.
- **The `tables` markdown extension** (`export.py:20`): the manifest names a "Tables" note whose file does not exist, so nothing in the demo path exercises it.
- **`datetime.utcnow()`** at `app.py:29`, `app.py:41`, `worker` path and `tests/test_share.py:10` — deprecated in 3.12 and naive; the expiry comparison at `app.py:41` compares two naive values so it works today, but it is a latent correctness issue the moment anything stores an aware timestamp. Noted, not a gate.

Each finding gets file:line evidence, and each is labelled advisory.

---

## Phase 9 — Report and sign-off checkpoint

**Write:** a verification report for the C4 card containing, per case: the exact commands run (including every environment adjustment I had to make and why), verbatim console evidence, each assert marked pass/fail/blocked against the evidence, and the extra evidence captured where the asserts under-specify the story. Plus the four CI gate results, the code-shape findings as advisory, and a named disclosure of every leg I delegated to a worker and what I re-ran myself.

**My anticipated verdict — C4 does not close.** Not because of the mechanical assert failures (those are wording), but because of three substantive gaps the card's own cases are too weak to catch:

1. **The worker does not render a preview.** It writes an 8-byte stub with a comment saying so. C4-c passes anyway because it only greps a log line. The card's story is not delivered.
2. **The share page's two interactive elements are both dead** — `/previews/{token}.png` and `/s/{token}/download` have no routes. C4-d passes anyway because it only checks link *text*.
3. **The export produces no page breaks**, so "cover page, one note per section" is not what lands in the PDF, and the "12 page(s)" message reports a section count as a page count.

Supporting concerns, reported but not by themselves blocking: `DATABASE_URL` as documented everywhere in this repo loads a driver that is not installed; migrations 0002 and 0003 are absent from the repository so `make migrate` on a clean volume cannot reproduce the C2/C3 schema; the test suite runs entirely on SQLite and covers none of C4's behaviour; eight of eleven demo fixture notes and the one fixture image do not exist.

**The checkpoint.** I present this to the human and ask for a ruling on each of the three substantive gaps: fix-now, backlog, or accept. I recommend fix-now for all three. **Branches:**
- *Fix-now* — C4 stays open, the report becomes the fix list, and I re-run the full gate from Phase 2 after the fix; no partial re-run.
- *Backlog* — I author each accepted gap as a durable case in the feature's `gates.md` in the same Setup/Action/Assert form, so the hole becomes permanent regression armour rather than a note that ages out. Concretely: a case asserting the preview file is a decodable PNG of non-trivial size, a case asserting `/s/{token}/download` returns 200 with `application/pdf` and `/previews/{token}.png` returns 200 with `image/png`, and a case asserting the exported PDF's real page count equals the number rendered on screen with the cover alone on page one.
- *Accept* — recorded with the approver named against it, and C4 closes as passed-with-known-gaps, with the gaps written into `gates.md` all the same.

I will also flag, separately from the C4 verdict, that C4-a's and C4-c's asserts are written such that they pass against code that does not do what the card says — those cases need strengthening whichever way the gaps are ruled, and that is a lesson for the next card's authoring, not a defect in this build.

**What I will refuse regardless of the ruling:** substituting a mock or an in-memory database for Postgres or Redis anywhere in this gate; reusing the developer's existing `quill-pg` volume to skip `make migrate`; swapping a `curl` for the browser in C4-d and calling it passed; auto-approving C4-e; and marking any assert passed on a worker's word without reading its raw output against the assert myself.