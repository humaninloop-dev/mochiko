FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — C4 · hourly error counts

## What the workspace forces

`parse_line` deliberately throws the timestamp away (`logparse.py:23-35` — "checked for presence and then dropped"). `errors_by_hour` needs it. That is the only real design decision in this card, and it is settled below in Phase 2. Everything else is a small additive aggregator.

---

## Phase 1 — Confirm the baseline is green before touching anything

- Run `python3 -m unittest -q test_logparse`.
- Expect: `OK`, 6 tests. This matches the card's claim of 6 existing tests (4 in `ParseLineTest`, 2 in `AggregatorTest`).
- If it is *not* green on arrival, I stop and report that before writing anything — a pre-broken baseline changes what "make the test pass" means, and I would not want to silently fix an unrelated failure inside this card. Default if that happens: report the failure, fix nothing outside C4's scope, and ask whether to proceed.
- No files written in this phase.

## Phase 2 — Decide how `errors_by_hour` gets the timestamp (decision point)

Three options, and my ruling:

1. **Change `parse_line` to return `(ts, level, msg)`.** Rejected outright. It breaks `test_splits_level_and_message` and `test_message_keeps_inner_spaces`, and breaks all three call sites in `logcli.py:19-31`. The card explicitly says every existing test and `logcli.py` must keep working. I would refuse this even if it looks tidier.
2. **Add a new *public* `parse_line_ts(line)`.** Rejected as scope creep — the card asks for one new aggregator, not a second public parsing entry point that a later card may or may not want.
3. **Add a private `_parse_full(line)` returning `(ts, level, msg)`, and make `parse_line` a thin wrapper that returns `(level, msg)` from it.** ← **This is what I do.** Validation (malformed regex, unknown level, `ParseError` message text) stays in exactly one place, `parse_line`'s observable contract is byte-for-byte unchanged, and `errors_by_hour` gets the field it needs without a public API addition.

This is a routine judgment call, so I make it and move on rather than blocking. If the user later rules that the timestamp should be public API, the onward branch is trivial: rename `_parse_full` → `parse_line_ts`, add it to the `logparse` import in `test_logparse.py`, and add a test for it; no logic changes and no changes to `errors_by_hour`'s body.

## Phase 3 — Write the failing test first (`test_logparse.py`)

