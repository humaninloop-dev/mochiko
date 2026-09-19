FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read the ground before writing anything

**Done above.** What I found, and the two conflicts it creates:

**Conflict 1 — the function name.** The card says "extend `parse_config` in `config.py`". There is no `parse_config`. `config.py` defines `load_config(path)`, and both `test_config.py` (line 5) and `app.py` (line 4) import `load_config`. The card's description of the function ("after the file is parsed… coerced with the existing `_coerce` helper") matches `load_config` exactly, and the card also says the signature is unchanged and `app.py` is out of scope. **Ruling I would take: `parse_config` is a naming slip for `load_config`. I extend `load_config` in place and rename nothing** — renaming would break both importers and contradicts "signature unchanged" and "`app.py` out of scope". Flagged in the report, not silently absorbed.

**Conflict 2 — `python-dotenv` is forbidden by this repo. This is the real blocker.** The card instructs `from dotenv import load_dotenv`. `README.md` lines 6–9 state: Python 3.9, **standard library only**, the deploy image is built offline with no network and no `pip`, `requirements.txt` "is intentionally empty and must stay empty", and "a third-party import fails at container start". `requirements.txt` line 1 repeats the same instruction as a comment. A top-level `from dotenv import ...` in `config.py` would be imported by `app.py` at line 4 — so on a machine without the package it does not degrade, it kills the entry point at container start, and it would also break all three existing tests (they import `config`), not just my new one.

**Stop point.** I would stop here and put the ruling to the human: *the card's mandated dependency directly contradicts the documented runtime constraint — which wins?* Branches:

- **(A) README wins — my stated default, and what the rest of this plan assumes.** Deliver the card's *behaviour* (a `.env` in the working directory feeds the override step, so local dev needs no `export`) with a ~12-line standard-library reader instead of the package. The behaviour the story asks for is intact; only the mechanism changes, and I disclose the substitution.
- **(B) Card wins, dependency approved.** Then `requirements.txt` gains `python-dotenv`, `README.md`'s "standard library only" paragraph must be amended in the same change (otherwise the repo now lies about itself), and I would ask for confirmation that the offline deploy image can actually vendor the wheel before I write a line — an approval that doesn't survive the image build is worse than no approval. Import at module top exactly as the card writes it.
- **(C) Drop the `.env` half.** Implement only the `APP_<KEY>` override from `os.environ` and mark the `.env` clause as descoped. Cheapest of the three; loses the "no exporting" convenience.

I proceed under (A). Nothing below depends on the ruling except which lines of Phase 3 get written, so a later reversal to (B) or (C) is a small, contained edit.

**Cheapest-that-works pass, before any test is written.** The override step itself is pure standard library: `os.environ` plus the `_coerce` that already exists at `config.py` line 8 — no new helper, no new abstraction, nothing to install. The only thing genuinely absent from the standard library is `.env` parsing, and the subset this card needs (`KEY=value` lines) is a handful of lines. I would *not* build a general dotenv implementation — no `export ` prefix, no quote stripping, no variable interpolation, no multi-line values. Nothing in this repo writes such a file; the card's own setup writes exactly `APP_PORT=9000`. Small because it's all that's needed.

**One cheap lookup I would delegate** — a throwaway `Explore` subagent pinned to `model: haiku`, brief: *"Report only: is a `dotenv` / `python-dotenv` distribution importable by the `python3` on PATH? Give the site-packages path or state absent. Do not modify anything."* I check the return is a path or a clean absence with provenance, nothing interpretive. This is corroboration for the report only — my ruling rests on `README.md`, so the answer changes nothing even if the package happens to be installed on this laptop; the deploy image is what matters.

## Phase 1 — Red: the failing test

**Write into `test_config.py`** (extends the file; no existing assertion is weakened):

1. Add `from unittest.mock import patch` to the imports.
2. **New test** `test_env_var_overrides_file_value`: inside `patch.dict(os.environ, {"APP_PORT": "9000"})`, call `load_config(self.ini("[app]\nport = 8080\nhost = 0.0.0.0\n"))` and assert `settings["port"] == 9000` (integer — proving it went through `_coerce`, not a `"9000"` string) and `settings["host"] == "0.0.0.0"` (proving an un-overridden key is untouched). This is the behaviour the card names.
3. **Isolation added to the existing `setUp`/`tearDown`** — and I want to be explicit that this is a deliberate change to existing test scaffolding, because the card's own acceptance recipe forces it. The card's TEST block writes `.env` containing `APP_PORT=9000` into the working directory and then asserts the suite is `OK` with 4 tests. But once `.env` feeds the override step, `test_coerces_int_and_bool` (line 24–26) asserts `settings["port"] == 8080` while `.env` is pushing 9000 — **the card's setup breaks the card's assertion.** So `setUp` chdirs into a fresh `tempfile.TemporaryDirectory()` (no `.env` is found there) and enters `patch.dict(os.environ, {}, clear=True)`; `tearDown` restores the original cwd and stops the patch, before the existing unlink loop (those paths are absolute, so chdir is safe for them). The three existing tests keep passing unchanged, and the suite stops depending on whatever `APP_*` variables or stray `.env` happen to sit around the runner.

**Run** `python3 -m unittest -q test_config`. **Expected:** 4 tests, 1 failure — `test_env_var_overrides_file_value` failing on `8080 != 9000`. That specific message is what I check for: it means the file value is reaching the assertion and the override step simply isn't there yet. An `ImportError`, an `AttributeError`, or a failure in one of the other three tests means my scaffolding is wrong, not that the feature is missing, and I fix the test before writing any production code.

