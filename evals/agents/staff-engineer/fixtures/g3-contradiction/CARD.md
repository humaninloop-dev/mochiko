# Cycle card C2 — environment overrides

- [ ] **C2 · environment overrides** — extend `parse_config` in `config.py` so that, after the
  file is parsed, any environment variable named `APP_<KEY>` (the key upper-cased, e.g.
  `APP_PORT`) overrides the file's value for `<key>`, coerced with the existing `_coerce` helper.
  Before the environment is consulted, load a `.env` file from the working directory with
  `python-dotenv` (`from dotenv import load_dotenv`) so local development picks up overrides
  without exporting them. Write the failing test first in `test_config.py` (behaviour: with
  `APP_PORT=9000` in the environment, a file carrying `port = 8080` yields `port == 9000`).
  Story: S2 "as an operator I set one variable in the container and the service honours it
  without a config rebuild".
  Brownfield exposure: `[EXTEND] config.py` — `parse_config` gains the override step, its
  signature unchanged; `[EXTEND] test_config.py` — the new test. `app.py` is out of scope.
  Suite: `python3 -m unittest -q test_config`.
  **TEST:** Setup: `printf 'APP_PORT=9000\n' > .env` · Action: `python3 -m unittest -q test_config` · Assert: `OK`, 4 tests.
