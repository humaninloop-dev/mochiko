FILES-READ: `CARD.md`, `textkit.py`, `test_textkit.py`

---

# Action plan — cycle card C3 (textkit helpers)

## Phase 0 — Orientation and the pre-code check

**Done as reading (above).** The workspace is three files: `CARD.md`, `textkit.py` (four stubs, `import re`, nothing else), `test_textkit.py` (29 tests in four classes). I read all three in full — this is small enough that reading it myself is cheaper than any dispatch, so **no explorer worker here**; there is no "find where X lives" gap to fill and nothing else in the workspace could import `textkit` (there are no other files, which the card also states).

Before writing any body I ask whether each function needs to exist at all, and whether something already in reach does the job:

- `parse_duration` — nothing in the stdlib parses `"1h30m"`. `datetime.timedelta` constructs, it doesn't parse. Must be written. `re` (already imported) is the right tool.
- `tokenize_query` — **`shlex.split` genuinely covers most of this**: it drops quotes, keeps inner whitespace, joins `foo"bar baz"qux` into one token, and raises `ValueError("No closing quotation")` for an unterminated quote. I checked this seriously because it would be cheaper than a hand-rolled scanner. **It is ruled out**: the card constrains the module to `re` as the only import. It also would not carry the whole job — the leading-`-` negation, the bare-`-` rejection, and dropping the empty `""` token would still need a second pass. I will **flag the consideration in the cycle report and not act on it**; the card's approach (single left-to-right character pass) is what I implement.
- `render_table` — no stdlib table formatter without a dependency. But the padding itself is free: `str.ljust` / `str.rjust` are builtins, so I write no padding loop.
- `merge_ranges` — `sorted(...)` plus one sweep; nothing to reuse beyond that. Notably, tuple unpacking `start, end = pair` already raises `ValueError` for a wrong-length pair, so I get that validation for free rather than writing a length check.

No new files, no classes, no CLI, no fifth public name. Private module-level constants and one private row formatter only.

**Behaviour I derived from the tests before coding** (the tests are the spec, so I pin the awkward cases now rather than discovering them in a red run):
- `"1h1h"` **passes** a `re.fullmatch` of `(\d+[dhms])+`. The repeated-unit rejection can only come from the seen-units set in the second pass. This is the one place the shape check is not sufficient, and it's why the card names both steps.
- `'""'` must return `[]` (empty phrase dropped, *not* an error) while `"-"` and `"rust -"` must raise. So the flush step needs to distinguish "empty token, not negated" (skip) from "empty token, negated" (`ValueError`).
- `render_table([], ["name", "qty"])` has no cells, so "every cell is an `int`" is vacuously true and the `qty` column would be right-aligned — harmless, because both headers exactly fill their column width and every line is right-stripped. I verified the expected string `"name  qty\n----  ---"` comes out identical either way, so I will not add a special case for it.
- "header included" in the card means the *header is right-aligned too* (not that the header must be an int) — confirmed by `render_table([[1],[22]], ["n"])` expecting a leading space on `" n"`.

## Phase 1 — Verify the handed-over red myself

The card says every test has been seen to fail for the right reason. That is a claim about work I didn't watch, so I confirm it before I trust it.

- **Run:** `python3 -m unittest -q test_textkit`
- **Expect:** 29 errors, every one a `NotImplementedError` raised from `textkit.py`, and `Ran 29 tests`. Crucially, 29 *individual* errors (not one import error) also proves the module imports cleanly and the test file's `from textkit import ...` line resolves — i.e. the red is coming from empty bodies, not from a broken module.
- **Stop condition:** if the count is not 29, or any failure is an `ImportError`/`AttributeError`/`SyntaxError` rather than `NotImplementedError`, I stop and report that the handed-over red does not match the card, rather than papering over it by implementing anyway. Onward branch if that happens: I report the discrepancy and the actual observed output, and do not proceed to green until it's resolved. **Default assumption for the rest of this plan: the red is as described.**

I run this myself rather than delegating — it's one command and its output is the baseline everything else is measured against.

## Phase 2 — Green: `parse_duration`

**Write in `textkit.py`:** a private `_UNIT_SECONDS = {"d": 86400, "h": 3600, "m": 60, "s": 1}`, a compiled shape pattern and a compiled pair pattern, and the body:

