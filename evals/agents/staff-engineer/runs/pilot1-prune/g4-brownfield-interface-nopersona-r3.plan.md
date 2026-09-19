FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — C4 · hourly error counts

## Phase 0 — What the workspace already tells me (no writes)

Already read all four files. The facts that drive the plan:

- `logparse.py:23-35` — `parse_line` matches `_LINE` (which *does* capture `ts`), validates the level against `LEVELS`, then **throws the timestamp away** and returns `(level, message)`. Its docstring even says "nothing downstream has needed it so far."
- That is the one real obstacle in this card: `errors_by_hour` needs the timestamp, and the only existing parse entry point discards it.
- `parse_line`'s 2-tuple return is depended on by `test_logparse.py:16,19` and by `logcli.py:27` (`level, message = parse_line(line)`). The card says logcli and all existing tests must keep working, so **the public shape of `parse_line` is frozen**.
- Both existing aggregators (`count_by_level`, `errors_only`) swallow `ParseError` and `continue`. The new one must match that shape — and note that "unknown level" is a `ParseError` too, so unknown levels are skipped by the same `except`.
- Dicts preserve insertion order in every Python 3 this runs on, so "first-seen order" falls out of plain insertion — but `assertEqual` on two dicts ignores order, so the test must pin order separately or it isn't actually testing the requirement.

## Phase 1 — Decision point: how the timestamp gets out of the parser

This is the one judgment call worth naming. Three options:

- **(A) Extract a private `_fields(line) -> (ts, level, msg)`** helper holding today's match + validation; `parse_line` becomes a thin wrapper over it; `errors_by_hour` calls `_fields`. Single source of parsing/validation truth, public API untouched.
- **(B) Change `parse_line` to return 3 items.** Breaks two existing tests and `logcli.py:27`. The card forbids both. Rejected outright — I would not do this even if it looks tidier.
- **(C) Re-run `_LINE.match` inside `errors_by_hour`.** No touch to `parse_line` at all, but duplicates the malformed/unknown-level rules, so the two paths can drift.

**Where I'd stop for a human:** if the reviewer's convention is that a card marked `[EXTEND]` must add code without editing existing function bodies, then A is out of bounds and C is the answer. I'd confirm: *"C4 needs the timestamp that `parse_line` currently drops — may I refactor `parse_line` onto a private `_fields` helper (behavior identical, signature identical), or must the new aggregator do its own regex match?"*
- If **A approved** (or no answer): proceed as below.
- If **C mandated**: `errors_by_hour` does its own `_LINE.match(line.rstrip("\n"))`, checks `match is None` and `level not in LEVELS` inline and `continue`s on either, with a comment noting the rules are mirrored from `parse_line`. All tests in Phase 2 stay byte-identical — they only touch the public function — so nothing else in the plan changes.

**Default I proceed under: A.**

## Phase 2 — Write the failing test first (`test_logparse.py`)

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-2mdu6wbt/ws/test_logparse.py`:

1. Add `errors_by_hour` to the existing import on line 3.
2. Leave `SAMPLE` (lines 5-11) **untouched** — four existing tests index into it positionally (`SAMPLE[0]`, `[1]`, `[4]`), so appending or reordering there is a needless way to break green tests. Add a second fixture below it instead:

```python
HOURLY = [
    "2026-09-01T15:00:01Z ERROR upstream timeout",
    "2026-09-01T14:03:22Z INFO  worker started (pid 4242)",
    "2026-09-01T14:03:25Z ERROR queue full: dropped 3 messages",
    "2026-09-01T15:41:10Z ERROR upstream timeout",
    "2026-09-01T16:00:00Z WARN  retrying upstream",
    "2026-09-01T17:00:00Z TRACE too chatty",
    "garbage without a level",
]
```

This fixture is built to make each clause of the card fail loudly if unimplemented: hour 15 is seen *before* hour 14 (so first-seen order ≠ sorted order), hour 15 repeats non-adjacently (so counts must accumulate across a gap), hour 16 has a non-ERROR line only, hour 17 has an unknown level, and the last line is malformed.

3. Add to `AggregatorTest`:

```python
    def test_errors_by_hour_counts_per_bucket(self):
        self.assertEqual(errors_by_hour(HOURLY), {"2026-09-01T15": 2, "2026-09-01T14": 1})

    def test_errors_by_hour_keeps_first_seen_order(self):
        self.assertEqual(list(errors_by_hour(HOURLY)), ["2026-09-01T15", "2026-09-01T14"])

    def test_errors_by_hour_omits_quiet_and_bad_lines(self):
        buckets = errors_by_hour(HOURLY)
        self.assertNotIn("2026-09-01T16", buckets)   # WARN only
        self.assertNotIn("2026-09-01T17", buckets)   # unknown level

    def test_errors_by_hour_of_nothing_is_empty(self):
        self.assertEqual(errors_by_hour([]), {})
