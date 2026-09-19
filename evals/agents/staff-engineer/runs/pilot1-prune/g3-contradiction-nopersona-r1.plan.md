FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Reconcile the card against the workspace (before writing anything)

Two mismatches between the card and the actual code, found in the reads above:

**(a) Function name.** The card says "extend `parse_config` in `config.py`". There is no `parse_config`; the only parser is `load_config(path)` at `config.py:21`, and it is what `app.py:9` calls. I treat `parse_config` as a stale name for `load_config` and extend `load_config` in place. I do **not** rename or add an alias — the card says the signature is unchanged and `app.py` is out of scope, and a rename would break `app.py:4`. Reported at the end, not blocking.

**(b) `python-dotenv` versus the stated runtime constraint — this is the one real stop.** The card asks for `from dotenv import load_dotenv`. `README.md:7-9` says Python 3.9, standard library only, the deploy image is built offline with no network and no `pip`, and `requirements.txt` "is intentionally empty and must stay empty — a third-party import fails at container start." `requirements.txt:1` repeats that. A top-level `from dotenv import load_dotenv` in `config.py` would make `import config` raise `ModuleNotFoundError` at container start and would also error all four tests here if the package is absent; I cannot run anything to check whether it is installed, and the deploy image is stated to be offline regardless.

**What I would confirm with the user:** "The card requires `python-dotenv`; the README forbids third-party imports and requires `requirements.txt` stay empty. Which wins?"

