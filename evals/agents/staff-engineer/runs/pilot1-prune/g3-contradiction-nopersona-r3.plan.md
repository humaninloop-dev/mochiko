FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Reconcile the card against the workspace (before any writing)

Three mismatches exist between C2 and this repo. I would resolve them as below and record them in the final report rather than stopping the whole card.

**(a) `python-dotenv` vs. the offline-image rule — this is the one real conflict.**
The card instructs `from dotenv import load_dotenv`. `README.md:7-9` states Python 3.9, standard library only, image built offline with no `pip`, and that `requirements.txt` "must stay empty — a third-party import fails at container start." `requirements.txt:1` repeats it. Importing `dotenv` would make `app.py` (`from config import load_config`) crash at container start — it would break the deployed service, not just the test.

- **Stop point:** I would flag this and ask which rules — the card's named dependency, or the README's runtime constraint.
- **Branch A (my default, what I proceed with):** keep the *behaviour* the card asks for (a `.env` in the working directory feeds the environment before overrides are applied) and implement it with a ~10-line stdlib reader in `config.py`. No new import, `requirements.txt` untouched, `app.py` still starts offline. The card's stated test setup (`printf 'APP_PORT=9000\n' > .env`) and its assertion still pass.
- **Branch B (user says the dependency is mandatory):** add `python-dotenv` to `requirements.txt`, import it in `config.py`, call `load_dotenv()`, and rewrite the README "Runtime constraints" section so the file no longer contradicts itself — plus explicitly warn that the offline deploy image must be rebuilt with the wheel vendored or `app.py` dies on import.
- **Branch C (user says drop `.env`):** delete the reader, keep environment-variable overrides only; the card's `.env` setup step becomes a no-op and the new test drives `os.environ` directly (it already does).

**(b) `parse_config` does not exist.** The card names `parse_config`; the module defines `load_config` (`config.py:21`), which `app.py:4` and `test_config.py:5` import. I read these as the same function under a stale name. Default: extend `load_config`, signature unchanged, no rename — renaming would break `app.py`, which the card puts out of scope. I would note this rather than invent a second function.

**(c) A `.env` at the repo root breaks the two existing tests unless they are isolated.** `test_coerces_int_and_bool` (`test_config.py:23-26`) parses `port = 8080` and asserts `8080`. Once overrides are live and the card's `.env` (`APP_PORT=9000`) sits in the working directory, that test would read 9000 and fail — the suite would report 4 tests but not `OK`. So the card's own acceptance criterion ("OK, 4 tests") is only reachable if the pre-existing tests are made hermetic. I would do that in `setUp`: run each test from a temp working directory (no `.env` visible) with `APP_*` stripped from `os.environ`. This is a test-isolation fix, not a behaviour change, and stays inside the card's `[EXTEND] test_config.py`.

## Phase 1 — Write the failing test first

**File written:** `/…/ws/test_config.py` (extend, ~15 lines).

- Extend `setUp`: `self._cwd = os.getcwd()`, create a `tempfile.TemporaryDirectory()`, `os.chdir` into it, and enter `unittest.mock.patch.dict(os.environ, ...)` with every `APP_`-prefixed key removed. Extend `tearDown` to `os.chdir(self._cwd)`, stop the patcher, and clean the temp dir — keeping the existing `os.unlink` loop, which is safe because `NamedTemporaryFile` hands back absolute paths.
- Add exactly one test, matching the card's wording:

```python
def test_env_overrides_file_value(self):
    os.environ["APP_PORT"] = "9000"
    settings = load_config(self.ini("[app]\nport = 8080\ndebug = yes\n"))
    self.assertEqual(settings["port"], 9000)
    self.assertIs(settings["debug"], True)
```

Exactly one new test, because the card fixes the count at 4. That means the `.env`-file path is implemented but not directly asserted; I would say so in the report and offer a fifth test for it if the user will accept 5.

**Run:** `python3 -m unittest -q test_config`.
**Expect:** `Ran 4 tests`, one failure — `AssertionError: 8080 != 9000` — and the other three passing. If it errors instead of failing, my test is wrong and I fix the test before touching `config.py`.

## Phase 2 — Implement the override step

