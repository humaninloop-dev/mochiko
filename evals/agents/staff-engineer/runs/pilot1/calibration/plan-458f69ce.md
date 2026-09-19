FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Confirm the starting state matches what the card claims

**Do:** Run the full suite once before touching anything: `python3 -m unittest -q test_textkit`.

**Expect:** 29 errors, one per test, every one a `NotImplementedError` raised from a stub in `textkit.py` — matching the card exactly. This is the red phase; the card states it was already observed, and I re-run it only to confirm the workspace hasn't drifted from that claim, not to redo it.

**Stop condition:** If the count isn't 29, or any failure is something other than `NotImplementedError` from a stub (an import error, a syntax error, a test that already passes), I stop and report the discrepancy rather than implementing over an unknown baseline. The card is explicit about the expected red, so a mismatch means the card and the workspace disagree and that's a human call, not mine to paper over.

**No test writing in this cycle.** Every test already exists and is the behaviour spec. `test_textkit.py` will not be edited — that is a hard line, including in the case where a test looks wrong to me. If I believe a test is wrong, I stop and report it; I do not adjust the spec to fit my code.

## Phase 1 — Pre-code check: can any of this not be written?

**Do:** Before writing bodies, check whether each function can be avoided, borrowed, or reduced to builtins.

- `parse_duration` — nothing in the standard library parses `"1h30m"`. `datetime`/`timedelta` don't. Must be written; `re` (already imported) does the shape check, so the body stays small.
- `tokenize_query` — **`shlex.split` is a genuine near-miss and I will examine it, not dismiss it.** It handles quotes, collapses whitespace, joins quote-adjacent runs (so `foo"bar baz"qux` → `foobar bazqux`), and raises `ValueError` on an unterminated quote. But it fails this spec on three counts: it doesn't produce the `negated` flag or reject a bare `-`; it applies backslash-escape semantics this spec doesn't ask for; and the card constrains the module to `re` as the only import. I would reject it and **record the consideration and the reason in the report** rather than silently hand-rolling.
- `render_table` — no stdlib table formatter without a dependency. But padding is `str.ljust`/`str.rjust` and trimming is `str.rstrip`; I write no padding loops.
- `merge_ranges` — no stdlib interval merge. Sorting is the builtin `sorted`. The sweep is a few lines.

**Outcome:** all four bodies must be written; three of them lean on builtins rather than new code. No new imports. No class, no CLI, no public name beyond the four; any helper is private and only introduced if it removes real duplication.

**Delegation:** none. The whole workspace is three files and I have read all of them in full. There is no locate, no enumeration, and no bulk read here that would justify spawning a cheap reader — dispatching one would cost more than it saves. I note this deliberately rather than inventing a spawn.

## Phase 2 — `parse_duration` (write to `textkit.py`)

**Do:** Implement the body following the approach the card fixes.

