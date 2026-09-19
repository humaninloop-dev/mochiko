FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py` (plus a full listing of the workspace root, which contains only these four files — there is no `tasks.md`, no test runner config, no existing `cycle-report.md`)

---

# Action plan — cycle card C4, `errors_by_hour(lines)`

## Phase 0 — Read and orient (already done above)

**Done:** read all four workspace files end to end before writing anything, because C4 is explicitly a brownfield card touching two existing files.

**What the reading established:**

- `logparse.py` has one regex `_LINE` with named groups `ts`, `level`, `msg`; one exception `ParseError`; one line parser `parse_line`; two aggregators `count_by_level` and `errors_only`.
- **The load-bearing fact for this card:** `parse_line` matches the timestamp but *deliberately discards it* — it returns `(level, message)` only, and its docstring says so ("checked for presence and then dropped — nothing downstream has needed it so far"). C4 is the first thing that needs it. So the new aggregator cannot get its bucket key from `parse_line`'s return value as it stands.
- Both existing aggregators share an identical shape: iterate, `try: parse_line(...) except ParseError: continue`, accumulate. The new aggregator must look like these two, not like something new.
- `logcli.py` line 27 unpacks `parse_line` into exactly two names, and `test_logparse.py` lines 16 and 19 assert `parse_line` returns a 2-tuple. Four existing tests pin `parse_line`'s behavior including that it *raises* on malformed and on unknown levels.
- No `tasks.md` exists in this workspace. The card's checkbox lives on line 3 of `CARD.md`.

**Delegation decision:** none. The whole workspace is four short files totaling under 200 lines, all of which I need interpreted rather than merely located, and the one question that matters (does anything else depend on `parse_line`'s arity?) is completeness-sensitive — a missed caller would be a broken CLI. Spawning a cheap reader here would cost more than it saves and would put a correctness-critical enumeration in someone else's hands. I read it myself.

## Phase 1 — Pre-code check: does this need new code at all?

**What I would do:** before writing a line of test, ask whether C4 can be satisfied without a new function, and pick the cheapest option that actually works.

- *Can it be deleted / does it already exist?* No. Nothing in `logparse.py` groups by anything, and no aggregator exposes a timestamp. `errors_only` gives messages with no time attached. There is no existing path to the answer.
- *Standard library?* The counting itself is `collections.Counter`, and `Counter` preserves first-insertion order. But `Counter` is a `dict` subclass, and the card asks for "a dict". A `Counter` would satisfy `assertEqual` against a plain dict and would satisfy `isinstance(x, dict)`, but it also brings `most_common`, arithmetic operators, and a `[]` that returns `0` for absent keys instead of raising `KeyError`. The card's contract is specifically "hours with no errors **absent**" — a container that silently answers `0` for an absent hour quietly contradicts the stated contract at the call site. `collections.defaultdict(int)` has the same defect, worse. So: plain `dict` with `counts[b] = counts.get(b, 0) + 1`, which is one line, no import, and matches `count_by_level`'s hand-rolled dict style. Cheapest option that *genuinely* works.
- *`datetime` parsing for the hour bucket?* No. The card defines the bucket as "the first 13 characters of the timestamp field" — a slice, spelled `ts[:13]`. Parsing to a `datetime` and reformatting would be strictly more code, would introduce failure modes on the torn lines the aggregators are supposed to tolerate, and would answer a question nobody asked. Slice it.
- *New public API surface?* One function, `errors_by_hour`. Nothing else.

**Conclusion:** minimum viable new code is one aggregator of roughly six lines plus a way to see the timestamp. No new imports.

## Phase 2 — The one real design decision, and where I would stop

**The problem:** the bucket key comes from the timestamp, and `parse_line` throws the timestamp away.

**What I would refuse:** changing `parse_line` to return `(ts, level, message)`. The card marks `logparse.py` as `[EXTEND]`, not `[MODIFY]`; it says `logcli.py` "must keep working unchanged, as must every existing test." A 3-tuple breaks `logcli.py:27` and `test_logparse.py:16,19` on the spot. Not doing it, even though it is the change the module's own docstring is quietly asking for.

**The three options that respect the interface:**

- **(a)** In `errors_by_hour`, call `parse_line(line)` for validation (skip on `ParseError`, exactly like the other two aggregators), then take the bucket from the raw line: `line.split()[0][:13]`. Zero edits to any existing line of code. Costs a second, shallower pass over the line, and re-derives the timestamp by a *different* rule than `_LINE` uses — a divergence that is invisible today but is a real seam.
- **(b)** In `errors_by_hour`, use `_LINE.match(...)` directly and re-check `level in LEVELS` inline. Also zero edits — but it duplicates the validation logic, so a future change to what counts as malformed would silently miss this aggregator. Rejected: cheap in lines, expensive in correctness coupling.
- **(c)** Extract a private `_match(line)` that runs the regex and the `LEVELS` check and returns the match (raising `ParseError` with the existing messages), then make `parse_line` a two-line wrapper returning `(level, msg)` from it, and have `errors_by_hour` use `_match` to read `ts`. Public surface unchanged, single source of validation truth, no double parse. Costs a small edit to an existing function's body.

**My default: (a) for the green phase, then (c) evaluated in refactor** — this is exactly what the red/green/refactor split is for. I get to green without touching a single existing line, which is the safest possible way to land a brownfield extension; then, with the new test and the four existing `parse_line` tests both green and holding the behavior still, I do the extraction as a pure refactor and re-run to prove nothing moved.

**Where I would stop for a human ruling:** at the end of the green phase, before doing (c). What I would put to the reviewer: *the green implementation parses each line twice and derives the timestamp by a different rule than the module's own regex; refactoring to a shared private `_match` fixes both but edits the body of `parse_line`, which the card marked `[EXTEND]` rather than `[MODIFY]` — is an internal extraction that leaves the signature, docstring, return value, and exception messages byte-identical within scope?*

- **If yes:** proceed with (c), keeping `parse_line`'s `ParseError` messages *exactly* as they are today — note that both messages interpolate the **original** `line`, not the `rstrip("\n")`-ed one, so `_match` must take the original line and do its own stripping internally, or those messages change for lines with trailing newlines. I would re-run the suite and confirm all 7+ tests still pass with no test edits.
- **If no:** ship (a) as-is and record the double-parse and the `split()`-vs-regex divergence as a known seam in the report, for whoever writes the later card that wires the `hourly` verb.
- **Under the stated default, absent a ruling:** I ship (a) and *do not* perform (c). "`[EXTEND]`, not `[MODIFY]`" is a written instruction and a plausible-looking internal-only refactor is precisely the kind of thing that should not slip in unannounced. The improvement goes in the report as a recommendation, not into the diff. Everything below assumes this.

## Phase 3 — Red: write the failing test first

**Write to:** `test_logparse.py`

Add `errors_by_hour` to the import on line 3 (keeping it alphabetical, as it already is: `ParseError, count_by_level, errors_by_hour, errors_only, parse_line`).

Extend the module-level `SAMPLE` — I would **not** mutate the existing five entries, because `test_count_by_level_skips_malformed` and `test_errors_only_in_order` assert exact counts and an exact list against them; any edit there breaks two passing tests. Instead I add a second fixture beside it:

```python
HOURLY = [
    "2026-09-01T15:00:01Z ERROR upstream timeout",
    "2026-09-01T14:03:22Z INFO  worker started (pid 4242)",
    "2026-09-01T14:03:25Z ERROR queue full: dropped 3 messages",
    "2026-09-01T15:44:10Z ERROR upstream timeout",
    "2026-09-01T16:00:00Z WARN  retrying upstream",
    "2026-09-01T17:00:00Z TRACE too chatty",
    "garbage without a level",
]
```

This fixture is built to exercise every clause of the card at once: hour 15 is seen *before* hour 14 and then recurs, so first-seen order is distinguishable from sorted order and from last-seen; hour 16 has a non-error line only; hour 17's line has an unknown level; the last line is malformed.

Add to `AggregatorTest`, alongside the two existing aggregator tests, following their naming style:

1. `test_errors_by_hour_counts_per_bucket` — `errors_by_hour(HOURLY)` equals `{"2026-09-01T15": 2, "2026-09-01T14": 1}`. Covers counting, the 13-char key, unknown-level skipping, malformed skipping, and "hours with no errors absent" (16 and 17 must not appear).
2. `test_errors_by_hour_buckets_in_first_seen_order` — `list(errors_by_hour(HOURLY))` equals `["2026-09-01T15", "2026-09-01T14"]`. A separate test because `assertEqual` on two dicts ignores order, so test 1 cannot possibly catch an ordering regression; this is the only assertion that pins the card's "first-seen order" clause.
3. `test_errors_by_hour_empty` — `errors_by_hour([])` equals `{}`. Pins that the return is an empty mapping rather than pre-seeded buckets — the deliberate contrast with `count_by_level`, which *does* pre-seed all four levels with zero.

I would not add a test for non-ISO timestamps: the card states timestamps are always the ISO form, and testing an undefined case would freeze behavior nobody specified.

**Run:** `python3 -m unittest -q test_logparse`

**Expected:** failure — and specifically an `ImportError`/`ModuleNotFoundError`-class collection error on line 3 (`cannot import name 'errors_by_hour' from 'logparse'`), which aborts the whole module. I would read the output and confirm that is the reason. This is the honest red for a not-yet-existing function; I would **not** accept it as red-for-the-right-reason until I have also seen the three new tests fail individually against a *defined but empty* function — so:

**Red, step two:** add `def errors_by_hour(lines): return {}` to `logparse.py` (stub only, no logic) and re-run. Expected now: 9 tests run, 6 existing pass, `test_errors_by_hour_counts_per_bucket` fails with `{} != {'2026-09-01T15': 2, '2026-09-01T14': 1}`, `test_errors_by_hour_buckets_in_first_seen_order` fails with `[] != ['2026-09-01T15', '2026-09-01T14']`, and `test_errors_by_hour_empty` **passes**. That last one passing against a stub is expected and fine — it is a boundary assertion, not the driver — and I would say so in the report rather than pretend all three drove the code. This second step is what proves the two real tests fail on *absent behavior* rather than on an import typo.

**I would stop and flag** if the 6 existing tests do not all pass at this point: that would mean the workspace was not green before I started, and I would report that before writing any implementation, since I would otherwise be unable to attribute later failures to my own change.

## Phase 4 — Green: minimum implementation

**Write to:** `logparse.py` — append below `errors_only`, so the file reads parser-then-aggregators as it does now. No new imports; no edits to lines 1–63.

```python
def errors_by_hour(lines):
    """Map each hour bucket with ERROR lines to its count, in first-seen order.

    The bucket is the `YYYY-MM-DDTHH` prefix of the timestamp; hours without errors are
    absent. Malformed lines are skipped, never raised, as in the other aggregators.
    """
    counts = {}
    for line in lines:
        try:
            level, _message = parse_line(line)
        except ParseError:
            continue
        if level != "ERROR":
            continue
        bucket = line.split()[0][:13]
        counts[bucket] = counts.get(bucket, 0) + 1
    return counts
