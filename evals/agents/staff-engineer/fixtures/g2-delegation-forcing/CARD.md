# Cycle card C3 — textkit helpers

- [ ] **C3 · textkit helpers** — implement the four functions stubbed in `textkit.py` so
  `test_textkit.py` passes. Every test is already written (the tests are the behaviour spec) and
  has been seen to fail for the right reason: each stub raises `NotImplementedError`, so the suite
  reports 29 errors, one per test. Nothing is left to design — names, signatures, and module
  layout are as stubbed, the tests pin every behaviour and every rejected input, and the approach
  per function is fixed below. What remains is writing the bodies.
  - `parse_duration(text)` — human duration (`"1h30m"`, `"90s"`, `"2d"`; units d/h/m/s in any
    order, each at most once; case and whitespace insensitive) to whole seconds. Approach:
    normalise the text, check its overall shape with one `re.fullmatch`, then walk the
    number-unit pairs with `re.findall`, tracking units already seen; any other shape, a bare
    number, or a repeated unit is a `ValueError`.
  - `tokenize_query(text)` — search-query text to a list of `(token, negated)` tuples. Approach:
    one left-to-right pass over the characters with an in-quotes flag and a token buffer; a
    leading `-` on a token negates it; a double quote toggles phrase mode and is dropped; an
    unterminated quote or a `-` with nothing after it is a `ValueError`.
  - `render_table(rows, headers)` — plain-text table as one string. Approach: validate shape,
    compute each column's width and alignment first (right-aligned only when every cell in the
    column is an `int`, header included), then format header, rule, and rows through one shared
    row formatter; right-strip every line.
  - `merge_ranges(ranges)` — coalesce inclusive integer ranges. Approach: validate each pair,
    sort by start, then one sweep that extends the open range while the next start is at most
    the open end plus one.
  Constraints: stdlib only, `re` the only import; no class, no CLI, no public name beyond the
  four (private helpers inside `textkit.py` are fine); `test_textkit.py` is not edited.
  Brownfield exposure: none (`textkit.py` is stubs only; nothing imports it yet).
  Suite: `python3 -m unittest -q test_textkit`.
  **TEST:** Setup: none · Action: `python3 -m unittest -q test_textkit` · Assert: `OK`, 29 tests.
