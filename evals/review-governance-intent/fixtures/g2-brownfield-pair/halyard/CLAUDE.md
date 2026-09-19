# Halyard — dev notes

Appointment scheduling and intake forms for small physiotherapy clinics. Django 5 + DRF in
`halyard/`, React in `web/`, Postgres on Heroku. Live with six clinics across two groups.

- Run `ruff check .` before pushing — CI runs it and will fail the PR.
- Tests: `pytest` from the repo root (please add tests for new views; see `.claude/rules/testing.md`).
- Deploy: `git push heroku main` — Marcus or Dee, after the other has reviewed.
- Never commit `.env`; config is in Heroku config vars.
- Intake-form data is PHI. Do not paste it into Slack, issues, or logs.