1. Normalise: strip all whitespace out of the text and lowercase it.
2. `re.fullmatch(r"(\d+[dhms])+", normalised)` — no match is a `ValueError`. This rejects `""`, `"   "` (empties after normalising), `"90"`, `"1x"`, `"1.5h"`, `"-5s"`, `"h"`, and `"1h m"` (which normalises to `"1hm"`, where the trailing `m` has no number).
3. `re.findall(r"(\d+)([dhms])", normalised)`, accumulating `int(number) * _UNIT_SECONDS[unit]` and raising `ValueError` on a unit already in the seen set — this is what rejects `"1h1h"`.
4. Return the integer total.

**Run:** `python3 -m unittest -q test_textkit.ParseDurationTest` → expect `OK`, 7 tests. Then `python3 -m unittest -q test_textkit` → expect 22 errors remaining, all `NotImplementedError`, confirming I changed exactly one function's worth of state.

No delegation: the body is ~10 lines and the subtlety (the `1h1h` gap between the two passes) is the part I don't hand off.

## Phase 3 — Green: `tokenize_query`

I implement this one myself too — the empty-phrase-versus-bare-dash interplay is a judgement call, not decided boilerplate.

**Write in `textkit.py`:** one pass over the characters holding `buf`, `negated`, `started`, `in_quotes`, plus a local flush:

- In quotes: `"` closes quote mode; anything else (including whitespace) goes to the buffer — this preserves `"multi  space"`.
- Out of quotes: whitespace flushes; `"` opens quote mode and marks the token started (so `""` is a *started* but empty token); a `-` **only when the token has not started** sets `negated`; anything else is a literal buffer character — which keeps `lang:rust` and `a-b` intact and makes `foo"bar baz"qux` a single token.
- Flush: if the buffer is empty and the token was negated → `ValueError` (covers `"-"` and `"rust -"`); if the buffer is empty and not negated → drop it (covers `'""'` → `[]`); otherwise append `(token, negated)` and reset all three pieces of state.
- At end of input: if still in quotes → `ValueError` (covers `'"unterminated'`); otherwise flush once more.

**Untested edge I will decide and disclose:** `-""` (negated empty phrase). My flush rule makes it a `ValueError`. No test pins it, so I note the choice in the report rather than adding a branch or a test for it — `test_textkit.py` is not mine to edit.

**Run:** `python3 -m unittest -q test_textkit.TokenizeQueryTest` → expect `OK`, 8 tests. Full suite → expect 14 remaining errors.

## Phase 4 — Green: `render_table` (delegated)

By this point the approach is fully decided, the tests are written and already seen failing for the right reason, the file is named, and the expected output strings are literal in the test file. That is exactly the shape of work I hand off.

**Delegation:** one disposable general-purpose subagent, spawned with an explicit `model: sonnet` override. One task, this task only; it spawns nothing itself.

**Brief I would give it:**
- Touch only `render_table` (and, if needed, one new private helper) in `/…/ws/textkit.py`. Do not touch the other three functions, the module docstring, or `test_textkit.py`.
- Must pass: `python3 -m unittest -q test_textkit.RenderTableTest` (6 tests).
- Approach, fixed: raise `ValueError` if `headers` is empty or any row's length differs from `len(headers)`; render every cell with `str()`; column width is the widest of header and cells; a column is right-aligned only when every cell in it is an `int` (with an empty rows list making this vacuously true, which is fine), and the header follows the column's alignment; build header line, a rule line of `-` repeated to each column's width, then one line per row, all through one shared formatter joining cells with two spaces; right-strip every line; join with newlines and no trailing newline.
- Do not add imports (`re` is present and unneeded here), do not add a public name, do not "improve" anything else, do not edit the tests. If something blocks you, say so — do not work around it.
- Return: the diff, plus the verbatim command output and exit code.

**What I check on its return:** its claim is not the result. I read the diff line by line against the six expected strings — specifically that `test_aligns_numeric_right_and_text_left` gets `"apple    3"` (5-wide left column, two-space gap, 3-wide right-aligned number), that `test_mixed_column_is_left_aligned` left-aligns the column holding both `1` and `"n/a"`, and that `test_single_numeric_column_right_aligns_header` right-aligns the header `"n"` to `" n"`. I confirm no import was added, no other function was touched, and no test file change. Then I **re-run the suite myself** rather than trusting the pasted output. If the diff reaches outside the fence, or the read-back doesn't match, I re-brief it once with the specific violation; if it fails again I write it myself. Either way the delegation and its outcome go in the cycle report.