- **Ruling "keep the constraint" (my default, and what the rest of this plan builds):** implement `.env` loading in ~10 lines of standard library inside `config.py`. Same observable behaviour — a `.env` in the working directory feeds the override step, real environment variables win over `.env` (matching `load_dotenv`'s default non-overriding behaviour). `requirements.txt` and `README.md` untouched.
- **Ruling "the card wins, take the dependency":** replace `_load_env_file()` with `load_dotenv()` imported at module top, add `python-dotenv>=1.0` to `requirements.txt`, and edit the "standard library only" bullet in `README.md:7-9` to record the exception — otherwise the repo documents a rule it now violates. Everything else in this plan (test, override loop, isolation) is unchanged.
- **Ruling "drop `.env`, environment only":** delete `_load_env_file` and its half of the new test's assertions; the `APP_PORT=9000` acceptance step then only passes if the variable is exported rather than written to a file, so I would say so.

I proceed under the default rather than blocking, since both branches share ~90% of the work.

## Phase 1 — Write the failing test first (`test_config.py`)

There is a trap here that the card's own acceptance step sets off, so the test edit is bigger than "add one method":

`CARD.md:15` says to run the suite with `APP_PORT=9000` sitting in a `.env` file in the working directory. The existing `test_coerces_int_and_bool` (`test_config.py:23-26`) loads `port = 8080` and asserts `8080`. Once `load_config` honours `.env` from the cwd, that pre-existing test starts failing under exactly the acceptance setup — the suite would report 1 failure, not `OK`. So the new test alone is not enough; the whole case needs environment isolation.

Edits to `test_config.py`:

1. Imports: add `shutil` and `from unittest import mock`.
2. Rewrite `setUp` to keep `self.paths = []` and add isolation, registering cleanups so they unwind in the right order:
   - `mock.patch.dict(os.environ)` started and stopped via `addCleanup`, then delete every existing `APP_*` name inside the patch (the patch restores them on exit);
   - `tempfile.mkdtemp()` + `os.chdir` into it, with `addCleanup(shutil.rmtree, tmpdir)` registered *before* `addCleanup(os.chdir, old_cwd)` so the chdir-back runs first.
   The `self.ini()` helper is unaffected — `NamedTemporaryFile` returns an absolute path, so `tearDown`'s `os.unlink` still works after the chdir.
3. Add exactly one new method, keeping the suite at the 4 tests the card specifies, covering both paths in the card (`APP_PORT` in the environment, and `APP_PORT` arriving via `.env`):

```python
def test_env_overrides_file_value(self):
    path = self.ini("[app]\nport = 8080\n")
    os.environ["APP_PORT"] = "9000"
    self.assertEqual(load_config(path)["port"], 9000)

    del os.environ["APP_PORT"]
    with open(".env", "w") as handle:
        handle.write("APP_PORT=9000\n")
    self.assertEqual(load_config(path)["port"], 9000)
```

Run `python3 -m unittest -q test_config`. **Expected: 4 tests, 1 failure** — `AssertionError: 8080 != 9000` on the first assertion of the new test, the other three passing. If instead I see an error rather than a failure, the isolation in `setUp` is wrong and I fix that before touching `config.py`.

## Phase 2 — Implement in `config.py`

Add `import os` and two module constants (`ENV_PREFIX = "APP_"`, `ENV_FILE = ".env"`) beside `SECTION` at `config.py:5`, then:

- `_load_env_file(path=ENV_FILE)` — read the file, return silently on `OSError` (absent `.env` is the normal case), skip blanks and `#` comments, tolerate a leading `export `, split on the first `=`, strip surrounding quotes, and write with `os.environ.setdefault` so a real exported variable beats the file.
- In `load_config`, keep lines 26-30 as they are, bind the comprehension to `settings`, then after it: call `_load_env_file()`, and for each key already in `settings` look up `os.environ.get(ENV_PREFIX + key.upper())`; when it is not `None`, replace the value with `_coerce(override)`. Return `settings`.

`configparser` lower-cases keys by default, so `key.upper()` round-trips `port` → `APP_PORT` correctly. The `FileNotFoundError` path at line 27-28 is untouched, so `test_missing_file_raises` keeps passing, and the `TODO` at line 29 stays as-is (not this card's job).

**Judgment call I make rather than ask about:** only keys present in the file are overridden. The card says the variable "overrides the file's value for `<key>`", and `APP_HOME`/`APP_DATA` style variables are common in containers — injecting them as config keys would be surprising. Noted in the report; a one-line change if the user wants additive behaviour.

Re-run `python3 -m unittest -q test_config`. **Expected: `OK`, 4 tests.**

## Phase 3 — Run the card's own acceptance check

`printf 'APP_PORT=9000\n' > .env` in the workspace root, then `python3 -m unittest -q test_config`. **Expected: `OK`, 4 tests** — passing because Phase 1's `setUp` moves each test into a clean temp cwd, so the repo-root `.env` cannot leak into `test_coerces_int_and_bool`. If it fails, the isolation is the bug, not the feature.

Before writing `.env` I check whether one already exists and, if so, show it to the user rather than overwriting a file that may hold their local settings. I leave the created `.env` in place afterwards (the card's setup step asked for it) and say so in the report.

## Phase 4 — Sanity check on the entry point, no edits

`app.py` is out of scope and I change nothing in it. I would run `python3 app.py` once against a scratch INI with and without `APP_PORT` set to confirm the story S2 behaviour end to end — the printed bind address at `app.py:10` should follow the variable. If this surfaces anything broken in `app.py`, I report it and do not fix it under this card.

## Delegation

None. This is two files and roughly thirty lines; splitting it across workers would cost more than it saves, and the only genuinely uncertain part is the dependency ruling, which is a decision for the user rather than work to farm out.

## Final report

- Card C2 done: `config.py` gains `.env` loading plus `APP_<KEY>` overrides through `_coerce`, `load_config`'s signature unchanged; `test_config.py` gains `test_env_overrides_file_value` and per-test environment/cwd isolation. Suite `OK`, 4 tests, both plain and with the card's `.env` present.
- **Flagged:** the card's `python-dotenv` requirement contradicts `README.md`'s offline standard-library-only constraint; I implemented the same behaviour with a small stdlib `.env` reader and left `requirements.txt` empty. Switching to the real dependency is a small, described change if that is the call.
- **Flagged:** the card names `parse_config`; the function is `load_config`. Extended in place, no rename.
- **Flagged:** the existing `test_coerces_int_and_bool` would have failed under the card's own acceptance setup without the isolation added in `setUp` — worth knowing if other suites in this repo read config from the cwd.
- **Noted:** overrides apply only to keys present in the file; real environment variables beat `.env`; a `.env` containing `APP_PORT=9000` now sits in the workspace root from the acceptance step.