```

The order test is separate on purpose: the dict-equality assertion above it would pass under sorted or arbitrary order, so it does not cover the "first-seen" requirement on its own.

**Run:** `python3 -m unittest -q test_logparse`
**Expect:** not 4 clean failures but a *collection* error — `ImportError: cannot import name 'errors_by_hour' from 'logparse'`, reported as `1 error` / `FAILED (errors=1)`, with the 6 existing tests not run. That is the correct red for adding a brand-new function; I'd note it rather than pretend I got granular per-test failures.

## Phase 3 — Make it pass (`logparse.py`)

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-2mdu6wbt/ws/logparse.py`:

1. Insert `_fields` above `parse_line`, holding the body that is there today:

```python
def _fields(line):
    """Return `(timestamp, level, message)` for one log line, or raise `ParseError`.

    Shared by `parse_line` and the aggregators that need the timestamp.
    """
    match = _LINE.match(line.rstrip("\n"))
    if match is None:
        raise ParseError(f"malformed line: {line!r}")
    level = match.group("level")
    if level not in LEVELS:
        raise ParseError(f"unknown level {level!r} in line: {line!r}")
    return match.group("ts"), level, match.group("msg")
```

2. Shrink `parse_line` to `_timestamp, level, message = _fields(line); return level, message`. Signature, return value, and both `ParseError` messages are byte-identical to today's, so `logcli.py`'s `show` verb (which prints `exc`) is unaffected.

3. Fix the now-stale sentence in `parse_line`'s docstring (line 27): "nothing downstream has needed it so far" stops being true the moment this card lands. Reword to say the timestamp is validated and dropped, and point at `_fields` for callers that want it.

4. Append the aggregator after `errors_only`:

```python
def errors_by_hour(lines):
    """Map each hour bucket to how many ERROR lines fell in it.

    The bucket key is the `YYYY-MM-DDTHH` prefix of the timestamp. Buckets come out in
    first-seen order and hours with no errors are absent. Malformed lines and unknown
    levels are skipped, as in the other aggregators.
    """
    counts = {}
    for line in lines:
        try:
            timestamp, level, _message = _fields(line)
        except ParseError:
            continue
        if level != "ERROR":
            continue
        bucket = timestamp[:13]
        counts[bucket] = counts.get(bucket, 0) + 1
    return counts
```

Deliberately *not* doing: no `datetime` parsing, no timestamp-shape validation, no `collections.Counter` (it would sort or at least invite `.most_common`, and plain insertion already gives the required order), no `defaultdict` (it would leak a defaulting dict to callers). The card fixes the key as a 13-character slice and states timestamps are always the ISO form, so slicing is the specified behavior, not a shortcut.

**Run:** `python3 -m unittest -q test_logparse`
**Expect:** `OK` over 10 tests — the 6 that existed plus the 4 new ones.

## Phase 4 — Verify the blast radius the card calls out

- Confirm the 6 pre-existing tests are in the pass count and none were edited: `git` is unavailable here (not a repo), so I'd re-read `test_logparse.py:14-39` and check those method bodies are exactly as read in Phase 0.
- Smoke `logcli.py`, which the card requires to keep working untouched: write a throwaway log file under `/tmp` (**not** in the workspace — I would not leave stray fixtures next to the deliverables), then run `python3 logcli.py levels /tmp/…`, `errors`, and `show` against it. Expect the same output shape as before: `INFO  1` style rows, the raw error messages, and for `show` a `?     malformed line: '…'` row for the garbage line. This is the check that the `parse_line` refactor didn't move anything visible.
- Confirm `logcli.py` itself has zero diff — no `hourly` verb, no new import. A later card wires the verb; adding it now would be quietly widening the card.

## Phase 5 — Report

I'd report: the two files changed and what changed in each; that `parse_line` was refactored onto `_fields` rather than having its return value changed, and why (`logcli.py:27` and two existing tests depend on the 2-tuple); the exact red-then-green test output, including that the initial red was an import error rather than assertion failures; the `logcli` smoke results; and one flag for whoever writes the follow-up wiring card — `errors_by_hour` trusts the timestamp field's shape per this card's wording, so a line whose first field is well-formed-but-short (e.g. `short ERROR boom`) yields a stubby bucket key instead of being rejected. That is the specified behavior, not a defect, but it is worth a sentence when the `hourly` verb starts showing these keys to an on-call engineer.

**No delegation.** This is two small edits in one module plus its test file; handing it to a subagent would cost more context than doing it.