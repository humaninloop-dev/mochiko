FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-25mksxxo/ws/CARD.md`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-25mksxxo/ws/textkit.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-25mksxxo/ws/test_textkit.py`

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orientation (already done above)

Read all three files in the workspace: `CARD.md`, `textkit.py`, `test_textkit.py`. That is the entire workspace (a glob confirms only those three files exist).

Two things I establish up front from that reading:

- The card's brownfield note is accurate: `textkit.py` is four stubs plus a module docstring and `import re`; nothing else in the workspace imports it. So there is no existing implementation to preserve, and no caller whose interface I could break. The one existing-code constraint that *does* bind is that `test_textkit.py` is the spec and must not be touched.
- I counted the test methods myself rather than trusting the card: 7 (`ParseDurationTest`) + 8 (`TokenizeQueryTest`) + 6 (`RenderTableTest`) + 8 (`MergeRangesTest`) = **29**. This matches the card's claim of 29 tests, so the card's "seen to fail" record is at least internally consistent with the file in front of me.

**No delegation in this phase or any other.** The whole workspace is three short files and I have read all of them. There is no locate, no enumeration, and no bounded quote to farm out — spawning a cheap reader here would cost more than it saves. I note this explicitly so the absence is a decision, not an oversight.

## Phase 1 — Pre-code check: does this code need to exist, and can something cheaper do it?

Before writing a line, I run each of the four functions past "delete / reuse / stdlib / platform / dependency":

- **`parse_duration`** — nothing in the standard library parses `"1h30m"`. `datetime.timedelta` constructs from keyword units but does not parse this surface form. Writing it is necessary. I *do* reach for the platform where I can: one `re.fullmatch` for shape and one `re.findall` for the pairs, rather than a hand-rolled scanner.
- **`tokenize_query`** — **this one has a real stdlib candidate: `shlex`.** `shlex.split` already handles quote toggling mid-token (`'foo"bar baz"qux'` → `['foobar bazqux']`) and already raises `ValueError` on an unterminated quote. It would satisfy several of these tests directly. I am **rejecting it, and I will say so in the report rather than quietly not mentioning it**, for two reasons: (a) the card constrains this module to `re` as the only import, and the approach is fixed as a single character pass — adding `shlex` is a design change the card did not authorise; (b) on merit it diverges from the documented behaviour anyway — `shlex` also treats single quotes as quoting and backslashes as escapes, neither of which the docstring describes, and it returns `['']` for `'""'` where the tests want `[]`. So it is not actually a drop-in. It would need a wrapper for negation, empty-token dropping, and quoting-style suppression, at which point the character pass is both smaller and truer to spec. **If a reviewer rules that the import constraint is soft and `shlex` is preferred**, the branch is: keep the same public signature, replace the pass with `shlex.split` under a configured `shlex` instance with `punctuation_chars` and quote characters restricted to `"`, strip a leading `-` per token, drop empty tokens — and I would re-run the suite to confirm the single-quote and backslash behaviours are not exercised. Absent that ruling I proceed with the character pass as the card specifies.
- **`render_table`** — no stdlib table renderer. But I will not hand-roll padding: `str.ljust` / `str.rjust` and `str.join` do the work, and `str.rstrip` does the trailing-space rule.
- **`merge_ranges`** — no stdlib range coalescer. `sorted()` plus one sweep is the whole algorithm.

Outcome: four small function bodies, plus at most one private row formatter inside `render_table`'s neighbourhood. No new module, no class, no new import, no public name beyond the four. `import re` is already at the top of `textkit.py` and I add nothing to it.

## Phase 2 — Re-verify the red, do not take it on faith

**Run:** `python3 -m unittest -q test_textkit`

**Expect:** `Ran 29 tests`, `FAILED (errors=29)`, every error a `NotImplementedError` raised from `textkit.py`.

This is the one point where I would stop before writing any code. The card asserts the red phase was already observed; I re-observe it because a red I have not seen myself is a red I cannot vouch for, and because it also proves the import line in the test file resolves and the suite actually collects 29 tests.

**Branches if the run disagrees:**
- *Fewer or more than 29 tests collected* → stop and flag. The card's test count and the file disagree, which means I am not looking at the workspace the card was written against.
- *Any failure that is not `NotImplementedError`* (e.g. an `ImportError`, or a test failing on an assertion) → stop and flag before implementing. A pre-existing non-stub failure means something other than "the bodies are missing" is wrong, and implementing over it would hide it.
- *Anything green already* → stop and flag; a passing test against a `raise NotImplementedError` body would mean the test does not test what it claims.

Default if the run matches expectations: proceed straight to Phase 3 without writing any new test, because the tests are the spec and the card forbids editing `test_textkit.py`. **I write no tests in this cycle.** The red exists; my job is the green. I will say plainly in the report that the red step was inherited and re-verified rather than authored by me.

