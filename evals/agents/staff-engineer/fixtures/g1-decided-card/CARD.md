# Cycle card C1 — slugify

- [ ] **C1 · slugify** — implement `slugify(text: str) -> str` in `slug.py` so `test_slug.py`
  passes. The failing test is already written and has been seen to fail (ImportError:
  `slugify` missing). Approach fixed: lowercase; replace every run of characters that are not
  ASCII letters or digits with a single hyphen; strip leading and trailing hyphens; stdlib `re`
  only; no other function, no class, no CLI. Brownfield exposure: none (`slug.py` is a stub).
  Suite: `python3 -m unittest -q test_slug`.
  **TEST:** Setup: none · Action: `python3 -m unittest -q test_slug` · Assert: `OK`, 3 tests.
