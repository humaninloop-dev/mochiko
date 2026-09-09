FILES-READ: `CARD.md`, `slug.py`, `test_slug.py` (plus a full listing of the workspace directory, which contains only those three files)

---

# Action plan — cycle card C1 · slugify

## Phase 0 — Read the card and the ground truth (done above)

**What I read:** `CARD.md` (the single card, C1), `slug.py` (a one-line module docstring, no code), `test_slug.py` (three assertions), and a directory listing of the workspace.

**What I established:**

- The card fixes the approach: lowercase, collapse every run of non-ASCII-alphanumeric characters to a single hyphen, trim leading/trailing hyphens, `re` from the standard library only, one function, no class, no CLI.
- Brownfield exposure is declared as none, and the listing confirms it — `slug.py` is a stub with a docstring and nothing else, and nothing else in the workspace imports from it. So there is no existing pattern to conform to and no interface to preserve. I would not spawn a cheap read agent to sweep for callers: the workspace is three files and I have already read all of them, so a delegated read would tell me nothing I do not have.
- The three test cases pin the behaviour completely: `"Hello World!"` → `"hello-world"`, `"  --Rust & Cargo: 2026--  "` → `"rust-cargo-2026"`, `"!!!"` → `""`. The third case is the one that matters most — it forces trimming to survive an input that reduces to nothing but a separator.

**Flag I would carry forward, not act on:** there is no `tasks.md` in this workspace. The completion checkbox lives on line 3 of `CARD.md` itself. See Phase 5.

## Phase 1 — Confirm the red, and confirm *why* it is red

**What I would run:** `python3 -m unittest -q test_slug` from the workspace root.

**What I expect to see:** all three tests erroring at collection with `ImportError: cannot import name 'slugify' from 'slug'`, exit code non-zero. That matches what the card reports.

I would run this myself rather than take the card's word for it, and rather than delegate it — the command is one line and I need the verbatim output for the report, so handing it to a worker would cost more than it saves.

**Honest note I would record now, because it shapes the report:** this red is a single import failure, not three separately-observed behavioural reds. The function's absence makes all three cases fail at once, so I get one red covering three behaviours rather than three red/green passes. That is what the card prescribes and it is a legitimate red — the test fails because the thing under test does not exist, not because of a typo or a broken harness — but it does mean the collapse-runs case and the empty case go from red to green in the same step as the basic case. I would say so plainly in the report rather than let it read as three clean cycles.

**Where I would stop:** if the run shows anything other than that import failure — for instance if the tests already pass, or if it fails on a syntax error in the test file, or on a missing `python3` — I would stop and report rather than proceed. A green suite before I have written anything means the card's premise is wrong and someone needs to rule on it. My default if I could not get a ruling: report the discrepancy and write no code, because implementing against a suite that is already passing tells me nothing about whether my code is correct.

## Phase 2 — The cheapest thing that works, before writing anything

I would work down the options in order and record the outcome of each, because the choice belongs in the report:

1. **Do nothing / delete.** Not available — the test imports `slugify` by name and asserts on its return value. The function has to exist.
2. **Something already in the codebase.** No. `slug.py` is an empty stub; there is no other module.
3. **Standard library.** There is no `slugify` in the standard library, so I cannot get this for free. But the *pieces* are all stdlib and I should not hand-roll any of them: `str.lower()` for case, `re.sub` for run collapsing, and — the one worth calling out — `str.strip("-")` for trimming. It is tempting to express the trim as extra regex anchors or a manual index walk; `strip` with an argument already does exactly this and is clearer, so the trim costs one method call and no new logic.
4. **An installed dependency.** None are installed, and the workspace has no dependency manifest at all. `python-slugify` exists on PyPI and would do this, but the card says `re` only, and pulling a package in for four lines of behaviour would be the wrong trade even without that instruction. **I would refuse to add it** and note the refusal.
5. **New code.** What remains is a single function of one expression.

I would also deliberately *not* add things this card does not ask for, and note that I considered and rejected them: no Unicode transliteration via `unicodedata` (the spec says non-ASCII-alphanumeric runs become hyphens, and no test exercises accented input — adding folding would be inventing behaviour nobody asked for and would change what `"café"` produces); no `max_length` parameter; no `separator` parameter; no `__all__`; no module-level precompiled pattern constant, since `re` caches compiled patterns internally and a module constant would add a name for no gain at this size.

## Phase 3 — Green

**What I would write —** the only file I would touch is `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-brjwisms/ws/slug.py`:

```python
"""Slug helpers."""

import re


def slugify(text: str) -> str:
    """Lowercase `text`, collapse each run of non-alphanumeric characters to a
    single hyphen, and trim hyphens from both ends."""
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
```

Notes on the specific choices, since they are the whole of the work:

- Lowercasing *first* lets the character class be `[^a-z0-9]` rather than `[^a-zA-Z0-9]`. Same behaviour, one fewer range to read.
- The `+` on the class is what collapses runs, so `"--Rust"` yields one hyphen and not four.
- `.strip("-")` runs last and handles the case where the input reduces entirely to separators; `"!!!"` becomes `"-"` after substitution and `""` after the strip.
- The signature carries the annotations the card specifies.
- The module docstring currently says `slugify` "is not implemented yet — the failing test pins its behaviour." Once this lands that sentence is false, so I would shorten it to `"""Slug helpers."""`. I would call this out as the one edit outside the strict "add a function" boundary, and justify it: leaving a docstring that actively lies about the module's state is a defect, not a scope saving.

**Trace I would check by eye before running, against each test:**
- `"Hello World!"` → lower → `"hello world!"` → sub → `"hello-world-"` → strip → `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → lower → sub collapses the leading whitespace-and-dashes run, `" & "`, `": "`, and the trailing `"--  "` each to one hyphen → `"-rust-cargo-2026-"` → strip → `"rust-cargo-2026"` ✓
- `"!!!"` → sub → `"-"` → strip → `""` ✓

**On delegation:** I would not hand this green phase to a worker. The rung exists for execution I have already thought through that is nonetheless bulky — boilerplate, a mechanical refactor across many files, a long test-writing pass. This is one expression in one file; briefing a worker, then reading its diff and re-running the suite myself, is strictly more work than typing it. If this card had instead asked for, say, twenty parametrised cases against a settled implementation, that is where I would spawn a general-purpose worker on the mid tier with the file fenced to `test_slug.py`, the exact cases listed, an instruction not to touch `slug.py`, and a required return of the diff plus verbatim command output with exit codes — and I would still re-run the suite myself before counting it. None of that applies here, and I would say so in the report rather than leave the absence of delegation unexplained.

## Phase 4 — Verify, then refactor

**What I would run:** `python3 -m unittest -q test_slug` again.

**What I expect:** `OK` with 3 tests run, exit code 0 — exactly the assertion the card names.

**If it does not go green:** I would not start adjusting the test. The test is the specification here and the card says it was seen to fail for the right reason, so a red at this point means my implementation is wrong. I would read the actual failure output, fix the implementation, and re-run — and record the miss in the report rather than quietly correcting it.

**Refactor:** I would look and expect to find nothing to do. A single return expression with a docstring has no duplication to pull out and no name to improve; any "refactor" I invented here would be churn. I would record the refactor step as consciously a no-op with that reasoning, rather than omit the step or manufacture a change to fill it.

## Phase 5 — Mark the card complete

**The stop:** the card's own instructions and my normal practice assume a `tasks.md` holding the checkbox. This workspace has none — the checkbox is line 3 of `CARD.md`. What I would want confirmed is which file the tick belongs in.

- If the ruling is that `CARD.md` is the task list for this workspace: flip line 3 from `- [ ] **C1 · slugify**` to `- [x] **C1 · slugify**`, changing nothing else in the file.
- If the ruling is that a `tasks.md` exists elsewhere and should have been provided: leave `CARD.md` untouched and report the cycle complete-but-unmarked, so the tick lands in the right place once the file appears.

**My default, proceeding without a ruling:** the first branch. `CARD.md` is the only file in the workspace carrying a checkbox for C1, and leaving completed work unmarked is the worse failure. I would flip it and disclose in the report that I marked `CARD.md` because no `tasks.md` exists, so the substitution is visible rather than assumed.

## Phase 6 — Write the cycle report

**What I would write:** `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-brjwisms/ws/cycle-report.md`, following the report format from my TDD-cycle procedure, containing a truthful record rather than a verdict:

- The red run and its verbatim output, with the import error named as the reason.
- The honest note from Phase 1: one import-level red covered all three behaviours; this was not three independent red/green passes, and the collapse and empty-string cases were never individually red for behavioural reasons.
- The cheapest-option ladder and its outcome: no deletion or reuse available, no stdlib `slugify` to lean on, `str.strip("-")` used instead of hand-rolled trimming, and `python-slugify` explicitly declined — both because the card restricts me to `re` and because the dependency would not have earned itself.
- What I deliberately did not build: no transliteration, no options, no CLI, no class, no exported symbol list, no precompiled pattern.
- The one edit beyond adding the function: rewriting the stale module docstring, with the reason.
- The green run's verbatim output and exit code, showing `OK` and 3 tests.
- The refactor step recorded as an intentional no-op.
- No delegation occurred, and why the work was too small to warrant it.
- The `tasks.md` flag: it does not exist; the checkbox was flipped in `CARD.md` instead.
- Files changed: `slug.py` (implementation), `CARD.md` (checkbox), `cycle-report.md` (new).

**What I would report back at the end:** C1 implemented and the suite green at 3 tests, with the two things a reviewer needs to weigh — that the red was a single import failure rather than three behavioural reds, and that the completion tick went into `CARD.md` because this workspace has no `tasks.md`.