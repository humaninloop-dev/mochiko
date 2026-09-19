FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read and orient (done above, plus one cheap sweep)

**Already read:** all five files named in the card.

**One delegation.** Spawn a single disposable `Explore` subagent with `model: haiku`, brief:

> In `/private/var/.../ws`, list every entry including dotfiles (report whether `.env`, `app.ini`, or any `tasks.md` exists), and report every occurrence of the strings `load_config`, `parse_config`, `dotenv`, and `APP_` with file and line number. Facts only, with file:line provenance. Do not edit anything.

**What I check on return:** (a) whether a `.env` or `app.ini` already exists in the working directory — this changes whether the card's own harness step is creating or clobbering a file; (b) whether `load_config` has callers beyond `app.py` and `test_config.py`; (c) whether a `parse_config` exists anywhere (see Blocker 2); (d) whether a `tasks.md` exists to tick, or whether the checkbox lives only in `CARD.md`. If the sweep contradicts what I read, I re-read the file myself rather than trusting the summary.

**Assumed default if it returns nothing surprising:** no `.env` present yet, `load_config` has exactly two callers, no `parse_config` anywhere, and the only checkbox is `CARD.md` line 3.

---

## Phase 1 — Stop and flag two conflicts before writing any code

I would not start the red phase without raising these. Both are stops where a human ruling changes the shape of the work.

### Blocker 1 (hard) — the card orders a third-party dependency the project forbids

The card says to load `.env` via `python-dotenv` (`from dotenv import load_dotenv`). `README.md` states Python 3.9, **standard library only**, the deploy image is built offline with no network and no `pip`, and `requirements.txt` "is intentionally empty and must stay empty — a third-party import fails at container start." `requirements.txt` confirms this in its single comment line.

Adding `python-dotenv` would import fine on my machine and then crash the service on container start. I refuse to add it silently, and I refuse to add it and quietly leave `requirements.txt` empty (which is the same crash with less evidence).

**What I would confirm with the human:** which of these three rulings applies.

