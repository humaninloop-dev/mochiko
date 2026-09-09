FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Ground the card against the workspace (done during reading)

**What I did/would do:** read all three files in the workspace — there are only three (`CARD.md`, `textkit.py`, `test_textkit.py`), so the whole surface fits in my head and nothing is hidden.

Facts I confirmed and would carry into the work:

- `textkit.py` has exactly four stubs, each `raise NotImplementedError`, and one import: `re`. No other names, no class, no `__main__` block.
- `test_textkit.py` has 4 test classes with 7 + 8 + 6 + 8 = **29 test methods**, matching the card's "29 errors, one per test" claim.
- The card's brownfield note ("nothing imports it yet") is verifiable directly: the only importer in the workspace is `test_textkit.py` line 3. So this is greenfield-inside-a-stub — I read the stubs' docstrings as the interface contract and will preserve every signature exactly as stubbed.

**Delegation decision:** none. The usual reason to hand a sweep to a cheap `Explore` subagent — "who else imports this?", "where does this pattern live?" — evaporates in a three-file workspace I have already read in full. Spawning a Haiku reader here would cost more than it saves and would return facts I already hold. I will state this in the report rather than silently skipping it.

**What I would refuse up front:**
- Editing `test_textkit.py` in any way. The tests are the spec; if a test looked wrong I would stop and report, not adjust it.
- Adding any import beyond `re`, adding a class, a CLI, `__all__`, type annotations-as-scope-creep, or any fifth public name. Private `_`-prefixed helpers inside `textkit.py` are permitted and I will use them sparingly.
- Touching the module docstring's meaning or the four function docstrings, which already state the contract.

---

## Phase 1 — Re-establish red myself

**Run:** `python3 -m unittest -q test_textkit`

**Expect:** 29 errors, every one a `NotImplementedError` raised from a `textkit.py` stub — not an `ImportError`, not a collection failure, not 28 or 30. This is cheap and it confirms the card's premise is true of *this* checkout rather than taken on faith.

**Stop condition:** if the count is not 29, or any failure is something other than `NotImplementedError` from the stubs (e.g. an import error, a syntax error, a genuine assertion failure), I stop and report the discrepancy between the card and the workspace before writing a line of implementation. The card asserts a specific starting state; if reality differs, the rest of the card's reasoning may differ too.

I will not write new tests. The card is explicit that the red phase is complete and the tests are the behaviour spec; adding my own duplicate tests would be scope creep, and the discipline that matters here is *not implementing ahead of a failing test*, which the existing 29 already enforce.

---

## Phase 2 — Cheapest-that-works pass before any code

For each of the four, I ask whether it needs writing at all:

- **`parse_duration`** — no stdlib parser for `"1h30m"` shapes (`datetime.timedelta` doesn't parse; `time.strptime` is wrong-shaped and would need a second import). Must be written. `re` is already imported and is the card's prescribed tool.
- **`tokenize_query`** — `shlex.split` is the obvious "already exists" candidate and it does handle quotes. I reject it: it would need a second import (barred), it raises `ValueError` on unterminated quotes but *not* on a bare `-`, it does not carry the negation flag, and critically it would split `foo"bar baz"qux` differently from what `test_quotes_toggle_mid_token` pins. The character pass the card prescribes is both cheaper and correct.
- **`render_table`** — no stdlib table renderer (`str.format` / f-string padding is the primitive, and I'll lean on `str.ljust`/`rjust`).
- **`merge_ranges`** — no stdlib interval merge; `sorted` plus one sweep is the whole job.

**Conclusion:** all four bodies get written, each thin, using only `re`, `sorted`, and string methods. I'd record this reasoning — particularly the `shlex` rejection — in the report, because "why didn't you reuse the obvious thing" is exactly the question a reviewer should ask.

---

## Phase 3 — Green, one function at a time, verifying after each

I implement in the card's order, running the suite after each function so each step is independently verified rather than one big-bang edit. Each edit is to **`/…/ws/textkit.py`** only.

### 3a. `parse_duration(text)`

Body: a module-level private `_UNIT_SECONDS = {"d": 86400, "h": 3600, "m": 60, "s": 1}`; normalise by stripping all whitespace and lowercasing; one `re.fullmatch` of a repeated number-unit shape against the normalised text; then `re.findall` of number/unit pairs, accumulating into a total while tracking units already seen and raising `ValueError` on a repeat.

Why this shape satisfies each rejection in `test_rejects_malformed`:
- `""` and `"   "` normalise to empty → the one-or-more shape can't match.
- `"90"` — no unit → no match. `"h"` — no number → no match. `"1x"` — bad unit → no match. `"1.5h"` — the `.` isn't in the shape. `"-5s"` — the leading `-` isn't in the shape.
- `"1h m"` normalises to `"1hm"` → trailing bare `m` fails the fullmatch. (This one is worth calling out: whitespace-stripping happens *before* the shape check, which is precisely why this input still fails.)
- `"1h1h"` passes the shape and is caught by the seen-units set.

**Run:** suite. **Expect:** 7 `ParseDurationTest` tests pass, 22 errors remain.

### 3b. `tokenize_query(text)`

Body: single left-to-right character loop with `in_quotes` flag, a list buffer, and a `negated` flag; a private `_flush` closure/helper that raises `ValueError` when `negated` is set but the buffer is empty (the bare-`-` case) and otherwise appends `("".join(buf), negated)` only when the buffer is non-empty.

Rules in the loop: `"` toggles `in_quotes` and is never appended; whitespace outside quotes flushes, inside quotes is appended literally; `-` sets `negated` only when we're outside quotes at the very start of a token (empty buffer, not already negated) and is otherwise a literal character; anything else appends. After the loop: `in_quotes` still true → `ValueError` for the unterminated quote, checked *before* the final flush; then final flush.

Why this satisfies the pinned cases: `"a-b"` and `"lang:rust"` keep their punctuation because the `-`/`:` aren't token-initial; `'foo"bar baz"qux'` yields the single glued token because quotes only toggle a mode and never break the buffer; `'""'` yields `[]` because the flush drops an empty buffer; `'-"cargo build" rust'` yields the negated phrase because `-` is seen while the buffer is empty and outside quotes.

**Run:** suite. **Expect:** 15 passing, 14 errors.

### 3c. `render_table(rows, headers)`

Body order: validate (`not headers` → `ValueError`; any row whose length differs from `len(headers)` → `ValueError`); build a per-column list of rendered cell strings via `str(cell)`; per column, width = max of header and cell string lengths, and right-align that column when every *row cell* in it is an `int` (the header string tags along with whatever alignment the column got — that's what `test_aligns_numeric_right_and_text_left` pins with `qty` sitting right-aligned above ints); then a single private `_format_row(values, widths, right)` that pads each value and joins with two spaces, right-stripping the result. Header line, rule line (`"-" * width` per column, through the same formatter), then one line per row; `"\n".join(...)` with no trailing newline.

Two judgement calls I would record rather than code around:
- With `rows == []`, "every cell is an int" is vacuously true, so both columns nominally right-align — but every value is exactly its column width, so the rendering is byte-identical either way and `test_header_and_rule_only_when_no_rows` passes. I will not add a special case for a distinction that cannot be observed.
- `isinstance(cell, int)` also admits `bool`. No test exercises it and the docstring says `int`; I take the plain reading and add no extra guard.

**Run:** suite. **Expect:** 21 passing, 8 errors.

### 3d. `merge_ranges(ranges)`

Body: first pass validates — unpack each pair, require both bounds `isinstance(..., int)` (this is what rejects `(1, "2")`) and `start <= end` (rejects `(3, 1)`), else `ValueError`. Then `sorted(...)` by start, then one sweep holding an open `(start, end)`: while the next start is at most the open end **plus one**, extend the open end to the max of the two; otherwise emit and reopen. Return a list of tuples.

This gives: `(1,3)+(4,6)` merge (adjacent), `(1,3)+(5,6)` stay split (gap of one), `(5,5)+(6,6)` merge to `(5,6)`, containment and duplicates collapse via the `max`, unsorted input handled by the sort, `[]` returns `[]`.

**Run:** suite. **Expect:** `OK`, **29 tests** — the card's stated assertion.

---

## Phase 4 — Refactor

With the suite green I make one tidying pass and re-run after it:

- Keep `_UNIT_SECONDS` and `_format_row` as the only private helpers; collapse anything that turned out duplicated between the header/rule/row paths in `render_table` into that one formatter.
- Verify the constraint list by inspection: still exactly one import (`re`), still exactly four public names, no class, no CLI block, `test_textkit.py` untouched.
- **Run:** `python3 -m unittest -q test_textkit` once more. **Expect:** unchanged `OK`, 29 tests. If refactoring moved the result at all, I revert to the green state rather than chase it.

---

## Phase 5 — Card checkbox

**Stop for confirmation:** the card names an updated `tasks.md`, but **no `tasks.md` exists in this workspace** — the `- [ ]` checkbox for C3 lives on line 3 of `CARD.md`. I would confirm that ticking `CARD.md` is the intended equivalent.

- If confirmed (my default, and how I proceed): edit `/…/ws/CARD.md` line 3, `- [ ]` → `- [x]`, changing nothing else in the file.
- If the ruling is that a `tasks.md` exists elsewhere and should be the tracker: I'd leave `CARD.md` alone and tick the corresponding entry there instead.
- If the ruling is that neither should be touched: I skip the edit and say so in the report.

I would not create a `tasks.md` — inventing a tracking file nobody asked for is exactly the kind of "while I'm in here" addition I avoid.

---

## Phase 6 — Report

**Write:** `/…/ws/cycle-report.md` — a truthful account, not a verdict. It would carry:

- The verified red state from Phase 1 (29 errors, all `NotImplementedError`) and the green state after each of 3a–3d, with the final `OK` / 29 tests.
- The reuse-before-writing reasoning, including the explicit **rejection of `shlex.split`** for `tokenize_query` and why (extra import, wrong glued-token behaviour, no bare-`-` rejection, no negation flag) — so the choice is auditable rather than assumed.
- The two `render_table` judgement calls left uncoded: the invisible vacuous right-alignment when `rows` is empty, and `bool` passing `isinstance(x, int)`. Both unexercised by tests; both deliberate non-additions.
- The `tasks.md`/`CARD.md` discrepancy and what I did about it.
- The no-delegation decision and its reason (three-file workspace, fully read).
- Confirmation that `test_textkit.py` is byte-unchanged and that the module still has one import and four public names.
- Anything that went differently from the plan — for instance, if the whitespace-then-shape ordering in `parse_duration` or the flush ordering in `tokenize_query` had to be reworked to satisfy `"1h m"` or `'""'`, that rework gets written down rather than smoothed over.

**Final message to the requester:** cycle C3 complete, four bodies implemented, suite `OK` at 29 tests, card checkbox ticked in `CARD.md` (flagged: no `tasks.md` in this workspace), plus the two noted judgement calls carried forward for review.