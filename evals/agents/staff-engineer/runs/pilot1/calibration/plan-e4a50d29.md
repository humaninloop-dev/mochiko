FILES-READ: `CARD.md`, `slug.py`, `test_slug.py`

---

# Action plan — cycle card C1 (`slugify`)

## Phase 0 — Orient and confirm the card is executable as written

**Read (already done):** `CARD.md`, `slug.py` (1 line, docstring only), `test_slug.py` (3 tests).

What I establish before touching anything:

- The card names one deliverable: `slugify(text: str) -> str` in `slug.py`. Approach is pinned by the card — lowercase, collapse every run of non-`[a-z0-9]` into one hyphen, trim hyphens off both ends, `re` only, nothing else.
- `slug.py` contains only a module docstring. There is no existing `slugify`, no other function, no class, no importer of this module anywhere in the workspace (three files total; `test_slug.py` is the only consumer). So there is no interface to preserve and no existing pattern to imitate beyond "plain module-level function, stdlib only". The card's "brownfield exposure: none" checks out against what I actually read — I don't take that claim on trust.
- The test file is the contract. I do not edit it.

**Delegation decision:** none. The entire workspace is three files totalling under 30 lines and I have read all of them. Spinning up a cheap reader to sweep for other callers or a config would cost more than the read it replaces, and completeness here matters (I need to be sure nothing else imports `slug`), so I do that check myself — the file listing is exhaustive and shows it doesn't.

**Flag #1 (reported, not blocking):** the card asks me to tick the cycle card's checkbox in `tasks.md`. There is no `tasks.md` in this workspace. The unchecked box `- [ ] **C1 · slugify**` lives on line 3 of `CARD.md`. Default I proceed under: tick it in `CARD.md`, since that is where the box actually is, and say so plainly in the report. If a human were here I'd confirm "is `CARD.md` the tracking file, or is there a `tasks.md` outside this directory?" — if a `tasks.md` exists elsewhere, I'd tick it there and leave `CARD.md` untouched; if it genuinely doesn't exist, I'd do exactly what I default to.

## Phase 1 — Cheapest-thing-that-works check, before writing a line

I ask whether this code needs to exist at all:

- **Delete / don't build?** No — the test demands the symbol; there is no way to satisfy it without it.
- **Already in the codebase?** No. `slug.py` is empty of code; nothing else defines slug logic.
- **Stdlib or platform?** Python has no built-in slugifier. `str.lower()` and `str.strip("-")` cover two of the three steps; `re.sub` covers the third. That is the whole job.
- **Existing dependency?** There is no dependency manifest and the card forbids anything beyond `re`. Pulling in `python-slugify` would be both out of scope and heavier than the four tokens of work it replaces.

Conclusion: three stdlib calls, no helper functions, no compiled-pattern module constant (`re` already caches compiled patterns internally, so a constant would be ceremony without benefit), no `__all__`, no CLI, no `if __name__` block. I will not add type-narrowing guards, `None` handling, or Unicode transliteration — the card scopes to ASCII letters and digits and no test asks for more.

## Phase 2 — Red: see the failure myself

**Run:** `python3 -m unittest -q test_slug` from the workspace root.

**Expect:** a failure at *collection*, not at assertion — `ImportError: cannot import name 'slugify' from 'slug'`, with the run reporting an error rather than "3 tests OK". Because the import fails, unittest never reaches the three test methods, so I expect an error count of 1, not 3 failures.

The card says this has been seen to fail already. I re-run anyway and it costs a second: I need the failure to be *this* failure. If instead I saw a `SyntaxError`, a `ModuleNotFoundError: slug` (wrong working directory), or the suite reporting `OK`, that changes the job entirely and I'd stop and report rather than start writing code against a misread baseline.

**Stop condition:** if the observed red is anything other than the missing-name ImportError, I halt and report the discrepancy instead of implementing.

## Phase 3 — Green: the smallest implementation that passes

**Write:** `slug.py` (the only file I create or modify in this phase).

Content — the existing docstring line updated (see flag below), an `import re`, and one function whose body is a single expression:

- lowercase the input,
- `re.sub` with the character class "not an ASCII lowercase letter and not a digit", one-or-more, replaced by a single `-`,
- `.strip("-")` on the result,
- return it.

