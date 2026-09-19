FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

# Action plan — cycle card C1 (`slugify`)

## Phase 0 — Confirm the card's shape against the workspace (done above, no work product)

**What I did / would do:** Read all three files before touching anything.

Findings that drive the rest of the plan:
- `slug.py` is a single docstring line, no imports, no functions — a true stub, so there is no existing behaviour to preserve and no caller to keep compatible. The card's "Brownfield exposure: none" checks out.
- `test_slug.py` pins exactly three cases and imports only `slugify` from `slug`. Nothing else in the workspace imports `slug` (only these three files exist), so the module-level docstring is the only thing I need to be careful about rewriting.
- The three assertions are fully satisfied by the approach the card fixes; I traced each one by hand before writing code (see Phase 2). No ambiguity in the spec that would change what I build, so nothing here needs a decision from the user.

**What I would not do, and why:** the card fixes the approach and forbids extra surface. So no `unicodedata` transliteration (`café` → `caf`, not `cafe`), no `max_length`/`separator` parameters, no `__main__` block or CLI, no second helper, no class. If I think any of those are worth having, that is a proposal for a later card, not something I slip into this one.

## Phase 1 — Re-confirm the red test and its reason

**What I would run:** `python3 -m unittest -q test_slug` from the workspace root.

**What I expect:** a failure, and specifically `ImportError: cannot import name 'slugify' from 'slug'` surfacing as 1 error at collection of the module (unittest reports this as a single `_FailedTest` error for `test_slug`, not 3 separate failures). That is the right reason to be red: the function is absent, not wrong.

**Stop condition:** if it is already green, or red for some other reason (syntax error, missing file, a stray `slugify` defined elsewhere on the path), I stop and report that the card's premise does not match the workspace rather than writing an implementation over an unexplained state. Onward branch if the user then says "proceed anyway": I would implement as planned and note in the final report that the red-for-the-right-reason step could not be reproduced.

## Phase 2 — Implement `slugify` in `slug.py`

**File written:** `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-n5fvdqmz/ws/slug.py` (edit in place; keep the module docstring but drop the now-false "not implemented yet" clause).

**Content I intend:**

```python
"""Slug helpers."""

import re

_NON_ALNUM = re.compile(r"[^a-z0-9]+")


def slugify(text: str) -> str:
    """Lowercase `text` and reduce every run of non-alphanumeric characters to one hyphen."""
    return _NON_ALNUM.sub("-", text.lower()).strip("-")
```

**Why this exact form:** `re` only, one function, no extra names beyond a module-private compiled pattern. Lowercasing first means the character class only needs `a-z0-9`. The hyphen is itself outside the class, so a literal `--` in the input is absorbed into the surrounding run instead of being preserved — that is what `test_collapses_runs_and_trims` requires. `.strip("-")` handles both ends, including the all-punctuation case where the whole string collapses to a single `-`.

**Hand-trace of each pinned case before running anything:**
- `"Hello World!"` → `"hello world!"` → sub → `"hello-world-"` → strip → `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → lower → sub replaces the leading `"  --"`, `" & "`, `": "`, and trailing `"--  "` runs → `"-rust-cargo-2026-"` → strip → `"rust-cargo-2026"` ✓
- `"!!!"` → sub → `"-"` → strip → `""` ✓

**Judgment call I would make myself, not escalate:** the compiled-module-level pattern versus an inline `re.sub`. Both are one function and stdlib `re`; I use the compiled constant because it is the idiom for a pattern applied on every call, and `_NON_ALNUM` is private so it does not widen the module's public surface. If a reviewer objects to any module-level name at all, the inline `re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")` is a one-line swap with identical behaviour.

## Phase 3 — Verify

**What I would run:** `python3 -m unittest -q test_slug` in the workspace root.

**What I expect:** `OK`, 3 tests run, no failures or errors — exactly the card's assert line. I would paste the actual output into the report rather than paraphrasing it.

**If it is not green:** I fix the implementation, not the test. `test_slug.py` is the spec for this card and I would not edit it; if I ever concluded the test itself were wrong, I would stop and say so rather than change it quietly.

**Extra checks I would run beyond the suite, without adding files:** a quick `python3 -c` sanity pass on inputs the suite does not cover, to make sure I understand the behaviour I am shipping rather than to expand scope — `""` → `""`, `"already-a-slug"` → `"already-a-slug"`, `"a  b"` → `"a-b"`, `"2026"` → `"2026"`. I would not add these as test cases; the card fixes the suite at 3 tests, and growing it is a separate decision.

## Phase 4 — Mark the card and report

**File written:** `CARD.md` — flip `- [ ] **C1 · slugify**` to `- [x]`, changing nothing else on the line. This is the one bookkeeping edit the card's checkbox implies; if the convention in this project is that a human ticks the box, that is a trivially revertible one-character change and I would say in the report that I made it.

**No delegation anywhere in this plan.** The task is a three-line function against a three-assertion suite that I have already read in full; handing it to another worker would cost more than doing it and would put a second, unread implementation between me and the result. Nothing here needs a cloud review either.

**What I would report:**
1. The red state and its exact reason (`ImportError` on `slugify`), quoted from Phase 1.
2. The diff to `slug.py` — one import, one compiled pattern, one function, plus the docstring correction — and the `CARD.md` tick.
3. The verbatim green output: `OK`, 3 tests.
4. Two behaviours a reader might not expect from the name, stated plainly rather than fixed: non-ASCII letters are dropped, not transliterated (`"café"` → `"caf"`, `"日本語"` → `""`), and because `.lower()` runs before filtering, a dotted capital `İ` decomposes and leaves a bare `i` behind. Both follow directly from the approach the card fixed; I flag them as candidates for a future card and leave the decision to the user.