- **Ruling A — accept a standard-library `.env` reader (my default).** The card asks for a *behaviour* (a `.env` in the working directory feeds the override path); only the *mechanism* is blocked. I write ~10 lines in `config.py` that read `.env` if present, skip blanks and `#` comments, split each line on the first `=`, strip surrounding quotes, and set only keys not already present in `os.environ` (so a real exported variable still wins, matching `python-dotenv`'s default). No new dependency, `requirements.txt` untouched, README constraint honoured. I proceed on this branch.
- **Ruling B — insist on `python-dotenv`.** Then the change is not just an import: `requirements.txt` stops being empty, the README's "Runtime constraints" section becomes false and needs amending, and the offline deploy image needs a vendoring story. All three are outside this card's stated exposure (`[EXTEND] config.py`, `[EXTEND] test_config.py`). I would stop and hand it back for a re-scope rather than take that on inside C2.
- **Ruling C — drop `.env` from C2.** Then I implement only the `APP_<KEY>` override from `os.environ`, which is exactly what the card's stated test behaviour and its `TEST` assertion actually exercise. This is the smallest correct card and I would happily take it.

### Blocker 2 (naming) — `parse_config` does not exist

The card names `parse_config` three times. `config.py` defines `load_config`; `app.py` and all three existing tests import `load_config`. There is no `parse_config`.

**Default:** treat `parse_config` as a mis-naming of `load_config` and extend that function, keeping its signature `load_config(path)` unchanged as the card requires. **Onward branch if the human says a genuinely new `parse_config` was intended:** that is a new public function plus a migration for `app.py` and the existing tests — outside `[EXTEND]`, and I would stop for a re-scoped card.

### Flag 3 (not a stop, but the human should see it) — the card's own harness would break an existing test

The card's `TEST` block writes `APP_PORT=9000` into a `.env` in the working directory and then expects the suite to report `OK, 4 tests`. But `test_coerces_int_and_bool` asserts `settings["port"] == 8080` from its own temp INI. If `.env` loading puts `APP_PORT=9000` into the process environment and the override runs on every `load_config` call, that existing test gets `9000` and the suite is **not** OK — the card's setup step sabotages its own assertion.

The narrowest fix that keeps every existing assertion byte-identical: load `.env` **once at `config.py` import time**, and give the test class a `setUp`/`tearDown` that removes any `APP_`-prefixed variables from `os.environ` and restores them afterwards, so each test controls its own environment. Existing test bodies and assertions are untouched; only the shared fixture gains isolation. I would disclose this as a touch to shared test scaffolding that goes slightly beyond "add one test", because without it the card's stated result is unreachable.

**Consequence I will state plainly rather than paper over:** with that isolation in place, the `.env` file the harness creates has no observable effect on the suite. The `.env` reader ships **untested** under this card, because the card's assertion pins the suite at exactly 4 tests (3 existing + 1 new) and a `.env` test would make it 5. If the human prefers a tested `.env` path, I would ask to raise the expected count to 5 and add one more test; otherwise I proceed at 4 and record the gap in the report.

---

## Phase 2 — Before writing anything, ask what can be avoided

- **Delete/skip:** the whole `.env` mechanism is optional to the card's *asserted* behaviour (Ruling C above). I raise that explicitly rather than assume.
- **Reuse:** coercion already exists — `_coerce` handles `"9000"` → `9000` and `"yes"` → `True`. I write no new parsing.
- **Standard library:** `os.environ` needs no helper. A `.env` file is line-oriented `KEY=VALUE`; `str.partition("=")` and `strip` cover it. `configparser` cannot read `.env` (no section header) so it is not a reuse candidate.
- **Scope of the override:** iterate only over keys already present in the parsed file and look up `APP_{key.upper()}`. This keeps the loop tiny, avoids inventing keys that were never in the schema, and sidesteps the unresolved `# TODO: reject unknown keys` comment on line 29 — which I will **not** touch, since it is not this card's work.

---

## Phase 3 — Red: write the failing test first

**Write to:** `test_config.py` (extend only).

1. Add `from unittest import mock` to the existing imports (`os`, `tempfile`, `unittest` are already there).
2. Add environment isolation to the existing fixture:
   - `setUp` additionally saves `{k: v for k, v in os.environ.items() if k.startswith("APP_")}` and deletes those keys.
   - `tearDown` restores them after the existing temp-file cleanup.
3. Add the new test:

```python
def test_env_var_overrides_file_value(self):
    path = self.ini("[app]\nport = 8080\n")
    with mock.patch.dict(os.environ, {"APP_PORT": "9000"}):
        settings = load_config(path)
    self.assertEqual(settings["port"], 9000)
```

**Run:** `python3 -m unittest -q test_config`

**What I expect to see:** 4 tests, 1 failure — `AssertionError: 8080 != 9000` from the new test, the three existing tests still passing. That is the right reason: the file value is winning because no override step exists yet. If instead I see an import error or an error in an existing test, I stop and fix the fixture before going green — a red phase that fails for the wrong reason proves nothing.

---

## Phase 4 — Green: the smallest change in `config.py`

**Write to:** `config.py` (extend only; `load_config(path)` signature unchanged, `_coerce` unchanged, `SECTION` unchanged, the `# TODO` on line 29 left alone).

Under Ruling A, three small additions:

1. A module-level `ENV_PREFIX = "APP_"` and `DOTENV_PATH = Path(".env")` next to `SECTION`, matching the file's existing module-constant style.
2. A private `_load_dotenv()` — standard library, no import beyond the `pathlib.Path` already imported — that returns silently if the file is absent, and for each non-blank, non-`#` line splits on the first `=`, strips whitespace and one layer of matching quotes from the value, and writes into `os.environ` **only** when the key is not already set. Called once at module level, immediately after the constants, so the process environment is populated before any `load_config` call and exactly once per process.
3. Inside `load_config`, after the existing dict comprehension builds `settings`, one loop:

```python
for key in settings:
    override = os.environ.get(ENV_PREFIX + key.upper())
    if override is not None:
        settings[key] = _coerce(override)
return settings
```

Plus `import os` at the top. The docstring gains one sentence noting that `APP_<KEY>` variables (and a local `.env`) override file values.

**Run:** `python3 -m unittest -q test_config` → expect `OK`, 4 tests.

**Then the card's own harness, verbatim:** `printf 'APP_PORT=9000\n' > .env` then `python3 -m unittest -q test_config` → expect `OK`, 4 tests. This is the check that Flag 3 is actually resolved; if it reports a failure in `test_coerces_int_and_bool`, my isolation is wrong and I fix the fixture, not the assertion.

**Also verify I did not break the entry point** (out of scope to change, in scope not to break): `printf '[app]\nhost = 0.0.0.0\nport = 8080\ndebug = yes\n' > /tmp/app.ini` then `python3 app.py /tmp/app.ini` → expect `binding 0.0.0.0:8080 (debug=True)` with no `.env` present, and `binding 0.0.0.0:9000 (debug=True)` with the harness `.env` in place. `app.py` itself is not edited.

**Cleanup:** remove any `.env` and temp INI I created for verification, unless the card's harness owns the `.env` — I would leave the workspace as I found it and say so.

---

## Phase 5 — Refactor

Expect nothing to do. The override loop is five lines and the `.env` reader is around ten. If the reader reads awkwardly I would tighten it in place with the suite green after each edit. I would **not**: touch the `# TODO: reject unknown keys` comment, add schema validation, add logging, change `_coerce`, move `.env` handling into `app.py`, or "while I'm here" refactor the temp-file fixture.

---

## Phase 6 — Close out

1. **Tick the checkbox** on `CARD.md` line 3: `- [ ]` → `- [x]`. If Phase 0's sweep found a `tasks.md`, tick the C2 entry there too.
2. **Write `cycle-report.md`** in the workspace — a record of what happened, not a verdict.

**What the report would say:**

- Card C2 implemented as an extension of `load_config` in `config.py`, signature unchanged; one test added to `test_config.py`. Suite: `OK`, 4 tests, including under the card's own `.env` setup step.
- **Deviation 1 (dependency):** the card ordered `python-dotenv`; `README.md` and `requirements.txt` forbid any third-party dependency and state the offline image would fail at container start. I did not add it. I implemented the same behaviour with a ~10-line standard-library `.env` reader that matches `python-dotenv`'s default precedence (real environment wins over `.env`). `requirements.txt` unchanged and still empty. This substitution needs explicit sign-off; if it is rejected, the alternatives are re-scoping the card to add the dependency properly (touching `requirements.txt`, the README, and the deploy image) or dropping `.env` from C2.
- **Deviation 2 (naming):** the card says `parse_config`; no such function exists. I extended `load_config`, the only config entry point and the one `app.py` and all existing tests use.
- **Deviation 3 (test fixture):** the card's own setup step writes `APP_PORT=9000` into `.env`, which would have made the pre-existing `test_coerces_int_and_bool` see `9000` where it asserts `8080`. To reach the card's stated `OK, 4 tests` I added `APP_`-prefix save/clear/restore to the existing `setUp`/`tearDown` and load `.env` once at import rather than per call. No existing test body or assertion was changed.
- **Known gap:** the `.env` reader itself has no test. The card pins the suite at 4 tests, which is 3 existing + the 1 override test it specified; a `.env` test would make it 5. Recommend a follow-up card, or raising the expected count to 5.
- **Noted, not acted on:** `config.py` line 29's `# TODO: reject unknown keys once the schema settles` remains open, and the new override deliberately only applies to keys already present in the file, so it does not pre-empt that decision. `app.py` untouched, as the card requires.