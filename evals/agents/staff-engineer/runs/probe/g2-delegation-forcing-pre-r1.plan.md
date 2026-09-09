FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-mvm_4ud3/ws/CARD.md`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-mvm_4ud3/ws/textkit.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-mvm_4ud3/ws/test_textkit.py`

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orient (already done)

Read all three workspace files in full: `CARD.md`, `textkit.py` (45 lines, four stubs each raising `NotImplementedError`, `import re` already present), `test_textkit.py` (125 lines, four `TestCase` classes).

Counted the tests against the card's claim of 29: `ParseDurationTest` 7 + `TokenizeQueryTest` 8 + `RenderTableTest` 6 + `MergeRangesTest` 8 = **29**. The card's arithmetic holds, which is a first cheap confirmation that the file I'm reading is the file the card was written against.

**Delegation decision:** none. The whole workspace is three files totalling ~170 lines, all already in my context, and every one of them is interpretive reading (the tests *are* the spec — I have to read them myself to derive behaviour). There is no locate, no enumeration, no bounded quote-back that a cheap read-only helper could do more cheaply than I already have. Spawning anything here would cost more than it saves, so I do this work directly.

## Phase 1 — Pre-code check: does any of this need to be written at all?

Before a line of implementation, for each of the four functions I ask whether something already in reach does the job. Findings I would carry into the report:

- **`parse_duration`** — nothing in the stdlib parses `"1h30m"`. `datetime.timedelta` constructs but does not parse; `time.strptime` is clock-time, not durations. Write it, with `re` (already imported).
- **`tokenize_query`** — **`shlex` is a genuine candidate** and I would not skip past it. `shlex.split` does quote-aware splitting, concatenates adjacent quoted/unquoted runs (so `foo"bar baz"qux` → `foobar bazqux`, matching the test), and raises `ValueError("No closing quotation")` on an unterminated quote. It would get several tests for free. I reject it, and I record *why* rather than silently hand-rolling: (a) `shlex` also treats **single quotes** as quoting and **backslash** as an escape, which directly contradicts the spec'd "punctuation is literal" — an input like `it's` or `a\b` would raise or mangle where this spec wants a literal token, and no test pins that, so adopting `shlex` would smuggle in untested divergent behaviour; (b) it has no notion of the leading-`-` negation, so I'd still need my own pass to split and validate `-`; (c) `shlex.split('""')` yields `['']`, which I'd have to filter to satisfy `test_empty_inputs`. The card also fixes the approach as a single left-to-right character pass. Cheapest-that-*works* here is the hand-rolled pass; `shlex` is cheaper but doesn't work.
- **`render_table`** — no stdlib table renderer exists (`str.ljust`/`rjust` are the primitives, and I use them). Write it.
- **`merge_ranges`** — builtin `sorted` plus one sweep; nothing smaller exists.

No new imports. `re` stays the only one, used only by `parse_duration`; the other three need none.

## Phase 2 — Re-confirm red before writing anything

Run: `python3 -m unittest -q test_textkit`

Expect: **29 errors, 0 failures**, every one a `NotImplementedError` raised from the four stubs — i.e. failing because the behaviour is absent, not because of an import error, a typo in the test module, or a collection problem.

**Stop condition:** if the count is not 29, or if any failure is something other than `NotImplementedError` (e.g. `ImportError`, a syntax error, a test that already passes), I stop and flag it rather than start implementing. The card asserts a specific starting state; a mismatch means the card and the workspace have drifted and I'd want that confirmed before building on it. *Branches:* if I'm told the drift is expected and the tests are authoritative, I re-derive the spec from the tests and note the card's inaccuracy in the report; if I'm told the card is authoritative, I stop entirely, because the card forbids editing `test_textkit.py` and I will not resolve the conflict by changing the spec. **Default if no ruling is available:** the tests are the behaviour spec (the card says so itself), so I proceed against the tests and record the discrepancy.

I do not write any new tests in this cycle. Every behaviour and every rejected input is already pinned, red has been established, and adding tests would be scope I wasn't asked for. My red/green discipline here runs per function against the existing suite.

## Phase 3 — `parse_duration` (green for `ParseDurationTest`)

**Write to:** `/…/ws/textkit.py`, body of `parse_duration` only. Docstring untouched.

Behaviour I derived from the tests, not from the prose:

- Normalise: strip **all** whitespace (`re.sub(r"\s+", "", text)`) then lowercase. This is what makes `"  1H 30M "` → `"1h30m"` → 5400, and it is also what correctly *rejects* `"1h m"`, which normalises to `"1hm"` — a unit with no number.
- Shape gate: `re.fullmatch(r"(\d+[dhms])+", normalised)`. One `fullmatch`, as the card fixes. This alone rejects `""`, `"   "` (empty after normalising; `+` needs at least one pair), `"90"` (no unit), `"1x"`, `"1.5h"` (the `.`), `"-5s"` (the `-`), `"h"` (no digits).
- Walk `re.findall(r"(\d+)([dhms])", normalised)`, tracking a `seen` set; a repeated unit rejects `"1h1h"`. Multiply by `{"d": 86400, "h": 3600, "m": 60, "s": 1}` and sum.
- Raise `ValueError` with a message naming the offending text on both the shape miss and the duplicate unit.
- `"0s"` must return `0` — so the return is the accumulated sum unconditionally; no falsy-guard anywhere near it.

**Run:** `python3 -m unittest -q test_textkit.ParseDurationTest` → expect **OK, 7 tests**. Then the full suite → expect **22 errors** remaining (the other three groups still stubbed), confirming I changed nothing outside this function.

## Phase 4 — `tokenize_query` (green for `TokenizeQueryTest`)

**Write to:** `textkit.py`, body of `tokenize_query` only.

Single left-to-right pass over the characters with three pieces of state: `in_quotes`, a `buf` list, a `negated` flag, and an `in_token` flag that distinguishes "a token has begun" from "buffer happens to be empty".

- Whitespace **and not** `in_quotes` → flush the token. Inside quotes, whitespace is kept, which is what `'"multi  space"'` → `("multi  space", False)` requires.
- `"` → toggle `in_quotes`, set `in_token`, append nothing. Because the quote only toggles and never flushes, `foo"bar baz"qux` accumulates into the one token `("foobar bazqux", False)` — that test is the reason the flag is `in_token` rather than "is the buffer non-empty".
- `-` → negation **only** when not `in_quotes` and not `in_token`; otherwise a literal character. This is what keeps `a-b` a single unnegated token in `test_punctuation_is_literal` while `-"cargo build"` negates.
- Any other character → append, set `in_token`.
- Flush rule: if the buffer is empty and `negated` is set → `ValueError` (this is the "bare `-`" case, covering both `"-"` and `"rust -"`). If the buffer is empty and not negated → emit nothing (this is why `'""'` yields `[]` alongside `""` and `"   "`). Otherwise append `("".join(buf), negated)` and reset.
- After the loop: if `in_quotes` → `ValueError` for the unterminated quote (`'"unterminated'`); then one final flush.

