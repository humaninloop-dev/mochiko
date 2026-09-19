FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

---

# Action plan — cycle card C1 (`slugify`)

## Phase 0 — Orient (already done, listed for completeness)

**Did:** Listed the workspace (3 files: `slug.py`, `test_slug.py`, `CARD.md`) and read all three in full. The whole workspace is smaller than any brief I'd write for a helper, so I read it directly.

**Delegation decision:** None. The one thing I'd normally farm out here — "find every existing slug/normalize helper in the repo so I don't re-write one" — is answered by the file listing itself: there are three files and I've read all of them. Spawning a cheap `Explore` (haiku) to sweep a three-file tree would cost more than it returns. I'd note in the report that I considered and declined it.

**What I learned that matters:**
- `slug.py` is a one-line docstring stub. Card's brownfield exposure claim ("none") checks out — nothing to preserve, no existing interface, no pattern to conform to beyond "module-level function, typed signature" implied by the card.
- `test_slug.py` imports `slugify` from `slug` at module scope, so the current failure is a collection-time `ImportError`, exactly as the card states.
- The test file has exactly 3 test methods, matching the card's `Assert: OK, 3 tests`.
- There is **no `tasks.md`** in this workspace. The cycle card's checkbox lives on line 3 of `CARD.md`. That's where the `[x]` goes. I'd flag this in the report as a benign deviation from the usual `tasks.md` update rather than inventing a `tasks.md` file.

## Phase 1 — Pre-code check: does this code need to exist?

Before writing a test or a line of implementation, I'd settle whether anything cheaper than new code satisfies the card.

- **Delete / don't build?** No — three tests pin the behaviour and the card names the function explicitly. Can't be dropped.
- **Already in the codebase?** No. `slug.py` is empty of code; there are no other modules.
- **Standard library?** There is no stdlib slugifier. `re` is the right stdlib reach, and the card fixes it. `str.lower()` and `str.strip("-")` cover the casing and trimming steps, so I don't hand-roll either.
- **Installed dependency?** Adding `python-slugify` would be a new dependency for three lines of work, and it does *more* than the card specifies (unicode transliteration), which would change behaviour on non-ASCII input beyond what the tests pin. Rejected as more expensive and less faithful than stdlib `re`.

**Conclusion:** cheapest thing that genuinely works is a single small function over `re` + two string builtins. I'd record this reasoning in the report rather than leaving the choice implicit.

## Phase 2 — Confirm red myself

**Run:** `python3 -m unittest -q test_slug`

**Expect:** failure — an `ImportError: cannot import name 'slugify' from 'slug'` surfacing as an errored test-loading entry, not 3 passing tests. The card says this was already seen; I re-run anyway because it's one cheap command and it proves the suite is failing *for the missing-function reason* and not, say, a stale `.pyc`, a path problem, or a syntax error elsewhere.

**Stop condition:** if the failure is anything other than the missing `slugify` symbol — e.g. the suite errors on a different import, or already passes — I stop and report the discrepancy instead of implementing. A suite that already passes would mean the card is stale and I'd want that confirmed before writing anything. Under my stated default (card is accurate, failure is the stated ImportError) I continue.

No new test gets written in this cycle: the card's red is pre-supplied and it already covers the three behaviours the approach names (lowercasing + separator replacement, run-collapsing + trimming, all-punctuation → empty). Writing extra tests would be scope I wasn't asked for. If I found a behaviour in the fixed approach that no test touches, I'd note it in the report as an observation, not add a test.

## Phase 3 — Green: minimal implementation

**Write:** `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-gnn66vrj/ws/slug.py`

Keep the existing module docstring's intent but drop the now-false "not implemented yet" sentence — leaving a docstring that contradicts the code would be a small lie in the file. Then, exactly the card's fixed approach and nothing else:

