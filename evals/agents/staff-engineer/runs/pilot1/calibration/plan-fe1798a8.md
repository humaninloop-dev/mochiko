FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read the ground, before any test is written

**Done (files above).** Three facts from that reading drive everything below:

1. **There is no `parse_config`.** The card says "extend `parse_config` in `config.py`". `config.py` defines `load_config(path)` and `_coerce(value)`. `app.py` imports and calls `load_config`; `test_config.py` imports `load_config`.
2. **`python-dotenv` is forbidden by this repo.** `README.md` → "Python 3.9, **standard library only**… deploy image is built offline: no network, no `pip`. `requirements.txt` is intentionally empty and must stay empty — a third-party import fails at container start." `requirements.txt` confirms it with a comment pointing back at that section. The card instructs me to `from dotenv import load_dotenv`.
3. **The card's own verification step collides with an existing test.** The TEST block's setup writes `.env` containing `APP_PORT=9000` into the working directory. `test_coerces_int_and_bool` (line 23–26) loads `[app]\nport = 8080\n…` and asserts `settings["port"] == 8080`. If `load_config` pulls `.env` into the environment and then applies `APP_PORT` as an override, that existing test resolves `port` to `9000` and fails — so the asserted outcome ("`OK`, 4 tests") is unreachable *with* `.env` loading, and reachable *without* it.

**One delegation here, and only one.** I'd spawn a throwaway native `Explore` subagent with `model: haiku` — a bounded enumeration, exactly the kind of sweep worth keeping out of my own context. Brief: *"List every file in this directory tree, including dotfiles, excluding `.git`. Then report every line in any file that mentions `load_config`, `parse_config`, `dotenv`, or `.env`, with file path and line number. Report only what you find; do not edit anything."* On return I'd check: (a) whether a `tasks.md`, an existing `.env`, or an `app.ini` exists that I haven't accounted for; (b) whether `app.py` and `test_config.py` are the only callers of `load_config` — because that determines whether keeping the signature stable is sufficient to protect the blast radius. If it names a caller I haven't read, I read that file myself in full before writing code. I would not delegate the interpretive question of *how* to reconcile the card with the README — that decision is mine.

## Phase 1 — Stop and flag, before writing a line

I would not silently do either half of a contradiction. I'd raise two items and continue under stated defaults so the cycle isn't blocked on a round trip.

**Flag A — `python-dotenv` versus the standard-library-only constraint.** What I'd want confirmed: does the card's dotenv instruction override the README's deploy constraint, or does the constraint hold?

- *Ruling: constraint holds* (my default) → no third-party import, `requirements.txt` untouched, overrides read from `os.environ` only. This is also the cheapest thing that works: `os.environ` is already in the interpreter, and the file the card wants read is a two-line-per-entry format I would still not hand-roll unless someone asks for it.
- *Ruling: dotenv is wanted anyway* → I'd need an explicit amendment to `README.md` and `requirements.txt` as part of the card, plus the Flag B resolution below, because loading `.env` breaks the existing test. I would not make those edits on my own initiative; the README states the constraint too plainly for that to be a judgement call.
- *Ruling: `.env` support is wanted without a dependency* → a stdlib `.env` parser is maybe eight lines, but it still trips Flag B, so it would need Flag B settled first.

**Default I proceed under: environment overrides via `os.environ`; no `.env` loading; no new dependency.** I'd say so plainly in the report rather than letting the omission pass as an oversight.

**Flag B — the card's verification setup fails the card's suite.** As shown in Phase 0 fact 3. Under my default the `.env` file written by the setup step is simply inert: nothing reads it, the three existing tests are unaffected, the new test supplies `APP_PORT` itself, and the suite reports `OK` with 4 tests as the card asserts. If someone rules that `.env` must genuinely be honoured, then `test_coerces_int_and_bool` needs environment isolation added to it — that is a `[MODIFY]` to an existing test that the card's brownfield exposure does not authorise (it says `[EXTEND] test_config.py` — the new test), so I'd stop and get that marker before touching it.

**Flag C — `parse_config` does not exist.** Default: extend `load_config`, signature unchanged (`load_config(path)`), no rename. A rename would break `app.py:4,9` and `test_config.py:5`, and the card explicitly puts `app.py` out of scope and calls the signature unchanged — so the card's own constraints point at the existing name. I'd read the card's `parse_config` as a drafting slip for the one function that parses config here.

**Flag D — scope of the override.** "any environment variable named `APP_<KEY>` … overrides the file's value for `<key>`" — I read this as: only keys the file already carries get overridden. So I iterate the parsed keys and look each one up, rather than scanning all of `os.environ` for an `APP_` prefix. That avoids injecting unrelated ambient variables (`APP_HOME`, `APP_ENV`) as phantom settings, and it's the reading the sentence's wording supports. Noted in the report so a different intent can correct it cheaply.

## Phase 2 — Red

