FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt`

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read the ground truth before touching anything

Already done: all five workspace files plus the card. What the reading establishes:

- The function the card calls `parse_config` **does not exist**. The real function is `config.load_config(path)`, imported by `app.py:4` and `test_config.py:5`. The card also says the signature is unchanged and `app.py` is out of scope — so a rename is ruled out by the card itself. I read this as a naming slip in the card and implement inside `load_config`. I will not rename, and I will record the discrepancy in the report.
- `_coerce` (config.py:8–18) already does exactly the string→bool/int/str coercion the card wants applied to env values. Reuse it; write no new coercion.
- The suite today is 3 tests; the card's acceptance expects 4. So exactly one new test.
- `README.md:6–9` states: Python 3.9, **standard library only**, image built offline with no network and no `pip`, `requirements.txt` "intentionally empty and must stay empty", and *"a third-party import fails at container start"*. `requirements.txt` repeats this. The card instructs `from dotenv import load_dotenv` from `python-dotenv`. **These are in direct conflict.**

## Phase 1 — Stop and flag the dependency conflict (before any code)

This is the one place I stop rather than proceed silently.

**What I would put to a human, with evidence:** the card's `.env`/`python-dotenv` instruction contradicts the project's stated runtime constraints. Adding the import would not merely violate a doc — per the README it would crash the service at container start in an image that cannot `pip install` the package. I would also state the supporting evidence I'd gather: whether `dotenv` is even importable in this checkout (I expect it is not, since `requirements.txt` is empty), which would make the card's import a hard `ImportError` in the suite as well.

**What I would confirm:** which of the two documents governs — the card or the README.

**Branches:**
- *README governs (my default, see below):* ship the environment-variable override only, using `os.environ`. No import of `dotenv`, no edit to `requirements.txt`. The `.env`-file portion of the card is left unimplemented and reported as blocked.
- *Card governs and the dependency is genuinely approved:* add `python-dotenv` to `requirements.txt`, add `load_dotenv()` at the top of the override step, and require the README's "Runtime constraints" section be amended in the same change — otherwise the repo now documents two contradictory rules. I would still push back once, because "no pip in the deploy image" is an infrastructure fact that an approval does not change.
- *Middle ruling — `.env` behaviour wanted, third-party dependency not:* implement a ~12-line stdlib reader in `config.py` that parses `KEY=VALUE` lines from `./.env` into `os.environ` without overwriting already-set variables. I would not do this unilaterally, because substituting my own design for a named library is a design decision, not an implementation one.

**Default I proceed under:** README governs. Rationale: the README describes a deployment reality, the card describes a convenience; and — important for honesty — the card's own acceptance check (`printf 'APP_PORT=9000\n' > .env` then run the suite, expect OK/4 tests) does **not** actually discriminate between the branches. My test will set `APP_PORT` in the process environment directly, so the suite passes green whether or not `.env` is read. I will say exactly that in the report rather than let a green run imply the `.env` requirement was met.

Nothing in Phase 1 blocks the rest of the card: the story ("operator sets one variable in the container and the service honours it") is satisfied entirely by `os.environ`.

## Phase 2 — Cheapest-thing-that-works pass before writing a test

- Does this need to exist? Yes — no override path exists in `load_config` today.
- Can it be reused? `_coerce` covers coercion. `os.environ` covers the environment. `str.removeprefix` is 3.9+… actually `removeprefix` is 3.9, which is fine, but I'll use slicing to stay obviously safe on the stated 3.9 floor.
- Third-party? Rejected in Phase 1.
- Net new code: roughly four lines inside `load_config` plus `import os`. No new module, no new class, no config schema, no changes to `app.py`.

**One design ambiguity I resolve and disclose:** the card says env vars *"override the file's value for `<key>`"*. I implement it as override-only-for-keys-already-in-the-file, not as "inject any `APP_*` variable as a new key". Reason: a machine's ambient environment can carry unrelated `APP_`-prefixed variables, and injecting them would silently invent config keys the file never declared — right next to the existing `# TODO: reject unknown keys` note. The card's test only exercises a key present in the file, so this is the narrower, safer reading. It goes in the report as a judgement call, not a fact.

## Phase 3 — Baseline

Run `python3 -m unittest -q test_config`. Expect: `OK`, 3 tests. If it is not green before I start, I stop and report that instead of building on a broken baseline.

## Phase 4 — Red

