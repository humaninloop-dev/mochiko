FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orient, and settle the pre-code question

**What I would do.** I have already read all three files in the workspace (170 lines total), so
orientation is done. Before writing anything I ask whether any of this code needs to exist:

- Could I delete instead of write? No — the four functions are the deliverable; the tests import
  them by name.
- Does something already do this? `datetime.timedelta` parses ISO-8601, not `"1h30m"`;
  `shlex.split` is close to `tokenize_query` but gets the pinned cases wrong (it would not carry
  the negation flag, and `foo"bar baz"qux` → `foobar bazqux` only coincidentally while `""`
  yields `['']` not `[]`, and it accepts a bare `-`); `csv`/`string.Template` do not format
  aligned tables; there is no stdlib interval merge. So four small bodies genuinely need writing.
- The card also forbids new imports beyond `re`, which independently rules out a dependency.

Conclusion: write the four bodies, nothing else. `re` is needed only by `parse_duration`; the
other three are plain character/sort passes as the card fixes them. No new public names; at most
one private helper inside `render_table`'s implementation.

**Brownfield exposure.** The card says none, and I confirm it: `textkit.py` is stubs only and
nothing in the workspace imports it except the test file. So there is no existing behaviour to
preserve beyond the module docstring, the four signatures, and the four docstrings — all of which
I keep verbatim and edit nothing but the `raise NotImplementedError` lines.

