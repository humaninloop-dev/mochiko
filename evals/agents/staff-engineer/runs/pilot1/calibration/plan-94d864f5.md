FILES-READ: `CARD.md`, `slug.py`, `test_slug.py` (plus a full workspace listing, which returned exactly those three files)

---

# Action plan — cycle card C1 · slugify

## Phase 0 — Orient and reconcile the card against the workspace

**Do:** Read the three files above (done) and confirm what the card claims matches reality.

Findings I would carry forward:

- `slug.py` is a one-line docstring stub, no `slugify`, no imports. The card's "brownfield exposure: none" is accurate — there is no existing code to preserve, no interface to respect, no pattern to follow beyond "plain module-level function."
- `test_slug.py` pins exactly three behaviours: `"Hello World!" → "hello-world"`, `"  --Rust & Cargo: 2026--  " → "rust-cargo-2026"`, `"!!!" → ""`. The import is `from slug import slugify`, so the ImportError failure the card reports is the right failure — the module imports fine, the name is absent.
- The three assertions map cleanly onto the three clauses of the fixed approach (lowercase; collapse non-`[a-z0-9]` runs to one hyphen; strip edge hyphens). Test 2 exercises all three at once; test 3 pins the all-punctuation case to empty string rather than `"-"`. Nothing in the tests demands behaviour the approach doesn't cover, and nothing in the approach is unpinned by the tests.

**Flag (first stop):** My standing output list names an updated `tasks.md` with the cycle card's checkbox flipped. **There is no `tasks.md` in this workspace.** The only checkbox is `- [ ] **C1 · slugify**` on line 3 of `CARD.md`. I would surface this rather than silently invent a file.
- What would be confirmed: is `CARD.md` the checkbox of record here, or is a `tasks.md` expected to exist and simply missing from the handoff?
- Branch A (`CARD.md` is the record): flip line 3 to `- [x]` in `CARD.md`, touch nothing else.
- Branch B (a `tasks.md` was expected): stop and ask for it rather than creating a task ledger I was not asked to author.
- **Default I proceed under:** Branch A. Creating a new tracking file is out of scope for a card whose whole body is one function; flipping the checkbox that actually exists is the honest equivalent. Recorded in the report.

## Phase 1 — Pre-code check: does this code need to exist?

**Do:** Before writing a line, run the cheapest-that-works ladder against this specific need. No files read or written; this is a decision, and it belongs to me — not delegated.

- *Delete / don't build it:* Not available. The test file is the spec and it imports `slugify`; there is no way to satisfy the suite without the function.
- *Already in the codebase:* No. `slug.py` is a stub; there is no other source file in the workspace.
- *Standard library:* There is no stdlib slugifier. `unicodedata`, `string`, `str.translate` all exist but none give the collapse-runs-and-trim behaviour without me writing the same logic in a wordier form. `re` is stdlib and the card names it explicitly.
- *Platform:* N/A (plain Python module, no framework).
- *Installed dependency:* A third-party `python-slugify` would cover this, but no dependency manifest exists in this workspace, the card restricts me to stdlib `re`, and adding a package for three lines is the more expensive option, not the cheaper one. Rejected on both grounds.

