FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py` (plus a directory listing confirming those three are the whole workspace — no CLAUDE.md, no config, no other modules)

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Confirm the starting state
- Re-read is done: the card, the four stubs in `textkit.py`, and all 29 tests in `test_textkit.py` (7 duration + 8 tokenizer + 6 table + 8 ranges = 29, matching the card's count).
- Run `python3 -m unittest -q test_textkit` once as a baseline. Expect 29 errors, every one a `NotImplementedError` raised from a stub. If instead I see an import error, a different count, or a failure that isn't `NotImplementedError`, I stop and report that the workspace doesn't match the card before writing any code.
- No delegation for any phase. This is one ~120-line pure-stdlib module with a complete test spec in front of me; splitting it across workers would cost more than it saves and would risk four people re-deriving the same shared row formatter. I do it directly.
- Nothing here is destructive or outward-facing; `textkit.py` contains only stubs and nothing imports it, so there is no caller to break. No confirmation gate is needed before editing it. I will not touch `test_textkit.py` — the card forbids it and the tests are the spec.

## Phase 1 — `parse_duration(text)`
Edit `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-m67gimkz/ws/textkit.py`, replacing the `raise NotImplementedError` in `parse_duration` with, per the card's fixed approach:
1. Normalise: strip every whitespace run out of the text and lowercase it (`re.sub(r"\s+", "", text).lower()`). This covers `"  1H 30M "` → `"1h30m"`.
2. Shape check with one `re.fullmatch(r"(\d+[dhms])+", norm)`; no match → `ValueError`. This is what rejects `""`, `"   "` (normalises to empty), `"90"`, `"1x"`, `"1.5h"` (the `.` isn't in the alphabet), `"-5s"`, `"h"`, and `"1h m"` (normalises to `"1hm"`, and `m` has no digits).
3. Walk `re.findall(r"(\d+)([dhms])", norm)`, keeping a `seen` set of units; a unit already in `seen` → `ValueError`. This is the only thing that rejects `"1h1h"`, which passes the shape check.
4. Sum `int(number) * {"d": 86400, "h": 3600, "m": 60, "s": 1}[unit]` and return it.

Expected against the tests: `"90s"`→90, `"1h30m"`→5400, `"1d2h3m4s"`→93784, `"30m1h"`→5400 (order-free because the units are a set, not a sequence), `"0s"`→0, and all nine malformed inputs raise.

## Phase 2 — `tokenize_query(text)`
Single left-to-right character pass with three pieces of state: `buf` (list of chars), `in_quotes` (bool), `negated` (bool). A local `flush()` closure:
- if `buf` is non-empty → append `("".join(buf), negated)` to the output;
- elif `negated` → `ValueError` (this is the bare-dash rejection);
- then reset `buf` and `negated`.

Per character:
- `"` → toggle `in_quotes`, append nothing (quote is dropped). Because the buffer is *not* flushed here, `'foo"bar baz"qux'` accumulates into the single token `"foobar bazqux"`, which is exactly what `test_quotes_toggle_mid_token` pins.
- whitespace (`ch.isspace()`, so tabs count) and not `in_quotes` → `flush()`. Inside quotes it's an ordinary character, so `'"multi  space"'` keeps its two inner spaces.
- `-` with an empty `buf`, `negated` still False, and not `in_quotes` → set `negated = True`, consume it. Anywhere else the `-` is a literal, which is what keeps `"a-b"` a single un-negated token.
- anything else → append to `buf`. `:` gets no special treatment, so `"lang:rust"` stays one literal token.

After the loop: if `in_quotes` → `ValueError` (unterminated quote), checked *before* the final `flush()`. Then `flush()` and return the list.

Expected: `"rust cargo"` and `"  rust \t cargo  "` both → `[("rust", False), ("cargo", False)]`; `'-"cargo build" rust'` → `[("cargo build", True), ("rust", False)]`; `""`, `"   "`, and `'""'` all → `[]` (empty buffer with no pending negation flushes nothing); `'"unterminated'`, `"rust -"`, and `"-"` all raise.

Judgment call I make myself and will note in the report: `'-""'` (a negated empty phrase) is untested and lands on `ValueError` under this rule. I'm leaving it there rather than inventing a carve-out the spec doesn't ask for.

