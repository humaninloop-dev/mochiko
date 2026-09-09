FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-6h_cw5ag/ws/CARD.md`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-6h_cw5ag/ws/textkit.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-6h_cw5ag/ws/test_textkit.py`

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orient and confirm the card's premises

**Read:** already done — `CARD.md`, `textkit.py` (45 lines, four stubs, `import re` only), `test_textkit.py` (125 lines, 4 test classes).

**Checks I do by hand, not by delegation:**

- Test count reconciliation against the card's claim of 29: `ParseDurationTest` 7 + `TokenizeQueryTest` 8 + `RenderTableTest` 6 + `MergeRangesTest` 8 = **29**. ✅ Matches. This is completeness-sensitive, so I count it myself rather than farming it out.
- Workspace inventory: the directory holds exactly `CARD.md`, `textkit.py`, `test_textkit.py`. Nothing imports `textkit`, matching the card's "brownfield exposure: none". No brownfield reading procedure is needed; I would *not* pull in the brownfield approach for this card and would say so in the report.
- No `tasks.md` exists in this workspace. **Flag:** the checkbox I am to flip lives in `CARD.md` line 3 (`- [ ] **C3 · textkit helpers**`). I proceed against `CARD.md` and note the substitution in the report rather than inventing a `tasks.md`.

**Delegation decision for this phase:** none. A cheap locate/enumerate worker (haiku Explore) would be the right tool for "does anything else import textkit" in a large repo, but the whole workspace is three files I have already read — spawning would cost more than it saves. I record this as a deliberate non-delegation.

## Phase 1 — Pre-code check (before writing any body)

The cheapest-thing-that-works ladder, run per function, with the card's `re`-only / stdlib-only / no-new-public-names constraint binding:

- **Delete / don't write?** No — four stubs with 29 tests pinning them; the code has to exist.
- **Reuse in-repo?** Nothing exists to reuse; `textkit.py` is stubs only.
- **Stdlib instead of hand-rolled?**
  - `parse_duration` — `re` does the shape check and the pair walk. No hand-written scanner.
  - `tokenize_query` — I considered `shlex.split` (posix mode would even produce `foobar bazqux` for `foo"bar baz"qux` and raise on an unterminated quote). **Rejected**, and I will say why in the report: the card fixes `re` as the only import, and `shlex` has no notion of a leading `-` negation nor of "bare `-` is an error", so I'd be post-processing its output and re-implementing the interesting half anyway. Hand-written single pass stands.
  - `render_table` — reuse `str.ljust` / `str.rjust` / `str.rstrip` / `"\n".join` rather than manual padding arithmetic.
  - `merge_ranges` — reuse built-in `sorted`; the sweep is four lines.
- **New public surface?** None. Only the four names stay public; any shared helper is `_`-prefixed and module-private (I expect exactly one: a row formatter for `render_table`).

## Phase 2 — Confirm the red state myself

**Run:** `python3 -m unittest -q test_textkit`

**Expect:** `Ran 29 tests`, `FAILED (errors=29)`, every entry a `NotImplementedError` raised from `textkit.py`. The card asserts this; I verify it rather than take it on trust, because a green or import-broken suite here would mean the card and the workspace disagree.

