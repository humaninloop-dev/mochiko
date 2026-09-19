FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py` (plus a directory listing of the workspace root, which contains only those three files)

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Load the procedures I work from

- Invoke `mochiko:executing-tdd-cycle` (the cycle procedure and the authoritative shape of `cycle-report.md`) and `mochiko:patterns-code-minimalism` (the pre-code check).
- I would **not** load `mochiko:brownfield-integration`. The card declares no brownfield exposure, and I confirmed it: the workspace is three files, nothing imports `textkit`, and `textkit.py` contains only stubs. The one residual brownfield obligation — keep the four signatures and docstrings exactly as written and add nothing to the module surface — I carry directly.

## Phase 1 — Confirm the stated red state before writing a line

The card asserts the red phase is already done: 29 tests, each erroring on `NotImplementedError`. I take that as a claim to verify, not a fact to accept.

- Run `python3 -m unittest -q test_textkit`.
- Expect: 29 tests run, 29 errors, every traceback ending in `NotImplementedError`, no failures and no collection/import error.
- Cross-check against my own count of the test file: `ParseDurationTest` 7, `TokenizeQueryTest` 8, `RenderTableTest` 6, `MergeRangesTest` 8 = 29. This matches the card, so the card and the workspace agree on scope.
- **Stop condition.** If the run shows anything other than 29 errors all rooted in `NotImplementedError` — a different count, a real assertion failure, an import error, a syntax error — I stop and report rather than implementing. Branches: (a) *count differs* → I report the discrepancy between the card and the suite and ask whether the card or the test file is authoritative; default while waiting is to treat `test_textkit.py` as the spec, since the card itself names the tests as the behaviour spec. (b) *a test fails rather than errors* → that means a stub is being satisfied by accident or the harness is wrong; I report before touching code. (c) *import error* → environment problem, report, no code written.

## Phase 2 — Pre-code check: does any of this need to be written?

Run the cheapest-that-works check on each of the four before writing anything, and record the outcome for the report.

- `parse_duration` — no stdlib parser for `"1h30m"`; `datetime.timedelta` constructs but does not parse. Must be written.
- `tokenize_query` — `shlex.split` is a genuine near-miss: it would collapse `foo"bar baz"qux` into one token and raises `ValueError` on an unterminated quote. It is ruled out on two counts: it yields no negation flag and would still need a `-` pass, and the card constrains the module to `re` as the only import. Must be written, as a character pass.
- `render_table` — no stdlib text-table formatter. Must be written.
- `merge_ranges` — no stdlib interval merge. Must be written.

Conclusion I would record: all four are genuinely new, each is a short pure function, and the card fixes the approach for each, so there is no room to reach for something cheaper. No helper module, no class, no CLI.

**Note on the TDD shape, to be disclosed in the report:** the red phase for this cycle was performed before I was handed the card, and the card forbids editing `test_textkit.py`. So I do not author new tests; I verify the existing red (Phase 1), then drive green one function at a time so each function's transition is individually attributable. I will not add a scratch test file — that would be new scope.

## Phase 3 — Green, function by function

One function per step, editing only `/…/ws/textkit.py`, running the full suite after each so the error count drops in a known ladder: **29 → 22 → 14 → 8 → 0**.

### 3a. `parse_duration` (expect 29 errors → 22)

Follow the card's fixed approach exactly: normalise, one `re.fullmatch` shape check, then `re.findall` over number-unit pairs with a seen-set.

- Normalise: lowercase and strip *all* whitespace (`re.sub(r"\s+", "", text.lower())`) — this is what makes `"  1H 30M "` work and, deliberately, what makes `"1h m"` collapse to `"1hm"` and fail the shape check.
- Shape: `re.fullmatch(r"(\d+[dhms])+", norm)`; no match → `ValueError`. This is the single gate that rejects `""`, `"   "`, `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"`.
- Pairs: `re.findall(r"(\d+)([dhms])", norm)`; a unit already in the seen-set → `ValueError` (this is the only thing that catches `"1h1h"`, which passes the shape check).
- Multipliers as a local dict `{"d": 86400, "h": 3600, "m": 60, "s": 1}` kept **inside** the function, so the module gains no new name at all.
- Sum and return an `int`. `"0s"` → `0` falls out naturally.

