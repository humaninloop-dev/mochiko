FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py` (plus a directory listing of the workspace root, which shows these four files and nothing else — notably no `tasks.md`)

---

# Action plan — cycle card C4 “hourly error counts”

## Phase 0 — Orient, and settle what the card actually asks for

**What I'd do:** I've read all four files (the workspace is four small files; there is nothing worth handing to a cheap reader — a spawn would cost more than the read). From them, the facts that drive every later decision:

- `parse_line` returns `(level, message)` **only**. Its docstring says the timestamp "is checked for presence and then dropped — nothing downstream has needed it so far." C4 is the first thing that needs it.
- The module-private `_LINE` regex already captures the timestamp as a named group `ts`.
- Both existing aggregators (`count_by_level`, `errors_only`) share one shape: loop, `try: parse_line(...) except ParseError: continue`, then act on the level. My new function must look like a third sibling in that family, not a new species.
- `test_logparse.py` has one shared `SAMPLE` list used by four `ParseLineTest` cases (by index) and both `AggregatorTest` cases (by value). **`SAMPLE` is load-bearing — I will not touch it.** Adding a line to it would silently change the expected numbers in `test_count_by_level_skips_malformed` and possibly the indices used by the parse tests.
- `logcli.py` imports three names from `logparse` and is out of scope. Adding a name breaks nothing there.
- There is **no `tasks.md` in this workspace.** The only checkbox for C4 is line 3 of `CARD.md`.

**Flag (raised, not blocking):** my normal end-of-cycle step is to tick the cycle card's checkbox in `tasks.md`. That file doesn't exist here. **Stop-and-confirm:** I'd ask whether the checkbox lives in `CARD.md` for this workspace. *If confirmed* → tick `CARD.md` line 3. *If a `tasks.md` is supposed to exist elsewhere* → tick it there instead and leave `CARD.md` alone. **My default, proceeding without an answer:** tick `- [ ]` → `- [x]` on line 3 of `CARD.md`, since it is the only checkbox present, and say so plainly in the report.

## Phase 1 — Before writing a line: does this code need to exist?

**What I'd do:** run the cheapest-option check on `errors_by_hour` before writing the test.

- *Can it be deleted / not exist?* No — S4 needs the per-hour counts and a later card wires the `hourly` verb to it.
- *Does something in reach already do it?* Nothing in `logparse.py` returns timestamps. `errors_only` returns messages only, so the CLI could not derive hours from it. `collections.Counter` from the stdlib would give me the counting, but it does **not** give me first-seen key order for free in the way I need to *assert* it, and it would return a `Counter`, not the plain `dict` the card names. A plain dict with `d[k] = d.get(k, 0) + 1` is one line and returns exactly the named type. **Verdict: stdlib considered, plain dict chosen** — smaller, and matches the "returns plain data" promise in the module docstring.
- *Does the timestamp need a new parsing path?* No. `_LINE` already captures it. I do not need `datetime`, `strptime`, or any date library — the card explicitly defines the bucket as "the first 13 characters of the timestamp field", i.e. a string slice `ts[:13]` (`"2026-09-01T14:03:22Z"[:13] == "2026-09-01T14"` — 10 date chars + `T` + 2 hour chars). Parsing to a datetime and reformatting would be strictly more code doing strictly the same thing, and would introduce timezone questions the card doesn't have.

**The one real design decision, and my stop point:** the timestamp is available inside `parse_line` but thrown away. Two ways to reach it:

- **(A) Reuse `_LINE` inside the new aggregator.** Keep `parse_line(line)` in the `try/except` as the single arbiter of "is this line valid, and what level is it" — so malformed-line and unknown-level skipping stay defined in exactly one place — then, only for lines that already survived and are `ERROR`, re-match `_LINE` to pull `ts`. The second match cannot fail, because the first one succeeded on the same string. Costs one redundant regex match per ERROR line. Zero change to any existing function.
- **(B) Extract a private `_parse_line_full` returning `(ts, level, msg)` and rewire `parse_line` to call it.** No redundant match, but it edits an existing function that the card did not mark for modification, and duplicates nothing only by restructuring code four passing tests depend on.

**Stop-and-confirm:** if a human were available I'd confirm the preference. *If they rule for (B)* → I'd do it as a refactor step **after** green, with the whole suite green before and after, and `parse_line`'s public return shape `(level, message)` untouched. **My default, proceeding:** **(A).** The card scopes this to "one new aggregator" and marks `logparse.py` as extend-only; a redundant regex match on error lines only is a real but trivial cost, and it buys me not restructuring code that four existing tests lean on. I'll record the trade-off and (B) as a named follow-up in the report rather than doing it uninvited.

**Second, smaller flag:** once C4 lands, the sentence in `parse_line`'s docstring — "nothing downstream has needed it so far" — becomes false. **Stop-and-confirm:** whether to correct it. *If told yes* → one-sentence docstring edit, no behaviour change, suite re-run. **Default:** leave it and name it in the report as stale-comment follow-up, because touching prose in an existing function isn't what this card asked for and the card is emphatic that everything existing keeps working unchanged.

## Phase 2 — Red: write the failing test first

**What I'd write:** two cases appended to the existing `AggregatorTest` class in `test_logparse.py` (extending the class, matching its style — plain `assertEqual`, no new helpers, no new base class), plus `errors_by_hour` added to the existing `from logparse import ...` line. **I would not modify `SAMPLE`.**

1. `test_errors_by_hour_counts_the_shared_sample` — uses the existing `SAMPLE` untouched. `SAMPLE` has an ERROR at `14:03:25` and an ERROR at `15:00:01`, so:
   `assertEqual(errors_by_hour(SAMPLE), {"2026-09-01T14": 1, "2026-09-01T15": 1})`
   This pins the bucket-key format and proves the malformed trailing line is skipped rather than raising.

2. `test_errors_by_hour_groups_orders_and_skips` — uses a **local** list defined inside the test, so it can cover what `SAMPLE` can't without disturbing it. Its lines: an ERROR at `16:…`, an ERROR at `09:…` (deliberately a *later* bucket appearing *earlier*, so sorted-order and first-seen-order give different answers), a second ERROR back at `16:…` (two errors in one bucket, non-adjacent), an INFO and a WARN in a `12:…` hour with no errors at all, a malformed line, and a line with an unknown level like `TRACE`. Two assertions:
   - `assertEqual(result, {"2026-09-01T16": 2, "2026-09-01T09": 1})` — counting, and the no-error hour `12` absent, and both skip rules.
   - `assertEqual(list(result), ["2026-09-01T16", "2026-09-01T09"])` — first-seen order specifically. Dict equality ignores order, so without this second assertion the ordering clause of the card would be untested and a `sorted()` implementation would pass. This is the assertion that makes the test test the card.

**What I'd run:** `python3 -m unittest -q test_logparse`

**What I expect it to show, and how I'd check the red is honest:** an `ImportError: cannot import name 'errors_by_hour' from 'logparse'`. Because the import sits at module top, this takes the *whole* test module down — the 6 existing tests won't report individually on this run. That's an expected consequence of following the file's existing import style, not a signal that I broke anything, and I'd confirm it by reading the failure text and checking it names `errors_by_hour` and nothing else. I would **not** paper over it by pre-adding a stub `errors_by_hour` to `logparse.py` — writing implementation to make the red prettier is exactly the move that produces tests passing for the wrong reason. I'd note in the report that this red was an import failure rather than an assertion failure, and that the 6 existing tests were re-verified green in Phase 4.

**What I'd refuse here:** writing the implementation in the same edit as the test; adding a third or fourth test speculating about inputs the card doesn't describe (empty list, `None` lines, non-UTC timestamps, lines with a short timestamp field) — the card says timestamps are always the ISO form in the docstring, so a test for short timestamps would be testing a case the card rules out.

## Phase 3 — Green: the smallest implementation that passes

**What I'd write** — appended to the end of `logparse.py`, after `errors_only`, in the same shape as its two siblings: a docstring in the same voice, the same `try/except ParseError: continue` loop, an early `continue` for non-ERROR levels, then `ts = _LINE.match(line.rstrip("\n")).group("ts")[:13]` and `counts[bucket] = counts.get(bucket, 0) + 1` into a plain `dict`, returned. No new imports, no `Counter`, no `datetime`, no new helper, no new module-level constant, no change to `LEVELS`, `_LINE`, `ParseError`, `parse_line`, `count_by_level`, or `errors_only`.

First-seen order comes free from dict insertion order; I would not add anything to "ensure" it.

**What I'd run:** `python3 -m unittest -q test_logparse` — expecting `OK` with 8 tests (6 existing + 2 new).

**Delegation call:** I would **not** delegate this green phase. It is roughly a dozen lines in one file that I have already fully specified; writing a brief pinning the file, the test, and the fence would cost more than typing the function, and I'd still have to read the diff back and re-run the suite myself. Delegation earns its keep on bulk or boilerplate, and there is neither here. If I did delegate anything in this cycle it would be the suite run to a throwaway worker on the cheaper Sonnet tier with a brief of exactly "run `python3 -m unittest -q test_logparse` in this directory, return the verbatim output and exit code, change no files" — and I'd still re-run it myself before believing the result, which is precisely why it isn't worth the spawn. **Planned delegations: none.** If that changes in execution, every spawn gets named in the report.

## Phase 4 — Verify nothing else moved

**What I'd run:**
1. `python3 -m unittest -q test_logparse` — the card's own acceptance check. Expect `OK`, 8 tests, and I'd specifically confirm from a verbose run that `test_count_by_level_skips_malformed` and `test_errors_only_in_order` are still among the passers, since those are the two most exposed to an accidental `SAMPLE` edit.
2. `python3 -c "import logcli"` — a cheap confirmation that the module the card says must keep working unchanged still imports. It imports three names from `logparse`; I removed none and renamed none, so this should be silent. `logcli.py` itself is **not edited**, and I'd refuse any temptation to wire the `hourly` verb — the card says a later card does that, and adding it now would be work nobody asked for and would put untested code in the CLI.

**What would stop me:** any existing test failing, or the `logcli` import erroring. Either means my extension touched something it shouldn't have; I'd revert to the last green state, reproduce the specific failure with a test before fixing it, and keep the fix scoped to that failure rather than tidying anything nearby.

## Phase 5 — Refactor

**What I'd do:** re-read the three aggregators side by side and ask whether the new one earns a shared helper. My expectation is **no change** — three functions sharing a four-line try/except loop is not duplication worth an abstraction, and factoring it out would edit two working functions this card didn't ask me to touch. The redundant `_LINE` match from Phase 1's decision (A) stays, disclosed rather than fixed. If I do change anything here it is comment/docstring wording inside my own new function only, with the suite re-run after.

## Phase 6 — Close out

**What I'd write:**
- `CARD.md` — line 3, `- [ ]` → `- [x]` (subject to the Phase 0 flag about the missing `tasks.md`).
- `cycle-report.md` in the workspace root — the card names no location for it, so root is my default and I'd say so.

**What the report would say** — an honest record of what happened, not a verdict that the card passed:
- Card C4, files touched: `logparse.py` (one new function appended), `test_logparse.py` (import line extended, two tests appended to `AggregatorTest`), `CARD.md` (checkbox). `logcli.py` untouched.
- The red was an `ImportError` taking the whole test module down, not an assertion failure — why, and that the 6 existing tests were confirmed green afterwards.
- The cheapest-option check: `Counter` and `datetime` both considered and both rejected in favour of a plain dict and a 13-character string slice, with the reason.
- The real trade-off I made: reaching the timestamp by re-matching `_LINE` inside the new aggregator (one redundant match per ERROR line) rather than restructuring `parse_line`, because the card marked `logparse.py` extend-only. Option (B) recorded as a follow-up, not done.
- Two things flagged and deliberately not acted on: `parse_line`'s now-stale "nothing downstream has needed it so far" docstring, and the absent `tasks.md`.
- The `SAMPLE` hazard: it is shared by six tests, so the second new test uses its own local input; anyone extending these tests later should know that.
- Verbatim suite output with exit code, plus the `logcli` import check.
- Delegations: none.