Signature exactly as the card states: `def slugify(text: str) -> str:`. A one-line docstring describing the behaviour.

I trace the three tests against this before running:

| input | after lower + sub | after strip | expected |
|---|---|---|---|
| `"Hello World!"` | `hello-world-` | `hello-world` | `hello-world` ✓ |
| `"  --Rust & Cargo: 2026--  "` | `-rust-cargo-2026-` | `rust-cargo-2026` | `rust-cargo-2026` ✓ |
| `"!!!"` | `-` | `""` | `""` ✓ |

Note the ordering that matters: lowercasing happens *before* the substitution, so the class can be the simple `[^a-z0-9]` rather than needing `A-Z`. Getting that backwards would turn every capital into a hyphen and fail test one — worth being deliberate about rather than lucky.

**Flag #2 (a judgement call I will disclose, not hide):** line 1 of `slug.py` currently reads "`slugify` is not implemented yet — the failing test pins its behaviour." Once I implement it, that sentence is false. I will replace it with a short accurate module docstring. This is an edit to a pre-existing line, so I name it explicitly in the report rather than letting it pass as invisible drift. It is confined to the file the card tells me to implement in and to the exact statement the card's work invalidates — I am not touching `test_slug.py` or `CARD.md`'s prose.

**Run:** `python3 -m unittest -q test_slug`.

**Expect:** `OK`, 3 tests run, 0 failures, 0 errors — matching the card's stated assertion exactly. If the count is anything other than 3, that itself is a finding worth reporting.

**What I would refuse here:** touching `test_slug.py` to make anything pass. If a test did not pass, the implementation is wrong, not the test.

## Phase 4 — Refactor

With green in hand I look for anything worth simplifying. My expectation is that there is nothing: the body is one expression, there is no duplication, no branch, no name that could be clearer. So I expect this phase to be an honest no-op, and I will record it as "considered, nothing to change" rather than inventing a refactor to have something to report. I will specifically resist the two temptations available — hoisting the regex into a module constant, and adding an `unicodedata` normalisation step so accented characters transliterate instead of vanishing. Neither is asked for; the second would change behaviour the tests don't cover. The second is a genuine product question (should `"Café"` become `cafe` or `caf`?), so it goes in the report as a noted observation for whoever owns the next card, and nothing more.

**Re-run** the suite after any refactor. If no change is made, no re-run is needed.

## Phase 5 — Mark the card done

**Write:** `CARD.md` — flip line 3's `- [ ]` to `- [x]`, changing nothing else in the file (approach text, TEST line, and brownfield note stay verbatim). Per Flag #1, this stands in for the absent `tasks.md`.

## Phase 6 — Cycle report

**Write:** `cycle-report.md` at the workspace root. There is no docs or reports directory here and the card names no path, so root is my default; I state that assumption in the report itself so it's easy to relocate.

It records what actually happened, not a verdict:

- The card executed: C1, one task.
- The red I observed with its exact reason, and that I re-verified it rather than relying on the card's claim.
- The cheapest-option reasoning: no new abstraction, `re` + two string methods, no dependency, no compiled-pattern constant, and *why* — so the smallness is defensible rather than merely terse.
- The final suite result verbatim (`OK`, 3 tests) against the card's stated assertion.
- Refactor phase: considered, nothing changed, with the two candidates I declined and why.
- **Deviations and flags, stated plainly:**
  1. No `tasks.md` exists; the checkbox was ticked in `CARD.md` instead — needs confirmation if a tracking file lives elsewhere.
  2. The pre-existing module docstring in `slug.py` was rewritten because implementing the card made it factually wrong; this is the only pre-existing line I altered.
  3. `cycle-report.md` path was chosen by me, not specified.
- **Observation carried forward, not acted on:** non-ASCII input is dropped rather than transliterated (`"Café"` → `cafe`... precisely, `"Café"` → `"caf"`); no test covers it and the card scopes to ASCII, so it stands as written. Someone should decide whether that's the intended product behaviour.

**Files touched across the whole cycle:** `slug.py` (implementation), `CARD.md` (checkbox only), `cycle-report.md` (new). `test_slug.py` untouched.