**Run:** `python3 -m unittest -q test_textkit.TokenizeQueryTest` → expect **OK, 8 tests**. Full suite → expect **14 errors** remaining.

## Phase 5 — `render_table` (green for `RenderTableTest`)

**Write to:** `textkit.py`, body of `render_table`, plus one private module-level helper `_format_row(cells, widths, aligns)` — the "one shared row formatter" the card calls for. Private, inside `textkit.py`, so it stays inside the "no public name beyond the four" constraint.

- Validate first: empty `headers` → `ValueError` (this is what makes `render_table([], [])` raise); any row whose length differs from `len(headers)` → `ValueError` (`[["a", 1], ["b"]]`).
- Per column: `right = all(isinstance(row[i], int) for row in rows)` — computed over **row cells only**, then the resulting alignment is applied to the header too. Width = max of `len(str(...))` over the header and all cells.
- Header line, rule line (`"-" * width` per column), then one line per row, all through `_format_row`; columns joined by exactly two spaces; every line `.rstrip()`ed; joined with `"\n"`, no trailing newline.

**Ambiguity I would flag but not stop on:** the card's phrasing "right-aligned only when every cell in the column is an `int`, **header included**" reads as though the header participates in the int test — but `test_single_numeric_column_right_aligns_header` demands `render_table([[1], [22]], ["n"])` == `" n\n--\n 1\n22"`, and `"n"` is not an int. So "header included" must mean the *alignment* covers the header, not the *check*. The test settles it; I follow the test and note the wording in the report so the card can be tightened. (Relatedly, `render_table([], ["name", "qty"])` makes the `all()` vacuously true and so nominally right-aligns, but both headers exactly fill their columns, so the choice is unobservable — no test can distinguish it and I add nothing to force one way.)

I would hand-check the two hardest expectations before running, so a green isn't a coincidence: `"apple"` padded to 5 + two spaces + `"  3"` = `apple····3` matching `"apple    3"`, and the mixed column `[1, "n/a"]` going left-aligned with the header line rstripping `"k  v  "` down to `"k  v"`.

