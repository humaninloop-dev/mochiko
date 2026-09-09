FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4, `errors_by_hour`

## Phase 0 — Read and orient (done)

I read all four files in the workspace; there is nothing else in it. The whole surface is ~150 lines, so I read it myself rather than dispatching a reader — a cheap-read subagent would cost more in round-trip than the files cost in context. **No delegation on this card.** (The one thing that would have earned a spawn — sweeping a large codebase for other callers of `parse_line` — is answerable by eye here: the callers are `logparse.py` itself and `logcli.py`.)

What I established before writing anything:

- `parse_line` returns `(level, message)` only. Its docstring says the timestamp "is checked for presence and then dropped — nothing downstream has needed it so far." **C4 is the first thing downstream that needs it.** This is the central constraint of the card.
- The timestamp is nonetheless captured by the module-level `_LINE` regex as group `ts`. So the value is reachable without inventing a second parsing rule.
- Both existing aggregators (`count_by_level`, `errors_only`) share one shape: iterate, `try: parse_line(...) except ParseError: continue`, then act on the level. My new function must look like its neighbours.
- Existing tests: 4 in `ParseLineTest`, 2 in `AggregatorTest` = the 6 the card expects to keep passing. Four of them plus `logcli.py` depend on `parse_line`'s exact 2-tuple return.
- `SAMPLE` in the test file already contains exactly the material I need: two ERRORs in different hours (`14:03:25`, `15:00:01`), an INFO and a WARN to be excluded, and a malformed line to be skipped.

## Phase 1 — The pre-code decision: how to reach the timestamp

Before writing a test I settle the design question, because it determines what the test may assume.

The tempting move is to make `parse_line` return the timestamp too. **I refuse that.** The card marks `logparse.py` as `[EXTEND]`, not `[MODIFY]`; changing that return tuple would break `test_splits_level_and_message`, `test_message_keeps_inner_spaces`, and the `show` verb's unpacking in `logcli.py` — all of which the card says must keep working. Adding an optional flag or a parallel `parse_line_with_ts` is a second public entry point the card didn't ask for.

Cheapest thing that genuinely works: **`errors_by_hour` keeps using `parse_line` for validation and skip semantics — identical to its neighbours — and re-matches the already-compiled `_LINE` to lift `group("ts")[:13]` for the lines that survive.** No new regex, no new notion of what a field is, no public surface changed, nothing existing touched. It parses the line twice, which I will state plainly in the report rather than hide; the alternative (restructuring `parse_line` into a private matcher plus a public wrapper) is a refactor of working code that this card does not need, and I'd rather leave that unmade than smuggle it in.

Also rejected as cheaper-but-wrong: `line.split()[0][:13]`. It would work on today's data but plants a second, divergent definition of "the timestamp field" next to the regex that owns it.