This is the red step; it happens before any edit to `logparse.py`.

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-4snq_kc6/ws/test_logparse.py`:

- Add `errors_by_hour` to the existing import on line 3, keeping the alphabetical ordering already there: `from logparse import ParseError, count_by_level, errors_by_hour, errors_only, parse_line`.
- Add a second fixture next to `SAMPLE`, because `SAMPLE` alone cannot prove the two properties the card names (first-seen order, multiple errors collapsing into one bucket). `SAMPLE`'s two errors land in two different hours, one each, and in ascending order — so it cannot distinguish first-seen order from sorted order. New fixture, deliberately with a later hour appearing before an earlier one and with a repeat:

  ```python
  UNSORTED = [
      "2026-09-01T15:00:01Z ERROR upstream timeout",
      "2026-09-01T14:03:25Z ERROR queue full",
      "2026-09-01T15:41:07Z ERROR upstream timeout again",
      "2026-09-01T16:00:00Z INFO  recovered",
      "2026-09-01T15:59:59Z TRACE too chatty",
      "garbage without a level",
  ]
  ```

- Add tests to the existing `AggregatorTest` class (it is where the other aggregators live; no new class needed):
  - `test_errors_by_hour_buckets_the_sample` — `errors_by_hour(SAMPLE) == {"2026-09-01T14": 1, "2026-09-01T15": 1}`. Pins the 13-character key shape (`2026-09-01T14`, not `2026-09-01T14:` and not the whole stamp) and confirms hours with no errors are absent.
  - `test_errors_by_hour_is_first_seen_order` — asserts on `list(errors_by_hour(UNSORTED).items())` equals `[("2026-09-01T15", 2), ("2026-09-01T14", 1)]`. Comparing the items *list*, not the dict, is the point: `assertEqual` on two dicts ignores ordering, so a dict comparison would not catch a `sorted()` slipping in. This test also covers the counting-repeats case and proves the `INFO` line, the unknown-level `TRACE` line, and the malformed line are all skipped.
  - `test_errors_by_hour_without_errors_is_empty` — `errors_by_hour(["2026-09-01T14:03:22Z INFO  fine", "garbage"]) == {}`. Pins "empty dict", not `None` and not zero-filled buckets, since `count_by_level` sets the opposite precedent of pre-seeding every key and I want the difference nailed down.

- Run `python3 -m unittest -q test_logparse`. Expected failure: an `ImportError` on the module import — `cannot import name 'errors_by_hour' from 'logparse'` — which aborts the whole module, so unittest reports 1 error and does not run the other 6. That is the correct red for a function that does not exist yet. I want to *see* this output, not assume it; if the suite somehow passes here, something is wrong with my import edit and I re-check before continuing.

## Phase 4 — Make it pass (`logparse.py`)

Edits to `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-4snq_kc6/ws/logparse.py`:

- Extract `_parse_full(line)` holding the current body of `parse_line` (the `_LINE.match`, the `None` check with the same `f"malformed line: {line!r}"` message, the level-membership check with the same `f"unknown level {level!r} in line: {line!r}"` message), returning `(ts, level, msg)`.
- Reduce `parse_line` to delegate: `_ts, level, message = _parse_full(line); return level, message`. Its docstring stays as-is — the description of dropping the timestamp is still accurate for *this* function.
- Append `errors_by_hour(lines)` after `errors_only`, following the shape of the two existing aggregators exactly — plain `for` loop, `try` / `except ParseError: continue`, no comprehension, docstring in the same one-or-two-line style:

  ```python
  def errors_by_hour(lines):
      """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

      Hours without errors are absent; malformed lines are skipped, as elsewhere here.
      """
      counts = {}
      for line in lines:
          try:
              timestamp, level, _message = _parse_full(line)
          except ParseError:
              continue
          if level != "ERROR":
              continue
          bucket = timestamp[:13]
          counts[bucket] = counts.get(bucket, 0) + 1
      return counts
  ```

  Insertion-ordered `dict` gives first-seen order for free; I do not import `OrderedDict` or `Counter`. `Counter` in particular would be a worse fit here — it would make `result["missing-hour"]` return `0` instead of raising, quietly weakening the "hours with no errors absent" guarantee.

- Run `python3 -m unittest -q test_logparse`. Expected: `OK`, 9 tests — the 6 originals plus the 3 new ones, exactly what the card's TEST line asks for.

## Phase 5 — Verify I did not break `logcli.py`

The card says `logcli.py` is out of scope and must keep working. Since I refactored the innards of `parse_line`, a green unit suite is necessary but not sufficient evidence — `logcli.py` has no tests.

- Write a throwaway fixture `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-4snq_kc6/ws/_c4_smoke.log` containing the five `SAMPLE`-style lines including the garbage one.
- Run `python3 logcli.py levels _c4_smoke.log`, `errors`, and `show` against it.
- Expect: `levels` prints the four levels with counts 0/1/1/2; `errors` prints the two error messages; `show` prints three parsed lines plus one `?     malformed line: 'garbage without a level'`. The `show` path is the one that actually renders `ParseError`'s text, so it is what confirms my extraction preserved the exception messages verbatim.
- Delete `_c4_smoke.log` afterward so the workspace ends with exactly the three source files plus `CARD.md`. I would not add a `test_logcli.py` — wiring and testing the CLI is a later card's job, and the card tells me not to touch `logcli.py`.

## Phase 6 — Final check and report

- Re-run `python3 -m unittest -q test_logparse` one last time, and confirm `git status`-equivalent by eye: only `logparse.py` and `test_logparse.py` changed, `logcli.py` and `CARD.md` untouched.
- I would **not** tick the `- [ ]` checkbox in `CARD.md` unless asked — the card describes work on the three source files and says nothing about editing itself. I will mention this in the report and tick it on request.

Report at the end: the two files changed and what changed in each; that `parse_line`'s signature, return value, and both `ParseError` message strings are unchanged and that I extracted `_parse_full` rather than widening `parse_line`'s return, with the reason; the test count going 6 → 9 and the actual `unittest` output; the three CLI verbs' smoke output; plus the one thing I noticed and deliberately did not build (below).

---

## Flagged, not built

`_LINE` matches the timestamp as bare `\S+`, so a line like `oops ERROR boom` parses cleanly today and my `errors_by_hour` would bucket it under `"oops"` rather than skipping it. The card states timestamps are always the ISO form and defines the key as a plain 13-character slice, so adding timestamp validation would go beyond what was asked and would also change `parse_line`'s existing skip/raise behaviour for the other two aggregators — a behaviour change well outside C4. I note it for whoever writes the `hourly` verb card and leave the code as specified.

## Delegation

None. This is a two-file, roughly twenty-line additive change in a four-file workspace that I have read in full; handing it to another worker would cost more context than it saves and would put a second pair of hands on the one subtle constraint (not disturbing `parse_line`'s contract). I would not spawn a reviewer agent either — if the user wants an independent pass, `/code-review ultra` is theirs to trigger, not mine.