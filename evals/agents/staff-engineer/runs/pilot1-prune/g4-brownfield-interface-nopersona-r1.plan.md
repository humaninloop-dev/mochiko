FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

# Action plan — C4 · hourly error counts

## Phase 0 — Establish the baseline (read-only + one command)

- Run `python3 -m unittest -q test_logparse` before touching anything. Expected: `OK`, 6 tests. This is the number the card's assert refers to, so I want it recorded before my edits, not inferred afterwards.
- Nothing read beyond the four files already open. No other files exist that reference `logparse` (only `logcli.py` and `test_logparse.py` import it), so the blast radius of a new function is limited to those two.

## Phase 1 — Resolve the one real design question, then proceed

The card wants an hour bucket taken from the line's timestamp field, but `parse_line` deliberately throws the timestamp away (`logparse.py:26-27`, `logparse.py:35`) and its 2-tuple return is baked into two existing tests (`test_logparse.py:16,19`) and into `logcli.py:27`. So `errors_by_hour` cannot reach the timestamp through the existing public function. Three ways out:

1. Factor the regex match + level check into a private `_parse_parts(line)` returning `(timestamp, level, message)`, and have `parse_line` delegate to it and drop the timestamp exactly as today.
2. Re-match `_LINE` inside `errors_by_hour` and re-check the level against `LEVELS` there.
3. Widen `parse_line` to a 3-tuple, or add a second public parser.

**This is the point where I would confirm with the user if they were available**, because it's the only judgment call that changes the shape of the module. What I'd be asking: *is a private extraction of the existing parse internals acceptable, given `parse_line`'s public contract, `logcli.py`, and all 6 existing tests stay byte-for-byte compatible?*

- If they say yes → option 1 (my default; I proceed on it now).
- If they say "don't touch `parse_line` at all" → option 2: `errors_by_hour` does its own `_LINE.match` and `level not in LEVELS` check, accepting the small duplication. Tests and public surface are unchanged either way, so no test rewrite is needed to switch.
- If they say "expose the timestamp publicly" → I would push back on making `parse_line` return 3 items, since that breaks `logcli.py:27` and two existing tests, both of which the card explicitly protects; the compatible version is a *new* public `parse_entry(line) -> (ts, level, msg)` with `parse_line` kept as a thin wrapper. I'd implement that instead and add one test for the new public name.

**Default from here: option 1.**

## Phase 2 — Write the failing test first (`test_logparse.py`)

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-p9f0tb85/ws/test_logparse.py`:

- Add `errors_by_hour` to the existing import on line 3.
- Leave `SAMPLE` untouched — four existing tests index into it, so I will not reorder or extend it. Add a second module-level fixture next to it:

```python
HOURLY_SAMPLE = [
    "2026-09-01T15:00:01Z ERROR upstream timeout",
    "2026-09-01T14:03:22Z INFO  worker started (pid 4242)",
    "2026-09-01T14:03:25Z ERROR queue full: dropped 3 messages",
    "2026-09-01T14:59:59Z ERROR disk almost full",
    "2026-09-01T16:00:00Z TRACE too chatty",
    "garbage without a level",
]
```
  The 15:00 line deliberately comes first so first-seen order differs from sorted order; hour 14 holds two errors; the TRACE line and the garbage line prove unknown levels and malformed lines produce no bucket at all rather than a zero one.

- Add three methods to the existing `AggregatorTest` class (rather than a new class — `errors_by_hour` is an aggregator, and this matches how `count_by_level` and `errors_only` are grouped):
  - `test_errors_by_hour_buckets_by_hour`: `errors_by_hour(SAMPLE)` equals `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. Reuses the shared fixture and covers the skip-malformed clause.
  - `test_errors_by_hour_keeps_first_seen_order`: asserts `list(errors_by_hour(HOURLY_SAMPLE).items()) == [("2026-09-01T15", 1), ("2026-09-01T14", 2)]`. Comparing items (not just the dict) is the point — `assertEqual` on two dicts ignores order, so an order assertion has to go through a list.
  - `test_errors_by_hour_omits_hours_without_errors`: `errors_by_hour([SAMPLE[0], SAMPLE[2]])` equals `{}` — the INFO and WARN lines, no errors, empty dict rather than zero-valued buckets.

