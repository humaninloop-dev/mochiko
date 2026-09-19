FILES-READ: `CARD.md`, `logparse.py`, `test_logparse.py`, `logcli.py`

---

# Action plan — cycle card C4, `errors_by_hour(lines)`

## Phase 0 — Read the whole surface before writing anything (done)

I read all four files in full, not just the spans the card names, because C4 is an extension of an existing module with an established aggregator shape and I need to match it rather than invent one.

What the reading established:

- `parse_line(line)` returns a **2-tuple** `(level, message)` and deliberately discards the timestamp — its docstring says so explicitly ("checked for presence and then dropped — nothing downstream has needed it so far").
- The module-level regex `_LINE` already captures the timestamp as a named group `ts`; the capture exists, it's just thrown away.
- Both existing aggregators (`count_by_level`, `errors_only`) share one skip idiom: `try: parse_line(...) except ParseError: continue`. That single idiom covers *both* skip rules the card names — malformed lines and unknown levels — because `parse_line` raises `ParseError` for each.
- Dicts preserve insertion order on every Python 3.7+, so "first-seen order" needs no `OrderedDict` and no sorting.
- `logcli.py` unpacks `level, message = parse_line(line)` in one place and imports three names; `test_logparse.py` asserts the exact 2-tuple in two tests.

**Delegation decision:** none. The entire workspace is four small files totalling well under 200 lines and I have already read each of them end to end. Spawning a cheap reader here would cost more than it saves and would give me second-hand facts about code I need first-hand judgement on. If this workspace had been a large tree I'd have sent a cheap `Explore` (model `haiku`) to locate every caller of `parse_line`; here I can see there is exactly one outside the tests.

## Phase 1 — Cheapest-thing-that-works check, before a single test is written

The card asks for a bucket key drawn from the timestamp, and `parse_line` throws the timestamp away. That's the whole design question in this card, so I settle it first.

Options, cheapest first:

1. **Nothing existing already does this.** No stdlib or dependency shortcut applies — the bucketing is a slice of a string field, and there is no third-party anything in this workspace. Nothing to reuse wholesale.
2. **Change `parse_line` to return `(ts, level, message)`.** — **Refused.** The card marks `logparse.py` `[EXTEND]`, not `[MODIFY]`, and it explicitly says `logcli.py` must keep working unchanged and every existing test must still pass. A 3-tuple breaks the two tuple-equality tests and the unpack in `logcli.py`'s `show` verb. I will not do this, and I'll say so in the report rather than silently doing it.
3. **Add an optional flag or a second return mode to `parse_line`.** — Rejected. It's an interface change in disguise, and it's more machinery than the card asks for.
4. **Extract a shared private helper (`_match_line`) used by both `parse_line` and the new function.** — Viable and touches no public interface, but it edits the body of working, tested code for one new caller. I hold this as the fallback, not the first move.
5. **Have `errors_by_hour` call `parse_line` for the skip rules, then read the already-captured `ts` group off the existing `_LINE` regex for lines that survived.** — **Chosen.** It adds one function and changes zero existing lines. It reuses the module's own regex and the module's own skip semantics, so the two skip rules can't drift from the other aggregators. Its only cost is matching the regex a second time on lines that already parsed — negligible for this workload, and the second match is guaranteed to succeed precisely because the first one did.

I will disclose the double-match trade openly in the report rather than quietly paying it.

**Stop point (would confirm with a human, would not block on it):** whether option 4 — the shared helper — is preferred over the small redundancy of option 5. If the ruling were "prefer the helper," I'd move the `_LINE.match` + `None` check into `_match_line(line)`, have `parse_line` call it and keep its exact signature, message text, and 2-tuple return, and have `errors_by_hour` call it too; the six existing tests are the guard that this stayed behaviour-preserving. If the ruling were "no edits to working code at all" (the stricter reading of `[EXTEND]`), option 5 stands. **My default is option 5**, because it is the one that cannot regress `parse_line`, and I proceed on it.

## Phase 2 — Red: write the failing test first, in `test_logparse.py`

**Write to:** `/…/ws/test_logparse.py` (extend only — I do not touch the existing `SAMPLE` list or any existing test, since two of them assert against `SAMPLE` and the card requires all six to keep passing).

Changes:

