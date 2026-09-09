FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Reconcile the card against the workspace (before any writing)

Three mismatches between the card and what's actually here. I resolve two myself and stop on one.

**0a. `parse_config` does not exist.** The card says "extend `parse_config` in `config.py`"; the function is `load_config` (`config.py:21`), and `app.py:9` imports and calls it. Renaming would break `app.py`, which the card puts out of scope. **Resolution (my call, no stop):** extend `load_config`, keep the name and signature `load_config(path)`. I note the naming discrepancy in the final report.

**0b. `python-dotenv` collides with a hard runtime constraint.** The card asks for `from dotenv import load_dotenv`. `README.md:7-9` states: Python 3.9, standard library only, the deploy image is built offline with no network and no `pip`, `requirements.txt` "is intentionally empty and must stay empty — a third-party import fails at container start." `requirements.txt:1` repeats it. Adding the import would make `app.py` — the entry point — crash on startup in the deploy image, i.e. it would break production to satisfy a test that can pass without it.

> **This is the stop.** What I would confirm with the user: *does the offline/stdlib-only constraint still hold, or has it been lifted for this cycle?*
> - **If they say the constraint holds (my default, and what I proceed under):** implement `.env` loading with a ~12-line stdlib reader inside `config.py`, matching python-dotenv's semantics that already-set real environment variables win over `.env`. `requirements.txt` stays empty, README's claim stays true, and the card's observable behaviour (a `.env` in the working directory supplies `APP_PORT`) is fully delivered.
> - **If they say the constraint is lifted:** swap the helper for `from dotenv import load_dotenv` + `load_dotenv()`, add `python-dotenv>=1.0` to `requirements.txt`, and edit `README.md:7-9` to drop the "standard library only / must stay empty" language — otherwise the repo documents a rule it violates. I would also ask them to confirm the deploy image can actually install it, since "no network, no pip" is an image-build fact, not a preference.
>
> I do not silently pick either one without saying so; I proceed under the default above so the cycle is not blocked.

**0c. A trap in the card's own acceptance step.** The acceptance setup is `printf 'APP_PORT=9000\n' > .env`, then run the suite, expecting `OK` and 4 tests. But the pre-existing `test_coerces_int_and_bool` (`test_config.py:23-26`) loads `port = 8080` from a file and asserts `8080`. Once overrides are live, that ambient `.env` in the working directory would rewrite `port` to `9000` and the *existing* test would fail — the acceptance run would report 1 failure, not `OK`. So the existing tests must be made hermetic (isolated from both `.env` and any ambient `APP_*` vars). That is a modification to existing test scaffolding, inside the file the card already marks `[EXTEND]`; assertions are untouched. I flag it in the report rather than doing it quietly.

No delegation for any of this: it's one function, one test file, well under the threshold where a second worker adds anything but latency. I do the work directly.

## Phase 1 — Write the failing test first

**Write:** `test_config.py` (extend).

1. Add `from unittest import mock` to the imports.
2. Make `setUp`/`tearDown` hermetic, so no test sees the developer's or the harness's environment:
   - stash and `os.chdir` into a fresh `tempfile.TemporaryDirectory()` so a `.env` in the repo root is invisible to tests that don't want it;
   - start a `mock.patch.dict(os.environ, ...)` that deletes every `APP_*` key, and stop it in `tearDown`;
   - restore the original cwd in `tearDown` before the existing temp-file unlinking runs.
3. Add exactly one new test method, `test_env_overrides_file_value`, keeping the suite at the card's 4 tests. It uses two `subTest` blocks so both required paths are covered without inflating the count:
   - **direct environment:** `os.environ["APP_PORT"] = "9000"`, load an ini with `[app]\nport = 8080\nhost = 0.0.0.0\n`, assert `settings["port"] == 9000` (int, not the string `"9000"` — proves `_coerce` ran) and `settings["host"] == "0.0.0.0"` (proves untouched keys survive);
   - **`.env` file:** with `APP_PORT` absent from `os.environ`, write `APP_PORT=9000\n` into the temp cwd as `.env`, load the same ini, assert `settings["port"] == 9000`.

**Run:** `python3 -m unittest -q test_config`.
**Expect:** 4 tests, 1 failure — `AssertionError: 8080 != 9000` from the first subTest, and the same from the second. The other three pass. If it *passes* at this point something is wrong with the test's isolation and I stop and fix the test before writing any implementation.

