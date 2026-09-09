FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-423lz0t1/ws/CARD.md`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-423lz0t1/ws/logparse.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-423lz0t1/ws/test_logparse.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-423lz0t1/ws/logcli.py`

---

# Action plan — cycle card C4, `errors_by_hour`

## Phase 0 — Orientation (already done above)

Four files, all small; I read every one of them end to end before planning. No delegation to a cheap reader here, and I want to say why rather than invent one: the entire relevant surface is ~150 lines across four files that I have already loaded. There is no locate, no enumeration, and no bounded quote left to farm out — a subagent spawn would cost more than it saves and would return facts I already hold. If the workspace had been a large repo I would have dispatched a `haiku`-tier `Explore` to answer "which files import from `logparse`, and does any name `errors_by_hour` already exist" — here I answered that by reading all four files directly.

**The one thing worth flagging before any code:** `parse_line` *deliberately discards the timestamp* (`logparse.py:26–27`: "checked for presence and then dropped — nothing downstream has needed it so far"). C4 is the first thing that needs it. So the card's "as the existing aggregators do" cannot be satisfied by a plain copy of the `count_by_level` / `errors_only` shape — those get everything they need from `parse_line`, and this one does not. See Phase 3 for how I resolve that without touching `parse_line`.

## Phase 1 — Baseline, before writing anything

- Run `python3 -m unittest -q test_logparse`.
- Expect: `OK`, 6 tests. This is the number the card's assert refers to, and I want it observed rather than assumed — if the suite is not already green, the card's premise is wrong and I stop and report that instead of building on a broken base.
- Also run `python3 -c "import logcli"` to confirm the CLI module imports cleanly against the current `logparse`. This is my before-picture for "logcli must keep working unchanged."
- No files written in this phase.

## Phase 2 — Cheapest-thing-that-works check, before the test

Run through the options for getting an hour bucket, cheapest first:

1. **Delete / don't build** — no, the card is a genuine new capability for story S4.
2. **Reuse something already in the module** — yes. `_LINE` already captures the timestamp in a named group `ts` (`logparse.py:16`); it is simply thrown away by `parse_line`. Nothing new needs to be parsed, matched, or imported.
3. **Stdlib** — I will *not* reach for `datetime.fromisoformat` or `collections.Counter`/`OrderedDict`. The card defines the bucket as a literal string slice (`first 13 characters`), so parsing to a datetime and reformatting would be strictly more code with more failure modes. Plain dicts have preserved insertion order since Python 3.7, so "first-seen order" is free from the platform — no `OrderedDict`, no sort, no ordering bookkeeping.
4. **New dependency** — none. This is a stdlib-only, zero-import change.

Net: the new function is a loop, a slice, and a dict. `import re` and `_LINE` stay as they are; I add no module-level names other than `errors_by_hour` itself.

## Phase 3 — The design decision I would flag (and my default)

**The tension:** `errors_by_hour` needs the timestamp; the one function that already validates a line throws the timestamp away.

**What I refuse:** changing `parse_line` to return a 3-tuple `(ts, level, message)`. The card marks `logparse.py` as `[EXTEND]`, not `[MODIFY]`, and that return shape is a live public contract with three consumers I can point at — `logcli.py:27`, `test_logparse.py:16`, `test_logparse.py:19`. Widening it would break the CLI the card explicitly says must keep working. I also will not add a second public parsing entry point (`parse_line_with_timestamp` or similar) — the card asked for one new aggregator, and a new public function nobody called for is scope I did not earn.

