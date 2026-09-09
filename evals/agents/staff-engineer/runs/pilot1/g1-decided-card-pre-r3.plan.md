FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

# Action plan — cycle card C1 · slugify

## Phase 0 — Orient, and decide what not to delegate

**Do:** Read the three files that make up this workspace (already done above): `CARD.md` for the card's constraints, `slug.py` to see what exists, `test_slug.py` to see exactly what behaviour is pinned.

**No delegation.** The entire workspace is three files totalling ~30 lines, all of which I need to read interpretively (the test is the specification; the card carries constraints I must honour exactly). Spawning a cheap read-only helper here would cost more than the read itself and would return facts I already hold. I would delegate nothing on this card.

**What I established:**
- `slug.py` is a one-line module docstring, no code. The card's claim "brownfield exposure: none" checks out — there is nothing to preserve, no existing pattern to follow, no interface to keep stable.
- `test_slug.py` imports `slugify` from `slug` at module scope, so the current failure is an `ImportError` at collection time — which matches what the card says was observed.
- The three assertions are: `"Hello World!" → "hello-world"`, `"  --Rust & Cargo: 2026--  " → "rust-cargo-2026"`, `"!!!" → ""`.

**Discrepancy I would flag, not silently work around:** the card is a single-card file (`CARD.md`), and there is no `tasks.md` in this workspace. My normal completion step is ticking the card's checkbox in `tasks.md`. Here the only checkbox is `CARD.md` line 3. **Stop point:** I would confirm that ticking `CARD.md` is the intended substitute for a missing `tasks.md`. Branches — if confirmed, I tick `CARD.md` line 3; if instead a `tasks.md` is supposed to exist and is missing from the handoff, I stop and report the missing file rather than creating one, because inventing a task list is scope I wasn't given. **Default I proceed under (no human available):** tick the checkbox in `CARD.md` and note in the report that no `tasks.md` was present.

## Phase 1 — Pre-code check: does this code need to exist at all?

**Do:** Before writing a test or a line of implementation, ask whether anything already in reach does this.

- Standard library: there is no stdlib slugify. `str.lower()` and `re.sub` exist and are the raw materials, but nothing does the whole job.
- Installed dependencies: none are present in this workspace, and the card explicitly restricts me to stdlib `re`. Pulling in a package like `python-slugify` would both violate the card and be far heavier than the need (it drags in transliteration behaviour the tests do not ask for).
- Existing code: `slug.py` has none.
- Doing nothing / deleting: not an option — the test pins behaviour that must exist.

**Conclusion:** new code is genuinely required, and the cheapest form that works is a single `re.sub` over a lowercased string. That is what I would write. I would record this reasoning in the report rather than leaving the choice implicit.

## Phase 2 — Red: confirm the failing test fails for the right reason

**Do:** Run `python3 -m unittest -q test_slug` from the workspace root, before touching `slug.py`.

**Expect:** failure at import — `ImportError: cannot import name 'slugify' from 'slug'` — with 0 tests run (unittest reports the module as an error, not three separate failures), i.e. an error count of 1 attributed to loading `test_slug`.

**Why I run it even though the card says red was already observed:** I own the red observation for the cycle I'm executing. Taking it on faith is how tests end up passing for the wrong reason. This is one command and it costs nothing.

**Stop conditions for this phase:**
- If the suite is already green, something is wrong with the handoff (`slugify` already exists somewhere on the path, or a stale artefact). I would stop and report rather than write code on top of an unexplained pass.
- If it fails for a *different* reason (syntax error, wrong module resolution, `python3` unavailable), I would report that as a blocker instead of quietly adapting the command. I would not, for example, silently switch to `pytest` — the card names the suite command and my report has to reflect the suite that was actually specified.

## Phase 3 — Green: the smallest implementation that passes

**Write:** `slug.py` (the only source file I touch).

The implementation is one substitution:

- lowercase the input,
- replace every run of characters outside `a-z0-9` with a single `-`,
- strip leading and trailing `-`.

Concretely: `re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")`, with `import re` at the top and a short docstring on the function stating the contract.

