FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

# Action plan — cycle card C3 (textkit helpers)

The card is a fill-in-the-bodies job: four stubs, 29 tests already red for the right reason (`NotImplementedError`), approach fixed per function, `test_textkit.py` off-limits. No design work, no delegation, no subagents — this is one ~120-line file that I would write in a single pass and verify with the suite. Below is what I would actually write, derived cell-by-cell from the tests.

## Phase 1 — Read and pin the spec (done above)

- Read `CARD.md`, `textkit.py`, `test_textkit.py`. Nothing else exists in the workspace; nothing imports `textkit` yet, so there is no brownfield caller to keep compatible.
- Confirm test count matches the card: ParseDuration 7 + TokenizeQuery 8 + RenderTable 6 + MergeRanges 8 = **29**. It does.
- Constraints I would hold myself to: stdlib only, `re` stays the sole import, no class, no CLI, only `parse_duration` / `tokenize_query` / `render_table` / `merge_ranges` as module-level public names (helpers stay local to their function or underscore-prefixed), and `test_textkit.py` is not touched.

## Phase 2 — `parse_duration`

Write into `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-jr9t_mvd/ws/textkit.py`, replacing the stub body, keeping the docstring:

- Normalise: strip all whitespace with `re.sub(r"\s+", "", text)` then lowercase. This makes `"  1H 30M "` → `"1h30m"` and `"1h m"` → `"1hm"`.
- Shape check: `re.fullmatch(r"(?:\d+[dhms])+", normalised)`; no match → `ValueError` with the original text in the message. This alone rejects `""`, `"   "` (both normalise to empty, and `+` needs one repeat), `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1hm"`.
- Walk pairs: `for number, unit in re.findall(r"(\d+)([dhms])", normalised)` with a `seen` set; a unit already in `seen` → `ValueError` (this is what rejects `"1h1h"`). Multiply by a module-private `_UNIT_SECONDS = {"d": 86400, "h": 3600, "m": 60, "s": 1}` and accumulate.
- `"0s"` → 0 falls out naturally; the function returns an `int` because every term is `int * int`.

Expected effect on the suite: the 7 `ParseDurationTest` tests go green.

## Phase 3 — `tokenize_query`

Single left-to-right character pass, exactly as the card fixes it:

- State: `tokens = []`, `buffer = []`, `in_quotes = False`, `negated = False`.
- A local `flush()`: if the buffer is empty and `negated` is set → `ValueError` (bare `-`); if the buffer is empty and not negated → do nothing (this is what makes `'""'` and all-whitespace input return `[]`); otherwise append `("".join(buffer), negated)` and reset both buffer and `negated`.
- Per character, in this order: `"` toggles `in_quotes` and is dropped; if `in_quotes`, append the character verbatim (keeps `"multi  space"` intact); whitespace calls `flush()`; a `-` when the buffer is empty and `negated` is not yet set turns negation on; anything else appends.
- After the loop: `in_quotes` still true → `ValueError` (unterminated quote); then a final `flush()`.

Why this satisfies the trickier cases: `'foo"bar baz"qux'` never flushes because the quote toggle only affects the flag, so the buffer accumulates to `"foobar bazqux"`; `"lang:rust a-b"` keeps `:` and the interior `-` literal because the `-` branch requires an empty buffer; `"rust -"` and `"-"` both reach the final flush with an empty buffer and `negated` true, raising.

One behaviour the tests do not pin: `-""` (negation followed by an empty phrase). My implementation raises `ValueError` there, consistent with the bare-`-` rule. I would note it in the final report rather than invent a special case.

Expected effect: the 8 `TokenizeQueryTest` tests go green.

## Phase 4 — `render_table`

Validate first, measure second, format last:

- Validation: `headers` empty → `ValueError`; any row whose length differs from `len(headers)` → `ValueError` naming the offending index. This covers both assertions in `test_rejects_ragged_rows_and_empty_headers`.
- Per column `i`: `right = all(isinstance(row[i], int) for row in rows)` — note this is vacuously true when `rows` is empty, which is harmless because the header line is right-stripped and the widths are identical either way (`render_table([], ["name","qty"])` gives `"name  qty\n----  ---"` under either alignment).
- Cell text is `str(cell)`; width is the max of the header length and all cell lengths in that column.
- One local row formatter `format_row(cells)` joins `cell.rjust(w)` or `cell.ljust(w)` with a two-space separator and returns `line.rstrip()`. Header, rule (`"-" * width` per column, put through the same joiner), and every data row go through it, then `"\n".join(...)` with no trailing newline.