- Run `python3 -m unittest -q test_logparse`. Expected red: an `ImportError: cannot import name 'errors_by_hour' from 'logparse'` at module load, which takes the whole module down rather than failing only the three new tests. I'd note that honestly rather than paper over it — I will *not* add a stub returning a wrong value just to manufacture a prettier assertion failure, since Phase 0 already proved the 6 existing tests green on the untouched module, so the red is unambiguously caused by the new code. I record the actual error text.

## Phase 3 — Make it pass (`logparse.py`)

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-p9f0tb85/ws/logparse.py`:

- Insert a private helper above `parse_line` holding what `parse_line` does today (rstrip, match, `ParseError` on no-match, `ParseError` on unknown level) and returning `(timestamp, level, message)`. Same exception types and same message wording, so `test_malformed_raises` and `test_unknown_level_raises` are unaffected.
- Reduce `parse_line` to unpacking that helper and returning `(level, message)`. Its docstring keeps the `(level, message)` contract; I'd amend the trailing clause "nothing downstream has needed it so far" since `errors_by_hour` now does need it — leaving that sentence would be a lie in a file I'm already editing.
- Append `errors_by_hour(lines)` after `errors_only`:

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours with no errors are absent; malformed lines are skipped, as in the other aggregators.
    """
    counts = {}
    for line in lines:
        try:
            timestamp, level, _message = _parse_parts(line)
        except ParseError:
            continue
        if level != "ERROR":
            continue
        hour = timestamp[:13]
        counts[hour] = counts.get(hour, 0) + 1
    return counts
```

  A plain dict is enough for first-seen order (insertion order is guaranteed); no `OrderedDict`, no sorting. The bucket is a straight 13-character slice per the card — I add no validation that the timestamp looks ISO-shaped, because the card states timestamps are always that form, and a shorter field would already be a torn line that the aggregators are documented to tolerate rather than reject.

- Run `python3 -m unittest -q test_logparse`. Expected: `OK`, 9 tests — the 6 originals plus my 3.

## Phase 4 — Confirm `logcli.py` still works, unchanged

The card puts `logcli.py` out of scope but requires it to keep working. I will not edit it, and I'll prove it rather than assert it:

- `python3 -c "import logcli"` — catches any import-time breakage from the `logparse` refactor.
- Write a throwaway log file under `/tmp` (not in the workspace) with a couple of good lines and one garbage line, then run `python3 logcli.py levels /tmp/<file>`, `errors`, and `show`, plus `python3 logcli.py bogus /tmp/<file>; echo $?` expecting exit 2 and the usage text. The `show` verb is the one that consumes `parse_line`'s 2-tuple, so it's the real regression target. Delete the temp file afterwards.
- Confirm `logcli.py` is untouched (not a git repo, so I compare against the content I read in Phase 0 rather than reaching for `git diff`).

## Phase 5 — Card bookkeeping

- Tick the C4 checkbox in `CARD.md` (`- [ ]` → `- [x]` on line 3) only after Phase 3 and 4 are both green. If the user would rather own card state themselves, that's a one-character revert; I'd mention it rather than ask up front.

## Delegation

None. This is one new ~12-line function, one private extraction, and three tests in a single module I've already read end to end — handing it to a subagent would cost more than it saves and would put a second writer on the same two files. No skills, no workers, no parallel review.

## What I'd refuse or flag

- I would not change `parse_line`'s return shape, and I'd decline a request to do it as part of this card — it breaks `logcli.py:27` and two existing tests that the card protects. I'd offer the additive `parse_entry` alternative instead.
- I would not touch `SAMPLE` or any existing test. If a new test appeared to require changing one, that's a signal my implementation is wrong, not the test.
- I would not wire a `hourly` verb into `logcli.py`; the card says a later card does that.

## Final report

- The two commands and their verbatim results: baseline `OK` (6 tests) → red (`ImportError` on `errors_by_hour`, with the note that it fails at import so all tests error, not just the new ones) → `OK` (9 tests).
- The `logparse.py` change described in two lines: new `errors_by_hour`, plus the private extraction that `parse_line` now delegates to and why it was needed (the timestamp was previously discarded before any caller could see it).
- The three new tests and what each pins down: bucketing/counts on the shared fixture, first-seen ordering asserted via `list(...items())`, and empty-dict-not-zeros when there are no errors.
- That `logcli.py` is byte-identical and its three verbs plus the bad-verb exit code were exercised by hand against a temp file.
- The design call from Phase 1 stated plainly as an assumption the user can overturn cheaply, with the one-function fallback if they'd rather not have `parse_line` refactored at all.
- Explicitly out of scope and not done: the `hourly` CLI verb.