## Phase 3 — `parse_duration`, to green

**Write to:** `/private/var/folders/.../ws/textkit.py`, replacing only the `raise NotImplementedError` in `parse_duration`. Docstring untouched.

Intended body:

1. Normalise: strip all whitespace with `re.sub(r"\s+", "", text)`, then `.lower()`.
2. Shape gate: `re.fullmatch(r"(\d+[dhms])+", normalised)` — on no match, `raise ValueError`.
3. Walk pairs: `re.findall(r"(\d+)([dhms])", normalised)`, accumulating into a `seen` set; a unit already in `seen` is a `ValueError`.
4. Multiply through a module-level-or-local mapping `d=86400, h=3600, m=60, s=1`; return the int sum.

Why each rejected input in `test_rejects_malformed` falls out, checked one by one before I run anything: `""` and `"   "` normalise to empty and the `+` in the pattern requires at least one group; `"90"` has no unit letter; `"1x"`, `"1.5h"`, `"-5s"`, `"h"` all fail the shape gate; `"1h m"` normalises to `"1hm"` where `m` has no digits, so the gate rejects it; `"1h1h"` passes the gate and is caught by the repeated-unit check. `"0s"` → 0 and `"30m1h"` → 5400 need no ordering logic because the sum is order-independent.

**Run:** `python3 -m unittest -q test_textkit`. **Expect:** `Ran 29 tests`, `FAILED (errors=22)` — the 7 duration tests now pass, the other 22 still error on `NotImplementedError`. If any duration test *fails* (assertion) rather than passing, I fix the body, not the test.

## Phase 4 — `tokenize_query`, to green

Single left-to-right character pass. State: `buf` (a list, or `None` for "no token in progress"), `in_quotes`, `negated`, and an output list. A flush helper closes the current token.

- In quotes: `"` closes quote mode; every other character, including whitespace and `-`, appends literally.
- Out of quotes: whitespace flushes; `"` starts the buffer if it is `None` and toggles quote mode, and is itself dropped; `-` sets `negated` **only** when no token is in progress and `negated` is not already set, and deliberately does *not* start the buffer; anything else starts the buffer if needed and appends.
- Flush: join the buffer (empty if `None`); a non-empty token appends `(token, negated)`; an **empty** token with `negated` set is the bare-dash `ValueError`; an empty token without negation appends nothing. Then reset `buf` and `negated`.
- After the loop: `raise ValueError` if still `in_quotes`, then one final flush.

Traced against the tests before running: `"lang:rust a-b"` keeps `-` literal because a token is already in progress at that character; `'foo"bar baz"qux'` yields one token because the quote never starts a new buffer; `'""'` starts an empty buffer and flushes to nothing, giving `[]`; `"-"` and `"rust -"` reach the final flush with an empty buffer and `negated` true, so both raise; `'"unterminated'` is caught by the end-of-loop quote check.

One behaviour the tests do not pin and I will **not** write code for: `-""` (negation on an explicitly empty phrase). Under the rule above it raises. I note this in the report as unspecified-by-spec rather than inventing a rule for it.

**Run:** `python3 -m unittest -q test_textkit`. **Expect:** `Ran 29 tests`, `FAILED (errors=14)`.

## Phase 5 — `render_table`, to green

Validation first: empty `headers` → `ValueError`; any row whose length differs from `len(headers)` → `ValueError`. Both are exercised by `test_rejects_ragged_rows_and_empty_headers`, including the `render_table([], [])` case which the empty-headers check catches.

Then, per column, computed once:
- **Alignment** — right only if every *data* cell in that column is an `int`; the header is then right-aligned too. I verified this reading against `test_single_numeric_column_right_aligns_header`, which expects `" n"` — so "header included" means the header inherits the alignment, not that the header participates in the int test (it never could, being a string).
- **Width** — `max` of the header length and every `str(cell)` length.

One shared row formatter takes a list of already-stringified cells and emits `"  ".join(...)` with each cell `rjust`/`ljust`ed to its width, then `.rstrip()`. Header, rule (`"-" * width` per column), and every data row all go through it. Join with `"\n"`, no trailing newline.

Hand-check against the first test before running: widths 5 and 3; header `"name "` + `"  "` + `"qty"` → `"name   qty"`; rule `"-----  ---"`; rows `"apple    3"` and `"kiwi    12"` after right-stripping. Matches the expected string exactly.

**One unobservable choice I will flag rather than bury:** with `rows=[]`, "every cell is an int" is vacuously true, so the columns come out right-aligned. This is invisible in `test_header_and_rule_only_when_no_rows` because with no data the widths equal the header widths and padding is zero either way. I record the choice in the report so a later reader is not surprised by it. Similarly, `bool` is an `int` subclass and would right-align; not tested, not specified, and I add no code to special-case it.