Hand-checked against each expected string: `"name   qty"` / `"-----  ---"` / `"apple    3"` / `"kiwi    12"` (widths 5 and 3, col 1 right); `"k  v"` / `"-  ---"` / `"a  1"` / `"b  n/a"` (col 1 mixed → left, so the header and `"1"` are padded and then stripped); `" n"` / `"--"` / `" 1"` / `"22"` (single all-int column right-aligns the header too).

Note I would flag rather than "fix": `isinstance(True, int)` is true in Python, so a column of booleans would right-align. Nothing in the tests or the card mentions bools, so I would not add a special case, just mention it.

Expected effect: the 6 `RenderTableTest` tests go green.

## Phase 5 — `merge_ranges`

- Validation loop first, before any comparison: each item must unpack to exactly two values; both bounds must satisfy `isinstance(x, int)`; `start > end` → `ValueError`. Doing the type check before the sort matters — `[(1, "2")]` must raise `ValueError`, not let a `str`/`int` comparison escape as `TypeError`.
- `sorted(pairs)` (by start, then end), then one sweep: hold an open `(start, end)`; for each next pair, if `next_start <= open_end + 1`, set `open_end = max(open_end, next_end)`; otherwise emit the open range and open a new one. Emit the last at the end.
- `[]` returns `[]` early. Results are tuples in a list, matching the assertions.

Hand-checked: `[(1,3),(2,6),(8,10)]` → `[(1,6),(8,10)]`; `[(1,3),(4,6)]` merges on the `+1` touch rule; `[(1,3),(5,6)]` stays split (gap of one); `[(1,10),(2,3),(1,10)]` collapses to `[(1,10)]` via the `max`; `[(5,5),(6,6)]` → `[(5,6)]`.

Expected effect: the 8 `MergeRangesTest` tests go green.

## Phase 6 — Run the suite and confirm

- Run exactly what the card names: `python3 -m unittest -q test_textkit` from the workspace root.
- Expected output: `OK`, 29 tests, 0 failures, 0 errors — the card's TEST line.
- If anything fails, I fix `textkit.py`, never the test. I would also re-read the failing assertion literally (these are exact-string comparisons for the table) rather than guessing at the padding.
- Sanity checks beyond the suite: confirm `import re` is still the only import, confirm no module-level name was added besides the four functions and the private `_UNIT_SECONDS` map, and confirm `test_textkit.py` is byte-identical to what I read.

## Where I would stop for a human

Nothing here is genuinely blocking, so I would not stop under normal conditions. The one scenario that would justify stopping: if a test turned out to be unsatisfiable or two tests contradicted each other, so that the only way to green is editing `test_textkit.py`. That crosses an explicit constraint, so I would stop, show the two conflicting assertions, and ask which is authoritative. Branches: if the ruling is "the test is wrong, fix it," I would make the minimal test edit and say so prominently in the report; if the ruling is "the test is right, the card's approach is wrong," I would change the approach in `textkit.py` and note the divergence from the card's prescribed method. Default while unanswered: implement to satisfy the tests as written, since the card states the tests are the spec. Having hand-traced all 29 assertions against the implementations above, I do not expect to hit this.

## Delegation

None. One file, four short pure functions, a fixed approach, and a fast local suite — spinning up any worker would cost more than it returns, and I cannot dispatch one in this run anyway.

## What I would report at the end

- The card item ticked, with the suite result quoted verbatim (`OK`, 29 tests).
- One line per function on how it was implemented, matching the card's prescribed approach.
- The two under-specified spots I resolved by judgment and would want a second opinion on if they ever matter: `-""` raising `ValueError`, and `bool` counting as `int` for right-alignment.
- Confirmation that `test_textkit.py` is unmodified and `re` remains the only import.