**Delegation decision.** None. The entire workspace is three short files already in my context;
spawning a cheap reader would cost more than it saves and would return facts I already hold.
(If `textkit.py` had callers I had to survey, that enumeration is exactly what I would hand to a
disposable `Explore` subagent pinned to `model: haiku` with a brief like "list every file that
imports textkit and quote the import line" — but here the answer is already known to be none.)

## Phase 1 — Confirm red before touching anything

**Run:** `python3 -m unittest -q test_textkit` in the workspace directory.

**Expect:** 29 errors, one per test, every traceback ending in `NotImplementedError` raised from
`textkit.py` — not a collection error, not an `ImportError`. This is the card's claim and I verify
it myself rather than take it on trust; the whole cycle rests on the tests failing for the stub
reason and not because of a typo in the import line.

**Stop condition.** If the count is not 29, or any failure is something other than
`NotImplementedError` (e.g. an import failure, a syntax error, a test erroring in setup), I stop
and report the discrepancy rather than implementing over a broken baseline. Branch if that
happens: if it is a trivially explained environment issue (wrong working directory) I correct the
invocation and re-run; if the test file itself is inconsistent with the card I halt and flag it,
because I will not edit `test_textkit.py` to make the story fit.

## Phase 2 — Green `parse_duration`

**Write:** `textkit.py`, body of `parse_duration` only.

Approach exactly as the card fixes it:

1. Normalise: strip all whitespace out of `text` and lowercase it (`re.sub(r"\s+", "", text).lower()`)
   — this is what makes `"  1H 30M "` equal `"1h30m"` and what turns `"1h m"` into the
   invalid `"1hm"`.
2. One shape check: `re.fullmatch(r"(?:\d+[dhms])+", normalised)`; no match → `ValueError`.
   This single check rejects `""`, `"   "`, `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"`.
3. Walk `re.findall(r"(\d+)([dhms])", normalised)`, accumulating
   `int(number) * {"d": 86400, "h": 3600, "m": 60, "s": 1}[unit]` and tracking a `seen` set of
   units; a unit already in `seen` → `ValueError`. That is the only thing the fullmatch cannot
   catch, and it is what rejects `"1h1h"`.
4. Return the whole-seconds `int`.

**Run:** the suite. **Expect:** all 7 `ParseDurationTest` tests pass; the remaining 22 still error
with `NotImplementedError`. Specifically I expect `"0s"` → `0` (so I must not treat a falsy total
as an error) and `"30m1h"` → `5400` (order-independent accumulation, not a positional parse).

**Refactor check:** the multiplier map is the only thing worth naming; I would keep it as a
module-level private constant only if a second function needed it — nothing does, so it stays a
local literal. No refactor.

## Phase 3 — Green `tokenize_query`

**Write:** `textkit.py`, body of `tokenize_query` only.

One left-to-right character pass with four pieces of state: `buf` (list of chars), `negated`,
`in_quotes`, and `started` (whether a token is currently open, which is what keeps `-` literal in
the middle of a word).

- `"` → flip `in_quotes`, mark `started`, append nothing. This drops the quotes and makes
  `foo"bar baz"qux` accumulate into one buffer → `("foobar bazqux", False)`.
- whitespace and not `in_quotes` → flush the token. Inside quotes, whitespace is appended, which
  is what preserves `"multi  space"`.
- `-` and not `in_quotes` and not `started` → set `negated`, mark `started`, append nothing. Only
  a *leading* dash negates, so `a-b` stays one literal token and `lang:rust` is untouched.
- anything else → append, mark `started`.

Flush semantics (the subtle part, and where I would slow down): a flush emits `(text, negated)`
only when `buf` is non-empty; if `buf` is empty but `negated` is set, that is the bare `-` case →
`ValueError`. So `'""'` and `"   "` and `""` all emit nothing and return `[]`, while `"-"` and
`"rust -"` raise. Flush resets `buf`, `negated`, `started`.

After the loop: flush once more, then if `in_quotes` is still set → `ValueError` for the
unterminated quote. (I would raise on the unterminated quote regardless of flush order; I will
order the final flush and the quote check so `'"unterminated'` raises the quote error rather than
silently emitting a token — checking `in_quotes` *before* the final flush is the safer order and
that is what I would write.)

**Run:** the suite. **Expect:** 15 passing (7 + 8), 14 still erroring. The tests I watch hardest
are `test_quotes_toggle_mid_token`, `test_empty_inputs` (the `'""'` case is what punishes a naive
"emit whatever the quotes contained" flush), and the three rejection strings.

## Phase 4 — Green `render_table`

**Write:** `textkit.py`, body of `render_table`, plus at most one private helper
(`_format_row`) inside the module — permitted by the card, and the card explicitly wants header,
rule, and rows to go through one shared formatter.

1. Validate first: empty (or falsy) `headers` → `ValueError`; any row whose length differs from
   `len(headers)` → `ValueError`. This covers `render_table([], [])` and the ragged
   `[["a", 1], ["b"]]`.
2. Per column `i`: `right = all(isinstance(row[i], int) for row in rows)` — vacuously true for a
   column with no rows, which is harmless because a right-aligned header in its own exact width
   is identical to a left-aligned one (the no-rows test expects `"name  qty"`, and both alignments
   produce it). Width = `max(len(str(cell)) for the header and every cell in that column)`.
3. `_format_row(cells)` = join `str(cell).rjust(w)` or `.ljust(w)` per column with `"  "`, then
   `.rstrip()`.
4. Build `[_format_row(headers)] + [_format_row(["-" * w ...])] + [_format_row(row) for row in rows]`
   and `"\n".join(...)` with no trailing newline. The rule row goes through the same formatter, so
   the two-space gutter is defined in exactly one place.

I would hand-check the four expected strings before running, because they are exact:
`"name   qty"` (5-wide `name`, gutter, 3-wide right `qty`), `"-----  ---"`, `"apple    3"`,
`"kiwi    12"`; the mixed column `["a", 1] / ["b", "n/a"]` must go *left* (`"k  v"` after rstrip);
and the single numeric column must right-align its header to `" n"` — so rstrip only, never strip,
or that test fails.

**Run:** the suite. **Expect:** 21 passing, 8 still erroring. If any table test fails it will be a
one-character width or gutter discrepancy; I would diff the expected and actual strings rather
than guess.

**Note I would record, not act on:** `isinstance(True, int)` is true, so a column of booleans
would right-align. No test covers it and the card asks for exactly what it asks for, so I add no
`bool` exclusion — I mention it in the report as a known, untested edge.

## Phase 5 — Green `merge_ranges`

**Write:** `textkit.py`, body of `merge_ranges` only.

1. Validate every pair up front: unpack `start, end`; `isinstance(start, int)` and
   `isinstance(end, int)` both required → `ValueError` for `(1, "2")`; `start > end` →
   `ValueError` for `(3, 1)`. Validation runs over the whole input before any merging, so a bad
   pair anywhere raises.
2. `sorted(ranges)` (sorting by start, ties by end, which is fine and cheaper than a key
   function).
3. One sweep: hold `(open_start, open_end)`; for each next `(s, e)`, if `s <= open_end + 1` then
   `open_end = max(open_end, e)` (the `max` is what makes the contained `(2, 3)` inside `(1, 10)`
   a no-op), else emit and reopen. Emit the last one. The `+ 1` is what merges the adjacent
   `(1,3),(4,6)` while leaving the one-gap `(1,3),(5,6)` split.
4. Return a list of tuples (the tests compare against tuples, so I normalise to `tuple` on emit);
   `[]` in → `[]` out, with the sweep guarded on an empty input.

**Run:** the suite. **Expect:** `OK`, 29 tests — the card's assertion.

## Phase 6 — Refactor pass and final verification

Read the finished `textkit.py` top to bottom as a whole and check: no import beyond `re`; no
class; no CLI or `__main__` block; exactly four public names plus any underscore-prefixed helper;
the module docstring's stub language ("Each body below is a stub…") is now stale — I would update
that one sentence, since it is the file's own description of itself and would otherwise be a lie,
and flag the edit in the report as the one text change outside the four bodies. The four function
docstrings stay exactly as written.

**Run:** `python3 -m unittest -q test_textkit` one final time. **Expect:** `OK`, 29 tests, zero
failures, zero errors. `test_textkit.py` unmodified.

**What I would refuse along the way:** editing `test_textkit.py` for any reason; adding validation,
options, type hints, or convenience wrappers the tests do not pin; adding a fifth public function;
"while I'm here" tidying of anything outside the four bodies (the stale module docstring is the
single exception, and it is disclosed).

## Phase 7 — Mark the card and write the report

**Write:** `CARD.md` — flip `- [ ] **C3 · textkit helpers**` to `- [x]`, changing nothing else in
the card.

**Stop / confirm.** There is no `tasks.md` in this workspace; the checkbox lives in `CARD.md`, and
there is no stated location for the cycle report. I would confirm with the requester where the
report belongs and whether ticking the box in `CARD.md` is the intended equivalent of updating a
task list. Branches: if they name a directory (e.g. a cycle folder), I write the report there and
leave `CARD.md`'s checkbox ticked regardless; if they say a `tasks.md` exists elsewhere outside
this workspace, I flag that I cannot see it and do not invent one. Absent an answer I proceed on
my default: report at `cycle-report.md` in the workspace root, checkbox ticked in `CARD.md`.

**Write:** `cycle-report.md` — a truthful account, not a verdict: the confirmed red baseline (29
errors, all `NotImplementedError`); the four green steps in order with the pass counts observed at
each; the pre-code check and why `shlex.split` and `timedelta` were considered and rejected for
`tokenize_query` and `parse_duration`; the judgment calls I made where the tests are silent
(vacuous right-alignment of a column with no rows, `bool` counting as `int` for alignment, the
final-flush-versus-unterminated-quote ordering, negation only on a token-initial dash); the one
edit outside the four bodies (the stale module docstring sentence); the outstanding ambiguity
about the report's location and the missing `tasks.md`; and any place where a first implementation
attempt failed a test and what the actual discrepancy was, recorded as it happened rather than
smoothed over.

**Final message back:** cycle C3 implemented, suite green at 29 tests, `test_textkit.py`
untouched, constraints held (stdlib only, `re` the sole import, no class or CLI, four public
names), plus the flagged docstring edit and the report-location question.