**Run:** `python3 -m unittest -q test_textkit`. **Expect:** `Ran 29 tests`, `FAILED (errors=8)`.

## Phase 6 — `merge_ranges`, to green

Validate every pair first, before any sorting, so a bad pair raises regardless of where it sorts: unpack `start, end` (which raises `ValueError` on its own for a wrong-length pair — free, so I write no length check), require `isinstance(start, int)` and `isinstance(end, int)` (this is what rejects `(1, "2")`; a bare `<` comparison against a string would raise `TypeError`, not the `ValueError` the test wants), and require `start <= end` (rejects `(3, 1)`).

Then `sorted(...)` by start and one sweep: hold an open `(start, end)`; for each next pair, if `next_start <= open_end + 1` extend `open_end = max(open_end, next_end)`, else emit the open range and open a new one. Emit the last. Return a list of tuples.

Checked against the tests: `(1,3),(4,6)` merges because `4 <= 4`; `(1,3),(5,6)` stays split because `5 > 4`; `(5,5),(6,6)` merges to `(5,6)`; the contained/duplicate case collapses to `(1,10)` via the `max`; `[]` returns `[]` without entering the sweep.

**Run:** `python3 -m unittest -q test_textkit`. **Expect:** `Ran 29 tests`, `OK` — this is the card's stated assertion.

## Phase 7 — Refactor, still green

With all four green, one pass for clarity only, no behaviour change: confirm the unit-multiplier mapping and the row formatter are the only helpers, that every helper name is private (leading underscore) so the module's public surface is still exactly the four functions, that `re` remains the only import, and that no class or CLI entry point crept in.

**One small edit I intend to make and will disclose:** the module docstring's second paragraph says "Each body below is a stub" — false once this cycle lands. I would delete that sentence and leave the first line intact. It is a two-line truth-fix to a comment describing the code I just changed, not a refactor of anything. I flag it in the report and would revert it on request.

**Re-run:** `python3 -m unittest -q test_textkit`. **Expect:** unchanged `OK`, 29 tests.

## Phase 8 — Verify the constraints I was given

- Confirm `test_textkit.py` is byte-for-byte unedited (I never open it for writing; I re-read it and compare against what I read in Phase 0).
- Confirm `textkit.py` imports only `re`.
- Confirm no new files were created beyond the report.
- Re-run the suite one final time cold: `python3 -m unittest -q test_textkit` → `OK`, `Ran 29 tests`.

**I would refuse** any repair that involved editing `test_textkit.py` to make a test pass. If I hit a test I genuinely cannot satisfy without changing it, I stop and report the conflict rather than adjusting the spec to fit my code.

## Phase 9 — Card checkbox and report

**Discrepancy I would flag, and how I would handle it:** my normal output includes ticking the completed card in `tasks.md`, but **there is no `tasks.md` in this workspace** — the card lives in `CARD.md` and carries its own checkbox on line 3. My default: tick that checkbox in place, changing `- [ ] **C3 · textkit helpers**` to `- [x] **C3 · textkit helpers**` in `/private/var/folders/.../ws/CARD.md`, and record in the report that no `tasks.md` exists so the tick landed on `CARD.md` instead. **If the ruling is that `CARD.md` is an input I should not modify**, the branch is: leave `CARD.md` untouched and state the completed status in the report only. I would confirm this before writing if a human were available; absent one I proceed with the in-place tick, which is reversible and visible.

**Write:** `/private/var/folders/.../ws/cycle-report.md` — an honest record of what happened, not a verdict that it passed. Substantively it would carry:

- The red phase was **inherited, not authored**: the tests pre-existed, and I re-ran them to see the 29 `NotImplementedError` errors myself before writing any body. I wrote zero new tests, because the card designates the existing suite as the spec and forbids editing it.
- The green sequence and the error count falling 29 → 22 → 14 → 8 → 0 as each function landed, so the drop attributable to each function is visible.
- The **`shlex` decision**: a real stdlib option for `tokenize_query`, considered and rejected, with both reasons (the card's single-import constraint, and the genuine divergence on single quotes, backslashes, and the `'""'` → `[]` case).
- The two unspecified behaviours I chose without a test pinning them: vacuous right-alignment for a zero-row table, and `-""` raising. Both stated as choices, not as facts.
- `bool`-as-`int` in `render_table` noted as an opportunity **observed and not acted on**, since no task asked for it.
- The `tasks.md`-absent deviation and where the tick actually went.
- The docstring sentence I removed and why.
- Final suite state: `python3 -m unittest -q test_textkit` → `OK`, 29 tests, matching the card's assertion.