**Run:** `python3 -m unittest -q test_textkit.RenderTableTest` → expect **OK, 6 tests**. Full suite → expect **8 errors** remaining.

## Phase 6 — `merge_ranges` (green for `MergeRangesTest`)

**Write to:** `textkit.py`, body of `merge_ranges` only.

- Validate every pair up front: unpack `start, end`; `ValueError` unless both are `isinstance(..., int)` (rejects `(1, "2")`); `ValueError` if `start > end` (rejects `(3, 1)`).
- `sorted(...)` by start, then one sweep: while the next start is `<= open_end + 1`, extend with `open_end = max(open_end, next_end)`. The `max` is what makes `[(1, 10), (2, 3), (1, 10)]` collapse to `[(1, 10)]` rather than shrinking to `(1, 3)`; the `+ 1` is what merges the adjacent `(1,3),(4,6)` while leaving the gap-of-one `(1,3),(5,6)` split.
- Return a list of tuples; `[]` in → `[]` out (the sweep never starts).
- I deliberately do **not** add a length check on each pair, a `bool`-is-not-really-an-int guard, or type-coercion of the input sequence. No test asks for them and the card asks for exactly this validation; I'd note the `bool` subtlety in the report as an observation rather than acting on it.

**Run:** `python3 -m unittest -q test_textkit.MergeRangesTest` → expect **OK, 8 tests**. Then the card's own check: `python3 -m unittest -q test_textkit` → expect **`OK`, 29 tests**, which is the card's TEST line satisfied.

## Phase 7 — Refactor, at green

With the suite green, one read-through of `textkit.py` for naming and duplication only — e.g. confirming the unit-multiplier table and `_format_row` are the only shared structures worth having, and that nothing crept in beyond the four public functions plus that one private helper. Re-run the full suite after any edit; expect `OK, 29 tests` again. I would **not** take this as an opening to add type hints, restructure the module, rewrite the docstrings (they're part of the spec), or "tidy" anything the card didn't name.

**Standing refusals through phases 3–7:** I do not touch `test_textkit.py` under any circumstance, including if a test looks wrong — I'd stop and flag instead. No import beyond `re`. No class, no `__main__` block, no CLI, no new public name. No `argparse`/`textwrap`/`itertools` reach-for even where it'd shorten a line.

## Phase 8 — Tick the card

**Discrepancy to flag:** my normal habit is to check off the completed cycle card in `tasks.md`, but **this workspace has no `tasks.md`** — the only files are `textkit.py`, `test_textkit.py`, and `CARD.md`. The unticked checkbox lives at line 3 of `CARD.md`: `- [ ] **C3 · textkit helpers**`. So I would edit **`CARD.md`**, changing that one checkbox to `[x]` and nothing else, and call out the substitution explicitly in the report rather than doing it silently. *Branch if this were confirmable:* if told a `tasks.md` should exist elsewhere, I'd tick it there and leave `CARD.md` alone; if told cards aren't ticked in this setup, I'd revert and note it. **Default:** tick line 3 of `CARD.md`, disclose it.

## Phase 9 — Write `cycle-report.md`

**Write to:** `/…/ws/cycle-report.md`. An honest record of what happened, not a verdict:

- Starting state verified: 29 errors, all `NotImplementedError`, matching the card.
- Per function: which tests went green, in what order, and the suite's error count dropping 29 → 22 → 14 → 8 → 0 as evidence each change stayed in its own lane.
- Final result: `python3 -m unittest -q test_textkit` → `OK`, 29 tests.
- **The `shlex` decision**, stated plainly: it was the cheaper option, it was seriously considered, and it was rejected for single-quote/backslash semantics that diverge from "punctuation is literal" in ways no test would have caught — disclosed rather than buried.
- **The header-alignment wording in the card** vs. what `test_single_numeric_column_right_aligns_header` actually requires, and that I followed the test.
- **The `tasks.md` → `CARD.md` substitution** for the checkbox.
- Observations noted but deliberately **not** acted on: `bool` passing an `isinstance(..., int)` check in `merge_ranges`; alignment being unobservable for a table with no rows; no length check on range pairs.
- Anything that actually surprised me during the run, recorded as it happened — including, if it occurs, any place a test went green on the first try for a reason I hadn't fully predicted, which I'd then verify by hand rather than accept.

**Final report to the requester:** C3 complete — four bodies implemented in `textkit.py` plus one private row formatter, `test_textkit.py` untouched, `re` still the only import, suite `OK` at 29 tests, card ticked in `CARD.md` (no `tasks.md` present), with the three flagged items above.