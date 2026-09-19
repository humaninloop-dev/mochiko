FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py` (and a glob of the workspace root, which contains only those three files)

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Baseline confirmation

**Do:** Re-read the three files (done above) and confirm the card's premise before writing anything.

- Count the test methods to check the card's "29" is real: 7 (`ParseDurationTest`) + 8 (`TokenizeQueryTest`) + 6 (`RenderTableTest`) + 8 (`MergeRangesTest`) = 29. ✔
- Confirm all four stubs raise `NotImplementedError` and that `re` is already the only import. ✔
- Run `python3 -m unittest -q test_textkit` once before touching code. **Expected:** 29 errors, every one a `NotImplementedError` raised from a `textkit.py` stub — not an `ImportError`, not a collection error.

**Stop condition:** if the baseline shows anything other than 29 `NotImplementedError` errors (e.g. a missing module, a syntax error, a different count), I stop and report the discrepancy rather than implementing against a spec whose baseline doesn't match the card. Branch: if the count differs only because the card is stale, I proceed and note the real number in the report; if a test errors for a *different* reason, that's a workspace problem and I surface it before writing code.

## Phase 1 — Pin the exact behaviour from the tests

No file changes; this is the arithmetic I do first so Phase 2 is a single clean write. Key derivations from `test_textkit.py`:

- **`parse_duration`** — `"  1H 30M "` (line 20) forces normalise = strip *all* whitespace, then lowercase. The rejected set (line 26) is covered by: shape check `(\d+[dhms])+` as a `fullmatch` kills `""`, `"   "`, `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"` (which normalises to `"1hm"`, where the trailing `m` has no digits). Only `"1h1h"` survives the shape check, so the repeated-unit check must be a separate pass. Units: d=86400, h=3600, m=60, s=1.
- **`tokenize_query`** — line 54 (`'foo"bar baz"qux'` → one token) proves a quote toggles a mode *inside* a token rather than starting a new one. Line 58 (`'""'` → `[]`) and line 61 (`"-"`, `"rust -"` → error) together mean the flush rule is: non-empty buffer → emit; empty buffer *with* a pending negation → `ValueError`; empty buffer without → emit nothing. So `-` must be tracked as a separate flag, not as a buffer character. Negation only applies to a `-` seen outside quotes with an empty buffer and no negation already pending; that keeps `a-b` and `lang:rust` literal (line 47).
- **`render_table`** — the docstring's "all `int` … (header included)" means the *header participates in the alignment*, not in the int test (headers are strings; `"qty"` is right-aligned at line 69). I verified the expected strings by hand:
  - line 69: widths 5/3, col 1 right → `"name   qty"` / `"-----  ---"` / `"apple    3"` / `"kiwi    12"`. ✔
  - line 72 (no rows): the int test is vacuously true, widths equal the header widths, so alignment is unobservable — no ambiguity. ✔
  - line 76: col 1 is `1` and `"n/a"` → left; the header line `"k  v  "` must right-strip to `"k  v"`. ✔
  - line 79: single all-int column, width 2 → `" n"` / `"--"` / `" 1"` / `"22"`. ✔
  - Rule cells are `"-" * width`, which fill the column exactly, so the rule can go through the same row formatter as the header and the data rows — the card's "one shared row formatter" holds.
- **`merge_ranges`** — merge when `next_start <= open_end + 1`; line 108 (gap of one stays split) and line 111 (`(5,5)`,`(6,6)` merge) fix the boundary exactly. Sort by start; extend with `max` so `(1,10),(2,3)` stays `(1,10)` (line 105). Return tuples.

## Phase 2 — Write the implementations

**Write:** `textkit.py` (the only file I modify). Keep the module docstring, drop its now-false "Each body below is a stub" paragraph. Keep `re` as the sole import. Public names stay exactly the four; anything else is `_`-prefixed.

Module level: `_UNIT_SECONDS = {"d": 86400, "h": 3600, "m": 60, "s": 1}`, plus compiled `_DURATION_SHAPE = re.compile(r"(\d+[dhms])+")` and `_DURATION_PAIR = re.compile(r"(\d+)([dhms])")`.

