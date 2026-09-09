FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4 (`errors_by_hour`)

## Phase 0 — Read the ground, before any code

**Done as the first act (already read above):** all four workspace files in full — `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`. The card names `logparse.py` and `test_logparse.py` as existing code I'm extending, so reading them whole (not grepping) is mandatory; `logcli.py` I read because the card says it must keep working unchanged, and I need to know exactly which `logparse` names it depends on (`ParseError`, `count_by_level`, `errors_only`, `parse_line`).

**Facts I take forward:**
- Lines are `ts level msg`, matched by module-private `_LINE` regex, anchored at position 0 with `(?P<ts>\S+)`.
- `parse_line` returns `(level, message)` only — **it deliberately discards the timestamp** ("checked for presence and then dropped — nothing downstream has needed it so far").
- Existing aggregators (`count_by_level`, `errors_only`) share one shape: loop, `try: parse_line(...) except ParseError: continue`, accumulate.
- `SAMPLE` in the test file is shared by four assertions; its contents are load-bearing for the two existing aggregator tests.
- `LEVELS` is a tuple; `"ERROR"` is a member.

**No delegation in this phase.** The entire workspace is four short files totalling under 200 lines, all of which I need interpretively (I'm deciding how to reach the timestamp without breaking an interface). Handing that to a cheap reader would cost more than it saves and would return facts I'd have to re-read anyway. The one sweep that would have justified a throwaway cheap reader — "is there any other caller of `logparse` anywhere in the tree?" — is already answered exhaustively by the file listing: three Python files, all read. So: zero subagents spawned for this card, deliberately.

## Phase 1 — Pre-code check: does this need new code at all, and what's the cheapest thing that works?

Before writing a test I settle the design question the card creates, because it determines what the test may assume.

1. **Can I delete/avoid instead?** No — nothing existing produces hour buckets, and the story (S4, on-call engineer sees which hour errors clustered) needs real per-hour counts. New code is warranted, but it should be one small function.
2. **Can I reuse `parse_line`?** Only partially. `parse_line` gives me the validation I want (malformed → `ParseError`, unknown level → `ParseError`) but throws away the one field I need. So a pure `parse_line`-based implementation is impossible without changing its return value.
3. **Cheapest options, ranked:**
   - **(A) Re-match `_LINE` inside the new aggregator** and read `group("ts")[:13]`. No interface change, no new import, reuses the existing regex rather than inventing a second parser. Since only `ERROR` is counted, the single check `level != "ERROR"` already skips unknown levels too (an unknown level is by definition not `"ERROR"`), so I get the card's "malformed lines and unknown levels are skipped" behaviour for free without duplicating the `LEVELS` membership test. **This is my choice.**
   - (B) Call `parse_line` for validation *and* slice `line[:13]` for the bucket. Works only because the regex is anchored with no leading whitespace allowed — correct, but it's a clever coupling a future reader would have to re-derive. Rejected as too cute.
   - (C) Change `parse_line` to return the timestamp. Rejected — see the stop below.
4. **What I will *not* write:** no `datetime`/`dateutil` parsing (the card guarantees the ISO form; parsing and reformatting would be strictly more code for the same key), no `collections.OrderedDict` (plain `dict` has preserved insertion order for years — the platform already gives me "first-seen order"), no `Counter` (a plain dict keeps the module's "returns plain data" promise and needs no import), no timestamp validation, no new helper, no `hours_by_level` generalisation, no touching `logcli.py`.

### Stop / decision point I would surface

The card marks `logparse.py` as `[EXTEND]` only, but the field the feature needs is the one `parse_line` throws away. The tempting move is to widen `parse_line` to `(ts, level, message)` — that is a modification of an existing public interface with no `[MODIFY]` marker, and it would force edits to `logcli.py`'s `show` verb and three existing tests, both of which the card explicitly forbids changing. **I would refuse that on my own authority and proceed with option (A)**, recording the refusal in the report rather than silently working around it.

- What I'd confirm with a human if asked: "Is `errors_by_hour` allowed to re-match the timestamp locally, or do you want `parse_line` widened in a later card?"
- **If the ruling is "re-match locally"** (my stated default): proceed exactly as planned below.
- **If the ruling is "widen `parse_line`"**: I stop and escalate rather than execute, because it contradicts the card's own constraint that `logcli.py` and every existing test keep working unchanged — that's a card-scope change, not an implementation detail, and it belongs in its own card with a `[MODIFY]` marker.
- **If the ruling is "add a separate timestamp accessor"**: that's a second public function the card didn't ask for; I'd note it as a follow-up and still ship (A).

## Phase 2 — Red: write the failing test(s) first

**File written:** `test_logparse.py` (extend only; `SAMPLE` left byte-for-byte alone, since four existing assertions depend on it).

Changes:
1. Add `errors_by_hour` to the existing single `from logparse import ...` line, keeping its alphabetical-ish ordering style.
2. Add three tests to the existing `AggregatorTest` class (same class, same naming style as `test_count_by_level_skips_malformed` / `test_errors_only_in_order`):
   - `test_errors_by_hour_buckets_sample` — `errors_by_hour(SAMPLE)` equals `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. Reuses the shared fixture; pins the 13-character key shape and the skipping of the trailing garbage line.
   - `test_errors_by_hour_counts_and_keeps_first_seen_order` — a small local list where an hour-15 error comes first, then two hour-14 errors, plus a non-ERROR line. Asserts the dict equals `{"2026-09-01T15": 1, "2026-09-01T14": 2}` **and** separately asserts `list(result) == ["2026-09-01T15", "2026-09-01T14"]`, because dict equality alone does not pin order and "first-seen order" is an explicit requirement of the card.
   - `test_errors_by_hour_skips_malformed_and_unknown_levels` — a list of only a malformed line, a `TRACE` line, and an `INFO` line; asserts `{}`. This covers both "malformed/unknown skipped" and "hours with no errors absent" in one assertion.

**Test I would run:** `python3 -m unittest -q test_logparse`

**What I expect it to show:** a collection-time `ImportError: cannot import name 'errors_by_hour' from 'logparse'` — i.e. the whole module fails to import, so the run is a single error rather than three clean failures. I would read the traceback and confirm it names `errors_by_hour` specifically. That is the right reason to be red (the function genuinely does not exist yet); I would not accept a red I couldn't attribute to the missing name. I note honestly that in this phase the six existing tests are also blocked by that import — expected, and resolved in Phase 3, not a sign I broke them.

## Phase 3 — Green: the smallest implementation that passes

**File written:** `logparse.py` — one new function appended after `errors_only`, following the existing house style (docstring in the same voice as its neighbours, same loop shape, no new imports).

Shape of what I'd add:

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours without errors are absent; malformed lines and unknown levels are skipped.
    """
    counts = {}
    for line in lines:
        match = _LINE.match(line.rstrip("\n"))
        if match is None or match.group("level") != "ERROR":
            continue
        bucket = match.group("ts")[:13]
        counts[bucket] = counts.get(bucket, 0) + 1
    return counts
```

Notes I'd carry into the report: the `rstrip("\n")` mirrors `parse_line` so a trailing-newline line behaves identically; the single `!= "ERROR"` test is what subsumes the unknown-level rule; the plain dict is what supplies first-seen ordering.

**Test I would run:** `python3 -m unittest -q test_logparse`
**What I expect it to show:** `OK` with 9 tests — the 6 pre-existing plus my 3. If ordering fails I'd suspect I reached for something order-losing; if the sample test fails on key length I'd re-check the 13-character slice against `2026-09-01T14:03:22Z` (chars 0–12 = `2026-09-01T14`). I would fix only the thing that failed.

## Phase 4 — Refactor and regression guard

1. Re-read my two edited files end to end and ask whether anything can come *out*. Specifically: is the `errors_only`/`errors_by_hour` overlap worth factoring into a shared helper? **Answer: no.** Two five-line loops that share a shape but not a return type do not earn an abstraction, and extracting one would touch `errors_only`, which is existing working code the card didn't open. I record this as a noticed-but-not-acted-on opportunity.
2. **Confirm `logcli.py` is untouched** — it imports four names from `logparse`, all still present with unchanged signatures; my change is purely additive, so the `levels`, `errors`, and `show` verbs are unaffected. Because the card promises the CLI "must keep working unchanged" and there is no CLI test suite to prove it, I'd also do one cheap manual smoke: write a throwaway log file to a system temp directory (**not** into the workspace — I add no files the card didn't ask for) and run `python3 logcli.py levels <tmp>` and `python3 logcli.py show <tmp>`, expecting the same output shape as before. If that smoke misbehaved I'd treat it as a real regression and stop rather than ship.
3. Confirm I did not modify `SAMPLE`, did not modify any of the four `parse_line` tests, and did not add a `hourly` verb (a later card owns that).

## Phase 5 — Bookkeeping and report

1. **Card checkbox.** My usual home for this is `tasks.md`, and **there is no `tasks.md` in this workspace** — the checkbox for C4 lives on line 3 of `CARD.md`. I'd flag that mismatch and, under my default, tick it there: `- [x] **C4 · hourly error counts**`, changing nothing else in the card text.
2. **Write `cycle-report.md`** at the workspace root (no cycle/report directory exists here; I'd flag the assumption rather than invent a directory tree). It records what actually happened, not a pass/fail verdict:
   - The card's own tension: the feature needs the timestamp, and `parse_line` discards it by design. What I chose (re-match the existing `_LINE` regex inside the new aggregator), what I refused (widening `parse_line`, which had no `[MODIFY]` marker and would have forced edits to `logcli.py` and three existing tests), and why.
   - The cheap-option ladder I walked: no `datetime`, no `OrderedDict`, no `Counter` — plain dict insertion order is the platform doing the work for me.
   - The observation that checking `level != "ERROR"` covers the unknown-level rule without a second `LEVELS` lookup, so a reader knows that omission is deliberate.
   - That the red phase presented as a single collection-time `ImportError` rather than three discrete failures, because the new name goes on the file's shared import line, and that I verified the error named `errors_by_hour`.
   - Final suite state: `python3 -m unittest -q test_logparse` → `OK`, 9 tests, the 6 originals intact.
   - The `logcli.py` smoke check and its result.
   - Noted-not-done: the `errors_only`/`errors_by_hour` loop similarity (left alone), and the `hourly` verb wiring (a later card's work).
   - The two workspace assumptions I made (`CARD.md` checkbox in lieu of `tasks.md`; report at workspace root) so a reviewer can correct them cheaply.

**Files I would have written by the end:** `test_logparse.py`, `logparse.py`, `CARD.md` (checkbox only), `cycle-report.md`. **Files I would not touch:** `logcli.py`.