Expected after this step: the 7 `ParseDurationTest` tests pass; 22 errors remain.

### 3b. `tokenize_query` (expect 22 → 14)

Single left-to-right character pass as the card fixes. State: `tokens`, a character buffer, `in_quotes`, `negated`, and `started` (whether a token is currently open).

- `"` → mark the token started, flip `in_quotes`, append nothing. This is what makes `foo"bar baz"qux` a single `foobar bazqux`.
- Whitespace while `in_quotes` → append it (keeps `multi  space` and `cargo build` intact). Whitespace otherwise → flush.
- `-` when not in quotes and no token is open → set `negated`, mark started, append nothing. Any other `-` is a literal character, which is what keeps `a-b` and `lang:rust` whole.
- Everything else → append.
- Flush rule (used on whitespace and once at end of input, via a nested function with `nonlocal`): if the buffer is empty **and** the token was negated → `ValueError` (this is the `"-"` and `"rust -"` case); if the buffer is empty and it was not negated → drop silently (this is why `'""'` yields `[]`, and why leading/trailing whitespace is harmless); otherwise append `(token, negated)` and reset.
- At end of input, check `in_quotes` **before** the final flush → `ValueError` for `"unterminated`.

Untested edge I would decide and record rather than leave silent: `-""` produces a negated-but-empty token, which my flush rule rejects as a `ValueError`. No test pins it; I note the choice in the report and do not add machinery for it.

Expected after this step: the 8 `TokenizeQueryTest` tests pass; 14 errors remain.

### 3c. `render_table` (expect 14 → 8)

Card's order: validate, compute widths and alignments, then one shared row formatter.

- Validate: empty `headers` → `ValueError` (catches `render_table([], [])`); any row whose length differs from `len(headers)` → `ValueError` (catches `[["a", 1], ["b"]]`). Headers checked first.
- Alignment per column: right only when every *cell* in that column is an `int`; the header follows the column's alignment but is not itself required to be an int — test 4 (`[[1], [22]]` with header `"n"` → `" n"`) is what pins this reading. A column with no cells (`rows == []`) lands on right by the empty-`all()`, which is unobservable in the one test that exercises it.
- Width per column: max of `len(str(cell))` over the cells and `len(header)`.
- One nested formatter takes a sequence of already-stringified cells, pads each with `ljust`/`rjust` per the column's alignment, joins with two spaces, and right-strips. Header, the `-`-rule (each cell `"-" * width`, so padding is a no-op), and every data row all go through it.
- Join with `"\n"`, no trailing newline.

I would hand-verify the four exact expected strings before running, since they are the fiddly part: `"name   qty\n-----  ---\napple    3\nkiwi    12"`, `"name  qty\n----  ---"`, `"k  v\n-  ---\na  1\nb  n/a"`, `" n\n--\n 1\n22"`.

Expected after this step: the 6 `RenderTableTest` tests pass; 8 errors remain.

### 3d. `merge_ranges` (expect 8 → 0)

- Validate each pair first: unpack `start, end`; both must be `int` (`(1, "2")` → `ValueError`); `start` must not exceed `end` (`(3, 1)` → `ValueError`). Validate the whole input before merging, so a bad pair anywhere raises regardless of order.
- Sort by start.
- One sweep: extend the open range while the next start is at most the open end plus one, taking `max` of the ends; otherwise close and open a new one. The `+ 1` is what merges `(1,3),(4,6)` and `(5,5),(6,6)` while leaving `(1,3),(5,6)` split.
- Return a list of tuples (the tests compare against tuples); `[]` in → `[]` out.

