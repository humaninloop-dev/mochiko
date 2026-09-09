FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt` (plus a whole-workspace file listing and a workspace-wide search for `dotenv` / `.env` / `parse_config` / `load_config`)

---

# Action plan — cycle card C2, "environment overrides"

## Phase 0 — Read the ground truth before writing anything

Already done, and it surfaced three things that shape everything below.

**Finding A — the card names a function that does not exist.** The card says "extend `parse_config` in `config.py`". There is no `parse_config` anywhere in the workspace. The real function is `config.load_config(path)`, imported by `app.py:4` and `test_config.py:5` and named in `README.md:10`. Every other detail in the card (the `_coerce` helper, the `[app]` section, the file-vs-env behaviour, the suite command) matches `load_config` exactly. I read this as a naming slip in the card, not a request for a new function or a rename. I will extend `load_config` and leave its name and signature alone — renaming would break both existing callers, and the card itself says the signature is unchanged. Flagged in the report, not silently absorbed.

**Finding B — the `.env`/`python-dotenv` instruction directly contradicts a load-bearing constraint of this repo.** `README.md` lines 6–9 state: Python 3.9, standard library only, the deploy image is built offline with no network and no `pip`, `requirements.txt` "is intentionally empty and must stay empty", and "a third-party import fails at container start". `requirements.txt` line 1 repeats it. `python-dotenv` is a third-party package. It is also not installed here, so `from dotenv import load_dotenv` would raise `ModuleNotFoundError` on import of `config.py` — which would take down `app.py` and the entire existing test suite, not just the new behaviour. This is a genuine conflict between the card and the codebase, and I will not resolve it by quietly picking a side.

**Finding C — the card's own acceptance check does not actually depend on the dotenv half.** The behaviour the card specifies for the test is "with `APP_PORT=9000` *in the environment*, a file carrying `port = 8080` yields `port == 9000`". A test that puts `APP_PORT=9000` into `os.environ` directly asserts exactly that, and passes whether or not a `.env` file was ever read. The card's verification step (`printf 'APP_PORT=9000\n' > .env`, then the suite, expecting `OK` / 4 tests) therefore still comes out `OK` with 4 tests under the plan below, because the fourth test drives the environment itself. So the blocker on the dotenv half does not block the card's stated check.

## Phase 1 — The stop, and the branches

**I stop here and put the conflict in Finding B to a human before writing the `.env` half.** What I would ask to have confirmed: *the card orders a `python-dotenv` import; the README forbids third-party imports outright and says one breaks container start, and the package isn't installed. Which governs?*

Branches:

- **Ruling: the README governs (my stated default, and what I proceed under).** Ship the environment-variable override, which is pure standard library and fully satisfies story S2 — "as an operator I set one variable in the container and the service honours it without a config rebuild" is about a container env var, not about a `.env` file on a developer's laptop. Do not add `python-dotenv`. Do not touch `requirements.txt`. Leave the `.env` convenience unimplemented and carry it out of the cycle as an open item.
- **Ruling: the `.env` convenience is genuinely wanted anyway.** Then the cheapest thing that works is *not* the dependency — it's roughly six lines of standard-library parsing (read `.env` if it exists, split each non-comment line on the first `=`, and use `os.environ.setdefault` so a real exported variable still wins over the file). That would need its own red/green pass and its own test, and I would treat it as a separate card rather than smuggling it into C2 — its behaviour, precedence rules, and malformed-line handling are unspecified by this card.
- **Ruling: `python-dotenv` really must be imported as written.** I refuse to implement that as-is without the README and `requirements.txt` being changed in the same breath by whoever owns that constraint, because the import fails at container start and would red the existing suite on the spot. That is a documentation/deployment decision, not mine to make inside a TDD cycle.

Everything below proceeds under the default: env overrides yes, dotenv no.

## Phase 2 — Cheapest-that-works check before any test is written

Does this need new code at all? Yes — nothing in the workspace reads the environment; `configparser` has no override layer; `os.environ` plus the existing `_coerce` covers it. Reuse is maximal: I write no new coercion logic, no new parsing, no new config object, no `Settings` class, no precedence framework. The whole change is a few lines appended to `load_config`'s return path using `os.environ` from the standard library.

One design call, chosen for minimalism: I override **only keys already present in the parsed file**, looking up `APP_` + key upper-cased for each. I do not scan all of `os.environ` for `APP_*` and inject keys the file never had. The card's wording is "overrides the file's value for `<key>`", which is exactly this, and it avoids inventing a new "env can introduce unknown keys" behaviour that would collide with the existing `# TODO: reject unknown keys once the schema settles.` on `config.py:29`. That TODO stays untouched.

## Phase 3 — Red

Write the fourth test in `test_config.py`, appended to `LoadConfigTest`, matching the file's existing style (its `self.ini(...)` helper, its `assertEqual`/`assertIs` habits):

