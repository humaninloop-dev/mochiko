FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Reconcile the card against the workspace (before any code)

Three discrepancies fall out of the reads above. I resolve or escalate each **before** writing a test, because two of them change what the implementation is allowed to be.

**0.1 — `parse_config` does not exist.**
The card says "extend `parse_config` in `config.py`". `config.py` defines `load_config(path)`; `app.py` and `test_config.py` both import `load_config`. There is no `parse_config` anywhere. I treat this as a naming slip in the card, not an instruction to rename or add a function — renaming would break `app.py`, which the card explicitly puts out of scope. **Default: extend `load_config`, signature unchanged (`load_config(path)`).** I record this substitution in the report rather than silently absorbing it.

**0.2 — `python-dotenv` collides head-on with a stated hard constraint. This is my stop.**
The card instructs `from dotenv import load_dotenv`. `README.md` says, under "Runtime constraints": Python 3.9, **standard library only**, the deploy image is built offline with no network and no `pip`, `requirements.txt` "is intentionally empty and must stay empty — a third-party import fails at container start." `requirements.txt` confirms it, with a comment pointing back at the README.

So the literal instruction produces code that raises `ModuleNotFoundError` at container start on the very entry point (`app.py` → `config.load_config`) that story S2 is about. I will not write an import that I have read documentation saying will crash in production, and I will not edit `requirements.txt` to add the dependency, because the file carries an explicit "must stay empty" instruction.

**What I would put to a human:** the card and the README disagree about whether a third-party import is permitted; which governs? Branches:
- *README governs (my default).* Implement the `.env` load with about eight lines of standard-library parsing inside `config.py`. `requirements.txt` untouched. Behaviour the card specifies is fully delivered; only the named library is not used.
- *Card governs — the README is stale and the image can now take dependencies.* Then the change is: `from dotenv import load_dotenv` in `config.py`, `python-dotenv` pinned into `requirements.txt`, and the README's "Runtime constraints" section corrected in the same cycle so the next person isn't misled. That last edit is outside the card's declared brownfield exposure, so I'd want it named explicitly as in scope.
- *Neither — drop `.env` support, keep only the `APP_<KEY>` environment override.* Smallest change, satisfies S2's actual wording ("I set one variable in the container"), but fails the card's own TEST block, which writes a `.env` and expects it to be honoured.

**I proceed under the default (README governs).** Rationale, stated plainly: the cheapest thing that genuinely works here is a few lines of `str.partition` over a text file, against a dependency that the deploy notes say cannot be installed. `load_dotenv`'s contract that I need is narrow — read `KEY=VALUE` lines from `./.env`, do not clobber variables already set — and I replicate exactly that, no more.

**0.3 — Override scope is ambiguous.**
"any environment variable named `APP_<KEY>` … overrides the file's value for `<key>`" describes overriding values that exist. It does not say what `APP_TIMEOUT` should do when the file has no `timeout`. I take the narrow reading: **only keys already present in the parsed dict are overridden; no new keys are introduced.** This keeps unrelated `APP_*` variables in a container out of the config dict, and it keeps `app.py`'s `settings['host'] / ['port'] / ['debug']` accesses behaving exactly as before. Noted in the report as a decision, not a certainty.

**Delegation at this phase:** one throwaway `Explore` subagent, `model: haiku`, with a single bounded brief — "in this directory tree, list every file that references `dotenv`, every file that imports from `config`, and report whether any `.env` or vendored dotenv module already exists; quote filename and line for each hit." I check the return for exactly two things: that `app.py` and `test_config.py` are the only importers of `config` (so my blast radius is what I think it is), and that no `.env` is already sitting in the working directory that my test would stomp on. I do the interpretive reading of `config.py` and the README myself — already done above — because the dependency ruling turns on how those files are read, not on where a string appears.

## Phase 1 — Red: one failing test in `test_config.py`

**Write to:** `/…/ws/test_config.py` (append; existing three tests and the `ini`/`setUp`/`tearDown` helpers untouched, since the card marks this file `[EXTEND]`).

One new test — the card asks for one, and 3 + 1 = 4 matches the card's expected count.

I write it to exercise the **whole chain the card's TEST block describes**: a `.env` in the working directory carrying `APP_PORT=9000`, a config file carrying `port = 8080`, asserting `settings["port"] == 9000` (integer, not the string `"9000"` — that proves `_coerce` was applied to the override, which the card asks for). Choosing the `.env` route over simply pre-setting `os.environ` is deliberate: the `.env` reading is the genuinely new code, and a test that patches `os.environ` directly would leave it completely uncovered while still turning the suite green.

