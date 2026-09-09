FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4 (`errors_by_hour`)

## Phase 0 — Read and orient (done during planning)

Read all four files in the workspace end to end (there are only four, all small). Findings that drive every later decision:

- `logparse.py` already has two aggregators (`count_by_level`, `errors_only`) that share one shape: iterate lines, `try: parse_line(line) / except ParseError: continue`, accumulate. My new function must look like a third sibling of those, not a new style.
- **The blocking fact:** `parse_line` returns `(level, message)` only. Its docstring says the timestamp is "checked for presence and then dropped — nothing downstream has needed it so far." C4 is the first thing that needs it. So the timestamp is *not* reachable through the existing public function.
- The module-level `_LINE` regex has a named `ts` group and is already the single place that knows the line shape.
- `logcli.py` imports exactly `ParseError, count_by_level, errors_only, parse_line`. Nothing I do may disturb that import line or those three functions' return shapes.
- `SAMPLE` in `test_logparse.py` is shared by `test_count_by_level_skips_malformed`, which asserts exact counts `{DEBUG:0, INFO:1, WARN:1, ERROR:2}`. **I will not add lines to `SAMPLE`** — doing so silently breaks an existing test. New fixture data goes in a new local constant.

**No delegation.** The whole workspace is four files totalling under 150 lines and I have read all of them. Spawning a cheap read subagent here would cost more than it saves, and the one question that matters (how to reach the timestamp) is an interpretive judgement about an interface boundary, which I do not hand off. I would spawn nothing for this card.

## Phase 1 — Decide how to reach the timestamp, and flag it

Before writing a test I settle the design question, because it determines the test's import.

Three candidate routes, cheapest first:

1. **Change `parse_line` to return the timestamp too** (e.g. `(ts, level, message)`). Cleanest long-term, and the docstring practically invites it — but the card marks `logparse.py` `[EXTEND]`, not `[MODIFY]`, and this would break `errors_only`, `count_by_level`, all three `parse_line` call sites in `logcli.py`, and four existing tests. **Refused** — modifying an existing interface without an explicit modify marker is out of scope for this card, and the card explicitly says `logcli.py` must keep working unchanged.
2. **Add a new public timestamp-extracting helper.** New public surface the card did not ask for. Rejected as scope creep.
3. **Inside `errors_by_hour`, reuse what already exists:** call `parse_line(line)` for the skip decision (so "malformed lines and unknown levels are skipped" stays defined in exactly one place, identical to the other two aggregators), and for the lines that survive as `ERROR`, take the timestamp from the existing `_LINE` regex's `ts` group. No new interface, no changed interface, no duplicated validation rules. **Chosen.**

Cost of route 3: a second regex match, incurred only on ERROR lines. Acceptable, and it beats duplicating the level/malformed rules.

Rejected micro-optimisation: slicing `line[:13]` directly. It happens to be equivalent today (the regex anchors `^\S+`, so a parsing line cannot have leading whitespace), but it hard-codes a positional assumption into a second place. The card says the key is the first 13 characters *of the timestamp field*; I take it from the timestamp field.

**Stop-and-confirm point.** I would surface to the human: *"`parse_line` deliberately drops the timestamp; C4 is the first consumer that needs it. Card says `[EXTEND]`, so I'm going with the non-invasive route (re-read the `ts` group inside the new aggregator) rather than widening `parse_line`'s return. Confirm?"* Branches:
- **Ruled "keep `[EXTEND]`"** → proceed exactly as planned below.
- **Ruled "`[MODIFY]` is authorised"** → separate, larger change: widen `parse_line` to return the timestamp, update `count_by_level`, `errors_only`, the three `logcli.py` call sites, and the two `parse_line` tests in `test_logparse.py` that assert 2-tuples — with its own red step per changed test. I would ask for that to be its own card.
- **No answer available** → proceed under the default, route 3, and record the decision and its rationale in the report.

I do not wait for the answer; I proceed under the default.

## Phase 2 — Red: write the failing tests

Write into `test_logparse.py` (path: `test_logparse.py`). Two edits:

1. Extend the existing import line to `from logparse import ParseError, count_by_level, errors_by_hour, errors_only, parse_line` (alphabetical, matching the existing ordering).
2. Add tests to the existing `AggregatorTest` class — same class the other two aggregator tests live in, same naming style (`test_<function>_<behaviour>`).

New fixture, added beside `SAMPLE`, **not merged into it**:

```python
HOURLY = [
    "2026-09-01T14:03:25Z ERROR queue full",          # hour 14, first seen
    "2026-09-01T15:00:01Z ERROR upstream timeout",    # hour 15
    "2026-09-01T14:59:59Z ERROR late straggler",      # hour 14 again, must not reorder
    "2026-09-01T16:00:00Z INFO  all clear",           # hour 16 has no errors -> absent
    "2026-09-01T16:00:01Z WARN  slow",                # ditto
    "garbage without a level",                        # malformed -> skipped
    "2026-09-01T17:00:00Z TRACE too chatty",          # unknown level -> skipped, 17 absent
]
```

