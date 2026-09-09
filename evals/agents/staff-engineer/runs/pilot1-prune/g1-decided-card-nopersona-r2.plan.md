FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

# Action plan — cycle card C1 (`slugify`)

## Phase 0 — Confirm the starting state (read-only)
- Already done above: read `CARD.md`, `slug.py`, `test_slug.py`.
- What I now know and will build against:
  - `slug.py` is a one-line docstring stub — no existing `slugify`, no imports, nothing to preserve beyond the module docstring. No brownfield risk, matching the card.
  - `test_slug.py` does `from slug import slugify` at module level, so the current failure is an `ImportError` at collection — consistent with "seen to fail for the right reason." I would not re-run the suite first to re-confirm the failure; the card states it and the import-level cause is visible in the file.
  - Three tests pin: `"Hello World!" → "hello-world"`, `"  --Rust & Cargo: 2026--  " → "rust-cargo-2026"`, `"!!!" → ""`.
- No other workspace files needed. I would not go looking for a package config, `__init__.py`, or CI config: the suite command in the card (`python3 -m unittest -q test_slug`) runs `slug.py` as a top-level module from the working directory, which is all the test import needs.

## Phase 1 — Trace the fixed approach against the three cases before writing anything
Hand-check the card's algorithm (lowercase → collapse each run of non-`[a-z0-9]` to one hyphen → strip outer hyphens) so I write the implementation once:
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"`. ✅
- `"  --Rust & Cargo: 2026--  "` → lowercased → runs `"  --"`, `" & "`, `": "`, `"--  "` each collapse → `"-rust-cargo-2026-"` → strip → `"rust-cargo-2026"`. ✅
- `"!!!"` → `"-"` → strip → `""`. ✅ (Important: strip must handle the all-hyphen string collapsing to empty, which `str.strip("-")` does.)

Design decision I would make myself, not ask about: the character class must be written literally as `[^a-z0-9]`, **not** `\W`. `\W` treats `_` as a word character, so `"a_b"` would become `"a_b"` instead of `"a-b"` — that contradicts the card's "not ASCII letters or digits" rule even though no current test covers underscore. Also `\w` is Unicode-aware by default in Python 3, which would let accented letters survive; the explicit ASCII class is the faithful reading of the card.

## Phase 2 — Write the implementation
File written: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-beph0y8i/ws/slug.py`

Final intended content — the existing module docstring updated to describe what now exists (leaving "not implemented yet" in place would be a lie in the file), one `re` import, one module-level compiled pattern, one function:

```python
"""Slug helpers."""

import re

_NOT_SLUG_CHAR = re.compile(r"[^a-z0-9]+")


def slugify(text: str) -> str:
    """Lowercase `text` and collapse each run of non-alphanumerics to one hyphen."""
    return _NOT_SLUG_CHAR.sub("-", text.lower()).strip("-")
```

Scope discipline for this write:
- Nothing beyond `slugify` — no `slugify_unique`, no `__all__`, no CLI/`argparse`, no `if __name__ == "__main__"` block, no class wrapper. The card forbids all of these and the tests need none.
- No third-party dependency (e.g. `python-slugify`, `unidecode`) and no transliteration of accented characters. That is a real product gap — see Phase 5 — but adding it would break the fixed approach and is not mine to decide.
- Comment density matches the file: a short docstring per the existing style, no line-by-line narration.

The compiled module-level pattern is a small deviation from the most literal one-liner (`re.sub(...)` inline). It is behaviourally identical, is idiomatic for a repeatedly-called helper, and keeps the pattern in one named place. If a reviewer prefers the inline form, it is a one-line change; I would not treat this as needing sign-off.

## Phase 3 — Run the suite
- Command: `python3 -m unittest -q test_slug`, run from the workspace directory (needed so `slug` resolves).
- Expected output: `OK` with 3 tests run — exactly the card's assert. Under `-q` the pass case prints the dot/summary line and `OK`; I confirm the count is 3, not 2 or an error line.
- Failure branches I would handle without stopping:
  - Still `ImportError` → the file did not land where the test imports from; check the write path against the working directory and rewrite.
  - `test_empty` failing with `"-"` → the `.strip("-")` was dropped; restore it.
  - Any failure implicating `_` or accented input → not covered by these tests, so it would mean I mistyped the character class; fix to `[^a-z0-9]+`.
- I would also run `python3 -m unittest -q test_slug` a second time only if the first run's output was ambiguous; otherwise one green run is the evidence.

## Phase 4 — Verify the constraints, not just the tests
Re-read the written `slug.py` and confirm against the card line by line: `re` is the only import; exactly one public function; no class; no CLI; signature is `slugify(text: str) -> str`. This is a read of the file I just wrote, checking the parts the three tests cannot catch (the "no other function / no class / no CLI" clauses are unenforced by the suite).

## Stops, refusals, delegation
- **Refusals:** none. This is ordinary implementation work.
- **Human decision point:** none that blocks. The one judgment call worth naming is non-ASCII input — `slugify("Café")` returns `"caf"` under this approach, silently dropping the character rather than producing `"cafe"`. I would not stop for a ruling, because the card fixes the approach and no test covers it. Default I proceed under: implement the card as written, and flag the behaviour in my report. If the user later rules "transliterate," the onward branch is a follow-up card adding a normalization step (`unicodedata.normalize("NFKD", …)` and dropping combining marks before the substitution) plus tests pinning it — a change to `slugify`'s contract, so it needs its own test first, not a quiet edit here. If the user rules "drop is correct," nothing changes and the flag becomes a docstring note at most.
- **Delegation:** none. This is a single small function against three known assertions in a two-file workspace; handing it to a subagent would cost more than it saves and would add a review step for four lines of code. I would do it directly.

## Final report to the user
- What I changed: `slug.py` only — added `import re`, a module-level compiled pattern, and `slugify`, and corrected the module docstring that said the function was unimplemented.
- Test result, stated plainly with the actual output: `python3 -m unittest -q test_slug` → `OK`, 3 tests. If anything had failed I would paste the failure rather than summarize it.
- Constraints honoured: stdlib `re` only, one function, no class, no CLI.
- One flag, not a hedge: non-ASCII letters are removed rather than transliterated (`"Café"` → `"caf"`), and underscores become hyphens — both follow from the card's rule, neither is pinned by a test, and either could be worth a follow-up card.
- No memory file written for this: it is routine work fully recorded by the card and the resulting code, with nothing non-obvious about the user or the project to retain.