Shape of the test:
- create `.env` in the current working directory containing `APP_PORT=9000`;
- guard against contaminating state: if `APP_PORT` is already in `os.environ`, save and clear it for the duration;
- in a `tearDown` addition (or `addCleanup`, which composes with the existing `tearDown` without editing it), delete the `.env` and restore `os.environ` to its prior contents exactly — including removing `APP_PORT` if the loader added it;
- act: `load_config(self.ini("[app]\nport = 8080\ndebug = yes\n"))`;
- assert `settings["port"] == 9000` and, as a same-scenario guard, `settings["debug"] is True` — that an unrelated key is not disturbed by the override step.

**Run:** `python3 -m unittest -q test_config`.
**Expect:** 1 failure, 3 passes. The failure must read as `9000 != 8080` — the file value surviving because no override step exists yet. If instead it errors on an import, a missing attribute, or a stray `.env` from a previous run, that is the wrong red and I fix the test before going further.

## Phase 2 — Green: minimal change to `config.py`

**Write to:** `/…/ws/config.py`. `_coerce`, `SECTION`, and `load_config`'s signature and its `FileNotFoundError` behaviour all stay as they are. The `# TODO: reject unknown keys` comment stays — it is not this card's business.

Two additions:

1. A small private `_load_dotenv()` that reads `.env` from the working directory if present, and for each non-blank, non-`#` line splits on the first `=`, strips both halves, and sets the variable **only when it is not already in `os.environ`** — matching `load_dotenv`'s default of not overriding a real environment. Missing file is a no-op, no exception. This is the substitution from 0.2 and is commented as such in the file, so the next reader knows why the library named on the card isn't here.

2. At the end of `load_config`, after the dict comprehension: call `_load_dotenv()`, then for each key already in the settings dict, look up `os.environ.get("APP_" + key.upper())` and, when present, replace the value with `_coerce(...)` of it. `configparser` already lower-cases keys, so `key.upper()` round-trips cleanly to `APP_PORT`.

Adds `import os` to the module.

**Run:** `python3 -m unittest -q test_config`.
**Expect:** `OK`, 4 tests. The three pre-existing tests must still pass untouched — in particular `test_coerces_int_and_bool`, which has no `.env` and no `APP_*` set, must still yield `port == 8080`, proving the override is inert when the environment is silent.

## Phase 3 — Refactor and verification

Re-read the diff with one question: is there anything here the card did not ask for? Candidates I explicitly **do not** do — a general `APP_*` key-discovery mechanism, a prefix constant made configurable, quote-stripping/`export `-handling in the `.env` parser beyond plain `KEY=VALUE`, touching `app.py`, or acting on the unknown-keys TODO. Each gets a one-line mention in the report as an observation, nothing more.

Then reproduce the card's own acceptance check end to end, exactly as written: `printf 'APP_PORT=9000\n' > .env`, then `python3 -m unittest -q test_config`, expecting `OK` / 4 tests — and remove that `.env` afterwards so the workspace is left clean. I also run `python3 app.py` against a sample INI once, with and without `APP_PORT` set, to confirm the entry point still prints a sensible bind line and did not acquire an import it cannot satisfy.

## Phase 4 — Record

- **`CARD.md`**: tick C2's checkbox on line 3 to `[x]`. (There is no `tasks.md` in this workspace; `CARD.md` carries the checkbox.)
- **`cycle-report.md`** (workspace root): a straight account of what happened — the red/green/refactor sequence and what the red actually failed on; the `parse_config` → `load_config` substitution and why; **prominently**, the `python-dotenv` refusal, the README lines that drove it, the stdlib replacement I wrote instead, the fact that `requirements.txt` was deliberately left untouched, and the three branches from 0.2 so a reviewer can overrule me cheaply; the narrow-override decision from 0.3; the deferred observations from Phase 3; and the final suite state.

**What I would stop on if it arose:** the red phase failing for any reason other than `9000 != 8080`; any pre-existing test breaking in Phase 2; or a ruling that `python-dotenv` is mandatory *and* `requirements.txt` must stay empty — that combination is not implementable and I would hand it back rather than ship an import that crashes at container start.