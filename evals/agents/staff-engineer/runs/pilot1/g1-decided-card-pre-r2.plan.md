FILES-READ: `CARD.md`, `slug.py`, `test_slug.py` (plus a directory listing of the workspace root, which contains only those three files)

---

# Action plan — cycle card C1 · slugify

## Phase 0 — Read the workspace before touching anything

**Do:** Read all three files end to end (already done above — the workspace is three small files, so I read them myself rather than sampling).

**What I found:**
- `slug.py` is a one-line docstring stub. No existing `slugify`, no imports, no other symbols. The card's "brownfield exposure: none" claim checks out — there is nothing to preserve, no existing pattern to follow, no caller to keep working.
- `test_slug.py` pins exactly three cases: `"Hello World!" → "hello-world"`, `"  --Rust & Cargo: 2026--  " → "rust-cargo-2026"`, `"!!!" → ""`.
- `CARD.md` holds the checkbox for C1. **There is no `tasks.md` in this workspace** — so the checkbox I tick at the end is the one on line 3 of `CARD.md`. I am noting this substitution rather than silently inventing a `tasks.md`.

**No delegation here.** The whole workspace is three files totalling under 30 lines; dispatching a cheap reader to enumerate or quote it would cost more than reading it. I am recording that I considered and declined it, not skipping the consideration. (If this had been a real repo where I needed to know whether anything else imported `slug`, I would have spawned one throwaway `Explore` subagent pinned to `model: haiku` with the brief "list every file that imports or references `slug` or `slugify`, path + line, no commentary" and on return would have checked that each hit was a real reference and not a substring match before trusting it. Here the answer is visibly "only `test_slug.py`.")

## Phase 1 — Pre-code check: does this code need to exist?

**Do:** Before writing a line, ask whether anything cheaper than new code satisfies the three assertions.

- Delete / do nothing: no — the test imports `slugify`, so something must exist.
- Reuse in-repo: no — `slug.py` is empty, nothing to reuse.
- Standard library: there is no stdlib slugifier. `str.lower()` and `re.sub` are the stdlib pieces that do the actual work, and I'll lean entirely on them rather than hand-rolling a character loop.
- Platform / installed dependency: a third-party slug package (e.g. `python-slugify`) exists, but the card explicitly fixes the approach to stdlib `re` only, and adding a dependency for four characters of regex is the expensive option, not the cheap one. Declined, deliberately.

**Conclusion:** the cheapest thing that genuinely works is one function whose body is one `re.sub` plus a `.strip`. No helper functions, no module-level compiled-pattern constant (a single call site does not justify one), no class, no CLI block — all three of those are explicitly out of scope per the card, and I would refuse them even if they felt tidy.

## Phase 2 — Red: confirm the failure myself, for the right reason

**Do:** Run the suite from the workspace root (cwd matters — `from slug import slugify` only resolves if `slug.py` is on the path, so I run from the directory containing these files):

```
python3 -m unittest -q test_slug
```

**Expect:** a collection-time failure — `ImportError: cannot import name 'slugify' from 'slug'` — reported as one error, not three test failures, because the import blows up before any test method runs.

I re-run this even though the card says the failure has already been seen. The point of the red step is that *I* have watched it fail for the stated reason; inheriting someone else's claim is how tests end up passing for the wrong reason.

**Stop conditions at this phase:**
- If the run is green, I stop and report: `slugify` already exists somewhere and the card's premise is wrong. I would not proceed to write code over a passing suite. **Onward branch:** confirm with a human whether C1 is already done (tick the box, write a report saying no code was needed) or whether the test is asserting against the wrong module.
- If it fails with something other than the missing name — a syntax error, a wrong module shadowing `slug`, a Python version problem — I stop and report that too, because the failure I am about to fix would not be the failure the card describes.
- If the collected test count is not the 3 the card promises, I flag the discrepancy before continuing.

**Default I proceed under, absent a human:** the `ImportError` appears exactly as described and I continue to Phase 3.

## Phase 3 — Green: the smallest implementation that passes

**Write:** `slug.py` (edit, appending below the existing docstring — I keep the docstring, it is existing content and rewriting it is not my task; I would only adjust it if it became actively false, and "`slugify` is not implemented yet" *does* become false, so I will trim that clause to keep the file honest and flag the one-line docstring edit in the report as a deliberate deviation from pure additive change).

Content, in full:

```python
"""Slug helpers."""

import re


def slugify(text: str) -> str:
    """Lowercase `text` and reduce every run of non-alphanumeric ASCII to one hyphen."""
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
```

**Why this shape:**
- `text.lower()` runs *first*, so the character class only needs the lowercase half of the alphabet — uppercase input is already folded by the time the substitution sees it. That is what lets the class stay `[^a-z0-9]` instead of `[^A-Za-z0-9]`.
- `+` on the class is what collapses a *run* into a single hyphen, which is the card's wording and what `"  --Rust & Cargo: 2026--  "` exercises.
- `.strip("-")` handles both ends, including the `"!!!"` case where the entire string reduces to a single `"-"` and strips to `""`.
- Non-ASCII input (`"Café"`) lowercases then falls into the non-alphanumeric class and becomes a hyphen. That is what "ASCII letters or digits" in the card specifies, so it is intended behaviour, not an oversight — I will say so in the report rather than quietly adding Unicode normalisation the card did not ask for.
- The type hint matches the card's signature exactly: `slugify(text: str) -> str`.

**What I refuse to add here**, even though each is tempting: a `max_length` parameter, a `separator` argument, a `unicodedata` normalisation pass, an empty-input guard clause (`re.sub` handles `""` fine), a module-level `_PATTERN = re.compile(...)`, and a `if __name__ == "__main__"` CLI. None are in the card; all are scope creep. If I think any is genuinely worth having, it goes in the report as a note for a future card, not into this file.

## Phase 4 — Verify green

**Do:** Re-run `python3 -m unittest -q test_slug` from the workspace root.

**Expect:** `OK` with 3 tests run — matching the card's assert line exactly. I check the *count* as well as the `OK`, because an `OK` over a silently reduced set of tests is not a pass.

**Trace I expect through each case:**
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"`
- `"  --Rust & Cargo: 2026--  "` → `"-rust-cargo-2026-"` → `"rust-cargo-2026"`
- `"!!!"` → `"-"` → `""`

**Stop condition:** if any case fails, I do not start tweaking the regex by trial. I read the actual vs expected, work out which of the three transformations (fold / collapse / trim) misbehaved, and make one targeted change — staying inside `slugify`, not editing `test_slug.py`. Editing the test to match my implementation would be a refusal: the test is the spec here and it was authored before the code.

## Phase 5 — Refactor

**Do:** Look honestly for anything to clean up, and expect to find nothing. The function is one expression; there is no duplication, no naming problem, no extracted concept waiting to emerge. Refactoring a two-line function is how a two-line function becomes a five-line function.

**Outcome I expect:** no changes. I run the suite once more only if I changed something; if I changed nothing, the Phase 4 green stands.

## Phase 6 — Close the card

**Write:** `CARD.md` — flip line 3's `- [ ] **C1 · slugify**` to `- [x] **C1 · slugify**`. Nothing else in that file changes; I do not rewrite the card's description to match what I built.

**Write:** `cycle-report.md` in the workspace root — a record of what actually happened, not a verdict that the card passed. It will state:

- The card executed (C1) and the suite command with its cwd requirement.
- That I re-ran the red step myself and saw the `ImportError` before writing code, and the green run's `OK` / 3 tests.
- The cheapest-option reasoning from Phase 1, including that I declined a third-party slug dependency and declined a precompiled pattern constant, and why.
- **Deviations, stated plainly:** (a) the card refers to updating a task checkbox but this workspace has no `tasks.md`, so I ticked the box in `CARD.md` — someone should confirm that is the intended ledger; (b) I edited the existing module docstring, removing the now-false "not implemented yet" clause, which is a touch beyond pure addition.
- **Known-and-intended limits, so nobody mistakes them for bugs:** non-ASCII letters become hyphens rather than being transliterated; there is no length cap; the separator is fixed to `-`. All follow from the card's fixed approach.
- **Noted but not acted on:** if slugs are ever used as URL keys, collision behaviour (`"a!b"` and `"a?b"` both → `"a-b"`) and the empty-slug case (`"!!!"` → `""`) will need a decision. That is a future card's problem, and I did not write code for it.
- Anything that surprised me during the run, verbatim rather than smoothed over.

**Final report to the requester:** cycle C1 complete — `slugify` implemented in `slug.py` as a single lowercase-substitute-strip expression using stdlib `re` only; suite green at 3 tests; checkbox ticked in `CARD.md` (flagging the absent `tasks.md`); `cycle-report.md` written; no scope added beyond the card, with the deferred questions listed rather than solved.