**My default (what I would build):** `errors_by_hour` matches `_LINE` once itself, and uses that single match for both the skip decision and the bucket key:

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours with no errors are absent. Malformed lines and unknown levels are skipped.
    """
    counts = {}
    for line in lines:
        match = _LINE.match(line.rstrip("\n"))
        if match is None or match.group("level") != "ERROR":
            continue
        hour = match.group("ts")[:13]
        counts[hour] = counts.get(hour, 0) + 1
    return counts
```

Unknown levels fall out for free: `TRACE` is not `ERROR`, so it is skipped by the same test that skips `WARN`. Malformed lines fail the match and are skipped. Same `_LINE`, same `rstrip("\n")`, same tolerant never-raise posture as its two siblings.

**The alternative I weighed and rejected:** keep the sibling shape exactly — `try: level, _ = parse_line(line) except ParseError: continue` — and then re-run `_LINE.match(line)` a second time purely to recover the timestamp. It parses every line twice and forces the reader to convince themselves the second match cannot be `None`. I judged the single match clearer.

**Where a human would rule.** This is the point I would surface rather than bury: *should the timestamp become part of `parse_line`'s public return, so future aggregators stop re-matching?* I do not stop the cycle for it — my default needs no ruling and no interface change — but I would name it in the report as a decision the reviewer may want to take.
- If the ruling is **"leave `parse_line` alone"** (my default): ship as above; nothing changes.
- If the ruling is **"expose the timestamp"**: that is a `[MODIFY]` on a contract with three call sites, plus updates to two existing assertions and to `logcli.py` — a separate card, not a quiet rider on C4. I would decline to fold it in here and write it up as a follow-up.

Two smaller things I would flag but **not** act on: `parse_line`'s docstring line "nothing downstream has needed it so far" is now stale, and a short/odd timestamp would make `[:13]` yield a truncated key. The card states timestamps are always the ISO form, so I add no validation and no defensive branch — noted in the report, not coded.

## Phase 4 — Red: the failing test

Write to `test_logparse.py` (append a new class; **the existing `SAMPLE` list and all six existing tests are left byte-for-byte alone** — `SAMPLE` is shared, and editing it to suit my new test would be the classic way to break someone else's green test).

Add `errors_by_hour` to the existing `from logparse import ...` line, and a new class:

```python
class ErrorsByHourTest(unittest.TestCase):
    def test_counts_errors_in_each_hour(self):
        self.assertEqual(
            errors_by_hour(SAMPLE), {"2026-09-01T14": 1, "2026-09-01T15": 1}
        )

    def test_buckets_in_first_seen_not_sorted_order(self):
        lines = [
            "2026-09-01T15:00:01Z ERROR later hour first",
            "2026-09-01T14:03:25Z ERROR earlier hour second",
            "2026-09-01T15:30:00Z ERROR later hour again",
        ]
        self.assertEqual(list(errors_by_hour(lines)), ["2026-09-01T15", "2026-09-01T14"])
        self.assertEqual(errors_by_hour(lines), {"2026-09-01T15": 2, "2026-09-01T14": 1})

    def test_hours_without_errors_are_absent(self):
        lines = [
            "2026-09-01T14:00:00Z WARN  retrying upstream",
            "2026-09-01T15:00:01Z ERROR upstream timeout",
        ]
        self.assertEqual(errors_by_hour(lines), {"2026-09-01T15": 1})

    def test_skips_malformed_and_unknown_levels(self):
        lines = [
            "garbage without a level",
            "2026-09-01T14:03:22Z TRACE too chatty",
            "2026-09-01T14:09:00Z ERROR real one",
        ]
        self.assertEqual(errors_by_hour(lines), {"2026-09-01T14": 1})

    def test_no_lines_gives_empty_dict(self):
        self.assertEqual(errors_by_hour([]), {})
```

Why these five and not fewer: SAMPLE alone has one error per hour, in ascending order, so it cannot tell a correct implementation apart from one that sorts keys or that counts hours instead of errors. The second test is the one that would catch a `sorted()` slipping in, and it needs the explicit `list(...)` assertion because `assertEqual` on two dicts ignores order entirely. Every test asserts on returned behaviour — none of them reach into `_LINE` or otherwise pin the implementation.

Run `python3 -m unittest -q test_logparse`. **Expected red:** `ImportError: cannot import name 'errors_by_hour' from 'logparse'`. I want to be precise that this is a module-level import failure, so it takes the whole file down and the six existing tests do not report in this run — that is exactly why Phase 1's baseline exists, so I already know they were green. I confirm the failure names the missing symbol and is not a typo or a stale `.pyc` before moving on.

## Phase 5 — Green

Write `errors_by_hour` (Phase 3 body) into `logparse.py`, appended after `errors_only` so the file keeps its parse-then-aggregators order. No other line of `logparse.py` changes.

Run `python3 -m unittest -q test_logparse`. **Expected:** `OK`, **11 tests** — the 6 that existed plus my 5. If the count is not 11 I have clobbered something and I go find out what.

If anything is still red I fix only the thing the failure points at; a surprise here is information about my implementation, not an invitation to reshape the module.

## Phase 6 — Refactor + the untouched-neighbour check

- Refactor pass: I expect to keep the body as written — five lines, one branch, nothing to extract. If I change anything it is docstring wording only, and the suite must stay green after.
- Confirm `logcli.py` is unmodified and still healthy: `python3 -c "import logcli"`, plus an end-to-end smoke running `levels`, `errors`, and `show` against a small scratch log file written **outside** the workspace (under `/tmp`) so I leave no stray fixture behind. Expect identical output to the pre-change behaviour and exit 0. This is the card's "must keep working unchanged" made observable rather than assumed.
- Final confirming run of `python3 -m unittest -q test_logparse` → `OK`, 11 tests. This is the card's stated assert.

## Phase 7 — Bookkeeping and report

- **`tasks.md` does not exist in this workspace.** The card's own checkbox lives at `CARD.md:3`. I would tick that one — `- [x] **C4 · hourly error counts**` — leaving the rest of `CARD.md` untouched, and note in the report that I substituted the card file for the absent `tasks.md` rather than creating a `tasks.md` nobody asked for.
- Write `cycle-report.md` in the workspace root.

**Files written across the whole cycle:** `test_logparse.py` (append), `logparse.py` (append), `CARD.md` (one checkbox), `cycle-report.md` (new).

**What the report would say, honestly:**
- Baseline 6 green → red on missing symbol → green at 11.
- The real friction of this card: `parse_line` drops the timestamp, so the new aggregator could not be a copy of its siblings. I matched `_LINE` directly inside `errors_by_hour` instead of routing through `parse_line`, stated plainly as a deviation from the sibling pattern, with the reason (the alternative parses each line twice) and the rejected option (widening `parse_line`'s return, which would have broken `logcli.py:27` and two existing assertions and needs a `[MODIFY]` card of its own).
- Cheapest-that-works disclosure: reused the existing `_LINE` regex and relied on plain-dict insertion order for the first-seen requirement; no `datetime`, no `Counter`, no `OrderedDict`, no new imports or dependencies.
- Noted-not-acted-on: `parse_line`'s "nothing downstream has needed it so far" docstring is now stale; short/malformed timestamps would produce a truncated bucket key, left unguarded per the card's stated guarantee; `logcli.py` still has no `hourly` verb, which is the later card's job.
- No blockers, no assumptions made silently, no existing test or interface changed.