FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orient and confirm the card's premises

**Do:** I have read all three workspace files (there are only three). Confirm the facts the card asserts, because the card's whole claim is "nothing is left to design":

- `textkit.py` holds exactly four public functions, all `raise NotImplementedError`, with `import re` already present. ✔ confirmed by reading.
- `test_textkit.py` imports exactly those four names and contains 4 test classes: 7 + 8 + 6 + 8 = **29 test methods**. ✔ matches the card's "29 errors, one per test".
- Brownfield exposure: none. Nothing else in the workspace exists to import `textkit`. ✔ confirmed — the workspace is only these three files, so there is no existing caller, no existing convention file, no config. I will not go looking for patterns that do not exist.

**Flag (procedural, not a blocker):** the card says the checkbox lives in `tasks.md`, but there is **no `tasks.md` in this workspace** — the cycle checkbox is line 3 of `CARD.md` (`- [ ] **C3 · textkit helpers**`). Default: I flip the checkbox in `CARD.md` and say so in the report. If a human wants a separate `tasks.md`, that is a one-line change either way; I would not create a new file on my own initiative.

**Stop point I would describe rather than pass:** where does `cycle-report.md` belong? There is no cycle directory here. Default: workspace root, `cycle-report.md`. If told it belongs under a cycle folder, I would write it there instead; nothing else in the plan changes.

## Phase 1 — Pre-code check, before any body is written

**Do:** ask, per function, whether the code needs to exist at all, and whether something already in reach does the job. Findings I would record verbatim in the report:

- `parse_duration` — nothing in the stdlib parses `"1h30m"`. `datetime.timedelta` constructs but does not parse; `time.strptime` is clock-format only. Must be written. `re` (already imported) does the shape check.
- `tokenize_query` — **`shlex.split` is a real candidate**: it handles whitespace splitting and would even produce `"foobar bazqux"` for `foo"bar baz"qux`. I rule it out and disclose why: (a) the card constrains this module to `re` as the only import, (b) `shlex` also honours backslash escapes and single quotes, which the tests do not specify and which would silently change behaviour, and (c) it has no notion of the leading-`-` negation flag or of rejecting a bare `-`, so a wrapper would end up nearly as large as the direct pass. Direct character pass it is.
- `render_table` — `str.ljust` / `str.rjust` are builtins and do the padding; I will not hand-roll padding arithmetic and will not reach for `textwrap` or any third-party table library.
- `merge_ranges` — builtin `sorted` with a key; no library needed.
- No new public names beyond the four. Any helper is module-private (leading underscore). No class, no CLI, no `if __name__` block in `textkit.py`.

**Refuse:** editing `test_textkit.py` in any way, for any reason. If a test looked wrong I would stop and report it rather than change it.

## Phase 2 — Re-establish red myself

**Do:** run `python3 -m unittest -q test_textkit` before touching anything.

**Expect:** 29 errors, every one `NotImplementedError`, exit status non-zero. This is me confirming the card's claim with my own eyes rather than taking it on trust — the count (29) and the *reason* (NotImplementedError, not ImportError, not a collection failure) both matter.

**Stop on:** any deviation — a different test count, an `ImportError`, a failure that is not `NotImplementedError`. That would mean the workspace is not what the card describes, and I would report the discrepancy rather than paper over it. Onward branch if the count differs but the reason is still `NotImplementedError` for every test: continue, and record the true count in the report as a correction to the card. Onward branch if the reason differs for any test: stop and report, since "seen to fail for the right reason" would be false.

## Phase 3 — Green, function by function

I take the card's fixed approach per function. Each function is written and the suite re-run for that class (`python3 -m unittest -q test_textkit.ParseDurationTest`, etc.) before moving on, so a failure is attributable to one body. Only `textkit.py` is written.