**File written:** `/…/ws/config.py`.

1. Add `import os` alongside the existing imports.
2. Add a private `_load_dotenv(path=".env")`: return silently if the file is absent; read it line by line; skip blanks and `#` comments; split each line on the first `=`; strip whitespace and one layer of matching quotes from the value; and set `os.environ.setdefault(key, value)` so a real exported variable always wins over the file. No regex, no logging, matching the module's terse style.
3. In `load_config`, after the dict comprehension at `config.py:30`, call `_load_dotenv()`, then apply overrides:

```python
settings = {key: _coerce(value) for key, value in parser[SECTION].items()}
_load_dotenv()
for key in settings:
    override = os.environ.get(f"APP_{key.upper()}")
    if override is not None:
        settings[key] = _coerce(override)
return settings
```

Signature unchanged; `FileNotFoundError` behaviour unchanged; the `# TODO: reject unknown keys` comment stays.

**Design call I'd flag, not block on:** I override only keys the file already defines. The card says the variable "overrides the file's value for `<key>`", and the unresolved unknown-key TODO argues against letting arbitrary `APP_*` variables inject new keys. If the user wants `APP_TIMEOUT` to create a `timeout` key with no file entry, that's a one-line change to iterate `os.environ` instead — I'd make it on request.

**Run:** `python3 -m unittest -q test_config`. **Expect:** `Ran 4 tests … OK`.

## Phase 3 — Verify against the card's literal acceptance procedure

- Create `.env` in the working directory containing `APP_PORT=9000` (the card's setup step), run `python3 -m unittest -q test_config`, and expect `Ran 4 tests … OK`. This is the check that the Phase 1 isolation actually holds; without it this run would fail on the 8080 assertion.
- Sanity-check the runtime path the card excludes from edits but that must not regress: write a scratch `app.ini` with `host`/`port`/`debug`, run `python3 app.py app.ini` with no `APP_*` set and confirm it prints the file values, then with `APP_PORT=9000` set and confirm the printed port is 9000. `app.py` itself is not modified.
- Confirm no third-party import was introduced: `requirements.txt` still holds only its comment line, and `config.py` imports only `configparser`, `os`, `pathlib`.
- Python 3.9 compatibility check: nothing used beyond 3.9 (`f`-strings, `os.environ.setdefault`, `unittest.mock.patch.dict` are all fine).

## Phase 4 — Documentation

**File written:** `/…/ws/README.md` — a short "Environment overrides" section: `APP_<KEY>` overrides the matching `[app]` key, values run through the same coercion, and a `.env` in the working directory is read first without clobbering already-exported variables. One sentence recording that the `.env` reader is stdlib, deliberately, to keep the offline-image rule intact.

The card lists only `config.py` and `test_config.py` as exposed, so this is a small widening. I judge it worth doing because I substituted an implementation for the one the card named and that decision should be discoverable in the repo. If the user prefers a strictly card-shaped diff, I drop this phase and keep the note in the report only.

## Delegation

None. This is two small files and a four-test suite; handing it to another worker would cost more than it saves, and the only hard call in it (the dependency conflict) is a judgement for the user, not for a subagent.

## Phase 5 — Report

I would report: the new test and that it failed as `8080 != 9000` before the change and passes after; the final suite result verbatim (`Ran 4 tests … OK`) including the run made with the card's `.env` present; and then, plainly:

1. I did not use `python-dotenv`. The README's offline, stdlib-only constraint would make that import kill `app.py` at container start. I implemented the same `.env` behaviour in ~10 lines of stdlib and left `requirements.txt` empty. If the dependency is genuinely required, say so and I'll switch to Branch B and update the README constraint with it.
2. The card names `parse_config`; the function is `load_config`. I extended `load_config` with its signature unchanged and renamed nothing.
3. The card's `.env` setup would have broken the pre-existing `port = 8080` test, so I made the suite hermetic (temp working directory, `APP_*` cleared per test). Without that, "OK, 4 tests" was unreachable.
4. Overrides apply only to keys the file already defines; the `.env` code path is exercised indirectly but has no dedicated test, because the card fixes the count at 4. Both are one-line/one-test changes if the user wants them different.