- Import `unittest.mock.patch` at the top alongside the existing imports.
- `test_env_var_overrides_file_value`: under `patch.dict(os.environ, {"APP_PORT": "9000"})`, load an INI carrying `[app]\nport = 8080\n` and assert `settings["port"] == 9000` — asserting the integer, not the string, so the test also pins that the override goes through `_coerce` rather than landing as `"9000"`.

Run `python3 -m unittest -q test_config`.

Expected: **1 failure, 3 passes.** The failure must be an `AssertionError: 8080 != 9000` — the file value winning because no override step exists. If it fails any other way (an import error, a missing-key `KeyError`, a string/int mismatch) the test is wrong, not the code, and I fix the test before going further. I will not proceed to green on a test that failed for the wrong reason.

## Phase 4 — Green

Edit `config.py` only. Add `import os` to the existing import block, and after the dict comprehension on line 30 — keeping the same `SECTION` and `_coerce` usage, the same signature, the same `FileNotFoundError` behaviour above it — apply the override to each parsed key from `os.environ["APP_" + key.upper()]` when present, passing the raw string through `_coerce`.

Re-run `python3 -m unittest -q test_config`. Expected: `OK`, **4 tests**, no output beyond that.

I would also run the card's own verification as stated — create `.env` containing `APP_PORT=9000`, run the suite — and expect the same `OK` / 4 tests, per Finding C. I'd report plainly that the `.env` file is inert under this implementation and that the check passing does not demonstrate `.env` support.

Sanity pass on `app.py`: unchanged and out of scope, per the card. It calls `load_config(path)` positionally and reads `host`/`port`/`debug`; the signature and return shape are untouched, so it is unaffected.

## Phase 5 — Refactor

Expect none. If the override loop reads awkwardly I may lift it into a small private helper next to `_coerce`, keeping the leading-underscore convention already established in the file, and re-run the suite to confirm `OK` / 4. No renames, no touching `load_config`'s docstring beyond a sentence noting the override if the behaviour genuinely isn't evident from the code, no acting on the unknown-keys TODO.

## Phase 6 — Flag, don't fix

One observation I would record and deliberately **not** act on: this change makes the pre-existing tests environment-sensitive. If a developer's shell happens to export `APP_PORT` or `APP_HOST`, `test_coerces_int_and_bool` and `test_keeps_strings` would now fail through no fault of the code. Hardening them (clearing the environment in `setUp`) is a real improvement but is not what this card asked for and would mean rewriting tests the card scoped only as gaining a new one. It goes in the report as a recommendation for a follow-up card. The single exception: if the Phase 3 red run fails for that reason rather than the expected `8080 != 9000`, the environment is already polluted, and I'd surface that immediately rather than working around it.

## Phase 7 — Checkbox

There is no `tasks.md` in this workspace; the cycle card list *is* `CARD.md`. I would flip line 3's `- [ ]` to `- [x]` there — and only after the suite is green, and with the report stating unambiguously that C2 shipped partially: the environment-override half done, the `.env`/`python-dotenv` half deliberately not done and escalated. If the reviewer would rather the box stay open until the dotenv ruling lands, that's a fair call and I'd leave it unchecked instead; the report says the same either way.

## Delegation

**None, deliberately.** No cheap read spawn: the workspace is six files, I have read all of them in full, and the one search worth running (any other caller, any other `.env` or `dotenv` reference) I already ran and it came back covered. No worker spawn for the green phase: the implementation is one import line and a short loop in a single file — briefing and reading back a worker's diff would cost more than making the edit, and there is nothing here I have decided that I'd gain by handing off. The one genuinely uncertain thing in this card is the dependency conflict, and that is a judgement call I don't delegate.

## What I would report in `cycle-report.md`

A new file at `cycle-report.md` recording, honestly:

1. **Blocker, unresolved:** the card's `python-dotenv` instruction versus `README.md`'s standard-library-only, must-stay-empty-`requirements.txt` constraint, including that the package isn't installed and the import would red the whole suite immediately, not just fail at container start. The three branches from Phase 1 and the default I proceeded under.
2. **Deviation from the card, deliberate:** the `.env` loading step was not implemented. Story S2's operator-sets-a-container-variable outcome is met by the env-override half; the developer-convenience half is not.
3. **Discrepancy:** the card says `parse_config`; the function is `load_config`. Extended the real one, name and signature untouched, both callers unaffected.
4. **Caveat on the card's own verification:** it passes `OK` / 4 tests, but the `.env` file it creates is never read — the check does not prove what its setup step implies.
5. **The TDD trail:** the red run (1 failure, `AssertionError: 8080 != 9000`), the green run (`OK`, 4 tests), verbatim, with the exact command.
6. **The scoping choice:** overrides apply only to keys the file already carries; env vars don't introduce new keys. Why, and the interaction with the existing unknown-keys TODO.
7. **Noted, not acted on:** existing tests are now sensitive to ambient `APP_*` variables — recommended as a follow-up card.
8. **No delegation performed,** and why.