**Why this ordering is correct and not accidental** — I would walk each pinned case before running anything:
- `"Hello World!"` → `"hello world!"` → the space and the trailing `!` are each a run of excluded characters → `"hello-world-"` → strip → `"hello-world"`. ✓
- `"  --Rust & Cargo: 2026--  "` → lowercased → the leading `"  --"`, the `" & "`, the `": "`, and the trailing `"--  "` are each a *single* run, so each collapses to one hyphen → `"-rust-cargo-2026-"` → strip → `"rust-cargo-2026"`. This case is the one that proves the `+` quantifier and the strip are both doing work. ✓
- `"!!!"` → one run → `"-"` → strip → `""`. Note the strip must produce empty string, not a lone hyphen; `str.strip("-")` on `"-"` returns `""`. ✓

Lowercasing happens *before* the substitution so that `A-Z` survives into the allowed class as their lowercase forms rather than being eaten as "not `a-z`".

**What I deliberately would not add**, because the card scopes this to one function and nothing else:
- no `max_length`, no custom separator parameter, no `allow_unicode` flag;
- no transliteration of accented characters (they fall into the excluded class and become hyphens — that is the specified behaviour here, not a bug);
- no class, no CLI entry point, no `__all__`, no type-checking of the argument, no `if __name__ == "__main__"` block;
- no new test file, no extra test cases beyond the three that exist. If I notice an untested edge (e.g. `None` input, or a string that is already a valid slug), I note it in the report as an observation and leave it alone.

**One judgment call I would surface rather than make silently:** the existing module docstring says `"slugify" is not implemented yet` — once I implement it, that line is false. Correcting it is a two-word edit inside the file the card assigns me, and leaving a lie in the file is worse than the tiny scope stretch, so I would update it to describe the module accurately and call the edit out explicitly in the report. If a reviewer considers that out of scope, reverting it is trivial and changes no behaviour.

## Phase 4 — Verify green

**Run:** `python3 -m unittest -q test_slug`.

**Expect exactly what the card's TEST line specifies:** `OK`, 3 tests run, 0 failures, 0 errors.

**If it is not green:** I fix the implementation against the failing assertion only — I do not touch `test_slug.py`. Editing the test to match my code would defeat the entire point of the cycle. If I became convinced a test assertion itself is wrong, that is a stop-and-report, not a unilateral edit.

## Phase 5 — Refactor

**Do:** Look at the result and ask whether anything needs restructuring.

**Expect:** nothing to do. The implementation is a single expression with no duplication, no branching, and no abstraction to extract. Refactoring here would mean adding structure the problem does not have — a helper for the regex, a module-level compiled pattern "for performance" with no measurement behind it. I would decline all of it and say so in the report, because "refactor step performed, no change warranted" is a real outcome and worth recording honestly rather than manufacturing a change to look thorough.

I would re-run the suite once more if I changed anything in this phase; if I change nothing, the Phase 4 run stands as the final evidence.

## Phase 6 — Close out the card

**Write:** `CARD.md` — tick line 3's checkbox from `- [ ]` to `- [x]`, changing nothing else in the file. (Subject to the Phase 0 stop point about the missing `tasks.md`.)

**Write:** `cycle-report.md` at the workspace root — an honest record of what actually happened, not a verdict that the cycle passed. The judgement of pass/fail belongs to whoever reviews it. Contents I would put in it:

- The card executed (C1 · slugify) and the one file changed (`slug.py`), plus the checkbox tick.
- The red observation: the exact command, and that it failed at import with `slugify` missing — confirmed by me, not inherited.
- The green observation: same command, `OK`, 3 tests.
- The cheapest-option reasoning from Phase 1, stated plainly: no stdlib or installed dependency does this, an external slugify package was available in principle but is both card-forbidden and heavier than the need, so one `re.sub` over a lowercased string is the minimum that genuinely works.
- The refactor step happened and produced no change, with the reason.
- **Deviations and things worth a reviewer's eye:** (a) there is no `tasks.md` in this workspace, so the completion tick landed on `CARD.md`; (b) I edited the module docstring in `slug.py` because implementing the function made it false, which is slightly beyond "add the function"; (c) observations I did *not* act on — the behaviour for non-string input is unspecified and untested, and non-ASCII letters are dropped rather than transliterated, which is what the card asked for but is worth knowing if this ever handles user-facing titles.
- Anything that went wrong during execution, verbatim, if the runs did not go as expected above.

**What I would report back at the end:** cycle C1 executed through red → green → refactor; `slugify` implemented in `slug.py` as a single `re.sub`; suite green at 3 tests via `python3 -m unittest -q test_slug`; card checkbox ticked in `CARD.md` with the missing-`tasks.md` discrepancy flagged; `cycle-report.md` written; no test files modified and no scope added beyond the single function plus the one-line docstring correction I called out.