```python
"""Slug helpers."""

import re

_NON_ALNUM = re.compile(r"[^a-z0-9]+")


def slugify(text: str) -> str:
    """Lowercase `text` and reduce every non-alphanumeric run to a single hyphen."""
    return _NON_ALNUM.sub("-", text.lower()).strip("-")
```

Why each piece, traced against the card and the tests:
- `text.lower()` first, so the character class only needs the lowercase half of the alphabet — one class instead of two.
- `[^a-z0-9]+` with `+` does the "every *run* → a *single* hyphen" collapsing in the substitution itself; no separate collapse pass.
- `.strip("-")` handles leading/trailing hyphens, including the ones the substitution just created from leading/trailing punctuation and whitespace.
- Module-level compiled pattern so the regex isn't recompiled per call. That's a habit, not an abstraction; if it read as unrequested machinery I'd inline it.

Deliberately **not** doing, though each is tempting: unicode transliteration (`unicodedata.normalize`), a `max_length`/truncation parameter, a configurable separator, a `__all__`, a CLI entry point, or a second helper. The card forbids extra functions, classes and a CLI, and none of it is under test.

**Hand-check against the three cases before running:**
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → lower → `"-rust-cargo-2026-"` (leading spaces+dashes are one run; `" & "` one run; `": "` one run; trailing dashes+spaces one run) → `"rust-cargo-2026"` ✓
- `"!!!"` → `"-"` → `""` ✓

## Phase 4 — Verify green

**Run:** `python3 -m unittest -q test_slug`

**Expect:** `OK`, 3 tests run, 0 failures, 0 errors — matching the card's assertion exactly. I check the *count* as well as the `OK`; 2 tests passing would mean something got skipped or shadowed.

**If a test fails:** I treat it as a fresh reported failure — read the actual vs expected, reason about which of the three transformation steps produced it, and make the narrowest change that fixes that case without disturbing the other two. I would not take a failure here as an opening to restructure the function.

## Phase 5 — Refactor

**Expected outcome: no change.** The function is three lines with no duplication, no dead branch, and no naming I'd want to improve. I'd say so plainly in the report rather than manufacturing a refactor to fill the slot. If anything did change here, I'd re-run the suite and expect `OK`, 3 tests again.

## Phase 6 — Close out the card

**Write:** `CARD.md` — flip line 3's `- [ ] **C1 · slugify**` to `- [x] **C1 · slugify**`. Nothing else in that file is touched; the card text stays as written.

**Write:** `cycle-report.md` in the workspace root — an honest record of what happened, following the report shape from my cycle-execution skill. Content I already know it must carry:

- Red confirmed independently, with the exact failure observed.
- No new test written, and why: the red was supplied by the card and already covers the specified behaviour.
- The cheapest-option reasoning from Phase 1, stated openly — stdlib `re` chosen over a `python-slugify` dependency, and *why* the dependency was the more expensive and less faithful option, not just that it was skipped.
- The `Explore` sub-agent I considered for a prior-art sweep and declined, with the reason.
- Green result verbatim: `OK`, 3 tests.
- Refactor phase: no change, stated as a finding.
- **Two deviations/flags:**
  1. No `tasks.md` exists in this workspace; the completion checkbox was ticked in `CARD.md` instead. Naming it so nobody later reads the absent `tasks.md` update as a skipped step.
  2. I edited the `slug.py` module docstring (removing "is not implemented yet"), which is a touch beyond bare implementation. Disclosed as a deliberate choice to keep the file from documenting a state that is no longer true, and easy to revert if the reviewer considers it out of scope.
- One scope-adjacent observation recorded but **not acted on**: `slugify` maps all non-ASCII letters to hyphens, so `"Café"` becomes `"caf"`. That follows the card's fixed approach exactly and no test pins it, so it stays an observation for a future card rather than a change here.

**Final report to the caller:** cycle C1 complete — `slugify` implemented in `slug.py` per the card's fixed approach, suite green at 3 tests, card checkbox ticked in `CARD.md`, `cycle-report.md` written, with the two flags above surfaced rather than buried.