- Normalise: strip all whitespace out with `re.sub(r"\s+", "", text)`, then lowercase.
- Shape check: one `re.fullmatch(r"(\d+[dhms])+", norm)`; no match → `ValueError`. This alone rejects `""`, `"   "` (both normalise to empty, and `+` requires at least one pair), `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"` (normalises to `"1hm"`, which the pattern can't complete).
- Walk pairs with `re.findall(r"(\d+)([dhms])", norm)`, tracking seen units in a set; a repeat → `ValueError`. This is what catches `"1h1h"`, which passes the shape check.
- Multiply against `d=86400, h=3600, m=60, s=1` and sum.

**Run:** `python3 -m unittest -q test_textkit.ParseDurationTest` → expect 7 tests, `OK`. Specifically `"90s"`→90, `"1h30m"`→5400, `"1d2h3m4s"`→93784, `"30m1h"`→5400, `"  1H 30M "`→5400, `"0s"`→0, and all nine malformed strings raising.

**Not doing:** no type-checking of `text`, no unit aliases (`"min"`, `"sec"`), no float support, no negative durations. Nothing asked for them.

## Phase 3 — `tokenize_query` (write to `textkit.py`)

**Do:** One left-to-right character pass with three pieces of state: `in_quotes`, a buffer, and `negated`.

- A double quote toggles `in_quotes` and is dropped from the buffer — never flushes the token. This is what makes `foo"bar baz"qux` a single token `"foobar bazqux"`.
- Whitespace flushes only when not `in_quotes`; inside quotes it goes into the buffer, preserving `"multi  space"`.
- A `-` negates only when the buffer is empty, `negated` isn't already set, and we're not in quotes; otherwise it's a literal character. This keeps `a-b` and `lang:rust` intact while negating `-"cargo build"`.
- Flush emits `(buffer, negated)` only when the buffer is non-empty — which is exactly why `'""'` yields `[]` rather than an empty token, with no special case needed.
- Flush with `negated` set but an empty buffer → `ValueError`. Covers both `"-"` and `"rust -"`.
- After the loop: if `in_quotes` → `ValueError` (unterminated), then final flush.

**Run:** `python3 -m unittest -q test_textkit.TokenizeQueryTest` → expect 8 tests, `OK`.

**Flag in the report (not a stop):** `-""` is untested and my rule raises on it, since it's a dash with no token behind it. That's consistent with the bare-dash rule and I won't add an untested special case to change it — but I'll disclose the choice rather than let it sit as an accident.

## Phase 4 — `render_table` (write to `textkit.py`)

**Decision point I would surface — and how I'd resolve it.** The card phrases the alignment rule as "right-aligned only when every cell in the column is an `int`, header included," which could mean the *header string itself* must be an `int`. Under that reading no column would ever right-align and three tests would fail. The tests settle it: `render_table([[1], [22]], ["n"])` expects `" n\n--\n 1\n22"`, where the header `"n"` is right-aligned. So "header included" means the header *participates in the alignment*, not in the int test. I proceed under that reading, note it in the report, and would only revisit it if a human ruled otherwise — in which case the tests would have to change, which is out of my scope and a stop.

**Do:**

- Validate first: empty `headers` → `ValueError`; any row whose length differs from `len(headers)` → `ValueError`.
- Per column: `right = all(isinstance(cell, int) for cell in column_cells)` over the row cells only. Note this is vacuously true when there are no rows — harmless, because with no rows the width equals the header width and left/right produce identical output (`render_table([], ["name","qty"])` → `"name  qty\n----  ---"` either way).
- Width = max of `len(str(...))` over the header and that column's cells.
- One shared row formatter (as the card prescribes) that takes a list of already-stringified cells, applies `rjust`/`ljust` per column, joins with two spaces, and right-strips. Header line, rule line (`"-" * width` per column), and each data row all go through it — that's the one place duplication would otherwise creep in.
- Join with `"\n"`, no trailing newline.

**Run:** `python3 -m unittest -q test_textkit.RenderTableTest` → expect 6 tests, `OK`. I'd pay particular attention to the exact-string comparisons: `"name   qty\n-----  ---\napple    3\nkiwi    12"` and the mixed-column `"k  v\n-  ---\na  1\nb  n/a"`, since off-by-one spacing is the likely failure mode here.

**Not doing:** no column-count validation on `headers` vs. cell types, no `bool`-vs-`int` special-casing, no configurable separator or alignment override.

## Phase 5 — `merge_ranges` (write to `textkit.py`)

**Do:**

- Validate every pair before anything else: both bounds `isinstance(..., int)`, then `start <= end`; either violation → `ValueError`. **The order matters** — checking the types first is what makes `(1, "2")` raise `ValueError` rather than a `TypeError` from comparing `int` to `str`, which is what the test demands.
- `sorted(...)` by start.
- One sweep: hold an open `(start, end)`; while the next start is `<= open_end + 1`, extend the open end to `max(open_end, next_end)`; otherwise close and open a new one. The `+ 1` is what merges the adjacent `(1,3),(4,6)` while leaving the one-gap `(1,3),(5,6)` split.
- Return a list of tuples; `[]` in → `[]` out.

**Run:** `python3 -m unittest -q test_textkit.MergeRangesTest` → expect 8 tests, `OK`.

## Phase 6 — Refactor pass

**Do:** With all four green, re-read `textkit.py` end to end looking only for duplication that actually exists — most likely the row formatter in `render_table`, which is already factored by design. I would *not* introduce a shared validation helper across functions, a constants module, or any abstraction the tests don't exercise; four independent pure functions that don't share shape don't need a common spine. Confirm the docstrings still describe the behaviour accurately, confirm `re` is the only import and is genuinely used, and confirm no public name beyond the four exists.

**Run:** the full suite again after any refactor edit.

## Phase 7 — Full suite and card update

**Run:** `python3 -m unittest -q test_textkit` → expect `OK`, 29 tests, zero failures, zero errors. This is the card's stated acceptance check verbatim.

**Stop condition:** anything short of a clean 29 and I do not tick the checkbox; I report the failing test and its actual output.

**Write:** `CARD.md` — flip line 3's `- [ ]` to `- [x]`.

**Flag:** the card describes updating `tasks.md`, but this workspace has no `tasks.md` — the checkbox lives in `CARD.md`. I'd tick it there and say so in the report rather than creating a `tasks.md` that nothing asked for.

**Write:** `cycle-report.md` at the workspace root (new file), an honest record covering:

- Baseline confirmed: 29 errors, all `NotImplementedError`; the red phase was pre-established by the card, so this cycle ran green-and-refactor only — stated plainly as a deviation from a normal full cycle rather than glossed.
- Per-function green transitions, in order, with the class-level runs.
- **`shlex.split` considered for `tokenize_query` and rejected** — it matches the quote handling and even the unterminated-quote `ValueError`, but can't express negation or the bare-dash rejection, brings unrequested escape semantics, and violates the card's `re`-only import constraint. Disclosed rather than left implicit.
- The `render_table` "header included" ambiguity and the reading the tests forced.
- The untested `-""` behaviour and why I left it raising.
- `tasks.md` absent; checkbox ticked in `CARD.md` instead.
- Final result: `OK`, 29 tests; `test_textkit.py` unmodified; no new imports; no public names beyond the four; no CLI, no class.

## Standing refusals for this cycle

- I will not edit `test_textkit.py`, for any reason, including to make a test I disagree with pass.
- I will not add `shlex`, `datetime`, or any import beyond the existing `re`.
- I will not add a class, a CLI entry point, `__all__`, or any fifth public name.
- I will not add validation, options, or edge-case handling the 29 tests don't exercise — including the type-checking and format extensions each function could plausibly want.
- If any test fails in a way that suggests the spec itself is inconsistent, I stop and report it rather than bending code around it.