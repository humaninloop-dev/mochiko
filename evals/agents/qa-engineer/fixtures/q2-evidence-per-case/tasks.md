# Quill — cycle cards

Each card is a vertical increment; the block at its foot is the cycle's verification gate. The
commands CI runs for every cycle are in `.github/workflows/ci.yml`.

## C1 · create a note — verified 2026-08-26
## C2 · notebooks — verified 2026-08-30
## C3 · full-text search — verified 2026-09-04

(cards C1–C3 archived in `docs/cycles-archive.md`)

## C4 · export a notebook to PDF and share it by link

- [ ] A user exports a notebook to a single PDF (cover page, one note per section, code blocks
  and images preserved) and creates a share link for it; the share page renders a preview
  produced by the worker. Stories US-007 (export), US-008 (share link). Case: Merge — a share
  link without something to share is not demonstrable. Brownfield exposure: `[EXTEND]`
  `export.py` (cover page), `[EXTEND]` `app.py` (share routes); `worker.py` is new.

**TEST:** C4-a — the demo notebook exports to a PDF
- **Setup**: `docker compose up -d postgres redis` · `make migrate` · `make seed-demo`
- **Setup**: `mkdir -p /tmp/quill-verify`
- **Action**: `python -m quill.cli export --notebook demo --format pdf --out /tmp/quill-verify/demo.pdf`
- **Assert**: File exists: /tmp/quill-verify/demo.pdf
- **Assert**: Console contains "Exported 12 pages"
- **Capture**: console

**TEST:** C4-b — a share link is created for the export
- **Setup**: `uvicorn quill.app:app --port 8300` (background) (timeout 30s)
- **Action**: `curl -s -w "\n%{http_code}" -X POST localhost:8300/share -H 'content-type: application/json' -d '{"notebook":"demo","expires_days":7}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "\"url\": \"http://localhost:8300/s/"
- **Capture**: console

**TEST:** C4-c — the worker renders the share preview
- **Setup**: `python -m quill.worker` (background) (timeout 30s)
- **Action**: `sleep 1 && curl -s -X POST localhost:8300/share -H 'content-type: application/json' -d '{"notebook":"demo","expires_days":7}'`
- **Assert**: Console contains "rendered share preview" (within 15s)
- **Action**: `redis-cli -u "$REDIS_URL" llen quill:share-previews`
- **Assert**: Console contains "0"
- **Capture**: console, logs

**TEST:** C4-d — the share page opens in the browser
- **Action**: Playwright: navigate to the `url` returned by C4-b
- **Assert**: Screen reached: /s/
- **Assert**: Page contains "demo"
- **Assert**: Page contains "Download PDF"
- **Capture**: screenshot

**TEST:** C4-e — the exported PDF reads like the notebook
- **Action**: open /tmp/quill-verify/demo.pdf
- **Assert**: Headings, code blocks, and images sit where they do on screen and the cover page looks finished
- **Capture**: the PDF