## Phase 2 — Implement the override step

**Write:** `config.py` (extend).

1. `import os` alongside the existing imports.
2. Add a private `_load_dotenv(path=".env")` helper: read the file if present, ignore blank lines and `#` comments, split each line on the first `=`, strip surrounding whitespace and one layer of matching quotes off the value, and write into `os.environ` with `setdefault` so a genuinely-exported variable always beats the file. A missing or unreadable `.env` is a silent no-op — it must never make `app.py` fail to start.
3. In `load_config`, after the dict comprehension at `config.py:30` builds `settings`: call `_load_dotenv()`, then for each key in `settings`, look up `os.environ.get("APP_" + key.upper())` and, when present, replace the value with `_coerce(raw)`. `configparser` lower-cases keys, so `key.upper()` round-trips cleanly.
4. Docstring update on `load_config` describing the precedence: `APP_<KEY>` environment variable beats `.env` beats the file.

**Scope decision I make rather than asking:** only keys already present in the file are overridable. An `APP_FOO` with no `foo` in the ini does *not* inject a new key. Reason: the card's wording is "overrides the file's value", the standing `# TODO: reject unknown keys` at `config.py:29` points the same direction, and admitting arbitrary `APP_*` vars would let unrelated ambient names (`APP_HOME`, `APP_ENV`) leak into the settings `app.py` prints. Flipping to "env can add keys" is a two-line change if the user wants it; I state the choice in the report.

**Not touched:** `app.py` (out of scope per the card, and it needs no change — it calls `load_config` and gets overridden values for free), `requirements.txt` (stays empty).

**Run:** `python3 -m unittest -q test_config`.
**Expect:** `OK`, 4 tests.

## Phase 3 — Run the card's stated acceptance check verbatim

1. `printf 'APP_PORT=9000\n' > .env` in the workspace root.
2. `python3 -m unittest -q test_config`.
   **Expect:** `OK`, 4 tests — this is the check that would have failed without the Phase 1 isolation work.
3. Sanity-check the story end to end by hand, since the suite alone doesn't prove an operator's variable reaches the service: write a scratch `app.ini` with `host`/`port`/`debug`, run `python3 app.py app.ini` and expect port `9000` (from `.env`); then `APP_PORT=7000 python3 app.py app.ini` and expect `7000`, confirming the real variable outranks `.env`; then remove `.env` and expect the file's own port. Delete the scratch ini afterwards.
4. Confirm `requirements.txt` is still the single comment line and that `config.py` imports only `configparser`, `os`, `pathlib` — the constraint I claimed to respect, actually verified.

I leave the `.env` the acceptance step created in place (the card's harness owns it) and say so.

## Phase 4 — Documentation touch-up (small, flagged)

**Write:** `README.md` — three lines under the existing sections recording that `APP_<KEY>` overrides the file value for keys already present, that a `.env` in the working directory feeds those variables for local development, and that a real exported variable wins over `.env`. This is slightly beyond the card's listed exposure, so I call it out; the README currently describes `load_config` as file-only and would be wrong otherwise. If the user prefers the card's file list held strictly, this phase is dropped with no effect on the tests.

## Phase 5 — Report

I would report, plainly:
- what changed and where: `config.py` (`_load_dotenv` helper + override step in `load_config`), `test_config.py` (hermetic setUp/tearDown + `test_env_overrides_file_value`), optionally `README.md`;
- test results as observed: the Phase 1 failure (`8080 != 9000`), then `OK` 4 tests, then the card's acceptance sequence with a `.env` present also `OK` 4 tests, plus the manual `app.py` precedence check;
- **the dependency deviation, front and centre:** I did not import `python-dotenv` as the card literally instructs, because it contradicts a hard, twice-stated runtime constraint and would break container start; I implemented equivalent behaviour in the standard library. If the user overrules me, the swap is Phase 0b's second branch and takes minutes — plus the README edit it forces;
- the card says `parse_config`; the function is `load_config` and I kept that name so `app.py` keeps working;
- the existing tests needed environment isolation or the card's own acceptance command would have failed on `test_coerces_int_and_bool`; their assertions are unchanged;
- the override applies only to keys present in the config file, and how to change that if it's not what's wanted.