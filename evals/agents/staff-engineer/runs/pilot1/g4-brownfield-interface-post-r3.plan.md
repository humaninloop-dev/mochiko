FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4 (`errors_by_hour`)

## Phase 0 — Read and orient (done above)

Read all four workspace files in full before writing anything, because this card touches existing code (`[EXTEND]` on both `logparse.py` and `test_logparse.py`) and names `logcli.py` as a thing that must keep working.

What the read established:

- `logparse.py` has one regex `_LINE` with named groups `ts`, `level`, `msg`; `parse_line` validates shape + level membership in `LEVELS`, **drops the timestamp**, and returns `(level, message)`.
- Both existing aggregators (`count_by_level`, `errors_only`) share one shape: iterate, `try: parse_line(line) except ParseError: continue`, accumulate. That is the house pattern my new function must match.
- `test_logparse.py` has exactly **6** tests across two classes and one module-level `SAMPLE` fixture whose contents are asserted on by two of them.
- `logcli.py` imports `ParseError, count_by_level, errors_only, parse_line` — none of which I may change the signature of.

**The one real design constraint I found:** `parse_line` deliberately discards the timestamp ("checked for presence and then dropped — nothing downstream has needed it so far"), but C4 needs it. Phase 1 resolves this without modifying `parse_line`.

## Phase 1 — Pre-code check: is new code needed, and what is the cheapest version?

Before writing a test, decide what the smallest thing that could work is.

- **Delete / already exists?** No hour-bucketing exists anywhere in the three files. Nothing to reuse or delete. New code is genuinely required.
- **Stdlib / platform?** `collections.Counter` would count, but I'd still need the bucketing loop and Counter buys nothing over `dict.get(k, 0) + 1`, while adding an import and a non-`dict` return type. `datetime.fromisoformat` would parse the timestamp, but the card defines the bucket as *the first 13 characters of the timestamp field* — a string slice — so parsing to a datetime and reformatting would be strictly more machinery for the same answer, plus it would start rejecting timestamps the existing parser accepts. Declined both; plain `dict` + slice.
- **Ordering.** "First-seen order" needs no `OrderedDict` — a plain `dict` preserves insertion order. No new type.
- **Getting the timestamp — the decision point.** Three candidates:
  1. Change `parse_line` to also return `ts`. **Refused.** The card grants `[EXTEND]`, not `[MODIFY]`; it would break `logcli.py`'s three call sites and two existing tests. Not doing this.
  2. Add a private `_fields()` helper returning `(ts, level, msg)` and have `parse_line` delegate to it. Legitimate, but it edits existing working code for a function that has exactly one new caller.
  3. **Chosen (default):** call `parse_line` for validation exactly as the other two aggregators do, and take the timestamp from the line itself — `line.split()[0][:13]`. This is safe precisely *because* `parse_line` already succeeded: `_LINE` anchors `ts` as `^\S+`, so the first whitespace-separated token is byte-identical to the `ts` group. Zero existing code modified, skip-semantics inherited rather than duplicated, house pattern followed exactly.

If, mid-implementation, option 3 turns out not to hold (e.g. some line form where `split()[0]` and the `ts` group diverge), I fall back to option 2 — a private helper with `parse_line` delegating, public behaviour unchanged — and disclose the switch in the report. I would **not** silently fall back to option 1.

## Phase 2 — Baseline run (before touching anything)

Run `python3 -m unittest -q test_logparse` and record the verbatim output.

- **Expect:** `OK`, 6 tests.
- **Why first:** once my new test adds `errors_by_hour` to the import line, an `ImportError` will stop the whole module from loading, so the existing 6 cannot be observed during the red phase. I want the "6 green before I started" fact on the record now.
- **Stop condition:** if the baseline is not a clean `OK`, or the count isn't 6, I stop and report a pre-existing broken suite rather than building on it — the card's assert clause explicitly banks on 6 passing tests.

I would run this myself rather than hand it to a worker; a 6-test unittest run costs less to execute than to brief.

## Phase 3 — Red: write the failing test(s)

Edit **`test_logparse.py`** only.

1. Add `errors_by_hour` to the existing `from logparse import ...` line (keeping alphabetical order: `ParseError, count_by_level, errors_by_hour, errors_only, parse_line`).
2. Leave `SAMPLE` **untouched** — two existing tests assert exact results over it; adding a line there would break them.
3. Add a new module-level fixture for the ordering case, and a new `class ErrorsByHourTest(unittest.TestCase)` after `AggregatorTest`, matching the existing plain-`assertEqual` style (no new imports, no helpers, no parametrisation).

Three focused tests:

- **`test_counts_errors_per_hour`** — over the existing `SAMPLE`: expects `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. This one test simultaneously covers malformed-line skipping (`"garbage without a level"`), non-ERROR levels being ignored (the INFO and WARN lines), and "hours with no errors absent" (no key for any other hour).
- **`test_buckets_in_first_seen_order`** — over a new fixture ordered so first-seen order differs from sorted order, e.g. an ERROR at `T15`, then one at `T14`, then a second at `T15`. Asserts **both** `list(result) == ["2026-09-01T15", "2026-09-01T14"]` (order — dict equality alone would not catch a sorted implementation) and `result == {"2026-09-01T15": 2, "2026-09-01T14": 1}` (the repeat-hour increment).
- **`test_skips_unknown_level`** — a `TRACE` line alongside one real ERROR in the same hour; expects the TRACE contributes nothing. `SAMPLE` has no unknown-level line, so this rule is otherwise untested for the new function.

All three assert on returned data, not on how it's computed — nothing references `_LINE`, `parse_line` call counts, or internals.

Run `python3 -m unittest -q test_logparse`.

- **Expect:** failure, and specifically `ImportError: cannot import name 'errors_by_hour' from 'logparse'`. I check the message actually names `errors_by_hour` — that is the right reason to be red (the function does not exist yet). If it fails for any other reason (a typo in my fixture, a syntax error), I fix the test until the failure is the one I intended, before writing a line of implementation.
- I would **not** stub `errors_by_hour` to convert this into an assertion-level failure; that's implementation written before red.

## Phase 4 — Green: minimal implementation

Edit **`logparse.py`** only. Append after `errors_only` (no import changes, no change to `LEVELS`, `_LINE`, `ParseError`, `parse_line`, or either existing aggregator):

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours without errors are absent; malformed lines are skipped, as in the other aggregators.
    """
    counts = {}
    for line in lines:
        try:
            level, _message = parse_line(line)
        except ParseError:
            continue
        if level != "ERROR":
            continue
        hour = line.split()[0][:13]
        counts[hour] = counts.get(hour, 0) + 1
    return counts
```

No defensive handling of short or non-ISO timestamps: the card states timestamps are always the docstring's ISO form, and inventing validation the card didn't ask for is scope creep. I note the assumption in the report instead of coding around it.

Run `python3 -m unittest -q test_logparse`.

- **Expect:** `OK`, **9** tests — my 3 plus the original 6, all green. If any of the original 6 has moved, I stop: it would mean I changed something I claimed not to.

I would write these ~10 lines myself rather than brief a worker — describing the fence would cost more than the edit, and the read-back would cost more again.

## Phase 5 — Refactor

Reread the new function against the two beside it. Expected outcome: **no refactor needed** — it already mirrors `errors_only`'s loop shape, adds no import, and duplicates no validation logic. If review shows the `line.split()[0]` step reading as opaque, the only change I'd make is a clarifying inline comment (it's safe because `parse_line` already matched `^\S+`), not a restructure.

Things I will notice and deliberately **not** do, recording them as observations only:
- `count_by_level` and `errors_only` could share a "parsed entries" generator — a tempting tidy-up, out of scope, would touch code this card didn't open.
- `parse_line` could expose `ts` for the later `hourly` verb — that's a `[MODIFY]` and belongs to whoever owns that card.

Rerun the suite after any refactor touch; expect `OK`, 9 tests.

## Phase 6 — Verify the card's out-of-scope guarantees

- Confirm `logcli.py` is byte-unchanged and its four imports still resolve. I do **not** add an `hourly` verb — the card explicitly assigns that to a later card, and adding it would be exactly the "while I'm in here" move I refuse.
- Confirm no `SAMPLE` line was altered.
- Sanity-check the CLI still runs (`python3 logcli.py` with no args → usage text, exit 2), since my change is import-visible to it.

## Phase 7 — Mark the card done

**Discrepancy I would flag, with my default:** my instructions say to tick the card in `tasks.md`, but this workspace has no `tasks.md` — the checkbox lives at line 3 of `CARD.md`. This is a mismatch between the expected layout and the actual workspace, not a blocker. I would not stop for a human on it; if a human were available the question would be *"tick the box in `CARD.md`, or create a `tasks.md`?"* — if the ruling were "create `tasks.md`", I'd create it mirroring the card line; my stated default is the other branch: **flip `- [ ]` to `- [x]` in `CARD.md` line 3, create nothing, and record the deviation in the report.**

## Phase 8 — Write `cycle-report.md`

Write **`cycle-report.md`** at the workspace root. Honest record, including:

- Baseline: 6 tests `OK` (verbatim output). Final: 9 tests `OK` (verbatim output). Red-phase output quoted, showing the `ImportError` naming `errors_by_hour`.
- **The interesting difficulty of this cycle:** `parse_line` deliberately discards the timestamp that C4 needs. I did not modify it — I state the three options, why option 1 was refused as an ungranted `[MODIFY]` that would break `logcli.py` and two tests, why option 2 was declined as unnecessary edits to working code for a single caller, and why option 3 (reuse `parse_line` for validation, read `ts` off the line) is sound — the regex anchors `ts` as the first non-space token, so `split()[0]` is exactly the field `parse_line` validated.
- Cheapest-option disclosure: plain `dict` over `collections.Counter`; string slice over `datetime` parsing; insertion order over `OrderedDict`. No new imports, no new dependencies.
- Assumption recorded, not coded around: timestamps are always the ISO form; malformed/short timestamps are out of scope for this card.
- Scope held: `logcli.py` untouched, no `hourly` verb added, `SAMPLE` untouched, no opportunistic refactor of the existing aggregators — with the two noted opportunities listed as observations for later cards.
- **Delegation: none.** Every phase here was small enough that briefing and reading back a worker would have cost more than doing it. Nothing to disclose beyond the fact that I considered it and declined.
- Deviation: no `tasks.md` in this workspace; the checkbox was flipped in `CARD.md` instead.

## Files I would write

- `test_logparse.py` (extended — import line, one new fixture, one new test class)
- `logparse.py` (extended — one new function appended)
- `CARD.md` (checkbox flipped)
- `cycle-report.md` (new)

Explicitly **not** written: `logcli.py`.