1. `parse_duration` — `re.sub(r"\s+", "", text).lower()`; `_DURATION_SHAPE.fullmatch(...)` or `ValueError`; then `_DURATION_PAIR.findall(...)`, accumulating into a total with a `seen` set of units, raising `ValueError` on a repeat.
2. `tokenize_query` — a `for ch in text` pass over `buf` / `negated` / `in_quotes`, with a nested `flush()` (closing over the three via `nonlocal`) called on whitespace and once after the loop; the unterminated-quote check sits between the loop and the final flush.
3. `render_table` — reject falsy `headers`; reject any row whose length differs from `len(headers)`; build per-column cell tuples via `zip(*rows)` (empty tuples when `rows` is empty); width = max of `len(str(cell))` over the cells and the header; right-align flag = `all(isinstance(cell, int) for cell in cells)`. Then one private `_format_row(cells, widths, right)` that `rjust`/`ljust`es each cell, joins on two spaces and right-strips; call it for the header, the rule, and each row; `"\n".join`.
4. `merge_ranges` — validate each pair (unpack to `start, end`; both `isinstance(..., int)`; `start <= end`), `sorted(...)`, one sweep extending `merged[-1]` while `start <= merged[-1][1] + 1`.

**Deliberately not doing:** no `isinstance(text, str)` guards, no `bool`-vs-`int` special-casing, no extra keyword arguments, no `__all__`, no CLI, no class. Nothing here is pinned by a test or asked for by the card, and each would be scope I invented. I'll note the `bool`-is-an-`int` wrinkle in `render_table`/`merge_ranges` in the final report as a known, untested edge rather than silently designing around it.

## Phase 3 — Run the suite

**Run:** `python3 -m unittest -q test_textkit`. **Expected:** `OK`, 29 tests, zero failures/errors — exactly the card's TEST line.

Branches if it isn't green:

- A `render_table` string mismatch → re-derive that case's expected padding by hand against the literal in the test and fix the formatter. The suspects are the right-strip and the two-space join.
- `tokenize_query` returning `[("", False)]` for `'""'` or not raising on `"rust -"` → the flush rule is wrong; correct the empty-buffer/pending-negation split.
- A `parse_duration` rejection leaking through → tighten the `fullmatch`, not the test.
- In every case I change `textkit.py` only. `test_textkit.py` is the spec and I do not edit it; if I ever conclude a test is *wrong*, I stop and raise it with the user instead of editing around it.

## Phase 4 — Constraint check

**Do:** re-read the finished `textkit.py` and verify against the card's constraints:

- `re` is the only import; stdlib only.
- Exactly four names without a leading underscore: `parse_duration`, `tokenize_query`, `render_table`, `merge_ranges`. `_UNIT_SECONDS`, `_DURATION_SHAPE`, `_DURATION_PAIR`, `_format_row` are private.
- No class, no `if __name__ == "__main__"` block, no argument parsing.
- `test_textkit.py` byte-identical to what I read (no writes were made to it at any point).
- Each function's stated approach matches the card: regex shape + `findall` walk; single character pass; widths-and-alignment-then-shared-formatter; validate, sort, one sweep.

## Phase 5 — Card checkbox (the one human-decision point)

The card is `- [ ] **C3 · textkit helpers**`. Ticking it is an edit to `CARD.md`, which is outside the two files named in the request, though the "not edited" constraint applies only to `test_textkit.py`.

**Default I proceed under:** once the suite is green, flip line 3 of `CARD.md` to `- [x]` and say so in the report — marking the card done is the card's own completion mechanism, and it's a one-character, trivially reversible change. **Branch if the user prefers cards left untouched:** revert that single character; nothing else in the deliverable depends on it. **Branch if the suite is not green:** the box stays unticked regardless, and the report leads with the failure.

## Delegation

None. This is one ~90-line module against a fixed 29-test spec in a three-file workspace; handing it to a subagent would cost more context than it saves and would put a second pair of hands on the only file I'm editing. I do all four functions myself in a single write.

## What I report at the end

- The suite result verbatim (`OK`, 29 tests) — or, if it isn't green, the actual failing output rather than a claim of success.
- The four functions as implemented, one line each on the approach actually taken, confirming it matches what the card fixed.
- Confirmation that `test_textkit.py` was not touched and that the stdlib-only / `re`-only / four-public-names / no-class / no-CLI constraints hold.
- Whether I ticked the C3 checkbox in `CARD.md`.
- The two untested edges I left alone by choice: `bool` counts as an `int` for `render_table` right-alignment and `merge_ranges` bound validation, and non-string input to `parse_duration`/`tokenize_query` raises `TypeError`/`AttributeError` rather than `ValueError`. Neither is pinned by a test; I flag them so the user can decide if they matter, and I don't change behaviour on my own to cover them.