## Phase 3 — `render_table(rows, headers)`
Order matters here — validate, then measure, then format through one shared formatter.
1. Validate: empty `headers` → `ValueError`; any row whose length differs from `len(headers)` → `ValueError`. Both branches of `test_rejects_ragged_rows_and_empty_headers` are covered, and the empty-headers check runs first so `render_table([], [])` raises rather than returning `""`.
2. Per column: `width = max(len(str(cell)) for that column's cells and its header)`; `right = all(isinstance(cell, int) for cell in that column's cells)` — vacuously True when there are no rows, which is harmless because with no rows the only line affected is the header and it is padded to at least its own length.
3. Private helper `_format_row(cells, widths, rights)` inside `textkit.py` (the card permits private helpers): pad each `str(cell)` with `rjust` or `ljust` per that column's flag, join with two spaces, and `rstrip()` the result. Use it for the header row and every data row.
4. Rule line: `"-" * width` per column joined by two spaces (the rule is built from the same widths, so it tracks them automatically).
5. Join header + rule + rows with `"\n"`, no trailing newline.

Expected, checked by hand against each assertion: `[["apple",3],["kiwi",12]]/["name","qty"]` → widths 5 and 3, col 1 right-aligned including its header → `"name   qty\n-----  ---\napple    3\nkiwi    12"`. `[]/["name","qty"]` → `"name  qty\n----  ---"`. `[["a",1],["b","n/a"]]/["k","v"]` → col 1 mixed so left-aligned, and the header/first row lines right-strip down to `"k  v"` / `"a  1"`. `[[1],[22]]/["n"]` → width 2, right-aligned → `" n\n--\n 1\n22"`; note `rstrip()` (not `strip()`) so the leading pad survives. `test_no_trailing_whitespace` passes because every line goes through the same right-stripping formatter and the join adds no tail.

Judgment call I make myself and will note: `isinstance(cell, int)` treats `True`/`False` as numeric, since `bool` subclasses `int`. No test exercises it and the card says "all `int`", so I take the plain reading rather than adding an unrequested `bool` exclusion.

## Phase 4 — `merge_ranges(ranges)`
1. Validate every item first, before any merging: unpack a two-element pair (a wrong-arity item → `ValueError`), require `isinstance(start, int)` and `isinstance(end, int)` (this is what rejects `(1, "2")`), and require `start <= end` (rejects `(3, 1)`). Validating up front means a bad pair raises even if an earlier pair would have merged fine.
2. `sorted(pairs)` — safe because validation has already guaranteed both bounds are ints, so no `str`/`int` comparison can blow up.
3. One sweep holding `(open_start, open_end)`: if the next `start <= open_end + 1`, extend with `open_end = max(open_end, end)`; otherwise emit the open range and open a new one. Emit the last one at the end. Return a list of tuples.

Expected: overlapping `[(1,3),(2,6),(8,10)]`→`[(1,6),(8,10)]`; adjacent `[(1,3),(4,6)]`→`[(1,6)]` (the `+1`); unsorted `[(8,10),(1,3)]`→`[(1,3),(8,10)]`; contained/duplicate `[(1,10),(2,3),(1,10)]`→`[(1,10)]` (the `max` keeps the open end from shrinking); gap of one `[(1,3),(5,6)]` stays split (5 > 4); `[(5,5),(6,6)]`→`[(5,6)]`; `[]`→`[]` (early return before the sweep).

## Phase 5 — Verify
- Run `python3 -m unittest -q test_textkit`. Expected: `OK`, 29 tests, no failures or errors — the card's TEST line exactly.
- If anything fails, I fix `textkit.py`, never the test. A test that looks "wrong" is the spec disagreeing with my implementation; the card is explicit that the tests are the behaviour spec and are not to be edited. If I ever concluded a test was genuinely self-contradictory, that is where I stop and bring it to you rather than editing it — the branch would be: you say the test is right → I rework the implementation; you say the test is wrong → I still don't touch it without you saying so explicitly, and I'd ask whether to amend the card. I don't expect to reach this; I've hand-traced all 29 assertions above and they're consistent.
- Re-read the final `textkit.py` to confirm the constraints hold: `re` is still the only import (and still used, by `parse_duration`), no class, no CLI/`__main__` block, and the only module-level public names are the four functions plus any `_`-prefixed helper (`_format_row`).
- Leave the docstrings as written — they already describe the implemented behaviour.

## Phase 6 — Report
I'd report: the four bodies implemented in `textkit.py` with a one-line note on each approach; the suite result verbatim (`OK`, 29 tests) or the actual failures if it isn't green; the two untested edge decisions I made on my own (`bool` counting as `int` for column alignment, `'-""'` raising as a bare dash); and that `test_textkit.py` was not modified. I would not tick the checkbox in `CARD.md` unless you want the card updated — I'd ask, and default to leaving `CARD.md` untouched since the card doesn't list itself as an output.