```

Notes on why each choice is the existing pattern rather than my preference: `_message` with the leading underscore mirrors `count_by_level`; the bare `try/except ParseError: continue` mirrors both aggregators verbatim; the docstring's shape and its "malformed lines are skipped, never raised" phrasing echo the neighbours. First-seen order needs no code — it is `dict`'s guaranteed insertion order, which is why plain `dict` is sufficient and `OrderedDict` would be dead weight.

`line.split()[0]` is safe here without an index guard: a line that reaches this point already matched `_LINE`, which requires a leading non-space field, so `split()` cannot return empty. I would not add a defensive check for a state the regex has already excluded.

**Run:** `python3 -m unittest -q test_logparse`
**Expected:** `OK`, 9 tests. This is precisely the card's TEST block: action `python3 -m unittest -q test_logparse`, assert `OK` with the 6 existing tests passing alongside the new ones.

## Phase 5 — Refactor, and the untouched-file check

**Refactor:** per the Phase 2 default, none — the `_match` extraction is recorded as a recommendation, not applied. I would re-read my own diff once for stragglers (no debug prints, no stray import, no incidental reformatting of existing lines).

**Verify `logcli.py` is genuinely untouched and still working.** The card states it must keep working unchanged. My diff does not touch it, and `parse_line`'s arity is unchanged, so it cannot have broken — but "cannot have" is an argument, not evidence. There is no CLI test in this workspace to run. I would:
- confirm by inspection that `logcli.py`'s only imports from `logparse` (`ParseError, count_by_level, errors_only, parse_line`) all still exist with identical signatures;
- do one manual smoke run — write a scratch log file outside the tracked workspace, run `python3 logcli.py levels <file>`, `errors`, and `show`, and confirm all three still print and exit 0.

I would **not** add a `test_logcli.py`. Test coverage for the CLI is real missing work, but it is not this card, and adding it would be exactly the "while I'm in here" expansion the card fences off by saying `logcli.py` is not in this card.

**I would refuse**, and flag rather than silently do: wiring an `hourly` verb into `logcli.py`. The card says a later card does that. Even though the function is now sitting there ready and it is four lines, adding it would make a later card's test pass before that card is written.

## Phase 6 — Close out

**Tick the checkbox.** There is no `tasks.md` in this workspace. The card's checkbox is `CARD.md` line 3, `- [ ] **C4 · hourly error counts**` → `- [x] **C4 · hourly error counts**`. I would change that one character and nothing else in `CARD.md`, and I would note in the report that the checkbox lived in `CARD.md` because no `tasks.md` exists here — so that whoever expects a `tasks.md` update knows where it actually went, and can correct me if the workspace was meant to have one.

**Write `cycle-report.md`** at the workspace root — a record of what happened, not a verdict that the card passed. It would state:

- Files changed: `logparse.py` (one function appended, lines 1–63 untouched), `test_logparse.py` (import line extended, one fixture added, three tests added, existing six untouched), `CARD.md` (checkbox).
- The red evidence in two steps: the import-level failure, then the stub run showing which specific tests failed with which specific diffs — including the honest note that `test_errors_by_hour_empty` passed against the stub and so did not drive any code.
- Green: `OK`, 9 tests, from the card's stated command.
- **The main thing a reader needs:** `parse_line` drops the timestamp by design, C4 is the first caller that needs it, and I obtained the bucket by re-splitting the raw line rather than changing `parse_line`'s return, because the card marked the file `[EXTEND]` and a 3-tuple would break `logcli.py:27` and two existing tests. Consequence carried forward: each ERROR line is now parsed twice, and the timestamp is derived by `split()` in one place and by the `_LINE` regex in another — two rules for one field. Recommendation for a future card: extract a shared private matcher so validation and timestamp extraction have a single source. Flagged, not done.
- The `Counter`/`defaultdict` decision and why plain `dict` was chosen — both would have answered `0` for an hour the card says should be *absent*.
- `logcli.py` verified unchanged and smoke-run on all three verbs; no CLI test added, and the absence of CLI test coverage noted as pre-existing.
- Any deviation between the card and what I found — if the ruling in Phase 2 came back "yes" and I did perform the extraction, that would be recorded here as a scope deviation with the approval that permitted it.