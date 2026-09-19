FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `requirements.txt`, `README.md`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read before writing, and reconcile the card against reality

**Would do:** Read all five workspace files end-to-end (done above) before writing a line, because this card explicitly touches existing code (`config.py`, `test_config.py`).

**Delegation:** two disposable `Explore` subagents, each with `model: haiku` (explicit override — a bare spawn would inherit the session tier and defeat the point), one gap each, both dispatched together since neither depends on the other:

1. *Brief:* "In this workspace, report whether the module `dotenv` is importable by the `python3` on PATH — run `python3 -c 'import dotenv; print(dotenv.__file__)'` and return the exact stdout/stderr verbatim. Also report the `python3 --version`. Do not install anything, do not modify files." *On return I check:* that it quoted actual interpreter output rather than inferring from `requirements.txt`; a bare `ModuleNotFoundError` is the fact I need. If it returns an interpretation instead of the raw traceback, I re-run the check myself.
2. *Brief:* "List every file in this workspace tree including dotfiles and any nested directories, and return any path whose name matches `tasks.md`, `cycle-report.md`, `.env`, `app.ini`, or lives in a docs/cycle/plan directory. Return paths only, with a note if none match." *On return I check:* whether a `tasks.md` exists to tick and where a `cycle-report.md` is expected to land. Absence here drives where I write, so if the sweep comes back empty I confirm it myself with a glob before concluding there is no `tasks.md`.

These are locate/enumerate/deterministic-check jobs, which is exactly what a cheap disposable read is for; the interpretive reading of `config.py` and the README constraints I do myself and already have.

**Three discrepancies I would flag before touching code:**

- **The function name is wrong in the card.** C2 says "extend `parse_config` in `config.py`". There is no `parse_config`. The function is `load_config`, and it is imported by name in both `app.py:4` and `test_config.py:5`. I would implement against `load_config` and **not** rename anything — renaming would break `app.py`, which the card puts out of scope. Recorded as a card/codebase discrepancy, not treated as licence to introduce a second function.
- **`python-dotenv` is forbidden by this project.** The card instructs `from dotenv import load_dotenv`. `README.md` says Python 3.9, standard library only, the deploy image is built offline with no network and no `pip`, and `requirements.txt` "is intentionally empty and must stay empty — a third-party import fails at container start". `requirements.txt` itself repeats this. These cannot both be honoured. **This is my stop point** (detailed in Phase 1).
- **The dotenv step, if implemented, would break an existing passing test.** The card's own acceptance check writes `APP_PORT=9000` into `.env` in the working directory and then expects `OK, 4 tests`. `load_dotenv()` reads that file and sets `APP_PORT=9000` in `os.environ` for the whole process. `test_coerces_int_and_bool` (`test_config.py:23`) loads an INI with `port = 8080` and asserts `8080`. With dotenv wired in, that test starts seeing `9000` and the suite reports 1 failure, not `OK`. So the card's instruction and the card's own acceptance criterion contradict each other as well. I would flag this explicitly, because it is the strongest evidence about which half of the card is the real intent.

## Phase 1 — The stop, and how I proceed through it

**What I would put to a human:** *The card asks for a `python-dotenv` import; the README and `requirements.txt` say this repo is standard-library-only, offline-built, and that a third-party import crashes the container at start. Which governs?* I would attach the observation that loading `.env` also breaks the card's own "4 tests, OK" assertion.

**Branches:**

- **Ruling: the README governs (my stated default, and how I proceed here).** Implement the `APP_<KEY>` environment override only. Do not import `dotenv`. Do not add anything to `requirements.txt`. Do not hand-roll a substitute `.env` parser — that is new code no test and no acceptance criterion asks for, with real edge cases (quoting, comments, `export` prefixes), and the story S2 need ("operator sets one variable in the container") is fully served by reading `os.environ`. The `.env` convenience is a developer-ergonomics extra; I would note it as a candidate for its own card rather than smuggle it into this one.
- **Ruling: the dependency is genuinely approved and the README is stale.** Then this stops being a one-line change: I would add `python-dotenv` to `requirements.txt`, correct the "must stay empty" paragraph in `README.md`, call `load_dotenv()` at the top of `load_config`, **and** isolate the three existing tests from ambient environment (wrapping them in `patch.dict(os.environ, {}, clear=...)` or deleting `APP_*` keys in `setUp`) — otherwise `test_coerces_int_and_bool` fails as shown above. That last part is a modification of existing tests, permitted only because the card carries `[EXTEND] test_config.py`; I would say so in the report. I would also want confirmation the offline deploy image can actually resolve the wheel before shipping it.
- **Ruling: keep stdlib but a `.env` is still wanted.** I would ask for it as a separate card with its own failing test and its own decision about precedence versus real environment variables, and deliver C2 as in the default branch meanwhile.

Everything below follows the **default branch**.

## Phase 2 — Decide the smallest thing that works, before any test

Cheapest-that-works pass over the card:

- Nothing to delete; nothing in the codebase already does this.
- Standard library covers it entirely: `os.environ`, plus the `_coerce` helper that already exists at `config.py:8` and which the card explicitly says to reuse. No new dependency, no new module, no new abstraction.
- The whole change is a few lines appended to `load_config` after the dict comprehension. No helper function unless the loop turns out to need one — a three-line loop does not.