**Branches at this stop:**
- All 29 are `NotImplementedError` → proceed to Phase 3.
- Count differs but every error is still `NotImplementedError` → proceed, and record the true count as a card discrepancy in the report (the card's `TEST` line asserts 29, so the final assertion would need to be read against the real number).
- Any collection error, `ImportError`, or a failure that is *not* `NotImplementedError` → **stop**. That means the red phase was not what the card says it was. I report the exact output and do not start writing bodies against an unverified baseline. Under my stated default I would resume only after the baseline is explained; if it turned out to be an environment problem (e.g. no `python3` on PATH) I'd report that as a blocker rather than working around it with a different runner.

## Phase 3 — `parse_duration` (green)

**Write:** body of `parse_duration` in `/…/ws/textkit.py`.

Approach exactly as the card fixes it:
1. Normalise: strip every whitespace character out with `re.sub(r"\s+", "", text)` and lowercase — this covers `"  1H 30M "` and the tab-free case-insensitivity requirement in one step.
2. Shape gate: `re.fullmatch(r"(\d+[dhms])+", normalised)`; no match → `ValueError`. This alone rejects `""`, `"   "` (normalises to `""`), `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"` (normalises to `"1hm"`, whose trailing `m` has no digits).
3. Walk `re.findall(r"(\d+)([dhms])", normalised)`, accumulate `int(value) * {"d": 86400, "h": 3600, "m": 60, "s": 1}[unit]`, tracking a `seen` set; a unit already in `seen` → `ValueError`. This is the only thing that catches `"1h1h"`.
4. Return the integer total.

The multiplier table is a module-private constant (`_DURATION_UNITS`), not a new public name.

**Run:** `python3 -m unittest -q test_textkit.ParseDurationTest` → expect `Ran 7 tests`, `OK`. Then the full suite → expect 22 errors remaining, all `NotImplementedError`.

## Phase 4 — `tokenize_query` (green)

**Write:** body of `tokenize_query` in `textkit.py`.

Single left-to-right character pass with three pieces of state: `buf` (list of chars), `negated`, `in_quotes`, plus a `started` flag that marks "we are inside a token" so a `-` is only a negation marker at token start.

- In quotes: `"` closes; anything else (whitespace included) goes into `buf` — this keeps `"multi  space"` intact.
- Outside quotes: `"` opens quote mode and marks the token started (so `foo"bar baz"qux` accumulates into one buffer → `foobar bazqux`); whitespace flushes; `-` at token start sets `negated`; everything else, `-` included mid-token, is a literal character (`a-b`, `lang:rust`).
- Flush = if `buf` non-empty, append `("".join(buf), negated)`; **else if `negated`, raise `ValueError`** (this is the bare-`-` rule that makes `"rust -"` and `"-"` fail); then reset all three pieces of state. An empty buffer with no negation flushes to nothing, which is what makes `'""'`, `""` and `"   "` all return `[]`.
- After the loop: `in_quotes` still true → `ValueError` (`'"unterminated'`); then a final flush.

**Run:** `python3 -m unittest -q test_textkit.TokenizeQueryTest` → expect `Ran 8 tests`, `OK`. I would pay particular attention to `test_quotes_toggle_mid_token` and `test_empty_inputs`, the two that punish a design where quoting starts a fresh token.

## Phase 5 — `render_table` (green)

**Write:** body of `render_table` plus one module-private row formatter in `textkit.py`.

1. Validate first: falsy `headers` → `ValueError`; any row whose length differs from `len(headers)` → `ValueError`. (`render_table([], [])` is caught by the empty-headers check.)
2. Per column: `right = all(isinstance(row[i], int) for row in rows)`; width = max of `len(str(cell))` over that column's cells and `len(headers[i])`.
3. `_format_row(cells, widths, rights)` → `"  ".join(str(c).rjust(w) if r else str(c).ljust(w) …)`, then `.rstrip()`.
4. Emit: header line through the same formatter (so a numeric column right-aligns its header, per `test_single_numeric_column_right_aligns_header` expecting `" n"`), then the rule line `"-" * width` per column joined by two spaces, then one line per row. `"\n".join(lines)`, no trailing newline.

Hand-verification I do before running, against the four exact-string tests:
- `[["apple",3],["kiwi",12]]` / `["name","qty"]` → widths 5, 3; col1 right → `"name   qty"` / `"-----  ---"` / `"apple    3"` / `"kiwi    12"`. Matches.
- `[]` / `["name","qty"]` → widths 4, 3 → `"name  qty"` / `"----  ---"`. Matches. (With no rows the `all(...)` over an empty column is vacuously true, so col1 counts as right-aligned — harmless here because header width equals column width; I note this as an unpinned-by-tests behaviour rather than adding code to special-case it.)
- `[["a",1],["b","n/a"]]` / `["k","v"]` → col1 mixed → left, width 3 → `"k  v"` (after rstrip) / `"-  ---"` / `"a  1"` / `"b  n/a"`. Matches.
- `[[1],[22]]` / `["n"]` → width 2, right → `" n"` / `"--"` / `" 1"` / `"22"`. Matches.

**Flag, no action:** `bool` is a subclass of `int`, so a column of `True`/`False` would right-align. No test pins it and the card says `int`; I leave `isinstance` as-is and record the decision rather than writing unrequested type-narrowing.

**Run:** `python3 -m unittest -q test_textkit.RenderTableTest` → expect `Ran 6 tests`, `OK`.

## Phase 6 — `merge_ranges` (green)

**Write:** body of `merge_ranges` in `textkit.py`.

1. Validate every entry: unpack to `start, end`; both `isinstance(..., int)` else `ValueError` (catches `(1, "2")`); `start > end` → `ValueError` (catches `(3, 1)`).
2. `sorted(pairs)` — sorts by start, then end.
3. One sweep: hold `(cur_start, cur_end)`; for each next pair, if `nxt_start <= cur_end + 1` then `cur_end = max(cur_end, nxt_end)`, else close out and open a new range. The `+ 1` is what merges `(1,3)+(4,6)` and `(5,5)+(6,6)` while leaving `(1,3)+(5,6)` split.
4. Return a list of tuples; `[]` in → `[]` out (the sweep never starts).

**Run:** `python3 -m unittest -q test_textkit.MergeRangesTest` → expect `Ran 8 tests`, `OK`.

## Phase 7 — Refactor

With all 29 green, tidy without changing behaviour:

- Keep the four docstrings exactly as stubbed — they are the written spec and the card says signatures and layout stand.
- **Amend the module docstring** (`textkit.py` lines 3–4): "Each body below is a stub…" is false once implemented. I replace those two lines with a one-line note that the tests in `test_textkit.py` are the behaviour spec. This is a two-line truthfulness fix inside the file I'm already implementing, and I disclose it in the report rather than doing it silently. If the reviewer's ruling is that even this is out of scope, the branch is trivial: revert those two lines, nothing else depends on them.
- Confirm the only new module-level names are `_DURATION_UNITS` and `_format_row` (both private); public surface remains the four functions.
- Confirm `re` is still the only import and no `re` import was left unused (it is used by `parse_duration`).
- **Re-run the full suite after refactoring** — `python3 -m unittest -q test_textkit` → expect `Ran 29 tests`, `OK`.

I would **refuse** any tempting extras here: no type hints, no `__all__`, no CLI entry point, no input-coercion for `str`-typed range bounds, no caching. The card asks for four bodies.

## Phase 8 — Final verification (the card's TEST line)

**Run exactly:** `python3 -m unittest -q test_textkit`
**Assert:** output contains `Ran 29 tests` and `OK`, exit code 0. I capture this output verbatim for the report.

If anything is red at this point, I do not patch the test file (`test_textkit.py` is explicitly not editable) — I fix `textkit.py`, or if a test looks unsatisfiable under the card's constraints I stop and report the contradiction with the specific test named.

## Phase 9 — Bookkeeping and report

**Write:**
- `/…/ws/CARD.md` — flip line 3 from `- [ ]` to `- [x]`. Nothing else in the card changes.
- `/…/ws/cycle-report.md` — the honest record of this cycle.

**What the report says:**
- Card C3, one card, four functions, 29 tests, final state `OK` with the verbatim command and output.
- The red baseline I observed myself in Phase 2 and how it compared to the card's claim.
- The pre-code ladder result per function, including the `shlex` option I considered for `tokenize_query` and the two reasons I rejected it (import constraint; it doesn't carry negation or the bare-`-` error).
- Reuse actually taken: `re` for duration parsing, `str.ljust`/`rjust`/`rstrip` and `"\n".join` for the table, built-in `sorted` for range merging.
- New private names introduced (`_DURATION_UNITS`, `_format_row`) and confirmation the public surface is still the four functions.
- Deviations and flags, plainly: no `tasks.md` in this workspace so the checkbox was flipped in `CARD.md`; the module-docstring stub sentence was corrected; `bool`-as-`int` in `render_table`/`merge_ranges` and the vacuous right-alignment of an empty-rows column are unpinned by tests and left as-is by choice.
- Delegation section: **none**. No haiku Explore (the workspace is three files, all already read by me), no sonnet worker for any green phase. The bodies here are small but each has an edge case that the tests punish precisely — `foo"bar baz"qux` staying one token, `'""'` returning `[]`, the header participating in right-alignment, `+1` adjacency — so I keep them. Recorded as a decision, not an omission.