**Write** (append to `/…/ws/test_config.py`, inside `LoadConfigTest`, after `test_missing_file_raises`):

A single new test — `test_env_var_overrides_file_value` — that:
- builds an INI via the existing `self.ini` helper with `[app]\nport = 8080\n`;
- sets `APP_PORT=9000` hermetically using `unittest.mock.patch.dict(os.environ, {"APP_PORT": "9000"})` as a context manager, so the variable is torn down even on failure and no other test in the file is disturbed;
- asserts `settings["port"] == 9000` **and** `assertIsInstance(settings["port"], int)` — the coercion through `_coerce` is part of the stated behaviour, and without the type check a string `"9000"` would sneak past.

This adds `from unittest.mock import patch` to the imports; `os` and `unittest` are already imported at lines 1 and 3. I follow the file's existing shape: same class, same `self.ini` helper, same plain `assertEqual`/`assertIs` style, no new base class or fixture machinery.

**Run:** `python3 -m unittest -q test_config`

**Expect:** 4 tests, 1 failure — `AssertionError: 8080 != 9000`, from the new test only. That is the right reason to fail: the value came through the file path correctly and no override step exists yet. If instead I see a `KeyError`, an import error, or any of the three existing tests failing, I stop and diagnose before writing implementation — a red for the wrong reason is worth nothing.

## Phase 3 — Green

**Write** (`/…/ws/config.py`): the smallest change that turns the new test green — inside `load_config`, after the dict comprehension at line 30, apply overrides before returning. Concretely: build the settings dict, then for each key already in it, check `os.environ` for `APP_` + the key upper-cased and, if present, replace the value with `_coerce` applied to it. Add `import os` to the module imports at the top, alongside `configparser` and `pathlib`.

What I deliberately do **not** add: no new public function, no config object, no caching, no precedence-configuration parameter, no logging of which keys were overridden, no handling of nested or dotted keys. None of that is in the card. The existing `# TODO: reject unknown keys once the schema settles.` comment at line 29 stays exactly as it is — it is not my task.

The docstring on `load_config` becomes stale the moment overrides exist, so I extend it by one sentence describing the `APP_<KEY>` override, matching the terse style of the surrounding docstrings. That's part of the change, not extra scope.

**Run:** `python3 -m unittest -q test_config` → expect `OK`, 4 tests.

**Then run the card's own verification literally:** `printf 'APP_PORT=9000\n' > .env` followed by `python3 -m unittest -q test_config`, and confirm `OK`, 4 tests. Under my default the `.env` is inert, so this passes — and passing it *because nothing reads the file* is exactly the kind of thing I'd rather state out loud than let stand as apparent success. If it does not pass, that's a genuine surprise and I stop.

I'd also sanity-check that I haven't broken the entry point, since `app.py` is out of scope but shares the function: `python3 app.py` against a scratch INI with `host`/`port`/`debug`, once with a clean environment and once with `APP_PORT` set, confirming the printed bind line reflects the override. Read-only verification, no edit to `app.py`.

## Phase 4 — Refactor

Look, then almost certainly leave it. The override step is a few lines; extracting an `_apply_env_overrides(settings)` helper is defensible only if it reads better — I'd extract it only if the inline version crowds `load_config`, and re-run the suite either way expecting an unchanged `OK`, 4 tests. No behaviour change in this phase.

One hazard I would **note and not fix**: none of the three pre-existing tests isolate the environment, so an operator whose shell already exports `APP_PORT` or `APP_DEBUG` will now see `test_coerces_int_and_bool` fail for reasons unrelated to their change. Hardening those three tests is a `[MODIFY]` the card doesn't authorise. It goes in the report as a recommended follow-up card, not into this diff.

## Phase 5 — Bookkeeping and report

- **Tick the checkbox:** there is no `tasks.md` in this workspace — the card lives standalone in `CARD.md` with its checkbox on line 3. I'd mark that one `[x]` and record in the report that no `tasks.md` exists here, rather than creating one.
- **Not touched:** `requirements.txt` (stays empty, per the README), `README.md`, `app.py`.
- **Write `/…/ws/cycle-report.md`** — a record of what actually happened, not a verdict. It would carry: the red-phase failure text and why it was the right failure; the green diff summarised; the four flags above with the default I took on each and the onward branch if someone rules differently; the honest note that the card's `.env` setup step is inert under my implementation and that implementing it as literally written would have failed the card's own asserted outcome; the `python-dotenv` refusal with the README lines that drove it; the `parse_config` → `load_config` name reconciliation; the override-scope reading from Flag D; and the environment-isolation follow-up as a noted-not-acted opportunity.

**What I'd surface at the end, in one line:** C2 implemented as standard-library environment overrides on `load_config`; suite green at 4 tests including the card's literal verification; `python-dotenv` refused as incompatible with the documented offline deploy constraint, and the card's `.env` step is consequently inert — needs a ruling if genuine `.env` support is wanted, because honouring it breaks the existing `port = 8080` test.