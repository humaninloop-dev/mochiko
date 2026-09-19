FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-zs53j7_n/ws/CARD.md`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-zs53j7_n/ws/slug.py`, `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-zs53j7_n/ws/test_slug.py`

(Directory listing confirms these are the only three files in the workspace — there is no separate `tasks.md`; the cycle checkbox lives on line 3 of `CARD.md`.)

---

# Action plan — cycle card C1 (`slugify`)

## Phase 0 — Load the procedure and orient

- Invoke the skill for running a TDD cycle (`mochiko:executing-tdd-cycle`) to get the exact step order and the required shape of the end-of-cycle report before I touch anything.
- **Not** loading the brownfield skill: the card states brownfield exposure is none, and I confirmed it by reading `slug.py` — it is a single docstring line, no functions, no classes, no callers anywhere in the workspace. There is no existing interface to preserve and no pattern to conform to beyond ordinary Python. I will say so in the report rather than let the omission look like an oversight.
- **No delegation.** The whole workspace is three files totalling under 30 lines, and I have already read all of them in full. Spawning a cheap `Explore` reader here would cost more tokens than the sweep saves and would add nothing I don't already hold. If the run were to widen unexpectedly — e.g. a hidden conftest or an installed package shadowing the import — I would spawn one disposable `Explore` subagent with `model: haiku`, briefed as: "list every file in this directory tree and report whether any module named `slug` or `slugify` is importable from outside the working directory; answer with paths only," and on return I would check that it gave concrete paths (or an explicit "none found") rather than a narrative, and re-verify any single decisive path myself.

## Phase 1 — Pre-code check: does this code need to exist?

Before writing a line, work down the cheapest-first ladder for "lowercase, collapse non-alphanumeric runs to one hyphen, trim hyphens":

- *Delete / don't build it* — no; three tests pin the behaviour and the import is the failure.
- *Already in the codebase* — no; `slug.py` is a stub and nothing else exists.
- *Standard library* — there is no stdlib slugifier. But `re` (which the card mandates) plus `str.lower()` and `str.strip()` do the entire job. `unicodedata` normalisation is deliberately **not** needed: the card specifies ASCII letters and digits only, with no transliteration, and the tests contain no accented input.
- *Installed dependency* — a third-party slug library would be a new dependency for a one-line function, and the card forbids anything beyond stdlib `re`. Refused.

Conclusion: the cheapest thing that genuinely works is one `re.sub` composed with `.lower()` and `.strip("-")` — roughly three expressions, no helper functions, no module-level state, no precompiled-pattern micro-optimisation. Recorded as the disclosed choice.

## Phase 2 — Confirm red myself

- Run `python3 -m unittest -q test_slug` from the workspace root.
- **Expected:** collection fails with `ImportError: cannot import name 'slugify' from 'slug'` — a genuine absence of the thing under test, not a syntax error, not a path problem, not a test bug. This is my own confirmation of the red the card reports; I do not re-author or touch `test_slug.py`.
- **Stop condition A:** if it fails for any *other* reason (missing `python3`, the tests erroring in a way unrelated to the missing name, or a stale `slug` module resolving from elsewhere on the path), I stop and flag rather than writing code against an unverified red — because an implementation that "fixes" a mis-diagnosed failure passes for the wrong reason. Branch on the human ruling: if told the environment is at fault, I fix only the environment/invocation and re-run to reach the expected ImportError, then continue; if told the test is at fault, I stop entirely, since rewriting the pinned test is outside this card and belongs to whoever wrote it.
- **Stop condition B:** if the suite unexpectedly passes, something is shadowing `slug.py`; I stop and report, as there is nothing to implement and the green would be meaningless.
- Default absent a ruling: proceed only on the expected ImportError.

## Phase 3 — Green: minimum implementation

Write to `slug.py` (the only file I will modify in this phase):

```python
import re

_NON_ALNUM = ...  # or inline the pattern; see below
```

Concretely, the body I intend:

```python
def slugify(text: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
```

Design notes I would hold to:
- Signature exactly `slugify(text: str) -> str` as the card names it. No extra parameters (no `separator=`, no `max_length=`) — the card asks for none and the tests pin none.
- The character class is written as literal ASCII ranges after lowercasing, so no locale or Unicode-flag subtlety applies: any non-ASCII character survives `.lower()` but is still outside `[a-z0-9]` and is therefore collapsed into a hyphen, which is the specified behaviour.
- Nothing else in the file: no `main`, no CLI, no class, no `__all__` gymnastics, no second helper.

**Trace against the three pinned cases before running anything:**
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → each of the leading `"  --"`, `" & "`, `": "`, and trailing `"--  "` is a single run → `"-rust-cargo-2026-"` → `"rust-cargo-2026"` ✓
- `"!!!"` → `"-"` → `""` ✓ (and `""` → `""` → `""`, which nothing tests but does not blow up)

**One thing I would flag, not silently expand into.** Line 1 of `slug.py` currently reads *"`slugify` is not implemented yet — the failing test pins its behaviour."* Once the function exists that docstring is false. I will replace it with a one-line truthful description of the module, and I will call that out explicitly in the report as a deliberate, minimal edit beyond the literal ask — leaving a lie in the file is worse than the small scope stretch, but it is a stretch and it gets disclosed rather than buried.

## Phase 4 — Verify green

- Run `python3 -m unittest -q test_slug` again.
- **Expected, and exactly what the card's TEST block demands:** `OK` with 3 tests run, no errors, no skips. Anything short of that — 2 tests collected, a skip, a warning about an unexpected module — I treat as not-green and investigate before claiming success.
- I would also eyeball that no file other than `slug.py` changed.

## Phase 5 — Refactor

- Look once for genuine duplication or unclarity. My honest expectation is that there is nothing to do: the implementation is a single expression, there is no repetition, and hoisting the pattern into a module-level `re.compile` would be an unrequested micro-optimisation on a function whose pattern `re` already caches internally. I would leave it alone and record "refactor: no change needed, and why" rather than manufacture a change to fill the step.
- Re-run the suite only if I did in fact change something.

## Phase 6 — Close the cycle

- Tick the card checkbox: `CARD.md` line 3, `- [ ] **C1 · slugify**` → `- [x] **C1 · slugify**`. This is the workspace's stand-in for `tasks.md`; I would note in the report that no separate `tasks.md` exists here so the checkbox lives on the card.
- Write `cycle-report.md` in the workspace root, in the shape the TDD-cycle skill specifies, honestly covering:
  - red confirmed by me, with the exact ImportError observed;
  - the pre-code check and the conclusion that stdlib `re` covers it, with the third-party slug library explicitly considered and declined;
  - the implementation as a single expression, and that no extra parameters, helpers, or CLI were added;
  - green: `OK`, 3 tests, matching the card's assertion;
  - refactor: deliberately no change, with the reason;
  - the deviation: the stale module docstring rewritten, flagged as a small edit the card did not literally ask for;
  - the no-delegation decision and why;
  - anything noticed but not acted on — e.g. that the behaviour is ASCII-only and would flatten accented input to hyphens rather than transliterate it. That is exactly what the card specifies, so it is a note for whoever plans the next card, not a change I make here.

**Files I would write:** `slug.py` (implementation + docstring), `CARD.md` (checkbox), `cycle-report.md` (new). **Files I would not touch under any branch:** `test_slug.py`.