**3a · `parse_duration`** — normalise: strip all whitespace, lowercase. Shape gate: one `re.fullmatch(r"(\d+[dhms])+", norm)`; anything else (empty, `"   "` → empty after normalising, `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, `"1h m"` → `"1hm"`) fails the gate → `ValueError`. Then `re.findall(r"(\d+)([dhms])")`, walking pairs with a `seen` set; a repeated unit (`"1h1h"`) → `ValueError`. Sum against a private `_UNIT_SECONDS = {"d": 86400, "h": 3600, "m": 60, "s": 1}`. `"0s"` → `0`.

**3b · `tokenize_query`** — one left-to-right character pass with `in_quotes`, a `buf` list, `negated`, and a `started` flag. Inside quotes: `"` closes, everything else (whitespace included) buffers. Outside: whitespace flushes; `"` opens and marks the token started; `-` negates *only* when nothing has started yet (so `a-b` and `lang:rust` keep their punctuation literally); anything else buffers. Flush rule: non-empty buffer → append `(text, negated)`; empty buffer with `negated` set → `ValueError` (covers `"-"` and `"rust -"`); empty buffer without negation → emit nothing (covers `""` → `[]`). At end of input, `in_quotes` still true → `ValueError` (covers `'"unterminated`), checked *before* the final flush. `foo"bar baz"qux` falls out naturally as one buffer, `"foobar bazqux"`.

**3c · `render_table`** — validate first: empty `headers` → `ValueError`; any row whose length differs from `headers` → `ValueError`. Then per column compute the rendered cells via `str(cell)`, the width as the max over header and cells, and the alignment: right only when every *row* cell in that column is an `int`, else left — and the alignment applies to the header and the rule line too. One private `_format_row(cells, widths, aligns)` doing `rjust`/`ljust` with a two-space separator, used for the header, the `-`-rule row, and every data row. Every line right-stripped; joined with `\n`, no trailing newline.
*Ambiguity I would note, not guess at silently:* with zero rows, "every cell is an int" is vacuously true, so an empty table's columns nominally right-align — but the width then equals the header width exactly, so padding is a no-op and the output is identical either way. The tests cannot distinguish it. I take the vacuous-true reading (simplest code) and say so in the report.

**3d · `merge_ranges`** — a private validator per item: it must be a two-element pair, both bounds `int`, `start <= end`; otherwise `ValueError`. The type check runs *before* the ordering comparison, so `(1, "2")` raises `ValueError` rather than a `TypeError` leaking from `1 > "2"`. Then `sorted` by start, and one sweep holding an open `(start, end)`: while the next start is `<= end + 1`, extend `end` to `max(end, next_end)`; otherwise close and open a new one. Returns a list of tuples; `[]` → `[]`. Gap of one (`(1,3),(5,6)`) stays split because `5 > 4`; adjacency (`(4,6)` after `(1,3)`) merges.

## Phase 4 — Full suite, then refactor

**Do:** run `python3 -m unittest -q test_textkit`.
**Expect:** `OK`, 29 tests, exit 0 — exactly the card's assert.

**Refactor pass (tests stay green, re-run after):** the only things I would touch are naming and the shared row formatter — confirm `_UNIT_SECONDS`, `_format_row`, and the range validator are the only private names, that they earn their place, and that no dead branch survives. I would *not* generalise `render_table`, add type hints the module doesn't otherwise use, add a `__all__`, or "while I'm here" tidy the docstrings. If I spot something tempting, it goes in the report as a note, not into the diff.

**Verify the constraints hold:** `re` still the sole import; four public names only; no class; no CLI; `test_textkit.py` byte-identical to what I read.

## Phase 5 — Delegation call

I would **not** delegate here, and I would say so rather than leave it implicit. The green phase is four short bodies in one file that I have already reasoned through end to end; briefing a worker with the per-function approach, the fence, and the return shape costs more than writing the bodies, and I would still have to read the diff and re-run the suite myself before it counted. The cheap-read rung has nothing to do either — the workspace is three files and I have read all of them, so there is no sweep to farm out and no enumeration where a miss would matter. If the card had been ten files wide, `render_table` alone (decided approach, one test class judging it) would have been the natural piece to hand to a bounded worker with a sonnet override, fenced to `textkit.py` and `RenderTableTest`, returning the diff plus verbatim suite output — and I would have re-run that suite myself on its return before accepting it.

## Phase 6 — Close out

**Write:**
- `textkit.py` — the four bodies plus private helpers (the only source file changed).
- `CARD.md` — flip line 3 to `- [x]`, changing nothing else on the card.
- `cycle-report.md` — new, at workspace root.

**Report (honest record of what actually happened), covering:**
- Baseline red confirmed by me: 29 errors, all `NotImplementedError`; final green: `OK`, 29 tests, with the verbatim command and output for both.
- The cheapest-that-works decisions and what I passed over: `shlex.split` genuinely considered for `tokenize_query` and rejected for the import constraint plus unspecified escape/quote semantics; `str.ljust`/`rjust` and `sorted` used instead of hand-rolled equivalents; `datetime` no help for duration parsing.
- Deviations and flags: no `tasks.md` exists, so the checkbox was flipped in `CARD.md`; `cycle-report.md` placed at the workspace root absent a cycle directory.
- The one place the tests under-specify behaviour (column alignment when `rows` is empty) and the reading I took, so a reviewer can overrule it cheaply.
- Scope statement: `test_textkit.py` untouched; no public name added beyond the four; nothing implemented that the tests do not pin.
- No delegation was made, and why.