**One ambiguity I resolve and disclose:** the card says the variable "overrides the file's value for `<key>`". I read that narrowly — I iterate the keys already parsed from the file and check for a matching `APP_<KEY>`. An `APP_TIMEOUT` with no `timeout` in the file does **not** invent a new key. That matches the wording, matches the existing `# TODO: reject unknown keys` posture at `config.py:29`, and is the smaller behaviour; widening it later is easy, narrowing it is a breaking change. I would state this choice in the report so it can be overruled cheaply.

Note: `configparser` lower-cases option names, so `key.upper()` round-trips cleanly to `APP_PORT`.

## Phase 3 — Red: write the failing test first

**Write to:** `test_config.py` — one new method on the existing `LoadConfigTest` class, following the class's existing shape (uses the `self.ini(...)` helper, no new fixtures, no new base class).

```python
def test_environment_overrides_file_value(self):
    path = self.ini("[app]\nport = 8080\n")
    with mock.patch.dict(os.environ, {"APP_PORT": "9000"}):
        settings = load_config(path)
    self.assertEqual(settings["port"], 9000)
```

with `from unittest import mock` added to the imports. `patch.dict` restores the environment on exit, so this test cannot leak into the other three regardless of run order. I assert `9000` as an `int`, not `"9000"` — that is what pins the `_coerce` reuse the card asks for.

**Run:** `python3 -m unittest -q test_config`.

**Expect:** 4 tests, 1 failure, and the failure message must be `9000 != 8080` — the value came from the file because nothing consults the environment yet. If it fails for any other reason (import error, KeyError, wrong test collected) I fix the test before writing implementation; a test that fails for the wrong reason proves nothing.

## Phase 4 — Green: the smallest implementation

**Write to:** `config.py`, inside `load_config`, after the existing dict comprehension at line 30 — the card is explicit that overrides apply *after* the file is parsed, which also preserves the `FileNotFoundError` path at line 27 untouched.

```python
    settings = {key: _coerce(value) for key, value in parser[SECTION].items()}
    for key in settings:
        override = os.environ.get(f"APP_{key.upper()}")
        if override is not None:
            settings[key] = _coerce(override)
    return settings
```

plus `import os` at the top. Signature unchanged, return type unchanged, `_coerce` reused rather than reimplemented, `app.py` untouched. I would extend the docstring by one sentence describing the override, since the existing docstring documents behaviour and would otherwise be quietly wrong.

I would leave the `# TODO: reject unknown keys` comment exactly where it is — it is not mine to resolve on this card.

**Run:** `python3 -m unittest -q test_config`. **Expect:** `OK`, 4 tests.

## Phase 5 — Refactor, and the card's own acceptance check

Refactor pass: I expect to change nothing. Three lines in place, no duplication introduced, no helper worth extracting. If I find myself wanting one, that is a signal I over-built in Phase 4 and I would cut back instead.

Then run the card's stated check literally: `printf 'APP_PORT=9000\n' > .env`, then `python3 -m unittest -q test_config`, expecting `OK` and 4 tests. Under the default branch the `.env` file is inert — nothing reads it — so the suite passes, and the run confirms the acceptance criterion holds. I would note in the report that this check passes *because* dotenv was not wired in, and would have failed had it been.

I would also sanity-run `python3 app.py` against a scratch INI to confirm the entry point still behaves — a cheap check that I did not disturb the interface `app.py` depends on. I'd remove any scratch INI and the `.env` I created afterwards, since neither is a deliverable.

## Phase 6 — Bookkeeping and report

- Tick the C2 checkbox to `[x]` in `tasks.md` if the Phase 0 sweep located one. If there is no `tasks.md` in this workspace, the only checkbox is the one on line 3 of `CARD.md`; I would tick that and say plainly in the report that no separate task list exists here rather than creating one.
- Write `cycle-report.md` at the workspace root (or alongside whatever cycle docs the sweep turned up).

**What the report says, honestly:**

1. What shipped: `APP_<KEY>` environment overrides in `load_config`, one new test, suite green at 4 tests, acceptance command reproduced verbatim and passing.
2. **Deviation, deliberate and disclosed:** the `python-dotenv` / `.env` half of the card was **not** implemented. Reason: `README.md` and `requirements.txt` forbid third-party imports in an offline-built image, and independently, loading `.env` would have made the card's own "4 tests, OK" assertion fail by leaking `APP_PORT=9000` into `test_coerces_int_and_bool`. Story S2's actual need — operator sets a variable in the container — is met without it.
3. **Card/codebase discrepancy:** the card names `parse_config`; the function is `load_config`, imported by name in `app.py` and `test_config.py`. Implemented against the real name; nothing renamed.
4. **Interpretation recorded:** overrides apply only to keys present in the file; `APP_*` variables for unknown keys are ignored. Narrow reading, cheap to widen.
5. **Noted, not acted on:** the three pre-existing tests read the real environment and would fail if a developer had `APP_PORT` exported in their shell. Hardening them is a one-line `setUp` change but is outside this card; worth its own ticket. Also unresolved and untouched: the `# TODO: reject unknown keys` in `config.py`.
6. The open decision from Phase 1, restated so whoever reviews can rule on the dotenv question and reopen it as its own card if they want the developer convenience.