**Stop point I'd raise but not block on.** If a reviewer wants `parse_line` to expose the timestamp properly, that is a real and defensible design — it just isn't this card. What I'd confirm: *may C4 change `parse_line`'s public return?* If the ruling is **no** (my default, and what the card's `[EXTEND]` marker says), I proceed exactly as above. If the ruling is **yes**, the work changes shape: the card needs a `[MODIFY]` marker, `logcli.py` comes into scope after all despite the card excluding it, and the two `parse_line` tests must be updated — I'd want that re-scoped as its own card rather than folded in here. I plan and proceed under **no**.

Second, smaller flag, recorded and **not acted on**: once `errors_by_hour` exists, the sentence "nothing downstream has needed it so far" in `parse_line`'s docstring is stale. Editing it is a one-line improvement in a function this card has no business touching, so I note it in the report as a follow-up and leave the file alone.

## Phase 2 — Red: write the failing test first

**Write to:** `test_logparse.py` (append only; no existing line altered).

Add `errors_by_hour` to the existing top-level import from `logparse`, matching the file's import style, and add three tests to `AggregatorTest`:

1. `test_errors_by_hour_buckets_and_skips` — `errors_by_hour(SAMPLE)` equals `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. Covers the 13-character bucket key, exclusion of INFO/WARN, and the skipped malformed line, using the fixture that's already there.
2. `test_errors_by_hour_counts_and_keeps_first_seen_order` — a small local list where a 15:00 ERROR appears before a second 14:00 ERROR, plus an unknown-level `TRACE` line to confirm it's skipped. Asserts both the counts (`{"2026-09-01T15": 1, "2026-09-01T14": 2}` for that arrangement) **and** `list(result)` for the key order, since `assertEqual` on dicts ignores ordering and "first-seen order" is an explicit requirement of the card.
3. `test_errors_by_hour_omits_hours_without_errors` — a list of only INFO/WARN lines returns `{}`. This is the "hours with no errors absent" clause; a dict pre-seeded with hours would fail it.

I will not write tests for timestamps that aren't the ISO form — the card states they always are, so defending against short or garbage timestamps would be code and tests the card didn't ask for.

**Run:** `python3 -m unittest -q test_logparse`.
**Expect:** failure, and I check *which* failure. Because the import is at module top, the expected output is a collection-time `ImportError: cannot import name 'errors_by_hour' from 'logparse'` — the module fails to load, so all tests are reported as errored, not just the three new ones. That is the honest red for a function that does not exist yet, but it means this run does **not** re-confirm the 6 existing tests; Phase 4 does. If the failure text is anything other than that missing name, I stop and investigate rather than pressing on to green.

## Phase 3 — Green: the smallest implementation

**Write to:** `logparse.py`, appended after `errors_only`, no existing line touched. One function, shaped like its two neighbours:

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours with no errors are absent. Malformed lines and unknown levels are skipped,
    as in the other aggregators.
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

Notes on what is deliberately *absent*: no `collections.Counter` or `defaultdict` (a plain dict already preserves insertion order and keeps the return type identical to what `count_by_level` hands back — a Counter would leak a subclass into the CLI's future formatting code); no `datetime` parsing (the card defines the bucket as a 13-character slice, and parsing to a datetime and reformatting would be more code doing more work with more failure modes); no sorting (first-seen order is what was asked for); no `strip()` beyond the `rstrip("\n")` that mirrors `parse_line`. The re-match is safe without a `None` guard precisely because `parse_line` already succeeded on the same string.

**Run:** `python3 -m unittest -q test_logparse`. **Expect:** `OK`, 9 tests.

## Phase 4 — Refactor and verify the untouched work

Refactor pass: I expect to make no changes. The function is nine lines and already follows the file's established loop-and-skip idiom; there is no duplication worth extracting that wouldn't mean editing `parse_line`. If I find myself wanting to extract a shared helper across all three aggregators, I record that as a follow-up rather than doing it — it would rewrite two working functions this card doesn't cover.

Verification that the rest still stands:
- The same suite run is the card's stated gate; I confirm the 6 pre-existing tests appear as passing alongside the 3 new ones, since the red run couldn't tell me that.
- `logcli.py` — I neither open it for editing nor change anything it imports, so its three verbs are untouched by construction. As cheap belt-and-braces I'd write a couple of the `SAMPLE` lines to a scratch file outside the workspace and run `python3 logcli.py levels`, `errors`, and `show` against it, expecting the same output as before my change. If any of those had shifted, that would mean I'd altered a shared interface and I'd revert to Phase 1.
- I confirm my diff touches exactly two files and adds only: one import name, three test methods, one function.

## Phase 5 — Close out

**Write to:** `CARD.md` — flip line 3's `- [ ]` to `- [x]`. **Flag:** there is no `tasks.md` in this workspace; the card's own checkbox is the only tracker present, so that's what I tick, and I say so in the report rather than creating a `tasks.md` nobody asked for.

**Write to:** `cycle-report.md` in the workspace root — an account of what actually happened, not a verdict:

- The card's one real friction: `parse_line` deliberately discards the timestamp that C4 needs, and how I got at it via the existing `_LINE` group instead of changing a public return that four tests and the CLI depend on.
- The honest cost of that choice: `errors_by_hour` matches each surviving line twice. Named, not buried, with the reason I accepted it over refactoring `parse_line`.
- The red phase surfaced as a module-level `ImportError` rather than three isolated failures, so the existing six were only re-confirmed green in Phase 4.
- Deliberate omissions: no `Counter`/`defaultdict`, no `datetime`, no defensive handling of non-ISO timestamps (the card rules them out), no `logcli.py` wiring — the `hourly` verb is a later card and I left it unwired even though the plumbing is two lines.
- Follow-ups noted, not done: the now-stale "nothing downstream has needed it so far" line in `parse_line`'s docstring; the possibility of a shared skip-loop helper across the three aggregators; a lurking behaviour where a well-formed line with a non-ISO first field would bucket under a nonsense key, which is out of scope while the card guarantees ISO timestamps.
- Final state: suite `OK` at 9 tests, two files changed, `logcli.py` unmodified.