Edit `test_config.py` (the card's `[EXTEND]` marker covers this). Add `from unittest import mock` to the existing imports and one test to `LoadConfigTest`, following the file's existing shape — same `self.ini(...)` helper, same assertion style:

```python
def test_env_var_overrides_file_value(self):
    path = self.ini("[app]\nport = 8080\n")
    with mock.patch.dict(os.environ, {"APP_PORT": "9000"}):
        settings = load_config(path)
    self.assertEqual(settings["port"], 9000)
```

`mock.patch.dict` rather than assigning `os.environ` directly, so the variable cannot leak into the other three tests or out of the run.

Run `python3 -m unittest -q test_config`. **Expected failure — and I check the reason, not just the redness:** `AssertionError: 8080 != 9000`, 4 tests run, 1 failure. If instead I see `ImportError`, `KeyError`, or an error from `tearDown`, the test is failing for the wrong reason and I fix the test before writing any implementation.

## Phase 5 — Green

Edit `config.py`. Add `import os` alongside `configparser`, and insert the override step in `load_config` between the parse and the return — keeping the signature, the `FileNotFoundError` behaviour, the `SECTION` constant, the existing TODO comment, and the docstring style intact:

```python
settings = {key: _coerce(value) for key, value in parser[SECTION].items()}
for key in settings:
    override = os.environ.get(f"{ENV_PREFIX}{key.upper()}")
    if override is not None:
        settings[key] = _coerce(override)
return settings
```

with `ENV_PREFIX = "APP_"` next to `SECTION`, and the docstring extended by one sentence describing the override. No other file changes. `app.py` untouched. `requirements.txt` untouched and still empty.

Run the suite. Expect `OK`, 4 tests.

## Phase 6 — Refactor

Reread the diff. My expectation is that there is nothing to extract — four lines inside the function that already owned this responsibility. I would specifically resist pulling out an `_apply_env_overrides` helper or a settings class; neither is asked for and neither pays for itself at this size. Re-run the suite after any change; it must stay `OK`/4.

## Phase 7 — Acceptance rehearsal, honestly reported

Run the card's own check: `printf 'APP_PORT=9000\n' > .env`, then `python3 -m unittest -q test_config`. Expect `OK`, 4 tests. I would then delete the `.env` I created so it is not left behind as an untracked artifact, and re-run to confirm still `OK`/4 — that second run is the evidence that the green is coming from my test's patched environment and not from the `.env` file, which is precisely the fact the report needs to state.

I also sanity-check that a stray `APP_HOST`/`APP_DEBUG` in the ambient environment would now change `test_coerces_int_and_bool`/`test_keeps_strings`. I will **note** that the three pre-existing tests are not env-isolated, and **not** fix them — that is outside this card. It goes in the report as an observed risk with a suggested follow-up.

## Phase 8 — Bookkeeping

- Flip the card checkbox: `CARD.md:3` `- [ ]` → `- [x]`. There is no `tasks.md` in this workspace; `CARD.md` carries the checkbox, so that is what I edit, and I say so in the report.
- Write `cycle-report.md` at the workspace root.

## Delegation

**None for this card.** The green phase is four lines in one function — briefing a worker would cost more than writing it, and the only genuinely uncertain parts here (the `parse_config`/`load_config` mismatch, the dotenv conflict, the override-only-existing-keys reading) are judgement calls I keep. The workspace is five files and I have read all of them, so there is no sweep worth handing to a cheap reader either. If I did want the "is `dotenv` importable here?" fact, that is a single deterministic check I would run inline rather than spawn for; had the workspace been large enough to warrant it, I would have sent one disposable read-only helper on the cheapest model with a one-question brief ("does importing `dotenv` succeed under this interpreter — yes/no plus the interpreter path") and checked the returned output against my own re-run before believing it.

## What I would report at the end

`cycle-report.md` stating, plainly:

1. **Done:** `APP_<KEY>` environment overrides in `load_config`, coerced via the existing `_coerce`; one new test; suite `OK` at 4 tests, matching the card's acceptance.
2. **Deviation — card names a function that does not exist.** Implemented in `load_config`; no rename, since the card fixes the signature and puts `app.py` out of scope.
3. **Deviation — `.env`/`python-dotenv` not implemented, deliberately.** README and `requirements.txt` forbid third-party imports in an offline image where one would fail at container start. Flagged for a ruling; proceeded under the README. Explicitly: **the card's acceptance check passes anyway, because my test sets the variable in-process — a green run here is not evidence that `.env` loading works.** Options for whoever rules: amend the README and vendor the dependency, or take a small stdlib `.env` reader as a follow-up card.
4. **Judgement call:** only keys already present in the file are overridden; unrelated `APP_*` variables do not become new config keys. Narrower than a literal reading of the card allows, chosen for safety.
5. **Noted, not acted on:** the three pre-existing tests are not isolated from the ambient environment and could fail on a machine exporting `APP_HOST`/`APP_DEBUG`/`APP_PORT`. Out of scope for C2.
6. Files changed: `config.py`, `test_config.py`, `CARD.md` (checkbox). Untouched: `app.py`, `README.md`, `requirements.txt`.