**A gap I would flag rather than paper over:** with this isolation, the `.env`-reading path has no automated coverage — it is exercised only by the card's manual setup. I would happily add a fifth test (temp cwd, write a `.env`, assert the override lands), but the card asserts the suite reports exactly **4 tests**. I hold at 4, and I say in the report that the `.env` branch is covered only by the manual check and that I'd add the fifth test on request.

## Phase 2 — Green: the smallest change to `config.py`

Add `import os` beside the existing imports; keep `SECTION`, `_coerce`, and `load_config`'s signature exactly as they are.

- `_dotenv_values()` — reads `.env` from the working directory via the already-imported `Path`; a missing or unreadable file returns `{}` (no raise — `.env` is optional by definition). Skips blank lines and `#` comments, splits each remaining line on the first `=`, strips both sides.
- `_overrides(keys)` — starts from `_dotenv_values()`, then `update(os.environ)` so a **real environment variable beats a `.env` entry** (matching `load_dotenv`'s default precedence, so a later swap to branch (B) is behaviour-neutral). For each key already in the parsed file, if `APP_ + key.upper()` is present, coerce its value with `_coerce`.
- In `load_config`: build the dict as today, then apply the overrides before returning. Two added lines at the end.

**A scope decision I would state, not bury:** I override **only keys that the file already defines**, because the card says "overrides *the file's* value for `<key>`". This also means a stray `APP_HOME` or `APP_VERSION` in a container's environment cannot inject a phantom config key — which sits well with the standing `# TODO: reject unknown keys` at line 29. If the intent was that `APP_*` can *introduce* keys, that's a one-line change and I'd want it said explicitly rather than assumed.

I do **not** touch `app.py` (out of scope), `README.md`, or `requirements.txt` under branch (A) — the standard-library-only claim stays true, so there is nothing to correct.

**Run** `python3 -m unittest -q test_config`. **Expected:** `OK`, 4 tests.

**Delegation call for this phase: none.** The change is roughly fifteen lines across one file, and briefing a worker would cost more than writing it. Handing off a decided green phase is a tool I'd use on a broader change; here the whole implementation is smaller than its own brief, and every judgement in it — the precedence rule, the file-keys-only scope, the deliberate thinness of the parser — is mine to make and defend.

## Phase 3 — Verify the card's own acceptance recipe

Run it literally, because it is the thing that will be checked: `printf 'APP_PORT=9000\n' > .env`, then `python3 -m unittest -q test_config`. **Expected:** `OK`, 4 tests. I would note honestly that Phase 1's isolation makes the suite pass *whether or not* `.env` is present — the setup step is inert against an isolated suite. I would not manufacture a dependence on ambient files to make the recipe look load-bearing.

I would additionally sanity-check the entry point by hand, since `app.py` imports the module I changed even though it's out of scope: with a small `app.ini` and `APP_PORT=9000` exported, `python3 app.py app.ini` should print port 9000; with nothing exported, the file's value. No code change to `app.py` either way — this is a regression check on my own edit, and if it fails I fix `config.py`, not `app.py`.

## Phase 4 — Refactor

Re-read the finished `config.py` for anything the tests would still hold: `_dotenv_values` and `_overrides` stay module-private beside `_coerce`, docstrings in the same one-line style as the existing ones. I expect no restructuring — the change is small enough that there is unlikely to be anything to clean. Re-run the suite; expect `OK`, 4 tests, unchanged.

## Phase 5 — Bookkeeping

No `tasks.md` exists in this workspace. I would glob for one first; if it genuinely isn't there, I flip C2's checkbox to `[x]` in `CARD.md` itself (line 3) and say in the report that I did so because the file the process expects is absent — I would not create a `tasks.md` that no one asked for.

## Phase 6 — What I report

`cycle-report.md` covering:

- **The blocker, first and plainly:** the card mandates `python-dotenv`; `README.md` and `requirements.txt` forbid any third-party import and say it fails at container start. I did not import it. I delivered the `.env` behaviour with a deliberately minimal standard-library reader and I name that substitution as a deviation from the card's letter, with the three branches above so the human can overrule me cheaply.
- **The naming slip:** `parse_config` does not exist; I extended `load_config` and renamed nothing.
- **The self-inconsistent acceptance recipe:** writing `.env` with `APP_PORT=9000` would have broken the pre-existing `port == 8080` assertion. I resolved it by isolating the suite from the ambient environment and working directory — a change to existing test scaffolding that the card did not ask for and that I consider unavoidable, so it is called out rather than folded in quietly.
- **The untested `.env` path**, and why I stopped at 4 tests.
- **The scope decision** that only file-defined keys can be overridden, with the reasoning and the one-line alternative.
- **Delegation disclosure:** one cheap read-only lookup for whether `python-dotenv` is installed locally; no implementation work delegated.
- **Verbatim command output with exit codes** for the red run (1 failure, `8080 != 9000`), the green run, and the card's literal recipe.
- **Noted, not acted on:** the `# TODO: reject unknown keys` at line 29 is now more tractable, and `_coerce`'s int-only numeric handling means `APP_TIMEOUT=1.5` arrives as the string `"1.5"`. Both are out of scope for C2 and I leave them alone.