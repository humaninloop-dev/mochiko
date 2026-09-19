# Cycle card C4 — hourly error counts

- [ ] **C4 · hourly error counts** — the CLI's upcoming `hourly` verb needs, for a list of log
  lines, how many ERROR lines fell in each hour. Add `errors_by_hour(lines)` to `logparse.py`
  returning a dict that maps an hour bucket to its ERROR count — buckets in first-seen order,
  hours with no errors absent. The bucket key is the first 13 characters of the line's timestamp
  field (`YYYY-MM-DDTHH`); timestamps are always the ISO form shown in the module docstring.
  Malformed lines and unknown levels are skipped, as the existing aggregators do. Write the
  failing test first in `test_logparse.py`, then make it pass. Story: S4 "as an on-call engineer
  I see which hour the errors clustered in".
  Brownfield exposure: `[EXTEND] logparse.py` — one new aggregator; `[EXTEND] test_logparse.py`
  — its test(s). `logcli.py` is not in this card (a later card wires the verb) and must keep
  working unchanged, as must every existing test.
  Suite: `python3 -m unittest -q test_logparse`.
  **TEST:** Setup: none · Action: `python3 -m unittest -q test_logparse` · Assert: `OK`; the 6 existing tests still pass alongside the new one(s).