- Add `errors_by_hour` to the existing `from logparse import …` line.
- Add a second module-level fixture beside `SAMPLE` (a new name, e.g. `HOURS`) rather than editing `SAMPLE`. It deliberately contains: an error in a later hour appearing *before* an error in an earlier hour (so first-seen order is distinguishable from sorted order), two errors sharing one hour (so I'm testing a count, not a presence flag), an hour containing only `INFO`/`WARN` lines and no error, an unknown-level line (`TRACE`), and a malformed line.
- Add three tests to `AggregatorTest`, matching the naming and assertion style already used there:
  1. `test_errors_by_hour_counts_errors_per_hour` — against the existing `SAMPLE`, expect `{"2026-09-01T14": 1, "2026-09-01T15": 1}`. This reuses the fixture the other aggregator tests use and confirms the 13-character slice lands on `YYYY-MM-DDTHH`.
  2. `test_errors_by_hour_buckets_in_first_seen_order` — against `HOURS`, assert on `list(result.items())` (not just the dict) so ordering is genuinely asserted and not incidentally satisfied by equality; includes the hour with a count of 2.
  3. `test_errors_by_hour_omits_hours_without_errors_and_skips_bad_lines` — against `HOURS`, assert the error-free hour's key and the unknown-level line's hour are both absent, and that the malformed line didn't blow up. Plus `errors_by_hour([]) == {}`.

**Run:** `python3 -m unittest -q test_logparse`

**What I expect it to show:** an `ImportError` — `cannot import name 'errors_by_hour' from 'logparse'` — which aborts the whole module and reports an error, not three clean failures. I want that specific message; it is the right reason to fail (the function genuinely does not exist). I'll note in the report that at this instant the six existing tests are masked by the import error, which is expected and is resolved the moment the function exists. If I saw anything else — a pass, a syntax error, a failure naming something other than `errors_by_hour` — I'd stop and fix the test before writing any implementation.

## Phase 3 — Green: the smallest implementation in `logparse.py`

**Write to:** `/…/ws/logparse.py` — append one function after `errors_only`. No edits above it; `LEVELS`, `_LINE`, `ParseError`, `parse_line`, `count_by_level`, and `errors_only` are left byte-for-byte as they are.

Shape (matching the surrounding style — docstring naming the skip behaviour, same loop-and-`continue` idiom):

```python
def errors_by_hour(lines):
    """Map each hour bucket (`YYYY-MM-DDTHH`) to its ERROR count, in first-seen order.

    Hours with no errors are absent. Malformed lines and unknown levels are skipped.
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

Notes on the choices, all of which I'd defend in the report:

- `parse_line` is the gate, so the two skip rules are the module's, not a private copy.
- The re-match is unguarded on purpose: it cannot return `None`, because `parse_line` on the identical `rstrip`'d string already matched. Adding a defensive `if match is None` would be dead code.
- A plain dict with `counts.get(...)` rather than `collections.Counter` or `defaultdict`: both would import machinery to save one clause, and `Counter`'s repr/type would leak into any future caller comparison. The card asks for a dict.
- No sorting, no `OrderedDict` — insertion order is the required order and the language already gives it.
- The `[:13]` slice is exactly what the card specifies. I do **not** parse the timestamp with `datetime`; the card states timestamps are always the ISO form, and reaching for `datetime.fromisoformat` would add a failure mode (and a trailing-`Z` compatibility wrinkle) the card didn't ask me to handle.

**Run:** `python3 -m unittest -q test_logparse`

**What I expect:** `OK` with 9 tests — the 6 existing plus my 3. If the ordering test fails, my suspicion is my expected-pairs list, not the implementation, and I'd re-derive it from the fixture by hand before touching `logparse.py`.

## Phase 4 — Refactor, and confirm nothing else moved

- Re-read my added function against `count_by_level`/`errors_only` for consistency of naming, docstring voice, and loop shape. I expect no changes; if the double-match reads badly on a second look I'd revisit option 4 from Phase 1 and re-run the full suite as the safety net.
- Confirm `logcli.py` was never opened for edit. It still imports three names and unpacks a 2-tuple, all of which still exist unchanged. C4 does not wire the `hourly` verb, and I will not add it — the card says a later card does that, and adding it now would be scope I wasn't given.
- Re-run `python3 -m unittest -q test_logparse` and expect `OK`. As a cheap extra I'd run `python3 -c "import logcli"` to confirm the CLI's imports still resolve; this is one command, not a new test file, and I would not add CLI tests this card.

**Would refuse / would flag along the way:**
- Refuse any change to `parse_line`'s signature or return type (breaks two tests and the `show` verb).
- Refuse to add the `hourly` verb, an `--hour` filter, timestamp validation, or a `hours_by_level` generalisation — none are in this card.
- Refuse to edit or extend the shared `SAMPLE` fixture, since existing assertions are pinned to it.
- Flag, not fix: `parse_line`'s docstring line "nothing downstream has needed it so far" is now stale — something downstream does need the timestamp. It's one comment in existing code that the card didn't scope to me; I note it for the next card rather than editing it.

## Phase 5 — Close the card and report

- **Tick the checkbox.** The card is `- [ ]` on line 3 of `CARD.md`. **Discrepancy to flag:** there is no `tasks.md` in this workspace — the card lives in `CARD.md`. **My default:** flip line 3 of `CARD.md` to `- [x]` and say plainly in the report that I did that in the absence of a `tasks.md`. If told a `tasks.md` should exist elsewhere, I'd revert `CARD.md` and tick there instead; if told to leave `CARD.md` untouched, I'd leave the checkbox and record the completion in the report only.
- **Write `cycle-report.md`** at the workspace root, beside `CARD.md` (flagging that path as my assumption, since there's no docs directory to suggest another home).

What the report will actually say, honestly:

- What I built: one new aggregator, `errors_by_hour`, plus three tests. No existing line of `logparse.py`, `test_logparse.py`, or `logcli.py` was changed apart from the test file's import line and its new fixture/tests.
- The real difficulty of this card: `parse_line` throws away the exact field the card needs. I name the four options I weighed, why the 3-tuple change was refused (`[EXTEND]` not `[MODIFY]`; it breaks two tests and the `show` verb), and why I took the reuse-the-regex route.
- The trade I accepted: `_LINE` is matched twice per surviving line. Cheap, safe, and it keeps the skip rules single-sourced — but it *is* redundant work and I'm not hiding it. The shared-helper alternative is described so a reviewer can overrule me in one step.
- Red evidence: the first run failed with `ImportError: cannot import name 'errors_by_hour'`, which also masked the six existing tests for that one run; green run showed `OK` with all 9.
- The stale docstring sentence in `parse_line`, noted and deliberately not fixed.
- The `tasks.md`/`CARD.md` checkbox substitution.
- Explicitly out of scope and left undone: the `hourly` CLI verb.