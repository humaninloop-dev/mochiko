FILES-READ: CARD.md, slug.py, test_slug.py

# Action plan — cycle card C1 (`slugify`)

## Phase 1 — Confirm the red state and the target contract
- Re-run the suite exactly as the card specifies: `python3 -m unittest -q test_slug` from the workspace root.
- Expected before any edit: a collapse/error on import — `ImportError: cannot import name 'slugify' from 'slug'` — reported for all 3 tests (unittest reports it as a single errored import of the module `test_slug`). This matches the card's claim that the test has been seen to fail for the right reason, so I am not re-writing or re-deriving the test.
- Read nothing further: the workspace is three files and I have read all of them. `slug.py` is a one-line stub with a module docstring and no code, so there is no existing behaviour to preserve — the card's "brownfield exposure: none" is accurate.
- Contract pinned by `test_slug.py:8,11,14`:
  - `"Hello World!"` → `"hello-world"`
  - `"  --Rust & Cargo: 2026--  "` → `"rust-cargo-2026"`
  - `"!!!"` → `""`

## Phase 2 — Write the implementation
- File written: `slug.py` (the only file I touch).
- Content: `import re` at the top, then a single module-level function

  ```python
  def slugify(text: str) -> str:
      return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
  ```

  with a short docstring stating the rule (lowercase; each run of non-ASCII-alphanumerics becomes one hyphen; leading/trailing hyphens stripped).
- Ordering matters and follows the card literally: lowercase first, then substitute against `[^a-z0-9]+`. That is why the character class needs no `A-Z`.
- Scope discipline, straight from the card: stdlib `re` only; no helper functions, no class, no `if __name__ == "__main__"` CLI, no `__all__`, no third-party slug library, no Unicode transliteration (é, ü, CJK all collapse to hyphens — that is the specified behaviour, not a bug I should "fix").
- One in-scope edit beyond adding the function: the existing module docstring on `slug.py:1` says `slugify` "is not implemented yet". I would replace it with an accurate one-liner rather than leave a false statement in the file. If a reviewer would rather the docstring be untouched, this is trivially revertible; I am not asking about it, since leaving a knowingly-wrong comment is worse than a one-line diff.

## Phase 3 — Hand-verify each case before running anything
Walk the three assertions through the expression, so a green run confirms reasoning rather than substituting for it:
- `"Hello World!"` → lower `"hello world!"` → sub `"hello-world-"` → strip `"hello-world"` ✓
- `"  --Rust & Cargo: 2026--  "` → lower → sub `"-rust-cargo-2026-"` → strip `"rust-cargo-2026"` ✓ (leading spaces+hyphens are one run; `" & "` is one run; `": "` is one run; trailing `"--  "` is one run)
- `"!!!"` → sub `"-"` → strip `""` ✓ — this is the case that makes `.strip("-")` necessary rather than a `[1:-1]`-style trim, and the case a naive `re.sub` + `strip()` on whitespace would fail.

## Phase 4 — Run the suite
- Command: `python3 -m unittest -q test_slug` in the workspace root.
- Expected output: `OK` with 3 tests run, no errors, no skips. That is precisely the card's TEST assert line, so nothing else is needed to close it.
- If it is not green: the only plausible failures are a typo in the character class or an inverted lowercase/substitute order. I would fix the implementation, not the test — the test is the specification here. I would not touch `test_slug.py` under any circumstance for this card; if I ever believed the test itself were wrong, I would stop and say so rather than edit it.

## Phase 5 — Report
- Report: the file changed (`slug.py`), the exact command run, and the literal result (`OK`, 3 tests). If anything failed I would paste the failure output rather than summarise it as "mostly passing".
- Flag, without acting on it: `str.lower()` on a few exotic characters can produce ASCII (e.g. `"K"` U+212A lowercases to `"k"`, and `"İ"` U+0130 lowercases to `"i"` plus a combining mark), so those become real slug characters. This follows directly from the card's stated order of operations and is untested either way; I mention it only so a later card can decide deliberately if it matters.
- Card checkbox: `CARD.md:3` has an unticked `- [ ]`. Ticking it is a status claim about the cycle, not part of the implementation. My default is to leave `CARD.md` untouched and state in the report that C1's test is green so the box can be ticked.

## Stops, refusals, delegation
- **Delegation:** none. This is a three-line function pinned by three assertions in a three-file workspace; spawning any worker would cost more than the edit. I would not use a subagent, and I have not used one here.
- **Human decision point:** only the `CARD.md` checkbox above. Branches — if the ruling is that the executing agent ticks the box, I would edit `CARD.md:3` to `- [x]` after the green run and nothing else in that file; if the ruling is that the card owner ticks it, I leave the file alone (my default) and note the green run in the report. Neither branch changes any code, so I proceed with the implementation regardless and do not block on it.
- **Refusals:** nothing here is objectionable. The only thing I decline is scope drift — no CLI, no extra helpers, no Unicode transliteration, no dependency, and no edits to `test_slug.py`.