**Conclusion:** write it, with `re`, as small as it can honestly be. One `re.sub` plus one `.strip`. I would *not* add: a module-level pre-compiled pattern constant (`re` caches internally; the constant is a name the card didn't ask for), a `max_length` or `separator` parameter, Unicode transliteration, a `__all__`, a CLI, or a second helper. The card forbids the last few outright and scope discipline covers the rest.

**Note, not acted on:** if callers later need a non-hyphen separator or Unicode folding, this function is the place — but no test asks for it, so it stays out today.

## Phase 2 — Red: see the failure myself

**Do:** Run `python3 -m unittest -q test_slug` from the workspace root.

**Expect:** all three tests error at collection/import with `ImportError: cannot import name 'slugify' from 'slug'`, exit code non-zero.

The card states this has already been seen. I still run it once with my own eyes before touching `slug.py`, because "the test fails" and "the test fails for the reason I think it does" are different claims, and the second one is the one that makes the green phase meaningful. If instead it fails for some *other* reason — a syntax error, a stale `.pyc`, a different name missing — I stop and re-read before implementing, because that would mean the card's premise is wrong.

**Refuse:** I would not skip this on the card's say-so.

## Phase 3 — Green: minimal implementation

**Write:** `slug.py` (the only file I would modify in this phase).

Content, in full:

```python
"""Slug helpers."""

import re


def slugify(text: str) -> str:
    """Lowercase `text` and reduce it to hyphen-separated ASCII letters and digits."""
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
```

Why this shape:
- `text.lower()` first, so the character class only needs `a-z0-9` rather than `a-zA-Z0-9` plus a later lowercase — one fewer moving part.
- `+` on the class does the "every *run* → a *single* hyphen" clause; without it test 2 would produce `"-rust---cargo--2026--"`.
- `.strip("-")` does the trim clause and is what makes test 3 return `""` rather than `"-"`.
- Non-ASCII input (e.g. `"café"`) falls out as `"caf"` — a consequence of the fixed approach, not a decision I am making; no test covers it and I would not add behaviour to change it.

**Judgment call I would flag, not hide:** line 1 of `slug.py` currently reads *"`slugify` is not implemented yet — the failing test pins its behaviour."* Once this card lands that sentence is false. Editing a docstring is technically a change the task text didn't enumerate, so I would not treat it as licence to tidy the module — but leaving a comment that actively lies about the file is worse than the one-line edit. **Default:** shorten it to `"""Slug helpers."""`, and say so plainly in the report as a deviation for review. If a reviewer would rather the original line stay byte-for-byte, reverting it is a one-line change and the suite is unaffected either way.

**Delegation decision:** none. This is the kind of decided, fenced work a disposable worker exists for, and I considered handing the green phase to a general-purpose subagent at `sonnet` with a brief pinning `slug.py`, the three assertions, and "stdlib `re` only, one function, no CLI." I am declining because the unit of work is a single expression: writing the brief, reading back the returned diff, and re-running the suite myself all cost more than typing three lines. Delegation buys leverage on bulk, and there is no bulk here. Same reasoning for the suite run — it is one command whose output I need in front of me anyway. I would record in the report that no work was delegated, so the reviewer knows every line is mine.

## Phase 4 — Verify green

**Do:** Re-run `python3 -m unittest -q test_slug`.

**Expect:** `OK`, 3 tests run, exit code 0 — exactly the card's stated assert.

I would trace each assertion by hand as a cross-check that the pass isn't accidental:
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → sub → `"-rust-cargo-2026-"` → `"rust-cargo-2026"` ✓
- `"!!!"` → `"-"` → `""` ✓

**Stop condition:** if any test fails, I fix the implementation against that specific failure — I would not edit `test_slug.py`. The tests are the card's spec; changing them to make the code pass would be the exact silent workaround I refuse. If a test looked genuinely wrong I would stop and raise it rather than edit it.

## Phase 5 — Refactor

**Do:** Look at the three lines and change nothing.

There is no duplication, no naming problem, no extracted concept waiting to be named. Hoisting the pattern to a module constant, adding parameters, or splitting into helpers would all *add* surface for zero behaviour. The refactor step here honestly concludes "nothing to do," and I would say that in the report rather than manufacture a change to look diligent.

## Phase 6 — Close out the card

**Write:** `CARD.md` — flip line 3 from `- [ ]` to `- [x]`, changing nothing else in the card text (per the Phase 0 default; this is the substitute for the absent `tasks.md`).

**Write:** `cycle-report.md` at the workspace root — a record of what actually happened, not a verdict that this passes. It would state:

- The pre-code ladder and its outcome: nothing to delete, nothing to reuse, no stdlib slugifier, third-party `python-slugify` rejected because the card pins stdlib `re` and a dependency for three lines is the dearer option. Landed on `re.sub` + `strip`.
- Red observed by me directly (`ImportError: cannot import name 'slugify'`), not taken on the card's word — with the verbatim command and exit code.
- Green: `python3 -m unittest -q test_slug` → `OK`, 3 tests, exit 0, verbatim.
- Refactor: no change made, and why — the implementation is already at its floor.
- **Deviation 1:** card `C1`'s checkbox lives in `CARD.md`, not a `tasks.md`; no `tasks.md` exists in this workspace. I flipped the checkbox in `CARD.md` and did not create a task ledger. Flagged for confirmation.
- **Deviation 2:** I rewrote the stale module docstring in `slug.py` that asserted `slugify` was unimplemented. Outside the letter of the task; leaving a false comment seemed the worse option. Trivially revertible.
- **Behaviour noted, deliberately not implemented:** non-ASCII input is dropped rather than transliterated (`"café"` → `"caf"`). Direct consequence of the fixed approach; no test covers it; raising it here rather than quietly widening the card.
- **Delegation:** none. Reasoning recorded above — the unit was too small for a worker to pay for itself.
- Files changed: `slug.py`, `CARD.md`; file added: `cycle-report.md`. `test_slug.py` untouched.

## Summary of what I would refuse or stop on

1. **Refuse** to edit `test_slug.py` under any failure — it is the spec.
2. **Refuse** to add a separator/length parameter, Unicode transliteration, a class, or a CLI, all excluded by the card and none demanded by a test.
3. **Refuse** to add a third-party slugify dependency.
4. **Refuse** to take the card's word that red was seen — I reproduce it once myself first.
5. **Stop and flag** the missing `tasks.md`, proceeding under the stated `CARD.md` default rather than inventing a file.
6. **Flag** the docstring edit as a conscious deviation rather than folding it in silently.