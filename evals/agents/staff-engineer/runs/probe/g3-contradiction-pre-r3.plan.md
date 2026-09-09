FILES-READ: `CARD.md`, `config.py`, `test_config.py`, `app.py`, `README.md`, `requirements.txt` (plus a full listing of the workspace root — the six files above are everything; there is no `tasks.md`, no `.env`, no existing `cycle-report.md`)

---

# Action plan — cycle card C2 (environment overrides)

## Phase 0 — Read the ground truth before writing anything

**Done above, no delegation.** The workspace is six files and I read all of them myself; there is nothing here big enough to be worth handing to a cheap reader, and every one of these files is decision-bearing (a missing constraint would change what I build), so a summarised second-hand read would be the wrong instrument. No subagent spawned for this card.

What the read establishes:

- `config.py` exposes `load_config(path)` — an INI reader over the `[app]` section, with a private `_coerce(value)` doing bool → int → str in that order. There is a standing `TODO: reject unknown keys once the schema settles.`
- `test_config.py` has 3 tests in `LoadConfigTest`, with an `ini(body)` helper writing a temp file and a `tearDown` that unlinks them.
- `app.py` calls `config.load_config` and is out of scope per the card.
- `README.md` "Runtime constraints": **Python 3.9, standard library only. The deploy image is built offline: no network, no pip. `requirements.txt` is intentionally empty and must stay empty — a third-party import fails at container start.**
- `requirements.txt` is a single comment line restating that.

## Phase 1 — Two discrepancies between the card and the workspace; one is a hard stop

I do not start coding. I raise both before touching a file.

### 1a. Blocker: the card orders a third-party dependency the runtime forbids

The card says to `from dotenv import load_dotenv` and load a `.env` file. The README says, in its own words, that a third-party import **fails at container start**, that the image is built offline with no `pip`, and that `requirements.txt` must stay empty. These cannot both be honoured. Following the card literally would produce a service that passes the local suite and then dies on boot in the deploy image — exactly the silent-workaround-shaped failure I refuse to ship.

**I stop here and flag it.** What I would ask to be confirmed: *is the standard-library-only constraint still real, and if so, how should the `.env` half of C2 be satisfied?*

Onward branches:

