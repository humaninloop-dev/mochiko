FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

# Action plan — cycle card C1 (slugify)

## Phase 0 — Orient and confirm the ground truth

**Do:** I have already listed the workspace (three files: `slug.py`, `test_slug.py`, `CARD.md`) and read all three in full. That is the whole surface area, so nothing further needs locating.

**Read:** `CARD.md` (the card text and its embedded test contract), `slug.py` (a one-line module docstring, no code), `test_slug.py` (three assertions).

**What I take from the card:** the shape is fixed for me — a single function `slugify(text: str) -> str`, lowercase, collapse each run of non-`[A-Za-z0-9]` characters into one hyphen, trim leading/trailing hyphens, `re` from the standard library only, and nothing else in the file: no second function, no class, no command-line entry point. Brownfield exposure is stated as none and the file contents confirm it — there is no existing implementation, no caller, no interface to preserve.

**Delegation:** none. Delegating a cheap read here would cost more than it saves — the entire workspace is three files totalling under thirty lines and it is already in my context. The one sweep that could have justified a throwaway `Explore` at the haiku tier ("does anything else import `slug`?") is already answered by the directory listing: `test_slug.py` is the only other Python file. I will not spawn anything.

## Phase 1 — Cheapest-that-works check, before writing any code

**Do:** ask whether this code needs to exist at all, and whether something already in reach does it.

- Delete / don't build: no — the test pins a behaviour the project has asked for.
- Already in the codebase: no — `slug.py` is an empty stub and nothing else defines slug logic.
- Standard library: there is no stdlib slugifier. `unicodedata` and `str.translate` could contribute, but they do not remove the need for a pattern-based collapse and would be more code, not less.
- Installed dependency: a third-party package such as `python-slugify` would do this, but nothing is installed here and the card explicitly restricts me to stdlib `re`. Pulling in a dependency for three lines would be the expensive option, not the cheap one. Rejected, and I will say so in the report rather than leave the choice silent.

**Conclusion:** the cheapest thing that genuinely works is one `re.sub` plus `str.lower` plus `str.strip`. That is what I will write.

## Phase 2 — Re-confirm red, myself

**Do:** run `python3 -m unittest -q test_slug` before touching anything.

**Expect:** a failure at import — `ImportError: cannot import name 'slugify' from 'slug'` — meaning zero tests actually execute. This is the failure the card reports, and I want to see it with my own eyes rather than inherit the claim, because a test that passes for the wrong reason is exactly what this step exists to catch.

**Stop condition:** if the suite instead reports `OK`, or fails for some other reason (a collection error, a syntax error, a different missing name), I stop and do not write implementation code. That would mean the card and the workspace disagree.
- Branch if it already passes: the card is stale or I am in the wrong directory — I would confirm the working directory, and if it genuinely passes, report the card as already satisfied and write no code.
- Branch if it fails differently: I would report the actual failure text and ask which is authoritative before proceeding. Default while waiting: no code written.

**Note on the red phase generally:** the card says the failing test is already written and has been seen to fail, and I am not being asked to author a new one. I therefore write no additional test. The three existing assertions cover the basic case, the collapse-and-trim case, and the all-punctuation case, which is the behaviour the card describes. Adding a fourth test of my own would be work the task did not ask for.

## Phase 3 — Green: the minimum that passes

**Write:** `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-t_fzq_cr/ws/slug.py` — the only file I will modify.

**Content:** an `import re`, then a single function:

- lowercase the input,
- `re.sub` with the character class "one or more characters that are not an ASCII letter or digit" replaced by a single `-`,
- `.strip("-")` on the result,
- the signature typed exactly as the card gives it, `slugify(text: str) -> str`, with a one-line docstring.

I will apply the three steps in the order the card states them (lowercase first, then substitute, then strip), since the approach is fixed and I have no license to reorder it.

**Why I expect this to satisfy each assertion:**
- `"Hello World!"` → `"hello world!"` → `"hello-world-"` → `"hello-world"`.
- `"  --Rust & Cargo: 2026--  "` → the leading spaces-and-dashes are one run → one hyphen, `" & "` → one hyphen, `": "` → one hyphen, the trailing `"--  "` → one hyphen; strip removes the outer two → `"rust-cargo-2026"`.
- `"!!!"` → one run → `"-"` → strip → `""`.

**One in-file tidy I will make and disclose:** the existing module docstring asserts "`slugify` is not implemented yet". After my change that sentence is false, and it sits in the very file I am editing describing the very thing I am implementing. I will replace it with an accurate one-line description. I will call this out in the report as a deviation from a strictly zero-touch edit rather than slipping it in.

**What I will not do,** even though the file is open in front of me: no `__all__`, no pre-compiled module-level pattern, no `unicodedata` normalisation for accented characters, no max-length or custom-separator parameter, no `if __name__ == "__main__"` block, no second helper. None of these are asked for; each would be scope I invented.

## Phase 4 — Verify green

**Run:** `python3 -m unittest -q test_slug`.

**Expect exactly what the card's test contract names:** `OK` with 3 tests run, zero failures, zero errors. I check the count, not just the word `OK` — a suite that silently collected fewer tests would also print `OK`.

**Stop condition:** if any assertion fails, I do not paper over it. I read the actual-versus-expected values, fix the substitution or the ordering, and re-run — staying inside `slugify`. I would not edit `test_slug.py` to make a failure go away; the test is the specification here, and changing it would be the silent workaround I refuse.

## Phase 5 — Refactor

**Do:** look at the result honestly and expect to change nothing. Three chained standard-library calls in one expression have no duplication to extract and no name to improve. I will say "no refactor was warranted" in the report rather than manufacture a cosmetic change to fill the step.

**Re-run** the suite if I do touch anything, so green is confirmed after the final state of the file, not before it.

## Phase 6 — Record

**Do:** tick the card's checkbox and write the cycle report.

- **Checkbox:** the workspace has no `tasks.md`. The unticked box for C1 lives on line 3 of `CARD.md`. I will flag this discrepancy — my normal deliverable names `tasks.md` — and, under my stated default, mark the box in `CARD.md` (`- [ ]` → `- [x]`), changing nothing else in that file. Branch if told otherwise: if the ruling is that a `tasks.md` should exist elsewhere and be the system of record, I would leave `CARD.md` untouched and tick the box there instead.
- **Report:** `cycle-report.md` in the workspace root, following the report format from my TDD-cycle skill (which I would load at execution time and treat as the authority on its structure rather than inventing headings here).

**What the report will actually say:**
- Red was re-confirmed independently, with the exact import error quoted.
- Green was reached with a three-operation implementation; the suite went from an import error to 3 tests passing.
- The cheap-option ladder I walked and why I landed on stdlib `re`: no deletion possible, nothing reusable in the codebase, no stdlib slugifier, and a third-party slug package rejected as both out of the card's bounds and more expensive than the three lines it would replace.
- The one deviation: rewriting the now-false module docstring.
- No refactor was needed, and why.
- Two observations noted but deliberately not acted on: lowercasing before substituting means a small number of non-ASCII characters whose lowercase form contains an ASCII letter (for example `İ`) would contribute that letter to the slug rather than becoming a hyphen — the tests do not exercise it and the card fixed the ordering, so I leave it as a flagged question rather than a change; and the checkbox lives in `CARD.md` because no `tasks.md` exists.
- No blockers, no assumptions made in place of a missing answer.