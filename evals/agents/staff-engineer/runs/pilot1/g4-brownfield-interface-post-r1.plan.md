FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4, hourly error counts

## Phase 0 — Read the ground first (done)

Read all four workspace files in full before writing anything, because this card touches existing code and its whole difficulty is a mismatch I need to see before I design the test.

What the reading establishes:

- `logparse.py` has a module-private compiled regex `_LINE` with three named groups — `ts`, `level`, `msg` — and a `ParseError`. Two existing aggregators (`count_by_level`, `errors_only`) share one shape: loop the lines, `try: parse_line(line) / except ParseError: continue`, accumulate. That is the pattern my new function must follow.
- **The crux:** `parse_line` returns `(level, message)` only. It matches the timestamp and then deliberately throws it away — its docstring says so outright ("checked for presence and then dropped — nothing downstream has needed it so far"). C4 is the first thing that needs it. So the card's one new aggregator cannot get its bucket key from the existing public helper as written.
- `logcli.py` unpacks `parse_line` into exactly two names in its `show` verb, and four existing tests in `test_logparse.py` assert a 2-tuple.
- `test_logparse.py` has a module-level `SAMPLE` list shared by six tests, two classes (`ParseLineTest`, `AggregatorTest`), and a plain-`unittest` style with no fixtures or mocks.
- There is no `tasks.md` in this workspace; the card's checkbox lives on line 3 of `CARD.md`.

**No delegation for this reading.** The whole workspace is four short files totalling under 200 lines, and the decision I have to make hinges on interpreting how `parse_line` is used rather than on locating anything. Farming that out would cost more than it saves and would hand the interpretive part to the wrong reader.

## Phase 1 — The decision I have to make before any test: how the bucket key gets its timestamp

I have to settle this before writing the test, because the test's import line and the shape of what I assert depend on it. Four candidates, cheapest first:

1. **Change `parse_line` to return `(timestamp, level, message)`.** Rejected. The card grants `[EXTEND] logparse.py` — one new aggregator — and nothing else; there is no `[MODIFY]` marker on `parse_line`. This change would break the `show` verb in `logcli.py`, which the card explicitly says must keep working unchanged, and four existing tests, which the card explicitly says must still pass. The docstring's "nothing downstream has needed it so far" reads like an invitation, and I am declining it: it is a hint for a future card, not a licence in this one.
2. **Add a second public helper (e.g. `parse_line_with_timestamp`) and build on it.** Rejected. The card asks for one new function. A second public name is surface the card didn't ask for, and it would need its own tests and its own docs to earn its keep.
3. **Reuse the existing `_LINE` regex directly inside the new aggregator.** *Chosen.* The timestamp is already captured by name in a compiled pattern that lives in the same module. Using `_LINE.match(...).group("ts")[:13]` from inside `logparse.py` is reuse of what is already in reach, adds no new public surface, touches no existing interface, and keeps the malformed/unknown-level skipping delegated to `parse_line` exactly as the other two aggregators do. The cost is that the line is matched twice — once inside `parse_line`, once for the timestamp. That is a few microseconds on a function that already re-scans, and I would rather pay it than break an interface.
4. **Slice the raw line: `line[:13]`.** Rejected even though it is the very cheapest. It happens to work — `_LINE` anchors `ts` at `^` with no leading-whitespace tolerance, so the timestamp does start at index 0 — but it is correct only by coincidence of the current regex, reads as a magic slice of an arbitrary string, and would silently produce nonsense keys the day the format grows a prefix. The named group says what it means.

**Stop point I would surface to a human.** Before implementing, I would confirm this ruling with whoever owns the card, since option 1 is the change the module's own docstring is fishing for:

- *If they confirm "stay within `[EXTEND]`"* (my default, and what I proceed on absent an answer): implement option 3 as below, and record in the report that broadening `parse_line` is the better long-term shape and wants its own card carrying a `[MODIFY]` marker plus the `logcli.py` call-site update.
- *If they rule "widen `parse_line` now"*: I would stop and ask for the card to be reissued with `[MODIFY] logparse.py` and `[MODIFY] logcli.py` on it, because that version of the work changes a published return shape and a CLI verb, and it contradicts two explicit constraints written into C4. I would not fold it in quietly under the current card.

## Phase 2 — Red: write the failing test first

Write into `test_logparse.py` (`[EXTEND]`, appending only):

- Extend the existing import on line 3 to also import `errors_by_hour`, keeping the alphabetical order already in use: `from logparse import ParseError, count_by_level, errors_by_hour, errors_only, parse_line`.
- **Leave `SAMPLE` untouched.** Six existing tests depend on its exact contents; editing it would be an unrequested change to shared fixture state. Where I need lines `SAMPLE` doesn't have, I add a *new* module-level constant beside it, e.g. `HOURLY_SAMPLE`, containing: two ERROR lines in hour `2026-09-01T15`, then one ERROR line in the earlier hour `2026-09-01T14` (so first-seen order and sorted order disagree and the test can actually tell them apart), plus a `WARN`-only hour, a `TRACE` (unknown level) line in its own hour, and a malformed line.
- Add the tests to the existing `AggregatorTest` class, matching the file's plain-`assertEqual` style:
  1. `test_errors_by_hour_counts_per_bucket` — `errors_by_hour(SAMPLE)` equals `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. Reusing `SAMPLE` here also pins that the malformed trailing line is skipped and that keys are the 13-character `YYYY-MM-DDTHH` form.
  2. `test_errors_by_hour_keeps_first_seen_order_and_omits_quiet_hours` — against `HOURLY_SAMPLE`, assert both the mapping (hour 15 → 2, hour 14 → 1) and `list(...keys()) == ["2026-09-01T15", "2026-09-01T14"]`. The key-list assertion is the only thing that proves *first-seen* rather than sorted or arbitrary order, and the mapping proves the WARN-only hour, the TRACE hour, and the malformed line contribute no buckets.
  3. `test_errors_by_hour_of_empty_input` — `errors_by_hour([])` equals `{}`. One line, pins the degenerate case the CLI will hit on an empty log file.

Three tests, no more. They assert observable behaviour of the returned dict — counts, membership, key order — and nothing about how the function reaches it.

**Run:** `python3 -m unittest -q test_logparse`.

**What I expect and require to see:** failure, and specifically an `ImportError`/`cannot import name 'errors_by_hour' from 'logparse'` at collection — that is the right reason to fail (the function does not exist), not a wrong assertion or a typo'd fixture. If instead I saw the existing six tests error out, that would mean I damaged the shared import or `SAMPLE`, and I would fix that before going further. I will not proceed to green on a failure I haven't read.

## Phase 3 — Green: the smallest thing that passes

Append to `logparse.py`, after `errors_only`, following the two aggregators' exact shape:

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to how many ERROR lines fell in it.

    Buckets appear in first-seen order; hours without errors are absent. Malformed lines
    and unknown levels are skipped, as the other aggregators do.
    """
    counts = {}
    for line in lines:
        try:
            level, _message = parse_line(line)
        except ParseError:
            continue
        if level != "ERROR":
            continue
        hour = _LINE.match(line.rstrip("\n")).group("ts")[:13]
        counts[hour] = counts.get(hour, 0) + 1
    return counts
```

Deliberate non-choices, so they're on the record rather than accidental:

- A plain `dict` gives first-seen order for free on every supported Python; no `OrderedDict`, no `collections.Counter` (which would order by insertion too but adds an import and a non-`dict` return type the card didn't ask for), no `defaultdict` (leaks a factory into the returned object).
- No `datetime` parsing. The card fixes the bucket as the first 13 characters of the timestamp field and states timestamps are always the ISO form; parsing to a datetime and reformatting would be strictly more code, more failure modes, and would change nothing observable.
- The `_LINE.match(...)` is safe to dereference without a `None` check *only because* `parse_line` already succeeded for this same string on the previous statement — same pattern, same input. I'd rather have that be true than add a defensive branch no test can reach.
- No changes to `parse_line`, `count_by_level`, `errors_only`, `LEVELS`, `_LINE`, or `ParseError`. No changes to `logcli.py` at all — the card says a later card wires the `hourly` verb, so I add no verb, no `elif`, and no docstring edit there. Noting the opportunity is not acting on it.

**Run:** `python3 -m unittest -q test_logparse`. Expect `OK` with 9 tests — the 6 existing plus my 3.

**Not delegated.** The green phase here is roughly ten lines in one file. Briefing a worker, then reading its diff back and re-running the suite myself, is strictly more work than typing it, and this particular implementation turns on the `_LINE`-reuse judgement I just made rather than on volume. If it returned option 4's `line[:13]` I'd be rewriting it anyway.

## Phase 4 — Refactor

Re-read the three aggregators side by side and look honestly for duplication worth removing. My expectation is that I extract nothing: the shared `try/except ParseError: continue` prologue is three lines repeated three times, and factoring it into a generator helper would add a new module-private name and an indirection to save nothing, while making the two functions the card told me not to touch churn. If I find something genuinely worth it I'd do it and re-run the suite; otherwise I record "no refactor taken, and why" rather than inventing one.

Then a final confirming run of `python3 -m unittest -q test_logparse`, expecting `OK`.

## Phase 5 — Verify the untouched things are actually untouched

The card makes two promises beyond the new function, so I check them rather than assume them:

- `logcli.py` is byte-for-byte unchanged (I never opened it for writing).
- Its three verbs still work: a smoke run of `python3 logcli.py levels <a temp log file>`, `errors`, and `show`, expecting the same output shape as before and exit code 0 — since `show` is the verb that would have broken under the interface change I declined, and I want the evidence, not the argument.

If any of that misbehaves, I stop and report rather than patching `logcli.py`, because it is out of this card's scope.

## Phase 6 — Close out

- Flip the card's checkbox on line 3 of `CARD.md`: `- [ ] **C4 · hourly error counts**` → `- [x] **C4 · hourly error counts**`. (There is no `tasks.md` in this workspace; `CARD.md` is where the checkbox lives. If a `tasks.md` were expected to exist elsewhere, I would flag its absence rather than create one.)
- Write `cycle-report.md` at the workspace root.

**What the report says** — a record of what happened, not a verdict that it passed:

- Files changed: `logparse.py` (one new function, +~18 lines, nothing else altered), `test_logparse.py` (one import line extended, one new fixture constant, three tests added to `AggregatorTest`), `CARD.md` (checkbox).
- The red phase verbatim: the command, the `ImportError` on `errors_by_hour`, and that I confirmed it failed for the absence of the function rather than a broken fixture. Then the green run's `OK` with the test count.
- **The main difficulty, stated plainly:** `parse_line` discards the timestamp, so the card's bucket key was not reachable through the existing public helper. I reused the module's own `_LINE` regex inside the new aggregator instead of widening `parse_line`'s return tuple, because the card granted extend-only and widening would have broken `logcli.py`'s `show` verb and four existing tests. Cost disclosed: each ERROR line is matched twice.
- **Noted, not acted on:** `parse_line`'s docstring now understates reality — a downstream caller *does* need the timestamp. Broadening it to return the timestamp (and updating `logcli.py`'s unpack) is a reasonable future card and needs a `[MODIFY]` marker on both files. I did not do it here.
- The order assertion is the load-bearing one: `list(keys())` is checked explicitly, since a mapping-only assertion would pass under sorted order too and quietly let a future `sorted()` slip in.
- No subagents or workers were dispatched at any point in this cycle; all reading, testing, and implementation was done directly.
- Any point where the Phase 1 stop was resolved differently than my stated default, and what changed as a result.