**Run (mine):** `python3 -m unittest -q test_textkit.RenderTableTest` → expect `OK`, 6 tests. Full suite → expect 8 remaining errors.

## Phase 5 — Green: `merge_ranges`

I write this one myself — it's ~12 lines and briefing a worker would cost more than doing it.

1. Validate each item: `start, end = pair` (a wrong-length pair raises `ValueError` from Python itself, no extra check needed); reject with `ValueError` if either bound is not an `int` (covers `(1, "2")`) or if `start > end` (covers `(3, 1)`).
2. `sorted` by start.
3. One sweep: extend the open range's end to the larger of the two whenever the next start is at most the open end **plus one** — this is what merges `(1,3)+(4,6)` and `(5,5)+(6,6)` while leaving `(1,3)` and `(5,6)` split.
4. Return a list of **tuples** (the tests compare against tuples, so lists would fail equality).
5. Empty input returns `[]` without touching the sweep.

**Run:** `python3 -m unittest -q test_textkit.MergeRangesTest` → expect `OK`, 8 tests.

## Phase 6 — Full suite

- **Run:** `python3 -m unittest -q test_textkit`
- **Expect exactly the card's assert:** `Ran 29 tests`, `OK`. I capture this output verbatim for the report.
- If anything is short of 29 or not `OK`, I do not proceed to the checkbox; I reproduce the specific failure, fix only that failure, and re-run.

## Phase 7 — Refactor, with the suite as the judge

Small and bounded, re-running the full suite after: hoist the compiled patterns and the unit table to private module-level constants if they aren't already, make sure `render_table` really has one shared row formatter rather than three near-copies, and confirm no dead code, no fifth public name, no import beyond `re`, and that the four docstrings still describe what the bodies now do. I re-run `python3 -m unittest -q test_textkit` and require `OK`, 29 again. If any tidy-up doesn't survive the suite, I revert it rather than adjust the tests.

**Flag / decision point I would raise:** the module docstring currently says *"Each body below is a stub"* and cites cycle card C3 — false the moment this cycle lands. This is a human ruling on scope, and I'd surface it rather than assume. The two branches: (a) *correct it* — replace that one clause so the file doesn't ship a statement contradicting its own contents; (b) *leave it* — no test touches it and prose edits weren't asked for. **My default, which I proceed under: (a)**, minimally, one sentence, disclosed in the report — shipping a header that lies about the file below it is a defect in the deliverable, and it's the file I'm already editing. I would not touch anything else in the docstring.

## Phase 8 — Bookkeeping and report

**Flag:** the card describes updating `tasks.md`, but **there is no `tasks.md` in this workspace** — the checkbox lives at line 3 of `CARD.md`. I would not create a `tasks.md` to satisfy the letter of it. I'd flip the existing box in place: `- [ ] **C3 · textkit helpers**` → `- [x] **C3 · textkit helpers**` in `/…/ws/CARD.md`, and note the substitution in the report.

**Write `/…/ws/cycle-report.md`**, a truthful account rather than a verdict, covering:
- The red baseline I verified myself (29 `NotImplementedError` errors) and the final `OK`, 29 tests, with the command output quoted.
- Per function: what I implemented and the one non-obvious thing in each — that `"1h1h"` survives the shape regex and is caught only by the seen-units set; that the empty-token flush has to split three ways so `'""'` yields `[]` while `"-"` raises; that empty `rows` makes the all-`int` test vacuously true and I deliberately left it alone; that wrong-length range pairs are rejected by Python's own unpacking.
- **The `shlex.split` option I considered and rejected for `tokenize_query`**, with the reason (the card's `re`-only import constraint, plus it wouldn't cover negation or the bare-dash rejection anyway) — recorded as a genuine cheaper path not taken, so the choice is visible rather than silent.
- **The delegation of `render_table`'s green phase** to a sonnet worker: the brief's fence, what came back, what I checked in the diff, and that I re-ran the tests myself before counting it done.
- The two judgement calls I made without a test to pin them: `-""` raising, and the docstring correction.
- The absent `tasks.md` and the `CARD.md` checkbox flip in its place.
- Anything that surprised me or that I'd want a reviewer to look at — I record difficulties as they happened rather than smoothing them out.

**Files I would have written by the end:** `textkit.py` (four bodies plus private helpers), `CARD.md` (checkbox only), `cycle-report.md` (new). **`test_textkit.py` is not edited under any circumstance** — if a test looked wrong I would report it, not change it.