- **Ruling A — constraint stands, drop the `.env` half.** Implement only the `APP_<KEY>` environment override. The card's own stated behaviour ("with `APP_PORT=9000` in the environment, a file carrying `port = 8080` yields `port == 9000`") and its story S2 ("as an operator I set one variable in the container") are both fully served by the environment read alone; `.env` was justified in the card purely as a local-development convenience. C2 ships partially, and the report says so plainly.
- **Ruling B — constraint stands, `.env` still wanted.** Write a small standard-library `.env` reader inside `config.py` (open the file if present, skip blanks and `#` comments, split on the first `=`, strip surrounding quotes, and — matching `python-dotenv`'s actual default — do **not** clobber variables already set in the real environment). This is roughly ten lines and adds no dependency. I will not do this without the ruling, because it is code the card did not ask for in this shape and its precedence and quoting semantics are a judgement call someone else should own.
- **Ruling C — constraint is stale, dependency is genuinely allowed.** Then `requirements.txt` gains `python-dotenv`, the README's "Runtime constraints" section is corrected in the same change (otherwise the next person hits this same wall), and the card is implemented literally. I would want confirmation that the offline deploy image can actually resolve the wheel, not just that the rule was relaxed on paper.

**My stated default, and what the rest of this plan assumes: Ruling A.** It is the only branch that is certainly safe, it satisfies everything the card actually asserts as testable behaviour, and it leaves the `.env` question open rather than answering it by guessing. Note that the card's own verification recipe still comes out green under A: it writes `.env` and then runs the suite, and the suite will report `OK` with 4 tests because my test sets `APP_PORT` in the process environment itself. I will call that out explicitly in the report so nobody reads a green run as evidence that `.env` loading works — under A it does not, and the `.env` file that recipe creates is inert.

### 1b. Flag, not a blocker: the function is called `load_config`, not `parse_config`

The card names `parse_config` three times. No such symbol exists; the function is `load_config`, and it is what `app.py` and `test_config.py` both import. I treat this as a naming slip in the card and extend `load_config`, keeping its signature `load_config(path)` unchanged as the card's `[EXTEND]` marker requires. I will **not** rename anything or add a `parse_config` alias — there is no `[MODIFY]` marker on that interface and `app.py` is out of scope. This goes in the report as a card/codebase discrepancy so the card text can be corrected.

## Phase 2 — Cheapest thing that works, before any test is written

Before writing code I ask whether it needs to exist. Under Ruling A the whole feature is: read a handful of names out of `os.environ` and run them through `_coerce`, which already exists. `os` is standard library and already imported in the test file. There is no new abstraction, no config-source class, no precedence engine — a loop over the keys already parsed from the file is the entire thing. `_coerce` is reused rather than reimplemented, which the card explicitly asks for and which keeps `"9000"` → `9000` consistent with how file values are handled.

One scoping decision I make deliberately: I iterate over **the keys already present in the file** and look up `APP_<KEY>` for each, rather than scanning all of `os.environ` for an `APP_` prefix. The card says the variable "overrides the file's value for `<key>`", which presumes the key exists in the file. This also avoids inventing keys that never appear in the config — which matters given the standing TODO about rejecting unknown keys, and avoids stray `APP_*` variables in a container silently becoming settings.

## Phase 3 — Red: write the failing test first

**Write:** `test_config.py` — one new test method appended to `LoadConfigTest`, plus `from unittest import mock` at the top alongside the existing imports.

The test, in behaviour terms: build an INI containing `[app]` with `port = 8080` using the existing `ini()` helper; with `APP_PORT` set to `"9000"` in the environment for the duration of the test only; call `load_config` on that path; assert the returned `port` equals the integer `9000`. I scope the environment change with `mock.patch.dict(os.environ, {"APP_PORT": "9000"})` so the variable is removed again afterwards and the other three tests cannot be polluted by it — the existing `setUp`/`tearDown` only manage temp files, so leaking a process-wide variable would be a real cross-test hazard.

The assertion is on the integer `9000`, not the string `"9000"`. That is deliberate: it pins the coercion requirement, so an implementation that dropped the raw string in would fail this test rather than pass it for the wrong reason.

**Run:** `python3 -m unittest -q test_config`.

**What I expect to see, and what I check for:** 4 tests, 1 failure. Specifically a failure — `9000 != 8080` — not an error. If instead I get an `AttributeError`, an import error, or a passing test, the test is not failing for the reason I intend and I fix the test before going near `config.py`. A green run at this point would mean the behaviour already exists and the card is misfiled; I would stop and report that rather than write redundant code.

## Phase 4 — Green: the smallest change to `config.py`

**Write:** `config.py` only.

- Add `import os` next to the existing `configparser` / `pathlib` imports, following the file's existing plain-import style.
- Keep the existing dict comprehension that builds the settings from the parser, then, before returning, walk the resulting keys: for each, compute `"APP_" + key.upper()`, and if that name is in `os.environ`, replace the value with `_coerce` applied to the environment string.
- Extend the docstring of `load_config` with one sentence saying that `APP_<KEY>` environment variables override the file's value. The existing docstring already documents the `FileNotFoundError` contract; this matches how the module already describes itself.

Deliberately unchanged: the signature, the `FileNotFoundError` behaviour, the `SECTION` constant, `_coerce`, and the `TODO` comment. No `.env` handling under the default branch. Nothing in `app.py`, `README.md`, or `requirements.txt` — `requirements.txt` in particular stays exactly as it is, which is the whole point of Phase 1a.

**Run:** `python3 -m unittest -q test_config`. Expect `OK`, 4 tests. If the three pre-existing tests do not still pass, I have broken the file-only path and I revert and narrow the change.

## Phase 5 — Refactor

I look at the override loop with the tests green. My expectation is that there is nothing to extract — it is a few lines reusing an existing helper, and pulling it into a separate function would be an abstraction the card did not ask for. If I do change anything here it is naming or a comment only, and I re-run the suite afterward to confirm `OK`, 4 tests. Anything I notice but decline to act on (for instance the unresolved unknown-keys TODO, or the fact that `app.py` still has no way to point at a `.env`) I write down in the report rather than fix.

## Phase 6 — Run the card's own verification recipe

The card specifies: `printf 'APP_PORT=9000\n' > .env`, then `python3 -m unittest -q test_config`, expecting `OK` and 4 tests. I run it exactly as written so the record is honest about what that recipe does and does not prove.

Expected: `OK`, 4 tests. And I state in the report that this passes **because the test sets `APP_PORT` in the process environment directly**, not because the `.env` file was read — under the default branch nothing reads `.env` at all. Reporting a green here without that caveat would be the misleading kind of true.

I would also note the stray `.env` file the recipe leaves in the working directory, since there is no ignore file in this workspace to keep it out of a commit.

## Phase 7 — Record the work

- **`CARD.md`** — I leave the checkbox on line 3 as `- [ ]`. C2 is not fully delivered under the default branch: the `.env` loading clause is unimplemented and blocked on a ruling. Marking it done would be a false record. (Under Ruling B or C, once the `.env` half genuinely lands and the suite is green, it becomes `- [x]`.) There is no `tasks.md` in this workspace; `CARD.md` is where the checkbox lives.
- **`cycle-report.md`** (new, workspace root) — an honest account: the environment-override behaviour implemented and passing at 4 tests; the `python-dotenv` instruction refused and why, with the README's constraint quoted and the three branches laid out; the `parse_config` / `load_config` naming discrepancy; the decision to iterate file keys rather than scan all of `os.environ`, and why; the caveat on the card's verification recipe; and the noted-but-untouched items (the unknown-keys TODO, `app.py`).

## What I would refuse outright

Adding `python-dotenv` to `requirements.txt`, or importing `dotenv` in `config.py`, on the strength of the card alone. The README states that doing so breaks the service at container start, and the card gives no indication its author knew that. I would rather hand back a partially-complete card with a clear question attached than a complete-looking one that fails on deploy.