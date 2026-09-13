# Codebase analysis — Halyard

**Run:** 2026-09-09 · **Mode:** brownfield

## Detected stack (detect-stack baseline)

| Area | Detected | Evidence |
|------|----------|----------|
| Runtime | Python 3.12 | `runtime.txt`, `.github/workflows/ci.yml` |
| Framework | Django 5.1, Django REST Framework 3.15 | `requirements.txt`, `halyard/` |
| Data | Postgres 16 (Heroku `standard-0` add-on); Django migrations (41) | `app.json`, `halyard/*/migrations/` |
| Front end | React 18 with Vite under `web/` | `web/package.json` |
| Errors | sentry-sdk 2.13 (`halyard/settings/production.py`) | settings |
| Logging | Django default logging to stdout; Papertrail add-on drains Heroku logs; **no PHI redaction filter found** in `halyard/settings/` or `halyard/logging.py` | settings, grep |
| CI | GitHub Actions: `ruff check .` on every PR and push — **no test job** | `.github/workflows/ci.yml` |
| Tests | 14 pytest tests under `tests/`; last recorded local run 2026-09-02 (`tests/RUNS.md`) | `tests/` |
| Deploy | Heroku (`Procfile`, `app.json`): one `standard-1x` web dyno, `release:` phase runs migrations; deploys by `git push heroku main` | `Procfile`, `app.json`, `CLAUDE.md` |
| Hosting tier | `app.json` declares stack `heroku-24` and `standard-1x` dynos; no Private Space or Shield configuration present in the repo | `app.json` |
| Secret hygiene | `.env` gitignored; no secret-scanning hook or CI step | `.gitignore` |
| Dependencies | `requirements.txt` pinned exactly; no lockfile tool; no vulnerability scanning | `requirements.txt` |

## Data classes detected

- Patient identity and contact data — `patients` table (name, date of birth, phone, email).
- **Protected health information** — `intake_forms` (presenting condition, medications, prior injuries, free-text notes); `appointments.notes` (clinician free text).
- Clinic staff accounts — `users` (Django auth).

## Integrations detected

- Twilio (SMS appointment reminders) — `halyard/reminders/twilio_client.py`.
- Postmark (email) — `halyard/notifications/postmark.py`.
- Sentry — production only; `send_default_pii` is not set (defaults to false).

## Observations for the session

- Every intake-form field is stored in clear in Postgres; no field-level encryption.
- `halyard/api/views.py` — 31 DRF views; 29 carry `permission_classes = [IsAuthenticated]`; `HealthCheckView` and `PublicBookingWidgetView` are `AllowAny` by design.
- The booking flow has an end-to-end test (`tests/test_booking_flow.py`); nothing exercises the intake form's PHI handling beyond validation.
- Two committers in the last 180 days (Marcus, Dee); every merged PR on `main` has one approving review from the other.