Tests:

- `test_errors_by_hour_counts_and_orders` — `assertEqual(errors_by_hour(HOURLY), {"2026-09-01T14": 2, "2026-09-01T15": 1})`. This one assertion covers: 13-char bucket key, per-hour counting, malformed skipped, unknown level skipped, error-free hours absent.
- `test_errors_by_hour_preserves_first_seen_order` — `assertEqual(list(errors_by_hour(HOURLY)), ["2026-09-01T14", "2026-09-01T15"])`. Separate because `assertEqual` on dicts ignores order, so the ordering requirement in the card would otherwise be untested. Asserts hour 14 leads even though its second line comes after hour 15's.
- `test_errors_by_hour_empty` — `assertEqual(errors_by_hour([]), {})`. Cheap, pins the "no errors → empty dict, not zero-filled" contract that distinguishes it from `count_by_level`.

Run `python3 -m unittest -q test_logparse`.

**Expected red:** `ImportError: cannot import name 'errors_by_hour' from 'logparse'`. I would read the failure text and confirm it names `errors_by_hour` specifically. I flag one honest caveat: because the import is at module top, this collapses the whole file — all 6 existing tests error out too. That is the correct red for a not-yet-existing function, but it means this run does *not* re-confirm the existing 6; Phase 4 does that. I would not paper over this by stubbing the function to get a "prettier" red — a stub returning `{}` would make the empty-list test pass green in the red phase, which is exactly the sort of passes-for-the-wrong-reason I refuse.

## Phase 3 — Green: the simplest implementation

Append to `logparse.py` (path: `logparse.py`), after `errors_only`, mirroring the existing aggregators' structure and docstring voice:

```python
def errors_by_hour(lines):
    """Map each hour that saw an ERROR to how many; first-seen order, quiet hours absent.

    The key is the `YYYY-MM-DDTHH` prefix of the timestamp field. Malformed lines and
    unknown levels are skipped, as in the other aggregators.
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

Notes on the choices, all in service of "smallest thing that actually works":
- Plain `dict` + `.get`, not `collections.Counter` or `defaultdict`. A `Counter` prints and compares as a `Counter`, and a `defaultdict` leaks a factory into the returned value — both are worse return types for a caller that will format CLI output. Ordinary dict insertion order already gives first-seen ordering; no `OrderedDict`, no sort key.
- No `datetime` parsing. The card says the key is a 13-character prefix of a guaranteed-ISO timestamp; parsing to a datetime and reformatting would do more work and introduce timezone questions the card does not have.
- The `.match(...)` is unguarded for `None` deliberately: it can only run after `parse_line` matched the same string with the same regex. I would not add a defensive `if match is None` branch that no test can reach.
- No touching of `LEVELS`, `_LINE`, `parse_line`, or the other two aggregators.

Run `python3 -m unittest -q test_logparse`. **Expected: `OK`, 9 tests** (6 existing + 3 new).

## Phase 4 — Verify nothing else moved

- Confirm the run reports 9 tests and `OK` — this is where the existing 6 get their re-confirmation after the Phase 2 collapse. If any of the original 6 fail, I stop and treat it as my regression, not as a flaky test.
- Confirm `logcli.py` is byte-for-byte untouched and its import line still resolves: run `python3 logcli.py` with no arguments and expect the usage docstring plus exit code 2 (it exercises the import without needing a log file). The `hourly` verb is deliberately **not** added — the card says a later card wires it, and adding it now would be exactly the "while I'm in here" creep I refuse.
- Refactor pass: I expect to change nothing. The function is nine lines and already matches its two siblings. I would resist the tempting-looking refactor of hoisting the shared `try/parse/continue` preamble out of all three aggregators — that would rewrite two functions this card did not ask me to touch. I'd note it as an observation only.

## Phase 5 — Tick the card and report

- Tick the checkbox: `CARD.md` line 3, `- [ ]` → `- [x]`. **Flag:** the card's stated deliverable is an updated `tasks.md`, but there is no `tasks.md` in this workspace — the checkbox lives in `CARD.md` itself. I tick it there and say so rather than creating a `tasks.md` nobody asked for.
- Write `cycle-report.md` recording, honestly: the red was an `ImportError` that collapsed the whole file, and why that's the right red here; the `parse_line` timestamp-drop constraint and my choice of the non-invasive route over widening the signature, including the confirm I would have sought and the default I proceeded under; that I deliberately did not extend `SAMPLE` because `test_count_by_level_skips_malformed` asserts exact counts against it; that `logcli.py` was untouched and the `hourly` verb intentionally left unwired for the later card; the rejected refactor of the shared aggregator preamble, noted and not acted on; final suite state 9 tests, `OK`.

**What I would refuse outright in this card:** changing `parse_line`'s return shape; adding the `hourly` verb to `logcli.py`; adding a public timestamp helper; editing `SAMPLE`; refactoring the two existing aggregators.