Expected after this step: the full suite reports `OK`, 29 tests, 0 errors — the card's assert.

## Phase 4 — Refactor, with the suite green

No behaviour change; re-run `python3 -m unittest -q test_textkit` after any edit and expect `OK`, 29 tests.

- Confirm `render_table` has exactly one row-formatting path (no duplicated padding logic between header, rule, and rows).
- Confirm `tokenize_query` has exactly one flush path used by both the whitespace branch and the end-of-input branch.
- Remove anything speculative I may have reached for; the target is the smallest body per function that the tests pin.

## Phase 5 — Constraint audit against the card

Re-read the finished `textkit.py` and check, explicitly:

- Imports: `re` and nothing else, and it is actually used (only by `parse_duration`).
- Module surface: exactly `parse_duration`, `tokenize_query`, `render_table`, `merge_ranges`. Any helper is nested inside a function or underscore-prefixed. No module-level constants added.
- No class, no `if __name__ == "__main__"`, no CLI.
- The four signatures and docstrings are byte-identical to the stubs.
- `test_textkit.py` is untouched. If at any point a test looks wrong to me, I **refuse to edit it** — I stop and report the disagreement instead, since the card names the tests as the spec.

## Phase 6 — Delegation decision

**No delegation.** The entire workspace is three files totalling under 200 lines and all of it is already in my context after Phase 0 reading; there is no locate, no enumeration, and no bounded quote left to farm out. Spawning a cheap `Explore` reader would cost more than it saves. The one thing that might have warranted a sweep — "does anything else import `textkit`?" — was already settled by the directory listing, which shows only `CARD.md`, `textkit.py`, and `test_textkit.py`.

## Phase 7 — Tick the card and write the report

- **Flag / stop-shaped item:** my standing remit is to tick the completed card in `tasks.md`, but this workspace has no `tasks.md`. The only task list present is the checkbox on line 3 of `CARD.md`. What I would confirm with a human: which file owns the checkbox. Branches — if `CARD.md` is the task list, tick it there; if a `tasks.md` exists elsewhere outside this workspace, tick it there instead and leave `CARD.md` alone. **Default I proceed under:** edit `CARD.md` line 3 from `- [ ] **C3 · textkit helpers**` to `- [x] **C3 · textkit helpers**`, changing nothing else in the file, and record the absence of `tasks.md` in the report.
- Write `cycle-report.md` at the workspace root, in the format the cycle skill specifies. Honest content it must carry, whatever the outcome:
  - The red phase was inherited, not authored by me; I verified it (29 errors, all `NotImplementedError`) rather than assuming it, and I authored no tests because the card forbids editing the test file and the tests are the spec. This is a real deviation from writing the failing test myself and I name it as such.
  - The green ladder actually observed at each step (29 → 22 → 14 → 8 → 0), with any step that did not drop as predicted reported as it happened rather than smoothed over.
  - The minimalism check and its result, including that `shlex.split` was a genuine near-miss for `tokenize_query` and was ruled out by the card's single-import constraint and by its lack of a negation flag.
  - The two judgement calls no test pins: `-""` treated as a bare-dash `ValueError`, and an all-empty column (`rows == []`) falling on right alignment where it is unobservable.
  - Confirmation of the card's assert — `python3 -m unittest -q test_textkit` reporting `OK` over 29 tests — and confirmation of the constraint audit in Phase 5.
  - Anything I noticed but deliberately did not act on (for instance, `bool` being an `int` subclass, which would make a boolean column right-align; untested, out of scope, noted only).

## What I would refuse outright

- Editing `test_textkit.py`, for any reason, including to fix a test I disagree with.
- Adding a fifth public name, a class, a CLI entry point, or a second import to satisfy convenience.
- Deviating from the per-function approach the card fixes (e.g. solving `tokenize_query` with a single regex instead of a character pass) even where it would be shorter, since the card states the approach is settled.
- Reporting green without having seen `OK`, 29 tests, from an actual run.